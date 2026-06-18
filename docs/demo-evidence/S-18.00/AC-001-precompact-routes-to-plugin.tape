# AC-001 / PC1 — PreCompact routes to registered plugin
# BC-1.15.001 PC1: dispatcher iterates registered plugins in priority order,
# invokes each via the standard protocol, accumulates results.
#
# Story: S-18.00  VHS: 0.11.0
# Run from worktree root: vhs docs/demo-evidence/S-18.00/AC-001-precompact-routes-to-plugin.tape

Require bash

Output docs/demo-evidence/S-18.00/AC-001-precompact-routes-to-plugin.gif
Output docs/demo-evidence/S-18.00/AC-001-precompact-routes-to-plugin.webm

Set Shell bash
Set FontSize 14
Set FontFamily "Menlo"
Set Width 900
Set Height 300
Set Padding 20
Set Theme "Dracula"

Sleep 500ms
Type "bash docs/demo-evidence/S-18.00/demo-run-ac001.sh"
Enter
Sleep 3s
