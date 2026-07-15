//! `read_prefix` host function — bounded partial read (head-c semantics).
//!
//! Returns at most `max_bytes` bytes from the start of a file (equivalent to
//! `head -c max_bytes`). This function is GUARANTEED never to return
//! OUTPUT_TOO_LARGE (-3); by construction `max_bytes` IS the output cap.
//!
//! Additive FFI entry point in the `vsdd` WASM import namespace. HOST_ABI_VERSION
//! remains 1. `read_file` all-or-nothing semantics are UNCHANGED (BC-1.17.001
//! Invariant 2).
//!
//! Capability model: deny-by-default. If the plugin has no
//! `Capabilities::read_prefix` block, every call is denied with CAPABILITY_DENIED
//! (-1). A plugin that has only `capabilities.read_file` also receives
//! CAPABILITY_DENIED — the two capabilities are independent (BC-1.17.001
//! Invariant 3, defense-in-depth).
//!
//! Path traversal defense is identical to `read_file`: uses
//! `resolve_path_for_allowlist` from `path_util.rs` (BC-1.17.001 Invariant 4).
//!
//! BC-1.17.001 v1.6 — Story S-19.06.

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use serde_json::{Map, Value};
use wasmtime::Linker;

use super::memory::{read_wasm_string, write_wasm_bytes, write_wasm_u32};
use super::path_util::{PathAllowDecision, check_path_allowed};
use super::{HostCallError, HostCaller, HostContext, codes};
use crate::internal_log::InternalEvent;

/// Register the `vsdd::read_prefix` host function with the wasmtime linker.
///
/// Mirrors the registration shape of `read_file::register`. The 6-parameter
/// pointer/length wire ABI is identical to `read_file` (BC-1.17.001 v1.6
/// §(a) layering parenthetical — `-> i32` wire ABI; `Result<Vec<u8>, HostError>`
/// is the SDK safe-wrapper return type in hook-sdk).
pub fn register(linker: &mut Linker<HostContext>) -> Result<(), HostCallError> {
    linker
        .func_wrap(
            "vsdd",
            "read_prefix",
            |mut caller: HostCaller<'_>,
             path_ptr: u32,
             path_len: u32,
             max_bytes: u32,
             timeout_ms: u32,
             out_ptr_out: u32,
             out_len_out: u32|
             -> i32 {
                let _ = timeout_ms; // accepted for ABI stability; enforced via epoch interruption
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

/// All of read_prefix's host-side logic that doesn't touch guest memory.
///
/// Split out so it is unit-testable without a live WASM instance (mirrors
/// `read_file::prepare`). Returns `(bytes, out_ptr_sentinel)` on success or
/// a negative error code on failure.
///
/// Implementation:
///   1. Capability check — require `capabilities.read_prefix` block; deny on absent
///      (does NOT fall back to `capabilities.read_file`).
///   2. Path resolution — `check_path_allowed` via `path_util.rs` (same
///      rejoin + starts_with algorithm as `read_file`).
///   3. max_bytes = 0 — return empty payload immediately, no file opened.
///   4. Existence check — absent allowlisted file → NOT_FOUND (-5) +
///      `internal.file_not_found`.
///   5. Bounded read — open file, read at most `max_bytes` bytes from start
///      using `take` semantics (head-c).
///   6. Directory / OS error — return INTERNAL_ERROR (-99).
///   7. NEVER emit or return OUTPUT_TOO_LARGE (-3) — `max_bytes` IS the cap.
pub(crate) fn prepare(
    ctx: &HostContext,
    path: &str,
    max_bytes: u32,
) -> Result<(Vec<u8>, u32), i32> {
    // 1. Capability check: require capabilities.read_prefix block.
    //    Does NOT fall back to capabilities.read_file (BC-1.17.001 Invariant 3).
    let caps = ctx.capabilities.read_prefix.as_ref().ok_or_else(|| {
        emit_denial(ctx, path, "no_read_prefix_capability", None);
        codes::CAPABILITY_DENIED
    })?;

    // 2. Path resolution: resolve and check against capabilities.read_prefix path_allow.
    //    Uses the same ancestor-walk + rejoin algorithm as read_file (BC-1.17.001 Invariant 4).
    let resolved = resolve_for_read(Path::new(path), &ctx.cwd);

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

    // 3. max_bytes = 0: return empty payload without opening the file (BC-1.17.001 EC-001).
    if max_bytes == 0 {
        return Ok((Vec::new(), 0));
    }

    // 4+5. Bounded read: open file and read at most max_bytes bytes from start.
    //      read_prefix_bounded uses take() semantics — NEVER hits OUTPUT_TOO_LARGE.
    match read_prefix_bounded(&resolved, max_bytes as usize) {
        Ok(bytes) => Ok((bytes, 0)),
        Err(PrefixReadErr::NotFound) => {
            let ev = InternalEvent::now(crate::internal_log::INTERNAL_FILE_NOT_FOUND)
                .with_trace_id(&ctx.dispatcher_trace_id)
                .with_session_id(&ctx.session_id)
                .with_plugin_name(&ctx.plugin_name)
                .with_plugin_version(&ctx.plugin_version)
                .with_field("function", Value::String("read_prefix".to_string()))
                .with_field("reason", Value::String("file_not_found".to_string()))
                .with_field("path", Value::String(path.to_string()))
                .with_field(
                    "resolved",
                    Value::String(resolved.to_string_lossy().into_owned()),
                );
            ctx.emit_internal(ev);
            Err(codes::NOT_FOUND)
        }
        Err(PrefixReadErr::Other) => Err(codes::INTERNAL_ERROR),
    }
}

enum PrefixReadErr {
    /// Path is in the allow-list but the file does not exist.
    NotFound,
    /// Directory target, OS-level I/O error, or other non-NotFound failure.
    Other,
}

/// Resolve a path for reading. Relative paths are resolved under `base`
/// (the project working directory, `$CLAUDE_PROJECT_DIR`). Absolute paths
/// are used as-is. Mirrors `read_file::resolve_for_read`.
fn resolve_for_read(path: &Path, base: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        base.join(path)
    }
}

/// Read at most `max_bytes` bytes from the start of a file (head-c semantics).
///
/// Uses `take(max_bytes as u64)` + `read_to_end` so that:
///   - Files smaller than `max_bytes` return their full content (no padding).
///   - Files larger than `max_bytes` return exactly `max_bytes` bytes.
///   - OUTPUT_TOO_LARGE is NEVER returned — `max_bytes` IS the cap.
fn read_prefix_bounded(path: &Path, max_bytes: usize) -> Result<Vec<u8>, PrefixReadErr> {
    let file = File::open(path).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            PrefixReadErr::NotFound
        } else {
            PrefixReadErr::Other
        }
    })?;
    let mut buf = Vec::with_capacity(max_bytes.min(65_536));
    let mut limited = file.take(max_bytes as u64);
    limited
        .read_to_end(&mut buf)
        .map_err(|_| PrefixReadErr::Other)?;
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
    let ev = ctx.denial_event("read_prefix", reason, details);
    ctx.emit_internal(ev);
}

