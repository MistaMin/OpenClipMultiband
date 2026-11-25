// Copyright (c) 2026 Marcos Quirino Deida
//! OpenClip Multiband editor, one page. The large history shows the original signal and what the
//! clipping made of it: the whole signal on top, then every band in its own colour (red is what
//! the clipper took off). Beside it the waveform box shows the selected algorithm's curve and a
//! sine bent by it; its pencil button switches to a curve you draw. Below: the crossover strip,
//! the band panel (one All Bands set of Drive, Ceiling and Gain, opening into one column per band)
//! and the Mix & Output sliders, with IN/OUT meters on the rails.
//!
//! Follows the GoodLookin DRI performance and readability rules: static chrome and control values
//! in the cached layer, live traces as pixel-snapped column fills and short-piece curves, text >= 8 pt,
//! and every parameter bound to a real SDK `Control` (drag, wheel, double-click type, host menu,
//! accessibility); the drawn-curve points are set by drawing in the curve view.

// The kit, theme and raster helpers are shared with sibling plugins; not every helper is used here.
#![allow(dead_code)]

pub mod analyser;
pub mod history;
pub mod kit;
#[cfg(target_os = "macos")]
mod popout;
pub(crate) mod pops;
#[cfg(target_os = "windows")]
mod popwin;
#[cfg(target_os = "linux")]
mod popx11;
pub mod raster;
pub mod theme;

use std::sync::Arc;

use openplugin::core::editor::{AccessAction, AccessNode, AccessRange, DamageRect, DamageTracker, Key, Modifiers, ParamMenu, ParamMenuAction, SizeRange, TextEdit, Tooltip};
use openplugin::core::{Editor, EditorHost, EditorState, Frame};
use openplugin::dri::goodlookindri::scene::{Point, Rect, Scene};
use openplugin::dri::{ControlSet, SceneCanvas};

use crate::crossover::{FREQ_MAX, FREQ_MIN};
use crate::shared::{HistCol, Shared, HIST_LANES};
use crate::{
    OpenClipMultiband, BANDS, CURVE_POINTS, INFO, MAX_BANDS, MAX_SPLITS, P_AA, P_ALGORITHM, P_ALL_CEILING, P_ALL_DRIVE, P_ALL_GAIN, P_AUTO_GAIN, P_BANDS_OPEN,
    P_BAND_COUNT, P_BLEND, P_CHARACTER, P_CURVE_BASE, P_CURVE_ON, P_DELTA, P_INPUT, P_MIX, P_OUTPUT, P_OUT_CEILING, P_OUT_CEILING_ON, P_OVERSAMPLING, P_SHAPE,
    P_SINGLE_CEILING, P_SINGLE_DRIVE, P_XOVER_BASE, P_XOVER_SLOPE, P_ZERO_LATENCY,
};
use kit::{inside, Cell, Kind, Size};
use raster::R;
use theme::{load_theme, store_theme, theme, Theme, ThemeId};

pub const WIDTH: u32 = 1080;
pub const HEIGHT: u32 = 768;

// Header controls (y = 14..38)
const BANDS_CTRL: R = (204.0, 14.0, 138.0, 24.0);
const SLOPE_CTRL: R = (394.0, 14.0, 144.0, 24.0);
const DELTA_RECT: R = (562.0, 15.0, 76.0, 22.0);
const AUTO_GAIN_RECT: R = (646.0, 15.0, 114.0, 22.0);
const ZERO_LATENCY_RECT: R = (768.0, 15.0, 118.0, 22.0);
const THEME_BUTTON: R = (994.0, 15.0, 72.0, 22.0);

// Rails & center page
const LEFT_RAIL: R = (8.0, 56.0, 84.0, 704.0);
const RIGHT_RAIL: R = (988.0, 56.0, 84.0, 704.0);
const PAGE: R = (100.0, 56.0, 880.0, 704.0);

// Algorithm row
const ALGO_RECT: R = (184.0, 66.0, 170.0, 24.0);
const AA_RECT: R = (436.0, 66.0, 160.0, 24.0);
const OS_RECT: R = (706.0, 66.0, 262.0, 24.0);

// History (the large part), the waveform box and the Mix & Output sliders beside it
const HIST_RECT: R = (112.0, 100.0, 600.0, 360.0);
const SIM_RECT: R = (724.0, 100.0, 244.0, 150.0);
const PENCIL_RECT: R = (930.0, 104.0, 34.0, 20.0);
/// Opens the history in a window of its own.
const HIST_POP: R = (634.0, 104.0, 74.0, 16.0);
/// Columns the history ring keeps.
pub const HISTORY_COLS: usize = crate::shared::HIST_COLS;
/// Curve view: input 0..U_RANGE across, output 0..CURVE_TOP up (ceiling units; the curve is odd).
const CURVE_VIEW: R = (730.0, 128.0, 114.0, 116.0);
const U_RANGE: f32 = 2.0;
const CURVE_TOP: f32 = 1.1;
/// Sine view: one cycle across, -WAVE_RANGE..WAVE_RANGE up.
const WAVE_VIEW: R = (850.0, 128.0, 112.0, 116.0);
const WAVE_RANGE: f32 = 1.15;
const MIX_PANEL: R = (724.0, 260.0, 244.0, 200.0);
const OUT_ON_RECT: R = (882.0, 263.0, 80.0, 18.0);
const SLIDER_X: f32 = 730.0;
const SLIDER_W: f32 = 232.0;
const SLIDER_Y0: f32 = 284.0;
const SLIDER_PITCH: f32 = 25.0;

// Crossover strip
const MB_PLOT: R = (140.0, 470.0, 804.0, 48.0);
const MB_LO: f32 = -72.0;
const MB_HI: f32 = 6.0;
const MB_SPEC_BINS: usize = 96;
const MB_SPEC_SIZE: usize = 4096;

// Band panel: one All Bands set (large knobs), opening into an ALL column and one column per band
const BAND_PANEL: R = (112.0, 538.0, 856.0, 218.0);
const OPEN_RECT: R = (856.0, 542.0, 106.0, 20.0);
const ALL_COL: R = (116.0, 562.0, 120.0, 190.0);
/// Band columns when open: x from, width.
const BAND_AREA: (f32, f32) = (242.0, 722.0);
const COL_GAP: f32 = 6.0;
const COL_TOP: f32 = 562.0;
const COL_HEAD_Y: f32 = 574.0;
/// Open: Drive and Ceiling side by side on the first row, Gain on the second.
const ROW_Y: [f32; 2] = [584.0, 648.0];
const PILL_Y: f32 = 718.0;
/// Closed: the All Bands knobs and the row of Solo / Mute / Bypass per band.
const ALL_KNOB_X: [f32; 3] = [266.0, 480.0, 694.0];
const ALL_KNOB_Y: f32 = 568.0;
const CHIP_Y: f32 = 730.0;

// Meter rails
const METER_TOP: f32 = 68.0;
const METER_BOTTOM: f32 = 636.0;
const M_LO: f32 = -60.0;
const M_HI: f32 = 6.0;

fn whole_db(v: f32) -> String {
    let r = v.round();
    if r == 0.0 { "0".into() } else { format!("{r:+.0}") }
}

fn short_freq(hz: f32) -> String {
    if hz >= 1000.0 {
        let k = hz / 1000.0;
        if k >= 10.0 { format!("{k:.0}k") } else { format!("{k:.1}k") }
    } else {
        format!("{hz:.0}")
    }
}

fn db_to_gain(db: f32) -> f32 {
    10f32.powf(db / 20.0)
}

/// Closed band panel: group `b` of `count` Solo / Mute / Bypass groups: (x, width).
fn chip_group(b: usize, count: usize) -> (f32, f32) {
    let w = (BAND_PANEL.2 - 12.0) / count.max(1) as f32;
    (BAND_PANEL.0 + 6.0 + b as f32 * w, w)
}

/// Column `b` of `count` open band columns: (x, width).
fn band_col(b: usize, count: usize) -> (f32, f32) {
    let w = (BAND_AREA.1 - COL_GAP * count.saturating_sub(1) as f32) / count.max(1) as f32;
    (BAND_AREA.0 + b as f32 * (w + COL_GAP), w)
}

/// History lanes inside `r`: (lane, y, height). Lane 0 is the whole signal, lane 1 + b band b.
fn lanes_in(r: R, bands: usize) -> Vec<(usize, f32, f32)> {
    let (top, h) = (r.1 + 4.0, r.3 - 8.0);
    let gap = 4.0;
    let all = (h * 0.38).round();
    let each = (h - all - gap * bands as f32) / bands as f32;
    let mut v = vec![(0, top, all)];
    for b in 0..bands {
        v.push((1 + b, top + all + gap + b as f32 * (each + gap), each));
    }
    v
}

fn active_bands_of(values: &[f32]) -> usize {
    (values[P_BAND_COUNT].round() as usize + 1).clamp(1, MAX_BANDS)
}

/// Frequency span `[lo, hi]` Hz of active band `b`.
fn band_bounds_of(values: &[f32], b: usize) -> (f32, f32) {
    let mut splits: Vec<f32> = (0..active_bands_of(values) - 1).map(|k| values[P_XOVER_BASE + k]).collect();
    splits.sort_by(|a, b| a.total_cmp(b));
    let lo = if b == 0 { FREQ_MIN } else { splits.get(b - 1).copied().unwrap_or(FREQ_MIN) };
    let hi = splits.get(b).copied().unwrap_or(FREQ_MAX);
    (lo, hi.max(lo))
}

