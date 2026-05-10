# AC-006 — Merging Contract subsection
# Captures: awk showing merging contract and collision events
# AC coverage: AC-006

Output docs/demo-evidence/S-12.06/AC-006-merging.gif

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

# Show Merging Contract subsection
Type "awk '/^### Merging Contract/,/^### /' crates/hook-sdk/HOST_ABI.md | head -40"
Enter
Sleep 2s

# Show merge_collision event reference
Type "grep -n 'merge_collision\\|collision' crates/hook-sdk/HOST_ABI.md | head -8"
Enter
Sleep 2s

Sleep 2s
