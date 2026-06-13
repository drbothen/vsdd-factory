Output docs/demo-evidence/S-17.03/AC-007-008-three-state-health-display.gif
Output docs/demo-evidence/S-17.03/AC-007-008-three-state-health-display.webm

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

Type "# AC-007/AC-008: Three-state lock status display (shared helper)"
Enter
Sleep 800ms

Type "# State 1: Factory lock FREE (no lock block)"
Enter
Type "bash $BIN/factory-lock-status.sh $F/fixture-free.md dev@x.com"
Enter
Sleep 2000ms

Type "# State 2: HELD by this session (self-held, unexpired)"
Enter
Type "bash $BIN/factory-lock-status.sh $F/fixture-self.md dev@x.com"
Enter
Sleep 2000ms

Type "# State 3: HELD by foreign holder (other@x.com)"
Enter
Type "bash $BIN/factory-lock-status.sh $F/fixture-foreign.md dev@x.com"
Enter
Sleep 2000ms

Type "# Both health skills invoke the SAME helper (grep proof)"
Enter
Type "grep -c 'factory-lock-status.sh' plugins/vsdd-factory/skills/factory-health/SKILL.md plugins/vsdd-factory/skills/factory-worktree-health/SKILL.md"
Enter
Sleep 2000ms

Sleep 2s