/// The ceiling a history lane shows: the output ceiling for the whole signal, the band's own
/// (all-bands set plus its own) for a band.
fn lane_ceiling_of(values: &[f32], lane: usize) -> f32 {
    if lane == 0 {
        if values[P_OUT_CEILING_ON] >= 0.5 { db_to_gain(values[P_OUT_CEILING]) } else { 1.0 }
    } else {
        db_to_gain(OpenClipMultiband::band_settings(values, lane - 1)[1])
    }
}

/// How far each history lane should reach: its loudest visible level (input or output), never closer
/// than 1.25 times its ceiling.
pub fn lane_targets(cols: &[HistCol], values: &[f32]) -> [f32; HIST_LANES] {
    let mut out = [0.0; HIST_LANES];
    for (lane, o) in out.iter_mut().enumerate() {
        let peak = cols.iter().map(|c| c[lane].iter().fold(0.0f32, |m, v| m.max(v.abs()))).fold(0.0f32, f32::max);
        *o = (1.25 * lane_ceiling_of(values, lane)).max(1.05 * peak);
    }
    out
}

/// Lane scales that follow `targets` up at once and down at 6 dB a second, so a peak leaving the
/// view never makes the lane jump.
pub fn settle_scales(scales: &mut [f32; HIST_LANES], targets: &[f32; HIST_LANES], dt: f32) {
    let fall = 10f32.powf(-6.0 * dt.max(0.0) / 20.0);
    for (s, t) in scales.iter_mut().zip(targets) {
        *s = t.max(*s * fall);
    }
}

/// The history in `r`: per lane the original (red where it was clipped off) and the result, the
/// lane's ceiling and its name, each lane drawn to `scales`. Shared by the editor and the pop-out.
pub fn paint_history(s: &mut Scene, t: &Theme, r: R, cols: &[HistCol], values: &[f32], scales: &[f32; HIST_LANES]) {
    kit::plot(s, t, r);
    let n = active_bands_of(values);
    let right = r.0 + r.2 - 2.0;
    let m = cols.len().min((r.2 - 4.0).max(0.0) as usize);
    let cols = &cols[cols.len() - m..];
    for (lane, y, h) in lanes_in(r, n) {
        // Half the waveform: the level on each side folded up from the lane's floor, twice as tall
        // as a two-sided wave, so transients keep their shape.
        let base = y + h - 1.0;
        s.line(Point::new(r.0 + 2.0, base), Point::new(right, base), t.grid, 1.0);
        if lane > 0 {
            s.line(Point::new(r.0 + 2.0, y - 2.0), Point::new(right, y - 2.0), t.grid_strong, 1.0);
        }
        let col = if lane == 0 { t.text } else { t.band(lane - 1) };
        let ceiling = lane_ceiling_of(values, lane);
        let level = |c: &[f32; 4], k: usize| c[k].abs().max(c[k + 1].abs());
        let scale = scales[lane].max(1e-6);
        let map = |v: f32| base - (v / scale).min(1.0) * (h - 4.0);
        if m >= 2 {
            let x = |j: usize| right - (m - 1 - j) as f32;
            let pts = |k: usize| -> Vec<Point> { cols.iter().enumerate().map(|(j, c)| Point::new(x(j), map(level(&c[lane], k)))).collect() };
            kit::area(s, &pts(0), base, t.over.with_alpha(0.45));
            kit::area(s, &pts(2), base, col.with_alpha(0.85));
        }
        kit::dashed_h(s, r.0 + 2.0, right, map(ceiling), 4.0, 4.0, col.with_alpha(0.6), 1.0);
        let label = if lane == 0 {
            "ALL".to_string()
        } else {
            let (lo, hi) = band_bounds_of(values, lane - 1);
            format!("B{}  {}-{}", lane, short_freq(lo), short_freq(hi))
        };
        plate_label(s, t, r.0 + 5.0, y + 2.0, &label, col);
    }
}

/// An 8 pt label on a plate in the plot colour, its top left at (x, y): readable over a trace.
fn plate_label(s: &mut Scene, t: &Theme, x: f32, y: f32, text: &str, col: openplugin::dri::goodlookindri::color::Color) {
    let tw = goodlookindri_shell::text::width(text, 8.0);
    s.fill_round(Rect::new(x, y, tw + 8.0, 12.0), 3.0, t.plot_bg.with_alpha(0.9), t.plot_bg.with_alpha(0.9), true);
    s.text(Point::new(x + 4.0, y + 6.0), text, col, 8.0, false);
}

fn mb_y(db: f32) -> f32 {
    let n = ((db - MB_LO) / (MB_HI - MB_LO)).clamp(0.0, 1.0);
    MB_PLOT.1 + MB_PLOT.3 - n * MB_PLOT.3
}

fn mb_freq_x(hz: f32) -> f32 {
    kit::freq_x(hz, MB_PLOT.0, MB_PLOT.0 + MB_PLOT.2, FREQ_MIN, FREQ_MAX)
}

fn meter_y(db: f32) -> f32 {
    let n = ((db - M_LO) / (M_HI - M_LO)).clamp(0.0, 1.0);
    METER_BOTTOM - n * (METER_BOTTOM - METER_TOP)
}

fn to_db(v: f32) -> f32 {
    if v > 1e-9 { 20.0 * v.log10() } else { -200.0 }
}

fn curve_x(u: f32) -> f32 {
    CURVE_VIEW.0 + CURVE_VIEW.2 * (u / U_RANGE).clamp(0.0, 1.0)
}

fn curve_y(v: f32) -> f32 {
    CURVE_VIEW.1 + CURVE_VIEW.3 * (1.0 - (v / CURVE_TOP).clamp(0.0, 1.0))
}

fn wave_y(v: f32) -> f32 {
    WAVE_VIEW.1 + WAVE_VIEW.3 * 0.5 * (1.0 - (v / WAVE_RANGE).clamp(-1.0, 1.0))
}

/// Samples of the active curve across `-U_RANGE..=U_RANGE` (ceiling units).
const TAB: usize = 401;

pub struct ClipEditor {
    host: EditorHost,
    controls: ControlSet,
    cells: Vec<Cell>,
    values: Vec<f32>,
    theme: ThemeId,
    shared: Option<Arc<Shared>>,
    history: Vec<HistCol>,
    meters: [[f32; 2]; 2],
    auto_gain_db: f32,
    mb_spec: [analyser::Spectrum; 2],
    taps: [Vec<f32>; 4],
    dragging: bool,
    xover_moved: bool,
    modifiers: Modifiers,
    keys_active: bool,
    canvas: SceneCanvas,
    damage: DamageTracker,
    /// The open Algorithm or Anti-Alias list.
    popover: Option<kit::Popover>,
    /// The history is open in a window of its own.
    popped: bool,
    /// How far each history lane reaches (settled, see `settle_scales`).
    hist_scale: [f32; HIST_LANES],
    /// Last point drawn in the curve view (ceiling units), while drawing.
    drawing: Option<(f32, f32)>,
    /// The active curve, sampled, and the settings it was sampled at.
    curve: Vec<f32>,
    curve_key: Option<Vec<u32>>,
}

pub fn make_editor(host: EditorHost) -> Option<Box<dyn Editor>> {
    Some(Box::new(ClipEditor::new(host)))
}

impl ClipEditor {
    pub fn new(host: EditorHost) -> Self {
        let mut e = Self {
            controls: ControlSet::new(host.clone()),
            host,
            cells: Vec::new(),
            values: INFO.params.iter().map(|p| p.default).collect(),
            theme: load_theme(),
            shared: None,
            history: Vec::new(),
            meters: [[-200.0; 2]; 2],
            auto_gain_db: 0.0,
            mb_spec: std::array::from_fn(|_| analyser::Spectrum::new(MB_SPEC_SIZE, MB_SPEC_BINS, FREQ_MIN, FREQ_MAX)),
            taps: std::array::from_fn(|_| vec![0.0; MB_SPEC_SIZE]),
            dragging: false,
            xover_moved: false,
            modifiers: Modifiers::NONE,
            keys_active: true,
            canvas: SceneCanvas::new(WIDTH as f32, HEIGHT as f32),
            damage: DamageTracker::new(),
            popover: None,
            popped: false,
            hist_scale: [0.0; HIST_LANES],
            drawing: None,
            curve: vec![0.0; TAB],
            curve_key: None,
        };
        e.build();
        e.refresh_curve();
        e
    }

