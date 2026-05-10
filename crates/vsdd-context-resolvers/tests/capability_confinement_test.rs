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

/// AC-005 (VP-076-A, VP-076-B): `path_allow = [".factory/"]` blocks reads outside the prefix.
///
/// Integration test: run resolver with a read attempt for `/etc/passwd`;
/// assert `CapabilityDenied` is received by the resolver, which must then
/// return `ResolverOutput { key: "wave_context", value: None }` (not a trap).
/// No `/etc/passwd` content must appear in `plugin_config`.
///
/// VP-076-A: capability denial is enforced.
/// VP-076-B: no sensitive data leaks through a denied read.
#[test]
#[ignore = "Requires built WASM artifact + factory-dispatcher integration harness \
            — author in Step 3 after Step 4 build (cargo build --target wasm32-wasip1)"]
fn test_BC_4_12_003_capability_denied_for_etc_passwd() {
    // AC-005 + VP-076-A/B
    unimplemented!(
        "Step 3: use the factory-dispatcher ResolverLoader test harness to run the WASM \
         artifact with a mock path_allow = [\".factory/\"]. Inject an attempt to read \
         /etc/passwd. Assert HostError::CapabilityDenied is the host response and \
         ResolverOutput {{ key: \"wave_context\", value: None }}. Assert no /etc/passwd bytes appear in plugin_config."
    )
}

/// AC-005 (VP-076-C): Capability denial emits a `resolver.capability_denied` audit event.
///
/// Integration test: same setup as AC-005 VP-076-A/B; additionally assert that
/// the dispatcher emits a `resolver.capability_denied` structured event with
/// fields: `resolver = "wave_context"`, `denied_path = "/etc/passwd"`.
///
/// VP-076-C: audit trail is written for all capability denials.
#[test]
#[ignore = "Requires built WASM artifact + factory-dispatcher integration harness \
            — author in Step 3 after Step 4 build (cargo build --target wasm32-wasip1)"]
fn test_BC_4_12_003_capability_denied_emits_audit_event() {
    // AC-005 + VP-076-C
    unimplemented!(
        "Step 3: after running the resolver with a denied read, inspect the dispatcher's \
         event sink for a 'resolver.capability_denied' event. Assert fields: \
         resolver = 'wave_context', denied_path = '/etc/passwd'."
    )
}

/// AC-006 (VP-076-D): Reads within `path_allow = [".factory/"]` succeed.
///
/// Positive capability test: create a temp project with `.factory/wave-state.yaml`
/// and `.factory/STATE.md`, run WaveContextResolver with `project_dir = <temp_project>`,
/// assert `wave_context` key is present in `plugin_config` with a non-null value.
///
/// Note (pass-2 amendment, 2026-05-10): This test is deferred to S-12.08's bats harness
/// per VP-076 pass-2 spec amendment. The test remains as an `#[ignore]`'d stub here;
/// the S-12.08 bats test exercises the full dispatcher-with-resolver end-to-end path.
/// See VP-076 §Proof Harness Locations for rationale.
///
/// VP-076-D: the happy path capability grant works end-to-end.
#[test]
#[ignore = "Requires built WASM artifact + factory-dispatcher integration harness \
            — author in Step 3 after Step 4 build (cargo build --target wasm32-wasip1)"]
fn test_BC_4_12_003_can_read_within_path_allow() {
    // AC-006 + VP-076-D
    unimplemented!(
        "Step 3: create a temp dir with .factory/wave-state.yaml (valid YAML fixture) \
         and .factory/STATE.md. Run the WASM resolver via the dispatcher harness with \
         project_dir pointing to the temp dir. Assert ResolverOutput.value is Some and \
         the injected plugin_config[\"wave_context\"] contains stories, wave_id, cycle_id."
    )
}
