use super::*;
use clip_api::bench;

const RATES: [f32; 4] = [44_100.0, 48_000.0, 96_000.0, 192_000.0];

fn blamp_engine() -> Box<dyn ClipEngine> {
    Box::new(HardBlamp::new())
}

fn os_engine() -> Box<dyn ClipEngine> {
    Box::new(HardOs::new())
}

fn noise(n: usize, rms: f32, seed: u64) -> Vec<f32> {
    let mut s = seed | 1;
    (0..n)
        .map(|_| {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            ((s >> 40) as f32 / (1u64 << 24) as f32 - 0.5) * 3.46 * rms
        })
        .collect()
}

fn sine(n: usize, hz: f32, fs: f32, amp: f32) -> Vec<f32> {
    (0..n).map(|t| amp * (std::f32::consts::TAU * hz * t as f32 / fs).sin()).collect()
}

fn prepared(make: fn() -> Box<dyn ClipEngine>, fs: f32, ceiling: f32, shape: f32, character: f32) -> Box<dyn ClipEngine> {
    let mut e = make();
    e.prepare(fs, 512);
    e.set(ceiling, shape, character);
    e
}

/// Pins shape and character whatever the bench asks for (`bench::rail` always asks for 0.5).
struct Force(Box<dyn ClipEngine>, f32, f32);

impl ClipEngine for Force {
    fn name(&self) -> &'static str {
        self.0.name()
    }
    fn prepare(&mut self, fs: f32, b: usize) {
        self.0.prepare(fs, b)
    }
    fn reset(&mut self) {
        self.0.reset()
    }
    fn set(&mut self, c: f32, _: f32, _: f32) {
        self.0.set(c, self.1, self.2)
    }
    fn latency(&self) -> usize {
        self.0.latency()
    }
    fn process(&mut self, l: &mut [f32], r: &mut [f32]) {
        self.0.process(l, r)
    }
}

// ------------------------------------------------------------------------------- contract

#[test]
fn hard_blamp_contract() {
    bench::assert_contract(&blamp_engine);
}

#[test]
fn hard_os_contract() {
    bench::assert_contract(&os_engine);
}

#[test]
fn labels_fit() {
    for (label, make) in ENGINES {
        assert!(label.len() <= 12);
        assert_eq!(*label, make().name());
    }
}

// ----------------------------------------------------------------------------------- maths

#[test]
fn residual_closed_form_matches_definition() {
    for i in 0..=100 {
        let d = i as f64 / 100.0;
        let r = residual(d as f32, 0.0);
        for (k, v) in r.iter().enumerate() {
            assert!((*v as f64 - blamp(k as f64 - 1.0 - d)).abs() < 1e-6, "d {d} slot {k}");
        }
        // a very narrow knee is the corner
        let rk = residual(d as f32, 1.001e-3);
        for (a, b) in r.iter().zip(rk) {
            assert!((a - b).abs() < 3e-4);
        }
    }
    // even, zero at the support edges, area 1/6 (half the kernel's variance)
    for i in 0..400 {
        let t = i as f64 / 100.0 - 2.0;
        assert!((blamp(t) - blamp(-t)).abs() < 1e-12);
        let num = (blamp_int(t + 1e-5) - blamp_int(t - 1e-5)) / 2e-5;
        assert!((num - blamp(t)).abs() < 1e-5, "blamp_int' != blamp at {t}");
    }
    assert!(blamp(2.0).abs() < 1e-15 && blamp(-2.0).abs() < 1e-15);
    assert!((blamp_int(5.0) - 1.0 / 6.0).abs() < 1e-12 && blamp_int(-5.0) == 0.0);
}

#[test]
fn fit_is_exact_on_polynomials() {
    for taps in [4, 8] {
        let fit = Fit::new(taps);
        let poly = |t: f64| 0.3 - 0.7 * t + 0.2 * t * t - 0.05 * t * t * t;
        let mut x = [0.0f32; 8];
        for i in 0..taps {
            x[8 - taps + i] = poly(i as f64 + 1.0 - (taps / 2) as f64) as f32;
        }
        let p = fit.poly(&x);
        for t in [0.0f64, 0.25, 0.6, 1.0] {
            let v: f64 = p.iter().enumerate().map(|(j, &c)| c as f64 * t.powi(j as i32)).sum();
            assert!((v - poly(t)).abs() < 1e-5, "taps {taps} t {t}: {v} vs {}", poly(t));
        }
    }
}

