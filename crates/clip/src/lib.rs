// Copyright (c) 2026 Marcos Quirino Deida
//! The clip section of OpenClip Multiband: Drive, then one clipping algorithm through the chosen
//! anti-aliasing (Hann-B1 ADAA, adaptive ADAA or none) and oversampling (1x to 64x), behind the
//! olm-api `Module` contract.
//!
//! Every algorithm is built and prepared up front, so switching never allocates; the old and new
//! algorithms crossfade over 10 ms. Each is padded to the slowest one's latency, so the section's
//! latency follows only the quality setting. A quality change fades out for 5 ms, applies, and
//! fades back in. In Zero Latency mode the section runs adaptive ADAA at 1x and an algorithm that
//! needs look-ahead falls back to `ZERO_FALLBACK`.

pub mod algos;

use std::sync::LazyLock;

use algos::{ExpKnee, Ferro, Hard, Rail, SmoothClip, ToneClip};
use clip_api::shaper::{Shaped, CURVE_POINTS};
pub use clip_api::shaper::Shaper;
use clip_api::ClipEngine;
use olm_api::{Kind, Meters, Module, ParamDesc};

pub use clip_api::shaper::{Drawn, Pt, AA_LABELS, CURVE_POINTS as DRAWN_POINTS, OS_LABELS};

pub type Factory = fn() -> Box<dyn ClipEngine>;
pub type ShaperFactory = fn() -> Box<dyn Shaper>;

fn eng<S: Shaper + Default + 'static>() -> Box<dyn ClipEngine> {
    Box::new(Shaped::new(S::default()))
}

fn shp<S: Shaper + Default + 'static>() -> Box<dyn Shaper> {
    Box::new(S::default())
}

/// Every algorithm as a bare curve and stages (the editor draws these), in host choice order.
pub const ALGOS: &[(&str, ShaperFactory)] = &[
    ("Honey", shp::<ExpKnee>),
    ("Flint", shp::<Hard>),
    ("Ghost", shp::<SmoothClip>),
    ("Ember", shp::<ToneClip>),
    ("Magnet", shp::<Ferro>),
    ("Volt", shp::<Rail>),
];

/// The same algorithms as engines. Append only once released: sessions store the index.
pub const ENGINES: &[(&str, Factory)] = &[
    ("Honey", eng::<ExpKnee>),
    ("Flint", eng::<Hard>),
    ("Ghost", eng::<SmoothClip>),
    ("Ember", eng::<ToneClip>),
    ("Magnet", eng::<Ferro>),
    ("Volt", eng::<Rail>),
];

fn master8x() -> Box<dyn ClipEngine> {
    (clip_master::ENGINES[0].1)()
}

/// The output ceiling: the true-peak-safe master engine, and Honey for Zero Latency.
pub const OUTPUT_ENGINES: &[(&str, Factory)] = &[("Master 8x", master8x), ("Honey", eng::<ExpKnee>)];

/// Engine used in Zero Latency mode when the chosen one has latency.
pub const ZERO_FALLBACK: &str = "Honey";

pub mod index {
    pub const ALGORITHM: usize = 0;
    pub const DRIVE: usize = 1;
    pub const CEILING: usize = 2;
    pub const SHAPE: usize = 3;
    pub const CHARACTER: usize = 4;
    pub const AA: usize = 5;
    pub const OS: usize = 6;
}

static LABELS: LazyLock<&'static [&'static str]> = LazyLock::new(|| Box::leak(ENGINES.iter().map(|e| e.0).collect::<Vec<_>>().into_boxed_slice()));

static PARAMS: LazyLock<Vec<ParamDesc>> = LazyLock::new(|| {
    vec![
        ParamDesc::choice("algorithm", "Algorithm", &LABELS, 0),
        // Wider than any one host control: a band adds its own offset to the All Bands set.
        ParamDesc::new("drive", "Drive", -24.0, 48.0, 0.0, Kind::Db),
        ParamDesc::new("ceiling", "Ceiling", -48.0, 0.0, 0.0, Kind::Db),
        ParamDesc::new("shape", "Shape", 0.0, 100.0, 50.0, Kind::Percent).steps(101),
        ParamDesc::new("character", "Character", 0.0, 100.0, 50.0, Kind::Percent).steps(101),
        ParamDesc::choice("aa", "Anti-Alias", AA_LABELS, 0),
        ParamDesc::choice("oversampling", "Oversampling", OS_LABELS, 0),
    ]
});

