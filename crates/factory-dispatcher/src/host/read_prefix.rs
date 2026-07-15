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
