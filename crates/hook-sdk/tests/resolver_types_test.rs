//! Resolver SDK type and macro tests — S-12.05 Step 3 (Red Gate).
//!
//! All tests require `--features resolver-authoring`.
//!
//! Coverage map:
//!   AC-001 → test_BC_4_12_002_resolver_abi_version_is_1
//!   AC-002 → test_BC_4_12_002_resolver_input_serde_roundtrip
//!             test_BC_4_12_002_resolver_input_agent_type_none_serializes_as_null
//!             test_BC_4_12_002_resolver_input_canonical_json_shape
//!   AC-003 → test_BC_4_12_002_resolver_output_value_none_is_null
//!             test_BC_4_12_002_resolver_output_value_some_serializes_correctly
//!   AC-004 → test_BC_4_12_002_resolver_input_is_not_hook_payload (structural)
//!             test_BC_4_12_002_type_mismatch_compile_error (trybuild)
//!   AC-005 → test_BC_4_12_002_resolver_macro_generates_resolve_export (trybuild — FAILS in Red Gate)
//!   AC-006 → test_BC_4_12_002_resolver_macro_rejects_wrong_signature (trybuild — FAILS in Red Gate)
//!   AC-007 → test_BC_4_12_002_resolver_authoring_feature_gates_types (structural: lib.rs source scan)
//!             Note: trybuild negative-compile for feature-gating is unreliable because trybuild
//!             inherits the parent crate's feature flags. The authoritative check is the Cargo.toml
//!             feature definition + lib.rs cfg gate verified by source scan in this test.
//!   AC-008 → test_BC_4_12_002_abi_versions_are_independently_defined
//!   AC-009 → test_BC_4_12_002_hook_payload_and_hook_result_surfaces_unchanged
//!   AC-010 → prop_BC_4_12_002_resolver_serde_roundtrip_deterministic
//!             (VP-075)
//!
//! Red Gate state: AC-005 and AC-006 trybuild tests FAIL because the
//! #[resolver] macro body is `todo!()` — compilation of crates using
//! the macro panics instead of generating the `resolve()` export.

#[cfg(feature = "resolver-authoring")]
mod tests {
    use serde_json::{Value, json};
    use vsdd_hook_sdk::resolver::{RESOLVER_ABI_VERSION, ResolverInput, ResolverOutput};

    // ── AC-001: RESOLVER_ABI_VERSION constant ─────────────────────────────────

    /// BC-4.12.002 PC4: RESOLVER_ABI_VERSION must equal 1.
    ///
    /// Traces: AC-001, BC-4.12.002 postcondition 4.
    #[test]
    fn test_BC_4_12_002_resolver_abi_version_is_1() {
        assert_eq!(
            RESOLVER_ABI_VERSION, 1u32,
            "RESOLVER_ABI_VERSION must be 1 (BC-4.12.002 PC4, AC-001)"
        );
    }

    // ── AC-002: ResolverInput type shape and serde ────────────────────────────

    /// BC-4.12.002 PC2: ResolverInput serde round-trip preserves all fields.
    ///
    /// Canonical test vector from BC-4.12.002:
    ///   event_type="SubagentStop", hook_event_name="validate-...",
    ///   agent_type=Some("wave-gate"), project_dir="/repo", plugin_config={}
    ///
    /// Traces: AC-002, BC-4.12.002 postcondition 2.
    #[test]
    fn test_BC_4_12_002_resolver_input_serde_roundtrip() {
        let input = ResolverInput {
            event_type: "SubagentStop".into(),
            hook_event_name: "validate-per-story-adversary-convergence".into(),
            agent_type: Some("wave-gate".into()),
            project_dir: "/repo".into(),
            plugin_config: json!({}),
        };

        let json = serde_json::to_string(&input).expect("ResolverInput must serialize");
        let round: ResolverInput =
            serde_json::from_str(&json).expect("ResolverInput must deserialize from its own JSON");

        assert_eq!(
            round, input,
            "ResolverInput serde round-trip must preserve all fields (AC-002)"
        );
    }

