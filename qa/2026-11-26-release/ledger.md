# Scope ledger: OpenClip Multiband 1.0.0 Beta release candidate

Project: OpenClip Multiband   Started: 2026-11-26   Build: Release, macOS universal (arm64 + x86_64, macOS 11.0), OpenPluginSDK
`53e198c`, GoodLookin DRI `3fe0426` (tree)

Sources of scope: the first version's ledger (`qa/2026-11-25-initial/ledger.md`, items S1 to S28, all carried over) and
the owner's requests of 2026-11-26: fix the REAPER render knock in the SDK, fix the OpenGrid9 bridge's LV2 parameters,
make Magnet and Volt reach the ceiling, keep the waveform box as it is, sign the AAX with the product's wrap GUID,
run the full QA, then release (no standalone; VST3, AU, CLAP, LV2, AAX; Windows and Linux from GitHub Actions; a
`.pkg` for macOS, no DMG).

## New and changed items

| ID | Item | Code | GUI | Test | Evidence | Status |
|---|---|---|---|---|---|---|
| S29 | No mid-render knock in hosts that call `setProcessing(true)` from their main thread (F1) | OpenPluginSDK `native/vst3/OpenPluginVST3.cpp` (PR #54, merged as `53e198c`) | none | SDK proof `check_late_processing_start` (fails on the old shell, passes now; SDK CI 20 of 20 on macOS, Windows, Linux); 20 repeat renders in REAPER | `reaper/repeat-renders.txt`: 20 renders bit-identical | verified |
| S30 | Magnet and Volt rise towards the ceiling; Shape sets the knee | `crates/clip/src/algos.rs` (`soft_knee`, `knee_start`) | Algorithm list, waveform box | `magnet_and_volt_reach_the_ceiling` (sine 6 dB over: within 1.5 dB of the ceiling at Shape 0, 50, 100 %, never above it); antiderivative tests | `cargo-test.log`; screenshot `6band-open-graphite` (Magnet) | verified |
| S31 | AU registers as "OpenClip Multiband" | `scripts/build.sh` | none | auval | `validators/auval.log` ("AudioUnit Name: OpenClip Multiband") | verified |
| S32 | AAX PACE-signed with the product's wrap GUID `1D9994F0-E93C-11F1-8EC5-005056920FF7` (Signing Only) | release skill `sign_aax.sh` | none | codesign verify, wraptool verify, AAX Validator, Pro Tools | `aax/` and `protools/` (local only) | verified |
| S33 | Waveform box stays half height (owner's choice, 2026-11-26) | unchanged | Waveform box | screenshots | `docs/images/*` | verified |

Items S1 to S28 were re-run on this build through the suites below.

## Checks

| Check | Result |
|---|---|
| `cargo test --release --workspace` | PASS, 112 tests |
| Build | universal, macOS 11.0 minimum, every format (VST3, AU, CLAP, LV2, AAX); SDK and DRI at their pins |
| auval | PASS ("AudioUnit Name: OpenClip Multiband") |
| clap-validator 0.4.1 | PASS (34 passed, 0 failed, 10 skipped) |
| pluginval 1.0.4, strictness 10, GUI tests on | PASS, VST3 and AU |
| AAX Validator 2024.6 (PACE-signed bundle) | PASS 6 of 6 (warnings: algorithm-context gaps, from the SDK's AAX shell; many steps on a discrete parameter, the ceilings' 0.1 dB steps) |
| REAPER 7.80 suites: VST3, CLAP, AU | ALL PASS each: loads, finite, not silent, deterministic, editor opens, every parameter changes the sound, every parameter shown, state, editor reopens 5 times |
| REAPER repeat renders (VST3) | 20 of 20 bit-identical |
| OpenGrid9 suites: VST3, AU | PASS each: loads, finite, not silent, deterministic, every parameter changes the sound, state (editor checks SKIPPED: headless) |
| OpenGrid9 suite: LV2 | loads, finite, not silent, state PASS; repeat renders differ in the first 21 to 27 ms only (F5: the host does not reset LV2 plug-ins between bounces), so its sweep is SKIPPED. The same code is swept through VST3, CLAP and AU |
| Pro Tools Studio 2025.10, AAX Native, 48 kHz / 256 | PASS: scan and PACE check, insert widths (stereo: stereo and mono; mono: mono), editors, typing and drag, offline vs real-time null -300 dB, output peak -1.03 dBFS under the -1.0 dB ceiling, wet vs bypass -6.8 dB, bypass equal to the input (-300 dB), save, close, reopen with a bit-identical bounce, 0 stuck-plug-in or engine errors |
| Screenshots | all six seen: 4 bands, open bands, drawn curve, algorithm list, 6 bands with Magnet (graphite), 3 bands (light) |
| Editor frame cost, 2x | 15.4, 15.4, 16.7 ms per frame (budget 33 ms), load average 18.6 |

## Findings

- **F1 (fixed in the SDK).** The VST3 shell reset the engine in `setProcessing(true)` on the calling thread; REAPER calls it
  on its main thread while its render thread runs, so a late call passed a block through dry and reset the engine
  mid-render. The reset now runs on the audio thread before the stream's first block, and a late call after blocks
  started resets nothing.
- **F5 (host behaviour).** Hosts that do not reset a plug-in between renders carry the last render's tail into the next
  one's first milliseconds (OpenGrid9 with LV2; REAPER once in the first run). After a reset the plug-in is bit-exact.
- **F7 (fixed in the QA tool).** The OpenGrid9 bridge read LV2 values by port where Ardour numbers controls; values were
  shifted (OpenMultiband too). It now maps control numbers as Ardour's `nth_parameter` does.
- **Pro Tools, first Return.** After typing a value, the first Return left the field open; a click in the field and
  Return committed it. Minor; to look at in the SDK editor.
- **Pro Tools quit.** Pro Tools hung while quitting, in its ARA teardown of another vendor's installed component (that
  component's analytics thread did not stop); this plug-in was not on the stack. The test instance was ended.

## Exceptions (each with its reason)

| Parameter / check | Exception | Reason | Decided by |
|---|---|---|---|
| Bands Open | audio | opens the band panel in the editor; no audio by design | QA |
| VST3 Bypass (`80:1113149523`), REAPER Bypass, Wet, Delta | visual | host-provided; the host's own controls are theirs | QA |
| LV2 Freewheel | audio, visual | the LV2 host's freewheel port; the host sets it during export | QA (NEW) |
| OpenGrid9 LV2 sweep | SKIPPED | F5: the host does not reset LV2 plug-ins between bounces | QA (NEW) |
| History fall rate | not applied | the owner asked to see each transient's shape | QA, owner to confirm |
| AAX "many steps" warning | accepted | ceilings move in 0.1 dB steps (up to 481); a validator warning, not a failure | QA (NEW) |

## Not done

| Item | Why |
|---|---|
| Pro Tools: other rate/buffer contexts, HDX, AudioSuite, automation lanes | one context tested; no HDX hardware |
| Logic Pro | not installed |
| Windows and Linux in a DAW | built and unit-tested by GitHub Actions; not run in a host |
| On-screen GPU and host CPU | not measured this run |
| Listening | the owner |
