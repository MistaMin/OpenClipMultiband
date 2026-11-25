use super::*;
use clip_api::bench;
use std::f64::consts::PI;

#[test]
fn master_contract() {
    bench::assert_contract(&ENGINES[0].1);
}

fn prepared(rate: f32, c: f32, shape: f32, character: f32) -> Master {
    let mut engine = Master::new();
    engine.prepare(rate, 512);
    engine.set(c, shape, character);
    engine
}

#[test]
fn interval_average_matches_quadrature_and_is_odd() {
    for shape in [0.0, 0.5, 1.0] {
        let ctl = prepared(48000.0, 0.5, shape, 0.5).coefficients();
        let c = ctl.ceiling;
        let w = ctl.width;
        for mid in [0.0, c - w, c, c + w, 1.3 * c, 8.0 * c] {
            for radius in [
                0.0,
                0.0002 * w,
                0.0021 * w,
                0.01 * w,
                0.3 * w,
                2.0 * w,
                1.5 * c,
                8.0 * c,
            ] {
                let mean = ctl.mean(mid, radius);
                let reference = (0..4096)
                    .map(|i| ctl.clip(mid + radius * ((i as f32 + 0.5) / 2048.0 - 1.0)) as f64)
                    .sum::<f64>()
                    / 4096.0;
                assert!(
                    (mean as f64 - reference).abs() < 0.00015 * c as f64,
                    "mid {mid}, radius {radius}: {mean} versus {reference}"
                );
                assert_eq!(mean, -ctl.mean(-mid, radius));
            }
        }
    }
}

#[test]
fn latency_flatness_and_dc() {
    for rate in [44100.0, 48000.0, 96000.0, 192000.0] {
        let mut e = prepared(rate, 1.0, 0.5, 0.5);
        let error = bench::flatness_db(&mut e, rate as f64);
        println!("{rate} Hz worst small-signal amplitude error {error:.5} dB");
        assert!(error <= 0.05);
        e.reset();
        let mut x = vec![0.0; 640];
        x[192] = 0.01;
        let y = bench::run(&mut e, &x, &[7, 211]);
        let peak = y
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.abs().total_cmp(&b.abs()))
            .unwrap()
            .0;
        assert_eq!(peak, 192 + e.latency());
        let mut sweep_error = 0.0_f64;
        for bin in 1..=2000 {
            let w = 2.0 * PI * (bin as f64 * 10.0) / rate as f64;
            let gain = y
                .iter()
                .enumerate()
                .map(|(i, &v)| v as f64 * (w * (i as f64 - peak as f64)).cos())
                .sum::<f64>()
                / 0.01;
            sweep_error = sweep_error.max((20.0 * gain.log10()).abs());
        }
        println!("{rate} Hz, 10 Hz grid impulse-response error {sweep_error:.5} dB");
        assert!(sweep_error < 0.05);
        for k in 1..96 {
            assert!((y[peak + k] - y[peak - k]).abs() < 2e-8);
        }
        e.reset();
        let y = bench::run(&mut e, &[0.125; 640], &[151]);
        assert!((y[639] - 0.125).abs() < 2e-7);
        let y = bench::run(&mut e, &[0.0; 640], &[289]);
        assert_eq!(y[639], 0.0);
    }
}

#[test]
fn stereo_link_symmetry_and_recovery() {
    fn render(swap: bool, character: f32) -> (Vec<f32>, Vec<f32>) {
        let n = 16384;
        let mut bed = vec![0.25; n];
        let mut transient = (0..n)
            .map(|i| {
                if (1024..3072).contains(&i) {
                    if i % 13 < 6 {
                        12.0
                    } else {
                        -12.0
                    }
                } else {
                    0.0
                }
            })
            .collect::<Vec<_>>();
        let mut e = prepared(48000.0, 1.0, 0.5, character);
        if swap {
            for (a, b) in bed.iter_mut().zip(&mut transient) {
                *a = -*a;
                *b = -*b;
            }
            e.process(&mut transient, &mut bed);
        } else {
            e.process(&mut bed, &mut transient);
        }
        (bed, transient)
    }
    let (bed, hot) = render(false, 0.0);
    let (neg_bed, neg_hot) = render(true, 0.0);
    for (a, b) in bed.iter().chain(&hot).zip(neg_bed.iter().chain(&neg_hot)) {
        assert_eq!(*a, -*b);
    }
    let (slow, _) = render(false, 1.0);
    assert!(bed[3000] < 0.24 && slow[3500] < bed[3500] - 0.005);
    assert!((bed[12000] - 0.25).abs() < 1e-6);
}

#[test]
fn automation_obeys_rail_and_is_partition_invariant() {
    fn render(blocks: &[usize]) -> Vec<f32> {
        let mut e = prepared(48000.0, 1.0, 0.0, 0.0);
        let mut y = Vec::new();
        for (c, s, k) in [
            (1.0, 0.0, 0.0),
            (1.0, 1.0, 1.0),
            (0.06, 0.5, 0.0),
            (0.6, 0.0, 0.3),
            (2.0, -1.0, 2.0),
        ] {
            e.set(c, s, k);
            let section = bench::run(&mut e, &[0.98; 2600], blocks);
            assert!(section.iter().all(|x| x.abs() <= c.min(1.0)));
            y.extend(section);
        }
        y
    }
    let a = render(&[512]);
    let b = render(&[1, 7, 55, 213]);
    assert!(a.iter().zip(b).all(|(v, w)| (v - w).abs() < 1e-6));
    let max_step = a[2599..5200]
        .windows(2)
        .map(|w| (w[1] - w[0]).abs())
        .fold(0.0, f32::max);
    assert!(max_step < 0.0006 && a[2599] - a[5199] > 0.03);
}

