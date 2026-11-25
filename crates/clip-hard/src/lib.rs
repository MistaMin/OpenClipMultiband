//! Hard clipping engines for OpenClip Multiband.
//!
//! * **Hard BLAMP** (`HardBlamp`): a hard clip at the host rate whose corners are band-limited
//!   by polyBLAMP residuals placed at the fractional instant the signal crosses the ceiling
//!   (found on an 8-point Lagrange fit), then a short peak catcher that can also hold the
//!   inter-sample peak. Always sample-peak safe. 13 samples latency at every rate.
//! * **Hard OS** (`HardOs`): the same corner-corrected clip run at 4x (2x from 64 kHz up)
//!   between linear-phase half-band filters, so the output is the band-limited clip. Band
//!   limiting overshoots; Character decides how much of that overshoot the catcher removes.
//!   Sample-peak safe from Character 50 % up.
//!
//! Shape (both): knee half-width, 0 = razor-hard corner, 1 = a quadratic knee from 1.9 dB under
//! the ceiling to 1.6 dB over it. Character: peak control (see each engine). REPORT.md has the
//! maths and the measurements.

// max/min instead of clamp is deliberate: it maps NaN to a bound (clamp passes NaN through);
// the ENGINES type is the one the plugin expects.
#![allow(clippy::manual_clamp, clippy::type_complexity)]

use clip_api::ClipEngine;
use std::f64::consts::PI;

pub const ENGINES: &[(&str, fn() -> Box<dyn ClipEngine>)] =
    &[("Hard BLAMP", || Box::new(HardBlamp::new())), ("Hard OS", || Box::new(HardOs::new()))];

/// Knee half-width at Shape 100 %, as a fraction of the ceiling.
const KNEE_MAX: f32 = 0.2;
/// The loosest inter-sample allowance of the catcher (+3 dB over the ceiling).
const TP_ALLOW: f32 = 0.41;
/// Parameter glide time constant.
const GLIDE_S: f32 = 0.01;
/// Half-width of the catcher's gain dip, samples.
const DIP_H: usize = 4;
/// Catcher latency: the dip half-width plus the interpolator's look-ahead.
const CATCH_LATENCY: usize = DIP_H + 4;
/// Inputs below this are zero (keeps denormals out of the filters).
const TINY: f32 = 1e-20;

// ------------------------------------------------------------------------------ parameters

#[derive(Clone, Copy)]
struct Glide {
    now: f32,
    target: f32,
}

impl Glide {
    #[inline]
    fn step(&mut self, k: f32) -> f32 {
        self.now += k * (self.target - self.now);
        // land exactly, so a settled ceiling is the ceiling and a settled knee of 0 is 0
        if (self.target - self.now).abs() < 1e-6 {
            self.now = self.target;
        }
        self.now
    }
}

/// Ceiling, knee and character, glided per sample so `set` never clicks.
#[derive(Clone, Copy)]
struct Params {
    ceiling: Glide,
    knee: Glide,
    character: Glide,
    k: f32,
    snap: bool,
}

impl Params {
    fn new() -> Self {
        let g = |v| Glide { now: v, target: v };
        Self { ceiling: g(1.0), knee: g(0.0), character: g(0.5), k: 1.0, snap: true }
    }

    fn prepare(&mut self, fs: f32) {
        self.k = 1.0 - (-1.0 / (GLIDE_S * fs)).exp();
        self.snap = true;
    }

    fn set(&mut self, ceiling: f32, shape: f32, character: f32) {
        // max/min rather than clamp: a NaN setting falls to the low end instead of passing
        self.ceiling.target = ceiling.max(0.01).min(1.0);
        self.knee.target = KNEE_MAX * shape.max(0.0).min(1.0);
        self.character.target = character.max(0.0).min(1.0);
        if self.snap {
            for g in [&mut self.ceiling, &mut self.knee, &mut self.character] {
                g.now = g.target;
            }
            self.snap = false;
        }
    }

    /// (ceiling, knee half-width, character) for the next sample.
    #[inline]
    fn step(&mut self) -> (f32, f32, f32) {
        let c = self.ceiling.step(self.k);
        (c, c * self.knee.step(self.k), self.character.step(self.k))
    }
}

