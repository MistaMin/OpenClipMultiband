# Scope ledger: QA of the audit fixes, the SDK move and the tree as it stands

Project: OpenClip Multiband   Code: `397ba8c` + uncommitted changes of two sessions (`final/tree.txt`, diff sha1 `94161ac19036`)
SDK: OpenPluginSDK `d93d926` (latest `main`)   Run: 2026-11-27 01:20 to 10:30   Load: 14 to 157

Sources of scope:
- Request (2026-11-27): "fix all 7" (this session's ponytail audit), then `/plugin-qa`; "once the rebuild is done run /plugin-qa".
- Owner (2026-11-27, on the removed SDK folder): "Repoint to the current sdk version and if there is a bug correct it. the latest
  version of the sdk has bug fixes that are essential to performance and stability".
- Changes made meanwhile by another agent session in the same tree (its own audit, "fix 2, 3 and 7"): the four
  design crates deleted (`git rm`, staged); `max_latency` and the dry delay sized from the clippers' real latencies
  (`src/lib.rs`), `accepts_sample_rate` 8 kHz and up, test `latency_never_exceeds_max_latency`. QA'd here as part of the tree.
- The earlier ledgers (`qa/2026-11-26-*`): sound, parameters, state and editor must not change.

Autogain and Delta: already in the plugin, not asked again.

Result: 18 items: 15 verified (A10 with one exception the owner accepted), 2 verified in process only (A1, A2: no host passes
a NaN here), 1 verified after the push (A14). Findings: 8 (Q1 corrected, Q2 resolved, Q6 accepted, Q7 fixed, Q3 noted; Q4, Q5,
Q8 open). Owner (2026-11-27): Q6 "ignore it"; Q5 "open the issue"; commit on main and push.

## Items

| ID | Item | Source | Code | GUI | Test | Evidence | Status |
|---|---|---|---|---|---|---|---|
| A1 | One NaN or infinity at the input no longer silences the plugin (2+ bands) or turns its output to NaN (ceiling off) | audit 1 | `src/crossover.rs:360` | none | `tests/host_behaviour.rs` `one_nan_sample_does_not_silence_the_output` | passes (`final/test-list/tree-T-TEST.log`); with the guard removed it fails (`[0.0, 0.356]`). Host route: REAPER passes no NaN to a plug-in (`nan-host.md`) | verified in process |
| A2 | One NaN no longer freezes Auto Gain | audit 2 | `src/autogain.rs:75` | none | `one_nan_sample_does_not_freeze_auto_gain` | passes; with only this guard removed it fails (`[1.40, 0.35]`, 12 dB too loud) | verified in process |
| A3 | `build.sh` stops when the SDK is not at the pin | audit 3 | `scripts/build.sh:26` | none | the check line, wrong and right pin | `sdk-pin-check.txt`. The shared SDK moved twice during this run (`19fac89`, `1f608fe`, `d93d926`), the case the check exists for | verified |
| A4 | Failing tests no longer write `~/Library/Logs/OpenClipMultiband/panic.log`; hosts still do | audit 4 | `src/lib.rs:318` | none | a test that panics after creating the plug-in; the same binary without `CARGO_MANIFEST_DIR` | `sdk-1f608fe/panic-log.txt`: 168 lines before, 168 after the failing test under cargo, 170 run as a host would; log restored to 168 | verified |
| A5 | The design crates no longer built or tested | audit 5 (this session: excluded; the other session: deleted) | crates deleted, `Cargo.toml` `[workspace]` | none | `cargo test --release --workspace` (the CI command) | `final/workspace.log`: 70 passed, 0 failed, 10 ignored; members olm-api, clip, clip-api, clip-master, openclipmultiband | verified (macOS) |
| A6 | One delay line, sound unchanged | audit 6 | `crates/clip/src/lib.rs:104` | none | renders in REAPER against 1.0.0 Beta | `final/crossbuild.txt`: VST3, CLAP, AU default, heavy, alt and reopened bit-identical to 1.0.0 Beta; the 1.0.0 Beta project reopens with every value as 1.0.0 Beta reopens it | verified |
| A7 | Release profile speed | audit 7 | `Cargo.toml:30` | none | interleaved bench, codegen-units 1 against 16, 3 rounds | `bench-cgu/result.txt`: codegen-units 1 is 10 % slower in all 15 cases (load 25 to 31); reverted with a comment. `final/test-list/tree-T-BENCH.log`: within 1 to 3 % of the baseline | audit finding wrong; reverted (Q7) |
| A8 | The full test list passes | `AGENTS.md` | `tests/`, `examples/bench.rs` | none | `run_tests.py --tier full` | `final/full-tier.txt`: T-LINT, T-CLIPPY, T-TEST, T-SOAK, T-FUZZ, T-BENCH pass (407 s) | verified |
| A9 | Signed build of all five formats, AAX PACE-signed, installed, nothing shadowing it | profile (Signing) | `scripts/build.sh` | none | codesign, wraptool, `/Library/Audio/Plug-Ins` | `final/install.txt`, `final/aax-sign.log`: all valid, Developer ID, timestamped 10:03; wraptool exit 0; 0 system copies | verified |
| A10 | Format validators pass | profile | shells | none | auval, pluginval strictness 10 (VST3, AU), clap-validator | `final/validators/`: auval SUCCEEDED; pluginval SUCCESS for VST3 and AU; clap-validator 33 passed, **1 failed**: `process-varying-sample-rates`, `activate()` returns false at 1234.57 Hz since `accepts_sample_rate` refuses rates under 8 kHz (the other session's change). Every earlier build passed 34 | verified, with one exception (Q6, owner: ignore) |
| A11 | REAPER suite passes in every format | profile | whole plugin | every control | `reaper_qa.py suite` | `final/reaper/{VST3,CLAP,AU}/report.md`: 9 of 9 PASS each (deterministic, every parameter heard and shown, state, editor reopen) | verified |
| A12 | OpenGrid9 suite passes | profile | whole plugin | none | `og9_qa.py suite` | `final/og9/{VST3,AU}`: every check PASS (editor checks SKIPPED: headless); `final/og9/LV2`: repeat render differs in its first 48 ms only (-3.5 dB), the known host exception (no reset between bounces), sweep SKIPPED as at 1.0.0 Beta (Q4) | verified (LV2 sweep SKIPPED, known exception) |
| A13 | Parameters and editor unchanged | PR-3, 1.0.0 Beta | `params()`, `src/ui/` untouched | every page | screenshots against `docs/images`; host parameter lists; crossbuild reopen | `final/screenshots.txt`: all six pixel-identical; suites: VST3 84, CLAP 83, AU 83 parameters as before | verified |
| A14 | Windows and Linux builds pass | profile | `.github/workflows/build.yml` (pin moved) | none | GitHub Actions on `ab13349` | run 46009911561: Windows x64 and Linux x86_64 (VST3, CLAP, LV2) success; `cargo test --release --workspace` in both | verified |
| A15 | The plug-in builds and passes on the latest SDK | owner | pin `d93d926` in `scripts/build.sh`, CI, `Cargo.toml`, `CHANGELOG.md` | none | everything above | A6 to A13 ran on `d93d926` | verified |
| A16 | The other session's changes hold in hosts | its audit | `src/lib.rs` (`max_latency`, dry delay, sample-rate floor) | none | its test `latency_never_exceeds_max_latency`, A6 to A13 | in the full test list (A8); hosts as above; the floor fails clap-validator (A10) | verified except A10 |

## Out of scope

| Item | Why | Who |
|---|---|---|
| GUI coverage, GUI performance, meter ballistics | no editor, painting, meter or parameter code changed | scope of the change |
| Pro Tools | no AAX-specific change; the AAX validates and is PACE-signed; driving Pro Tools takes the screen | scope (offer stands) |
| Bench baseline re-recording | the baseline still holds (A7) | not needed |

## Findings

| ID | Finding | Status |
|---|---|---|
| Q1 | The audit said finding 1 "most likely explains F9". Wrong for the REAPER runs where F9 was seen: REAPER turns a NaN in a media item into 0 before the FX chain and EEL2 cannot make one, so no NaN reaches a plug-in there (`nan-host.md`) | corrected |
| Q2 | Another session removed `OpenPluginSDK-main` (about 01:39); the shared link dangled | resolved: link to `/Volumes/Developer 2/Dependencies/OpenPluginSDK`, pin `d93d926` (owner) |
| Q3 | In the test REAPER (no audio device) a parameter write lands only with the next block; `crossbuild.py` waits 2 s for it, `nan.py` case 2 did not | noted (harness) |
| Q4 | OpenGrid9 LV2, second bounce: the first 48 ms differ. Intermittent in 1.0.0 Beta (1 of 3 runs) and now (3 of 4); the leftover is larger now (-3.5 dB peak against -19.1 dB). The host does not reset LV2 between bounces; no plug-in mechanism found (`og9-ab/result.txt`) | open (known exception) |
| Q5 | OpenPluginSDK `1b966b1` ("edits right after setProcessing(true) reach the processor's state"), with this plug-in's code before the other session's latency change: VST3 renders in REAPER after parameter writes start from a different state (first 50 ms -5 dB relative, under -100 dB by 0.35 s) in 7 of 7 runs, and 1 run stayed silent until the project was reopened. Not with SDK `19fac89` (0 of 3), not with that one line reverted on `d93d926` (0 of 2), not with the final tree (0 of 3, and the suite passes). Not reproducible in process; every parameter is idempotent (`sdk-1f608fe/zz_idem.rs`); REAPER sends no parameter points in those blocks (`sdk-1f608fe` diagnostic logs). Mechanism not found | open: OpenPluginSDK issue https://github.com/MistaMin/OpenPluginSDK/issues/62 (owner asked for it) |
| Q6 | clap-validator `process-varying-sample-rates` fails since the plug-in refuses rates under 8 kHz (A10). RT-6 allows refusing a rate; the validator counts it as a failure | accepted: owner "ignore it" (2026-11-27); the floor stays, the test is a listed exception |
| Q7 | Audit finding 7 had it backwards: `codegen-units = 1` costs 10 % here | fixed (reverted) |
| Q8 | F9 (a short disturbance in the middle of a REAPER render) is still intermittent: the suite's repeat render failed with SDK `19fac89` (1 of 2, load 100) and newer (2 of 3); passed in the final run | open (pre-existing) |

## Exceptions

| Check | Exception | Reason | Who |
|---|---|---|---|
| clap-validator `process-varying-sample-rates` | fails at 1234.57 Hz | the plug-in refuses rates under 8 kHz on purpose (its time constants are designed for 8 kHz and up); RT-6 allows refusing a rate | owner, 2026-11-27 (NEW) |
| OpenGrid9 LV2 repeat render | first 48 ms differ; sweep SKIPPED | the host does not reset LV2 between bounces (Q4) | known since 1.0.0 Beta |

## Open questions

None. Answered by the owner on 2026-11-27: Q6 ignore; Q5 issue opened (#62); commit on main and push (`ab13349`).
