Output docs/demo-evidence/S-8.05/AC-001-registry-migration.gif
Output docs/demo-evidence/S-8.05/AC-001-registry-migration.webm

Require grep

Set FontFamily "FiraCode Nerd Font Mono"
Set Theme "Catppuccin Mocha"
Set Width 1200
Set Height 600
Set FontSize 14
Set Padding 24

Hide
Type "cd /Users/jmagady/Dev/vsdd-factory/.worktrees/S-8.05"
Enter
Sleep 500ms
Show

Type "# AC-001: Registry migration — native WASM, no script_path, no exec_subprocess"
Enter
Sleep 300ms

Type "grep -A 6 validate-pr-review-posted plugins/vsdd-factory/hooks-registry.toml"
Enter
Sleep 1500ms

Type "# Crate Cargo.toml: lib + bin, wasm32-wasip1 target"
Enter
Sleep 300ms

Type "cat crates/hook-plugins/validate-pr-review-posted/Cargo.toml"
Enter
Sleep 2000ms
