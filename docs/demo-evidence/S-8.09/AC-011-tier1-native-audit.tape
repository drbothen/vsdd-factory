Output docs/demo-evidence/S-8.09/AC-011-tier1-native-audit.gif
Output docs/demo-evidence/S-8.09/AC-011-tier1-native-audit.webm

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

Type "# AC-011: 0 Tier 1 legacy-bash-adapter refs -- W-15 closure audit"
Enter
Sleep 800ms

Type "grep -n 'legacy-bash-adapter' plugins/vsdd-factory/hooks-registry.toml | grep -E 'handoff-validator|pr-manager|track-agent|update-wave|validate-pr|session-learning|warn-pending|regression-gate' || echo '0 TIER 1 REFS (AC-011 PASS)'"
Enter
Sleep 1500ms

Type "test -f docs/E-8-tier1-native-audit.md && echo 'AUDIT DOC EXISTS'"
Enter
Sleep 1000ms

Type "grep 'YES' docs/E-8-tier1-native-audit.md | head -9"
Enter
Sleep 1500ms

Type "grep 'W-15 Tier 1 migration complete' docs/E-8-tier1-native-audit.md"
Enter
Sleep 2s
