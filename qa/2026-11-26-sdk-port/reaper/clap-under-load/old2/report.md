# Plugin QA: CLAP: OpenClip Multiband

| Check | Status | Detail |
|---|---|---|
| loads | PASS | CLAP: OpenClip Multiband (OpenGrid), 83 parameters |
| finite output | PASS | peak -4.0 dBFS |
| not silent | PASS | rms -14.9 dBFS |
| deterministic render | FAIL | repeat differs by -1.5 dB |
| editor opens | PASS | 1080x821, 38.8% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | FAIL | Input Gain, Output Gain, Zero Latency, Auto Gain, Delta, Bands, Crossover Slope, Crossover 1, Crossover 2, Crossover 3, Crossover 4, Crossover 5, Algorithm, Shape, Character, Single/Multi Blend, Dry/Wet Mix, Single Drive, Single Ceiling, Output Ceiling On, Output Ceiling, Anti-Alias, Oversampling, Bands Open, Drawn Curve, Curve Point 1, Curve Point 2, Curve Point 3, Curve Point 4, Curve Point 5, Curve Point 6, Curve Point 7, Curve Point 8, Curve Point 9, Curve Point 10, Curve Point 11, Curve Point 12, Curve Point 13, Curve Point 14, Curve Point 15, Curve Point 16, All Bands Drive, All Bands Ceiling, All Bands Gain, Band 1 Drive, Band 1 Ceiling, Band 1 Gain, Band 1 Solo, Band 1 Mute, Band 1 Bypass, Band 2 Drive, Band 2 Ceiling, Band 2 Gain, Band 2 Solo, Band 2 Mute, Band 2 Bypass, Band 3 Drive, Band 3 Ceiling, Band 3 Gain, Band 3 Solo, Band 3 Mute, Band 3 Bypass, Band 4 Drive, Band 4 Ceiling, Band 4 Gain, Band 4 Solo, Band 4 Mute, Band 4 Bypass, Band 5 Drive, Band 5 Ceiling, Band 5 Gain, Band 5 Solo, Band 5 Mute, Band 5 Bypass, Band 6 Drive, Band 6 Ceiling, Band 6 Gain, Band 6 Solo, Band 6 Mute, Band 6 Bypass, Bypass, Wet, Delta |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 1 | Output Gain |  | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 2 | Zero Latency |  | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 3 | Auto Gain |  | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 4 | Delta |  | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 5 | Bands |  | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 6 | Crossover Slope |  | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 7 | Crossover 1 |  | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 8 | Crossover 2 |  | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 9 | Crossover 3 |  | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 10 | Crossover 4 | Bands=0.8 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 11 | Crossover 5 | Bands=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 12 | Algorithm | All Bands Drive=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 13 | Shape | All Bands Drive=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 14 | Character | All Bands Drive=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 15 | Single/Multi Blend | All Bands Drive=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 16 | Dry/Wet Mix | All Bands Drive=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 17 | Single Drive | Single/Multi Blend=0 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 18 | Single Ceiling | Single/Multi Blend=0 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 19 | Output Ceiling On | Input Gain=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 20 | Output Ceiling | Input Gain=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 21 | Anti-Alias | All Bands Drive=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 22 | Oversampling | All Bands Drive=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 23 | Bands Open |  | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 24 | Drawn Curve | All Bands Drive=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 25 | Curve Point 1 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 26 | Curve Point 2 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 27 | Curve Point 3 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 28 | Curve Point 4 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 29 | Curve Point 5 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 30 | Curve Point 6 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 31 | Curve Point 7 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 32 | Curve Point 8 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 33 | Curve Point 9 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 34 | Curve Point 10 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 35 | Curve Point 11 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 36 | Curve Point 12 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 37 | Curve Point 13 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 38 | Curve Point 14 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 39 | Curve Point 15 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 40 | Curve Point 16 | Drawn Curve=1, All Bands Drive=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 41 | All Bands Drive |  | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 42 | All Bands Ceiling |  | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 43 | All Bands Gain |  | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 44 | Band 1 Drive | Bands Open=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 45 | Band 1 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 46 | Band 1 Gain | Bands Open=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 47 | Band 1 Solo |  | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 48 | Band 1 Mute |  | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 49 | Band 1 Bypass | All Bands Drive=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 50 | Band 2 Drive | Bands Open=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 51 | Band 2 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 52 | Band 2 Gain | Bands Open=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 53 | Band 2 Solo |  | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 54 | Band 2 Mute |  | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 55 | Band 2 Bypass | All Bands Drive=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 56 | Band 3 Drive | Bands Open=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 57 | Band 3 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 58 | Band 3 Gain | Bands Open=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 59 | Band 3 Solo |  | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 60 | Band 3 Mute |  | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 61 | Band 3 Bypass | All Bands Drive=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 62 | Band 4 Drive | Bands Open=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 63 | Band 4 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 64 | Band 4 Gain | Bands Open=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 65 | Band 4 Solo |  | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 66 | Band 4 Mute |  | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 67 | Band 4 Bypass | All Bands Drive=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 68 | Band 5 Drive | Bands=0.8, Bands Open=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 69 | Band 5 Ceiling | Bands=0.8, Bands Open=1, All Bands Ceiling=0.5 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 70 | Band 5 Gain | Bands=0.8, Bands Open=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 71 | Band 5 Solo | Bands=0.8, Bands Open=0 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 72 | Band 5 Mute | Bands=0.8, Bands Open=0 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 73 | Band 5 Bypass | Bands=0.8, All Bands Drive=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 74 | Band 6 Drive | Bands=1, Bands Open=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 75 | Band 6 Ceiling | Bands=1, Bands Open=1, All Bands Ceiling=0.5 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 76 | Band 6 Gain | Bands=1, Bands Open=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 77 | Band 6 Solo | Bands=1, Bands Open=0 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 78 | Band 6 Mute | Bands=1, Bands Open=0 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 79 | Band 6 Bypass | Bands=1, All Bands Drive=1 | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 80 | Bypass |  | skipped (render not deterministic)  | NOT SHOWN (0.004%) |
| 81 | Wet |  | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
| 82 | Delta |  | skipped (render not deterministic)  | NOT SHOWN (0.000%) |
