//! TEMPORARY (QA 2026-11-27, finding Q5): is writing a parameter's current value again a no-op?

use openclipmultiband::{OpenClipMultiband, INFO};
use openplugin::core::Plugin;

const HEAVY: &[(usize, f32)] = &[(0, 0.7), (5, 1.0), (12, 0.6), (13, 0.3), (14, 0.5), (15, 0.5), (16, 0.9), (17, 0.6), (21, 0.5), (22, 0.6), (41, 0.6)];
const ALT: &[(usize, f32)] = &[(2, 1.0), (5, 0.4), (12, 0.4), (24, 1.0), (17, 0.5), (41, 0.7)];

fn noise(n: usize) -> Vec<f32> {
    let mut s = 1u32;
    (0..n).map(|i| {
        s = s.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        0.4 * ((s >> 8) as f32 / 8_388_608.0 - 1.0) + 0.3 * (i as f32 * 0.05).sin()
    }).collect()
}

fn render(p: &mut OpenClipMultiband, x: &[f32]) -> Vec<f32> {
    let mut out = Vec::with_capacity(x.len() * 2);
    for chunk in x.chunks(512) {
        let (mut l, mut r) = (chunk.to_vec(), chunk.to_vec());
        p.process(&mut [&mut l[..], &mut r[..]], chunk.len());
        out.extend(l.iter().chain(&r));
    }
    out
}

fn fresh(setting: &[(usize, f32)]) -> OpenClipMultiband {
    let mut p = OpenClipMultiband::default();
    p.prepare(48000.0, 512, 2);
    for &(i, v) in setting {
        let info = &INFO.params[i];
        p.set_param(info.id, info.from_normalized(v));
    }
    let _ = render(&mut p, &noise(48000)); // settle
    p
}

#[test]
#[ignore]
fn rewriting_a_current_value_changes_nothing() {
    let x = noise(48000);
    for (name, setting) in [("default", &[][..]), ("heavy", HEAVY), ("alt", ALT)] {
        let mut a = fresh(setting);
        a.reset();
        let want = render(&mut a, &x);
        let mut bad = Vec::new();
        for (i, info) in INFO.params.iter().enumerate() {
            let mut b = fresh(setting);
            b.reset();
            let v = b.param(info.id).unwrap();
            b.set_param(info.id, v);
            let got = render(&mut b, &x);
            let d = want.iter().zip(&got).map(|(a, b)| (a - b).abs()).fold(0f32, f32::max);
            if d > 0.0 {
                bad.push(format!("{i} {} ({:.0} dB)", info.id, 20.0 * d.log10()));
            }
        }
        println!("{name}: {} parameters not idempotent after a reset: {}", bad.len(), bad.join(", "));
    }
}
