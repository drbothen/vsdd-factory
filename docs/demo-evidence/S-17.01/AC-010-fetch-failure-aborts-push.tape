# VHS Demo — S-17.01 AC-010
# AC-010: fetch failure before CAS push aborts the push (real git fetch failure)
#
# Removes the bare remote so fetch fails. Push MUST NOT be attempted.
#
# Usage: vhs AC-010-fetch-failure-aborts-push.tape
# Output: AC-010-fetch-failure-aborts-push.gif + .webm

Output docs/demo-evidence/S-17.01/AC-010-fetch-failure-aborts-push.gif
Output docs/demo-evidence/S-17.01/AC-010-fetch-failure-aborts-push.webm

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

Type "# S-17.01 AC-010: fetch failure aborts CAS push — push NOT attempted"
Enter
Sleep 400ms

Type "bash docs/demo-evidence/S-17.01/run-fetch-failure-demo.sh"
Enter
Sleep 10s

Sleep 2s
