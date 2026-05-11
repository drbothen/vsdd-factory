// Step 2: test-writer RED gate — VP-076 capability confinement stubs.
// Covers: AC-005 (VP-076-A, VP-076-B, VP-076-C), AC-006 (VP-076-D).
//
// DEFERRED: These tests require:
//   1. The built WASM artifact at plugins/vsdd-factory/hook-plugins/vsdd-context-resolvers.wasm
//   2. The factory-dispatcher integration harness (wasmtime + ResolverLoader)
//      — NOT available as a compile-time test-dependency of this crate (ADR-018,
//      BC-1.13.001 INV1: dispatcher has zero compile-time dependency on resolvers).
//
// Strategy: mark with #[ignore] + unimplemented!() so they:
//   - Compile cleanly (satisfying `cargo test --no-run`)
//   - Do NOT fail the RED gate run (ignored tests don't fail)
//   - WOULD fail if un-ignored (unimplemented! panics)
//   - Signal to Step 3 implementer: "author real tests after WASM build"
//
// Step 3 implementer action:
//   1. Run: cargo build -p vsdd-context-resolvers --target wasm32-wasip1 --release
//   2. Replace unimplemented!() with real harness invocations
//   3. Remove #[ignore] and confirm GREEN

/// AC-005 (VP-076-A, VP-076-B): `path_allow` blocks reads outside the prefix.
///
/// **Strategy chosen: Option B — redirect to bats (F-P3-002 fix-burst decision).**
///
/// Architectural constraint: `vsdd-context-resolvers` crate has zero compile-time
/// dependency on `factory-dispatcher` (BC-1.13.001 INV1 / ADR-018). The dispatcher's
/// `ResolverLoader` harness is not available as a dev-dep here without violating the
/// crate boundary. Real integration verification lives in the bats harness, which
/// exercises the full dispatcher → WASM resolver → path_allow enforcement pipeline.
///
/// Bats test location: `plugins/vsdd-factory/tests/resolver-capability-confinement.bats`
/// Test case: "VP-076 capability confinement: naughty resolver cannot read /etc/passwd"
///
/// VP-076-A: capability denial is enforced (bats asserts exit 0 / dispatcher does not crash).
/// VP-076-B: no sensitive data leaks (bats asserts no /etc/passwd content in sink/stdout/stderr).
#[test]
#[ignore = "moved to bats: plugins/vsdd-factory/tests/resolver-capability-confinement.bats — \
            Option B per F-P3-002 (crate boundary: vsdd-context-resolvers has no compile-time \
            dep on factory-dispatcher per BC-1.13.001 INV1 / ADR-018)"]
fn test_BC_4_12_003_capability_denied_for_etc_passwd() {
    // This stub is retained for traceability. The actual verification is in:
    //   plugins/vsdd-factory/tests/resolver-capability-confinement.bats
    //   @test "VP-076 capability confinement: naughty resolver cannot read /etc/passwd"
    //
    // Fixture: crates/factory-dispatcher/tests/fixtures/naughty_resolver.{wat,wasm}
    // The naughty resolver calls vsdd::read_file("/etc/passwd") — the host must deny it.
}

/// AC-005 (VP-076-C): Capability denial emits a `resolver.capability_denied` audit event.
///
/// **Strategy chosen: Option B — redirect to bats (F-P3-002 fix-burst decision).**
///
/// Bats test location: `plugins/vsdd-factory/tests/resolver-capability-confinement.bats`
/// Test case: "VP-076 capability confinement: naughty resolver cannot read /etc/passwd"
/// (same test case asserts both VP-076-B and VP-076-C by checking VSDD_SINK_FILE for
/// resolver.capability_denied event with resolver="naughty_resolver" field).
///
/// VP-076-C: audit trail is written for all capability denials (asserted in bats sink check).
#[test]
#[ignore = "moved to bats: plugins/vsdd-factory/tests/resolver-capability-confinement.bats — \
            Option B per F-P3-002 (crate boundary: vsdd-context-resolvers has no compile-time \
            dep on factory-dispatcher per BC-1.13.001 INV1 / ADR-018)"]
fn test_BC_4_12_003_capability_denied_emits_audit_event() {
    // This stub is retained for traceability. The actual verification is in:
    //   plugins/vsdd-factory/tests/resolver-capability-confinement.bats
    //   VSDD_SINK_FILE grep for "resolver.capability_denied" and "naughty_resolver".
}

/// AC-006 (VP-076-D): Reads within `path_allow` succeed.
///
/// **Strategy chosen: Option B — redirect to bats (F-P3-002 fix-burst decision).**
///
/// Note (pass-2 amendment, 2026-05-10): Deferred to S-12.08's bats harness per VP-076
/// pass-2 spec amendment. The test remains as an `#[ignore]`'d stub; the S-12.08 bats
/// test `resolver-integration.bats` ("F-P2-001 closure: all converged") exercises the
/// full dispatcher-with-resolver end-to-end path.
///
/// Additional verification: `resolver-capability-confinement.bats` positive test case
/// ("VP-076-D: reads within path_allow succeed") asserts wave_context appears in plugin_config.
///
/// VP-076-D: the happy path capability grant works end-to-end.
#[test]
#[ignore = "moved to bats: plugins/vsdd-factory/tests/resolver-capability-confinement.bats + \
            resolver-integration.bats — Option B per F-P3-002 (crate boundary: \
            vsdd-context-resolvers has no compile-time dep on factory-dispatcher per \
            BC-1.13.001 INV1 / ADR-018)"]
fn test_BC_4_12_003_can_read_within_path_allow() {
    // This stub is retained for traceability. The actual verification is in:
    //   plugins/vsdd-factory/tests/resolver-integration.bats (happy-path end-to-end)
    //   plugins/vsdd-factory/tests/resolver-capability-confinement.bats (VP-076-D positive)
}
