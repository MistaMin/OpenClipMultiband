// Copyright (c) 2026 Marcos Quirino Deida
//! The clipping algorithms as `Shaper`s: a static curve (with antiderivative and slope, so every
//! algorithm runs through the shared ADAA) between the stages that make each one what it is.
//! Ported from the design crates (`clip-soft`, `clip-hard`, `clip-tone`, `clip-analog`); their
//! REPORT.md files hold the maths and measurements.

use clip_api::shaper::{ftz, Glide, Pt, Shaper};

/// Scales a curve given in ceiling units (`u = x / c`) back to signal units.
fn scaled(c: f64, p: Pt) -> Pt {
    Pt { f: c * p.f, big_f: c * c * p.big_f, df: p.df }
}

// ---------------------------------------------------------------------- Honey (exponential knee)

/// Exponential knee from the DSP wiki, blended toward a parabolic knee by Character.
/// Shape sets the knee start `t = 0.10 + 0.85 shape` (0.8 gives the wiki's 0.78).
pub struct ExpKnee {
    g: Glide,
}

impl Default for ExpKnee {
    fn default() -> Self {
        Self { g: Glide::new([1.0, 0.8, 0.0]) }
    }
}

pub fn exp_knee(u: f64, t: f64, m: f64) -> Pt {
    let k = 1.0 - t;
    let v = u.abs();
    if v <= t {
        return Pt::linear(u);
    }
    let w = v - t;
    // exp(-w/k) - 1, clamped so the exponential never goes subnormal.
    let em = (-(w / k).min(700.0)).exp_m1();
    let (fe, fi_e, de) = (t - k * em, w + k * k * em, 1.0 + em);
    let (fp, fi_p, dp) = if w < 2.0 * k {
        (t + w - w * w / (4.0 * k), t * w + 0.5 * w * w - w * w * w / (12.0 * k), 1.0 - w / (2.0 * k))
    } else {
        (1.0, 2.0 * k * t + 4.0 / 3.0 * k * k + w - 2.0 * k, 0.0)
    };
    Pt { f: (fe + m * (fp - fe)).copysign(u), big_f: 0.5 * t * t + fi_e + m * (fi_p - fi_e), df: de + m * (dp - de) }
}

impl Shaper for ExpKnee {
    fn name(&self) -> &'static str {
        "Honey"
    }
    fn recommended(&self) -> [f64; 2] {
        [0.8, 0.0]
    }
    fn set_rate(&mut self, rate: f64) {
        self.g.set_rate(rate, [10.0; 3]);
    }
    fn set(&mut self, c: f64, s: f64, ch: f64) {
        self.g.set(c, s, ch);
    }
    fn ceiling(&self) -> f64 {
        self.g.cur[0]
    }
    fn pre(&mut self, x: [f64; 2]) -> [f64; 2] {
        self.g.tick();
        x
    }
    fn curve(&self, x: f64) -> Pt {
        let [c, shape, m] = self.g.cur;
        scaled(c, exp_knee(x / c, 0.10 + 0.85 * shape, m))
    }
}

// ------------------------------------------------------------------------------ Flint (hard clip)

/// Hard clip at the ceiling with a quadratic knee of half-width `0.2 shape` (in ceiling units).
pub struct Hard {
    g: Glide,
}

impl Default for Hard {
    fn default() -> Self {
        Self { g: Glide::new([1.0, 0.0, 0.0]) }
    }
}

pub fn hard_knee(u: f64, w: f64) -> Pt {
    let v = u.abs();
    let a = 1.0 - w;
    if v <= a {
        return Pt::linear(u);
    }
    if w > 1e-9 && v < 1.0 + w {
        let e = v - a;
        return Pt { f: (v - e * e / (4.0 * w)).copysign(u), big_f: 0.5 * v * v - e * e * e / (12.0 * w), df: 1.0 - e / (2.0 * w) };
    }
    let top = 0.5 * (1.0 + w) * (1.0 + w) - 2.0 * w * w / 3.0;
    Pt { f: 1f64.copysign(u), big_f: top + v - (1.0 + w), df: 0.0 }
}

impl Shaper for Hard {
    fn name(&self) -> &'static str {
        "Flint"
    }
    fn recommended(&self) -> [f64; 2] {
        [0.0, 0.0]
    }
    fn set_rate(&mut self, rate: f64) {
        self.g.set_rate(rate, [5.0; 3]);
    }
    fn set(&mut self, c: f64, s: f64, ch: f64) {
        self.g.set(c, s, ch);
    }
    fn ceiling(&self) -> f64 {
        self.g.cur[0]
    }
    fn pre(&mut self, x: [f64; 2]) -> [f64; 2] {
        self.g.tick();
        x
    }
    fn curve(&self, x: f64) -> Pt {
        let [c, shape, _] = self.g.cur;
        scaled(c, hard_knee(x / c, 0.2 * shape))
    }
}

