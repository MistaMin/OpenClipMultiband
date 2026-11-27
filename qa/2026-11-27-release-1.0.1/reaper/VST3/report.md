# Plugin QA: VST3: OpenClip Multiband

| Check | Status | Detail |
|---|---|---|
| loads | PASS | VST3: OpenClip Multiband (OpenGrid), 84 parameters |
| finite output | PASS | peak -4.1 dBFS |
| not silent | PASS | rms -14.9 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1080x821, 0.5% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-1.4 dB) | ok (7.928%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (5.063%) |
| 2 | Zero Latency |  | ok (-6.0 dB) | ok (3.470%) |
| 3 | Auto Gain |  | ok (-6.0 dB) | ok (3.129%) |
| 4 | Delta |  | ok (-4.0 dB) | ok (2.518%) |
| 5 | Bands |  | ok (-1.7 dB) | ok (4.715%) |
| 6 | Crossover Slope |  | ok (1.4 dB) | ok (3.502%) |
| 7 | Crossover 1 |  | ok (-1.2 dB) | ok (3.311%) |
| 8 | Crossover 2 |  | ok (-1.4 dB) | ok (3.515%) |
| 9 | Crossover 3 |  | ok (-1.5 dB) | ok (2.954%) |
| 10 | Crossover 4 | Bands=0.8 | ok (-2.2 dB) | ok (4.696%) |
| 11 | Crossover 5 | Bands=1 | ok (-1.9 dB) | ok (4.614%) |
| 12 | Algorithm | All Bands Drive=1 | ok (3.8 dB) | ok (10.213%) |
| 13 | Shape | All Bands Drive=1 | ok (-5.3 dB) | ok (9.883%) |
| 14 | Character | All Bands Drive=1 | ok (-13.8 dB) | ok (9.770%) |
| 15 | Single/Multi Blend | All Bands Drive=1 | ok (-1.4 dB) | ok (9.947%) |
| 16 | Dry/Wet Mix | All Bands Drive=1 | ok (-1.4 dB) | ok (10.385%) |
| 17 | Single Drive | Single/Multi Blend=0 | ok (-1.9 dB) | ok (7.671%) |
| 18 | Single Ceiling | Single/Multi Blend=0 | ok (-3.5 dB) | ok (3.119%) |
| 19 | Output Ceiling On | Input Gain=1 | ok (2.3 dB) | ok (8.763%) |
| 20 | Output Ceiling | Input Gain=1 | ok (-1.1 dB) | ok (8.361%) |
| 21 | Anti-Alias | All Bands Drive=1 | ok (3.7 dB) | ok (4.693%) |
| 22 | Oversampling | All Bands Drive=1 | ok (4.0 dB) | ok (9.186%) |
| 23 | Bands Open |  | excused (-3.7 dB) | ok (4.860%) |
| 24 | Drawn Curve | All Bands Drive=1 | ok (-2.6 dB) | ok (9.916%) |
| 25 | Curve Point 1 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (10.924%) |
| 26 | Curve Point 2 | Drawn Curve=1, All Bands Drive=1 | ok (0.1 dB) | ok (10.874%) |
| 27 | Curve Point 3 | Drawn Curve=1, All Bands Drive=1 | ok (-2.4 dB) | ok (10.577%) |
| 28 | Curve Point 4 | Drawn Curve=1, All Bands Drive=1 | ok (-2.3 dB) | ok (9.816%) |
| 29 | Curve Point 5 | Drawn Curve=1, All Bands Drive=1 | ok (-2.8 dB) | ok (10.554%) |
| 30 | Curve Point 6 | Drawn Curve=1, All Bands Drive=1 | ok (-3.0 dB) | ok (10.895%) |
| 31 | Curve Point 7 | Drawn Curve=1, All Bands Drive=1 | ok (-2.1 dB) | ok (10.282%) |
| 32 | Curve Point 8 | Drawn Curve=1, All Bands Drive=1 | ok (-0.5 dB) | ok (10.103%) |
| 33 | Curve Point 9 | Drawn Curve=1, All Bands Drive=1 | ok (0.6 dB) | ok (10.092%) |
| 34 | Curve Point 10 | Drawn Curve=1, All Bands Drive=1 | ok (2.0 dB) | ok (10.650%) |
| 35 | Curve Point 11 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (9.984%) |
| 36 | Curve Point 12 | Drawn Curve=1, All Bands Drive=1 | ok (2.9 dB) | ok (10.514%) |
| 37 | Curve Point 13 | Drawn Curve=1, All Bands Drive=1 | ok (2.1 dB) | ok (10.062%) |
| 38 | Curve Point 14 | Drawn Curve=1, All Bands Drive=1 | ok (-1.1 dB) | ok (10.640%) |
| 39 | Curve Point 15 | Drawn Curve=1, All Bands Drive=1 | ok (-0.6 dB) | ok (10.635%) |
| 40 | Curve Point 16 | Drawn Curve=1, All Bands Drive=1 | ok (3.2 dB) | ok (10.219%) |
| 41 | All Bands Drive |  | ok (-1.4 dB) | ok (10.034%) |
| 42 | All Bands Ceiling |  | ok (-3.2 dB) | ok (3.388%) |
| 43 | All Bands Gain |  | ok (-1.7 dB) | ok (7.645%) |
| 44 | Band 1 Drive | Bands Open=1 | ok (-0.9 dB) | ok (6.803%) |
| 45 | Band 1 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-13.3 dB) | ok (5.627%) |
| 46 | Band 1 Gain | Bands Open=1 | ok (1.0 dB) | ok (5.761%) |
| 47 | Band 1 Solo |  | ok (-5.7 dB) | ok (3.727%) |
| 48 | Band 1 Mute |  | ok (-11.8 dB) | ok (2.750%) |
| 49 | Band 1 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (9.214%) |
| 50 | Band 2 Drive | Bands Open=1 | ok (-1.0 dB) | ok (7.857%) |
| 51 | Band 2 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-12.8 dB) | ok (5.967%) |
| 52 | Band 2 Gain | Bands Open=1 | ok (1.1 dB) | ok (6.712%) |
| 53 | Band 2 Solo |  | ok (-4.9 dB) | ok (3.829%) |
| 54 | Band 2 Mute |  | ok (-9.7 dB) | ok (2.434%) |
| 55 | Band 2 Bypass | All Bands Drive=1 | ok (-0.4 dB) | ok (9.234%) |
| 56 | Band 3 Drive | Bands Open=1 | ok (-0.9 dB) | ok (7.315%) |
| 57 | Band 3 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-12.9 dB) | ok (5.390%) |
| 58 | Band 3 Gain | Bands Open=1 | ok (1.0 dB) | ok (6.841%) |
| 59 | Band 3 Solo |  | ok (-4.5 dB) | ok (3.643%) |
| 60 | Band 3 Mute |  | ok (-10.8 dB) | ok (3.279%) |
| 61 | Band 3 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (9.168%) |
| 62 | Band 4 Drive | Bands Open=1 | ok (1.4 dB) | ok (8.291%) |
| 63 | Band 4 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-10.7 dB) | ok (5.803%) |
| 64 | Band 4 Gain | Bands Open=1 | ok (0.1 dB) | ok (6.627%) |
| 65 | Band 4 Solo |  | ok (-6.6 dB) | ok (2.781%) |
| 66 | Band 4 Mute |  | ok (-8.4 dB) | ok (2.886%) |
| 67 | Band 4 Bypass | All Bands Drive=1 | ok (1.8 dB) | ok (8.772%) |
| 68 | Band 5 Drive | Bands=0.8, Bands Open=1 | ok (-0.6 dB) | ok (9.860%) |
| 69 | Band 5 Ceiling | Bands=0.8, Bands Open=1, All Bands Ceiling=0.5 | ok (-13.0 dB) | ok (7.440%) |
| 70 | Band 5 Gain | Bands=0.8, Bands Open=1 | ok (-0.2 dB) | ok (8.563%) |
| 71 | Band 5 Solo | Bands=0.8, Bands Open=0 | ok (-5.7 dB) | ok (5.115%) |
| 72 | Band 5 Mute | Bands=0.8, Bands Open=0 | ok (-11.9 dB) | ok (4.683%) |
| 73 | Band 5 Bypass | Bands=0.8, All Bands Drive=1 | ok (1.0 dB) | ok (10.951%) |
| 74 | Band 6 Drive | Bands=1, Bands Open=1 | ok (-0.9 dB) | ok (8.841%) |
| 75 | Band 6 Ceiling | Bands=1, Bands Open=1, All Bands Ceiling=0.5 | ok (-13.1 dB) | ok (7.923%) |
| 76 | Band 6 Gain | Bands=1, Bands Open=1 | ok (-0.3 dB) | ok (8.826%) |
| 77 | Band 6 Solo | Bands=1, Bands Open=0 | ok (-4.9 dB) | ok (5.623%) |
| 78 | Band 6 Mute | Bands=1, Bands Open=0 | ok (-12.1 dB) | ok (4.805%) |
| 79 | Band 6 Bypass | Bands=1, All Bands Drive=1 | ok (0.0 dB) | ok (11.750%) |
| 80 | Bypass |  | ok (-1.7 dB) | excused (2.842%) |
| 81 | Bypass |  | ok (-1.7 dB) | excused (2.013%) |
| 82 | Wet |  | ok (-1.7 dB) | excused (2.051%) |
| 83 | Delta |  | ok (-3.5 dB) | excused (1.901%) |
