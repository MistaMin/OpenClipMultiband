# Plugin QA: LV2: OpenClip Multiband

| Check | Status | Detail |
|---|---|---|
| loads | PASS | OpenClip Multiband, 81 parameters |
| finite output | PASS | peak -4.4 dBFS |
| not silent | PASS | rms -15.0 dBFS |
| deterministic render | FAIL | repeat differs by -3.5 dB |
| editor opens | SKIPPED | visual checks disabled |
| every parameter changes the sound | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | SKIPPED | headless: OpenGrid9 runs without an editor window |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | skipped (render not deterministic)  | -  |
| 1 | Output Gain |  | skipped (render not deterministic)  | -  |
| 2 | Zero Latency |  | skipped (render not deterministic)  | -  |
| 3 | Auto Gain |  | skipped (render not deterministic)  | -  |
| 4 | Delta |  | skipped (render not deterministic)  | -  |
| 5 | Bands |  | skipped (render not deterministic)  | -  |
| 6 | Crossover Slope |  | skipped (render not deterministic)  | -  |
| 7 | Crossover 1 |  | skipped (render not deterministic)  | -  |
| 8 | Crossover 2 |  | skipped (render not deterministic)  | -  |
| 9 | Crossover 3 |  | skipped (render not deterministic)  | -  |
| 10 | Crossover 4 | Bands=0.8 | skipped (render not deterministic)  | -  |
| 11 | Crossover 5 | Bands=1 | skipped (render not deterministic)  | -  |
| 12 | Algorithm | All Bands Drive=1 | skipped (render not deterministic)  | -  |
| 13 | Shape | All Bands Drive=1 | skipped (render not deterministic)  | -  |
| 14 | Character | All Bands Drive=1 | skipped (render not deterministic)  | -  |
| 15 | Single/Multi Blend | All Bands Drive=1 | skipped (render not deterministic)  | -  |
| 16 | Dry/Wet Mix | All Bands Drive=1 | skipped (render not deterministic)  | -  |
| 17 | Single Drive | Single/Multi Blend=0 | skipped (render not deterministic)  | -  |
| 18 | Single Ceiling | Single/Multi Blend=0 | skipped (render not deterministic)  | -  |
| 19 | Output Ceiling On | Input Gain=1 | skipped (render not deterministic)  | -  |
| 20 | Output Ceiling | Input Gain=1 | skipped (render not deterministic)  | -  |
| 21 | Anti-Alias | All Bands Drive=1 | skipped (render not deterministic)  | -  |
| 22 | Oversampling | All Bands Drive=1 | skipped (render not deterministic)  | -  |
| 23 | Bands Open |  | skipped (render not deterministic)  | -  |
| 24 | Drawn Curve | All Bands Drive=1 | skipped (render not deterministic)  | -  |
| 25 | Curve Point 1 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | -  |
| 26 | Curve Point 2 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | -  |
| 27 | Curve Point 3 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | -  |
| 28 | Curve Point 4 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | -  |
| 29 | Curve Point 5 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | -  |
| 30 | Curve Point 6 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | -  |
| 31 | Curve Point 7 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | -  |
| 32 | Curve Point 8 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | -  |
| 33 | Curve Point 9 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | -  |
| 34 | Curve Point 10 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | -  |
| 35 | Curve Point 11 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | -  |
| 36 | Curve Point 12 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | -  |
| 37 | Curve Point 13 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | -  |
| 38 | Curve Point 14 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | -  |
| 39 | Curve Point 15 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | -  |
| 40 | Curve Point 16 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | -  |
| 41 | All Bands Drive |  | skipped (render not deterministic)  | -  |
| 42 | All Bands Ceiling |  | skipped (render not deterministic)  | -  |
| 43 | All Bands Gain |  | skipped (render not deterministic)  | -  |
| 44 | Band 1 Drive | Bands Open=1 | skipped (render not deterministic)  | -  |
| 45 | Band 1 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | skipped (render not deterministic)  | -  |
| 46 | Band 1 Gain | Bands Open=1 | skipped (render not deterministic)  | -  |
| 47 | Band 1 Solo |  | skipped (render not deterministic)  | -  |
| 48 | Band 1 Mute |  | skipped (render not deterministic)  | -  |
| 49 | Band 1 Bypass | All Bands Drive=1 | skipped (render not deterministic)  | -  |
| 50 | Band 2 Drive | Bands Open=1 | skipped (render not deterministic)  | -  |
| 51 | Band 2 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | skipped (render not deterministic)  | -  |
| 52 | Band 2 Gain | Bands Open=1 | skipped (render not deterministic)  | -  |
| 53 | Band 2 Solo |  | skipped (render not deterministic)  | -  |
| 54 | Band 2 Mute |  | skipped (render not deterministic)  | -  |
| 55 | Band 2 Bypass | All Bands Drive=1 | skipped (render not deterministic)  | -  |
| 56 | Band 3 Drive | Bands Open=1 | skipped (render not deterministic)  | -  |
| 57 | Band 3 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | skipped (render not deterministic)  | -  |
| 58 | Band 3 Gain | Bands Open=1 | skipped (render not deterministic)  | -  |
| 59 | Band 3 Solo |  | skipped (render not deterministic)  | -  |
| 60 | Band 3 Mute |  | skipped (render not deterministic)  | -  |
| 61 | Band 3 Bypass | All Bands Drive=1 | skipped (render not deterministic)  | -  |
| 62 | Band 4 Drive | Bands Open=1 | skipped (render not deterministic)  | -  |
| 63 | Band 4 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | skipped (render not deterministic)  | -  |
| 64 | Band 4 Gain | Bands Open=1 | skipped (render not deterministic)  | -  |
| 65 | Band 4 Solo |  | skipped (render not deterministic)  | -  |
| 66 | Band 4 Mute |  | skipped (render not deterministic)  | -  |
| 67 | Band 4 Bypass | All Bands Drive=1 | skipped (render not deterministic)  | -  |
| 68 | Band 5 Drive | Bands=0.8, Bands Open=1 | skipped (render not deterministic)  | -  |
| 69 | Band 5 Ceiling | Bands=0.8, Bands Open=1, All Bands Ceiling=0.5 | skipped (render not deterministic)  | -  |
| 70 | Band 5 Gain | Bands=0.8, Bands Open=1 | skipped (render not deterministic)  | -  |
| 71 | Band 5 Solo | Bands=0.8, Bands Open=0 | skipped (render not deterministic)  | -  |
| 72 | Band 5 Mute | Bands=0.8, Bands Open=0 | skipped (render not deterministic)  | -  |
| 73 | Band 5 Bypass | Bands=0.8, All Bands Drive=1 | skipped (render not deterministic)  | -  |
| 74 | Band 6 Drive | Bands=1, Bands Open=1 | skipped (render not deterministic)  | -  |
| 75 | Band 6 Ceiling | Bands=1, Bands Open=1, All Bands Ceiling=0.5 | skipped (render not deterministic)  | -  |
| 76 | Band 6 Gain | Bands=1, Bands Open=1 | skipped (render not deterministic)  | -  |
| 77 | Band 6 Solo | Bands=1, Bands Open=0 | skipped (render not deterministic)  | -  |
| 78 | Band 6 Mute | Bands=1, Bands Open=0 | skipped (render not deterministic)  | -  |
| 79 | Band 6 Bypass | Bands=1, All Bands Drive=1 | skipped (render not deterministic)  | -  |
| 80 | Freewheel |  | skipped (render not deterministic)  | -  |
