# AC-001/002/004/010 — Happy-path: valid wave-state.yaml → injected file list + INJECTED_FILE_COUNT sentinel
# AC-001: git-sourced read (stale working-tree copy ignored)
# AC-002: exact union injection with INJECTED_FILE_COUNT sentinel
# AC-004: STATE.md always injected (state_pointer)
# AC-010: deduplication — BC-6.24.001.md listed in 2 stories but counted once (INJECTED_FILE_COUNT=4)

Output docs/demo-evidence/S-18.03/AC-001-002-004-010-happy-path.gif
Output docs/demo-evidence/S-18.03/AC-001-002-004-010-happy-path.webm

Require bash

Set Theme "Dracula"
Set Width 1200
Set Height 700
Set Padding 20
Set FontSize 14

Type "# AC-001/002/004/010: rehydrate-wave happy-path"
Enter
Sleep 400ms
Type "# wave-state.yaml has 2 stories sharing BC-6.24.001.md (dedup test)"
Enter
Sleep 400ms
Type "# stale .factory/wave-state.yaml is present — skill must ignore it (git-sourced)"
Enter
Sleep 600ms
Type ""
Enter
Sleep 200ms

Type "bash /tmp/demo-ac001.sh"
Enter
Sleep 5s
