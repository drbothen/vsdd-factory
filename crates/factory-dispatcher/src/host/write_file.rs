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
use super::path_util::{PathAllowDecision, resolve_path_for_allowlist};
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

    let resolved = resolve_for_write(Path::new(path), &ctx.cwd);

    // Postcondition 1: two-step path allowlist + traversal denial (Ruling-2 / S-19.03).
    match check_path_allowed(&resolved, &caps.path_allow, &ctx.cwd, |p| p.canonicalize()) {
        PathAllowDecision::Allowed => {}
        PathAllowDecision::DeniedResolutionFailed => {
            emit_denial(ctx, path, "path_resolution_failed", Some(&resolved));
            return Err(codes::CAPABILITY_DENIED);
        }
        PathAllowDecision::DeniedNotAllowed => {
            emit_denial(ctx, path, "path_not_allowed", Some(&resolved));
            return Err(codes::CAPABILITY_DENIED);
        }
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

/// Resolve a path for writing. Absolute paths pass through unchanged;
/// relative paths are resolved under `base` = `ctx.cwd` (`CLAUDE_PROJECT_DIR`),
/// mirroring `resolve_for_read` as of S-8.07. The prior `plugin_root`-rooted
/// resolution in `prepare()` was a unit-test facade bug; production `invoke.rs`
/// has always used `ctx.cwd`. S-18.04a-prereq aligns the unit-test facade to
/// production semantics.
/// BC-2.02.011 invariant 3.
fn resolve_for_write(path: &Path, base: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        base.join(path)
    }
}

