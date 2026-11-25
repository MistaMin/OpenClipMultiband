//! Static soft-knee clippers made alias-poor at 1x with first-order antiderivative
//! anti-aliasing (ADAA). See REPORT.md for the maths and the measurements.
//!
//! The curve, in units where the ceiling is 1 (`u = x / ceiling`, output `ceiling * f(u)`):
//! * `f(u) = u` for `|u| <= t` (a wire below the knee);
//! * above the knee (`w = |u| - t`, `k = 1 - t`) a blend, set by Character, of
//!   - the exponential knee `t + k (1 - exp(-w / k))`, which approaches the rail asymptotically,
//!   - the parabolic knee `t + w - w^2 / (4k)`, which reaches the rail at `w = 2k` and stays there.
//!
//! Both halves are odd, continuous in value and slope at the knee, monotone, and bounded by the
//! rail, so every blend is too. Their antiderivatives are closed-form, so the blend's is as well.

use clip_api::ClipEngine;

/// Shape 0..1 maps to the knee threshold `t = T_MIN + T_SPAN * shape` (0.10..0.95 of the ceiling).
const T_MIN: f64 = 0.10;
const T_SPAN: f64 = 0.85;
/// Settings glide with this one-pole time constant, so a moving control never steps the curve.
const SMOOTH_MS: f64 = 10.0;
/// Below this interval width (in ceiling units) the box mean is the curve at the midpoint: its
/// error there is under `f'' d^2 / 24 < 1e-11`, while the quotient would lose precision.
const CLOSE: f64 = 1e-6;
/// Regulariser of the slope fits, in squared ceiling units.
const EPS: f64 = 1e-24;
/// All-pass coefficient of the residual methods' linear path.
const AP: f64 = 1.0 / 3.0;
/// States below this are flushed to zero so long silences never reach subnormals.
const TINY: f64 = 1e-250;

/// The 1x ADAA method an engine runs. Only the winner is exported; the rest stay for the bench.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(not(test), allow(dead_code))]
enum Method {
    /// The static curve, no anti-aliasing.
    Naive,
    /// Residual ADAA1 as it ships elsewhere: instantaneous compression blend.
    Residual,
    /// Residual ADAA1 with the compression `|c|` peak-held (instant attack, `hold_ms` release).
    PeakHeld,
    /// Residual ADAA1 with the adaptive moving-slope gain on the linear correction.
    Adaptive,
    /// Two-segment Hann-B1 box kernel with a 4 ms fitted-slope correction (1 sample latency).
    HannB1,
    /// Hann-B1 box kernel whose top-end correction fades out with the peak-held compression.
    B1Held,
    /// As B1Held, with the fitted slope on the correction.
    B1HeldFit,
}

/// Curve value, antiderivative and slope at one point.
#[derive(Clone, Copy)]
struct Point {
    u: f64,
    f: f64,
    big_f: f64,
    df: f64,
}

#[derive(Clone, Copy)]
struct Curve {
    t: f64,
    /// 0 = exponential knee, 1 = parabolic knee.
    mix: f64,
}

