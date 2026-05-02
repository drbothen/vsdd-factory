Output docs/demo-evidence/S-8.05/AC-002-bash-deleted-hooksjson-absent.gif
Output docs/demo-evidence/S-8.05/AC-002-bash-deleted-hooksjson-absent.webm

Require bash

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

Type "# AC-002: .sh deleted; hooks.json entry positively absent"
Enter
Sleep 300ms

Type "ls plugins/vsdd-factory/hooks/validate-pr-review-posted.sh 2>&1 || echo 'DELETED (expected)'"
Enter
Sleep 1500ms

Type "grep -r 'validate-pr-review-posted' plugins/vsdd-factory/hooks/hooks.json* 2>&1 || echo 'ABSENT from all hooks.json files (expected)'"
Enter
Sleep 1500ms
