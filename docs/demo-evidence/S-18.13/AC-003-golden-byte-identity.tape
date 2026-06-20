# AC-003 — Golden byte-identity: --emit-handoff stdout == frozen golden fixture
# Story S-18.13: wave-handoff skill writes HANDOFF.md via Write tool (gate-trigger fix)
# BC-5.41.001 PC10 — byte-identity write-path verification
# Verifies --emit-handoff output matches the golden fixture committed before refactor

Output docs/demo-evidence/S-18.13/AC-003-golden-byte-identity.gif
Output docs/demo-evidence/S-18.13/AC-003-golden-byte-identity.webm

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

Type "# AC-003: golden byte-identity -- --emit-handoff stdout == frozen fixture"
Enter
Sleep 0.5s

Type "ls plugins/vsdd-factory/tests/fixtures/wave-handoff-golden/"
Enter
Sleep 1.5s

Type "cd plugins/vsdd-factory/tests && bats --filter 'test_BC_5_41_001_PC10_S18_13_AC003_emit_handoff_stdout_matches_golden' wave-handoff.bats"
Enter
Sleep 20s

Sleep 2s
