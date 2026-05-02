Output docs/demo-evidence/S-8.03/AC-001-wasm-crate-registry-migration.gif
Output docs/demo-evidence/S-8.03/AC-001-wasm-crate-registry-migration.webm

Require cat

Set FontFamily "FiraCode Nerd Font Mono"
Set Theme "Catppuccin Mocha"
Set Width 1200
Set Height 600
Set FontSize 14
Set Padding 24
Set Framerate 30

Hide
Type "cd /Users/jmagady/Dev/vsdd-factory/.worktrees/S-8.03"
Enter
Sleep 500ms
Show

Type "# AC-001: WASM crate exists, targets wasm32-wasip1, registry updated (no legacy fields)"
Enter
Sleep 500ms

Type "cat crates/hook-plugins/track-agent-stop/Cargo.toml"
Enter
Sleep 1500ms

Type "grep -A 8 'track-agent-stop' plugins/vsdd-factory/hooks-registry.toml | head -10"
Enter
Sleep 1500ms

Type "ls -lh target/wasm32-wasip1/release/track-agent-stop.wasm"
Enter
Sleep 1500ms

Sleep 2s