// ---------------------------------------------------------------- Ghost (look-ahead smooth clip)

const HILBERT: usize = 15;
const BOX_48K: usize = 24;
const RELEASE_MS: f64 = 30.0;
const KNEE_DB: f64 = 6.0;
const ONE: f64 = (1u64 << 30) as f64;

/// Look-ahead clip: each clip's gain reduction is spread over about 1 ms (Hilbert envelope,
/// running minimum, recovery, two exact boxes). The curve after it is a hard clip that only
/// catches rounding. Shape: knee from hard (0) to 6 dB under the ceiling. Character: up to 30 ms
/// of extra recovery.
pub struct SmoothClip {
    g: Glide,
    rate: f64,
    k: usize,
    mask: usize,
    x: [Vec<f64>; 2],
    n: usize,
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
}

impl Default for SmoothClip {
    fn default() -> Self {
        let mut hil = [0.0; HILBERT.div_ceil(2)];
        for (i, h) in hil.iter_mut().enumerate() {
            let k = (2 * i + 1) as f64;
            let ph = std::f64::consts::PI * k / (HILBERT + 1) as f64;
            *h = 2.0 / (std::f64::consts::PI * k) * (0.42 + 0.5 * ph.cos() + 0.08 * (2.0 * ph).cos());
        }
        let mut s = Self {
            g: Glide::new([1.0, 0.5, 0.3]),
            rate: 48_000.0,
            k: BOX_48K,
            mask: 0,
            x: [Vec::new(), Vec::new()],
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
        };
        s.alloc(48_000.0);
        s.set_rate(48_000.0);
        s
    }
}

impl SmoothClip {
    fn box_len(rate: f64) -> usize {
        ((BOX_48K as f64 * rate / 48_000.0).round() as usize).max(HILBERT)
    }
}

impl Shaper for SmoothClip {
    fn name(&self) -> &'static str {
        "Ghost"
    }
    fn recommended(&self) -> [f64; 2] {
        [0.5, 0.3]
    }
    fn alloc(&mut self, max_rate: f64) {
        let k = Self::box_len(max_rate);
        let size = (HILBERT + 2 * k).next_power_of_two();
        self.x = [vec![0.0; size], vec![0.0; size]];
        self.dq_t = vec![0; size];
        self.dq_g = vec![1.0; size];
        self.box1 = vec![0; k];
        self.box2 = vec![0; k];
    }
    fn set_rate(&mut self, rate: f64) {
        self.rate = rate;
        self.k = Self::box_len(rate).min(self.box1.len());
        self.mask = (HILBERT + 2 * self.k).next_power_of_two().min(self.x[0].len()) - 1;
        self.g.set_rate(rate, [5.0; 3]);
        for x in &mut self.x {
            x.fill(0.0);
        }
        (self.n, self.head, self.tail, self.rel, self.bi) = (0, 0, 0, 1.0, 0);
        let q = ONE as u64;
        let k = self.k as u64;
        self.box1[..self.k].fill(q);
        self.box2[..self.k].fill(q * k);
        self.sum1 = q * k;
        self.sum2 = self.sum1 * k;
    }
    fn set(&mut self, c: f64, s: f64, ch: f64) {
        self.g.set(c, s, ch);
    }
    fn ceiling(&self) -> f64 {
        self.g.cur[0]
    }
    fn latency(&self) -> usize {
        HILBERT + 2 * self.k - 2
    }
    fn pre(&mut self, x: [f64; 2]) -> [f64; 2] {
        self.g.tick();
        let [c, shape, character] = self.g.cur;
        let t = c * 10f64.powf(-KNEE_DB * shape / 20.0);
        let tau = RELEASE_MS * 1e-3 * character * self.rate;
        let alpha = if tau > 1e-3 { (-1.0 / tau).exp() } else { 0.0 };
        let (k, m, w) = (self.k, self.mask, 2 * self.k - 1);
        let n = self.n;
        self.x[0][n & m] = x[0];
        self.x[1][n & m] = x[1];

        // Analytic envelope of x[n - H], the louder channel.
        let mut env2: f64 = 0.0;
        for xs in &self.x {
            let mid = xs[n.wrapping_sub(HILBERT) & m];
            let mut h = 0.0;
            for (j, cj) in self.hil.iter().enumerate() {
                let o = 2 * j + 1;
                h += cj * (xs[n.wrapping_sub(HILBERT + o) & m] - xs[n.wrapping_sub(HILBERT - o) & m]);
            }
            env2 = env2.max(mid * mid + h * h);
        }
        let g = if env2 > 1e-30 {
            let e = env2.sqrt();
            if e <= t {
                1.0
            } else {
                let span = c - t;
                (if span < 1e-9 { c } else { t + span * ((e - t) / span).tanh() }) / e
            }
        } else {
            1.0
        };

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
        self.rel = if gmin < self.rel { gmin } else { self.rel + (1.0 - alpha) * (gmin - self.rel) };

        // Two exact boxes of k: a triangle that never exceeds the minimum it smooths.
        let q = (self.rel * ONE) as u64;
        self.sum1 = self.sum1 + q - self.box1[self.bi];
        self.box1[self.bi] = q;
        self.sum2 = self.sum2 + self.sum1 - self.box2[self.bi];
        self.box2[self.bi] = self.sum1;
        self.bi = if self.bi + 1 == k { 0 } else { self.bi + 1 };
        let s = self.sum2 as f64 / (ONE * (k * k) as f64);
        let d = self.latency();
        self.n = n + 1;
        [self.x[0][n.wrapping_sub(d) & m] * s, self.x[1][n.wrapping_sub(d) & m] * s]
    }
    fn curve(&self, x: f64) -> Pt {
        let c = self.g.cur[0];
        scaled(c, hard_knee(x / c, 0.0))
    }
}

