# Plugin QA: CLAP: OpenClip Multiband

| Check | Status | Detail |
|---|---|---|
| loads | PASS | CLAP: OpenClip Multiband (OpenGrid), 83 parameters |
| finite output | PASS | peak -4.1 dBFS |
| not silent | PASS | rms -14.9 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1080x821, 0.2% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-1.4 dB) | ok (8.952%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (5.347%) |
| 2 | Zero Latency |  | ok (-1.0 dB) | ok (3.097%) |
| 3 | Auto Gain |  | ok (0.9 dB) | ok (3.575%) |
| 4 | Delta |  | ok (-4.0 dB) | ok (3.076%) |
| 5 | Bands |  | ok (-1.7 dB) | ok (5.663%) |
| 6 | Crossover Slope |  | ok (1.4 dB) | ok (3.250%) |
| 7 | Crossover 1 |  | ok (-1.2 dB) | ok (3.707%) |
| 8 | Crossover 2 |  | ok (-1.4 dB) | ok (3.603%) |
| 9 | Crossover 3 |  | ok (-1.5 dB) | ok (2.898%) |
| 10 | Crossover 4 | Bands=0.8 | ok (-2.2 dB) | ok (4.957%) |
| 11 | Crossover 5 | Bands=1 | ok (-1.9 dB) | ok (4.821%) |
| 12 | Algorithm | All Bands Drive=1 | ok (3.8 dB) | ok (10.073%) |
| 13 | Shape | All Bands Drive=1 | ok (-5.3 dB) | ok (10.165%) |
| 14 | Character | All Bands Drive=1 | ok (-25.7 dB) | ok (10.324%) |
| 15 | Single/Multi Blend | All Bands Drive=1 | ok (-1.4 dB) | ok (9.608%) |
| 16 | Dry/Wet Mix | All Bands Drive=1 | ok (-1.4 dB) | ok (9.791%) |
| 17 | Single Drive | Single/Multi Blend=0 | ok (-1.9 dB) | ok (8.537%) |
| 18 | Single Ceiling | Single/Multi Blend=0 | ok (-3.5 dB) | ok (3.052%) |
| 19 | Output Ceiling On | Input Gain=1 | ok (2.5 dB) | ok (9.119%) |
| 20 | Output Ceiling | Input Gain=1 | ok (-1.5 dB) | ok (8.500%) |
| 21 | Anti-Alias | All Bands Drive=1 | ok (3.7 dB) | ok (7.083%) |
| 22 | Oversampling | All Bands Drive=1 | ok (4.9 dB) | ok (9.334%) |
| 23 | Bands Open |  | excused (-1.4 dB) | ok (5.311%) |
| 24 | Drawn Curve | All Bands Drive=1 | ok (-2.6 dB) | ok (10.834%) |
| 25 | Curve Point 1 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (10.850%) |
| 26 | Curve Point 2 | Drawn Curve=1, All Bands Drive=1 | ok (0.1 dB) | ok (10.678%) |
| 27 | Curve Point 3 | Drawn Curve=1, All Bands Drive=1 | ok (-2.4 dB) | ok (10.206%) |
| 28 | Curve Point 4 | Drawn Curve=1, All Bands Drive=1 | ok (-2.3 dB) | ok (9.481%) |
| 29 | Curve Point 5 | Drawn Curve=1, All Bands Drive=1 | ok (-2.8 dB) | ok (9.745%) |
| 30 | Curve Point 6 | Drawn Curve=1, All Bands Drive=1 | ok (-3.0 dB) | ok (9.910%) |
| 31 | Curve Point 7 | Drawn Curve=1, All Bands Drive=1 | ok (-2.1 dB) | ok (9.372%) |
| 32 | Curve Point 8 | Drawn Curve=1, All Bands Drive=1 | ok (-0.5 dB) | ok (9.601%) |
| 33 | Curve Point 9 | Drawn Curve=1, All Bands Drive=1 | ok (0.6 dB) | ok (10.234%) |
| 34 | Curve Point 10 | Drawn Curve=1, All Bands Drive=1 | ok (2.0 dB) | ok (10.509%) |
| 35 | Curve Point 11 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (9.980%) |
| 36 | Curve Point 12 | Drawn Curve=1, All Bands Drive=1 | ok (2.9 dB) | ok (10.394%) |
| 37 | Curve Point 13 | Drawn Curve=1, All Bands Drive=1 | ok (2.1 dB) | ok (10.662%) |
| 38 | Curve Point 14 | Drawn Curve=1, All Bands Drive=1 | ok (-1.1 dB) | ok (10.777%) |
| 39 | Curve Point 15 | Drawn Curve=1, All Bands Drive=1 | ok (-0.6 dB) | ok (10.273%) |
| 40 | Curve Point 16 | Drawn Curve=1, All Bands Drive=1 | ok (3.2 dB) | ok (9.977%) |
| 41 | All Bands Drive |  | ok (-1.4 dB) | ok (9.513%) |
| 42 | All Bands Ceiling |  | ok (-3.2 dB) | ok (3.637%) |
| 43 | All Bands Gain |  | ok (-1.7 dB) | ok (6.570%) |
| 44 | Band 1 Drive | Bands Open=1 | ok (-0.9 dB) | ok (6.021%) |
| 45 | Band 1 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-4.5 dB) | ok (6.095%) |
| 46 | Band 1 Gain | Bands Open=1 | ok (1.0 dB) | ok (6.703%) |
| 47 | Band 1 Solo |  | ok (-5.0 dB) | ok (3.662%) |
| 48 | Band 1 Mute |  | ok (-11.8 dB) | ok (3.170%) |
| 49 | Band 1 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (8.955%) |
| 50 | Band 2 Drive | Bands Open=1 | ok (-1.0 dB) | ok (8.018%) |
| 51 | Band 2 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-2.2 dB) | ok (6.420%) |
| 52 | Band 2 Gain | Bands Open=1 | ok (1.1 dB) | ok (7.548%) |
| 53 | Band 2 Solo |  | ok (-0.6 dB) | ok (3.866%) |
| 54 | Band 2 Mute |  | ok (-9.7 dB) | ok (3.602%) |
| 55 | Band 2 Bypass | All Bands Drive=1 | ok (-0.4 dB) | ok (9.536%) |
| 56 | Band 3 Drive | Bands Open=1 | ok (-0.9 dB) | ok (8.155%) |
| 57 | Band 3 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-2.3 dB) | ok (5.743%) |
| 58 | Band 3 Gain | Bands Open=1 | ok (1.0 dB) | ok (7.148%) |
| 59 | Band 3 Solo |  | ok (-1.6 dB) | ok (3.503%) |
| 60 | Band 3 Mute |  | ok (-10.8 dB) | ok (3.191%) |
| 61 | Band 3 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (9.883%) |
| 62 | Band 4 Drive | Bands Open=1 | ok (1.4 dB) | ok (8.402%) |
| 63 | Band 4 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-3.0 dB) | ok (5.845%) |
| 64 | Band 4 Gain | Bands Open=1 | ok (0.1 dB) | ok (6.407%) |
| 65 | Band 4 Solo |  | ok (-3.5 dB) | ok (2.804%) |
| 66 | Band 4 Mute |  | ok (-8.4 dB) | ok (3.348%) |
| 67 | Band 4 Bypass | All Bands Drive=1 | ok (1.8 dB) | ok (7.948%) |
| 68 | Band 5 Drive | Bands=0.8, Bands Open=1 | ok (-0.6 dB) | ok (9.618%) |
| 69 | Band 5 Ceiling | Bands=0.8, Bands Open=1, All Bands Ceiling=0.5 | ok (-3.3 dB) | ok (7.634%) |
| 70 | Band 5 Gain | Bands=0.8, Bands Open=1 | ok (-0.2 dB) | ok (8.652%) |
| 71 | Band 5 Solo | Bands=0.8, Bands Open=0 | ok (-3.7 dB) | ok (5.909%) |
| 72 | Band 5 Mute | Bands=0.8, Bands Open=0 | ok (-11.9 dB) | ok (5.203%) |
| 73 | Band 5 Bypass | Bands=0.8, All Bands Drive=1 | ok (1.0 dB) | ok (10.617%) |
| 74 | Band 6 Drive | Bands=1, Bands Open=1 | ok (-0.9 dB) | ok (8.675%) |
| 75 | Band 6 Ceiling | Bands=1, Bands Open=1, All Bands Ceiling=0.5 | ok (-4.4 dB) | ok (8.135%) |
| 76 | Band 6 Gain | Bands=1, Bands Open=1 | ok (-0.3 dB) | ok (8.770%) |
| 77 | Band 6 Solo | Bands=1, Bands Open=0 | ok (-3.0 dB) | ok (5.796%) |
| 78 | Band 6 Mute | Bands=1, Bands Open=0 | ok (-12.1 dB) | ok (5.216%) |
| 79 | Band 6 Bypass | Bands=1, All Bands Drive=1 | ok (0.0 dB) | ok (11.689%) |
| 80 | Bypass |  | ok (-1.7 dB) | ok (3.462%) |
| 81 | Wet |  | ok (-1.7 dB) | ok (3.164%) |
| 82 | Delta |  | ok (-3.5 dB) | ok (3.500%) |
