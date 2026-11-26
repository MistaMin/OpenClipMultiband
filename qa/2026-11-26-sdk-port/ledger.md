# Scope ledger: port OpenClip Multiband to the new OpenPluginSDK

Project: OpenClip Multiband   Branch / commit: from `main` at `8b27625` (tag `v1.0.0-beta`); port `9b7de27` on `sdk-dri-in-tree`, merged into `main`   Started: 2026-11-26   Agent: Claude Code (Opus 5.5)

Sources of scope:
- Request (2026-11-26): "port to new openpluginsdk".
- OpenPluginSDK `main` at `19fac89` (pull request #55, "GoodLookin DRI in the tree"): the SDK now carries GoodLookin DRI
  as a git subtree under `crates/goodlookindri` and names it by path; it no longer takes it from GitHub, and its CI drops
  the sibling checkout (`docs/PUBLISHING.md`). This plugin is pinned to `53e198c` with DRI `3fe0426` from the sibling
  folder `Dependencies/GoodLookinDri`.
- The 1.0.0 Beta ledgers: `qa/2026-11-25-initial/ledger.md` (S1 to S28) and `qa/2026-11-26-release/ledger.md` (S29 to S33).
  Every item there is carried over: the port must keep all of them.
- The project profile (`plugin-qa/projects/openclipmultiband.md`) and the plugin author wiki (openplugin-dev: PR-2 identity,
  PR-3 parameters are part of the session format).

What the port is: the plugin's own code does not change. The SDK's changes between the two revisions are the DRI move
and its CI (S5, S6), so the port is the dependency wiring (S1 to S4) plus the proof that nothing a user hears, sees or
saved changed (S9 to S12).

Autogain and Delta: already in the plugin (Auto Gain, Delta; first ledger), not asked again.

## Items

| ID | Item | Source | Code | GUI | Test | Evidence | Status |
|---|---|---|---|---|---|---|---|
| S1 | Builds against OpenPluginSDK `main` `19fac89` | request | `scripts/build.sh:8` (`SDK_REV`), `.github/workflows/build.yml:13` (`OPENPLUGIN_SDK_REF`), `Cargo.toml:16-19` (comments) | none: build wiring | `build-check.txt` (no notes; SDK at `19fac89`), `build.log` | verified |
| S2 | GoodLookin DRI comes from the SDK's tree: one copy of each DRI crate, none from the sibling folder | SDK `docs/PUBLISHING.md` | `Cargo.toml:20` (`goodlookindri-shell` path to `OpenPluginSDK/crates/goodlookindri/shell`), `Cargo.toml:22-26` (`[patch]` removed), `Cargo.lock` | none | `cargo tree -i goodlookindri` and `-i goodlookindri-shell`: one package each, at the SDK path; no "patch was not used" warning | `build-check.txt`: one `goodlookindri` and one `goodlookindri-shell`, both under `OpenPluginSDK/crates/goodlookindri`; no package from the sibling folder; `Cargo.lock` unchanged | verified |
| S3 | The build script no longer looks for the sibling DRI | S2 | `scripts/build.sh:9-10,28-29` (`DRI`, `DRI_REV` and their check) | none | `grep` empty; `build.log` exit 0 | verified |
| S4 | Windows and Linux builds without the GoodLookinDri checkout | S2 | `.github/workflows/build.yml:14` (`GOODLOOKINDRI_REF`), `:34-38` and `:91-95` (the checkouts), the comment at `:25` | none | GitHub Actions run 45848878649 on `9b7de27`: Windows x64 and Linux x86_64 success, 112 passed each, SDK ref `19fac89`, no DRI checkout; artifacts `windows-x64` (2.3 MB), `linux-x86_64` (4.1 MB) | verified |
| S5 | The DRI code is the one 1.0.0 Beta shipped | SDK diff | none | none | tree of `19fac89:crates/goodlookindri` against `3fe0426^{tree}` | `sdk-diff.txt`: both `aa38ec9` (2026-11-26) | verified |
| S6 | No other SDK change reaches the plugin | SDK diff | none | none | `git diff 53e198c 19fac89` outside `crates/goodlookindri`: CI, `Cargo.lock` (2 source lines), `Cargo.toml` (path entries), `docs/PUBLISHING.md` only | `sdk-diff.txt` (2026-11-26) | verified |
| S7 | Unit and project tests pass | profile | workspace | none | `cargo-test.log`: 112 passed, 0 failed, no warnings | verified |
| S8 | macOS build: universal (arm64 + x86_64), macOS 11.0, VST3, AU, CLAP, LV2, AAX | profile | `scripts/build.sh` | none | `build-check.txt`: all five `x86_64 arm64`, macOS 11.0 | verified |
| S9 | The sound is unchanged | port | `src/`, `crates/` (unchanged) | none | `crossbuild.txt` (`crossbuild.py`): VST3, CLAP and AU at the defaults, the heavy setting (6 bands, Ember, adaptive ADAA, 16x) and a Zero Latency / Ghost / drawn-curve setting: bit-identical to 1.0.0 Beta. First try in `reaper/first-try/`: renders started right after a change differ for 0.3 to 1 s in both builds alike (the change lands at a different time each run); the method now waits 2 s and renders once before measuring | verified |
| S10 | The editor is unchanged | port | `src/ui/` (unchanged) | every page, the six documented states | `screenshots.txt`: all six pixel-identical; 4 bands open and 6 bands (graphite) looked at | verified |
| S11 | All 80 parameters keep their ids, order, names and defaults (PR-3) | port; wiki PR-3 | `src/lib.rs` (unchanged) | covered by S14's visual sweep | dump the host list from the new build in REAPER (VST3, CLAP, AU) and diff it with `params-1.0.0-beta-*.tsv`: no difference | `params-diff.txt`: VST3 84, CLAP 83, AU 83: index, ident, name and default equal | verified |
| S12 | A session saved with 1.0.0 Beta reopens with the same settings (PR-2, PR-3) | port; wiki PR-2 | state code (unchanged) | none | `crossbuild.txt`: each 1.0.0 Beta project reopens in the new build with every value equal to what 1.0.0 Beta itself reopens, and its render bit-identical. AU: four curve points read back 0.13 where 0.125 was saved, the same in 1.0.0 Beta (F8) | verified |
| S13 | Format validators pass | profile | none | none | `validators/`: auval PASS; pluginval strictness 10 SUCCESS (VST3, AU); clap-validator 34 passed, 0 failed, 10 skipped | verified |
| S14 | REAPER suites pass | profile | none | editor | VST3 `reaper/VST3/report.md` all PASS; AU rerun `reaper/AU/report.md` all PASS; CLAP `reaper/clap-under-load/new2`, `new3` all PASS. Failed runs while the Mac was loaded (F9): `reaper/first-run/` (CLAP, AU), `reaper/CLAP/`, `clap-under-load/suite1`; 1.0.0 Beta fails the same way (`clap-under-load/old2`) | verified (F9) |
| S15 | OpenGrid9 suites pass | profile | none | none (headless) | `og9/*/report.md`: VST3 and AU all PASS; LV2 loads, finite, not silent, state PASS, repeat renders differ in the first 28 ms only (F5, as in the release) | verified |
| S16 | The documents say where the DRI comes from | S2 | `README.md:83-85` (Build from source), `CHANGELOG.md` (Unreleased), `THIRD_PARTY_LICENSES.md:11` | none | README build section and CHANGELOG Unreleased line; THIRD_PARTY_LICENSES line still true (same MIT code, now from the SDK tree) | verified |
| S17 | The profile, the wiki page and memory carry the new pin | profile | `plugin-qa/projects/openclipmultiband.md:4`, wiki `pages/openclipmultiband.md`, memory | none | profile line 4 and OpenGrid9 note, wiki Stack row and `environment.md`, memory; wiki check: no new failures | verified |
| S18 | Commit as `MistaMin` on a side copy, no AI attribution lines, no third-party brand names; merged into `main` after S4 | conventions; Q3 | git | none | `9b7de27` by `MistaMin <80925498+MistaMin@users.noreply.github.com>`, no attribution lines; brand names only where the release ledger already has them (host names in QA reports); merged into `main` by fast-forward, with this ledger update on top | verified |
| S19 | OpenMultiband builds against `19fac89` with the DRI from the SDK's tree | Q1 | `OpenMultiband/Cargo.toml:18-28`, `scripts/build.sh:8-10` and its DRI check, `.github/workflows/build.yml:13-14,34-38,91-95` | none | `cargo tree -i goodlookindri`: one package at the SDK path | verified |
| S20 | OpenMultiband's tests pass and it builds (its SDK jump `c6fa6a0` to `19fac89` also brings ARA, opt-in, and the VST3 reset fix) | Q1 | workspace | none | `cargo test --release --workspace`: 99 passed, 0 failed, no warnings; `scripts/build.sh` exit 0, no notes, all five formats `x86_64 arm64`, macOS 11.0 (logs in the session scratchpad) | verified |
| S21 | OpenDynamics (branch `feature/auto-loudness-ara`) builds against `19fac89` with the DRI from the SDK's tree | Q1 | `OpenDynamics/Cargo.toml:17-27`, `scripts/build.sh:15-17` and its DRI check, `.github/workflows/*.yml` | none | `cargo tree -i goodlookindri`: one package at the SDK path | verified |
| S22 | OpenDynamics' tests pass and it builds (its SDK jump `f3f9962` to `19fac89` also brings the VST3 reset fix) | Q1 | workspace | none | `cargo test --release --workspace`: 115 passed, 0 failed, no warnings; `scripts/build.sh` exit 0, no notes, all five formats `x86_64 arm64`, macOS 11.0 (logs in the session scratchpad) | verified |
| S23 | F8: curve points 1, 3, 5, 7 default to whole percents (13, 38, 63, 88 %) | owner, 2026-11-26 ("whole numbers on F8") | `src/lib.rs:169-173` (`.round()`), test `defaults_resolve` | Curve Point 1 to 16 (waveform box, host list) | `defaults_resolve` checks all 16 defaults are whole; REAPER: the F8 build's AU reports 13/38/63/88 % before a save and after reopening a 1.0.0 Beta project; every render bit-identical to 1.0.0 Beta (the SDK already snapped the values to the 1 % steps when it pushed them) | `cargo-test-f8.log` (112 passed); `crossbuild-F8.txt`; `reaper/F8-AU/params.json` | verified |
| S24 | OpenClip Multiband signed: VST3, AU, CLAP, LV2 with Developer ID (hardened runtime, timestamp); AAX also PACE-signed with the product's wrap GUID; installed | owner, 2026-11-26 | `scripts/build.sh` with `CODESIGN_IDENTITY`; release skill `sign_aax.sh` | none | `codesign --verify --deep --strict` and the authority of each installed bundle; `wraptool verify`; auval, pluginval (strictness 10), clap-validator on the signed bundles | all five installed bundles valid, authority Developer ID Application, runtime flag, timestamped; AAX PACE-signed ("signed, but not wrapped", as for every AAX); auval on the installed AU, pluginval strictness 10 (VST3, AU) SUCCESS, clap-validator 34 passed, 0 failed (`validators/*-signed.log`); `build-signed.log`, `aax-sign.log`; six screenshots from the F8 code pixel-identical (`screenshots.txt`) | verified |
| S25 | openplugin-dev's commit tester builds a commit whose SDK sits outside the repository, and its realtime lint leaves test modules out, as its skill says | owner, 2026-11-26 | openplugin-dev `scripts/run_tests.py` (`new_worktree`), `scripts/rt_lint.py` (`test_module_files`), 0.1.1 (`3005e75`) | none | 2 new self-tests (both fail on the old scripts), 15 of 15 pass, CI green; the fixed tester on `46731a9`: lint PASS, `cargo test --release` PASS (clippy still fails: 18 warnings, a project matter, PR-5) | openplugin-dev CI run 45852811777 | verified |
| S26 | OpenMultiband's port committed and merged | owner, 2026-11-26 ("commit and merge all") | OpenMultiband `09d7618` | none | GitHub Actions run 45852124838: Windows and Linux success, 99 passed each, no DRI checkout; `main` fast-forwarded | | verified |
| S27 | OpenDynamics' port committed and merged (from its `main`, which already holds the merged Auto Loudness and ARA work) | owner, 2026-11-26 | OpenDynamics `cfb017e` | none | GitHub Actions run 45852140643: Windows and Linux success, 115 passed each, no DRI checkout; `main` fast-forwarded | | verified |
| S28 | F9 repeated: REAPER suites twice in each format on the signed F8 build, load recorded | owner, 2026-11-26 ("F9: repeat") | none | editor | `reaper_qa.py suite` VST3, CLAP, AU, two rounds | `reaper/f9-repeat/`: VST3, CLAP and AU, two rounds each, every check PASS (deterministic -240 dB, every parameter heard and shown, state), load averages 12 to 26: F9 did not repeat | verified |

## Findings

- **F8 (exists in 1.0.0 Beta; fixed, S23).** The odd curve points defaulted to 12.5, 37.5, 62.5 and 87.5 % while the
  parameter moves in 1 % steps, so the AU reported 12.5 % until a session was saved and reopened, then 13 %. The plug-in
  itself already ran with 13 % (the SDK snaps values to the steps when it pushes them), so the sound never differed. The
  defaults are now whole percents (owner, 2026-11-26).
- **F9 (exists in 1.0.0 Beta).** With the Mac heavily loaded by other sessions (load average 17 to 73), CLAP and AU in
  REAPER are sometimes not repeatable: a 40 to 240 ms disturbance in the middle of one render, and in one run silence
  for the rest of the session. The 1.0.0 Beta CLAP does the same; not a reset or activation during a block (diagnostic
  build), no timing logic while processing. Cause not found: `f9-under-load.txt`. Repeated on the signed F8 build
  (S28): six suites at load averages 12 to 26 all passed; it came back only above that load, or with OpenGrid9
  suites running alongside.

## Parameters (host list vs ledger)

Host list dumped from: the 1.0.0 Beta release build in REAPER 7.80 (`qa/2026-11-26-release/reaper/*/report.json`), saved as
`params-1.0.0-beta-VST3.tsv` (84: the 80 plug-in parameters, the VST3 Bypass, and REAPER's Bypass, Wet and Delta),
`params-1.0.0-beta-CLAP.tsv` and `params-1.0.0-beta-AU.tsv` (83 each: the 80, and REAPER's three).
All in a row above: yes. The port changes no parameter code, so every parameter is in S11 (kept, diffed against the
baseline) and S14 (the audio and visual sweeps). None is renamed or dropped.

## Exceptions (each needs a reason)

| Parameter / check | Exception | Reason | Decided by |
|---|---|---|---|
| The release ledger's exceptions (Bands Open audio; host Bypass, Wet, Delta visual; LV2 Freewheel; OpenGrid9 LV2 sweep; AAX "many steps") | carried over | unchanged plug-in; reasons in `qa/2026-11-26-release/ledger.md` | QA |

## Open questions (answered by the owner, 2026-11-26)

- **Q1. The shared SDK folder.** OpenMultiband and OpenDynamics build from the same `Dependencies/OpenPluginSDK` folder and
  take the DRI from the sibling folder; with that folder at `19fac89` neither resolves ("package collision in the
  lockfile", `resolve-check.txt`). Answer: port them too, with the same change, then their unit tests and a build (S19
  to S22); full host QA only if the owner asks.
- **Q2. The openplugin-dev test list** (`tests/conformance.rs`, `host_behaviour.rs`, `soak.rs`, `examples/bench.rs`,
  `bench/baseline.json`, `AGENTS.md`). Answer: not now; its own task later.
- **Q3. How the port ends.** Answer: commit on a side copy, run S4 on GitHub, merge into the main version; no release.

## Out of scope

| Item | Decided by | Where |
|---|---|---|
| A release; PACE signing, AAX Validator, Pro Tools | owner, 2026-11-26 (Q3: no release). The AAX code is unchanged (S5, S6); S8 builds it unsigned | Q3 |
| The openplugin-dev test list | owner, 2026-11-26 (Q2: not now) | Q2 |
| Commits of OpenMultiband and OpenDynamics, and their host QA | Q1 covers the change, tests and a build; committing them is asked separately | Q1 |
| Editor frame cost | QA: the DRI code is the same (S5); last measured 15.4 to 16.7 ms at 2x | `qa/2026-11-26-release/frame-cost.txt` |
| Windows and Linux in a DAW, Logic Pro | as in the release: no host here | release ledger, "Not done" |
| Listening | owner; S9 proves the sound bit-identical | |
| The 1.0.0 Beta Plugin Manager upload | owner, separate (it needs the hosting password) | release report |
| OpenLimiter | not affected: it builds from its own `deps/` copy | `OpenLimiter/Cargo.toml:30-42` |
