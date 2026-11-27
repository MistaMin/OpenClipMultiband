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
| 0 | Input Gain |  | ok (-1.4 dB) | ok (7.867%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (5.711%) |
| 2 | Zero Latency |  | ok (-0.2 dB) | ok (4.177%) |
| 3 | Auto Gain |  | ok (0.9 dB) | ok (2.973%) |
| 4 | Delta |  | ok (-4.0 dB) | ok (3.084%) |
| 5 | Bands |  | ok (-1.7 dB) | ok (5.020%) |
| 6 | Crossover Slope |  | ok (1.4 dB) | ok (3.272%) |
| 7 | Crossover 1 |  | ok (-1.2 dB) | ok (3.461%) |
| 8 | Crossover 2 |  | ok (-1.4 dB) | ok (3.681%) |
| 9 | Crossover 3 |  | ok (-1.5 dB) | ok (3.540%) |
| 10 | Crossover 4 | Bands=0.8 | ok (-2.2 dB) | ok (5.269%) |
| 11 | Crossover 5 | Bands=1 | ok (-1.9 dB) | ok (5.751%) |
| 12 | Algorithm | All Bands Drive=1 | ok (3.8 dB) | ok (10.258%) |
| 13 | Shape | All Bands Drive=1 | ok (-5.3 dB) | ok (10.183%) |
| 14 | Character | All Bands Drive=1 | ok (-25.7 dB) | ok (10.526%) |
| 15 | Single/Multi Blend | All Bands Drive=1 | ok (-1.4 dB) | ok (10.085%) |
| 16 | Dry/Wet Mix | All Bands Drive=1 | ok (-1.4 dB) | ok (10.059%) |
| 17 | Single Drive | Single/Multi Blend=0 | ok (-1.9 dB) | ok (9.013%) |
| 18 | Single Ceiling | Single/Multi Blend=0 | ok (-3.5 dB) | ok (3.217%) |
| 19 | Output Ceiling On | Input Gain=1 | ok (2.5 dB) | ok (8.752%) |
| 20 | Output Ceiling | Input Gain=1 | ok (-1.5 dB) | ok (8.767%) |
| 21 | Anti-Alias | All Bands Drive=1 | ok (4.5 dB) | ok (5.200%) |
| 22 | Oversampling | All Bands Drive=1 | ok (4.9 dB) | ok (9.179%) |
| 23 | Bands Open |  | excused (-1.4 dB) | ok (5.609%) |
| 24 | Drawn Curve | All Bands Drive=1 | ok (-2.6 dB) | ok (10.753%) |
| 25 | Curve Point 1 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (11.294%) |
| 26 | Curve Point 2 | Drawn Curve=1, All Bands Drive=1 | ok (0.1 dB) | ok (10.530%) |
| 27 | Curve Point 3 | Drawn Curve=1, All Bands Drive=1 | ok (-2.4 dB) | ok (10.563%) |
| 28 | Curve Point 4 | Drawn Curve=1, All Bands Drive=1 | ok (-2.3 dB) | ok (10.527%) |
| 29 | Curve Point 5 | Drawn Curve=1, All Bands Drive=1 | ok (-2.8 dB) | ok (10.638%) |
| 30 | Curve Point 6 | Drawn Curve=1, All Bands Drive=1 | ok (-3.0 dB) | ok (10.578%) |
| 31 | Curve Point 7 | Drawn Curve=1, All Bands Drive=1 | ok (-2.1 dB) | ok (10.261%) |
| 32 | Curve Point 8 | Drawn Curve=1, All Bands Drive=1 | ok (-0.5 dB) | ok (10.466%) |
| 33 | Curve Point 9 | Drawn Curve=1, All Bands Drive=1 | ok (0.6 dB) | ok (9.747%) |
| 34 | Curve Point 10 | Drawn Curve=1, All Bands Drive=1 | ok (2.0 dB) | ok (10.730%) |
| 35 | Curve Point 11 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (10.240%) |
| 36 | Curve Point 12 | Drawn Curve=1, All Bands Drive=1 | ok (2.9 dB) | ok (10.110%) |
| 37 | Curve Point 13 | Drawn Curve=1, All Bands Drive=1 | ok (2.1 dB) | ok (10.319%) |
| 38 | Curve Point 14 | Drawn Curve=1, All Bands Drive=1 | ok (-1.1 dB) | ok (9.929%) |
| 39 | Curve Point 15 | Drawn Curve=1, All Bands Drive=1 | ok (-0.6 dB) | ok (10.150%) |
| 40 | Curve Point 16 | Drawn Curve=1, All Bands Drive=1 | ok (3.2 dB) | ok (10.721%) |
| 41 | All Bands Drive |  | ok (-1.4 dB) | ok (10.198%) |
| 42 | All Bands Ceiling |  | ok (-3.2 dB) | ok (3.903%) |
| 43 | All Bands Gain |  | ok (-1.7 dB) | ok (7.517%) |
| 44 | Band 1 Drive | Bands Open=1 | ok (-0.9 dB) | ok (7.118%) |
| 45 | Band 1 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-4.5 dB) | ok (5.763%) |
| 46 | Band 1 Gain | Bands Open=1 | ok (1.0 dB) | ok (6.321%) |
| 47 | Band 1 Solo |  | ok (-5.0 dB) | ok (3.753%) |
| 48 | Band 1 Mute |  | ok (-11.8 dB) | ok (3.066%) |
| 49 | Band 1 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (9.008%) |
| 50 | Band 2 Drive | Bands Open=1 | ok (-1.0 dB) | ok (7.278%) |
| 51 | Band 2 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-2.2 dB) | ok (6.216%) |
| 52 | Band 2 Gain | Bands Open=1 | ok (1.1 dB) | ok (7.746%) |
| 53 | Band 2 Solo |  | ok (-0.6 dB) | ok (3.627%) |
| 54 | Band 2 Mute |  | ok (-9.7 dB) | ok (3.128%) |
| 55 | Band 2 Bypass | All Bands Drive=1 | ok (-0.4 dB) | ok (9.479%) |
| 56 | Band 3 Drive | Bands Open=1 | ok (-0.9 dB) | ok (7.781%) |
| 57 | Band 3 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-2.3 dB) | ok (6.205%) |
| 58 | Band 3 Gain | Bands Open=1 | ok (1.0 dB) | ok (6.808%) |
| 59 | Band 3 Solo |  | ok (-1.6 dB) | ok (3.736%) |
| 60 | Band 3 Mute |  | ok (-10.8 dB) | ok (3.113%) |
| 61 | Band 3 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (9.260%) |
| 62 | Band 4 Drive | Bands Open=1 | ok (1.4 dB) | ok (8.063%) |
| 63 | Band 4 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-3.0 dB) | ok (5.666%) |
| 64 | Band 4 Gain | Bands Open=1 | ok (0.1 dB) | ok (7.014%) |
| 65 | Band 4 Solo |  | ok (-3.5 dB) | ok (3.424%) |
| 66 | Band 4 Mute |  | ok (-8.4 dB) | ok (3.450%) |
| 67 | Band 4 Bypass | All Bands Drive=1 | ok (1.8 dB) | ok (9.079%) |
| 68 | Band 5 Drive | Bands=0.8, Bands Open=1 | ok (-0.6 dB) | ok (9.979%) |
| 69 | Band 5 Ceiling | Bands=0.8, Bands Open=1, All Bands Ceiling=0.5 | ok (-3.3 dB) | ok (8.025%) |
| 70 | Band 5 Gain | Bands=0.8, Bands Open=1 | ok (-0.2 dB) | ok (9.417%) |
| 71 | Band 5 Solo | Bands=0.8, Bands Open=0 | ok (-3.7 dB) | ok (6.197%) |
| 72 | Band 5 Mute | Bands=0.8, Bands Open=0 | ok (-11.9 dB) | ok (5.611%) |
| 73 | Band 5 Bypass | Bands=0.8, All Bands Drive=1 | ok (1.0 dB) | ok (11.253%) |
| 74 | Band 6 Drive | Bands=1, Bands Open=1 | ok (-0.9 dB) | ok (8.624%) |
| 75 | Band 6 Ceiling | Bands=1, Bands Open=1, All Bands Ceiling=0.5 | ok (-4.4 dB) | ok (8.111%) |
| 76 | Band 6 Gain | Bands=1, Bands Open=1 | ok (-0.3 dB) | ok (9.126%) |
| 77 | Band 6 Solo | Bands=1, Bands Open=0 | ok (-3.0 dB) | ok (5.912%) |
| 78 | Band 6 Mute | Bands=1, Bands Open=0 | ok (-12.1 dB) | ok (5.388%) |
| 79 | Band 6 Bypass | Bands=1, All Bands Drive=1 | ok (0.0 dB) | ok (11.787%) |
| 80 | Bypass |  | ok (-1.7 dB) | ok (3.337%) |
| 81 | Wet |  | ok (-1.7 dB) | ok (3.287%) |
| 82 | Delta |  | ok (-3.5 dB) | ok (2.970%) |