#[test]
fn knee_is_continuous_c1_and_under_the_ceiling() {
    for &(c, h) in &[(1.0f32, 0.2f32), (0.3, 0.06), (1.0, 0.0)] {
        let mut prev = knee(0.0, c, h);
        let mut prev_slope = 1.0;
        let dx = 1e-4;
        for i in 1..30_000 {
            let x = i as f32 * dx;
            let y = knee(x, c, h);
            assert!(y <= c && (y - prev).abs() <= dx * 1.001, "jump at {x}");
            let slope = (y - prev) / dx;
            if h > 0.0 {
                assert!((slope - prev_slope).abs() < 0.02, "slope jump at {x}: {prev_slope} -> {slope}");
            }
            assert_eq!(knee(-x, c, h), -y);
            prev = y;
            prev_slope = slope;
        }
    }
}

// -------------------------------------------------------------------------- ceiling claims

/// Loud test signals: noise, high sines, a step train, and an inter-sample torture pattern.
fn torture(fs: f32) -> Vec<Vec<f32>> {
    let mut v = vec![noise(12_000, 8.0, 3), noise(12_000, 1.2, 4)];
    for hz in [997.0, 6_100.0, 11_900.0, 17_000.0] {
        v.push(sine(12_000, hz, fs, 6.0));
    }
    v.push((0..12_000).map(|t| if (t / 29) % 2 == 0 { 5.0 } else { -5.0 }).collect());
    v.push((0..12_000).map(|t| [1.0, 1.0, -1.0, -1.0, 1.0, -1.0][t % 6] * 3.0).collect());
    v
}

fn worst_sample_ratio(make: fn() -> Box<dyn ClipEngine>, characters: &[f32]) -> f64 {
    let mut worst: f64 = 0.0;
    for fs in RATES {
        for x in torture(fs) {
            for &ceiling in &[1.0f32, 0.5, 0.0891] {
                for &shape in &[0.0, 0.5, 1.0] {
                    for &ch in characters {
                        let mut e = prepared(make, fs, ceiling, shape, ch);
                        let y = bench::run(e.as_mut(), &x, &[512, 7]);
                        let pk = y.iter().fold(0.0f32, |m, v| m.max(v.abs()));
                        worst = worst.max(pk as f64 / ceiling as f64);
                    }
                }
            }
        }
    }
    worst
}

#[test]
fn hard_blamp_never_passes_the_ceiling() {
    let w = worst_sample_ratio(blamp_engine, &[0.0, 0.5, 1.0]);
    assert!(w <= 1.0, "sample peak {w}x");
}

#[test]
fn hard_os_holds_the_ceiling_from_character_half() {
    let w = worst_sample_ratio(os_engine, &[0.5, 0.75, 1.0]);
    assert!(w <= 1.0, "sample peak {w}x");
}

#[test]
fn hard_os_free_mode_overshoot_is_bounded() {
    // at Character 0 the overshoot of band limiting passes, up to the 2x (+6 dB) backstop
    let w = worst_sample_ratio(os_engine, &[0.0]);
    assert!(w > 1.0 && w <= 2.0, "free-mode peak {w}x");
}

#[test]
fn true_peak_drops_with_character() {
    for make in [blamp_engine as fn() -> Box<dyn ClipEngine>, os_engine] {
        let tp = |ch: f32| {
            let f = move || Box::new(Force(make(), 0.0, ch)) as Box<dyn ClipEngine>;
            bench::rail(&f, 48_000.0).1
        };
        let (lo, hi) = (tp(0.5), tp(1.0));
        assert!(hi < lo && hi < 1.45, "{}: true peak {lo} at 0.5, {hi} at 1", make().name());
    }
}

// --------------------------------------------------------------------------- wire and phase

