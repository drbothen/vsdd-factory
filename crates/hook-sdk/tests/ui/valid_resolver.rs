// AC-005 trybuild test: #[resolver] macro compiles successfully on valid signature.
//
// This crate applies #[resolver] to a function with the CORRECT signature:
//   fn resolve_impl(input: ResolverInput) -> ResolverOutput
//
// Expected: compiles without error (no .stderr file = must pass).
//
// GREEN: macro implemented. This trybuild case asserts that #[resolver]
// applied to the canonical signature compiles cleanly and emits a
// #[cfg(target_arch="wasm32")] resolve() body. (Host build elides body;
// wasm32 export verified by tests/wasm32_resolver_export_integration.rs.)
//
// Traces: AC-005, BC-4.12.002 postcondition 5.

// Note: this fixture relies on `vsdd-hook-sdk`'s [dependencies] serde_json
// (regular, not dev-only). If serde_json is ever demoted to dev-dependencies,
// this trybuild fixture would silently break AC-005.
use vsdd_hook_sdk::{ResolverInput, ResolverOutput, resolver};

// Note: trybuild compiles this on the HOST target. The macro's #[cfg(target_arch="wasm32")]
// gate elides the `extern "C" fn resolve` body on host, so this t.pass() test only
// verifies macro acceptance — NOT export presence. The named-export verification lives
// in tests/wasm32_resolver_export_integration.rs (currently #[ignore]'d; workspace
// WASM build covers the compile path).
//
// (Test Plan note: AC-005 coverage is split — host trybuild verifies macro
// acceptance only; named-export verification lives in
// tests/wasm32_resolver_export_integration.rs (#[ignore] by default).
// Per BC-4.12.002 PC1 (packed-i64 ABI), wasm32 export presence is the
// authoritative AC-005 verification target.)
#[resolver]
fn resolve_impl(input: ResolverInput) -> ResolverOutput {
    ResolverOutput {
        key: "test-context".to_string(),
        value: Some(serde_json::json!({ "test": true })),
    }
}
