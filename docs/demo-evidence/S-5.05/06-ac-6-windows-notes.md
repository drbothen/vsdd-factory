# AC-6 — Windows-specific notes

**AC statement:** Section "Windows-specific notes" populated.

**Evidence type:** file snippet

## Section heading + content (lines 148-166)

```markdown
## Windows-specific notes

v1.0 ships four hooks as native WASM that work on Windows without git-bash:

- `capture-commit-activity`
- `capture-pr-activity`
- `block-ai-attribution`
- `emit_event` (host function, not a hook per se, but used by the above)

The remaining 26 hooks in the legacy inventory are bash scripts routed
through `legacy-bash-adapter.wasm`. On Windows, the adapter invokes them
via git-bash. If git-bash is not installed or not in `PATH`, those hooks
will fail silently (the dispatcher logs `exit_code: 1` on the
`plugin.completed` event, visible in `dispatcher-internal.jsonl`).

More native WASM ports are planned for post-1.0 stories (S-2.5 and
onwards), which will reduce the git-bash dependency surface on Windows.
For now, Windows operators should install git-bash from
https://gitforwindows.org/ before running the upgrade procedure.
```

## Commentary

Section distinguishes the 4 native WASM hooks (no git-bash needed) from
the 26 bash hooks that require git-bash via the legacy adapter. Includes
the failure mode (silent exit_code:1 in JSONL log) and the install URL.
