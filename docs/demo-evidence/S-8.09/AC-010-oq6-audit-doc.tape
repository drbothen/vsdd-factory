Output docs/demo-evidence/S-8.09/AC-010-oq6-audit-doc.gif
Output docs/demo-evidence/S-8.09/AC-010-oq6-audit-doc.webm

Set FontFamily "FiraCode Nerd Font Mono"
Set FontSize 14
Set Width 1200
Set Height 700
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

Type "# AC-010: OQ-6 security-reviewer audit complete (RESOLVED)"
Enter
Sleep 800ms

Type "test -f docs/oq-6-regression-gate-security-audit.md && echo 'AUDIT DOC EXISTS'"
Enter
Sleep 1000ms

Type "grep -E 'signoff_agent_id|signoff_timestamp|recommended_binary_allow' docs/oq-6-regression-gate-security-audit.md | head -5"
Enter
Sleep 1500ms

Type "# Falsifiable check: signoff_agent_id is non-empty:"
Enter
Sleep 600ms

Type "grep 'signoff_agent_id:' docs/oq-6-regression-gate-security-audit.md"
Enter
Sleep 1500ms

Type "# Recommended capabilities: read_file + write_file + emit_event (no jq needed)"
Enter
Sleep 2s
