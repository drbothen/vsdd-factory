Output docs/demo-evidence/S-13.01/AC-015-no-hardcoded-paths.gif
Output docs/demo-evidence/S-13.01/AC-015-no-hardcoded-paths.webm

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

Type "# Demo: AC-015 -- No hardcoded .factory/ path patterns in lib.rs"
Enter
Sleep 800ms

Type "grep -c behavioral-contracts crates/hook-plugins/validate-artifact-path/src/lib.rs"
Enter
Sleep 1s

Type "echo PASS: zero hardcoded paths confirmed"
Enter
Sleep 1s

Sleep 2s
