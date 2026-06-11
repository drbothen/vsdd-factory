# VHS Demo — S-17.01 AC-002
# AC-002: clear removes factory_lock key entirely (not null) from frontmatter
#
# Usage: vhs AC-002-clear-key-absent.tape
# Output: AC-002-clear-key-absent.gif + .webm

Output docs/demo-evidence/S-17.01/AC-002-clear-key-absent.gif
Output docs/demo-evidence/S-17.01/AC-002-clear-key-absent.webm

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

Type "# S-17.01 AC-002: clear removes factory_lock key entirely (not null)"
Enter
Sleep 400ms

Type "bash docs/demo-evidence/S-17.01/run-ac002-demo.sh"
Enter
Sleep 6s

Sleep 2s
