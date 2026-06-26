# VHS Demo Tape — S-18.05 F-P5-001
# F-P5-001 (degraded SHA path): STATE.md readable but origin/develop ref absent →
#   sha=UNKNOWN in stdout + log status=warn + exit 0
#   (This is the behavioral-bug-fix path confirmed by adv Pass-5)
#
# BC-7.07.002 postcondition 1 / invariant 2 / VP-089 §1

Output docs/demo-evidence/S-18.05/FP5001-degraded-sha.gif
Output docs/demo-evidence/S-18.05/FP5001-degraded-sha.webm

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

Type "bash /tmp/demo-s1805-fp5001-degraded-sha.sh"
Enter
Wait+Screen /PASS/
Sleep 2s
