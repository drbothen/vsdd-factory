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

use vsdd_hook_sdk::resolver::{ResolverInput, ResolverOutput};
// resolver is re-exported from vsdd_hook_sdk under the resolver-authoring feature (F-P2-008)
use vsdd_hook_sdk::resolver as resolver_macro;

/// Trivial resolver: returns the project dir as context.
///
/// BC-4.12.002 PC5: the `#[resolver]` macro generates a
/// `pub extern "C" fn resolve(input_ptr: i32, input_len: i32) -> i64` WASM
/// export that wraps this function.
#[resolver_macro]
fn resolve_impl(input: ResolverInput) -> ResolverOutput {
    ResolverOutput {
        key: "wasm-export-test".to_string(),
        value: Some(serde_json::json!({ "project_dir": input.project_dir })),
    }
}
