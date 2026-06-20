# AC-005 — ONE atomic git commit: HANDOFF.md + wave-state.yaml (HAS-NEXT-WAVE)
# Story S-18.13: wave-handoff skill writes HANDOFF.md via Write tool (gate-trigger fix)
# BC-5.41.002 PC6 — atomicity at git-commit boundary
# --commit stages BOTH files in ONE atomic git commit on HAS-NEXT-WAVE path

Output docs/demo-evidence/S-18.13/AC-005-atomic-commit-has-next-wave.gif
Output docs/demo-evidence/S-18.13/AC-005-atomic-commit-has-next-wave.webm

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

Type "# AC-005: --commit creates ONE atomic commit with BOTH files (HAS-NEXT-WAVE)"
Enter
Sleep 0.5s

Type "cd plugins/vsdd-factory/tests && bats --filter 'test_BC_5_41_002_PC6_S18_13_AC005_commit_creates_one_atomic_commit_has_next_wave' wave-handoff.bats"
Enter
Sleep 20s

Sleep 2s
