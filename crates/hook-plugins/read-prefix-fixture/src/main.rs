//! FFI boundary integration gate fixture for S-19.06 AC-007 Gate 4.
//!
//! This crate is the compile/link witness for bats T-009f: it proves that
//! `vsdd_hook_sdk::host::read_prefix` resolves to the correct
//! `#[link(wasm_import_module = "vsdd")]` import when built for
//! `wasm32-wasip1`. The fixture is NEVER executed — it exists only to
//! confirm the import resolves at link time.
//!
//! Wasm32 target: the `vsdd::read_prefix` import is satisfiable only by a
//! real dispatcher. Attempting to instantiate this fixture standalone (without
//! a dispatcher providing the `vsdd` import namespace) fails at instantiation
//! with an unresolved-import error; no return value is produced.
//!
//! Non-wasm (host_stubs) target: the stub returns -1, which maps to
//! `HostError::CapabilityDenied`. This path is exercised by
//! `cargo test -p factory-dispatcher -- host::read_prefix`.
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
