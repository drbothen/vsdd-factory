# AC-002 — Live gate-firing evidence: dispatcher-integration bats tests
# Story S-18.13: wave-handoff skill writes HANDOFF.md via Write tool (gate-trigger fix)
# BC-5.41.001 PC10 — PostToolUse validate-wave-handoff-completeness gate fires
# Test approach: mock PostToolUse injection (AC-002 option b)
# Positive: complete HANDOFF.md → factory-dispatcher + gate → exit 0, no block
# Negative: incomplete HANDOFF.md → exit 2, blocking_plugins=validate-wave-handoff-completeness

Output docs/demo-evidence/S-18.13/AC-002-postuse-gate-firing-live-evidence.gif
Output docs/demo-evidence/S-18.13/AC-002-postuse-gate-firing-live-evidence.webm

Set Shell "zsh"
Set FontFamily "Menlo"
Set FontSize 13
Set Width 1200
Set Height 550
Set Theme "Catppuccin Mocha"
Set WindowBar Colorful
Set BorderRadius 8
Set Padding 20
Set Framerate 30
Set TypingSpeed 50ms

Hide
Type "cd /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-18.13 && clear"
Enter
Sleep 1s
Show

Type "# AC-002: gate-firing live evidence (positive + negative dispatcher-integration)"
Enter
Sleep 0.5s

Type "cd plugins/vsdd-factory/tests && bats --filter 'test_BC_5_41_001_PC10_S18_13_AC002_postuse_gate_fires' wave-handoff.bats"
Enter
Sleep 30s

Sleep 2s
