//! The contract every OpenClip Multiband clipping engine implements, and the bench that
//! measures all of them the same way.
//!
//! The plugin splits audio into bands; each band runs one `ClipEngine` (and the summed output
//! runs another). The plugin applies the band's drive gain before the engine and its output
//! trim after it, so an engine only clips: it maps an input that may be far above `ceiling` to
//! an output that stays at or under it.
//!
//! Rules for an implementation:
//! * `prepare` allocates everything. `set`, `process` and `reset` never allocate, lock, block or
//!   panic. Index with masks or checked lengths.
//! * `process` works in place on two equal-length slices no longer than `max_block`, and gives
//!   the same output whatever the block sizes (state lives per sample, not per block).
//! * `set` may be called before every block with new values and must not click.
//! * `latency` is fixed after `prepare` and `set_quality` (it may depend on the rate and quality).
//! * Silence in gives silence out (no DC left behind by asymmetric curves).

/// One clipping algorithm.
pub trait ClipEngine: Send {
    /// The selector label: at most 12 characters, no product or brand names.
    fn name(&self) -> &'static str;

    /// Allocates for this sample rate and block size, and clears state.
    fn prepare(&mut self, sample_rate: f32, max_block: usize);

    /// Clears histories. Keeps settings.
    fn reset(&mut self);

    /// `ceiling`: the linear peak the output is held to (0.01..=1.0; 1.0 is 0 dBFS).
    /// `shape` and `character`: 0..=1 macro controls whose meaning the engine documents
    /// (for example knee width and asymmetry). Out-of-range values are clamped.
    fn set(&mut self, ceiling: f32, shape: f32, character: f32);

    /// Samples between input and output.
    fn latency(&self) -> usize;

    /// Clips one block in place.
    fn process(&mut self, left: &mut [f32], right: &mut [f32]);

    /// Anti-aliasing method (index into `shaper::AA_LABELS`) and oversampling (index into
    /// `shaper::OS_LABELS`). Clears state and may change `latency`. Never allocates. Engines with
    /// their own fixed anti-aliasing ignore it.
    fn set_quality(&mut self, _aa: usize, _os: usize) {}

    /// Twin mode: the same filters and stages with a straight-line curve, so the dry path carries
    /// exactly the engine's linear response. Engines that cannot do this ignore it.
    fn set_twin(&mut self, _on: bool) {}

    /// A drawn curve (`shaper::CURVE_POINTS` outputs, 0..=1 of the ceiling) in place of the
    /// algorithm's own, or `None` for the algorithm's curve. Never allocates.
    fn set_custom(&mut self, _points: Option<&[f32]>) {}
}

pub mod shaper;

pub mod bench {
    //! Measurements shared by every engine crate. Call `report` from an ignored test and paste
    //! its table into the crate's REPORT.md; call `assert_contract` from a normal test.
    use super::ClipEngine;
    use std::f64::consts::PI;
    use std::fmt::Write;

    pub const FS: f32 = 48_000.0;
    const BLOCK: usize = 512;
    const N: usize = 16_384;
    /// Drives (dB) applied to a -6 dBFS tone in the alias table.
    pub const DRIVES_DB: [f64; 4] = [0.0, 6.0, 12.0, 24.0];
    const ALIAS_TONES_HZ: [f64; 5] = [1_000.0, 2_500.0, 5_000.0, 8_000.0, 12_000.0];

    fn fft(re: &mut [f64], im: &mut [f64]) {
        let n = re.len();
        let mut j = 0;
        for i in 1..n {
            let mut bit = n >> 1;
            while j & bit != 0 {
                j ^= bit;
                bit >>= 1;
            }
            j |= bit;
            if i < j {
                re.swap(i, j);
                im.swap(i, j);
            }
        }
        let mut len = 2;
        while len <= n {
            let (ws, wc) = (-2.0 * PI / len as f64).sin_cos();
            for s in (0..n).step_by(len) {
                let (mut cr, mut ci) = (1.0, 0.0);
                for k in 0..len / 2 {
                    let (a, b) = (s + k, s + k + len / 2);
                    let tr = re[b] * cr - im[b] * ci;
                    let ti = re[b] * ci + im[b] * cr;
                    re[b] = re[a] - tr;
                    im[b] = im[a] - ti;
                    re[a] += tr;
                    im[a] += ti;
                    let nc = cr * wc - ci * ws;
                    ci = cr * ws + ci * wc;
                    cr = nc;
                }
            }
            len <<= 1;
        }
    }

