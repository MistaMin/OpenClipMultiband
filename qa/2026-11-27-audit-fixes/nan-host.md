# Can a NaN reach the plug-in in REAPER? (A1, A2, Q1)

REAPER 7, test instance from `reaper_qa.py launch`, the installed 397ba8c VST3 (the build without fixes 1 and 2), 48 kHz.
In process, that build stays silent for good after one NaN at defaults (`tests/host_behaviour.rs`, guard removed).

| Route | Files | Result |
|---|---|---|
| NaN in a 32-bit float WAV item | `nan.py` (first version), `reaper/nan-old-VST3-itemnan/` | the old build renders bit-identical to the clean item (cases 0, 1) and Auto Gain is not frozen (case 2): REAPER passes 0 for the NaN |
| NaN written by a JSFX ahead of the plug-in | `nan.py`, `nan_once.jsfx`, `reaper/nan-old-VST3/` | the old build recovers within about 115 ms: it got a finite value |
| JSFX NaN, then a probe JSFX (range test, since EEL2's `!=` compares with a tolerance) | `probe.py`, `ocm_nan_probe.jsfx`, `reaper/probe/` | the probe sees 0 at the sample, no mark |
| `inf - inf` checked inside the same JSFX | `probe_self.py`, `ocm_nan_self.jsfx`, `reaper/probe-self/` | finite: EEL2 does not produce a NaN at all |

So in REAPER no NaN reaches a plug-in from items or from JSFX, and fixes 1 and 2 cannot be shown in this host. The
F9 REAPER suites used a 24-bit PCM test signal, which cannot hold a NaN. The JSFX files were copied to REAPER's
`Effects/plugin-qa/` for the runs and removed afterwards.

Case 2 of `reaper/nan-old-VST3` is not valid: its project was saved straight after the parameter writes, before the
plug-in processed a block (no audio device in the test instance), so it rendered with the default 4 bands (Q3).