impl Curve {
    fn at(self, u: f64) -> Point {
        let (t, k, m) = (self.t, 1.0 - self.t, self.mix);
        let v = u.abs();
        if v <= t {
            return Point { u, f: u, big_f: 0.5 * u * u, df: 1.0 };
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
        let f = fe + m * (fp - fe);
        Point { u, f: f.copysign(u), big_f: 0.5 * t * t + fi_e + m * (fi_p - fi_e), df: de + m * (dp - de) }
    }

    /// Mean of the curve over the straight line between two samples (first-order ADAA).
    fn mean(self, a: &Point, b: &Point) -> f64 {
        if a.u.abs() <= self.t && b.u.abs() <= self.t {
            return 0.5 * (a.u + b.u);
        }
        let d = a.u - b.u;
        if d.abs() < CLOSE {
            return self.at(0.5 * (a.u + b.u)).f;
        }
        (a.big_f - b.big_f) / d
    }
}

/// A trailing moving sum, re-added from its buffer at every wrap so rounding left by loud
/// values can never outlive them.
struct MovSum {
    buf: Vec<f64>,
    pos: usize,
    sum: f64,
}

impl MovSum {
    fn new(len: usize) -> Self {
        MovSum { buf: vec![0.0; len.max(1)], pos: 0, sum: 0.0 }
    }
    fn clear(&mut self) {
        self.buf.iter_mut().for_each(|v| *v = 0.0);
        self.pos = 0;
        self.sum = 0.0;
    }
    fn push(&mut self, v: f64) -> f64 {
        let Some(old) = self.buf.get_mut(self.pos) else { return self.sum };
        self.sum += v - *old;
        *old = v;
        self.pos += 1;
        if self.pos >= self.buf.len() {
            self.pos = 0;
            self.sum = self.buf.iter().sum();
        }
        self.sum
    }
}

/// Two cascaded moving sums: triangular weights over `2 len - 1` samples.
struct TriWindow(MovSum, MovSum);

impl TriWindow {
    fn push(&mut self, v: f64) -> f64 {
        let s = self.0.push(v);
        self.1.push(s)
    }
}

/// One channel's history.
struct Chan {
    x1: f64,
    x2: f64,
    ap_x1: f64,
    ap_y1: f64,
    seg1: f64,
    hold: f64,
    /// Adaptive method's smoothed statistics: dx^2, dx df, d2^2, f' d2^2.
    s: [f64; 4],
    p: TriWindow,
    c: TriWindow,
}

impl Chan {
    fn new(len: usize) -> Self {
        Chan {
            x1: 0.0,
            x2: 0.0,
            ap_x1: 0.0,
            ap_y1: 0.0,
            seg1: 0.0,
            hold: 0.0,
            s: [0.0; 4],
            p: TriWindow(MovSum::new(len), MovSum::new(len)),
            c: TriWindow(MovSum::new(len), MovSum::new(len)),
        }
    }

    fn clear(&mut self) {
        (self.x1, self.x2, self.ap_x1, self.ap_y1, self.seg1, self.hold, self.s) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, [0.0; 4]);
        for w in [&mut self.p, &mut self.c] {
            w.0.clear();
            w.1.clear();
        }
    }
}

fn flush(v: f64) -> f64 {
    if v.abs() < TINY {
        0.0
    } else {
        v
    }
}

/// Weight of the box path in the residual methods, from the compression `c`: one half at 30 %.
fn blend(c: f64) -> f64 {
    let c8 = (c * c) * (c * c);
    let c8 = c8 * c8;
    c8 / (c8 + 6.561e-5)
}

/// A soft-knee clipper with one of the 1x ADAA methods.
pub struct SoftClip {
    name: &'static str,
    method: Method,
    /// Targets and smoothed values of ceiling, threshold, knee mix.
    target: [f64; 3],
    cur: [f64; 3],
    glide: f64,
    hold_rho: f64,
    hold_ms: f64,
    stat_alpha: f64,
    /// Hold the output to the ceiling after the method.
    safe: bool,
    fresh: bool,
    ch: [Chan; 2],
}

impl SoftClip {
    /// The shipping engine: peak-held residual ADAA1 (1 ms release), output held to the ceiling.
    pub fn new() -> Self {
        let mut e = Self::with("Exp Knee", Method::PeakHeld);
        (e.safe, e.hold_ms) = (true, 1.0);
        e.prepare(48_000.0, 512);
        e
    }

    fn with(name: &'static str, method: Method) -> Self {
        let mut e = SoftClip {
            name,
            method,
            target: [1.0, T_MIN + T_SPAN * 0.8, 0.0],
            cur: [1.0, T_MIN + T_SPAN * 0.8, 0.0],
            glide: 0.0,
            hold_rho: 0.0,
            hold_ms: 0.5,
            stat_alpha: 0.0,
            safe: false,
            fresh: true,
            ch: [Chan::new(1), Chan::new(1)],
        };
        e.prepare(48_000.0, 512);
        e
    }