// ---------------------------------------------------------------------------
// S-19.06 tests — T-001..T-008, T-010, T-012, T-012_MUTANT_VERIFY,
//                 T-013a, T-013b, T-013_MUTANT_VERIFY  (14 tests total)
//
// History: T-001..T-008 + T-010 (9 tests) were written against the
// `todo!()`-body stub.  At that stub phase every call to prepare() panicked
// with "not yet implemented: S-19.06: implement read_prefix prepare function";
// that panic was the Red Gate.  After implementation, prepare() is fully
// replaced; these 9 tests now serve as correctness gates (assertions that
// were preempted by the panic now run and verify actual return values).
//
// T-012, T-012_MUTANT_VERIFY, T-013a, T-013b, and T-013_MUTANT_VERIFY are
// cascade-remediation regression locks added after implementation (findings
// F-P2-001 and F-P4-001).  They were written green — the correct prepare()
// already passed them at commit time.  Each MUTANT_VERIFY companion injects
// the specific ordering violation the lock is designed to catch, proving the
// gate is live (TD-VSDD-059).
//
// T-001  AC-001  BC-1.17.001 PC-1 + PC-6    bounded prefix: 100-byte file → 50 bytes
// T-002  AC-001  BC-1.17.001 PC-6            byte-exact: partial UTF-8 seq returned untrimmed
// T-003  AC-002  BC-1.17.001 PC-2            short file: 30-byte file → 30 bytes, no padding
// T-004  AC-003  BC-1.17.001 PC-3            NEVER OUTPUT_TOO_LARGE: 10000-byte file
// T-005  AC-004  BC-1.17.001 PC-4            no capability block → CAPABILITY_DENIED (-1)
// T-006  AC-004  BC-1.17.001 Invariant 3     read_file cap only → CAPABILITY_DENIED (-1)
// T-007  AC-005  BC-1.17.001 PC-5            absent allowlisted → NOT_FOUND + file_not_found event
// T-008  AC-006  BC-1.17.001 EC-001          max_bytes=0 → empty payload, exit 0
// T-012  F-P2-001 BC-1.17.001 EC-001         absent + max_bytes=0 → Ok(empty), no NOT_FOUND
// T-012_MUTANT_VERIFY  TD-VSDD-059           mutation witness: reorder short-circuit → NOT_FOUND
// T-013a F-P4-001 BC-1.17.001 Inv 3/PC-4    no-cap + max_bytes=0 → CAPABILITY_DENIED
// T-013b F-P4-001 BC-1.17.001 Inv 3/PC-4    outside path_allow + max_bytes=0 → CAPABILITY_DENIED
// T-013_MUTANT_VERIFY  TD-VSDD-059           mutation witness: hoist short-circuit → leaks Ok(empty)
// T-010  EC-004  BC-1.17.001 EC-004          path outside path_allow → CAPABILITY_DENIED + event
// ---------------------------------------------------------------------------

#[cfg(test)]
#[allow(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::panic,
    non_snake_case
)]
mod tests {
    use super::*;
    use crate::host::{codes, test_support::*};
    use crate::registry::{Capabilities, ReadPrefixCaps};

    /// Helper: build Capabilities with only the `read_prefix` block set.
    ///
    /// Mirrors `test_support::allow_read` for `read_file`. Parallel helper
    /// for the independent `capabilities.read_prefix` capability block
    /// (BC-1.17.001 Invariant 3 defense-in-depth).
    fn allow_read_prefix(paths: &[&str]) -> Capabilities {
        Capabilities {
            read_prefix: Some(ReadPrefixCaps {
                path_allow: paths.iter().map(|s| s.to_string()).collect(),
            }),
            ..Default::default()
        }
    }

    // -----------------------------------------------------------------------
    // T-001 (AC-001): 100-byte file, max_bytes=50 → exactly 50 bytes, exit 0
    //
    // BC-1.17.001 PC-1 (bounded prefix) + PC-6 (byte-exact).
    // Red Gate history: panicked at stub phase (prepare() was todo!()). Now a correctness gate.
    // -----------------------------------------------------------------------

