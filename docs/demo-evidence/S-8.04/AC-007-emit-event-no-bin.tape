Output docs/demo-evidence/S-8.04/AC-007-emit-event-no-bin.gif
Output docs/demo-evidence/S-8.04/AC-007-emit-event-no-bin.webm

Require bash

Set FontFamily "FiraCode Nerd Font Mono"
Set Theme "Catppuccin Mocha"
Set Width 1200
Set Height 600
Set FontSize 14
Set Padding 24

Hide
Type "cd /Users/jmagady/Dev/vsdd-factory/.worktrees/S-8.04"
Enter
Sleep 500ms
Show

Type "# AC-007: host::emit_event replaces bin/emit-event; bin/emit-event NOT removed (D-10)"
Enter
Sleep 500ms

Type "grep -c emit_event crates/hook-plugins/update-wave-state-on-merge/src/main.rs"
Enter
Sleep 800ms

Type "grep bin/emit-event crates/hook-plugins/update-wave-state-on-merge/src/main.rs 2>&1 || echo NO_BIN_EMIT_EVENT_REF"
Enter
Sleep 800ms

Type "ls bin/emit-event 2>/dev/null && echo EXISTS_NOT_REMOVED || echo MISSING_UNEXPECTED"
Enter
Sleep 2000ms
