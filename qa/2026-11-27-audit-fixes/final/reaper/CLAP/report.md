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
| 0 | Input Gain |  | ok (-1.4 dB) | ok (8.411%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (5.407%) |
| 2 | Zero Latency |  | ok (-0.2 dB) | ok (4.358%) |
| 3 | Auto Gain |  | ok (0.9 dB) | ok (2.437%) |
| 4 | Delta |  | ok (-4.0 dB) | ok (3.594%) |
| 5 | Bands |  | ok (-1.7 dB) | ok (5.251%) |
| 6 | Crossover Slope |  | ok (1.4 dB) | ok (3.276%) |
| 7 | Crossover 1 |  | ok (-1.2 dB) | ok (4.168%) |
| 8 | Crossover 2 |  | ok (-1.4 dB) | ok (4.041%) |
| 9 | Crossover 3 |  | ok (-1.5 dB) | ok (3.784%) |
| 10 | Crossover 4 | Bands=0.8 | ok (-2.2 dB) | ok (5.192%) |
| 11 | Crossover 5 | Bands=1 | ok (-1.9 dB) | ok (4.772%) |
| 12 | Algorithm | All Bands Drive=1 | ok (3.8 dB) | ok (10.382%) |
| 13 | Shape | All Bands Drive=1 | ok (-5.3 dB) | ok (9.719%) |
| 14 | Character | All Bands Drive=1 | ok (-25.7 dB) | ok (9.879%) |
| 15 | Single/Multi Blend | All Bands Drive=1 | ok (-1.4 dB) | ok (9.617%) |
| 16 | Dry/Wet Mix | All Bands Drive=1 | ok (-1.4 dB) | ok (9.607%) |
| 17 | Single Drive | Single/Multi Blend=0 | ok (-1.9 dB) | ok (8.643%) |
| 18 | Single Ceiling | Single/Multi Blend=0 | ok (-3.5 dB) | ok (2.346%) |
| 19 | Output Ceiling On | Input Gain=1 | ok (2.5 dB) | ok (8.449%) |
| 20 | Output Ceiling | Input Gain=1 | ok (-1.5 dB) | ok (8.609%) |
| 21 | Anti-Alias | All Bands Drive=1 | ok (3.7 dB) | ok (5.352%) |
| 22 | Oversampling | All Bands Drive=1 | ok (4.3 dB) | ok (10.245%) |
| 23 | Bands Open |  | excused (-0.3 dB) | ok (5.572%) |
| 24 | Drawn Curve | All Bands Drive=1 | ok (-2.6 dB) | ok (9.836%) |
| 25 | Curve Point 1 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (11.030%) |
| 26 | Curve Point 2 | Drawn Curve=1, All Bands Drive=1 | ok (0.1 dB) | ok (10.295%) |
| 27 | Curve Point 3 | Drawn Curve=1, All Bands Drive=1 | ok (-2.4 dB) | ok (10.764%) |
| 28 | Curve Point 4 | Drawn Curve=1, All Bands Drive=1 | ok (-2.3 dB) | ok (10.121%) |
| 29 | Curve Point 5 | Drawn Curve=1, All Bands Drive=1 | ok (-2.8 dB) | ok (10.017%) |
| 30 | Curve Point 6 | Drawn Curve=1, All Bands Drive=1 | ok (-3.0 dB) | ok (9.527%) |
| 31 | Curve Point 7 | Drawn Curve=1, All Bands Drive=1 | ok (-2.1 dB) | ok (10.104%) |
| 32 | Curve Point 8 | Drawn Curve=1, All Bands Drive=1 | ok (-0.5 dB) | ok (9.673%) |
| 33 | Curve Point 9 | Drawn Curve=1, All Bands Drive=1 | ok (0.6 dB) | ok (9.323%) |
| 34 | Curve Point 10 | Drawn Curve=1, All Bands Drive=1 | ok (2.0 dB) | ok (9.156%) |
| 35 | Curve Point 11 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (9.938%) |
| 36 | Curve Point 12 | Drawn Curve=1, All Bands Drive=1 | ok (2.9 dB) | ok (10.448%) |
| 37 | Curve Point 13 | Drawn Curve=1, All Bands Drive=1 | ok (2.1 dB) | ok (9.318%) |
| 38 | Curve Point 14 | Drawn Curve=1, All Bands Drive=1 | ok (-1.1 dB) | ok (9.890%) |
| 39 | Curve Point 15 | Drawn Curve=1, All Bands Drive=1 | ok (-0.6 dB) | ok (9.585%) |
| 40 | Curve Point 16 | Drawn Curve=1, All Bands Drive=1 | ok (3.2 dB) | ok (10.007%) |
| 41 | All Bands Drive |  | ok (-1.4 dB) | ok (9.735%) |
| 42 | All Bands Ceiling |  | ok (-3.2 dB) | ok (3.596%) |
| 43 | All Bands Gain |  | ok (-1.7 dB) | ok (6.836%) |
| 44 | Band 1 Drive | Bands Open=1 | ok (-0.9 dB) | ok (6.452%) |
| 45 | Band 1 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-4.5 dB) | ok (6.079%) |
| 46 | Band 1 Gain | Bands Open=1 | ok (1.0 dB) | ok (6.840%) |
| 47 | Band 1 Solo |  | ok (-5.0 dB) | ok (3.655%) |
| 48 | Band 1 Mute |  | ok (-11.8 dB) | ok (3.091%) |
| 49 | Band 1 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (8.978%) |
| 50 | Band 2 Drive | Bands Open=1 | ok (-1.0 dB) | ok (7.542%) |
| 51 | Band 2 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-2.2 dB) | ok (6.213%) |
| 52 | Band 2 Gain | Bands Open=1 | ok (1.1 dB) | ok (7.411%) |
| 53 | Band 2 Solo |  | ok (-0.6 dB) | ok (3.858%) |
| 54 | Band 2 Mute |  | ok (-9.7 dB) | ok (2.409%) |
| 55 | Band 2 Bypass | All Bands Drive=1 | ok (-0.4 dB) | ok (9.777%) |
| 56 | Band 3 Drive | Bands Open=1 | ok (-0.9 dB) | ok (7.188%) |
| 57 | Band 3 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-2.3 dB) | ok (6.220%) |
| 58 | Band 3 Gain | Bands Open=1 | ok (1.0 dB) | ok (7.238%) |
| 59 | Band 3 Solo |  | ok (-1.6 dB) | ok (3.264%) |
| 60 | Band 3 Mute |  | ok (-10.8 dB) | ok (2.488%) |
| 61 | Band 3 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (8.936%) |
| 62 | Band 4 Drive | Bands Open=1 | ok (1.4 dB) | ok (7.818%) |
| 63 | Band 4 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-3.0 dB) | ok (5.223%) |
| 64 | Band 4 Gain | Bands Open=1 | ok (0.1 dB) | ok (6.423%) |
| 65 | Band 4 Solo |  | ok (-3.5 dB) | ok (2.808%) |
| 66 | Band 4 Mute |  | ok (-8.4 dB) | ok (3.247%) |
| 67 | Band 4 Bypass | All Bands Drive=1 | ok (1.8 dB) | ok (8.414%) |
| 68 | Band 5 Drive | Bands=0.8, Bands Open=1 | ok (-0.6 dB) | ok (8.847%) |
| 69 | Band 5 Ceiling | Bands=0.8, Bands Open=1, All Bands Ceiling=0.5 | ok (-3.3 dB) | ok (7.728%) |
| 70 | Band 5 Gain | Bands=0.8, Bands Open=1 | ok (-0.2 dB) | ok (9.457%) |
| 71 | Band 5 Solo | Bands=0.8, Bands Open=0 | ok (-3.7 dB) | ok (5.542%) |
| 72 | Band 5 Mute | Bands=0.8, Bands Open=0 | ok (-11.9 dB) | ok (4.744%) |
| 73 | Band 5 Bypass | Bands=0.8, All Bands Drive=1 | ok (1.0 dB) | ok (10.918%) |
| 74 | Band 6 Drive | Bands=1, Bands Open=1 | ok (-0.9 dB) | ok (7.918%) |
| 75 | Band 6 Ceiling | Bands=1, Bands Open=1, All Bands Ceiling=0.5 | ok (-4.4 dB) | ok (8.502%) |
| 76 | Band 6 Gain | Bands=1, Bands Open=1 | ok (-0.3 dB) | ok (8.140%) |
| 77 | Band 6 Solo | Bands=1, Bands Open=0 | ok (-3.0 dB) | ok (5.617%) |
| 78 | Band 6 Mute | Bands=1, Bands Open=0 | ok (-12.1 dB) | ok (5.694%) |
| 79 | Band 6 Bypass | Bands=1, All Bands Drive=1 | ok (0.0 dB) | ok (11.270%) |
| 80 | Bypass |  | ok (-1.7 dB) | ok (2.946%) |
| 81 | Wet |  | ok (-1.7 dB) | ok (2.353%) |
| 82 | Delta |  | ok (-3.5 dB) | ok (3.428%) |