/// Index of an algorithm label in `ENGINES`.
pub fn engine_index(label: &str) -> Option<usize> {
    ENGINES.iter().position(|e| e.0 == label)
}

const FADE_MS: f32 = 10.0;
const DIP_MS: f32 = 5.0;
const SMOOTH_MS: f32 = 5.0;
/// Settings move in steps this long while they glide.
const CHUNK: usize = 32;
/// Room for the slowest algorithm at 64x, in host samples.
const MAX_PAD: usize = 1024;

#[derive(Default)]
struct Delay {
    buf: Vec<f32>,
    pos: usize,
}

impl Delay {
    fn alloc(&mut self, len: usize) {
        self.buf = vec![0.0; len + 1];
        self.pos = 0;
    }

    #[inline]
    fn push(&mut self, x: f32, d: usize) -> f32 {
        let n = self.buf.len();
        if d == 0 || n == 0 {
            return x;
        }
        self.buf[self.pos] = x;
        let y = self.buf[(self.pos + n - d.min(n - 1)) % n];
        self.pos = (self.pos + 1) % n;
        y
    }
}

pub struct Clipper {
    engines: Vec<Box<dyn ClipEngine>>,
    lat: Vec<usize>,
    pads: Vec<[Delay; 2]>,
    max_lat: usize,
    fallback: usize,
    zero: bool,
    algo: usize,
    active: usize,
    prev: Option<usize>,
    fade: f32,
    fade_step: f32,
    /// Quality asked for (aa, os) and in force; a change dips the output while it is applied.
    quality: (usize, usize),
    applied: (usize, usize),
    dip: f32,
    dip_step: f32,
    drive: f32,
    drive_target: f32,
    /// Ceiling (linear), shape, character: current and target.
    set_now: [f32; 3],
    set_to: [f32; 3],
    smooth: f32,
    chunk_smooth: f32,
    scratch: [Vec<f32>; 2],
    meters: Meters,
    view: [f32; 4],
}

impl Default for Clipper {
    fn default() -> Self {
        Self::with(ENGINES)
    }
}

impl Clipper {
    /// A clipper over `list` (algorithm parameter indexes it).
    pub fn with(list: &[(&str, Factory)]) -> Self {
        let engines: Vec<Box<dyn ClipEngine>> = list.iter().map(|e| (e.1)()).collect();
        let n = engines.len();
        let mut c = Self {
            engines,
            lat: vec![0; n],
            pads: (0..n).map(|_| Default::default()).collect(),
            max_lat: 0,
            fallback: list.iter().position(|e| e.0 == ZERO_FALLBACK).unwrap_or(0),
            zero: false,
            algo: 0,
            active: 0,
            prev: None,
            fade: 0.0,
            fade_step: 1.0,
            quality: (0, 0),
            applied: (usize::MAX, 0),
            dip: 1.0,
            dip_step: 1.0,
            drive: 1.0,
            drive_target: 1.0,
            set_now: [1.0, 0.5, 0.5],
            set_to: [1.0, 0.5, 0.5],
            smooth: 1.0,
            chunk_smooth: 1.0,
            scratch: Default::default(),
            meters: Meters::default(),
            view: [0.0, 0.0, -200.0, -200.0],
        };
        for (i, d) in PARAMS.iter().enumerate() {
            c.set_param(i, d.default);
        }
        c
    }

    /// The dry path's twin: every engine with a straight-line curve.
    pub fn set_twin(&mut self, on: bool) {
        for e in &mut self.engines {
            e.set_twin(on);
        }
    }

    /// A drawn curve in place of every algorithm's own, or `None`.
    pub fn set_custom(&mut self, points: Option<&[f32; CURVE_POINTS]>) {
        for e in &mut self.engines {
            e.set_custom(points.map(|p| &p[..]));
        }
    }

