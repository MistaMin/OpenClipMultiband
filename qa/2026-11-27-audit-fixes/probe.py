#!/usr/bin/env python3
"""Does a NaN written by one JSFX reach the next effect in REAPER? nan_once -> probe -> plugin (bypassed)."""
import os, re, subprocess, sys
import numpy as np
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE); sys.path.insert(0, os.path.expanduser("~/.claude/skills/plugin-daw-test/scripts"))
import nan, reaper_qa as rq  # noqa: E402

qa = os.path.join(HERE, "reaper", "probe")
os.makedirs(qa, exist_ok=True)
rq.launch(qa)
b = rq.Bridge(qa)
try:
    b.call("add_fx", name=nan.FX["VST3"])
    b.call("add_audio", path=nan.signal(qa))
    b.call("save_project", path=os.path.join(qa, "saved.rpp"))
    text = open(os.path.join(qa, "saved.rpp")).read()
    head, sep, rest = text.partition("<FXCHAIN")
    chain, bypass, tail = rest.partition("BYPASS 0 0 0")
    js = lambda path, v: 'BYPASS 0 0 0\n<JS "%s" ""\n%s %s\n>\nFLOATPOS 0 0 0 0\nWAK 0 0\n' % (path, v, " ".join(["-"] * 63))
    open(os.path.join(qa, "probe.rpp"), "w").write(head + sep + chain + js("plugin-qa/ocm_nan_once.jsfx", 48012)
                                                    + js("plugin-qa/ocm_nan_probe.jsfx", "-") + "BYPASS 1 0 0" + tail)
    b.call("open_project", path=os.path.join(qa, "probe.rpp"))
    b.call("render", outdir=qa, name="probe", seconds="2")
    _, ch = rq.read_wav(os.path.join(qa, "probe.wav"))
    marks = np.nonzero(ch[1] > 0.9)[0]
    print("right-channel marks at samples:", marks[:5], "count", len(marks), "| left at 48012:", ch[0][48010:48015])
finally:
    subprocess.run([sys.executable, rq.__file__, "quit", "--qa-dir", qa], capture_output=True)
