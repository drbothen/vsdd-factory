// Test files use .expect()/.unwrap()/.panic!() for failure reporting.
// The workspace-level deny is for production code; these are the test carve-outs.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
// Covers: AC-001, AC-002a, AC-002b, AC-003, AC-004, AC-007, AC-008, AC-009, AC-010.
//
// Naming convention: test_BC_4_12_NNN_<assertion>() per DF-037 test-writer rules.

use serde_json::Value;
use vsdd_context_resolvers::resolve_wave_context_pure;
use vsdd_context_resolvers::wave_context::{
    WaveEntry, WaveState, find_active_wave, parse_cycle_id_from_state_md, parse_wave_state,
};
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

/// Build a WaveState with one active wave containing the given stories.
fn wave_state_with_active_wave(wave_id: &str, stories: Vec<&str>) -> WaveState {
    WaveState {
        waves: vec![WaveEntry {
            wave: wave_id.to_string(),
            stories: stories.iter().map(|s| s.to_string()).collect(),
            stories_merged: vec![],
            gate_status: None, // not completed → active
            current_wave: None,
            next_gate_required: None,
        }],
    }
}

/// A fully-populated wave state fixture for happy-path tests.
fn populated_wave_state() -> WaveState {
    wave_state_with_active_wave("F4", vec!["S-12.07", "S-12.08"])
}

// ─── AC-001 ──────────────────────────────────────────────────────────────────

