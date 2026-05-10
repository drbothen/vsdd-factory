# AC-005 — Lobster workflow steps including adversary-convergence + backup steps
# Captures: phase-3-tdd-implementation.lobster step names showing adversary-convergence wiring
# AC coverage: AC-005 (BC-5.39.001 PC1)

Output docs/demo-evidence/S-12.01/AC-005-lobster-steps.gif

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
Type "cd /Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.01"
Enter
Sleep 0.5s
Type "clear"
Enter
Sleep 0.3s
Show

# Show all step names from the lobster file
Type "grep '^ *- name:' plugins/vsdd-factory/workflows/phases/phase-3-tdd-implementation.lobster"
Enter
Sleep 2.5s

Sleep 1s

# Highlight adversary-convergence specifically
Type "grep -A2 'adversary-convergence' plugins/vsdd-factory/workflows/phases/phase-3-tdd-implementation.lobster | head -8"
Enter
Sleep 2s

Sleep 2s
