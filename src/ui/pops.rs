//! Pop-out windows (from OpenLimiter): the history in a window of its own. It belongs to the
//! plug-in instance (its `Shared`), not to the editor, so it stays open when the plug-in window
//! closes; the instance closes it when it goes away ([`close_all`]). Where the window was is kept
//! in the preferences and used the next time it opens.

use std::sync::{Arc, Mutex, MutexGuard};

use openplugin::core::Editor;

use super::history::HistoryWindow;
use super::theme::pref_file;
use crate::shared::Shared;

#[cfg(target_os = "macos")]
use super::popout::Window;
#[cfg(target_os = "windows")]
use super::popwin::Window;
#[cfg(target_os = "linux")]
use super::popx11::Window;

/// What a platform window gets to build its editor from (Linux builds it on its own thread).
pub type Make = Box<dyn FnOnce() -> Box<dyn Editor> + Send>;

pub const SUPPORTED: bool = cfg!(any(target_os = "macos", target_os = "windows", target_os = "linux"));

#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
struct Pop {
    owner: usize,
    win: Window,
}

// macOS and Windows windows are only touched on the main thread, where the editor and the
// instance live; Linux windows run on their own threads behind atomics.
#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
unsafe impl Send for Pop {}

#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
impl Drop for Pop {
    fn drop(&mut self) {
        if let Some(frame) = self.win.frame() {
            save(frame);
        }
    }
}

#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
static POPS: Mutex<Vec<Pop>> = Mutex::new(Vec::new());

#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
fn pops() -> MutexGuard<'static, Vec<Pop>> {
    POPS.lock().unwrap_or_else(|e| e.into_inner())
}

/// Takes out the windows matching `gone`; the caller drops them after the lock is released.
#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
fn take(gone: impl Fn(&Pop) -> bool) -> Vec<Pop> {
    let mut all = pops();
    let (out, keep) = std::mem::take(&mut *all).into_iter().partition(|p| gone(p));
    *all = keep;
    out
}

fn owner(shared: &Shared) -> usize {
    shared as *const Shared as usize
}

/// Whether the history window is open for this instance. Forgets windows the user closed.
pub fn is_open(shared: &Shared) -> bool {
    #[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
    {
        drop(take(|p| !p.win.is_open()));
        pops().iter().any(|p| p.owner == owner(shared))
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        let _ = shared;
        false
    }
}

/// Opens the history window, or closes it when it is open.
pub fn toggle(shared: &Arc<Shared>) {
    #[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
    {
        if is_open(shared) {
            drop(take(|p| p.owner == owner(shared)));
            return;
        }
        let made = shared.clone();
        let make: Make = Box::new(move || Box::new(HistoryWindow::new(made)));
        if let Some(win) = Window::open(make, "OpenClip Multiband History", load()) {
            pops().push(Pop { owner: owner(shared), win });
        }
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    let _ = shared;
}

/// The instance is going away: close its windows before its code can be unloaded.
pub fn close_all(shared: &Shared) {
    #[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
    drop(take(|p| p.owner == owner(shared)));
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    let _ = shared;
}

/// `x y w h` of the window last time, in the platform's own screen units.
fn load() -> Option<[i32; 4]> {
    parse(&std::fs::read_to_string(pref_file("pops")?).ok()?)
}

fn parse(text: &str) -> Option<[i32; 4]> {
    let line = text.lines().find(|l| l.split_whitespace().next() == Some("history"))?;
    let v: Vec<i32> = line.split_whitespace().skip(1).filter_map(|n| n.parse().ok()).collect();
    (v.len() == 4 && v[2] > 0 && v[3] > 0).then(|| [v[0], v[1], v[2], v[3]])
}

fn save(f: [i32; 4]) {
    if cfg!(test) {
        return;
    }
    let Some(path) = pref_file("pops") else { return };
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let _ = std::fs::write(path, format!("history {} {} {} {}\n", f[0], f[1], f[2], f[3]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_reads_back() {
        assert_eq!(parse("history 10 -20 500 560\n"), Some([10, -20, 500, 560]));
        // A zero size is no frame.
        assert_eq!(parse("history 1 2 0 5\n"), None);
        assert_eq!(parse(""), None);
    }
}
