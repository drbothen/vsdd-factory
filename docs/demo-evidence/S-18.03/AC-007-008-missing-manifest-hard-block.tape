# AC-007 / AC-008 — No wave-state.yaml + no EPIC-COMPLETE: RehydrationError + non-zero exit
# AC-007: hard block with canonical RehydrationError message
# AC-008: no file injection (no RAG fallback) — INJECTED_FILE_COUNT sentinel absent
# factory-artifacts branch is empty — no wave-state.yaml, no HANDOFF.md

Output docs/demo-evidence/S-18.03/AC-007-008-missing-manifest-hard-block.gif
Output docs/demo-evidence/S-18.03/AC-007-008-missing-manifest-hard-block.webm

Require bash

Set Theme "Dracula"
Set Width 1200
Set Height 700
Set Padding 20
Set FontSize 14

Type "# AC-007/008: no wave-state.yaml + no EPIC-COMPLETE HANDOFF.md"
Enter
Sleep 400ms
Type "# factory-artifacts branch is empty (no manifest at all)"
Enter
Sleep 400ms
Type "# Expected: RehydrationError on stderr, no file injection, non-zero exit"
Enter
Sleep 600ms
Type ""
Enter
Sleep 200ms

Type "bash /tmp/demo-ac007.sh ; echo Exit_code=$?"
Enter
Sleep 4s
