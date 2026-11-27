#!/bin/sh
# Repeats of the two VST3 failures on OpenPluginSDK 1f608fe (finding Q5): crossbuild twice, the suite's checks twice.
cd /Volumes/Developer/OpenGrid/OpenClipMultiband/qa/2026-11-27-audit-fixes/sdk-1f608fe
S=~/.claude/skills/plugin-daw-test/scripts/reaper_qa.py
for r in 1 2; do
  echo "round $r $(uptime | sed 's/.*averages: //')"
  (cd ../../2026-11-26-sdk-port && python3 crossbuild.py run W$r VST3 > /dev/null 2>&1)
  D=reaper/repeat-VST3-$r; mkdir -p $D
  python3 $S launch --qa-dir $D > /dev/null 2>&1
  python3 $S suite --qa-dir $D --fx "VST3: OpenClip Multiband" --only '^no parameter matches this$' > $D.suite.log 2>&1
  python3 $S quit --qa-dir $D > /dev/null 2>&1
  grep "deterministic" $D.suite.log
done
