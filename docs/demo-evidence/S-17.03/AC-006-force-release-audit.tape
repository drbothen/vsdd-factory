Output docs/demo-evidence/S-17.03/AC-006-force-release-audit.gif
Output docs/demo-evidence/S-17.03/AC-006-force-release-audit.webm

Require bash

Set FontFamily "Menlo"
Set Theme "Catppuccin Mocha"
Set Width 1200
Set Height 600
Set FontSize 14
Set Padding 24
Set Framerate 30

Hide
Type "cd /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-17.03"
Enter
Sleep 500ms
Type "BIN=plugins/vsdd-factory/bin"
Enter
Type "F=docs/demo-evidence/S-17.03/fixtures"
Enter
Sleep 300ms
Show

Type "# AC-006: force-release audit — PROCEED_FORCE_STEAL with 4 audit fields"
Enter
Sleep 800ms

Type "# fixture-foreign.md: holder=other@x.com; caller=dev@x.com + --force"
Enter
Type "bash $BIN/factory-unlock-decide.sh $F/fixture-foreign.md dev@x.com --force"
Enter
Sleep 2000ms

Type "echo exit:$?"
Enter
Sleep 1000ms

Sleep 2s
