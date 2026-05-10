//! Trivial resolver example compiled to wasm32-wasip1.
//!
//! This crate exists solely as a build artifact for the WASM export integration
//! test (`tests/wasm32_resolver_export_integration.rs`). It verifies that the
//! `#[resolver]` macro correctly generates an `extern "C" fn resolve` export
//! in the compiled `.wasm` binary (BC-4.12.002 PC1/PC5).
//!
//! Build command:
//!   cargo build --target wasm32-wasip1 -p wasm-resolver-export --release
//!
//! The resulting `.wasm` file is inspected by the integration test to confirm
//! `resolve` appears in the export section.
//!
//! Note: `resolve_impl` appears as dead code on non-wasm32 host builds because
//! the macro-generated `extern "C" fn resolve(...)` is gated to wasm32 targets.
//! The `dead_code` warning is suppressed here — the function IS used on target.

use vsdd_hook_sdk::resolver as resolver_macro;
use vsdd_hook_sdk::resolver::{ResolverInput, ResolverOutput};

/// Trivial resolver: returns the project dir as context.
///
/// BC-4.12.002 PC5: the `#[resolver]` macro generates a
/// `pub extern "C" fn resolve(input_ptr: i32, input_len: i32) -> i64` WASM
/// export that wraps this function.
#[allow(dead_code)]
#[resolver_macro]
fn resolve_impl(input: ResolverInput) -> ResolverOutput {
    ResolverOutput {
        key: "wasm-export-test".to_string(),
        value: Some(serde_json::json!({ "project_dir": input.project_dir })),
    }
}
