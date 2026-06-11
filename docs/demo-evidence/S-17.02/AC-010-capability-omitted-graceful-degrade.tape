# AC-010 (T-8): capability-omitted registry → Edit → graceful degrade → CONTINUE
# BC-4.13.001 Invariant 6: CapabilityDenied → fail-open; guard never crashes.
# Story S-17.02 — verify-factory-lock WASM guard

Output docs/demo-evidence/S-17.02/AC-010-capability-omitted-graceful-degrade.gif
Output docs/demo-evidence/S-17.02/AC-010-capability-omitted-graceful-degrade.webm

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

Type "bash docs/demo-evidence/S-17.02/demo-runner.sh ac010"
Enter
Sleep 8s
