// Copyright (c) 2026 Marcos Quirino Deida
//! OpenClip Multiband: a 1-to-6-band Linkwitz-Riley multiband clipper built on OpenMultiband,
//! blending multiband clipping, single-band clipping and the dry signal in phase.
//!
//! ```text
//! in -> Input Gain -> Linkwitz-Riley Crossover (1..=6 bands, 12/24/48 dB/oct)
//!    -> Multi:  per band Drive -> Clipper -> Band Gain / Bypass / Solo / Mute -> sum
//!    -> Single: all-pass-matched full band -> Drive -> Clipper
//!    -> Dry:    all-pass-matched full band -> linear twin of the clipper
//!    -> Single <-> Multi blend -> Dry <-> Wet mix -> Output Ceiling -> Output Gain -> Delta -> Auto Gain -> out
//! ```
//!
//! One algorithm runs in every clipper. The three paths line up exactly: the crossover's matched
//! all-pass makes the full-band signal phase-identical to the band sum, every clipper has the same
//! padded latency, and the dry path runs through the same algorithm at -60 dB (its linear region),
//! so it carries the same small-signal phase as the clipped paths. Blending therefore never
//! comb-filters, and Delta nulls when nothing clips.

mod autogain;
pub mod crossover;
mod shared;
mod text;
mod ui;

use std::sync::{Arc, LazyLock};

use clip::Clipper;
use clip::{AA_LABELS, OS_LABELS};
use crossover::{Crossover, Slope, DEFAULT_FREQS, FREQ_MAX, FREQ_MIN, SLOPE_LABELS};
use olm_api::Module;
use openplugin::core::EditorState;
use openplugin::prelude::*;

pub use crossover::{MAX_BANDS, MAX_SPLITS};
pub use shared::Shared;
pub use ui::make_editor;

pub const BAND_COUNT_CHOICES: &[&str] = &["1", "2", "3", "4", "5", "6"];

// Global parameter indices.
pub const P_INPUT: usize = 0;
pub const P_OUTPUT: usize = 1;
pub const P_ZERO_LATENCY: usize = 2;
pub const P_AUTO_GAIN: usize = 3;
pub const P_DELTA: usize = 4;
pub const P_BAND_COUNT: usize = 5;
pub const P_XOVER_SLOPE: usize = 6;
pub const P_XOVER_BASE: usize = 7; // 5 crossover frequencies: 7..12
pub const P_ALGORITHM: usize = 12;
pub const P_SHAPE: usize = 13;
pub const P_CHARACTER: usize = 14;
/// Single (0 %) to Multi (100 %).
pub const P_BLEND: usize = 15;
/// Dry (0 %) to Wet (100 %).
pub const P_MIX: usize = 16;
pub const P_SINGLE_DRIVE: usize = 17;
pub const P_SINGLE_CEILING: usize = 18;
pub const P_OUT_CEILING_ON: usize = 19;
pub const P_OUT_CEILING: usize = 20;
pub const P_AA: usize = 21;
pub const P_OVERSAMPLING: usize = 22;
/// Editor only: the band columns are open.
pub const P_BANDS_OPEN: usize = 23;
/// The drawn curve replaces the algorithm's curve.
pub const P_CURVE_ON: usize = 24;
pub const P_CURVE_BASE: usize = 25; // CURVE_POINTS points: 25..41
pub const CURVE_POINTS: usize = clip::DRAWN_POINTS;
/// One set for all bands; each band's own Drive, Ceiling and Gain add on top.
pub const P_ALL_DRIVE: usize = P_CURVE_BASE + CURVE_POINTS;
pub const P_ALL_CEILING: usize = P_ALL_DRIVE + 1;
pub const P_ALL_GAIN: usize = P_ALL_DRIVE + 2;
pub const P_BAND_BASE: usize = P_ALL_DRIVE + 3;
const BAND_STRIDE: usize = 6;

/// Parameter indices for one band.
#[derive(Clone, Copy, Debug)]
pub struct BandInfo {
    pub drive: usize,
    pub ceiling: usize,
    pub gain: usize,
    pub solo: usize,
    pub mute: usize,
    pub bypass: usize,
}

pub const BANDS: [BandInfo; MAX_BANDS] = {
    let mut a = [BandInfo { drive: 0, ceiling: 0, gain: 0, solo: 0, mute: 0, bypass: 0 }; MAX_BANDS];
    let mut b = 0;
    while b < MAX_BANDS {
        let p = P_BAND_BASE + b * BAND_STRIDE;
        a[b] = BandInfo { drive: p, ceiling: p + 1, gain: p + 2, solo: p + 3, mute: p + 4, bypass: p + 5 };
        b += 1;
    }
    a
};

const DEFAULT_ENGINE: &str = "Honey";

fn leak(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

static PARAMS: LazyLock<Vec<ParamInfo>> = LazyLock::new(|| {
    let labels: &'static [&'static str] = Box::leak(clip::ENGINES.iter().map(|e| e.0).collect::<Vec<_>>().into_boxed_slice());
    let db = |id: &'static str, name: &'static str, min: f32, max: f32, default: f32, steps: u32, group: &'static str| {
        // Whole dB everywhere but ceilings, which keep a tenth; the same text for host and editor.
        let shown: fn(f32) -> String = if id.to_lowercase().ends_with("ceiling") { text::db_tenth } else { text::db };
        ParamInfo::new(id, name, min, max, default, Unit::Db).with_steps(steps).with_group(group).with_text(shown, text::parse_db)
    };
    let pct = |id: &'static str, name: &'static str, default: f32, group: &'static str| {
        ParamInfo::new(id, name, 0.0, 100.0, default, Unit::Percent).with_steps(101).with_group(group)
    };
    let toggle = |id: &'static str, name: &'static str, on: bool, group: &'static str| {
        ParamInfo::new(id, name, 0.0, 1.0, f32::from(u8::from(on)), Unit::Toggle).with_group(group)
    };
    let mut params = vec![
        db("input", "Input Gain", -24.0, 24.0, 0.0, 49, "Global"),
        db("output", "Output Gain", -24.0, 24.0, 0.0, 49, "Global"),
        toggle("zeroLatency", "Zero Latency", false, "Global"),
        toggle("autoGain", "Auto Gain", false, "Global"),
        toggle("delta", "Delta", false, "Global"),
        ParamInfo::new("bandCount", "Bands", 0.0, (MAX_BANDS - 1) as f32, 3.0, Unit::Choice)
            .with_choices(BAND_COUNT_CHOICES)
            .with_steps(MAX_BANDS as u32)
            .with_group("Crossover"),
        ParamInfo::new("xoverSlope", "Crossover Slope", 0.0, (SLOPE_LABELS.len() - 1) as f32, 1.0, Unit::Choice)
            .with_choices(SLOPE_LABELS)
            .with_steps(SLOPE_LABELS.len() as u32)
            .with_group("Crossover"),
    ];
    for (k, &def) in DEFAULT_FREQS.iter().enumerate() {
        let n = k + 1;
        params.push(
            ParamInfo::new(leak(format!("xover{n}")), leak(format!("Crossover {n}")), FREQ_MIN, FREQ_MAX, def, Unit::Hz)
                .with_taper(Taper::Log)
                .with_text(text::hz, text::parse_hz)
                .with_group("Crossover"),
        );
    }
    let default_engine = clip::engine_index(DEFAULT_ENGINE).unwrap_or(0) as f32;
    params.extend([
        ParamInfo::new("algorithm", "Algorithm", 0.0, (labels.len() - 1) as f32, default_engine, Unit::Choice)
            .with_choices(labels)
            .with_steps(labels.len() as u32)
            .with_group("Clipper"),
        // Honey's recommended knee (-2.2 dB) and its pure exponential curve.
        pct("shape", "Shape", 80.0, "Clipper"),
        pct("character", "Character", 0.0, "Clipper"),
        pct("blend", "Single/Multi Blend", 100.0, "Blend"),
        pct("mix", "Dry/Wet Mix", 100.0, "Blend"),
        db("singleDrive", "Single Drive", 0.0, 24.0, 0.0, 25, "Single"),
        // Tenths of a dB: a ceiling needs -0.3 or -1.0.
        db("singleCeiling", "Single Ceiling", -24.0, 0.0, 0.0, 241, "Single"),
        toggle("outCeilingOn", "Output Ceiling On", true, "Output"),
        // Headroom for later sample-rate conversion (Master 8x report).
        db("outCeiling", "Output Ceiling", -24.0, 0.0, -1.0, 241, "Output"),
        ParamInfo::new("aa", "Anti-Alias", 0.0, (AA_LABELS.len() - 1) as f32, 0.0, Unit::Choice)
            .with_choices(AA_LABELS)
            .with_steps(AA_LABELS.len() as u32)
            .with_group("Clipper"),
        ParamInfo::new("oversampling", "Oversampling", 0.0, (OS_LABELS.len() - 1) as f32, 0.0, Unit::Choice)
            .with_choices(OS_LABELS)
            .with_steps(OS_LABELS.len() as u32)
            .with_group("Clipper"),
        toggle("bandsOpen", "Bands Open", false, "Global"),
        toggle("curveDraw", "Drawn Curve", false, "Curve"),
    ]);
    for k in 1..=CURVE_POINTS {
        // Output at input 2k/16 of the ceiling, in percent of the ceiling. Default: a plain hard
        // clip, out = min(in, ceiling), in whole percent like every value the 1 % steps can hold
        // (points 1, 3, 5, 7 sit 0.5 % above the line), so a saved session reopens as it was.
        let at = 2.0 * k as f32 / CURVE_POINTS as f32;
        params.push(pct(leak(format!("curve{k}")), leak(format!("Curve Point {k}")), (100.0 * at.min(1.0)).round(), "Curve"));
    }
    params.extend([
        db("allDrive", "All Bands Drive", 0.0, 24.0, 0.0, 25, "All Bands"),
        db("allCeiling", "All Bands Ceiling", -24.0, 0.0, 0.0, 241, "All Bands"),
        db("allGain", "All Bands Gain", -24.0, 24.0, 0.0, 49, "All Bands"),
    ]);
    for b in 0..MAX_BANDS {
        let n = b + 1;
        let grp = leak(format!("Band {n}"));
        let id = |s: &str| leak(format!("b{n}.{s}"));
        let name = |s: &str| leak(format!("Band {n} {s}"));
        // Offsets from the All Bands set: push or pull the band by up to 24 dB either way.
        params.extend([
            db(id("drive"), name("Drive"), -24.0, 24.0, 0.0, 49, grp),
            db(id("ceiling"), name("Ceiling"), -24.0, 24.0, 0.0, 481, grp),
            db(id("gain"), name("Gain"), -24.0, 24.0, 0.0, 49, grp),
            toggle(id("solo"), name("Solo"), false, grp),
            toggle(id("mute"), name("Mute"), false, grp),
            toggle(id("bypass"), name("Bypass"), false, grp),
        ]);
    }
    params
});