#[test]
fn below_the_knee_is_a_delayed_wire() {
    for fs in RATES {
        let (lo, hi) = (sine(6_000, 1_000.0, fs, 0.25), sine(6_000, 17_000.0, fs, 0.125));
        let x: Vec<f32> = lo.iter().zip(&hi).map(|(a, b)| a + b).collect();
        // Hard BLAMP: bit-exact delay
        let mut e = prepared(blamp_engine, fs, 1.0, 1.0, 1.0);
        let lat = e.latency();
        let y = bench::run(e.as_mut(), &x, &[512, 13]);
        assert!((lat..x.len()).all(|t| y[t] == x[t - lat]), "Hard BLAMP not a wire at {fs}");
        // Hard OS: the half-bands' ripple only
        let mut e = prepared(os_engine, fs, 1.0, 1.0, 1.0);
        let lat = e.latency();
        assert!(lat <= 64, "latency {lat} at {fs}");
        let y = bench::run(e.as_mut(), &x, &[512, 13]);
        let (mut err, mut sig) = (0.0f64, 0.0f64);
        for t in 500..x.len() {
            err += ((y[t] - x[t - lat]) as f64).powi(2);
            sig += (x[t - lat] as f64).powi(2);
        }
        let null = 10.0 * (err / sig).log10();
        assert!(null < -70.0, "Hard OS null {null:.1} dB at {fs}");
    }
}

#[test]
fn flat_at_every_rate() {
    for fs in RATES {
        for make in [blamp_engine as fn() -> Box<dyn ClipEngine>, os_engine] {
            let mut e = prepared(make, fs, 1.0, 0.5, 0.5);
            let f = bench::flatness_db(e.as_mut(), fs as f64);
            assert!(f <= 0.05, "{} at {fs}: {f} dB", e.name());
        }
    }
}

#[test]
fn odd_symmetric_and_dc_free() {
    for make in [blamp_engine as fn() -> Box<dyn ClipEngine>, os_engine] {
        let x = noise(8_000, 3.0, 21);
        let neg: Vec<f32> = x.iter().map(|v| -v).collect();
        let mut e = prepared(make, 48_000.0, 0.7, 0.4, 0.8);
        let a = bench::run(e.as_mut(), &x, &[512]);
        e.reset();
        let b = bench::run(e.as_mut(), &neg, &[512]);
        let bad = a.iter().zip(&b).position(|(p, q)| *p != -*q);
        assert!(bad.is_none(), "{} is not odd at {bad:?}: {} vs {}", e.name(), a[bad.unwrap_or(0)], b[bad.unwrap_or(0)]);
        e.reset();
        let s = bench::run(e.as_mut(), &sine(48_000, 1_000.0, 48_000.0, 8.0), &[512]);
        // a whole number of periods (48 samples each)
        let mean = s[960..960 + 48 * 900].iter().map(|&v| v as f64).sum::<f64>() / (48.0 * 900.0);
        assert!(mean.abs() < 1e-6, "{} DC {mean}", e.name());
    }
}

#[test]
fn os_impulse_peaks_at_the_latency() {
    for fs in RATES {
        let mut e = prepared(os_engine, fs, 1.0, 0.0, 0.0);
        let mut x = vec![0.0f32; 256];
        x[10] = 0.5;
        let y = bench::run(e.as_mut(), &x, &[512]);
        let at = (0..y.len()).max_by(|&a, &b| y[a].abs().total_cmp(&y[b].abs())).unwrap();
        assert_eq!(at, 10 + e.latency(), "at {fs}");
    }
}

// ------------------------------------------------------------------------- moving settings

/// Largest sample-to-sample step of `y` in a range.
fn max_step(y: &[f32], r: std::ops::Range<usize>) -> f32 {
    r.map(|t| (y[t] - y[t - 1]).abs()).fold(0.0, f32::max)
}

#[test]
fn setting_jumps_do_not_click() {
    let fs = 48_000.0;
    for make in [blamp_engine as fn() -> Box<dyn ClipEngine>, os_engine] {
        // a low tone, driven: its steady steps are small, so a jump would show
        for (from, to) in [((1.0, 0.0, 0.0), (0.25, 1.0, 1.0)), ((0.2, 1.0, 1.0), (1.0, 0.0, 0.0))] {
            let mut e = prepared(make, fs, from.0, from.1, from.2);
            let x = sine(24_000, 100.0, fs, 2.0);
            let (mut l, mut r) = (x.clone(), x.clone());
            for (k, (bl, br)) in l.chunks_mut(256).zip(r.chunks_mut(256)).enumerate() {
                if k == 40 {
                    e.set(to.0, to.1, to.2);
                }
                e.process(bl, br);
            }
            let steady = max_step(&l, 20_000..24_000).max(max_step(&l, 6_000..10_000));
            let around = max_step(&l, 10_000..12_000);
            assert!(around <= 1.05 * steady, "{}: step {around} around the jump, {steady} steady", e.name());
        }
    }
}

