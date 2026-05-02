# AC-7: vsdd-hook-sdk minor version bump

**Criterion:** `crates/hook-sdk/Cargo.toml` version bumped from `0.1.0` to `0.2.0`.
CHANGELOG entry added under "## Added" for `host::write_file`.

**Trace:** BC-2.02.011 postcondition 5 (path resolution error / no parent directory).

---

## Version in Cargo.toml

```toml
name = "vsdd-hook-sdk"
version = "0.2.0"
```

Confirmed: `grep '^version' crates/hook-sdk/Cargo.toml` returns `version = "0.2.0"`.

---

## CHANGELOG.md Entry

```markdown
## [0.2.0] - 2026-05-02

### Added
- `host::write_file(path, contents, max_bytes, timeout_ms) -> Result<(), HostError>` SDK API
  for hook plugins to write to host-allowlisted paths. New WriteFileCaps capability schema
  (`path_allow`, `max_bytes_per_call`). HOST_ABI_VERSION unchanged at 1 (D-6 Option A
  additive-only). See BC-2.02.011 for full invariants.

### Security
- Fixed path-traversal vulnerability in `path_allowed()` for both `read_file` and `write_file`
  dispatcher bindings: paths are now canonicalized before the allowlist prefix check, preventing
  `../` escapes from bypassing capability gates. (BC-2.02.011 EC-001 / BC-2.02.001 EC-001 —
  same bug existed in both host functions.)
```

---

## CHANGELOG Test

```
test test_BC_2_02_011_ac7_changelog_contains_write_file_entry ... ok
```

---

## hook-sdk-macros Peer-Dep Decision

`crates/hook-sdk-macros/Cargo.toml` version remains at `0.1.0`. No new macro surface
was added in S-8.10 — the `write_file` function does not require any proc-macro support.
The `vsdd-hook-sdk-macros` pin in `crates/hook-sdk/Cargo.toml` remains at `0.1.0`.
Rationale documented here per AC-7 note.

**Status: PASS**
