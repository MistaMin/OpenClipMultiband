//! `clip-tone`: clippers whose behaviour depends on the programme or shapes the tone.
//!
//! * [`SmoothClip`] ("Smooth Clip"): every clip's gain reduction is spread over a short window
//!   with bounded look-ahead, so peaks are held at the ceiling by a smooth gain instead of a
//!   bent waveform. Sample-peak exact; the detector reads an analytic (Hilbert) envelope, so
//!   inter-sample peaks are mostly caught too.
//! * [`ToneClip`] ("Tone Clip"): a static tanh-knee clip with peak-held residual ADAA1, inside
//!   a matched pre-/de-emphasis pair (Shape: highs clip harder, the distortion comes out darker)
//!   and with a bias for even harmonics whose DC is removed exactly (Character).
//!
//! The maths, measurements and limits are in `REPORT.md` next to this crate.
#![forbid(unsafe_code)]

use clip_api::ClipEngine;

type Factory = fn() -> Box<dyn ClipEngine>;

pub const ENGINES: &[(&str, Factory)] = &[
    ("Smooth Clip", || Box::new(SmoothClip::new())),
    ("Tone Clip", || Box::new(ToneClip::new())),
];

/// Parameter smoothing time constant.
const SMOOTH_MS: f64 = 5.0;

fn one_pole(ms: f64, fs: f64) -> f64 {
    1.0 - (-1.0 / (ms * 1e-3 * fs)).exp()
}

/// Flushes values that would decay into the subnormal range (slow on x86).
fn ftz(x: f64) -> f64 {
    if x.abs() < 1e-30 {
        0.0
    } else {
        x
    }
}

/// Parameters smoothed per sample: `[ceiling, shape, character]`.
struct Params {
    target: [f64; 3],
    cur: [f64; 3],
    a: [f64; 3],
    /// The first `set` after `prepare`/`reset` jumps instead of gliding.
    fresh: bool,
}

impl Params {
    fn new() -> Self {
        Self { target: [1.0, 0.5, 0.5], cur: [1.0, 0.5, 0.5], a: [1.0; 3], fresh: true }
    }
    fn set(&mut self, ceiling: f32, shape: f32, character: f32) {
        let clamp = |v: f32, lo: f32, hi: f32| if v.is_finite() { v.clamp(lo, hi) as f64 } else { lo as f64 };
        self.target = [clamp(ceiling, 0.01, 1.0), clamp(shape, 0.0, 1.0), clamp(character, 0.0, 1.0)];
        if self.fresh {
            self.cur = self.target;
        }
    }
    /// Advances one sample; true when anything derived from the values must be recomputed.
    fn tick(&mut self) -> bool {
        let old = self.cur;
        for ((c, t), a) in self.cur.iter_mut().zip(self.target).zip(self.a) {
            *c += a * (t - *c);
        }
        let moved = self.fresh || old != self.cur;
        self.fresh = false;
        moved
    }
}

// ---------------------------------------------------------------------------------------------
// Smooth Clip
// ---------------------------------------------------------------------------------------------

/// Hilbert half-length (odd); the FIR is `2 * HILBERT + 1` taps, same in samples at every rate.
const HILBERT: usize = 15;
/// Box length at 48 kHz; scaled with the rate. Latency is `HILBERT + 2 * BOX - 2` samples.
const BOX_48K: usize = 24;
/// Longest extra gain recovery (Character 1), ms.
const RELEASE_MS: f64 = 30.0;
/// Knee start at Shape 1, dB under the ceiling.
const KNEE_DB: f64 = 6.0;
/// Fixed-point scale of the gain: sums of floored integers are exact, so they never drift.
const ONE: f64 = (1u64 << 30) as f64;

/// Look-ahead gain-spreading clipper. See the crate docs and `REPORT.md`.
pub struct SmoothClip {
    p: Params,
    fs: f64,
    k: usize,
    mask: usize,
    /// Audio delay rings and the ceiling that was used for each detector sample.
    xl: Vec<f32>,
    xr: Vec<f32>,
    ceil: Vec<f64>,
    n: usize,
    /// Running minimum over `2k - 1` samples: monotone deque of (time, gain).
    dq_t: Vec<usize>,
    dq_g: Vec<f64>,
    head: usize,
    tail: usize,
    rel: f64,
    box1: Vec<u64>,
    box2: Vec<u64>,
    bi: usize,
    sum1: u64,
    sum2: u64,
    hil: [f64; HILBERT.div_ceil(2)],
    /// Derived from Shape and Character: knee start / ceiling, recovery coefficient.
    knee: f64,
    alpha: f64,
}

