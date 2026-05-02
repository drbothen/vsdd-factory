Output docs/demo-evidence/S-8.04/AC-005-gate-status-flip.gif
Output docs/demo-evidence/S-8.04/AC-005-gate-status-flip.webm

Require bash

Set FontFamily "FiraCode Nerd Font Mono"
Set Theme "Catppuccin Mocha"
Set Width 1200
Set Height 700
Set FontSize 14
Set Padding 24

Hide
Type "cd /Users/jmagady/Dev/vsdd-factory/.worktrees/S-8.04"
Enter
Sleep 500ms
Type "CARGO=/Users/jmagady/.rustup/toolchains/1.95.0-aarch64-apple-darwin/bin/cargo"
Enter
Sleep 200ms
Show

Type "# AC-005: gate_status flip to pending — 4-case truth table (absent/null/not_started/other)"
Enter
Sleep 500ms

Type "$CARGO test -p update-wave-state-on-merge -- BC_7_03_086 2>&1 | tail -20"
Enter
Sleep 8000ms
