# VHS Demo Tape — S-18.05 AC-005 / EC-002
# AC-005 (postcondition 5 + EC-002): factory-artifacts unreachable →
#   stdout WARN advisory + log status=warn + exit 0
#
# BC-7.07.002 postcondition 5 + edge case EC-002 / VP-089 §4

Output docs/demo-evidence/S-18.05/AC-005-ec002-unreachable.gif
Output docs/demo-evidence/S-18.05/AC-005-ec002-unreachable.webm

Require bash

Set Shell "bash"
Set FontFamily "Menlo"
Set FontSize 18
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
Type "PS1='$ ' && cd /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-18.05"
Enter
Sleep 0.5s
Show

Type "bash /tmp/demo-s1805-ac005-ec002-unreachable.sh"
Enter
Wait+Screen /PASS/
Sleep 2s
