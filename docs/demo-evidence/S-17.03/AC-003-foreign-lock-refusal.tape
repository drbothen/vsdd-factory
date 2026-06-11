Output docs/demo-evidence/S-17.03/AC-003-foreign-lock-refusal.gif
Output docs/demo-evidence/S-17.03/AC-003-foreign-lock-refusal.webm

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

Type "# AC-003: foreign-lock refusal — REFUSED_FOREIGN_LOCK + 5-field message"
Enter
Sleep 800ms

Type "# fixture-foreign.md: holder=other@x.com (unexpired); git stubbed to succeed"
Enter
Type "PATH=/tmp/demo-git-stub:$PATH bash $BIN/factory-lock-acquire-precheck.sh $F/fixture-foreign.md 2>&1; echo exit:$?"
Enter
Sleep 3000ms

Sleep 2s
