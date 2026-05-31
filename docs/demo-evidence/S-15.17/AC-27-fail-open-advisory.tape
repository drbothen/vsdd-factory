# AC-27 — Fail-open-to-advisory on unresolvable per_pass_trajectory flag (inv-15)
# Demonstrates: active cycle INDEX.md absent/unreadable → flag defaults to FALSE →
#               PC3/PC4/PC5 advisory only, NEVER Block (inv-15 fail-open-to-advisory)
#
# Traces to: BC-5.39.009 Precondition 7 Step 4; invariant 15

Output docs/demo-evidence/S-15.17/AC-27-fail-open-advisory.gif
Output docs/demo-evidence/S-15.17/AC-27-fail-open-advisory.webm

Set Shell "zsh"
Set FontFamily "FiraCode Nerd Font Mono"
Set FontSize 18
Set Width 1200
Set Height 600
Set Theme "Catppuccin Mocha"
Set WindowBar Colorful
Set BorderRadius 8
Set Padding 20
Set Framerate 30
Set TypingSpeed 40ms

Hide
Type "cd /Users/jmagady/Dev/vsdd-factory/.worktrees/S-15.17/plugins/vsdd-factory/tests"
Enter
Sleep 2s
Type "clear"
Enter
Sleep 1s
Show

Type "# AC-27: cycle INDEX.md unreadable → per_pass_trajectory unresolvable → advisory, NEVER Block"
Enter
Sleep 1s

Type "bats validate-trajectory-tail-cell-completeness/pass-per-pass-flag-unreadable-failopen-advisory.bats"
Enter
Sleep 8s