pub static INFO: LazyLock<PluginInfo> = LazyLock::new(|| {
    PluginInfo::new("com.opengrid.openclipmultiband", "OpenClip Multiband", "OpenGrid", env!("CARGO_PKG_VERSION"), Category::Dynamics, &PARAMS)
        .with_url("https://github.com/MistaMin/OpenClipMultiband")
        .with_editor(true)
});

static LAYOUTS: &[IoConfig] = &[IoConfig::STEREO];

/// Max latency of a clipper plus the output ceiling (ms), with headroom.
const MAX_LATENCY_MS: f32 = 10.0;
const FADE_MS: f32 = 10.0;
/// Fade out and back in around topology switches (Zero Latency, band count, slope).
const DIP_MS: f32 = 5.0;
const DELTA_GLIDE_MS: f32 = 50.0;
/// The dry twin runs this far down, inside every engine's linear region.
const TWIN_SCALE: f32 = 1e-3;

#[derive(Default)]
struct Delay {
    buf: Vec<f32>,
    pos: usize,
}

impl Delay {
    fn alloc(&mut self, len: usize) {
        self.buf = vec![0.0; len.max(1)];
        self.pos = 0;
    }

    fn clear(&mut self) {
        self.buf.fill(0.0);
    }

    #[inline]
    fn push(&mut self, x: f32, d: usize) -> f32 {
        let n = self.buf.len();
        if n == 0 || d == 0 {
            return x;
        }
        self.buf[self.pos] = x;
        let r = self.buf[(self.pos + n - d.min(n - 1)) % n];
        self.pos = (self.pos + 1) % n;
        r
    }
}

const EMPTY_COL: [f32; 4] = [f32::INFINITY, f32::NEG_INFINITY, f32::INFINITY, f32::NEG_INFINITY];

/// The output ceiling's clipper (its own engine list).
fn clipper(algorithm: f32, shape: f32, character: f32) -> Clipper {
    let mut c = Clipper::with(clip::OUTPUT_ENGINES);
    c.set_param(clip::index::ALGORITHM, algorithm);
    c.set_param(clip::index::SHAPE, shape);
    c.set_param(clip::index::CHARACTER, character);
    c
}

struct BandState {
    clip: Clipper,
    gain: f32,
    gain_target: f32,
    solo: bool,
    mute: bool,
    bypass: bool,
    /// 0 = active, 1 = bypassed (10 ms crossfade).
    bypass_fade: f32,
    /// Smoothed audibility (0 = muted/non-soloed, 1 = audible).
    aud: f32,
    /// Clean crossover band delayed by the clipper latency for click-free Band Bypass.
    dry_band: [Delay; 2],
}

impl BandState {
    fn new() -> Self {
        Self {
            clip: Clipper::default(),
            gain: 1.0,
            gain_target: 1.0,
            solo: false,
            mute: false,
            bypass: false,
            bypass_fade: 0.0,
            aud: 1.0,
            dry_band: Default::default(),
        }
    }
}

pub struct OpenClipMultiband {
    values: Vec<f32>,
    crossover: Crossover,
    bands: [BandState; MAX_BANDS],
    single: Clipper,
    /// The dry path: the selected algorithm run in its linear region, for phase alignment.
    twin: Clipper,
    out_clip: Clipper,
    out_on: bool,
    out_fade: f32,
    out_dry: [Delay; 2],
    blend: f32,
    blend_target: f32,
    mix: f32,
    mix_target: f32,
    in_gain: f32,
    in_gain_target: f32,
    out_gain: f32,
    out_gain_target: f32,
    smooth: f32,
    fade_step: f32,
    sample_rate: f32,
    max_block: usize,
    // Pre-allocated scratch buffers (zero allocation on audio thread).
    band_l: [Vec<f32>; MAX_BANDS],
    band_r: [Vec<f32>; MAX_BANDS],
    band_dry: [Vec<f32>; 2],
    ref_raw: [Vec<f32>; 2],
    single_buf: [Vec<f32>; 2],
    dry_buf: [Vec<f32>; 2],
    pre: [Vec<f32>; 2],
    pre_delay: [Delay; 2],
    sum: [Vec<f32>; 2],
    mono: Vec<f32>,
    /// History: per lane (whole, then bands) the input and output mid signal of this block, and
    /// the column being gathered ([in min, in max, out min, out max]).
    hist_in: [Vec<f32>; shared::HIST_LANES],
    hist_out: [Vec<f32>; shared::HIST_LANES],
    hist_col: [[f32; 4]; shared::HIST_LANES],
    hist_fill: usize,
    hist_len: usize,
    view: [f32; 4],
    shared: Arc<Shared>,
    // Topology state (dips 5 ms when changed live).
    zero_want: bool,
    zero: bool,
    bands_want: usize,
    bands_active: usize,
    slope_want: Slope,
    slope_active: Slope,
    dip: f32,
    dip_step: f32,
    autogain: autogain::AutoGain,
    delta: bool,
    delta_mix: f32,
    delta_step: f32,
    silent_for: usize,
    silence_logged: bool,
}

fn log_line(msg: &str) {
    if cfg!(test) {
        return;
    }
    let path = if cfg!(target_os = "macos") {
        std::env::var_os("HOME").map(|h| std::path::PathBuf::from(h).join("Library/Logs/OpenClipMultiband/panic.log"))
    } else {
        ui::theme::pref_file("panic.log")
    };
    if let Some(path) = path {
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
            use std::io::Write;
            let _ = writeln!(f, "{} {msg}", env!("CARGO_PKG_VERSION"));
        }
    }
}

fn log_panics() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let prev = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            log_line(&format!("{info}"));
            prev(info);
        }));
    });
}

fn db_to_gain(db: f32) -> f32 {
    10f32.powf(db / 20.0)
}