// --------------------------------------------------------------------------- alias claims

const TONES: [f64; 5] = [1_000.0, 2_500.0, 5_000.0, 8_000.0, 12_000.0];

fn worst_alias(e: &mut dyn ClipEngine, drive: f64) -> f64 {
    TONES.iter().map(|&hz| bench::alias_dbc(e, 48_000.0, hz, drive)).fold(f64::NEG_INFINITY, f64::max)
}

#[test]
fn alias_beats_the_hard_clip_reference() {
    // the reference row: -12.7 dBc at +12 dB, -7.6 dBc at +24 dB
    let mut e = prepared(blamp_engine, 48_000.0, 1.0, 0.0, 0.5);
    let (a12, a24) = (worst_alias(e.as_mut(), 12.0), worst_alias(e.as_mut(), 24.0));
    assert!(a12 < -22.7 && a24 < -30.0, "Hard BLAMP {a12} / {a24}");
    let mut e = prepared(os_engine, 48_000.0, 1.0, 0.0, 0.0);
    let (a12, a24) = (worst_alias(e.as_mut(), 12.0), worst_alias(e.as_mut(), 24.0));
    assert!(a12 < -60.0 && a24 < -60.0, "Hard OS free {a12} / {a24}");
    let mut e = prepared(os_engine, 48_000.0, 1.0, 0.0, 0.5);
    let a12 = worst_alias(e.as_mut(), 12.0);
    assert!(a12 < -22.7, "Hard OS sample-safe {a12}");
}

// ------------------------------------------------------- minimum-phase comparison (report)

/// Polyphase IIR half-band (two all-pass branches): the low-delay, non-linear-phase option.
/// Coefficients from the classic elliptic half-band design (Jacobi theta-series form).
fn iir_halfband(n_coefs: usize, tbw: f64) -> Vec<f64> {
    let mut k = ((1.0 - 2.0 * tbw) * PI / 4.0).tan();
    k *= k;
    let kk = (1.0 - k * k).powf(0.25);
    let e = 0.5 * (1.0 - kk) / (1.0 + kk);
    let e4 = e.powi(4);
    let q = e * (1.0 + e4 * (2.0 + e4 * (15.0 + 150.0 * e4)));
    let order = (2 * n_coefs + 1) as f64;
    (0..n_coefs)
        .map(|index| {
            let c = (index + 1) as f64;
            let (mut num, mut den) = (0.0, 0.0);
            let mut i = 0i32;
            loop {
                let v = q.powi(i * (i + 1)) * ((2 * i + 1) as f64 * c * PI / order).sin() * if i % 2 == 0 { 1.0 } else { -1.0 };
                num += v;
                i += 1;
                if v.abs() < 1e-30 {
                    break;
                }
            }
            let mut i = 1i32;
            loop {
                let v = q.powi(i * i) * ((2 * i) as f64 * c * PI / order).cos() * if i % 2 == 1 { -1.0 } else { 1.0 };
                den += v;
                i += 1;
                if v.abs() < 1e-30 {
                    break;
                }
            }
            let ww = num * q.powf(0.25) / (den + 0.5);
            let w2 = ww * ww;
            let x = ((1.0 - w2 * k) * (1.0 - w2 / k)).sqrt() / (1.0 + w2);
            (1.0 - x) / (1.0 + x)
        })
        .collect()
}

#[derive(Clone)]
struct AllPass {
    a: Vec<f64>,
    x1: Vec<f64>,
    y1: Vec<f64>,
}

impl AllPass {
    fn new(a: Vec<f64>) -> Self {
        let n = a.len();
        Self { a, x1: vec![0.0; n], y1: vec![0.0; n] }
    }
    fn tick(&mut self, mut x: f64) -> f64 {
        for i in 0..self.a.len() {
            let y = self.a[i] * (x - self.y1[i]) + self.x1[i];
            self.x1[i] = x;
            self.y1[i] = y;
            x = y;
        }
        x
    }
}

