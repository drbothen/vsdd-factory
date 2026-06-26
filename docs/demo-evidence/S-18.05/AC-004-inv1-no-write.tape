# VHS Demo Tape — S-18.05 AC-004 / Invariant 1
# AC-004 (postcondition 4 / invariant 1): hook makes NO write to factory-artifacts
#   HEAD is identical before and after hook invocation
#
# BC-7.07.002 postcondition 4 + invariant 1 / VP-089 §3

Output docs/demo-evidence/S-18.05/AC-004-inv1-no-write.gif
Output docs/demo-evidence/S-18.05/AC-004-inv1-no-write.webm

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

Type "bash /tmp/demo-s1805-ac004-inv1-no-write.sh"
Enter
Wait+Screen /PASS/
Sleep 2s
