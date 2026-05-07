# AC-003 — Three-Perimeter Scope Contract in adversary.md
# Captures: adversary.md Three-Perimeter Scope Contract section
# AC coverage: AC-003 (BC-5.39.002 PC1)

Output docs/demo-evidence/S-12.01/AC-003-three-perimeter-contract.gif

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

# Show the Three-Perimeter section header + intro
Type "grep -n 'Three-Perimeter\\|Perimeter 1\\|Perimeter 2\\|Perimeter 3' plugins/vsdd-factory/agents/adversary.md"
Enter
Sleep 2s

Sleep 1s

# Show the full Three-Perimeter intro paragraph
Type "grep -A8 'Three-Perimeter Scope Contract' plugins/vsdd-factory/agents/adversary.md | head -10"
Enter
Sleep 2.5s

Sleep 2s
