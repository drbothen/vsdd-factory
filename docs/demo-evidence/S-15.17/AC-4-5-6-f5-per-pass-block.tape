# AC-4/5/6 — Cycle-conditional Block (F5 per-pass cycle, per_pass_trajectory: true)
# Demonstrates: F5-style cycle with PC3/PC4/PC5 tail-less → cascade Block
# This is the genuine ADV-EDP1-P75-HIGH-002 finding class the hook catches.
#
# Traces to: BC-5.39.009 PC3/PC4/PC5 (cycle-conditional); Precondition 7; invariant 14

Output docs/demo-evidence/S-15.17/AC-4-5-6-f5-per-pass-block.gif
Output docs/demo-evidence/S-15.17/AC-4-5-6-f5-per-pass-block.webm

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

Type "# AC-4/5/6: F5 per-pass (per_pass_trajectory:true) tail-less PC3/PC4/PC5 → cascade Block"
Enter
Sleep 1s

Type "bats validate-trajectory-tail-cell-completeness/fail-state-f5-per-pass-block.bats"
Enter
Sleep 8s
