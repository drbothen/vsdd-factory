# VHS Demo Tape — S-18.06 AC-006
# AC-006 (BC-4.15.001 INV4): command_preview truncated at 120 chars + U+2026 ellipsis
#
# Command: 149-char command containing "grep -r" (exceeds 120-char threshold)
# Expected: command_preview = first 120 chars + U+2026 (121 code points total)
#           Identical truncation in both stderr channel and plugin.log

Output docs/demo-evidence/S-18.06/AC-006-truncation.gif
Output docs/demo-evidence/S-18.06/AC-006-truncation.webm

Require bash

Set Shell "bash"
Set FontFamily "Menlo"
Set FontSize 16
Set Width 1200
Set Height 600
Set Theme "Catppuccin Mocha"
Set WindowBar Colorful
Set BorderRadius 8
Set Padding 20
Set Framerate 30
Set TypingSpeed 50ms
Set WaitTimeout 30s

Hide
Type "PS1='$ ' && cd /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-18.06"
Enter
Sleep 0.5s
Show

Type "bash /tmp/demo-s1806-ac006.sh"
Enter
Sleep 6s
