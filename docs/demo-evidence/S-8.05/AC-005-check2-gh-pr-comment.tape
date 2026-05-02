Output docs/demo-evidence/S-8.05/AC-005-check2-gh-pr-comment.gif
Output docs/demo-evidence/S-8.05/AC-005-check2-gh-pr-comment.webm

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

Type "# AC-005: Check 2 — gh pr comment fallback detected (BC-7.04.043)"
Enter
Sleep 300ms

Type "grep -A 4 'Check 2' crates/hook-plugins/validate-pr-review-posted/src/lib.rs | head -6"
Enter
Sleep 1500ms

Type "grep -A 2 'gh pr comment.*instead of' crates/hook-plugins/validate-pr-review-posted/src/lib.rs"
Enter
Sleep 1500ms