#[derive(Clone)]
struct IirStage {
    up: [AllPass; 2],
    dn: [AllPass; 2],
    held: f64,
}

impl IirStage {
    fn new(coefs: &[f64]) -> Self {
        let b0: Vec<f64> = coefs.iter().step_by(2).copied().collect();
        let b1: Vec<f64> = coefs.iter().skip(1).step_by(2).copied().collect();
        let ap = [AllPass::new(b0), AllPass::new(b1)];
        Self { up: ap.clone(), dn: ap, held: 0.0 }
    }
    fn up(&mut self, x: f64) -> (f64, f64) {
        (self.up[0].tick(x), self.up[1].tick(x))
    }
    fn down(&mut self, e: f64, o: f64) -> f64 {
        let y = 0.5 * (self.dn[0].tick(e) + self.dn[1].tick(self.held));
        self.held = o;
        y
    }
}

/// `Hard OS` at Character 0 with the half-bands swapped for IIR ones (comparison only).
struct HardOsIir {
    p: Params,
    s1: Vec<IirStage>,
    s2: Vec<IirStage>,
    clip: [Corner; 2],
    fit: Fit,
    coefs: (Vec<f64>, Vec<f64>),
    latency: usize,
}

impl HardOsIir {
    fn new(latency: usize) -> Self {
        // stage 1: 20 k -> 28 k at 96 kHz; stage 2: 20 k -> 76 k at 192 kHz; ~100 dB both
        let coefs = (iir_halfband(8, 8.0 / 96.0 / 2.0), iir_halfband(3, 56.0 / 192.0 / 2.0));
        let s1 = vec![IirStage::new(&coefs.0); 2];
        let s2 = vec![IirStage::new(&coefs.1); 2];
        Self { p: Params::new(), s1, s2, clip: [Corner::default(); 2], fit: Fit::new(4), coefs, latency }
    }
}

impl ClipEngine for HardOsIir {
    fn name(&self) -> &'static str {
        "Hard OS IIR"
    }
    fn prepare(&mut self, fs: f32, _: usize) {
        self.p.prepare(fs);
        self.reset();
    }
    fn reset(&mut self) {
        self.s1 = vec![IirStage::new(&self.coefs.0); 2];
        self.s2 = vec![IirStage::new(&self.coefs.1); 2];
        self.clip = [Corner::default(); 2];
    }
    fn set(&mut self, c: f32, s: f32, ch: f32) {
        self.p.set(c, s, ch);
    }
    fn latency(&self) -> usize {
        self.latency
    }
    fn process(&mut self, left: &mut [f32], right: &mut [f32]) {
        for (l, r) in left.iter_mut().zip(right.iter_mut()) {
            let (c, h, _) = self.p.step();
            for (i, v) in [l, r].into_iter().enumerate() {
                let (a0, a1) = self.s1[i].up(*v as f64);
                let mut w = [0.0; 2];
                for (wj, a) in w.iter_mut().zip([a0, a1]) {
                    let (b0, b1) = self.s2[i].up(a);
                    let c0 = self.clip[i].tick(b0 as f32, c, h, &self.fit) as f64;
                    let c1 = self.clip[i].tick(b1 as f32, c, h, &self.fit) as f64;
                    *wj = self.s2[i].down(c0, c1);
                }
                *v = self.s1[i].down(w[0], w[1]) as f32;
            }
        }
    }
}

/// Group delay (samples) of an engine's small-signal path at `hz`, from its impulse response.
fn group_delay(e: &mut dyn ClipEngine, hz: f64) -> f64 {
    let mut x = vec![0.0f32; 4_096];
    x[0] = 0.01;
    e.reset();
    let y = bench::run(e, &x, &[512]);
    let phase = |f: f64| {
        let w = 2.0 * PI * f / 48_000.0;
        let (mut re, mut im) = (0.0, 0.0);
        for (n, &v) in y.iter().enumerate() {
            re += v as f64 * (w * n as f64).cos();
            im -= v as f64 * (w * n as f64).sin();
        }
        im.atan2(re)
    };
    let df = 1.0;
    let mut dp = phase(hz + df) - phase(hz - df);
    while dp > PI {
        dp -= 2.0 * PI;
    }
    while dp < -PI {
        dp += 2.0 * PI;
    }
    -dp / (2.0 * PI * 2.0 * df / 48_000.0)
}

