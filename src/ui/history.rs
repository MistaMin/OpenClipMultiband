//! The history in a window of its own, at any size. It reads only the shared history ring and the
//! shared parameter mirror (never the `take_*` values the main editor consumes), so it runs next to
//! the main editor and keeps running with the main editor closed.

use std::sync::Arc;

use openplugin::core::editor::{DamageRect, DamageTracker, SizeRange};
use openplugin::core::{Editor, Frame};
use openplugin::dri::goodlookindri::scene::{Rect, Scene};
use openplugin::dri::SceneCanvas;

use super::theme::{load_theme, theme};
use super::{lane_targets, paint_history, settle_scales, HISTORY_COLS};
use crate::shared::{HistCol, Shared, HIST_LANES};

pub struct HistoryWindow {
    shared: Arc<Shared>,
    size: (u32, u32),
    canvas: SceneCanvas,
    damage: DamageTracker,
    cols: Vec<HistCol>,
    values: Vec<f32>,
    scales: [f32; HIST_LANES],
}

impl HistoryWindow {
    pub fn new(shared: Arc<Shared>) -> Self {
        let size = (900, 440);
        let values = shared.params();
        Self { shared, size, canvas: SceneCanvas::new(size.0 as f32, size.1 as f32), damage: DamageTracker::new(), cols: Vec::new(), values, scales: [0.0; HIST_LANES] }
    }

    fn plot(&self) -> (f32, f32, f32, f32) {
        (8.0, 8.0, self.size.0 as f32 - 16.0, self.size.1 as f32 - 16.0)
    }
}

impl Editor for HistoryWindow {
    fn size(&self) -> (u32, u32) {
        self.size
    }

    /// The window reads the parameters from the shared mirror on every tick.
    fn param_changed(&mut self, _index: usize, _plain: f32) {}

    fn size_range(&self) -> SizeRange {
        SizeRange::between((360, 220), (4096, 2400))
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.size = (width.max(360), height.max(220));
        self.canvas = SceneCanvas::new(self.size.0 as f32, self.size.1 as f32);
        self.damage.add_all();
    }

    fn tick(&mut self, dt: f32) {
        // One column per pixel, up to everything the ring keeps.
        let want = (self.plot().2 as usize).saturating_sub(4).min(HISTORY_COLS);
        self.cols = self.shared.read_history(want);
        self.values = self.shared.params();
        settle_scales(&mut self.scales, &lane_targets(&self.cols, &self.values), dt);
        self.damage.add_all();
    }

    fn damage(&mut self) -> Option<Vec<DamageRect>> {
        self.damage.pending().map(<[DamageRect]>::to_vec)
    }

    fn render(&mut self, scale: f32) -> Frame<'_> {
        let t = theme(load_theme());
        let (w, h) = (self.size.0 as f32, self.size.1 as f32);
        let mut layer = Scene::new();
        layer.fill_rect(Rect::new(0.0, 0.0, w, h), t.bg, t.bg, true);
        let mut live = Scene::new();
        paint_history(&mut live, t, self.plot(), &self.cols, &self.values, &self.scales);
        let damage = self.damage.pending();
        let key = (u64::from(self.size.0) << 32) | u64::from(self.size.1);
        let frame = self.canvas.frame_damaged(scale, key, |dl| dl.append(layer), |dl| dl.append(live), damage);
        self.damage.clear();
        frame
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resizes_and_draws_from_shared_state_alone() {
        let shared = Arc::new(Shared::default());
        let mut col = [[0.0f32; 4]; crate::shared::HIST_LANES];
        for c in col.iter_mut() {
            *c = [-1.2, 1.2, -0.7, 0.7];
        }
        for _ in 0..2000 {
            shared.push_history(&col);
        }
        let mut w = HistoryWindow::new(shared);
        w.resize(1400, 700);
        w.tick(1.0 / 30.0);
        assert_eq!(w.size(), (1400, 700));
        assert_eq!(w.cols.len(), (1400 - 16 - 4).min(HISTORY_COLS));
        let f = w.render(1.0);
        assert_eq!((f.width, f.height), (1400, 700));
        w.resize(10, 10);
        assert_eq!(w.size(), (360, 220), "kept above the minimum");
    }

    /// At the widest window the history still has a column for every pixel (no blank start).
    #[test]
    fn widest_window_is_filled() {
        let shared = Arc::new(Shared::default());
        let col = [[-0.5f32, 0.5, -0.4, 0.4]; crate::shared::HIST_LANES];
        for _ in 0..crate::shared::HIST_COLS + 100 {
            shared.push_history(&col);
        }
        let mut w = HistoryWindow::new(shared);
        let max = w.size_range().max;
        w.resize(max.0, max.1);
        w.tick(1.0 / 30.0);
        assert_eq!(w.cols.len(), w.plot().2 as usize - 4, "every pixel of the widest window has history");
    }
}
