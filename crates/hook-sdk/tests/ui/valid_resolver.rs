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

#[resolver]
fn resolve_impl(input: ResolverInput) -> ResolverOutput {
    ResolverOutput {
        key: "test-context".to_string(),
        value: Some(serde_json::json!({ "test": true })),
    }
}