    /// BC-4.12.002 PC2 + EC-001: agent_type: None serializes as JSON null,
    /// NOT as an omitted field.
    ///
    /// Traces: AC-002, BC-4.12.002 postcondition 2.
    #[test]
    fn test_BC_4_12_002_resolver_input_agent_type_none_serializes_as_null() {
        let input = ResolverInput {
            event_type: "PreToolUse".into(),
            hook_event_name: "my-hook".into(),
            agent_type: None,
            project_dir: "/project".into(),
            plugin_config: json!(null),
        };

        let json_str = serde_json::to_string(&input).expect("must serialize");
        let parsed: Value = serde_json::from_str(&json_str).expect("must produce valid JSON");

        assert!(
            parsed.get("agent_type").is_some(),
            "agent_type must be present in JSON (as null), not omitted (BC-4.12.002 PC2)"
        );
        assert!(
            parsed["agent_type"].is_null(),
            "agent_type: None must serialize as JSON null, not omitted (AC-002)"
        );
    }

    /// BC-4.12.002 PC2: All five required field names are present in serialized JSON.
    ///
    /// The field names are EXACT per spec — do not rename, omit, or add.
    /// Traces: AC-002, BC-4.12.002 postcondition 2.
    #[test]
    fn test_BC_4_12_002_resolver_input_canonical_json_shape() {
        let input = ResolverInput {
            event_type: "PostToolUse".into(),
            hook_event_name: "my-hook".into(),
            agent_type: None,
            project_dir: "/abs/path".into(),
            plugin_config: json!({"key": "value"}),
        };

        let json_str = serde_json::to_string(&input).expect("must serialize");
        let parsed: Value = serde_json::from_str(&json_str).expect("must be valid JSON");

        // All five field names must be present exactly as specified (BC-4.12.002 PC2)
        for field in &[
            "event_type",
            "hook_event_name",
            "agent_type",
            "project_dir",
            "plugin_config",
        ] {
            assert!(
                parsed.get(*field).is_some(),
                "ResolverInput JSON must contain field '{}' (BC-4.12.002 PC2, AC-002)",
                field
            );
        }

        // No extra fields (the spec says "no additions")
        let obj = parsed.as_object().expect("must be a JSON object");
        assert_eq!(
            obj.len(),
            5,
            "ResolverInput JSON must have exactly 5 fields (BC-4.12.002 PC2, AC-002)"
        );
    }

    // ── AC-003: ResolverOutput type shape and serde ───────────────────────────

    /// BC-4.12.002 PC3: ResolverOutput with value=None serializes as
    /// {"key":"foo","value":null} — NOT {"key":"foo"} (value must not be omitted).
    ///
    /// Canonical test vector: ResolverOutput { key: "foo", value: None }
    ///   → '{"key":"foo","value":null}'
    ///
    /// Traces: AC-003, BC-4.12.002 postcondition 3, EC-001.
    #[test]
    fn test_BC_4_12_002_resolver_output_value_none_is_null() {
        let output = ResolverOutput {
            key: "foo".into(),
            value: None,
        };

        let json_str = serde_json::to_string(&output).expect("must serialize");
        // Canonical exact match from story AC-003
        assert_eq!(
            json_str, r#"{"key":"foo","value":null}"#,
            "ResolverOutput {{key,value:None}} must serialize as {{\"key\":...,\"value\":null}} (AC-003)"
        );
    }

    /// BC-4.12.002 PC3: ResolverOutput with value=Some serializes the inner
    /// Value as the field value.
    ///
    /// Canonical test vector: ResolverOutput { key: "foo", value: Some({"x":1}) }
    ///   → '{"key":"foo","value":{"x":1}}'
    ///
    /// Traces: AC-003, BC-4.12.002 postcondition 3.
    #[test]
    fn test_BC_4_12_002_resolver_output_value_some_serializes_correctly() {
        let output = ResolverOutput {
            key: "foo".into(),
            value: Some(json!({"x": 1})),
        };

        let json_str = serde_json::to_string(&output).expect("must serialize");
        // Canonical exact match from story AC-003
        assert_eq!(
            json_str, r#"{"key":"foo","value":{"x":1}}"#,
            "ResolverOutput with Some value must serialize the nested JSON (AC-003)"
        );
    }

    // ── AC-004: ResolverInput is structurally distinct from HookPayload ────────

