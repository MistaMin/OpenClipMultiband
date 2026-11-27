//! One clipping algorithm as a static curve between optional stages, and `Shaped`, the engine
//! that runs any of them through the chosen anti-aliasing: fitted-slope Hann-B1 ADAA, adaptive
//! moving-slope ADAA (both from the DSP wiki's ADAA topic) or none, at 1x to 64x oversampling.
//!
//! The ADAA always works on the curve; the oversampler wraps the whole algorithm (stages and
//! curve), so a stage such as a slew limiter or a look-ahead gain also runs at the high rate.

use crate::ClipEngine;
use std::f64::consts::PI;

/// Curve value, antiderivative and slope at one input.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Pt {
    pub f: f64,
    pub big_f: f64,
    pub df: f64,
}

impl Pt {
    pub fn linear(x: f64) -> Self {
        Pt { f: x, big_f: 0.5 * x * x, df: 1.0 }
    }
}

/// One clipping algorithm. Inputs and outputs are in signal units (the ceiling is a setting).
pub trait Shaper: Send {
    fn name(&self) -> &'static str;
    /// Allocates for the highest processing rate it will run at.
    fn alloc(&mut self, _max_rate: f64) {}
    /// Sets the processing rate (host rate times oversampling) and clears state. Never allocates.
    fn set_rate(&mut self, rate: f64);
    /// Targets: ceiling (linear), shape and character (0..=1). Gliding happens in `pre`.
    fn set(&mut self, ceiling: f64, shape: f64, character: f64);
    /// The current (glided) ceiling, which a drawn curve scales with.
    fn ceiling(&self) -> f64;
    /// Shape and character its stages run at while a drawn curve replaces its curve.
    fn recommended(&self) -> [f64; 2] {
        [0.5, 0.5]
    }
    /// Processing-rate samples of delay added by `pre` and `post`.
    fn latency(&self) -> usize {
        0
    }
    /// Stage before the curve, both channels at once; called once per processing-rate frame.
    fn pre(&mut self, x: [f64; 2]) -> [f64; 2] {
        x
    }
    /// The static curve with the current settings. Odd or not, monotone, slope within 0..=1.
    fn curve(&self, x: f64) -> Pt;
    /// Stage after the curve.
    fn post(&mut self, y: [f64; 2]) -> [f64; 2] {
        y
    }
}

impl Shaper for Box<dyn Shaper> {
    fn name(&self) -> &'static str {
        (**self).name()
    }
    fn alloc(&mut self, max_rate: f64) {
        (**self).alloc(max_rate)
    }
    fn set_rate(&mut self, rate: f64) {
        (**self).set_rate(rate)
    }
    fn set(&mut self, ceiling: f64, shape: f64, character: f64) {
        (**self).set(ceiling, shape, character)
    }
    fn ceiling(&self) -> f64 {
        (**self).ceiling()
    }
    fn recommended(&self) -> [f64; 2] {
        (**self).recommended()
    }
    fn latency(&self) -> usize {
        (**self).latency()
    }
    fn pre(&mut self, x: [f64; 2]) -> [f64; 2] {
        (**self).pre(x)
    }
    fn curve(&self, x: f64) -> Pt {
        (**self).curve(x)
    }
    fn post(&mut self, y: [f64; 2]) -> [f64; 2] {
        (**self).post(y)
    }
}

/// Points of a drawn curve: output (0..=1 of the ceiling) at inputs `2 k / CURVE_POINTS` of the
/// ceiling, k = 1..=CURVE_POINTS; the curve is a straight line between them, odd, flat after.
pub const CURVE_POINTS: usize = 16;

/// A drawn curve with its antiderivative at every node, so ADAA stays exact.
#[derive(Clone, Copy)]
pub struct Drawn {
    v: [f64; CURVE_POINTS + 1],
    big: [f64; CURVE_POINTS + 1],
}

