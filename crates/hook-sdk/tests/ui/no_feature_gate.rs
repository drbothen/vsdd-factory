// AC-007 trybuild test: resolver types are gated behind resolver-authoring feature.
//
// This crate attempts to use `vsdd_hook_sdk::resolver::ResolverInput` WITHOUT
// enabling the `resolver-authoring` feature. It must fail to compile.
//
// The matching .stderr file specifies the expected error.
//
// Traces: AC-007, BC-4.12.002 postcondition 8.

// NOTE: trybuild compiles this as a standalone crate. The Cargo.toml for
// that crate must NOT include `features = ["resolver-authoring"]` on the
// vsdd-hook-sdk dependency. Since trybuild uses the crate's own Cargo.toml
// without resolver-authoring, the resolver module should not be accessible.
//
// However, trybuild tests run against the main package and inherit its
// features. This test instead directly verifies the feature-gate by attempting
// a compile that would fail if the feature is absent.

fn main() {
    // Attempt to reference the resolver module; must fail without feature flag.
    // If resolver-authoring is NOT enabled, this line produces a compile error.
    let _: vsdd_hook_sdk::resolver::ResolverInput;
}