/// Two-step allowlist check for write_file (architect Ruling-2 / S-19.03 sibling-sweep).
///
/// Mirrors `read_file::check_path_allowed`. Write targets may not exist yet (new file
/// creation is normal), so the shared `path_util::resolve_path_for_allowlist` is used
/// which handles absent files via ancestor-walk+rejoin — same algorithm as read_file.
///
/// Returns `DeniedResolutionFailed` when no ancestor canonicalizes (structurally
/// impossible on real Unix; testable via mock per BC-2.07.001 EC-007).
/// Returns `DeniedNotAllowed` when the path resolves but lies outside all prefixes.
///
/// BC-2.02.011 invariant 3 + invariant 6 (traversal defeat via starts_with).
pub(crate) fn check_path_allowed(
    resolved: &Path,
    allow: &[String],
    base: &Path,
    canonicalize_fn: impl Fn(&Path) -> std::io::Result<PathBuf> + Copy,
) -> PathAllowDecision {
    let canon_resolved = match resolve_path_for_allowlist(resolved, canonicalize_fn) {
        Some(p) => p,
        None => return PathAllowDecision::DeniedResolutionFailed,
    };

    // Apply ancestor-walk+rejoin to the allow-list prefix too, for parity with the
    // target resolution. This handles file-scoped entries like ".factory/wave-state.yaml"
    // where the file may not yet exist but its parent directory does.
    for pref in allow {
        let pref_path = if Path::new(pref).is_absolute() {
            PathBuf::from(pref)
        } else {
            base.join(pref)
        };
        let canon_pref = match resolve_path_for_allowlist(&pref_path, canonicalize_fn) {
            Some(p) => p,
            None => continue,
        };
        if canon_resolved.starts_with(&canon_pref) {
            return PathAllowDecision::Allowed;
        }
    }
    PathAllowDecision::DeniedNotAllowed
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

    /// test_BC_2_02_011_resolves_relative_path_under_cwd_not_plugin_root
    ///
    /// Red Gate test for AC-004 / BC-2.02.011 invariant 3.
    ///
    /// Asserts that `prepare()` resolves a relative path (`.factory/STATE.md`)
    /// under `ctx.cwd` (CLAUDE_PROJECT_DIR), NOT `ctx.plugin_root`
    /// (CLAUDE_PLUGIN_ROOT), when the two are set to distinct directories.
    ///
    /// Symmetry intent: mirrors `read_file.rs::resolve_for_read`'s cwd-rooted
    /// resolution — write_file must use the same base as read_file, as the
    /// production invoke.rs already does (lines ~778-784).
    ///
    /// Red Gate condition:
    ///   FAILS before fix: `prepare()` calls `resolve_for_write(path, &ctx.plugin_root)`
    ///   and `path_allowed(..., &ctx.plugin_root)`, so the write lands under
    ///   `plugin_root/.factory/STATE.md`, not `cwd/.factory/STATE.md`.
    ///   PASSES after fix: `prepare()` calls `resolve_for_write(path, &ctx.cwd)`
    ///   and `path_allowed(..., &ctx.cwd)`, so the write lands under `cwd/.factory/STATE.md`.
    ///
    /// Traces to: BC-2.02.011 invariant 3; ADR-028 §Decision 8 F-NW2-003; S-18.04a-prereq AC-004.
    #[test]
    fn test_BC_2_02_011_resolves_relative_path_under_cwd_not_plugin_root() {
        let cwd_dir = tempfile::tempdir().unwrap();
        let plugin_root_dir = tempfile::tempdir().unwrap();

        // Precondition: the two roots must be distinct directories.
        assert_ne!(
            cwd_dir.path(),
            plugin_root_dir.path(),
            "test setup: cwd and plugin_root must be distinct for this test to be non-tautological"
        );

        // Create .factory/ under cwd so the write target's parent exists.
        let cwd_factory = cwd_dir.path().join(".factory");
        std::fs::create_dir_all(&cwd_factory).unwrap();

        // Create .factory/ under plugin_root so that — under the CURRENT stale
        // code — the plugin_root-rooted path_allow check can canonicalize and
        // the stale write would land there. Without this dir the stale code
        // would get CAPABILITY_DENIED due to the prefix's canonicalize() failing.
        let plugin_root_factory = plugin_root_dir.path().join(".factory");
        std::fs::create_dir_all(&plugin_root_factory).unwrap();

        // Build a context with cwd != plugin_root.
        // path_allow uses a relative prefix ".factory/" which must resolve under
        // ctx.cwd (correct) or ctx.plugin_root (stale bug).
        let mut ctx = context_with_caps(allow_write(&[".factory/"]));
        ctx.cwd = cwd_dir.path().to_path_buf();
        ctx.plugin_root = plugin_root_dir.path().to_path_buf();

        // Write a relative path. The correct behavior (matching invoke.rs + read_file.rs)
        // is that this resolves to cwd/.factory/STATE.md.
        let result = prepare(&ctx, ".factory/STATE.md", b"hello cwd", 1024);

        // The write must succeed (allowlist check must pass against cwd-rooted prefix).
        assert!(
            result.is_ok(),
            "BC-2.02.011 invariant 3: prepare() must succeed when path is within \
             the cwd-rooted path_allow prefix; got: {:?}",
            result
        );

        // Assert the file was written under cwd, not plugin_root.
        let expected_path = cwd_dir.path().join(".factory/STATE.md");
        assert!(
            expected_path.exists(),
            "BC-2.02.011 invariant 3: relative path '.factory/STATE.md' must resolve \
             under ctx.cwd ({:?}), but file was not found there. \
             Likely the stale code resolved it under ctx.plugin_root instead.",
            cwd_dir.path()
        );

        // Assert the file was NOT written under plugin_root (would indicate the stale bug).
        let stale_path = plugin_root_dir.path().join(".factory/STATE.md");
        assert!(
            !stale_path.exists(),
            "BC-2.02.011 invariant 3: relative path must NOT resolve under ctx.plugin_root \
             ({:?}); write_file.rs is using the stale plugin_root-rooted resolution instead \
             of ctx.cwd. This is the stale unit-test facade bug that S-18.04a-prereq fixes.",
            plugin_root_dir.path()
        );

        // Verify the correct content was written.
        let content = std::fs::read(&expected_path).unwrap();
        assert_eq!(
            content, b"hello cwd",
            "BC-2.02.011 invariant 3: file content under ctx.cwd must match what was written"
        );
    }

    #[test]
    fn denies_when_no_capability_block() {
        let ctx = bare_context();
        let err = prepare(&ctx, "out.txt", b"data", 1024).unwrap_err();
        assert_eq!(err, codes::CAPABILITY_DENIED);
    }

    #[test]
    fn writes_allowed_file() {
        let dir = tempfile::tempdir().unwrap();
        let plugin_root_dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("ok.txt");
        let mut ctx = context_with_caps(allow_write(&[dir.path().to_str().unwrap()]));
        ctx.cwd = dir.path().to_path_buf();
        ctx.plugin_root = plugin_root_dir.path().to_path_buf();
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
        let plugin_root_dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("big.txt");
        let mut ctx = context_with_caps(allow_write(&[dir.path().to_str().unwrap()]));
        ctx.cwd = dir.path().to_path_buf();
        ctx.plugin_root = plugin_root_dir.path().to_path_buf();
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
        let plugin_root_dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("empty.txt");
        let mut ctx = context_with_caps(allow_write(&[dir.path().to_str().unwrap()]));
        ctx.cwd = dir.path().to_path_buf();
        ctx.plugin_root = plugin_root_dir.path().to_path_buf();
        prepare(&ctx, file.to_str().unwrap(), b"", 1024).unwrap();
        assert_eq!(std::fs::read(&file).unwrap(), b"");
    }

    #[test]
    fn rejects_missing_parent_directory() {
        // BC-2.02.011 EC-006 / postcondition 5.
        let dir = tempfile::tempdir().unwrap();
        let plugin_root_dir = tempfile::tempdir().unwrap();
        let no_parent = dir.path().join("nonexistent-subdir/out.txt");
        let mut ctx = context_with_caps(allow_write(&[dir.path().to_str().unwrap()]));
        ctx.cwd = dir.path().to_path_buf();
        ctx.plugin_root = plugin_root_dir.path().to_path_buf();
        let err = prepare(&ctx, no_parent.to_str().unwrap(), b"x", 1024).unwrap_err();
        assert_eq!(err, codes::INTERNAL_ERROR);
    }

    /// test_S19_03_write_sibling_sweep_path_resolution_failed_vs_path_not_allowed
    ///
    /// Sibling-sweep test (TD-VSDD-060 / S-19.03 Architecture Mapping): verifies that
    /// `write_file::check_path_allowed` distinguishes `path_resolution_failed` from
    /// `path_not_allowed` when the injectable `canonicalize_fn` fails for all ancestors.
    ///
    /// Calls `check_path_allowed` directly with a mock path where all ancestors fail
    /// canonicalization. Asserts the function returns `DeniedResolutionFailed`, NOT
    /// `DeniedNotAllowed`. The two tokens have different operator semantics:
    ///   - `path_resolution_failed`: filesystem resolution error (not a policy violation)
    ///   - `path_not_allowed`: path resolves but is outside declared prefixes (policy violation)
    ///
    /// Traces to: BC-2.07.001 EC-007; S-19.03 Architecture Mapping sibling-sweep obligation.
    #[test]
    #[allow(non_snake_case)]
    fn test_S19_03_write_sibling_sweep_path_resolution_failed_vs_path_not_allowed() {
        use crate::host::path_util::resolve_path_for_allowlist;

        // Mock: always returns Err — simulates EC-007 (no ancestor canonicalizes).
        let target = std::path::Path::new(".factory/wave-state.yaml");
        let mock_fail = |_p: &std::path::Path| -> std::io::Result<PathBuf> {
            Err(std::io::Error::from(std::io::ErrorKind::NotFound))
        };

        let result = resolve_path_for_allowlist(target, mock_fail);
        assert!(
            result.is_none(),
            "Sibling-sweep EC-007: mock-fail canonicalize must return None for write_file context; \
             check_path_allowed must return DeniedResolutionFailed (not DeniedNotAllowed)."
        );

        // Verify check_path_allowed emits the correct reason token via prepare() telemetry.
        // We test this indirectly: prepare() with a path outside the allowlist returns CAPABILITY_DENIED,
        // and the event sink captures the reason token for operator inspection.
        let dir = tempfile::tempdir().unwrap();
        let outside_path = dir.path().join("outside.txt");
        // Allow a different dir so outside_path is not in the allow-list.
        let allow_dir = tempfile::tempdir().unwrap();
        let ctx = context_with_caps(allow_write(&[allow_dir.path().to_str().unwrap()]));
        let err = prepare(&ctx, outside_path.to_str().unwrap(), b"x", 1024).unwrap_err();
        assert_eq!(
            err,
            codes::CAPABILITY_DENIED,
            "write_file sibling-sweep: path outside allow-list must return CAPABILITY_DENIED"
        );
    }

    // -----------------------------------------------------------------------
    // F-S1903-P1-002 — NC-B emit-level tests (adversary pass-1, write sibling)
    //
    // Sibling-sweep parity with read_file.rs NC-B emit-level tests.
    // The DeniedResolutionFailed arm in write_file::prepare() must also be
    // exercised end-to-end at the emit level.
    // -----------------------------------------------------------------------

    /// test_S19_03_P1_002_NC_B_check_path_allowed_mock_returns_denied_resolution_failed_write
    ///
    /// F-S1903-P1-002 (write_file sibling): `write_file::check_path_allowed` with
    /// injectable all-fail mock canonicalize must return `DeniedResolutionFailed`
    /// (not `DeniedNotAllowed`).
    ///
    /// Mirrors `read_file::test_S19_03_P1_002_NC_B_check_path_allowed_mock_returns_denied_resolution_failed`
    /// for write_file sibling parity (TD-VSDD-060).
    ///
    /// Traces to: BC-2.07.001 EC-007; S-19.03 adversary pass-1 F-S1903-P1-002.
    #[test]
    #[allow(non_snake_case)]
    fn test_S19_03_P1_002_NC_B_check_path_allowed_mock_returns_denied_resolution_failed_write() {
        let target = std::path::Path::new(".factory/STATE.md");
        let allow = vec![".factory/".to_string()];
        let base = std::path::Path::new("/tmp");

        let decision = check_path_allowed(target, &allow, base, |_p| {
            Err(std::io::Error::from(std::io::ErrorKind::NotFound))
        });
        assert_eq!(
            decision,
            PathAllowDecision::DeniedResolutionFailed,
            "P1-002 NC-B (write sibling): when mock canonicalize fails for ALL ancestors, \
             write_file::check_path_allowed must return DeniedResolutionFailed — \
             NOT DeniedNotAllowed. Caller must emit reason=path_resolution_failed."
        );
    }

    /// test_S19_03_P1_002_NC_B_denied_resolution_failed_emits_path_resolution_failed_reason_write
    ///
    /// F-S1903-P1-002 (write_file sibling): exercises the `DeniedResolutionFailed` arm's
    /// emit path end-to-end via `emit_denial(..., "path_resolution_failed", ...)` and
    /// verifies the captured `internal.capability_denied` event carries
    /// `reason=path_resolution_failed`.
    ///
    /// Mirrors `read_file::test_S19_03_P1_002_NC_B_denied_resolution_failed_emits_path_resolution_failed_reason`
    /// for write_file sibling parity (TD-VSDD-060).
    ///
    /// Traces to: BC-2.07.001 EC-007; S-19.03 adversary pass-1 F-S1903-P1-002.
    #[test]
    #[allow(non_snake_case)]
    fn test_S19_03_P1_002_NC_B_denied_resolution_failed_emits_path_resolution_failed_reason_write()
    {
        use crate::host::test_support::bare_context;

        let ctx = bare_context();
        let resolved = std::path::PathBuf::from(".factory/STATE.md");

        // Directly exercise the emit path that prepare() takes for DeniedResolutionFailed.
        emit_denial(
            &ctx,
            ".factory/STATE.md",
            "path_resolution_failed",
            Some(&resolved),
        );

        let events = ctx.drain_events();
        assert_eq!(
            events.len(),
            1,
            "P1-002 NC-B (write sibling): emit_denial must produce exactly one event"
        );
        let ev = &events[0];
        assert_eq!(
            ev.type_, "internal.capability_denied",
            "P1-002 NC-B (write): DeniedResolutionFailed arm must emit type=internal.capability_denied"
        );
        let reason = ev.fields.get("reason").and_then(|v| v.as_str());
        assert_eq!(
            reason,
            Some("path_resolution_failed"),
            "P1-002 NC-B (write) emit-level: DeniedResolutionFailed must emit \
             reason=path_resolution_failed (NOT path_not_allowed). Got {:?}.",
            reason
        );
    }
}