    fn tick(&mut self, ci: usize, x: f64) -> f64 {
        let x = if x.is_finite() { x } else { 0.0 };
        let [ceil, t, mix] = self.cur;
        let curve = Curve { t, mix };
        let (method, hold_rho, alpha, safe) = (self.method, self.hold_rho, self.stat_alpha, self.safe);
        let Some(s) = self.ch.get_mut(ci) else { return 0.0 };
        let inv = 1.0 / ceil;
        let (p0, p1) = (curve.at(x * inv), curve.at(s.x1 * inv));
        let u2 = s.x2 * inv;
        let mean = curve.mean(&p0, &p1);
        // The residual methods' linear path: a first-order all-pass, run in input units.
        let ap = AP * x + s.ap_x1 - AP * s.ap_y1;
        (s.ap_x1, s.ap_y1) = (x, flush(ap));
        let corr = ap * inv - 0.5 * (p0.u + p1.u);
        let comp = if p0.u != 0.0 { 1.0 - p0.f / p0.u } else { 0.0 };
        s.hold = flush(comp.abs().max(hold_rho * s.hold));
        let d2 = p0.u - 2.0 * p1.u + u2;
        let y = match method {
            Method::Naive => p0.f,
            Method::Residual => mean + (1.0 - blend(comp)) * corr,
            Method::PeakHeld => mean + (1.0 - blend(s.hold)) * corr,
            Method::Adaptive => {
                let dx = p0.u - p1.u;
                let v = [dx * dx, dx * (p0.f - p1.f), d2 * d2, p1.df * d2 * d2];
                for (st, v) in s.s.iter_mut().zip(v) {
                    *st = flush(*st + alpha * (v - *st));
                }
                let gs = ((s.s[1] + EPS) / (s.s[0] + EPS)).clamp(0.0, 1.0);
                let m = ((s.s[3] + EPS) / (s.s[2] + EPS)).clamp(0.0, 1.0);
                let sigma = s.s[2] / (s.s[0] + EPS);
                let rho = 2.0 / 3.0 * (1.0 - 0.25 * sigma).max(0.0).sqrt();
                mean + (gs - rho * (gs - m)) * corr
            }
            Method::HannB1 | Method::B1Held | Method::B1HeldFit => {
                let dx = p0.u - p1.u;
                let pw = s.p.push(dx * dx);
                let cw = s.c.push(dx * (p0.f - p1.f));
                let k = ((cw + EPS) / (pw + EPS)).clamp(0.0, 1.0);
                let g = match method {
                    Method::HannB1 => k,
                    Method::B1Held => 1.0 - blend(s.hold),
                    _ => k * (1.0 - blend(s.hold)),
                };
                // seg1 is kept in output units: it was the previous sample's box mean.
                let y = 0.5 * (mean + s.seg1 * inv) - 0.25 * g * d2;
                s.seg1 = mean * ceil;
                y
            }
        };
        let y = if safe { y.clamp(-1.0, 1.0) } else { y };
        (s.x2, s.x1) = (s.x1, x);
        let y = y * ceil;
        // Keeps the f32 output out of subnormals while a tail dies away.
        if y.abs() < 1e-30 {
            0.0
        } else {
            y
        }
    }
}

impl Default for SoftClip {
    fn default() -> Self {
        Self::new()
    }
}

impl ClipEngine for SoftClip {
    fn name(&self) -> &'static str {
        self.name
    }

    fn prepare(&mut self, sample_rate: f32, _max_block: usize) {
        let fs = if sample_rate.is_finite() && sample_rate > 1_000.0 { sample_rate as f64 } else { 48_000.0 };
        self.glide = 1.0 - (-1.0 / (fs * SMOOTH_MS * 1e-3)).exp();
        self.hold_rho = (-1.0 / (fs * self.hold_ms * 1e-3)).exp();
        self.stat_alpha = 1.0 - (-1.0 / (fs * 2e-3)).exp();
        let len = (fs * 2e-3).round() as usize;
        self.ch = [Chan::new(len), Chan::new(len)];
        self.reset();
    }

    fn reset(&mut self) {
        self.ch.iter_mut().for_each(Chan::clear);
        self.cur = self.target;
        self.fresh = true;
    }

    fn set(&mut self, ceiling: f32, shape: f32, character: f32) {
        let unit = |v: f32, d: f64| if v.is_finite() { (v as f64).clamp(0.0, 1.0) } else { d };
        self.target = [
            if ceiling.is_finite() { (ceiling as f64).clamp(0.01, 1.0) } else { self.target[0] },
            T_MIN + T_SPAN * unit(shape, (self.target[1] - T_MIN) / T_SPAN),
            unit(character, self.target[2]),
        ];
        if self.fresh {
            self.cur = self.target;
        }
    }

    fn latency(&self) -> usize {
        usize::from(matches!(self.method, Method::HannB1 | Method::B1Held | Method::B1HeldFit))
    }

    fn process(&mut self, left: &mut [f32], right: &mut [f32]) {
        let n = left.len().min(right.len());
        if n > 0 {
            self.fresh = false;
        }
        for (l, r) in left.iter_mut().zip(right.iter_mut()) {
            for (c, t) in self.cur.iter_mut().zip(self.target) {
                *c += self.glide * (t - *c);
                if (t - *c).abs() < 1e-9 {
                    *c = t;
                }
            }
            *l = self.tick(0, *l as f64) as f32;
            *r = self.tick(1, *r as f64) as f32;
        }
    }
}

