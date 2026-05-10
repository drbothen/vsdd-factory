Output docs/demo-evidence/S-12.02/cargo-test-green.gif

Require cargo

Set FontFamily "FiraCode Nerd Font Mono"
Set Theme "Dracula"
Set Width 1200
Set Height 600
Set FontSize 14
Set Padding 20

Hide
Type "cd /Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.02"
Enter
Sleep 0.5
Show

Type "cargo test -p validate-per-story-adversary-convergence 2>&1 | tail -10"
Enter
Sleep 15

Sleep 2
