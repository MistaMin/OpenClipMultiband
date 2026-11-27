#!/usr/bin/env python3
"""One NaN sample at the input, in REAPER: does the output come back? (audit findings 1 and 2)

  nan.py <format> <label>    format VST3, CLAP or AU; label names the build (old, new);
                             writes reaper/nan-<label>-<format>/nan.json and prints a table

REAPER turns a NaN in a media item into 0 before the FX chain, so the NaN comes from a JSFX ahead of
the plugin (nan_once.jsfx, copied to REAPER's Effects/plugin-qa/ocm_nan_once.jsfx for the run), as from a faulty plug-in earlier in the chain. Each case renders a 1 kHz
sine at -6 dBFS twice, each time from a reopened project (a new instance): once with the JSFX set to
never, once with one NaN in the left channel. The output level from 3 s to the end must match the clean render
within 1 dB. Auto Gain is tested with one band (no crossover) and Output Gain +12 dB, NaN at the
11th sample: a frozen Auto Gain stays at 0 dB, 12 dB louder than the clean render.
"""
import json, os, struct, subprocess, sys

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.expanduser("~/.claude/skills/plugin-daw-test/scripts"))
import reaper_qa as rq  # noqa: E402

SR, SECONDS = 48000, 8
FX = {"VST3": "VST3: OpenClip Multiband", "CLAP": "CLAP: OpenClip Multiband", "AU": "AU: OpenClip Multiband (OpenGrid)"}
CASES = [  # (label, {param name: normalized value}, NaN sample)
    ("defaults (4 bands, ceiling on)", {}, SR + 12),
    ("Output Ceiling off", {"Output Ceiling On": 0.0}, SR + 12),
    ("Auto Gain, 1 band, Output +12 dB", {"Bands": 0.0, "Auto Gain": 1.0, "Output Gain": 0.75}, 10),
]
REMOVE_SELECTED_TRACKS = 40005


def write_float_wav(path, data):
    raw = data.astype("<f4").tobytes()
    with open(path, "wb") as f:
        f.write(b"RIFF" + struct.pack("<I", 36 + len(raw)) + b"WAVEfmt " +
                struct.pack("<IHHIIHH", 16, 3, 2, SR, SR * 8, 8, 32) + b"data" + struct.pack("<I", len(raw)) + raw)


def signal(qa):
    t = np.arange(SR * SECONDS) / SR
    path = os.path.join(qa, "sine.wav")
    write_float_wav(path, np.stack([0.5 * np.sin(2 * np.pi * 1000 * t)] * 2, axis=1))
    return path


def with_nan_source(rpp, out, nan_at):
    """The saved project with nan_once.jsfx as the first effect of its FX chain."""
    text = open(rpp).read()
    head, sep, rest = text.partition("<FXCHAIN")
    chain, bypass, tail = rest.partition("BYPASS ")
    assert sep and bypass, "no FX chain in " + rpp
    js = 'BYPASS 0 0 0\n<JS "%s" ""\n%d %s\n>\nFLOATPOS 0 0 0 0\nWAK 0 0\n' % (
        "plugin-qa/ocm_nan_once.jsfx", nan_at, " ".join(["-"] * 63))
    open(out, "w").write(head + sep + chain + js + bypass + tail)


def rms_db(path):
    _, ch = rq.read_wav(path)
    tail = np.concatenate([c[3 * SR:] for c in ch])
    return 20 * np.log10(max(np.sqrt(np.mean(tail ** 2)), 1e-15)) if np.all(np.isfinite(tail)) else float("nan")


def main(fmt, label):
    qa = os.path.join(HERE, "reaper", "nan-%s-%s" % (label, fmt))
    os.makedirs(qa, exist_ok=True)
    rq.launch(qa)
    b = rq.Bridge(qa)
    rows = []
    try:
        def render(name, values, nan_at):
            b.call("add_fx", name=FX[fmt])
            b.call("add_audio", path=sine)  # also selects that track
            index = {p["name"]: p["index"] for p in b.call("params")["params"]}
            for n, v in values.items():
                b.call("set_param", index=str(index[n]), value=repr(v))
            saved = os.path.join(qa, name + "-saved.rpp")
            b.call("save_project", path=saved)
            b.call("action", id=str(REMOVE_SELECTED_TRACKS))
            with_nan_source(saved, os.path.join(qa, name + ".rpp"), nan_at)
            b.call("open_project", path=os.path.join(qa, name + ".rpp"))
            b.call("render", outdir=qa, name=name, seconds=str(SECONDS))
            b.call("action", id=str(REMOVE_SELECTED_TRACKS))
            return rms_db(os.path.join(qa, name + ".wav"))

        sine = signal(qa)
        for k, (case, values, nan_at) in enumerate(CASES):
            want = render("case%d-clean" % k, values, -1)
            got = render("case%d-nan" % k, values, nan_at)
            b.call("render", outdir=qa, name="empty%d" % k, seconds="1")  # the case's tracks are gone
            _, ch = rq.read_wav(os.path.join(qa, "empty%d.wav" % k))
            left = max(float(np.max(np.abs(c))) for c in ch)
            ok = abs(got - want) < 1.0 and left == 0.0
            rows.append({"case": case, "nan_at_sample": nan_at, "clean_db": round(want, 2), "nan_db": round(got, 2),
                         "leftover_peak": left, "pass": bool(ok)})
            print("%-34s clean %7.2f dB   one NaN %7.2f dB   leftover %g   %s" % (
                case, want, got, left, "PASS" if ok else "FAIL"), flush=True)
    finally:
        json.dump(rows, open(os.path.join(qa, "nan.json"), "w"), indent=1)
        subprocess.run([sys.executable, rq.__file__, "quit", "--qa-dir", qa], capture_output=True)


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
