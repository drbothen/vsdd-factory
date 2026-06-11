Output docs/demo-evidence/S-17.03/AC-012-self-relock-noop.gif
Output docs/demo-evidence/S-17.03/AC-012-self-relock-noop.webm

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

Type "# AC-012/EC-001: self-relock noop — NOOP_SELF_HELD"
Enter
Sleep 800ms

Type "# fixture-self.md: holder=dev@x.com (same as current user — self-held)"
Enter
Type "PATH=/tmp/demo-git-stub:$PATH bash $BIN/factory-lock-acquire-precheck.sh $F/fixture-self.md"
Enter
Sleep 2000ms

Type "echo exit:$?"
Enter
Sleep 1000ms

Sleep 2s
