# Plugin QA: AU: OpenClip Multiband (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | AU: OpenClip Multiband (OpenGrid), 83 parameters |
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
| 0 | Input Gain |  | ok (-1.4 dB) | ok (4.303%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (3.286%) |
| 2 | Zero Latency |  | ok (-0.2 dB) | ok (1.790%) |
| 3 | Auto Gain |  | ok (-33.3 dB) | ok (2.539%) |
| 4 | Delta |  | ok (-4.0 dB) | ok (2.334%) |
| 5 | Bands |  | ok (-1.7 dB) | ok (2.941%) |
| 6 | Crossover Slope |  | ok (1.4 dB) | ok (2.238%) |
| 7 | Crossover 1 |  | ok (-1.2 dB) | ok (2.490%) |
| 8 | Crossover 2 |  | ok (-2.7 dB) | ok (2.295%) |
| 9 | Crossover 3 |  | ok (-1.5 dB) | ok (2.338%) |
| 10 | Crossover 4 | Bands=0.8 | ok (-2.2 dB) | ok (3.691%) |
| 11 | Crossover 5 | Bands=1 | ok (-1.8 dB) | ok (2.997%) |
| 12 | Algorithm | All Bands Drive=1 | ok (0.4 dB) | ok (4.054%) |
| 13 | Shape | All Bands Drive=1 | ok (-7.5 dB) | ok (3.944%) |
| 14 | Character | All Bands Drive=1 | ok (-25.7 dB) | ok (3.675%) |
| 15 | Single/Multi Blend | All Bands Drive=1 | ok (-1.4 dB) | ok (3.184%) |
| 16 | Dry/Wet Mix | All Bands Drive=1 | ok (-1.4 dB) | ok (3.649%) |
| 17 | Single Drive | Single/Multi Blend=0 | ok (-1.9 dB) | ok (2.654%) |
| 18 | Single Ceiling | Single/Multi Blend=0 | ok (-5.2 dB) | ok (1.417%) |
| 19 | Output Ceiling On | Input Gain=1 | ok (-1.6 dB) | ok (4.662%) |
| 20 | Output Ceiling | Input Gain=1 | ok (-1.5 dB) | ok (4.264%) |
| 21 | Anti-Alias | All Bands Drive=1 | ok (3.5 dB) | ok (3.770%) |
| 22 | Oversampling | All Bands Drive=1 | ok (4.1 dB) | ok (0.312%) |
| 23 | Bands Open |  | excused (-240.0 dB) | ok (4.124%) |
| 24 | Drawn Curve | All Bands Drive=1 | ok (-21.7 dB) | ok (3.081%) |
| 25 | Curve Point 1 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (3.257%) |
| 26 | Curve Point 2 | Drawn Curve=1, All Bands Drive=1 | ok (0.1 dB) | ok (4.492%) |
| 27 | Curve Point 3 | Drawn Curve=1, All Bands Drive=1 | ok (-2.4 dB) | ok (4.037%) |
| 28 | Curve Point 4 | Drawn Curve=1, All Bands Drive=1 | ok (-2.3 dB) | ok (3.564%) |
| 29 | Curve Point 5 | Drawn Curve=1, All Bands Drive=1 | ok (-2.8 dB) | ok (3.374%) |
| 30 | Curve Point 6 | Drawn Curve=1, All Bands Drive=1 | ok (-3.0 dB) | ok (3.380%) |
| 31 | Curve Point 7 | Drawn Curve=1, All Bands Drive=1 | ok (-2.1 dB) | ok (3.358%) |
| 32 | Curve Point 8 | Drawn Curve=1, All Bands Drive=1 | ok (-0.5 dB) | ok (3.288%) |
| 33 | Curve Point 9 | Drawn Curve=1, All Bands Drive=1 | ok (0.6 dB) | ok (3.498%) |
| 34 | Curve Point 10 | Drawn Curve=1, All Bands Drive=1 | ok (2.0 dB) | ok (2.938%) |
| 35 | Curve Point 11 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (3.741%) |
| 36 | Curve Point 12 | Drawn Curve=1, All Bands Drive=1 | ok (2.9 dB) | ok (4.423%) |
| 37 | Curve Point 13 | Drawn Curve=1, All Bands Drive=1 | ok (2.1 dB) | ok (2.922%) |
| 38 | Curve Point 14 | Drawn Curve=1, All Bands Drive=1 | ok (-1.1 dB) | ok (3.598%) |
| 39 | Curve Point 15 | Drawn Curve=1, All Bands Drive=1 | ok (-0.6 dB) | ok (2.976%) |
| 40 | Curve Point 16 | Drawn Curve=1, All Bands Drive=1 | ok (3.2 dB) | ok (4.008%) |
| 41 | All Bands Drive |  | ok (-1.4 dB) | ok (3.121%) |
| 42 | All Bands Ceiling |  | ok (-8.6 dB) | ok (2.190%) |
| 43 | All Bands Gain |  | ok (-1.7 dB) | ok (2.674%) |
| 44 | Band 1 Drive | Bands Open=1 | ok (-0.9 dB) | ok (4.784%) |
| 45 | Band 1 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-13.3 dB) | ok (4.507%) |
| 46 | Band 1 Gain | Bands Open=1 | ok (1.0 dB) | ok (3.910%) |
| 47 | Band 1 Solo |  | ok (-5.7 dB) | ok (1.911%) |
| 48 | Band 1 Mute |  | ok (-11.8 dB) | ok (1.884%) |
| 49 | Band 1 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (3.309%) |
| 50 | Band 2 Drive | Bands Open=1 | ok (-0.8 dB) | ok (9.180%) |
| 51 | Band 2 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-12.8 dB) | ok (4.294%) |
| 52 | Band 2 Gain | Bands Open=1 | ok (1.1 dB) | ok (4.149%) |
| 53 | Band 2 Solo |  | ok (-4.9 dB) | ok (1.946%) |
| 54 | Band 2 Mute |  | ok (-9.7 dB) | ok (2.064%) |
| 55 | Band 2 Bypass | All Bands Drive=1 | ok (-1.0 dB) | ok (11.356%) |
| 56 | Band 3 Drive | Bands Open=1 | ok (-0.9 dB) | ok (4.406%) |
| 57 | Band 3 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-12.9 dB) | ok (4.363%) |
| 58 | Band 3 Gain | Bands Open=1 | ok (1.0 dB) | ok (4.101%) |
| 59 | Band 3 Solo |  | ok (-4.5 dB) | ok (1.644%) |
| 60 | Band 3 Mute |  | ok (-10.8 dB) | ok (1.855%) |
| 61 | Band 3 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (2.905%) |
| 62 | Band 4 Drive | Bands Open=1 | ok (-0.5 dB) | ok (4.408%) |
| 63 | Band 4 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-10.7 dB) | ok (3.911%) |
| 64 | Band 4 Gain | Bands Open=1 | ok (0.1 dB) | ok (4.123%) |
| 65 | Band 4 Solo |  | ok (-6.6 dB) | ok (1.886%) |
| 66 | Band 4 Mute |  | ok (-8.4 dB) | ok (1.676%) |
| 67 | Band 4 Bypass | All Bands Drive=1 | ok (1.8 dB) | ok (3.106%) |
| 68 | Band 5 Drive | Bands=0.8, Bands Open=1 | ok (-0.6 dB) | ok (5.801%) |
| 69 | Band 5 Ceiling | Bands=0.8, Bands Open=1, All Bands Ceiling=0.5 | ok (-13.0 dB) | ok (5.657%) |
| 70 | Band 5 Gain | Bands=0.8, Bands Open=1 | ok (-0.2 dB) | ok (5.857%) |
| 71 | Band 5 Solo | Bands=0.8, Bands Open=0 | ok (-5.7 dB) | ok (3.427%) |
| 72 | Band 5 Mute | Bands=0.8, Bands Open=0 | ok (-11.9 dB) | ok (3.574%) |
| 73 | Band 5 Bypass | Bands=0.8, All Bands Drive=1 | ok (1.0 dB) | ok (4.017%) |
| 74 | Band 6 Drive | Bands=1, Bands Open=1 | ok (-1.2 dB) | ok (5.905%) |
| 75 | Band 6 Ceiling | Bands=1, Bands Open=1, All Bands Ceiling=0.5 | ok (-13.1 dB) | ok (5.706%) |
| 76 | Band 6 Gain | Bands=1, Bands Open=1 | ok (-0.3 dB) | ok (6.138%) |
| 77 | Band 6 Solo | Bands=1, Bands Open=0 | ok (-4.9 dB) | ok (3.558%) |
| 78 | Band 6 Mute | Bands=1, Bands Open=0 | ok (-12.1 dB) | ok (3.562%) |
| 79 | Band 6 Bypass | Bands=1, All Bands Drive=1 | ok (0.0 dB) | ok (12.198%) |
| 80 | Bypass |  | ok (-1.7 dB) | ok (2.182%) |
| 81 | Wet |  | ok (-1.7 dB) | ok (2.122%) |
| 82 | Delta |  | ok (-3.5 dB) | ok (2.064%) |
