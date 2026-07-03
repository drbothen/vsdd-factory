# AC-003-004-non-qualifying-no-injection.tape
# VP-093-C and VP-093-D: non-qualifying events produce no git_context injection.
#   - Scenario C: PostToolUse Bash "git push" → git_context ABSENT
#   - Scenario D: PostToolUse Edit → git_context ABSENT
# Positive-coverage sentinel confirms plugin WAS invoked in both cases.
#
# Traces to: AC-003 (BC-1.16.001 PC3), AC-004 (PC4), AC-008 (INV2),
#            AC-012 (EC-004 git push), AC-013 (VP-093-C + VP-093-D).
# Story: S-18.04b-prereq

Output docs/demo-evidence/S-18.04b-prereq/AC-003-004-non-qualifying-no-injection.gif
Output docs/demo-evidence/S-18.04b-prereq/AC-003-004-non-qualifying-no-injection.webm

Require bash
Require jq

Set Shell "bash"
Set FontSize 16
Set Width 1200
Set Height 600
Set Theme "Catppuccin Mocha"
Set WindowBar Colorful
Set BorderRadius 8
Set Padding 20
Set Framerate 30
Set TypingSpeed 30ms

Hide
Type "cd /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-18.04b-prereq/docs/demo-evidence/S-18.04b-prereq && clear"
Enter
Sleep 2s
Show

Type "bash run-demo.sh C"
Enter
Sleep 6s

Sleep 1s

Type "bash run-demo.sh D"
Enter
Sleep 6s

Sleep 2s
