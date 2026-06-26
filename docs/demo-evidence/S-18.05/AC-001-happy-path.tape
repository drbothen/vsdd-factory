# VHS Demo Tape — S-18.05 AC-001
# AC-001 (postcondition 1): happy path — readable factory-artifacts STATE.md
#   + resolvable origin/develop → stdout re-anchor block with git-sourced values + exit 0
#
# BC-7.07.002 postcondition 1 / VP-089 §1

Output docs/demo-evidence/S-18.05/AC-001-happy-path.gif
Output docs/demo-evidence/S-18.05/AC-001-happy-path.webm

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

Type "bash /tmp/demo-s1805-ac001-pc1.sh"
Enter
Wait+Screen /PASS/
Sleep 2s