fn lin_to_db(v: f32) -> f32 {
    if v > 1e-9 { 20.0 * v.log10() } else { -200.0 }
}

/// Pop-out windows of an instance read its shared state: they close before the instance goes.
impl Drop for OpenClipMultiband {
    fn drop(&mut self) {
        ui::pops::close_all(&self.shared);
    }
}

impl Default for OpenClipMultiband {
    fn default() -> Self {
        log_panics();
        let values: Vec<f32> = INFO.params.iter().map(|p| p.default).collect();
        let mut p = Self {
            values: values.clone(),
            crossover: Crossover::default(),
            bands: std::array::from_fn(|_| BandState::new()),
            single: Clipper::default(),
            twin: {
                let mut t = Clipper::default();
                t.set_twin(true);
                t
            },
            out_clip: clipper(0.0, 50.0, 50.0),
            out_on: true,
            out_fade: 1.0,
            out_dry: Default::default(),
            blend: 1.0,
            blend_target: 1.0,
            mix: 1.0,
            mix_target: 1.0,
            in_gain: 1.0,
            in_gain_target: 1.0,
            out_gain: 1.0,
            out_gain_target: 1.0,
            smooth: 1.0,
            fade_step: 1.0,
            sample_rate: 48_000.0,
            max_block: 0,
            band_l: std::array::from_fn(|_| Vec::new()),
            band_r: std::array::from_fn(|_| Vec::new()),
            band_dry: Default::default(),
            ref_raw: Default::default(),
            single_buf: Default::default(),
            dry_buf: Default::default(),
            pre: Default::default(),
            pre_delay: Default::default(),
            sum: Default::default(),
            mono: Vec::new(),
            hist_in: std::array::from_fn(|_| Vec::new()),
            hist_out: std::array::from_fn(|_| Vec::new()),
            hist_col: [EMPTY_COL; shared::HIST_LANES],
            hist_fill: 0,
            hist_len: 240,
            view: [0.0; 4],
            shared: Arc::new(Shared::default()),
            zero_want: false,
            zero: false,
            bands_want: 4,
            bands_active: 4,
            slope_want: Slope::Db24,
            slope_active: Slope::Db24,
            dip: 1.0,
            dip_step: 1.0,
            autogain: autogain::AutoGain::new(48_000.0, false),
            delta: false,
            delta_mix: 0.0,
            delta_step: 1.0,
            silent_for: 0,
            silence_logged: false,
        };
        for (i, &v) in values.iter().enumerate() {
            p.set_param_index(i, v);
        }
        p.apply_topology();
        for b in &mut p.bands {
            b.bypass_fade = if b.bypass { 1.0 } else { 0.0 };
        }
        p.out_fade = if p.out_on { 1.0 } else { 0.0 };
        p.blend = p.blend_target;
        p.mix = p.mix_target;
        p
    }
}

impl OpenClipMultiband {
    /// Every clipper that runs the selected algorithm.
    fn clippers(&mut self) -> impl Iterator<Item = &mut Clipper> {
        self.bands.iter_mut().map(|b| &mut b.clip).chain([&mut self.single, &mut self.twin])
    }

    fn topology_pending(&self) -> bool {
        self.zero != self.zero_want || self.bands_active != self.bands_want || self.slope_active != self.slope_want
    }

    fn apply_topology(&mut self) {
        self.zero = self.zero_want;
        self.bands_active = self.bands_want;
        self.slope_active = self.slope_want;
        self.crossover.set_topology(self.bands_active, self.slope_active);
        let zero = self.zero;
        for c in self.clippers() {
            c.set_zero_latency(zero);
            c.reset();
        }
        self.out_clip.set_zero_latency(zero);
        self.out_clip.reset();
        for b in &mut self.bands {
            b.dry_band[0].clear();
            b.dry_band[1].clear();
        }
        for d in self.out_dry.iter_mut().chain(self.pre_delay.iter_mut()) {
            d.clear();
        }
    }

    fn total_latency(&self) -> usize {
        self.twin.latency() + self.out_clip.latency()
    }

