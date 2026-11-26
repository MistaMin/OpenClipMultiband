//! TS-2 and TS-3. Ignored in a plain `cargo test`; the commit tester runs them with `--ignored`.
//! OPENPLUGIN_SOAK_SEEDS sets the number of seeds (24 per commit, 200 for a release).

use openclipmultiband::OpenClipMultiband;
use openplugin::testing::{Options, conformance};

openplugin::testing::install_allocator!();

/// The loudest output the ranges allow, kept on purpose by the owner: Output Ceiling off, Input +24 dB, Drive
/// +48 dB, Band Gain +48 dB and Output +24 dB over six bands, and Honey and Flint overshoot their band ceiling.
/// Every algorithm, anti-alias mode, 1x and 64x oversampling, Shape, Character and the drawn curve at 48 and
/// 96 kHz with those gains peaked at 88,270 (+98.9 dBFS, Honey, drawn curve, sweep); this is 7 dB above it.
const MAX_OUTPUT: f32 = 200_000.0;

fn seeds() -> u64 {
    std::env::var("OPENPLUGIN_SOAK_SEEDS").ok().and_then(|s| s.parse().ok()).unwrap_or(24)
}

#[test]
#[ignore]
fn soak_many_seeds() {
    for seed in 1..=seeds() {
        let report = conformance(OpenClipMultiband::default, Options { fuzz_sessions: 12, latency_depends_on_params: true, max_output: MAX_OUTPUT, ..Options::default() }.with_seed(seed));
        assert!(report.passed(), "seed {seed}:\n{report}");
    }
}

#[test]
#[ignore]
fn fuzz_strict() {
    let options =
        Options {
        fuzz_sessions: 32,
        strict_hostile_output: true,
        strict_denormals: true,
        latency_depends_on_params: true,
        max_output: MAX_OUTPUT,
        ..Options::default()
    };
    conformance(OpenClipMultiband::default, options).assert_pass();
}