// ------------------------------------------------------------------ Ember (tone-shaped clip)

const TONE_KNEE: f64 = 0.5;
const BIAS_MAX: f64 = 0.25;
const EMPH_DB: f64 = 12.0;
const EMPH_HZ: f64 = 3_000.0;
const DC_HZ: f64 = 5.0;

/// Tanh knee (rail 1, linear to 0.5): value, antiderivative, slope.
fn tanh_knee(t: f64) -> Pt {
    let a = t.abs();
    if a <= TONE_KNEE {
        return Pt::linear(t);
    }
    let r = 1.0 - TONE_KNEE;
    let z = (a - TONE_KNEE) / r;
    let e = (-2.0 * z).exp();
    let th = (1.0 - e) / (1.0 + e);
    Pt {
        f: (TONE_KNEE + r * th).copysign(t),
        big_f: 0.5 * TONE_KNEE * TONE_KNEE + TONE_KNEE * (a - TONE_KNEE) + r * r * (z + e.ln_1p() - std::f64::consts::LN_2),
        df: 1.0 - th * th,
    }
}

/// Tanh knee inside a matched pre-/de-emphasis pair (Shape: up to 12 dB more drive on highs, so
/// they clip first and the distortion comes out darker), with a bias for even harmonics
/// (Character) and a 5 Hz DC blocker after the curve.
pub struct ToneClip {
    g: Glide,
    rate: f64,
    pole: f64,
    b: (f64, f64),
    dc: f64,
    /// Per channel: emphasis in/out, DC blocker in/out, de-emphasis in/out.
    st: [[f64; 6]; 2],
}

impl Default for ToneClip {
    fn default() -> Self {
        let mut s = Self { g: Glide::new([1.0, 0.3, 0.3]), rate: 48_000.0, pole: 0.0, b: (1.0, 0.0), dc: 0.0, st: [[0.0; 6]; 2] };
        s.set_rate(48_000.0);
        s
    }
}

