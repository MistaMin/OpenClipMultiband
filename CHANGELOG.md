# Changelog

## 1.0.1 - 2026-11-27

- Builds against OpenPluginSDK 07f97e1, which carries GoodLookin DRI in its own tree; the separate GoodLookin DRI
  checkout is no longer needed. The DRI code is the same as in 1.0.0 Beta, so sound and editor are unchanged.
- Curve points 1, 3, 5 and 7 default to whole percents (13, 38, 63, 88 %), on the parameter's 1 % steps. Before, an
  Audio Unit reported 12.5, 37.5, 62.5 and 87.5 % until a session was saved and reopened. The sound is unchanged:
  the plug-in already ran with the whole-percent values.
- The latency reported to the host changes in the same block as the Anti-Alias, Oversampling or Zero Latency
  change that causes it (the audio still switches after its 5 ms fade), so hosts realign at once.
- The plug-in reports a 600 ms tail, the longest ring-down of its crossovers (all at 20 Hz, 48 dB, 6 bands).
- No file is written from the audio thread any more (a diagnostic for silent output is removed), and output samples
  smaller than 1e-30 are set to zero, so hosts that do not flush denormals never receive one.
- One invalid input sample (NaN or infinity, for example from a plug-in earlier in the chain) no longer silences the
  plug-in for good (2 bands or more) or turns its output invalid (Output Ceiling off), and no longer freezes Auto Gain.
- The maximum latency reported to hosts is the clippers' real worst case, so the AU and AAX bypass delay is long enough
  at every rate; sample rates below 8 kHz are refused.

## 1.0.0 Beta - 2026-11-26

First version, built from OpenMultiband 1.0.0 Beta.

- Three phase-aligned paths: per-band clipping (Multi), full-band clipping (Single) and Dry, with Blend and Mix, then a true-peak Output Ceiling (-1.0 dB).
- One Algorithm for every clipper: Honey, Flint, Ghost, Ember, Magnet, Volt; global Shape and Character.
- Every algorithm through Hann-B1 ADAA, adaptive ADAA or none, at 1x, 4x, 8x, 16x, 32x or 64x oversampling.
- Draw your own curve (pencil in the waveform box), used by every clipper with the algorithm's stages kept.
- One All Bands Drive / Ceiling / Gain set; open the bands to add each band's own on top. Solo, Mute, Bypass per band.
- Editor: large history of the original and clipped signal, whole mix and each band in its own colour, popping out into a resizable window that outlives the plug-in window; waveform box; crossover strip; Mix & Output sliders.
- History lanes show half the waveform (level folded up from the lane floor), twice as tall; each lane's zoom rises at
  once and settles at 6 dB a second, so it stays steady under heavy clipping.
- Value text for hosts and the editor: whole dB, ceilings in tenths, at most three digits, kHz from 1000 Hz. Curve
  points are percent of the ceiling.
- Magnet and Volt rise towards the ceiling (they used to level off 3 to 5 dB under it); Shape now sets how soft their
  knee is.
