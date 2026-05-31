# AC-2 + AC-3 — ALWAYS-Block (cycle-invariant): current_step + Last Updated missing tail
# Demonstrates: PC1 and PC2 Block regardless of cycle type (UNAFFECTED by per_pass_trajectory gate)
#
# Traces to: BC-5.39.009 PC1; PC2; Precondition 7

Output docs/demo-evidence/S-15.17/AC-2-3-always-block.gif
Output docs/demo-evidence/S-15.17/AC-2-3-always-block.webm

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

Type "# AC-2/3: current_step + Last Updated missing tail → ALWAYS Block (cycle-invariant)"
Enter
Sleep 1s

Type "bats validate-trajectory-tail-cell-completeness/fail-state-frontmatter-missing-tail.bats validate-trajectory-tail-cell-completeness/fail-state-last-updated-missing-tail.bats"
Enter
Sleep 8s