    /// BC-4.12.002 INV1: ResolverInput and HookPayload are separate structs.
    /// There is no From/Into/Deref relationship between them.
    ///
    /// This structural test verifies they are not type aliases and have
    /// independent field sets. The negative compile test (type mismatch)
    /// is in tests/ui/type_mismatch.rs via trybuild.
    ///
    /// Traces: AC-004, BC-4.12.002 invariant 1.
    #[test]
    fn test_BC_4_12_002_resolver_input_is_not_hook_payload() {
        use vsdd_hook_sdk::HookPayload;

        // Verify ResolverInput has its own field set (not the same as HookPayload)
        let resolver_input = ResolverInput {
            event_type: "SubagentStop".into(),
            hook_event_name: "hook-name".into(),
            agent_type: None,
            project_dir: "/repo".into(),
            plugin_config: json!({}),
        };

        // ResolverInput does NOT have tool_name, session_id, dispatcher_trace_id,
        // tool_input, tool_response — these are HookPayload-specific fields.
        // The following code must NOT compile if we accidentally give ResolverInput
        // those fields. We verify the distinct field sets by constructing both.
        let _hook_payload_json = r#"{
            "event_name": "SubagentStop",
            "tool_name": "",
            "session_id": "s",
            "dispatcher_trace_id": "t"
        }"#;

        let hook_payload: HookPayload =
            serde_json::from_str(_hook_payload_json).expect("HookPayload must deserialize");

        // HookPayload has event_name; ResolverInput has event_type — distinct fields.
        // The types are structurally independent per BC-4.12.002 INV1.
        assert_ne!(
            std::mem::size_of::<ResolverInput>(),
            std::mem::size_of::<HookPayload>(),
            "ResolverInput and HookPayload must not be the same type (AC-004)"
        );

