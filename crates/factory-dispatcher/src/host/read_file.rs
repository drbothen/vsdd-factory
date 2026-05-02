//! `read_file` host function.
//!
//! Resolves the requested path, verifies it's under one of the
//! registry-declared `path_allow` prefixes, opens with a hard
//! `max_bytes` cap, and writes the owned buffer back into guest memory
//! via `out_ptr_out` / `out_len_out` out-params.
//!
//! Capability model: deny-by-default. If the plugin has no
//! `Capabilities::read_file` block, every call is denied. Paths that
//! escape the allow-list (e.g. via `..` traversal) are denied and
//! emit `internal.capability_denied`.

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use serde_json::{Map, Value};
use wasmtime::Linker;

use super::memory::{read_wasm_string, write_wasm_bytes, write_wasm_u32};
use super::{HostCallError, HostCaller, HostContext, codes};

pub fn register(linker: &mut Linker<HostContext>) -> Result<(), HostCallError> {
    linker
        .func_wrap(
            "vsdd",
            "read_file",
            |mut caller: HostCaller<'_>,
             path_ptr: u32,
             path_len: u32,
             max_bytes: u32,
             timeout_ms: u32,
             out_ptr_out: u32,
             out_len_out: u32|
             -> i32 {
                let _ = timeout_ms; // accepted for ABI stability; enforced in S-1.5 via epoch interruption
                let path = match read_wasm_string(&mut caller, path_ptr, path_len) {
                    Ok(s) => s,
                    Err(_) => return codes::INVALID_ARGUMENT,
                };
                let (body, out_ptr) = {
                    let ctx = caller.data();
                    match prepare(ctx, &path, max_bytes) {
                        Ok(pair) => pair,
                        Err(code) => return code,
                    }
                };

                // Write the owned bytes into a fresh allocation inside
                // guest memory via the `out_ptr_out` / `out_len_out`
                // sentinels. For this story we take the simple path of
                // writing the bytes directly at a caller-provided
                // address (see HOST_ABI.md); the SDK wrapper supplies
                // `out_ptr` from a pre-allocated buffer.
                if write_wasm_u32(&mut caller, out_ptr_out, out_ptr).is_err() {
                    return codes::INVALID_ARGUMENT;
                }
                if write_wasm_u32(&mut caller, out_len_out, body.len() as u32).is_err() {
                    return codes::INVALID_ARGUMENT;
                }
                match write_wasm_bytes(&mut caller, out_ptr, body.len() as u32, &body) {
                    Ok(_) => codes::OK,
                    Err(_) => codes::INVALID_ARGUMENT,
                }
            },
        )
        .map_err(|e| HostCallError::Linker(e.to_string()))?;
    Ok(())
}

/// All of read_file's host-side logic that doesn't touch guest memory,
/// split out so it's unit-testable without a live wasm instance.
pub(crate) fn prepare(
    ctx: &HostContext,
    path: &str,
    max_bytes: u32,
) -> Result<(Vec<u8>, u32), i32> {
    let caps = ctx.capabilities.read_file.as_ref().ok_or_else(|| {
        emit_denial(ctx, path, "no_read_file_capability", None);
        codes::CAPABILITY_DENIED
    })?;

    // Relative paths are resolved under `ctx.cwd` (the project root,
    // i.e. `$CLAUDE_PROJECT_DIR`) so that project-relative files like
    // `.factory/wave-state.yaml` and `.claude/settings.local.json` are
    // found in the project directory, not the plugin directory.
    let resolved = resolve_for_read(Path::new(path), &ctx.cwd);

    if !path_allowed(&resolved, &caps.path_allow, &ctx.cwd) {
        emit_denial(ctx, path, "path_not_allowed", Some(&resolved));
        return Err(codes::CAPABILITY_DENIED);
    }

    match read_bounded(&resolved, max_bytes as usize) {
        Ok(bytes) => Ok((bytes, 0)),
        Err(ReadErr::TooLarge) => {
            emit_denial(ctx, path, "output_too_large", Some(&resolved));
            Err(codes::OUTPUT_TOO_LARGE)
        }
        Err(ReadErr::Other) => Err(codes::INTERNAL_ERROR),
    }
}

enum ReadErr {
    TooLarge,
    Other,
}

