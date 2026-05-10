# AC-004 — Four deferred-finding categories in adversary.md
# Captures: adversary.md showing cross-story, integration, system-level, architectural categories
# AC coverage: AC-004 (BC-5.39.002 PC2, INV2)

Output docs/demo-evidence/S-12.01/AC-004-deferred-categories.gif

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

# Show the 4 deferred categories
Type "grep -A8 'four deferred-finding categories' plugins/vsdd-factory/agents/adversary.md | head -10"
Enter
Sleep 2s

Sleep 1s

# Show deferred categories from Perimeter 1 section
Type "grep -n 'cross-story\\|integration\\|system-level\\|architectural' plugins/vsdd-factory/agents/adversary.md | head -12"
Enter
Sleep 2.5s

Sleep 2s
