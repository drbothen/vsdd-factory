# AC-006-initial-commit-empty-parent-fields.tape
# VP-093-E: initial commit (no HEAD^) → head_parent_subject and head_parent_sha
#           are "" (empty string), NOT null, NOT absent.
#           head_subject and head_sha are fully populated.
#
# Traces to: AC-006 (BC-1.16.001 PC6/INV5 four-field completeness),
#            AC-011 (initial-commit "" not null), AC-013 (VP-093-E).
# Story: S-18.04b-prereq

Output docs/demo-evidence/S-18.04b-prereq/AC-006-initial-commit-empty-parent-fields.gif
Output docs/demo-evidence/S-18.04b-prereq/AC-006-initial-commit-empty-parent-fields.webm

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

Type "bash run-demo.sh E"
Enter
Sleep 8s

Sleep 2s