    fn run(&mut self, left: &mut [f32], right: &mut [f32]) {
        let n = left.len();
        if self.dip <= 0.0 && self.topology_pending() {
            self.apply_topology();
        }
        self.autogain.feed_raw(left, right);

        let mut peak_in = [0.0f32; 2];
        for i in 0..n {
            self.in_gain += (self.in_gain_target - self.in_gain) * self.smooth;
            left[i] *= self.in_gain;
            right[i] *= self.in_gain;
            peak_in[0] = peak_in[0].max(left[i].abs());
            peak_in[1] = peak_in[1].max(right[i].abs());
        }

        // Split into 1..=6 bands plus the matched all-pass full-band reference.
        let [ref_l, ref_r] = &mut self.ref_raw;
        self.crossover.split_block(left, right, &mut self.band_l, &mut self.band_r, &mut ref_l[..n], &mut ref_r[..n]);
        let lat = self.twin.latency();

        // Single and dry paths both start from the reference, so they share the band sum's phase.
        for c in 0..2 {
            self.single_buf[c][..n].copy_from_slice(&self.ref_raw[c][..n]);
            for (d, r) in self.dry_buf[c][..n].iter_mut().zip(&self.ref_raw[c][..n]) {
                *d = r * TWIN_SCALE;
            }
        }
        {
            let [sl, sr] = &mut self.single_buf;
            self.single.process(&mut sl[..n], &mut sr[..n]);
            let [dl, dr] = &mut self.dry_buf;
            self.twin.process(&mut dl[..n], &mut dr[..n]);
        }
        for v in self.dry_buf.iter_mut().flat_map(|c| c[..n].iter_mut()) {
            *v /= TWIN_SCALE;
        }
        let red = |m: &Clipper, view: &mut [f32; 4]| {
            let got = m.view(view);
            if got >= 4 { ([view[0], view[1]], [view[2], view[3]]) } else { ([0.0; 2], [-200.0; 2]) }
        };
        let (g, d) = red(&self.single, &mut self.view);
        self.shared.single(g, d, self.blend_target < 1.0 && self.mix_target > 0.0);

        // Multiband path.
        self.sum[0][..n].fill(0.0);
        self.sum[1][..n].fill(0.0);
        let active = self.bands_active;
        let any_solo = (0..active).any(|b| self.bands[b].solo);
        let step = self.fade_step;
        let smooth = self.smooth;
        for b in 0..MAX_BANDS {
            if b >= active {
                self.shared.band_clip(b, [0.0; 2], [-200.0; 2], false);
                self.shared.band_summary(b, 0.0, -200.0);
                self.hist_in[1 + b][..n].fill(0.0);
                self.hist_out[1 + b][..n].fill(0.0);
                continue;
            }
            let bl = &mut self.band_l[b][..n];
            let br = &mut self.band_r[b][..n];
            self.band_dry[0][..n].copy_from_slice(bl);
            self.band_dry[1][..n].copy_from_slice(br);
            let band = &mut self.bands[b];
            let aud_target = if any_solo {
                if band.solo { 1.0 } else { 0.0 }
            } else if band.mute {
                0.0
            } else {
                1.0
            };
            let skip = band.bypass && band.bypass_fade >= 1.0;
            let mut clip_db = 0.0;
            if skip {
                self.shared.band_clip(b, [0.0; 2], [-200.0; 2], false);
            } else {
                if !band.bypass && band.bypass_fade >= 1.0 {
                    band.clip.reset();
                }
                band.clip.process(bl, br);
                let (g, d) = red(&band.clip, &mut self.view);
                clip_db = g[0].min(g[1]);
                self.shared.band_clip(b, g, d, true);
            }
            let byp_target = if band.bypass { 1.0 } else { 0.0 };
            let mut band_peak = 0.0f32;
            let drive = db_to_gain(Self::band_settings(&self.values, b)[0]);
            let (hin, hout) = (&mut self.hist_in[1 + b], &mut self.hist_out[1 + b]);
            for i in 0..n {
                let dry_l = band.dry_band[0].push(self.band_dry[0][i], lat);
                let dry_r = band.dry_band[1].push(self.band_dry[1][i], lat);
                hin[i] = 0.5 * (dry_l + dry_r) * if skip { 1.0 } else { drive };
                hout[i] = if skip { hin[i] } else { 0.5 * (bl[i] + br[i]) };
                band.gain += (band.gain_target - band.gain) * smooth;
                band.aud += (aud_target - band.aud) * smooth;
                if band.bypass_fade != byp_target {
                    band.bypass_fade = if band.bypass_fade < byp_target { (band.bypass_fade + step).min(1.0) } else { (band.bypass_fade - step).max(0.0) };
                }
                let (wl, wr) = if skip {
                    (dry_l, dry_r)
                } else {
                    let (al, ar) = (bl[i] * band.gain, br[i] * band.gain);
                    (al + (dry_l - al) * band.bypass_fade, ar + (dry_r - ar) * band.bypass_fade)
                };
                let (ol, or) = (wl * band.aud, wr * band.aud);
                band_peak = band_peak.max(ol.abs()).max(or.abs());
                self.sum[0][i] += ol;
                self.sum[1][i] += or;
            }
            self.shared.band_summary(b, if skip { 0.0 } else { clip_db }, lin_to_db(band_peak));
        }

        // Single <-> Multi, then Dry <-> Wet.
        for i in 0..n {
            self.blend += (self.blend_target - self.blend) * smooth;
            self.mix += (self.mix_target - self.mix) * smooth;
            for (c, out) in [&mut *left, &mut *right].into_iter().enumerate() {
                let single = self.single_buf[c][i];
                let wet = single + (self.sum[c][i] - single) * self.blend;
                let dry = self.dry_buf[c][i];
                out[i] = dry + (wet - dry) * self.mix;
            }
        }

        // Output ceiling (latency kept when off, so switching it never moves the plugin's delay).
        let out_lat = self.out_clip.latency();
        self.sum[0][..n].copy_from_slice(left);
        self.sum[1][..n].copy_from_slice(right);
        if self.out_on || self.out_fade > 0.0 {
            if self.out_on && self.out_fade <= 0.0 {
                self.out_clip.reset();
            }
            self.out_clip.process(left, right);
        }
        let target = if self.out_on { 1.0 } else { 0.0 };
        for i in 0..n {
            let dl = self.out_dry[0].push(self.sum[0][i], out_lat);
            let dr = self.out_dry[1].push(self.sum[1][i], out_lat);
            if self.out_fade != target {
                self.out_fade = if self.out_fade < target { (self.out_fade + step).min(1.0) } else { (self.out_fade - step).max(0.0) };
            }
            if self.out_fade < 1.0 {
                left[i] = dl + (left[i] - dl) * self.out_fade;
                right[i] = dr + (right[i] - dr) * self.out_fade;
            }
            // Delta and spectrum reference: the dry twin, through the same output delay.
            self.pre[0][i] = self.pre_delay[0].push(self.dry_buf[0][i], out_lat);
            self.pre[1][i] = self.pre_delay[1].push(self.dry_buf[1][i], out_lat);
        }
        let (g, d) = red(&self.out_clip, &mut self.view);
        self.shared.output_clip(g, d, self.out_on);

        self.shared.push_taps([&self.pre[0][..n], &self.pre[1][..n]], [left, right]);
        for i in 0..n {
            self.hist_in[0][i] = 0.5 * (self.pre[0][i] + self.pre[1][i]);
            self.hist_out[0][i] = 0.5 * (left[i] + right[i]);
        }
        self.gather_history(n);

        let mut peak_out = [0.0f32; 2];
        let dip_to = if self.topology_pending() { 0.0 } else { 1.0 };
        let delta_to = if self.delta { 1.0 } else { 0.0 };
        for i in 0..n {
            self.out_gain += (self.out_gain_target - self.out_gain) * smooth;
            if self.dip != dip_to {
                self.dip = if self.dip < dip_to { (self.dip + self.dip_step).min(1.0) } else { (self.dip - self.dip_step).max(0.0) };
            }
            let g = self.out_gain * self.dip;
            left[i] *= g;
            right[i] *= g;
            peak_out[0] = peak_out[0].max(left[i].abs());
            peak_out[1] = peak_out[1].max(right[i].abs());
            // Keep g so Delta scales the reference identically, sample by sample.
            self.sum[0][i] = g;
        }
        self.shared.peaks(peak_in, peak_out);
        self.watch_silence(n, peak_in, peak_out);
        self.autogain.feed_out(left, right);

        if self.delta_mix != 0.0 || delta_to != 0.0 {
            for i in 0..n {
                if self.delta_mix != delta_to {
                    self.delta_mix = if self.delta_mix < delta_to { (self.delta_mix + self.delta_step).min(1.0) } else { (self.delta_mix - self.delta_step).max(0.0) };
                }
                let m = self.delta_mix * self.sum[0][i];
                left[i] -= m * self.pre[0][i];
                right[i] -= m * self.pre[1][i];
            }
        }
        self.autogain.apply(left, right);
        self.shared.set_auto_gain(self.autogain.gain_db());
    }

    /// Folds this block's lane signals into history columns and publishes each full column.
    fn gather_history(&mut self, n: usize) {
        for i in 0..n {
            for (lane, col) in self.hist_col.iter_mut().enumerate() {
                let (a, b) = (self.hist_in[lane][i], self.hist_out[lane][i]);
                *col = [col[0].min(a), col[1].max(a), col[2].min(b), col[3].max(b)];
            }
            self.hist_fill += 1;
            if self.hist_fill >= self.hist_len {
                self.shared.push_history(&self.hist_col);
                self.hist_col = [EMPTY_COL; shared::HIST_LANES];
                self.hist_fill = 0;
            }
        }
    }

    /// Band `b`'s Drive, Ceiling and Gain in force: the all-bands set plus the band's own offset
    /// (up to 48 dB of range per band; a ceiling never goes above 0 dBFS).
    pub fn band_settings(values: &[f32], b: usize) -> [f32; 3] {
        let bl = BANDS[b];
        [
            (values[P_ALL_DRIVE] + values[bl.drive]).clamp(-24.0, 48.0),
            (values[P_ALL_CEILING] + values[bl.ceiling]).clamp(-48.0, 0.0),
            (values[P_ALL_GAIN] + values[bl.gain]).clamp(-48.0, 48.0),
        ]
    }

    fn apply_band(&mut self, b: usize) {
        let [drive, ceiling, gain] = Self::band_settings(&self.values, b);
        let band = &mut self.bands[b];
        band.clip.set_param(clip::index::DRIVE, drive);
        band.clip.set_param(clip::index::CEILING, ceiling);
        band.gain_target = db_to_gain(gain);
    }

    /// Hands the drawn curve (or none) to every clipper that runs the selected algorithm.
    fn update_custom(&mut self) {
        let mut pts = [0.0f32; CURVE_POINTS];
        for (p, v) in pts.iter_mut().zip(&self.values[P_CURVE_BASE..P_CURVE_BASE + CURVE_POINTS]) {
            *p = v / 100.0;
        }
        let on = self.values[P_CURVE_ON] >= 0.5;
        for c in self.clippers() {
            c.set_custom(on.then_some(&pts));
        }
    }

    fn watch_silence(&mut self, n: usize, peak_in: [f32; 2], peak_out: [f32; 2]) {
        let all_muted = (0..self.bands_active).all(|b| self.bands[b].mute) && self.blend_target >= 1.0 && self.mix_target >= 1.0;
        if !all_muted && peak_in[0].max(peak_in[1]) > 1e-4 && peak_out[0].max(peak_out[1]) == 0.0 {
            self.silent_for += n;
        } else {
            self.silent_for = 0;
        }
        if self.silent_for > self.sample_rate as usize && !self.silence_logged {
            self.silence_logged = true;
            let msg = format!(
                "silent output with input: bands {} slope {:?} delta {} mix {} dip {} zero {} lat {} in {} out {} ag {} sr {} block {}",
                self.bands_active, self.slope_active, self.delta, self.delta_mix, self.dip, self.zero, self.total_latency(), self.values[P_INPUT], self.values[P_OUTPUT], self.autogain.gain_db(), self.sample_rate, self.max_block
            );
            log_line(&msg);
        }
    }
}

impl Plugin for OpenClipMultiband {
    fn state_is_parameters_only(&self) -> bool {
        true
    }

