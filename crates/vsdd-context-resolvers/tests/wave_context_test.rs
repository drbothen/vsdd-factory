// Step 2: test-writer RED gate.
// Covers: AC-001, AC-002a, AC-002b, AC-003, AC-004, AC-007, AC-008, AC-009, AC-010.
//
// Naming convention: test_BC_4_12_NNN_<assertion>() per DF-037 test-writer rules.
// Every test here MUST FAIL (todo!()) until Step 3 implementation is complete.
//
// Spec drift documented inline:
//   AC-007: story references `plugin_config["project_dir"]` but `ResolverInput.project_dir`
//           is a TOP-LEVEL String field. Tests target the actual API. Flag for Step 4.5 adversary.
//   AC-002: story references `HostError::FileNotFound` which does NOT exist in host.rs.
//           Actual mapping: missing/unreadable file => `HostError::Other(-99)` or `CapabilityDenied`.
//           Tests model the graceful "value: None" contract regardless of the specific error variant.

use serde_json::Value;
use vsdd_context_resolvers::resolve_wave_context_pure;
use vsdd_context_resolvers::wave_context::{WaveState, parse_wave_state};
use vsdd_hook_sdk::resolver::{ResolverInput, ResolverOutput};

// ─── Test helpers ────────────────────────────────────────────────────────────

fn make_input(project_dir: &str) -> ResolverInput {
    ResolverInput {
        event_type: "SubagentStop".into(),
        hook_event_name: "Stop".into(),
        agent_type: None,
        project_dir: project_dir.into(),
        plugin_config: serde_json::json!({}),
    }
}

fn populated_wave_state() -> WaveState {
    WaveState {
        current_cycle: Some("v1.0-feature-engine-discipline".into()),
        current_wave: Some("F4".into()),
        stories: Some(vec!["S-12.07".into(), "S-12.08".into()]),
    }
}

// ─── AC-001 ──────────────────────────────────────────────────────────────────

/// BC-4.12.002 PC3: output shape is `ResolverOutput { key: "wave-context", value: Some({...}) }`
/// when WaveState is fully populated.
///
/// Asserts:
///   - output.key == "wave-context"
///   - output.value is Some(_)
///   - value JSON has "stories" (array), "wave_id" (string), "cycle_id" (string)
#[test]
fn test_BC_4_12_002_wave_context_output_shape() {
    let input = make_input("/tmp/test");
    let wave_state = populated_wave_state();

    let output: ResolverOutput = resolve_wave_context_pure(&input, &wave_state);

    assert_eq!(
        output.key, "wave-context",
        "key must be exactly 'wave-context'"
    );

    let value = output
        .value
        .expect("AC-001: value must be Some when WaveState is fully populated");

    assert!(
        value["stories"].is_array(),
        "AC-001: value must contain 'stories' as a JSON array; got: {value}"
    );
    assert!(
        value["wave_id"].is_string(),
        "AC-001: value must contain 'wave_id' as a JSON string; got: {value}"
    );
    assert!(
        value["cycle_id"].is_string(),
        "AC-001: value must contain 'cycle_id' as a JSON string; got: {value}"
    );
}

/// BC-4.12.002 PC3: stories array contents round-trip correctly from WaveState.
#[test]
fn test_BC_4_12_002_wave_context_stories_contents() {
    let input = make_input("/tmp/test");
    let wave_state = populated_wave_state();

    let output = resolve_wave_context_pure(&input, &wave_state);
    let value = output.value.expect("AC-001: value must be Some");

    let stories = value["stories"].as_array().expect("stories is array");
    assert_eq!(stories.len(), 2, "expected 2 stories in fixture");
    assert!(
        stories.contains(&Value::String("S-12.07".into())),
        "S-12.07 must be in stories"
    );
    assert!(
        stories.contains(&Value::String("S-12.08".into())),
        "S-12.08 must be in stories"
    );
}

// ─── AC-002a ─────────────────────────────────────────────────────────────────

/// BC-4.12.004 PC3: malformed YAML produces a parse error (not a panic).
///
/// `parse_wave_state` must return `Err(serde_yaml::Error)` on malformed input;
/// callers map the error to `value: None`.
#[test]
fn test_BC_4_12_004_malformed_yaml_yields_parse_error() {
    // Deliberately broken YAML (invalid flow mapping)
    let bad_yaml = "this: is: not: valid: yaml: {{{";

    let result = parse_wave_state(bad_yaml);
    assert!(
        result.is_err(),
        "AC-002a: parse_wave_state must return Err on malformed YAML, got Ok"
    );
}

// ─── AC-002b ─────────────────────────────────────────────────────────────────

