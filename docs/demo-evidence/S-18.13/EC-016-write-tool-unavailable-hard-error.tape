# EC-016 — Write tool unavailable → HandoffWriteToolUnavailable hard error (no bash fallback)
# Story S-18.13: wave-handoff skill writes HANDOFF.md via Write tool (gate-trigger fix)
# BC-5.41.001 EC-016: Write tool unavailable → HandoffWriteToolUnavailable; NO bash fallback

Output docs/demo-evidence/S-18.13/EC-016-write-tool-unavailable-hard-error.gif
Output docs/demo-evidence/S-18.13/EC-016-write-tool-unavailable-hard-error.webm

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

Type "# EC-016: Write tool unavailable → HandoffWriteToolUnavailable hard error"
Enter
Sleep 0.5s

Type "cd plugins/vsdd-factory/tests && bats --filter 'test_BC_5_41_001_EC016_S18_13_handoff_write_tool_unavailable_hard_error' wave-handoff.bats"
Enter
Sleep 15s

Sleep 2s
