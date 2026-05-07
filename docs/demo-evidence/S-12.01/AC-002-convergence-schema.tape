# AC-002 — Convergence schema: 3 clean passes + NITPICK_ONLY + state JSON schema
# Captures: per-story-delivery.md Step 4.5 convergence criterion and state file schema
# AC coverage: AC-002 (BC-5.39.001 PC2, PC5)

Output docs/demo-evidence/S-12.01/AC-002-convergence-schema.gif

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

# Show convergence criterion text
Type "grep -A5 'Convergence criterion' plugins/vsdd-factory/workflows/phases/per-story-delivery.md | head -10"
Enter
Sleep 2s

Sleep 1s

# Show the state file schema
Type "grep -A20 'Convergence state file' plugins/vsdd-factory/workflows/phases/per-story-delivery.md | head -22"
Enter
Sleep 2.5s

Sleep 2s
