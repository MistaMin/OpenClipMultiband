//! TS-1: the conformance kit on this plugin. Debug builds run Options::quick() (the Stop hook);
//! release builds run every rate and block size (the commit tester).

use openclipmultiband::OpenClipMultiband;
use openplugin::testing::{Options, conformance};

openplugin::testing::install_allocator!();

/// The loudest output the ranges allow, kept on purpose by the owner: Output Ceiling off, Input +24 dB, Drive
/// +48 dB, Band Gain +48 dB and Output +24 dB over six bands, and Honey and Flint overshoot their band ceiling.
/// Every algorithm, anti-alias mode, 1x and 64x oversampling, Shape, Character and the drawn curve at 48 and
/// 96 kHz with those gains peaked at 88,270 (+98.9 dBFS, Honey, drawn curve, sweep); this is 7 dB above it.
const MAX_OUTPUT: f32 = 200_000.0;

#[test]
fn conforms() {
    // Oversampling, ADAA and Zero Latency set the latency.
    let base = if cfg!(debug_assertions) { Options::quick() } else { Options::default() };
    let options = Options { latency_depends_on_params: true, max_output: MAX_OUTPUT, ..base };
    conformance(OpenClipMultiband::default, options).assert_pass();
}
