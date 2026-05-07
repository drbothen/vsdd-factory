Output docs/demo-evidence/S-13.01/AC-013-vp072-sot.gif
Output docs/demo-evidence/S-13.01/AC-013-vp072-sot.webm

Require bats

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
Set TypingSpeed 50ms

Hide
Type "cd /Users/jmagady/Dev/vsdd-factory/.worktrees/S-13.01"
Enter
Sleep 300ms
Type "clear"
Enter
Sleep 300ms
Show

Type "# Demo: AC-013 — VP-072 single-source-of-truth invariant: 28/28 ok"
Enter
Sleep 800ms

Type "bats plugins/vsdd-factory/tests/vp-072-sot-invariant.bats"
Enter
Sleep 10s

Sleep 2s
