# AC-002 — Resolver Lifecycle subsection
# Captures: awk showing Resolver Lifecycle subsection content
# AC coverage: AC-002

Output docs/demo-evidence/S-12.06/AC-002-resolver-lifecycle.gif

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

# Show Resolver Lifecycle subsection
Type "awk '/^### Resolver Lifecycle/,/^### /' crates/hook-sdk/HOST_ABI.md | head -35"
Enter
Sleep 2s

Sleep 2s
