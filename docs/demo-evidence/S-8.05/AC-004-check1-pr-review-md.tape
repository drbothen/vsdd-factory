Output docs/demo-evidence/S-8.05/AC-004-check1-pr-review-md.gif
Output docs/demo-evidence/S-8.05/AC-004-check1-pr-review-md.webm

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

Type "# AC-004: Check 1 — pr-review.md not written error (BC-7.04.042)"
Enter
Sleep 300ms

Type "grep -A 4 'Check 1' crates/hook-plugins/validate-pr-review-posted/src/lib.rs | head -6"
Enter
Sleep 1500ms

Type "grep -A 2 'pr-review.md may not have been' crates/hook-plugins/validate-pr-review-posted/src/lib.rs"
Enter
Sleep 1500ms
