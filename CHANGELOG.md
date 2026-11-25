# Changelog

## Unreleased

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
