Output docs/demo-evidence/S-8.03/AC-002-bash-deletion.gif
Output docs/demo-evidence/S-8.03/AC-002-bash-deletion.webm

Require grep

Set FontFamily "FiraCode Nerd Font Mono"
Set Theme "Catppuccin Mocha"
Set Width 1200
Set Height 600
Set FontSize 14
Set Padding 24
Set Framerate 30

Hide
Type "cd /Users/jmagady/Dev/vsdd-factory/.worktrees/S-8.03"
Enter
Sleep 500ms
Show

Type "# AC-002: track-agent-stop.sh absent; hooks.json entries absent (positive grep returns no match)"
Enter
Sleep 500ms

Type "ls plugins/vsdd-factory/hooks/track-agent-stop.sh 2>&1 || echo 'CONFIRMED: .sh deleted'"
Enter
Sleep 1500ms

Type "grep -r 'track-agent-stop' plugins/vsdd-factory/hooks/hooks.json.* 2>&1 || echo 'CONFIRMED: no hooks.json entries'"
Enter
Sleep 1500ms

Sleep 2s
