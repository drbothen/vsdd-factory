//! `write_file` host function (BC-2.02.011).
//!
//! Receives a guest-owned byte slice via the **input-pointer protocol**
//! (`contents_ptr`, `contents_len`), validates the destination path against
//! the plugin's `capabilities.write_file.path_allow` allowlist, enforces a
//! mandatory `max_bytes` cap, and writes the bytes to the filesystem with
//! `std::fs::write`.
//!
//! Capability model: deny-by-default. If the plugin has no
//! `Capabilities::write_file` block, every call returns
//! `CAPABILITY_DENIED (-1)`. Paths that escape the allow-list (e.g. via
//! `..` traversal) are denied and emit `internal.capability_denied`.
//!
//! Protocol difference from `read_file`: `read_file` uses the
//! **output-pointer protocol** (host writes into guest memory via
//! out-params); `write_file` uses the **input-pointer protocol** (the SDK
//! passes guest-owned bytes and the dispatcher copies them via
//! `read_wasm_bytes`). BC-2.02.011 invariant 4.

use std::path::{Path, PathBuf};

use serde_json::{Map, Value};
use wasmtime::Linker;

use super::memory::read_wasm_bytes;
use super::{HostCallError, HostCaller, HostContext, codes};

pub fn register(linker: &mut Linker<HostContext>) -> Result<(), HostCallError> {
    linker
        .func_wrap(
            "vsdd",
            "write_file",
            |mut caller: HostCaller<'_>,
             path_ptr: u32,
             path_len: u32,
             contents_ptr: u32,
             contents_len: u32,
             max_bytes: u32,
             timeout_ms: u32|
             -> i32 {
                let _ = timeout_ms; // accepted for ABI stability; enforced in S-1.5 via epoch interruption

                // Read path from guest memory.
                let path_bytes = match read_wasm_bytes(&mut caller, path_ptr, path_len) {
                    Ok(b) => b,
                    Err(_) => return codes::INVALID_ARGUMENT,
                };
                let path = match std::str::from_utf8(&path_bytes) {
                    Ok(s) => s.to_string(),
                    Err(_) => return codes::INVALID_ARGUMENT,
                };

                // Read contents from guest memory (input-pointer protocol).
                let contents = match read_wasm_bytes(&mut caller, contents_ptr, contents_len) {
                    Ok(b) => b,
                    Err(_) => return codes::INVALID_ARGUMENT,
                };

                let ctx = caller.data().clone();
                match prepare(&ctx, &path, &contents, max_bytes) {
                    Ok(()) => codes::OK,
                    Err(code) => code,
                }
            },
        )
        .map_err(|e| HostCallError::Linker(e.to_string()))?;
    Ok(())
}

/// All of write_file's host-side logic that doesn't touch guest memory,
/// split out so it's unit-testable without a live wasm instance.
///
/// BC-2.02.011 postconditions 1-5.
fn prepare(ctx: &HostContext, path: &str, contents: &[u8], max_bytes: u32) -> Result<(), i32> {
    // Postcondition 1: deny-by-default capability check (BC-2.02.011 §1).
    let caps = ctx.capabilities.write_file.as_ref().ok_or_else(|| {
        emit_denial(ctx, path, "no_write_file_capability", None);
        codes::CAPABILITY_DENIED
    })?;

    let resolved = resolve_for_write(Path::new(path), &ctx.plugin_root);

    // Postcondition 1: path allowlist + traversal denial.
    if !path_allowed(&resolved, &caps.path_allow, &ctx.plugin_root) {
        emit_denial(ctx, path, "path_not_allowed", Some(&resolved));
        return Err(codes::CAPABILITY_DENIED);
    }

    // Postcondition 2: byte cap enforced before any write (BC-2.02.011 §2).
    // Effective cap: minimum of the call argument and the per-capability override.
    let effective_cap = match caps.max_bytes_per_call {
        Some(cap_override) => max_bytes.min(cap_override),
        None => max_bytes,
    };
    if contents.len() as u64 > effective_cap as u64 {
        emit_denial(ctx, path, "output_too_large", Some(&resolved));
        return Err(codes::OUTPUT_TOO_LARGE);
    }

    // Postcondition 3 / 5: write or propagate I/O error.
    std::fs::write(&resolved, contents).map_err(|_e| {
        // Postcondition 5: path resolution / missing parent → INTERNAL_ERROR.
        // Mirrors `read_file.rs` Err(ReadErr::Other) → codes::INTERNAL_ERROR.
        codes::INTERNAL_ERROR
    })
}

/// Mirror of `read_file::resolve_for_read`: absolute paths pass through;
/// relative paths are joined with `plugin_root`.
/// BC-2.02.011 invariant 3.
fn resolve_for_write(path: &Path, plugin_root: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        plugin_root.join(path)
    }
}

