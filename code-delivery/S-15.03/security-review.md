# S-15.03 Security Review — `last-amended-migrate` CLI

**PR:** #805 (feature/S-15.03 -> develop)
**Reviewer:** `vsdd-factory:security-reviewer` (dispatched by pr-manager during PR lifecycle
step 4, per this project's CANONICAL PRINCIPLE — security findings are fixed in-scope, not
deferred)
**Scope:** New crate `crates/last-amended-migrate/` (CLI reads/writes 5 governed `.factory/`
frontmatter files; no network I/O; no secrets handling)

## Summary

3 findings, all fixed and re-verified before merge. No critical/high findings remain
unresolved.

| # | Finding | Severity | CWE | Fix | Commit |
|---|---------|----------|-----|-----|--------|
| SEC-001 | Incomplete output escaping + dead `InvalidYamlProduced` pre-write gate | HIGH | CWE-116 (Improper Encoding/Escaping of Output) | `escape.rs` extended to escape every C0 control char (not just literal `"`); new `yaml_guard.rs` wires up a strict-YAML `safe_load` validation immediately before every write in `migrate_file`/`rotate_changelog`, returning `InvalidYamlProduced` instead of writing corrupt output. `serde`/`serde_norway` moved from dev- to real dependencies to support the validator. | `8bd6dde2` |
| SEC-002 | No path allowlist on `--path`/`--registry` CLI args | MEDIUM | CWE-73 (External Control of File Name or Path) | New `path_guard.rs` enforces: `migrate --path` must resolve to one of the 5 `TARGET_FILES` under `--factory-root`; `rotate --path` must sit under a `.factory/` ancestor; `register --registry` must match the full expected trailing path shape (`plugins/vsdd-factory/config/artifact-path-registry.yaml`, tightened during review cycle 2 from an initial basename-only check per finding S4). New `MigrateError::PathNotAllowed` variant. | `8efc2753` (initial), `4ff64788` (S4 tightening) |
| SEC-003 | TOCTOU (time-of-check-to-time-of-use) race condition on read-then-write file operations | LOW | CWE-367 (TOCTOU Race Condition) | New `atomic_write.rs::write_atomic` (temp-sibling-file + atomic `rename`) now backs every write in `migrate_file`, `rotate_changelog`, and `register_artifact_paths`. Extended during review (finding N2) to preserve the pre-existing target file's permission bits and `fsync` the temp file (plus best-effort directory fsync on Unix) before rename. | `74efd406` (initial), `6944b620` (N2 hardening) |

## Detail — SEC-001 (HIGH, CWE-116)

**Before:** `escape_value` only escaped a literal `"` character. Any other control character
(newline, tab, other C0 controls) embedded in a `last_amended`/`changelog` text field passed
through unescaped into a double-quoted YAML scalar, which either produces invalid YAML (parse
failure on next read) or, in edge cases, could shift where a scalar terminates — a form of
output-encoding defect. `MigrateError::InvalidYamlProduced` existed in the error enum but was
never constructed or checked anywhere — a dead gate.

**After:** `escape.rs` escapes every C0 control character (`\n`, `\r`, `\t` via their named YAML
escapes; other C0 controls via `\xHH`). `yaml_guard.rs` performs a strict `serde_norway`
`safe_load` parse of the fully-rendered output immediately before every write and returns
`Err(InvalidYamlProduced)` — refusing to write — if the produced content does not parse as
valid YAML. This was further hardened in review cycle 2 (finding S1) to also escape literal
backslashes not part of a recognized escape sequence, using bounded 3-char lookahead
classification, so Windows-style paths (`C:\Users\config`) round-trip correctly rather than
being rejected by the new guard.

**Verification:** `bc_10_13_001_sec001_yaml_gate_test.rs` and the S1 backslash-escaping tests
(6 dedicated tests including an idempotency check and a strict-YAML round-trip) — all pass.
Independently re-verified by the cycle-3 pr-reviewer via direct code read, not taken on trust.

## Detail — SEC-002 (MEDIUM, CWE-73)

**Before:** `migrate --path <arbitrary>`, `rotate --path <arbitrary>`, and
`register --registry <arbitrary>` accepted any filesystem path with no allowlist — a
CLI-argument-controlled write target with no scoping to the 5 intended governed files or the
`.factory/` tree.

**After:** `path_guard.rs` enforces per-subcommand allowlists (see table above). Initially
`register --registry` checked only the basename, which cycle-2 finding S4 correctly identified
as insufficient (a same-named file in a different directory would pass); tightened to compare
the full trailing path shape after canonicalization.

**Verification:** `bc_10_13_001_sec002_path_allowlist_test.rs` plus
`..._same_basename_wrong_directory_is_rejected` (added for S4) — all pass.

## Detail — SEC-003 (LOW, CWE-367)

**Before:** Writers read the current file content, computed a new value, then wrote it back
with a plain overwrite — a window exists between read and write where a concurrent process
could modify the file, and a crash mid-write could leave a corrupt/truncated file on disk.

**After:** `atomic_write.rs::write_atomic` writes to a temp file in the same directory (so the
subsequent `rename` is on the same filesystem and thus atomic), `fsync`s it, and atomically
renames it over the target — eliminating the corrupt-partial-write failure mode. Extended
(finding N2) to preserve the target's existing file-permission bits (so `write_atomic` doesn't
silently reset e.g. a `0600`-mode file to the process umask default) and to `fsync` the
containing directory on Unix as a best-effort durability improvement.

**Verification:** `bc_10_13_001_sec003_atomic_write_test.rs` plus
`test_BC_10_13_001_N2_write_atomic_preserves_non_default_file_mode` — all pass.

## Correction (transparency note)

An earlier draft of this PR's description claimed write targets were "constrained to the 5
explicitly-registered sidecar-eligible paths via `eligibility.rs`" at a point before SEC-002
had landed. That claim was inaccurate at the time it was written (`eligibility.rs` only
classifies a string; it never touched or constrained paths). This was corrected once SEC-002's
actual path-allowlist enforcement landed in `path_guard.rs`/`cli.rs`. Recorded here per this
project's production-grade-default principle (no overclaiming in spec/review artifacts).

## Non-findings (checked, no issue)

- No new network I/O in this crate.
- No new external dependencies beyond what's already in the workspace (`serde`/`serde_norway`
  moved from dev- to real dependencies as part of SEC-001; both were already used elsewhere).
- No secrets handling — the CLI operates only on `.factory/` markdown frontmatter text.
- `register_artifact_paths`'s `description:` field values are compile-time constants (not
  attacker-controlled input), so cycle-3 finding S8 (it skips the SEC-001 `yaml_guard` gate and
  emits an unquoted plain scalar) is safe today; it is tracked as a non-blocking follow-up
  suggestion rather than a security finding, since no attacker-reachable path exists.

## Disposition

**No critical/high findings remain unresolved.** All 3 findings fixed and independently
re-verified by the cycle-3 fresh-eyes `pr-reviewer` pass (which re-derived each fix from the
code and tests rather than taking commit messages on trust). Merge is not blocked on security
grounds.

https://claude.ai/code/session_01NEupPWaRRWmhr8uSsD5YGg