    fn info(&self) -> &'static PluginInfo {
        &INFO
    }

    fn prepare(&mut self, sample_rate: f32, max_block: usize, _channels: usize) {
        self.sample_rate = sample_rate.max(1.0);
        self.max_block = max_block.max(1);
        let sr = self.sample_rate;
        let block = self.max_block;
        let dry_len = (MAX_LATENCY_MS * 1e-3 * sr).ceil() as usize + 2;
        self.crossover.prepare(sr);
        for c in self.clippers() {
            c.prepare(sr, block);
        }
        self.out_clip.prepare(sr, block);
        for b in &mut self.bands {
            b.dry_band[0].alloc(dry_len);
            b.dry_band[1].alloc(dry_len);
            b.gain = b.gain_target;
            b.bypass_fade = if b.bypass { 1.0 } else { 0.0 };
        }
        for d in self.out_dry.iter_mut().chain(self.pre_delay.iter_mut()) {
            d.alloc(dry_len);
        }
        self.out_fade = if self.out_on { 1.0 } else { 0.0 };
        let buf = || vec![0.0f32; block];
        self.band_l = std::array::from_fn(|_| buf());
        self.band_r = std::array::from_fn(|_| buf());
        for v in [&mut self.band_dry, &mut self.ref_raw, &mut self.single_buf, &mut self.dry_buf, &mut self.pre, &mut self.sum] {
            *v = [buf(), buf()];
        }
        self.mono = buf();
        self.hist_in = std::array::from_fn(|_| buf());
        self.hist_out = std::array::from_fn(|_| buf());
        self.hist_len = (sr * shared::HIST_SECONDS_PER_COL).round().max(1.0) as usize;
        (self.hist_col, self.hist_fill) = ([EMPTY_COL; shared::HIST_LANES], 0);

        self.smooth = 1.0 - (-1.0 / (0.005 * sr)).exp();
        self.fade_step = 1.0 / (FADE_MS * 1e-3 * sr);
        self.dip_step = 1.0 / (DIP_MS * 1e-3 * sr);
        self.delta_step = 1.0 / (DELTA_GLIDE_MS * 1e-3 * sr);
        self.delta_mix = if self.delta { 1.0 } else { 0.0 };
        if self.topology_pending() {
            self.apply_topology();
        }
        let any_solo = (0..self.bands_active).any(|b| self.bands[b].solo);
        for b in &mut self.bands {
            b.aud = if any_solo {
                if b.solo { 1.0 } else { 0.0 }
            } else if b.mute {
                0.0
            } else {
                1.0
            };
        }
        self.dip = 1.0;
        self.autogain = autogain::AutoGain::new(sr, self.values[P_AUTO_GAIN] >= 0.5);
        self.in_gain = self.in_gain_target;
        self.out_gain = self.out_gain_target;
        self.blend = self.blend_target;
        self.mix = self.mix_target;
        self.shared.set_sample_rate(sr);
    }

    fn reset(&mut self) {
        self.crossover.reset();
        for c in self.clippers() {
            c.reset();
        }
        self.out_clip.reset();
        for b in &mut self.bands {
            b.dry_band[0].clear();
            b.dry_band[1].clear();
            b.gain = b.gain_target;
            b.bypass_fade = if b.bypass { 1.0 } else { 0.0 };
        }
        for d in self.out_dry.iter_mut().chain(self.pre_delay.iter_mut()) {
            d.clear();
        }
        self.out_fade = if self.out_on { 1.0 } else { 0.0 };
        self.in_gain = self.in_gain_target;
        self.out_gain = self.out_gain_target;
        self.blend = self.blend_target;
        self.mix = self.mix_target;
    }

    fn set_param(&mut self, id: &str, value: f32) -> bool {
        match INFO.params.iter().position(|p| p.id == id) {
            Some(i) => self.set_param_index(i, value),
            None => false,
        }
    }

    fn set_param_index(&mut self, index: usize, value: f32) -> bool {
        let Some(info) = INFO.params.get(index) else { return false };
        let value = info.clamp(value);
        self.values[index] = value;
        self.shared.set_param(index, value);
        let topology = |p: &mut Self| {
            if p.max_block == 0 {
                p.apply_topology();
            }
        };
        match index {
            P_INPUT => self.in_gain_target = db_to_gain(value),
            P_OUTPUT => self.out_gain_target = db_to_gain(value),
            P_ZERO_LATENCY => {
                self.zero_want = value >= 0.5;
                topology(self);
            }
            P_AUTO_GAIN => self.autogain.on = value >= 0.5,
            P_DELTA => self.delta = value >= 0.5,
            P_BAND_COUNT => {
                self.bands_want = (value.round() as usize + 1).clamp(1, MAX_BANDS);
                topology(self);
            }
            P_XOVER_SLOPE => {
                self.slope_want = Slope::from_param(value);
                topology(self);
            }
            i if (P_XOVER_BASE..P_XOVER_BASE + MAX_SPLITS).contains(&i) => self.crossover.set_freq(i - P_XOVER_BASE, value),
            P_ALGORITHM | P_SHAPE | P_CHARACTER | P_AA | P_OVERSAMPLING => {
                let local = match index {
                    P_ALGORITHM => clip::index::ALGORITHM,
                    P_SHAPE => clip::index::SHAPE,
                    P_CHARACTER => clip::index::CHARACTER,
                    P_AA => clip::index::AA,
                    _ => clip::index::OS,
                };
                for c in self.clippers() {
                    c.set_param(local, value);
                }
            }
            P_BLEND => self.blend_target = value / 100.0,
            P_MIX => self.mix_target = value / 100.0,
            P_SINGLE_DRIVE => self.single.set_param(clip::index::DRIVE, value),
            P_SINGLE_CEILING => self.single.set_param(clip::index::CEILING, value),
            P_OUT_CEILING_ON => self.out_on = value >= 0.5,
            P_OUT_CEILING => self.out_clip.set_param(clip::index::CEILING, value),
            P_BANDS_OPEN => {}
            P_ALL_DRIVE | P_ALL_CEILING | P_ALL_GAIN => {
                for b in 0..MAX_BANDS {
                    self.apply_band(b);
                }
            }
            i if i == P_CURVE_ON || (P_CURVE_BASE..P_CURVE_BASE + CURVE_POINTS).contains(&i) => self.update_custom(),
            i => {
                let Some(b) = i.checked_sub(P_BAND_BASE).map(|k| k / BAND_STRIDE).filter(|&b| b < MAX_BANDS) else { return false };
                let bl = BANDS[b];
                let band = &mut self.bands[b];
                match i {
                    _ if i == bl.solo => band.solo = value >= 0.5,
                    _ if i == bl.mute => band.mute = value >= 0.5,
                    _ if i == bl.bypass => band.bypass = value >= 0.5,
                    _ => self.apply_band(b),
                }
            }
        }
        true
    }

    fn param(&self, id: &str) -> Option<f32> {
        INFO.params.iter().position(|p| p.id == id).map(|i| self.values[i])
    }

    fn supported_layouts(&self) -> &'static [IoConfig] {
        LAYOUTS
    }

    fn process(&mut self, io: &mut [&mut [f32]], frames: usize) {
        if self.max_block == 0 {
            return;
        }
        let mut mono = std::mem::take(&mut self.mono);
        let Some((first, rest)) = io.split_first_mut() else { return };
        let mut start = 0;
        while start < frames {
            let n = (frames - start).min(self.max_block);
            let l = &mut first[start..start + n];
            match rest.first_mut() {
                Some(r) => self.run(l, &mut r[start..start + n]),
                None => {
                    let r = &mut mono[..n];
                    r.copy_from_slice(l);
                    self.run(l, r);
                }
            }
            start += n;
        }
        self.mono = mono;
    }

    fn latency(&self) -> usize {
        self.total_latency()
    }

    fn max_latency(&self) -> Option<usize> {
        Some((MAX_LATENCY_MS * 1e-3 * self.sample_rate).ceil() as usize)
    }

    fn meters(&self) -> MeterFrame {
        let mut m = MeterFrame::SILENT;
        let bands = (0..self.bands_active).filter(|&b| !self.bands[b].bypass && !self.bands[b].mute).map(|b| self.bands[b].clip.meters().reduction_db);
        let out = if self.out_on { self.out_clip.meters().reduction_db } else { 0.0 };
        m.reduction_db = bands.chain([self.single.meters().reduction_db, out]).fold(0.0f32, f32::max);
        m
    }

    fn editor_state(&self) -> Option<EditorState> {
        Some(self.shared.clone())
    }

    fn note_ports(&self) -> NotePorts {
        NotePorts { input: false, output: false }
    }

    fn lv2_uri(&self) -> Option<&'static str> {
        Some("https://opengrid.studio/plugins/openclipmultiband")
    }

    fn aax_ids(&self) -> Option<openplugin::core::AaxIds> {
        Some(openplugin::core::AaxIds { manufacturer: *b"OpGd", product: *b"OCMb", mono: *b"OCbM", stereo: *b"OCbS" })
    }
}