/// The engines the plugin offers (the signature the brief fixes).
#[allow(clippy::type_complexity)]
pub const ENGINES: &[(&str, fn() -> Box<dyn ClipEngine>)] = &[("Exp Knee", || Box::new(SoftClip::new()))];

#[cfg(test)]
mod tests {
    use super::*;
    use clip_api::bench;

    /// Every method, for the comparison.
    const METHODS: [(&str, Method); 7] = [
        ("Naive", Method::Naive),
        ("Residual", Method::Residual),
        ("PeakHeld", Method::PeakHeld),
        ("Adaptive", Method::Adaptive),
        ("HannB1", Method::HannB1),
        ("B1Held", Method::B1Held),
        ("B1HeldFit", Method::B1HeldFit),
    ];

    fn engine(m: Method) -> Box<dyn ClipEngine> {
        Box::new(SoftClip::with("test", m))
    }

    #[test]
    fn exported_engines_pass_contract() {
        for (label, make) in ENGINES {
            assert!(label.len() <= 12);
            assert_eq!(make().name(), *label);
            bench::assert_contract(make);
        }
    }

    #[test]
    fn curve_is_odd_bounded_monotone_and_smooth_at_knees() {
        for t in [0.1, 0.3, 0.78125, 0.95] {
            for mix in [0.0, 0.4, 1.0] {
                let c = Curve { t, mix };
                let k = 1.0 - t;
                let mut prev = c.at(0.0);
                for i in 1..40_000 {
                    let u = i as f64 * 1e-4;
                    let p = c.at(u);
                    assert_eq!(c.at(-u).f, -p.f);
                    assert_eq!(c.at(-u).big_f, p.big_f);
                    assert!(p.f <= 1.0 && p.f >= prev.f, "t {t} mix {mix} u {u}");
                    prev = p;
                }
                // Value and slope continuous at the knee and (parabolic part) at the rail.
                for knee in [t, t + 2.0 * k] {
                    let (a, b) = (c.at(knee - 1e-12), c.at(knee + 1e-12));
                    assert!((a.f - b.f).abs() < 1e-9 && (a.df - b.df).abs() < 1e-9 && (a.big_f - b.big_f).abs() < 1e-9);
                }
            }
        }
    }

    #[test]
    fn antiderivative_matches_curve() {
        for t in [0.1, 0.5, 0.78125, 127.0 / 128.0] {
            for mix in [0.0, 0.5, 1.0] {
                let c = Curve { t, mix };
                let mut worst: f64 = 0.0;
                for i in -3000..3000 {
                    let u = i as f64 * 1e-3 + 3e-7;
                    let h = 1e-6;
                    let num = (c.at(u + h).big_f - c.at(u - h).big_f) / (2.0 * h);
                    let slope = (c.at(u + h).f - c.at(u - h).f) / (2.0 * h);
                    worst = worst.max((num - c.at(u).f).abs());
                    assert!((slope - c.at(u).df).abs() < 1e-4 * (1.0 + 1.0 / (1.0 - t)), "f' at t {t} u {u}");
                }
                assert!(worst < 1e-8, "t {t} mix {mix}: {worst}");
            }
        }
    }

    #[test]
    fn close_samples_are_continuous() {
        let c = Curve { t: 0.6, mix: 0.3 };
        for a in [0.59999, 0.6, 0.7, 1.1, 1.4, 1.5, 3.0, -0.65] {
            let pa = c.at(a);
            let (mut last, mut d_last) = (c.mean(&pa, &c.at(a + 2.0 * CLOSE)), 2.0 * CLOSE);
            for i in 1..400 {
                let d = 2.0 * CLOSE * (0.99f64).powi(i);
                let m = c.mean(&pa, &c.at(a + d));
                // The mean may move by at most half the interval change (slope <= 1), nothing more.
                assert!((m - last).abs() <= 0.5 * (d_last - d) + 1e-10, "a {a} d {d}: {m} vs {last}");
                (last, d_last) = (m, d);
            }
            assert!((last - c.at(a).f).abs() < 1e-6);
        }
    }