/// BC-4.12.002 PC3: output shape is `ResolverOutput { key: "wave_context", value: Some({...}) }`
/// when WaveState is fully populated.
///
/// Asserts:
///   - output.key == "wave_context"
///   - output.value is Some(_)
///   - value JSON has "stories" (array), "wave_id" (string), "cycle_id" (string)
#[test]
fn test_BC_4_12_002_wave_context_output_shape() {
    let input = make_input("/tmp/test");
    let wave_state = populated_wave_state();

    let output: ResolverOutput =
        resolve_wave_context_pure(&input, &wave_state, Some("v1.0-feature-engine-discipline"));

    assert_eq!(
        output.key, "wave_context",
        "key must be exactly 'wave_context' (underscore, per BC-4.12.005 PC7)"
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

    let output = resolve_wave_context_pure(&input, &wave_state, Some("v1.0-cycle"));
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
/// `parse_wave_state` must return `Err(serde_norway::Error)` on malformed input;
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

/// AC-004 (LOW-004): parse_wave_state fails on YAML missing the required `wave` field.
///
/// `WaveEntry.wave` is a required `String` field (no `#[serde(default)]`).
/// YAML that provides stories but no `wave:` key must fail deserialization — the
/// schema is fail-loud for required fields, which prevents silent empty-wave injection.
#[test]
fn test_parse_wave_state_rejects_missing_required_wave_field() {
    // YAML has a waves entry with stories but no `wave:` key.
    // serde_norway must return Err because `wave` is a required String field.
    let yaml = "waves:\n  - stories: [\"S-12.07\"]";

    let result = parse_wave_state(yaml);
    assert!(
        result.is_err(),
        "AC-004: parse_wave_state must return Err when required `wave` field is absent; got Ok"
    );
}

// ─── AC-002b ─────────────────────────────────────────────────────────────────

/// BC-4.12.004 PC3 + BC-4.12.002 PC3: resolve_wave_context_pure with a default
/// (empty waves) WaveState returns `value: None`.
///
/// This models the post-parse-failure path: resolve_impl catches the parse
/// error and calls resolve_wave_context_pure with `WaveState::default()`.
/// The output must NOT be Some; it must be None.
#[test]
fn test_BC_4_12_004_resolve_pure_with_default_wavestate_yields_none() {
    let input = make_input("/tmp/test");
    let wave_state = WaveState::default();

    let output = resolve_wave_context_pure(&input, &wave_state, Some("v1.0-cycle"));

    assert_eq!(
        output.key, "wave_context",
        "key must always be 'wave_context'"
    );
    assert!(
        output.value.is_none(),
        "AC-002b: value must be None when WaveState is empty waves (post-error path); got: {:?}",
        output.value
    );
}

/// AC-002b: missing cycle_id also yields value: None.
///
/// When resolve_impl cannot read STATE.md (or STATE.md has no current_cycle),
/// cycle_id is None. The pure fn must return value: None even with a valid wave_state.
#[test]
fn test_BC_4_12_004_resolve_pure_with_none_cycle_id_yields_none() {
    let input = make_input("/tmp/test");
    let wave_state = populated_wave_state();

    let output = resolve_wave_context_pure(&input, &wave_state, None);

    assert_eq!(output.key, "wave_context");
    assert!(
        output.value.is_none(),
        "AC-002b: value must be None when cycle_id is None; got: {:?}",
        output.value
    );
}

// ─── AC-003 ──────────────────────────────────────────────────────────────────

/// HIGH-002: empty stories list yields `value: Some({stories: [], ...})`.
///
/// When wave-state.yaml is present with an active wave that has `stories: []`,
/// the resolver emits Some so the hook sees the empty wave and returns Continue
/// per BC-4.10.001 EC-001 (vacuously all stories cleared). Returning None here
/// would cause the hook to Block with WAVE_CONTEXT_MISSING — incorrect for an
/// empty wave.
///
/// Note: previously asserted value: None (AC-003 old spec); updated for S-12.08
/// HIGH-002 fix. Only returns value: None when there is NO active wave at all.
#[test]
fn test_resolve_pure_with_empty_stories_emits_some_with_empty_array() {
    let input = make_input("/tmp/test");
    let wave_state = WaveState {
        waves: vec![WaveEntry {
            wave: "F4".to_string(),
            stories: vec![],
            stories_merged: vec![],
            gate_status: None,
            current_wave: None,
            next_gate_required: None,
        }],
    };

    let output = resolve_wave_context_pure(&input, &wave_state, Some("v1.0-cycle"));

    assert_eq!(output.key, "wave_context");
    let value = output.value.expect(
        "HIGH-002: active wave with empty stories must yield value: Some (not None), \
         so hook sees the empty wave and returns Continue per BC-4.10.001 EC-001",
    );
    let stories = value["stories"]
        .as_array()
        .expect("stories must be a JSON array");
    assert!(
        stories.is_empty(),
        "HIGH-002: empty active wave must produce empty stories array in wave_context; \
         got: {:?}",
        stories
    );
    assert!(
        value["wave_id"].is_string(),
        "wave_id must be present in empty-wave output; got: {:?}",
        value
    );
    assert!(
        value["cycle_id"].is_string(),
        "cycle_id must be present in empty-wave output; got: {:?}",
        value
    );
}

// ─── AC-004 ──────────────────────────────────────────────────────────────────

/// BC-4.12.002 PC3 + EC-004: missing wave_id (wave entry absent) must not panic.
///
/// When waves list is empty, find_active_wave returns None → value: None.
/// Resolver does NOT trap. (WaveEntry.wave is a required String, so there's no
/// partial wave — but an empty waves list is the correct proxy for AC-004.)
#[test]
fn test_BC_4_12_002_missing_wave_id_no_panic() {
    let input = make_input("/tmp/test");
    // Empty waves list → no active wave → value: None (no panic path).
    let wave_state = WaveState { waves: vec![] };

    let output = resolve_wave_context_pure(&input, &wave_state, Some("v1.0-cycle"));

    assert_eq!(output.key, "wave_context");

    // Primary assertion: must not panic. Reaching this line proves no panic.
    // Secondary: value must be None (no active wave).
    assert!(
        output.value.is_none(),
        "AC-004: no active wave must yield value: None; got: {:?}",
        output.value
    );
}

/// AC-004 ext: all-completed waves yields value: None (no active wave left).
#[test]
fn test_BC_4_12_002_all_completed_waves_yields_none() {
    let input = make_input("/tmp/test");
    let wave_state = WaveState {
        waves: vec![
            WaveEntry {
                wave: "F3".to_string(),
                stories: vec!["S-12.05".to_string()],
                stories_merged: vec!["S-12.05".to_string()],
                gate_status: Some("completed".to_string()),
                current_wave: None,
                next_gate_required: None,
            },
            WaveEntry {
                wave: "F4".to_string(),
                stories: vec!["S-12.07".to_string()],
                stories_merged: vec!["S-12.07".to_string()],
                gate_status: Some("completed".to_string()),
                current_wave: None,
                next_gate_required: None,
            },
        ],
    };

    let output = resolve_wave_context_pure(&input, &wave_state, Some("v1.0-cycle"));

    assert_eq!(output.key, "wave_context");
    assert!(
        output.value.is_none(),
        "AC-004: all waves completed must yield value: None; got: {:?}",
        output.value
    );
}

/// AC-004 ext: active wave is the LAST non-completed entry (not the first).
#[test]
fn test_BC_4_12_002_last_non_completed_wave_is_active() {
    let input = make_input("/tmp/test");
    let wave_state = WaveState {
        waves: vec![
            WaveEntry {
                wave: "F3".to_string(),
                stories: vec!["S-12.05".to_string()],
                stories_merged: vec!["S-12.05".to_string()],
                gate_status: Some("completed".to_string()),
                current_wave: None,
                next_gate_required: None,
            },
            WaveEntry {
                wave: "F4".to_string(),
                stories: vec!["S-12.07".to_string(), "S-12.08".to_string()],
                stories_merged: vec![],
                gate_status: Some("pending".to_string()), // not completed → active
                current_wave: None,
                next_gate_required: None,
            },
        ],
    };

    let output = resolve_wave_context_pure(&input, &wave_state, Some("v1.0-cycle"));

    let value = output.value.expect("active wave must yield Some value");
    assert_eq!(
        value["wave_id"].as_str(),
        Some("F4"),
        "active wave must be F4 (last non-completed)"
    );
}

// ─── AC-007 ──────────────────────────────────────────────────────────────────

/// BC-4.12.002 PC2: `ResolverInput.project_dir` is the top-level field used for
/// path construction.
///
/// The resolver uses `input.project_dir` (top-level String field on `ResolverInput`,
/// per BC-4.12.002 PC2), NOT `plugin_config["project_dir"]`. This test verifies
/// the actual struct layout.
///
/// data-shape pin: verifies ABI struct shape, not behavior (POL-11 opt-out)
#[test]
fn test_BC_4_12_002_project_dir_is_top_level_field() {
    let input = make_input("/tmp/test-project");

    // The top-level field must be directly accessible.
    assert_eq!(
        input.project_dir, "/tmp/test-project",
        "AC-007 (corrected): project_dir must be a top-level field on ResolverInput"
    );

    // plugin_config should NOT contain project_dir (dispatcher does not inject it there)
    assert!(
        input.plugin_config.get("project_dir").is_none(),
        "project_dir must NOT be inside plugin_config — it is a top-level field"
    );
}

// ─── STATE.md parsing ────────────────────────────────────────────────────────

/// BC-4.12.002 (F-003): parse_cycle_id_from_state_md extracts current_cycle from
/// YAML frontmatter.
#[test]
fn test_parse_cycle_id_from_state_md_happy_path() {
    let state_md = r#"---
document_type: pipeline-state
current_cycle: v1.0-feature-engine-discipline-pass-1
status: draft
---

# Pipeline State
"#;

    let cycle = parse_cycle_id_from_state_md(state_md);
    assert_eq!(
        cycle.as_deref(),
        Some("v1.0-feature-engine-discipline-pass-1"),
        "must extract current_cycle from STATE.md frontmatter"
    );
}

/// STATE.md with no frontmatter yields None (no panic).
#[test]
fn test_parse_cycle_id_from_state_md_no_frontmatter() {
    let state_md = "# Pipeline State\n\nNo frontmatter here.\n";
    let cycle = parse_cycle_id_from_state_md(state_md);
    assert!(
        cycle.is_none(),
        "no frontmatter must yield None, got: {:?}",
        cycle
    );
}

/// STATE.md with frontmatter but no current_cycle key yields None.
#[test]
fn test_parse_cycle_id_from_state_md_missing_key() {
    let state_md = "---\ndocument_type: pipeline-state\nstatus: draft\n---\n\n# Body\n";
    let cycle = parse_cycle_id_from_state_md(state_md);
    assert!(
        cycle.is_none(),
        "missing current_cycle key must yield None, got: {:?}",
        cycle
    );
}

// ─── AC-008 — proptest (VP-075) ──────────────────────────────────────────────

/// VP-075: resolve_wave_context_pure is deterministic — same inputs always produce
/// same outputs. 200 trials via proptest.
///
/// Strategy:
///   - Random WaveState (waves: Vec<WaveEntry> with random fields)
///   - Random ResolverInput
///   - Random cycle_id (Option<String>)
///   - Call resolve_wave_context_pure twice, assert outputs equal.
///
/// Must complete within 5s (proptest default timeout).
use proptest::prelude::*;

fn arb_wave_entry() -> impl Strategy<Value = WaveEntry> {
    (
        any::<String>(),
        prop::collection::vec(any::<String>(), 0..5),
        prop::collection::vec(any::<String>(), 0..5),
        prop::option::of(any::<String>()),
    )
        .prop_map(|(wave, stories, stories_merged, gate_status)| WaveEntry {
            wave,
            stories,
            stories_merged,
            gate_status,
            current_wave: None,
            next_gate_required: None,
        })
}

fn arb_wave_state() -> impl Strategy<Value = WaveState> {
    prop::collection::vec(arb_wave_entry(), 0..4).prop_map(|waves| WaveState { waves })
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 200,
        ..ProptestConfig::default()
    })]

    #[test]
    fn test_BC_4_12_002_prop_resolve_wave_context_is_deterministic(
        wave_state in arb_wave_state(),
        cycle_id in prop::option::of(any::<String>()),
        event_type in any::<String>(),
        hook_event_name in any::<String>(),
        agent_type in prop::option::of(any::<String>()),
        project_dir in any::<String>(),
    ) {
        // VP-075: same (input, wave_state, cycle_id) triple must produce identical output.
        let input = ResolverInput {
            event_type,
            hook_event_name,
            agent_type,
            project_dir,
            plugin_config: serde_json::json!({}),
        };

        let output_a = resolve_wave_context_pure(&input, &wave_state, cycle_id.as_deref());
        let output_b = resolve_wave_context_pure(&input, &wave_state, cycle_id.as_deref());

        prop_assert_eq!(
            output_a, output_b,
            "VP-075 violation: resolve_wave_context_pure is not deterministic"
        );
    }
}

