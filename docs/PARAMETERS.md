# OpenClip Multiband parameter reference

Generated from the plugin (`cargo test --release --lib dump_manual_params -- --ignored`). OpenClip Multiband exposes **80 host parameters**: 44 global and 6 per band across 6 bands. Bands 2 through 6 use the same names, ranges and defaults as Band 1.

## Global

| Parameter | Range or choices | Default |
|---|---|---|
| Input Gain | -24 dB to 24 dB | 0 dB |
| Output Gain | -24 dB to 24 dB | 0 dB |
| Zero Latency | Off, On | Off |
| Auto Gain | Off, On | Off |
| Delta | Off, On | Off |

## Crossover

| Parameter | Range or choices | Default |
|---|---|---|
| Bands | 1, 2, 3, 4, 5, 6 | 4 |
| Crossover Slope | 12 dB, 24 dB, 48 dB | 24 dB |
| Crossover 1 | 20.0 Hz to 20.0 kHz | 120 Hz |
| Crossover 2 | 20.0 Hz to 20.0 kHz | 750 Hz |
| Crossover 3 | 20.0 Hz to 20.0 kHz | 3.00 kHz |
| Crossover 4 | 20.0 Hz to 20.0 kHz | 8.00 kHz |
| Crossover 5 | 20.0 Hz to 20.0 kHz | 14.0 kHz |

## Clipper

| Parameter | Range or choices | Default |
|---|---|---|
| Algorithm | Honey, Flint, Ghost, Ember, Magnet, Volt | Honey |
| Shape | 0% to 100% | 80% |
| Character | 0% to 100% | 0% |

## Blend

| Parameter | Range or choices | Default |
|---|---|---|
| Single/Multi Blend | 0% to 100% | 100% |
| Dry/Wet Mix | 0% to 100% | 100% |

## Single

| Parameter | Range or choices | Default |
|---|---|---|
| Single Drive | 0 dB to 24 dB | 0 dB |
| Single Ceiling | -24.0 dB to 0.0 dB | 0.0 dB |

## Output

| Parameter | Range or choices | Default |
|---|---|---|
| Output Ceiling On | Off, On | On |
| Output Ceiling | -24.0 dB to 0.0 dB | -1.0 dB |

## Clipper

| Parameter | Range or choices | Default |
|---|---|---|
| Anti-Alias | Hann-B1 ADAA, Adaptive ADAA, No ADAA | Hann-B1 ADAA |
| Oversampling | 1x, 4x, 8x, 16x, 32x, 64x | 1x |

## Global

| Parameter | Range or choices | Default |
|---|---|---|
| Bands Open | Off, On | Off |

## Curve

| Parameter | Range or choices | Default |
|---|---|---|
| Drawn Curve | Off, On | Off |
| Curve Point 1 | 0% to 100% | 13% |
| Curve Point 2 | 0% to 100% | 25% |
| Curve Point 3 | 0% to 100% | 38% |
| Curve Point 4 | 0% to 100% | 50% |
| Curve Point 5 | 0% to 100% | 63% |
| Curve Point 6 | 0% to 100% | 75% |
| Curve Point 7 | 0% to 100% | 88% |
| Curve Point 8 | 0% to 100% | 100% |
| Curve Point 9 | 0% to 100% | 100% |
| Curve Point 10 | 0% to 100% | 100% |
| Curve Point 11 | 0% to 100% | 100% |
| Curve Point 12 | 0% to 100% | 100% |
| Curve Point 13 | 0% to 100% | 100% |
| Curve Point 14 | 0% to 100% | 100% |
| Curve Point 15 | 0% to 100% | 100% |
| Curve Point 16 | 0% to 100% | 100% |

## All Bands

| Parameter | Range or choices | Default |
|---|---|---|
| All Bands Drive | 0 dB to 24 dB | 0 dB |
| All Bands Ceiling | -24.0 dB to 0.0 dB | 0.0 dB |
| All Bands Gain | -24 dB to 24 dB | 0 dB |

## Per band (Band 1 shown; identical for Bands 2 to 6)

| Parameter | Range or choices | Default |
|---|---|---|
| Band 1 Drive | -24 dB to 24 dB | 0 dB |
| Band 1 Ceiling | -24.0 dB to 24.0 dB | 0.0 dB |
| Band 1 Gain | -24 dB to 24 dB | 0 dB |
| Band 1 Solo | Off, On | Off |
| Band 1 Mute | Off, On | Off |
| Band 1 Bypass | Off, On | Off |
