# AC-001-qualifying-bash-git-commit-injection.tape
# VP-093-A: qualifying PostToolUse Bash git-commit → dispatcher injects
#           git_context with all four fields populated (head_subject, head_sha,
#           head_parent_subject, head_parent_sha).
#
# Traces to: AC-001 (BC-1.16.001 PC1), AC-006 (four-field completeness),
#            AC-007 (end-to-end via dispatcher binary), AC-013 (VP-093-A).
# Story: S-18.04b-prereq

Output docs/demo-evidence/S-18.04b-prereq/AC-001-qualifying-bash-git-commit-injection.gif
Output docs/demo-evidence/S-18.04b-prereq/AC-001-qualifying-bash-git-commit-injection.webm

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

Type "bash run-demo.sh A"
Enter
Sleep 8s

Sleep 2s
