Output docs/demo-evidence/S-8.09/AC-008-no-bin-emit-event-in-crate.gif
Output docs/demo-evidence/S-8.09/AC-008-no-bin-emit-event-in-crate.webm

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

Type "# AC-008: host::emit_event replaces bin/emit-event; bin/emit-event NOT removed"
Enter
Sleep 800ms

Type "# No executable subprocess call to bin/emit-event in crate:"
Enter
Sleep 600ms

Type "grep -rn 'Command.*emit-event\|exec.*emit-event\|Process.*emit-event' crates/hook-plugins/regression-gate/src/ && echo 'FOUND' || echo 'NO EXEC REFS (AC-008 PASS)'"
Enter
Sleep 1500ms

Type "# host::emit_event IS used:"
Enter
Sleep 600ms

Type "grep -n 'host::emit_event' crates/hook-plugins/regression-gate/src/lib.rs | head -5"
Enter
Sleep 1500ms

Type "# bin/emit-event binary still present (D-10 deferral):"
Enter
Sleep 600ms

Type "test -f plugins/vsdd-factory/bin/emit-event && echo 'bin/emit-event PRESENT (D-10 respected)'"
Enter
Sleep 2s
