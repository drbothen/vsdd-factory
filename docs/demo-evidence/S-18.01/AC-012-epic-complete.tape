# VHS tape: AC-012 — EPIC-COMPLETE path
# Demonstrates: verbatim 3-line stdout announcement + HANDOFF.md with epic_status:complete + no wave-state.yaml
# ACs: AC-012, AC-005 (next_wave_stories: [] on EPIC-COMPLETE)
Output docs/demo-evidence/S-18.01/AC-012-epic-complete.gif
Output docs/demo-evidence/S-18.01/AC-012-epic-complete.webm

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

Type "bash docs/demo-evidence/S-18.01/demo-AC012-epic-complete.sh"
Enter
Wait+Screen@15s /DONE.*EPIC-COMPLETE/
Sleep 2s
