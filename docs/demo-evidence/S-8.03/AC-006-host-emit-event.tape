Output docs/demo-evidence/S-8.03/AC-006-host-emit-event.gif
Output docs/demo-evidence/S-8.03/AC-006-host-emit-event.webm

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

Type "# AC-006: host::emit_event used; no reference to bin/emit-event in crate"
Enter
Sleep 500ms

Type "grep -n 'emit_event\\|bin/emit-event' crates/hook-plugins/track-agent-stop/src/lib.rs"
Enter
Sleep 1500ms

Type "grep -rn 'bin/emit-event' crates/hook-plugins/track-agent-stop/ 2>&1 || echo 'CONFIRMED: no bin/emit-event references'"
Enter
Sleep 1500ms

Type "ls -lh target/wasm32-wasip1/release/track-agent-stop.wasm"
Enter
Sleep 1500ms

Sleep 2s