impl Drawn {
    const STEP: f64 = 2.0 / CURVE_POINTS as f64;

    pub fn new(points: &[f32]) -> Self {
        let mut v = [0.0; CURVE_POINTS + 1];
        for (d, p) in v[1..].iter_mut().zip(points) {
            *d = if p.is_finite() { (*p as f64).clamp(0.0, 1.0) } else { 0.0 };
        }
        let mut big = [0.0; CURVE_POINTS + 1];
        for k in 1..=CURVE_POINTS {
            big[k] = big[k - 1] + 0.5 * Self::STEP * (v[k - 1] + v[k]);
        }
        Self { v, big }
    }

    /// The curve in ceiling units.
    pub fn at(&self, u: f64) -> Pt {
        let a = u.abs();
        let top = 2.0;
        if a >= top {
            let last = self.v[CURVE_POINTS];
            return Pt { f: last.copysign(u), big_f: self.big[CURVE_POINTS] + last * (a - top), df: 0.0 };
        }
        let k = ((a / Self::STEP) as usize).min(CURVE_POINTS - 1);
        let t = a - k as f64 * Self::STEP;
        let m = (self.v[k + 1] - self.v[k]) / Self::STEP;
        Pt { f: (self.v[k] + m * t).copysign(u), big_f: self.big[k] + self.v[k] * t + 0.5 * m * t * t, df: m }
    }
}

/// Per-sample glide of `[ceiling, shape, character]`; the first `set` after a rate change jumps.
pub struct Glide {
    pub cur: [f64; 3],
    target: [f64; 3],
    a: [f64; 3],
    fresh: bool,
}

impl Glide {
    pub fn new(start: [f64; 3]) -> Self {
        Self { cur: start, target: start, a: [1.0; 3], fresh: true }
    }
    /// Glide times in ms for the three values.
    pub fn set_rate(&mut self, rate: f64, ms: [f64; 3]) {
        for (a, ms) in self.a.iter_mut().zip(ms) {
            *a = 1.0 - (-1.0 / (ms * 1e-3 * rate)).exp();
        }
        self.cur = self.target;
        self.fresh = true;
    }
    pub fn set(&mut self, ceiling: f64, shape: f64, character: f64) {
        let unit = |v: f64| if v.is_finite() { v.clamp(0.0, 1.0) } else { 0.0 };
        self.target = [if ceiling.is_finite() { ceiling.clamp(0.01, 1.0) } else { 1.0 }, unit(shape), unit(character)];
        if self.fresh {
            self.cur = self.target;
            self.fresh = false;
        }
    }
    /// Advances one sample; true while anything moved.
    pub fn tick(&mut self) -> bool {
        let mut moved = false;
        for ((c, t), a) in self.cur.iter_mut().zip(self.target).zip(self.a) {
            if *c != t {
                *c += a * (t - *c);
                if (t - *c).abs() < 1e-9 {
                    *c = t;
                }
                moved = true;
            }
        }
        moved
    }
}

pub fn ftz(v: f64) -> f64 {
    if v.abs() < 1e-30 {
        0.0
    } else {
        v
    }
}

/// Anti-aliasing method applied to every algorithm's curve.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aa {
    HannB1,
    Adaptive,
    None,
}

pub const AA_LABELS: &[&str] = &["Hann-B1 ADAA", "Adaptive ADAA", "No ADAA"];
pub const OS_LABELS: &[&str] = &["1x", "4x", "8x", "16x", "32x", "64x"];
/// Half-band stages for each `OS_LABELS` entry.
const OS_STAGES: [usize; 6] = [0, 2, 3, 4, 5, 6];
const MAX_STAGES: usize = 6;
const MAX_FACTOR: usize = 1 << MAX_STAGES;

impl Aa {
    pub fn from_index(i: usize) -> Self {
        match i {
            0 => Aa::HannB1,
            1 => Aa::Adaptive,
            _ => Aa::None,
        }
    }
}

