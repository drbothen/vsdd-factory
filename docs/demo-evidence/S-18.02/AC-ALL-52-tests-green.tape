# AC-ALL: All 52 unit tests pass
# Story S-18.02 — validate-wave-handoff-completeness WASM PostToolUse gate
# Shows: cargo test -p validate-wave-handoff-completeness -- summary (52 passed)

Output docs/demo-evidence/S-18.02/AC-ALL-52-tests-green.gif
Output docs/demo-evidence/S-18.02/AC-ALL-52-tests-green.webm

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

Type "cargo test -p validate-wave-handoff-completeness 2>&1 | grep -E 'running [0-9]+ test|test result'"
Enter
Sleep 10s
