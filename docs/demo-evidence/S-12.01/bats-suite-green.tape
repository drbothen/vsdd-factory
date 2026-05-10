# bats-suite-green — per-story-adversary-workflow.bats 31/31 ok
# Captures: bats test suite pass showing all 31 tests green
# AC coverage: cross-cutting all ACs (BC-5.39.001, BC-5.39.002)

Output docs/demo-evidence/S-12.01/bats-suite-green.gif

Set Shell "zsh"
Set FontFamily "FiraCode Nerd Font Mono"
Set FontSize 18
Set Width 1200
Set Height 600
Set Theme "Catppuccin Mocha"
Set WindowBar Colorful
Set BorderRadius 8
Set Padding 20
Set Framerate 30
Set TypingSpeed 40ms

Hide
Type "cd /Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.01"
Enter
Sleep 0.5s
Type "clear"
Enter
Sleep 0.3s
Show

Type "bats plugins/vsdd-factory/tests/per-story-adversary-workflow.bats"
Enter
Sleep 12s

Sleep 2s