    fn t(&self) -> &'static Theme {
        theme(self.theme)
    }

    fn active_bands(&self) -> usize {
        active_bands_of(&self.values)
    }

    fn bands_open(&self) -> bool {
        self.values[P_BANDS_OPEN] >= 0.5
    }

    fn drawn_on(&self) -> bool {
        self.values[P_CURVE_ON] >= 0.5
    }

    fn points(&self) -> [f32; CURVE_POINTS] {
        let mut p = [0.0; CURVE_POINTS];
        for (p, v) in p.iter_mut().zip(&self.values[P_CURVE_BASE..P_CURVE_BASE + CURVE_POINTS]) {
            *p = v / 100.0;
        }
        p
    }

    /// Frequency span `[lo, hi]` Hz of active band `b`.
    fn band_bounds(&self, b: usize) -> (f32, f32) {
        band_bounds_of(&self.values, b)
    }

    /// The selected algorithm's own curve (ceiling 1) at `u`.
    fn algorithm_curve(&self) -> impl Fn(f64) -> f64 {
        let algo = (self.values[P_ALGORITHM].round().max(0.0) as usize).min(clip::ALGOS.len() - 1);
        let mut s = (clip::ALGOS[algo].1)();
        s.set_rate(48_000.0);
        s.set(1.0, self.values[P_SHAPE] as f64 / 100.0, self.values[P_CHARACTER] as f64 / 100.0);
        move |u| s.curve(u).f
    }

    /// Resamples the active curve (drawn or the algorithm's) when its settings changed.
    fn refresh_curve(&mut self) {
        let mut key: Vec<u32> = [P_ALGORITHM, P_SHAPE, P_CHARACTER, P_CURVE_ON].iter().map(|&p| self.values[p].to_bits()).collect();
        key.extend(self.points().iter().map(|v| v.to_bits()));
        if self.curve_key.as_ref() == Some(&key) {
            return;
        }
        self.curve_key = Some(key);
        let at = |i: usize| (i as f64 / (TAB - 1) as f64 * 2.0 - 1.0) * U_RANGE as f64;
        if self.drawn_on() {
            let d = clip::Drawn::new(&self.points());
            self.curve = (0..TAB).map(|i| d.at(at(i)).f as f32).collect();
        } else {
            let f = self.algorithm_curve();
            self.curve = (0..TAB).map(|i| f(at(i)) as f32).collect();
        }
    }

    /// The active curve at `u` (ceiling units), from the table.
    fn curve_at(&self, u: f32) -> f32 {
        let p = ((u / U_RANGE + 1.0) * 0.5 * (TAB - 1) as f32).clamp(0.0, (TAB - 1) as f32);
        let i = (p as usize).min(TAB - 2);
        let t = p - i as f32;
        self.curve[i] + (self.curve[i + 1] - self.curve[i]) * t
    }

    fn make_cells(&self) -> Vec<Cell> {
        let mut cells = vec![
            Cell::knob(Size::Small, P_INPUT, "Input", 10.0, 662.0),
            Cell::knob(Size::Small, P_OUTPUT, "Output", 990.0, 662.0),
            Cell::segmented(P_BAND_COUNT, "Bands", BANDS_CTRL),
            Cell::segmented(P_XOVER_SLOPE, "Slope", SLOPE_CTRL),
            Cell::pill(P_DELTA, "Delta", DELTA_RECT),
            Cell::pill(P_AUTO_GAIN, "Auto Gain", AUTO_GAIN_RECT),
            Cell::pill(P_ZERO_LATENCY, "Zero Latency", ZERO_LATENCY_RECT),
            Cell::dropdown(P_ALGORITHM, "Algorithm", ALGO_RECT).tip("Clipping algorithm, used by every clipper"),
            Cell::dropdown(P_AA, "Anti-Alias", AA_RECT).tip("Anti-aliasing applied to every algorithm's curve"),
            Cell::segmented(P_OVERSAMPLING, "Oversampling", OS_RECT),
            Cell::pill(P_CURVE_ON, "", PENCIL_RECT).tip("Draw your own curve (replaces the algorithm's curve)"),
            Cell::pill(P_BANDS_OPEN, "Open Bands", OPEN_RECT).tip("Show each band's own Drive, Ceiling and Gain"),
        ];

        // Crossover handles inside MB_PLOT (horizontal drag tracking 1:1 with log Hz).
        let n = self.active_bands();
        let mut prev_right = MB_PLOT.0;
        for k in 0..n - 1 {
            let x = mb_freq_x(self.values[P_XOVER_BASE + k]).clamp(MB_PLOT.0 + 8.0, MB_PLOT.0 + MB_PLOT.2 - 8.0);
            let left = (x - 8.0).max(prev_right + 1.0).min(MB_PLOT.0 + MB_PLOT.2 - 16.0);
            prev_right = left + 16.0;
            cells.push(
                Cell::grip_h(P_XOVER_BASE + k, (left, MB_PLOT.1 + 2.0, 16.0, MB_PLOT.3 - 4.0))
                    .drag(MB_PLOT.2)
                    .tip(format!("Drag Crossover {} frequency (double-click to type Hz)", k + 1)),
            );
        }

        let mini = Size::Mini.cell().0;
        let alls = [(P_ALL_DRIVE, "Drive"), (P_ALL_CEILING, "Ceiling"), (P_ALL_GAIN, "Gain")];
        // Drive and Ceiling side by side, Gain under them, centred in a column at `x`, `w` wide.
        let trio = |cells: &mut Vec<Cell>, x: f32, w: f32, ps: [(usize, &'static str); 3], tip: &str| {
            let pair = 2.0 * mini + 3.0;
            let x0 = x + (w - pair) * 0.5;
            for (k, (p, label)) in ps.into_iter().enumerate() {
                let (kx, ky) = if k < 2 { (x0 + k as f32 * (mini + 3.0), ROW_Y[0]) } else { (x + (w - mini) * 0.5, ROW_Y[1]) };
                cells.push(Cell::knob(Size::Mini, p, label, kx, ky).tip(tip.to_string()));
            }
        };
        if self.bands_open() {
            trio(&mut cells, ALL_COL.0, ALL_COL.2, alls, "All bands");
            for (b, bl) in BANDS.iter().enumerate().take(n) {
                let (x, w) = band_col(b, n);
                trio(&mut cells, x, w, [(bl.drive, "Drive"), (bl.ceiling, "Ceiling"), (bl.gain, "Gain")], &format!("Band {}, on top of All Bands", b + 1));
                let pw = ((w - 12.0) / 3.0).min(24.0);
                let px = x + (w - 3.0 * pw - 8.0) * 0.5;
                for (k, (p, label, tip)) in [(bl.solo, "S", "Solo"), (bl.mute, "M", "Mute"), (bl.bypass, "B", "Bypass")].into_iter().enumerate() {
                    cells.push(Cell::pill(p, label, (px + k as f32 * (pw + 4.0), PILL_Y, pw, 18.0)).tip(format!("{tip} Band {}", b + 1)));
                }
            }
        } else {
            for ((p, label), x) in alls.into_iter().zip(ALL_KNOB_X) {
                cells.push(Cell::knob(Size::Big, p, label, x, ALL_KNOB_Y).tip("All bands"));
            }
            for (b, bl) in BANDS.iter().enumerate().take(n) {
                let (x, _) = chip_group(b, n);
                for (k, (p, label, tip)) in [(bl.solo, "S", "Solo"), (bl.mute, "M", "Mute"), (bl.bypass, "B", "Bypass")].into_iter().enumerate() {
                    cells.push(Cell::pill(p, label, (x + 24.0 + k as f32 * 21.0, CHIP_Y, 18.0, 18.0)).tip(format!("{tip} Band {}", b + 1)));
                }
            }
        }

        let row = |k: usize| SLIDER_Y0 + k as f32 * SLIDER_PITCH;
        for (k, (p, label)) in [
            (P_SINGLE_DRIVE, "Single Drive"),
            (P_SINGLE_CEILING, "Single Ceiling"),
            (P_BLEND, "Single-Multi"),
            (P_MIX, "Dry-Wet"),
            (P_OUT_CEILING, "Out Ceiling"),
            (P_SHAPE, "Shape"),
            (P_CHARACTER, "Character"),
        ]
        .into_iter()
        .enumerate()
        {
            // Shape and Character do nothing while the drawn curve is on.
            let off = self.drawn_on() && (p == P_SHAPE || p == P_CHARACTER);
            cells.push(Cell::new(Kind::FaderH, p, label, (SLIDER_X, row(k), SLIDER_W, 22.0)).dim(off));
        }
        cells.push(Cell::pill(P_OUT_CEILING_ON, "Ceiling On", OUT_ON_RECT).tip("Output ceiling on or off"));
        cells
    }

    fn build(&mut self) {
        self.cells = self.make_cells();
        let focus = self.controls.focused();
        let mut set = ControlSet::new(self.host.clone());
        for c in &self.cells {
            set.add(kit::control_for(c));
        }
        set.set_modifiers(self.modifiers);
        for c in &self.cells {
            set.param_changed(c.param, self.values[c.param]);
        }
        set.set_focus(focus.filter(|&f| f < self.cells.len()));
        // Baseline initial state so the first interaction does not treat untouched controls as changed.
        let _ = set.take_changed();
        self.controls = set;
        self.damage.add_all();
    }

    fn sync(&mut self) {
        let changed = self.controls.take_changed();
        if changed.is_empty() {
            return;
        }
        let mut rebuild = false;
        let mut edits = Vec::new();
        for id in changed {
            if let Some(c) = self.controls.get(id) {
                let (p, v) = (c.param, c.value());
                let prev = self.values[p];
                if v != prev {
                    rebuild |= p == P_BAND_COUNT;
                    self.xover_moved |= (P_XOVER_BASE..P_XOVER_BASE + MAX_SPLITS).contains(&p);
                    self.values[p] = v;
                    edits.push((p, prev, v));
                }
            }
        }
        for (p, prev, v) in edits {
            if p == P_CURVE_ON && v >= 0.5 && prev < 0.5 {
                self.seed_curve();
            }
            rebuild |= p == P_BANDS_OPEN || p == P_CURVE_ON;
        }
        if rebuild {
            self.build();
        }
        self.damage.add_all();
    }

    /// One click's worth of change to `p`, sent to the host.
    fn set_value(&mut self, p: usize, v: f32) {
        self.values[p] = v;
        self.host.set(p, v);
        self.controls.param_changed(p, v);
        let _ = self.controls.take_changed();
        self.damage.add_all();
    }

    /// Starting to draw: an untouched curve takes the selected algorithm's shape first.
    fn seed_curve(&mut self) {
        let untouched = (0..CURVE_POINTS).all(|k| (self.values[P_CURVE_BASE + k] - INFO.params[P_CURVE_BASE + k].default).abs() < 1e-6);
        if !untouched {
            return;
        }
        let f = self.algorithm_curve();
        for k in 0..CURVE_POINTS {
            let u = 2.0 * (k + 1) as f64 / CURVE_POINTS as f64;
            self.set_value(P_CURVE_BASE + k, INFO.params[P_CURVE_BASE + k].clamp(100.0 * f(u) as f32));
        }
    }

    /// Sets the drawn points between the last drawn position and (u, v), in ceiling units.
    fn draw_to(&mut self, u: f32, v: f32) {
        let (u0, v0) = self.drawing.unwrap_or((u, v));
        let node = |u: f32| ((u.abs() * CURVE_POINTS as f32 / 2.0).round() as usize).clamp(1, CURVE_POINTS);
        let (k0, k1) = (node(u0), node(u));
        let (lo, hi) = (k0.min(k1), k0.max(k1));
        for k in lo..=hi {
            let t = if k1 == k0 { 1.0 } else { (k as f32 - k0 as f32) / (k1 as f32 - k0 as f32) };
            // Odd curve: a point drawn left of centre mirrors to the right.
            let val = (v0 * u0.signum() + (v * u.signum() - v0 * u0.signum()) * t).clamp(0.0, 1.0);
            let p = P_CURVE_BASE + k - 1;
            let pct = INFO.params[p].clamp(100.0 * val);
            if self.values[p] != pct {
                self.set_value(p, pct);
            }
        }
        self.drawing = Some((u, v));
    }

    fn view_point(x: f32, y: f32) -> (f32, f32) {
        let u = ((x - CURVE_VIEW.0) / CURVE_VIEW.2 * U_RANGE).max(1e-3);
        let v = (1.0 - (y - CURVE_VIEW.1) / CURVE_VIEW.3) * CURVE_TOP;
        (u, v)
    }

    // ---- painting ----

    fn paint_layer(&self, s: &mut Scene) {
        let t = self.t();
        s.fill_rect(Rect::new(0.0, 0.0, WIDTH as f32, HEIGHT as f32), t.bg, t.bg, true);
        s.text(Point::new(16.0, 27.0), "OpenClip Multiband", t.text, 15.0, false);
        s.text(Point::new(162.0, 27.0), "BANDS", t.dim, 8.5, false);
        s.text(Point::new(354.0, 27.0), "SLOPE", t.dim, 8.5, false);
        kit::button(s, t, THEME_BUTTON, "THEME", false);

        kit::panel(s, t, LEFT_RAIL);
        kit::panel(s, t, RIGHT_RAIL);
        kit::panel(s, t, PAGE);
        for (x, label) in [(50.0, "IN"), (1030.0, "OUT")] {
            s.text(Point::new(x, METER_BOTTOM + 12.0), label, t.dim, 9.0, true);
        }
        for rail in [22.0, 1002.0] {
            for c in 0..2 {
                let x = rail + c as f32 * 18.0;
                s.fill_round(Rect::new(x, METER_TOP, 14.0, METER_BOTTOM - METER_TOP), 3.0, t.seg_off, t.seg_off, true);
            }
            for db in [6.0f32, 0.0, -6.0, -12.0, -24.0, -36.0, -48.0, -60.0] {
                let y = meter_y(db);
                s.line(Point::new(rail + 40.0, y), Point::new(rail + 44.0, y), t.grid_strong, 1.0);
                s.text(Point::new(rail + 47.0, y), format!("{db:.0}"), t.dim, 8.0, false);
            }
        }
        for (x, label) in [(112.0, "ALGORITHM"), (366.0, "ANTI-ALIAS"), (612.0, "OVERSAMPLE")] {
            s.text(Point::new(x, ALGO_RECT.1 + 12.0), label, t.dim, 8.5, false);
        }

        // Waveform box: curve view (positive half; the curve is odd) and one cycle of a sine through it.
        kit::inset(s, t, SIM_RECT, 7.0);
        s.text(Point::new(SIM_RECT.0 + 10.0, PENCIL_RECT.1 + 10.0), if self.drawn_on() { "WAVEFORM  ·  DRAWN" } else { "WAVEFORM" }, t.text, 9.0, false);
        kit::plot(s, t, CURVE_VIEW);
        kit::plot(s, t, WAVE_VIEW);
        // Labels in the corners the traces leave free, under the traces.
        let curve_label = if self.drawn_on() { "DRAW HERE" } else { "IN -> OUT" };
        let lw = goodlookindri_shell::text::width(curve_label, 8.0) + 8.0;
        plate_label(s, t, CURVE_VIEW.0 + CURVE_VIEW.2 - lw - 3.0, CURVE_VIEW.1 + CURVE_VIEW.3 - 15.0, curve_label, t.dim);
        plate_label(s, t, WAVE_VIEW.0 + 3.0, wave_y(0.0) + 5.0, "+6 dB OVER", t.dim);
        kit::dashed_h(s, CURVE_VIEW.0 + 2.0, CURVE_VIEW.0 + CURVE_VIEW.2 - 2.0, curve_y(1.0), 3.0, 3.0, t.over.with_alpha(0.7), 1.0);
        kit::dashed(s, &[Point::new(curve_x(0.0), curve_y(0.0)), Point::new(curve_x(CURVE_TOP), curve_y(CURVE_TOP))], 3.0, 3.0, t.dim, 1.0);
        let half = TAB / 2;
        let curve_pts: Vec<Point> = (half..TAB)
            .map(|i| {
                let u = (i as f32 / (TAB - 1) as f32 * 2.0 - 1.0) * U_RANGE;
                Point::new(curve_x(u), curve_y(self.curve[i]))
            })
            .collect();
        kit::curve(s, curve_pts, t.accent, 2.0);
        if self.drawn_on() {
            for k in 1..=CURVE_POINTS {
                let u = 2.0 * k as f32 / CURVE_POINTS as f32;
                let v = self.values[P_CURVE_BASE + k - 1] / 100.0;
                s.fill_ellipse_solid(Scene::ellipse_at(Point::new(curve_x(u), curve_y(v)), 2.5), t.text);
            }
        }
        for v in [-1.0f32, 1.0] {
            kit::dashed_h(s, WAVE_VIEW.0 + 2.0, WAVE_VIEW.0 + WAVE_VIEW.2 - 2.0, wave_y(v), 3.0, 3.0, t.over.with_alpha(0.7), 1.0);
        }
        s.line(Point::new(WAVE_VIEW.0, wave_y(0.0)), Point::new(WAVE_VIEW.0 + WAVE_VIEW.2, wave_y(0.0)), t.grid_strong, 1.0);
        let wave = |f: &dyn Fn(f32) -> f32| -> Vec<Point> {
            (0..=112)
                .map(|i| {
                    let ph = i as f32 / 112.0 * 2.0 * std::f32::consts::PI;
                    Point::new(WAVE_VIEW.0 + WAVE_VIEW.2 * i as f32 / 112.0, wave_y(f(2.0 * ph.sin())))
                })
                .collect()
        };
        kit::curve(s, wave(&|u| u), t.dim, 1.0);
        kit::curve(s, wave(&|u| self.curve_at(u)), t.accent, 2.0);

        // Crossover strip.
        let n = self.active_bands();
        kit::plot(s, t, MB_PLOT);
        let mb_bottom = MB_PLOT.1 + MB_PLOT.3;
        for f in [50.0f32, 100.0, 200.0, 500.0, 1000.0, 2000.0, 5000.0, 10_000.0] {
            let x = mb_freq_x(f);
            s.line(Point::new(x, MB_PLOT.1), Point::new(x, mb_bottom), t.grid, 1.0);
            s.text(Point::new(x, mb_bottom + 9.0), short_freq(f), t.dim, 8.0, true);
        }
        let any_solo = (0..n).any(|b| self.values[BANDS[b].solo] >= 0.5);
        for (b, bl) in BANDS.iter().enumerate().take(n) {
            let (lo, hi) = self.band_bounds(b);
            let (x0, x1) = (mb_freq_x(lo), mb_freq_x(hi));
            let bc = t.band(b);
            let audible = if any_solo { self.values[bl.solo] >= 0.5 } else { self.values[bl.mute] < 0.5 };
            let alpha = if audible { 0.10 } else { 0.03 };
            s.fill_rect(Rect::new(x0, MB_PLOT.1 + 1.0, (x1 - x0).max(1.0), MB_PLOT.3 - 2.0), bc.with_alpha(alpha), bc.with_alpha(alpha), true);

        }
        let mut prev_badge_right = MB_PLOT.0;
        for k in 0..n - 1 {
            let f = self.values[P_XOVER_BASE + k];
            let x = mb_freq_x(f).clamp(MB_PLOT.0 + 4.0, MB_PLOT.0 + MB_PLOT.2 - 4.0);
            s.line(Point::new(x, MB_PLOT.1), Point::new(x, mb_bottom), t.text.with_alpha(0.75), 1.4);
            let tag = short_freq(f);
            let bw = (goodlookindri_shell::text::width(&tag, 8.0) + 8.0).max(26.0);
            let bx = (x - bw * 0.5).clamp(MB_PLOT.0 + 2.0, MB_PLOT.0 + MB_PLOT.2 - bw - 2.0);
            let by = if bx < prev_badge_right + 2.0 { MB_PLOT.1 + 22.0 } else { MB_PLOT.1 + 4.0 };
            prev_badge_right = bx + bw;
            s.fill_round(Rect::new(bx, by, bw, 14.0), 4.0, t.panel, t.panel, true);
            s.stroke_round(Rect::new(bx, by, bw, 14.0), 4.0, t.accent, 1.0);
            s.text(Point::new(bx + bw * 0.5, by + 7.0), tag, t.text, 8.0, true);
        }
        // Band panel.
        kit::inset(s, t, BAND_PANEL, 7.0);
        s.text(Point::new(BAND_PANEL.0 + 10.0, OPEN_RECT.1 + 10.0), "BANDS", t.text, 9.0, false);
        if self.bands_open() {
            s.text(Point::new(BAND_PANEL.0 + 62.0, OPEN_RECT.1 + 10.0), "ALL + EACH BAND ON TOP", t.dim, 8.0, false);
            kit::inset(s, t, ALL_COL, 6.0);
            s.text(Point::new(ALL_COL.0 + ALL_COL.2 * 0.5, COL_HEAD_Y), "ALL", t.text, 9.0, true);
            for b in 0..n {
                let (cx, cw) = band_col(b, n);
                let (lo, hi) = self.band_bounds(b);
                kit::inset(s, t, (cx, COL_TOP, cw, ALL_COL.3), 6.0);
                s.fill_ellipse_solid(Scene::ellipse_at(Point::new(cx + 10.0, COL_HEAD_Y), 3.5), t.band(b));
                s.text(Point::new(cx + 18.0, COL_HEAD_Y), format!("B{}", b + 1), t.text, 9.0, false);
                let range = format!("{}-{}", short_freq(lo), short_freq(hi));
                if cw - 42.0 >= goodlookindri_shell::text::width(&range, 8.0) {
                    kit::text_right(s, cx + cw - 5.0, COL_HEAD_Y, &range, t.dim, 8.0);
                }
            }
        } else {
            s.text(Point::new(BAND_PANEL.0 + 62.0, OPEN_RECT.1 + 10.0), "ONE SET FOR ALL BANDS", t.dim, 8.0, false);
            for b in 0..n {
                let (x, _) = chip_group(b, n);
                s.fill_ellipse_solid(Scene::ellipse_at(Point::new(x + 6.0, CHIP_Y + 9.0), 3.5), t.band(b));
                s.text(Point::new(x + 12.0, CHIP_Y + 9.0), format!("{}", b + 1), t.text, 8.5, false);
            }
        }
        // Mix & Output sliders.
        kit::inset(s, t, MIX_PANEL, 7.0);
        s.text(Point::new(MIX_PANEL.0 + 10.0, OUT_ON_RECT.1 + 9.0), "MIX & OUTPUT", t.text, 9.0, false);

        let focused_param = self.focused_param();
        for (cell, c) in self.cells.iter().zip(self.controls.iter()) {
            kit::paint_static(s, t, cell);
            kit::paint_value(s, t, cell, c, c.value(), focused_param == Some(cell.param));
        }
        // The pencil on its button.
        let (px, py) = (PENCIL_RECT.0 + PENCIL_RECT.2 * 0.5, PENCIL_RECT.1 + PENCIL_RECT.3 * 0.5);
        let ink = if self.drawn_on() { t.on_accent } else { t.text };
        s.line(Point::new(px - 5.0, py + 5.0), Point::new(px + 4.0, py - 4.0), ink, 3.0);
        s.line(Point::new(px + 5.0, py - 5.0), Point::new(px + 6.5, py - 6.5), ink, 2.0);
        s.line(Point::new(px - 7.0, py + 7.0), Point::new(px - 5.5, py + 5.5), ink, 1.5);
    }

    fn focused_param(&self) -> Option<usize> {
        if !self.keys_active {
            return None;
        }
        self.controls.focused().and_then(|f| self.controls.get(f).map(|fc| fc.param))
    }

    fn paint_live(&self, s: &mut Scene) {
        let t = self.t();

        paint_history(s, t, HIST_RECT, &self.history, &self.values, &self.hist_scale);
        if pops::SUPPORTED {
            kit::button(s, t, HIST_POP, "POP OUT", self.popped);
        }

        // Spectrum in the crossover strip.
        let mb_bottom = MB_PLOT.1 + MB_PLOT.3;
        let pts = |k: usize| -> Vec<Point> {
            (0..MB_SPEC_BINS).map(|i| Point::new(MB_PLOT.0 + MB_PLOT.2 * i as f32 / (MB_SPEC_BINS - 1) as f32, mb_y(self.mb_spec[k].db[i]))).collect()
        };
        let post = pts(1);
        kit::area(s, &post, mb_bottom, t.out_fill.with_alpha(t.out_fill_alpha));
        kit::curve(s, pts(0), t.in_line.with_alpha(0.65), 1.0);
        kit::curve(s, post, t.out_line, 1.3);

        if self.values[P_AUTO_GAIN] >= 0.5 {
            let ag = format!("AUTO GAIN {} dB", whole_db(self.auto_gain_db));
            kit::text_right(s, OPEN_RECT.0 - 12.0, OPEN_RECT.1 + 10.0, &ag, t.dim, 8.5);
        }

        // IN / OUT peak meters.
        for (mi, x0) in [(0usize, 22.0f32), (1, 1002.0)] {
            for c in 0..2 {
                let db = self.meters[mi][c];
                if db > M_LO {
                    let y = meter_y(db);
                    let col = if db > 0.0 { t.over } else { t.seg_on };
                    s.fill_round(Rect::new(x0 + c as f32 * 18.0, y, 14.0, METER_BOTTOM - y), 3.0, col, col, true);
                }
            }
        }
        if let Some(p) = &self.popover {
            p.paint(s, t);
        }
    }
}