const EPS: f64 = 1e-24;
/// Under this interval width the box mean is the curve at the midpoint.
const CLOSE: f64 = 1e-7;
/// Coefficient of the residual method's first-order all-pass `(1/3 + z^-1) / (1 + z^-1 / 3)`.
const AP: f64 = 1.0 / 3.0;

/// A trailing moving sum with a fixed capacity and an active length; re-added from its buffer at
/// every wrap so rounding left by loud values never outlives them.
struct MovSum {
    buf: Vec<f64>,
    len: usize,
    pos: usize,
    sum: f64,
}

impl MovSum {
    fn new(cap: usize) -> Self {
        Self { buf: vec![0.0; cap.max(1)], len: 1, pos: 0, sum: 0.0 }
    }
    fn restart(&mut self, len: usize) {
        self.len = len.clamp(1, self.buf.len());
        self.buf[..self.len].fill(0.0);
        self.pos = 0;
        self.sum = 0.0;
    }
    fn push(&mut self, v: f64) -> f64 {
        self.sum += v - self.buf[self.pos];
        self.buf[self.pos] = v;
        self.pos += 1;
        if self.pos >= self.len {
            self.pos = 0;
            self.sum = self.buf[..self.len].iter().sum();
        }
        self.sum
    }
}

/// Two cascaded moving sums: triangular weights over `2 len - 1` samples.
struct Tri(MovSum, MovSum);

impl Tri {
    fn new(cap: usize) -> Self {
        Self(MovSum::new(cap), MovSum::new(cap))
    }
    fn restart(&mut self, len: usize) {
        self.0.restart(len);
        self.1.restart(len);
    }
    fn push(&mut self, v: f64) -> f64 {
        let s = self.0.push(v);
        self.1.push(s)
    }
}

/// One channel of ADAA state.
struct AaChan {
    x1: f64,
    x2: f64,
    p1: Pt,
    seg1: f64,
    ap_x1: f64,
    ap_y1: f64,
    /// Adaptive method's smoothed statistics: dx^2, dx df, d2^2, f' d2^2.
    s: [f64; 4],
    p: Tri,
    c: Tri,
}

impl AaChan {
    fn new(cap: usize) -> Self {
        Self { x1: 0.0, x2: 0.0, p1: Pt::default(), seg1: 0.0, ap_x1: 0.0, ap_y1: 0.0, s: [0.0; 4], p: Tri::new(cap), c: Tri::new(cap) }
    }
    fn restart(&mut self, len: usize) {
        (self.x1, self.x2, self.p1, self.seg1, self.ap_x1, self.ap_y1, self.s) = (0.0, 0.0, Pt::default(), 0.0, 0.0, 0.0, [0.0; 4]);
        self.p.restart(len);
        self.c.restart(len);
    }

    /// What the method does to a straight-line curve (the dry twin): Hann-B1 is a one-sample
    /// delay, the adaptive method its all-pass, no ADAA a wire. Same result, none of the fitting.
    fn linear(&mut self, aa: Aa, x0: f64) -> f64 {
        let x0 = if x0.is_finite() { x0 } else { 0.0 };
        let y = match aa {
            Aa::None => x0,
            Aa::HannB1 => self.x1,
            Aa::Adaptive => {
                let ap = AP * x0 + self.ap_x1 - AP * self.ap_y1;
                (self.ap_x1, self.ap_y1) = (x0, ftz(ap));
                ap
            }
        };
        (self.x2, self.x1) = (self.x1, x0);
        ftz(y)
    }

