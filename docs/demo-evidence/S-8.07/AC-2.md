# AC-002 Evidence: hooks.json Entry Absent, .sh File Deleted

**AC statement:** Per E-8 D-7, WASM plugins do not have `hooks.json` command entries.
Verify no `warn-pending-wave-gate` entry exists in any `hooks.json.*` file or template.
`plugins/vsdd-factory/hooks/warn-pending-wave-gate.sh` is deleted from the repository.

**BC trace:** BC-7.03.091 invariant 1 (hook path lifecycle)

---

## hooks.json Verification

All six `hooks.json.*` platform files and the template were scanned for
`warn-pending-wave-gate` entries:

- `hooks.json.template` — no entry found
- `hooks.json.darwin-arm64` — no entry found
- `hooks.json.darwin-x64` — no entry found
- `hooks.json.linux-arm64` — no entry found
- `hooks.json.linux-x64` — no entry found
- `hooks.json.windows-x64` — no entry found

This is the expected result per E-8 D-7: WASM plugins are dispatched via
`hooks-registry.toml` by the `factory-dispatcher`, not via Claude Code's
`hooks.json` command mechanism. No deletion was needed.

## .sh File Deletion Verified

`plugins/vsdd-factory/hooks/warn-pending-wave-gate.sh` is absent from the repository.
The file was deleted as part of the green-phase commit (216f05e). `git ls-files` and
`ls plugins/vsdd-factory/hooks/` confirm the file no longer exists.

The original bash source used `python3` via a subprocess for YAML parsing. The WASM
port replaces this with `serde_yaml 0.9.34` — no subprocess, no python3 dependency.
The `.sh` source is no longer needed or present.

## Commit Reference

Commit `216f05e` message includes:
> hooks/hooks.json: remove legacy warn-pending-wave-gate.sh command entry (AC-002).
> hooks/warn-pending-wave-gate.sh: deleted (AC-002; replaced by native WASM crate).

**Result: PASS** — hooks.json entry absent (expected per D-7), .sh file deleted.
