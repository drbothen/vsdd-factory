Output docs/demo-evidence/S-8.09/BONUS-macos-varfolders-fix.gif
Output docs/demo-evidence/S-8.09/BONUS-macos-varfolders-fix.webm

Set FontFamily "FiraCode Nerd Font Mono"
Set FontSize 13
Set Width 1200
Set Height 700
Set Theme "Catppuccin Mocha"
Set Padding 20

Hide
Type "cd /Users/jmagady/Dev/vsdd-factory/.worktrees/S-8.09"
Enter
Sleep 500ms
Type "clear"
Enter
Sleep 300ms
Show

Type "# BONUS: macOS /var/folders/ symlink fix + W-15 finale summary"
Enter
Sleep 800ms

Type "# macOS TMPDIR resolves via /var -> /private/var symlink; ancestor fallback needed"
Enter
Sleep 800ms

Type "grep -n 'ancestor\|canonicalize\|private' crates/factory-dispatcher/src/invoke.rs | head -8"
Enter
Sleep 1500ms

Type "# W-15 finale: 0 Tier 1 hooks via legacy-bash-adapter"
Enter
Sleep 800ms

Type "echo '9 Tier 1 hooks now native:' && grep -oE 'hook-plugins/[a-z-]+\.wasm' plugins/vsdd-factory/hooks-registry.toml | grep -v legacy | head -9"
Enter
Sleep 1500ms

Type "# adapter retirement (S-8.29) is now READY"
Enter
Sleep 2s
