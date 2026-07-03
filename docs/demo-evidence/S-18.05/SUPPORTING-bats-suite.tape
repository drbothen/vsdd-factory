# VHS Demo Tape — S-18.05 Supporting: full bats suite
# Shows all 11/11 bats tests passing for postcompact-reanchor.bats
# Covers: AC-001, AC-002, AC-003, AC-004, AC-005, AC-006, AC-008, AC-009, AC-010, F-P5-001, EC-005

Output docs/demo-evidence/S-18.05/SUPPORTING-bats-suite.gif
Output docs/demo-evidence/S-18.05/SUPPORTING-bats-suite.webm

Require bash
Require bats

Set Shell "bash"
Set FontFamily "Menlo"
Set FontSize 16
Set Width 1200
Set Height 700
Set Theme "Catppuccin Mocha"
Set WindowBar Colorful
Set BorderRadius 8
Set Padding 20
Set Framerate 30
Set TypingSpeed 50ms
Set WaitTimeout 60s

Hide
Type "PS1='$ ' && cd /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-18.05"
Enter
Sleep 0.5s
Show

Type "bash /tmp/demo-s1805-bats-suite.sh"
Enter
Wait+Screen /11\/11 PASS/
Sleep 2s
