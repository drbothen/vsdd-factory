# VHS Demo Tape — S-18.05 AC-006 / EC-003
# AC-006 (EC-003): STATE.md present but current_cycle/current_step absent →
#   context=UNKNOWN + log status=warn + exit 0
#
# BC-7.07.002 edge case EC-003 / VP-089 §4

Output docs/demo-evidence/S-18.05/AC-006-ec003-fields-absent.gif
Output docs/demo-evidence/S-18.05/AC-006-ec003-fields-absent.webm

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

Type "bash /tmp/demo-s1805-ac006-ec003-fields-absent.sh"
Enter
Wait+Screen /PASS/
Sleep 2s