// ─── gate_status four-case YAML truth table ──────────────────────────────────

/// MED-003a: YAML `gate_status: ~` (null) parses to `None`; wave is treated as active.
///
/// Case 2 of the gate_status four-case truth table: YAML null → `Option<String>::None` →
/// wave is NOT in a terminal state → wave is active.
#[test]
fn test_gate_status_yaml_null() {
    let yaml = r#"
waves:
  - wave: "F5"
    stories: ["S-13.01"]
    gate_status: ~
"#;
    let state = parse_wave_state(yaml).expect("valid YAML must parse");
    let active = find_active_wave(&state);
    assert!(
        active.is_some(),
        "gate_status: ~ (null) must produce None → wave is active; got None"
    );
    assert_eq!(active.unwrap().wave, "F5", "active wave must be F5");
}

/// MED-003b: YAML omits `gate_status:` key entirely; serde default → `None`; wave is active.
///
/// Case 1 of the gate_status four-case truth table: key absent → `Option<String>::None` →
/// wave is NOT in a terminal state → wave is active.
#[test]
fn test_gate_status_yaml_key_absent() {
    let yaml = r#"
waves:
  - wave: "F5"
    stories: ["S-13.01"]
"#;
    let state = parse_wave_state(yaml).expect("valid YAML must parse");
    let active = find_active_wave(&state);
    assert!(
        active.is_some(),
        "absent gate_status key must produce None → wave is active"
    );
}

