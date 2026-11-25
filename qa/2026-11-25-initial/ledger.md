# Scope ledger: OpenClip Multiband, first version

Project: OpenClip Multiband   Branch / commit: `main`, first commit   Started: 2026-11-25   Build: Release, macOS universal

Sources of scope (owner requests, 2026-11-25, in order):
- "Using OpenMultiband as the backbone, create me a clipper plugin", clipping algorithms from the wiki and new
  designs, including the wiki's exponential knee.
- Rework 1: one algorithm from a dropdown; phase-aligned blend of single-band, multiband and dry "so you get more of
  the sound you want and not phasing".
- Rework 2: history of the original against the clipped signal, whole mix and each band in its own colour and lane,
  the large part of the window; every algorithm through Hann-B1 ADAA (default), adaptive ADAA or none, at 1x (default),
  4x, 8x, 16x, 32x, 64x; waveform box with a pencil to draw a curve within the selected algorithm.
- Rework 3: one band set that opens into the bands; single / ceiling / blend / wet / output / character as sliders;
  less boring algorithm names; history detachable, resizable, on screen after the plug-in window closes.
- Rework 4: taller knobs with the bands closed; waveform box half height and zoomed; history about 20 % larger; the
  pop-out filled at any width; Shape and Character greyed while the pencil is on; band knobs centred at 0 as ±24 dB
  offsets from ALL.
- History at half the waveform (level), "this lets us see more of the transients shape"; history stable under heavy
  clipping ("on heavy compression the all history was not stable").
- Docs: `README.md`, `docs/USER_MANUAL.md`, `docs/PARAMETERS.md` (generated).

## Items