// ------------------------------------------------------------------- the corrected corner

/// The static curve: a hard clip at `c` with a quadratic knee of half-width `h` centred on `c`
/// (C1: slope 1 at `c - h`, slope 0 at `c + h`, never above `c`).
#[inline]
fn knee(x: f32, c: f32, h: f32) -> f32 {
    let a = x.abs();
    // branch-free: noise drives these comparisons at random, a branch would mispredict
    let y = if h == 0.0 {
        a.min(c)
    } else {
        let v = (a - c + h).max(0.0).min(2.0 * h);
        (a - v * v / (4.0 * h) - (a - c - h).max(0.0)).min(c)
    };
    y.copysign(x)
}

/// `(B * ramp)(t) - ramp(t)` for the cubic B-spline kernel `B` (support -2..2): the polyBLAMP
/// residual of a unit slope change at t = 0. Even in `t`, zero for |t| >= 2. (`residual` uses
/// its closed form; this is the definition the tests check it against.)
#[cfg(test)]
fn blamp(t: f64) -> f64 {
    if t.abs() >= 2.0 {
        return 0.0;
    }
    let p = |u: f64| {
        let u = u.max(0.0);
        u * u * u * u * u
    };
    (p(t + 2.0) - 4.0 * p(t + 1.0) + 6.0 * p(t) - 4.0 * p(t - 1.0)) / 120.0 - t.max(0.0)
}

/// `blamp_int` on each unit segment u = t + 2 in j..j+1 is a degree-6 polynomial in s = u - j:
/// the sum of `(-1)^k C(4,k) (s + j - k)^6 / 720` over k <= j, less `(s + j - 2)^2 / 2` from
/// j = 2 on (the integral of the truncated-power form of `blamp`). Expanded at compile time.
static SEGMENTS: [[f64; 7]; 4] = {
    let (b4, b6) = ([1.0, 4.0, 6.0, 4.0, 1.0], [1.0, 6.0, 15.0, 20.0, 15.0, 6.0, 1.0]);
    let mut out = [[0.0; 7]; 4];
    let mut j = 0;
    while j < 4 {
        let mut k = 0;
        while k <= j {
            let (a, sign) = ((j - k) as f64, if k % 2 == 0 { 1.0 } else { -1.0 });
            let mut m = 0;
            while m <= 6 {
                let (mut ap, mut e) = (1.0, 0);
                while e < 6 - m {
                    ap *= a;
                    e += 1;
                }
                out[j][m] += sign * b4[k] * b6[m] * ap / 720.0;
                m += 1;
            }
            k += 1;
        }
        if j >= 2 {
            let a = (j - 2) as f64;
            out[j][0] -= a * a / 2.0;
            out[j][1] -= a;
            out[j][2] -= 0.5;
        }
        j += 1;
    }
    out
};

/// Antiderivative of `blamp`: 0 up to -2, 1/6 from 2 on.
#[inline]
fn blamp_int(t: f64) -> f64 {
    // clamping is exact (the segments meet 0 and 1/6 at the ends) and keeps this branch-free
    let u = t.max(-2.0).min(2.0) + 2.0;
    let j = (u.floor() as usize).min(3);
    let s = u - j as f64;
    let c = &SEGMENTS[j];
    let s2 = s * s;
    // Estrin: three short chains instead of one of six
    (c[0] + c[1] * s) + s2 * ((c[2] + c[3] * s) + s2 * ((c[4] + c[5] * s) + s2 * c[6]))
}

/// Residuals for the four samples around a corner at fraction `d` (0..1) past the second of
/// them. A knee crossed in `width` samples is a corner smeared by a box of that width, so its
/// residual is the box average of `blamp`, from `blamp_int`.
#[inline]
fn residual(d: f32, width: f32) -> [f32; 4] {
    if width < 1e-3 {
        let p5 = |u: f32| u * u * u * u * u;
        let e = 1.0 - d;
        [p5(e), p5(1.0 + e) - 4.0 * p5(e), p5(1.0 + d) - 4.0 * p5(d), p5(d)].map(|v| v / 120.0)
    } else {
        let (d, w) = (d as f64, width as f64);
        let inv = 1.0 / w;
        [0, 1, 2, 3].map(|k| {
            let t = k as f64 - 1.0 - d;
            ((blamp_int(t + 0.5 * w) - blamp_int(t - 0.5 * w)) * inv) as f32
        })
    }
}

