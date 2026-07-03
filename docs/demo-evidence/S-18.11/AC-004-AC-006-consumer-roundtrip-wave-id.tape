# VHS Demo Tape — S-18.11 AC-004 + AC-006
# AC-004 (BC-5.41.002 PC3; BC-5.41.001 PC2+PC3): wave-handoff consumer round-trip —
#   migrated sprint-state.yaml parsed; S-1.02 (draft) in next_wave_stories;
#   S-1.01 (merged) excluded; no BrokenSprintState; exit 0.
# AC-006 (BC-5.41.001 PC2; BC-5.41.004 PC3+INV-3): wave_id = wave-group ordinal —
#   fixture-leading-run.yaml: 10 merged in ONE block + 2 draft →
#   one completed terminal wave group → wave_id = 2 (NOT 11).
# ADR-026 §Wave-Identity Derivation.

Output docs/demo-evidence/S-18.11/AC-004-AC-006-consumer-roundtrip-wave-id.gif
Output docs/demo-evidence/S-18.11/AC-004-AC-006-consumer-roundtrip-wave-id.webm

Require bash

Set Shell "bash"
Set FontFamily "Menlo"
Set FontSize 14
Set Width 1200
Set Height 580
Set Theme "Catppuccin Mocha"
Set WindowBar Colorful
Set BorderRadius 8
Set Padding 20
Set Framerate 30
Set TypingSpeed 40ms
Set WaitTimeout 30s

Hide
Type "PS1='demo $ '"
Enter
Sleep 0.3s
Show

Type "# AC-004 (BC-5.41.002 PC3): consumer round-trip — draft appears, merged excluded"
Enter
Type "bats --filter test_wave_handoff_parses_migrated_sprint_state plugins/vsdd-factory/tests/sprint-state-format.bats"
Enter
Sleep 10s

Type "# AC-006 (BC-5.41.001 PC2): wave_id = wave-group ordinal; 10-merged-1-block = wave_id 2"
Enter
Type "bats --filter test_wave_id_wave_group_ordinal plugins/vsdd-factory/tests/sprint-state-format.bats"
Enter
Sleep 6s
Type "echo exit:$?"
Enter
Sleep 1s
