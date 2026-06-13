# VHS Demo — S-17.01 AC-004
# AC-004: mid-burst renew updates expires_at to now+2700; locked_at unchanged
#
# Usage: vhs AC-004-renew-expires-at-advances.tape
# Output: AC-004-renew-expires-at-advances.gif + .webm

Output docs/demo-evidence/S-17.01/AC-004-renew-expires-at-advances.gif
Output docs/demo-evidence/S-17.01/AC-004-renew-expires-at-advances.webm

Require bash

Set Shell "zsh"
Set FontFamily "Menlo"
Set FontSize 16
Set Width 1200
Set Height 620
Set Theme "Catppuccin Mocha"
Set WindowBar Colorful
Set BorderRadius 8
Set Padding 20
Set Framerate 30
Set TypingSpeed 40ms

Hide
Type "cd /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-17.01"
Enter
Sleep 1s
Type "clear"
Enter
Sleep 500ms
Show

Type "# S-17.01 AC-004: renew advances expires_at; locked_at unchanged"
Enter
Sleep 400ms

Type "bash docs/demo-evidence/S-17.01/run-ac004-demo.sh"
Enter
Sleep 8s

Sleep 2s
