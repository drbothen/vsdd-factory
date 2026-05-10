// AC-005 trybuild test: #[resolver] macro compiles successfully on valid signature.
//
// This crate applies #[resolver] to a function with the CORRECT signature:
//   fn resolve_impl(input: ResolverInput) -> ResolverOutput
//
// Expected: compiles without error (no .stderr file = must pass).
//
// RED GATE: FAILS because the macro body is `todo!()` — the macro panics
// during compilation instead of generating the `resolve()` export.
// Will pass GREEN after the Step 4 implementer completes the macro.
//
// Traces: AC-005, BC-4.12.002 postcondition 5.

use vsdd_hook_sdk::{ResolverInput, ResolverOutput, resolver};

// Note: trybuild compiles this on the HOST target. The macro's #[cfg(target_arch="wasm32")]
// gate elides the `extern "C" fn resolve` body on host, so this t.pass() test only
// verifies macro acceptance — NOT export presence. The named-export verification lives
// in tests/wasm32_resolver_export_integration.rs (currently #[ignore]'d; workspace
// WASM build covers the compile path).
#[resolver]
fn resolve_impl(input: ResolverInput) -> ResolverOutput {
    ResolverOutput {
        key: "test-context".to_string(),
        value: Some(serde_json::json!({ "test": true })),
    }
}
