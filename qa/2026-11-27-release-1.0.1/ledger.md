# Scope ledger: OpenClip Multiband 1.0.1 release

Project: OpenClip Multiband   Code: branch `release-1.0.1` on `1187de4` (version 1.0.1, SDK pin moved)   Started: 2026-11-27
Build: Release, macOS universal (arm64 + x86_64, macOS 11.0), OpenPluginSDK `07f97e1` (latest `main`; the shared checkout
had moved from `d93d926` by two commits, PR #63, which changes nothing for a plug-in without host-space parameters).

Sources of scope:
- Request (2026-11-27): release; owner: version "1.0.1 (no Beta)", "Publish when green".
- The changes since 1.0.0 Beta (`CHANGELOG.md`, section 1.0.1), each QA'd in `qa/2026-11-26-test-list` and
  `qa/2026-11-27-audit-fixes`. This ledger re-runs the release checks on the 1.0.1 build.
- 1.0.0 Beta: sound, parameters, state and editor must not change.

Autogain and Delta: already in the plugin. Standalone: none (as 1.0.0 Beta).

## Items

| ID | Item | Test | Evidence | Status |
|---|---|---|---|---|
| R1 | Universal build of five formats, macOS 11.0, bundle version 1.0.1 | `lipo`, `otool`, Info.plist | `build.log`; all five `x86_64 arm64`, minos 11.0, 1.0.1 | verified |
| R2 | Developer ID signatures; AAX PACE-signed with the product's wrap GUID | codesign, wraptool | `installer.log`, `aax-sign.log` | verified |
| R3 | Installer signed, notarized, stapled; five formats plus manual and licences | notarytool, stapler, spctl, `pkgutil --expand` | `installer.log` (Accepted; "Notarized Developer ID") | verified |
| R4 | Full test list | `run_tests.py --tier full` | `full-tier.txt`: 6 passed, 0 failed (668 s) | verified |
| R5 | Workspace tests | `cargo test --release --workspace` | `workspace.log`: 70 passed, 0 failed, 9 ignored | verified |
| R6 | Validators | auval, pluginval strictness 10 (VST3, AU), clap-validator, AAX Validator | `validators/`: auval SUCCEEDED; pluginval SUCCESS twice; clap-validator 33 passed, 1 failed (`process-varying-sample-rates`: rates under 8 kHz refused; owner 2026-11-27 "ignore it"); AAX Validator 6 of 6 (`aax/`, local) | verified, one accepted exception |
| R7 | REAPER suite, VST3, CLAP, AU | `reaper_qa.py suite` | `reaper/*.suite.log`: 9 of 9 PASS each | verified |
| R8 | Sound and state equal to 1.0.0 Beta in REAPER | `crossbuild.py run R`, `compare R` | `crossbuild.txt`: every render bit-identical in VST3, CLAP, AU; the 1.0.0 Beta project reopens with every value; RESULT PASS | verified |
| R9 | Host delay compensation follows Anti-Alias, Oversampling, Zero Latency | `latency.py` | `latency-{VST3,CLAP,AU}.txt`: 126 / 151 / 0 at the first read; 1x renders null (-240 dB) | verified |
| R10 | OpenGrid9 suite, VST3, AU, LV2 | `og9_qa.py suite` | `og9/`: VST3 and AU every check PASS (editor SKIPPED: headless); LV2 repeat render differs (-3.2 dB), the known host exception, sweep SKIPPED | verified (LV2 sweep SKIPPED, known exception) |
| R11 | Editor unchanged | six screenshots against `docs/images` | `screenshots.txt`: pixel-identical | verified |
| R12 | Manual and documents match the code | manual text, PDF pages, README, changelog, notes | `docs/USER_MANUAL.md` (tail, rate floor, latency report), PDF rebuilt (7 pages, 1.0.1) | verified |
| R13 | Windows and Linux builds and tests | GitHub Actions on the release commit | see `publish.txt` | see `publish.txt` |
| R14 | AAX in Pro Tools | Pro Tools session | not run: Pro Tools was in the owner's use for the whole run; the 1.0.0 Beta pass stands for the unchanged parts | unverified (stated in the release notes) |

## Findings

| ID | Finding | Status |
|---|---|---|
| H1 | Renders came out as exact silence in 3 of the first 17 REAPER sessions (VST3 twice, AU once from a step on), at load 40 to 110 with other sessions' REAPER instances running. A diagnostic build (never committed) logged, per render, the peak the plug-in received and the peak it returned, and any run of blocks with input but no output: in 76 further sessions with 13 silent ones, every render was either input 0 and output 0, or both non-zero (397 and 793 renders), and no block had input without output (`silent-renders/`). The test REAPER fed the plug-in silence; the plug-in never dropped sound. A project reopened in the same REAPER played again. The checks were re-run one host at a time and pass (R8, R9). This also accounts for the "stuck silent" run of SDK issue #62 and part of F9. | harness; not a plug-in fault |
| H2 | The shared SDK checkout moved during the release (`d93d926` to `07f97e1`); `scripts/build.sh` stopped the first build, as it should. Pin moved (owner's standing instruction: follow the latest SDK). | resolved |

## Exceptions

| Check | Exception | Reason | Decided by |
|---|---|---|---|
| clap-validator `process-varying-sample-rates` | accepted failure | the plug-in refuses rates under 8 kHz | owner, 2026-11-27 |
| OpenGrid9 LV2 sweep | SKIPPED | the host does not reset LV2 between bounces | QA, as 1.0.0 Beta |

## Not done

| Item | Why |
|---|---|
| Pro Tools pass on 1.0.1 (and delay compensation after an Oversampling change) | Pro Tools in the owner's use |
| Windows and Linux in a DAW | built and unit-tested by GitHub Actions only |
| Logic Pro | not installed |
| Listening | the owner |
