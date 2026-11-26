# Plugin QA: CLAP: OpenClip Multiband

| Check | Status | Detail |
|---|---|---|
| loads | PASS | CLAP: OpenClip Multiband (OpenGrid), 83 parameters |
| finite output | PASS | peak -4.1 dBFS |
| not silent | PASS | rms -14.9 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1080x821, 38.9% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-1.4 dB) | ok (5.725%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (3.758%) |
| 2 | Zero Latency |  | ok (-0.2 dB) | ok (2.814%) |
| 3 | Auto Gain |  | ok (0.9 dB) | ok (2.223%) |
| 4 | Delta |  | ok (-4.0 dB) | ok (1.974%) |
| 5 | Bands |  | ok (-1.7 dB) | ok (2.776%) |
| 6 | Crossover Slope |  | ok (1.4 dB) | ok (2.210%) |
| 7 | Crossover 1 |  | ok (-1.2 dB) | ok (2.292%) |
| 8 | Crossover 2 |  | ok (-1.4 dB) | ok (2.393%) |
| 9 | Crossover 3 |  | ok (-1.5 dB) | ok (2.009%) |
| 10 | Crossover 4 | Bands=0.8 | ok (-2.2 dB) | ok (2.806%) |
| 11 | Crossover 5 | Bands=1 | ok (-1.9 dB) | ok (2.568%) |
| 12 | Algorithm | All Bands Drive=1 | ok (3.8 dB) | ok (7.717%) |
| 13 | Shape | All Bands Drive=1 | ok (-5.3 dB) | ok (7.980%) |
| 14 | Character | All Bands Drive=1 | ok (-25.7 dB) | ok (7.466%) |
| 15 | Single/Multi Blend | All Bands Drive=1 | ok (-1.4 dB) | ok (7.808%) |
| 16 | Dry/Wet Mix | All Bands Drive=1 | ok (-1.4 dB) | ok (7.990%) |
| 17 | Single Drive | Single/Multi Blend=0 | ok (-1.9 dB) | ok (6.693%) |
| 18 | Single Ceiling | Single/Multi Blend=0 | ok (-3.5 dB) | ok (1.912%) |
| 19 | Output Ceiling On | Input Gain=1 | ok (2.5 dB) | ok (5.949%) |
| 20 | Output Ceiling | Input Gain=1 | ok (-1.5 dB) | ok (5.777%) |
| 21 | Anti-Alias | All Bands Drive=1 | ok (3.7 dB) | ok (3.359%) |
| 22 | Oversampling | All Bands Drive=1 | ok (4.9 dB) | ok (6.640%) |
| 23 | Bands Open |  | excused (-3.7 dB) | ok (3.177%) |
| 24 | Drawn Curve | All Bands Drive=1 | ok (-2.6 dB) | ok (7.928%) |
| 25 | Curve Point 1 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (8.560%) |
| 26 | Curve Point 2 | Drawn Curve=1, All Bands Drive=1 | ok (0.1 dB) | ok (8.172%) |
| 27 | Curve Point 3 | Drawn Curve=1, All Bands Drive=1 | ok (-2.4 dB) | ok (7.452%) |
| 28 | Curve Point 4 | Drawn Curve=1, All Bands Drive=1 | ok (-2.3 dB) | ok (7.789%) |
| 29 | Curve Point 5 | Drawn Curve=1, All Bands Drive=1 | ok (-2.8 dB) | ok (7.923%) |
| 30 | Curve Point 6 | Drawn Curve=1, All Bands Drive=1 | ok (-3.0 dB) | ok (7.660%) |
| 31 | Curve Point 7 | Drawn Curve=1, All Bands Drive=1 | ok (-2.1 dB) | ok (7.839%) |
| 32 | Curve Point 8 | Drawn Curve=1, All Bands Drive=1 | ok (-0.5 dB) | ok (8.042%) |
| 33 | Curve Point 9 | Drawn Curve=1, All Bands Drive=1 | ok (0.6 dB) | ok (7.405%) |
| 34 | Curve Point 10 | Drawn Curve=1, All Bands Drive=1 | ok (2.0 dB) | ok (8.013%) |
| 35 | Curve Point 11 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (7.779%) |
| 36 | Curve Point 12 | Drawn Curve=1, All Bands Drive=1 | ok (2.9 dB) | ok (7.912%) |
| 37 | Curve Point 13 | Drawn Curve=1, All Bands Drive=1 | ok (2.1 dB) | ok (7.309%) |
| 38 | Curve Point 14 | Drawn Curve=1, All Bands Drive=1 | ok (-1.1 dB) | ok (8.097%) |
| 39 | Curve Point 15 | Drawn Curve=1, All Bands Drive=1 | ok (-0.6 dB) | ok (7.338%) |
| 40 | Curve Point 16 | Drawn Curve=1, All Bands Drive=1 | ok (3.2 dB) | ok (7.634%) |
| 41 | All Bands Drive |  | ok (-1.4 dB) | ok (7.333%) |
| 42 | All Bands Ceiling |  | ok (-3.2 dB) | ok (2.260%) |
| 43 | All Bands Gain |  | ok (-1.7 dB) | ok (6.007%) |
| 44 | Band 1 Drive | Bands Open=1 | ok (-0.9 dB) | ok (4.354%) |
| 45 | Band 1 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-4.5 dB) | ok (3.105%) |
| 46 | Band 1 Gain | Bands Open=1 | ok (1.0 dB) | ok (4.359%) |
| 47 | Band 1 Solo |  | ok (-5.0 dB) | ok (2.628%) |
| 48 | Band 1 Mute |  | ok (-11.8 dB) | ok (2.314%) |
| 49 | Band 1 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (6.810%) |
| 50 | Band 2 Drive | Bands Open=1 | ok (-1.0 dB) | ok (4.735%) |
| 51 | Band 2 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-2.2 dB) | ok (3.201%) |
| 52 | Band 2 Gain | Bands Open=1 | ok (1.1 dB) | ok (4.362%) |
| 53 | Band 2 Solo |  | ok (-0.6 dB) | ok (2.259%) |
| 54 | Band 2 Mute |  | ok (-9.7 dB) | ok (1.965%) |
| 55 | Band 2 Bypass | All Bands Drive=1 | ok (-0.4 dB) | ok (6.922%) |
| 56 | Band 3 Drive | Bands Open=1 | ok (-0.9 dB) | ok (5.080%) |
| 57 | Band 3 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-2.3 dB) | ok (3.414%) |
| 58 | Band 3 Gain | Bands Open=1 | ok (1.0 dB) | ok (4.458%) |
| 59 | Band 3 Solo |  | ok (-1.6 dB) | ok (2.460%) |
| 60 | Band 3 Mute |  | ok (-10.8 dB) | ok (1.804%) |
| 61 | Band 3 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (7.055%) |
| 62 | Band 4 Drive | Bands Open=1 | ok (1.4 dB) | ok (5.528%) |
| 63 | Band 4 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-3.0 dB) | ok (3.082%) |
| 64 | Band 4 Gain | Bands Open=1 | ok (0.1 dB) | ok (4.457%) |
| 65 | Band 4 Solo |  | ok (-3.5 dB) | ok (2.310%) |
| 66 | Band 4 Mute |  | ok (-8.4 dB) | ok (2.201%) |
| 67 | Band 4 Bypass | All Bands Drive=1 | ok (1.8 dB) | ok (6.396%) |
| 68 | Band 5 Drive | Bands=0.8, Bands Open=1 | ok (-0.6 dB) | ok (5.418%) |
| 69 | Band 5 Ceiling | Bands=0.8, Bands Open=1, All Bands Ceiling=0.5 | ok (-3.3 dB) | ok (4.219%) |
| 70 | Band 5 Gain | Bands=0.8, Bands Open=1 | ok (-0.2 dB) | ok (5.490%) |
| 71 | Band 5 Solo | Bands=0.8, Bands Open=0 | ok (-3.7 dB) | ok (3.145%) |
| 72 | Band 5 Mute | Bands=0.8, Bands Open=0 | ok (-11.9 dB) | ok (3.214%) |
| 73 | Band 5 Bypass | Bands=0.8, All Bands Drive=1 | ok (1.0 dB) | ok (7.745%) |
| 74 | Band 6 Drive | Bands=1, Bands Open=1 | ok (-0.9 dB) | ok (5.313%) |
| 75 | Band 6 Ceiling | Bands=1, Bands Open=1, All Bands Ceiling=0.5 | ok (-4.4 dB) | ok (3.994%) |
| 76 | Band 6 Gain | Bands=1, Bands Open=1 | ok (-0.3 dB) | ok (5.019%) |
| 77 | Band 6 Solo | Bands=1, Bands Open=0 | ok (-3.0 dB) | ok (3.202%) |
| 78 | Band 6 Mute | Bands=1, Bands Open=0 | ok (-12.1 dB) | ok (2.805%) |
| 79 | Band 6 Bypass | Bands=1, All Bands Drive=1 | ok (0.0 dB) | ok (7.705%) |
| 80 | Bypass |  | ok (-1.7 dB) | ok (2.189%) |
| 81 | Wet |  | ok (-1.7 dB) | ok (1.884%) |
| 82 | Delta |  | ok (-3.5 dB) | ok (1.817%) |
