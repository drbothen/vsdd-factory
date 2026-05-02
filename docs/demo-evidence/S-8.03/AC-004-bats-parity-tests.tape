Output docs/demo-evidence/S-8.03/AC-004-bats-parity-tests.gif
Output docs/demo-evidence/S-8.03/AC-004-bats-parity-tests.webm

Require bats

Set FontFamily "FiraCode Nerd Font Mono"
Set Theme "Catppuccin Mocha"
Set Width 1200
Set Height 600
Set FontSize 13
Set Padding 24
Set Framerate 30

Hide
Type "cd /Users/jmagady/Dev/vsdd-factory/.worktrees/S-8.03"
Enter
Sleep 500ms
Show

Type "# AC-004: bats parity tests — empty/blocked/ok/fallback chains/malformed-JSON (17/17)"
Enter
Sleep 500ms

Type "bats tests/integration/E-8-hook-plugins/track-agent-stop.bats"
Enter
Sleep 8000ms

Sleep 2s
