# Plugin QA: CLAP: OpenClip Multiband

| Check | Status | Detail |
|---|---|---|
| loads | PASS | CLAP: OpenClip Multiband (OpenGrid), 83 parameters |
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
| 0 | Input Gain |  | ok (-1.4 dB) | ok (8.141%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (5.462%) |
| 2 | Zero Latency |  | ok (-0.2 dB) | ok (3.530%) |
| 3 | Auto Gain |  | ok (0.9 dB) | ok (2.700%) |
| 4 | Delta |  | ok (-4.0 dB) | ok (3.000%) |
| 5 | Bands |  | ok (-1.7 dB) | ok (4.526%) |
| 6 | Crossover Slope |  | ok (1.4 dB) | ok (2.157%) |
| 7 | Crossover 1 |  | ok (-1.2 dB) | ok (3.421%) |
| 8 | Crossover 2 |  | ok (-1.4 dB) | ok (3.650%) |
| 9 | Crossover 3 |  | ok (-1.5 dB) | ok (3.184%) |
| 10 | Crossover 4 | Bands=0.8 | ok (-2.2 dB) | ok (4.933%) |
| 11 | Crossover 5 | Bands=1 | ok (-1.9 dB) | ok (5.135%) |
| 12 | Algorithm | All Bands Drive=1 | ok (3.8 dB) | ok (9.879%) |
| 13 | Shape | All Bands Drive=1 | ok (-5.3 dB) | ok (9.806%) |
| 14 | Character | All Bands Drive=1 | ok (-25.7 dB) | ok (10.087%) |
| 15 | Single/Multi Blend | All Bands Drive=1 | ok (-1.4 dB) | ok (9.310%) |
| 16 | Dry/Wet Mix | All Bands Drive=1 | ok (-1.4 dB) | ok (10.048%) |
| 17 | Single Drive | Single/Multi Blend=0 | ok (-1.9 dB) | ok (8.733%) |
| 18 | Single Ceiling | Single/Multi Blend=0 | ok (-3.5 dB) | ok (1.768%) |
| 19 | Output Ceiling On | Input Gain=1 | ok (2.5 dB) | ok (7.269%) |
| 20 | Output Ceiling | Input Gain=1 | ok (-1.5 dB) | ok (7.306%) |
| 21 | Anti-Alias | All Bands Drive=1 | ok (3.7 dB) | ok (5.191%) |
| 22 | Oversampling | All Bands Drive=1 | ok (4.9 dB) | ok (9.834%) |
| 23 | Bands Open |  | excused (-2.7 dB) | ok (5.553%) |
| 24 | Drawn Curve | All Bands Drive=1 | ok (-2.6 dB) | ok (9.854%) |
| 25 | Curve Point 1 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (10.660%) |
| 26 | Curve Point 2 | Drawn Curve=1, All Bands Drive=1 | ok (0.1 dB) | ok (10.059%) |
| 27 | Curve Point 3 | Drawn Curve=1, All Bands Drive=1 | ok (-2.4 dB) | ok (9.838%) |
| 28 | Curve Point 4 | Drawn Curve=1, All Bands Drive=1 | ok (-2.3 dB) | ok (9.739%) |
| 29 | Curve Point 5 | Drawn Curve=1, All Bands Drive=1 | ok (-2.8 dB) | ok (9.498%) |
| 30 | Curve Point 6 | Drawn Curve=1, All Bands Drive=1 | ok (-3.0 dB) | ok (9.540%) |
| 31 | Curve Point 7 | Drawn Curve=1, All Bands Drive=1 | ok (-2.1 dB) | ok (9.771%) |
| 32 | Curve Point 8 | Drawn Curve=1, All Bands Drive=1 | ok (-0.5 dB) | ok (9.382%) |
| 33 | Curve Point 9 | Drawn Curve=1, All Bands Drive=1 | ok (0.6 dB) | ok (9.564%) |
| 34 | Curve Point 10 | Drawn Curve=1, All Bands Drive=1 | ok (2.0 dB) | ok (9.945%) |
| 35 | Curve Point 11 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (9.417%) |
| 36 | Curve Point 12 | Drawn Curve=1, All Bands Drive=1 | ok (2.9 dB) | ok (9.783%) |
| 37 | Curve Point 13 | Drawn Curve=1, All Bands Drive=1 | ok (2.1 dB) | ok (9.518%) |
| 38 | Curve Point 14 | Drawn Curve=1, All Bands Drive=1 | ok (-1.1 dB) | ok (10.380%) |
| 39 | Curve Point 15 | Drawn Curve=1, All Bands Drive=1 | ok (-0.6 dB) | ok (9.673%) |
| 40 | Curve Point 16 | Drawn Curve=1, All Bands Drive=1 | ok (3.2 dB) | ok (9.898%) |
| 41 | All Bands Drive |  | ok (-1.4 dB) | ok (9.178%) |
| 42 | All Bands Ceiling |  | ok (-3.2 dB) | ok (3.936%) |
| 43 | All Bands Gain |  | ok (-1.7 dB) | ok (7.230%) |
| 44 | Band 1 Drive | Bands Open=1 | ok (-0.9 dB) | ok (6.794%) |
| 45 | Band 1 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-4.5 dB) | ok (6.097%) |
| 46 | Band 1 Gain | Bands Open=1 | ok (1.0 dB) | ok (7.010%) |
| 47 | Band 1 Solo |  | ok (-5.0 dB) | ok (3.578%) |
| 48 | Band 1 Mute |  | ok (-11.8 dB) | ok (2.899%) |
| 49 | Band 1 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (8.754%) |
| 50 | Band 2 Drive | Bands Open=1 | ok (-1.0 dB) | ok (7.252%) |
| 51 | Band 2 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-2.2 dB) | ok (6.267%) |
| 52 | Band 2 Gain | Bands Open=1 | ok (1.1 dB) | ok (7.083%) |
| 53 | Band 2 Solo |  | ok (-0.6 dB) | ok (3.839%) |
| 54 | Band 2 Mute |  | ok (-9.7 dB) | ok (3.588%) |
| 55 | Band 2 Bypass | All Bands Drive=1 | ok (-0.4 dB) | ok (8.711%) |
| 56 | Band 3 Drive | Bands Open=1 | ok (-0.9 dB) | ok (7.339%) |
| 57 | Band 3 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-2.3 dB) | ok (6.339%) |
| 58 | Band 3 Gain | Bands Open=1 | ok (1.0 dB) | ok (6.633%) |
| 59 | Band 3 Solo |  | ok (-1.6 dB) | ok (4.013%) |
| 60 | Band 3 Mute |  | ok (-10.8 dB) | ok (3.225%) |
| 61 | Band 3 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (8.761%) |
| 62 | Band 4 Drive | Bands Open=1 | ok (1.4 dB) | ok (7.964%) |
| 63 | Band 4 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-3.0 dB) | ok (5.911%) |
| 64 | Band 4 Gain | Bands Open=1 | ok (0.1 dB) | ok (6.874%) |
| 65 | Band 4 Solo |  | ok (-3.5 dB) | ok (3.397%) |
| 66 | Band 4 Mute |  | ok (-8.4 dB) | ok (3.604%) |
| 67 | Band 4 Bypass | All Bands Drive=1 | ok (1.8 dB) | ok (8.868%) |
| 68 | Band 5 Drive | Bands=0.8, Bands Open=1 | ok (-0.6 dB) | ok (8.815%) |
| 69 | Band 5 Ceiling | Bands=0.8, Bands Open=1, All Bands Ceiling=0.5 | ok (-3.3 dB) | ok (8.019%) |
| 70 | Band 5 Gain | Bands=0.8, Bands Open=1 | ok (-0.2 dB) | ok (8.839%) |
| 71 | Band 5 Solo | Bands=0.8, Bands Open=0 | ok (-3.7 dB) | ok (5.809%) |
| 72 | Band 5 Mute | Bands=0.8, Bands Open=0 | ok (-11.9 dB) | ok (5.591%) |
| 73 | Band 5 Bypass | Bands=0.8, All Bands Drive=1 | ok (1.0 dB) | ok (11.203%) |
| 74 | Band 6 Drive | Bands=1, Bands Open=1 | ok (-0.9 dB) | ok (8.898%) |
| 75 | Band 6 Ceiling | Bands=1, Bands Open=1, All Bands Ceiling=0.5 | ok (-4.4 dB) | ok (7.979%) |
| 76 | Band 6 Gain | Bands=1, Bands Open=1 | ok (-0.3 dB) | ok (8.701%) |
| 77 | Band 6 Solo | Bands=1, Bands Open=0 | ok (-3.0 dB) | ok (5.860%) |
| 78 | Band 6 Mute | Bands=1, Bands Open=0 | ok (-12.1 dB) | ok (4.919%) |
| 79 | Band 6 Bypass | Bands=1, All Bands Drive=1 | ok (0.0 dB) | ok (11.584%) |
| 80 | Bypass |  | ok (-1.7 dB) | ok (2.525%) |
| 81 | Wet |  | ok (-1.7 dB) | ok (2.585%) |
| 82 | Delta |  | ok (-3.5 dB) | ok (2.595%) |