    /// Power spectrum of `x` (length N), bins 0..N/2.
    fn power(x: &[f64]) -> Vec<f64> {
        let mut re = x.to_vec();
        let mut im = vec![0.0; x.len()];
        fft(&mut re, &mut im);
        (0..x.len() / 2).map(|b| re[b] * re[b] + im[b] * im[b]).collect()
    }

    /// An odd FFT bin near `hz`, so the tone is coherent and its harmonics never share a bin.
    fn bin_for(hz: f64, fs: f64) -> usize {
        ((hz * N as f64 / fs).round() as usize) | 1
    }

    /// Runs `input` through the engine in blocks of the given sizes (cycled), stereo, identical
    /// channels. Returns the left output.
    pub fn run(e: &mut dyn ClipEngine, input: &[f32], blocks: &[usize]) -> Vec<f32> {
        let mut l = input.to_vec();
        let mut r = input.to_vec();
        let (mut at, mut i) = (0, 0);
        while at < l.len() {
            let n = blocks[i % blocks.len()].min(BLOCK).min(l.len() - at).max(1);
            e.process(&mut l[at..at + n], &mut r[at..at + n]);
            at += n;
            i += 1;
        }
        l
    }

    /// The last N output samples of a steady tone (`amp` peak, coherent bin), after warm-up.
    fn tone(e: &mut dyn ClipEngine, bin: usize, amp: f64) -> Vec<f64> {
        e.reset();
        let total = 2 * N + e.latency();
        let x: Vec<f32> = (0..total).map(|t| (amp * (2.0 * PI * (bin * t) as f64 / N as f64).sin()) as f32).collect();
        let y = run(e, &x, &[BLOCK]);
        y[total - N..].iter().map(|&v| v as f64).collect()
    }

    fn db(p: f64) -> f64 {
        10.0 * p.max(1e-30).log10()
    }

    /// Alias power (every in-band 20 Hz..20 kHz bin that is not DC, the tone or a harmonic)
    /// relative to the fundamental, dBc.
    pub fn alias_dbc(e: &mut dyn ClipEngine, fs: f64, hz: f64, drive_db: f64) -> f64 {
        let bin = bin_for(hz, fs);
        let y = tone(e, bin, 0.5 * 10f64.powf(drive_db / 20.0));
        let p = power(&y);
        let (lo, hi) = ((20.0 * N as f64 / fs) as usize, ((20_000.0 * N as f64 / fs) as usize).min(N / 2 - 1));
        let alias: f64 = (lo..=hi).filter(|b| b % bin != 0).map(|b| p[b]).sum();
        db(alias / p[bin])
    }

    /// Harmonic levels H2..=H9 of a tone, dBc.
    pub fn harmonics_dbc(e: &mut dyn ClipEngine, fs: f64, hz: f64, drive_db: f64) -> [f64; 8] {
        let bin = bin_for(hz, fs);
        let y = tone(e, bin, 0.5 * 10f64.powf(drive_db / 20.0));
        let p = power(&y);
        let mut h = [f64::NEG_INFINITY; 8];
        for (i, v) in h.iter_mut().enumerate() {
            let b = bin * (i + 2);
            if b < N / 2 {
                *v = db(p[b] / p[bin]);
            }
        }
        h
    }

    /// Largest small-signal (-40 dBFS) level error across 20 Hz..20 kHz, dB.
    pub fn flatness_db(e: &mut dyn ClipEngine, fs: f64) -> f64 {
        let amp = 0.01;
        let mut worst: f64 = 0.0;
        for i in 0..16 {
            let hz = 20.0 * 1000f64.powf(i as f64 / 15.0);
            let bin = bin_for(hz, fs).max(1);
            let p = power(&tone(e, bin, amp));
            let gain = p[bin].sqrt() * 2.0 / N as f64 / amp;
            worst = worst.max((20.0 * gain.log10()).abs());
        }
        worst
    }

    fn noise(n: usize, rms: f64, seed: u64) -> Vec<f32> {
        let mut s = seed | 1;
        (0..n)
            .map(|_| {
                s ^= s << 13;
                s ^= s >> 7;
                s ^= s << 17;
                // Sum of 4 uniforms: near-Gaussian with unit variance.
                let mut v = 0.0;
                for k in 0..4 {
                    v += ((s >> (16 * k)) & 0xffff) as f64 / 65_535.0 - 0.5;
                }
                (v * 3f64.sqrt() * rms) as f32
            })
            .collect()
    }

