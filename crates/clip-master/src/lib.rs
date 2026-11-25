//! Stereo-linked, look-ahead final clipper.
mod filters;

use clip_api::ClipEngine;
use filters::{INNER, OUTER, TOP};

type Factory = fn() -> Box<dyn ClipEngine>;

pub const ENGINES: &[(&str, Factory)] = &[("Master 8x", || Box::new(Master::new()))];

const TILE: usize = 64;
const GAIN_TAPS: usize = 5;
const FORECAST_DELAY: usize = 16 + GAIN_TAPS - 1;

// Keep the previous FIR support at the front. Each MAC loop below then runs over
// contiguous time samples, including when a caller splits a tile at any point.
struct Window<const CAP: usize> {
    samples: [f32; CAP],
}
impl<const CAP: usize> Default for Window<CAP> {
    fn default() -> Self {
        Self {
            samples: [0.0; CAP],
        }
    }
}
impl<const CAP: usize> Window<CAP> {
    fn put(&mut self, history: usize, x: &[f32]) {
        self.samples[history..history + x.len()].copy_from_slice(x);
    }
    fn finish(&mut self, history: usize, n: usize) {
        self.samples.copy_within(n..n + history, 0);
    }
}

#[inline]
fn add_pair(out: &mut [f32], a: &[f32], b: &[f32], tap: f32) {
    for ((y, &x), &z) in out.iter_mut().zip(a).zip(b) {
        *y = (x + z).mul_add(tap, *y);
    }
}

impl<const CAP: usize> Window<CAP> {
    fn interpolate<const M: usize>(&mut self, x: &[f32], out: &mut [f32], taps: &[f32; M]) {
        let n = x.len();
        self.put(2 * M, x);
        let a = &self.samples;
        let mut mids = [0.0; 4 * TILE];
        for (i, &h) in taps.iter().enumerate() {
            add_pair(&mut mids[..n], &a[M - i..], &a[M + 1 + i..], h);
        }
        for ((pair, &even), &odd) in out
            .as_chunks_mut::<2>()
            .0
            .iter_mut()
            .zip(&a[M..])
            .zip(&mids[..n])
        {
            pair.copy_from_slice(&[even, odd]);
        }
        self.finish(2 * M, n);
    }
}

#[derive(Default)]
struct Decimator<const CAP: usize> {
    even: Window<CAP>,
    odd: Window<CAP>,
}
impl<const CAP: usize> Decimator<CAP> {
    fn apply(&mut self, x: &[f32], out: &mut [f32], centre: f32, even: &[f32], odd: &[f32]) {
        let m = odd.len();
        let n = out.len();
        for ((pair, e), o) in x
            .as_chunks::<2>()
            .0
            .iter()
            .zip(&mut self.even.samples[2 * m..])
            .zip(&mut self.odd.samples[2 * m..])
        {
            *e = pair[0];
            *o = pair[1];
        }
        for (y, &s) in out.iter_mut().zip(&self.even.samples[m..]) {
            *y = centre * s;
        }
        for (i, &h) in even.iter().enumerate() {
            add_pair(
                out,
                &self.even.samples[m - 1 - i..],
                &self.even.samples[m + 1 + i..],
                h,
            );
        }
        for (i, &h) in odd.iter().enumerate() {
            add_pair(
                out,
                &self.odd.samples[m - 1 - i..],
                &self.odd.samples[m + i..],
                h,
            );
        }
        self.even.finish(2 * m, n);
        self.odd.finish(2 * m, n);
    }
}

