# AC-25 — Milestone-cycle no-block (the live-STATE.md regression case, ADR-023 brick-fix)
# Demonstrates: per_pass_trajectory absent/false → tail-less PC3/PC4/PC5 → Continue + advisory
#               (NOT Block); this is the exact case that would have bricked the pipeline under v1.8.
#
# Traces to: BC-5.39.009 EC-021; Precondition 7; invariant 14

Output docs/demo-evidence/S-15.17/AC-25-milestone-cycle-no-block.gif
Output docs/demo-evidence/S-15.17/AC-25-milestone-cycle-no-block.webm

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

Type "# AC-25: Milestone-cycle → NO Block (ADR-023 brick-fix)"
Enter
Sleep 1s

Type "bats validate-trajectory-tail-cell-completeness/pass-milestone-cycle-no-block.bats"
Enter
Sleep 8s
