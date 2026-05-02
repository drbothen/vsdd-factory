//! AC-4 SDK unit tests for `host::write_file`.
//!
//! Tests the non-wasm stub behavior: `ffi::write_file` returns `-1` on
//! non-wasm targets (no dispatcher present), and the SDK wrapper converts
//! that via `HostError::from_code(-1)` → `HostError::CapabilityDenied`.
//!
//! These tests are named with the BC-based pattern required by TDD convention.
//! They exercise the SDK wrapper (`host::write_file`) not the dispatcher
//! implementation (`write_file::prepare`).
//!
//! References:
//! - BC-2.02.011 postcondition 1, invariant 2, invariant 4
//! - S-8.10 AC-4 (SDK unit tests)
//!
//! Note: all tests here run on non-wasm targets only (the ffi stubs are
//! non-wasm; on wasm32 these tests cannot compile without a dispatcher).

use vsdd_hook_sdk::host::{HostError, write_file};

// ---------------------------------------------------------------------------
// test_BC_2_02_011_sdk_stub_returns_capability_denied
// ---------------------------------------------------------------------------

/// BC-2.02.011 (AC-4a): non-wasm stub for `ffi::write_file` returns `-1`,
/// and the SDK wrapper converts that to `Err(HostError::CapabilityDenied)`.
///
/// Traces: BC-2.02.011 postcondition 1; S-8.10 AC-4(a).
#[test]
fn test_BC_2_02_011_sdk_stub_returns_capability_denied() {
    // On non-wasm targets the FFI stub always returns -1 (no dispatcher).
    let result = write_file("/some/path.txt", b"data", 1024, 5000);
    assert_eq!(
        result,
        Err(HostError::CapabilityDenied),
        "non-wasm stub must return Err(CapabilityDenied) (AC-4a, BC-2.02.011 postcondition 1)"
    );
}

// ---------------------------------------------------------------------------
// test_BC_2_02_011_sdk_max_bytes_and_timeout_accepted_without_panic
// ---------------------------------------------------------------------------

/// BC-2.02.011 (AC-4b): `max_bytes` and `timeout_ms` parameters are
/// accepted without panic. The stub simply ignores them and returns -1.
///
/// Traces: BC-2.02.011 invariant 2 (max_bytes mandatory, no opt-out); AC-4(b).
#[test]
fn test_BC_2_02_011_sdk_max_bytes_and_timeout_accepted_without_panic() {
    // max_bytes=0 (boundary), timeout_ms=0 (EC-004) — no panic.
    let _ = write_file("/p", b"", 0, 0);
    // max_bytes=u32::MAX, timeout_ms=u32::MAX — no panic.
    let _ = write_file("/p", b"x", u32::MAX, u32::MAX);
    // Both cases return Err(CapabilityDenied) from the stub; no assertion
    // beyond "no panic" is the point of this test.
}

// ---------------------------------------------------------------------------
// test_BC_2_02_011_sdk_empty_contents_accepted_without_panic
// ---------------------------------------------------------------------------

/// BC-2.02.011 EC-005 (SDK level): calling `write_file` with an empty
/// contents slice is accepted by the SDK wrapper without panic.
/// The stub returns -1 (capability denied) — normal behavior on non-wasm.
#[test]
fn test_BC_2_02_011_sdk_empty_contents_accepted_without_panic() {
    let result = write_file("/tmp/empty.txt", b"", 1024, 5000);
    // Stub always returns -1; just verify the call doesn't panic.
    assert!(
        result.is_err(),
        "non-wasm stub returns error for empty contents (no dispatcher)"
    );
}

// ---------------------------------------------------------------------------
// test_BC_2_02_011_sdk_error_enum_covers_all_write_file_outcomes
// ---------------------------------------------------------------------------

/// BC-2.02.011 invariant 5 (SDK level): the `HostError` enum covers all
/// outcomes `write_file` can return.  This test enumerates each variant
/// and verifies it exists as a distinct value.
///
/// No new error variants were introduced by this story; this test asserts
/// the enum is structurally stable.
///
/// Traces: BC-2.02.011 invariant 5; S-8.10 Architecture Compliance Rule 4.
#[test]
fn test_BC_2_02_011_sdk_error_enum_covers_all_write_file_outcomes() {
    // On non-wasm targets the stub always returns -1 (CapabilityDenied).
    // We verify the full variant set by constructing each directly.
    let _ = HostError::CapabilityDenied; // -1: no capability / path traversal
    let _ = HostError::Timeout;          // -2: exceeded timeout_ms
    let _ = HostError::OutputTooLarge;   // -3: contents.len() > max_bytes
    let _ = HostError::InvalidArgument;  // -4: bad path UTF-8
    let _ = HostError::Other(-99);       // -99: fs I/O error / missing parent

    // Non-equality: each variant is distinguishable (PartialEq derived).
    assert_ne!(HostError::CapabilityDenied, HostError::Timeout);
    assert_ne!(HostError::Timeout, HostError::OutputTooLarge);
    assert_ne!(HostError::OutputTooLarge, HostError::InvalidArgument);
    assert_ne!(HostError::InvalidArgument, HostError::Other(-99));
}

// ---------------------------------------------------------------------------
// test_BC_2_02_011_sdk_version_bumped_to_0_2_0
// ---------------------------------------------------------------------------

/// AC-7 (S-8.10): `vsdd-hook-sdk` version must be `0.2.0` after adding
/// `host::write_file` (new public API function requires a minor version bump).
///
/// Traces: S-8.10 AC-7.
#[test]
fn test_BC_2_02_011_sdk_version_bumped_to_0_2_0() {
    assert_eq!(
        vsdd_hook_sdk::VERSION,
        "0.2.0",
        "vsdd-hook-sdk crate version must be 0.2.0 after adding host::write_file (AC-7)"
    );
}