impl SmoothClip {
    pub fn new() -> Self {
        let mut hil = [0.0; HILBERT.div_ceil(2)];
        for (i, h) in hil.iter_mut().enumerate() {
            // Ideal 2/(pi k) on odd k, Blackman window over the 2H+1 taps.
            let k = (2 * i + 1) as f64;
            let ph = std::f64::consts::PI * k / (HILBERT + 1) as f64;
            *h = 2.0 / (std::f64::consts::PI * k) * (0.42 + 0.5 * ph.cos() + 0.08 * (2.0 * ph).cos());
        }
        let mut s = Self {
            p: Params::new(),
            fs: 48_000.0,
            k: BOX_48K,
            mask: 0,
            xl: Vec::new(),
            xr: Vec::new(),
            ceil: Vec::new(),
            n: 0,
            dq_t: Vec::new(),
            dq_g: Vec::new(),
            head: 0,
            tail: 0,
            rel: 1.0,
            box1: Vec::new(),
            box2: Vec::new(),
            bi: 0,
            sum1: 0,
            sum2: 0,
            hil,
            knee: 1.0,
            alpha: 0.0,
        };
        s.prepare(48_000.0, 512);
        s
    }

    fn delay(&self) -> usize {
        HILBERT + 2 * self.k - 2
    }

    /// Gain that brings envelope `e` to the knee curve under `c` (knee from `t` up).
    fn required(e: f64, c: f64, t: f64) -> f64 {
        if e <= t {
            return 1.0;
        }
        let span = c - t;
        let out = if span < 1e-9 { c } else { t + span * ((e - t) / span).tanh() };
        out / e
    }
}

impl Default for SmoothClip {
    fn default() -> Self {
        Self::new()
    }
}

