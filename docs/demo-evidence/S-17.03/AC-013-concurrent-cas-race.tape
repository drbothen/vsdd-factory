Output docs/demo-evidence/S-17.03/AC-013-concurrent-cas-race.gif
Output docs/demo-evidence/S-17.03/AC-013-concurrent-cas-race.webm

Require bash
Require bats

Set FontFamily "Menlo"
Set Theme "Catppuccin Mocha"
Set Width 1200
Set Height 600
Set FontSize 14
Set Padding 24
Set Framerate 30

Hide
Type "cd /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-17.03/plugins/vsdd-factory/tests"
Enter
Sleep 500ms
Show

Type "# AC-013: concurrent CAS race — one wins, one AcquireRaceRejected"
Enter
Sleep 800ms

Type "bats factory-lock-skills-integration.bats --filter 'concurrent'"
Enter
Sleep 8000ms

Sleep 2s