impl Shaper for ToneClip {
    fn name(&self) -> &'static str {
        "Ember"
    }
    fn recommended(&self) -> [f64; 2] {
        [0.3, 0.3]
    }
    fn set_rate(&mut self, rate: f64) {
        self.rate = rate;
        let k = (std::f64::consts::PI * EMPH_HZ.min(0.4 * rate) / rate).tan();
        self.pole = (1.0 - k) / (1.0 + k);
        self.dc = (-2.0 * std::f64::consts::PI * DC_HZ / rate).exp();
        self.g.set_rate(rate, [5.0, 5.0, 100.0]);
        self.st = [[0.0; 6]; 2];
        self.update();
    }
    fn set(&mut self, c: f64, s: f64, ch: f64) {
        self.g.set(c, s, ch);
        self.update();
    }
    fn ceiling(&self) -> f64 {
        self.g.cur[0]
    }
    fn pre(&mut self, x: [f64; 2]) -> [f64; 2] {
        if self.g.tick() {
            self.update();
        }
        let (b0, b1) = self.b;
        let mut out = [0.0; 2];
        for (c, (st, x)) in self.st.iter_mut().zip(x).enumerate() {
            // P = (1 - p z^-1) / (b0 + b1 z^-1)
            let xp = ftz((x - self.pole * st[0] - b1 * st[1]) / b0);
            (st[0], st[1]) = (x, xp);
            out[c] = xp;
        }
        out
    }
    fn curve(&self, x: f64) -> Pt {
        let [c, _, character] = self.g.cur;
        let b = BIAS_MAX * character;
        let s = 1.0 / (1.0 + b);
        let u = x / c;
        let p = tanh_knee(u / s + b);
        scaled(c, Pt { f: s * (p.f - b), big_f: s * s * p.big_f - s * b * u, df: p.df })
    }
    fn post(&mut self, y: [f64; 2]) -> [f64; 2] {
        let c = self.g.cur[0];
        let (b0, b1) = self.b;
        let mut out = [0.0; 2];
        for (ch, (st, y)) in self.st.iter_mut().zip(y).enumerate() {
            // DC blocker, then the ceiling, then D = (b0 + b1 z^-1) / (1 - p z^-1) (non-negative
            // impulse response summing to 1, so it cannot raise a peak), then the ceiling again.
            let v = ftz(y - st[2] + self.dc * st[3]);
            (st[2], st[3]) = (y, v);
            let v = v.clamp(-c, c);
            let d = ftz(b0 * v + b1 * st[4] + self.pole * st[5]);
            (st[4], st[5]) = (v, d);
            out[ch] = d.clamp(-c, c);
        }
        out
    }
}

impl ToneClip {
    fn update(&mut self) {
        let g = 10f64.powf(-EMPH_DB * self.g.cur[1] / 20.0);
        let half = 0.5 * (1.0 - g) * (1.0 - self.pole);
        self.b = (g + half, half - g * self.pole);
    }
}

// --------------------------------------------------------- Magnet and Volt (soft saturators)

/// `s v / sqrt(s^2 + v^2)`: bounded at `s`, slope 1 at 0.
fn soft_core(u: f64, s: f64) -> Pt {
    let r = (s * s + u * u).sqrt();
    Pt { f: s * u / r, big_f: s * r - s * s, df: s * s * s / (r * r * r) }
}

/// Soft saturator driven harder above 1.8 kHz (matched emphasis pair). Shape lowers the
/// saturation level (1.0 to 0.25 of the ceiling); Character raises the high-frequency drive (up
/// to +9.5 dB).
pub struct Ferro {
    g: Glide,
    lp: f64,
    /// Per channel: low-passed input, low-passed output.
    st: [[f64; 2]; 2],
}

impl Default for Ferro {
    fn default() -> Self {
        let mut s = Self { g: Glide::new([1.0, 0.2, 0.1]), lp: 0.0, st: [[0.0; 2]; 2] };
        s.set_rate(48_000.0);
        s
    }
}

impl Shaper for Ferro {
    fn name(&self) -> &'static str {
        "Magnet"
    }
    fn recommended(&self) -> [f64; 2] {
        [0.2, 0.1]
    }
    fn set_rate(&mut self, rate: f64) {
        self.lp = 1.0 - (-2.0 * std::f64::consts::PI * 1_800.0 / rate).exp();
        self.g.set_rate(rate, [5.0; 3]);
        self.st = [[0.0; 2]; 2];
    }
    fn set(&mut self, c: f64, s: f64, ch: f64) {
        self.g.set(c, s, ch);
    }
    fn ceiling(&self) -> f64 {
        self.g.cur[0]
    }
    fn pre(&mut self, x: [f64; 2]) -> [f64; 2] {
        self.g.tick();
        let k = 2.0 * self.g.cur[2];
        let mut out = [0.0; 2];
        for (c, (st, x)) in self.st.iter_mut().zip(x).enumerate() {
            out[c] = (1.0 + k) * x - k * st[0];
            st[0] = ftz(st[0] + self.lp * (x - st[0]));
        }
        out
    }
    fn curve(&self, x: f64) -> Pt {
        let [c, shape, _] = self.g.cur;
        scaled(c, soft_core(x / c, 1.0 - 0.75 * shape))
    }
    fn post(&mut self, y: [f64; 2]) -> [f64; 2] {
        let [c, _, character] = self.g.cur;
        let k = 2.0 * character;
        let mut out = [0.0; 2];
        for (ch, (st, y)) in self.st.iter_mut().zip(y).enumerate() {
            let o = (y + k * st[1]) / (1.0 + k);
            st[1] = ftz(st[1] + self.lp * (o - st[1]));
            out[ch] = o.clamp(-c, c);
        }
        out
    }
}

