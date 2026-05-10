# AC-007 — orchestrator.md MANDATORY STEPS referencing Step 4.5
# Captures: orchestrator.md showing reconciled MANDATORY STEPS language
# AC coverage: AC-007 (BC-5.39.001)

Output docs/demo-evidence/S-12.01/AC-007-mandatory-steps-reconciled.gif

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

# Search for Step 4.5 references in orchestrator.md
Type "grep -n '4.5\\|adversary.convergence\\|adversary convergence' plugins/vsdd-factory/agents/orchestrator/orchestrator.md | head -15"
Enter
Sleep 2.5s

Sleep 1s

# Show the MANDATORY STEPS section context
Type "grep -B2 -A6 'Step 4.5\\|4\\.5' plugins/vsdd-factory/agents/orchestrator/orchestrator.md | head -20"
Enter
Sleep 2s

Sleep 2s