impl Editor for ClipEditor {
    fn size(&self) -> (u32, u32) {
        (WIDTH, HEIGHT)
    }

    fn size_range(&self) -> SizeRange {
        SizeRange::fixed(WIDTH, HEIGHT)
    }

    fn bind(&mut self, state: Option<EditorState>) {
        self.shared = state.and_then(|s| s.downcast::<Shared>().ok());
    }

    fn param_changed(&mut self, index: usize, plain: f32) {
        let Some(v) = self.values.get_mut(index) else { return };
        if *v == plain {
            return;
        }
        *v = plain;
        self.controls.param_changed(index, plain);
        let _ = self.controls.take_changed();
        if index == P_BAND_COUNT || index == P_BANDS_OPEN || index == P_CURVE_ON || (!self.dragging && (P_XOVER_BASE..P_XOVER_BASE + MAX_SPLITS).contains(&index)) {
            self.build();
        }
        self.damage.add_all();
    }

    fn tick(&mut self, dt: f32) {
        let Some(sh) = self.shared.clone() else { return };
        self.history = sh.read_history((HIST_RECT.2 - 4.0) as usize);
        settle_scales(&mut self.hist_scale, &lane_targets(&self.history, &self.values), dt);
        self.popped = pops::is_open(&sh);
        self.auto_gain_db = sh.auto_gain_db();
        for (c, tap) in self.taps.iter_mut().enumerate() {
            sh.read_tap(c, tap);
        }
        let sr = sh.sample_rate();
        let [a, b, c, d] = &self.taps;
        self.mb_spec[0].update(a, Some(b), sr, dt);
        self.mb_spec[1].update(c, Some(d), sr, dt);
        let (pin, pout) = sh.take_peaks();
        let fall = 24.0 * dt;
        for c in 0..2 {
            self.meters[0][c] = to_db(pin[c]).max(self.meters[0][c] - fall);
            self.meters[1][c] = to_db(pout[c]).max(self.meters[1][c] - fall);
        }
        self.damage.add_all();
    }

