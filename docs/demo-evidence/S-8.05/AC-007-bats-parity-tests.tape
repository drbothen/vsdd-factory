Output docs/demo-evidence/S-8.05/AC-007-bats-parity-tests.gif
Output docs/demo-evidence/S-8.05/AC-007-bats-parity-tests.webm

Require bats

Set FontFamily "FiraCode Nerd Font Mono"
Set Theme "Catppuccin Mocha"
Set Width 1200
Set Height 700
Set FontSize 13
Set Padding 24

Hide
Type "cd /Users/jmagady/Dev/vsdd-factory/.worktrees/S-8.05"
Enter
Sleep 500ms
Type "export PATH=/Users/jmagady/.cargo/bin:$PATH"
Enter
Sleep 300ms
Show

Type "# AC-007: Bats parity tests — 14 tests, all pass"
Enter
Sleep 300ms

Type "bats tests/integration/E-8-hook-plugins/validate-pr-review-posted.bats"
Enter
Sleep 8000ms
