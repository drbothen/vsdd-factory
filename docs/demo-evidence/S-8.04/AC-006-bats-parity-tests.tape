Output docs/demo-evidence/S-8.04/AC-006-bats-parity-tests.gif
Output docs/demo-evidence/S-8.04/AC-006-bats-parity-tests.webm

Require bats

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
Show

Type "# AC-006: 20 bats parity tests (grep-gate + fixtures + dispatcher parity)"
Enter
Sleep 500ms

Type "bats tests/integration/E-8-hook-plugins/update-wave-state-on-merge.bats"
Enter
Sleep 12000ms
