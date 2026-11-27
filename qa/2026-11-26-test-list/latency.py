#!/usr/bin/env python3
"""The host's delay compensation after Anti-Alias, Oversampling and Zero Latency changes, in REAPER.

  latency.py <format>     format VST3, CLAP or AU; writes reaper/latency-<format>/latency.json and prints a table

Flint, one band, Mix 0 and the Output Ceiling off make the plug-in a pure delay (unit test
`mix_zero_with_output_ceiling_off_is_a_delayed_wire`). After each change the script reads REAPER's plug-in delay
(`pdc`) at once and every 50 ms, then waits 2 s, renders 4 s and compares the render with the source: with the
right compensation the render nulls against it.
"""
import json, os, subprocess, sys, time

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.expanduser("~/.claude/skills/plugin-daw-test/scripts"))
import reaper_qa as rq  # noqa: E402

SIGNAL = os.path.join(HERE, "..", "2026-11-26-release", "reaper", "VST3", "testsignal.wav")
FX = {"VST3": "VST3: OpenClip Multiband", "CLAP": "CLAP: OpenClip Multiband", "AU": "AU: OpenClip Multiband (OpenGrid)"}
BASE = {"Algorithm": 0.2, "Bands": 0.0, "Dry/Wet Mix": 0.0, "Output Ceiling On": 0.0}  # Flint, 1 band, dry, no ceiling
STEPS = [  # (label, {param name: normalized value}); each step starts from the one before
    ("Hann-B1, 1x", {}),
    ("Hann-B1, 4x", {"Oversampling": 0.2}),
    ("Hann-B1, 64x", {"Oversampling": 1.0}),
    ("No ADAA, 64x", {"Anti-Alias": 1.0}),
    ("Adaptive, 4x", {"Anti-Alias": 0.5, "Oversampling": 0.2}),
    ("Zero Latency on", {"Zero Latency": 1.0}),
    ("Zero Latency off", {"Zero Latency": 0.0}),
    ("Hann-B1, 1x again", {"Anti-Alias": 0.0, "Oversampling": 0.0}),
]


def main(fmt):
    qa = os.path.join(HERE, "reaper", "latency-%s" % fmt)
    os.makedirs(qa, exist_ok=True)
    rq.launch(qa)
    b = rq.Bridge(qa)
    rows = []
    try:
        b.call("add_fx", name=FX[fmt])
        b.call("add_audio", path=os.path.abspath(SIGNAL))
        ps = b.call("params")["params"]
        index = {p["name"]: p["index"] for p in ps}
        with open(os.path.join(HERE, "params-%s.tsv" % fmt), "w") as f:  # the host's list at defaults (S13)
            f.write("index\tident\tname\tdefault\n" + "".join("%d\t%s\t%s\t%s\n" % (p["index"], p["ident"], p["name"], p["value"]) for p in ps))

        def pdc():
            return int(float(b.call("info")["latency"] or 0))

        def write(values):
            for name, v in values.items():
                b.call("set_param", index=str(index[name]), value=repr(v))

        write(BASE)
        for label, change in STEPS:
            before = pdc()
            t0 = time.time()
            write(change)
            seen = [(0.0, pdc())]
            while time.time() - t0 < 2.0:
                time.sleep(0.05)
                v = pdc()
                if v != seen[-1][1]:
                    seen.append((round(time.time() - t0, 2), v))
            b.call("render", outdir=qa, name="warmup", seconds="4")
            name = "step%d" % len(rows)
            b.call("render", outdir=qa, name=name, seconds="4")
            diff = rq.difference_db(os.path.join(qa, name + ".wav"), os.path.abspath(SIGNAL))
            rows.append({"step": label, "pdc_before": before, "pdc_changes": seen, "pdc_final": pdc(),
                         "render_vs_source_db": round(diff, 1)})
            print("%-20s pdc %4d -> %4d (changes at %s s)  render vs source %.1f dB" % (
                label, before, rows[-1]["pdc_final"], ", ".join("%.2f" % t for t, _ in seen[1:]) or "-", diff), flush=True)
    finally:
        json.dump(rows, open(os.path.join(qa, "latency.json"), "w"), indent=1)
        # the CLI's quit also kills REAPER by its pid file when the bridge does not answer
        subprocess.run([sys.executable, rq.__file__, "quit", "--qa-dir", qa], capture_output=True)


if __name__ == "__main__":
    main(sys.argv[1])
