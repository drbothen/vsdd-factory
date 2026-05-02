Output docs/demo-evidence/S-8.05/AC-006-check3-advisory-block.gif
Output docs/demo-evidence/S-8.05/AC-006-check3-advisory-block.webm

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

Type "# AC-006: Check 3a/3b + advisory block-mode (BC-7.04.044)"
Enter
Sleep 300ms

Type "grep -A 4 'Check 3a' crates/hook-plugins/validate-pr-review-posted/src/lib.rs | head -6"
Enter
Sleep 1500ms

Type "grep -A 4 'Check 3b' crates/hook-plugins/validate-pr-review-posted/src/lib.rs | head -6"
Enter
Sleep 1200ms

Type "grep -A 4 'host::emit_event' crates/hook-plugins/validate-pr-review-posted/src/main.rs | head -8"
Enter
Sleep 1500ms
