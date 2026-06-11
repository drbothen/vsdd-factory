Output docs/demo-evidence/S-17.03/CRLF-parity-fix.gif
Output docs/demo-evidence/S-17.03/CRLF-parity-fix.webm

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

Type "# CRLF parity fix: CRLF STATE.md with foreign lock → HELD not FREE"
Enter
Sleep 800ms

Type "# Confirm fixture has CRLF line endings"
Enter
Type "file $F/fixture-crlf-foreign-actual.md"
Enter
Sleep 1500ms

Type "# LF-only fixture — baseline (same content, LF endings)"
Enter
Type "bash $BIN/factory-lock-status.sh $F/fixture-foreign.md dev@x.com"
Enter
Sleep 1500ms

Type "# CRLF fixture — must also show HELD (guard parity with LF files)"
Enter
Type "bash $BIN/factory-lock-status.sh $F/fixture-crlf-foreign-actual.md dev@x.com"
Enter
Sleep 2000ms

Sleep 2s
