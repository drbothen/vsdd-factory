Output docs/demo-evidence/S-8.09/AC-007-bats-parity-tests.gif
Output docs/demo-evidence/S-8.09/AC-007-bats-parity-tests.webm

Require bats

Set FontFamily "FiraCode Nerd Font Mono"
Set FontSize 13
Set Width 1200
Set Height 800
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

Type "# AC-007: Bats parity tests (22 tests: 9 AC-007 scenarios + registry/file checks)"
Enter
Sleep 800ms

Type "bats tests/integration/E-8-hook-plugins/regression-gate.bats"
Enter
Sleep 12s