    /// One processing-rate sample through the method, with `curve` the current curve.
    fn tick(&mut self, aa: Aa, x0: f64, curve: &dyn Fn(f64) -> Pt, alpha: f64) -> f64 {
        let x0 = if x0.is_finite() { x0 } else { 0.0 };
        let p0 = curve(x0);
        // The previous point is re-evaluated with the current settings, so a moving setting never
        // subtracts an antiderivative from another curve.
        let (x1, x2) = (self.x1, self.x2);
        let p1 = if aa == Aa::None { self.p1 } else { curve(x1) };
        let d = x0 - x1;
        let mean = || if d.abs() < CLOSE { curve(0.5 * (x0 + x1)).f } else { (p0.big_f - p1.big_f) / d };
        let y = match aa {
            Aa::None => p0.f,
            Aa::Adaptive => {
                let m = mean();
                let ap = AP * x0 + self.ap_x1 - AP * self.ap_y1;
                (self.ap_x1, self.ap_y1) = (x0, ftz(ap));
                let d2 = x0 - 2.0 * x1 + x2;
                let v = [d * d, d * (p0.f - p1.f), d2 * d2, p1.df * d2 * d2];
                for (st, v) in self.s.iter_mut().zip(v) {
                    *st = ftz(*st + alpha * (v - *st));
                }
                let gs = ((self.s[1] + EPS) / (self.s[0] + EPS)).clamp(0.0, 1.0);
                let mm = ((self.s[3] + EPS) / (self.s[2] + EPS)).clamp(0.0, 1.0);
                let sigma = self.s[2] / (self.s[0] + EPS);
                let rho = 2.0 / 3.0 * (1.0 - 0.25 * sigma).max(0.0).sqrt();
                m + (gs - rho * (gs - mm)) * (ap - 0.5 * (x0 + x1))
            }
            Aa::HannB1 => {
                let m = mean();
                let pw = self.p.push(d * d);
                let cw = self.c.push(d * (p0.f - p1.f));
                let k = ((cw + EPS) / (pw + EPS)).clamp(0.0, 1.0);
                let y = 0.5 * (m + self.seg1) - 0.25 * k * (x0 - 2.0 * x1 + x2);
                self.seg1 = m;
                y
            }
        };
        (self.x2, self.x1, self.p1) = (self.x1, x0, p0);
        ftz(y)
    }
}

// ------------------------------------------------------------------------------- oversampling

fn bessel_i0(x: f64) -> f64 {
    let (mut sum, mut term) = (1.0, 1.0);
    for k in 1..100 {
        term *= (x / (2.0 * k as f64)).powi(2);
        sum += term;
        if term < 1e-17 * sum {
            break;
        }
    }
    sum
}

/// Kaiser half-band lowpass between `low_rate` and twice it: flat to 20 kHz, stopped from
/// `low_rate - 20 kHz` (where images and aliases would land in 0..20 kHz), 80 dB where the length
/// allows. The centre delay is capped at `max_m` high-rate samples. Returns the long polyphase
/// branch h[0], h[2], ..., h[4K+2], summing to 1/2.
fn halfband(low_rate: f64, max_m: usize) -> Vec<f32> {
    const ATTEN: f64 = 80.0;
    let delta = ((low_rate - 40_000.0) / (2.0 * low_rate)).max(0.02);
    let need = (((ATTEN - 7.95) / (14.36 * delta) - 2.0) / 4.0).ceil().max(0.0) as usize;
    let k = need.min((max_m - 1) / 2);
    let (n, m) = (4 * k + 3, 2 * k + 1);
    let atten = (14.36 * delta * (n - 1) as f64 + 7.95).min(ATTEN);
    let beta = 0.1102 * (atten - 8.7);
    let mut g: Vec<f64> = (0..=m)
        .map(|i| {
            let off = (2 * i) as f64 - m as f64;
            let sinc = (PI * off / 2.0).sin() / (PI * off / 2.0);
            0.5 * sinc * bessel_i0(beta * (1.0 - (off / m as f64).powi(2)).max(0.0).sqrt()) / bessel_i0(beta)
        })
        .collect();
    let s: f64 = g.iter().sum();
    g.iter_mut().for_each(|v| *v *= 0.5 / s);
    g.into_iter().map(|v| v as f32).collect()
}

