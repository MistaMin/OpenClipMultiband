//! ST-9: host behaviour the conformance kit does not drive.

use openclipmultiband::OpenClipMultiband;
use openplugin::prelude::*;

fn stereo(frames: usize, value: f32) -> Vec<Vec<f32>> {
    vec![vec![value; frames]; 2]
}

fn run(plugin: &mut OpenClipMultiband, buffers: &mut [Vec<f32>], frames: usize) {
    let mut io: Vec<&mut [f32]> = buffers.iter_mut().map(|c| c.as_mut_slice()).collect();
    plugin.process(&mut io, frames);
}

fn finite(buffers: &[Vec<f32>]) -> bool {
    buffers.iter().flatten().all(|s| s.is_finite())
}

#[test]
fn process_before_prepare_is_defined() {
    let mut plugin = OpenClipMultiband::default();
    let mut buffers = stereo(64, 0.5);
    run(&mut plugin, &mut buffers, 64);
    assert!(finite(&buffers));
}

#[test]
fn a_rate_change_without_a_reset_keeps_working() {
    let mut plugin = OpenClipMultiband::default();
    plugin.prepare(44100.0, 512, 2);
    run(&mut plugin, &mut stereo(512, 0.25), 512);
    plugin.prepare(96000.0, 512, 2); // a device switch: no reset in between
    for _ in 0..200 {
        let mut buffers = stereo(512, 0.25);
        run(&mut plugin, &mut buffers, 512);
        assert!(finite(&buffers));
    }
}

#[test]
fn every_frame_count_up_to_the_buffer_length_works() {
    let mut plugin = OpenClipMultiband::default();
    plugin.prepare(48000.0, 512, 2);
    for frames in [1usize, 2, 3, 7, 64, 333, 511, 512] {
        let mut buffers = stereo(512, 0.1);
        run(&mut plugin, &mut buffers, frames);
        assert!(finite(&buffers), "frames = {frames}");
    }
}

/// RMS of the last block after one NaN sample in a 1 kHz sine, next to a twin that got no NaN.
/// `after` is set on the second block, once the NaN is in.
fn after_one_nan(before: &[(&str, f32)], after: &[(&str, f32)], secs: f32) -> [f32; 2] {
    let sr = 48000.0;
    let mut rms = [0.0; 2];
    for (twin, out) in rms.iter_mut().enumerate() {
        let mut plugin = OpenClipMultiband::default();
        plugin.prepare(sr, 512, 2);
        for &(id, v) in before {
            plugin.set_param(id, v);
        }
        for b in 0..1 + (secs * sr) as usize / 512 {
            if b == 1 {
                for &(id, v) in after {
                    plugin.set_param(id, v);
                }
            }
            let sine: Vec<f32> =
                (0..512).map(|i| 0.5 * (std::f32::consts::TAU * 1000.0 * (b * 512 + i) as f32 / sr).sin()).collect();
            let mut buffers = vec![sine.clone(), sine];
            if b == 0 && twin == 0 {
                buffers[0][100] = f32::NAN;
            }
            run(&mut plugin, &mut buffers, 512);
            *out = (buffers[0].iter().map(|s| s * s).sum::<f32>() / 512.0).sqrt();
        }
    }
    rms
}

fn same_level([got, want]: [f32; 2]) -> bool {
    (20.0 * (got / want).log10()).abs() < 1.0
}

#[test]
fn one_nan_sample_does_not_silence_the_output() {
    for ceiling in [1.0, 0.0] {
        let rms = after_one_nan(&[("outCeilingOn", ceiling)], &[], 1.0);
        assert!(same_level(rms), "Output Ceiling On = {ceiling}: {rms:?} one second after the NaN");
    }
}

#[test]
fn one_nan_sample_does_not_freeze_auto_gain() {
    let rms = after_one_nan(&[("autoGain", 1.0)], &[("output", 12.0)], 3.0);
    assert!(same_level(rms), "{rms:?}: Auto Gain did not follow Output Gain after the NaN");
}

/// The AU and AAX shells size their bypass delay from `max_latency`: no setting may report more.
#[test]
fn latency_never_exceeds_max_latency() {
    for sr in [8_000.0, 11_025.0, 44_100.0, 192_000.0] {
        let mut p = OpenClipMultiband::default();
        p.prepare(sr, 256, 2);
        let max = p.max_latency().expect("max latency");
        for algo in 0..6 {
            for aa in 0..3 {
                for os in 0..6 {
                    for (id, v) in [("algorithm", algo as f32), ("aa", aa as f32), ("oversampling", os as f32)] {
                        assert!(p.set_param(id, v), "{id}");
                    }
                    assert!(p.latency() <= max, "{sr} Hz algo {algo} aa {aa} os {os}: {} > {max}", p.latency());
                }
            }
        }
    }
}

#[test]
fn rates_below_8_khz_are_refused() {
    let p = OpenClipMultiband::default();
    assert!(!p.accepts_sample_rate(4_000.0));
    assert!(p.accepts_sample_rate(8_000.0) && p.accepts_sample_rate(192_000.0));
}