/// MED-003c: Explicit `gate_status: "completed"` marks wave as terminal (not active).
///
/// Case 4 of the gate_status four-case truth table: `"completed"` is a terminal state.
#[test]
fn test_gate_status_yaml_completed() {
    let yaml = r#"
waves:
  - wave: "F4"
    stories: ["S-12.07"]
    gate_status: "completed"
"#;
    let state = parse_wave_state(yaml).expect("valid YAML must parse");
    let active = find_active_wave(&state);
    assert!(
        active.is_none(),
        "gate_status: completed must be terminal → no active wave; got Some"
    );
}

/// MED-003d: Explicit `gate_status: "in_progress"` (non-terminal) → wave is active.
///
/// Case 3 of the gate_status four-case truth table: any non-terminal status string →
/// `Some("in_progress")` → wave is NOT in TERMINAL_STATES → wave is active.
#[test]
fn test_gate_status_yaml_other_value() {
    let yaml = r#"
waves:
  - wave: "F5"
    stories: ["S-13.01"]
    gate_status: "in_progress"
"#;
    let state = parse_wave_state(yaml).expect("valid YAML must parse");
    let active = find_active_wave(&state);
    assert!(
        active.is_some(),
        "gate_status: in_progress (non-terminal) must → wave is active"
    );
}