#[test]
#[ignore]
fn report() {
    type Mk = Box<dyn Fn() -> Box<dyn ClipEngine>>;
    let pinned = |make: fn() -> Box<dyn ClipEngine>, s: f32, c: f32| -> Mk { Box::new(move || Box::new(Force(make(), s, c)) as Box<dyn ClipEngine>) };
    // recommended first, then the corners the brief asks for
    let rows: [(fn() -> Box<dyn ClipEngine>, &[(f32, f32)]); 2] =
        [(blamp_engine, &[(0.0, 0.5), (0.0, 0.0), (1.0, 1.0), (0.0, 1.0)]), (os_engine, &[(0.0, 0.5), (0.0, 0.0), (1.0, 1.0), (0.0, 1.0)])];
    println!("Rail figures here are measured at the row's own shape/character (bench::rail alone pins 0.5/0.5).\n");
    for (make, settings) in rows {
        for &(s, c) in settings {
            let mk = pinned(make, s, c);
            println!("{}", bench::report(mk.as_ref(), s, c));
            let mut e = mk();
            e.prepare(48_000.0, 512);
            e.set(1.0, s, c);
            for d in [12.0, 24.0] {
                let v: Vec<String> = TONES.iter().map(|&hz| format!("{:.1}", bench::alias_dbc(e.as_mut(), 48_000.0, hz, d))).collect();
                println!("Per tone at +{d} dB (1k / 2.5k / 5k / 8k / 12k): {}", v.join(" / "));
            }
            println!();
        }
    }
    cpu_table();
    println!("### Latency by sample rate\n\n| Engine | 44.1 k | 48 k | 96 k | 192 k |\n|---|---:|---:|---:|---:|");
    for make in [blamp_engine as fn() -> Box<dyn ClipEngine>, os_engine] {
        let l: Vec<String> = RATES.iter().map(|&fs| prepared(make, fs, 1.0, 0.0, 0.5).latency().to_string()).collect();
        println!("| {} | {} |", make().name(), l.join(" | "));
    }
    println!("\n### Linear-phase against IIR (low-delay) half-bands, Hard OS at Character 0\n");
    let lin = pinned(os_engine, 0.0, 0.0);
    let gd_iir = {
        let mut e = HardOsIir::new(0);
        e.prepare(48_000.0, 512);
        e.set(1.0, 0.0, 0.0);
        [100.0, 1_000.0, 10_000.0, 20_000.0].map(|f| group_delay(&mut e, f))
    };
    let iir_lat = gd_iir[0].round() as usize;
    let iir: Mk = Box::new(move || Box::new(HardOsIir::new(iir_lat)) as Box<dyn ClipEngine>);
    let mut e = lin();
    e.prepare(48_000.0, 512);
    e.set(1.0, 0.0, 0.0);
    let gd_lin = [100.0, 1_000.0, 10_000.0, 20_000.0].map(|f| group_delay(e.as_mut(), f) - CATCH_LATENCY as f64);
    // stage-1 responses at the 96 kHz rate: H = 0.5 (A0(z^2) + z^-1 A1(z^2)) for the IIR,
    // the full Kaiser half-band for the FIR
    let resp = |f: f64| {
        let w = 2.0 * PI * f / 96_000.0;
        let z2 = (-(2.0 * w)).sin_cos(); // e^{-j2w} as (im, re)
        let ap = |coefs: &[f64]| {
            let (mut re, mut im) = (1.0, 0.0);
            for &a in coefs {
                // (a + z^-2) / (1 + a z^-2)
                let (nr, ni) = (a + z2.1, z2.0);
                let (dr, di) = (1.0 + a * z2.1, a * z2.0);
                let d = dr * dr + di * di;
                let (qr, qi) = ((nr * dr + ni * di) / d, (ni * dr - nr * di) / d);
                (re, im) = (re * qr - im * qi, re * qi + im * qr);
            }
            (re, im)
        };
        let c = iir_halfband(8, 8.0 / 96.0 / 2.0);
        let b0: Vec<f64> = c.iter().step_by(2).copied().collect();
        let b1: Vec<f64> = c.iter().skip(1).step_by(2).copied().collect();
        let (a0, a1) = (ap(&b0), ap(&b1));
        let z1 = (-w).sin_cos();
        let (r1, i1) = (a1.0 * z1.1 - a1.1 * z1.0, a1.0 * z1.0 + a1.1 * z1.1);
        let iir = 0.5 * ((a0.0 + r1).powi(2) + (a0.1 + i1).powi(2)).sqrt();
        let branch = halfband(48_000.0, 49);
        let m = branch.len() - 1;
        let (mut re, mut im) = (0.5 * (w * m as f64).cos(), -0.5 * (w * m as f64).sin());
        for (i, &g) in branch.iter().enumerate() {
            let n = (2 * i) as f64;
            re += g as f64 * (w * n).cos();
            im -= g as f64 * (w * n).sin();
        }
        (20.0 * iir.log10(), 20.0 * (re * re + im * im).sqrt().log10())
    };
    let (p_iir, p_fir) = resp(20_000.0);
    let (s_iir, s_fir) = resp(28_000.0);
    println!("Stage 1 at 48 kHz: 20 kHz {p_fir:.4} dB (FIR) / {p_iir:.4} dB (IIR); 28 kHz {s_fir:.1} dB (FIR) / {s_iir:.1} dB (IIR)\n");
    println!("Group delay of the filter path, samples at 100 Hz / 1 k / 10 k / 20 kHz (catcher excluded):");
    println!("- linear phase: {}", gd_lin.map(|v| format!("{v:.2}")).join(" / "));
    println!("- IIR: {}\n", gd_iir.map(|v| format!("{v:.2}")).join(" / "));
    println!("{}", bench::report(lin.as_ref(), 0.0, 0.0));
    println!("{}", bench::report(iir.as_ref(), 0.0, 0.0));
}

