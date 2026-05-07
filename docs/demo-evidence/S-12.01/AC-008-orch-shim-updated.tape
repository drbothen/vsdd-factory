# AC-008 — orchestrator/per-story-delivery.md parallel Step 4.5 reference
# Captures: orchestrator/per-story-delivery.md showing c2 adversary convergence step
# AC coverage: AC-008 (BC-5.39.001)

Output docs/demo-evidence/S-12.01/AC-008-orch-shim-updated.gif

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

# Show the per-story-delivery orchestrator shim with Step 4.5 (c2)
Type "grep -n 'adversary\\|4.5\\|c2' plugins/vsdd-factory/agents/orchestrator/per-story-delivery.md | head -15"
Enter
Sleep 2.5s

Sleep 1s

# Show the delivery step list context
Type "grep -A3 'c2. Run Step 4.5' plugins/vsdd-factory/agents/orchestrator/per-story-delivery.md | head -8"
Enter
Sleep 2s

Sleep 2s
