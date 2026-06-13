# AC-002 (T-3): foreign expired lock → Edit → CONTINUE
# BC-4.13.001 PC2: LockExpired — no block on past expires_at.
# Story S-17.02 — verify-factory-lock WASM guard

Output docs/demo-evidence/S-17.02/AC-002-foreign-expired-lock-continue.gif
Output docs/demo-evidence/S-17.02/AC-002-foreign-expired-lock-continue.webm

Set Shell "zsh"
Set FontFamily "Menlo"
Set FontSize 16
Set Width 1200
Set Height 300
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

Type "bash docs/demo-evidence/S-17.02/demo-runner.sh ac002"
Enter
Sleep 8s
