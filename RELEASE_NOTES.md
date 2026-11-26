# OpenClip Multiband 1.0.0 Beta

First public pre-release of **OpenClip Multiband**, a 1-to-6-band multiband clipper by OpenGrid.

## What's in this release

- **Three phase-aligned paths:** per-band clipping (Multi), full-band clipping (Single) and Dry, blended with
  Single-Multi and Dry-Wet, then a true-peak Output Ceiling (-1.0 dB by default). The paths share one phase, so any
  blend adds up without comb filtering.
- **Six algorithms:** Honey (exponential knee), Flint (hard clip), Ghost (look-ahead smooth clip), Ember (tone-shaped,
  even harmonics), Magnet and Volt (soft saturation rising to the ceiling), with Shape and Character.
- **Anti-aliasing for every algorithm:** fitted-slope Hann-B1 ADAA (default), adaptive moving-slope ADAA or none, at
  1x, 4x, 8x, 16x, 32x or 64x oversampling.
- **Draw your own curve:** the pencil in the waveform box replaces the algorithm's curve in every clipper; its stages,
  the anti-aliasing and the oversampling stay.
- **Bands:** an all-pass-matched Linkwitz-Riley crossover (12, 24 or 48 dB/oct); one Drive / Ceiling / Gain set for all
  bands that opens into each band's own ±24 dB offsets; Solo, Mute and Bypass per band.
- **Editor:** a large history of the original against the clipped signal for the whole mix and each band in its own
  colour, which pops out into a resizable window that stays open without the plug-in window; Auto Gain, Delta, Zero
  Latency and seven themes.

## Formats and requirements

- **macOS 11.0 or later (universal `arm64` + `x86_64`):** VST3, Audio Unit, CLAP, LV2 and PACE Eden-signed AAX (Pro
  Tools). Developer ID signed and Apple notarized `.pkg` installer.
- **Windows 10 or later (`x64`):** VST3, CLAP and LV2 in `OpenClipMultiband-1.0.0-Beta-windows-x64-setup.exe` (built
  by GitHub Actions; not code-signed, so SmartScreen may ask).
- **Linux (`x86_64`, glibc 2.35+):** VST3, CLAP and LV2 in `OpenClipMultiband-1.0.0-Beta-linux-x86_64.tar.gz` (built
  by GitHub Actions).

## Installation

- **macOS:** run `OpenClipMultiband-1.0.0-Beta-macos.pkg` and pick the formats you want; the manual and licences
  install into `/Library/Application Support/OpenClipMultiband`.
- **Windows:** run `OpenClipMultiband-1.0.0-Beta-windows-x64-setup.exe` and choose VST3, CLAP and/or LV2.
- **Linux:** extract `OpenClipMultiband-1.0.0-Beta-linux-x86_64.tar.gz` (`usr/lib/vst3`, `usr/lib/clap`, `usr/lib/lv2`,
  `usr/share/doc/openclipmultiband`) into `/` or your home folder's plug-in paths.

## Checks it passed

- **Automated tests:** 112 Rust tests (algorithms, anti-aliasing and oversampling, phase alignment of every path,
  render repeatability, latency, editor layout, text and readability) on macOS, Windows x64 and Linux x86_64.
- **Format validators (macOS):** `auval`, `clap-validator`, `pluginval` (strictness 10, VST3 and AU) and Avid's AAX
  Validator.
- **Hosts (macOS):** REAPER (VST3, CLAP, Audio Unit: every parameter changes the sound and has a control, state
  survives save and reopen, renders repeat bit for bit, editor lifecycle), OpenGrid9 (VST3, Audio Unit, LV2) and Pro
  Tools Studio 2025.10 (AAX: offline and real-time bounces identical, bypass transparent, save and reopen recall,
  typing and dragging in the editor).

## Known limits

- Windows and Linux binaries passed the automated tests only, not a Windows or Linux DAW.
- The Windows installer is not code-signed.
- No standalone app.
- Logic Pro has not been tested.
- In Pro Tools, the first Return after typing a value into a control may not commit it; click the field and press
  Return again.
- First public beta (pre-release). Please report issues on GitHub.

## Release files

- `OpenClipMultiband-1.0.0-Beta-macos.pkg` (+ `.sha256`): signed, notarized universal macOS installer (VST3, AU, CLAP,
  LV2, AAX, manual and licences)
- `OpenClipMultiband-1.0.0-Beta-windows-x64-setup.exe` (+ `.sha256`): Windows x64 installer (VST3, CLAP, LV2)
- `OpenClipMultiband-1.0.0-Beta-linux-x86_64.tar.gz` (+ `.sha256`): Linux x86_64 archive (VST3, CLAP, LV2)
- `OpenClipMultiband-User-Manual.pdf`: user manual and parameter reference
- `opengrid-manifest.json`: OpenGrid Plugin Manager manifest (schema 2)
