//! TEMPORARY (QA 2026-11-27, finding Q5): crossbuild.py's heavy -> alt sequence, in process.

use openclipmultiband::{OpenClipMultiband, INFO};
use openplugin::core::Plugin;

const HEAVY: &[(usize, f32)] = &[(0, 0.7), (5, 1.0), (12, 0.6), (13, 0.3), (14, 0.5), (15, 0.5), (16, 0.9), (17, 0.6), (21, 0.5), (22, 0.6), (41, 0.6)];
const ALT: &[(usize, f32)] = &[(2, 1.0), (5, 0.4), (12, 0.4), (24, 1.0), (17, 0.5), (41, 0.7)];

fn signal() -> Vec<f32> {
    let path = std::env::var("F9_SIGNAL").expect("F9_SIGNAL: raw f32 stereo");
    std::fs::read(path).unwrap().chunks_exact(4).map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]])).collect()
}

/// Writes a setting the way crossbuild.py does: every key of either setting, the setting's value or the default.
fn write(p: &mut OpenClipMultiband, setting: &[(usize, f32)]) {
    let keys: Vec<usize> = HEAVY.iter().chain(ALT).map(|k| k.0).collect();
    for &i in &keys {
        let info = &INFO.params[i];
        let plain = setting.iter().find(|k| k.0 == i).map_or(info.default, |k| info.from_normalized(k.1));
        p.set_param(info.id, plain);
    }
}

/// 4 s render of the 3 s signal; returns the peak of each half second.
fn render(p: &mut OpenClipMultiband, x: &[f32], block: usize) -> Vec<f32> {
    let n = 4 * 48000;
    let (mut l, mut r) = (vec![0f32; block], vec![0f32; block]);
    let mut peaks = vec![0f32; 8];
    let mut t = 0;
    while t < n {
        let m = block.min(n - t);
        for i in 0..m {
            let k = t + i;
            l[i] = x.get(2 * k).copied().unwrap_or(0.0);
            r[i] = x.get(2 * k + 1).copied().unwrap_or(0.0);
        }
        p.process(&mut [&mut l[..m], &mut r[..m]], m);
        for i in 0..m {
            let v = if l[i].is_finite() { l[i].abs() } else { f32::INFINITY };
            peaks[(t + i) / 24000] = peaks[(t + i) / 24000].max(v);
        }
        t += m;
    }
    peaks
}

#[test]
#[ignore]
fn heavy_then_alt() {
    let x = signal();
    for block in [128usize, 512, 1024] {
        for reprepare in [false, true] {
            let mut p = OpenClipMultiband::default();
            p.prepare(48000.0, block, 2);
            let mut out = Vec::new();
            for (name, setting) in [("default", &[][..]), ("heavy", HEAVY), ("alt", ALT)] {
                let lat = p.latency();
                write(&mut p, setting);
                if reprepare && p.latency() != lat {
                    p.prepare(48000.0, block, 2); // a host that restarts the plug-in on a latency change
                }
                for take in ["warmup", "render"] {
                    p.reset();
                    let peaks = render(&mut p, &x, block);
                    out.push(format!("{name}/{take}: {}", peaks.iter().map(|v| format!("{:.0}", 20.0 * v.max(1e-12).log10())).collect::<Vec<_>>().join(" ")));
                }
            }
            println!("block {block}, re-prepare on latency change {reprepare}:\n  {}", out.join("\n  "));
        }
    }
}
