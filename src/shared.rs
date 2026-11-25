//! Lock-free engine-to-editor telemetry: per-band, single and output clip amounts, band summaries,
//! input/output peaks, auto-gain and aligned spectrum taps.

use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering, Ordering::Relaxed};

use crate::MAX_BANDS;

struct F(AtomicU32);

impl Default for F {
    fn default() -> Self {
        Self(AtomicU32::new(f32::NAN.to_bits()))
    }
}

impl F {
    fn get(&self) -> f32 {
        f32::from_bits(self.0.load(Relaxed))
    }
    fn set(&self, v: f32) {
        self.0.store(v.to_bits(), Relaxed);
    }
    fn merge(&self, v: f32, keep: impl Fn(f32, f32) -> bool) {
        let old = self.get();
        if old.is_nan() || keep(v, old) {
            self.set(v);
        }
    }
    fn take(&self, empty: f32) -> f32 {
        let v = f32::from_bits(self.0.swap(f32::NAN.to_bits(), Relaxed));
        if v.is_nan() { empty } else { v }
    }
}

#[derive(Default)]
struct SectionLive {
    gain: [F; 2],
    level: [F; 2],
    active: AtomicBool,
}

impl SectionLive {
    fn merge(&self, gain: [f32; 2], level: [f32; 2], active: bool) {
        for c in 0..2 {
            self.gain[c].merge(gain[c], |v, old| v.abs() > old.abs());
            self.level[c].merge(level[c], |v, old| v > old);
        }
        self.active.store(active, Relaxed);
    }

    fn take(&self) -> Live {
        Live { gain: [self.gain[0].take(0.0), self.gain[1].take(0.0)], level: [self.level[0].take(-200.0), self.level[1].take(-200.0)], active: self.active.load(Relaxed) }
    }
}

#[derive(Default)]
struct BandLive {
    clip: SectionLive,
    /// Net dynamic gain applied to the band (dB, signed) for the multiband overview bar.
    net_gain: F,
    /// Peak output level of the band (dBFS) for overview activity.
    level: F,
}

/// History lanes: the whole signal, then one per band.
pub const HIST_LANES: usize = 1 + MAX_BANDS;
/// History columns kept, and the time each one covers.
pub const HIST_COLS: usize = 4096;
pub const HIST_SECONDS_PER_COL: f32 = 0.005;
/// One column: per lane [input min, input max, output min, output max].
pub type HistCol = [[f32; 4]; HIST_LANES];

struct History {
    buf: Vec<AtomicU32>,
    head: AtomicUsize,
}

impl Default for History {
    fn default() -> Self {
        Self { buf: (0..HIST_COLS * HIST_LANES * 4).map(|_| AtomicU32::new(0)).collect(), head: AtomicUsize::new(0) }
    }
}

/// Samples kept per tap channel, a power of two.
pub const TAP_LEN: usize = 8192;

struct Taps {
    buf: Vec<AtomicU32>,
    head: AtomicUsize,
}

impl Default for Taps {
    fn default() -> Self {
        Self { buf: (0..4 * TAP_LEN).map(|_| AtomicU32::new(0)).collect(), head: AtomicUsize::new(0) }
    }
}

pub struct Shared {
    taps: Taps,
    history: History,
    /// Every host parameter, mirrored for windows that live without the editor (pop-outs).
    params: Vec<F>,
    bands: [BandLive; MAX_BANDS],
    single: SectionLive,
    output: SectionLive,
    peak_in: [F; 2],
    peak_out: [F; 2],
    rate: F,
    auto_gain: F,
}

impl Default for Shared {
    fn default() -> Self {
        Self {
            taps: Taps::default(),
            history: History::default(),
            params: (0..crate::INFO.params.len()).map(|_| F::default()).collect(),
            bands: Default::default(),
            single: SectionLive::default(),
            output: SectionLive::default(),
            peak_in: Default::default(),
            peak_out: Default::default(),
            rate: F::default(),
            auto_gain: F::default(),
        }
    }
}

/// One clipper (a band's, the single clipper or the output ceiling) as of the last tick.
#[derive(Clone, Copy, Debug, Default)]
pub struct Live {
    /// Signed gain per channel, dB.
    pub gain: [f32; 2],
    /// Detector level per channel, dBFS.
    pub level: [f32; 2],
    pub active: bool,
}

/// Summary for one band on the multiband overview plot.
#[derive(Clone, Copy, Debug, Default)]
pub struct BandSummary {
    /// Net signed dynamic gain in dB.
    pub gain_db: f32,
    /// Post-band peak level in dBFS.
    pub level_db: f32,
}

impl Shared {
    pub(crate) fn band_clip(&self, band: usize, gain: [f32; 2], level: [f32; 2], active: bool) {
        if let Some(b) = self.bands.get(band) {
            b.clip.merge(gain, level, active);
        }
    }

