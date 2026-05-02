Output docs/demo-evidence/S-8.09/AC-012-bin-emit-event-present.gif
Output docs/demo-evidence/S-8.09/AC-012-bin-emit-event-present.webm

Set FontFamily "FiraCode Nerd Font Mono"
Set FontSize 14
Set Width 1200
Set Height 500
Set Theme "Catppuccin Mocha"
Set Padding 20

Hide
Type "cd /Users/jmagady/Dev/vsdd-factory/.worktrees/S-8.09"
Enter
Sleep 500ms
Type "clear"
Enter
Sleep 300ms
Show

Type "# AC-012: bin/emit-event PRESENT (E-8 D-10 deferral to S-8.29)"
Enter
Sleep 800ms

Type "test -f plugins/vsdd-factory/bin/emit-event && echo 'PRESENT' || echo 'MISSING'"
Enter
Sleep 1500ms

Type "# Tier 2/3 bash hooks depend on this binary until S-8.29:"
Enter
Sleep 600ms

Type "file plugins/vsdd-factory/bin/emit-event"
Enter
Sleep 2s