/// Polyphase half-band interpolator by 2. Delay `branch.len() - 1` high-rate samples.
struct Up {
    g: Vec<f32>,
    buf: Vec<f32>,
    pos: usize,
}

impl Up {
    fn new(branch: &[f32]) -> Self {
        let l = branch.len();
        Self { g: branch.iter().map(|v| 2.0 * v).collect(), buf: vec![0.0; 2 * l], pos: 0 }
    }
    fn clear(&mut self) {
        self.buf.fill(0.0);
    }
    #[inline]
    fn tick(&mut self, x: f32) -> (f32, f32) {
        let l = self.g.len();
        self.pos = if self.pos == 0 { l - 1 } else { self.pos - 1 };
        self.buf[self.pos] = x;
        self.buf[self.pos + l] = x;
        let w = &self.buf[self.pos..self.pos + l];
        (self.g.iter().zip(w).map(|(a, b)| a * b).sum(), w[l / 2 - 1])
    }
}

/// Polyphase half-band decimator by 2. Delay `branch.len() - 1` high-rate samples.
struct Down {
    g: Vec<f32>,
    even: Vec<f32>,
    odd: Vec<f32>,
    pos: usize,
}

impl Down {
    fn new(branch: &[f32]) -> Self {
        let l = branch.len();
        Self { g: branch.to_vec(), even: vec![0.0; 2 * l], odd: vec![0.0; 2 * l], pos: 0 }
    }
    fn clear(&mut self) {
        self.even.fill(0.0);
        self.odd.fill(0.0);
    }
    #[inline]
    fn tick(&mut self, e: f32, o: f32) -> f32 {
        let l = self.g.len();
        self.pos = if self.pos == 0 { l - 1 } else { self.pos - 1 };
        self.even[self.pos] = e;
        self.even[self.pos + l] = e;
        self.odd[self.pos] = o;
        self.odd[self.pos + l] = o;
        self.g.iter().zip(&self.even[self.pos..self.pos + l]).map(|(a, b)| a * b).sum::<f32>() + 0.5 * self.odd[self.pos + l / 2]
    }
}

/// Any `Shaper` with the chosen anti-aliasing and oversampling, as a `ClipEngine`.
pub struct Shaped<S: Shaper> {
    s: S,
    aa: Aa,
    stages: usize,
    /// The dry path's twin: same stages and filters, straight-line curve.
    twin: bool,
    /// A drawn curve in place of the shaper's own.
    drawn: Option<Drawn>,
    fs: f64,
    up: [Vec<Up>; 2],
    down: [Vec<Down>; 2],
    /// Delay of each stage's up (and of its down) filter, in that stage's samples.
    stage_delay: Vec<usize>,
    ach: [AaChan; 2],
    /// Delay at the top rate that makes the total latency whole host samples.
    pad: [[f64; MAX_FACTOR]; 2],
    pad_len: usize,
    pad_pos: usize,
    alpha: f64,
    lat: usize,
}

impl<S: Shaper> Shaped<S> {
    pub fn new(s: S) -> Self {
        let mut e = Self {
            s,
            aa: Aa::HannB1,
            stages: 0,
            twin: false,
            drawn: None,
            fs: 48_000.0,
            up: [Vec::new(), Vec::new()],
            down: [Vec::new(), Vec::new()],
            stage_delay: Vec::new(),
            ach: [AaChan::new(1), AaChan::new(1)],
            pad: [[0.0; MAX_FACTOR]; 2],
            pad_len: 0,
            pad_pos: 0,
            alpha: 0.0,
            lat: 0,
        };
        e.prepare(48_000.0, 512);
        e
    }

    fn rate(&self) -> f64 {
        self.fs * (1usize << self.stages) as f64
    }

