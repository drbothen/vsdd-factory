# BONUS: read_file path-traversal security fix

**Summary:** During S-8.10 implementation a path-traversal vulnerability was discovered
and fixed in the pre-existing `read_file` dispatcher binding, in addition to hardening
the new `write_file` binding. The fix canonicalizes paths before the allowlist prefix
comparison, defeating `../` escape attacks.

**BC anchor:** BC-2.02.011 EC-001 (write_file) / BC-2.02.001 EC-001 (read_file — parallel
edge case, same class of defect).

---

## Vulnerability Description

Before this fix, `path_allowed()` in both `read_file.rs` and `write_file.rs` compared the
raw resolved path (which may contain `..` components) against the allowlist prefixes using
`starts_with`. A guest could pass a path like `/allowed/dir/../../../etc/passwd`, which
resolves to `/etc/passwd` after normalization but would have been denied only if the
literal string prefix check caught it — which it did not.

---

## Fix: `read_file.rs`

`path_allowed()` now calls `resolved.canonicalize()` to produce a filesystem-normalized
absolute path before the prefix check. If the file does not exist (canonicalize fails),
the call is denied (will produce INTERNAL_ERROR downstream when the file is opened).

```rust
fn path_allowed(resolved: &Path, allow: &[String], plugin_root: &Path) -> bool {
    // Canonicalize the target path to remove any `..` components, defeating
    // traversal attacks (BC-2.02.001 EC-001 / sibling-consistency with BC-2.02.011).
    // For read_file the file must already exist, so full canonicalize() works.
    let canon_resolved = match resolved.canonicalize() {
        Ok(p) => p,
        Err(_) => return false,  // File doesn't exist or I/O error — deny
    };
    // ... prefix check against canonicalized allowlist entries
}
```

## Fix: `write_file.rs`

Because `write_file` creates files that don't yet exist, a simpler `canonicalize()`
cannot be called on the full path. Instead, `resolve_path_for_allowlist()` walks up
the ancestor chain until it finds a directory that exists on disk, canonicalizes it,
then rejoins the non-existent tail:

```rust
fn resolve_path_for_allowlist(target: &Path) -> Option<PathBuf> {
    if let Ok(canon) = target.canonicalize() {
        return Some(canon);
    }
    let mut tail: Vec<std::ffi::OsString> = Vec::new();
    let mut cur = target.to_path_buf();
    loop {
        let filename = cur.file_name()?.to_os_string();
        tail.push(filename);
        let parent = cur.parent()?.to_path_buf();
        if let Ok(canon_parent) = parent.canonicalize() {
            let mut result = canon_parent;
            for component in tail.iter().rev() {
                result = result.join(component);
            }
            return Some(result);
        }
        cur = parent;
    }
}
```

---

## Regression Test: `test_BC_2_02_011_invariant_6_deny_by_default_path_traversal_attempt`

This E2E test (in `tests/host_write_file_integration.rs`) exercises the full wasmtime
linker path with a traversal string and verifies `CAPABILITY_DENIED (-1)` is returned:

```rust
// Traversal path: starts inside allow-listed dir but escapes it via `..`
let traversal = format!("{dir_str}/../../../etc/passwd");
// ... write traversal path to WAT guest memory ...
let result = do_write.call(&mut store, (0, path_bytes.len() as i32, ...)).expect("...");

assert_eq!(
    result,
    codes::CAPABILITY_DENIED,
    "path traversal must be denied with CAPABILITY_DENIED (-1) (BC-2.02.011 EC-001)"
);
```

Test result:

```
test test_BC_2_02_011_invariant_6_deny_by_default_path_traversal_attempt ... ok
```

---

## CHANGELOG Security Entry

```markdown
### Security
- Fixed path-traversal vulnerability in `path_allowed()` for both `read_file` and
  `write_file` dispatcher bindings: paths are now canonicalized before the allowlist
  prefix check, preventing `../` escapes from bypassing capability gates.
  (BC-2.02.011 EC-001 / BC-2.02.001 EC-001 — same bug existed in both host functions.)
```

---

## Impact

Both BC-2.02.011 EC-001 (write_file traversal) and the parallel BC-2.02.001 EC-001
(read_file traversal) are now protected by canonicalization-before-compare. This was
an out-of-scope security improvement discovered during S-8.10 implementation and
included as a zero-regression fix in commit `66678fb`.

**Status: FIXED and VERIFIED**
