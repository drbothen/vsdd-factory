// Trybuild compile-fail: #[resolver] must reject async fn.
//
// The WASM resolver entrypoint is synchronous (BC-4.12.002 PC5).
// Applying #[resolver] to an async fn must emit a compile-time error.
//
// Expected: fails with a message about async not being supported.
// Traces: BC-4.12.002 PC5, F-P2-006.

#[allow(unused_imports)]
use vsdd_hook_sdk::resolver::{ResolverInput, ResolverOutput};
use vsdd_hook_sdk::resolver;

#[resolver]
async fn resolve_impl(input: ResolverInput) -> ResolverOutput {
    let _ = input;
    ResolverOutput {
        key: "test".to_string(),
        value: None,
    }
}
