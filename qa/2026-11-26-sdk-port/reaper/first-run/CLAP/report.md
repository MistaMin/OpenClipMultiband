# Plugin QA: CLAP: OpenClip Multiband

| Check | Status | Detail |
|---|---|---|
| loads | PASS | CLAP: OpenClip Multiband (OpenGrid), 83 parameters |
| finite output | PASS | peak -4.1 dBFS |
| not silent | PASS | rms -14.9 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1080x821, 0.0% animates by itself |
| every parameter changes the sound | FAIL | no sound at either end of the sweep, so these were never heard (the audio went silent mid-run, or the parameter mutes at both ends and needs an audio: exception): Output Gain, Zero Latency, Auto Gain, Delta, Bands, Crossover Slope, Crossover 1, Crossover 2, Crossover 3, Crossover 4, Crossover 5, Algorithm, Shape, Character, Single/Multi Blend, Dry/Wet Mix, Single Drive, Single Ceiling, Output Ceiling On, Output Ceiling, Anti-Alias, Oversampling, Drawn Curve, Curve Point 1, Curve Point 2, Curve Point 3, Curve Point 4, Curve Point 5, Curve Point 6, Curve Point 7, Curve Point 8, Curve Point 9, Curve Point 10, Curve Point 11, Curve Point 12, Curve Point 13, Curve Point 14, Curve Point 15, Curve Point 16, All Bands Drive, All Bands Ceiling, All Bands Gain, Band 1 Drive, Band 1 Ceiling, Band 1 Gain, Band 1 Solo, Band 1 Mute, Band 1 Bypass, Band 2 Drive, Band 2 Ceiling, Band 2 Gain, Band 2 Solo, Band 2 Mute, Band 2 Bypass, Band 3 Drive, Band 3 Ceiling, Band 3 Gain, Band 3 Solo, Band 3 Mute, Band 3 Bypass, Band 4 Drive, Band 4 Ceiling, Band 4 Gain, Band 4 Solo, Band 4 Mute, Band 4 Bypass, Band 5 Drive, Band 5 Ceiling, Band 5 Gain, Band 5 Solo, Band 5 Mute, Band 5 Bypass, Band 6 Drive, Band 6 Ceiling, Band 6 Gain, Band 6 Solo, Band 6 Mute, Band 6 Bypass, Bypass, Wet, Delta |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Input Gain |  | ok (-4.1 dB) | ok (2.306%) |
| 1 | Output Gain |  | SILENT (-4.1 dB) | ok (2.306%) |
| 2 | Zero Latency |  | SILENT (-4.1 dB) | ok (2.542%) |
| 3 | Auto Gain |  | SILENT (-4.1 dB) | ok (2.558%) |
| 4 | Delta |  | SILENT (-4.1 dB) | ok (2.438%) |
| 5 | Bands |  | SILENT (-4.1 dB) | ok (3.715%) |
| 6 | Crossover Slope |  | SILENT (-4.1 dB) | ok (2.457%) |
| 7 | Crossover 1 |  | SILENT (-4.1 dB) | ok (2.407%) |
| 8 | Crossover 2 |  | SILENT (-4.1 dB) | ok (2.405%) |
| 9 | Crossover 3 |  | SILENT (-4.1 dB) | ok (2.343%) |
| 10 | Crossover 4 | Bands=0.8 | SILENT (-240.0 dB) | ok (3.700%) |
| 11 | Crossover 5 | Bands=1 | SILENT (-240.0 dB) | ok (3.715%) |
| 12 | Algorithm | All Bands Drive=1 | SILENT (-240.0 dB) | ok (2.575%) |
| 13 | Shape | All Bands Drive=1 | SILENT (-240.0 dB) | ok (2.468%) |
| 14 | Character | All Bands Drive=1 | SILENT (-240.0 dB) | ok (2.485%) |
| 15 | Single/Multi Blend | All Bands Drive=1 | SILENT (-240.0 dB) | ok (2.393%) |
| 16 | Dry/Wet Mix | All Bands Drive=1 | SILENT (-240.0 dB) | ok (2.393%) |
| 17 | Single Drive | Single/Multi Blend=0 | SILENT (-240.0 dB) | ok (2.348%) |
| 18 | Single Ceiling | Single/Multi Blend=0 | SILENT (-240.0 dB) | ok (2.304%) |
| 19 | Output Ceiling On | Input Gain=1 | SILENT (-240.0 dB) | ok (2.306%) |
| 20 | Output Ceiling | Input Gain=1 | SILENT (-240.0 dB) | ok (2.316%) |
| 21 | Anti-Alias | All Bands Drive=1 | SILENT (-240.0 dB) | ok (2.425%) |
| 22 | Oversampling | All Bands Drive=1 | SILENT (-240.0 dB) | ok (2.571%) |
| 23 | Bands Open |  | excused (-4.1 dB) | ok (4.126%) |
| 24 | Drawn Curve | All Bands Drive=1 | SILENT (-240.0 dB) | ok (2.628%) |
| 25 | Curve Point 1 | Drawn Curve=1, All Bands Drive=1 | SILENT (-240.0 dB) | ok (2.760%) |
| 26 | Curve Point 2 | Drawn Curve=1, All Bands Drive=1 | SILENT (-240.0 dB) | ok (2.735%) |
| 27 | Curve Point 3 | Drawn Curve=1, All Bands Drive=1 | SILENT (-240.0 dB) | ok (2.713%) |
| 28 | Curve Point 4 | Drawn Curve=1, All Bands Drive=1 | SILENT (-240.0 dB) | ok (2.700%) |
| 29 | Curve Point 5 | Drawn Curve=1, All Bands Drive=1 | SILENT (-240.0 dB) | ok (2.686%) |
| 30 | Curve Point 6 | Drawn Curve=1, All Bands Drive=1 | SILENT (-240.0 dB) | ok (2.668%) |
| 31 | Curve Point 7 | Drawn Curve=1, All Bands Drive=1 | SILENT (-240.0 dB) | ok (2.641%) |
| 32 | Curve Point 8 | Drawn Curve=1, All Bands Drive=1 | SILENT (-240.0 dB) | ok (2.628%) |
| 33 | Curve Point 9 | Drawn Curve=1, All Bands Drive=1 | SILENT (-240.0 dB) | ok (2.628%) |
| 34 | Curve Point 10 | Drawn Curve=1, All Bands Drive=1 | SILENT (-240.0 dB) | ok (2.628%) |
| 35 | Curve Point 11 | Drawn Curve=1, All Bands Drive=1 | SILENT (-240.0 dB) | ok (2.628%) |
| 36 | Curve Point 12 | Drawn Curve=1, All Bands Drive=1 | SILENT (-240.0 dB) | ok (2.628%) |
| 37 | Curve Point 13 | Drawn Curve=1, All Bands Drive=1 | SILENT (-240.0 dB) | ok (2.628%) |
| 38 | Curve Point 14 | Drawn Curve=1, All Bands Drive=1 | SILENT (-240.0 dB) | ok (2.628%) |
| 39 | Curve Point 15 | Drawn Curve=1, All Bands Drive=1 | SILENT (-240.0 dB) | ok (2.628%) |
| 40 | Curve Point 16 | Drawn Curve=1, All Bands Drive=1 | SILENT (-240.0 dB) | ok (2.628%) |
| 41 | All Bands Drive |  | SILENT (-4.1 dB) | ok (2.393%) |
| 42 | All Bands Ceiling |  | SILENT (-4.1 dB) | ok (2.259%) |
| 43 | All Bands Gain |  | SILENT (-4.1 dB) | ok (2.344%) |
| 44 | Band 1 Drive | Bands Open=1 | SILENT (-240.0 dB) | ok (4.135%) |
| 45 | Band 1 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | SILENT (-240.0 dB) | ok (4.519%) |
| 46 | Band 1 Gain | Bands Open=1 | SILENT (-240.0 dB) | ok (4.135%) |
| 47 | Band 1 Solo |  | SILENT (-4.1 dB) | ok (2.290%) |
| 48 | Band 1 Mute |  | SILENT (-4.1 dB) | ok (2.290%) |
| 49 | Band 1 Bypass | All Bands Drive=1 | SILENT (-240.0 dB) | ok (2.423%) |
| 50 | Band 2 Drive | Bands Open=1 | SILENT (-240.0 dB) | ok (4.134%) |
| 51 | Band 2 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | SILENT (-240.0 dB) | ok (4.583%) |
| 52 | Band 2 Gain | Bands Open=1 | SILENT (-240.0 dB) | ok (4.135%) |
| 53 | Band 2 Solo |  | SILENT (-4.1 dB) | ok (2.290%) |
| 54 | Band 2 Mute |  | SILENT (-4.1 dB) | ok (2.290%) |
| 55 | Band 2 Bypass | All Bands Drive=1 | SILENT (-240.0 dB) | ok (2.423%) |
| 56 | Band 3 Drive | Bands Open=1 | SILENT (-240.0 dB) | ok (4.134%) |
| 57 | Band 3 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | SILENT (-240.0 dB) | ok (4.583%) |
| 58 | Band 3 Gain | Bands Open=1 | SILENT (-240.0 dB) | ok (4.133%) |
| 59 | Band 3 Solo |  | SILENT (-4.1 dB) | ok (2.290%) |
| 60 | Band 3 Mute |  | SILENT (-4.1 dB) | ok (2.290%) |
| 61 | Band 3 Bypass | All Bands Drive=1 | SILENT (-240.0 dB) | ok (2.423%) |
| 62 | Band 4 Drive | Bands Open=1 | SILENT (-240.0 dB) | ok (4.134%) |
| 63 | Band 4 Ceiling | Bands Open=1, All Bands Ceiling=0.5 | SILENT (-240.0 dB) | ok (4.586%) |
| 64 | Band 4 Gain | Bands Open=1 | SILENT (-240.0 dB) | ok (4.133%) |
| 65 | Band 4 Solo |  | SILENT (-4.1 dB) | ok (2.290%) |
| 66 | Band 4 Mute |  | SILENT (-4.1 dB) | ok (2.290%) |
| 67 | Band 4 Bypass | All Bands Drive=1 | SILENT (-240.0 dB) | ok (2.423%) |
| 68 | Band 5 Drive | Bands=0.8, Bands Open=1 | SILENT (-240.0 dB) | ok (5.785%) |
| 69 | Band 5 Ceiling | Bands=0.8, Bands Open=1, All Bands Ceiling=0.5 | SILENT (-240.0 dB) | ok (5.754%) |
| 70 | Band 5 Gain | Bands=0.8, Bands Open=1 | SILENT (-240.0 dB) | ok (5.785%) |
| 71 | Band 5 Solo | Bands=0.8, Bands Open=0 | SILENT (-240.0 dB) | ok (3.728%) |
| 72 | Band 5 Mute | Bands=0.8, Bands Open=0 | SILENT (-240.0 dB) | ok (3.725%) |
| 73 | Band 5 Bypass | Bands=0.8, All Bands Drive=1 | SILENT (-240.0 dB) | ok (3.858%) |
| 74 | Band 6 Drive | Bands=1, Bands Open=1 | SILENT (-240.0 dB) | ok (5.956%) |
| 75 | Band 6 Ceiling | Bands=1, Bands Open=1, All Bands Ceiling=0.5 | SILENT (-240.0 dB) | ok (5.882%) |
| 76 | Band 6 Gain | Bands=1, Bands Open=1 | SILENT (-240.0 dB) | ok (5.956%) |
| 77 | Band 6 Solo | Bands=1, Bands Open=0 | SILENT (-240.0 dB) | ok (3.736%) |
| 78 | Band 6 Mute | Bands=1, Bands Open=0 | SILENT (-240.0 dB) | ok (3.736%) |
| 79 | Band 6 Bypass | Bands=1, All Bands Drive=1 | SILENT (-240.0 dB) | ok (3.870%) |
| 80 | Bypass |  | SILENT (-4.1 dB) | ok (2.533%) |
| 81 | Wet |  | SILENT (-4.1 dB) | ok (2.261%) |
| 82 | Delta |  | SILENT (-4.1 dB) | ok (2.287%) |