        // Verify the distinct module paths are stable
        let _ = resolver_input;
        let _ = hook_payload;
    }

    /// BC-4.12.002 INV1: The Rust compiler rejects assigning a `HookPayload` where
    /// a `ResolverInput` is expected — no implicit conversion exists between them.
    ///
    /// This trybuild compile-fail test is the authoritative falsifiable witness for
    /// INV1's "no implicit conversions" guarantee. A future `From<HookPayload> for
    /// ResolverInput` impl would be caught here at compile time.
    ///
    /// Traces: AC-004, BC-4.12.002 invariant 1.
    #[test]
    fn test_BC_4_12_002_type_mismatch_compile_error() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/ui/type_mismatch.rs");
    }

    // ── AC-005 and AC-006: #[resolver] macro (trybuild — FAILS in Red Gate) ───

    /// BC-4.12.002 PC5: #[resolver] macro generates a valid `resolve()` export
    /// when applied to `fn resolve_impl(input: ResolverInput) -> ResolverOutput`.
    ///
    /// RED GATE: This test FAILS because the macro body is `todo!()`.
    /// The trybuild compilation panics instead of generating the export.
    /// The test will pass GREEN once the implementer completes the macro.
    ///
    /// Traces: AC-005, BC-4.12.002 postcondition 5.
    #[test]
    fn test_BC_4_12_002_resolver_macro_generates_resolve_export() {
        let t = trybuild::TestCases::new();
        // This .rs file applies #[resolver] to a valid resolve_impl signature.
        // Must compile successfully — fails RED because macro is todo!().
        t.pass("tests/ui/valid_resolver.rs");
    }

    /// BC-4.12.002 PC5: #[resolver] macro emits a compile-time error with a
    /// message containing "resolve_impl" and "ResolverOutput" when applied to
    /// a function with the wrong signature.
    ///
    /// Traces: AC-006, BC-4.12.002 postcondition 5.
    #[test]
    fn test_BC_4_12_002_resolver_macro_rejects_wrong_signature() {
        let t = trybuild::TestCases::new();
        // This .rs file applies #[resolver] to a wrong signature (fn() -> String).
        // Must fail with a message referencing the expected signature.
        t.compile_fail("tests/ui/wrong_sig.rs");
    }

    /// BC-4.12.002 PC5 + F-P2-006: #[resolver] must reject async fn with a
    /// clear diagnostic. The WASM resolver entrypoint is synchronous; async
    /// is not supported (mirrors #[hook] asyncness check).
    ///
    /// Traces: BC-4.12.002 PC5, F-P2-006.
    #[test]
    fn test_BC_4_12_002_resolver_macro_rejects_async_fn() {
        let t = trybuild::TestCases::new();
        // Applies #[resolver] to `async fn resolve_impl(...)`.
        // Must fail with a message about async not being supported.
        t.compile_fail("tests/ui/async_resolver.rs");
    }

    // ── AC-007: resolver-authoring feature flag (structural + Cargo.toml check) ─

    /// BC-4.12.002 PC8: A crate without `resolver-authoring` feature MUST NOT
    /// have access to ResolverInput, ResolverOutput, or RESOLVER_ABI_VERSION.
    ///
    /// Structural verification at two levels:
    /// 1. This test MODULE is gated `#[cfg(feature = "resolver-authoring")]` —
    ///    the module does not exist without the feature, verifying the gate.
    /// 2. The resolver module in lib.rs is `#[cfg(feature = "resolver-authoring")]` —
    ///    verified by reading the lib.rs source.
    ///
    /// Note: trybuild negative compile tests for feature gating are unreliable
    /// because trybuild inherits the parent's feature flags. The authoritative
    /// check is the Cargo.toml feature definition and lib.rs cfg gate.
    ///
    /// Traces: AC-007, BC-4.12.002 postcondition 8.
    #[test]
    fn test_BC_4_12_002_resolver_authoring_feature_gates_types() {
        // Verify the resolver module is accessible via the feature-gated path.
        // This import succeeds only because the outer #[cfg(feature)] is active.
        let version: u32 = vsdd_hook_sdk::resolver::RESOLVER_ABI_VERSION;
        assert_eq!(
            version, 1u32,
            "RESOLVER_ABI_VERSION must be accessible only under resolver-authoring feature (AC-007)"
        );

        // Verify that the lib.rs source gates the resolver module under the feature.
        // Read lib.rs and confirm the #[cfg(feature = "resolver-authoring")] annotation.
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR must be set during cargo test");
        let lib_rs_path = std::path::Path::new(&manifest_dir)
            .join("src")
            .join("lib.rs");
        let lib_rs_content = std::fs::read_to_string(&lib_rs_path)
            .unwrap_or_else(|e| panic!("failed to read {}: {}", lib_rs_path.display(), e));

        assert!(
            lib_rs_content.contains(r#"cfg(feature = "resolver-authoring")"#),
            "crates/hook-sdk/src/lib.rs must gate the resolver module under \
             #[cfg(feature = \"resolver-authoring\")] (AC-007, BC-4.12.002 PC8)"
        );
        assert!(
            lib_rs_content.contains("pub mod resolver"),
            "crates/hook-sdk/src/lib.rs must contain `pub mod resolver` \
             under the resolver-authoring feature gate (AC-007)"
        );
    }

    // ── AC-008: RESOLVER_ABI_VERSION is independent from HOST_ABI_VERSION ─────

    /// BC-4.12.002 INV2: RESOLVER_ABI_VERSION and HOST_ABI_VERSION are defined
    /// in separate source locations (resolver.rs vs lib.rs) and evolve
    /// independently. A bump to one does NOT require a bump to the other.
    ///
    /// This test verifies:
    /// 1. Both constants are accessible from their public paths.
    /// 2. `RESOLVER_ABI_VERSION` is declared in `resolver.rs` (not lib.rs).
    /// 3. `HOST_ABI_VERSION` is declared in `lib.rs` (not resolver.rs).
    ///
    /// The structural source-location check enforces the INV2 independence
    /// without pinning either constant's value — a future bump to either
    /// version will not break this test.
    ///
    /// Traces: AC-008, BC-4.12.002 invariant 2.
    #[test]
    fn test_BC_4_12_002_abi_versions_are_independently_defined() {
        // Both constants must be accessible (BC-4.12.002 INV2).
        // We do NOT assert their values: both are currently 1, but INV2 explicitly
        // states that bumps to one version do NOT require bumps to the other.
        // Pinning values would reintroduce the coupling INV2 forbids.
        let _resolver_version: u32 = vsdd_hook_sdk::RESOLVER_ABI_VERSION;
        let _host_version: u32 = vsdd_hook_sdk::HOST_ABI_VERSION;

        // RESOLVER_ABI_VERSION must also be reachable via the module path.
        let _from_module: u32 = vsdd_hook_sdk::resolver::RESOLVER_ABI_VERSION;

        // Structural source-location check (BC-4.12.002 INV2 authoritative check):
        // RESOLVER_ABI_VERSION must be declared in resolver.rs, NOT lib.rs.
        // HOST_ABI_VERSION must be declared in lib.rs, NOT resolver.rs.
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR must be set during cargo test");
        let src = std::path::Path::new(&manifest_dir).join("src");

        let resolver_rs = std::fs::read_to_string(src.join("resolver.rs"))
            .unwrap_or_else(|e| panic!("failed to read resolver.rs: {}", e));
        let lib_rs = std::fs::read_to_string(src.join("lib.rs"))
            .unwrap_or_else(|e| panic!("failed to read lib.rs: {}", e));

        // Check that RESOLVER_ABI_VERSION is *declared* (as a const) in resolver.rs.
        assert!(
            resolver_rs.contains("pub const RESOLVER_ABI_VERSION"),
            "RESOLVER_ABI_VERSION must be declared (pub const) in resolver.rs (BC-4.12.002 INV2, AC-008)"
        );
        // Check that HOST_ABI_VERSION is *not declared* in resolver.rs (may appear in doc-comments).
        assert!(
            !resolver_rs.contains("pub const HOST_ABI_VERSION"),
            "HOST_ABI_VERSION must NOT be declared in resolver.rs — it belongs in lib.rs (AC-008)"
        );
        // Check that HOST_ABI_VERSION is *declared* (as a const) in lib.rs.
        assert!(
            lib_rs.contains("pub const HOST_ABI_VERSION"),
            "HOST_ABI_VERSION must be declared (pub const) in lib.rs (BC-4.12.002 INV2, AC-008)"
        );
        // Check that RESOLVER_ABI_VERSION is *not declared* in lib.rs (it's re-exported via pub use resolver::*).
        assert!(
            !lib_rs.contains("pub const RESOLVER_ABI_VERSION"),
            "RESOLVER_ABI_VERSION must NOT be declared in lib.rs — declare it in resolver.rs only (AC-008)"
        );
    }

    // ── AC-009: Existing Hook surface is unchanged (regression) ───────────────

    /// BC-4.12.002 postconditions 1-9 + invariants: Adding the resolver-authoring
    /// surface must not modify or break the existing Hook trait surface.
    ///
    /// HookPayload, HookResult, HOST_ABI_VERSION, and the #[hook] macro must
    /// remain accessible and functionally correct.
    ///
    /// Traces: AC-009, S-12.05 regression requirement.
    #[test]
    fn test_BC_4_12_002_hook_payload_and_hook_result_surfaces_unchanged() {
        use vsdd_hook_sdk::{HOST_ABI_VERSION, HookPayload, HookResult};

        // HOST_ABI_VERSION still == 1
        assert_eq!(
            HOST_ABI_VERSION, 1u32,
            "HOST_ABI_VERSION must remain 1 after resolver-authoring additions (AC-009)"
        );

        // HookPayload deserializes correctly (existing field set stable)
        let payload: HookPayload = serde_json::from_str(
            r#"{"event_name":"PreToolUse","tool_name":"Bash","session_id":"s","dispatcher_trace_id":"t"}"#
        ).expect("HookPayload deserialize must succeed (regression AC-009)");
        assert_eq!(payload.event_name, "PreToolUse");

        // HookResult Continue → exit code 0 (unchanged behavior)
        assert_eq!(
            HookResult::Continue.exit_code(),
            0,
            "HookResult::Continue exit code must remain 0 (AC-009)"
        );

        // HookResult Block → exit code 2 (unchanged behavior)
        assert_eq!(
            HookResult::block("reason").exit_code(),
            2,
            "HookResult::Block exit code must remain 2 (AC-009)"
        );
    }
}

