# Plugin QA: VST3: OpenClip Multiband

| Check | Status | Detail |
|---|---|---|
| loads | PASS | VST3: OpenClip Multiband (OpenGrid), 84 parameters |
| finite output | PASS | peak -4.1 dBFS |
| not silent | PASS | rms -14.9 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1080x821, 0.3% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-1.4 dB) | ok (8.453%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (5.059%) |
| 2 | Zero Latency |  | ok (-6.0 dB) | ok (3.775%) |
| 3 | Auto Gain |  | ok (0.9 dB) | ok (3.296%) |
| 4 | Delta |  | ok (-4.0 dB) | ok (2.767%) |
| 5 | Bands |  | ok (-1.7 dB) | ok (4.692%) |
| 6 | Crossover Slope |  | ok (1.4 dB) | ok (2.887%) |
| 7 | Crossover 1 |  | ok (-1.2 dB) | ok (3.928%) |
| 8 | Crossover 2 |  | ok (-1.4 dB) | ok (3.535%) |
| 9 | Crossover 3 |  | ok (-1.5 dB) | ok (2.909%) |
| 10 | Crossover 4 | Bands=0.8 | ok (-2.2 dB) | ok (4.720%) |
| 11 | Crossover 5 | Bands=1 | ok (-1.9 dB) | ok (5.615%) |
| 12 | Algorithm | All Bands Drive=1 | ok (3.8 dB) | ok (9.573%) |
| 13 | Shape | All Bands Drive=1 | ok (-2.7 dB) | ok (10.052%) |
| 14 | Character | All Bands Drive=1 | ok (-13.8 dB) | ok (10.412%) |
| 15 | Single/Multi Blend | All Bands Drive=1 | ok (-1.4 dB) | ok (10.394%) |
| 16 | Dry/Wet Mix | All Bands Drive=1 | ok (-1.4 dB) | ok (10.542%) |
| 17 | Single Drive | Single/Multi Blend=0 | ok (-1.9 dB) | ok (8.857%) |
| 18 | Single Ceiling | Single/Multi Blend=0 | ok (-3.5 dB) | ok (3.599%) |
| 19 | Output Ceiling On | Input Gain=1 | ok (2.3 dB) | ok (8.594%) |
| 20 | Output Ceiling | Input Gain=1 | ok (-1.1 dB) | ok (8.172%) |
| 21 | Anti-Alias | All Bands Drive=1 | ok (3.7 dB) | ok (4.863%) |
| 22 | Oversampling | All Bands Drive=1 | ok (4.0 dB) | ok (9.270%) |
| 23 | Bands Open |  | excused (-3.7 dB) | ok (5.837%) |
| 24 | Drawn Curve | All Bands Drive=1 | ok (-2.6 dB) | ok (10.034%) |
| 25 | Curve Point 1 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (11.045%) |
| 26 | Curve Point 2 | Drawn Curve=1, All Bands Drive=1 | ok (0.1 dB) | ok (10.613%) |
| 27 | Curve Point 3 | Drawn Curve=1, All Bands Drive=1 | ok (-2.4 dB) | ok (10.344%) |
| 28 | Curve Point 4 | Drawn Curve=1, All Bands Drive=1 | ok (-2.3 dB) | ok (10.659%) |
| 29 | Curve Point 5 | Drawn Curve=1, All Bands Drive=1 | ok (-2.8 dB) | ok (10.167%) |
| 30 | Curve Point 6 | Drawn Curve=1, All Bands Drive=1 | ok (-3.0 dB) | ok (10.776%) |
| 31 | Curve Point 7 | Drawn Curve=1, All Bands Drive=1 | ok (-2.1 dB) | ok (10.156%) |
| 32 | Curve Point 8 | Drawn Curve=1, All Bands Drive=1 | ok (-0.5 dB) | ok (10.911%) |
| 33 | Curve Point 9 | Drawn Curve=1, All Bands Drive=1 | ok (0.6 dB) | ok (10.425%) |
| 34 | Curve Point 10 | Drawn Curve=1, All Bands Drive=1 | ok (2.0 dB) | ok (10.272%) |
| 35 | Curve Point 11 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (10.777%) |
| 36 | Curve Point 12 | Drawn Curve=1, All Bands Drive=1 | ok (2.9 dB) | ok (10.879%) |
| 37 | Curve Point 13 | Drawn Curve=1, All Bands Drive=1 | ok (2.1 dB) | ok (10.964%) |
| 38 | Curve Point 14 | Drawn Curve=1, All Bands Drive=1 | ok (-1.1 dB) | ok (10.591%) |
| 39 | Curve Point 15 | Drawn Curve=1, All Bands Drive=1 | ok (-0.6 dB) | ok (11.094%) |
| 40 | Curve Point 16 | Drawn Curve=1, All Bands Drive=1 | ok (3.2 dB) | ok (10.315%) |
| 41 | All Bands Drive |  | ok (-1.4 dB) | ok (10.659%) |
| 42 | All Bands Ceiling |  | ok (-3.2 dB) | ok (3.183%) |
| 43 | All Bands Gain |  | ok (-1.7 dB) | ok (8.023%) |
| 44 | Band 1 Drive | Bands Open=1 | ok (-0.9 dB) | ok (7.452%) |
| 45 | Band 1 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-13.3 dB) | ok (5.942%) |
| 46 | Band 1 Gain | Bands Open=1 | ok (1.0 dB) | ok (6.859%) |
| 47 | Band 1 Solo |  | ok (-5.7 dB) | ok (3.738%) |
| 48 | Band 1 Mute |  | ok (-11.8 dB) | ok (3.156%) |
| 49 | Band 1 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (8.965%) |
| 50 | Band 2 Drive | Bands Open=1 | ok (-1.0 dB) | ok (7.758%) |
| 51 | Band 2 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-12.8 dB) | ok (6.126%) |
| 52 | Band 2 Gain | Bands Open=1 | ok (1.1 dB) | ok (6.391%) |
| 53 | Band 2 Solo |  | ok (-4.9 dB) | ok (3.841%) |
| 54 | Band 2 Mute |  | ok (-9.7 dB) | ok (3.601%) |
| 55 | Band 2 Bypass | All Bands Drive=1 | ok (-0.4 dB) | ok (9.882%) |
| 56 | Band 3 Drive | Bands Open=1 | ok (-0.9 dB) | ok (8.296%) |
| 57 | Band 3 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-12.9 dB) | ok (6.301%) |
| 58 | Band 3 Gain | Bands Open=1 | ok (1.0 dB) | ok (6.574%) |
| 59 | Band 3 Solo |  | ok (-4.5 dB) | ok (3.884%) |
| 60 | Band 3 Mute |  | ok (-10.8 dB) | ok (3.077%) |
| 61 | Band 3 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (9.599%) |
| 62 | Band 4 Drive | Bands Open=1 | ok (1.4 dB) | ok (7.267%) |
| 63 | Band 4 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-10.7 dB) | ok (6.024%) |
| 64 | Band 4 Gain | Bands Open=1 | ok (0.1 dB) | ok (7.148%) |
| 65 | Band 4 Solo |  | ok (-6.6 dB) | ok (3.238%) |
| 66 | Band 4 Mute |  | ok (-8.4 dB) | ok (3.735%) |
| 67 | Band 4 Bypass | All Bands Drive=1 | ok (1.8 dB) | ok (9.047%) |
| 68 | Band 5 Drive | Bands=0.8, Bands Open=1 | ok (-0.6 dB) | ok (9.361%) |
| 69 | Band 5 Ceiling | Bands=0.8, Bands Open=1, All Bands Ceiling=0.5 | ok (-13.0 dB) | ok (7.848%) |
| 70 | Band 5 Gain | Bands=0.8, Bands Open=1 | ok (-0.2 dB) | ok (9.346%) |
| 71 | Band 5 Solo | Bands=0.8, Bands Open=0 | ok (-5.7 dB) | ok (5.701%) |
| 72 | Band 5 Mute | Bands=0.8, Bands Open=0 | ok (-11.9 dB) | ok (5.802%) |
| 73 | Band 5 Bypass | Bands=0.8, All Bands Drive=1 | ok (1.0 dB) | ok (11.108%) |
| 74 | Band 6 Drive | Bands=1, Bands Open=1 | ok (-0.9 dB) | ok (8.488%) |
| 75 | Band 6 Ceiling | Bands=1, Bands Open=1, All Bands Ceiling=0.5 | ok (-13.1 dB) | ok (8.076%) |
| 76 | Band 6 Gain | Bands=1, Bands Open=1 | ok (-0.3 dB) | ok (8.769%) |
| 77 | Band 6 Solo | Bands=1, Bands Open=0 | ok (-4.9 dB) | ok (5.675%) |
| 78 | Band 6 Mute | Bands=1, Bands Open=0 | ok (-12.1 dB) | ok (4.922%) |
| 79 | Band 6 Bypass | Bands=1, All Bands Drive=1 | ok (0.0 dB) | ok (11.212%) |
| 80 | Bypass |  | ok (-1.7 dB) | excused (3.504%) |
| 81 | Bypass |  | ok (-1.7 dB) | excused (2.204%) |
| 82 | Wet |  | ok (-1.7 dB) | excused (2.340%) |
| 83 | Delta |  | ok (-3.5 dB) | excused (2.975%) |
