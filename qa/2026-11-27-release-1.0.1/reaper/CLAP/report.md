# Plugin QA: CLAP: OpenClip Multiband

| Check | Status | Detail |
|---|---|---|
| loads | PASS | CLAP: OpenClip Multiband (OpenGrid), 83 parameters |
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
| 0 | Input Gain |  | ok (-1.4 dB) | ok (8.510%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (5.713%) |
| 2 | Zero Latency |  | ok (-0.2 dB) | ok (4.039%) |
| 3 | Auto Gain |  | ok (-3.9 dB) | ok (3.111%) |
| 4 | Delta |  | ok (-4.0 dB) | ok (3.330%) |
| 5 | Bands |  | ok (-1.7 dB) | ok (5.043%) |
| 6 | Crossover Slope |  | ok (1.4 dB) | ok (3.453%) |
| 7 | Crossover 1 |  | ok (-1.2 dB) | ok (3.927%) |
| 8 | Crossover 2 |  | ok (-1.4 dB) | ok (3.829%) |
| 9 | Crossover 3 |  | ok (-1.5 dB) | ok (3.455%) |
| 10 | Crossover 4 | Bands=0.8 | ok (-2.2 dB) | ok (5.416%) |
| 11 | Crossover 5 | Bands=1 | ok (-1.9 dB) | ok (5.477%) |
| 12 | Algorithm | All Bands Drive=1 | ok (3.8 dB) | ok (10.095%) |
| 13 | Shape | All Bands Drive=1 | ok (-5.3 dB) | ok (10.216%) |
| 14 | Character | All Bands Drive=1 | ok (-25.7 dB) | ok (9.849%) |
| 15 | Single/Multi Blend | All Bands Drive=1 | ok (-1.4 dB) | ok (10.082%) |
| 16 | Dry/Wet Mix | All Bands Drive=1 | ok (-1.4 dB) | ok (9.782%) |
| 17 | Single Drive | Single/Multi Blend=0 | ok (-1.9 dB) | ok (8.569%) |
| 18 | Single Ceiling | Single/Multi Blend=0 | ok (-3.5 dB) | ok (2.667%) |
| 19 | Output Ceiling On | Input Gain=1 | ok (2.5 dB) | ok (7.845%) |
| 20 | Output Ceiling | Input Gain=1 | ok (-1.5 dB) | ok (8.735%) |
| 21 | Anti-Alias | All Bands Drive=1 | ok (4.6 dB) | ok (5.642%) |
| 22 | Oversampling | All Bands Drive=1 | ok (4.9 dB) | ok (9.015%) |
| 23 | Bands Open |  | excused (-3.7 dB) | ok (5.447%) |
| 24 | Drawn Curve | All Bands Drive=1 | ok (-2.6 dB) | ok (10.148%) |
| 25 | Curve Point 1 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (11.078%) |
| 26 | Curve Point 2 | Drawn Curve=1, All Bands Drive=1 | ok (0.1 dB) | ok (10.738%) |
| 27 | Curve Point 3 | Drawn Curve=1, All Bands Drive=1 | ok (-2.4 dB) | ok (10.548%) |
| 28 | Curve Point 4 | Drawn Curve=1, All Bands Drive=1 | ok (-2.3 dB) | ok (10.410%) |
| 29 | Curve Point 5 | Drawn Curve=1, All Bands Drive=1 | ok (-2.8 dB) | ok (10.173%) |
| 30 | Curve Point 6 | Drawn Curve=1, All Bands Drive=1 | ok (-3.0 dB) | ok (10.377%) |
| 31 | Curve Point 7 | Drawn Curve=1, All Bands Drive=1 | ok (-2.1 dB) | ok (10.464%) |
| 32 | Curve Point 8 | Drawn Curve=1, All Bands Drive=1 | ok (-0.5 dB) | ok (10.327%) |
| 33 | Curve Point 9 | Drawn Curve=1, All Bands Drive=1 | ok (0.6 dB) | ok (10.282%) |
| 34 | Curve Point 10 | Drawn Curve=1, All Bands Drive=1 | ok (2.0 dB) | ok (9.134%) |
| 35 | Curve Point 11 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (9.226%) |
| 36 | Curve Point 12 | Drawn Curve=1, All Bands Drive=1 | ok (2.9 dB) | ok (10.081%) |
| 37 | Curve Point 13 | Drawn Curve=1, All Bands Drive=1 | ok (2.1 dB) | ok (10.399%) |
| 38 | Curve Point 14 | Drawn Curve=1, All Bands Drive=1 | ok (-1.1 dB) | ok (10.070%) |
| 39 | Curve Point 15 | Drawn Curve=1, All Bands Drive=1 | ok (-0.6 dB) | ok (10.025%) |
| 40 | Curve Point 16 | Drawn Curve=1, All Bands Drive=1 | ok (3.2 dB) | ok (10.213%) |
| 41 | All Bands Drive |  | ok (-1.4 dB) | ok (9.593%) |
| 42 | All Bands Ceiling |  | ok (-3.2 dB) | ok (3.177%) |
| 43 | All Bands Gain |  | ok (-1.7 dB) | ok (6.678%) |
| 44 | Band 1 Drive | Bands Open=1 | ok (-0.9 dB) | ok (6.706%) |
| 45 | Band 1 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-4.5 dB) | ok (6.364%) |
| 46 | Band 1 Gain | Bands Open=1 | ok (1.0 dB) | ok (6.975%) |
| 47 | Band 1 Solo |  | ok (-5.0 dB) | ok (3.881%) |
| 48 | Band 1 Mute |  | ok (-11.8 dB) | ok (3.261%) |
| 49 | Band 1 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (9.036%) |
| 50 | Band 2 Drive | Bands Open=1 | ok (-1.0 dB) | ok (7.690%) |
| 51 | Band 2 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-2.2 dB) | ok (6.202%) |
| 52 | Band 2 Gain | Bands Open=1 | ok (1.1 dB) | ok (7.064%) |
| 53 | Band 2 Solo |  | ok (-0.6 dB) | ok (3.765%) |
| 54 | Band 2 Mute |  | ok (-9.7 dB) | ok (3.338%) |
| 55 | Band 2 Bypass | All Bands Drive=1 | ok (-0.4 dB) | ok (9.698%) |
| 56 | Band 3 Drive | Bands Open=1 | ok (-0.9 dB) | ok (7.373%) |
| 57 | Band 3 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-2.3 dB) | ok (6.186%) |
| 58 | Band 3 Gain | Bands Open=1 | ok (1.0 dB) | ok (7.140%) |
| 59 | Band 3 Solo |  | ok (-1.6 dB) | ok (3.445%) |
| 60 | Band 3 Mute |  | ok (-10.8 dB) | ok (2.420%) |
| 61 | Band 3 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (8.777%) |
| 62 | Band 4 Drive | Bands Open=1 | ok (1.4 dB) | ok (8.021%) |
| 63 | Band 4 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-3.0 dB) | ok (5.584%) |
| 64 | Band 4 Gain | Bands Open=1 | ok (0.1 dB) | ok (6.969%) |
| 65 | Band 4 Solo |  | ok (-3.5 dB) | ok (3.274%) |
| 66 | Band 4 Mute |  | ok (-8.4 dB) | ok (3.627%) |
| 67 | Band 4 Bypass | All Bands Drive=1 | ok (1.8 dB) | ok (9.033%) |
| 68 | Band 5 Drive | Bands=0.8, Bands Open=1 | ok (-0.6 dB) | ok (9.905%) |
| 69 | Band 5 Ceiling | Bands=0.8, Bands Open=1, All Bands Ceiling=0.5 | ok (-3.3 dB) | ok (8.400%) |
| 70 | Band 5 Gain | Bands=0.8, Bands Open=1 | ok (-0.2 dB) | ok (8.815%) |
| 71 | Band 5 Solo | Bands=0.8, Bands Open=0 | ok (-3.7 dB) | ok (6.005%) |
| 72 | Band 5 Mute | Bands=0.8, Bands Open=0 | ok (-11.9 dB) | ok (5.675%) |
| 73 | Band 5 Bypass | Bands=0.8, All Bands Drive=1 | ok (1.0 dB) | ok (11.008%) |
| 74 | Band 6 Drive | Bands=1, Bands Open=1 | ok (-0.9 dB) | ok (9.022%) |
| 75 | Band 6 Ceiling | Bands=1, Bands Open=1, All Bands Ceiling=0.5 | ok (-4.4 dB) | ok (8.007%) |
| 76 | Band 6 Gain | Bands=1, Bands Open=1 | ok (-0.3 dB) | ok (8.689%) |
| 77 | Band 6 Solo | Bands=1, Bands Open=0 | ok (-3.0 dB) | ok (6.119%) |
| 78 | Band 6 Mute | Bands=1, Bands Open=0 | ok (-12.1 dB) | ok (5.418%) |
| 79 | Band 6 Bypass | Bands=1, All Bands Drive=1 | ok (0.0 dB) | ok (11.291%) |
| 80 | Bypass |  | ok (-1.7 dB) | ok (3.558%) |
| 81 | Wet |  | ok (-1.7 dB) | ok (3.189%) |
| 82 | Delta |  | ok (-3.5 dB) | ok (2.758%) |
