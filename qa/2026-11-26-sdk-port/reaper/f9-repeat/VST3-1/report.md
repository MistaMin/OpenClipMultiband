# Plugin QA: VST3: OpenClip Multiband

| Check | Status | Detail |
|---|---|---|
| loads | PASS | VST3: OpenClip Multiband (OpenGrid), 84 parameters |
| finite output | PASS | peak -4.1 dBFS |
| not silent | PASS | rms -14.9 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1080x821, 0.0% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-1.4 dB) | ok (8.076%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (5.543%) |
| 2 | Zero Latency |  | ok (-1.0 dB) | ok (4.054%) |
| 3 | Auto Gain |  | ok (0.9 dB) | ok (2.716%) |
| 4 | Delta |  | ok (-4.0 dB) | ok (3.241%) |
| 5 | Bands |  | ok (-1.7 dB) | ok (4.202%) |
| 6 | Crossover Slope |  | ok (1.4 dB) | ok (3.494%) |
| 7 | Crossover 1 |  | ok (-1.2 dB) | ok (4.180%) |
| 8 | Crossover 2 |  | ok (-1.4 dB) | ok (4.228%) |
| 9 | Crossover 3 |  | ok (-1.5 dB) | ok (3.941%) |
| 10 | Crossover 4 | Bands=0.8 | ok (-2.2 dB) | ok (5.831%) |
| 11 | Crossover 5 | Bands=1 | ok (-1.9 dB) | ok (5.510%) |
| 12 | Algorithm | All Bands Drive=1 | ok (3.8 dB) | ok (10.393%) |
| 13 | Shape | All Bands Drive=1 | ok (-5.3 dB) | ok (10.542%) |
| 14 | Character | All Bands Drive=1 | ok (-25.7 dB) | ok (10.403%) |
| 15 | Single/Multi Blend | All Bands Drive=1 | ok (-1.4 dB) | ok (10.574%) |
| 16 | Dry/Wet Mix | All Bands Drive=1 | ok (-1.4 dB) | ok (10.080%) |
| 17 | Single Drive | Single/Multi Blend=0 | ok (-1.9 dB) | ok (8.406%) |
| 18 | Single Ceiling | Single/Multi Blend=0 | ok (-3.5 dB) | ok (2.626%) |
| 19 | Output Ceiling On | Input Gain=1 | ok (2.5 dB) | ok (8.510%) |
| 20 | Output Ceiling | Input Gain=1 | ok (-1.5 dB) | ok (8.127%) |
| 21 | Anti-Alias | All Bands Drive=1 | ok (3.7 dB) | ok (5.496%) |
| 22 | Oversampling | All Bands Drive=1 | ok (4.0 dB) | ok (9.492%) |
| 23 | Bands Open |  | excused (-3.7 dB) | ok (5.251%) |
| 24 | Drawn Curve | All Bands Drive=1 | ok (-2.6 dB) | ok (10.540%) |
| 25 | Curve Point 1 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (11.350%) |
| 26 | Curve Point 2 | Drawn Curve=1, All Bands Drive=1 | ok (0.1 dB) | ok (11.080%) |
| 27 | Curve Point 3 | Drawn Curve=1, All Bands Drive=1 | ok (-2.4 dB) | ok (10.981%) |
| 28 | Curve Point 4 | Drawn Curve=1, All Bands Drive=1 | ok (-2.3 dB) | ok (10.981%) |
| 29 | Curve Point 5 | Drawn Curve=1, All Bands Drive=1 | ok (-2.8 dB) | ok (11.219%) |
| 30 | Curve Point 6 | Drawn Curve=1, All Bands Drive=1 | ok (-3.0 dB) | ok (10.401%) |
| 31 | Curve Point 7 | Drawn Curve=1, All Bands Drive=1 | ok (-2.1 dB) | ok (11.346%) |
| 32 | Curve Point 8 | Drawn Curve=1, All Bands Drive=1 | ok (-0.5 dB) | ok (10.932%) |
| 33 | Curve Point 9 | Drawn Curve=1, All Bands Drive=1 | ok (0.6 dB) | ok (10.730%) |
| 34 | Curve Point 10 | Drawn Curve=1, All Bands Drive=1 | ok (2.0 dB) | ok (10.299%) |
| 35 | Curve Point 11 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (10.513%) |
| 36 | Curve Point 12 | Drawn Curve=1, All Bands Drive=1 | ok (2.9 dB) | ok (10.988%) |
| 37 | Curve Point 13 | Drawn Curve=1, All Bands Drive=1 | ok (2.1 dB) | ok (11.237%) |
| 38 | Curve Point 14 | Drawn Curve=1, All Bands Drive=1 | ok (-1.1 dB) | ok (10.798%) |
| 39 | Curve Point 15 | Drawn Curve=1, All Bands Drive=1 | ok (-0.6 dB) | ok (10.440%) |
| 40 | Curve Point 16 | Drawn Curve=1, All Bands Drive=1 | ok (3.2 dB) | ok (10.709%) |
| 41 | All Bands Drive |  | ok (-1.4 dB) | ok (10.873%) |
| 42 | All Bands Ceiling |  | ok (-3.2 dB) | ok (3.336%) |
| 43 | All Bands Gain |  | ok (-1.7 dB) | ok (7.341%) |
| 44 | Band 1 Drive | Bands Open=1 | ok (-0.9 dB) | ok (7.247%) |
| 45 | Band 1 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-4.5 dB) | ok (6.407%) |
| 46 | Band 1 Gain | Bands Open=1 | ok (1.0 dB) | ok (6.839%) |
| 47 | Band 1 Solo |  | ok (-5.0 dB) | ok (4.189%) |
| 48 | Band 1 Mute |  | ok (-11.8 dB) | ok (3.625%) |
| 49 | Band 1 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (9.479%) |
| 50 | Band 2 Drive | Bands Open=1 | ok (-1.0 dB) | ok (8.098%) |
| 51 | Band 2 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-2.2 dB) | ok (6.365%) |
| 52 | Band 2 Gain | Bands Open=1 | ok (1.1 dB) | ok (6.829%) |
| 53 | Band 2 Solo |  | ok (-0.6 dB) | ok (3.423%) |
| 54 | Band 2 Mute |  | ok (-9.7 dB) | ok (3.321%) |
| 55 | Band 2 Bypass | All Bands Drive=1 | ok (-0.4 dB) | ok (9.862%) |
| 56 | Band 3 Drive | Bands Open=1 | ok (-0.9 dB) | ok (7.684%) |
| 57 | Band 3 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-2.3 dB) | ok (5.775%) |
| 58 | Band 3 Gain | Bands Open=1 | ok (1.0 dB) | ok (6.846%) |
| 59 | Band 3 Solo |  | ok (-1.6 dB) | ok (3.807%) |
| 60 | Band 3 Mute |  | ok (-10.8 dB) | ok (2.891%) |
| 61 | Band 3 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (9.728%) |
| 62 | Band 4 Drive | Bands Open=1 | ok (1.4 dB) | ok (7.940%) |
| 63 | Band 4 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-3.0 dB) | ok (5.524%) |
| 64 | Band 4 Gain | Bands Open=1 | ok (0.1 dB) | ok (6.909%) |
| 65 | Band 4 Solo |  | ok (-3.5 dB) | ok (3.744%) |
| 66 | Band 4 Mute |  | ok (-8.4 dB) | ok (3.251%) |
| 67 | Band 4 Bypass | All Bands Drive=1 | ok (1.8 dB) | ok (8.794%) |
| 68 | Band 5 Drive | Bands=0.8, Bands Open=1 | ok (-0.6 dB) | ok (9.773%) |
| 69 | Band 5 Ceiling | Bands=0.8, Bands Open=1, All Bands Ceiling=0.5 | ok (-3.3 dB) | ok (8.056%) |
| 70 | Band 5 Gain | Bands=0.8, Bands Open=1 | ok (-0.2 dB) | ok (9.535%) |
| 71 | Band 5 Solo | Bands=0.8, Bands Open=0 | ok (-3.7 dB) | ok (6.102%) |
| 72 | Band 5 Mute | Bands=0.8, Bands Open=0 | ok (-11.9 dB) | ok (5.769%) |
| 73 | Band 5 Bypass | Bands=0.8, All Bands Drive=1 | ok (1.0 dB) | ok (11.274%) |
| 74 | Band 6 Drive | Bands=1, Bands Open=1 | ok (-0.9 dB) | ok (9.042%) |
| 75 | Band 6 Ceiling | Bands=1, Bands Open=1, All Bands Ceiling=0.5 | ok (-4.4 dB) | ok (8.565%) |
| 76 | Band 6 Gain | Bands=1, Bands Open=1 | ok (-0.3 dB) | ok (9.268%) |
| 77 | Band 6 Solo | Bands=1, Bands Open=0 | ok (-3.0 dB) | ok (6.320%) |
| 78 | Band 6 Mute | Bands=1, Bands Open=0 | ok (-12.1 dB) | ok (5.838%) |
| 79 | Band 6 Bypass | Bands=1, All Bands Drive=1 | ok (0.0 dB) | ok (11.555%) |
| 80 | Bypass |  | ok (-1.7 dB) | excused (3.944%) |
| 81 | Bypass |  | ok (-1.7 dB) | excused (3.149%) |
| 82 | Wet |  | ok (-1.7 dB) | excused (3.235%) |
| 83 | Delta |  | ok (-3.5 dB) | excused (2.481%) |
