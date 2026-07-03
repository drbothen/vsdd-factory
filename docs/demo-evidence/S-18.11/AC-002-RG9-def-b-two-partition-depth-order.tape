# VHS Demo Tape — S-18.11 AC-002 + RG-9
# AC-002 (BC-5.41.004 PC3 v1.4 def-b): two-partition ordering —
#   Partition A (terminal) before Partition B (non-terminal); intra-partition sort by
#   (full-graph wave-depth ASC, lex ASC). No phantom wave: field (INV-3).
# RG-9 (test_partitions_sorted_by_full_graph_depth_def_b): independently computes
#   full-graph depths from STORY-INDEX depends_on edges; asserts both partitions
#   monotonically non-decreasing by depth + lex tie-break + done-first contiguity.
#   Spot anchors: S-8.09 depth=4 precedes S-9.00 depth=5; S-4.07 depth=11 precedes S-4.08 depth=12.
# Also demonstrates test_sprint_state_stories_wave_order (AC-002 CI-portable fixture test).

Output docs/demo-evidence/S-18.11/AC-002-RG9-def-b-two-partition-depth-order.gif
Output docs/demo-evidence/S-18.11/AC-002-RG9-def-b-two-partition-depth-order.webm

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

Type "# AC-002 (BC-5.41.004 PC3 INV-3): wave-ascending, no phantom wave: field"
Enter
Type "bats --filter test_sprint_state_stories_wave_order plugins/vsdd-factory/tests/sprint-state-format.bats"
Enter
Sleep 6s

Type "# RG-9 (BC-5.41.004 PC3 def-b): full-graph depth order both partitions"
Enter
Type "bats --filter test_partitions_sorted_by_full_graph_depth_def_b plugins/vsdd-factory/tests/sprint-state-format.bats"
Enter
Sleep 6s
Type "echo exit:$?"
Enter
Sleep 1s