    /// 4x windowed-sinc interpolated peak of `y` (an estimate of the true peak).
    fn true_peak(y: &[f32]) -> f64 {
        const TAPS: isize = 16;
        let mut peak: f64 = 0.0;
        for n in TAPS as usize..y.len().saturating_sub(TAPS as usize) {
            peak = peak.max((y[n] as f64).abs());
            for ph in 1..4 {
                let frac = ph as f64 / 4.0;
                let mut acc = 0.0;
                for k in -TAPS + 1..=TAPS {
                    let t = k as f64 - frac;
                    let w = 0.5 + 0.5 * (PI * t / TAPS as f64).cos();
                    acc += y[(n as isize + k) as usize] as f64 * (PI * t).sin() / (PI * t) * w;
                }
                peak = peak.max(acc.abs());
            }
        }
        peak
    }

    /// Worst output peak relative to the ceiling over sines, noise and steps at +24 dB drive,
    /// at ceilings 1.0 and 0.5: (sample peak ratio, true peak ratio).
    pub fn rail(make: &dyn Fn() -> Box<dyn ClipEngine>, fs: f64) -> (f64, f64) {
        let g = 10f64.powf(24.0 / 20.0);
        let len = 24_000;
        let mut signals: Vec<Vec<f32>> = [1_000.0, 5_000.0, 10_000.0]
            .iter()
            .map(|hz| (0..len).map(|t| (0.5 * g * (2.0 * PI * hz * t as f64 / fs).sin()) as f32).collect())
            .collect();
        signals.push(noise(len, 0.5 * g, 7));
        signals.push((0..len).map(|t| if (t / 37) % 2 == 0 { (0.5 * g) as f32 } else { (-0.5 * g) as f32 }).collect());
        let (mut sp, mut tp): (f64, f64) = (0.0, 0.0);
        for ceiling in [1.0f32, 0.5] {
            for x in &signals {
                let mut e = make();
                e.prepare(fs as f32, BLOCK);
                e.set(ceiling, 0.5, 0.5);
                let y = run(e.as_mut(), x, &[BLOCK]);
                let pk = y.iter().fold(0.0f64, |m, &v| m.max((v as f64).abs()));
                sp = sp.max(pk / ceiling as f64);
                tp = tp.max(true_peak(&y) / ceiling as f64);
            }
        }
        (sp, tp)
    }

    /// Nanoseconds per stereo frame on 10 s of noise at +12 dB.
    pub fn cpu_ns(e: &mut dyn ClipEngine, fs: f64) -> f64 {
        let len = (fs * 10.0) as usize;
        let x = noise(len, 0.5 * 4.0, 3);
        e.reset();
        let t = std::time::Instant::now();
        let y = run(e, &x, &[BLOCK]);
        let ns = t.elapsed().as_nanos() as f64 / len as f64;
        std::hint::black_box(y);
        ns
    }

    /// The checks every engine must pass, at each sample rate and at three settings.
    pub fn assert_contract(make: &dyn Fn() -> Box<dyn ClipEngine>) {
        for fs in [44_100.0f32, 48_000.0, 96_000.0, 192_000.0] {
            for (shape, character) in [(0.0, 0.0), (0.5, 0.5), (1.0, 1.0)] {
                let mut e = make();
                e.prepare(fs, BLOCK);
                e.set(1.0, shape, character);
                let lat = e.latency();
                let x = noise(9_000, 2.0, 11);
                let a = run(e.as_mut(), &x, &[BLOCK]);
                assert!(a.iter().all(|v| v.is_finite()), "{}: non-finite output", e.name());
                e.reset();
                let b = run(e.as_mut(), &x, &[1, 3, 64, 17, 511, 2]);
                let diff = a.iter().zip(&b).fold(0.0f32, |m, (p, q)| m.max((p - q).abs()));
                assert!(diff <= 1e-6, "{}: output depends on block size ({diff})", e.name());
                assert_eq!(lat, e.latency(), "{}: latency changed", e.name());
                // Huge, tiny and zero input stay finite; silence returns to silence.
                let mut wild: Vec<f32> = noise(4_000, 1e4, 5);
                wild.extend(std::iter::repeat(1e-38).take(2_000));
                wild.extend(std::iter::repeat(0.0).take(fs as usize / 2));
                let y = run(e.as_mut(), &wild, &[BLOCK]);
                assert!(y.iter().all(|v| v.is_finite()), "{}: non-finite on extreme input", e.name());
                let tail = y[y.len() - 256..].iter().fold(0.0f32, |m, v| m.max(v.abs()));
                assert!(tail < 1e-6, "{}: silence does not return to silence ({tail})", e.name());
                // Settings changing every block stay finite and bounded.
                let mut l = noise(BLOCK, 2.0, 9);
                let mut r = l.clone();
                for k in 0..64 {
                    let v = (k % 7) as f32 / 6.0;
                    e.set(0.05 + 0.95 * v, 1.0 - v, v);
                    e.process(&mut l, &mut r);
                    assert!(l.iter().chain(&r).all(|s| s.is_finite() && s.abs() < 8.0), "{}: unstable when set() moves", e.name());
                }
            }
        }
    }

