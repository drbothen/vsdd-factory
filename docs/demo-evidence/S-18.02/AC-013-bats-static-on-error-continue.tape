# AC-013: bats STATIC test — on_error="continue" in production registry
# Story S-18.02 — validate-wave-handoff-completeness WASM PostToolUse gate
# BC-4.14.001 PC6: hooks-registry.toml specifies on_error="continue" for the gate
# Shows: AC-013 STATIC passes; LIVE tests skip (dispatcher not built in worktree)

Output docs/demo-evidence/S-18.02/AC-013-bats-static-on-error-continue.gif
Output docs/demo-evidence/S-18.02/AC-013-bats-static-on-error-continue.webm

Set Shell "zsh"
Set FontFamily "Menlo"
Set FontSize 14
Set Width 1200
Set Height 350
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

Type "bats plugins/vsdd-factory/tests/validate-wave-handoff-completeness/fail-open-on-crash.bats"
Enter
Sleep 8s
