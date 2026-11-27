# Scope ledger: QA of the test list and its fixes

Project: OpenClip Multiband   Commit: `main` at `397ba8c` (after `f31df7b` clippy clean), against the last QA'd build `c5091a2`   Started: 2026-11-26

Sources of scope:
- Request (2026-11-26): `/plugin-qa` after "commit" of the openplugin-dev test list.
- The changes since `c5091a2` (`git diff c5091a2 397ba8c`): clippy rewrites (`f31df7b`); the conformance kit, soak, fuzz,
  tail and bench tests; latency reported in the block of the Anti-Alias / Oversampling / Zero Latency write
  (`crates/clip/src/lib.rs` `latency_at`, `src/lib.rs` `total_latency`); a 600 ms tail (`src/lib.rs` `TAIL_MS`); the
  silent-output diagnostic removed; output under 1e-30 set to zero.
- Owner decision (2026-11-26): the output is not limited ("the audio engineer should know how to use his tools. We shouldn't
  limit the theoretical output"); the kit's output bound is the measured maximum plus 7 dB.
- The 1.0.0 Beta ledgers (`qa/2026-11-25-initial`, `qa/2026-11-26-release`) and the SDK port ledger
  (`qa/2026-11-26-sdk-port`): everything verified there must still hold, so the sound, parameters, state and editor must not
  change.

Autogain and Delta: already in the plugin, not asked again.

Result (2026-11-27 00:20): 16 items: 15 verified (S4 in process only; S12's LV2 sweep SKIPPED as at 1.0.0 Beta), 1 unverified (S16,
Pro Tools delay compensation: needs the owner's go-ahead). Out of scope: 4 (owner or scope of the change). Finding T1 fixed.

## Items

| ID | Item | Source | Code | GUI | Test | Evidence | Status |
|---|---|---|---|---|---|---|---|
| S1 | The clippy rewrites leave the sound bit-identical | `f31df7b` | `crates/clip-api/src/shaper.rs:611`, `src/crossover.rs:325,368` | none | in-process render of 32 settings, old and new code | `clippy-null.txt`: all 32 bit-identical (2026-11-26) | verified |
| S2 | Sound in hosts equal to 1.0.0 Beta: default, heavy, alt settings and the 1.0.0 Beta project reopened | 1.0.0 Beta ledgers | whole DSP | none | `crossbuild.py run C <fmt>` against phase A, VST3, CLAP, AU | `crossbuild.txt` (phase T against A, `../2026-11-26-sdk-port/crossbuild.py`): default, heavy, alt and reopened bit-identical to 1.0.0 Beta in VST3, CLAP, AU; the 1.0.0 Beta project reopens with every value as 1.0.0 Beta reopens it (AU curve points 1, 3, 5, 7 as in F8); RESULT PASS (load 24 to 111) | verified |
| S3 | The host's delay compensation follows Anti-Alias, Oversampling and Zero Latency changes | `397ba8c` | `src/lib.rs:498` (`total_latency`), `crates/clip/src/lib.rs:238` (`latency_at`) | Anti-Alias, Oversample, Zero Latency | REAPER `pdc` after each change; dry path (Mix 0, Output Ceiling off) rendered at 64x nulls with the source | `latency.txt`: in VST3, CLAP and AU REAPER had the new delay at the first read after every write (126 at 1x, 151 at 4x and 64x, 0 with Zero Latency); every render matches the source at lag 0 (1x: -240 dB; 4x and up: -29.6 dB from the oversampling filters); Zero Latency with Adaptive ADAA: lag +1 (its half-sample all-pass, by design) | verified |
| S4 | A 600 ms tail is reported | `397ba8c` | `src/lib.rs:209` (`TAIL_MS`), `:949` (`fn tail`) | none | `tests/tail.rs`; conformance `tail` check | `commit-tests.txt`: T-TEST passed (tail test at 44.1 and 96 kHz; kit `tail` check) | verified in process (REAPER does not show the tail it read) |
| S5 | No file is written from the audio thread | `397ba8c` | `src/lib.rs` (`watch_silence` removed) | none | log folder untouched by the host suites | `logs-check.txt`: the panic hook is the only `log_line` call; no 'silent output' line after the host runs (11 before, 11 after); the one new line (23:45) came from a leftover REAPER holding the old diagnostic build (finding T1) | verified |
| S6 | Output under 1e-30 is zero (no denormals for hosts without FTZ) | `397ba8c` | `src/lib.rs:694` | none | T-FUZZ (`strict_denormals`) | `commit-tests.txt`: T-FUZZ passed with `strict_denormals` | verified |
| S7 | The full test list passes on the commit | `AGENTS.md` | `tests/`, `examples/bench.rs`, `bench/baseline.json` | none | `run_tests.py --tier full --commit 397ba8c` | `commit-tests.txt`: 6 passed, 0 failed, 0 skipped (954 s) | verified |
| S8 | Every workspace test passes, release build | profile | all crates | none | `cargo test --release --workspace` | `cargo-test.log`: 117 passed, 0 failed, 13 ignored (the soak, fuzz, screenshot and bench tests run by S7 and S14) | verified |
| S9 | Signed build of all five formats, AAX PACE-signed, installed | profile (Signing) | `scripts/build.sh` | none | `codesign --verify`, `wraptool verify`, installed copies | `install.txt`: VST3, AU, CLAP, LV2 library, AAX valid, Developer ID, timestamped; runtime flag on all but the AAX (PACE re-signs it; the released AAX has the same flags); `wraptool verify` "signed, but not wrapped"; `aax-sign.log` | verified |
| S10 | Format validators pass | profile | shells | none | auval, pluginval strictness 10 (VST3, AU), clap-validator | `validators/`: auval SUCCEEDED; clap-validator 34 passed, 0 failed, 10 skipped; pluginval strictness 10 SUCCESS for VST3 and AU (load 136 to 151) | verified |
| S11 | REAPER suite passes in every format | profile | whole plugin | every control | `reaper_qa.py suite`, VST3, CLAP, AU | `reaper/{VST3,CLAP,AU}/report.md`: 9 of 9 checks PASS in each (load 38 to 159) | verified |
| S12 | OpenGrid9 suite passes | profile | whole plugin | none | `og9_qa.py suite`, VST3, AU, LV2 | `og9/{VST3,AU}/report.md`: every check PASS (editor checks SKIPPED: headless host); `og9/LV2/report.md`: repeat render differs by -19.1 dB in the first 20.8 ms only (`og9-lv2-repeat.txt`), the host not resetting LV2 between bounces, same figure as 1.0.0 Beta (`../2026-11-26-release/og9/LV2/report.md`), so its sweep is SKIPPED as then | verified (LV2 sweep SKIPPED: known host exception, unchanged) |
| S13 | Parameters unchanged: same 80 ids, names, ranges, defaults as 1.0.0 Beta | PR-3 | `src/lib.rs` `params()` | every control | host parameter dump against `qa/2026-11-26-sdk-port/params-1.0.0-beta-*.tsv` | `params-diff.txt`: VST3 84, CLAP 83, AU 83 parameters, index, ident, name and default equal to 1.0.0 Beta; only the AU's Curve Points 1, 3, 5, 7 read 13/38/63/88 % (F8) | verified |
| S14 | Editor unchanged | 1.0.0 Beta | `src/ui/` (two clippy attributes only) | every page | the six documented screenshots against `docs/images` | `screenshots.txt`: all six pixel-identical to `docs/images` | verified |
| S15 | Windows and Linux builds pass | profile | `.github/workflows/build.yml` | none | GitHub Actions on `397ba8c` | `commit-tests.txt`: run 45904341821, Windows x64 and Linux x86_64 success | verified |
| S16 | Pro Tools compensates the AAX's latency after an Oversampling change | `397ba8c` | AAX shell, `total_latency` | Oversample | Pro Tools session, delay compensation readout | Pro Tools log `Pro_Tools_2026_11_27_00_04_47.txt` (a launch by someone else): it scanned the new AAX (build 23:34:44) with a valid cache and no error, so it loads; the delay compensation after an Oversampling change was not tested (Pro Tools was in someone else's use, and the test takes the screen and pointer) | unverified (needs the owner's go-ahead to drive Pro Tools) |

## Out of scope

| Item | Why | Who |
|---|---|---|
| Output level limit | kept unlimited on purpose; kit bound = measured +98.9 dBFS plus 7 dB | owner, 2026-11-26 |
| GUI performance gate | no painting, meter or text code changed (`src/ui/` diff is two clippy attributes) | scope of the change |
| Meter ballistics | no meter or analyser code changed | scope of the change |
| Bench baseline re-recording | needs a quiet machine; recorded at load 23 | not this run |

## Findings

| ID | Finding | Status |
|---|---|---|
| T1 | Two REAPER test instances from the F9 diagnosis (scratchpad `silence/run1`, `silence/diag1`) kept running for 8 hours with the diagnostic CLAP build loaded (from the deleted `OpenClipMultiband.clap.diag-tmp`): one used about one core, and at 23:45 one wrote a "DIAG reset" line to `~/Library/Logs/OpenClipMultiband/panic.log`. The three REAPERs of this run's `latency.py` were also left running: its quit call timed out and the script assumed REAPER had gone. All five closed at 00:05 (they were this session's: their `reaper.log` paths). `latency.py` and `crossbuild.py` now quit through `reaper_qa.py quit`, which kills REAPER by its pid file. Part of F9's "heavy load" was these instances. | fixed (00:05) |

## Open questions
