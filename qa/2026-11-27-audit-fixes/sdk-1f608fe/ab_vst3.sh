#!/bin/sh
# VST3 on OpenPluginSDK 19fac89 against 1f608fe, same plug-in tree, same harness (finding Q5).
# Installs the 19fac89 VST3, runs crossbuild (X1, X2) and the suite's checks twice, then puts the 1f608fe build back.
cd /Volumes/Developer/OpenGrid/OpenClipMultiband/qa/2026-11-27-audit-fixes/sdk-1f608fe
OLD=${OLD:?the VST3 built against SDK 19fac89}
NEW=/Volumes/Developer/OpenGrid/OpenClipMultiband/dist/signed/OpenClipMultiband.vst3
DEST=~/Library/Audio/Plug-Ins/VST3/OpenClipMultiband.vst3
S=~/.claude/skills/plugin-daw-test/scripts/reaper_qa.py
rm -rf "$DEST" && ditto "$OLD" "$DEST"
for r in 1 2; do
  echo "round $r (19fac89) $(uptime | sed 's/.*averages: //')"
  (cd ../../2026-11-26-sdk-port && python3 crossbuild.py run X$r VST3 > /dev/null 2>&1)
  D=reaper/ab-19fac89-$r; mkdir -p $D
  python3 $S launch --qa-dir $D > /dev/null 2>&1
  python3 $S suite --qa-dir $D --fx "VST3: OpenClip Multiband" --only '^no parameter matches this$' > $D.suite.log 2>&1
  python3 $S quit --qa-dir $D > /dev/null 2>&1
  grep "deterministic" $D.suite.log
done
rm -rf "$DEST" && ditto "$NEW" "$DEST" && diff -rq "$NEW" "$DEST" > /dev/null && echo "1f608fe VST3 restored"