// ── AC-010: Proptest serde round-trip determinism (VP-075) ────────────────────
//
// Must be outside the `#[cfg(feature = "resolver-authoring")]` mod only if
// we want unconditional compilation, but since it uses resolver types it
// MUST be inside the feature gate. Placed in a separate module to avoid
// polluting the proptest import namespace.

#[cfg(feature = "resolver-authoring")]
mod proptest_tests {
    use proptest::prelude::*;
    use serde_json::{Value, json};
    use vsdd_hook_sdk::resolver::{ResolverInput, ResolverOutput};

    // Arbitrary strategy for serde_json::Value (limited depth for speed)
    fn arb_json_value() -> impl Strategy<Value = Value> {
        prop_oneof![
            Just(Value::Null),
            any::<bool>().prop_map(Value::Bool),
            any::<i64>().prop_map(|n| json!(n)),
            ".*".prop_map(Value::String),
        ]
    }

    // Arbitrary strategy for Option<String>
    fn arb_opt_string() -> impl Strategy<Value = Option<String>> {
        prop_oneof![Just(None), ".*".prop_map(Some),]
    }

    // Arbitrary strategy for ResolverInput
    fn arb_resolver_input() -> impl Strategy<Value = ResolverInput> {
        (
            ".*",             // event_type
            ".*",             // hook_event_name
            arb_opt_string(), // agent_type
            ".*",             // project_dir
            arb_json_value(), // plugin_config
        )
            .prop_map(
                |(event_type, hook_event_name, agent_type, project_dir, plugin_config)| {
                    ResolverInput {
                        event_type,
                        hook_event_name,
                        agent_type,
                        project_dir,
                        plugin_config,
                    }
                },
            )
    }

