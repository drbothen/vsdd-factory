# AC-003 — Resolver ABI Types subsection
# Captures: awk showing ResolverInput/Output types and OD-5 invariant
# AC coverage: AC-003

Output docs/demo-evidence/S-12.06/AC-003-abi-types.gif

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

# Show Resolver ABI Types subsection
Type "awk '/^### Resolver ABI Types/,/^### /' crates/hook-sdk/HOST_ABI.md | head -40"
Enter
Sleep 2s

# Show the OD-5 invariant reference
Type "grep -n 'OD-5\\|RESOLVER_ABI_VERSION' crates/hook-sdk/HOST_ABI.md | head -10"
Enter
Sleep 2s

Sleep 2s