    /// Under the knee the Hann-B1 methods are a one-sample delay, the naive curve a wire, and
    /// the residual methods exactly their unity-gain all-pass.
    #[test]
    fn wire_below_knee() {
        let x: Vec<f32> = (0..4_000).map(|n| 0.5 * (n as f32 * 0.37).sin() + 0.2 * (n as f32 * 0.011).sin()).collect();
        let mut ap = vec![0.0f64; x.len()];
        for n in 0..x.len() {
            let (x1, y1) = if n > 0 { (x[n - 1] as f64, ap[n - 1]) } else { (0.0, 0.0) };
            ap[n] = AP * x[n] as f64 + x1 - AP * y1;
        }
        for (_, m) in METHODS {
            let mut e = engine(m);
            e.set(1.0, 0.8, 0.0);
            let y = bench::run(e.as_mut(), &x, &[64]);
            let want: Vec<f64> = match (e.latency(), m) {
                (1, _) => std::iter::once(0.0).chain(x.iter().map(|v| *v as f64)).collect(),
                (_, Method::Naive) => x.iter().map(|v| *v as f64).collect(),
                _ => ap.clone(),
            };
            let err = y.iter().zip(&want).fold(0.0f64, |w, (a, b)| w.max((*a as f64 - b).abs()));
            assert!(err < 1e-6, "{m:?}: {err}");
        }
    }

    /// The shipping engine never puts a sample over the ceiling, at any drive or setting.
    #[test]
    fn shipping_engine_is_ceiling_safe() {
        let noise: Vec<f32> = (0..20_000u32).map(|n| (n.wrapping_mul(2_654_435_761) >> 8) as f32 / 8_388_608.0 - 1.0).collect();
        for ceiling in [1.0f32, 0.5, 0.05] {
            for (s, c) in [(0.0, 0.0), (0.8, 0.0), (1.0, 1.0), (0.3, 0.7)] {
                for drive in [1.0f32, 2.0, 4.0, 16.0] {
                    let mut e = SoftClip::new();
                    e.set(ceiling, s, c);
                    let x: Vec<f32> = noise.iter().map(|v| v * drive * ceiling * 2.0).collect();
                    let peak = bench::run(&mut e, &x, &[256]).iter().fold(0.0f32, |m, v| m.max(v.abs()));
                    assert!(peak <= ceiling, "ceiling {ceiling} shape {s} character {c} drive {drive}: {peak}");
                }
            }
        }
    }

    #[test]
    fn odd_symmetry_means_no_dc() {
        for (_, m) in METHODS {
            let mut e = engine(m);
            e.set(0.7, 0.4, 0.6);
            let x: Vec<f32> = (0..6_000).map(|n| 3.0 * (n as f32 * 0.05).sin() + 0.4).collect();
            let neg: Vec<f32> = x.iter().map(|v| -v).collect();
            let a = bench::run(e.as_mut(), &x, &[512]);
            e.reset();
            let b = bench::run(e.as_mut(), &neg, &[512]);
            assert!(a.iter().zip(&b).all(|(p, q)| p == &-q), "{m:?}");
        }
    }

    /// Sweeping every setting while a loud tone plays: the largest sample-to-sample step stays
    /// within the steps of the tone run at fixed settings at either end of the sweep.
    #[test]
    fn moving_settings_do_not_click() {
        for (_, m) in METHODS {
            let x: Vec<f32> = (0..48_000).map(|n| 2.0 * (n as f32 * 2.0 * std::f32::consts::PI * 200.0 / 48_000.0).sin()).collect();
            let step = |y: &[f32]| y.windows(2).skip(8).fold(0.0f32, |w, p| w.max((p[1] - p[0]).abs()));
            let mut fixed: f32 = 0.0;
            for (c, s, ch) in [(1.0, 0.0, 0.0), (0.25, 1.0, 1.0), (1.0, 1.0, 0.0), (0.25, 0.0, 1.0)] {
                let mut e = engine(m);
                e.set(c, s, ch);
                fixed = fixed.max(step(&bench::run(e.as_mut(), &x, &[64])));
            }
            let mut e = engine(m);
            e.set(1.0, 0.0, 0.0);
            let (mut l, mut r) = (x.clone(), x.clone());
            for (i, (bl, br)) in l.chunks_mut(64).zip(r.chunks_mut(64)).enumerate() {
                // Jump every control to its far end every 100 blocks.
                let up = (i / 100) % 2 == 1;
                e.set(if up { 0.25 } else { 1.0 }, up as u8 as f32, (i / 50 % 2) as f32);
                e.process(bl, br);
            }
            assert!(step(&l) <= 1.05 * fixed, "{m:?}: {} vs {fixed}", step(&l));
        }
    }

