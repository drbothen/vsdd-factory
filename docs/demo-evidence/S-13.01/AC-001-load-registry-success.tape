Output docs/demo-evidence/S-13.01/AC-001-load-registry-success.gif
Output docs/demo-evidence/S-13.01/AC-001-load-registry-success.webm

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

Type "# Demo: AC-001 — Registry loads: parse_registry never panics on any input"
Enter
Sleep 800ms

Type "cargo test -p validate-artifact-path ac001 2>&1 | tail -5"
Enter
Sleep 5s

Sleep 2s
