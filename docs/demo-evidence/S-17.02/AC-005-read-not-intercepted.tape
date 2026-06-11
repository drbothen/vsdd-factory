# AC-005 (T-5): foreign lock → Read → NOT intercepted → CONTINUE
# BC-4.13.001 PC5: Read not in tool regex — sync_plugins=0, guard never invoked.
# Story S-17.02 — verify-factory-lock WASM guard

Output docs/demo-evidence/S-17.02/AC-005-read-not-intercepted.gif
Output docs/demo-evidence/S-17.02/AC-005-read-not-intercepted.webm

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

Type "bash docs/demo-evidence/S-17.02/demo-runner.sh ac005"
Enter
Sleep 8s
