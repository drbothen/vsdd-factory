Output docs/demo-evidence/S-12.02/AC-003-graceful-degrade-no-wave-gate.gif

Require cargo

Set FontFamily "FiraCode Nerd Font Mono"
Set Theme "Dracula"
Set Width 1200
Set Height 600
Set FontSize 14
Set Padding 20

Hide
Type "cd /Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.02"
Enter
Sleep 0.5
Show

Type "cargo test -p validate-per-story-adversary-convergence test_BC_4_10_002_graceful_degrade_no_wave_gate_context -- --nocapture 2>&1 | tail -12"
Enter
Sleep 8

Sleep 2
