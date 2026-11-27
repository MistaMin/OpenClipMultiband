#!/bin/sh
# The checks that need a build, on OpenPluginSDK 1f608fe (2026-11-27).
cd /Volumes/Developer/OpenGrid/OpenClipMultiband
Q=qa/2026-11-27-audit-fixes/sdk-1f608fe
LOG="$HOME/Library/Logs/OpenClipMultiband/panic.log"
uptime > $Q/load.txt
python3 "/Volumes/Developer/Dev Projects/DSP Engineering Tool/scripts/run_tests.py" --tier full > $Q/full-tier.txt 2>&1
mkdir -p $Q/test-list && cp target/openplugin-dev/logs/tree-T-*.log $Q/test-list/
uptime >> $Q/load.txt
cargo test --release --workspace > $Q/workspace.log 2>&1; echo "workspace exit $?" >> $Q/workspace.log
# A4: a failing test must not write the panic log; the same binary without CARGO_MANIFEST_DIR (as in a host) must
cp "$LOG" $Q/panic.log.backup
cat > tests/zz_panic_log.rs <<'T'
#[test]
fn a_failing_test_does_not_write_the_panic_log() {
    let _plugin = openclipmultiband::OpenClipMultiband::default(); // installs the panic hook
    panic!("deliberate: QA of the panic log (audit finding 4)");
}
T
before=$(wc -l < "$LOG")
cargo test --release --test zz_panic_log > $Q/panic-test.log 2>&1; echo "cargo test exit $?" >> $Q/panic-test.log
under_cargo=$(wc -l < "$LOG")
bin=$(ls -t target/release/deps/zz_panic_log-* | grep -v '\.d$' | head -1)
env -u CARGO_MANIFEST_DIR "$bin" > $Q/panic-host-like.log 2>&1; echo "direct run exit $?" >> $Q/panic-host-like.log
host_like=$(wc -l < "$LOG")
tail -4 "$LOG" > $Q/panic-host-like-lines.txt
cp $Q/panic.log.backup "$LOG" && rm $Q/panic.log.backup
rm tests/zz_panic_log.rs
printf 'panic.log lines: %s before, %s after the failing test under cargo, %s after the same binary without CARGO_MANIFEST_DIR (log restored afterwards: %s lines)\n' \
  "$before" "$under_cargo" "$host_like" "$(wc -l < "$LOG")" > $Q/panic-log.txt
# A13: the six documented screenshots against docs/images
SHOTS=$PWD/$Q/shots cargo test --release --lib screenshots -- --ignored > $Q/screenshots-test.log 2>&1
python3 - "$Q" > $Q/screenshots.txt <<'P'
import os, sys
from PIL import Image, ImageChops
q = sys.argv[1]; ok = True
for f in sorted(os.listdir(os.path.join(q, "shots"))):
    name = os.path.splitext(f)[0].removesuffix("-1x")
    a = Image.open(os.path.join(q, "shots", f)).convert("RGB"); b = Image.open("docs/images/%s.png" % name).convert("RGB")
    same = a.size == b.size and ImageChops.difference(a, b).getbbox() is None
    ok &= same
    print("%s: %dx%d vs %dx%d: %s" % (name, *a.size, *b.size, "pixel-identical" if same else "DIFFERENT"))
print("RESULT: %s" % ("PASS" if ok else "FAIL"))
P
uptime >> $Q/load.txt
