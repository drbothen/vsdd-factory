# AC-002-fail-open-git-error-all-empty.tape
# VP-093-B: git error (non-git factory dir) → dispatcher injects all-empty
#           git_context; exits 0 (fail-open; pipeline never blocked).
#
# Traces to: AC-002 (BC-1.16.001 PC2), AC-009 (INV3 fail-open),
#            AC-013 (VP-093-B).
# Story: S-18.04b-prereq

Output docs/demo-evidence/S-18.04b-prereq/AC-002-fail-open-git-error-all-empty.gif
Output docs/demo-evidence/S-18.04b-prereq/AC-002-fail-open-git-error-all-empty.webm

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

Type "bash run-demo.sh B"
Enter
Sleep 8s

Sleep 2s
