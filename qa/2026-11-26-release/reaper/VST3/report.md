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
| 0 | Input Gain |  | ok (-1.4 dB) | ok (8.367%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (5.249%) |
| 2 | Zero Latency |  | ok (-1.0 dB) | ok (3.407%) |
| 3 | Auto Gain |  | ok (-6.0 dB) | ok (3.014%) |
| 4 | Delta |  | ok (-4.0 dB) | ok (2.439%) |
| 5 | Bands |  | ok (-1.7 dB) | ok (4.317%) |
| 6 | Crossover Slope |  | ok (1.4 dB) | ok (3.374%) |
| 7 | Crossover 1 |  | ok (-1.2 dB) | ok (3.469%) |
| 8 | Crossover 2 |  | ok (-1.4 dB) | ok (3.291%) |
| 9 | Crossover 3 |  | ok (-1.5 dB) | ok (3.281%) |
| 10 | Crossover 4 | Bands=0.8 | ok (-2.2 dB) | ok (4.762%) |
| 11 | Crossover 5 | Bands=1 | ok (-1.9 dB) | ok (5.087%) |
| 12 | Algorithm | All Bands Drive=1 | ok (3.8 dB) | ok (10.069%) |
| 13 | Shape | All Bands Drive=1 | ok (-5.3 dB) | ok (10.021%) |
| 14 | Character | All Bands Drive=1 | ok (-25.7 dB) | ok (10.167%) |
| 15 | Single/Multi Blend | All Bands Drive=1 | ok (-1.4 dB) | ok (9.850%) |
| 16 | Dry/Wet Mix | All Bands Drive=1 | ok (-1.4 dB) | ok (10.036%) |
| 17 | Single Drive | Single/Multi Blend=0 | ok (-1.9 dB) | ok (8.198%) |
| 18 | Single Ceiling | Single/Multi Blend=0 | ok (-3.5 dB) | ok (2.835%) |
| 19 | Output Ceiling On | Input Gain=1 | ok (2.5 dB) | ok (9.017%) |
| 20 | Output Ceiling | Input Gain=1 | ok (-1.5 dB) | ok (8.158%) |
| 21 | Anti-Alias | All Bands Drive=1 | ok (3.7 dB) | ok (4.736%) |
| 22 | Oversampling | All Bands Drive=1 | ok (4.0 dB) | ok (8.968%) |
| 23 | Bands Open |  | excused (-3.7 dB) | ok (4.743%) |
| 24 | Drawn Curve | All Bands Drive=1 | ok (-2.6 dB) | ok (10.328%) |
| 25 | Curve Point 1 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (10.526%) |
| 26 | Curve Point 2 | Drawn Curve=1, All Bands Drive=1 | ok (0.1 dB) | ok (10.758%) |
| 27 | Curve Point 3 | Drawn Curve=1, All Bands Drive=1 | ok (-2.4 dB) | ok (10.623%) |
| 28 | Curve Point 4 | Drawn Curve=1, All Bands Drive=1 | ok (-2.3 dB) | ok (10.171%) |
| 29 | Curve Point 5 | Drawn Curve=1, All Bands Drive=1 | ok (-2.8 dB) | ok (10.115%) |
| 30 | Curve Point 6 | Drawn Curve=1, All Bands Drive=1 | ok (-3.0 dB) | ok (10.039%) |
| 31 | Curve Point 7 | Drawn Curve=1, All Bands Drive=1 | ok (-2.1 dB) | ok (10.781%) |
| 32 | Curve Point 8 | Drawn Curve=1, All Bands Drive=1 | ok (-0.5 dB) | ok (10.310%) |
| 33 | Curve Point 9 | Drawn Curve=1, All Bands Drive=1 | ok (0.6 dB) | ok (10.330%) |
| 34 | Curve Point 10 | Drawn Curve=1, All Bands Drive=1 | ok (2.0 dB) | ok (10.251%) |
| 35 | Curve Point 11 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (10.250%) |
| 36 | Curve Point 12 | Drawn Curve=1, All Bands Drive=1 | ok (2.9 dB) | ok (10.465%) |
| 37 | Curve Point 13 | Drawn Curve=1, All Bands Drive=1 | ok (2.1 dB) | ok (10.280%) |
| 38 | Curve Point 14 | Drawn Curve=1, All Bands Drive=1 | ok (-1.1 dB) | ok (10.456%) |
| 39 | Curve Point 15 | Drawn Curve=1, All Bands Drive=1 | ok (-0.6 dB) | ok (10.081%) |
| 40 | Curve Point 16 | Drawn Curve=1, All Bands Drive=1 | ok (3.2 dB) | ok (10.267%) |
| 41 | All Bands Drive |  | ok (-1.4 dB) | ok (10.419%) |
| 42 | All Bands Ceiling |  | ok (-3.2 dB) | ok (3.489%) |
| 43 | All Bands Gain |  | ok (-1.7 dB) | ok (7.618%) |
| 44 | Band 1 Drive | Bands Open=1 | ok (-0.9 dB) | ok (6.922%) |
| 45 | Band 1 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-4.5 dB) | ok (6.058%) |
| 46 | Band 1 Gain | Bands Open=1 | ok (1.0 dB) | ok (6.650%) |
| 47 | Band 1 Solo |  | ok (-5.0 dB) | ok (3.494%) |
| 48 | Band 1 Mute |  | ok (-11.8 dB) | ok (2.589%) |
| 49 | Band 1 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (8.872%) |
| 50 | Band 2 Drive | Bands Open=1 | ok (-1.0 dB) | ok (7.599%) |
| 51 | Band 2 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-2.2 dB) | ok (5.552%) |
| 52 | Band 2 Gain | Bands Open=1 | ok (1.1 dB) | ok (7.139%) |
| 53 | Band 2 Solo |  | ok (-0.6 dB) | ok (3.462%) |
| 54 | Band 2 Mute |  | ok (-9.7 dB) | ok (2.573%) |
| 55 | Band 2 Bypass | All Bands Drive=1 | ok (-0.4 dB) | ok (9.349%) |
| 56 | Band 3 Drive | Bands Open=1 | ok (-0.9 dB) | ok (7.600%) |
| 57 | Band 3 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-2.3 dB) | ok (5.710%) |
| 58 | Band 3 Gain | Bands Open=1 | ok (1.0 dB) | ok (6.686%) |
| 59 | Band 3 Solo |  | ok (-1.6 dB) | ok (3.555%) |
| 60 | Band 3 Mute |  | ok (-10.8 dB) | ok (2.544%) |
| 61 | Band 3 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (9.007%) |
| 62 | Band 4 Drive | Bands Open=1 | ok (1.4 dB) | ok (8.048%) |
| 63 | Band 4 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-3.0 dB) | ok (5.599%) |
| 64 | Band 4 Gain | Bands Open=1 | ok (0.1 dB) | ok (6.872%) |
| 65 | Band 4 Solo |  | ok (-3.5 dB) | ok (3.076%) |
| 66 | Band 4 Mute |  | ok (-8.4 dB) | ok (3.334%) |
| 67 | Band 4 Bypass | All Bands Drive=1 | ok (1.8 dB) | ok (8.192%) |
| 68 | Band 5 Drive | Bands=0.8, Bands Open=1 | ok (-0.6 dB) | ok (9.036%) |
| 69 | Band 5 Ceiling | Bands=0.8, Bands Open=1, All Bands Ceiling=0.5 | ok (-3.3 dB) | ok (7.655%) |
| 70 | Band 5 Gain | Bands=0.8, Bands Open=1 | ok (-0.2 dB) | ok (8.842%) |
| 71 | Band 5 Solo | Bands=0.8, Bands Open=0 | ok (-3.7 dB) | ok (5.282%) |
| 72 | Band 5 Mute | Bands=0.8, Bands Open=0 | ok (-11.9 dB) | ok (5.494%) |
| 73 | Band 5 Bypass | Bands=0.8, All Bands Drive=1 | ok (1.0 dB) | ok (10.619%) |
| 74 | Band 6 Drive | Bands=1, Bands Open=1 | ok (-0.9 dB) | ok (8.510%) |
| 75 | Band 6 Ceiling | Bands=1, Bands Open=1, All Bands Ceiling=0.5 | ok (-4.4 dB) | ok (7.505%) |
| 76 | Band 6 Gain | Bands=1, Bands Open=1 | ok (-0.3 dB) | ok (8.487%) |
| 77 | Band 6 Solo | Bands=1, Bands Open=0 | ok (-3.0 dB) | ok (5.664%) |
| 78 | Band 6 Mute | Bands=1, Bands Open=0 | ok (-12.1 dB) | ok (5.352%) |
| 79 | Band 6 Bypass | Bands=1, All Bands Drive=1 | ok (0.0 dB) | ok (10.863%) |
| 80 | Bypass |  | ok (-1.7 dB) | excused (2.995%) |
| 81 | Bypass |  | ok (-1.7 dB) | excused (2.301%) |
| 82 | Wet |  | ok (-1.7 dB) | excused (2.674%) |
| 83 | Delta |  | ok (-3.5 dB) | excused (2.795%) |