| ID | Item | Code | GUI | Test | Evidence | Status |
|---|---|---|---|---|---|---|
| S1 | 1 to 6 bands, Linkwitz-Riley 12/24/48 dB/oct, crossover handles | `src/crossover.rs` | header Bands/Slope, crossover strip | `bands_isolate_their_frequency_ranges`, `rapid_frequency_automation_stays_finite_and_matched`; REAPER sweep | `cargo-test` 2026-11-25; screenshots 3/4/6 bands | verified |
| S2 | Six algorithms incl. the wiki exponential knee (Honey); names Honey, Flint, Ghost, Ember, Magnet, Volt | `crates/clip/src/algos.rs` | Algorithm list | antiderivative and contract tests in `crates/clip`, `crates/clip-api` | `cargo-test` | verified |
| S3 | One algorithm for every clipper, from a dropdown, 10 ms crossfade | `crates/clip/src/lib.rs` | Algorithm list | `lists_open_and_pick`; REAPER sweep `Algorithm` | screenshot `4band-list-dark` | verified |
| S4 | Single / Multi / Dry paths phase aligned; Blend and Mix never change the unclipped tone | `src/lib.rs` `run` | Single-Multi, Dry-Wet sliders | `all_three_paths_are_phase_aligned_for_every_algorithm` (mutation-checked) | `cargo-test` | verified |
| S5 | History: original against clipped, ALL plus one lane per band in its colour, large view | `src/ui/mod.rs` `paint_history` | History | screenshots; `heavy_clipping_history` | `4band-dark`, `6band-open-graphite` | verified |
| S6 | History lanes show half the waveform (level), twice as tall | `paint_history` | History | screenshots | `4band-dark` 2026-11-25 | verified |
| S7 | History steady under heavy clipping (zoom up at once, down 6 dB/s, input and output counted) | `lane_targets`, `settle_scales` | History, pop-out | `lane_zoom_rises_at_once_and_falls_slowly`; `heavy_clipping_history` (real DSP, +18 dB drive: largest one-tick zoom-out 1.022x) | test output | verified |
| S8 | ADAA Hann-B1 (default), adaptive, none for every algorithm | `crates/clip-api/src/shaper.rs` | Anti-Alias list | shaper contract at every quality; aliasing: none -30.3, B1 -46.2, adaptive -42.5 dBc | `cargo-test` | verified |
| S9 | Oversampling 1x (default), 4x, 8x, 16x, 32x, 64x | `shaper.rs` (half-band cascade) | Oversample row | shaper tests: OS4 -55.5, OS16 -78.7, B1 x16 -109 dBc; true delay test | `cargo-test` | verified |
| S10 | Waveform box: curve and a sine 6 dB over; pencil draws a curve that every clipper uses, stages kept | `src/ui/mod.rs`, `Drawn` | Waveform box, pencil | `pencil_seeds_from_the_algorithm_and_drawing_sets_points`, `drawn_curve_reaches_every_clipper`; REAPER sweep of 16 curve points | `4band-drawn-dark`, `3band-light` | verified |
| S11 | Waveform box half height, zoomed; labels on plates in free corners | `src/ui/mod.rs` | Waveform box | screenshots (labels crossed the traces before this run: fixed) | screenshots 2026-11-25 | verified |
| S12 | Shape and Character greyed while the pencil is on | `src/ui/mod.rs` `.dim(drawn_on)` | sliders | screenshot | `4band-drawn-dark` | verified |
| S13 | One band set (Drive, Ceiling, Gain) opening into ALL plus a column per band | `src/ui/mod.rs` | Bands panel, OPEN BANDS | `one_set_opens_into_the_bands` | `4band-open-dark`, `6band-open-graphite` | verified |
| S14 | Band knobs centred at 0, ±24 dB from ALL (each band pushed or pulled up to 48 dB) | `band_settings`, `apply_band` | band columns | `all_bands_set_adds_to_each_band`; REAPER sweep (band ceiling with ALL at -12 dB) | screenshots | verified |
| S15 | Knobs much taller with the bands closed | `ALL_KNOB_*`, `Size::Big` | Bands panel | screenshot | `4band-dark` | verified |
| S16 | Single Drive / Ceiling, Single-Multi, Dry-Wet, Out Ceiling, Shape, Character as sliders | `src/ui/mod.rs` | Mix & Output | screenshot; REAPER visual sweep | `4band-dark` | verified |
| S17 | History pop-out: detachable, resizable, stays after the plug-in window closes, closes with the instance | `src/ui/pops.rs`, `history.rs`, `popout.rs` | POP OUT | `resizes_and_draws_from_shared_state_alone`; REAPER by hand (earlier this session: opens, survives closing the plug-in window, 1400x700) | session notes | verified (macOS); Windows/Linux unverified (not run) |
| S18 | Pop-out filled at its widest | `history.rs` size range | pop-out | `widest_window_is_filled` | `cargo-test` | verified |
| S19 | Auto Gain and Delta | `src/autogain.rs`, `run` | header buttons | `matches_the_input_loudness`, `off_is_a_wire`; REAPER sweep | `cargo-test` | verified |
| S20 | Input and output section: IN / OUT peak meters with Input and Output gain | `src/ui/mod.rs` | side rails | `meters_read_and_fall_as_specified` (sample peak, instant rise, 24 dB/s fall, same at 30 and 60 Hz ticks) | `cargo-test` | verified (editor); host-rendered ballistics not measured |
| S21 | Value text: at most three digits, whole dB, ceilings in tenths, kHz from 1000, no "-0" | `src/text.rs`, `params()` | every knob and slider | `display_texts_follow_the_rules` (min, mid, max, default of every control), `prints_and_reads_back` | `cargo-test` | verified (fixed this run: "632.5 Hz", "20.00 kHz", "-17.5 dB" style texts before) |
| S22 | Every parameter has a control | `every_parameter_has_a_control` | all | static test; REAPER visual sweep | `reaper/final-{VST3,CLAP,AU}/report.md` | verified |
| S23 | Every parameter changes the sound | DSP | none | REAPER and OpenGrid9 audio sweeps with contexts | `reaper/final-*`, `og9/final-*` | verified |
| S24 | State survives save and reopen; editor reopens | SDK shell, `state_is_parameters_only` | none | REAPER and OpenGrid9 suites | `reaper/final-*`, `og9/final-*` | verified |
| S25 | Renders repeat bit for bit; re-sent host values change nothing audible | `src/lib.rs` | none | `renders_repeat_at_any_block_size` (new); REAPER suite | see Findings | verified in process; REAPER intermittent (F1) |
| S26 | Latency final before the first block (no change announced by a fresh engine) | `total_latency` | none | `latency_is_final_before_the_first_block` (new) | `cargo-test` | verified |
| S27 | Text whole, apart, at least 8 pt; controls inside the window and apart | `src/ui/mod.rs` | all | `text_is_whole_apart_and_readable`, `controls_stay_inside_window_and_do_not_overlap` | `cargo-test` | verified |
| S28 | Formats: VST3, AU, CLAP, LV2 (macOS universal); AAX built unsigned; AU named "OpenClip Multiband" | `scripts/build.sh` | none | validators and host suites below | `validators/`, `reaper/`, `og9/` | verified except AAX (F3); LV2 loads and renders only (F7) |

