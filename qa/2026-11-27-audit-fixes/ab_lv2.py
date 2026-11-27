#!/usr/bin/env python3
"""OpenGrid9 LV2 repeat render, 1.0.0 Beta against the new build, same harness (finding Q4).

  ab_lv2.py <1.0.0 Beta OpenClipMultiband.lv2> <new OpenClipMultiband.lv2>

For each build, in turn: installs the bundle as the user LV2, starts OpenGrid9 headless, and renders warmup, base and
base2 as the suite's "deterministic render" check does; prints where base and base2 differ. Puts the new bundle back.
"""
import os, shutil, subprocess, sys, time

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
SCRIPTS = os.path.expanduser("~/.claude/skills/plugin-daw-test/scripts")
sys.path.insert(0, SCRIPTS)
import reaper_qa as rq  # noqa: E402
import og9_qa as oq  # noqa: E402

LV2 = os.path.expanduser("~/Library/Audio/Plug-Ins/LV2/OpenClipMultiband.lv2")


def install(bundle):
    shutil.rmtree(LV2, ignore_errors=True)
    shutil.copytree(bundle, LV2, symlinks=True)


def repeat(label, bundle):
    qa = os.path.join(HERE, "og9-ab", label)
    os.makedirs(qa, exist_ok=True)
    install(bundle)
    oq.launch(qa)
    b = oq.Og9Bridge(qa)
    try:
        b.call("add_fx", name="LV2: OpenClip Multiband")
        b.call("add_audio", path=rq.make_test_signal(qa, 3.0))
        files = []
        for name in ("warmup", "base", "base2"):
            path = b.call("render", seconds=3.0, name=name)["file"]
            for _ in range(100):
                if os.path.exists(path) and os.path.getsize(path) > 44:
                    break
                time.sleep(0.05)
            files.append(path)
    finally:
        subprocess.run([sys.executable, os.path.join(SCRIPTS, "og9_qa.py"), "quit", "--qa-dir", qa], capture_output=True)
    sr, a = rq.read_wav(files[1])
    _, c = rq.read_wav(files[2])
    d = np.max(np.abs(np.stack(a) - np.stack(c)), axis=0)
    nz = np.nonzero(d)[0]
    last = "%d (%.1f ms)" % (nz[-1], 1000 * nz[-1] / sr) if len(nz) else "-"
    return "%-10s base vs base2: %s; %d differing frames, last %s; peak difference %.1f dB" % (
        label, "%.1f dB" % rq.difference_db(files[1], files[2]), len(nz), last, 20 * np.log10(max(d.max(), 1e-12)))


if __name__ == "__main__":
    beta, new = sys.argv[1], sys.argv[2]
    try:
        lines = [repeat("beta", beta), repeat("new", new), repeat("beta-2", beta), repeat("new-2", new)]
    finally:
        install(new)
    open(os.path.join(HERE, "og9-ab", "result.txt"), "w").write("\n".join(lines) + "\n")
    print("\n".join(lines))
