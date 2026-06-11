# VHS Demo — S-17.01 AC-005
# AC-005: state-burst CAS push rejection on concurrent write (real --force-with-lease)
#
# Uses a real git bare+clone fixture. Racer pushes between fetch and push.
# Demonstrates the actual --force-with-lease collision contract.
#
# Usage: vhs AC-005-cas-push-rejected.tape
# Output: AC-005-cas-push-rejected.gif + .webm

Output docs/demo-evidence/S-17.01/AC-005-cas-push-rejected.gif
Output docs/demo-evidence/S-17.01/AC-005-cas-push-rejected.webm

Require bash
Require git

Set Shell "zsh"
Set FontFamily "Menlo"
Set FontSize 15
Set Width 1200
Set Height 720
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

Type "# S-17.01 AC-005: CAS push rejected — real --force-with-lease collision"
Enter
Sleep 400ms

Type "bash docs/demo-evidence/S-17.01/run-cas-rejection-demo.sh"
Enter
Sleep 12s

Sleep 2s
