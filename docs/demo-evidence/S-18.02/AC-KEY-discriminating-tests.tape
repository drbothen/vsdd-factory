# AC-KEY: Discriminating tests for key ACs
# Story S-18.02 — validate-wave-handoff-completeness WASM PostToolUse gate
# BC-4.14.001 PC2a/PC3/PC4/PC8/INV2/INV3 + VP-083 F-P32-002
# Shows: named tests for AC-001, AC-002, AC-003, AC-005, AC-006, AC-011

Output docs/demo-evidence/S-18.02/AC-KEY-discriminating-tests.gif
Output docs/demo-evidence/S-18.02/AC-KEY-discriminating-tests.webm

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
Type "cd /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-18.02 && clear"
Enter
Sleep 1s
Show

Type "cargo test -p validate-wave-handoff-completeness -- ac_001 ac_002 ac_003 ac_005 ac_006 ac_011 2>&1 | grep -E 'test ac_|test result'"
Enter
Sleep 10s