    /// One markdown report of every measurement at 48 kHz, ceiling 0 dBFS, shape and character
    /// at the given values.
    pub fn report(make: &dyn Fn() -> Box<dyn ClipEngine>, shape: f32, character: f32) -> String {
        let fs = FS as f64;
        let mut e = make();
        e.prepare(FS, BLOCK);
        e.set(1.0, shape, character);
        let mut s = String::new();
        let _ = writeln!(s, "### {} (shape {shape}, character {character}, 48 kHz, latency {} smp)\n", e.name(), e.latency());
        let _ = writeln!(s, "| Worst alias dBc over 1k..12k tones, -6 dBFS + drive | 0 dB | +6 dB | +12 dB | +24 dB |");
        let _ = writeln!(s, "|---|---:|---:|---:|---:|");
        let mut row = String::from("| alias |");
        for d in DRIVES_DB {
            let worst = ALIAS_TONES_HZ.iter().map(|&hz| alias_dbc(e.as_mut(), fs, hz, d)).fold(f64::NEG_INFINITY, f64::max);
            let _ = write!(row, " {worst:.1} |");
        }
        let _ = writeln!(s, "{row}\n");
        let h = harmonics_dbc(e.as_mut(), fs, 1_000.0, 12.0);
        let _ = writeln!(s, "1 kHz at +12 dB, H2..H9 dBc: {}", h.iter().map(|v| format!("{v:.1}")).collect::<Vec<_>>().join(" / "));
        let _ = writeln!(s, "Small-signal (-40 dBFS) worst level error 20 Hz..20 kHz: {:.4} dB", flatness_db(e.as_mut(), fs));
        let (sp, tp) = rail(make, fs);
        let _ = writeln!(s, "Worst peak / ceiling at +24 dB (sines, noise, steps; ceilings 1.0 and 0.5): sample {sp:.4}x, true-peak {tp:.4}x");
        let _ = writeln!(s, "CPU: {:.1} ns per stereo frame (noise, +12 dB, release build)", cpu_ns(e.as_mut(), fs));
        s
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        /// A plain hard clip: the reference row and a check of the bench itself.
        struct Hard(f32);
        impl ClipEngine for Hard {
            fn name(&self) -> &'static str {
                "Hard"
            }
            fn prepare(&mut self, _: f32, _: usize) {}
            fn reset(&mut self) {}
            fn set(&mut self, c: f32, _: f32, _: f32) {
                self.0 = c.clamp(0.01, 1.0);
            }
            fn latency(&self) -> usize {
                0
            }
            fn process(&mut self, l: &mut [f32], r: &mut [f32]) {
                for v in l.iter_mut().chain(r.iter_mut()) {
                    *v = v.clamp(-self.0, self.0);
                }
            }
        }

        fn make() -> Box<dyn ClipEngine> {
            Box::new(Hard(1.0))
        }

        #[test]
        fn hard_clip_passes_contract_and_bench_reads_right() {
            assert_contract(&make);
            let mut e = make();
            e.set(1.0, 0.0, 0.0);
            // Below the ceiling a hard clip is a wire: no alias, no level error, rail exact.
            assert!(alias_dbc(e.as_mut(), 48_000.0, 5_000.0, 0.0) < -120.0);
            assert!(flatness_db(e.as_mut(), 48_000.0) < 1e-6);
            assert!(alias_dbc(e.as_mut(), 48_000.0, 5_000.0, 24.0) > -40.0);
            let (sp, tp) = rail(&make, 48_000.0);
            assert!((sp - 1.0).abs() < 1e-6 && tp > 1.0);
            let h = harmonics_dbc(e.as_mut(), 48_000.0, 1_000.0, 12.0);
            assert!(h[1] > -20.0 && h[0] < -100.0, "odd harmonics only: {h:?}");
        }

        #[test]
        #[ignore]
        fn hard_clip_report() {
            println!("{}", report(&make, 0.5, 0.5));
        }
    }
}
