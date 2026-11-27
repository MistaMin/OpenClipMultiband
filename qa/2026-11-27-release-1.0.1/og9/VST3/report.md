# Plugin QA: VST3: OpenClip Multiband

| Check | Status | Detail |
|---|---|---|
| loads | PASS | OpenClip Multiband, 81 parameters |
| finite output | PASS | peak -6.2 dBFS |
| not silent | PASS | rms -15.0 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | SKIPPED | visual checks disabled |
| every parameter changes the sound | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | SKIPPED | headless: OpenGrid9 runs without an editor window |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-1.5 dB) | -  |
| 1 | Output Gain |  | ok (17.2 dB) | -  |
| 2 | Zero Latency |  | ok (-6.0 dB) | -  |
| 3 | Auto Gain |  | ok (-24.5 dB) | -  |
| 4 | Delta |  | ok (-5.9 dB) | -  |
| 5 | Bands |  | ok (-1.7 dB) | -  |
| 6 | Crossover Slope |  | ok (-0.6 dB) | -  |
| 7 | Crossover 1 |  | ok (-1.5 dB) | -  |
| 8 | Crossover 2 |  | ok (-1.3 dB) | -  |
| 9 | Crossover 3 |  | ok (-2.3 dB) | -  |
| 10 | Crossover 4 | Bands=0.8 | ok (-4.4 dB) | -  |
| 11 | Crossover 5 | Bands=1 | ok (-5.9 dB) | -  |
| 12 | Algorithm | All Bands Drive=1 | ok (-2.3 dB) | -  |
| 13 | Shape | All Bands Drive=1 | ok (-8.8 dB) | -  |
| 14 | Character | All Bands Drive=1 | ok (-29.5 dB) | -  |
| 15 | Single/Multi Blend | All Bands Drive=1 | ok (-1.5 dB) | -  |
| 16 | Dry/Wet Mix | All Bands Drive=1 | ok (-1.5 dB) | -  |
| 17 | Single Drive | Single/Multi Blend=0 | ok (-1.8 dB) | -  |
| 18 | Single Ceiling | Single/Multi Blend=0 | ok (-6.8 dB) | -  |
| 19 | Output Ceiling On | Input Gain=1 | ok (13.3 dB) | -  |
| 20 | Output Ceiling | Input Gain=1 | ok (-1.5 dB) | -  |
| 21 | Anti-Alias | All Bands Drive=1 | ok (-7.3 dB) | -  |
| 22 | Oversampling | All Bands Drive=1 | ok (-8.4 dB) | -  |
| 23 | Bands Open |  | excused (-11.8 dB) | -  |
| 24 | Drawn Curve | All Bands Drive=1 | ok (-23.0 dB) | -  |
| 25 | Curve Point 1 | Drawn Curve=1, All Bands Drive=1 | ok (1.0 dB) | -  |
| 26 | Curve Point 2 | Drawn Curve=1, All Bands Drive=1 | ok (-0.4 dB) | -  |
| 27 | Curve Point 3 | Drawn Curve=1, All Bands Drive=1 | ok (-1.9 dB) | -  |
| 28 | Curve Point 4 | Drawn Curve=1, All Bands Drive=1 | ok (-2.7 dB) | -  |
| 29 | Curve Point 5 | Drawn Curve=1, All Bands Drive=1 | ok (-2.8 dB) | -  |
| 30 | Curve Point 6 | Drawn Curve=1, All Bands Drive=1 | ok (-2.0 dB) | -  |
| 31 | Curve Point 7 | Drawn Curve=1, All Bands Drive=1 | ok (-2.4 dB) | -  |
| 32 | Curve Point 8 | Drawn Curve=1, All Bands Drive=1 | ok (-1.3 dB) | -  |
| 33 | Curve Point 9 | Drawn Curve=1, All Bands Drive=1 | ok (-0.2 dB) | -  |
| 34 | Curve Point 10 | Drawn Curve=1, All Bands Drive=1 | ok (-1.0 dB) | -  |
| 35 | Curve Point 11 | Drawn Curve=1, All Bands Drive=1 | ok (-1.6 dB) | -  |
| 36 | Curve Point 12 | Drawn Curve=1, All Bands Drive=1 | ok (-1.8 dB) | -  |
| 37 | Curve Point 13 | Drawn Curve=1, All Bands Drive=1 | ok (-1.1 dB) | -  |
| 38 | Curve Point 14 | Drawn Curve=1, All Bands Drive=1 | ok (-1.3 dB) | -  |
| 39 | Curve Point 15 | Drawn Curve=1, All Bands Drive=1 | ok (-1.3 dB) | -  |
| 40 | Curve Point 16 | Drawn Curve=1, All Bands Drive=1 | ok (1.3 dB) | -  |
| 41 | All Bands Drive |  | ok (-0.1 dB) | -  |
| 42 | All Bands Ceiling |  | ok (-2.4 dB) | -  |
| 43 | All Bands Gain |  | ok (-0.1 dB) | -  |
| 44 | Band 1 Drive | Bands Open=1 | ok (-0.9 dB) | -  |
| 45 | Band 1 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-13.4 dB) | -  |
| 46 | Band 1 Gain | Bands Open=1 | ok (0.7 dB) | -  |
| 47 | Band 1 Solo |  | ok (-6.1 dB) | -  |
| 48 | Band 1 Mute |  | ok (-10.3 dB) | -  |
| 49 | Band 1 Bypass | All Bands Drive=1 | ok (-0.9 dB) | -  |
| 50 | Band 2 Drive | Bands Open=1 | ok (-0.6 dB) | -  |
| 51 | Band 2 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-12.9 dB) | -  |
| 52 | Band 2 Gain | Bands Open=1 | ok (1.2 dB) | -  |
| 53 | Band 2 Solo |  | ok (-7.1 dB) | -  |
| 54 | Band 2 Mute |  | ok (-9.3 dB) | -  |
| 55 | Band 2 Bypass | All Bands Drive=1 | ok (-1.0 dB) | -  |
| 56 | Band 3 Drive | Bands Open=1 | ok (-0.9 dB) | -  |
| 57 | Band 3 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-13.0 dB) | -  |
| 58 | Band 3 Gain | Bands Open=1 | ok (0.7 dB) | -  |
| 59 | Band 3 Solo |  | ok (-6.0 dB) | -  |
| 60 | Band 3 Mute |  | ok (-9.7 dB) | -  |
| 61 | Band 3 Bypass | All Bands Drive=1 | ok (-1.5 dB) | -  |
| 62 | Band 4 Drive | Bands Open=1 | ok (-0.5 dB) | -  |
| 63 | Band 4 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-10.4 dB) | -  |
| 64 | Band 4 Gain | Bands Open=1 | ok (0.2 dB) | -  |
| 65 | Band 4 Solo |  | ok (-6.7 dB) | -  |
| 66 | Band 4 Mute |  | ok (-9.0 dB) | -  |
| 67 | Band 4 Bypass | All Bands Drive=1 | ok (0.1 dB) | -  |
| 68 | Band 5 Drive | Bands=0.8, Bands Open=1 | ok (-1.6 dB) | -  |
| 69 | Band 5 Ceiling | Bands=0.8, Bands Open=1, All Bands Ceiling=0.5 | ok (-13.0 dB) | -  |
| 70 | Band 5 Gain | Bands=0.8, Bands Open=1 | ok (-1.3 dB) | -  |
| 71 | Band 5 Solo | Bands=0.8, Bands Open=0 | ok (-5.6 dB) | -  |
| 72 | Band 5 Mute | Bands=0.8, Bands Open=0 | ok (-12.0 dB) | -  |
| 73 | Band 5 Bypass | Bands=0.8, All Bands Drive=1 | ok (-1.5 dB) | -  |
| 74 | Band 6 Drive | Bands=1, Bands Open=1 | ok (-3.8 dB) | -  |
| 75 | Band 6 Ceiling | Bands=1, Bands Open=1, All Bands Ceiling=0.5 | ok (-13.2 dB) | -  |
| 76 | Band 6 Gain | Bands=1, Bands Open=1 | ok (-3.8 dB) | -  |
| 77 | Band 6 Solo | Bands=1, Bands Open=0 | ok (-5.6 dB) | -  |
| 78 | Band 6 Mute | Bands=1, Bands Open=0 | ok (-12.2 dB) | -  |
| 79 | Band 6 Bypass | Bands=1, All Bands Drive=1 | ok (-3.6 dB) | -  |
| 80 | Bypass |  | ok (-1.7 dB) | -  |
