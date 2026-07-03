# EC-017 — --commit with HANDOFF.md absent → HandoffFileAbsent hard error (both paths)
# Story S-18.13: wave-handoff skill writes HANDOFF.md via Write tool (gate-trigger fix)
# BC-5.41.001 EC-017: HANDOFF.md absent at --commit time → HandoffFileAbsent on BOTH paths

Output docs/demo-evidence/S-18.13/EC-017-handoff-file-absent-blocks-commit.gif
Output docs/demo-evidence/S-18.13/EC-017-handoff-file-absent-blocks-commit.webm

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

Type "# EC-017: --commit with HANDOFF.md absent → HandoffFileAbsent (HAS-NEXT-WAVE + EPIC-COMPLETE)"
Enter
Sleep 0.5s

Type "cd plugins/vsdd-factory/tests && bats --filter 'test_BC_5_41_001_EC017_S18_13_handoff_file_absent_blocks_commit' wave-handoff.bats"
Enter
Sleep 20s

Sleep 2s
