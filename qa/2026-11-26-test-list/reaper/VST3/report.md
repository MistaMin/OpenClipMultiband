# Plugin QA: VST3: OpenClip Multiband

| Check | Status | Detail |
|---|---|---|
| loads | PASS | VST3: OpenClip Multiband (OpenGrid), 84 parameters |
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
| 0 | Input Gain |  | ok (-1.4 dB) | ok (7.633%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (5.694%) |
| 2 | Zero Latency |  | ok (-0.2 dB) | ok (2.875%) |
| 3 | Auto Gain |  | ok (0.9 dB) | ok (2.624%) |
| 4 | Delta |  | ok (-4.0 dB) | ok (3.258%) |
| 5 | Bands |  | ok (-1.7 dB) | ok (4.918%) |
| 6 | Crossover Slope |  | ok (1.4 dB) | ok (3.589%) |
| 7 | Crossover 1 |  | ok (-1.2 dB) | ok (3.795%) |
| 8 | Crossover 2 |  | ok (-1.4 dB) | ok (4.049%) |
| 9 | Crossover 3 |  | ok (-1.5 dB) | ok (3.527%) |
| 10 | Crossover 4 | Bands=0.8 | ok (-2.2 dB) | ok (5.135%) |
| 11 | Crossover 5 | Bands=1 | ok (-1.9 dB) | ok (5.146%) |
| 12 | Algorithm | All Bands Drive=1 | ok (3.8 dB) | ok (8.617%) |
| 13 | Shape | All Bands Drive=1 | ok (-5.3 dB) | ok (9.204%) |
| 14 | Character | All Bands Drive=1 | ok (-25.7 dB) | ok (9.627%) |
| 15 | Single/Multi Blend | All Bands Drive=1 | ok (-1.4 dB) | ok (9.849%) |
| 16 | Dry/Wet Mix | All Bands Drive=1 | ok (-1.4 dB) | ok (8.968%) |
| 17 | Single Drive | Single/Multi Blend=0 | ok (-1.9 dB) | ok (7.810%) |
| 18 | Single Ceiling | Single/Multi Blend=0 | ok (-3.5 dB) | ok (3.054%) |
| 19 | Output Ceiling On | Input Gain=1 | ok (2.5 dB) | ok (8.539%) |
| 20 | Output Ceiling | Input Gain=1 | ok (-1.5 dB) | ok (7.303%) |
| 21 | Anti-Alias | All Bands Drive=1 | ok (3.7 dB) | ok (8.038%) |
| 22 | Oversampling | All Bands Drive=1 | ok (3.0 dB) | ok (9.313%) |
| 23 | Bands Open |  | excused (-3.1 dB) | ok (4.216%) |
| 24 | Drawn Curve | All Bands Drive=1 | ok (-2.6 dB) | ok (10.070%) |
| 25 | Curve Point 1 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (9.512%) |
| 26 | Curve Point 2 | Drawn Curve=1, All Bands Drive=1 | ok (0.1 dB) | ok (10.401%) |
| 27 | Curve Point 3 | Drawn Curve=1, All Bands Drive=1 | ok (-2.4 dB) | ok (10.112%) |
| 28 | Curve Point 4 | Drawn Curve=1, All Bands Drive=1 | ok (-2.3 dB) | ok (9.974%) |
| 29 | Curve Point 5 | Drawn Curve=1, All Bands Drive=1 | ok (-2.8 dB) | ok (9.856%) |
| 30 | Curve Point 6 | Drawn Curve=1, All Bands Drive=1 | ok (-3.0 dB) | ok (9.868%) |
| 31 | Curve Point 7 | Drawn Curve=1, All Bands Drive=1 | ok (-2.1 dB) | ok (9.716%) |
| 32 | Curve Point 8 | Drawn Curve=1, All Bands Drive=1 | ok (-0.5 dB) | ok (9.237%) |
| 33 | Curve Point 9 | Drawn Curve=1, All Bands Drive=1 | ok (0.6 dB) | ok (9.064%) |
| 34 | Curve Point 10 | Drawn Curve=1, All Bands Drive=1 | ok (2.0 dB) | ok (9.698%) |
| 35 | Curve Point 11 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (10.617%) |
| 36 | Curve Point 12 | Drawn Curve=1, All Bands Drive=1 | ok (2.9 dB) | ok (10.102%) |
| 37 | Curve Point 13 | Drawn Curve=1, All Bands Drive=1 | ok (2.1 dB) | ok (10.394%) |
| 38 | Curve Point 14 | Drawn Curve=1, All Bands Drive=1 | ok (-1.1 dB) | ok (10.537%) |
| 39 | Curve Point 15 | Drawn Curve=1, All Bands Drive=1 | ok (-0.6 dB) | ok (9.190%) |
| 40 | Curve Point 16 | Drawn Curve=1, All Bands Drive=1 | ok (3.2 dB) | ok (10.378%) |
| 41 | All Bands Drive |  | ok (-1.4 dB) | ok (10.437%) |
| 42 | All Bands Ceiling |  | ok (-3.2 dB) | ok (2.840%) |
| 43 | All Bands Gain |  | ok (-1.7 dB) | ok (7.853%) |
| 44 | Band 1 Drive | Bands Open=1 | ok (-0.9 dB) | ok (6.703%) |
| 45 | Band 1 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-4.5 dB) | ok (6.022%) |
| 46 | Band 1 Gain | Bands Open=1 | ok (1.0 dB) | ok (6.201%) |
| 47 | Band 1 Solo |  | ok (-5.0 dB) | ok (4.009%) |
| 48 | Band 1 Mute |  | ok (-11.8 dB) | ok (3.411%) |
| 49 | Band 1 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (8.652%) |
| 50 | Band 2 Drive | Bands Open=1 | ok (-1.0 dB) | ok (7.550%) |
| 51 | Band 2 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-2.2 dB) | ok (6.116%) |
| 52 | Band 2 Gain | Bands Open=1 | ok (1.1 dB) | ok (6.825%) |
| 53 | Band 2 Solo |  | ok (-0.6 dB) | ok (3.246%) |
| 54 | Band 2 Mute |  | ok (-9.7 dB) | ok (2.625%) |
| 55 | Band 2 Bypass | All Bands Drive=1 | ok (-0.4 dB) | ok (9.231%) |
| 56 | Band 3 Drive | Bands Open=1 | ok (-0.9 dB) | ok (7.267%) |
| 57 | Band 3 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-2.3 dB) | ok (6.214%) |
| 58 | Band 3 Gain | Bands Open=1 | ok (1.0 dB) | ok (7.126%) |
| 59 | Band 3 Solo |  | ok (-1.6 dB) | ok (3.497%) |
| 60 | Band 3 Mute |  | ok (-10.8 dB) | ok (3.402%) |
| 61 | Band 3 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (9.842%) |
| 62 | Band 4 Drive | Bands Open=1 | ok (1.4 dB) | ok (7.926%) |
| 63 | Band 4 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-3.0 dB) | ok (5.761%) |
| 64 | Band 4 Gain | Bands Open=1 | ok (0.1 dB) | ok (6.599%) |
| 65 | Band 4 Solo |  | ok (-3.5 dB) | ok (3.618%) |
| 66 | Band 4 Mute |  | ok (-8.4 dB) | ok (2.861%) |
| 67 | Band 4 Bypass | All Bands Drive=1 | ok (1.8 dB) | ok (8.572%) |
| 68 | Band 5 Drive | Bands=0.8, Bands Open=1 | ok (-0.6 dB) | ok (9.297%) |
| 69 | Band 5 Ceiling | Bands=0.8, Bands Open=1, All Bands Ceiling=0.5 | ok (-3.3 dB) | ok (8.118%) |
| 70 | Band 5 Gain | Bands=0.8, Bands Open=1 | ok (-0.2 dB) | ok (9.788%) |
| 71 | Band 5 Solo | Bands=0.8, Bands Open=0 | ok (-3.7 dB) | ok (5.389%) |
| 72 | Band 5 Mute | Bands=0.8, Bands Open=0 | ok (-11.9 dB) | ok (5.156%) |
| 73 | Band 5 Bypass | Bands=0.8, All Bands Drive=1 | ok (1.0 dB) | ok (11.424%) |
| 74 | Band 6 Drive | Bands=1, Bands Open=1 | ok (-0.9 dB) | ok (8.093%) |
| 75 | Band 6 Ceiling | Bands=1, Bands Open=1, All Bands Ceiling=0.5 | ok (-4.4 dB) | ok (7.828%) |
| 76 | Band 6 Gain | Bands=1, Bands Open=1 | ok (-0.3 dB) | ok (9.081%) |
| 77 | Band 6 Solo | Bands=1, Bands Open=0 | ok (-3.0 dB) | ok (5.931%) |
| 78 | Band 6 Mute | Bands=1, Bands Open=0 | ok (-12.1 dB) | ok (5.499%) |
| 79 | Band 6 Bypass | Bands=1, All Bands Drive=1 | ok (0.0 dB) | ok (11.554%) |
| 80 | Bypass |  | ok (-1.7 dB) | excused (3.112%) |
| 81 | Bypass |  | ok (-1.7 dB) | excused (2.241%) |
| 82 | Wet |  | ok (-1.7 dB) | excused (2.396%) |
| 83 | Delta |  | ok (-3.5 dB) | excused (2.261%) |
