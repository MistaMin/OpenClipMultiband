#!/usr/bin/env python3
"""Which parameter change leaves the VST3 silent on OpenPluginSDK 1f608fe? (finding Q5)

  vst3_silence.py <format> <label>   writes reaper/silence-<label>-<format>/silence.json and prints a table

Each case: a new track with a new instance and the suite's test signal, one change, 2 s, a warm-up render, a render
(as crossbuild.py does). Prints the render's peak and the host's latency (pdc) before and after the change.
"""
import json, os, subprocess, sys, time

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.expanduser("~/.claude/skills/plugin-daw-test/scripts"))
import reaper_qa as rq  # noqa: E402

SIGNAL = os.path.join(HERE, "..", "..", "2026-11-26-release", "reaper", "VST3", "testsignal.wav")
FX = {"VST3": "VST3: OpenClip Multiband", "CLAP": "CLAP: OpenClip Multiband", "AU": "AU: OpenClip Multiband (OpenGrid)"}
CASES = [  # (label, {param name: normalized value})
    ("no change", {}),
    ("Zero Latency on (latency 126 -> 0)", {"Zero Latency": 1.0}),
    ("Bands 3 (no latency change)", {"Bands": 0.4}),
    ("Oversampling 4x (latency 126 -> 151)", {"Oversampling": 0.4}),
]
REMOVE_SELECTED_TRACKS = 40005


def main(fmt, label):
    qa = os.path.join(HERE, "reaper", "silence-%s-%s" % (label, fmt))
    os.makedirs(qa, exist_ok=True)
    rq.launch(qa)
    b = rq.Bridge(qa)
    rows = []
    try:
        for k, (case, values) in enumerate(CASES):
            b.call("add_fx", name=FX[fmt])
            b.call("add_audio", path=os.path.abspath(SIGNAL))  # also selects the track
            index = {p["name"]: p["index"] for p in b.call("params")["params"]}
            pdc0 = b.call("info")["latency"]
            for n, v in values.items():
                b.call("set_param", index=str(index[n]), value=repr(v))
            time.sleep(2)
            b.call("render", outdir=qa, name="case%d-warmup" % k, seconds="4")
            b.call("render", outdir=qa, name="case%d" % k, seconds="4")
            pdc1 = b.call("info")["latency"]
            _, ch = rq.read_wav(os.path.join(qa, "case%d.wav" % k))
            peak = 20 * np.log10(max(max(float(np.max(np.abs(c))) for c in ch), 1e-12))
            rows.append({"case": case, "peak_db": round(peak, 1), "pdc_before": pdc0, "pdc_after": pdc1})
            print("%-40s peak %7.1f dBFS   pdc %s -> %s" % (case, peak, pdc0, pdc1), flush=True)
            b.call("action", id=str(REMOVE_SELECTED_TRACKS))
    finally:
        json.dump(rows, open(os.path.join(qa, "silence.json"), "w"), indent=1)
        subprocess.run([sys.executable, rq.__file__, "quit", "--qa-dir", qa], capture_output=True)


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