/// Soft saturator with overload recovery and an output slew limit. Shape lowers the saturation
/// level (1.0 to 0.35 of the ceiling); Character speeds recovery (30 to 2 ms) and raises the slew
/// limit (8 to 80 kHz).
pub struct Rail {
    g: Glide,
    rate: f64,
    /// Per channel: overload envelope, previous output.
    st: [[f64; 2]; 2],
}

impl Default for Rail {
    fn default() -> Self {
        let mut s = Self { g: Glide::new([1.0, 0.65, 0.6]), rate: 48_000.0, st: [[0.0; 2]; 2] };
        s.set_rate(48_000.0);
        s
    }
}

impl Shaper for Rail {
    fn name(&self) -> &'static str {
        "Volt"
    }
    fn recommended(&self) -> [f64; 2] {
        [0.65, 0.6]
    }
    fn set_rate(&mut self, rate: f64) {
        self.rate = rate;
        self.g.set_rate(rate, [5.0; 3]);
        self.st = [[0.0; 2]; 2];
    }
    fn set(&mut self, c: f64, s: f64, ch: f64) {
        self.g.set(c, s, ch);
    }
    fn ceiling(&self) -> f64 {
        self.g.cur[0]
    }
    fn pre(&mut self, x: [f64; 2]) -> [f64; 2] {
        self.g.tick();
        let [c, _, character] = self.g.cur;
        let release = (-1.0 / (self.rate * (0.03 - 0.028 * character))).exp();
        let mut out = [0.0; 2];
        for (ch, (st, x)) in self.st.iter_mut().zip(x).enumerate() {
            let excess = (x.abs() / c - 1.0).clamp(0.0, 10_000.0);
            st[0] = ftz(excess.max(st[0] * release));
            out[ch] = x / (1.0 + 0.18 * st[0]);
        }
        out
    }
    fn curve(&self, x: f64) -> Pt {
        let [c, shape, _] = self.g.cur;
        scaled(c, soft_core(x / c, 1.0 - 0.65 * shape))
    }
    fn post(&mut self, y: [f64; 2]) -> [f64; 2] {
        let [c, _, character] = self.g.cur;
        let slew = 2.0 * std::f64::consts::PI * (8_000.0 + 72_000.0 * character) / self.rate * c;
        let mut out = [0.0; 2];
        for (ch, (st, y)) in self.st.iter_mut().zip(y).enumerate() {
            st[1] = ftz((st[1] + (y - st[1]).clamp(-slew, slew)).clamp(-c, c));
            out[ch] = st[1];
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn curves() -> Vec<Box<dyn Shaper>> {
        crate::ALGOS.iter().map(|a| (a.1)()).collect()
    }

    #[test]
    fn every_curve_integrates_to_its_antiderivative_and_slope() {
        for mut s in curves() {
            for (shape, character) in [(0.0, 0.0), (0.5, 0.5), (1.0, 1.0), (0.8, 0.0)] {
                s.set_rate(48_000.0);
                s.set(0.7, shape, character);
                let _ = s.pre([0.0, 0.0]);
                let h = 1e-5;
                let mut u = -3.0;
                while u < 3.0 {
                    let (a, b) = (s.curve(u - h), s.curve(u + h));
                    let p = s.curve(u);
                    let num_f = (b.big_f - a.big_f) / (2.0 * h);
                    assert!((num_f - p.f).abs() < 2e-5, "{} {shape} {character} at {u}: F' {num_f} f {}", s.name(), p.f);
                    let num_d = (b.f - a.f) / (2.0 * h);
                    assert!((num_d - p.df).abs() < 2e-3, "{} {shape} {character} at {u}: f' {num_d} df {}", s.name(), p.df);
                    assert!(p.f.abs() <= 0.7 + 1e-9, "{} exceeds the ceiling at {u}: {}", s.name(), p.f);
                    u += 0.0137;
                }
            }
        }
    }

    #[test]
    fn every_curve_is_a_wire_near_zero() {
        for mut s in curves() {
            s.set_rate(48_000.0);
            s.set(1.0, 0.5, 0.5);
            let _ = s.pre([0.0, 0.0]);
            let p = s.curve(1e-4);
            assert!((p.f - 1e-4).abs() < 1e-9 && (p.df - 1.0).abs() < 1e-6, "{}: {p:?}", s.name());
        }
    }
}