openplugin::targets::export_clap!(OpenClipMultiband, editor = make_editor);
openplugin::targets::export_lv2!(OpenClipMultiband, editor = make_editor);
openplugin::targets::export_shell!(OpenClipMultiband, editor = make_editor);

#[cfg(test)]
mod tests {
    use super::*;

    fn block(p: &mut OpenClipMultiband, l: &mut [f32], r: &mut [f32]) {
        let n = l.len();
        let mut io: [&mut [f32]; 2] = [l, r];
        p.process(&mut io, n);
    }

    fn engine(label: &str) -> f32 {
        clip::engine_index(label).expect(label) as f32
    }

    fn tones(n: usize, amp: f32) -> Vec<f32> {
        (0..n).map(|i| amp * (0.4 * (i as f32 * 0.013).sin() + 0.35 * (i as f32 * 0.17).sin() + 0.25 * (i as f32 * 1.2).sin())).collect()
    }

    fn peak(x: &[f32]) -> f32 {
        x.iter().fold(0.0f32, |m, v| m.max(v.abs()))
    }

    #[test]
    fn params_are_unique_and_layout_is_complete() {
        let ids: std::collections::HashSet<_> = INFO.params.iter().map(|p| p.id).collect();
        assert_eq!(ids.len(), INFO.params.len());
        // AAX truncates parameter display names to 31 chars: keep every name unique within 31 chars.
        let mut names31 = std::collections::HashSet::new();
        for p in INFO.params {
            let short: String = p.name.chars().take(31).collect();
            assert!(names31.insert(short.clone()), "duplicate 31-char parameter name: {short:?} ({})", p.id);
        }
        assert_eq!(BANDS[MAX_BANDS - 1].bypass + 1, INFO.params.len());
        assert_eq!(INFO.params[BANDS[2].ceiling].id, "b3.ceiling");
        assert_eq!(INFO.params[P_OUT_CEILING].id, "outCeiling");
    }

    #[test]
    fn db_controls_move_in_whole_steps_and_ceilings_in_tenths() {
        for p in INFO.params.iter().filter(|p| matches!(p.unit, Unit::Db)) {
            assert!(p.steps > 1, "{} is not stepped", p.id);
            let unit = if p.id.to_lowercase().ends_with("ceiling") { 0.1 } else { 1.0 };
            let on_grid = |v: f32| ((v / unit) - (v / unit).round()).abs() < 1e-3;
            assert!(on_grid(p.default), "{} default {}", p.id, p.default);
            for v in [p.min + 0.37, (p.min + p.max) * 0.5 + 0.21, p.max - 0.04] {
                let c = p.clamp(v);
                assert!(on_grid(c), "{}: {v} -> {c}", p.id);
            }
        }
    }

    #[test]
    fn defaults_resolve() {
        let p = OpenClipMultiband::default();
        assert_eq!(p.param("algorithm"), Some(engine(DEFAULT_ENGINE)));
        assert_eq!(p.param("aa"), Some(0.0));
        assert_eq!(p.param("oversampling"), Some(0.0));
        assert_eq!(p.param("bandsOpen"), Some(0.0));
        assert_eq!(p.param("curve16"), Some(100.0));
        assert_eq!(p.param("curve4"), Some(50.0));
        // Every curve point default is on the parameter's 1 % grid (F8: off-grid defaults moved on reopen).
        let points: Vec<f32> = (1..=CURVE_POINTS).map(|k| p.param(&format!("curve{k}")).unwrap()).collect();
        assert_eq!(&points[..8], &[13.0, 25.0, 38.0, 50.0, 63.0, 75.0, 88.0, 100.0]);
        assert!(points.iter().all(|v| v.fract() == 0.0));
        assert!(p.param("outCeiling").is_some_and(|c| (c + 1.0).abs() < 1e-4));
        assert_eq!(p.param("blend"), Some(100.0));
        assert_eq!(p.param("mix"), Some(100.0));
    }

    /// The core promise: below the ceiling, single, multi and dry are the same signal, so any blend
    /// of them nulls against the dry reference in Delta, for every algorithm, band count and slope.
    #[test]
    fn all_three_paths_are_phase_aligned_for_every_algorithm() {
        let mut worst = Vec::new();
        for (a, (label, _)) in clip::ENGINES.iter().enumerate() {
            for (bands, slope) in [(3usize, 0.0f32), (5, 1.0), (6, 2.0)] {
                let mut p = OpenClipMultiband::default();
                p.set_param_index(P_ALGORITHM, a as f32);
                p.set_param("bandCount", (bands - 1) as f32);
                p.set_param("xoverSlope", slope);
                p.set_param("blend", 50.0);
                p.set_param("mix", 50.0);
                p.set_param("outCeilingOn", 0.0);
                p.set_param("delta", 1.0);
                p.prepare(48_000.0, 256, 2);
                let src = tones(6000, 0.01);
                let (mut l, mut r) = (src.clone(), src.clone());
                block(&mut p, &mut l, &mut r);
                let res = peak(&l[3000..]);
                if res > 1e-4 {
                    worst.push(format!("{label} bands {bands} slope {slope}: residual {res:.2e} (signal 1e-2)"));
                }
            }
        }
        assert!(worst.is_empty(), "paths not aligned:\n{}", worst.join("\n"));
    }

    #[test]
    fn mix_zero_with_output_ceiling_off_is_a_delayed_wire() {
        let mut p = OpenClipMultiband::default();
        p.set_param_index(P_ALGORITHM, engine("Flint"));
        p.set_param("bandCount", 0.0);
        p.set_param("mix", 0.0);
        p.set_param("outCeilingOn", 0.0);
        p.prepare(48_000.0, 256, 2);
        let src = tones(2048, 0.5);
        let (mut l, mut r) = (src.clone(), src.clone());
        block(&mut p, &mut l, &mut r);
        let lat = p.latency();
        assert!(lat < 1000, "{lat}");
        let err = l[lat..].iter().zip(&src).fold(0.0f32, |m, (a, b)| m.max((a - b).abs()));
        assert!(err < 1e-6, "{err}");
    }

    #[test]
    fn blend_picks_single_or_multi_and_the_output_ceiling_holds() {
        let run = |setup: &dyn Fn(&mut OpenClipMultiband)| {
            let mut p = OpenClipMultiband::default();
            p.set_param("outCeilingOn", 0.0);
            setup(&mut p);
            p.prepare(48_000.0, 256, 2);
            let src = tones(8192, 1.5);
            let (mut l, mut r) = (src.clone(), src.clone());
            block(&mut p, &mut l, &mut r);
            peak(&l[2048..])
        };
        // All single at -12 dB: the single clipper alone sets the peak.
        // No ADAA: the curve itself never passes the ceiling (the ADAA methods may overshoot it).
        let single = run(&|p| {
            p.set_param("aa", 2.0);
            p.set_param("blend", 0.0);
            p.set_param("singleCeiling", -12.0);
        });
        assert!(single <= db_to_gain(-12.0) + 1e-4, "{single}");
        // All multi with every band at -12 dB: the bands clip, the single setting is ignored.
        let multi = run(&|p| {
            p.set_param("aa", 2.0);
            p.set_param("singleCeiling", -24.0);
            for b in &BANDS {
                p.set_param_index(b.ceiling, -12.0);
            }
        });
        assert!(multi > db_to_gain(-12.0) && multi < 4.0 * db_to_gain(-12.0), "{multi}");
        // Half dry, then the output ceiling catches what the dry signal brings back.
        let capped = run(&|p| {
            p.set_param("mix", 50.0);
            p.set_param("outCeilingOn", 1.0);
            p.set_param("outCeiling", -3.0);
        });
        assert!(capped <= db_to_gain(-3.0) + 1e-3, "{capped}");
    }