/// Degree-7 polynomial and its derivative at `t`, Estrin's scheme (short dependency chains).
#[inline]
fn estrin(p: &[f32; 8], q: &[f32; 8], t: f32) -> (f32, f32) {
    let (t2, t4) = (t * t, t * t * t * t);
    let v = (p[0] + p[1] * t) + t2 * (p[2] + p[3] * t) + t4 * ((p[4] + p[5] * t) + t2 * (p[6] + p[7] * t));
    let dv = (q[0] + q[1] * t) + t2 * (q[2] + q[3] * t) + t4 * ((q[4] + q[5] * t) + t2 * q[6]);
    (v, dv)
}

/// Lagrange fit through the last `taps` input samples, as power-series coefficients, used to
/// find the crossing instant and the slope there. Nodes sit at t = 1 - taps/2 .. taps/2, so the
/// interval being examined is t = 0..1.
struct Fit {
    taps: usize,
    /// cols[i][j]: the t^j coefficient contributed by node i.
    cols: [[f32; 8]; 8],
}

impl Fit {
    fn new(taps: usize) -> Self {
        let node = |i: usize| i as f64 + 1.0 - (taps / 2) as f64;
        let mut cols = [[0.0; 8]; 8];
        for (i, col) in cols.iter_mut().enumerate().take(taps) {
            // expand L_i(t) = prod_{j != i} (t - t_j) / (t_i - t_j)
            let mut poly = [0.0f64; 8];
            poly[0] = 1.0;
            for (deg, j) in (0..taps).filter(|&j| j != i).enumerate() {
                let s = 1.0 / (node(i) - node(j));
                for p in (0..=deg + 1).rev() {
                    let lower = if p > 0 { poly[p - 1] } else { 0.0 };
                    poly[p] = (lower - node(j) * poly[p]) * s;
                }
            }
            for (c, v) in col.iter_mut().zip(poly) {
                *c = v as f32;
            }
        }
        Self { taps, cols }
    }

    /// Power-series coefficients of the fit through the window ending at `x[7]`.
    #[inline]
    fn poly(&self, x: &[f32; 8]) -> [f32; 8] {
        let mut p = [0.0f32; 8];
        for (col, &xi) in self.cols.iter().zip(&x[8 - self.taps..]) {
            for (pj, cj) in p.iter_mut().zip(col) {
                *pj += cj * xi;
            }
        }
        p
    }

    /// Adds the band-limiting residual of one corner (rail `lvl`, slope change sign `sgn`).
    #[cold]
    fn corner(&self, acc: &mut [f32; 4], x: &[f32; 8], lvl: f32, sgn: f32, h: f32) {
        let p = self.poly(x);
        let mut q = [0.0f32; 8];
        for j in 0..7 {
            q[j] = (j + 1) as f32 * p[j + 1];
        }
        let e = 8 - self.taps / 2;
        let (b, c) = (x[e - 1], x[e]);
        let mut t = ((lvl - b) / (c - b)).max(0.0).min(1.0);
        // the linear guess is already close; one Newton step suffices for a cubic at 4x rate
        for _ in 0..self.taps / 4 {
            let (v, dv) = estrin(&p, &q, t);
            if dv.abs() > 1e-9 {
                t = (t - (v - lvl) / dv).max(0.0).min(1.0);
            }
        }
        let m = estrin(&p, &q, t).1.abs();
        let width = if m > 1e-12 { 2.0 * h / m } else { 1e9 };
        let g = sgn * m;
        for (a, r) in acc.iter_mut().zip(residual(t, width)) {
            *a += g * r;
        }
    }
}

/// One channel of the corrected clip. Each crossing of a rail between the two middle samples
/// of the fit window adds `-slope * residual` (top rail) to the four samples around it.
#[derive(Clone, Copy, Default)]
struct Corner {
    x: [f32; 8],
    acc: [f32; 4],
}

