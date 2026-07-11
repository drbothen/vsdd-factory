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

use crate::internal_log::InternalEvent;
use super::memory::{read_wasm_string, write_wasm_bytes, write_wasm_u32};
use super::path_util::resolve_path_for_allowlist;
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

    // Two-step decomposed path check (architect Ruling-2 / S-19.03):
    //   Step 1 — resolve via ancestor-walk+rejoin (handles absent files correctly).
    //   Step 2 — pure prefix check against the allow-list.
    // The two denial reasons are emitted separately so operators can distinguish
    // filesystem resolution errors from genuine allowlist violations.
    match check_path_allowed(&resolved, &caps.path_allow, &ctx.cwd) {
        PathAllowDecision::Allowed => {},
        PathAllowDecision::DeniedResolutionFailed => {
            emit_denial(ctx, path, "path_resolution_failed", Some(&resolved));
            return Err(codes::CAPABILITY_DENIED);
        }
        PathAllowDecision::DeniedNotAllowed => {
            emit_denial(ctx, path, "path_not_allowed", Some(&resolved));
            return Err(codes::CAPABILITY_DENIED);
        }
    }

    match read_bounded(&resolved, max_bytes as usize) {
        Ok(bytes) => Ok((bytes, 0)),
        Err(ReadErr::TooLarge) => {
            emit_denial(ctx, path, "output_too_large", Some(&resolved));
            Err(codes::OUTPUT_TOO_LARGE)
        }
        Err(ReadErr::NotFound) => {
            // AC-002 (S-19.03): path is allowlisted but file is absent.
            // Emit `internal.file_not_found` (NOT `internal.capability_denied`)
            // and return `codes::NOT_FOUND (-5)` so plugins can distinguish
            // "absent file" from "genuine allowlist violation".
            let ev = InternalEvent::now("internal.file_not_found")
                .with_trace_id(&ctx.dispatcher_trace_id)
                .with_session_id(&ctx.session_id)
                .with_plugin_name(&ctx.plugin_name)
                .with_plugin_version(&ctx.plugin_version)
                .with_field("function", Value::String("read_file".to_string()))
                .with_field("reason", Value::String("file_not_found".to_string()))
                .with_field("path", Value::String(path.to_string()))
                .with_field(
                    "resolved",
                    Value::String(resolved.to_string_lossy().into_owned()),
                );
            ctx.emit_internal(ev);
            Err(codes::NOT_FOUND)
        }
        Err(ReadErr::Other) => Err(codes::INTERNAL_ERROR),
    }
}

/// Result of the two-step path allowlist check (architect Ruling-2).
enum PathAllowDecision {
    /// Path resolved and lies within an allowed prefix.
    Allowed,
    /// Ancestor-walk failed to canonicalize any ancestor — filesystem/traversal error.
    /// Caller emits `internal.capability_denied reason=path_resolution_failed`.
    DeniedResolutionFailed,
    /// Path resolved successfully but lies outside all allowed prefixes.
    /// Caller emits `internal.capability_denied reason=path_not_allowed`.
    DeniedNotAllowed,
}