## Parameters (host list vs ledger)

Host list dumped from: REAPER 7.80, VST3   Count: 84 (80 plugin parameters, plus the VST3 bypass the SDK shell adds and
REAPER's own Bypass, Wet and Delta). All in a row above: yes (globals S1 to S21, bands S13/S14, curve points S10).

## Checks

| Check | Result |
|---|---|
| `cargo test --release --workspace` | PASS, 111 tests (37 in the plugin) |
| `auval -v aufx OCMb Ogrd` | PASS (AU VALIDATION SUCCEEDED) |
| clap-validator 0.4.1 | PASS (34 passed, 0 failed, 10 skipped) |
| pluginval 1.0.4, strictness 10, VST3 and AU, GUI tests on | PASS |
| VST3 class ID | PASS: `73FD5944814BC7C329AED73EA164C946`, 32 hex digits, differs from OpenMultiband's |
| Frame benchmark, 2x (load average 19 during the run) | 4 bands closed 15.2 ms, 4 bands open 15.4 ms, 6 bands open 16.5 ms (budget 33 ms) |
| REAPER suite VST3, run 1 | loads, finite, not silent, editor opens, state, editor reopens PASS; deterministic FAIL (F1, audio sweep skipped); visual: host-provided Bypass x2, Wet, Delta NOT SHOWN (now excused by ident) |
| REAPER suite VST3, run 2 (build before F4/F6) | ALL PASS: deterministic (-240 dB), 83 parameters change the sound (contexts in `reaper-exceptions.txt`), every parameter shown, state, editor reopens |
| REAPER suites on the final build: VST3, CLAP, AU | ALL PASS in each: loads, finite, not silent, deterministic (-240 dB), editor opens, every parameter changes the sound (Bands Open excused), every parameter shown, state, editor reopens 5 times. Reports in `reaper/final-*/report.md` |
| OpenGrid9 LV2 | loads, finite, not silent PASS; deterministic FAIL = F5 (first 0.1 s only); sweep and state SKIPPED (F7) |
| OpenGrid9 VST3, AU (final build) | PASS each: loads, finite, not silent, deterministic (-240 dB), every parameter changes the sound, state; editor checks SKIPPED (headless host) |

## Findings

- **F1, REAPER render repeatability (open).** Run 1 of the suite: three renders of the same project differed by up
  to -1.8 dB inside 1.03 to 1.28 s; later by hand one render in five differed (-2.6 dB, 0.849 to 0.89 s). Each
  difference starts sharply and decays exponentially to bit-identical within about 40 ms: an IIR state disturbed once.
  Not reproduced since: 7 + 7 repeat renders of the plug-in (once under heavy CPU load) were bit-identical, as were
  REAPER's own EQ and OpenMultiband. Ruled out in process: block size (64 to 1024, odd sizes), a host re-sending every
  value mid-render (-132 dB), latency moving after activation, audio-thread reads of shared state (there are none).
  Suspected: the SDK's VST3 shell resets the engine in `setProcessing(true)` (`native/vst3/OpenPluginVST3.cpp`);
  if REAPER makes that call while its render thread already runs, the reset lands mid-render. Not proven; changing
  the SDK needs the owner's OK.
- **F2, display texts (fixed).** Crossover showed "632.5 Hz" / "20.00 kHz", dB controls "0.0 dB", curve points
  "0.12": now three digits, whole dB, ceilings in tenths, curve points in percent (`src/text.rs`, from OpenDynamics).
- **F3, AAX (skipped).** No PACE wrap GUID exists for this product, so the AAX is unsigned: AAX Validator and Pro
  Tools not run.
- **F5, hosts that do not reset between renders (host behaviour, explained).** In suite run 2 one render (Bands
  Open = 0, right after the Oversampling sweep at 64x with +24 dB drive) differed from the base by -3.7 dB in its first
  0.25 s, then about -100 dB. In process, rendering after that sweep without a reset gives the same pattern (-0.8 dB,
  -74 dB, -101 dB); with a reset it is bit-exact. OpenGrid9 (LV2) shows the same in the first 0.1 s; the OpenMultiband
  profile notes the same "non-reset bounce tail". The plug-in is correct after a reset; nothing to fix in the plug-in.
- **F6, AU name (fixed).** The AU registered as "OpenGrid: OpenClipMultiband" (the SDK build script names it after the
  bundle, which cannot hold a space). `scripts/build.sh` now sets "OpenGrid: OpenClip Multiband" and signs the AU
  again; auval reads "OpenClip Multiband".
- **F7, OpenGrid9 LV2 bridge (tooling, not the plug-in).** The bridge reads LV2 parameter values misaligned (values of
  None, steps of 1.818); OpenMultiband shows the same with impossible values (Crossover Slope "8000"). The TTL is
  correct. The bridge's `set_param` then fails on the nil value, so the LV2 sweep and state check could not run.
- **F4, waveform labels (fixed).** "IN -> OUT" and "+6 dB OVER" sat on the traces after the box was halved; now on
  plates in the corners the traces leave free.

## Exceptions (each needs a reason)

| Parameter / check | Exception | Reason | Decided by |
|---|---|---|---|
| Bands Open | audio | opens the band panel in the editor; no audio by design | QA (NEW) |
| VST3 Bypass (ident `80:1113149523`), REAPER Bypass, Wet, Delta | visual | host-provided; the host's own controls are theirs | QA (NEW) |
| History fall rate (GUI rule: about 60 dB/s between peaks) | not applied | the owner asked to see each transient's shape; a fall would hide it. Lanes show the 5 ms peak level per column | QA (NEW), owner to confirm |

Context lines (tested in the state where a parameter acts, not excused) are in `reaper-exceptions.txt`.

## Open questions

- Magnet and Volt settle 3 to 5 dB under the ceiling by design. Rescale them to reach it?
- The waveform box was halved under rework 4 ("wave forms half height"); the owner later meant the history lanes.
  Keep the smaller box, or restore it?
- F1: approve a look at the SDK shell's `setProcessing` reset, or leave it as is?
- F7: fix the OpenGrid9 bridge's LV2 parameter mapping (shared QA tool)?

## Not done

| Item | Why |
|---|---|
| Pro Tools (AAX) | no PACE wrap GUID: owner creates one |
| On-screen GPU / host CPU measurement | needs a visible host; not asked for in this run |
| Windows and Linux | built by GitHub Actions after a push; nothing pushed |
| Listening | needs the owner |