impl Corner {
    /// Output delay: 2 samples for the residual's reach plus the fit's look-ahead.
    fn latency(taps: usize) -> usize {
        1 + taps / 2
    }

    #[inline]
    fn tick(&mut self, xn: f32, c: f32, h: f32, fit: &Fit) -> f32 {
        self.x.copy_within(1.., 0);
        self.x[7] = xn;
        let e = 8 - fit.taps / 2;
        let mut acc = [self.acc[1], self.acc[2], self.acc[3], 0.0];
        acc[2] += knee(self.x[e], c, h);
        let (p0, p1) = (self.x[e - 1], self.x[e]);
        // the slope change is -|slope| at the top rail and +|slope| at the bottom one; the two
        // rails sum first so a mirrored input gives a mirrored output, bit for bit
        let (top, bottom) = ((p0 > c) != (p1 > c), (p0 < -c) != (p1 < -c));
        if top || bottom {
            let (mut t, mut b) = ([0.0f32; 4], [0.0f32; 4]);
            if top {
                fit.corner(&mut t, &self.x, c, -1.0, h);
            }
            if bottom {
                fit.corner(&mut b, &self.x, -c, 1.0, h);
            }
            for k in 0..4 {
                acc[k] += t[k] + b[k];
            }
        }
        self.acc = acc;
        // residuals pull inward, but two corners closer than the kernel can overlap
        acc[0].max(-c).min(c)
    }
}

// --------------------------------------------------------------------------- peak catcher

/// Shared tables for the catcher.
struct Tables {
    /// 4x interpolation, tp[i][j]: tap i (offsets -3..=4) of phase (j + 1) / 4 (Hann-windowed
    /// sinc); lane 3 is zero padding.
    tp: [[f32; 4]; 8],
    /// Raised-cosine dip window, offsets -DIP_H..=DIP_H.
    win: [f32; 2 * DIP_H + 1],
}

impl Tables {
    fn new() -> Self {
        let mut tp = [[0.0; 4]; 8];
        for (i, taps) in tp.iter_mut().enumerate() {
            for (j, v) in taps.iter_mut().enumerate().take(3) {
                let t = i as f64 - 3.0 - (j + 1) as f64 / 4.0;
                *v = ((PI * t).sin() / (PI * t) * (0.5 + 0.5 * (PI * t / 4.5).cos())) as f32;
            }
        }
        let mut win = [0.0; 2 * DIP_H + 1];
        for (k, w) in win.iter_mut().enumerate() {
            *w = (0.5 + 0.5 * (PI * (k as f64 - DIP_H as f64) / (DIP_H as f64 + 1.0)).cos()) as f32;
        }
        Self { tp, win }
    }
}

/// Holds the output under a sample threshold and, between samples, under a true-peak threshold,
/// by a short gain dip: each sample's need `e = 1 - threshold / peak` spreads over its
/// neighbours through the raised-cosine window and the deepest wins, so every sample gets at
/// least its own need (the guarantee) and the gain moves smoothly (the low alias).
#[derive(Clone, Copy)]
struct Catch {
    y: [f32; 64],
    e: [f32; 64],
    at: usize,
    quiet: usize,
    carry: f32,
}

impl Default for Catch {
    fn default() -> Self {
        Self { y: [0.0; 64], e: [0.0; 64], at: 0, quiet: 2 * DIP_H + 1, carry: 0.0 }
    }
}

