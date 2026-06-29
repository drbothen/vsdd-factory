# VHS Demo Tape — S-18.11 EC-010 + RG-8
# EC-010 (BC-5.41.004 v1.4 EC-010 tolerate path): supersession-edge tolerated —
#   S-3.04 (partial, superseded_by: ADR-015) in non-terminal partition;
#   merged dependents S-3.01/S-3.02/S-3.03/S-4.07/S-4.08 in terminal prefix.
#   Migration was EMITTED (supersession edge did NOT trigger TopoViolation abort).
# RG-8 (test_supersession_edge_tolerated_partition_placement): locks in the tolerate
#   path on real production data.

Output docs/demo-evidence/S-18.11/EC-010-RG8-supersession-tolerance.gif
Output docs/demo-evidence/S-18.11/EC-010-RG8-supersession-tolerance.webm

Require bash

Set Shell "bash"
Set FontFamily "Menlo"
Set FontSize 14
Set Width 1200
Set Height 560
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

Type "# EC-010 (BC-5.41.004): supersession-edge tolerated (S-3.04 in non-terminal partition)"
Enter
Type "bats --filter test_supersession_edge_tolerated_partition_placement plugins/vsdd-factory/tests/sprint-state-format.bats"
Enter
Sleep 8s
Type "echo exit:$?"
Enter
Sleep 1s
