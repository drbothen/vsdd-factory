Output docs/demo-evidence/S-8.04/AC-002-sh-deleted-hooksjson-removed.gif
Output docs/demo-evidence/S-8.04/AC-002-sh-deleted-hooksjson-removed.webm

Require bash

Set FontFamily "FiraCode Nerd Font Mono"
Set Theme "Catppuccin Mocha"
Set Width 1200
Set Height 600
Set FontSize 14
Set Padding 24

Hide
Type "cd /Users/jmagady/Dev/vsdd-factory/.worktrees/S-8.04"
Enter
Sleep 500ms
Show

Type "# AC-002: .sh deleted; hooks.json entries removed from all platform files"
Enter
Sleep 500ms

Type "ls plugins/vsdd-factory/hooks/update-wave-state-on-merge.sh 2>&1 || echo DELETED_AS_REQUIRED"
Enter
Sleep 1000ms

Type "grep -rl update-wave-state-on-merge plugins/vsdd-factory/hooks/ 2>&1 || echo NO_HOOKSJSON_ENTRIES"
Enter
Sleep 2000ms
