//! FFI boundary integration gate fixture for S-19.06 AC-007 Gate 4.
//!
//! Proves that `vsdd_hook_sdk::host::read_prefix` compiles and links
//! successfully when built for the `wasm32-wasip1` target. The call is
//! a no-op (max_bytes = 0, path = ""); on the wasm target the return value
//! is HostError::CapabilityDenied because no dispatcher is present in the
//! fixture binary. On the host target the stub returns -1 (CAPABILITY_DENIED).
//!
//! This file must remain as a minimal, import-only fixture. It is NOT a
//! production hook plugin — it exists only to satisfy T-009f (cargo build
//! -p read-prefix-fixture --target wasm32-wasip1 exits 0).
//!
//! BC-1.17.001 v1.6, S-19.06 AC-007.

use vsdd_hook_sdk::host;

fn main() {
    // Invoke read_prefix with degenerate inputs (empty path, max_bytes=0).
    // The return value is discarded — this call exists to exercise the
    // wasm32 extern linkage, not to produce meaningful output.
    // BC-1.17.001 EC-001: max_bytes=0 → empty payload; no file opened.
    let _result = host::read_prefix("", 0, 0);
}
