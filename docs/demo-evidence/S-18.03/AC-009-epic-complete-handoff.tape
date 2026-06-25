# AC-009 (EC-EPIC) — EPIC-COMPLETE path: HANDOFF.md with epic_status: complete
# No wave-state.yaml on factory-artifacts
# HANDOFF.md has epic_status: complete, next_wave_stories: []
# Expected: "Epic complete — no next-wave stories", STATE.md + arch_files injected, exit 0

Output docs/demo-evidence/S-18.03/AC-009-epic-complete-handoff.gif
Output docs/demo-evidence/S-18.03/AC-009-epic-complete-handoff.webm

Require bash

Set Theme "Dracula"
Set Width 1200
Set Height 700
Set Padding 20
Set FontSize 14

Type "# AC-009 (EC-EPIC): EPIC-COMPLETE path"
Enter
Sleep 400ms
Type "# No wave-state.yaml on factory-artifacts"
Enter
Sleep 400ms
Type "# HANDOFF.md has epic_status: complete, next_wave_stories: []"
Enter
Sleep 400ms
Type "# Expected: no RehydrationError, injects STATE.md + arch_files, exit 0"
Enter
Sleep 600ms
Type ""
Enter
Sleep 200ms

Type "bash /tmp/demo-ac009.sh"
Enter
Sleep 5s