    /// Quality in force for the current mode: Zero Latency runs adaptive ADAA at 1x.
    fn effective(&self) -> (usize, usize) {
        if self.zero { (1, 0) } else { self.quality }
    }

    fn apply_quality(&mut self) {
        let q = self.effective();
        for (e, eng) in self.engines.iter_mut().enumerate() {
            eng.set_quality(q.0, q.1);
            self.lat[e] = eng.latency();
            for d in &mut self.pads[e] {
                d.buf.fill(0.0);
            }
        }
        self.max_lat = self.lat.iter().copied().max().unwrap_or(0).min(MAX_PAD);
        self.applied = q;
        self.active = self.wanted();
        self.prev = None;
    }

    fn wanted(&self) -> usize {
        if self.zero && self.lat[self.algo] > 0 { self.fallback } else { self.algo }
    }

    fn pad_to(&self) -> usize {
        if self.zero { 0 } else { self.max_lat }
    }

    fn apply_settings(&mut self, e: usize) {
        let [c, s, k] = self.set_now;
        self.engines[e].set(c, s, k);
    }

    /// Runs engine `e` and its latency pad over the buffers, gliding settings in chunks.
    fn run_engine(&mut self, e: usize, l: &mut [f32], r: &mut [f32], glide: bool) {
        let pad = self.pad_to().saturating_sub(self.lat[e]);
        if !glide {
            self.apply_settings(e);
            self.engines[e].process(l, r);
        } else {
            let saved = self.set_now;
            let mut at = 0;
            while at < l.len() {
                let n = CHUNK.min(l.len() - at);
                for k in 0..3 {
                    self.set_now[k] += (self.set_to[k] - self.set_now[k]) * self.chunk_smooth;
                }
                self.apply_settings(e);
                self.engines[e].process(&mut l[at..at + n], &mut r[at..at + n]);
                at += n;
            }
            // The caller advances the glide once per block, for every engine alike.
            self.set_now = saved;
        }
        if pad > 0 {
            let [pl, pr] = &mut self.pads[e];
            for i in 0..l.len() {
                l[i] = pl.push(l[i], pad);
                r[i] = pr.push(r[i], pad);
            }
        }
    }

    fn start_engine(&mut self, e: usize) {
        self.engines[e].reset();
        for d in &mut self.pads[e] {
            d.buf.fill(0.0);
        }
    }
}