impl Catch {
    #[inline]
    fn tick(&mut self, y: f32, s_thr: f32, t_thr: f32, tab: &Tables) -> f32 {
        self.at = (self.at + 1) & 31;
        self.y[self.at] = y;
        self.y[self.at + 32] = y;
        // sample s (4 back) now has its look-ahead: window s-3 ..= s+4, contiguous in the copy
        let w = &self.y[self.at + 25..self.at + 33];
        let sp = w[3].abs();
        let mut need = if sp > s_thr { 1.0 - s_thr / sp } else { 0.0 };
        // two partial sums: half the dependency chain
        let (mut lo, mut hi) = ([0.0f32; 4], [0.0f32; 4]);
        for i in 0..4 {
            for j in 0..4 {
                lo[j] += tab.tp[i][j] * w[i];
                hi[j] += tab.tp[i + 4][j] * w[i + 4];
            }
        }
        let pk = (lo[0] + hi[0]).abs().max((lo[1] + hi[1]).abs()).max((lo[2] + hi[2]).abs());
        if pk > t_thr {
            need = need.max(1.0 - t_thr / pk);
        }
        // the peak between s and s+1 binds both samples
        let e = need.max(self.carry);
        self.carry = need;
        let s = (self.at + 28) & 31;
        self.e[s] = e;
        self.e[s + 32] = e;
        self.quiet = if e > 0.0 { 0 } else { (self.quiet + 1).min(2 * DIP_H + 1) };
        let out = self.y[(s + 32 - DIP_H) & 31];
        if self.quiet > 2 * DIP_H {
            return out;
        }
        // the window is symmetric, so walking it forward over s-2H ..= s is the same
        let mut dip: f32 = 0.0;
        for (wk, ek) in tab.win.iter().zip(&self.e[s + 32 - 2 * DIP_H..=s + 32]) {
            dip = dip.max(ek * wk);
        }
        out * (1.0 - dip)
    }
}

// ------------------------------------------------------------------------------ Hard BLAMP

/// Hard clip with band-limited corners at the host rate.
///
/// Character: true-peak strictness. 0 lets inter-sample peaks reach +3 dB over the ceiling,
/// 1 holds them at the ceiling. The sample peak is always at or under the ceiling.
pub struct HardBlamp {
    p: Params,
    clip: [Corner; 2],
    catch: [Catch; 2],
    fit: Fit,
    tab: Tables,
}

impl HardBlamp {
    pub fn new() -> Self {
        Self { p: Params::new(), clip: [Corner::default(); 2], catch: [Catch::default(); 2], fit: Fit::new(8), tab: Tables::new() }
    }
}

impl Default for HardBlamp {
    fn default() -> Self {
        Self::new()
    }
}

impl ClipEngine for HardBlamp {
    fn name(&self) -> &'static str {
        "Hard BLAMP"
    }

    fn prepare(&mut self, sample_rate: f32, _max_block: usize) {
        self.p.prepare(sample_rate);
        self.reset();
    }

    fn reset(&mut self) {
        self.clip = [Corner::default(); 2];
        self.catch = [Catch::default(); 2];
    }

    fn set(&mut self, ceiling: f32, shape: f32, character: f32) {
        self.p.set(ceiling, shape, character);
    }

    fn latency(&self) -> usize {
        Corner::latency(self.fit.taps) + CATCH_LATENCY
    }

    fn process(&mut self, left: &mut [f32], right: &mut [f32]) {
        for (l, r) in left.iter_mut().zip(right.iter_mut()) {
            let (c, h, ch) = self.p.step();
            let t_thr = c * (1.0 + TP_ALLOW * (1.0 - ch));
            for (i, v) in [l, r].into_iter().enumerate() {
                let x = if v.abs() < TINY { 0.0 } else { *v };
                let y = self.clip[i].tick(x, c, h, &self.fit);
                *v = self.catch[i].tick(y, c, t_thr, &self.tab).max(-c).min(c);
            }
        }
    }
}

// ---------------------------------------------------------------------------- oversampling

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

/// Target stop-band attenuation of the half-bands, dB.
const ATTEN: f64 = 80.0;

