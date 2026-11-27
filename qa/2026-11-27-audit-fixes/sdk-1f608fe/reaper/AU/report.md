# Plugin QA: AU: OpenClip Multiband (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | AU: OpenClip Multiband (OpenGrid), 83 parameters |
| finite output | PASS | peak -4.1 dBFS |
| not silent | PASS | rms -14.9 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1080x821, 0.1% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-1.4 dB) | ok (8.699%) |
| 1 | Output Gain |  | ok (-0.6 dB) | ok (4.824%) |
| 2 | Zero Latency |  | ok (-0.2 dB) | ok (3.698%) |
| 3 | Auto Gain |  | ok (0.9 dB) | ok (3.469%) |
| 4 | Delta |  | ok (-4.0 dB) | ok (2.878%) |
| 5 | Bands |  | ok (-1.7 dB) | ok (4.209%) |
| 6 | Crossover Slope |  | ok (1.4 dB) | ok (3.566%) |
| 7 | Crossover 1 |  | ok (-1.2 dB) | ok (3.873%) |
| 8 | Crossover 2 |  | ok (-1.4 dB) | ok (3.606%) |
| 9 | Crossover 3 |  | ok (-1.5 dB) | ok (3.647%) |
| 10 | Crossover 4 | Bands=0.8 | ok (-2.2 dB) | ok (5.517%) |
| 11 | Crossover 5 | Bands=1 | ok (-1.9 dB) | ok (5.449%) |
| 12 | Algorithm | All Bands Drive=1 | ok (3.8 dB) | ok (10.273%) |
| 13 | Shape | All Bands Drive=1 | ok (-5.3 dB) | ok (10.453%) |
| 14 | Character | All Bands Drive=1 | ok (-13.8 dB) | ok (10.384%) |
| 15 | Single/Multi Blend | All Bands Drive=1 | ok (-1.4 dB) | ok (10.527%) |
| 16 | Dry/Wet Mix | All Bands Drive=1 | ok (-1.4 dB) | ok (10.209%) |
| 17 | Single Drive | Single/Multi Blend=0 | ok (-1.9 dB) | ok (8.581%) |
| 18 | Single Ceiling | Single/Multi Blend=0 | ok (-3.5 dB) | ok (2.923%) |
| 19 | Output Ceiling On | Input Gain=1 | ok (2.3 dB) | ok (9.351%) |
| 20 | Output Ceiling | Input Gain=1 | ok (-1.1 dB) | ok (8.208%) |
| 21 | Anti-Alias | All Bands Drive=1 | ok (3.7 dB) | ok (5.306%) |
| 22 | Oversampling | All Bands Drive=1 | ok (4.0 dB) | ok (9.812%) |
| 23 | Bands Open |  | excused (-3.7 dB) | ok (5.749%) |
| 24 | Drawn Curve | All Bands Drive=1 | ok (-2.6 dB) | ok (10.433%) |
| 25 | Curve Point 1 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (11.133%) |
| 26 | Curve Point 2 | Drawn Curve=1, All Bands Drive=1 | ok (0.1 dB) | ok (11.389%) |
| 27 | Curve Point 3 | Drawn Curve=1, All Bands Drive=1 | ok (-2.4 dB) | ok (10.661%) |
| 28 | Curve Point 4 | Drawn Curve=1, All Bands Drive=1 | ok (-2.3 dB) | ok (10.740%) |
| 29 | Curve Point 5 | Drawn Curve=1, All Bands Drive=1 | ok (-2.8 dB) | ok (10.835%) |
| 30 | Curve Point 6 | Drawn Curve=1, All Bands Drive=1 | ok (-3.0 dB) | ok (10.815%) |
| 31 | Curve Point 7 | Drawn Curve=1, All Bands Drive=1 | ok (-2.1 dB) | ok (10.410%) |
| 32 | Curve Point 8 | Drawn Curve=1, All Bands Drive=1 | ok (-0.5 dB) | ok (10.820%) |
| 33 | Curve Point 9 | Drawn Curve=1, All Bands Drive=1 | ok (0.6 dB) | ok (10.289%) |
| 34 | Curve Point 10 | Drawn Curve=1, All Bands Drive=1 | ok (2.0 dB) | ok (10.527%) |
| 35 | Curve Point 11 | Drawn Curve=1, All Bands Drive=1 | ok (2.5 dB) | ok (10.272%) |
| 36 | Curve Point 12 | Drawn Curve=1, All Bands Drive=1 | ok (2.9 dB) | ok (10.700%) |
| 37 | Curve Point 13 | Drawn Curve=1, All Bands Drive=1 | ok (2.1 dB) | ok (10.708%) |
| 38 | Curve Point 14 | Drawn Curve=1, All Bands Drive=1 | ok (-1.1 dB) | ok (10.907%) |
| 39 | Curve Point 15 | Drawn Curve=1, All Bands Drive=1 | ok (-0.6 dB) | ok (10.516%) |
| 40 | Curve Point 16 | Drawn Curve=1, All Bands Drive=1 | ok (3.2 dB) | ok (10.700%) |
| 41 | All Bands Drive |  | ok (-1.4 dB) | ok (10.156%) |
| 42 | All Bands Ceiling |  | ok (-3.2 dB) | ok (3.416%) |
| 43 | All Bands Gain |  | ok (-1.7 dB) | ok (7.920%) |
| 44 | Band 1 Drive | Bands Open=1 | ok (-0.9 dB) | ok (6.366%) |
| 45 | Band 1 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-13.3 dB) | ok (6.274%) |
| 46 | Band 1 Gain | Bands Open=1 | ok (1.0 dB) | ok (6.828%) |
| 47 | Band 1 Solo |  | ok (-5.7 dB) | ok (4.022%) |
| 48 | Band 1 Mute |  | ok (-11.8 dB) | ok (3.291%) |
| 49 | Band 1 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (9.281%) |
| 50 | Band 2 Drive | Bands Open=1 | ok (-1.0 dB) | ok (7.790%) |
| 51 | Band 2 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-12.8 dB) | ok (6.171%) |
| 52 | Band 2 Gain | Bands Open=1 | ok (1.1 dB) | ok (7.100%) |
| 53 | Band 2 Solo |  | ok (-4.9 dB) | ok (3.876%) |
| 54 | Band 2 Mute |  | ok (-9.7 dB) | ok (3.088%) |
| 55 | Band 2 Bypass | All Bands Drive=1 | ok (-0.4 dB) | ok (10.229%) |
| 56 | Band 3 Drive | Bands Open=1 | ok (-0.9 dB) | ok (7.821%) |
| 57 | Band 3 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-12.9 dB) | ok (6.142%) |
| 58 | Band 3 Gain | Bands Open=1 | ok (1.0 dB) | ok (7.026%) |
| 59 | Band 3 Solo |  | ok (-4.5 dB) | ok (3.841%) |
| 60 | Band 3 Mute |  | ok (-10.8 dB) | ok (3.311%) |
| 61 | Band 3 Bypass | All Bands Drive=1 | ok (-1.2 dB) | ok (9.981%) |
| 62 | Band 4 Drive | Bands Open=1 | ok (1.4 dB) | ok (8.492%) |
| 63 | Band 4 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | ok (-10.7 dB) | ok (5.790%) |
| 64 | Band 4 Gain | Bands Open=1 | ok (0.1 dB) | ok (7.185%) |
| 65 | Band 4 Solo |  | ok (-6.6 dB) | ok (3.519%) |
| 66 | Band 4 Mute |  | ok (-8.4 dB) | ok (3.552%) |
| 67 | Band 4 Bypass | All Bands Drive=1 | ok (1.8 dB) | ok (8.749%) |
| 68 | Band 5 Drive | Bands=0.8, Bands Open=1 | ok (-0.6 dB) | ok (9.794%) |
| 69 | Band 5 Ceiling | Bands=0.8, Bands Open=1, All Bands Ceiling=0.5 | ok (-13.0 dB) | ok (8.071%) |
| 70 | Band 5 Gain | Bands=0.8, Bands Open=1 | ok (-0.2 dB) | ok (9.431%) |
| 71 | Band 5 Solo | Bands=0.8, Bands Open=0 | ok (-5.7 dB) | ok (5.820%) |
| 72 | Band 5 Mute | Bands=0.8, Bands Open=0 | ok (-11.9 dB) | ok (5.858%) |
| 73 | Band 5 Bypass | Bands=0.8, All Bands Drive=1 | ok (1.0 dB) | ok (11.138%) |
| 74 | Band 6 Drive | Bands=1, Bands Open=1 | ok (-0.9 dB) | ok (8.908%) |
| 75 | Band 6 Ceiling | Bands=1, Bands Open=1, All Bands Ceiling=0.5 | ok (-13.1 dB) | ok (8.139%) |
| 76 | Band 6 Gain | Bands=1, Bands Open=1 | ok (-0.3 dB) | ok (8.923%) |
| 77 | Band 6 Solo | Bands=1, Bands Open=0 | ok (-4.9 dB) | ok (6.088%) |
| 78 | Band 6 Mute | Bands=1, Bands Open=0 | ok (-12.1 dB) | ok (5.751%) |
| 79 | Band 6 Bypass | Bands=1, All Bands Drive=1 | ok (0.0 dB) | ok (11.481%) |
| 80 | Bypass |  | ok (-1.7 dB) | ok (3.777%) |
| 81 | Wet |  | ok (-1.7 dB) | ok (3.240%) |
| 82 | Delta |  | ok (-3.5 dB) | ok (3.151%) |
