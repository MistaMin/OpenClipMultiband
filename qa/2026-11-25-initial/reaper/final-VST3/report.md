# Plugin QA: VST3: OpenClip Multiband

| Check | Status | Detail |
|---|---|---|
| loads | PASS | VST3: OpenClip Multiband (OpenGrid), 84 parameters |
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
| 0 | Input Gain |  | ok (-1.4 dB) | ok (8.275%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (5.438%) |
| 2 | Zero Latency |  | ok (-1.0 dB) | ok (3.635%) |
| 3 | Auto Gain |  | ok (-6.0 dB) | ok (2.902%) |
| 4 | Delta |  | ok (-4.0 dB) | ok (2.986%) |
| 5 | Bands |  | ok (-1.7 dB) | ok (4.187%) |
| 6 | Crossover Slope |  | ok (1.4 dB) | ok (2.422%) |
| 7 | Crossover 1 |  | ok (-1.2 dB) | ok (3.385%) |
| 8 | Crossover 2 |  | ok (-1.4 dB) | ok (3.066%) |
| 9 | Crossover 3 |  | ok (-1.5 dB) | ok (3.132%) |
| 10 | Crossover 4 | Bands=0.8 | ok (-2.2 dB) | ok (4.937%) |
| 11 | Crossover 5 | Bands=1 | ok (-1.9 dB) | ok (5.222%) |
| 12 | Algorithm | All Bands Drive=1 | ok (2.7 dB) | ok (9.444%) |
| 13 | Shape | All Bands Drive=1 | ok (-7.5 dB) | ok (10.510%) |
| 14 | Character | All Bands Drive=1 | ok (-25.7 dB) | ok (9.860%) |
| 15 | Single/Multi Blend | All Bands Drive=1 | ok (-1.4 dB) | ok (9.921%) |
| 16 | Dry/Wet Mix | All Bands Drive=1 | ok (-1.4 dB) | ok (9.745%) |
| 17 | Single Drive | Single/Multi Blend=0 | ok (-1.9 dB) | ok (7.673%) |
| 18 | Single Ceiling | Single/Multi Blend=0 | ok (-3.5 dB) | ok (2.960%) |
| 19 | Output Ceiling On | Input Gain=1 | ok (2.5 dB) | ok (8.676%) |
| 20 | Output Ceiling | Input Gain=1 | ok (-1.5 dB) | ok (7.814%) |
| 21 | Anti-Alias | All Bands Drive=1 | ok (3.7 dB) | ok (5.550%) |
| 22 | Oversampling | All Bands Drive=1 | ok (4.0 dB) | ok (9.396%) |
| 23 | Bands Open |  | excused (-3.7 dB) | ok (5.223%) |
| 24 | Drawn Curve | All Bands Drive=1 | ok (-2.6 dB) | ok (9.425%) |
| 25 | Curve Point 1 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (10.733%) |
| 26 | Curve Point 2 | Drawn Curve=1, All Bands Drive=1 | ok (0.1 dB) | ok (10.496%) |
| 27 | Curve Point 3 | Drawn Curve=1, All Bands Drive=1 | ok (-2.4 dB) | ok (9.772%) |
| 28 | Curve Point 4 | Drawn Curve=1, All Bands Drive=1 | ok (-2.3 dB) | ok (9.938%) |
| 29 | Curve Point 5 | Drawn Curve=1, All Bands Drive=1 | ok (-2.8 dB) | ok (9.879%) |
| 30 | Curve Point 6 | Drawn Curve=1, All Bands Drive=1 | ok (-3.0 dB) | ok (10.224%) |
| 31 | Curve Point 7 | Drawn Curve=1, All Bands Drive=1 | ok (-2.1 dB) | ok (10.908%) |
| 32 | Curve Point 8 | Drawn Curve=1, All Bands Drive=1 | ok (-0.5 dB) | ok (10.108%) |
| 33 | Curve Point 9 | Drawn Curve=1, All Bands Drive=1 | ok (0.6 dB) | ok (9.957%) |
| 34 | Curve Point 10 | Drawn Curve=1, All Bands Drive=1 | ok (2.0 dB) | ok (10.508%) |
| 35 | Curve Point 11 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (9.320%) |
| 36 | Curve Point 12 | Drawn Curve=1, All Bands Drive=1 | ok (2.9 dB) | ok (10.401%) |
| 37 | Curve Point 13 | Drawn Curve=1, All Bands Drive=1 | ok (2.1 dB) | ok (10.815%) |
| 38 | Curve Point 14 | Drawn Curve=1, All Bands Drive=1 | ok (-1.1 dB) | ok (9.847%) |
| 39 | Curve Point 15 | Drawn Curve=1, All Bands Drive=1 | ok (-0.6 dB) | ok (10.479%) |
| 40 | Curve Point 16 | Drawn Curve=1, All Bands Drive=1 | ok (3.2 dB) | ok (10.047%) |
| 41 | All Bands Drive |  | ok (-1.4 dB) | ok (9.859%) |
| 42 | All Bands Ceiling |  | ok (-3.2 dB) | ok (3.168%) |
| 43 | All Bands Gain |  | ok (-1.7 dB) | ok (7.912%) |
| 44 | Band 1 Drive | Bands Open=1 | ok (-0.9 dB) | ok (6.462%) |
| 45 | Band 1 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-4.5 dB) | ok (5.821%) |
| 46 | Band 1 Gain | Bands Open=1 | ok (1.0 dB) | ok (6.078%) |
| 47 | Band 1 Solo |  | ok (-5.0 dB) | ok (3.334%) |
| 48 | Band 1 Mute |  | ok (-11.8 dB) | ok (2.697%) |
| 49 | Band 1 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (8.784%) |
| 50 | Band 2 Drive | Bands Open=1 | ok (-1.0 dB) | ok (7.537%) |
| 51 | Band 2 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-2.2 dB) | ok (5.522%) |
| 52 | Band 2 Gain | Bands Open=1 | ok (1.1 dB) | ok (6.865%) |
| 53 | Band 2 Solo |  | ok (-0.6 dB) | ok (3.381%) |
| 54 | Band 2 Mute |  | ok (-9.7 dB) | ok (2.943%) |
| 55 | Band 2 Bypass | All Bands Drive=1 | ok (-0.4 dB) | ok (9.406%) |
| 56 | Band 3 Drive | Bands Open=1 | ok (-0.9 dB) | ok (7.409%) |
| 57 | Band 3 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-2.3 dB) | ok (5.959%) |
| 58 | Band 3 Gain | Bands Open=1 | ok (1.0 dB) | ok (6.954%) |
| 59 | Band 3 Solo |  | ok (-1.6 dB) | ok (3.559%) |
| 60 | Band 3 Mute |  | ok (-10.8 dB) | ok (2.983%) |
| 61 | Band 3 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (9.019%) |
| 62 | Band 4 Drive | Bands Open=1 | ok (1.4 dB) | ok (8.211%) |
| 63 | Band 4 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-3.0 dB) | ok (5.493%) |
| 64 | Band 4 Gain | Bands Open=1 | ok (0.1 dB) | ok (6.866%) |
| 65 | Band 4 Solo |  | ok (-3.5 dB) | ok (3.179%) |
| 66 | Band 4 Mute |  | ok (-8.4 dB) | ok (3.033%) |
| 67 | Band 4 Bypass | All Bands Drive=1 | ok (1.8 dB) | ok (8.148%) |
| 68 | Band 5 Drive | Bands=0.8, Bands Open=1 | ok (-0.6 dB) | ok (9.098%) |
| 69 | Band 5 Ceiling | Bands=0.8, Bands Open=1, All Bands Ceiling=0.5 | ok (-3.3 dB) | ok (7.679%) |
| 70 | Band 5 Gain | Bands=0.8, Bands Open=1 | ok (-0.2 dB) | ok (9.372%) |
| 71 | Band 5 Solo | Bands=0.8, Bands Open=0 | ok (-3.7 dB) | ok (5.498%) |
| 72 | Band 5 Mute | Bands=0.8, Bands Open=0 | ok (-11.9 dB) | ok (5.408%) |
| 73 | Band 5 Bypass | Bands=0.8, All Bands Drive=1 | ok (1.0 dB) | ok (11.351%) |
| 74 | Band 6 Drive | Bands=1, Bands Open=1 | ok (-0.9 dB) | ok (8.560%) |
| 75 | Band 6 Ceiling | Bands=1, Bands Open=1, All Bands Ceiling=0.5 | ok (-4.4 dB) | ok (7.587%) |
| 76 | Band 6 Gain | Bands=1, Bands Open=1 | ok (-0.3 dB) | ok (8.474%) |
| 77 | Band 6 Solo | Bands=1, Bands Open=0 | ok (-3.0 dB) | ok (5.573%) |
| 78 | Band 6 Mute | Bands=1, Bands Open=0 | ok (-12.1 dB) | ok (5.117%) |
| 79 | Band 6 Bypass | Bands=1, All Bands Drive=1 | ok (0.0 dB) | ok (11.216%) |
| 80 | Bypass |  | ok (-1.7 dB) | excused (3.663%) |
| 81 | Bypass |  | ok (-1.7 dB) | excused (2.416%) |
| 82 | Wet |  | ok (-1.7 dB) | excused (2.721%) |
| 83 | Delta |  | ok (-3.5 dB) | excused (2.888%) |