    #[test]
    fn solo_mute_bypass_and_band_gain_act_independently() {
        let sr = 48_000.0;
        let run_sine = |freq: f32, setup: &dyn Fn(&mut OpenClipMultiband)| -> f32 {
            let mut p = OpenClipMultiband::default();
            p.set_param("outCeilingOn", 0.0);
            setup(&mut p);
            p.prepare(sr, 256, 2);
            let n = 8192;
            let mut l: Vec<f32> = (0..n).map(|i| 0.4 * (2.0 * std::f32::consts::PI * freq * i as f32 / sr).sin()).collect();
            let mut r = l.clone();
            block(&mut p, &mut l, &mut r);
            (l[n / 2..].iter().map(|v| v * v).sum::<f32>() / (n / 2) as f32).sqrt()
        };
        // Solo Band 1 (20..120 Hz): 60 Hz passes, 2 kHz is silenced.
        let low_solo = run_sine(60.0, &|p| {
            p.set_param("b1.solo", 1.0);
        });
        let hi_solo = run_sine(2000.0, &|p| {
            p.set_param("b1.solo", 1.0);
        });
        assert!(low_solo > 0.25 && hi_solo < 0.005, "solo: low={low_solo} hi={hi_solo}");
        // Mute Band 1: 60 Hz is an octave under the 120 Hz split, so only the LR4 skirt remains.
        let low_mute = run_sine(60.0, &|p| {
            p.set_param("b1.mute", 1.0);
        });
        let hi_mute = run_sine(2000.0, &|p| {
            p.set_param("b1.mute", 1.0);
        });
        assert!(low_mute < 0.02 && hi_mute > 0.25, "mute: low={low_mute} hi={hi_mute}");
        // Band 4 (3k..20k) gain +6 dB lifts 12 kHz by +6 dB without touching 60 Hz.
        let base_hi = run_sine(12_000.0, &|_| {});
        let boosted_hi = run_sine(12_000.0, &|p| {
            p.set_param("b4.gain", 6.0);
        });
        let base_low = run_sine(60.0, &|_| {});
        let boosted_low = run_sine(60.0, &|p| {
            p.set_param("b4.gain", 6.0);
        });
        assert!((boosted_hi / base_hi - db_to_gain(6.0)).abs() < 0.02, "hi ratio {}", boosted_hi / base_hi);
        assert!((boosted_low / base_low - 1.0).abs() < 0.02, "low ratio {}", boosted_low / base_low);
        // Band Bypass skips Band 1's clipper.
        let clipped = run_sine(60.0, &|p| {
            p.set_param("b1.ceiling", -24.0);
        });
        let bypassed = run_sine(60.0, &|p| {
            p.set_param("b1.ceiling", -24.0);
            p.set_param("b1.bypass", 1.0);
        });
        assert!(clipped < base_low * 0.5 && (bypassed / base_low - 1.0).abs() < 0.02, "clip={clipped} byp={bypassed} base={base_low}");
    }

    #[test]
    fn zero_latency_reports_and_is_zero() {
        let mut p = OpenClipMultiband::default();
        p.prepare(48_000.0, 256, 2);
        let normal = p.latency();
        assert!(normal > 0);
        p.set_param("zeroLatency", 1.0);
        for _ in 0..4 {
            block(&mut p, &mut vec![0.0; 256], &mut vec![0.0; 256]);
        }
        assert_eq!(p.latency(), 0);
        p.set_param("zeroLatency", 0.0);
        for _ in 0..4 {
            block(&mut p, &mut vec![0.0; 256], &mut vec![0.0; 256]);
        }
        assert_eq!(p.latency(), normal);
    }

    #[test]
    fn switches_do_not_click() {
        let mut p = OpenClipMultiband::default();
        p.prepare(48_000.0, 64, 2);
        let mut prev = 0.0f32;
        let mut worst = 0.0f32;
        for b in 0..400 {
            match b {
                80 => p.set_param("zeroLatency", 1.0),
                160 => p.set_param("bandCount", 5.0),
                240 => p.set_param("blend", 0.0),
                280 => p.set_param("mix", 0.0),
                320 => p.set_param_index(P_ALGORITHM, engine("Ember")),
                360 => p.set_param("outCeilingOn", 0.0),
                _ => true,
            };
            let mut l: Vec<f32> = (0..64).map(|i| ((b * 64 + i) as f32 * 0.02).sin() * 0.4).collect();
            let mut r = l.clone();
            block(&mut p, &mut l, &mut r);
            for v in l {
                worst = worst.max((v - prev).abs());
                prev = v;
            }
        }
        assert_eq!(p.latency(), 0);
        assert_eq!(p.bands_active, 6);
        assert!(worst < 0.03, "click step {worst}");
    }

    #[test]
    fn history_shows_each_band_before_and_after_its_clipper() {
        let mut p = OpenClipMultiband::default();
        p.set_param("b1.ceiling", -12.0);
        p.prepare(48_000.0, 256, 2);
        let sr = 48_000.0;
        let n = 48_000;
        let mut l: Vec<f32> = (0..n).map(|i| 0.6 * (2.0 * std::f32::consts::PI * 60.0 * i as f32 / sr).sin()).collect();
        let mut r = l.clone();
        block(&mut p, &mut l, &mut r);
        let cols = p.shared.read_history(50);
        let last = cols.last().expect("history");
        // Band 1 (60 Hz) is clipped to -12 dB: its output range sits under its input range.
        let (inp, out) = (last[1][1] - last[1][0], last[1][3] - last[1][2]);
        assert!(inp > 0.9 && out < 2.0 * db_to_gain(-12.0) + 0.02, "band 1 in {inp} out {out}");
        // The whole lane: dry input against the final output.
        assert!(last[0][1] > 0.5 && last[0][3] > 0.1, "{:?}", last[0]);
    }

    #[test]
    fn all_bands_set_adds_to_each_band() {
        let mut p = OpenClipMultiband::default();
        p.set_param("allDrive", 6.0);
        p.set_param("b2.drive", 4.0);
        p.set_param("allCeiling", -20.0);
        p.set_param("b2.ceiling", -10.0);
        p.set_param("allGain", 3.0);
        assert_eq!(OpenClipMultiband::band_settings(&p.values, 0), [6.0, -20.0, 3.0]);
        assert_eq!(OpenClipMultiband::band_settings(&p.values, 1), [10.0, -30.0, 3.0]);
        // A band pulled 24 dB under an All Bands set at 0 dB: no drive into its clipper at -24 dB.
        p.set_param("allDrive", 0.0);
        p.set_param("b3.drive", -24.0);
        p.set_param("b3.ceiling", 24.0);
        assert_eq!(OpenClipMultiband::band_settings(&p.values, 2), [-24.0, 0.0, 3.0], "ceiling stops at 0 dBFS");
        // The one set clips the band: a 60 Hz tone comes out at the all-bands ceiling.
        let mut q = OpenClipMultiband::default();
        q.set_param("outCeilingOn", 0.0);
        q.set_param("aa", 2.0);
        q.set_param("allCeiling", -12.0);
        q.set_param("bandCount", 0.0);
        q.prepare(48_000.0, 256, 2);
        let n = 9600;
        let mut l: Vec<f32> = (0..n).map(|i| 0.9 * (2.0 * std::f32::consts::PI * 60.0 * i as f32 / 48_000.0).sin()).collect();
        let mut r = l.clone();
        block(&mut q, &mut l, &mut r);
        let pk = peak(&l[4800..]);
        assert!(pk <= db_to_gain(-12.0) + 1e-4, "{pk}");
    }

    #[test]
    fn drawn_curve_reaches_every_clipper() {
        let run = |on: bool| {
            let mut p = OpenClipMultiband::default();
            p.set_param("outCeilingOn", 0.0);
            p.set_param("aa", 2.0);
            p.set_param("bandCount", 0.0);
            for k in 1..=CURVE_POINTS {
                p.set_param(&format!("curve{k}"), 25.0);
            }
            p.set_param("curveDraw", if on { 1.0 } else { 0.0 });
            p.prepare(48_000.0, 256, 2);
            let src = tones(4096, 0.8);
            let (mut l, mut r) = (src.clone(), src.clone());
            block(&mut p, &mut l, &mut r);
            peak(&l[2048..])
        };
        assert!(run(true) < 0.26 && run(false) > 0.5, "{} {}", run(true), run(false));
    }

