# VHS Demo Tape — S-18.06 AC-003
# AC-003 (BC-4.15.001 PC-A): no emission on no-match; Continue
#
# Command: cargo fmt --check --all (not in default pattern list)
# Expected: no advisory; silent Continue; exit 0; DELEGATION_COUNT=0

Output docs/demo-evidence/S-18.06/AC-003-no-match-silent.gif
Output docs/demo-evidence/S-18.06/AC-003-no-match-silent.webm

Require bash

Set Shell "bash"
Set FontFamily "Menlo"
Set FontSize 16
Set Width 1200
Set Height 500
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

Type "bash /tmp/demo-s1806-ac003.sh"
Enter
Sleep 6s