/// `gate_status: "passed"` is a canonical terminal value per BC-8.14.009.
/// All-passed waves → no active wave.
#[test]
fn test_gate_status_yaml_passed_is_terminal() {
    let yaml = r#"
waves:
  - wave: "F3"
    stories: ["S-12.05"]
    gate_status: "passed"
"#;
    let state = parse_wave_state(yaml).expect("valid YAML must parse");
    let active = find_active_wave(&state);
    assert!(
        active.is_none(),
        "gate_status: passed must be terminal → no active wave (MED-001 TERMINAL_STATES)"
    );
}

/// `gate_status: "deferred"` is a canonical terminal value per BC-8.14.009.
#[test]
fn test_gate_status_yaml_deferred_is_terminal() {
    let yaml = r#"
waves:
  - wave: "F3"
    stories: ["S-12.05"]
    gate_status: "deferred"
"#;
    let state = parse_wave_state(yaml).expect("valid YAML must parse");
    let active = find_active_wave(&state);
    assert!(
        active.is_none(),
        "gate_status: deferred must be terminal → no active wave (BC-8.14.009)"
    );
}

/// `gate_status: "failed"` is a canonical terminal value per BC-8.14.009.
/// A wave with gate_status: failed must NOT be returned as active.
#[test]
fn test_gate_status_yaml_failed_is_terminal() {
    let yaml = r#"
waves:
  - wave: "F3"
    stories: ["S-12.05"]
    gate_status: "failed"
"#;
    let state = parse_wave_state(yaml).expect("valid YAML must parse");
    let active = find_active_wave(&state);
    assert!(
        active.is_none(),
        "gate_status: failed must be terminal → no active wave (BC-8.14.009)"
    );
}

// ─── CRLF line ending support ────────────────────────────────────────────────

/// MED-004: parse_cycle_id_from_state_md normalizes CRLF line endings.
///
/// STATE.md files checked out on Windows or with `core.autocrlf = true` may
/// contain `\r\n` line endings. The parser must handle them correctly.
#[test]
fn test_parse_cycle_id_handles_crlf_line_endings() {
    // STATE.md with CRLF line endings (\r\n everywhere).
    let state_md_crlf = "---\r\ndocument_type: pipeline-state\r\ncurrent_cycle: v1.0-feature-engine-discipline-crlf\r\nstatus: draft\r\n---\r\n\r\n# Pipeline State\r\n";

    let cycle = parse_cycle_id_from_state_md(state_md_crlf);
    assert_eq!(
        cycle.as_deref(),
        Some("v1.0-feature-engine-discipline-crlf"),
        "must extract current_cycle from STATE.md with CRLF line endings (MED-004)"
    );
}

// ─── AC-009 ──────────────────────────────────────────────────────────────────

