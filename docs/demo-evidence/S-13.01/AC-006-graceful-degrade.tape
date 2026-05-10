Output docs/demo-evidence/S-13.01/AC-006-graceful-degrade.gif
Output docs/demo-evidence/S-13.01/AC-006-graceful-degrade.webm

Require cargo

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

Type "# Demo: AC-006 — Graceful degrade: absent or malformed registry returns Continue"
Enter
Sleep 800ms

Type "cargo test -p validate-artifact-path ac006 2>&1 | tail -6"
Enter
Sleep 5s

Sleep 2s
