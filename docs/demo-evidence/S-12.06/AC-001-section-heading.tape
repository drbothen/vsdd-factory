# AC-001 — Context Injection Contract section heading
# Captures: grep showing the section exists at the correct location
# AC coverage: AC-001

Output docs/demo-evidence/S-12.06/AC-001-section-heading.gif

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
Type "cd /Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.06"
Enter
Sleep 0.5s
Type "clear"
Enter
Sleep 0.3s
Show

# Show the Context Injection Contract section heading
Type "grep -n '^## Context Injection Contract' crates/hook-sdk/HOST_ABI.md"
Enter
Sleep 2s

# Show a few lines of context around the heading
Type "awk 'NR>=463 && NR<=470' crates/hook-sdk/HOST_ABI.md"
Enter
Sleep 2s

Sleep 2s