    #[test]
    fn long_silence_after_loud_has_no_subnormals() {
        for (_, m) in METHODS {
            let mut e = SoftClip::with("s", m);
            let mut x: Vec<f32> = (0..2_000).map(|n| 4.0 * (n as f32 * 0.3).sin()).collect();
            x.extend(std::iter::repeat_n(0.0, 200_000));
            let y = bench::run(&mut e, &x, &[512]);
            assert!(y.iter().all(|v| *v == 0.0 || v.is_normal()), "{m:?}");
            let st: Vec<f64> = e.ch.iter().flat_map(|s| [s.ap_y1, s.hold, s.s[0], s.s[1], s.s[2], s.s[3], s.p.1.sum, s.c.1.sum]).collect();
            assert!(st.iter().all(|v| *v == 0.0 || v.is_normal()), "{m:?}: {st:?}");
        }
    }

    /// The comparison rows: label, method, output held to the ceiling, hold time (ms).
    const CFGS: [(&str, Method, bool, f64); 15] = [
        ("Naive", Method::Naive, false, 0.5),
        ("Residual (shipping)", Method::Residual, false, 0.5),
        ("Peak-held 0.5 ms", Method::PeakHeld, false, 0.5),
        ("Peak-held 2 ms", Method::PeakHeld, false, 2.0),
        ("Peak-held 0.5 ms + rail", Method::PeakHeld, true, 0.5),
        ("Peak-held 1 ms + rail", Method::PeakHeld, true, 1.0),
        ("Peak-held 2 ms + rail", Method::PeakHeld, true, 2.0),
        ("Adaptive moving-slope", Method::Adaptive, false, 0.5),
        ("Fitted-slope Hann-B1", Method::HannB1, false, 0.5),
        ("Fitted-slope Hann-B1 + rail", Method::HannB1, true, 0.5),
        ("B1 held 0.5 ms", Method::B1Held, false, 0.5),
        ("B1 held 2 ms", Method::B1Held, false, 2.0),
        ("B1 held 2 ms + rail", Method::B1Held, true, 2.0),
        ("B1 held-fit 0.5 ms", Method::B1HeldFit, false, 0.5),
        ("B1 held-fit 2 ms", Method::B1HeldFit, false, 2.0),
    ];

    fn cfg(m: Method, safe: bool, hold_ms: f64) -> Box<dyn ClipEngine> {
        let mut e = SoftClip::with("cfg", m);
        (e.safe, e.hold_ms) = (safe, hold_ms);
        e.prepare(48_000.0, 512);
        Box::new(e)
    }

    /// Fundamental level (dB) of a -6 dBFS tone plus drive, from a single-bin DFT of the steady output.
    fn h1_db(e: &mut dyn ClipEngine, hz: f64, drive_db: f64) -> f64 {
        use std::f64::consts::PI;
        let n = 16_384;
        let bin = ((hz * n as f64 / 48_000.0).round() as usize) | 1;
        let amp = 0.5 * 10f64.powf(drive_db / 20.0);
        let x: Vec<f32> = (0..2 * n).map(|t| (amp * (2.0 * PI * (bin * t) as f64 / n as f64).sin()) as f32).collect();
        e.reset();
        let y = bench::run(e, &x, &[512]);
        let (mut re, mut im) = (0.0, 0.0);
        for (t, v) in y[n..].iter().enumerate() {
            let ph = 2.0 * PI * (bin * t) as f64 / n as f64;
            (re, im) = (re + *v as f64 * ph.cos(), im + *v as f64 * ph.sin());
        }
        20.0 * ((re * re + im * im).sqrt() * 2.0 / n as f64).log10()
    }