    fn damage(&mut self) -> Option<Vec<DamageRect>> {
        self.sync();
        self.damage.pending().map(<[DamageRect]>::to_vec)
    }

    fn render(&mut self, scale: f32) -> Frame<'_> {
        self.sync();
        self.refresh_curve();
        let mut h = 0xcbf2_9ce4_8422_2325u64;
        let mut mix = |v: u64| h = (h ^ v).wrapping_mul(0x0100_0000_01b3);
        mix(self.theme as u64);
        for v in &self.values {
            mix(v.to_bits() as u64);
        }
        let focused_param = self.focused_param();
        for (cell, c) in self.cells.iter().zip(self.controls.iter()) {
            mix(cell.param as u64);
            mix(c.value().to_bits() as u64);
            mix(u64::from(c.hover()) | (u64::from(c.pressed()) << 1) | (u64::from(focused_param == Some(cell.param)) << 2));
        }
        let mut layer = Scene::new();
        self.paint_layer(&mut layer);
        let mut live = Scene::new();
        self.paint_live(&mut live);
        let damage = self.damage.pending();
        let frame = self.canvas.frame_damaged(scale, h, |dl| dl.append(layer), |dl| dl.append(live), damage);
        self.damage.clear();
        frame
    }

    fn modifiers_changed(&mut self, modifiers: Modifiers) {
        self.modifiers = modifiers;
        self.controls.set_modifiers(modifiers);
    }

    fn mouse_down(&mut self, x: f32, y: f32, clicks: u32) {
        if let Some(p) = self.popover.take() {
            if let Some(i) = p.item_at(x, y) {
                self.set_value(p.param, INFO.params[p.param].min + i as f32);
            }
            self.damage.add_all();
            return;
        }
        if let Some(c) = self.cells.iter().find(|c| matches!(c.kind, Kind::Dropdown) && inside(c.rect, x, y)) {
            self.popover = Some(kit::Popover::open(c, self.values[c.param], (0.0, 0.0, WIDTH as f32, HEIGHT as f32)));
            self.damage.add_all();
            return;
        }
        if pops::SUPPORTED && inside(HIST_POP, x, y) {
            if let Some(sh) = self.shared.clone() {
                pops::toggle(&sh);
                self.popped = pops::is_open(&sh);
            }
            self.damage.add_all();
            return;
        }
        self.dragging = true;
        if inside(THEME_BUTTON, x, y) {
            self.theme = self.theme.next();
            store_theme(self.theme);
            self.damage.add_all();
            return;
        }
        if self.drawn_on() && inside(CURVE_VIEW, x, y) {
            let (u, v) = Self::view_point(x, y);
            self.drawing = None;
            self.draw_to(u, v);
            return;
        }
        if self.cells.iter().any(|c| inside(c.rect, x, y)) {
            self.controls.mouse_down(x, y, clicks);
            self.sync();
        }
    }

    fn mouse_move(&mut self, x: f32, y: f32) {
        if let Some(p) = &mut self.popover {
            let hover = p.item_at(x, y);
            if hover != p.hover {
                p.hover = hover;
                self.damage.add_all();
            }
            return;
        }
        if self.drawing.is_some() {
            let (u, v) = Self::view_point(x.clamp(CURVE_VIEW.0, CURVE_VIEW.0 + CURVE_VIEW.2), y);
            self.draw_to(u, v);
            return;
        }
        self.controls.mouse_move(x, y);
        self.sync();
    }

    fn mouse_up(&mut self, x: f32, y: f32) {
        self.dragging = false;
        self.drawing = None;
        self.controls.mouse_up(x, y);
        self.sync();
        // Refresh crossover handle hitboxes only after a crossover frequency moved;
        // rebuilding unconditionally would drop an active double-click text edit.
        if self.xover_moved {
            self.xover_moved = false;
            self.build();
        }
    }

    fn wheel(&mut self, x: f32, y: f32, dy: f32) {
        if let Some(p) = &mut self.popover {
            p.wheel(dy);
            self.damage.add_all();
            return;
        }
        self.controls.wheel(x, y, dy);
        self.sync();
    }

    fn right_mouse_down(&mut self, x: f32, y: f32) -> bool {
        self.controls.context_click(x, y)
    }

    fn take_param_menu(&mut self) -> Option<ParamMenu> {
        self.controls.take_param_menu()
    }

    fn param_menu_action(&mut self, param: u32, action: ParamMenuAction) -> Option<String> {
        let r = self.controls.param_menu_action(param, action);
        self.sync();
        r
    }

    fn tooltips(&self) -> Vec<Tooltip> {
        let mut tips = self.controls.tooltips();
        if self.drawn_on() {
            tips.push(Tooltip { rect: DamageRect::from_tuple(CURVE_VIEW), text: "Drag to draw the curve (input across, output up)".into() });
        }
        tips
    }

    fn accessibility_nodes(&self) -> Vec<AccessNode> {
        self.controls.accessibility_nodes()
    }

    fn accessibility_action(&mut self, node: usize, action: AccessAction) -> bool {
        let r = self.controls.accessibility_action(node, action);
        self.sync();
        r
    }

    fn accessibility_range(&self, node: usize) -> Option<AccessRange> {
        self.controls.accessibility_range(node)
    }

    fn accessibility_set_value(&mut self, node: usize, value: f64) -> bool {
        let r = self.controls.accessibility_set_value(node, value);
        self.sync();
        r
    }

    fn accessibility_focus(&mut self, node: usize) -> bool {
        self.controls.accessibility_focus(node)
    }

    fn wants_keys(&self) -> bool {
        self.controls.wants_keys()
    }

    fn key_down(&mut self, key: Key, modifiers: Modifiers) -> bool {
        let r = self.controls.key_down(key, modifiers);
        self.sync();
        r
    }

    fn key_up(&mut self, key: Key, modifiers: Modifiers) -> bool {
        self.controls.key_up(key, modifiers)
    }

    fn take_text_edit(&mut self) -> Option<TextEdit> {
        self.controls.take_text_edit()
    }

    fn text_edit_commit(&mut self, text: &str) -> bool {
        let r = self.controls.text_edit_commit(text);
        self.sync();
        r
    }

    fn text_edit_cancel(&mut self) {
        self.controls.text_edit_cancel();
    }

    fn focus_changed(&mut self, focused: bool) {
        self.keys_active = focused;
        if !focused {
            self.controls.release_keys();
        }
        self.damage.add_all();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn editor(bands: usize) -> ClipEditor {
        let mut e = ClipEditor::new(EditorHost::detached());
        e.param_changed(P_BAND_COUNT, (bands - 1) as f32);
        e
    }

    /// Fake history: a sine envelope per lane, clipped at 0.7.
    fn feed(shared: &Shared, cols: usize, drive: f32) {
        for j in 0..cols {
            let mut col = [[0.0f32; 4]; crate::shared::HIST_LANES];
            for (lane, c) in col.iter_mut().enumerate() {
                let a = drive * (0.5 + 0.4 * ((j + 17 * lane) as f32 * 0.07).sin()).abs();
                let o = a.min(0.7);
                *c = [-a, a, -o, o];
            }
            shared.push_history(&col);
        }
        shared.peaks([0.5, 0.4], [0.3, 0.25]);
        shared.set_auto_gain(-2.6);
    }

    #[test]
    fn knob_double_click_typing_and_drag_work() {
        let mut e = editor(4);
        e.param_changed(P_BANDS_OPEN, 1.0);
        let p = BANDS[2].ceiling;
        let cell = e.cells.iter().find(|c| c.param == p).expect("B3 ceiling cell");
        let (cx, cy) = (cell.rect.0 + cell.rect.2 * 0.5, cell.rect.1 + 16.0);
        e.mouse_down(cx, cy, 2);
        assert!(e.take_text_edit().is_some(), "double-click should open text edit");
        e.mouse_up(cx, cy);
        assert!(e.text_edit_commit("-6"), "text edit must survive mouse_up and commit");
        assert!((e.values[p] + 6.0).abs() < 0.01, "expected -6, got {}", e.values[p]);
        e.mouse_down(cx, cy, 1);
        e.mouse_move(cx, cy + 40.0);
        e.mouse_up(cx, cy + 40.0);
        assert!(e.values[p] < -7.0, "drag down should lower the ceiling, got {}", e.values[p]);
        assert_eq!(e.values[BANDS[1].ceiling], 0.0, "other bands stay put");
    }

    #[test]
    fn one_set_opens_into_the_bands() {
        let mut e = editor(4);
        let has = |e: &ClipEditor, p: usize| e.cells.iter().any(|c| c.param == p);
        assert!(has(&e, P_ALL_DRIVE) && !has(&e, BANDS[0].drive) && has(&e, BANDS[3].solo), "closed: one set and the band switches");
        e.mouse_down(OPEN_RECT.0 + 10.0, OPEN_RECT.1 + 10.0, 1);
        e.mouse_up(OPEN_RECT.0 + 10.0, OPEN_RECT.1 + 10.0);
        assert!(e.bands_open());
        assert!(has(&e, P_ALL_DRIVE) && has(&e, BANDS[0].drive) && has(&e, BANDS[3].gain) && !has(&e, BANDS[4].drive));
        // The all-bands Drive is one control: moving it leaves the bands' own values alone.
        let cell = e.cells.iter().find(|c| c.param == P_ALL_DRIVE).unwrap().rect;
        let (cx, cy) = (cell.0 + cell.2 * 0.5, cell.1 + 16.0);
        e.mouse_down(cx, cy, 2);
        assert!(e.take_text_edit().is_some());
        e.mouse_up(cx, cy);
        assert!(e.text_edit_commit("6"));
        assert_eq!(e.values[P_ALL_DRIVE], 6.0);
        assert_eq!(e.values[BANDS[1].drive], 0.0);
    }

    #[test]
    fn lists_open_and_pick() {
        let mut e = editor(4);
        for (rect, p, n) in [(ALGO_RECT, P_ALGORITHM, clip::ENGINES.len()), (AA_RECT, P_AA, clip::AA_LABELS.len())] {
            e.mouse_down(rect.0 + 10.0, rect.1 + 10.0, 1);
            let list = e.popover.clone().expect("list opens");
            assert_eq!(list.items.len(), n);
            let row = (n - 1).min(list.visible - 1);
            e.mouse_down(list.rect.0 + 20.0, list.rect.1 + 4.0 + row as f32 * list.row + list.row * 0.5, 1);
            assert!(e.popover.is_none());
            assert_eq!(e.values[p], (list.scroll + row) as f32);
        }
    }

    #[test]
    fn pencil_seeds_from_the_algorithm_and_drawing_sets_points() {
        let mut e = editor(4);
        e.mouse_down(PENCIL_RECT.0 + 10.0, PENCIL_RECT.1 + 10.0, 1);
        e.mouse_up(PENCIL_RECT.0 + 10.0, PENCIL_RECT.1 + 10.0);
        assert!(e.drawn_on());
        assert!(e.cells.iter().filter(|c| c.param == P_SHAPE || c.param == P_CHARACTER).all(|c| c.dim), "Shape and Character greyed");
        // Honey at Shape 80 %: linear to 0.78, so the first points follow the input.
        assert!((e.values[P_CURVE_BASE] - 12.0).abs() < 1.01 && e.values[P_CURVE_BASE + CURVE_POINTS - 1] > 90.0, "{:?}", e.points());
        // Draw a flat line at half the ceiling across the right half.
        let y = curve_y(0.5);
        e.mouse_down(curve_x(0.2), y, 1);
        e.mouse_move(curve_x(1.9), y);
        e.mouse_up(curve_x(1.9), y);
        for k in 2..CURVE_POINTS - 1 {
            assert!((e.values[P_CURVE_BASE + k] - 50.0).abs() < 2.0, "point {k}: {}", e.values[P_CURVE_BASE + k]);
        }
        e.refresh_curve();
        assert!((e.curve_at(1.0) - 0.5).abs() < 0.02 && (e.curve_at(-1.0) + 0.5).abs() < 0.02);
        // Off again keeps the drawing; on again does not reseed it.
        for _ in 0..2 {
            e.mouse_down(PENCIL_RECT.0 + 10.0, PENCIL_RECT.1 + 10.0, 1);
            e.mouse_up(PENCIL_RECT.0 + 10.0, PENCIL_RECT.1 + 10.0);
        }
        assert!((e.values[P_CURVE_BASE + 8] - 50.0).abs() < 2.0);
    }

    #[test]
    fn every_parameter_has_a_control() {
        let mut e = editor(MAX_BANDS);
        let mut seen = vec![false; INFO.params.len()];
        for open in [0.0, 1.0] {
            e.param_changed(P_BANDS_OPEN, open);
            e.build();
            for c in &e.cells {
                seen[c.param] = true;
            }
        }
        // The drawn-curve points are set by drawing in the curve view, not by a control each.
        let missing: Vec<&str> = seen.iter().enumerate().filter(|(i, s)| !**s && !(P_CURVE_BASE..P_CURVE_BASE + CURVE_POINTS).contains(i)).map(|(i, _)| INFO.params[i].id).collect();
        assert!(missing.is_empty(), "no control for {missing:?}");
    }

    #[test]
    fn controls_stay_inside_window_and_do_not_overlap() {
        let win: R = (0.0, 0.0, WIDTH as f32, HEIGHT as f32);
        let within = |c: &R, r: R| c.0 >= r.0 && c.1 >= r.1 && c.0 + c.2 <= r.0 + r.2 && c.1 + c.3 <= r.1 + r.3;
        let ov = |a: &R, b: &R| a.0 < b.0 + b.2 && b.0 < a.0 + a.2 && a.1 < b.1 + b.3 && b.1 < a.1 + a.3;
        for (bands, open) in (1..=MAX_BANDS).flat_map(|b| [(b, 0.0), (b, 1.0)]) {
            let mut e = editor(bands);
            e.param_changed(P_BANDS_OPEN, open);
            e.build();
            for (i, a) in e.cells.iter().enumerate() {
                assert!(within(&a.rect, win), "bands={bands}: {} outside window {:?}", a.label, a.rect);
                for b in &e.cells[i + 1..] {
                    assert!(!ov(&a.rect, &b.rect), "bands={bands}: {} {:?} overlaps {} {:?}", a.label, a.rect, b.label, b.rect);
                }
            }
        }
    }

    #[test]
    fn text_is_whole_apart_and_readable() {
        use goodlookindri_shell::text;
        use openplugin::dri::goodlookindri::scene::Cmd;
        let mut bad = Vec::new();
        for bands in 1..=MAX_BANDS {
            for drawn in [false, true] {
                let shared = Arc::new(Shared::default());
                let mut e = editor(bands);
                e.param_changed(P_BANDS_OPEN, if drawn { 1.0 } else { 0.0 });
                e.build();
                e.bind(Some(shared.clone() as EditorState));
                e.param_changed(P_AUTO_GAIN, 1.0);
                e.param_changed(P_CURVE_ON, if drawn { 1.0 } else { 0.0 });
                feed(&shared, 700, 1.4);
                e.tick(1.0 / 30.0);
                e.refresh_curve();
                let mut s = Scene::new();
                e.paint_layer(&mut s);
                e.paint_live(&mut s);
                let mut clips: Vec<(f32, f32, f32, f32)> = vec![];
                let mut texts: Vec<(f32, f32, f32, f32, String)> = vec![];
                for c in &s.cmds {
                    match c {
                        Cmd::ClipRound { rect, .. } => clips.push((rect.x, rect.y, rect.x + rect.w, rect.y + rect.h)),
                        Cmd::PopClip => {
                            clips.pop();
                        }
                        Cmd::Text { at, text: t, size, center, .. } => {
                            let w = text::width(t, *size);
                            let x0 = if *center { at.x - w * 0.5 } else { at.x };
                            let (y0, y1) = (at.y - size * 0.36, at.y + size * 0.36);
                            if let Some(cl) = clips.iter().fold(None::<(f32, f32, f32, f32)>, |a, c| Some(a.map_or(*c, |a| (a.0.max(c.0), a.1.max(c.1), a.2.min(c.2), a.3.min(c.3))))) {
                                if x0 < cl.0 - 0.5 || x0 + w > cl.2 + 0.5 || y0 < cl.1 - 0.5 || y1 > cl.3 + 0.5 {
                                    bad.push(format!("CLIPPED bands={bands}: {t:?}"));
                                }
                            }
                            if *size < 8.0 {
                                bad.push(format!("SMALL bands={bands}: {t:?} size {size}"));
                            }
                            if t.contains("-0.0") || t == "-0" || t.ends_with(" -0") {
                                bad.push(format!("MINUS ZERO bands={bands}: {t:?}"));
                            }
                            texts.push((x0, y0, x0 + w, y1, t.clone()));
                        }
                        _ => {}
                    }
                }
                for i in 0..texts.len() {
                    for j in i + 1..texts.len() {
                        let (a, b) = (&texts[i], &texts[j]);
                        if a.0 < b.2 - 0.5 && b.0 < a.2 - 0.5 && a.1 < b.3 - 0.5 && b.1 < a.3 - 0.5 && a.4 != b.4 && !a.4.trim().is_empty() && !b.4.trim().is_empty() {
                            bad.push(format!("OVERLAP bands={bands} drawn={drawn}: {:?} and {:?}", a.4, b.4));
                        }
                    }
                }
            }
        }
        assert!(bad.is_empty(), "{}", bad.join("\n"));
    }

    #[test]
    fn lane_zoom_rises_at_once_and_falls_slowly() {
        let mut scales = [0.0f32; HIST_LANES];
        settle_scales(&mut scales, &[4.0; HIST_LANES], 1.0 / 30.0);
        assert_eq!(scales[0], 4.0, "a loud peak is shown at once");
        // The peak leaves the view: one second later the lane has come down only 6 dB.
        for _ in 0..30 {
            settle_scales(&mut scales, &[1.0; HIST_LANES], 1.0 / 30.0);
        }
        assert!((scales[0] - 2.0).abs() < 0.05, "{}", scales[0]);
        // Output louder than input counts too.
        let col = [[-0.1f32, 0.1, -0.9, 0.9]; HIST_LANES];
        let mut v: Vec<f32> = INFO.params.iter().map(|p| p.default).collect();
        v[P_OUT_CEILING_ON] = 0.0;
        v[P_ALL_CEILING] = -24.0;
        let t = lane_targets(&[col], &v);
        assert!((t[1] - 0.945).abs() < 1e-3, "{t:?}");
    }

    #[test]
    fn meters_read_and_fall_as_specified() {
        for hz in [30.0f32, 60.0] {
            let shared = Arc::new(Shared::default());
            let mut e = editor(4);
            e.bind(Some(shared.clone() as EditorState));
            shared.peaks([1.0, 0.5012], [0.5012, 1.0]);
            e.tick(1.0 / hz);
            assert!(e.meters[0][0].abs() < 0.01 && (e.meters[0][1] + 6.0).abs() < 0.01, "{:?}", e.meters);
            assert!((e.meters[1][0] + 6.0).abs() < 0.01 && e.meters[1][1].abs() < 0.01);
            for _ in 0..hz as usize {
                e.tick(1.0 / hz);
            }
            assert!((e.meters[0][0] + 24.0).abs() < 0.1, "fall after 1s at {hz}Hz: {}", e.meters[0][0]);
        }
    }

    #[test]
    #[ignore]
    fn frame_cost() {
        for (bands, open) in [(4usize, false), (4, true), (6, true)] {
            let shared = Arc::new(Shared::default());
            let mut e = editor(bands);
            e.param_changed(P_BANDS_OPEN, if open { 1.0 } else { 0.0 });
            e.bind(Some(shared.clone() as EditorState));
            feed(&shared, 700, 1.4);
            e.tick(1.0 / 30.0);
            let _ = e.render(2.0);
            let (mut time, frames) = (std::time::Duration::ZERO, 30);
            for _ in 0..frames {
                feed(&shared, 6, 1.4);
                let mut seed = 11u32;
                let pre: Vec<f32> = (0..1600)
                    .map(|_| {
                        seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                        0.3 * ((seed >> 8) as f32 / (1u32 << 24) as f32 - 0.5)
                    })
                    .collect();
                shared.push_taps([&pre, &pre], [&pre, &pre]);
                e.tick(1.0 / 30.0);
                let t0 = std::time::Instant::now();
                let _ = e.render(2.0);
                time += t0.elapsed();
            }
            let panel = if open { "open" } else { "closed" };
            println!("{bands} bands, panel {panel} (1080x768 @2x): {:6.2} ms/frame", time.as_secs_f64() * 1000.0 / frames as f64);
        }
    }

    /// The editor fed by the real DSP under heavy clipping, rendered every second: `SHOTS/heavy-<s>.ppm`.
    #[test]
    #[ignore]
    fn heavy_clipping_history() {
        use openplugin::prelude::Plugin;
        let dir = std::env::var("SHOTS").unwrap_or_else(|_| format!("{}/qa/screenshots", env!("CARGO_MANIFEST_DIR")));
        std::fs::create_dir_all(&dir).expect("dir");
        let mut p = crate::OpenClipMultiband::default();
        for (id, v) in [("allDrive", 18.0), ("singleDrive", 12.0), ("mix", 50.0)] {
            p.set_param(id, v);
        }
        p.prepare(48_000.0, 256, 2);
        let shared = p.editor_state().expect("state");
        let mut e = editor(4);
        for (id, v) in [("allDrive", 18.0f32), ("singleDrive", 12.0), ("mix", 50.0)] {
            let i = INFO.params.iter().position(|q| q.id == id).unwrap();
            e.param_changed(i, v);
        }
        e.bind(Some(shared));
        let (fs, beat) = (48_000.0f32, 24_000usize);
        let mut seed = 5u32;
        let mut prev = [0.0f32; HIST_LANES];
        let mut worst_drop = 0.0f32;
        for blk in 0..(48_000 * 6 / 256) {
            let mut l: Vec<f32> = (0..256)
                .map(|i| {
                    let n = blk * 256 + i;
                    let t = (n % beat) as f32 / fs;
                    seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                    let noise = (seed >> 8) as f32 / (1u32 << 23) as f32 - 1.0;
                    let kick = (std::f32::consts::TAU * (50.0 + 120.0 * (-t * 30.0).exp()) * t).sin() * (-t * 7.0).exp();
                    let snare = if (n / beat) % 2 == 1 { 0.7 * noise * (-t * 18.0).exp() } else { 0.0 };
                    0.8 * kick + snare + 0.2 * (std::f32::consts::TAU * 55.0 * n as f32 / fs).sin()
                })
                .collect();
            let mut r = l.clone();
            let mut io: [&mut [f32]; 2] = [&mut l, &mut r];
            p.process(&mut io, 256);
            if blk % 6 == 5 {
                e.tick(1.0 / 31.25);
                for (k, v) in e.hist_scale.iter().enumerate() {
                    if prev[k] > 0.0 {
                        worst_drop = worst_drop.max(prev[k] / v);
                    }
                }
                prev = e.hist_scale;
            }
            if blk % 187 == 186 {
                e.damage.add_all();
                let f = e.render(1.0);
                let mut out = format!("P6\n{} {}\n255\n", f.width, f.height).into_bytes();
                for px in f.pixels {
                    out.extend_from_slice(&[(px >> 16) as u8, (px >> 8) as u8, *px as u8]);
                }
                std::fs::write(format!("{dir}/heavy-{}.ppm", blk / 187), out).expect("write");
            }
        }
        println!("largest one-tick zoom-out: {:.3}x", worst_drop);
        assert!(worst_drop < 1.03, "lanes jump: {worst_drop}");
    }

    /// Screenshots with realistic activity into `SHOTS` (`.ppm`). Run with `--ignored`.
    #[test]
    #[ignore]
    fn screenshots() {
        let dir = std::env::var("SHOTS").unwrap_or_else(|_| format!("{}/qa/screenshots", env!("CARGO_MANIFEST_DIR")));
        std::fs::create_dir_all(&dir).expect("dir");
        let scale: f32 = std::env::var("SCALE").ok().and_then(|v| v.parse().ok()).unwrap_or(1.0);
        for (name, th, bands, drawn, list, open) in [
            ("4band-dark", ThemeId::Dark, 4usize, false, false, false),
            ("4band-open-dark", ThemeId::Dark, 4, false, false, true),
            ("4band-drawn-dark", ThemeId::Dark, 4, true, false, false),
            ("4band-list-dark", ThemeId::Dark, 4, false, true, false),
            ("6band-open-graphite", ThemeId::Graphite, 6, false, false, true),
            ("3band-light", ThemeId::Light, 3, true, false, false),
        ] {
            let shared = Arc::new(Shared::default());
            let mut e = editor(bands);
            e.param_changed(P_BANDS_OPEN, if open { 1.0 } else { 0.0 });
            e.build();
            e.bind(Some(shared.clone() as EditorState));
            e.theme = th;
            e.param_changed(P_AUTO_GAIN, 1.0);
            e.param_changed(P_BLEND, 70.0);
            if drawn {
                e.mouse_down(PENCIL_RECT.0 + 10.0, PENCIL_RECT.1 + 10.0, 1);
                e.mouse_up(PENCIL_RECT.0 + 10.0, PENCIL_RECT.1 + 10.0);
                let pts = [0.15f32, 0.3, 0.45, 0.58, 0.68, 0.74, 0.76, 0.74, 0.7, 0.72, 0.78, 0.84, 0.88, 0.9, 0.9, 0.9];
                for (k, v) in pts.iter().enumerate() {
                    e.param_changed(P_CURVE_BASE + k, 100.0 * *v);
                }
            }
            for f in 0..60 {
                let mut col = [[0.0f32; 4]; crate::shared::HIST_LANES];
                for _ in 0..10 {
                    for (lane, c) in col.iter_mut().enumerate() {
                        let j = (f * 10) as f32;
                        let env = (0.35 + 0.65 * ((j * 0.02 + lane as f32).sin() * 0.5 + 0.5)) * if lane == 0 { 1.3 } else { 1.0 - 0.08 * lane as f32 };
                        let a = env * (1.0 + 0.25 * ((j * 0.31 + lane as f32 * 2.0).sin()));
                        let ceil = if lane == 0 { 0.89 } else { 0.7 };
                        *c = [-a * 0.93, a, -(a * 0.93).min(ceil), a.min(ceil)];
                    }
                    shared.push_history(&col);
                }
                let lvl = -14.0 - 2.3 * (f % 15) as f32;
                let pk = 10f32.powf(lvl / 20.0);
                shared.peaks([pk * 1.15, pk], [pk * 0.95, pk * 0.85]);
                let mut seed = 9u32 + f as u32;
                let pre: Vec<f32> = (0..800)
                    .map(|i| {
                        let n = (f * 800 + i) as f32 / 48_000.0;
                        seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                        let noise = (seed >> 8) as f32 / (1u32 << 24) as f32 - 0.5;
                        let tau = std::f32::consts::TAU;
                        pk * (0.45 * (tau * 80.0 * n).sin() + 0.28 * (tau * 440.0 * n).sin() + 0.16 * (tau * 2200.0 * n).sin() + 0.12 * noise)
                    })
                    .collect();
                let post: Vec<f32> = pre.iter().map(|v| v.clamp(-pk * 0.7, pk * 0.7)).collect();
                shared.push_taps([&pre, &pre], [&post, &post]);
                e.tick(1.0 / 30.0);
            }
            if list {
                e.mouse_down(ALGO_RECT.0 + 10.0, ALGO_RECT.1 + 10.0, 1);
            }
            e.damage.add_all();
            let f = e.render(scale);
            let mut out = format!("P6\n{} {}\n255\n", f.width, f.height).into_bytes();
            for px in f.pixels {
                out.extend_from_slice(&[(px >> 16) as u8, (px >> 8) as u8, *px as u8]);
            }
            std::fs::write(format!("{dir}/{name}-{scale:.0}x.ppm"), out).expect("write");
        }
    }
}
