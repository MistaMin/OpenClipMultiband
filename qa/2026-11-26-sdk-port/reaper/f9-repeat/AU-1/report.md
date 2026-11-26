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
| 0 | Input Gain |  | ok (-1.4 dB) | ok (3.670%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (0.953%) |
| 2 | Zero Latency |  | ok (-0.2 dB) | ok (2.441%) |
| 3 | Auto Gain |  | ok (-33.3 dB) | ok (2.578%) |
| 4 | Delta |  | ok (-4.0 dB) | ok (2.138%) |
| 5 | Bands |  | ok (-1.7 dB) | ok (2.751%) |
| 6 | Crossover Slope |  | ok (1.4 dB) | ok (2.354%) |
| 7 | Crossover 1 |  | ok (-1.2 dB) | ok (2.283%) |
| 8 | Crossover 2 |  | ok (-2.7 dB) | ok (2.389%) |
| 9 | Crossover 3 |  | ok (-1.5 dB) | ok (2.231%) |
| 10 | Crossover 4 | Bands=0.8 | ok (-2.2 dB) | ok (3.686%) |
| 11 | Crossover 5 | Bands=1 | ok (-1.8 dB) | ok (3.703%) |
| 12 | Algorithm | All Bands Drive=1 | ok (0.4 dB) | ok (3.484%) |
| 13 | Shape | All Bands Drive=1 | ok (-7.5 dB) | ok (3.461%) |
| 14 | Character | All Bands Drive=1 | ok (-25.7 dB) | ok (3.675%) |
| 15 | Single/Multi Blend | All Bands Drive=1 | ok (-1.4 dB) | ok (3.751%) |
| 16 | Dry/Wet Mix | All Bands Drive=1 | ok (-1.4 dB) | ok (3.802%) |
| 17 | Single Drive | Single/Multi Blend=0 | ok (-1.9 dB) | ok (2.698%) |
| 18 | Single Ceiling | Single/Multi Blend=0 | ok (-5.2 dB) | ok (2.532%) |
| 19 | Output Ceiling On | Input Gain=1 | ok (-1.6 dB) | ok (4.397%) |
| 20 | Output Ceiling | Input Gain=1 | ok (-1.5 dB) | ok (4.570%) |
| 21 | Anti-Alias | All Bands Drive=1 | ok (3.7 dB) | ok (3.683%) |
| 22 | Oversampling | All Bands Drive=1 | ok (4.1 dB) | ok (3.075%) |
| 23 | Bands Open |  | excused (-240.0 dB) | ok (4.294%) |
| 24 | Drawn Curve | All Bands Drive=1 | ok (-21.7 dB) | ok (3.790%) |
| 25 | Curve Point 1 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (4.676%) |
| 26 | Curve Point 2 | Drawn Curve=1, All Bands Drive=1 | ok (0.1 dB) | ok (3.833%) |
| 27 | Curve Point 3 | Drawn Curve=1, All Bands Drive=1 | ok (-2.4 dB) | ok (4.111%) |
| 28 | Curve Point 4 | Drawn Curve=1, All Bands Drive=1 | ok (-2.3 dB) | ok (3.702%) |
| 29 | Curve Point 5 | Drawn Curve=1, All Bands Drive=1 | ok (-2.8 dB) | ok (3.942%) |
| 30 | Curve Point 6 | Drawn Curve=1, All Bands Drive=1 | ok (-3.0 dB) | ok (3.831%) |
| 31 | Curve Point 7 | Drawn Curve=1, All Bands Drive=1 | ok (-2.1 dB) | ok (3.848%) |
| 32 | Curve Point 8 | Drawn Curve=1, All Bands Drive=1 | ok (-0.5 dB) | ok (4.185%) |
| 33 | Curve Point 9 | Drawn Curve=1, All Bands Drive=1 | ok (0.6 dB) | ok (3.841%) |
| 34 | Curve Point 10 | Drawn Curve=1, All Bands Drive=1 | ok (2.0 dB) | ok (4.193%) |
| 35 | Curve Point 11 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (3.567%) |
| 36 | Curve Point 12 | Drawn Curve=1, All Bands Drive=1 | ok (2.9 dB) | ok (3.785%) |
| 37 | Curve Point 13 | Drawn Curve=1, All Bands Drive=1 | ok (2.1 dB) | ok (4.179%) |
| 38 | Curve Point 14 | Drawn Curve=1, All Bands Drive=1 | ok (-1.1 dB) | ok (4.225%) |
| 39 | Curve Point 15 | Drawn Curve=1, All Bands Drive=1 | ok (-0.6 dB) | ok (4.153%) |
| 40 | Curve Point 16 | Drawn Curve=1, All Bands Drive=1 | ok (3.2 dB) | ok (3.998%) |
| 41 | All Bands Drive |  | ok (-1.4 dB) | ok (3.293%) |
| 42 | All Bands Ceiling |  | ok (-8.6 dB) | ok (3.284%) |
| 43 | All Bands Gain |  | ok (-1.7 dB) | ok (2.300%) |
| 44 | Band 1 Drive | Bands Open=1 | ok (-0.9 dB) | ok (4.954%) |
| 45 | Band 1 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-13.3 dB) | ok (4.996%) |
| 46 | Band 1 Gain | Bands Open=1 | ok (1.0 dB) | ok (5.013%) |
| 47 | Band 1 Solo |  | ok (-5.7 dB) | ok (2.220%) |
| 48 | Band 1 Mute |  | ok (-11.8 dB) | ok (2.746%) |
| 49 | Band 1 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (3.635%) |
| 50 | Band 2 Drive | Bands Open=1 | ok (-0.8 dB) | ok (5.028%) |
| 51 | Band 2 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-12.8 dB) | ok (4.924%) |
| 52 | Band 2 Gain | Bands Open=1 | ok (1.1 dB) | ok (4.842%) |
| 53 | Band 2 Solo |  | ok (-4.9 dB) | ok (2.442%) |
| 54 | Band 2 Mute |  | ok (-9.7 dB) | ok (2.373%) |
| 55 | Band 2 Bypass | All Bands Drive=1 | ok (-1.0 dB) | ok (3.665%) |
| 56 | Band 3 Drive | Bands Open=1 | ok (-0.9 dB) | ok (4.586%) |
| 57 | Band 3 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-12.9 dB) | ok (4.913%) |
| 58 | Band 3 Gain | Bands Open=1 | ok (1.0 dB) | ok (4.541%) |
| 59 | Band 3 Solo |  | ok (-4.5 dB) | ok (2.244%) |
| 60 | Band 3 Mute |  | ok (-10.8 dB) | ok (2.454%) |
| 61 | Band 3 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (4.362%) |
| 62 | Band 4 Drive | Bands Open=1 | ok (-0.5 dB) | ok (4.677%) |
| 63 | Band 4 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-10.7 dB) | ok (5.111%) |
| 64 | Band 4 Gain | Bands Open=1 | ok (0.1 dB) | ok (4.377%) |
| 65 | Band 4 Solo |  | ok (-6.6 dB) | ok (2.373%) |
| 66 | Band 4 Mute |  | ok (-8.4 dB) | ok (2.556%) |
| 67 | Band 4 Bypass | All Bands Drive=1 | ok (1.8 dB) | ok (3.832%) |
| 68 | Band 5 Drive | Bands=0.8, Bands Open=1 | ok (-0.6 dB) | ok (6.398%) |
| 69 | Band 5 Ceiling | Bands=0.8, Bands Open=1, All Bands Ceiling=0.5 | ok (-13.0 dB) | ok (5.867%) |
| 70 | Band 5 Gain | Bands=0.8, Bands Open=1 | ok (-0.2 dB) | ok (6.188%) |
| 71 | Band 5 Solo | Bands=0.8, Bands Open=0 | ok (-5.7 dB) | ok (3.566%) |
| 72 | Band 5 Mute | Bands=0.8, Bands Open=0 | ok (-11.9 dB) | ok (3.762%) |
| 73 | Band 5 Bypass | Bands=0.8, All Bands Drive=1 | ok (1.0 dB) | ok (5.086%) |
| 74 | Band 6 Drive | Bands=1, Bands Open=1 | ok (-1.2 dB) | ok (6.235%) |
| 75 | Band 6 Ceiling | Bands=1, Bands Open=1, All Bands Ceiling=0.5 | ok (-13.1 dB) | ok (6.270%) |
| 76 | Band 6 Gain | Bands=1, Bands Open=1 | ok (-0.3 dB) | ok (6.307%) |
| 77 | Band 6 Solo | Bands=1, Bands Open=0 | ok (-4.9 dB) | ok (3.754%) |
| 78 | Band 6 Mute | Bands=1, Bands Open=0 | ok (-12.1 dB) | ok (4.018%) |
| 79 | Band 6 Bypass | Bands=1, All Bands Drive=1 | ok (0.0 dB) | ok (4.423%) |
| 80 | Bypass |  | ok (-1.7 dB) | ok (1.825%) |
| 81 | Wet |  | ok (-1.7 dB) | ok (2.401%) |
| 82 | Delta |  | ok (-3.5 dB) | ok (2.510%) |
