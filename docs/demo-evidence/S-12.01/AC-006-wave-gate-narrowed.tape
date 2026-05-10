# AC-006 — wave-gate Gate 3 narrowed scope (cross-story focus, deferred_findings reference)
# Captures: wave-gate SKILL.md Gate 3 section
# AC coverage: AC-006 (BC-5.39.002 PC1, PC7)

Output docs/demo-evidence/S-12.01/AC-006-wave-gate-narrowed.gif

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

# Show Gate 3 section header and prerequisite
Type "grep -A20 'Gate 3: Adversarial Review' plugins/vsdd-factory/skills/wave-gate/SKILL.md | head -22"
Enter
Sleep 2.5s

Sleep 1s

# Show deferred_findings reference in Gate 3
Type "grep -n 'deferred_findings\\|cross-story\\|integration concerns' plugins/vsdd-factory/skills/wave-gate/SKILL.md | head -8"
Enter
Sleep 2s

Sleep 2s
