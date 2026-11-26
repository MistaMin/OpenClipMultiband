# Plugin QA: AU: OpenClip Multiband (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | AU: OpenClip Multiband (OpenGrid), 83 parameters |
| finite output | PASS | peak -4.1 dBFS |
| not silent | PASS | rms -14.9 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1080x821, 39.0% animates by itself |
| every parameter changes the sound | FAIL | no sound at either end of the sweep, so these were never heard (the audio went silent mid-run, or the parameter mutes at both ends and needs an audio: exception): Band 6 Drive, Band 6 Ceiling, Band 6 Gain, Band 6 Solo, Band 6 Mute, Band 6 Bypass, Bypass, Wet, Delta |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-1.4 dB) | ok (5.205%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (3.301%) |
| 2 | Zero Latency |  | ok (-0.2 dB) | ok (1.968%) |
| 3 | Auto Gain |  | ok (0.9 dB) | ok (2.241%) |
| 4 | Delta |  | ok (-4.0 dB) | ok (2.023%) |
| 5 | Bands |  | ok (-1.7 dB) | ok (2.239%) |
| 6 | Crossover Slope |  | ok (1.4 dB) | ok (1.748%) |
| 7 | Crossover 1 |  | ok (-1.2 dB) | ok (1.958%) |
| 8 | Crossover 2 |  | ok (-1.4 dB) | ok (1.825%) |
| 9 | Crossover 3 |  | ok (-1.5 dB) | ok (1.571%) |
| 10 | Crossover 4 | Bands=0.8 | ok (-2.2 dB) | ok (1.956%) |
| 11 | Crossover 5 | Bands=1 | ok (-1.9 dB) | ok (2.400%) |
| 12 | Algorithm | All Bands Drive=1 | ok (3.8 dB) | ok (7.229%) |
| 13 | Shape | All Bands Drive=1 | ok (-5.3 dB) | ok (7.885%) |
| 14 | Character | All Bands Drive=1 | ok (-13.8 dB) | ok (7.138%) |
| 15 | Single/Multi Blend | All Bands Drive=1 | ok (-1.4 dB) | ok (7.479%) |
| 16 | Dry/Wet Mix | All Bands Drive=1 | ok (-1.4 dB) | ok (7.437%) |
| 17 | Single Drive | Single/Multi Blend=0 | ok (-1.9 dB) | ok (6.577%) |
| 18 | Single Ceiling | Single/Multi Blend=0 | ok (-3.5 dB) | ok (1.906%) |
| 19 | Output Ceiling On | Input Gain=1 | ok (2.3 dB) | ok (5.872%) |
| 20 | Output Ceiling | Input Gain=1 | ok (-1.1 dB) | ok (5.176%) |
| 21 | Anti-Alias | All Bands Drive=1 | ok (3.7 dB) | ok (2.680%) |
| 22 | Oversampling | All Bands Drive=1 | ok (4.0 dB) | ok (6.931%) |
| 23 | Bands Open |  | excused (-3.1 dB) | ok (2.903%) |
| 24 | Drawn Curve | All Bands Drive=1 | ok (-2.6 dB) | ok (7.500%) |
| 25 | Curve Point 1 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (7.581%) |
| 26 | Curve Point 2 | Drawn Curve=1, All Bands Drive=1 | ok (0.1 dB) | ok (7.490%) |
| 27 | Curve Point 3 | Drawn Curve=1, All Bands Drive=1 | ok (-2.4 dB) | ok (7.565%) |
| 28 | Curve Point 4 | Drawn Curve=1, All Bands Drive=1 | ok (-2.3 dB) | ok (7.535%) |
| 29 | Curve Point 5 | Drawn Curve=1, All Bands Drive=1 | ok (-2.8 dB) | ok (7.460%) |
| 30 | Curve Point 6 | Drawn Curve=1, All Bands Drive=1 | ok (-3.0 dB) | ok (7.561%) |
| 31 | Curve Point 7 | Drawn Curve=1, All Bands Drive=1 | ok (-2.1 dB) | ok (7.312%) |
| 32 | Curve Point 8 | Drawn Curve=1, All Bands Drive=1 | ok (-0.5 dB) | ok (7.258%) |
| 33 | Curve Point 9 | Drawn Curve=1, All Bands Drive=1 | ok (0.6 dB) | ok (7.415%) |
| 34 | Curve Point 10 | Drawn Curve=1, All Bands Drive=1 | ok (2.0 dB) | ok (7.559%) |
| 35 | Curve Point 11 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (7.775%) |
| 36 | Curve Point 12 | Drawn Curve=1, All Bands Drive=1 | ok (2.9 dB) | ok (7.792%) |
| 37 | Curve Point 13 | Drawn Curve=1, All Bands Drive=1 | ok (2.1 dB) | ok (7.588%) |
| 38 | Curve Point 14 | Drawn Curve=1, All Bands Drive=1 | ok (-1.1 dB) | ok (7.100%) |
| 39 | Curve Point 15 | Drawn Curve=1, All Bands Drive=1 | ok (-0.6 dB) | ok (7.218%) |
| 40 | Curve Point 16 | Drawn Curve=1, All Bands Drive=1 | ok (3.2 dB) | ok (6.610%) |
| 41 | All Bands Drive |  | ok (-1.4 dB) | ok (6.692%) |
| 42 | All Bands Ceiling |  | ok (-3.2 dB) | ok (1.856%) |
| 43 | All Bands Gain |  | ok (-1.7 dB) | ok (5.852%) |
| 44 | Band 1 Drive | Bands Open=1 | ok (-0.9 dB) | ok (4.167%) |
| 45 | Band 1 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-13.3 dB) | ok (2.428%) |
| 46 | Band 1 Gain | Bands Open=1 | ok (1.0 dB) | ok (3.634%) |
| 47 | Band 1 Solo |  | ok (-5.7 dB) | ok (1.742%) |
| 48 | Band 1 Mute |  | ok (-11.8 dB) | ok (1.797%) |
| 49 | Band 1 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (6.545%) |
| 50 | Band 2 Drive | Bands Open=1 | ok (-1.0 dB) | ok (4.489%) |
| 51 | Band 2 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-12.8 dB) | ok (2.388%) |
| 52 | Band 2 Gain | Bands Open=1 | ok (1.1 dB) | ok (4.099%) |
| 53 | Band 2 Solo |  | ok (-4.9 dB) | ok (2.025%) |
| 54 | Band 2 Mute |  | ok (-9.7 dB) | ok (1.800%) |
| 55 | Band 2 Bypass | All Bands Drive=1 | ok (-0.4 dB) | ok (6.876%) |
| 56 | Band 3 Drive | Bands Open=1 | ok (-0.9 dB) | ok (4.588%) |
| 57 | Band 3 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-12.9 dB) | ok (3.359%) |
| 58 | Band 3 Gain | Bands Open=1 | ok (1.0 dB) | ok (4.557%) |
| 59 | Band 3 Solo |  | ok (-4.5 dB) | ok (2.335%) |
| 60 | Band 3 Mute |  | ok (-10.8 dB) | ok (2.126%) |
| 61 | Band 3 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (7.240%) |
| 62 | Band 4 Drive | Bands Open=1 | ok (1.4 dB) | ok (5.523%) |
| 63 | Band 4 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-10.7 dB) | ok (3.343%) |
| 64 | Band 4 Gain | Bands Open=1 | ok (0.1 dB) | ok (4.968%) |
| 65 | Band 4 Solo |  | ok (-6.6 dB) | ok (2.175%) |
| 66 | Band 4 Mute |  | ok (-8.4 dB) | ok (2.218%) |
| 67 | Band 4 Bypass | All Bands Drive=1 | ok (1.8 dB) | ok (6.180%) |
| 68 | Band 5 Drive | Bands=0.8, Bands Open=1 | ok (-0.6 dB) | ok (5.812%) |
| 69 | Band 5 Ceiling | Bands=0.8, Bands Open=1, All Bands Ceiling=0.5 | ok (-13.0 dB) | ok (3.927%) |
| 70 | Band 5 Gain | Bands=0.8, Bands Open=1 | ok (-0.2 dB) | ok (5.331%) |
| 71 | Band 5 Solo | Bands=0.8, Bands Open=0 | ok (-5.7 dB) | ok (2.988%) |
| 72 | Band 5 Mute | Bands=0.8, Bands Open=0 | ok (-11.9 dB) | ok (2.834%) |
| 73 | Band 5 Bypass | Bands=0.8, All Bands Drive=1 | ok (1.0 dB) | ok (7.174%) |
| 74 | Band 6 Drive | Bands=1, Bands Open=1 | SILENT (-240.0 dB) | ok (2.440%) |
| 75 | Band 6 Ceiling | Bands=1, Bands Open=1, All Bands Ceiling=0.5 | SILENT (-240.0 dB) | ok (2.349%) |
| 76 | Band 6 Gain | Bands=1, Bands Open=1 | SILENT (-240.0 dB) | ok (2.455%) |
| 77 | Band 6 Solo | Bands=1, Bands Open=0 | SILENT (-240.0 dB) | ok (1.335%) |
| 78 | Band 6 Mute | Bands=1, Bands Open=0 | SILENT (-240.0 dB) | ok (1.335%) |
| 79 | Band 6 Bypass | Bands=1, All Bands Drive=1 | SILENT (-240.0 dB) | ok (1.335%) |
| 80 | Bypass |  | SILENT (-4.1 dB) | ok (1.049%) |
| 81 | Wet |  | SILENT (-4.1 dB) | ok (0.792%) |
| 82 | Delta |  | SILENT (-4.1 dB) | ok (0.792%) |