/// Resolve a target path for allowlist comparison, canonicalizing to defeat
/// `..` traversal attacks (BC-2.02.011 EC-001 / invariant 6).
///
/// Because `write_file` creates files that don't yet exist, `canonicalize()`
/// cannot be called on the full path. Instead: walk up the ancestor chain
/// until we find a directory that exists on disk, canonicalize it, then
/// rejoin the non-existent tail. This handles both the common case (only the
/// target file doesn't exist yet) and deeper cases (parent subdir also doesn't
/// exist). If no ancestor exists (or the path has no parent), return `None`.
fn resolve_path_for_allowlist(target: &Path) -> Option<PathBuf> {
    if let Ok(canon) = target.canonicalize() {
        return Some(canon);
    }
    // Collect the non-existent tail components bottom-up, then find the
    // deepest ancestor that can be canonicalized.
    let mut tail: Vec<std::ffi::OsString> = Vec::new();
    let mut cur = target.to_path_buf();
    loop {
        let filename = cur.file_name()?.to_os_string();
        tail.push(filename);
        let parent = cur.parent()?.to_path_buf();
        if let Ok(canon_parent) = parent.canonicalize() {
            // Rejoin all tail components in original order.
            let mut result = canon_parent;
            for component in tail.iter().rev() {
                result = result.join(component);
            }
            return Some(result);
        }
        cur = parent;
    }
}

/// Pure-core path-prefix check (BC-2.02.011 invariant 3; purity
/// classification: pure-core, no I/O side-effects beyond the canonicalize
/// syscall which is read-only).
///
/// Paths are canonicalized before the prefix comparison to defeat `..`
/// traversal attacks (BC-2.02.011 EC-001 / invariant 6).
pub(crate) fn path_allowed(resolved: &Path, allow: &[String], plugin_root: &Path) -> bool {
    // Canonicalize the target path to remove any `..` components.
    // If canonicalization fails (e.g. parent doesn't exist), deny.
    let canon_resolved = match resolve_path_for_allowlist(resolved) {
        Some(p) => p,
        None => return false,
    };

    for pref in allow {
        let pref_path = if Path::new(pref).is_absolute() {
            PathBuf::from(pref)
        } else {
            plugin_root.join(pref)
        };
        // Canonicalize the allowlist prefix as well so both sides are
        // in the same canonical form.
        let canon_pref = match pref_path.canonicalize() {
            Ok(p) => p,
            // If the configured allowlist prefix doesn't exist on disk,
            // skip it — it can never match a real resolved path.
            Err(_) => continue,
        };
        if canon_resolved.starts_with(&canon_pref) {
            return true;
        }
    }
    false
}

fn emit_denial(ctx: &HostContext, requested: &str, reason: &str, resolved: Option<&Path>) {
    let mut details = Map::new();
    details.insert("path".to_string(), Value::String(requested.to_string()));
    if let Some(r) = resolved {
        details.insert(
            "resolved".to_string(),
            Value::String(r.to_string_lossy().into_owned()),
        );
    }
    let ev = ctx.denial_event("write_file", reason, details);
    ctx.emit_internal(ev);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::test_support::*;

    #[test]
    fn denies_when_no_capability_block() {
        let ctx = bare_context();
        let err = prepare(&ctx, "out.txt", b"data", 1024).unwrap_err();
        assert_eq!(err, codes::CAPABILITY_DENIED);
    }

    #[test]
    fn writes_allowed_file() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("ok.txt");
        let mut ctx = context_with_caps(allow_write(&[dir.path().to_str().unwrap()]));
        ctx.plugin_root = dir.path().to_path_buf();
        prepare(&ctx, file.to_str().unwrap(), b"hello", 1024).unwrap();
        assert_eq!(std::fs::read(&file).unwrap(), b"hello");
    }

    #[test]
    fn rejects_path_outside_allow_list() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("ok.txt");
        let ctx = context_with_caps(allow_write(&["/nowhere/that/exists"]));
        let err = prepare(&ctx, file.to_str().unwrap(), b"x", 1024).unwrap_err();
        assert_eq!(err, codes::CAPABILITY_DENIED);
    }

    #[test]
    fn rejects_content_exceeding_max_bytes() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("big.txt");
        let mut ctx = context_with_caps(allow_write(&[dir.path().to_str().unwrap()]));
        ctx.plugin_root = dir.path().to_path_buf();
        let data = vec![0u8; 2048];
        let err = prepare(&ctx, file.to_str().unwrap(), &data, 512).unwrap_err();
        assert_eq!(err, codes::OUTPUT_TOO_LARGE);
        // BC-2.02.011 postcondition 2: no bytes written to disk.
        assert!(!file.exists());
    }

    #[test]
    fn writes_empty_contents_creates_file() {
        // BC-2.02.011 EC-005: empty slice → file created/truncated to zero bytes.
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("empty.txt");
        let mut ctx = context_with_caps(allow_write(&[dir.path().to_str().unwrap()]));
        ctx.plugin_root = dir.path().to_path_buf();
        prepare(&ctx, file.to_str().unwrap(), b"", 1024).unwrap();
        assert_eq!(std::fs::read(&file).unwrap(), b"");
    }

    #[test]
    fn rejects_missing_parent_directory() {
        // BC-2.02.011 EC-006 / postcondition 5.
        let dir = tempfile::tempdir().unwrap();
        let no_parent = dir.path().join("nonexistent-subdir/out.txt");
        let mut ctx =
            context_with_caps(allow_write(&[dir.path().to_str().unwrap()]));
        ctx.plugin_root = dir.path().to_path_buf();
        let err = prepare(
            &ctx,
            no_parent.to_str().unwrap(),
            b"x",
            1024,
        )
        .unwrap_err();
        assert_eq!(err, codes::INTERNAL_ERROR);
    }
}