// The static knee integrates a cubic smoothstep slope. Its primitive gives an
// exact box mean along the local tangent at each oversampled point.
#[derive(Clone, Copy, Default)]
struct FrameSettings {
    ceiling: f32,
    width: f32,
    inv_span: f32,
    release: f32,
}
impl FrameSettings {
    fn primitive(self, x: f64) -> f64 {
        let ceiling = self.ceiling as f64;
        let width = self.width as f64;
        let k = ceiling - width;
        let span = 2.0 * width;
        let a = x.abs();
        let core = a.min(k);
        let d = (a - k).clamp(0.0, span);
        let t = d * self.inv_span as f64;
        0.5 * core * core
            + k * d
            + d * d * (0.5 + t * t * (-0.25 + 0.1 * t))
            + ceiling * (a - (ceiling + width)).max(0.0)
    }
    fn mean(self, mid: f32, half: f32) -> f32 {
        if mid.abs() + half <= self.ceiling - self.width {
            return mid;
        }
        if mid.abs() - half >= self.ceiling + self.width {
            return self.ceiling.copysign(mid);
        }
        // Limit cancellation for nearly equal endpoints.
        if half <= 0.002 * self.width {
            return self.clip(mid);
        }
        let (mid, half) = (mid as f64, half as f64);
        ((self.primitive(mid + half) - self.primitive(mid - half)) / (2.0 * half)) as f32
    }
    #[inline]
    fn clip(self, x: f32) -> f32 {
        let a = x.abs();
        let knee = self.ceiling - self.width;
        let t = ((a - knee) * self.inv_span).clamp(0.0, 1.0);
        let transition = t * (1.0 - t * t * (1.0 - 0.5 * t));
        (knee.min(a) + 2.0 * self.width * transition).copysign(x)
    }
}

#[derive(Default)]
struct Channel {
    up: Window<{ TILE + 36 }>,
    inner_up: Window<{ 2 * TILE + 10 }>,
    top_up: Window<{ 4 * TILE + 8 }>,
    top_down: Decimator<{ 4 * TILE + 8 }>,
    inner_down: Decimator<{ 2 * TILE + 10 }>,
    down: Decimator<{ TILE + 36 }>,
    intervals: Window<{ 8 * TILE + 9 }>,
    audio: Window<{ TILE + 31 }>,
    last_quarter: f32,
}

impl Channel {
    fn render(
        &mut self,
        x: &mut [f32],
        ctrl: &[FrameSettings],
        bank: &OutputFilter,
        probe: &PeakDetector,
        peak: &mut [f32],
    ) {
        let n = x.len();
        for v in x.iter_mut() {
            *v = if v.abs() >= 1e-24 {
                v.clamp(-1e20, 1e20)
            } else {
                0.0
            };
        }
        let mut x2 = [0.0; TILE * 2];
        let x2 = &mut x2[..n * 2];
        let mut x4 = [0.0; TILE * 4];
        let x4 = &mut x4[..n * 4];
        self.up.interpolate(x, x2, &OUTER);
        self.inner_up.interpolate(x2, x4, &INNER);
        let mut x8 = [0.0; TILE * 8];
        let x8 = &mut x8[..8 * n];
        self.top_up.interpolate(x4, x8, &TOP);
        // Read the centred tangent eight oversampled ticks behind the input.
        self.intervals.put(9, x8);
        let v = &self.intervals.samples;
        for (i, (out, &ctl)) in x8.as_chunks_mut::<8>().0.iter_mut().zip(ctrl).enumerate() {
            for (j, a) in out.iter_mut().enumerate() {
                let k = 8 * i + j;
                let mid = v[k + 1];
                let half = 0.25 * (v[k + 2] - v[k]).abs();
                *a = ctl.mean(mid, half);
            }
        }
        self.intervals.finish(9, 8 * n);
        self.top_down.apply(x8, x4, 0.5, &[], &TOP.map(|h| 0.5 * h));
        self.inner_down
            .apply(x4, x2, 0.5, &[], &INNER.map(|h| 0.5 * h));
        self.down.apply(x2, x, bank.centre, &bank.even, &bank.odd);
        self.audio.put(31, x);
        probe.detect(&self.audio.samples, peak, &mut self.last_quarter);
        let emit = 31 - FORECAST_DELAY;
        x.copy_from_slice(&self.audio.samples[emit..emit + n]);
        self.audio.finish(31, n);
    }
}

