Output docs/demo-evidence/S-8.05/AC-003-agent-scoping-bc2-02-012.gif
Output docs/demo-evidence/S-8.05/AC-003-agent-scoping-bc2-02-012.webm

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

Type "# AC-003: Agent scoping + BC-2.02.012 typed projection fallback chains"
Enter
Sleep 300ms

Type "grep -A 8 'BC-2.02.012 Postcondition 5' crates/hook-plugins/validate-pr-review-posted/src/lib.rs | head -10"
Enter
Sleep 1500ms

Type "grep -A 8 'BC-2.02.012 Postcondition 6' crates/hook-plugins/validate-pr-review-posted/src/lib.rs | head -10"
Enter
Sleep 1500ms

Type "grep -A 5 'pr-reviewer.*pr_reviewer.*pr-review-triage' crates/hook-plugins/validate-pr-review-posted/src/lib.rs || grep -A 4 'contains.*pr-reviewer' crates/hook-plugins/validate-pr-review-posted/src/lib.rs | head -6"
Enter
Sleep 2000ms
