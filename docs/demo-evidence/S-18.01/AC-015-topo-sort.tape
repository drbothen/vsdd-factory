# VHS tape: AC-015 — Multi-table topo-sort (BSD-portable, correct epic table selection)
# Demonstrates: dependency-ordered next-wave stories from production-shape multi-table STORY-INDEX
# ACs: AC-015, AC-002 (wave_id derivation), AC-014 (generated_from_handoff_sha)
Output docs/demo-evidence/S-18.01/AC-015-topo-sort.gif
Output docs/demo-evidence/S-18.01/AC-015-topo-sort.webm

Require bash

Set FontFamily "Menlo"
Set FontSize 14
Set Width 1280
Set Height 720
Set Theme "TokyoNight"
Set Padding 20
Set WaitTimeout 30s

Hide
Type "cd /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-18.01"
Enter
Sleep 500ms
Show

Type "bash docs/demo-evidence/S-18.01/demo-AC015-topo-sort.sh"
Enter
Wait+Screen@15s /DONE.*BSD-portable/
Sleep 2s