    pub(crate) fn band_summary(&self, band: usize, net_gain_db: f32, level_db: f32) {
        let Some(b) = self.bands.get(band) else { return };
        b.net_gain.merge(net_gain_db, |v, old| v.abs() > old.abs());
        b.level.merge(level_db, |v, old| v > old);
    }

    pub(crate) fn single(&self, gain: [f32; 2], level: [f32; 2], active: bool) {
        self.single.merge(gain, level, active);
    }

    pub(crate) fn output_clip(&self, gain: [f32; 2], level: [f32; 2], active: bool) {
        self.output.merge(gain, level, active);
    }

    pub(crate) fn peaks(&self, input: [f32; 2], output: [f32; 2]) {
        for c in 0..2 {
            self.peak_in[c].merge(input[c], |v, old| v > old);
            self.peak_out[c].merge(output[c], |v, old| v > old);
        }
    }

    pub(crate) fn push_taps(&self, pre: [&[f32]; 2], post: [&[f32]; 2]) {
        let head = self.taps.head.load(Relaxed);
        for (c, src) in [pre[0], pre[1], post[0], post[1]].into_iter().enumerate() {
            let base = c * TAP_LEN;
            for (j, v) in src.iter().enumerate() {
                self.taps.buf[base + ((head + j) & (TAP_LEN - 1))].store(v.to_bits(), Relaxed);
            }
        }
        self.taps.head.store(head.wrapping_add(pre[0].len()), Ordering::Release);
    }

    pub(crate) fn set_param(&self, index: usize, value: f32) {
        if let Some(p) = self.params.get(index) {
            p.set(value);
        }
    }

    /// The mirrored parameters (defaults for any not set yet).
    pub fn params(&self) -> Vec<f32> {
        self.params.iter().zip(crate::INFO.params.iter()).map(|(p, i)| if p.get().is_nan() { i.default } else { p.get() }).collect()
    }

    pub(crate) fn push_history(&self, col: &HistCol) {
        let h = &self.history;
        let head = h.head.load(Relaxed);
        let base = (head % HIST_COLS) * HIST_LANES * 4;
        for (i, v) in col.iter().flatten().enumerate() {
            h.buf[base + i].store(v.to_bits(), Relaxed);
        }
        h.head.store(head.wrapping_add(1), Ordering::Release);
    }

    /// The last `n` columns (at most `HIST_COLS`), oldest first; fewer before that many exist.
    pub fn read_history(&self, n: usize) -> Vec<HistCol> {
        let h = &self.history;
        let head = h.head.load(Ordering::Acquire);
        let n = n.min(HIST_COLS).min(head);
        (0..n)
            .map(|j| {
                let base = ((head - n + j) % HIST_COLS) * HIST_LANES * 4;
                let mut col = [[0.0f32; 4]; HIST_LANES];
                for (i, v) in col.iter_mut().flatten().enumerate() {
                    *v = f32::from_bits(h.buf[base + i].load(Relaxed));
                }
                col
            })
            .collect()
    }

    pub fn read_tap(&self, channel: usize, out: &mut [f32]) {
        let head = self.taps.head.load(Ordering::Acquire);
        let n = out.len().min(TAP_LEN);
        let base = (channel % 4) * TAP_LEN;
        for (j, o) in out[..n].iter_mut().enumerate() {
            let i = head.wrapping_sub(n).wrapping_add(j) & (TAP_LEN - 1);
            *o = f32::from_bits(self.taps.buf[base + i].load(Relaxed));
        }
    }

    pub(crate) fn set_auto_gain(&self, db: f32) {
        self.auto_gain.set(db);
    }

    pub fn auto_gain_db(&self) -> f32 {
        let v = self.auto_gain.get();
        if v.is_nan() { 0.0 } else { v }
    }

    pub(crate) fn set_sample_rate(&self, sr: f32) {
        self.rate.set(sr);
    }

    pub fn take_band_clip(&self, band: usize) -> Live {
        self.bands.get(band).map_or(Live::default(), |b| b.clip.take())
    }

    pub fn take_band_summary(&self, band: usize) -> BandSummary {
        let Some(b) = self.bands.get(band) else { return BandSummary::default() };
        BandSummary { gain_db: b.net_gain.take(0.0), level_db: b.level.take(-200.0) }
    }

    pub fn take_single(&self) -> Live {
        self.single.take()
    }

    pub fn take_output(&self) -> Live {
        self.output.take()
    }

    pub fn take_peaks(&self) -> ([f32; 2], [f32; 2]) {
        ([self.peak_in[0].take(0.0), self.peak_in[1].take(0.0)], [self.peak_out[0].take(0.0), self.peak_out[1].take(0.0)])
    }

    pub fn sample_rate(&self) -> f32 {
        let r = self.rate.get();
        if r.is_nan() { 48_000.0 } else { r }
    }
}