/// Kaiser half-band lowpass between `low_rate` and twice it: flat to 20 kHz, stopped from
/// `low_rate - 20 kHz` (where images and aliases would land in 0..20 kHz). Centre delay capped
/// at `max_m` high-rate samples (the attenuation then drops to what the length allows).
/// Returns the long polyphase branch h[0], h[2], ..., h[4K+2], summing to 1/2.
fn halfband(low_rate: f64, max_m: usize) -> Vec<f32> {
    let delta = ((low_rate - 40_000.0) / (2.0 * low_rate)).max(0.02);
    let need = (((ATTEN - 7.95) / (14.36 * delta) - 2.0) / 4.0).ceil().max(0.0) as usize;
    let k = need.min((max_m - 1) / 2);
    let (n, m) = (4 * k + 3, 2 * k + 1);
    let atten = (14.36 * delta * (n - 1) as f64 + 7.95).min(ATTEN);
    let beta = 0.1102 * (atten - 8.7);
    let mut g: Vec<f64> = (0..=m)
        .map(|i| {
            let off = (2 * i) as f64 - m as f64; // odd offsets from the centre tap
            let sinc = (PI * off / 2.0).sin() / (PI * off / 2.0);
            0.5 * sinc * bessel_i0(beta * (1.0 - (off / m as f64).powi(2)).max(0.0).sqrt()) / bessel_i0(beta)
        })
        .collect();
    let s: f64 = g.iter().sum();
    g.iter_mut().for_each(|v| *v *= 0.5 / s);
    g.into_iter().map(|v| v as f32).collect()
}

/// Dot product in eight independent lanes (a plain `sum` is one serial chain).
#[inline]
fn dot(a: &[f32], b: &[f32]) -> f32 {
    let mut acc = [0.0f32; 8];
    let (ca, cb) = (a.chunks_exact(8), b.chunks_exact(8));
    let tail: f32 = ca.remainder().iter().zip(cb.remainder()).map(|(x, y)| x * y).sum();
    for (x, y) in ca.zip(cb) {
        for k in 0..8 {
            acc[k] += x[k] * y[k];
        }
    }
    tail + ((acc[0] + acc[4]) + (acc[1] + acc[5])) + ((acc[2] + acc[6]) + (acc[3] + acc[7]))
}

/// Polyphase half-band interpolator by 2. Delay 2K+1 high-rate samples.
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

    /// (even, odd) high-rate outputs for one input.
    #[inline]
    fn tick(&mut self, x: f32) -> (f32, f32) {
        let l = self.g.len();
        self.pos = if self.pos == 0 { l - 1 } else { self.pos - 1 };
        self.buf[self.pos] = x;
        self.buf[self.pos + l] = x;
        let w = &self.buf[self.pos..self.pos + l];
        (dot(&self.g, w), w[l / 2 - 1])
    }
}

/// Polyphase half-band decimator by 2. Delay 2K+1 high-rate samples.
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

    #[inline]
    fn tick(&mut self, e: f32, o: f32) -> f32 {
        let l = self.g.len();
        self.pos = if self.pos == 0 { l - 1 } else { self.pos - 1 };
        self.even[self.pos] = e;
        self.even[self.pos + l] = e;
        self.odd[self.pos] = o;
        self.odd[self.pos + l] = o;
        dot(&self.g, &self.even[self.pos..self.pos + l]) + 0.5 * self.odd[self.pos + l / 2]
    }
}

struct OsChan {
    up1: Up,
    down1: Down,
    up2: Up,
    down2: Down,
    pad: [f32; 4],
    pad_at: usize,
    clip: Corner,
    catch: Catch,
}

impl OsChan {
    fn new(b1: &[f32], b2: &[f32]) -> Self {
        Self {
            up1: Up::new(b1),
            down1: Down::new(b1),
            up2: Up::new(b2),
            down2: Down::new(b2),
            pad: [0.0; 4],
            pad_at: 0,
            clip: Corner::default(),
            catch: Catch::default(),
        }
    }

    fn reset(&mut self) {
        for b in [&mut self.up1.buf, &mut self.up2.buf, &mut self.down1.even, &mut self.down1.odd, &mut self.down2.even, &mut self.down2.odd] {
            b.fill(0.0);
        }
        self.pad = [0.0; 4];
        self.clip = Corner::default();
        self.catch = Catch::default();
    }

    /// The clip at the oversampled rate, behind `pad` samples of delay that make the total
    /// latency a whole number of host samples.
    #[inline]
    fn clip(&mut self, x: f32, pad: usize, c: f32, h: f32, fit: &Fit) -> f32 {
        self.pad_at = (self.pad_at + 1) & 3;
        self.pad[self.pad_at] = x;
        let x = self.pad[(self.pad_at + 4 - pad) & 3];
        self.clip.tick(x, c, h, fit)
    }
}