    // Arbitrary strategy for ResolverOutput
    //
    // AC-010 spec-gap resolution (Option A): `Some(Value::Null)` is excluded from
    // the strategy. Per BC-4.12.002 EC-001: "value: null means key is absent" — the
    // dispatcher treats `ResolverOutput { value: None }` and `{ value: Some(Null) }`
    // identically (neither writes the key to plugin_config). Standard serde Option
    // semantics serialize both as `"value":null` and deserialize the JSON null back
    // to `None`, so `Some(Null)` is a degenerate input that does not survive a
    // serde round-trip unchanged. Filtering it avoids a false negative while
    // preserving full coverage of all semantically-distinct ResolverOutput values.
    fn arb_resolver_output() -> impl Strategy<Value = ResolverOutput> {
        (
            ".*",
            prop_oneof![
                Just(None),
                // Exclude Value::Null from Some variants: Some(Null) serializes
                // to JSON null, which deserializes back to None, not Some(Null).
                arb_json_value()
                    .prop_filter("Some(Null) excluded — degenerate per EC-001", |v| !v
                        .is_null())
                    .prop_map(Some),
            ],
        )
            .prop_map(|(key, value)| ResolverOutput { key, value })
    }

    // VP-075: ResolverInput serde round-trip is deterministic.
    // serialize(input) followed by deserialize yields an equal struct.
    // 100 proptest trials.
    // Traces: AC-010, VP-075.
    proptest! {
        #![proptest_config(proptest::test_runner::Config::with_cases(100))]

        #[test]
        fn prop_BC_4_12_002_resolver_input_serde_roundtrip_deterministic(
            input in arb_resolver_input()
        ) {
            let json = serde_json::to_string(&input)
                .expect("ResolverInput must serialize");
            let round: ResolverInput = serde_json::from_str(&json)
                .expect("ResolverInput must deserialize from its own JSON");
            prop_assert_eq!(
                round, input,
                "ResolverInput serde round-trip must be deterministic (VP-075, AC-010)"
            );
        }

        #[test]
        fn prop_BC_4_12_002_resolver_output_serde_roundtrip_deterministic(
            output in arb_resolver_output()
        ) {
            let json = serde_json::to_string(&output)
                .expect("ResolverOutput must serialize");
            let round: ResolverOutput = serde_json::from_str(&json)
                .expect("ResolverOutput must deserialize from its own JSON");
            prop_assert_eq!(
                round, output,
                "ResolverOutput serde round-trip must be deterministic (VP-075, AC-010)"
            );
        }
    }
}
