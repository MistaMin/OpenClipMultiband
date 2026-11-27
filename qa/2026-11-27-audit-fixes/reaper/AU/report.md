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
| 0 | Input Gain |  | ok (-1.4 dB) | ok (7.816%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (5.177%) |
| 2 | Zero Latency |  | ok (-0.2 dB) | ok (3.729%) |
| 3 | Auto Gain |  | ok (0.9 dB) | ok (3.218%) |
| 4 | Delta |  | ok (-4.0 dB) | ok (2.988%) |
| 5 | Bands |  | ok (-1.7 dB) | ok (4.526%) |
| 6 | Crossover Slope |  | ok (1.4 dB) | ok (3.103%) |
| 7 | Crossover 1 |  | ok (-1.2 dB) | ok (3.553%) |
| 8 | Crossover 2 |  | ok (-1.4 dB) | ok (3.379%) |
| 9 | Crossover 3 |  | ok (-1.5 dB) | ok (3.451%) |
| 10 | Crossover 4 | Bands=0.8 | ok (-2.2 dB) | ok (5.276%) |
| 11 | Crossover 5 | Bands=1 | ok (-1.9 dB) | ok (5.023%) |
| 12 | Algorithm | All Bands Drive=1 | ok (3.8 dB) | ok (9.960%) |
| 13 | Shape | All Bands Drive=1 | ok (-5.3 dB) | ok (10.034%) |
| 14 | Character | All Bands Drive=1 | ok (-13.8 dB) | ok (9.541%) |
| 15 | Single/Multi Blend | All Bands Drive=1 | ok (-1.4 dB) | ok (9.156%) |
| 16 | Dry/Wet Mix | All Bands Drive=1 | ok (-1.4 dB) | ok (10.132%) |
| 17 | Single Drive | Single/Multi Blend=0 | ok (-1.9 dB) | ok (7.673%) |
| 18 | Single Ceiling | Single/Multi Blend=0 | ok (-3.5 dB) | ok (2.690%) |
| 19 | Output Ceiling On | Input Gain=1 | ok (2.3 dB) | ok (8.114%) |
| 20 | Output Ceiling | Input Gain=1 | ok (-1.1 dB) | ok (7.275%) |
| 21 | Anti-Alias | All Bands Drive=1 | ok (3.6 dB) | ok (4.938%) |
| 22 | Oversampling | All Bands Drive=1 | ok (4.0 dB) | ok (9.630%) |
| 23 | Bands Open |  | excused (-3.7 dB) | ok (5.391%) |
| 24 | Drawn Curve | All Bands Drive=1 | ok (-2.6 dB) | ok (9.863%) |
| 25 | Curve Point 1 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (10.858%) |
| 26 | Curve Point 2 | Drawn Curve=1, All Bands Drive=1 | ok (0.1 dB) | ok (10.186%) |
| 27 | Curve Point 3 | Drawn Curve=1, All Bands Drive=1 | ok (-2.4 dB) | ok (10.209%) |
| 28 | Curve Point 4 | Drawn Curve=1, All Bands Drive=1 | ok (-2.3 dB) | ok (10.105%) |
| 29 | Curve Point 5 | Drawn Curve=1, All Bands Drive=1 | ok (-2.8 dB) | ok (9.829%) |
| 30 | Curve Point 6 | Drawn Curve=1, All Bands Drive=1 | ok (-3.0 dB) | ok (10.334%) |
| 31 | Curve Point 7 | Drawn Curve=1, All Bands Drive=1 | ok (-2.1 dB) | ok (10.349%) |
| 32 | Curve Point 8 | Drawn Curve=1, All Bands Drive=1 | ok (-0.5 dB) | ok (9.935%) |
| 33 | Curve Point 9 | Drawn Curve=1, All Bands Drive=1 | ok (0.6 dB) | ok (9.737%) |
| 34 | Curve Point 10 | Drawn Curve=1, All Bands Drive=1 | ok (2.0 dB) | ok (9.912%) |
| 35 | Curve Point 11 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (10.296%) |
| 36 | Curve Point 12 | Drawn Curve=1, All Bands Drive=1 | ok (2.9 dB) | ok (9.578%) |
| 37 | Curve Point 13 | Drawn Curve=1, All Bands Drive=1 | ok (2.1 dB) | ok (10.134%) |
| 38 | Curve Point 14 | Drawn Curve=1, All Bands Drive=1 | ok (-1.1 dB) | ok (9.978%) |
| 39 | Curve Point 15 | Drawn Curve=1, All Bands Drive=1 | ok (-0.6 dB) | ok (9.885%) |
| 40 | Curve Point 16 | Drawn Curve=1, All Bands Drive=1 | ok (3.2 dB) | ok (9.440%) |
| 41 | All Bands Drive |  | ok (-1.4 dB) | ok (10.274%) |
| 42 | All Bands Ceiling |  | ok (-3.2 dB) | ok (3.509%) |
| 43 | All Bands Gain |  | ok (-1.7 dB) | ok (7.524%) |
| 44 | Band 1 Drive | Bands Open=1 | ok (-0.9 dB) | ok (7.020%) |
| 45 | Band 1 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-13.3 dB) | ok (6.372%) |
| 46 | Band 1 Gain | Bands Open=1 | ok (1.0 dB) | ok (6.215%) |
| 47 | Band 1 Solo |  | ok (-5.7 dB) | ok (3.829%) |
| 48 | Band 1 Mute |  | ok (-11.8 dB) | ok (3.309%) |
| 49 | Band 1 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (9.005%) |
| 50 | Band 2 Drive | Bands Open=1 | ok (-1.0 dB) | ok (7.501%) |
| 51 | Band 2 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-12.8 dB) | ok (6.157%) |
| 52 | Band 2 Gain | Bands Open=1 | ok (1.1 dB) | ok (6.776%) |
| 53 | Band 2 Solo |  | ok (-4.9 dB) | ok (3.743%) |
| 54 | Band 2 Mute |  | ok (-9.7 dB) | ok (2.583%) |
| 55 | Band 2 Bypass | All Bands Drive=1 | ok (-0.4 dB) | ok (9.440%) |
| 56 | Band 3 Drive | Bands Open=1 | ok (-0.9 dB) | ok (7.844%) |
| 57 | Band 3 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-12.9 dB) | ok (6.132%) |
| 58 | Band 3 Gain | Bands Open=1 | ok (1.0 dB) | ok (6.938%) |
| 59 | Band 3 Solo |  | ok (-4.5 dB) | ok (3.761%) |
| 60 | Band 3 Mute |  | ok (-10.8 dB) | ok (2.980%) |
| 61 | Band 3 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (8.955%) |
| 62 | Band 4 Drive | Bands Open=1 | ok (1.4 dB) | ok (8.108%) |
| 63 | Band 4 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-10.7 dB) | ok (5.838%) |
| 64 | Band 4 Gain | Bands Open=1 | ok (0.1 dB) | ok (7.160%) |
| 65 | Band 4 Solo |  | ok (-6.6 dB) | ok (3.495%) |
| 66 | Band 4 Mute |  | ok (-8.4 dB) | ok (3.190%) |
| 67 | Band 4 Bypass | All Bands Drive=1 | ok (1.8 dB) | ok (8.736%) |
| 68 | Band 5 Drive | Bands=0.8, Bands Open=1 | ok (-0.6 dB) | ok (9.398%) |
| 69 | Band 5 Ceiling | Bands=0.8, Bands Open=1, All Bands Ceiling=0.5 | ok (-13.0 dB) | ok (7.871%) |
| 70 | Band 5 Gain | Bands=0.8, Bands Open=1 | ok (-0.2 dB) | ok (9.303%) |
| 71 | Band 5 Solo | Bands=0.8, Bands Open=0 | ok (-5.7 dB) | ok (6.054%) |
| 72 | Band 5 Mute | Bands=0.8, Bands Open=0 | ok (-11.9 dB) | ok (5.243%) |
| 73 | Band 5 Bypass | Bands=0.8, All Bands Drive=1 | ok (1.0 dB) | ok (10.934%) |
| 74 | Band 6 Drive | Bands=1, Bands Open=1 | ok (-0.9 dB) | ok (8.952%) |
| 75 | Band 6 Ceiling | Bands=1, Bands Open=1, All Bands Ceiling=0.5 | ok (-13.1 dB) | ok (8.123%) |
| 76 | Band 6 Gain | Bands=1, Bands Open=1 | ok (-0.3 dB) | ok (8.856%) |
| 77 | Band 6 Solo | Bands=1, Bands Open=0 | ok (-4.9 dB) | ok (6.139%) |
| 78 | Band 6 Mute | Bands=1, Bands Open=0 | ok (-12.1 dB) | ok (5.357%) |
| 79 | Band 6 Bypass | Bands=1, All Bands Drive=1 | ok (0.0 dB) | ok (11.236%) |
| 80 | Bypass |  | ok (-1.7 dB) | ok (2.886%) |
| 81 | Wet |  | ok (-1.7 dB) | ok (2.952%) |
| 82 | Delta |  | ok (-3.5 dB) | ok (2.485%) |
