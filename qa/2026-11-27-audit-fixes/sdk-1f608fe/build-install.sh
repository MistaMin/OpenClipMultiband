#!/bin/sh
# Signed build of the final tree on OpenPluginSDK 1f608fe, PACE-signed AAX, install, full test list (2026-11-27).
cd /Volumes/Developer/OpenGrid/OpenClipMultiband
Q=qa/2026-11-27-audit-fixes/sdk-1f608fe
U=~/Library/Audio/Plug-Ins; A="/Library/Application Support/Avid/Audio/Plug-Ins"
(uptime; CODESIGN_IDENTITY="${CODESIGN_IDENTITY:?the owner's Developer ID}" ./scripts/build.sh; echo "build exit $?"; uptime) > $Q/build-signed.log 2>&1
grep -q "build exit 0" $Q/build-signed.log || exit 1
for b in vst3 component clap lv2 aaxplugin; do rm -rf dist/signed/OpenClipMultiband.$b && ditto dist/OpenClipMultiband.$b dist/signed/OpenClipMultiband.$b; done
~/.claude/skills/Make-Plugin-Release/scripts/sign_aax.sh dist/signed/OpenClipMultiband.aaxplugin 1D9994F0-E93C-11F1-8EC5-005056920FF7 mistamin > $Q/aax-sign.log 2>&1 || exit 1
for pair in "vst3:$U/VST3" "component:$U/Components" "clap:$U/CLAP" "lv2:$U/LV2" "aaxplugin:$A"; do b=${pair%%:*}; d=${pair#*:}; rm -rf "$d/OpenClipMultiband.$b" && ditto dist/signed/OpenClipMultiband.$b "$d/OpenClipMultiband.$b"; done
killall -9 AudioComponentRegistrar 2>/dev/null
{ echo "Installed signed build of the working tree (397ba8c + audit fixes, OpenPluginSDK 1f608fe), $(date '+%Y-%m-%d %H:%M')"
  for p in "$U/VST3/OpenClipMultiband.vst3" "$U/Components/OpenClipMultiband.component" "$U/CLAP/OpenClipMultiband.clap" "$A/OpenClipMultiband.aaxplugin" "$U/LV2/OpenClipMultiband.lv2/OpenClipMultiband.dylib"; do
    echo "$(basename "$p"): $(codesign --verify --strict "$p" 2>&1 && echo valid || echo INVALID); $(codesign -dvv "$p" 2>&1 | grep -m1 '^Authority='); $(codesign -dv "$p" 2>&1 | grep -m1 '^Timestamp=')"; done
  echo "wraptool verify (AAX): exit $(/Applications/PACEAntiPiracy/Eden/Fusion/Current/bin/wraptool verify --in "$A/OpenClipMultiband.aaxplugin" > /dev/null 2>&1; echo $?)"
  echo "system copies in /Library/Audio/Plug-Ins: $(find /Library/Audio/Plug-Ins -maxdepth 2 -name 'OpenClipMultiband*' | wc -l | tr -d ' ')"; } > $Q/install.txt
python3 "/Volumes/Developer/Dev Projects/DSP Engineering Tool/scripts/run_tests.py" --tier full > $Q/full-tier-final.txt 2>&1
mkdir -p $Q/test-list-final && cp target/openplugin-dev/logs/tree-T-*.log $Q/test-list-final/
uptime >> $Q/build-signed.log
