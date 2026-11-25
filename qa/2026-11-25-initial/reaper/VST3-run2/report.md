# Plugin QA: VST3: OpenClip Multiband

| Check | Status | Detail |
|---|---|---|
| loads | PASS | VST3: OpenClip Multiband (OpenGrid), 84 parameters |
| finite output | PASS | peak -4.1 dBFS |
| not silent | PASS | rms -14.9 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1080x821, 0.6% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-1.4 dB) | ok (8.072%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (5.517%) |
| 2 | Zero Latency |  | ok (-1.0 dB) | ok (3.808%) |
| 3 | Auto Gain |  | ok (-6.0 dB) | ok (3.411%) |
| 4 | Delta |  | ok (-4.0 dB) | ok (3.424%) |
| 5 | Bands |  | ok (-1.7 dB) | ok (4.224%) |
| 6 | Crossover Slope |  | ok (1.4 dB) | ok (2.725%) |
| 7 | Crossover 1 |  | ok (-1.2 dB) | ok (3.296%) |
| 8 | Crossover 2 |  | ok (-1.4 dB) | ok (2.950%) |
| 9 | Crossover 3 |  | ok (-1.5 dB) | ok (3.017%) |
| 10 | Crossover 4 | Bands=0.8 | ok (-2.2 dB) | ok (4.971%) |
| 11 | Crossover 5 | Bands=1 | ok (-1.9 dB) | ok (5.263%) |
| 12 | Algorithm | All Bands Drive=1 | ok (2.7 dB) | ok (9.069%) |
| 13 | Shape | All Bands Drive=1 | ok (-7.5 dB) | ok (10.198%) |
| 14 | Character | All Bands Drive=1 | ok (-25.7 dB) | ok (10.270%) |
| 15 | Single/Multi Blend | All Bands Drive=1 | ok (-1.4 dB) | ok (9.609%) |
| 16 | Dry/Wet Mix | All Bands Drive=1 | ok (-1.4 dB) | ok (9.915%) |
| 17 | Single Drive | Single/Multi Blend=0 | ok (-1.9 dB) | ok (8.058%) |
| 18 | Single Ceiling | Single/Multi Blend=0 | ok (-3.5 dB) | ok (2.002%) |
| 19 | Output Ceiling On | Input Gain=1 | ok (2.5 dB) | ok (8.715%) |
| 20 | Output Ceiling | Input Gain=1 | ok (-1.5 dB) | ok (8.271%) |
| 21 | Anti-Alias | All Bands Drive=1 | ok (3.7 dB) | ok (5.332%) |
| 22 | Oversampling | All Bands Drive=1 | ok (4.0 dB) | ok (9.000%) |
| 23 | Bands Open |  | excused (-3.7 dB) | ok (5.690%) |
| 24 | Drawn Curve | All Bands Drive=1 | ok (-2.6 dB) | ok (10.284%) |
| 25 | Curve Point 1 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (11.089%) |
| 26 | Curve Point 2 | Drawn Curve=1, All Bands Drive=1 | ok (0.1 dB) | ok (10.511%) |
| 27 | Curve Point 3 | Drawn Curve=1, All Bands Drive=1 | ok (-2.4 dB) | ok (10.533%) |
| 28 | Curve Point 4 | Drawn Curve=1, All Bands Drive=1 | ok (-2.3 dB) | ok (10.046%) |
| 29 | Curve Point 5 | Drawn Curve=1, All Bands Drive=1 | ok (-2.8 dB) | ok (10.351%) |
| 30 | Curve Point 6 | Drawn Curve=1, All Bands Drive=1 | ok (-3.0 dB) | ok (10.392%) |
| 31 | Curve Point 7 | Drawn Curve=1, All Bands Drive=1 | ok (-2.1 dB) | ok (10.010%) |
| 32 | Curve Point 8 | Drawn Curve=1, All Bands Drive=1 | ok (-0.5 dB) | ok (10.388%) |
| 33 | Curve Point 9 | Drawn Curve=1, All Bands Drive=1 | ok (0.6 dB) | ok (9.994%) |
| 34 | Curve Point 10 | Drawn Curve=1, All Bands Drive=1 | ok (2.0 dB) | ok (10.822%) |
| 35 | Curve Point 11 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (10.076%) |
| 36 | Curve Point 12 | Drawn Curve=1, All Bands Drive=1 | ok (2.9 dB) | ok (10.465%) |
| 37 | Curve Point 13 | Drawn Curve=1, All Bands Drive=1 | ok (2.1 dB) | ok (10.244%) |
| 38 | Curve Point 14 | Drawn Curve=1, All Bands Drive=1 | ok (-1.1 dB) | ok (10.336%) |
| 39 | Curve Point 15 | Drawn Curve=1, All Bands Drive=1 | ok (-0.6 dB) | ok (10.122%) |
| 40 | Curve Point 16 | Drawn Curve=1, All Bands Drive=1 | ok (3.2 dB) | ok (10.445%) |
| 41 | All Bands Drive |  | ok (-1.4 dB) | ok (10.091%) |
| 42 | All Bands Ceiling |  | ok (-3.2 dB) | ok (3.450%) |
| 43 | All Bands Gain |  | ok (-1.7 dB) | ok (7.576%) |
| 44 | Band 1 Drive | Bands Open=1 | ok (-0.9 dB) | ok (6.378%) |
| 45 | Band 1 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-4.5 dB) | ok (6.343%) |
| 46 | Band 1 Gain | Bands Open=1 | ok (1.0 dB) | ok (6.605%) |
| 47 | Band 1 Solo |  | ok (-5.0 dB) | ok (3.945%) |
| 48 | Band 1 Mute |  | ok (-11.8 dB) | ok (2.982%) |
| 49 | Band 1 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (8.733%) |
| 50 | Band 2 Drive | Bands Open=1 | ok (-1.0 dB) | ok (7.477%) |
| 51 | Band 2 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-2.2 dB) | ok (5.887%) |
| 52 | Band 2 Gain | Bands Open=1 | ok (1.1 dB) | ok (7.137%) |
| 53 | Band 2 Solo |  | ok (-0.6 dB) | ok (3.427%) |
| 54 | Band 2 Mute |  | ok (-9.7 dB) | ok (2.879%) |
| 55 | Band 2 Bypass | All Bands Drive=1 | ok (-0.4 dB) | ok (9.368%) |
| 56 | Band 3 Drive | Bands Open=1 | ok (-0.9 dB) | ok (7.360%) |
| 57 | Band 3 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-2.3 dB) | ok (6.087%) |
| 58 | Band 3 Gain | Bands Open=1 | ok (1.0 dB) | ok (6.602%) |
| 59 | Band 3 Solo |  | ok (-1.6 dB) | ok (3.427%) |
| 60 | Band 3 Mute |  | ok (-10.8 dB) | ok (3.175%) |
| 61 | Band 3 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (8.737%) |
| 62 | Band 4 Drive | Bands Open=1 | ok (1.4 dB) | ok (8.286%) |
| 63 | Band 4 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-3.0 dB) | ok (5.309%) |
| 64 | Band 4 Gain | Bands Open=1 | ok (0.1 dB) | ok (7.062%) |
| 65 | Band 4 Solo |  | ok (-3.5 dB) | ok (3.203%) |
| 66 | Band 4 Mute |  | ok (-8.4 dB) | ok (3.205%) |
| 67 | Band 4 Bypass | All Bands Drive=1 | ok (1.8 dB) | ok (7.919%) |
| 68 | Band 5 Drive | Bands=0.8, Bands Open=1 | ok (-0.6 dB) | ok (9.511%) |
| 69 | Band 5 Ceiling | Bands=0.8, Bands Open=1, All Bands Ceiling=0.5 | ok (-13.0 dB) | ok (7.787%) |
| 70 | Band 5 Gain | Bands=0.8, Bands Open=1 | ok (-0.2 dB) | ok (8.851%) |
| 71 | Band 5 Solo | Bands=0.8, Bands Open=0 | ok (-3.7 dB) | ok (5.752%) |
| 72 | Band 5 Mute | Bands=0.8, Bands Open=0 | ok (-11.9 dB) | ok (5.435%) |
| 73 | Band 5 Bypass | Bands=0.8, All Bands Drive=1 | ok (1.0 dB) | ok (11.099%) |
| 74 | Band 6 Drive | Bands=1, Bands Open=1 | ok (-0.9 dB) | ok (8.692%) |
| 75 | Band 6 Ceiling | Bands=1, Bands Open=1, All Bands Ceiling=0.5 | ok (-4.4 dB) | ok (7.576%) |
| 76 | Band 6 Gain | Bands=1, Bands Open=1 | ok (-0.3 dB) | ok (8.393%) |
| 77 | Band 6 Solo | Bands=1, Bands Open=0 | ok (-3.0 dB) | ok (5.710%) |
| 78 | Band 6 Mute | Bands=1, Bands Open=0 | ok (-12.1 dB) | ok (5.331%) |
| 79 | Band 6 Bypass | Bands=1, All Bands Drive=1 | ok (0.0 dB) | ok (11.340%) |
| 80 | Bypass |  | ok (-1.7 dB) | excused (3.608%) |
| 81 | Bypass |  | ok (-1.7 dB) | excused (2.690%) |
| 82 | Wet |  | ok (-1.7 dB) | excused (2.743%) |
| 83 | Delta |  | ok (-3.5 dB) | excused (2.862%) |
