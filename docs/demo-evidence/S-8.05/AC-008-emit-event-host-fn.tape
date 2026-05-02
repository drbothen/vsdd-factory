Output docs/demo-evidence/S-8.05/AC-008-emit-event-host-fn.gif
Output docs/demo-evidence/S-8.05/AC-008-emit-event-host-fn.webm

Require grep

Set FontFamily "FiraCode Nerd Font Mono"
Set Theme "Catppuccin Mocha"
Set Width 1200
Set Height 600
Set FontSize 14
Set Padding 24

Hide
Type "cd /Users/jmagady/Dev/vsdd-factory/.worktrees/S-8.05"
Enter
Sleep 500ms
Show

Type "# AC-008: host::emit_event replaces bin/emit-event; bare statement form"
Enter
Sleep 300ms

Type "grep -n 'emit_event' crates/hook-plugins/validate-pr-review-posted/src/main.rs"
Enter
Sleep 1500ms

Type "grep -rn 'bin/emit-event' crates/hook-plugins/validate-pr-review-posted/ 2>&1 || echo 'ABSENT — no bin/emit-event reference (expected)'"
Enter
Sleep 1500ms

Type "ls -la bin/emit-event 2>&1 | head -2"
Enter
Sleep 1500ms