enum ReadErr {
    TooLarge,
    /// Path is in the allow-list but the file does not exist.
    /// Triggers `internal.file_not_found` + `codes::NOT_FOUND` (AC-002).
    NotFound,
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

/// Two-step allowlist check (architect Ruling-2 / S-19.03 AC-001).
///
/// Step 1: resolve via ancestor-walk+rejoin (handles absent files correctly,
///   unlike `Path::canonicalize()` which returns Err for non-existent files).
///   Returns `DeniedResolutionFailed` when even the root ancestor fails —
///   structurally impossible on real Unix filesystems, but testable via the
///   injectable mock in path_util tests (BC-2.07.001 EC-007).
///
/// Step 2: pure `starts_with` prefix check against each allow-list entry.
///   Allow-list entries that are relative are expanded under `base`.
///   Returns `DeniedNotAllowed` when the resolved path lies outside all prefixes.
///
/// Separating resolution failure from allowlist failure lets operators distinguish
/// filesystem errors from genuine access-policy violations in telemetry.
fn check_path_allowed(resolved: &Path, allow: &[String], base: &Path) -> PathAllowDecision {
    // Step 1: resolve with ancestor-walk+rejoin so absent-but-allowlisted files
    // get a synthesized canonical path instead of an opaque resolution failure.
    let canon_resolved = match resolve_path_for_allowlist(resolved, |p| p.canonicalize()) {
        Some(p) => p,
        None => return PathAllowDecision::DeniedResolutionFailed,
    };

    // Step 2: prefix check. Allow-list entries canonicalize normally (they must
    // exist for the check to succeed; absent allow-list prefixes are skipped).
    for pref in allow {
        let pref_path = if Path::new(pref).is_absolute() {
            PathBuf::from(pref)
        } else {
            base.join(pref)
        };
        let canon_pref = match pref_path.canonicalize() {
            Ok(p) => p,
            Err(_) => continue, // configured prefix doesn't exist — skip
        };
        if canon_resolved.starts_with(&canon_pref) {
            return PathAllowDecision::Allowed;
        }
    }
    PathAllowDecision::DeniedNotAllowed
}

fn read_bounded(path: &Path, max_bytes: usize) -> Result<Vec<u8>, ReadErr> {
    let mut file = File::open(path).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            ReadErr::NotFound
        } else {
            ReadErr::Other
        }
    })?;
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

    // -----------------------------------------------------------------------
    // S-19.03 Red Gate tests (T-001..T-004 + NC-B)
    //
    // These tests ALL fail or panic at Red Gate (stubs not implemented):
    //   T-001 — FAILS: returns CAPABILITY_DENIED (old path_allowed) not NOT_FOUND
    //   T-002 NC-A — PANICS: path_util::resolve_path_for_allowlist is todo!()
    //   T-003 — FAILS: emits capability_denied event, not file_not_found
    //   T-004 — FAILS: emits one capability_denied event (not zero)
    //   T-001 NC-B — PANICS: path_util::resolve_path_for_allowlist is todo!()
    // -----------------------------------------------------------------------

    /// test_S19_03_T001_absent_allowlisted_file_returns_NOT_FOUND
    ///
    /// T-001 (AC-001): when a file's path is within the allowlist but the file does
    /// not yet exist, `prepare()` must return `Err(codes::NOT_FOUND)` — NOT
    /// `Err(codes::CAPABILITY_DENIED)`.
    ///
    /// Root defect: old `path_allowed()` calls `canonicalize()` which fails for absent
    /// files and returns `false`, causing `prepare()` to return CAPABILITY_DENIED even
    /// when the path IS within the declared `path_allow` prefix.
    ///
    /// Red Gate: FAILS — `codes::NOT_FOUND` is currently -1000 (stub) and the
    /// old `path_allowed()` returns `false` for absent files, so `prepare()` returns
    /// `Err(CAPABILITY_DENIED = -1)`, not `Err(NOT_FOUND = -1000)`.
    ///
    /// Traces to: BC-2.07.001 part b+c; S-19.03 AC-001/AC-002; VP-098.
    #[test]
    #[allow(non_snake_case)]
    fn test_S19_03_T001_absent_allowlisted_file_returns_NOT_FOUND() {
        let dir = tempfile::tempdir().unwrap();
        let factory_dir = dir.path().join(".factory");
        std::fs::create_dir_all(&factory_dir).unwrap();
        // Allow the .factory/ directory; wave-state.yaml does NOT exist.
        let mut ctx = context_with_caps(allow_read(&[factory_dir.to_str().unwrap()]));
        ctx.cwd = dir.path().to_path_buf();
        let absent_path = factory_dir.join("wave-state.yaml");
        assert!(!absent_path.exists(), "test setup: target must not exist");

        let result = prepare(&ctx, absent_path.to_str().unwrap(), 65536);
        assert_eq!(
            result.unwrap_err(),
            codes::NOT_FOUND,
            "T-001 AC-001: absent file within allowlist must return NOT_FOUND (-5), \
             not CAPABILITY_DENIED (-1). Red Gate: currently returns CAPABILITY_DENIED \
             because old path_allowed() uses canonicalize() which fails for absent files."
        );
    }

    /// test_S19_03_T002_NC_A_path_util_callable_from_read_file_context
    ///
    /// T-002 Negative Control A (AC-001): `path_util::resolve_path_for_allowlist` is
    /// callable from the `read_file` module context. For an EXISTING path, it must
    /// return `Some(canonical_path)`. The allowlist `starts_with` check is done in
    /// `path_allowed`, not in `resolve_path_for_allowlist` itself.
    ///
    /// This test verifies the shared module is importable and the function callable —
    /// a prerequisite for the new `path_allowed()` implementation.
    ///
    /// Red Gate: PANICS — `resolve_path_for_allowlist` is `todo!()`.
    ///
    /// Traces to: BC-2.07.001 part b; S-19.03 AC-001 negative-control A (shared module
    /// extraction prerequisite).
    #[test]
    #[allow(non_snake_case)]
    fn test_S19_03_T002_NC_A_path_util_callable_from_read_file_context() {
        let dir = tempfile::tempdir().unwrap();
        let outside_file = dir.path().join("outside.txt");
        std::fs::write(&outside_file, b"x").unwrap();

        // Call shared path_util function from read_file module context.
        // Red Gate: panics (todo!()) — that IS the evidence this is not yet implemented.
        let result =
            crate::host::path_util::resolve_path_for_allowlist(&outside_file, |p| p.canonicalize());
        assert!(
            result.is_some(),
            "T-002 NC-A: existing path must resolve to Some(canonical_path); \
             allowlist check is done separately via starts_with in path_allowed."
        );
    }

    /// test_S19_03_T003_absent_allowlisted_file_emits_file_not_found_event
    ///
    /// T-003 (AC-002): when a file's path is allowlisted but the file is absent,
    /// `prepare()` must emit `internal.file_not_found` (NOT `internal.capability_denied`).
    ///
    /// Red Gate: FAILS — old `path_allowed()` returns `false` for absent files,
    /// causing `prepare()` to emit `internal.capability_denied reason=path_not_allowed`
    /// instead of `internal.file_not_found`.
    ///
    /// Traces to: BC-2.07.001 part c; S-19.03 AC-002; VP-098.
    #[test]
    #[allow(non_snake_case)]
    fn test_S19_03_T003_absent_allowlisted_file_emits_file_not_found_event() {
        let dir = tempfile::tempdir().unwrap();
        let factory_dir = dir.path().join(".factory");
        std::fs::create_dir_all(&factory_dir).unwrap();
        let mut ctx = context_with_caps(allow_read(&[factory_dir.to_str().unwrap()]));
        ctx.cwd = dir.path().to_path_buf();
        let absent_path = factory_dir.join("wave-state.yaml");

        let _ = prepare(&ctx, absent_path.to_str().unwrap(), 65536);
        let events = ctx.drain_events();

        let file_not_found_count = events.iter().filter(|e| e.type_ == "internal.file_not_found").count();
        assert_eq!(
            file_not_found_count,
            1,
            "T-003 AC-002: absent allowlisted file must emit exactly one 'internal.file_not_found' \
             event; got events with types: {:?}. Red Gate: currently emits capability_denied.",
            events.iter().map(|e| &e.type_).collect::<Vec<_>>()
        );
    }

    /// test_S19_03_T004_absent_allowlisted_file_zero_capability_denied_events
    ///
    /// T-004 (AC-002): when a file's path is allowlisted but the file does not exist,
    /// `prepare()` must emit ZERO `internal.capability_denied` events for
    /// `plugin_name=warn-pending-wave-gate`.
    ///
    /// Red Gate: FAILS — old code emits one `capability_denied reason=path_not_allowed`
    /// event (the original defect: `canonicalize()` failure masquerades as an allowlist
    /// violation, traced in rc.22 smoke FINDING-2, dispatcher trace bc687a0f).
    ///
    /// Traces to: BC-2.07.001 part c; S-19.03 AC-002 zero-false-positive; VP-098.
    #[test]
    #[allow(non_snake_case)]
    fn test_S19_03_T004_absent_allowlisted_file_zero_capability_denied_events() {
        let dir = tempfile::tempdir().unwrap();
        let factory_dir = dir.path().join(".factory");
        std::fs::create_dir_all(&factory_dir).unwrap();
        let mut ctx = context_with_caps(allow_read(&[factory_dir.to_str().unwrap()]));
        ctx.plugin_name = "warn-pending-wave-gate".to_string();
        ctx.cwd = dir.path().to_path_buf();
        let absent_path = factory_dir.join("wave-state.yaml");

        let _ = prepare(&ctx, absent_path.to_str().unwrap(), 65536);
        let events = ctx.drain_events();

        let cap_denied: Vec<_> = events
            .iter()
            .filter(|e| e.type_ == "internal.capability_denied")
            .collect();
        assert_eq!(
            cap_denied.len(),
            0,
            "T-004 AC-002: absent allowlisted file must emit ZERO 'internal.capability_denied' \
             events for plugin_name=warn-pending-wave-gate; got: {:?}. \
             Red Gate: currently emits one capability_denied with reason=path_not_allowed.",
            cap_denied
        );
    }

    /// test_S19_03_T001_NC_B_path_resolution_failed_token_via_path_util
    ///
    /// T-001 Negative Control B (AC-001, BC-2.07.001 EC-007): when `resolve_path_for_allowlist`
    /// returns `None` (injected mock returns Err for ALL ancestors), the dispatcher MUST
    /// emit `internal.capability_denied` with `reason=path_resolution_failed` — NOT
    /// `reason=path_not_allowed`. The two reason tokens are semantically distinct:
    ///   - `path_resolution_failed`: filesystem resolution error (traversal-defense exhausted)
    ///   - `path_not_allowed`: path resolves fine but is outside all allowed prefixes
    ///
    /// This test calls `path_util::resolve_path_for_allowlist` directly with the mock
    /// to verify the injectable parameter works (prerequisite for read_file integration).
    ///
    /// Red Gate: PANICS — `resolve_path_for_allowlist` is `todo!()`. Panic IS the Red Gate.
    ///
    /// Traces to: BC-2.07.001 EC-007; S-19.03 AC-001 negative-control B.
    #[test]
    #[allow(non_snake_case)]
    fn test_S19_03_T001_NC_B_path_resolution_failed_token_via_path_util() {
        let target = std::path::Path::new(".factory/wave-state.yaml");
        // Mock: always fails — simulates EC-007 (all ancestors fail canonicalization).
        let result = crate::host::path_util::resolve_path_for_allowlist(target, |_p| {
            Err(std::io::Error::from(std::io::ErrorKind::NotFound))
        });
        // After implementation: None → caller emits path_resolution_failed.
        assert!(
            result.is_none(),
            "T-001 NC-B: mock-canonicalize-all-fail must return None; \
             the calling path_allowed must then emit reason=path_resolution_failed \
             (NOT path_not_allowed). Red Gate: panics (todo!())."
        );
    }
}
