# AC-006 (EC-003) — Missing spec file: WARNING on stderr, continues, confirmation prompt, exit 0
# wave-state.yaml lists [A.md, missing-spec.md, C.md]
# A.md and C.md exist; missing-spec.md is absent from both filesystem and factory-artifacts
# Expected: WARNING names missing-spec.md, A.md + C.md still injected, "Confirm rehydration:", exit 0

Output docs/demo-evidence/S-18.03/AC-006-missing-spec-warn-continue.gif
Output docs/demo-evidence/S-18.03/AC-006-missing-spec-warn-continue.webm

Require bash

Set Theme "Dracula"
Set Width 1200
Set Height 700
Set Padding 20
Set FontSize 14

Type "# AC-006 (EC-003): missing spec file — warn and continue"
Enter
Sleep 400ms
Type "# wave-state.yaml lists: A.md, missing-spec.md, C.md"
Enter
Sleep 400ms
Type "# A.md and C.md exist; missing-spec.md is absent"
Enter
Sleep 400ms
Type "# Skill must: warn, inject remaining files, exit 0"
Enter
Sleep 600ms
Type ""
Enter
Sleep 200ms

Type "bash /tmp/demo-ac006.sh"
Enter
Sleep 5s
