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

use wasmtime::Linker;

use super::{HostCallError, HostCaller, HostContext};

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
            |_caller: HostCaller<'_>,
             _path_ptr: u32,
             _path_len: u32,
             _max_bytes: u32,
             _timeout_ms: u32,
             _out_ptr_out: u32,
             _out_len_out: u32|
             -> i32 {
                todo!("S-19.06: implement read_prefix host function body")
            },
        )
        .map_err(|e| HostCallError::Linker(e.to_string()))?;
    Ok(())
}

/// All of read_prefix's host-side logic that doesn't touch guest memory.
#[allow(dead_code)]
///
/// Split out so it is unit-testable without a live WASM instance (mirrors
/// `read_file::prepare`). Returns `(bytes, out_ptr_sentinel)` on success or
/// a negative error code on failure.
///
/// Implementation responsibilities (S-19.06 Tasks 10–11):
///   1. Capability check — require `capabilities.read_prefix` block; deny on absent
///      (does NOT fall back to `capabilities.read_file`).
///   2. Path resolution — `resolve_path_for_allowlist` + `check_path_allowed` from
///      `path_util.rs` (same rejoin + starts_with algorithm as `read_file`).
///   3. Existence check — absent allowlisted file → NOT_FOUND (-5) +
///      `internal.file_not_found`.
///   4. Bounded read — open file, read at most `max_bytes` bytes from start;
///      `max_bytes = 0` → return empty payload immediately, no file opened.
///   5. Timeout — respect `timeout_ms`; return TIMEOUT (-2) on expiry.
///   6. Directory / OS error — return INTERNAL_ERROR (-99).
///   7. NEVER emit or return OUTPUT_TOO_LARGE (-3) — `max_bytes` IS the cap.
pub(crate) fn prepare(
    _ctx: &HostContext,
    _path: &str,
    _max_bytes: u32,
) -> Result<(Vec<u8>, u32), i32> {
    todo!("S-19.06: implement read_prefix prepare function")
}

// ---------------------------------------------------------------------------
// S-19.06 Red Gate tests — T-001..T-008 + T-010
//
// All tests call `prepare()` which is `todo!()` in the stub.  Every test
// therefore PANICS at Red Gate (todo!() unwinds with a panic message
// "not yet implemented: S-19.06: implement read_prefix prepare function").
//
// The test names, assertion messages, and expected values are the load-bearing
// specification.  Once `prepare()` is implemented the assertions become the
// correctness gate; the panic IS the Red Gate.
//
// T-001  AC-001  BC-1.17.001 PC-1 + PC-6   bounded prefix: 100-byte file → 50 bytes
// T-002  AC-001  BC-1.17.001 PC-6           byte-exact: partial UTF-8 seq returned untrimmed
// T-003  AC-002  BC-1.17.001 PC-2           short file: 30-byte file → 30 bytes, no padding
// T-004  AC-003  BC-1.17.001 PC-3           NEVER OUTPUT_TOO_LARGE: 10000-byte file
// T-005  AC-004  BC-1.17.001 PC-4           no capability block → CAPABILITY_DENIED (-1)
// T-006  AC-004  BC-1.17.001 Invariant 3    read_file cap only → CAPABILITY_DENIED (-1)
// T-007  AC-005  BC-1.17.001 PC-5           absent allowlisted → NOT_FOUND + file_not_found event
// T-008  AC-006  BC-1.17.001 EC-001         max_bytes=0 → empty payload, exit 0
// T-010  EC-004  BC-1.17.001 EC-004         path outside path_allow → CAPABILITY_DENIED + event
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
    use crate::registry::{Capabilities, ReadFileCaps, ReadPrefixCaps};

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
    // Red Gate: PANICS — prepare() is todo!(). Panic IS the Red Gate evidence.
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
    // Red Gate: PANICS — prepare() is todo!().
    // -----------------------------------------------------------------------

    #[test]
    fn test_S19_06_T002_byte_exact_no_utf8_trimming_at_boundary() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("utf8.bin");

        // 48 ASCII bytes + U+4E2D 中 (E4 B8 AD) + 49 'B' bytes = 100 bytes total.
        // Position 50 boundary: [0..47]=A, [48]=E4, [49]=B8, [50]=AD(excluded), [51..]=B…
        let mut content = vec![b'A'; 48];
        content.extend_from_slice(&[0xE4_u8, 0xB8, 0xAD]); // 中 (U+4E2D), 3 bytes
        content.extend_from_slice(&vec![b'B'; 49]);
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
    // Red Gate: PANICS — prepare() is todo!().
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
    // Red Gate: PANICS — prepare() is todo!().
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
    // Red Gate: PANICS — prepare() is todo!().
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
    // Red Gate: PANICS — prepare() is todo!().
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
    // Red Gate: PANICS — prepare() is todo!().
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
        assert!(!absent_path.exists(), "test setup: target file must not exist");

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
            cap_denied_count,
            0,
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
    // Red Gate: PANICS — prepare() is todo!().
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
    // T-010 (EC-004): capabilities.read_prefix present, path_allow lists a different
    //   path → CAPABILITY_DENIED (-1) + capability_denied event reason=path_not_allowed
    //
    // BC-1.17.001 EC-004.
    // Red Gate: PANICS — prepare() is todo!().
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
