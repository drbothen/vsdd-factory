Output docs/demo-evidence/S-8.03/AC-003-exit-classification.gif
Output docs/demo-evidence/S-8.03/AC-003-exit-classification.webm

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

Type "# AC-003: BC-2.02.012 fallback chains + exit_class + byte-count RESULT_LEN + multiline BLOCKED"
Enter
Sleep 500ms

Type "grep -n 'agent_type\|subagent_name\|last_assistant_message' crates/hook-sdk/src/payload.rs | grep pub"
Enter
Sleep 1500ms

Type "grep -n 'as_deref\|BLOCKED_PATTERN\|bytes.*filter\|emit_event' crates/hook-plugins/track-agent-stop/src/lib.rs | head -12"
Enter
Sleep 1500ms

Sleep 2s
