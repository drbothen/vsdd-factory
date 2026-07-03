# AC-008 / INV3 — check-harness-version.sh: version >= threshold → exit 0; unset → exit 1 advisory
# BC-1.15.001 INV3: check-harness-version.sh registered as PreCompact plugin with
# on_error=continue. Never blocks. Shows success path, advisory (unset), advisory (below threshold).
#
# Story: S-18.00  VHS: 0.11.0
# Run from worktree root: vhs docs/demo-evidence/S-18.00/AC-008-check-harness-version.tape

Require bash

Output docs/demo-evidence/S-18.00/AC-008-check-harness-version.gif
Output docs/demo-evidence/S-18.00/AC-008-check-harness-version.webm

Set Shell bash
Set FontSize 14
Set FontFamily "Menlo"
Set Width 900
Set Height 380
Set Padding 20
Set Theme "Dracula"

Sleep 500ms
Type "bash docs/demo-evidence/S-18.00/demo-run-ac008.sh"
Enter
Sleep 3s
