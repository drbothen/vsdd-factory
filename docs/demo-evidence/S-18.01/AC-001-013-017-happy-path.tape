# VHS tape: AC-001/AC-013/AC-015/AC-017 — Happy-path has-next-wave
# Demonstrates: HANDOFF.md (9 base fields) + wave-state.yaml (6 fields) committed atomically
# ACs: AC-001, AC-002, AC-003, AC-013, AC-015, AC-017
Output docs/demo-evidence/S-18.01/AC-001-013-017-happy-path.gif
Output docs/demo-evidence/S-18.01/AC-001-013-017-happy-path.webm

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

Type "bash docs/demo-evidence/S-18.01/demo-AC001-AC013-AC017-happy-path.sh"
Enter
Wait+Screen@15s /DONE.*exit 0/
Sleep 2s
