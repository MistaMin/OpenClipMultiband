#!/usr/bin/env python3
"""Old build against new build in REAPER, through the plugin-daw-test bridge.

  crossbuild.py run <phase> <format>   phase A (1.0.0 Beta installed) or B (port installed); format VST3, CLAP or AU
  crossbuild.py compare [phase]        writes crossbuild.txt from what A and B (or another phase) left

Each run: one REAPER session, the plug-in on a track with the suite's test signal, renders at three settings, the
"heavy" values saved in a project, then project A-<format>.rpp (saved by phase A) reopened and rendered again.
Every change gets 2 s and a warm-up render before a render, a save or a read: a render started at once after a
change lets the change land at a time that differs from run to run, so two runs of one build differ for a moment.
"""
import json, os, subprocess, sys, time

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.expanduser("~/.claude/skills/plugin-daw-test/scripts"))
import reaper_qa as rq  # noqa: E402

SIGNAL = os.path.join(HERE, "..", "2026-11-26-release", "reaper", "VST3", "testsignal.wav")
FX = {"VST3": "VST3: OpenClip Multiband", "CLAP": "CLAP: OpenClip Multiband", "AU": "AU: OpenClip Multiband (OpenGrid)"}
# Normalized values by parameter index; anything not named stays at its default.
SETTINGS = {
    "default": {},
    # 6 bands, Ember, Shape 30 %, Character 50 %, Blend 50 %, Mix 90 %, drives up, adaptive ADAA, 16x
    "heavy": {0: 0.7, 5: 1.0, 12: 0.6, 13: 0.3, 14: 0.5, 15: 0.5, 16: 0.9, 17: 0.6, 21: 0.5, 22: 0.6, 41: 0.6},
    # Zero Latency, 3 bands, Ghost, drawn curve, drives up
    "alt": {2: 1.0, 5: 0.4, 12: 0.4, 24: 1.0, 17: 0.5, 41: 0.7},
}
KEYS = sorted({k for s in SETTINGS.values() for k in s})


def run(phase, fmt):
    qa = os.path.join(HERE, "reaper", "%s-%s" % (phase, fmt))
    os.makedirs(qa, exist_ok=True)
    rq.launch(qa)
    b = rq.Bridge(qa)
    out = {}
    try:
        b.call("add_fx", name=FX[fmt])
        b.call("add_audio", path=os.path.abspath(SIGNAL))
        defaults = {p["index"]: p["value"] for p in b.call("params")["params"]}
        def apply(s):
            for k in KEYS:
                b.call("set_param", index=str(k), value=repr(s.get(k, defaults[k])))
            settle()

        def settle():  # let every change (and a latency change) land before anything is measured or saved
            time.sleep(2)
            b.call("render", outdir=qa, name="warmup", seconds="4")

        for name, s in SETTINGS.items():
            apply(s)
            b.call("render", outdir=qa, name=name, seconds="4")
        apply(SETTINGS["heavy"])
        out["saved"] = b.call("params")["params"]
        b.call("save_project", path=os.path.join(qa, "%s-%s.rpp" % (phase, fmt)))
        a_project = os.path.join(HERE, "reaper", "A-%s" % fmt, "A-%s.rpp" % fmt)
        b.call("open_project", path=a_project)
        settle()
        out["reopened"] = b.call("params")["params"]
        b.call("render", outdir=qa, name="reopened", seconds="4")
    finally:
        json.dump(out, open(os.path.join(qa, "params.json"), "w"), indent=1)
        # the CLI's quit also kills REAPER by its pid file when the bridge does not answer
        subprocess.run([sys.executable, rq.__file__, "quit", "--qa-dir", qa], capture_output=True)


def compare(new="B"):
    import numpy as np
    lines, ok = [], True
    for fmt in FX:
        a_dir, b_dir = (os.path.join(HERE, "reaper", "%s-%s" % (p, fmt)) for p in ("A", new))
        for name in list(SETTINGS) + ["reopened"]:
            fa, fb = os.path.join(a_dir, name + ".wav"), os.path.join(b_dir, name + ".wav")
            _, ca = rq.read_wav(fa)
            _, cb = rq.read_wav(fb)
            same = len(ca[0]) == len(cb[0]) and all(np.array_equal(x, y) for x, y in zip(ca, cb))
            ok &= same
            lines.append("%-4s %-8s new vs 1.0.0 Beta: %s (difference %.1f dB, peak %.1f dBFS, %d frames)" % (
                fmt, name, "bit-identical" if same else "DIFFERENT", rq.difference_db(fa, fb),
                rq.analyse(fb)["peakDb"], len(cb[0])))
        # the reopened project against the heavy render of the same build, and the values it reopened with
        _, h = rq.read_wav(os.path.join(b_dir, "heavy.wav"))
        _, r = rq.read_wav(os.path.join(b_dir, "reopened.wav"))
        same = all(np.array_equal(x, y) for x, y in zip(h, r))
        ja, jb = (json.load(open(os.path.join(d, "params.json"))) for d in (a_dir, b_dir))
        pa, pb = ja["saved"], jb["reopened"]

        def differ(xs, ys):
            return [x["name"] for x, y in zip(xs, ys) if abs(x["value"] - y["value"]) > 1e-6 or x["text"] != y["text"]]
        diff = differ(pa, pb)
        old_own = differ(pa, ja["reopened"])  # what 1.0.0 Beta itself does with its own project
        as_old = not differ(ja["reopened"], pb) and len(ja["reopened"]) == len(pb)
        ok &= same and as_old and not (set(diff) - set(old_own))
        lines.append("%-4s 1.0.0 Beta project reopened by the new build: %d of %d values and texts equal to the saved ones%s; "
                     "%s 1.0.0 Beta reopening it; render %s the new build's heavy render" % (
            fmt, len(pb) - len(diff), len(pa),
            (" (differ: %s; 1.0.0 Beta reopens the same: %s)" % (", ".join(diff), "yes" if set(diff) <= set(old_own) else "NO")) if diff else "",
            "every value equal to" if as_old else "DIFFERENT from", "bit-identical to" if same else "DIFFERENT from"))
    lines.append("RESULT: %s" % ("PASS" if ok else "FAIL"))
    open(os.path.join(HERE, "crossbuild.txt" if new == "B" else "crossbuild-%s.txt" % new), "w").write("\n".join(lines) + "\n")
    print("\n".join(lines))
    return ok


if __name__ == "__main__":
    if sys.argv[1] == "run":
        run(sys.argv[2], sys.argv[3])
    else:
        sys.exit(0 if compare(*sys.argv[2:]) else 1)