    #[test]
    fn test_S19_06_T001_bounded_prefix_returns_exactly_max_bytes() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("data.bin");
        let content: Vec<u8> = (0u8..100).collect(); // 100 bytes, values 0x00..0x63
        std::fs::write(&file_path, &content).unwrap();

        let mut ctx = context_with_caps(allow_read_prefix(&[dir.path().to_str().unwrap()]));
        ctx.cwd = dir.path().to_path_buf();

        let (bytes, _out_ptr) = prepare(&ctx, file_path.to_str().unwrap(), 50)
            .expect("T-001 AC-001: 100-byte file with max_bytes=50 must succeed (exit 0)");

        assert_eq!(
            bytes.len(),
            50,
            "T-001 AC-001 PC-1: response byte count must equal max_bytes=50 (head-c semantics); \
             got {} bytes. BC-1.17.001 PC-1. Red Gate: panics at todo!().",
            bytes.len()
        );
        assert_eq!(
            bytes,
            &content[..50],
            "T-001 AC-001 PC-6: returned bytes must be the exact first 50 bytes of the file \
             (byte-exact; no UTF-8 normalization). BC-1.17.001 PC-6."
        );
    }

    // -----------------------------------------------------------------------
    // T-002 (AC-001): partial multi-byte UTF-8 sequence at 50-byte boundary
    // returned untrimmed — byte-exact semantics (BC-1.17.001 PC-6).
    //
    // File layout: 48 × 'A' (ASCII 0x41) + 中 (U+4E2D = E4 B8 AD, 3 bytes) + padding.
    // max_bytes=50 → positions 0..49 = 48 ASCII + first 2 bytes of 中 (E4 B8).
    // The third byte of 中 (AD) is at position 50 and must NOT be included.
    // BC-1.17.001 PC-6: no UTF-8 boundary trimming — the raw 50 bytes are returned
    // even though they contain a partial multi-byte sequence.
    //
    // Red Gate history: panicked at stub phase (prepare() was todo!()). Now a correctness gate.
    // -----------------------------------------------------------------------

    #[test]
    fn test_S19_06_T002_byte_exact_no_utf8_trimming_at_boundary() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("utf8.bin");

        // 48 ASCII bytes + U+4E2D 中 (E4 B8 AD) + 49 'B' bytes = 100 bytes total.
        // Position 50 boundary: [0..47]=A, [48]=E4, [49]=B8, [50]=AD(excluded), [51..]=B…
        let mut content = vec![b'A'; 48];
        content.extend_from_slice(&[0xE4_u8, 0xB8, 0xAD]); // 中 (U+4E2D), 3 bytes
        content.extend_from_slice(&[b'B'; 49]);
        assert_eq!(content.len(), 100, "test setup: content must be 100 bytes");

        std::fs::write(&file_path, &content).unwrap();

        let mut ctx = context_with_caps(allow_read_prefix(&[dir.path().to_str().unwrap()]));
        ctx.cwd = dir.path().to_path_buf();

        let (bytes, _) = prepare(&ctx, file_path.to_str().unwrap(), 50)
            .expect("T-002 AC-001: file with partial UTF-8 at boundary, max_bytes=50 must succeed");

        assert_eq!(
            bytes.len(),
            50,
            "T-002 AC-001 PC-6: response must be exactly max_bytes=50 bytes — NOT trimmed to 49 \
             to avoid the partial UTF-8 sequence. BC-1.17.001 PC-6: byte-exact, no UTF-8 \
             normalization. Caller is responsible for interpreting partial multi-byte sequences. \
             Red Gate: panics at todo!()."
        );
        // The partial sequence: bytes[48..50] must be the first 2 bytes of 中 = [E4, B8].
        assert_eq!(
            &bytes[48..50],
            &[0xE4_u8, 0xB8],
            "T-002 AC-001 PC-6: bytes[48..50] must be the first 2 bytes of U+4E2D 中 \
             (E4 B8) — partial multi-byte sequence returned untrimmed. \
             BC-1.17.001 PC-6: raw first max_bytes bytes, no trimming."
        );
    }

    // -----------------------------------------------------------------------
    // T-003 (AC-002): 30-byte file, max_bytes=100 → full 30 bytes, no padding, exit 0
    //
    // BC-1.17.001 PC-2: when file_size < max_bytes, full content returned, no padding.
    // Red Gate history: panicked at stub phase (prepare() was todo!()). Now a correctness gate.
    // -----------------------------------------------------------------------

    #[test]
    fn test_S19_06_T003_short_file_returns_full_content_no_padding() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("short.txt");
        let content = b"This is exactly 30 bytes!XXXXX";
        assert_eq!(content.len(), 30, "test setup: content must be 30 bytes");
        std::fs::write(&file_path, content).unwrap();

        let mut ctx = context_with_caps(allow_read_prefix(&[dir.path().to_str().unwrap()]));
        ctx.cwd = dir.path().to_path_buf();

        let (bytes, _) = prepare(&ctx, file_path.to_str().unwrap(), 100)
            .expect("T-003 AC-002: 30-byte file with max_bytes=100 must succeed (exit 0)");

        assert_eq!(
            bytes.len(),
            30,
            "T-003 AC-002 PC-2: response byte count must equal file_size=30, not max_bytes=100; \
             got {} bytes. No padding to fill up to max_bytes. \
             BC-1.17.001 PC-2. Red Gate: panics at todo!().",
            bytes.len()
        );
        assert_eq!(
            bytes.as_slice(),
            content.as_slice(),
            "T-003 AC-002: returned bytes must be the complete file content (no truncation marker)"
        );
    }

    // -----------------------------------------------------------------------
    // T-004 (AC-003): 10000-byte file, max_bytes=50 → return code ≠ OUTPUT_TOO_LARGE (-3)
    //
    // BC-1.17.001 PC-3: read_prefix NEVER returns OUTPUT_TOO_LARGE (-3).
    // This is the runtime load-bearing check (the AC-003 static gate confirms
    // OUTPUT_TOO_LARGE does not appear in non-comment source; this test confirms
    // it is not returned at runtime either).
    //
    // Red Gate history: panicked at stub phase (prepare() was todo!()). Now a correctness gate.
    // -----------------------------------------------------------------------

    #[test]
    fn test_S19_06_T004_never_returns_output_too_large() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("big.bin");
        let content = vec![0xAB_u8; 10_000];
        std::fs::write(&file_path, &content).unwrap();

        let mut ctx = context_with_caps(allow_read_prefix(&[dir.path().to_str().unwrap()]));
        ctx.cwd = dir.path().to_path_buf();

        let result = prepare(&ctx, file_path.to_str().unwrap(), 50);

        match result {
            Ok((bytes, _)) => {
                assert_eq!(
                    bytes.len(),
                    50,
                    "T-004 AC-003: successful bounded read of 10000-byte file with max_bytes=50 \
                     must return exactly 50 bytes"
                );
            }
            Err(code) => {
                assert_ne!(
                    code,
                    codes::OUTPUT_TOO_LARGE,
                    "T-004 AC-003 PC-3: read_prefix MUST NEVER return OUTPUT_TOO_LARGE (-3); \
                     got error code {}. max_bytes IS the cap — data beyond the cap is simply not \
                     read. This is the runtime load-bearing check distinguishing read_prefix from \
                     read_file. BC-1.17.001 PC-3. Red Gate: panics at todo!().",
                    code
                );
            }
        }
    }

    // -----------------------------------------------------------------------
    // T-005 (AC-004 negative): no capabilities.read_prefix → CAPABILITY_DENIED (-1)
    //
    // BC-1.17.001 PC-4: capability gate enforced before any filesystem access.
    // Red Gate history: panicked at stub phase (prepare() was todo!()). Now a correctness gate.
    // -----------------------------------------------------------------------

    #[test]
    fn test_S19_06_T005_no_capability_block_returns_capability_denied() {
        let ctx = bare_context(); // no capabilities at all

        let err = prepare(&ctx, "any/path.txt", 1024).unwrap_err();

        assert_eq!(
            err,
            codes::CAPABILITY_DENIED,
            "T-005 AC-004 PC-4: absent capabilities.read_prefix block must return \
             CAPABILITY_DENIED (-1) before any filesystem access. \
             BC-1.17.001 PC-4. Red Gate: panics at todo!()."
        );
    }

    // -----------------------------------------------------------------------
    // T-006 (AC-004 independence): capabilities.read_file only → CAPABILITY_DENIED (-1)
    //
    // BC-1.17.001 Invariant 3: read_file capability does NOT grant read_prefix access.
    // The two capabilities are independently declared (defense-in-depth).
    // Red Gate history: panicked at stub phase (prepare() was todo!()). Now a correctness gate.
    // -----------------------------------------------------------------------

    #[test]
    fn test_S19_06_T006_read_file_cap_only_returns_capability_denied() {
        // Plugin has ONLY capabilities.read_file — NOT capabilities.read_prefix.
        let ctx = context_with_caps(allow_read(&["/any/path"]));

        let err = prepare(&ctx, "/any/path/file.txt", 1024).unwrap_err();

        assert_eq!(
            err,
            codes::CAPABILITY_DENIED,
            "T-006 AC-004 Invariant 3: capabilities.read_file does NOT grant read_prefix access. \
             CAPABILITY_DENIED (-1) must be returned even when read_file capability is present. \
             Defense-in-depth: the two capabilities are independently declared. \
             BC-1.17.001 Invariant 3. Red Gate: panics at todo!()."
        );
    }

    // -----------------------------------------------------------------------
    // T-007 (AC-005): absent allowlisted file → NOT_FOUND (-5) +
    //   internal.file_not_found event + ZERO capability_denied events
    //
    // BC-1.17.001 PC-5 + Invariant 5.
    // Red Gate history: panicked at stub phase (prepare() was todo!()). Now a correctness gate.
    // -----------------------------------------------------------------------

    #[test]
    fn test_S19_06_T007_absent_allowlisted_file_returns_not_found_and_emits_event() {
        let dir = tempfile::tempdir().unwrap();
        let factory_dir = dir.path().join(".factory");
        std::fs::create_dir_all(&factory_dir).unwrap();
        // Allow .factory/; wave-state.yaml does NOT exist.
        let mut ctx = context_with_caps(allow_read_prefix(&[factory_dir.to_str().unwrap()]));
        ctx.cwd = dir.path().to_path_buf();
        let absent_path = factory_dir.join("wave-state.yaml");
        assert!(
            !absent_path.exists(),
            "test setup: target file must not exist"
        );

        let result = prepare(&ctx, absent_path.to_str().unwrap(), 65536);

        assert_eq!(
            result.unwrap_err(),
            codes::NOT_FOUND,
            "T-007 AC-005 PC-5: absent file within allowlist must return NOT_FOUND (-5), \
             NOT CAPABILITY_DENIED (-1). BC-1.17.001 PC-5, Invariant 5. \
             Red Gate: panics at todo!()."
        );

        let events = ctx.drain_events();

        let file_not_found_count = events
            .iter()
            .filter(|e| e.type_ == "internal.file_not_found")
            .count();
        assert_eq!(
            file_not_found_count,
            1,
            "T-007 AC-005: absent allowlisted file must emit exactly one 'internal.file_not_found' \
             event; got event types: {:?}. BC-1.17.001 PC-5.",
            events.iter().map(|e| &e.type_).collect::<Vec<_>>()
        );

        let cap_denied_count = events
            .iter()
            .filter(|e| e.type_ == "internal.capability_denied")
            .count();
        assert_eq!(
            cap_denied_count, 0,
            "T-007 AC-005: absent allowlisted file must emit ZERO 'internal.capability_denied' \
             events; got {} cap_denied events. BC-1.17.001 PC-5.",
            cap_denied_count
        );
    }

    // -----------------------------------------------------------------------
    // T-008 (AC-006 / EC-001): max_bytes=0 → empty payload (0 bytes), exit 0
    //   File is NOT opened (degenerate input; caller asked for zero bytes).
    //
    // BC-1.17.001 EC-001.
    // Red Gate history: panicked at stub phase (prepare() was todo!()). Now a correctness gate.
    // -----------------------------------------------------------------------

    #[test]
    fn test_S19_06_T008_max_bytes_zero_returns_empty_payload_exit_0() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("present.txt");
        std::fs::write(&file_path, b"some content that must not be read").unwrap();

        let mut ctx = context_with_caps(allow_read_prefix(&[dir.path().to_str().unwrap()]));
        ctx.cwd = dir.path().to_path_buf();

        let (bytes, _) = prepare(&ctx, file_path.to_str().unwrap(), 0)
            .expect("T-008 AC-006 EC-001: max_bytes=0 must succeed (exit 0), not error");

        assert!(
            bytes.is_empty(),
            "T-008 AC-006 EC-001: max_bytes=0 must return empty payload (0 bytes); \
             got {} bytes. BC-1.17.001 EC-001: caller asked for zero bytes — \
             valid degenerate input. File must NOT be opened. \
             Red Gate: panics at todo!().",
            bytes.len()
        );
    }

    // -----------------------------------------------------------------------
    // T-012 (F-P2-001 / EC-001 composite degenerate):
    //   ABSENT allowlisted file + max_bytes=0 → Ok(empty payload, exit 0)
    //   NO NOT_FOUND; ZERO internal.file_not_found events;
    //   ZERO capability_denied events.
    //
    // BC-1.17.001 EC-001: the max_bytes=0 short-circuit in prepare() step 3
    // precedes the existence check (step 4). When max_bytes=0, prepare()
    // returns Ok(empty, 0) WITHOUT opening the file and WITHOUT consulting
    // file existence — even when the path points to an absent file.
    //
    // Regression lock: the ordering invariant is structural, not accidental.
    // If the short-circuit were moved after the existence check, the absent
    // file would reach read_prefix_bounded → PrefixReadErr::NotFound →
    // internal.file_not_found emitted → Err(codes::NOT_FOUND). The
    // `result.expect(...)` below would then fail.
    //
    // Mutation evidence (TD-VSDD-059): see test_S19_06_T012_MUTANT_VERIFY
    // below for the mechanical mutation witness that confirms gate liveness.
    //
    // Red Gate note: PASSES at Red Gate — prepare() implements the
    // max_bytes=0 short-circuit (step 3) before the existence check (step 4).
    // -----------------------------------------------------------------------

    #[test]
    fn test_S19_06_T012_absent_file_max_bytes_zero_short_circuits_before_existence_check() {
        let dir = tempfile::tempdir().unwrap();
        let factory_dir = dir.path().join(".factory");
        std::fs::create_dir_all(&factory_dir).unwrap();
        // Allow .factory/; wave-state.yaml does NOT exist.
        let mut ctx = context_with_caps(allow_read_prefix(&[factory_dir.to_str().unwrap()]));
        ctx.cwd = dir.path().to_path_buf();
        let absent_path = factory_dir.join("wave-state.yaml");
        assert!(
            !absent_path.exists(),
            "test setup: target file must not exist"
        );

        // BC-1.17.001 EC-001: max_bytes=0 short-circuit (prepare step 3)
        // precedes the existence check (prepare step 4). An absent allowlisted
        // file with max_bytes=0 must succeed — file existence is never consulted.
        let result = prepare(&ctx, absent_path.to_str().unwrap(), 0);

        let (bytes, _) = result.expect(
            "T-012 F-P2-001 EC-001: absent allowlisted file + max_bytes=0 must succeed \
             (exit 0, empty payload). The max_bytes=0 short-circuit precedes the existence \
             check; file existence is NOT consulted. BC-1.17.001 EC-001. \
             Mutation: if short-circuit were moved after existence check, this would be \
             Err(NOT_FOUND) instead of Ok.",
        );

        assert!(
            bytes.is_empty(),
            "T-012 EC-001: max_bytes=0 must return empty payload (0 bytes); got {} bytes.",
            bytes.len()
        );

        let events = ctx.drain_events();

        let file_not_found_count = events
            .iter()
            .filter(|e| e.type_ == "internal.file_not_found")
            .count();
        assert_eq!(
            file_not_found_count, 0,
            "T-012 EC-001: max_bytes=0 short-circuit must emit ZERO 'internal.file_not_found' \
             events — the existence check (step 4) is never reached. Got {} events. \
             BC-1.17.001 EC-001.",
            file_not_found_count
        );

        let cap_denied_count = events
            .iter()
            .filter(|e| e.type_ == "internal.capability_denied")
            .count();
        assert_eq!(
            cap_denied_count, 0,
            "T-012 EC-001: must emit ZERO 'internal.capability_denied' events — \
             the file is within path_allow and is never read (max_bytes=0 short-circuit). \
             Got {} events. BC-1.17.001 EC-001.",
            cap_denied_count
        );
    }

    // -----------------------------------------------------------------------
    // T-012 mutation-liveness witness (TD-VSDD-059 mechanical mutation check)
    //
    // Demonstrates that T-012 is a live gate by exercising a local helper
    // `prepare_mutant` that implements the WRONG ordering: existence check
    // before max_bytes=0 short-circuit. The absent file triggers NotFound in
    // the mutant, whereas the correct implementation short-circuits first.
    //
    // This is NOT a passing test of the real prepare() — it is an in-module
    // proof that the T-012 assertions would have failed against the mutant
    // implementation.
    //
    // Evidence format: `prepare_mutant(absent, max_bytes=0)` returns
    // `Err(codes::NOT_FOUND)` (the mutation failure mode), confirming that
    // T-012's `result.expect(...)` would catch any reordering regression.
    // -----------------------------------------------------------------------

    #[test]
    fn test_S19_06_T012_MUTANT_VERIFY_short_circuit_reorder_causes_not_found() {
        // A local helper that mirrors prepare() but with the short-circuit
        // MOVED AFTER the existence check — the mutant ordering that T-012
        // is designed to catch.
        fn prepare_mutant(
            ctx: &HostContext,
            path: &str,
            max_bytes: u32,
        ) -> Result<(Vec<u8>, u32), i32> {
            // Step 1: capability check (unchanged)
            let caps = ctx
                .capabilities
                .read_prefix
                .as_ref()
                .ok_or(codes::CAPABILITY_DENIED)?;
            // Step 2: path check (unchanged)
            let resolved = resolve_for_read(std::path::Path::new(path), &ctx.cwd);
            match super::super::path_util::check_path_allowed(
                &resolved,
                &caps.path_allow,
                &ctx.cwd,
                |p| p.canonicalize(),
            ) {
                super::super::path_util::PathAllowDecision::Allowed => {}
                _ => return Err(codes::CAPABILITY_DENIED),
            }
            // Step 3 MUTANT: existence check BEFORE max_bytes=0 short-circuit.
            // This is the wrong ordering — the real prepare() checks max_bytes=0 first.
            match read_prefix_bounded(&resolved, max_bytes as usize) {
                Ok(bytes) => Ok((bytes, 0)),
                Err(PrefixReadErr::NotFound) => {
                    let ev = crate::internal_log::InternalEvent::now("internal.file_not_found")
                        .with_trace_id(&ctx.dispatcher_trace_id)
                        .with_session_id(&ctx.session_id)
                        .with_plugin_name(&ctx.plugin_name)
                        .with_plugin_version(&ctx.plugin_version);
                    ctx.emit_internal(ev);
                    Err(codes::NOT_FOUND)
                }
                Err(PrefixReadErr::Other) => Err(codes::INTERNAL_ERROR),
            }
        }

        let dir = tempfile::tempdir().unwrap();
        let factory_dir = dir.path().join(".factory");
        std::fs::create_dir_all(&factory_dir).unwrap();
        let mut ctx = context_with_caps(allow_read_prefix(&[factory_dir.to_str().unwrap()]));
        ctx.cwd = dir.path().to_path_buf();
        let absent_path = factory_dir.join("wave-state.yaml");
        assert!(!absent_path.exists(), "test setup: file must not exist");

        // Mutation evidence: the mutant returns NOT_FOUND for absent+max_bytes=0.
        // This is the behavior that T-012 would have caught via its `expect(...)`.
        let mutant_result = prepare_mutant(&ctx, absent_path.to_str().unwrap(), 0);
        assert_eq!(
            mutant_result.unwrap_err(),
            codes::NOT_FOUND,
            "T-012 mutation witness: prepare_mutant (short-circuit AFTER existence check) \
             must return NOT_FOUND for absent+max_bytes=0. This proves the gate in \
             test_S19_06_T012 is live: the real prepare() would pass where mutant fails."
        );

        let events = ctx.drain_events();
        let file_not_found_count = events
            .iter()
            .filter(|e| e.type_ == "internal.file_not_found")
            .count();
        assert_eq!(
            file_not_found_count, 1,
            "T-012 mutation witness: mutant must emit exactly 1 'internal.file_not_found' \
             event (existence check fires before short-circuit). Got {}.",
            file_not_found_count
        );
    }

    // -----------------------------------------------------------------------
    // T-013a (F-P4-001): NO capabilities.read_prefix block + max_bytes=0
    //   → CAPABILITY_DENIED (-1) + capability_denied event
    //   (capability check step 1 precedes max_bytes=0 short-circuit step 3)
    //
    // Ordering lock: every existing max_bytes=0 test (T-008, T-012) exercises a
    // caller that has BOTH capability AND an allowed path — a mutation hoisting
    // the step-3 short-circuit above steps 1+2 would still pass all prior tests
    // because the authorized callers reach step 3 unimpeded. T-013a locks step 1
    // ordering: a no-cap caller with max_bytes=0 MUST receive CAPABILITY_DENIED,
    // not Ok(empty). BC-1.17.001 Invariant 3 / PC-4 deny-by-default.
    // -----------------------------------------------------------------------

    #[test]
    fn test_S19_06_T013a_no_capability_max_bytes_zero_returns_capability_denied() {
        let ctx = bare_context(); // no capabilities at all

        let err = prepare(&ctx, "any/path.txt", 0).unwrap_err();

        assert_eq!(
            err,
            codes::CAPABILITY_DENIED,
            "T-013a F-P4-001: no capabilities.read_prefix block + max_bytes=0 must return \
             CAPABILITY_DENIED (-1). The capability check (step 1) precedes the max_bytes=0 \
             short-circuit (step 3) — an unauthorized caller must be denied even when \
             asking for zero bytes. BC-1.17.001 Invariant 3 / PC-4 deny-by-default."
        );

        let events = ctx.drain_events();
        let cap_denied_count = events
            .iter()
            .filter(|e| e.type_ == "internal.capability_denied")
            .count();
        assert_eq!(
            cap_denied_count, 1,
            "T-013a F-P4-001: CAPABILITY_DENIED must emit exactly one \
             'internal.capability_denied' event; got {} events. \
             BC-1.17.001 PC-4.",
            cap_denied_count
        );
    }

    // -----------------------------------------------------------------------
    // T-013b (F-P4-001): capabilities.read_prefix present, path OUTSIDE
    //   path_allow + max_bytes=0 → CAPABILITY_DENIED (-1) + event
    //   reason=path_not_allowed
    //   (path check step 2 precedes max_bytes=0 short-circuit step 3)
    //
    // Ordering lock: a mutation hoisting the step-3 short-circuit above step 2
    // would bypass the path-allowlist check for max_bytes=0 callers, returning
    // Ok(empty) to a caller whose path is outside the declared allow-list.
    // T-013b locks step 2 ordering. BC-1.17.001 Invariant 3 / PC-4.
    // -----------------------------------------------------------------------

    #[test]
    fn test_S19_06_T013b_path_outside_allowlist_max_bytes_zero_returns_capability_denied() {
        let dir = tempfile::tempdir().unwrap();
        let allowed_dir = dir.path().join("allowed");
        let other_dir = dir.path().join("other");
        std::fs::create_dir_all(&allowed_dir).unwrap();
        std::fs::create_dir_all(&other_dir).unwrap();
        // File exists so canonicalization succeeds; capability covers `allowed/` NOT `other/`.
        let target_file = other_dir.join("secret.txt");
        std::fs::write(&target_file, b"secret data").unwrap();

        let mut ctx = context_with_caps(allow_read_prefix(&[allowed_dir.to_str().unwrap()]));
        ctx.cwd = dir.path().to_path_buf();

        let result = prepare(&ctx, target_file.to_str().unwrap(), 0);

        assert_eq!(
            result.unwrap_err(),
            codes::CAPABILITY_DENIED,
            "T-013b F-P4-001: path outside capabilities.read_prefix path_allow + max_bytes=0 \
             must return CAPABILITY_DENIED (-1). The path check (step 2) precedes the \
             max_bytes=0 short-circuit (step 3) — a caller with a non-allowed path must be \
             denied even when asking for zero bytes. BC-1.17.001 Invariant 3 / PC-4."
        );

        let events = ctx.drain_events();
        let cap_denied: Vec<_> = events
            .iter()
            .filter(|e| e.type_ == "internal.capability_denied")
            .collect();
        assert_eq!(
            cap_denied.len(),
            1,
            "T-013b F-P4-001: path_allow mismatch + max_bytes=0 must emit exactly one \
             'internal.capability_denied' event; got {} events.",
            cap_denied.len()
        );

        let reason = cap_denied[0].fields.get("reason").and_then(|v| v.as_str());
        assert_eq!(
            reason,
            Some("path_not_allowed"),
            "T-013b F-P4-001: capability_denied event must carry reason=path_not_allowed; \
             got {:?}. BC-1.17.001 EC-004.",
            reason
        );
    }

    // -----------------------------------------------------------------------
    // T-013 mutation-liveness witness (TD-VSDD-059 mechanical mutation check)
    //
    // Demonstrates that T-013a/b are live gates by exercising a local helper
    // `prepare_mutant_hoisted_short_circuit` that implements the WRONG ordering:
    // max_bytes=0 short-circuit BEFORE the capability check (and path check).
    // A no-cap caller with max_bytes=0 gets Ok(empty) from the mutant where the
    // real prepare() returns Err(CAPABILITY_DENIED) — BC-1.17.001 Invariant 3
    // violated undetected by all prior tests that combined cap+path+max_bytes=0.
    //
    // This is NOT a passing test of the real prepare() — it is an in-module
    // proof that the T-013a/b assertions would have failed against this mutant
    // implementation, confirming the tests are not vacuous.
    //
    // Evidence format:
    //   `prepare_mutant_hoisted_short_circuit(no_cap_ctx, max_bytes=0)` →
    //   Ok(empty) (the mutation leak mode);
    //   `prepare(no_cap_ctx, max_bytes=0)` → Err(CAPABILITY_DENIED) (correct).
    // -----------------------------------------------------------------------

    #[test]
    fn test_S19_06_T013_MUTANT_VERIFY_hoisted_short_circuit_leaks_to_unauthorized_caller() {
        // A local helper that mirrors prepare() but with the max_bytes=0 short-circuit
        // MOVED ABOVE both the capability check (step 1) and path check (step 2).
        // This is the mutation that T-013a/b are designed to catch.
        fn prepare_mutant_hoisted_short_circuit(
            ctx: &HostContext,
            path: &str,
            max_bytes: u32,
        ) -> Result<(Vec<u8>, u32), i32> {
            // MUTANT step 3 HOISTED: max_bytes=0 short-circuit before steps 1+2.
            // Real prepare(): step 1 (capability) → step 2 (path) → step 3 (max_bytes=0).
            if max_bytes == 0 {
                return Ok((Vec::new(), 0));
            }
            // Step 1 (never reached for max_bytes=0 in the mutant)
            let caps = ctx
                .capabilities
                .read_prefix
                .as_ref()
                .ok_or(codes::CAPABILITY_DENIED)?;
            // Step 2 (never reached for max_bytes=0 in the mutant)
            let resolved = resolve_for_read(std::path::Path::new(path), &ctx.cwd);
            match super::super::path_util::check_path_allowed(
                &resolved,
                &caps.path_allow,
                &ctx.cwd,
                |p| p.canonicalize(),
            ) {
                super::super::path_util::PathAllowDecision::Allowed => {}
                _ => return Err(codes::CAPABILITY_DENIED),
            }
            // Step 4+5: bounded read
            match read_prefix_bounded(&resolved, max_bytes as usize) {
                Ok(bytes) => Ok((bytes, 0)),
                Err(PrefixReadErr::NotFound) => Err(codes::NOT_FOUND),
                Err(PrefixReadErr::Other) => Err(codes::INTERNAL_ERROR),
            }
        }

        // Mutation evidence: no-cap caller + max_bytes=0 → Ok(empty) from mutant.
        // This is the BC-1.17.001 Invariant 3 / PC-4 violation the mutation introduces.
        let no_cap_ctx = bare_context();
        let mutant_result = prepare_mutant_hoisted_short_circuit(&no_cap_ctx, "any/path.txt", 0);
        assert!(
            mutant_result.is_ok(),
            "T-013 mutation witness: prepare_mutant_hoisted_short_circuit (max_bytes=0 \
             short-circuit BEFORE capability check) must return Ok(empty) for no-cap caller \
             + max_bytes=0. This confirms T-013a is a live gate: the real prepare() would \
             return Err(CAPABILITY_DENIED) where the mutant leaks Ok(empty) to an \
             unauthorized caller. BC-1.17.001 Invariant 3 / PC-4."
        );
        let (bytes, _) = mutant_result.unwrap();
        assert!(
            bytes.is_empty(),
            "T-013 mutation witness: mutant must leak an empty payload (not bytes from file) \
             to the no-cap caller — Ok(empty) is the security-relevant leak mode."
        );

        // Cross-check: the real prepare() DOES deny the no-cap caller even with max_bytes=0,
        // confirming T-013a's assertion is meaningful (not vacuous).
        let real_result = prepare(&no_cap_ctx, "any/path.txt", 0);
        assert_eq!(
            real_result.unwrap_err(),
            codes::CAPABILITY_DENIED,
            "T-013 mutation witness: real prepare() must return CAPABILITY_DENIED for \
             no-cap + max_bytes=0 (proving T-013a is live: the real prepare denies where \
             the hoisted mutant leaks). BC-1.17.001 Invariant 3 / PC-4."
        );
    }

    // -----------------------------------------------------------------------
    // T-010 (EC-004): capabilities.read_prefix present, path_allow lists a different
    //   path → CAPABILITY_DENIED (-1) + capability_denied event reason=path_not_allowed
    //
    // BC-1.17.001 EC-004.
    // Red Gate history: panicked at stub phase (prepare() was todo!()). Now a correctness gate.
    // -----------------------------------------------------------------------

    #[test]
    fn test_S19_06_T010_path_outside_allowlist_returns_capability_denied_with_event() {
        let dir = tempfile::tempdir().unwrap();
        let allowed_dir = dir.path().join("allowed");
        let other_dir = dir.path().join("other");
        std::fs::create_dir_all(&allowed_dir).unwrap();
        std::fs::create_dir_all(&other_dir).unwrap();
        let target_file = other_dir.join("secret.txt");
        std::fs::write(&target_file, b"secret data").unwrap();

        // capabilities.read_prefix is present, but path_allow covers `allowed/` NOT `other/`.
        let mut ctx = context_with_caps(allow_read_prefix(&[allowed_dir.to_str().unwrap()]));
        ctx.cwd = dir.path().to_path_buf();

        let result = prepare(&ctx, target_file.to_str().unwrap(), 1024);

        assert_eq!(
            result.unwrap_err(),
            codes::CAPABILITY_DENIED,
            "T-010 EC-004: path outside capabilities.read_prefix path_allow must return \
             CAPABILITY_DENIED (-1). The read_prefix capability block IS present but the \
             requested path is not in path_allow. BC-1.17.001 EC-004. \
             Red Gate: panics at todo!()."
        );

        let events = ctx.drain_events();

        let cap_denied: Vec<_> = events
            .iter()
            .filter(|e| e.type_ == "internal.capability_denied")
            .collect();
        assert_eq!(
            cap_denied.len(),
            1,
            "T-010 EC-004: path_allow mismatch must emit exactly one 'internal.capability_denied' \
             event; got event types: {:?}",
            events.iter().map(|e| &e.type_).collect::<Vec<_>>()
        );

        let reason = cap_denied[0].fields.get("reason").and_then(|v| v.as_str());
        assert_eq!(
            reason,
            Some("path_not_allowed"),
            "T-010 EC-004: capability_denied event must carry reason=path_not_allowed; \
             got {:?}. BC-1.17.001 EC-004.",
            reason
        );
    }
}