fn finer_peak(signal: &[f32]) -> f64 {
    let mut peak = signal.iter().map(|s| s.abs() as f64).fold(0.0, f64::max);
    for phase in 1..16 {
        let h: [f64; 128] = std::array::from_fn(|i| {
            let t = i as f64 - 63.0 - phase as f64 / 16.0;
            let a = PI * t;
            a.sin() / a * (0.5 + 0.5 * (a / 64.0).cos())
        });
        for w in signal.windows(h.len()) {
            let value = w
                .iter()
                .zip(h.iter())
                .map(|(&x, &f)| x as f64 * f)
                .sum::<f64>();
            peak = peak.max(value.abs());
        }
    }
    peak
}

#[test]
fn independent_peak_grid_and_sample_rail() {
    let mut largest = 0.0_f64;
    for rate in [44100.0, 48000.0, 96000.0, 192000.0] {
        for (c, shape, character) in [(0.01, 0.0, 1.0), (0.47, 1.0, 0.0), (1.0, 0.5, 0.5)] {
            let mut e = prepared(rate, c, shape, character);
            let mut state = 11543_u32;
            let x = (0..8192)
                .map(|i| {
                    state = state.wrapping_mul(1664525).wrapping_add(1013904223);
                    let phase = i as f64 * 2.0 * PI;
                    let sample = match i / 1024 {
                        0 | 7 => 0.0,
                        1 => {
                            if i % 22 < 11 {
                                11.0
                            } else {
                                -11.0
                            }
                        }
                        2 => 6.0 * (phase * 0.439 + 0.37).sin(),
                        3 => (phase * 0.177 + 0.12).sin() + 2.0 * (phase * 0.459 + 0.83).cos(),
                        4 => (state as i32 as f64 / i32::MAX as f64) * 10.0,
                        5 => {
                            if i % 63 == 0 {
                                19.0
                            } else {
                                0.0
                            }
                        }
                        _ => 2.0 * (phase * (0.11 + i as f64 / 100000.0)).sin(),
                    };
                    sample as f32 * c
                })
                .collect::<Vec<_>>();
            let y = bench::run(&mut e, &x, &[31, 256, 1, 7]);
            assert!(y.iter().all(|v| v.is_finite() && v.abs() <= c));
            let ratio = finer_peak(&y) / c as f64;
            largest = largest.max(ratio);
            println!(
                "independent 16x peak, {rate} Hz, shape {shape}, character {character}: {ratio:.5}"
            );
        }
    }
    println!("Independent 16x peak maximum {largest:.5}");
    assert!(largest <= 1.02);
}

#[test]
fn phase_shifted_bursts_at_three_ceilings() {
    let mut worst = 0.0_f64;
    for rate in [44_100.0, 48_000.0] {
        for (c, shape, character) in [(0.0631, 0.25, 0.75), (0.81, 0.8, 0.1), (1.0, 0.0, 0.0)] {
            let mut e = prepared(rate, c, shape, character);
            let mut seed = 0x5ac37_u32;
            let x = (0..16384)
                .map(|i| {
                    seed = seed.wrapping_mul(22695477).wrapping_add(1);
                    let envelope = if i % 251 < 123 { 4.9 } else { 0.57 };
                    let n = i as f64;
                    let hz = [901.7, 6793.1, 11119.2, 18739.3, 20413.4];
                    let mut v = 0.0;
                    for (k, f) in hz.iter().enumerate() {
                        v += (n * (2.0 * PI * f / rate as f64) + (k * k) as f64 * 0.73).sin() / 3.0;
                    }
                    let burst = if i % 499 < 7 {
                        seed as i32 as f64 / i32::MAX as f64 * 3.6
                    } else {
                        0.0
                    };
                    c * (envelope * v + burst) as f32
                })
                .collect::<Vec<_>>();
            let y = bench::run(&mut e, &x, &[63, 1, 512]);
            worst = worst.max(finer_peak(&y) / c as f64);
        }
    }
    println!("Fresh 16x multitone/burst peak maximum {worst:.5}");
    assert!(worst <= 1.02);
}

#[test]
fn alias_rejection_at_master_defaults() {
    let mut engine = prepared(48000.0, 1.0, 0.5, 0.5);
    for hz in [1000.0, 2500.0, 5000.0, 8000.0, 12000.0] {
        for (drive, limit) in [(12.0, -60.0), (24.0, -55.0)] {
            let alias = bench::alias_dbc(&mut engine, 48000.0, hz, drive);
            assert!(alias < limit, "{hz} Hz, {drive} dB: {alias} dBc");
        }
    }
}
#[test]
fn bench_peak_and_step_train() {
    let (sp, tp) = bench::rail(&ENGINES[0].1, 48000.0);
    println!("Shared rail, sample {sp:.8}, 4x peak {tp:.8}");
    assert!(sp <= 1.0 && tp <= 1.02, "sample {sp}, interpolated {tp}");
    let x = (0..8192)
        .map(|n| if n / 37 % 2 == 0 { 7.924466 } else { -7.924466 })
        .collect::<Vec<_>>();
    for c in [0.5, 1.0] {
        let mut e = prepared(48000.0, c, 0.5, 0.5);
        let y = bench::run(&mut e, &x, &[512]);
        let tp = finer_peak(&y) / c as f64;
        println!("37-sample step train at ceiling {c}: {tp:.5} on 16x grid");
        assert!(tp <= 1.02);
    }
}

#[test]
#[ignore]
fn report() {
    for (shape, character) in [(0.5, 0.5), (0.0, 0.0), (1.0, 1.0)] {
        println!("{}", bench::report(&ENGINES[0].1, shape, character));
    }
}
