# AC-002: Bash file deleted; hooks.json entry positively absent

**BC trace:** BC-7.04.040 invariant 1
**Status:** PASS

## What was verified

- `plugins/vsdd-factory/hooks/validate-pr-review-posted.sh` has been deleted.
  `ls` confirms: `No such file or directory`.

- Positive verification: `grep -r validate-pr-review-posted plugins/vsdd-factory/hooks/hooks.json*`
  returns no results across all six platform files:
  - `hooks.json` (template)
  - `hooks.json.darwin-arm64`
  - `hooks.json.darwin-x64`
  - `hooks.json.linux-arm64`
  - `hooks.json.linux-x64`
  - `hooks.json.windows-x64`

Native WASM plugins route via `hooks-registry.toml` only (E-8 D-7 / DRIFT-004).
The dispatcher will never invoke `validate-pr-review-posted` via the hooks.json command path.

## Recording

[AC-002-bash-deleted-hooksjson-absent.gif](AC-002-bash-deleted-hooksjson-absent.gif)
