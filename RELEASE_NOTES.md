# OpenClip Multiband 1.0.1

A maintenance release of **OpenClip Multiband**, a 1-to-6-band multiband clipper by OpenGrid. The sound, the
parameters and the editor are the same as in 1.0.0 Beta: renders are bit-identical and sessions reopen unchanged.

## What's new

- **Latency changes reach the host at once:** the latency reported to the host changes in the same block as the
  Anti-Alias, Oversampling or Zero Latency change that causes it, so hosts realign immediately.
- **Tail:** the plug-in reports a 600 ms tail, the longest ring-down of its crossovers, so offline renders keep it.
- **Invalid input:** one invalid sample (NaN or infinity, for example from a plug-in earlier in the chain) no longer
  silences the plug-in for good, turns its output invalid or freezes Auto Gain.
- **Bypass delay:** the maximum latency given to hosts is the clippers' real worst case, so the Audio Unit and AAX
  bypass delay is long enough at every sample rate. Sample rates below 8 kHz are refused.
- **Curve points:** points 1, 3, 5 and 7 default to whole percents (13, 38, 63, 88 %) in every format. The sound is
  unchanged.
- **Audio thread:** no file is written from the audio thread any more, and output samples smaller than 1e-30 are set
  to zero, so hosts that do not flush denormals never receive one.
- Built on OpenPluginSDK `07f97e1`.

## Formats and requirements

- **macOS 11.0 or later (universal `arm64` + `x86_64`):** VST3, Audio Unit, CLAP, LV2 and PACE Eden-signed AAX (Pro
  Tools). Developer ID signed and Apple notarized `.pkg` installer.
- **Windows 10 or later (`x64`):** VST3, CLAP and LV2 in `OpenClipMultiband-1.0.1-windows-x64-setup.exe` (built by
  GitHub Actions; not code-signed, so SmartScreen may ask).
- **Linux (`x86_64`, glibc 2.35+):** VST3, CLAP and LV2 in `OpenClipMultiband-1.0.1-linux-x86_64.tar.gz` (built by
  GitHub Actions).

## Installation

- **macOS:** run `OpenClipMultiband-1.0.1-macos.pkg` and pick the formats you want; the manual and licences install
  into `/Library/Application Support/OpenClipMultiband`.
- **Windows:** run `OpenClipMultiband-1.0.1-windows-x64-setup.exe` and choose VST3, CLAP and/or LV2.
- **Linux:** extract `OpenClipMultiband-1.0.1-linux-x86_64.tar.gz` (`usr/lib/vst3`, `usr/lib/clap`, `usr/lib/lv2`,
  `usr/share/doc/openclipmultiband`) into `/` or your home folder's plug-in paths.

## Checks it passed

- **Automated tests:** the workspace tests and the full OpenPluginSDK test list (realtime lint, clippy, conformance
  kit, soak over 24 seeds, strict fuzz, benchmark against the baseline) on macOS; the workspace tests on Windows x64
  and Linux x86_64.
- **Format validators (macOS):** `auval`, `pluginval` (strictness 10, VST3 and AU) and Avid's AAX Validator pass.
  `clap-validator`: 33 of 34 tests pass; `process-varying-sample-rates` fails because the plug-in refuses its
  1234.57 Hz test rate (see Known limits).
- **Hosts (macOS):** REAPER (VST3, CLAP, Audio Unit: every parameter changes the sound and has a control, state
  survives save and reopen, renders repeat bit for bit, editor lifecycle) and OpenGrid9 (VST3, Audio Unit, LV2).
- **Against 1.0.0 Beta (REAPER; VST3, CLAP, Audio Unit):** renders at three settings are bit-identical, a 1.0.0 Beta
  project reopens with every value and renders bit-identically, and the six documented editor views are
  pixel-identical.
- **Delay compensation (REAPER; VST3, CLAP, Audio Unit):** after every Anti-Alias, Oversampling and Zero Latency
  change the host had the new latency at its first read (126 samples at 1x, 151 at 4x and above, 0 with Zero
  Latency at 48 kHz), and the dry path nulls against the source.

## Known limits

- Sample rates below 8 kHz are refused: the algorithms' time constants are designed for 8 kHz and up.
- Pro Tools: the AAX of this version passed Avid's AAX Validator and is PACE-signed, but it was not run in Pro Tools;
  the Pro Tools pass was made on 1.0.0 Beta. Delay compensation after an Oversampling change is untested there.
- Windows and Linux binaries passed the automated tests only, not a Windows or Linux DAW.
- The Windows installer is not code-signed.
- No standalone app.
- Logic Pro has not been tested.
- In Pro Tools, the first Return after typing a value into a control may not commit it; click the field and press
  Return again.

## Release files

- `OpenClipMultiband-1.0.1-macos.pkg` (+ `.sha256`): signed, notarized universal macOS installer (VST3, AU, CLAP, LV2,
  AAX, manual and licences)
- `OpenClipMultiband-1.0.1-windows-x64-setup.exe` (+ `.sha256`): Windows x64 installer (VST3, CLAP, LV2)
- `OpenClipMultiband-1.0.1-linux-x86_64.tar.gz` (+ `.sha256`): Linux x86_64 archive (VST3, CLAP, LV2)
- `OpenClipMultiband-User-Manual.pdf`: user manual and parameter reference
- `opengrid-manifest.json`: OpenGrid Plugin Manager manifest (schema 2)