    #[test]
    fn spectrum_taps_line_up_at_unity() {
        let mut p = OpenClipMultiband::default();
        p.set_param("outCeilingOn", 0.0);
        p.prepare(48_000.0, 512, 2);
        for b in 0..8 {
            let mut l: Vec<f32> = (0..512).map(|i| ((b * 512 + i) as f32 * 0.03).sin() * 0.5).collect();
            let mut r = l.clone();
            block(&mut p, &mut l, &mut r);
        }
        let (mut pre, mut post) = (vec![0.0f32; 2048], vec![0.0f32; 2048]);
        p.shared.read_tap(0, &mut pre);
        p.shared.read_tap(2, &mut post);
        assert!(post.iter().any(|v| *v != 0.0));
        let max_err = pre.iter().zip(&post).map(|(a, b)| (a - b).abs()).fold(0.0f32, f32::max);
        assert!(max_err < 1e-5, "tap alignment error {max_err}");
    }

    /// Every control's text at its minimum, middle and maximum (knob display rules: at most three
    /// digits, whole dB except ceilings, no 0.x values).
    #[test]
    fn display_texts_follow_the_rules() {
        let mut bad = Vec::new();
        for p in INFO.params.iter().filter(|p| !p.group.starts_with("Band ") || p.group == "Band 1") {
            if matches!(p.unit, Unit::Toggle | Unit::Choice) {
                continue;
            }
            for v in [p.min, p.from_normalized(0.5), p.max, p.default] {
                let text = p.format(v);
                let digits = text.chars().filter(|c| c.is_ascii_digit()).count();
                let fraction = text.trim_start_matches('-').starts_with("0.") && !p.id.to_lowercase().ends_with("ceiling");
                if digits > 3 || fraction || text.starts_with("-0") && !text.starts_with("-0.") {
                    bad.push(format!("{}: {v} -> {text:?}", p.id));
                }
            }
        }
        assert!(bad.is_empty(), "{}", bad.join("\n"));
    }

    /// A render repeats bit for bit after a reset at any block size, and a host re-sending every
    /// value mid-render (as the VST3 shell does after main-thread writes) changes nothing audible.
    #[test]
    fn renders_repeat_at_any_block_size() {
        let n = 48_000;
        let mut seed = 9u32;
        let all: Vec<f32> = (0..n)
            .map(|i| {
                seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                let burst = if (i / 6000) % 2 == 1 { 0.45 * ((seed >> 8) as f32 / (1u32 << 23) as f32 - 1.0) } else { 0.0 };
                burst + 0.5 * (i as f32 * 0.02).sin()
            })
            .collect();
        let mut p = OpenClipMultiband::default();
        p.set_param("allDrive", 6.0);
        p.prepare(48_000.0, 1024, 2);
        let render = |p: &mut OpenClipMultiband, blocks: &[usize], resend_at: Option<usize>| {
            p.reset();
            let (mut l, mut r) = (all.clone(), all.clone());
            let (mut at, mut k) = (0, 0);
            while at < n {
                if resend_at == Some(k) {
                    for (i, info) in INFO.params.iter().enumerate() {
                        let norm = info.to_normalized(p.values[i]) as f32;
                        p.set_param_index(i, info.from_normalized(norm));
                    }
                }
                let len = blocks[k % blocks.len()].min(n - at);
                block(p, &mut l[at..at + len], &mut r[at..at + len]);
                at += len;
                k += 1;
            }
            l
        };
        let first = render(&mut p, &[1024], None);
        let worst = |out: &[f32]| out.iter().zip(&first).map(|(a, b)| (a - b).abs()).fold(0.0f32, f32::max);
        for blocks in [vec![1024], vec![512], vec![64], vec![1000, 24, 333, 1024]] {
            assert_eq!(worst(&render(&mut p, &blocks, None)), 0.0, "{blocks:?}");
        }
        let resent = worst(&render(&mut p, &[1024], Some(20)));
        assert!(resent < 1e-5, "re-sent values moved the output by {resent}");
    }

    /// The latency a host reads right after activation is the one the first block runs with, so a
    /// fresh engine never announces a change (the SDK reports any move it sees in a block).
    #[test]
    fn latency_is_final_before_the_first_block() {
        for (rate, aa, os, zero) in [(48_000.0, 0.0, 0.0, 0.0), (44_100.0, 1.0, 2.0, 0.0), (96_000.0, 2.0, 1.0, 0.0), (48_000.0, 0.0, 3.0, 1.0)] {
            let mut p = OpenClipMultiband::default();
            p.set_param("aa", aa);
            p.set_param("oversampling", os);
            p.set_param("zeroLatency", zero);
            p.prepare(rate, 256, 2);
            let at_prepare = p.latency();
            p.reset();
            let at_reset = p.latency();
            let (mut l, mut r) = (vec![0.1f32; 256], vec![0.1f32; 256]);
            block(&mut p, &mut l, &mut r);
            assert_eq!((at_prepare, at_reset), (p.latency(), p.latency()), "rate {rate} aa {aa} os {os} zero {zero}");
        }
    }

    /// Writes docs/PARAMETERS.md for the manual.
    #[test]
    #[ignore]
    fn dump_manual_params() {
        let total = INFO.params.len();
        let per_band = BAND_STRIDE;
        let mut out = format!(
            "# OpenClip Multiband parameter reference\n\n\
             Generated from the plugin (`cargo test --release --lib dump_manual_params -- --ignored`). \
             OpenClip Multiband exposes **{total} host parameters**: {} global and {per_band} per band across {MAX_BANDS} bands. \
             Bands 2 through {MAX_BANDS} use the same names, ranges and defaults as Band 1.\n",
            P_BAND_BASE
        );
        let mut group = "";
        for p in INFO.params.iter() {
            if p.group.starts_with("Band ") && p.group != "Band 1" {
                continue;
            }
            if p.group != group {
                group = p.group;
                let heading = if group == "Band 1" { "Per band (Band 1 shown; identical for Bands 2 to 6)".to_string() } else { group.to_string() };
                out += &format!("\n## {heading}\n\n| Parameter | Range or choices | Default |\n|---|---|---|\n");
            }
            let (range, default) = match p.unit {
                Unit::Toggle => ("Off, On".to_string(), if p.default >= 0.5 { "On" } else { "Off" }.to_string()),
                Unit::Choice => (p.choices.join(", "), p.choices.get(p.default as usize).copied().unwrap_or("").to_string()),
                _ => (format!("{} to {}", p.format(p.min), p.format(p.max)), p.format(p.default)),
            };
            out += &format!("| {} | {range} | {default} |\n", p.name);
        }
        std::fs::create_dir_all(concat!(env!("CARGO_MANIFEST_DIR"), "/docs")).expect("docs dir");
        std::fs::write(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/PARAMETERS.md"), out).expect("write");
    }

    /// DSP cost on 64-sample blocks.
    #[test]
    #[ignore]
    fn dsp_profile() {
        for (name, bands, algorithm, aa, os) in [
            ("4 bands, Honey, B1 1x", 4, "Honey", 0.0, 0.0),
            ("6 bands, Ember, B1 1x", 6, "Ember", 0.0, 0.0),
            ("6 bands, Honey, B1 4x", 6, "Honey", 0.0, 1.0),
            ("6 bands, Honey, B1 16x", 6, "Honey", 0.0, 3.0),
            ("6 bands, Honey, B1 64x", 6, "Honey", 0.0, 5.0),
            ("6 bands, Flint, none 64x", 6, "Flint", 2.0, 5.0),
        ] {
            let mut p = OpenClipMultiband::default();
            p.set_param("bandCount", (bands - 1) as f32);
            p.set_param_index(P_ALGORITHM, engine(algorithm));
            p.set_param_index(P_AA, aa);
            p.set_param_index(P_OVERSAMPLING, os);
            p.set_param("blend", 50.0);
            p.prepare(48_000.0, 64, 2);
            let (mut l, mut r) = (vec![0.0f32; 64], vec![0.0f32; 64]);
            let blocks = 4000;
            let t0 = std::time::Instant::now();
            for b in 0..blocks {
                for i in 0..64 {
                    let s = ((b * 64 + i) as f32 * 0.04).sin() * 0.9;
                    l[i] = s;
                    r[i] = s * 0.9;
                }
                block(&mut p, &mut l, &mut r);
            }
            let us = t0.elapsed().as_secs_f64() * 1e6 / blocks as f64;
            let budget = 64.0 / 48_000.0 * 1e6;
            println!("{name:28}: {us:7.1} us/block ({:5.1}% of one core)", 100.0 * us / budget);
        }
    }
}
