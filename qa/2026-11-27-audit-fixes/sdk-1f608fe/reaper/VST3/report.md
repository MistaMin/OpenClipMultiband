# Plugin QA: VST3: OpenClip Multiband

| Check | Status | Detail |
|---|---|---|
| loads | PASS | VST3: OpenClip Multiband (OpenGrid), 84 parameters |
| finite output | PASS | peak -4.1 dBFS |
| not silent | PASS | rms -14.9 dBFS |
| deterministic render | FAIL | repeat differs by -5.5 dB |
| editor opens | PASS | 1080x821, 0.0% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | skipped (render not deterministic)  | ok (0.048%) |
| 1 | Output Gain |  | skipped (render not deterministic)  | ok (0.048%) |
| 2 | Zero Latency |  | skipped (render not deterministic)  | ok (0.282%) |
| 3 | Auto Gain |  | skipped (render not deterministic)  | ok (0.299%) |
| 4 | Delta |  | skipped (render not deterministic)  | ok (0.178%) |
| 5 | Bands |  | skipped (render not deterministic)  | ok (0.847%) |
| 6 | Crossover Slope |  | skipped (render not deterministic)  | ok (0.198%) |
| 7 | Crossover 1 |  | skipped (render not deterministic)  | ok (0.147%) |
| 8 | Crossover 2 |  | skipped (render not deterministic)  | ok (0.079%) |
| 9 | Crossover 3 |  | skipped (render not deterministic)  | ok (0.102%) |
| 10 | Crossover 4 | Bands=0.8 | skipped (render not deterministic)  | ok (1.455%) |
| 11 | Crossover 5 | Bands=1 | skipped (render not deterministic)  | ok (1.488%) |
| 12 | Algorithm | All Bands Drive=1 | skipped (render not deterministic)  | ok (0.316%) |
| 13 | Shape | All Bands Drive=1 | skipped (render not deterministic)  | ok (0.357%) |
| 14 | Character | All Bands Drive=1 | skipped (render not deterministic)  | ok (0.225%) |
| 15 | Single/Multi Blend | All Bands Drive=1 | skipped (render not deterministic)  | ok (0.177%) |
| 16 | Dry/Wet Mix | All Bands Drive=1 | skipped (render not deterministic)  | ok (0.177%) |
| 17 | Single Drive | Single/Multi Blend=0 | skipped (render not deterministic)  | ok (0.089%) |
| 18 | Single Ceiling | Single/Multi Blend=0 | skipped (render not deterministic)  | ok (0.089%) |
| 19 | Output Ceiling On | Input Gain=1 | skipped (render not deterministic)  | ok (0.203%) |
| 20 | Output Ceiling | Input Gain=1 | skipped (render not deterministic)  | ok (0.277%) |
| 21 | Anti-Alias | All Bands Drive=1 | skipped (render not deterministic)  | ok (0.166%) |
| 22 | Oversampling | All Bands Drive=1 | skipped (render not deterministic)  | ok (0.312%) |
| 23 | Bands Open |  | skipped (render not deterministic)  | ok (1.866%) |
| 24 | Drawn Curve | All Bands Drive=1 | skipped (render not deterministic)  | ok (0.368%) |
| 25 | Curve Point 1 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | ok (0.501%) |
| 26 | Curve Point 2 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | ok (0.476%) |
| 27 | Curve Point 3 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | ok (0.454%) |
| 28 | Curve Point 4 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | ok (0.439%) |
| 29 | Curve Point 5 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | ok (0.455%) |
| 30 | Curve Point 6 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | ok (0.475%) |
| 31 | Curve Point 7 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | ok (0.489%) |
| 32 | Curve Point 8 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | ok (0.476%) |
| 33 | Curve Point 9 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | ok (0.484%) |
| 34 | Curve Point 10 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | ok (0.510%) |
| 35 | Curve Point 11 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | ok (0.494%) |
| 36 | Curve Point 12 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | ok (0.520%) |
| 37 | Curve Point 13 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | ok (0.527%) |
| 38 | Curve Point 14 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | ok (0.538%) |
| 39 | Curve Point 15 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | ok (0.543%) |
| 40 | Curve Point 16 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | ok (0.472%) |
| 41 | All Bands Drive |  | skipped (render not deterministic)  | ok (0.133%) |
| 42 | All Bands Ceiling |  | skipped (render not deterministic)  | ok (0.727%) |
| 43 | All Bands Gain |  | skipped (render not deterministic)  | ok (0.086%) |
| 44 | Band 1 Drive | Bands Open=1 | skipped (render not deterministic)  | ok (1.873%) |
| 45 | Band 1 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | skipped (render not deterministic)  | ok (2.408%) |
| 46 | Band 1 Gain | Bands Open=1 | skipped (render not deterministic)  | ok (1.876%) |
| 47 | Band 1 Solo |  | skipped (render not deterministic)  | ok (0.030%) |
| 48 | Band 1 Mute |  | skipped (render not deterministic)  | ok (0.030%) |
| 49 | Band 1 Bypass | All Bands Drive=1 | skipped (render not deterministic)  | ok (0.163%) |
| 50 | Band 2 Drive | Bands Open=1 | skipped (render not deterministic)  | ok (1.875%) |
| 51 | Band 2 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | skipped (render not deterministic)  | ok (2.504%) |
| 52 | Band 2 Gain | Bands Open=1 | skipped (render not deterministic)  | ok (1.873%) |
| 53 | Band 2 Solo |  | skipped (render not deterministic)  | ok (0.030%) |
| 54 | Band 2 Mute |  | skipped (render not deterministic)  | ok (0.030%) |
| 55 | Band 2 Bypass | All Bands Drive=1 | skipped (render not deterministic)  | ok (0.163%) |
| 56 | Band 3 Drive | Bands Open=1 | skipped (render not deterministic)  | ok (1.875%) |
| 57 | Band 3 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | skipped (render not deterministic)  | ok (2.401%) |
| 58 | Band 3 Gain | Bands Open=1 | skipped (render not deterministic)  | ok (1.874%) |
| 59 | Band 3 Solo |  | skipped (render not deterministic)  | ok (0.030%) |
| 60 | Band 3 Mute |  | skipped (render not deterministic)  | ok (0.030%) |
| 61 | Band 3 Bypass | All Bands Drive=1 | skipped (render not deterministic)  | ok (0.163%) |
| 62 | Band 4 Drive | Bands Open=1 | skipped (render not deterministic)  | ok (1.875%) |
| 63 | Band 4 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | skipped (render not deterministic)  | ok (2.472%) |
| 64 | Band 4 Gain | Bands Open=1 | skipped (render not deterministic)  | ok (1.874%) |
| 65 | Band 4 Solo |  | skipped (render not deterministic)  | ok (0.030%) |
| 66 | Band 4 Mute |  | skipped (render not deterministic)  | ok (0.030%) |
| 67 | Band 4 Bypass | All Bands Drive=1 | skipped (render not deterministic)  | ok (0.163%) |
| 68 | Band 5 Drive | Bands=0.8, Bands Open=1 | skipped (render not deterministic)  | ok (3.538%) |
| 69 | Band 5 Ceiling | Bands=0.8, Bands Open=1, All Bands Ceiling=0.5 | skipped (render not deterministic)  | ok (3.509%) |
| 70 | Band 5 Gain | Bands=0.8, Bands Open=1 | skipped (render not deterministic)  | ok (3.538%) |
| 71 | Band 5 Solo | Bands=0.8, Bands Open=0 | skipped (render not deterministic)  | ok (1.481%) |
| 72 | Band 5 Mute | Bands=0.8, Bands Open=0 | skipped (render not deterministic)  | ok (1.477%) |
| 73 | Band 5 Bypass | Bands=0.8, All Bands Drive=1 | skipped (render not deterministic)  | ok (1.611%) |
| 74 | Band 6 Drive | Bands=1, Bands Open=1 | skipped (render not deterministic)  | ok (3.734%) |
| 75 | Band 6 Ceiling | Bands=1, Bands Open=1, All Bands Ceiling=0.5 | skipped (render not deterministic)  | ok (3.757%) |
| 76 | Band 6 Gain | Bands=1, Bands Open=1 | skipped (render not deterministic)  | ok (3.734%) |
| 77 | Band 6 Solo | Bands=1, Bands Open=0 | skipped (render not deterministic)  | ok (1.513%) |
| 78 | Band 6 Mute | Bands=1, Bands Open=0 | skipped (render not deterministic)  | ok (1.513%) |
| 79 | Band 6 Bypass | Bands=1, All Bands Drive=1 | skipped (render not deterministic)  | ok (1.646%) |
| 80 | Bypass |  | skipped (render not deterministic)  | excused (0.000%) |
| 81 | Bypass |  | skipped (render not deterministic)  | excused (0.004%) |
| 82 | Wet |  | skipped (render not deterministic)  | excused (0.000%) |
| 83 | Delta |  | skipped (render not deterministic)  | excused (0.000%) |
