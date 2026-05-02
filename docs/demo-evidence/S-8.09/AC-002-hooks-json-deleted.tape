Output docs/demo-evidence/S-8.09/AC-002-hooks-json-deleted.gif
Output docs/demo-evidence/S-8.09/AC-002-hooks-json-deleted.webm

Set FontFamily "FiraCode Nerd Font Mono"
Set FontSize 14
Set Width 1200
Set Height 600
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

Type "# AC-002: hooks.json entry deleted + regression-gate.sh deleted (BC-7.03.071 inv 1)"
Enter
Sleep 800ms

Type "# Post-deletion blast-radius validation:"
Enter
Sleep 600ms

Type "grep -r 'regression-gate' plugins/vsdd-factory/hooks/ --include='hooks.json*' && echo 'REFS FOUND' || echo 'ZERO MATCHES (AC-002 PASS)'"
Enter
Sleep 1500ms

Type "# regression-gate.sh deleted:"
Enter
Sleep 600ms

Type "test -f plugins/vsdd-factory/hooks/regression-gate.sh && echo 'STILL EXISTS' || echo 'DELETED (AC-002 PASS)'"
Enter
Sleep 2s