struct OutputFilter {
    centre: f32,
    even: [f32; 18],
    odd: [f32; 18],
}
impl OutputFilter {
    fn new(h: &[f32; 37]) -> Self {
        Self {
            centre: h[0],
            even: std::array::from_fn(|i| h[2 * i + 2]),
            odd: std::array::from_fn(|i| h[2 * i + 1]),
        }
    }
}

struct PeakDetector {
    sum: [f32; 16],
    diff: [f32; 16],
    half: [f32; 16],
}
impl PeakDetector {
    fn for_rate(rate: f32) -> Self {
        // At 44.1 kHz the passband approaches Nyquist. Its wider-band phase fit
        // maintains peak accuracy on bright, changing signals.
        if rate < 47_000.0 {
            return Self {
                sum: filters::QUARTER_SUM,
                diff: filters::QUARTER_DIFF,
                half: filters::HALF,
            };
        }
        fn h(t: f64) -> f32 {
            let a = std::f64::consts::PI * t;
            (a.sin() / a * (0.5 + 0.5 * (a / 16.0).cos())) as f32
        }
        Self {
            sum: std::array::from_fn(|i| 0.5 * (h(i as f64 - 15.25) + h(15.75 - i as f64))),
            diff: std::array::from_fn(|i| 0.5 * (h(i as f64 - 15.25) - h(15.75 - i as f64))),
            half: std::array::from_fn(|i| h(i as f64 - 15.5)),
        }
    }
    fn detect(&self, x: &[f32], peak: &mut [f32], last: &mut f32) {
        let n = peak.len();
        let (mut sum, mut diff, mut half) = ([0.0; TILE], [0.0; TILE], [0.0; TILE]);
        for i in 0..16 {
            let a = &x[i..i + n];
            let b = &x[31 - i..31 - i + n];
            for j in 0..n {
                let pair = a[j] + b[j];
                sum[j] = pair.mul_add(self.sum[i], sum[j]);
                diff[j] = (a[j] - b[j]).mul_add(self.diff[i], diff[j]);
                half[j] = pair.mul_add(self.half[i], half[j]);
            }
        }
        for j in 0..n {
            let q = sum[j] + diff[j];
            let tq = sum[j] - diff[j];
            let a = x[15 + j];
            let b = half[j];
            let (mut mid, mut left, mut right) = (a, *last, q);
            if q.abs() > mid.abs() {
                (mid, left, right) = (q, a, b);
            }
            if b.abs() > mid.abs() {
                (mid, left, right) = (b, q, tq);
            }
            if tq.abs() > mid.abs() {
                (mid, left, right) = (tq, b, x[16 + j]);
            }
            *last = tq;
            // A parabola through three signed samples estimates the extremum between grid points.
            let mag = mid.abs();
            let curve = (2.0 * mid - left - right).abs();
            let slope = (right - left).abs().min(curve);
            let refined = mag + 0.125 * slope * slope / curve.max(1e-12);
            peak[j] = peak[j].max(refined);
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Settings {
    ceiling: f32,
    shape: f32,
    character: f32,
}

struct Master {
    ch: [Channel; 2],
    probe: PeakDetector,
    down: OutputFilter,
    target: Settings,
    current: Settings,
    active: bool,
    rate: f32,
    envelope: f64,
    gain_history: [f64; GAIN_TAPS],
    cursor: usize,
    gain_sum: f64,
    smooth: f32,
}

impl Master {
    fn new() -> Self {
        let target = Settings {
            ceiling: 1.0,
            shape: 0.5,
            character: 0.5,
        };
        Self {
            ch: Default::default(),
            probe: PeakDetector::for_rate(48_000.0),
            down: OutputFilter::new(&filters::AUDIO),
            target,
            current: target,
            active: false,
            rate: 48_000.0,
            envelope: 1.0,
            gain_history: [1.0; GAIN_TAPS],
            cursor: 0,
            gain_sum: GAIN_TAPS as f64,
            smooth: 0.004,
        }
    }
    // Five forecast gain values spread the attack across the look-ahead.
    fn gain(&mut self, peak: f32, c: f32, release: f32) -> f32 {
        let required = (0.996 * c / peak.max(1e-12)).min(1.0);
        self.envelope =
            (required as f64).min(self.envelope + release as f64 * (1.0 - self.envelope));
        self.gain_sum += self.envelope - self.gain_history[self.cursor];
        self.gain_history[self.cursor] = self.envelope;
        self.cursor = (self.cursor + 1) % GAIN_TAPS;
        (self.gain_sum / GAIN_TAPS as f64) as f32
    }
    fn coefficients(&self) -> FrameSettings {
        let c = self.current.ceiling;
        let w = c * (0.035 + 0.40 * self.current.shape);
        let release = 1.0 / (self.rate * (0.008 + 0.072 * self.current.character));
        FrameSettings {
            ceiling: c,
            width: w,
            inv_span: 0.5 / w,
            release,
        }
    }
}

impl ClipEngine for Master {
    fn name(&self) -> &'static str {
        "Master 8x"
    }
    fn prepare(&mut self, rate: f32, _max_block: usize) {
        self.rate = if rate.is_finite() {
            rate.clamp(8000.0, 768000.0)
        } else {
            48_000.0
        };
        self.probe = PeakDetector::for_rate(self.rate);
        self.down = OutputFilter::new(if self.rate < 47_000.0 {
            &filters::NARROW
        } else {
            &filters::AUDIO
        });
        self.smooth = 1.0 - (-1.0 / (self.rate * 0.004)).exp();
        self.reset();
    }
    fn reset(&mut self) {
        self.ch = Default::default();
        self.current = self.target;
        self.active = false;
        self.envelope = 1.0;
        self.gain_history = [1.0; GAIN_TAPS];
        self.cursor = 0;
        self.gain_sum = GAIN_TAPS as f64;
    }
    fn set(&mut self, c: f32, s: f32, k: f32) {
        fn valid(x: f32, lo: f32, hi: f32, old: f32) -> f32 {
            if x.is_nan() {
                old
            } else {
                x.clamp(lo, hi)
            }
        }
        self.target = Settings {
            ceiling: valid(c, 0.01, 1.0, self.target.ceiling),
            shape: valid(s, 0.0, 1.0, self.target.shape),
            character: valid(k, 0.0, 1.0, self.target.character),
        };
        if !self.active {
            self.current = self.target;
        }
    }
    fn latency(&self) -> usize {
        36 + 5 + 2 + 1 + FORECAST_DELAY
    }
    fn process(&mut self, l: &mut [f32], r: &mut [f32]) {
        if l.is_empty() || r.is_empty() {
            return;
        }
        self.active = true;
        for (left, right) in l.chunks_mut(TILE).zip(r.chunks_mut(TILE)) {
            let n = left.len().min(right.len());
            let (left, right) = (&mut left[..n], &mut right[..n]);
            let mut controls = [self.coefficients(); TILE];
            if self.current != self.target {
                for control in &mut controls[..n] {
                    fn slew(x: &mut f32, target: f32, alpha: f32) {
                        let next = *x + alpha * (target - *x);
                        *x = if next == *x { target } else { next };
                    }
                    slew(&mut self.current.ceiling, self.target.ceiling, self.smooth);
                    slew(&mut self.current.shape, self.target.shape, self.smooth);
                    slew(
                        &mut self.current.character,
                        self.target.character,
                        self.smooth,
                    );
                    *control = self.coefficients();
                }
            }
            let mut peak = [0.0; TILE];
            self.ch[0].render(left, &controls, &self.down, &self.probe, &mut peak[..n]);
            self.ch[1].render(right, &controls, &self.down, &self.probe, &mut peak[..n]);
            for (((l, r), &p), ctl) in left.iter_mut().zip(right).zip(&peak).zip(&controls) {
                let gain = self.gain(p, ctl.ceiling, ctl.release);
                let rail = self.target.ceiling;
                *l = (*l * gain).clamp(-rail, rail);
                *r = (*r * gain).clamp(-rail, rail);
            }
        }
    }
}

#[cfg(test)]
mod tests;
