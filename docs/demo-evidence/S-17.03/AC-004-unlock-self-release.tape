Output docs/demo-evidence/S-17.03/AC-004-unlock-self-release.gif
Output docs/demo-evidence/S-17.03/AC-004-unlock-self-release.webm

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

Type "# AC-004: self-release — PROCEED_RELEASE with holder/locked_at/released_at"
Enter
Sleep 800ms

Type "# fixture-self.md: holder=dev@x.com; caller email=dev@x.com (match → self-release)"
Enter
Type "bash $BIN/factory-unlock-decide.sh $F/fixture-self.md dev@x.com"
Enter
Sleep 2000ms

Type "echo exit:$?"
Enter
Sleep 1000ms

Sleep 2s
