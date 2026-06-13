# AC-001 (T-2): foreign unexpired lock → Edit → BLOCK
# BC-4.13.001 PC1: ForeignLockHeld — all 5 required fields in block message.
# Story S-17.02 — verify-factory-lock WASM guard

Output docs/demo-evidence/S-17.02/AC-001-foreign-unexpired-lock-blocks.gif
Output docs/demo-evidence/S-17.02/AC-001-foreign-unexpired-lock-blocks.webm

Set Shell "zsh"
Set FontFamily "Menlo"
Set FontSize 16
Set Width 1200
Set Height 400
Set Theme "Catppuccin Mocha"
Set WindowBar Colorful
Set BorderRadius 8
Set Padding 20
Set Framerate 30
Set TypingSpeed 50ms

Hide
Type "cd /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-17.02 && clear"
Enter
Sleep 2s
Show

Type "bash docs/demo-evidence/S-17.02/demo-runner.sh ac001"
Enter
Sleep 8s
