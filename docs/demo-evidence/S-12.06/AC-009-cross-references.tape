# AC-009 — Cross-References (ADR-018 and BC refs)
# Captures: tail of section showing ADR-018 and 6 BC cross-refs
# AC coverage: AC-009

Output docs/demo-evidence/S-12.06/AC-009-cross-references.gif

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

# Show the Cross-References subsection
Type "awk '/^### Cross-References/,0' crates/hook-sdk/HOST_ABI.md"
Enter
Sleep 2s

# Confirm 6 BC refs + ADR-018
Type "grep -c 'BC-[0-9]' crates/hook-sdk/HOST_ABI.md"
Enter
Sleep 2s

Sleep 2s