/// Plain hard clip: the bench's reference row, for CPU calibration on a busy machine.
struct Plain(f32);

impl ClipEngine for Plain {
    fn name(&self) -> &'static str {
        "Plain clip"
    }
    fn prepare(&mut self, _: f32, _: usize) {}
    fn reset(&mut self) {}
    fn set(&mut self, c: f32, _: f32, _: f32) {
        self.0 = c;
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

/// Best of three timings, ns per stereo frame.
fn ns_per_frame(e: &mut dyn ClipEngine, x: &[f32]) -> f64 {
    (0..3)
        .map(|_| {
            e.reset();
            let t = std::time::Instant::now();
            std::hint::black_box(bench::run(e, x, &[512]));
            t.elapsed().as_nanos() as f64 / x.len() as f64
        })
        .fold(f64::INFINITY, f64::min)
}

fn cpu_table() {
    let inputs = [("noise +12 dB (bench)", noise(480_000, 2.0, 3)), ("1 kHz +12 dB", sine(480_000, 1_000.0, 48_000.0, 2.0)), ("noise -26 dBFS", noise(480_000, 0.05, 3))];
    println!("### CPU, ns per stereo frame at 48 kHz (best of 3)\n");
    println!("| Engine, shape / character | {} |\n|---|---:|---:|---:|", inputs.iter().map(|i| i.0).collect::<Vec<_>>().join(" | "));
    println!("| Plain clip (reference) | {} |", inputs.iter().map(|(_, x)| format!("{:.1}", ns_per_frame(&mut Plain(1.0), x))).collect::<Vec<_>>().join(" | "));
    for make in [blamp_engine as fn() -> Box<dyn ClipEngine>, os_engine] {
        for (s, c) in [(0.0, 0.0), (0.0, 0.5), (0.0, 1.0), (1.0, 0.5), (1.0, 1.0)] {
            let mut e = prepared(make, 48_000.0, 1.0, s, c);
            let v: Vec<String> = inputs.iter().map(|(_, x)| format!("{:.1}", ns_per_frame(e.as_mut(), x))).collect();
            println!("| {} {s} / {c} | {} |", e.name(), v.join(" | "));
        }
    }
    println!();
}
