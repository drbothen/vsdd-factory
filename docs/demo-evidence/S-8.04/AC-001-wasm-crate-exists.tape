Output docs/demo-evidence/S-8.04/AC-001-wasm-crate-exists.gif
Output docs/demo-evidence/S-8.04/AC-001-wasm-crate-exists.webm

Require cat

Set FontFamily "FiraCode Nerd Font Mono"
Set Theme "Catppuccin Mocha"
Set Width 1200
Set Height 600
Set FontSize 14
Set Padding 24

Hide
Type "cd /Users/jmagady/Dev/vsdd-factory/.worktrees/S-8.04"
Enter
Sleep 500ms
Show

Type "# AC-001: WASM crate exists, targets wasm32-wasip1, registry entry preserved"
Enter
Sleep 500ms

Type "cat crates/hook-plugins/update-wave-state-on-merge/Cargo.toml"
Enter
Sleep 1500ms

Type "sed -n '943,956p' plugins/vsdd-factory/hooks-registry.toml"
Enter
Sleep 2000ms