    /// Applies the current method and factor: rates, windows, latency. Clears state.
    fn configure(&mut self) {
        let rate = self.rate();
        self.s.set_rate(rate);
        self.alpha = 1.0 - (-1.0 / (rate * 2e-3)).exp();
        let win = (rate * 2e-3).round() as usize;
        for c in &mut self.ach {
            c.restart(win);
        }
        for c in 0..2 {
            self.up[c].iter_mut().for_each(Up::clear);
            self.down[c].iter_mut().for_each(Down::clear);
        }
        self.pad = [[0.0; MAX_FACTOR]; 2];
        self.pad_pos = 0;
        // Total delay in top-rate samples: each stage's up and down filters, the method, the stages.
        let top = 1usize << self.stages;
        let filters: usize = (1..=self.stages).map(|s| 2 * self.stage_delay[s - 1] * (top >> s)).sum();
        let t = filters + usize::from(self.aa == Aa::HannB1) + self.s.latency();
        self.pad_len = (top - t % top) % top;
        self.lat = (t + self.pad_len) / top;
    }

    fn frame(&mut self, x: [f64; 2]) -> [f64; 2] {
        let mut x = x;
        if self.pad_len > 0 {
            let i = (self.pad_pos + MAX_FACTOR - self.pad_len) % MAX_FACTOR;
            for (c, v) in x.iter_mut().enumerate() {
                self.pad[c][self.pad_pos] = *v;
                *v = self.pad[c][i];
            }
            self.pad_pos = (self.pad_pos + 1) % MAX_FACTOR;
        }
        let u = self.s.pre(x);
        let (aa, alpha, drawn) = (self.aa, self.alpha, self.drawn);
        if self.twin {
            let y = [self.ach[0].linear(aa, u[0]), self.ach[1].linear(aa, u[1])];
            return self.s.post(y);
        }
        let s = &self.s;
        let curve = |v: f64| match &drawn {
            Some(d) => {
                let c = s.ceiling();
                let p = d.at(v / c);
                Pt { f: c * p.f, big_f: c * c * p.big_f, df: p.df }
            }
            None => s.curve(v),
        };
        let y = [self.ach[0].tick(aa, u[0], &curve, alpha), self.ach[1].tick(aa, u[1], &curve, alpha)];
        self.s.post(y)
    }
}

