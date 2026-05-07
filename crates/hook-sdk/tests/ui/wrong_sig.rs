// AC-006 trybuild test: #[resolver] macro rejects wrong function signature.
//
// This crate applies #[resolver] to a function with the WRONG signature:
//   fn resolve_impl() -> String  (wrong: no input, wrong return type)
//
// Expected compile error: message must contain "resolve_impl" and "ResolverOutput"
// to indicate the macro validated the signature and emitted a diagnostic.
//
// The matching .stderr file (wrong_sig.stderr) specifies the expected error.
//
// RED GATE: FAILS because the macro body is `todo!()` — the actual compile error
// is "not yet implemented: S-12.05 Step 4 implementer..." rather than a
// signature validation diagnostic containing "resolve_impl" and "ResolverOutput".
// The .stderr mismatch causes the trybuild test to fail.
//
// Traces: AC-006, BC-4.12.002 postcondition 5.

use vsdd_hook_sdk_macros::resolver;

#[resolver]
fn resolve_impl() -> String {
    "wrong return type".to_string()
}
