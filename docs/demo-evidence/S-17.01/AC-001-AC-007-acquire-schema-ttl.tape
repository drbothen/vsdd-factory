# VHS Demo — S-17.01 AC-001 / AC-007
# AC-001: factory_lock block written with holder + locked_at + expires_at on acquire
# AC-007: expires_at - locked_at == 2700 seconds exactly (TTL constant)
#
# Usage: vhs AC-001-AC-007-acquire-schema-ttl.tape
# Output: AC-001-AC-007-acquire-schema-ttl.gif + .webm

Output docs/demo-evidence/S-17.01/AC-001-AC-007-acquire-schema-ttl.gif
Output docs/demo-evidence/S-17.01/AC-001-AC-007-acquire-schema-ttl.webm

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

Type "# S-17.01 AC-001/AC-007: acquire writes factory_lock block with TTL=2700s"
Enter
Sleep 400ms

Type "bash docs/demo-evidence/S-17.01/run-ac001-ac007-demo.sh"
Enter
Sleep 8s

Sleep 2s
