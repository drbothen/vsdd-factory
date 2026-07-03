# AC-002 / PC2 — PostCompact advisory-only: exit-2 + on_error=block → exit 0, no block_intent
# Contrast with AC-004: PostCompact NEVER sets block_intent regardless of plugin exit code.
# BC-1.15.001 PC2: PostCompact dispatch propagates exit codes but never triggers block.
#
# Story: S-18.00  VHS: 0.11.0
# Run from worktree root: vhs docs/demo-evidence/S-18.00/AC-002-postcompact-advisory-only.tape

Require bash

Output docs/demo-evidence/S-18.00/AC-002-postcompact-advisory-only.gif
Output docs/demo-evidence/S-18.00/AC-002-postcompact-advisory-only.webm

Set Shell bash
Set FontSize 14
Set FontFamily "Menlo"
Set Width 900
Set Height 300
Set Padding 20
Set Theme "Dracula"

Sleep 500ms
Type "bash docs/demo-evidence/S-18.00/demo-run-ac002.sh"
Enter
Sleep 3s
