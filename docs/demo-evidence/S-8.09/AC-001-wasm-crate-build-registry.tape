Output docs/demo-evidence/S-8.09/AC-001-wasm-crate-build-registry.gif
Output docs/demo-evidence/S-8.09/AC-001-wasm-crate-build-registry.webm

Require cargo

Set FontFamily "FiraCode Nerd Font Mono"
Set FontSize 14
Set Width 1200
Set Height 600
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

Type "# AC-001: WASM crate exists and builds for wasm32-wasip1"
Enter
Sleep 800ms

Type "ls crates/hook-plugins/regression-gate/"
Enter
Sleep 1s

Type "grep -E 'name|targets' crates/hook-plugins/regression-gate/Cargo.toml | head -5"
Enter
Sleep 1s

Type "ls target/wasm32-wasip1/release/regression-gate.wasm && echo 'WASM ARTIFACT: OK'"
Enter
Sleep 1s

Type "grep -A6 'name = .regression-gate.' plugins/vsdd-factory/hooks-registry.toml | head -7"
Enter
Sleep 1s

Type "# Result: native WASM crate built, registry entry confirmed"
Enter
Sleep 2s
