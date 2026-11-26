# Plugin QA: VST3: OpenClip Multiband

| Check | Status | Detail |
|---|---|---|
| loads | PASS | VST3: OpenClip Multiband (OpenGrid), 84 parameters |
| finite output | PASS | peak -4.1 dBFS |
| not silent | PASS | rms -14.9 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1080x821, 0.7% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-1.4 dB) | ok (7.836%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (5.286%) |
| 2 | Zero Latency |  | ok (-1.0 dB) | ok (3.521%) |
| 3 | Auto Gain |  | ok (-6.0 dB) | ok (2.705%) |
| 4 | Delta |  | ok (-4.0 dB) | ok (2.446%) |
| 5 | Bands |  | ok (-1.7 dB) | ok (4.586%) |
| 6 | Crossover Slope |  | ok (1.4 dB) | ok (3.133%) |
| 7 | Crossover 1 |  | ok (-1.2 dB) | ok (3.132%) |
| 8 | Crossover 2 |  | ok (-1.4 dB) | ok (3.128%) |
| 9 | Crossover 3 |  | ok (-1.5 dB) | ok (3.057%) |
| 10 | Crossover 4 | Bands=0.8 | ok (-2.2 dB) | ok (5.510%) |
| 11 | Crossover 5 | Bands=1 | ok (-1.9 dB) | ok (5.077%) |
| 12 | Algorithm | All Bands Drive=1 | ok (3.8 dB) | ok (9.506%) |
| 13 | Shape | All Bands Drive=1 | ok (-5.3 dB) | ok (9.858%) |
| 14 | Character | All Bands Drive=1 | ok (-25.7 dB) | ok (10.162%) |
| 15 | Single/Multi Blend | All Bands Drive=1 | ok (-1.4 dB) | ok (9.876%) |
| 16 | Dry/Wet Mix | All Bands Drive=1 | ok (-1.4 dB) | ok (9.578%) |
| 17 | Single Drive | Single/Multi Blend=0 | ok (-1.9 dB) | ok (8.140%) |
| 18 | Single Ceiling | Single/Multi Blend=0 | ok (-3.5 dB) | ok (2.804%) |
| 19 | Output Ceiling On | Input Gain=1 | ok (2.5 dB) | ok (8.717%) |
| 20 | Output Ceiling | Input Gain=1 | ok (-1.5 dB) | ok (8.605%) |
| 21 | Anti-Alias | All Bands Drive=1 | ok (3.7 dB) | ok (4.985%) |
| 22 | Oversampling | All Bands Drive=1 | ok (4.0 dB) | ok (9.290%) |
| 23 | Bands Open |  | excused (-3.7 dB) | ok (5.077%) |
| 24 | Drawn Curve | All Bands Drive=1 | ok (-2.6 dB) | ok (9.851%) |
| 25 | Curve Point 1 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (10.987%) |
| 26 | Curve Point 2 | Drawn Curve=1, All Bands Drive=1 | ok (0.1 dB) | ok (10.697%) |
| 27 | Curve Point 3 | Drawn Curve=1, All Bands Drive=1 | ok (-2.4 dB) | ok (10.094%) |
| 28 | Curve Point 4 | Drawn Curve=1, All Bands Drive=1 | ok (-2.3 dB) | ok (10.749%) |
| 29 | Curve Point 5 | Drawn Curve=1, All Bands Drive=1 | ok (-2.8 dB) | ok (10.030%) |
| 30 | Curve Point 6 | Drawn Curve=1, All Bands Drive=1 | ok (-3.0 dB) | ok (10.297%) |
| 31 | Curve Point 7 | Drawn Curve=1, All Bands Drive=1 | ok (-2.1 dB) | ok (10.212%) |
| 32 | Curve Point 8 | Drawn Curve=1, All Bands Drive=1 | ok (-0.5 dB) | ok (9.887%) |
| 33 | Curve Point 9 | Drawn Curve=1, All Bands Drive=1 | ok (0.6 dB) | ok (10.355%) |
| 34 | Curve Point 10 | Drawn Curve=1, All Bands Drive=1 | ok (2.0 dB) | ok (10.386%) |
| 35 | Curve Point 11 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (9.910%) |
| 36 | Curve Point 12 | Drawn Curve=1, All Bands Drive=1 | ok (2.9 dB) | ok (10.388%) |
| 37 | Curve Point 13 | Drawn Curve=1, All Bands Drive=1 | ok (2.1 dB) | ok (9.969%) |
| 38 | Curve Point 14 | Drawn Curve=1, All Bands Drive=1 | ok (-1.1 dB) | ok (10.420%) |
| 39 | Curve Point 15 | Drawn Curve=1, All Bands Drive=1 | ok (-0.6 dB) | ok (10.635%) |
| 40 | Curve Point 16 | Drawn Curve=1, All Bands Drive=1 | ok (3.2 dB) | ok (10.026%) |
| 41 | All Bands Drive |  | ok (-1.4 dB) | ok (9.948%) |
| 42 | All Bands Ceiling |  | ok (-3.2 dB) | ok (3.667%) |
| 43 | All Bands Gain |  | ok (-1.7 dB) | ok (7.640%) |
| 44 | Band 1 Drive | Bands Open=1 | ok (-0.9 dB) | ok (6.687%) |
| 45 | Band 1 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-4.5 dB) | ok (5.799%) |
| 46 | Band 1 Gain | Bands Open=1 | ok (1.0 dB) | ok (6.207%) |
| 47 | Band 1 Solo |  | ok (-5.0 dB) | ok (3.499%) |
| 48 | Band 1 Mute |  | ok (-11.8 dB) | ok (2.626%) |
| 49 | Band 1 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (9.050%) |
| 50 | Band 2 Drive | Bands Open=1 | ok (-1.0 dB) | ok (7.595%) |
| 51 | Band 2 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-2.2 dB) | ok (5.738%) |
| 52 | Band 2 Gain | Bands Open=1 | ok (1.1 dB) | ok (6.764%) |
| 53 | Band 2 Solo |  | ok (-0.6 dB) | ok (3.552%) |
| 54 | Band 2 Mute |  | ok (-9.7 dB) | ok (3.071%) |
| 55 | Band 2 Bypass | All Bands Drive=1 | ok (-0.4 dB) | ok (9.282%) |
| 56 | Band 3 Drive | Bands Open=1 | ok (-0.9 dB) | ok (7.648%) |
| 57 | Band 3 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-2.3 dB) | ok (5.901%) |
| 58 | Band 3 Gain | Bands Open=1 | ok (1.0 dB) | ok (6.452%) |
| 59 | Band 3 Solo |  | ok (-1.6 dB) | ok (3.617%) |
| 60 | Band 3 Mute |  | ok (-10.8 dB) | ok (3.092%) |
| 61 | Band 3 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (8.822%) |
| 62 | Band 4 Drive | Bands Open=1 | ok (1.4 dB) | ok (7.721%) |
| 63 | Band 4 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-3.0 dB) | ok (5.550%) |
| 64 | Band 4 Gain | Bands Open=1 | ok (0.1 dB) | ok (7.009%) |
| 65 | Band 4 Solo |  | ok (-3.5 dB) | ok (3.279%) |
| 66 | Band 4 Mute |  | ok (-8.4 dB) | ok (3.193%) |
| 67 | Band 4 Bypass | All Bands Drive=1 | ok (1.8 dB) | ok (7.950%) |
| 68 | Band 5 Drive | Bands=0.8, Bands Open=1 | ok (-0.6 dB) | ok (9.085%) |
| 69 | Band 5 Ceiling | Bands=0.8, Bands Open=1, All Bands Ceiling=0.5 | ok (-3.3 dB) | ok (7.566%) |
| 70 | Band 5 Gain | Bands=0.8, Bands Open=1 | ok (-0.2 dB) | ok (8.876%) |
| 71 | Band 5 Solo | Bands=0.8, Bands Open=0 | ok (-3.7 dB) | ok (5.604%) |
| 72 | Band 5 Mute | Bands=0.8, Bands Open=0 | ok (-11.9 dB) | ok (5.461%) |
| 73 | Band 5 Bypass | Bands=0.8, All Bands Drive=1 | ok (1.0 dB) | ok (10.746%) |
| 74 | Band 6 Drive | Bands=1, Bands Open=1 | ok (-0.9 dB) | ok (8.590%) |
| 75 | Band 6 Ceiling | Bands=1, Bands Open=1, All Bands Ceiling=0.5 | ok (-4.4 dB) | ok (7.963%) |
| 76 | Band 6 Gain | Bands=1, Bands Open=1 | ok (-0.3 dB) | ok (8.568%) |
| 77 | Band 6 Solo | Bands=1, Bands Open=0 | ok (-3.0 dB) | ok (5.733%) |
| 78 | Band 6 Mute | Bands=1, Bands Open=0 | ok (-12.1 dB) | ok (4.917%) |
| 79 | Band 6 Bypass | Bands=1, All Bands Drive=1 | ok (0.0 dB) | ok (10.940%) |
| 80 | Bypass |  | ok (-1.7 dB) | excused (3.191%) |
| 81 | Bypass |  | ok (-1.7 dB) | excused (1.660%) |
| 82 | Wet |  | ok (-1.7 dB) | excused (3.124%) |
| 83 | Delta |  | ok (-3.5 dB) | excused (2.659%) |