impl Module for Clipper {
    fn params(&self) -> &'static [ParamDesc] {
        &PARAMS
    }

    fn prepare(&mut self, sample_rate: f32, max_block: usize) {
        for eng in &mut self.engines {
            eng.prepare(sample_rate, max_block);
        }
        for p in &mut self.pads {
            for d in p {
                d.alloc(MAX_PAD);
            }
        }
        self.scratch = [vec![0.0; max_block], vec![0.0; max_block]];
        self.fade_step = 1.0 / (FADE_MS * 1e-3 * sample_rate).max(1.0);
        self.dip_step = 1.0 / (DIP_MS * 1e-3 * sample_rate).max(1.0);
        self.smooth = 1.0 - (-1.0 / (SMOOTH_MS * 1e-3 * sample_rate)).exp();
        self.chunk_smooth = 1.0 - (-(CHUNK as f32) / (SMOOTH_MS * 1e-3 * sample_rate)).exp();
        self.apply_quality();
        self.dip = 1.0;
        self.drive = self.drive_target;
        self.set_now = self.set_to;
    }

    fn reset(&mut self) {
        for e in 0..self.engines.len() {
            self.start_engine(e);
        }
        if self.applied != self.effective() {
            self.apply_quality();
        }
        self.active = self.wanted();
        self.prev = None;
        self.dip = 1.0;
        self.drive = self.drive_target;
        self.set_now = self.set_to;
    }

    fn set_param(&mut self, index: usize, value: f32) {
        let Some(d) = PARAMS.get(index) else { return };
        let v = value.clamp(d.min, d.max);
        match index {
            index::ALGORITHM => self.algo = (v.round() as usize).min(self.engines.len() - 1),
            index::DRIVE => self.drive_target = 10f32.powf(v / 20.0),
            index::CEILING => self.set_to[0] = 10f32.powf(v / 20.0),
            index::SHAPE => self.set_to[1] = v / 100.0,
            index::CHARACTER => self.set_to[2] = v / 100.0,
            index::AA => self.quality.0 = v.round() as usize,
            index::OS => self.quality.1 = v.round() as usize,
            _ => {}
        }
    }

    fn latency(&self) -> usize {
        self.pad_to()
    }

    fn process(&mut self, left: &mut [f32], right: &mut [f32]) {
        let n = left.len().min(right.len()).min(self.scratch[0].len());
        let (left, right) = (&mut left[..n], &mut right[..n]);
        if self.applied != self.effective() && self.dip <= 0.0 {
            self.apply_quality();
        }
        let mut pin = [0.0f32; 2];
        for i in 0..n {
            self.drive += (self.drive_target - self.drive) * self.smooth;
            left[i] *= self.drive;
            right[i] *= self.drive;
            pin[0] = pin[0].max(left[i].abs());
            pin[1] = pin[1].max(right[i].abs());
        }

        let want = self.wanted();
        if self.prev.is_none() && want != self.active {
            self.prev = Some(self.active);
            self.active = want;
            self.start_engine(want);
            self.fade = 0.0;
        }
        let glide = (0..3).any(|k| (self.set_to[k] - self.set_now[k]).abs() > 1e-5);

        if let Some(p) = self.prev {
            let [sl, sr] = &mut self.scratch;
            sl[..n].copy_from_slice(left);
            sr[..n].copy_from_slice(right);
            let (mut a, mut b) = (std::mem::take(&mut self.scratch[0]), std::mem::take(&mut self.scratch[1]));
            self.run_engine(p, &mut a[..n], &mut b[..n], glide);
            self.run_engine(self.active, left, right, glide);
            for i in 0..n {
                self.fade = (self.fade + self.fade_step).min(1.0);
                left[i] = a[i] + (left[i] - a[i]) * self.fade;
                right[i] = b[i] + (right[i] - b[i]) * self.fade;
            }
            self.scratch = [a, b];
            if self.fade >= 1.0 {
                self.prev = None;
            }
        } else {
            self.run_engine(self.active, left, right, glide);
        }
        if glide {
            let steps = n.div_ceil(CHUNK) as i32;
            let keep = (1.0 - self.chunk_smooth).powi(steps);
            for k in 0..3 {
                self.set_now[k] = self.set_to[k] + (self.set_now[k] - self.set_to[k]) * keep;
            }
        }
        // Quality changes dip the output while the engines are reconfigured.
        let dip_to = if self.applied != self.effective() { 0.0 } else { 1.0 };
        if self.dip != dip_to || self.dip < 1.0 {
            for i in 0..n {
                self.dip = if self.dip < dip_to { (self.dip + self.dip_step).min(1.0) } else { (self.dip - self.dip_step).max(dip_to) };
                left[i] *= self.dip;
                right[i] *= self.dip;
            }
        }

        let mut pout = [0.0f32; 2];
        for i in 0..n {
            pout[0] = pout[0].max(left[i].abs());
            pout[1] = pout[1].max(right[i].abs());
        }
        let db = |v: f32| if v > 1e-9 { 20.0 * v.log10() } else { -200.0 };
        let red = |c: usize| if pin[c] > 1e-6 { (db(pin[c]) - db(pout[c])).max(0.0) } else { 0.0 };
        let (rl, rr) = (red(0), red(1));
        self.meters = Meters { reduction_db: rl.max(rr), ..Meters::default() };
        self.view = [-rl, -rr, db(pin[0]), db(pin[1])];
    }

    fn meters(&self) -> Meters {
        self.meters
    }

    fn set_zero_latency(&mut self, on: bool) {
        if self.zero != on {
            self.zero = on;
            self.apply_quality();
        }
    }

    /// Slots: gain change dB (left, right; negative while clipping), driven input peak dB (left, right).
    fn view(&self, out: &mut [f32]) -> usize {
        let n = out.len().min(4);
        out[..n].copy_from_slice(&self.view[..n]);
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Magnet and Volt rise towards the ceiling: a sine 6 dB over a 0 dBFS ceiling comes out within
    /// 1.5 dB of it at every Shape (it used to level off 3 to 5 dB under), and never far above it.
    #[test]
    fn magnet_and_volt_reach_the_ceiling() {
        for name in ["Magnet", "Volt"] {
            for shape in [0.0f32, 0.5, 1.0] {
                let mut e = (ENGINES.iter().find(|e| e.0 == name).expect(name).1)();
                e.prepare(48_000.0, 512);
                e.set(1.0, shape, 0.0);
                let n = 48_000;
                let mut l: Vec<f32> = (0..n).map(|i| 2.0 * (std::f32::consts::TAU * 200.0 * i as f32 / 48_000.0).sin()).collect();
                let mut r = l.clone();
                for (a, b) in l.chunks_mut(512).zip(r.chunks_mut(512)) {
                    e.process(a, b);
                }
                let peak = l[n / 2..].iter().fold(0.0f32, |m, v| m.max(v.abs()));
                let db = 20.0 * peak.log10();
                assert!((-1.5..=0.3).contains(&db), "{name} shape {shape}: peak {db:.2} dB");
            }
        }
    }

    fn sine(n: usize, amp: f32) -> Vec<f32> {
        (0..n).map(|t| amp * (t as f32 * 0.05).sin()).collect()
    }

    fn with_engine(label: &str) -> Clipper {
        let mut c = Clipper::default();
        c.set_param(index::ALGORITHM, engine_index(label).expect(label) as f32);
        c
    }

    fn run(c: &mut Clipper, x: &[f32]) -> Vec<f32> {
        let (mut l, mut r) = (x.to_vec(), x.to_vec());
        for k in (0..x.len()).step_by(256) {
            let e = (k + 256).min(x.len());
            c.process(&mut l[k..e], &mut r[k..e]);
        }
        l
    }

    #[test]
    fn every_algorithm_at_every_quality_passes_the_contract() {
        for (_, make) in ENGINES.iter().chain(OUTPUT_ENGINES) {
            for (aa, os) in [(0, 0), (1, 0), (2, 0), (0, 1), (1, 3), (2, 5)] {
                let make = move || -> Box<dyn ClipEngine> {
                    let mut e = make();
                    e.set_quality(aa, os);
                    e
                };
                clip_api::bench::assert_contract(&make);
            }
        }
    }

    /// Ceiling-safe algorithms stay under the ceiling with no ADAA at every drive and corner (the
    /// ADAA methods may overshoot by design; the output ceiling catches that).
    #[test]
    fn clamped_algorithms_hold_the_ceiling() {
        let mut seed = 1u32;
        let mut signals: Vec<Vec<f32>> = [60.0f32, 1_000.0, 7_000.0, 15_000.0]
            .iter()
            .map(|hz| (0..9_600).map(|t| 0.5 * (std::f32::consts::TAU * hz * t as f32 / 48_000.0).sin()).collect())
            .collect();
        signals.push(
            (0..9_600)
                .map(|_| {
                    seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                    0.5 * ((seed >> 8) as f32 / (1u32 << 23) as f32 - 1.0)
                })
                .collect(),
        );
        let mut worst = Vec::new();
        for (label, make) in ENGINES.iter().filter(|e| ["Ember", "Magnet", "Volt"].contains(&e.0)) {
            for (shape, character) in [(0.0, 0.0), (1.0, 1.0), (0.5, 0.5)] {
                for drive_db in [0.0f32, 6.0, 12.0, 24.0] {
                    let g = 10f32.powf(drive_db / 20.0);
                    let mut e = make();
                    e.prepare(48_000.0, 512);
                    e.set_quality(0, 0);
                    e.set(0.25, shape, character);
                    for x in &signals {
                        e.reset();
                        let x: Vec<f32> = x.iter().map(|v| v * g).collect();
                        let y = clip_api::bench::run(e.as_mut(), &x, &[512]);
                        let peak = y.iter().fold(0.0f32, |m, v| m.max(v.abs())) / 0.25;
                        if peak > 1.0 + 1e-4 {
                            worst.push(format!("{label} {shape}/{character} +{drive_db} dB: {peak:.4}x"));
                        }
                    }
                }
            }
        }
        assert!(worst.is_empty(), "over the ceiling:\n{}", worst.join("\n"));
    }

    #[test]
    fn below_the_ceiling_it_matches_its_twin() {
        for (a, (label, _)) in ENGINES.iter().enumerate() {
            for (aa, os) in [(0.0, 0.0), (1.0, 2.0), (2.0, 1.0)] {
                let mk = |twin: bool| {
                    let mut c = Clipper::default();
                    c.set_param(index::ALGORITHM, a as f32);
                    c.set_param(index::AA, aa);
                    c.set_param(index::OS, os);
                    c.set_twin(twin);
                    c.prepare(48_000.0, 256);
                    c
                };
                let x = sine(4096, 0.05);
                let (p, q) = (run(&mut mk(false), &x), run(&mut mk(true), &x));
                let err = p.iter().zip(&q).skip(1024).fold(0.0f32, |m, (a, b)| m.max((a - b).abs()));
                assert!(err < 2e-4, "{label} aa {aa} os {os}: {err}");
            }
        }
    }

    #[test]
    fn quality_change_moves_latency_after_a_dip_without_clicks() {
        let mut c = with_engine("Flint");
        c.prepare(48_000.0, 256);
        let base = c.latency();
        c.set_param(index::OS, 3.0);
        let x = sine(256 * 40, 0.3);
        let (mut l, mut r) = (x.clone(), x.clone());
        let mut prev = 0.0f32;
        for k in (0..x.len()).step_by(256) {
            c.process(&mut l[k..k + 256], &mut r[k..k + 256]);
            for v in &l[k..k + 256] {
                assert!((v - prev).abs() < 0.1, "click");
                prev = *v;
            }
        }
        assert!(c.latency() > base, "{base} -> {}", c.latency());
        c.set_zero_latency(true);
        assert_eq!(c.latency(), 0);
    }

    #[test]
    fn drawn_curve_replaces_the_algorithm_curve() {
        let mut c = with_engine("Honey");
        c.set_param(index::AA, 2.0);
        c.set_param(index::CEILING, -6.0);
        c.prepare(48_000.0, 256);
        let pts = [0.4f32; CURVE_POINTS];
        c.set_custom(Some(&pts));
        let y = run(&mut c, &vec![1.5f32; 512]);
        assert!((y[511] - 0.4 * 10f32.powf(-6.0 / 20.0)).abs() < 1e-3, "{}", y[511]);
        c.set_custom(None);
        let y = run(&mut c, &vec![1.5f32; 512]);
        assert!(y[511] > 0.45, "{}", y[511]);
    }

    #[test]
    fn drawn_curve_takes_shape_and_character_out_of_play() {
        for (a, (label, _)) in ENGINES.iter().enumerate() {
            let run = |shape: f32, character: f32| {
                let mut c = Clipper::default();
                c.set_param(index::ALGORITHM, a as f32);
                c.set_param(index::SHAPE, shape);
                c.set_param(index::CHARACTER, character);
                c.prepare(48_000.0, 256);
                c.set_custom(Some(&[0.6; CURVE_POINTS]));
                run(&mut c, &sine(4096, 1.5))
            };
            let (p, q) = (run(0.0, 0.0), run(100.0, 100.0));
            let diff = p.iter().zip(&q).fold(0.0f32, |m, (a, b)| m.max((a - b).abs()));
            assert!(diff < 1e-6, "{label}: Shape/Character still act with a drawn curve ({diff})");
        }
    }

    #[test]
    fn moving_settings_and_algorithm_stay_bounded() {
        let mut c = Clipper::default();
        c.prepare(48_000.0, 256);
        let x = sine(256 * 200, 0.5);
        for k in 0..200 {
            c.set_param(index::ALGORITHM, (k / 20) as f32);
            c.set_param(index::CEILING, -((k % 13) as f32));
            c.set_param(index::DRIVE, (k % 25) as f32);
            let mut l = x[k * 256..(k + 1) * 256].to_vec();
            let mut r = l.clone();
            c.process(&mut l, &mut r);
            assert!(l.iter().all(|v| v.is_finite() && v.abs() <= 2.0));
        }
    }
}