impl ClipEngine for SmoothClip {
    fn name(&self) -> &'static str {
        "Smooth Clip"
    }

    fn prepare(&mut self, sample_rate: f32, _max_block: usize) {
        self.fs = sample_rate.max(8_000.0) as f64;
        self.k = ((BOX_48K as f64 * self.fs / 48_000.0).round() as usize).max(HILBERT);
        let size = (self.delay() + 1).next_power_of_two();
        self.mask = size - 1;
        self.xl = vec![0.0; size];
        self.xr = vec![0.0; size];
        self.ceil = vec![1.0; size];
        self.dq_t = vec![0; size];
        self.dq_g = vec![1.0; size];
        self.box1 = vec![0; self.k];
        self.box2 = vec![0; self.k];
        self.p.a = [one_pole(SMOOTH_MS, self.fs); 3];
        self.reset();
    }

    fn reset(&mut self) {
        self.xl.fill(0.0);
        self.xr.fill(0.0);
        self.ceil.fill(self.p.target[0]);
        self.n = 0;
        self.head = 0;
        self.tail = 0;
        self.rel = 1.0;
        let q = ONE as u64;
        self.box1.fill(q);
        self.box2.fill(q * self.k as u64);
        self.sum1 = q * self.k as u64;
        self.sum2 = self.sum1 * self.k as u64;
        self.bi = 0;
        self.p.cur = self.p.target;
        self.p.fresh = true;
    }

    fn set(&mut self, ceiling: f32, shape: f32, character: f32) {
        self.p.set(ceiling, shape, character);
    }

    fn latency(&self) -> usize {
        self.delay()
    }

    fn process(&mut self, left: &mut [f32], right: &mut [f32]) {
        let len = left.len().min(right.len());
        let (k, m, d, w) = (self.k, self.mask, self.delay(), 2 * self.k - 1);
        let scale = 1.0 / (ONE * (k * k) as f64);
        for i in 0..len {
            if self.p.tick() {
                let [_, shape, character] = self.p.cur;
                self.knee = 10f64.powf(-KNEE_DB * shape / 20.0);
                let tau = RELEASE_MS * 1e-3 * character * self.fs;
                self.alpha = if tau > 1e-3 { (-1.0 / tau).exp() } else { 0.0 };
            }
            let (c, t, alpha) = (self.p.cur[0], self.p.cur[0] * self.knee, self.alpha);

            let n = self.n;
            self.xl[n & m] = left[i];
            self.xr[n & m] = right[i];
            self.ceil[n & m] = c;

            // Analytic envelope of x[n - H], the louder channel.
            let mut env2: f64 = 0.0;
            for x in [&self.xl, &self.xr] {
                let mid = x[n.wrapping_sub(HILBERT) & m] as f64;
                let mut h = 0.0;
                for (j, cj) in self.hil.iter().enumerate() {
                    let o = 2 * j + 1;
                    h += cj * (x[n.wrapping_sub(HILBERT + o) & m] as f64 - x[n.wrapping_sub(HILBERT - o) & m] as f64);
                }
                env2 = env2.max(mid * mid + h * h);
            }
            let g = if env2 > 1e-30 { Self::required(env2.sqrt(), c, t) } else { 1.0 };

            // Minimum over the last 2k-1 gains.
            while self.tail != self.head && self.dq_g[(self.tail - 1) & m] >= g {
                self.tail -= 1;
            }
            self.dq_t[self.tail & m] = n;
            self.dq_g[self.tail & m] = g;
            self.tail += 1;
            while n - self.dq_t[self.head & m] >= w {
                self.head += 1;
            }
            let gmin = self.dq_g[self.head & m];

            // Instant attack, optional slower recovery; never above the minimum.
            self.rel = if gmin < self.rel { gmin } else { self.rel + (1.0 - alpha) * (gmin - self.rel) };

            // Two exact boxes of k: a triangle that never exceeds the minimum it smooths.
            let q = (self.rel * ONE) as u64;
            self.sum1 = self.sum1 + q - self.box1[self.bi];
            self.box1[self.bi] = q;
            self.sum2 = self.sum2 + self.sum1 - self.box2[self.bi];
            self.box2[self.bi] = self.sum1;
            self.bi = if self.bi + 1 == k { 0 } else { self.bi + 1 };
            let s = self.sum2 as f64 * scale;

            let cd = self.ceil[n.wrapping_sub(2 * k - 2) & m] as f32;
            left[i] = ((self.xl[n.wrapping_sub(d) & m] as f64 * s) as f32).clamp(-cd, cd);
            right[i] = ((self.xr[n.wrapping_sub(d) & m] as f64 * s) as f32).clamp(-cd, cd);
            self.n = n + 1;
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Tone Clip
// ---------------------------------------------------------------------------------------------

/// Knee start of the static curve (fraction of the rail).
const KNEE: f64 = 0.5;
/// Largest bias (Character 1), fraction of the rail. Must stay under `KNEE`.
const BIAS_MAX: f64 = 0.25;
/// Largest pre-emphasis (Shape 1), dB at Nyquist.
const EMPH_DB: f64 = 12.0;
/// Emphasis shelf corner.
const EMPH_HZ: f64 = 3_000.0;
/// Corner of the two one-poles that track the even residual's DC.
const DC_HZ: f64 = 8.0;
/// Release of the held compression measure (peak-held residual ADAA1).
const HOLD_MS: f64 = 0.5;
/// Character glides slower than the other settings, so the DC tracker keeps up with it.
const CHAR_MS: f64 = 100.0;
/// Under this interval width the ADAA mean is the two-point average.
const CLOSE: f64 = 1e-7;

/// Tanh-knee curve with rail 1: value and antiderivative. Linear up to `KNEE`, then
/// `KNEE + (1 - KNEE) tanh((|t| - KNEE) / (1 - KNEE))`; continuous to the second derivative.
fn curve(t: f64) -> (f64, f64) {
    let a = t.abs();
    if a <= KNEE {
        return (t, 0.5 * t * t);
    }
    let r = 1.0 - KNEE;
    let z = (a - KNEE) / r;
    let e = (-2.0 * z).exp();
    let f = KNEE + r * (1.0 - e) / (1.0 + e);
    let big_f = 0.5 * KNEE * KNEE + KNEE * (a - KNEE) + r * r * (z + e.ln_1p() - std::f64::consts::LN_2);
    (f.copysign(t), big_f)
}

fn curve_f(t: f64) -> f64 {
    if t.abs() <= KNEE {
        t
    } else {
        let z = (t.abs() - KNEE) / (1.0 - KNEE);
        (KNEE + (1.0 - KNEE) * z.tanh()).copysign(t)
    }
}

#[derive(Clone, Copy, Default)]
struct ToneChan {
    /// Pre-emphasis input and output histories.
    x1: f64,
    p1: f64,
    /// Previous normalised core input, all-pass output, held compression.
    u1: f64,
    ap1: f64,
    hold: f64,
    /// Cached curve point of the previous sample (reused while bias and scale are steady).
    t1: f64,
    f1: f64,
    big_f1: f64,
    /// Even-residual DC tracker.
    m1: f64,
    m2: f64,
    /// De-emphasis histories.
    v1: f64,
    y1: f64,
}

/// Emphasis / even-harmonic tanh-knee clipper. See the crate docs and `REPORT.md`.
pub struct ToneClip {
    p: Params,
    pole: f64,
    rho: f64,
    a_dc: f64,
    /// De-emphasis numerator (b0, b1), derived from Shape.
    b: (f64, f64),
    ch: [ToneChan; 2],
}

impl ToneClip {
    pub fn new() -> Self {
        let mut s = Self { p: Params::new(), pole: 0.0, rho: 0.0, a_dc: 0.0, b: (1.0, 0.0), ch: [ToneChan::default(); 2] };
        s.prepare(48_000.0, 512);
        s
    }

    #[allow(clippy::too_many_arguments)]
    fn tick(ch: &mut ToneChan, x: f64, c: f64, b0: f64, b1: f64, pole: f64, bias: f64, rho: f64, a_dc: f64) -> f64 {
        // Pre-emphasis P = (1 - p z^-1) / (b0 + b1 z^-1).
        let xp = ftz((x - pole * ch.x1 - b1 * ch.p1) / b0);
        ch.x1 = x;
        ch.p1 = xp;

        // Core, in units of the ceiling: s * (f(u/s + bias) - bias), s = 1 / (1 + bias).
        let s = 1.0 / (1.0 + bias);
        let u0 = xp / c;
        let t0 = u0 / s + bias;
        let t1 = ch.u1 / s + bias;
        let (f0, big_f0) = curve(t0);
        let (f1, big_f1) = if t1 == ch.t1 { (ch.f1, ch.big_f1) } else { curve(t1) };
        let dt = t0 - t1;
        let mean = if dt.abs() < CLOSE {
            0.5 * (f0 + f1)
        } else {
            ((big_f0 - big_f1) / dt).clamp(f0.min(f1), f0.max(f1))
        };
        let lin = ftz(u0 / 3.0 + ch.u1 - ch.ap1 / 3.0);
        let comp = if t0.abs() <= KNEE || u0 == 0.0 { 0.0 } else { (1.0 - s * (f0 - bias) / u0).abs() };
        ch.hold = ftz(comp.max(rho * ch.hold));
        let c8 = ch.hold.powi(8);
        let w = c8 / (c8 + 0.3f64.powi(8));
        // Interval mean of the curve, plus the all-pass linear part while it is not clipping.
        let core = s * (mean - bias) + (1.0 - w) * (lin - 0.5 * (u0 + ch.u1));
        ch.u1 = u0;
        ch.ap1 = lin;
        ch.t1 = t0;
        ch.f1 = f0;
        ch.big_f1 = big_f0;

        // Remove the DC of the even part only: e(u) = (fa(u) + fa(-u)) / 2 = bias * n(u), with
        // n in [-1, 0]. The tracker averages n and is scaled by the current bias, so a
        // Character move rescales the correction at once instead of waiting for the tracker.
        let n = if bias >= 1e-3 {
            s * (0.5 * (f0 + curve_f(bias - u0 / s)) - bias) / bias
        } else if t0.abs() <= KNEE {
            0.0
        } else {
            // The limit for bias -> 0: f'(u) - 1.
            -((t0.abs() - KNEE) / (1.0 - KNEE)).tanh().powi(2)
        };
        ch.m1 = ftz(ch.m1 + a_dc * (n - ch.m1));
        ch.m2 = ftz(ch.m2 + a_dc * (ch.m1 - ch.m2));
        let v = ((core - bias * ch.m2) * c).clamp(-c, c);

        // De-emphasis D = (b0 + b1 z^-1) / (1 - p z^-1): non-negative impulse response summing to 1.
        let y = ftz(b0 * v + b1 * ch.v1 + pole * ch.y1);
        ch.v1 = v;
        ch.y1 = y;
        y
    }
}

impl Default for ToneClip {
    fn default() -> Self {
        Self::new()
    }
}

impl ClipEngine for ToneClip {
    fn name(&self) -> &'static str {
        "Tone Clip"
    }

    fn prepare(&mut self, sample_rate: f32, _max_block: usize) {
        let fs = sample_rate.max(8_000.0) as f64;
        let k = (std::f64::consts::PI * EMPH_HZ.min(0.4 * fs) / fs).tan();
        self.pole = (1.0 - k) / (1.0 + k);
        self.rho = (-1.0 / (HOLD_MS * 1e-3 * fs)).exp();
        self.a_dc = 1.0 - (-2.0 * std::f64::consts::PI * DC_HZ / fs).exp();
        let a = one_pole(SMOOTH_MS, fs);
        self.p.a = [a, a, one_pole(CHAR_MS, fs)];
        self.reset();
    }

    fn reset(&mut self) {
        self.ch = [ToneChan::default(); 2];
        self.p.cur = self.p.target;
        self.p.fresh = true;
    }

    fn set(&mut self, ceiling: f32, shape: f32, character: f32) {
        self.p.set(ceiling, shape, character);
    }

    fn latency(&self) -> usize {
        0
    }

    fn process(&mut self, left: &mut [f32], right: &mut [f32]) {
        let len = left.len().min(right.len());
        let p = self.pole;
        for i in 0..len {
            if self.p.tick() {
                let g = 10f64.powf(-EMPH_DB * self.p.cur[1] / 20.0);
                let half = 0.5 * (1.0 - g) * (1.0 - p);
                self.b = (g + half, half - g * p);
            }
            let [c, _, character] = self.p.cur;
            let (b0, b1) = self.b;
            let bias = BIAS_MAX * character;
            let cf = c as f32;
            for (ch, x) in self.ch.iter_mut().zip([&mut left[i], &mut right[i]]) {
                *x = (Self::tick(ch, *x as f64, c, b0, b1, p, bias, self.rho, self.a_dc) as f32).clamp(-cf, cf);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clip_api::bench;
    use std::f64::consts::PI;

    fn sine(hz: f64, amp: f64, len: usize, fs: f64) -> Vec<f32> {
        (0..len).map(|t| (amp * (2.0 * PI * hz * t as f64 / fs).sin()) as f32).collect()
    }

    fn noise(len: usize, amp: f32, seed: u64) -> Vec<f32> {
        let mut s = seed | 1;
        (0..len)
            .map(|_| {
                s ^= s << 13;
                s ^= s >> 7;
                s ^= s << 17;
                ((s >> 11) as f64 / (1u64 << 53) as f64 * 2.0 - 1.0) as f32 * amp
            })
            .collect()
    }

    fn make(i: usize) -> Box<dyn ClipEngine> {
        (ENGINES[i].1)()
    }

    fn run(e: &mut dyn ClipEngine, x: &[f32]) -> Vec<f32> {
        bench::run(e, x, &[512])
    }

    fn peak(y: &[f32]) -> f32 {
        y.iter().fold(0.0f32, |m, v| m.max(v.abs()))
    }

    #[test]
    fn labels_fit() {
        for (label, make) in ENGINES {
            assert!(label.len() <= 12);
            assert_eq!(make().name(), *label);
        }
    }

    #[test]
    fn smooth_clip_contract() {
        bench::assert_contract(&ENGINES[0].1);
    }

    #[test]
    fn tone_clip_contract() {
        bench::assert_contract(&ENGINES[1].1);
    }

    #[test]
    fn smooth_clip_latency_budget() {
        let mut e = make(0);
        for (fs, max) in [(44_100.0, 64), (48_000.0, 64), (96_000.0, 128), (192_000.0, 256)] {
            e.prepare(fs, 512);
            assert!(e.latency() <= max, "{fs}: {}", e.latency());
        }
    }

    /// Both engines never put a sample over the ceiling, at every setting corner.
    #[test]
    fn sample_peak_never_exceeds_ceiling() {
        let fs = 48_000.0;
        let mut signals = vec![noise(20_000, 16.0, 3), noise(20_000, 3.0, 5)];
        for hz in [40.0, 1_000.0, 7_000.0, 15_000.0, 21_000.0] {
            signals.push(sine(hz, 8.0, 20_000, fs));
        }
        signals.push((0..20_000).map(|t| if (t / 23) % 2 == 0 { 6.0 } else { -6.0 }).collect());
        for i in 0..ENGINES.len() {
            for (shape, ch) in [(0.0, 0.0), (1.0, 1.0), (0.0, 1.0), (1.0, 0.0), (0.5, 0.5)] {
                for c in [1.0f32, 0.3, 0.02] {
                    for x in &signals {
                        let mut e = make(i);
                        e.prepare(fs as f32, 512);
                        e.set(c, shape, ch);
                        let x: Vec<f32> = x.iter().map(|v| v * c).collect();
                        let p = peak(&run(e.as_mut(), &x));
                        assert!(p <= c, "{} s{shape} c{ch} ceiling {c}: {p}", e.name());
                    }
                }
            }
        }
    }

    /// Below the knee Smooth Clip is a pure delay, bit for bit.
    #[test]
    fn smooth_clip_is_a_delay_below_knee() {
        let mut e = make(0);
        e.prepare(48_000.0, 512);
        e.set(1.0, 1.0, 1.0);
        let x = noise(4_000, 0.2, 9);
        let y = run(e.as_mut(), &x);
        let d = e.latency();
        assert!(y[..d].iter().all(|&v| v == 0.0));
        assert!(y[d..].iter().zip(&x).all(|(a, b)| a == b));
    }

    /// The spread gain makes far fewer harmonics than a static curve and keeps inter-sample
    /// peaks close to the ceiling.
    #[test]
    fn smooth_clip_harmonics_and_true_peak() {
        let mut e = make(0);
        e.prepare(48_000.0, 512);
        e.set(1.0, 0.5, 0.0);
        let h = bench::harmonics_dbc(e.as_mut(), 48_000.0, 1_000.0, 12.0);
        assert!(h.iter().all(|&v| v < -60.0), "{h:?}");
        let (sp, tp) = bench::rail(&ENGINES[0].1, 48_000.0);
        assert!(sp <= 1.0 && tp < 1.2, "{sp} {tp}");
    }

    /// Recovery: Character lengthens the gain recovery after a burst.
    #[test]
    fn smooth_clip_release_follows_character() {
        let fs = 48_000.0;
        let mut x = sine(200.0, 4.0, 4_800, fs);
        x.extend(sine(200.0, 0.5, 9_600, fs));
        let tail_peak = |ch: f32| {
            let mut e = make(0);
            e.prepare(fs as f32, 512);
            e.set(1.0, 0.0, ch);
            let y = run(e.as_mut(), &x);
            // 5..10 ms after the drop reaches the output.
            let at = 4_800 + e.latency() + 240;
            peak(&y[at..at + 240])
        };
        assert!(tail_peak(0.0) > 0.499, "{}", tail_peak(0.0));
        assert!(tail_peak(1.0) < 0.45, "{}", tail_peak(1.0));
    }

    /// The tanh knee is continuous in value and slope, and the antiderivative matches it.
    #[test]
    fn tone_curve_is_smooth_and_integrates() {
        let eps = 1e-9;
        let (a, _) = curve(KNEE - eps);
        let (b, _) = curve(KNEE + eps);
        assert!((a - b).abs() < 1e-8);
        let slope = (curve(KNEE + 2.0 * eps).0 - curve(KNEE + eps).0) / eps;
        assert!((slope - 1.0).abs() < 1e-6, "{slope}");
        for t in [-30.0, -1.2, -0.51, -0.2, 0.3, 0.55, 0.9, 2.0, 40.0] {
            let h = 1e-5;
            let d = (curve(t + h).1 - curve(t - h).1) / (2.0 * h);
            assert!((d - curve(t).0).abs() < 1e-8, "{t}: {d} vs {}", curve(t).0);
            assert!((curve_f(t) - curve(t).0).abs() < 1e-12);
            assert!(curve(t).0.abs() <= 1.0);
        }
    }

    /// The emphasis pair is an exact inverse below the knee, at every Shape.
    #[test]
    fn tone_clip_flat_below_knee() {
        for (shape, ch) in [(0.0, 0.0), (1.0, 1.0), (1.0, 0.0)] {
            let mut e = make(1);
            e.prepare(48_000.0, 512);
            e.set(1.0, shape, ch);
            let f = bench::flatness_db(e.as_mut(), 48_000.0);
            assert!(f < 0.001, "{shape}/{ch}: {f}");
        }
    }

    /// Character adds even harmonics, but the output stays DC-free while it clips.
    #[test]
    fn tone_clip_even_harmonics_without_dc() {
        let fs = 48_000.0;
        let x = sine(93.75, 2.0, 96_000, fs); // 512 samples per period
        let measure = |ch: f32| {
            let mut e = make(1);
            e.prepare(fs as f32, 512);
            e.set(1.0, 0.0, ch);
            let y = run(e.as_mut(), &x);
            let last = &y[y.len() - 512 * 32..];
            let dc = last.iter().map(|&v| v as f64).sum::<f64>() / last.len() as f64;
            let (mut re, mut im) = (0.0, 0.0);
            for (t, &v) in last.iter().enumerate() {
                let ph = 2.0 * PI * 2.0 * t as f64 / 512.0;
                re += v as f64 * ph.cos();
                im += v as f64 * ph.sin();
            }
            (dc, 2.0 * (re * re + im * im).sqrt() / last.len() as f64)
        };
        let (dc0, h2_0) = measure(0.0);
        let (dc1, h2_1) = measure(1.0);
        assert!(dc0.abs() < 1e-6 && h2_0 < 1e-5, "{dc0} {h2_0}");
        assert!(dc1.abs() < 1e-3, "dc {dc1}");
        assert!(h2_1 > 0.02, "H2 amplitude {h2_1}");
    }

    /// Character jumping between 0 and 1 on a clipping bass tone leaves no DC step: the mean
    /// over every period stays near zero through the moves.
    #[test]
    fn tone_clip_no_dc_step_when_character_moves() {
        let x = sine(93.75, 2.0, 512 * 200, 48_000.0);
        let mut e = make(1);
        e.prepare(48_000.0, 512);
        let (mut l, mut r) = (x.clone(), x.clone());
        for (k, (bl, br)) in l.chunks_mut(512).zip(r.chunks_mut(512)).enumerate() {
            e.set(1.0, 0.0, if (k / 25) % 2 == 0 { 0.0 } else { 1.0 });
            e.process(bl, br);
        }
        let worst = l.chunks(512).skip(2).fold(0.0f64, |m, p| m.max((p.iter().map(|&v| v as f64).sum::<f64>() / 512.0).abs()));
        assert!(worst < 6e-3, "period mean {worst}");
    }

    /// Silence stays exactly silent while Shape and Character move.
    #[test]
    fn tone_clip_silence_while_settings_move() {
        let mut e = make(1);
        e.prepare(48_000.0, 512);
        let (mut l, mut r) = (vec![0.0f32; 64], vec![0.0f32; 64]);
        for k in 0..200 {
            let v = (k % 9) as f32 / 8.0;
            e.set(0.1 + 0.9 * v, 1.0 - v, v);
            e.process(&mut l, &mut r);
            assert!(l.iter().chain(&r).all(|&s| s == 0.0));
        }
    }

    /// Moving Shape and Character on a clipping tone does not make a bigger sample-to-sample
    /// step than the tone has at either end of the move.
    #[test]
    fn settings_moves_do_not_click() {
        let fs = 48_000.0;
        let x = sine(220.0, 2.0, 48_000, fs);
        let step = |y: &[f32]| y.windows(2).skip(4_000).fold(0.0f32, |m, w| m.max((w[1] - w[0]).abs()));
        for i in 0..ENGINES.len() {
            let fixed = |s: f32, c: f32| {
                let mut e = make(i);
                e.prepare(fs as f32, 512);
                e.set(1.0, s, c);
                step(&run(e.as_mut(), &x))
            };
            let worst = fixed(0.0, 0.0).max(fixed(1.0, 1.0));
            let mut e = make(i);
            e.prepare(fs as f32, 512);
            let (mut l, mut r) = (x.clone(), x.clone());
            for (k, (bl, br)) in l.chunks_mut(256).zip(r.chunks_mut(256)).enumerate() {
                let v = if (k / 20) % 2 == 0 { 0.0 } else { 1.0 };
                e.set(1.0, v, v);
                e.process(bl, br);
            }
            let moved = step(&l);
            assert!(moved <= 1.1 * worst, "{}: {moved} vs {worst}", e.name());
        }
    }



    /// Power of bin `bin` of the last `n` samples of `y` (direct DFT).
    fn bin_power(y: &[f32], n: usize, bin: usize) -> f64 {
        let y = &y[y.len() - n..];
        let (mut re, mut im) = (0.0, 0.0);
        for (t, &v) in y.iter().enumerate() {
            let ph = 2.0 * PI * (bin * t % n) as f64 / n as f64;
            re += v as f64 * ph.cos();
            im += v as f64 * ph.sin();
        }
        re * re + im * im
    }

    /// Measurements the shared bench does not make: bass harmonics and bass-on-treble
    /// intermodulation, where a gain-based clip and a curve-based clip differ most.
    fn bass_report(make: &dyn Fn() -> Box<dyn ClipEngine>, shape: f32, character: f32) -> String {
        const N: usize = 16_384;
        let mut e = make();
        e.prepare(48_000.0, 512);
        e.set(1.0, shape, character);
        let h = bench::harmonics_dbc(e.as_mut(), 48_000.0, 100.0, 12.0);
        let tone = |bin: usize, amp: f64, t: usize| amp * (2.0 * PI * (bin * t % N) as f64 / N as f64).sin();
        let (lo, hi) = (21, 1025); // 61.5 Hz at +6 dB over the ceiling, 3.0 kHz at -20 dBFS
        let x: Vec<f32> = (0..3 * N).map(|t| (tone(lo, 2.0, t) + tone(hi, 0.1, t)) as f32).collect();
        e.reset();
        let y = run(e.as_mut(), &x);
        let p_hi = bin_power(&y, N, hi);
        let side: f64 = (1..=6).flat_map(|m| [hi - m * lo, hi + m * lo]).map(|b| bin_power(&y, N, b)).sum();
        // The machine is shared with other builds: the best of five runs is the steadier figure.
        let cpu = (0..5).map(|_| bench::cpu_ns(e.as_mut(), 48_000.0)).fold(f64::INFINITY, f64::min);
        format!(
            "CPU, best of 5 runs: {cpu:.1} ns per stereo frame\n100 Hz at +12 dB, H2..H5 dBc: {}\nBass-on-treble IM (61.5 Hz +6 dB over the ceiling, 3 kHz at -20 dBFS): sidebands {:.1} dBc, 3 kHz level {:+.2} dB\n",
            h[..4].iter().map(|v| format!("{v:.1}")).collect::<Vec<_>>().join(" / "),
            10.0 * (side / p_hi).log10(),
            10.0 * (p_hi.sqrt() * 2.0 / N as f64 / 0.1).powi(2).log10()
        )
    }

    /// Plain hard clip, only as the reference for `bass_report`.
    struct Hard(f32);
    impl ClipEngine for Hard {
        fn name(&self) -> &'static str {
            "Hard"
        }
        fn prepare(&mut self, _: f32, _: usize) {}
        fn reset(&mut self) {}
        fn set(&mut self, c: f32, _: f32, _: f32) {
            self.0 = c;
        }
        fn latency(&self) -> usize {
            0
        }
        fn process(&mut self, l: &mut [f32], r: &mut [f32]) {
            for v in l.iter_mut().chain(r.iter_mut()) {
                *v = v.clamp(-self.0, self.0);
            }
        }
    }

    #[test]
    #[ignore]
    fn report() {
        println!("### Hard clip reference (extra rows only)\n\n{}", bass_report(&|| Box::new(Hard(1.0)), 0.0, 0.0));
        // Recommended settings first, then both corners.
        for ((_, make), rec) in ENGINES.iter().zip([(0.5, 0.3), (0.3, 0.3)]) {
            for (s, c) in [rec, (0.0, 0.0), (1.0, 1.0)] {
                println!("{}{}", bench::report(make, s, c), bass_report(make, s, c));
            }
        }
    }
}
