# VHS Demo Tape — S-18.11 AC-001 + AC-003 + RG-7
# AC-001 (BC-5.41.004 PC1+PC2+INV-1): production sprint-state.yaml has top-level
#   stories: list with per-story {id, status} entries; stories: is a YAML sequence.
# AC-003 (BC-5.41.004 PC2+INV-2+EC-007): 149 stories; statuses match STORY-INDEX.
# RG-7 (test_real_production_file_completeness_and_status_fidelity):
#   PC4 completeness (no phantom/missing IDs) + PC2/INV-2 status-fidelity on real file.
# Shows: bats -f test_sprint_state_stories_list_present + test_real_production_file_completeness_and_status_fidelity

Output docs/demo-evidence/S-18.11/AC-001-AC-003-RG7-stories-list-and-status-fidelity.gif
Output docs/demo-evidence/S-18.11/AC-001-AC-003-RG7-stories-list-and-status-fidelity.webm

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

Type "# AC-001 (BC-5.41.004 PC1+PC2): stories: list present + per-story entries"
Enter
Type "bats --filter test_sprint_state_stories_list_present plugins/vsdd-factory/tests/sprint-state-format.bats"
Enter
Sleep 8s

Type "# RG-7 (BC-5.41.004 PC4+INV-2): 149 stories completeness + status fidelity"
Enter
Type "bats --filter test_real_production_file_completeness_and_status_fidelity plugins/vsdd-factory/tests/sprint-state-format.bats"
Enter
Sleep 12s
Type "echo exit:$?"
Enter
Sleep 1s
