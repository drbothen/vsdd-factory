# AC-004 / PC4 / VP-086 — PreCompact exit-2 → block_intent=true, dispatcher exits 2
# VP-086 property: PreCompact plugin exits 2; dispatcher propagates block_intent=true.
# A single exit-2 plugin is sufficient to trigger block (EC-001).
#
# Story: S-18.00  VHS: 0.11.0
# Run from worktree root: vhs docs/demo-evidence/S-18.00/AC-004-precompact-exit2-blocks.tape

Require bash

Output docs/demo-evidence/S-18.00/AC-004-precompact-exit2-blocks.gif
Output docs/demo-evidence/S-18.00/AC-004-precompact-exit2-blocks.webm

Set Shell bash
Set FontSize 14
Set FontFamily "Menlo"
Set Width 900
Set Height 300
Set Padding 20
Set Theme "Dracula"

Sleep 500ms
Type "bash docs/demo-evidence/S-18.00/demo-run-ac004.sh"
Enter
Sleep 3s
