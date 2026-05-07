# AC-001 — Step 4.5 insertion between Step 4 and Step 5
# Captures: per-story-delivery.md showing Step 4.5 between Step 4 and Step 5
# AC coverage: AC-001 (BC-5.39.001 PC1)

Output docs/demo-evidence/S-12.01/AC-001-step-4.5-inserted.gif

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

# Show Step 4.5 exists in per-story-delivery.md
Type "grep -n 'Step 4\\|Step 4.5\\|Step 5' plugins/vsdd-factory/workflows/phases/per-story-delivery.md | head -20"
Enter
Sleep 2s

Sleep 1s

# Show the Step 4.5 header content
Type "grep -A3 '^## Step 4.5' plugins/vsdd-factory/workflows/phases/per-story-delivery.md | head -8"
Enter
Sleep 2s

Sleep 2s
