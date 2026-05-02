# AC-002 Evidence — Bash Deletion and hooks.json Removal

**AC:** AC-002 (traces to BC-7.03.081 invariant 1)
**Statement:** `hooks.json` command entry for `track-agent-stop` is absent from all
platform-specific files and template (positive verification: grep returns no match).
`plugins/vsdd-factory/hooks/track-agent-stop.sh` is absent from the repository tree.

## Recording

- `AC-002-bash-deletion.gif` / `.webm` / `.tape`

## Verification Steps Shown

1. `ls plugins/vsdd-factory/hooks/track-agent-stop.sh 2>&1 || echo 'CONFIRMED: .sh deleted'`
   — shell exits non-zero, fallback confirms deletion.
2. `grep -r 'track-agent-stop' plugins/vsdd-factory/hooks/hooks.json.* 2>&1 || echo 'CONFIRMED: no hooks.json entries'`
   — grep finds no matches across all 6 platform files (darwin-arm64, darwin-x64,
   linux-arm64, linux-x64, windows-x64, template).

## Result

PASS — `.sh` deleted; all `hooks.json.*` entries absent; positive grep returns no match.
