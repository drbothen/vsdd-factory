# AC-006 — EPIC-COMPLETE --commit succeeds with HANDOFF.md alone (no wave-state.yaml)
# Story S-18.13: wave-handoff skill writes HANDOFF.md via Write tool (gate-trigger fix)
# BC-5.41.001 PC10 EPIC-COMPLETE arm + BC-5.41.002 PC6
# Single-file commit on EPIC-COMPLETE path is NOT a PC6 violation

Output docs/demo-evidence/S-18.13/AC-006-epic-complete-commit-handoff-alone.gif
Output docs/demo-evidence/S-18.13/AC-006-epic-complete-commit-handoff-alone.webm

Set Shell "zsh"
Set FontFamily "Menlo"
Set FontSize 13
Set Width 1200
Set Height 500
Set Theme "Catppuccin Mocha"
Set WindowBar Colorful
Set BorderRadius 8
Set Padding 20
Set Framerate 30
Set TypingSpeed 50ms

Hide
Type "cd /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-18.13 && clear"
Enter
Sleep 1s
Show

Type "# AC-006: EPIC-COMPLETE --commit exits 0 with HANDOFF.md alone, no HandoffFileAbsent"
Enter
Sleep 0.5s

Type "cd plugins/vsdd-factory/tests && bats --filter 'test_BC_5_41_001_PC10_S18_13_AC006_epic_complete_commit_arm_succeeds_without_wave_state' wave-handoff.bats"
Enter
Sleep 20s

Sleep 2s
