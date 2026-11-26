# Plugin QA: AU: OpenClip Multiband (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | AU: OpenClip Multiband (OpenGrid), 83 parameters |
| finite output | PASS | peak -4.1 dBFS |
| not silent | PASS | rms -14.9 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1080x821, 38.8% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-1.4 dB) | ok (5.574%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (2.955%) |
| 2 | Zero Latency |  | ok (-0.2 dB) | ok (1.726%) |
| 3 | Auto Gain |  | ok (0.9 dB) | ok (1.782%) |
| 4 | Delta |  | ok (-4.0 dB) | ok (1.358%) |
| 5 | Bands |  | ok (-1.7 dB) | ok (1.945%) |
| 6 | Crossover Slope |  | ok (1.4 dB) | ok (1.892%) |
| 7 | Crossover 1 |  | ok (-1.2 dB) | ok (2.071%) |
| 8 | Crossover 2 |  | ok (-1.4 dB) | ok (2.042%) |
| 9 | Crossover 3 |  | ok (-1.5 dB) | ok (1.954%) |
| 10 | Crossover 4 | Bands=0.8 | ok (-2.2 dB) | ok (2.699%) |
| 11 | Crossover 5 | Bands=1 | ok (-1.9 dB) | ok (2.671%) |
| 12 | Algorithm | All Bands Drive=1 | ok (3.8 dB) | ok (7.445%) |
| 13 | Shape | All Bands Drive=1 | ok (-5.3 dB) | ok (7.362%) |
| 14 | Character | All Bands Drive=1 | ok (-13.8 dB) | ok (7.419%) |
| 15 | Single/Multi Blend | All Bands Drive=1 | ok (-1.4 dB) | ok (7.082%) |
| 16 | Dry/Wet Mix | All Bands Drive=1 | ok (-1.4 dB) | ok (7.408%) |
| 17 | Single Drive | Single/Multi Blend=0 | ok (-1.9 dB) | ok (6.334%) |
| 18 | Single Ceiling | Single/Multi Blend=0 | ok (-3.5 dB) | ok (1.723%) |
| 19 | Output Ceiling On | Input Gain=1 | ok (2.3 dB) | ok (6.055%) |
| 20 | Output Ceiling | Input Gain=1 | ok (-1.1 dB) | ok (5.427%) |
| 21 | Anti-Alias | All Bands Drive=1 | ok (3.7 dB) | ok (2.676%) |
| 22 | Oversampling | All Bands Drive=1 | ok (4.0 dB) | ok (7.318%) |
| 23 | Bands Open |  | excused (-3.7 dB) | ok (3.306%) |
| 24 | Drawn Curve | All Bands Drive=1 | ok (-2.6 dB) | ok (7.650%) |
| 25 | Curve Point 1 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (8.048%) |
| 26 | Curve Point 2 | Drawn Curve=1, All Bands Drive=1 | ok (0.1 dB) | ok (7.768%) |
| 27 | Curve Point 3 | Drawn Curve=1, All Bands Drive=1 | ok (-2.4 dB) | ok (7.734%) |
| 28 | Curve Point 4 | Drawn Curve=1, All Bands Drive=1 | ok (-2.3 dB) | ok (7.646%) |
| 29 | Curve Point 5 | Drawn Curve=1, All Bands Drive=1 | ok (-2.8 dB) | ok (7.804%) |
| 30 | Curve Point 6 | Drawn Curve=1, All Bands Drive=1 | ok (-3.0 dB) | ok (7.520%) |
| 31 | Curve Point 7 | Drawn Curve=1, All Bands Drive=1 | ok (-2.1 dB) | ok (7.882%) |
| 32 | Curve Point 8 | Drawn Curve=1, All Bands Drive=1 | ok (-0.5 dB) | ok (7.818%) |
| 33 | Curve Point 9 | Drawn Curve=1, All Bands Drive=1 | ok (0.6 dB) | ok (7.741%) |
| 34 | Curve Point 10 | Drawn Curve=1, All Bands Drive=1 | ok (2.0 dB) | ok (7.720%) |
| 35 | Curve Point 11 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (7.629%) |
| 36 | Curve Point 12 | Drawn Curve=1, All Bands Drive=1 | ok (2.9 dB) | ok (7.444%) |
| 37 | Curve Point 13 | Drawn Curve=1, All Bands Drive=1 | ok (2.1 dB) | ok (7.777%) |
| 38 | Curve Point 14 | Drawn Curve=1, All Bands Drive=1 | ok (-1.1 dB) | ok (7.892%) |
| 39 | Curve Point 15 | Drawn Curve=1, All Bands Drive=1 | ok (-0.6 dB) | ok (7.714%) |
| 40 | Curve Point 16 | Drawn Curve=1, All Bands Drive=1 | ok (3.2 dB) | ok (7.480%) |
| 41 | All Bands Drive |  | ok (-1.4 dB) | ok (7.737%) |
| 42 | All Bands Ceiling |  | ok (-3.2 dB) | ok (1.954%) |
| 43 | All Bands Gain |  | ok (-1.7 dB) | ok (6.006%) |
| 44 | Band 1 Drive | Bands Open=1 | ok (-0.9 dB) | ok (4.646%) |
| 45 | Band 1 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-13.3 dB) | ok (3.265%) |
| 46 | Band 1 Gain | Bands Open=1 | ok (1.0 dB) | ok (3.940%) |
| 47 | Band 1 Solo |  | ok (-5.7 dB) | ok (2.247%) |
| 48 | Band 1 Mute |  | ok (-11.8 dB) | ok (2.077%) |
| 49 | Band 1 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (6.731%) |
| 50 | Band 2 Drive | Bands Open=1 | ok (-1.0 dB) | ok (4.885%) |
| 51 | Band 2 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-12.8 dB) | ok (3.427%) |
| 52 | Band 2 Gain | Bands Open=1 | ok (1.1 dB) | ok (4.519%) |
| 53 | Band 2 Solo |  | ok (-4.9 dB) | ok (2.526%) |
| 54 | Band 2 Mute |  | ok (-9.7 dB) | ok (2.024%) |
| 55 | Band 2 Bypass | All Bands Drive=1 | ok (-0.4 dB) | ok (7.118%) |
| 56 | Band 3 Drive | Bands Open=1 | ok (-0.9 dB) | ok (5.114%) |
| 57 | Band 3 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-12.9 dB) | ok (3.557%) |
| 58 | Band 3 Gain | Bands Open=1 | ok (1.0 dB) | ok (4.455%) |
| 59 | Band 3 Solo |  | ok (-4.5 dB) | ok (2.466%) |
| 60 | Band 3 Mute |  | ok (-10.8 dB) | ok (1.945%) |
| 61 | Band 3 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (7.070%) |
| 62 | Band 4 Drive | Bands Open=1 | ok (1.4 dB) | ok (5.701%) |
| 63 | Band 4 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-10.7 dB) | ok (3.078%) |
| 64 | Band 4 Gain | Bands Open=1 | ok (0.1 dB) | ok (4.808%) |
| 65 | Band 4 Solo |  | ok (-6.6 dB) | ok (2.072%) |
| 66 | Band 4 Mute |  | ok (-8.4 dB) | ok (2.388%) |
| 67 | Band 4 Bypass | All Bands Drive=1 | ok (1.8 dB) | ok (6.595%) |
| 68 | Band 5 Drive | Bands=0.8, Bands Open=1 | ok (-0.6 dB) | ok (5.576%) |
| 69 | Band 5 Ceiling | Bands=0.8, Bands Open=1, All Bands Ceiling=0.5 | ok (-13.0 dB) | ok (4.192%) |
| 70 | Band 5 Gain | Bands=0.8, Bands Open=1 | ok (-0.2 dB) | ok (5.606%) |
| 71 | Band 5 Solo | Bands=0.8, Bands Open=0 | ok (-5.7 dB) | ok (3.012%) |
| 72 | Band 5 Mute | Bands=0.8, Bands Open=0 | ok (-11.9 dB) | ok (2.933%) |
| 73 | Band 5 Bypass | Bands=0.8, All Bands Drive=1 | ok (1.0 dB) | ok (7.422%) |
| 74 | Band 6 Drive | Bands=1, Bands Open=1 | ok (-0.9 dB) | ok (5.117%) |
| 75 | Band 6 Ceiling | Bands=1, Bands Open=1, All Bands Ceiling=0.5 | ok (-13.1 dB) | ok (4.098%) |
| 76 | Band 6 Gain | Bands=1, Bands Open=1 | ok (-0.3 dB) | ok (4.872%) |
| 77 | Band 6 Solo | Bands=1, Bands Open=0 | ok (-4.9 dB) | ok (3.130%) |
| 78 | Band 6 Mute | Bands=1, Bands Open=0 | ok (-12.1 dB) | ok (2.737%) |
| 79 | Band 6 Bypass | Bands=1, All Bands Drive=1 | ok (0.0 dB) | ok (7.592%) |
| 80 | Bypass |  | ok (-1.7 dB) | ok (2.043%) |
| 81 | Wet |  | ok (-1.7 dB) | ok (1.509%) |
| 82 | Delta |  | ok (-3.5 dB) | ok (1.967%) |
