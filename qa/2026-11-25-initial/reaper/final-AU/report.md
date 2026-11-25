# Plugin QA: AU: OpenClip Multiband (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | AU: OpenClip Multiband (OpenGrid), 83 parameters |
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
| 0 | Input Gain |  | ok (-1.4 dB) | ok (8.471%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (4.844%) |
| 2 | Zero Latency |  | ok (-0.2 dB) | ok (3.375%) |
| 3 | Auto Gain |  | ok (0.9 dB) | ok (3.263%) |
| 4 | Delta |  | ok (-4.0 dB) | ok (2.275%) |
| 5 | Bands |  | ok (-1.7 dB) | ok (4.435%) |
| 6 | Crossover Slope |  | ok (1.4 dB) | ok (2.596%) |
| 7 | Crossover 1 |  | ok (-1.2 dB) | ok (3.548%) |
| 8 | Crossover 2 |  | ok (-1.4 dB) | ok (3.451%) |
| 9 | Crossover 3 |  | ok (-1.5 dB) | ok (3.538%) |
| 10 | Crossover 4 | Bands=0.8 | ok (-2.2 dB) | ok (5.100%) |
| 11 | Crossover 5 | Bands=1 | ok (-1.9 dB) | ok (5.158%) |
| 12 | Algorithm | All Bands Drive=1 | ok (2.7 dB) | ok (8.862%) |
| 13 | Shape | All Bands Drive=1 | ok (-7.5 dB) | ok (9.511%) |
| 14 | Character | All Bands Drive=1 | ok (-13.8 dB) | ok (10.147%) |
| 15 | Single/Multi Blend | All Bands Drive=1 | ok (-1.4 dB) | ok (10.041%) |
| 16 | Dry/Wet Mix | All Bands Drive=1 | ok (-1.4 dB) | ok (9.535%) |
| 17 | Single Drive | Single/Multi Blend=0 | ok (-1.9 dB) | ok (7.874%) |
| 18 | Single Ceiling | Single/Multi Blend=0 | ok (-3.5 dB) | ok (3.102%) |
| 19 | Output Ceiling On | Input Gain=1 | ok (2.3 dB) | ok (9.174%) |
| 20 | Output Ceiling | Input Gain=1 | ok (-1.1 dB) | ok (8.730%) |
| 21 | Anti-Alias | All Bands Drive=1 | ok (3.7 dB) | ok (4.743%) |
| 22 | Oversampling | All Bands Drive=1 | ok (4.0 dB) | ok (9.512%) |
| 23 | Bands Open |  | excused (-3.7 dB) | ok (4.899%) |
| 24 | Drawn Curve | All Bands Drive=1 | ok (-2.6 dB) | ok (10.105%) |
| 25 | Curve Point 1 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (10.807%) |
| 26 | Curve Point 2 | Drawn Curve=1, All Bands Drive=1 | ok (0.1 dB) | ok (10.837%) |
| 27 | Curve Point 3 | Drawn Curve=1, All Bands Drive=1 | ok (-2.4 dB) | ok (10.749%) |
| 28 | Curve Point 4 | Drawn Curve=1, All Bands Drive=1 | ok (-2.3 dB) | ok (9.966%) |
| 29 | Curve Point 5 | Drawn Curve=1, All Bands Drive=1 | ok (-2.8 dB) | ok (10.275%) |
| 30 | Curve Point 6 | Drawn Curve=1, All Bands Drive=1 | ok (-3.0 dB) | ok (10.492%) |
| 31 | Curve Point 7 | Drawn Curve=1, All Bands Drive=1 | ok (-2.1 dB) | ok (10.389%) |
| 32 | Curve Point 8 | Drawn Curve=1, All Bands Drive=1 | ok (-0.5 dB) | ok (10.185%) |
| 33 | Curve Point 9 | Drawn Curve=1, All Bands Drive=1 | ok (0.6 dB) | ok (9.786%) |
| 34 | Curve Point 10 | Drawn Curve=1, All Bands Drive=1 | ok (2.0 dB) | ok (9.782%) |
| 35 | Curve Point 11 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (9.880%) |
| 36 | Curve Point 12 | Drawn Curve=1, All Bands Drive=1 | ok (2.9 dB) | ok (9.879%) |
| 37 | Curve Point 13 | Drawn Curve=1, All Bands Drive=1 | ok (2.1 dB) | ok (10.504%) |
| 38 | Curve Point 14 | Drawn Curve=1, All Bands Drive=1 | ok (-1.1 dB) | ok (10.974%) |
| 39 | Curve Point 15 | Drawn Curve=1, All Bands Drive=1 | ok (-0.6 dB) | ok (9.787%) |
| 40 | Curve Point 16 | Drawn Curve=1, All Bands Drive=1 | ok (3.2 dB) | ok (10.558%) |
| 41 | All Bands Drive |  | ok (-1.4 dB) | ok (10.401%) |
| 42 | All Bands Ceiling |  | ok (-3.2 dB) | ok (3.312%) |
| 43 | All Bands Gain |  | ok (-1.7 dB) | ok (7.552%) |
| 44 | Band 1 Drive | Bands Open=1 | ok (-0.9 dB) | ok (6.817%) |
| 45 | Band 1 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-13.3 dB) | ok (6.028%) |
| 46 | Band 1 Gain | Bands Open=1 | ok (1.0 dB) | ok (6.518%) |
| 47 | Band 1 Solo |  | ok (-5.7 dB) | ok (3.104%) |
| 48 | Band 1 Mute |  | ok (-11.8 dB) | ok (2.276%) |
| 49 | Band 1 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (9.089%) |
| 50 | Band 2 Drive | Bands Open=1 | ok (-1.0 dB) | ok (7.494%) |
| 51 | Band 2 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-12.8 dB) | ok (5.193%) |
| 52 | Band 2 Gain | Bands Open=1 | ok (1.1 dB) | ok (7.386%) |
| 53 | Band 2 Solo |  | ok (-4.9 dB) | ok (3.139%) |
| 54 | Band 2 Mute |  | ok (-9.7 dB) | ok (3.085%) |
| 55 | Band 2 Bypass | All Bands Drive=1 | ok (-0.4 dB) | ok (9.566%) |
| 56 | Band 3 Drive | Bands Open=1 | ok (-0.9 dB) | ok (7.461%) |
| 57 | Band 3 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-12.9 dB) | ok (5.933%) |
| 58 | Band 3 Gain | Bands Open=1 | ok (1.0 dB) | ok (6.688%) |
| 59 | Band 3 Solo |  | ok (-4.5 dB) | ok (3.759%) |
| 60 | Band 3 Mute |  | ok (-10.8 dB) | ok (2.548%) |
| 61 | Band 3 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (9.420%) |
| 62 | Band 4 Drive | Bands Open=1 | ok (1.4 dB) | ok (7.818%) |
| 63 | Band 4 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-10.7 dB) | ok (5.272%) |
| 64 | Band 4 Gain | Bands Open=1 | ok (0.1 dB) | ok (7.108%) |
| 65 | Band 4 Solo |  | ok (-6.6 dB) | ok (3.353%) |
| 66 | Band 4 Mute |  | ok (-8.4 dB) | ok (3.287%) |
| 67 | Band 4 Bypass | All Bands Drive=1 | ok (1.8 dB) | ok (8.973%) |
| 68 | Band 5 Drive | Bands=0.8, Bands Open=1 | ok (-0.6 dB) | ok (9.717%) |
| 69 | Band 5 Ceiling | Bands=0.8, Bands Open=1, All Bands Ceiling=0.5 | ok (-13.0 dB) | ok (7.919%) |
| 70 | Band 5 Gain | Bands=0.8, Bands Open=1 | ok (-0.2 dB) | ok (9.565%) |
| 71 | Band 5 Solo | Bands=0.8, Bands Open=0 | ok (-5.7 dB) | ok (5.683%) |
| 72 | Band 5 Mute | Bands=0.8, Bands Open=0 | ok (-11.9 dB) | ok (5.308%) |
| 73 | Band 5 Bypass | Bands=0.8, All Bands Drive=1 | ok (1.0 dB) | ok (11.184%) |
| 74 | Band 6 Drive | Bands=1, Bands Open=1 | ok (-0.9 dB) | ok (8.467%) |
| 75 | Band 6 Ceiling | Bands=1, Bands Open=1, All Bands Ceiling=0.5 | ok (-13.1 dB) | ok (7.882%) |
| 76 | Band 6 Gain | Bands=1, Bands Open=1 | ok (-0.3 dB) | ok (8.263%) |
| 77 | Band 6 Solo | Bands=1, Bands Open=0 | ok (-4.9 dB) | ok (5.658%) |
| 78 | Band 6 Mute | Bands=1, Bands Open=0 | ok (-12.1 dB) | ok (4.875%) |
| 79 | Band 6 Bypass | Bands=1, All Bands Drive=1 | ok (0.0 dB) | ok (11.013%) |
| 80 | Bypass |  | ok (-1.7 dB) | ok (3.253%) |
| 81 | Wet |  | ok (-1.7 dB) | ok (2.919%) |
| 82 | Delta |  | ok (-3.5 dB) | ok (3.017%) |