/// BC-4.12.001: WaveContextResolver is registered in resolvers-registry.toml
/// under the canonical key "wave_context" (underscore, per BC-4.12.005 PC7).
/// Uses sentinel-based workspace root resolution.
#[test]
fn test_BC_4_12_001_wasm_artifact_registered_in_registry() {
    // Walk up from CARGO_MANIFEST_DIR to find workspace root by locating a
    // Cargo.toml that contains `[workspace]` — sentinel-based resolution that
    // survives directory restructuring (F-013).
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = find_workspace_root(manifest_dir)
        .expect("AC-009: must be able to find workspace root (Cargo.toml with [workspace])");

    let registry_path = workspace_root.join("plugins/vsdd-factory/resolvers-registry.toml");
    let contents = std::fs::read_to_string(&registry_path).unwrap_or_else(|e| {
        panic!(
            "AC-009: resolvers-registry.toml must exist at {}: {e}",
            registry_path.display()
        )
    });

    assert!(
        contents.contains(r#"name = "wave_context""#),
        "AC-009: resolvers-registry.toml must contain an entry with name = \"wave_context\" \
         (underscore, per BC-4.12.005 PC7); file contents:\n{contents}"
    );
    assert!(
        contents.contains(r#"context_key = "wave_context""#),
        "AC-009: resolvers-registry.toml must contain context_key = \"wave_context\"; \
         file contents:\n{contents}"
    );
    // F-P3-007: path_allow narrowed from [".factory/"] to exact files the resolver reads.
    // AC-009 updated: assert both required files are in path_allow (not directory prefix).
    // The WaveContextResolver reads only wave-state.yaml + STATE.md (BC-4.12.003 INV1).
    assert!(
        contents.contains(r#"".factory/wave-state.yaml""#),
        "AC-009: resolvers-registry.toml wave_context path_allow must include \
         \".factory/wave-state.yaml\" (F-P3-007 narrowed grant); file contents:\n{contents}"
    );
    assert!(
        contents.contains(r#"".factory/STATE.md""#),
        "AC-009: resolvers-registry.toml wave_context path_allow must include \
         \".factory/STATE.md\" (F-P3-007 narrowed grant); file contents:\n{contents}"
    );
}

/// Sentinel-based workspace root finder (F-013).
/// Climbs the directory tree from `start` until it finds a `Cargo.toml` that
/// contains `[workspace]`. Returns None if no such file is found at or above `start`.
fn find_workspace_root(start: &std::path::Path) -> Option<std::path::PathBuf> {
    let mut current = start;
    loop {
        let candidate = current.join("Cargo.toml");
        if candidate.exists()
            && let Ok(contents) = std::fs::read_to_string(&candidate)
            && contents.contains("[workspace]")
        {
            return Some(current.to_path_buf());
        }
        match current.parent() {
            Some(p) => current = p,
            None => return None,
        }
    }
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

// ─── F-009 / EC-002: event_type-agnostic behavior ────────────────────────────

/// BC-4.12.002 EC-002: resolver returns wave_context regardless of event_type.
/// Iterates over multiple event types and asserts consistent output.
#[test]
fn test_resolve_pure_returns_value_for_all_event_types() {
    let wave_state = populated_wave_state();
    let cycle_id = Some("v1.0-feature-engine-discipline-pass-1");

    let event_types = [
        "PreToolUse",
        "PostToolUse",
        "SubagentStop",
        "",
        "UnknownFutureEventType",
    ];

    // Compute reference output with the first event type.
    let reference_input = ResolverInput {
        event_type: event_types[0].to_string(),
        hook_event_name: "Stop".into(),
        agent_type: None,
        project_dir: "/tmp/test".into(),
        plugin_config: serde_json::json!({}),
    };
    let reference = resolve_wave_context_pure(&reference_input, &wave_state, cycle_id);

    for event_type in &event_types[1..] {
        let input = ResolverInput {
            event_type: event_type.to_string(),
            hook_event_name: "Stop".into(),
            agent_type: None,
            project_dir: "/tmp/test".into(),
            plugin_config: serde_json::json!({}),
        };
        let output = resolve_wave_context_pure(&input, &wave_state, cycle_id);
        assert_eq!(
            output, reference,
            "EC-002: resolve_wave_context_pure must return identical output for \
             event_type='{}' vs '{}'",
            event_type, event_types[0]
        );
    }
}