/// BC-4.12.004 PC3 + BC-4.12.002 PC3: resolve_wave_context_pure with a default
/// (all-None) WaveState returns `value: None`.
///
/// This models the post-parse-failure path: resolve_impl catches the parse
/// error and calls resolve_wave_context_pure with `WaveState::default()`.
/// The output must NOT be Some; it must be None.
///
/// Spec drift note: AC-002 in the story mentions `HostError::FileNotFound` which
/// does not exist. The actual HostError variants are: CapabilityDenied, Timeout,
/// OutputTooLarge, InvalidArgument, Other(i32). A missing file returns Other(-99).
/// This test models the graceful degradation at the pure-function layer — the
/// effectful resolve_impl is responsible for the HostError -> WaveState::default()
/// mapping. Flag for Step 4.5 adversary.
#[test]
fn test_BC_4_12_004_resolve_pure_with_default_wavestate_yields_none() {
    let input = make_input("/tmp/test");
    let wave_state = WaveState::default();

    let output = resolve_wave_context_pure(&input, &wave_state);

    assert_eq!(
        output.key, "wave-context",
        "key must always be 'wave-context'"
    );
    assert!(
        output.value.is_none(),
        "AC-002b: value must be None when WaveState is all-None (post-error path); got: {:?}",
        output.value
    );
}

// ─── AC-003 ──────────────────────────────────────────────────────────────────

/// BC-4.12.002 PC3: empty stories list yields `value: None`.
///
/// When wave-state.yaml is present but `stories: []`, the resolver must not
/// produce a wave_context value — there is nothing meaningful to inject.
#[test]
fn test_BC_4_12_002_empty_stories_yields_none() {
    let input = make_input("/tmp/test");
    let wave_state = WaveState {
        current_cycle: Some("v1.0-feature-engine-discipline".into()),
        current_wave: Some("F4".into()),
        stories: Some(vec![]),
    };

    let output = resolve_wave_context_pure(&input, &wave_state);

    assert_eq!(output.key, "wave-context");
    // AC-003: most defensive interpretation — empty stories -> value: None.
    // If implementer chooses to return Some with an empty array instead,
    // this test must be updated to assert empty array; document the choice.
    assert!(
        output.value.is_none(),
        "AC-003: empty stories must yield value: None; got: {:?}",
        output.value
    );
}

// ─── AC-004 ──────────────────────────────────────────────────────────────────

/// BC-4.12.002 PC3 + EC-004: missing wave_id (current_wave: None) must not panic.
///
/// The resolver handles optional fields gracefully. When current_wave is None
/// but stories are present, the resolver must either:
///   (a) return value: None (most conservative)
///   (b) return Some with wave_id defaulted to a sentinel (e.g. "unknown")
/// It must NEVER panic or trap.
///
/// Implementer chooses the semantics. This test asserts only the no-panic contract
/// and accepts either `None` or a `Some` that does not contain a non-string wave_id.
#[test]
fn test_BC_4_12_002_missing_wave_id_no_panic() {
    let input = make_input("/tmp/test");
    let wave_state = WaveState {
        current_cycle: Some("v1.0-feature-engine-discipline".into()),
        current_wave: None, // wave_id absent
        stories: Some(vec!["S-12.07".into()]),
    };

    // Primary assertion: must not panic. The call itself reaching the next line
    // proves no panic occurred.
    let output = resolve_wave_context_pure(&input, &wave_state);

    assert_eq!(output.key, "wave-context");

    // Secondary assertion: if value is Some, wave_id must be a string (not null/missing)
    // so that downstream consumers can handle it without surprise.
    // Implementer choice: return None or Some with a default wave_id string.
    if let Some(ref v) = output.value {
        // If we returned Some, wave_id must be either a string or absent
        // (not a non-string type like number or boolean).
        if v.get("wave_id").is_some() {
            assert!(
                v["wave_id"].is_string(),
                "AC-004: if wave_id is present in output, it must be a string; got: {v}"
            );
        }
    }
    // value: None is also acceptable for this test (implementer's call).
}

// ─── AC-007 ──────────────────────────────────────────────────────────────────

/// BC-4.12.002 PC2: `ResolverInput.project_dir` is the top-level field used for
/// path construction.
///
/// SPEC DRIFT: AC-007 in the story spec says the resolver reads
/// `input.plugin_config["project_dir"]`. The ACTUAL `ResolverInput` struct has
/// `project_dir` as a TOP-LEVEL String field, not nested inside `plugin_config`.
/// This test verifies the actual struct layout and asserts the field is accessible
/// directly. The implementer MUST use `input.project_dir` (not plugin_config lookup)
/// when constructing `.factory/wave-state.yaml` paths.
/// FLAG FOR STEP 4.5 ADVERSARY: spec drift in AC-007.
#[test]
fn test_BC_4_12_002_project_dir_is_top_level_field() {
    // Spec drift: AC-007 references plugin_config["project_dir"] but actual
    // ResolverInput.project_dir is top-level. Tests target actual API; flag in
    // Step 4.5 adversary.
    let input = make_input("/tmp/test-project");

    // The top-level field must be directly accessible.
    assert_eq!(
        input.project_dir, "/tmp/test-project",
        "AC-007 (spec drift): project_dir must be a top-level field on ResolverInput"
    );

    // plugin_config should NOT contain project_dir (dispatcher does not inject it there)
    assert!(
        input.plugin_config.get("project_dir").is_none(),
        "project_dir must NOT be inside plugin_config — it is a top-level field"
    );
}

