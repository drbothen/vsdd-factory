# AC-008 — Factory-agnostic invariant (bats test 35)
# Captures: running bats test 35 (factory-agnostic test) passing
# AC coverage: AC-008

Output docs/demo-evidence/S-12.06/AC-008-factory-agnostic.gif

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

# Show the factory-agnostic test passing (test 35)
Type "bats --filter 'AC-008' plugins/vsdd-factory/tests/resolver-host-abi-context-injection.bats"
Enter
Sleep 8s

Sleep 2s