impl<S: Shaper> ClipEngine for Shaped<S> {
    fn name(&self) -> &'static str {
        self.s.name()
    }

    fn prepare(&mut self, sample_rate: f32, _max_block: usize) {
        self.fs = if sample_rate.is_finite() && sample_rate >= 8_000.0 { sample_rate as f64 } else { 48_000.0 };
        let branches: Vec<Vec<f32>> = (1..=MAX_STAGES).map(|s| halfband(self.fs * (1u64 << (s - 1)) as f64, if s == 1 { 47 } else { 31 })).collect();
        self.stage_delay = branches.iter().map(|b| b.len() - 1).collect();
        self.up = std::array::from_fn(|_| branches.iter().map(|b| Up::new(b)).collect());
        self.down = std::array::from_fn(|_| branches.iter().map(|b| Down::new(b)).collect());
        let max_rate = self.fs * MAX_FACTOR as f64;
        let cap = (max_rate * 2e-3).round() as usize + 1;
        self.ach = [AaChan::new(cap), AaChan::new(cap)];
        self.s.alloc(max_rate);
        self.configure();
    }

    fn reset(&mut self) {
        self.configure();
    }

    fn set(&mut self, ceiling: f32, shape: f32, character: f32) {
        // A drawn curve takes Shape and Character out of play: the stages run at their usual settings.
        let [shape, character] = if self.drawn.is_some() { self.s.recommended() } else { [shape as f64, character as f64] };
        self.s.set(ceiling as f64, shape, character);
    }

    fn set_quality(&mut self, aa: usize, os: usize) {
        let (aa, stages) = (Aa::from_index(aa), OS_STAGES[os.min(OS_STAGES.len() - 1)]);
        if (aa, stages) != (self.aa, self.stages) {
            (self.aa, self.stages) = (aa, stages);
            self.configure();
        }
    }

    fn set_twin(&mut self, on: bool) {
        self.twin = on;
    }

    fn set_custom(&mut self, points: Option<&[f32]>) {
        self.drawn = points.map(Drawn::new);
    }

    fn latency(&self) -> usize {
        self.lat
    }

    fn process(&mut self, left: &mut [f32], right: &mut [f32]) {
        let n = left.len().min(right.len());
        let st = self.stages;
        let mut a = [[0.0f32; MAX_FACTOR]; 2];
        let mut b = [[0.0f32; MAX_FACTOR]; 2];
        for i in 0..n {
            if st == 0 {
                let y = self.frame([left[i] as f64, right[i] as f64]);
                (left[i], right[i]) = (y[0] as f32, y[1] as f32);
                continue;
            }
            (a[0][0], a[1][0]) = (left[i], right[i]);
            let mut len = 1;
            for s in 0..st {
                for c in 0..2 {
                    for j in 0..len {
                        let (e, o) = self.up[c][s].tick(a[c][j]);
                        (b[c][2 * j], b[c][2 * j + 1]) = (e, o);
                    }
                }
                std::mem::swap(&mut a, &mut b);
                len *= 2;
            }
            let [l, r] = &mut a;
            for (x0, x1) in l[..len].iter_mut().zip(&mut r[..len]) {
                let y = self.frame([*x0 as f64, *x1 as f64]);
                (*x0, *x1) = (y[0] as f32, y[1] as f32);
            }
            for s in (0..st).rev() {
                len /= 2;
                for c in 0..2 {
                    for j in 0..len {
                        b[c][j] = self.down[c][s].tick(a[c][2 * j], a[c][2 * j + 1]);
                    }
                }
                std::mem::swap(&mut a, &mut b);
            }
            (left[i], right[i]) = (a[0][0], a[1][0]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A plain hard clip: enough to test the methods and the oversampler.
    struct Hard {
        c: f64,
    }
    impl Shaper for Hard {
        fn name(&self) -> &'static str {
            "Hard"
        }
        fn set_rate(&mut self, _: f64) {}
        fn set(&mut self, c: f64, _: f64, _: f64) {
            self.c = c.clamp(0.01, 1.0);
        }
        fn ceiling(&self) -> f64 {
            self.c
        }
        fn curve(&self, x: f64) -> Pt {
            let c = self.c;
            if x.abs() <= c {
                Pt::linear(x)
            } else {
                Pt { f: c.copysign(x), big_f: c * x.abs() - 0.5 * c * c, df: 0.0 }
            }
        }
    }

    fn make() -> Box<dyn ClipEngine> {
        Box::new(Shaped::new(Hard { c: 1.0 }))
    }

    fn impulse_peak(e: &mut dyn ClipEngine) -> usize {
        let mut x = vec![0.0f32; 512];
        x[10] = 0.5;
        let y = crate::bench::run(e, &x, &[512]);
        let i = y.iter().enumerate().fold((0, 0.0f32), |m, (i, v)| if v.abs() > m.1 { (i, v.abs()) } else { m }).0;
        i - 10
    }

    #[test]
    fn every_quality_passes_the_contract_and_reports_its_true_delay() {
        for (aa, aa_name) in AA_LABELS.iter().enumerate() {
            for (os, os_name) in OS_LABELS.iter().enumerate() {
                let make = move || -> Box<dyn ClipEngine> {
                    let mut e = make();
                    e.set_quality(aa, os);
                    e
                };
                crate::bench::assert_contract(&make);
                let mut e = make();
                e.prepare(48_000.0, 512);
                e.set_quality(aa, os);
                e.set(1.0, 0.5, 0.5);
                let lat = e.latency();
                // Below the ceiling the engine is a linear filter: its impulse peaks at the latency
                // (the adaptive method's half-sample all-pass peaks one sample later at 1x).
                let peak = impulse_peak(e.as_mut());
                let slack = usize::from(aa == 1 && os == 0);
                assert!(peak >= lat && peak <= lat + slack, "{aa_name} {os_name}: latency {lat}, impulse peak {peak}");
                assert!(lat <= 64, "{aa_name} {os_name}: latency {lat}");
            }
        }
    }

    #[test]
    fn twin_matches_the_engine_below_the_ceiling() {
        for (aa, os) in [(0, 0), (1, 0), (0, 2), (2, 5)] {
            let mut e = make();
            let mut t = make();
            for x in [&mut e, &mut t] {
                x.prepare(48_000.0, 512);
                x.set_quality(aa, os);
                x.set(1.0, 0.5, 0.5);
            }
            t.set_twin(true);
            let x: Vec<f32> = (0..4096).map(|i| 0.4 * (i as f32 * 0.05).sin() + 0.3 * (i as f32 * 1.3).sin()).collect();
            let a = crate::bench::run(e.as_mut(), &x, &[512]);
            let b = crate::bench::run(t.as_mut(), &x, &[512]);
            let err = a.iter().zip(&b).fold(0.0f32, |m, (p, q)| m.max((p - q).abs()));
            assert!(err < 1e-6, "{} {}: {err}", AA_LABELS[aa], OS_LABELS[os]);
        }
    }

    #[test]
    fn drawn_curve_integrates_and_scales_with_the_ceiling() {
        let pts: Vec<f32> = (1..=CURVE_POINTS).map(|k| ((k as f32) * 0.11).sin().abs()).collect();
        let d = Drawn::new(&pts);
        let h = 1e-6;
        let mut u = -2.5;
        while u < 2.5 {
            let (a, b) = (d.at(u - h), d.at(u + h));
            assert!(((b.big_f - a.big_f) / (2.0 * h) - d.at(u).f).abs() < 1e-5, "at {u}");
            u += 0.0173;
        }
        assert_eq!(d.at(-0.5).f, -d.at(0.5).f);
        let mut e = make();
        e.prepare(48_000.0, 512);
        e.set_quality(2, 0);
        e.set(0.5, 0.5, 0.5);
        e.set_custom(Some(&pts));
        let (mut l, mut r) = (vec![2.0f32; 4], vec![-0.25f32; 4]);
        e.process(&mut l, &mut r);
        assert!((l[3] - 0.5 * pts[CURVE_POINTS - 1]).abs() < 1e-6 && (r[3] + 0.5 * d.at(0.5).f as f32).abs() < 1e-6, "{l:?} {r:?}");
    }

    #[test]
    fn methods_and_oversampling_cut_aliasing() {
        let alias = |aa: usize, os: usize| {
            let mut e = make();
            e.prepare(48_000.0, 512);
            e.set_quality(aa, os);
            e.set(1.0, 0.5, 0.5);
            crate::bench::alias_dbc(e.as_mut(), 48_000.0, 5_000.0, 12.0)
        };
        let naive = alias(2, 0);
        let b1 = alias(0, 0);
        let ad = alias(1, 0);
        let os4 = alias(2, 1);
        let os16 = alias(2, 3);
        let b1_16 = alias(0, 3);
        println!("naive {naive:.1} b1 {b1:.1} adaptive {ad:.1} os4 {os4:.1} os16 {os16:.1} b1x16 {b1_16:.1}");
        assert!(b1 < naive - 6.0 && ad < naive - 3.0, "naive {naive} b1 {b1} adaptive {ad}");
        assert!(os4 < naive - 15.0 && os16 < os4 - 3.0 && b1_16 < os16 + 3.0, "os4 {os4} os16 {os16} b1x16 {b1_16}");
    }
}