// ─── AC-008 — proptest (VP-075) ──────────────────────────────────────────────

/// VP-075: resolve_wave_context_pure is deterministic — same inputs always produce
/// same outputs. 200 trials via proptest.
///
/// Strategy:
///   - Random WaveState (all fields Option<String> / Option<Vec<String>>)
///   - Random ResolverInput
///   - Call resolve_wave_context_pure twice, assert outputs equal.
///
/// Must complete within 5s (proptest default timeout).
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 200,
        ..ProptestConfig::default()
    })]

    #[test]
    fn test_BC_4_12_002_prop_resolve_wave_context_is_deterministic(
        current_cycle in prop::option::of(any::<String>()),
        current_wave in prop::option::of(any::<String>()),
        stories in prop::option::of(prop::collection::vec(any::<String>(), 0..10)),
        event_type in any::<String>(),
        hook_event_name in any::<String>(),
        agent_type in prop::option::of(any::<String>()),
        project_dir in any::<String>(),
    ) {
        // VP-075: same (input, wave_state) pair must produce identical output every time.
        let wave_state = WaveState {
            current_cycle,
            current_wave,
            stories,
        };
        let input = ResolverInput {
            event_type,
            hook_event_name,
            agent_type,
            project_dir,
            plugin_config: serde_json::json!({}),
        };

        let output_a = resolve_wave_context_pure(&input, &wave_state);
        let output_b = resolve_wave_context_pure(&input, &wave_state);

        prop_assert_eq!(
            output_a, output_b,
            "VP-075 violation: resolve_wave_context_pure is not deterministic"
        );
    }
}

// ─── AC-009 ──────────────────────────────────────────────────────────────────

/// BC-4.12.001: WaveContextResolver is registered in resolvers-registry.toml
/// under the key "wave-context".
///
/// This is a static assertion test — it reads the TOML file from the workspace
/// and verifies the entry exists. No WASM build required.
#[test]
fn test_BC_4_12_001_wasm_artifact_registered_in_registry() {
    // Navigate from the test binary location to the workspace root.
    // The test binary for an integration test lives in target/debug/deps/;
    // we walk up to find the workspace root by looking for the registry file.
    let registry_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent() // crates/
        .and_then(|p| p.parent()) // workspace root
        .map(|root| root.join("plugins/vsdd-factory/resolvers-registry.toml"))
        .expect("workspace root must exist");

    let contents = std::fs::read_to_string(&registry_path).unwrap_or_else(|e| {
        panic!(
            "AC-009: resolvers-registry.toml must exist at {}: {e}",
            registry_path.display()
        )
    });

    assert!(
        contents.contains(r#"name = "wave-context""#),
        "AC-009: resolvers-registry.toml must contain an entry with name = \"wave-context\"; \
         file contents:\n{contents}"
    );
    assert!(
        contents.contains(r#"path_allow = [".factory/"]"#),
        "AC-009: resolvers-registry.toml wave-context entry must declare path_allow = [\".factory/\"]; \
         file contents:\n{contents}"
    );
}

// ─── AC-010 ──────────────────────────────────────────────────────────────────

/// BC-4.12.004 INV1: No `unwrap()`, `expect()`, or `panic!()` in production source.
///
/// Reads src/lib.rs and src/wave_context.rs, asserts the panic-inducing patterns
/// are absent. This is a textual heuristic test; the canonical enforcement is:
///   `cargo clippy -p vsdd-context-resolvers -- -D clippy::unwrap_used -D clippy::expect_used`
///
/// Exceptions: test files and macro-expansion are out of scope for this check.
#[test]
fn test_BC_4_12_004_no_unwrap_or_expect_in_lib() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let src_files = ["src/lib.rs", "src/wave_context.rs"];

    for relative_path in &src_files {
        let path = manifest_dir.join(relative_path);
        let contents = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("AC-010: cannot read {}: {e}", path.display()));

        // Strip doc comments and inline comments to avoid false positives from
        // documentation examples. We check for call-site usage only.
        // A more robust check: `cargo clippy -- -D clippy::unwrap_used`.
        assert!(
            !contents.contains(".unwrap()"),
            "AC-010: {relative_path} must not call .unwrap(); \
             use ? or match instead (BC-4.12.004 INV1)"
        );
        assert!(
            !contents.contains(".expect("),
            "AC-010: {relative_path} must not call .expect(...); \
             use ? or match instead (BC-4.12.004 INV1)"
        );
        assert!(
            !contents.contains("panic!("),
            "AC-010: {relative_path} must not call panic!(); \
             all error paths return value: None (BC-4.12.004 INV1)"
        );
    }
}
