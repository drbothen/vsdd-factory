# AC-003 Evidence — .factory/ absent: exit 0 immediately, no file created

**Story:** S-8.06 — Native port: session-learning (Stop)
**BC trace:** BC-7.03.078 postcondition 1 (skips when .factory/ absent)

---

## Claim

When `.factory/` directory does not exist: WASM plugin exits 0 immediately without
creating any files or writing to stderr. The guard is the first check in the plugin
body (before any file I/O attempts).

---

## Implementation excerpt (crates/hook-plugins/session-learning/src/lib.rs)

### .factory/ existence guard (lines 75-83) — first check in hook body

```rust
let factory_dir = if fs_root.is_empty() || fs_root == "." {
    Path::new(".factory").to_path_buf()
} else {
    Path::new(fs_root).join(".factory")
};

if !factory_dir.is_dir() {
    return HookResult::Continue;
}
```

`HookResult::Continue` maps to exit code 0 via `result.exit_code()` in `main.rs`.
No file creation, no stderr output. This is the first branch reached after stdin drain.

---

## Bats test output — Case 3 (AC-003)

```
ok 3 T5-case3 (AC-003): invocation without .factory/ exits 0, no file created
```

Test assertions verified:
- `[ ! -d "${FACTORY_DIR}" ]` — precondition: .factory/ does not exist
- `run run_plugin '{}'` — invoke the WASM plugin
- `[ "$status" -eq 0 ]` — exit 0
- `[ ! -f "${SIDECAR_FILE}" ]` — sidecar-learning.md was not created
- `[ ! -d "${FACTORY_DIR}" ]` — .factory/ directory was not created either

---

## Error path note

The guard returns `HookResult::Continue` (not `HookResult::Error`) because the absent
`.factory/` directory is a valid operational state — the hook simply has nothing to do.
The `on_error = "continue"` registry setting would handle an error too, but this path
avoids logging a spurious failure entirely. This matches the bash source's `exit 0`
in the absence check.

---

**Verdict: AC-003 PASS**