/// Resolve a path for reading. Relative paths are resolved under `base`
/// (the project working directory, `$CLAUDE_PROJECT_DIR`). Absolute paths
/// are used as-is.
fn resolve_for_read(path: &Path, base: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        base.join(path)
    }
}

/// Check whether a resolved path is within the allow-list. Allow-list entries
/// that are relative are expanded under `base` (the project working directory).
fn path_allowed(resolved: &Path, allow: &[String], base: &Path) -> bool {
    // Canonicalize the target path to remove any `..` components, defeating
    // traversal attacks (BC-2.02.001 EC-001 / sibling-consistency with BC-2.02.011).
    // For read_file the file must already exist, so full canonicalize() works.
    let canon_resolved = match resolved.canonicalize() {
        Ok(p) => p,
        // File doesn't exist or I/O error — deny (will produce INTERNAL_ERROR
        // downstream when read_bounded opens the file).
        Err(_) => return false,
    };

    for pref in allow {
        let pref_path = if Path::new(pref).is_absolute() {
            PathBuf::from(pref)
        } else {
            base.join(pref)
        };
        let canon_pref = match pref_path.canonicalize() {
            Ok(p) => p,
            // Configured allowlist prefix doesn't exist — skip.
            Err(_) => continue,
        };
        if canon_resolved.starts_with(&canon_pref) {
            return true;
        }
    }
    false
}

fn read_bounded(path: &Path, max_bytes: usize) -> Result<Vec<u8>, ReadErr> {
    let mut file = File::open(path).map_err(|_| ReadErr::Other)?;
    let metadata = file.metadata().map_err(|_| ReadErr::Other)?;
    if metadata.len() as usize > max_bytes {
        return Err(ReadErr::TooLarge);
    }
    let mut buf = Vec::with_capacity(metadata.len() as usize);
    file.read_to_end(&mut buf).map_err(|_| ReadErr::Other)?;
    if buf.len() > max_bytes {
        return Err(ReadErr::TooLarge);
    }
    Ok(buf)
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
    let ev = ctx.denial_event("read_file", reason, details);
    ctx.emit_internal(ev);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::test_support::*;
    use std::io::Write;

    #[test]
    fn denies_when_no_capability_block() {
        let ctx = bare_context();
        let err = prepare(&ctx, "foo.txt", 1024).unwrap_err();
        assert_eq!(err, codes::CAPABILITY_DENIED);
    }

    #[test]
    fn reads_allowed_file() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("ok.txt");
        std::fs::write(&file, b"hello world").unwrap();
        let mut ctx = context_with_caps(allow_read(&[dir.path().to_str().unwrap()]));
        // Absolute path in allow-list; cwd doesn't affect resolution.
        ctx.cwd = dir.path().to_path_buf();
        let (bytes, _) = prepare(&ctx, file.to_str().unwrap(), 1024).unwrap();
        assert_eq!(bytes, b"hello world");
    }

    #[test]
    fn rejects_path_outside_allow_list() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("ok.txt");
        std::fs::write(&file, b"x").unwrap();
        let ctx = context_with_caps(allow_read(&["/nowhere/that/exists"]));
        let err = prepare(&ctx, file.to_str().unwrap(), 1024).unwrap_err();
        assert_eq!(err, codes::CAPABILITY_DENIED);
    }

    #[test]
    fn rejects_file_exceeding_max_bytes() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("big.txt");
        let mut f = std::fs::File::create(&file).unwrap();
        f.write_all(&vec![0u8; 2048]).unwrap();
        let mut ctx = context_with_caps(allow_read(&[dir.path().to_str().unwrap()]));
        ctx.cwd = dir.path().to_path_buf();
        let err = prepare(&ctx, file.to_str().unwrap(), 512).unwrap_err();
        assert_eq!(err, codes::OUTPUT_TOO_LARGE);
    }

    #[test]
    fn relative_path_resolves_under_cwd() {
        // Relative paths (e.g. ".factory/wave-state.yaml") are resolved
        // under ctx.cwd ($CLAUDE_PROJECT_DIR), not plugin_root.
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("rel.txt"), b"yes").unwrap();
        let mut ctx = context_with_caps(allow_read(&["."]));
        ctx.cwd = dir.path().to_path_buf();
        let (bytes, _) = prepare(&ctx, "rel.txt", 1024).unwrap();
        assert_eq!(bytes, b"yes");
    }
}
