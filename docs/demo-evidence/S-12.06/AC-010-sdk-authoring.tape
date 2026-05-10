# AC-010 — SDK Authoring Surface subsection
# Captures: grep showing SDK Authoring Surface content
# AC coverage: AC-010

Output docs/demo-evidence/S-12.06/AC-010-sdk-authoring.gif

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

# Show SDK Authoring Surface section
Type "awk '/^### SDK Authoring Surface/,/^### /' crates/hook-sdk/HOST_ABI.md | head -40"
Enter
Sleep 2s

Sleep 2s