/// The corner-corrected hard clip at 4x (2x from 64 kHz up) between linear-phase half-bands.
///
/// Character: peak control of the band-limited result. 0 leaves it free (the overshoot of band
/// limiting passes: the cleanest spectrum); 0.5 holds sample peaks at the ceiling; 1 also holds
/// inter-sample peaks there. Sample-peak safe from 0.5 up.
pub struct HardOs {
    p: Params,
    ch: Vec<OsChan>,
    factor: usize,
    pad: usize,
    latency: usize,
    fit: Fit,
    tab: Tables,
}

impl HardOs {
    pub fn new() -> Self {
        let mut e = Self { p: Params::new(), ch: Vec::new(), factor: 4, pad: 0, latency: 0, fit: Fit::new(4), tab: Tables::new() };
        e.prepare(48_000.0, 0);
        e
    }

    #[inline]
    fn run(&mut self, i: usize, x: f32, c: f32, h: f32) -> f32 {
        let (pad, fit) = (self.pad, &self.fit);
        let ch = &mut self.ch[i];
        let (a0, a1) = ch.up1.tick(x);
        if self.factor == 4 {
            let mut v = [0.0; 2];
            for (vj, a) in v.iter_mut().zip([a0, a1]) {
                let (b0, b1) = ch.up2.tick(a);
                let c0 = ch.clip(b0, pad, c, h, fit);
                let c1 = ch.clip(b1, pad, c, h, fit);
                *vj = ch.down2.tick(c0, c1);
            }
            ch.down1.tick(v[0], v[1])
        } else {
            let c0 = ch.clip(a0, pad, c, h, fit);
            let c1 = ch.clip(a1, pad, c, h, fit);
            ch.down1.tick(c0, c1)
        }
    }
}

impl Default for HardOs {
    fn default() -> Self {
        Self::new()
    }
}

impl ClipEngine for HardOs {
    fn name(&self) -> &'static str {
        "Hard OS"
    }

    fn prepare(&mut self, sample_rate: f32, _max_block: usize) {
        self.p.prepare(sample_rate);
        let fs = sample_rate as f64;
        self.factor = if fs < 64_000.0 { 4 } else { 2 };
        // 49 keeps 44.1 kHz inside 64 samples; 48 kHz needs only 31
        let b1 = halfband(fs, 49);
        let b2 = halfband(2.0 * fs, 49);
        let (m1, m2) = (b1.len() - 1, b2.len() - 1);
        let f = self.factor;
        // delay in clip-rate samples: stage 1 is m1 at 2x each way, stage 2 is m2 at 4x each way
        let d = if f == 4 { 4 * m1 + 2 * m2 } else { 2 * m1 } + Corner::latency(self.fit.taps);
        self.pad = (f - d % f) % f;
        self.latency = (d + self.pad) / f + CATCH_LATENCY;
        self.ch = (0..2).map(|_| OsChan::new(&b1, &b2)).collect();
    }

    fn reset(&mut self) {
        self.ch.iter_mut().for_each(OsChan::reset);
    }

    fn set(&mut self, ceiling: f32, shape: f32, character: f32) {
        self.p.set(ceiling, shape, character);
    }

    fn latency(&self) -> usize {
        self.latency
    }

    fn process(&mut self, left: &mut [f32], right: &mut [f32]) {
        for (l, r) in left.iter_mut().zip(right.iter_mut()) {
            let (c, h, ch) = self.p.step();
            let s_thr = c * (1.0 + (1.0 - 2.0 * ch).max(0.0));
            let t_thr = s_thr.max(c * (1.0 + TP_ALLOW * (2.0 - 2.0 * ch).min(1.0)));
            for (i, v) in [l, r].into_iter().enumerate() {
                let x = if v.abs() < TINY { 0.0 } else { *v };
                let y = self.run(i, x, c, h);
                *v = self.ch[i].catch.tick(y, s_thr, t_thr, &self.tab).max(-s_thr).min(s_thr);
            }
        }
    }
}

#[cfg(test)]
mod tests;