    /// Worst sample peak / ceiling over 1k, 5k, 10k sines, noise and a step train at one drive,
    /// ceiling 1 (the bench's rail check runs only at +24 dB).
    fn peak_at(make: &dyn Fn() -> Box<dyn ClipEngine>, shape: f32, character: f32, drive_db: f64) -> f64 {
        use std::f64::consts::PI;
        let g = 0.5 * 10f64.powf(drive_db / 20.0);
        let mut sigs: Vec<Vec<f32>> =
            [1_000.0, 5_000.0, 10_000.0].iter().map(|hz| (0..24_000).map(|t| (g * (2.0 * PI * hz * t as f64 / 48_000.0).sin()) as f32).collect()).collect();
        let mut s = 7u64;
        sigs.push(
            (0..24_000)
                .map(|_| {
                    s ^= s << 13;
                    s ^= s >> 7;
                    s ^= s << 17;
                    (((s >> 11) as f64 / (1u64 << 53) as f64 - 0.5) * 3.46 * g) as f32
                })
                .collect(),
        );
        sigs.push((0..24_000).map(|t| if (t / 37) % 2 == 0 { g as f32 } else { -g as f32 }).collect());
        sigs.iter()
            .map(|x| {
                let mut e = make();
                e.set(1.0, shape, character);
                bench::run(e.as_mut(), x, &[512]).iter().fold(0.0f64, |m, v| m.max(v.abs() as f64))
            })
            .fold(0.0, f64::max)
    }

    /// Prints the method comparison and the bench for the shipping engines at their recommended
    /// settings and at the ends of both controls.
    #[test]
    #[ignore]
    fn report() {
        let tones = [1_000.0, 2_500.0, 5_000.0, 8_000.0, 12_000.0];
        for (s, c) in [(0.8, 0.0), (0.8, 1.0), (0.0, 0.0)] {
            println!("## Method comparison, shape {s}, character {c}, 48 kHz, ceiling 1\n");
            println!("| Method | Lat | Worst alias 0 / +6 / +12 / +24 dB | +24 dB alias at 1k / 2.5k / 5k / 8k / 12k | H1 vs naive, 8k / 12k at +12 dB | Flat dB | Bench sample / true peak (+24 dB) | Sample peak +6 / +12 dB | ns |");
            println!("|---|---:|---|---|---|---:|---|---|---:|");
            for (label, m, safe, hold) in CFGS {
                let make = move || cfg(m, safe, hold);
                let mut e = make();
                e.set(1.0, s, c);
                let worst: Vec<String> = bench::DRIVES_DB
                    .iter()
                    .map(|&d| format!("{:.1}", tones.iter().map(|&hz| bench::alias_dbc(e.as_mut(), 48_000.0, hz, d)).fold(f64::MIN, f64::max)))
                    .collect();
                let per: Vec<String> = tones.iter().map(|&hz| format!("{:.1}", bench::alias_dbc(e.as_mut(), 48_000.0, hz, 24.0))).collect();
                let mut naive = cfg(Method::Naive, false, 0.5);
                naive.set(1.0, s, c);
                let h1: Vec<String> =
                    [8_000.0, 12_000.0].iter().map(|&hz| format!("{:+.2}", h1_db(e.as_mut(), hz, 12.0) - h1_db(naive.as_mut(), hz, 12.0))).collect();
                let flat = bench::flatness_db(e.as_mut(), 48_000.0);
                let (sp, tp) = bench::rail(&make, 48_000.0);
                let ns = bench::cpu_ns(e.as_mut(), 48_000.0);
                let (p6, p12) = (peak_at(&make, s, c, 6.0), peak_at(&make, s, c, 12.0));
                println!(
                    "| {label} | {} | {} | {} | {} | {flat:.4} | {sp:.3}x / {tp:.3}x | {p6:.3}x / {p12:.3}x | {ns:.0} |",
                    e.latency(),
                    worst.join(" / "),
                    per.join(" / "),
                    h1.join(" / ")
                );
            }
            println!();
        }
        println!("## Shipping engines\n");
        for (_, make) in ENGINES {
            for (s, c) in [(0.8, 0.0), (0.0, 0.0), (1.0, 1.0), (0.0, 1.0), (1.0, 0.0)] {
                println!("{}", bench::report(make, s, c));
            }
        }
    }
}
