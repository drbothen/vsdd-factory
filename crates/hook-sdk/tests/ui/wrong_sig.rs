// AC-006 trybuild test: #[resolver] macro rejects wrong function signature.
// Applied to `fn resolve_impl() -> String` (wrong: no input, wrong return type).
// Expected compile error: see wrong_sig.stderr.
// Traces: AC-006, BC-4.12.002 postcondition 5.

use vsdd_hook_sdk::resolver;

// Imports above: vsdd_hook_sdk re-exports #[resolver] under feature resolver-authoring.
// The trybuild harness compiles this file with the features of the parent test run.
// !!=========================================================================
// !! Line position is pinned by tests/ui/wrong_sig.stderr (`--> tests/ui/wrong_sig.rs:20:1`).
// !! Do not change line count above without re-generating the .stderr file.
// !! Regenerate with: TRYBUILD=overwrite cargo test --features resolver-authoring
// !!   -p vsdd-hook-sdk --test resolver_types_test
// !!=========================================================================
// !! (blank lines removed from header to keep #[resolver] on line 20 per F-P2-003)
// !!=========================================================================
// !! Total lines above #[resolver]: 19 — do not add or remove lines above here.
// !!=========================================================================
#[resolver]
fn resolve_impl() -> String {
    "wrong return type".to_string()
}
