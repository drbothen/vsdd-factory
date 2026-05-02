Output docs/demo-evidence/S-8.09/AC-009-no-jq-dependency.gif
Output docs/demo-evidence/S-8.09/AC-009-no-jq-dependency.webm

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

Type "# AC-009: Self-contained WASM, serde_json native, no jq subprocess"
Enter
Sleep 800ms

Type "grep -rn 'jq' crates/hook-plugins/regression-gate/src/ && echo 'JQ FOUND' || echo 'JQ NOT FOUND (AC-009 PASS)'"
Enter
Sleep 1500ms

Type "# serde_json and chrono handle parsing and timestamps:"
Enter
Sleep 600ms

Type "grep -E 'serde_json|chrono' crates/hook-plugins/regression-gate/Cargo.toml"
Enter
Sleep 1500ms

Type "# OQ-6 audit confirmed binary_allow = [] (empty, no subprocesses):"
Enter
Sleep 600ms

Type "grep 'recommended_binary_allow' docs/oq-6-regression-gate-security-audit.md"
Enter
Sleep 2s
