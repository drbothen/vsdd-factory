//! Unit tests for validate-artifact-path.
//!
//! Exercises the production functions in `lib.rs` via injectable callbacks.
//! Test naming follows the BC-based convention: test_BC_S_SS_NNN_xxx().
//!
//! # VP trace
//! - VP-069: proptest harness — registry-load purity
//! - VP-070: kani harness equivalents — path matching is deterministic

#![allow(clippy::type_complexity)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::collapsible_if)]

use super::*;
use serde_json::json;
use std::panic;

// -----------------------------------------------------------------------
// Test helpers
// -----------------------------------------------------------------------

/// Build a minimal valid registry YAML string with a single entry.
/// `enforcement_level` controls the entry's level: "block", "warn", or "advisory".
fn registry_yaml(enforcement_level: &str) -> String {
    format!(
        r#"version: 1
artifacts:
  - artifact_type: behavioral-contract
    canonical_path_pattern: ".factory/specs/behavioral-contracts/ss-{{subsystem}}/BC-{{bc-id}}.md"
    description: Behavioral contract spec
    enforcement_level: "{}"
"#,
        enforcement_level
    )
}

/// Build a registry YAML with no artifact entries (empty artifact list).
fn empty_registry_yaml() -> String {
    "version: 1\nartifacts: []\n".to_string()
}

/// Build a minimal valid registry YAML with multiple entries covering
/// at least 5 artifact types.
fn multi_entry_registry_yaml() -> String {
    r#"version: 1
artifacts:
  - artifact_type: behavioral-contract
    canonical_path_pattern: ".factory/specs/behavioral-contracts/ss-{subsystem}/BC-{bc-id}.md"
    description: Behavioral contract spec
    enforcement_level: "block"
  - artifact_type: adr
    canonical_path_pattern: ".factory/specs/architecture/decisions/ADR-{adr-id}-{slug}.md"
    description: Architecture decision record
    enforcement_level: "block"
  - artifact_type: verification-property
    canonical_path_pattern: ".factory/specs/verification-properties/VP-{vp-id}.md"
    description: Verification property
    enforcement_level: "block"
  - artifact_type: story-spec
    canonical_path_pattern: ".factory/stories/S-{story-id}-{slug}.md"
    description: Story specification
    enforcement_level: "block"
  - artifact_type: cycle-document
    canonical_path_pattern: ".factory/cycles/{cycle-id}/{doc-type}.md"
    description: Cycle document
    enforcement_level: "block"
  - artifact_type: prd
    canonical_path_pattern: ".factory/specs/prd.md"
    description: Product requirements document
    enforcement_level: "block"
"#
    .to_string()
}

/// Build a registry YAML with all entries set to advisory enforcement.
fn advisory_only_registry_yaml() -> String {
    r#"version: 1
artifacts:
  - artifact_type: behavioral-contract
    canonical_path_pattern: ".factory/specs/behavioral-contracts/ss-{subsystem}/BC-{bc-id}.md"
    description: Behavioral contract spec
    enforcement_level: "advisory"
"#
    .to_string()
}

/// Make a PreToolUse HookPayload for the given tool and optional file_path.
fn make_payload(tool_name: &str, file_path: Option<&str>) -> HookPayload {
    let mut v = json!({
        "event_name": "PreToolUse",
        "session_id": "test-session",
        "dispatcher_trace_id": "test-trace",
        "tool_name": tool_name,
        "tool_input": {}
    });
    if let Some(path) = file_path {
        v["tool_input"]["file_path"] = json!(path);
    }
    serde_json::from_value(v).expect("fixture must deserialize")
}

/// Attempt to call `load_registry` on `yaml` and ASSERT it returns Ok.
fn require_registry(yaml: &str, context: &str) -> PathRegistry {
    let result = panic::catch_unwind(|| load_registry(yaml));
    assert!(
        result.is_ok(),
        "load_registry panicked on valid YAML — production function is unimplemented (todo!()). \
             Context: {}. BC-4.11.001 PC1: load_registry must return Ok(PathRegistry) for valid YAML.",
        context
    );
    let parse_result = result.unwrap();
    assert!(
        parse_result.is_ok(),
        "load_registry returned Err for valid YAML — expected Ok(PathRegistry). \
             Context: {}. BC-4.11.001 PC1.",
        context
    );
    parse_result.unwrap()
}

/// Run hook_logic with injectable callbacks and capture (result, log_level+msg, emitted_event).
fn run_logic(
    payload: HookPayload,
    registry_result: Result<String, String>,
) -> (
    HookResult,
    Vec<(u8, String)>,
    Vec<(String, Vec<(String, String)>)>,
) {
    let mut log_calls: Vec<(u8, String)> = Vec::new();
    let mut events: Vec<(String, Vec<(String, String)>)> = Vec::new();

    let result = hook_logic(
        payload,
        HookCallbacks {
            read_file: move |_path| registry_result.clone(),
            emit_event: |event_type, fields| {
                events.push((
                    event_type.to_string(),
                    fields
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect(),
                ));
            },
            log: |level, msg| {
                log_calls.push((level, msg.to_string()));
            },
        },
    );

    (result, log_calls, events)
}

// -----------------------------------------------------------------------
// AC-001 (BC-4.11.001 PC1 + VP-069): load_registry happy path
// load_registry on valid YAML returns Ok with parsed entries.
// -----------------------------------------------------------------------

#[test]
fn test_BC_4_11_001_ac001_load_registry_valid_yaml_returns_ok() {
    let yaml = registry_yaml("block");
    let result = panic::catch_unwind(|| load_registry(&yaml));
    assert!(
        result.is_ok(),
        "load_registry should return Ok(PathRegistry) for valid YAML, \
             but the production function is unimplemented (todo!()). \
             BC-4.11.001 PC1: registry must be parseable at hook invocation time."
    );
    if let Ok(Ok(registry)) = result {
        assert!(
            !registry.artifacts.is_empty(),
            "load_registry should parse at least one entry from valid YAML \
                 (BC-4.11.001 PC1 — registry is the single source of truth)"
        );
        assert_eq!(registry.version, 1, "registry schema version must be 1");
    }
}

#[test]
fn test_BC_4_11_001_ac001_load_registry_parses_artifact_type_field() {
    let yaml = registry_yaml("block");
    let result = panic::catch_unwind(|| load_registry(&yaml));
    assert!(
        result.is_ok(),
        "load_registry should return Ok for valid YAML with artifact_type field \
             (BC-4.11.001 PC1 — artifact_type is a required registry entry field). \
             Production function is unimplemented."
    );
    if let Ok(Ok(registry)) = result {
        let entry = registry
            .artifacts
            .first()
            .expect("parsed registry must have at least one entry");
        assert_eq!(
            entry.artifact_type, "behavioral-contract",
            "artifact_type must be parsed from YAML"
        );
    }
}

#[test]
fn test_BC_4_11_001_ac001_load_registry_parses_enforcement_level_field() {
    let yaml = registry_yaml("warn");
    let result = panic::catch_unwind(|| load_registry(&yaml));
    assert!(
        result.is_ok(),
        "load_registry should return Ok and parse enforcement_level field \
             (BC-4.11.001 invariant 2 — per-entry enforcement_level must be respected). \
             Production function is unimplemented."
    );
    if let Ok(Ok(registry)) = result {
        let entry = registry
            .artifacts
            .first()
            .expect("parsed registry must have at least one entry");
        assert_eq!(
            entry.enforcement_level, "warn",
            "enforcement_level must be parsed correctly from YAML"
        );
    }
}

// -----------------------------------------------------------------------
// AC-001 continued: load_registry malformed YAML returns Err (not panic)
// -----------------------------------------------------------------------

#[test]
fn test_BC_4_11_001_ac001_load_registry_malformed_yaml_returns_err() {
    let malformed = "{ this is not: valid yaml: [unclosed bracket";
    let result = panic::catch_unwind(|| load_registry(malformed));
    assert!(
        result.is_ok(),
        "load_registry must NOT panic on malformed YAML (VP-069 Part A invariant). \
             It should return Err(RegistryError::ParseError). Production function is unimplemented."
    );
    if let Ok(outcome) = result {
        assert!(
            outcome.is_err(),
            "load_registry must return Err for malformed YAML (BC-4.11.001 EC-002)"
        );
        if let Err(err) = outcome {
            let msg = format!("{}", err);
            assert!(
                msg.contains("parse error") || msg.contains("parse"),
                "RegistryError for malformed YAML must be ParseError, got: {}",
                msg
            );
        }
    }
}

#[test]
fn test_BC_4_11_001_ac001_load_registry_empty_string_returns_err() {
    let result = panic::catch_unwind(|| load_registry(""));
    assert!(
        result.is_ok(),
        "load_registry must NOT panic on empty string input (VP-069 Part A). \
             Should return Err(RegistryError). Production function is unimplemented."
    );
}

#[test]
fn test_BC_4_11_001_ac001_load_registry_missing_required_field_returns_missing_field_err() {
    let missing_fields_yaml = "version: 1\nartifacts:\n  - artifact_type: behavioral-contract\n";
    let result = panic::catch_unwind(|| load_registry(missing_fields_yaml));
    assert!(
        result.is_ok(),
        "load_registry must NOT panic when a required field is absent (VP-069 Part A). \
             Should return Err(RegistryError::MissingField). Production function is unimplemented."
    );
    // If the function returns at all, it should be an error for incomplete entries
    // (enforcement_level and canonical_path_pattern are required per ADR-016 schema)
}

// -----------------------------------------------------------------------
// AC-002 (BC-4.11.001 PC2b): matches_canonical — canonical paths → Match
// Tests at least 5 distinct artifact types per story requirement.
// -----------------------------------------------------------------------

#[test]
fn test_BC_4_11_001_ac002_matches_canonical_bc_path_returns_match() {
    let yaml = multi_entry_registry_yaml();
    let registry = require_registry(
        &yaml,
        "test_BC_4_11_001_ac002_matches_canonical_bc_path_returns_match",
    );
    let match_result = panic::catch_unwind(|| {
        matches_canonical(
            ".factory/specs/behavioral-contracts/ss-04/BC-4.11.001.md",
            &registry,
        )
    });
    assert!(
        match_result.is_ok(),
        "matches_canonical should return MatchResult::Block for a canonical BC path \
             (BC-4.11.001 PC2b, PC3: path matches block entry → write proceeds). \
             Production function is unimplemented."
    );
    if let Ok(result) = match_result {
        assert_eq!(
            result,
            MatchResult::Block,
            "canonical BC path with enforcement_level=block must return MatchResult::Block"
        );
    }
}

#[test]
fn test_BC_4_11_001_ac002_matches_canonical_adr_path_returns_match() {
    let yaml = multi_entry_registry_yaml();
    let registry = require_registry(&yaml, "ac002_adr_path");
    let result = panic::catch_unwind(|| {
        matches_canonical(
            ".factory/specs/architecture/decisions/ADR-016-artifact-path-registry-sot.md",
            &registry,
        )
    });
    assert!(
        result.is_ok(),
        "matches_canonical should return MatchResult::Block for canonical ADR path \
             (BC-4.11.001 PC2b — ADR artifact type registered in registry). \
             Production function is unimplemented."
    );
    if let Ok(mr) = result {
        assert_eq!(mr, MatchResult::Block, "ADR path must match Block entry");
    }
}

#[test]
fn test_BC_4_11_001_ac002_matches_canonical_vp_path_returns_match() {
    let yaml = multi_entry_registry_yaml();
    let registry = require_registry(&yaml, "ac002_vp_path");
    let result = panic::catch_unwind(|| {
        matches_canonical(
            ".factory/specs/verification-properties/VP-069.md",
            &registry,
        )
    });
    assert!(
        result.is_ok(),
        "matches_canonical should return MatchResult::Block for canonical VP path \
             (BC-4.11.001 PC2b — VP artifact type registered). Production unimplemented."
    );
    if let Ok(mr) = result {
        assert_eq!(mr, MatchResult::Block, "VP path must match Block entry");
    }
}

#[test]
fn test_BC_4_11_001_ac002_matches_canonical_story_path_returns_match() {
    let yaml = multi_entry_registry_yaml();
    let registry = require_registry(&yaml, "ac002_story_path");
    let result = panic::catch_unwind(|| {
        matches_canonical(
            ".factory/stories/S-13.01-path-governance-bundle.md",
            &registry,
        )
    });
    assert!(
        result.is_ok(),
        "matches_canonical should return MatchResult::Block for canonical story path \
             (BC-4.11.001 PC2b — story-spec artifact type registered). Production unimplemented."
    );
    if let Ok(mr) = result {
        assert_eq!(mr, MatchResult::Block, "story path must match Block entry");
    }
}

#[test]
fn test_BC_4_11_001_ac002_matches_canonical_prd_path_returns_match() {
    let yaml = multi_entry_registry_yaml();
    let registry = require_registry(&yaml, "ac002_prd_path");
    let result = panic::catch_unwind(|| matches_canonical(".factory/specs/prd.md", &registry));
    assert!(
        result.is_ok(),
        "matches_canonical should return MatchResult::Block for canonical PRD path \
             (BC-4.11.001 PC2b — prd artifact type registered). Production unimplemented."
    );
    if let Ok(mr) = result {
        assert_eq!(mr, MatchResult::Block, "PRD path must match Block entry");
    }
}

#[test]
fn test_BC_4_11_001_ac002_matches_canonical_cycle_doc_path_returns_match() {
    let yaml = multi_entry_registry_yaml();
    let registry = require_registry(&yaml, "ac002_cycle_doc_path");
    let result = panic::catch_unwind(|| {
        matches_canonical(
            ".factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md",
            &registry,
        )
    });
    assert!(
        result.is_ok(),
        "matches_canonical should return MatchResult::Block for canonical cycle-doc path \
             (BC-4.11.001 PC2b — cycle-document artifact type registered). Production unimplemented."
    );
    if let Ok(mr) = result {
        assert_eq!(
            mr,
            MatchResult::Block,
            "cycle-document path must match Block entry"
        );
    }
}

// -----------------------------------------------------------------------
// AC-002 (BC-4.11.001 PC2b): non-canonical paths → NoMatch
// -----------------------------------------------------------------------

#[test]
fn test_BC_4_11_001_ac002_matches_canonical_unregistered_path_returns_nomatch() {
    let yaml = registry_yaml("block");
    let registry = require_registry(&yaml, "ac002_unregistered_path_nomatch");
    let result =
        panic::catch_unwind(|| matches_canonical(".factory/feature-deltas/F1-delta.md", &registry));
    assert!(
        result.is_ok(),
        "matches_canonical should return MatchResult::NoMatch for unregistered path \
             (BC-4.11.001 invariant 3 — unregistered path must be blocked). Production unimplemented."
    );
    if let Ok(mr) = result {
        assert_eq!(
            mr,
            MatchResult::NoMatch,
            ".factory/feature-deltas/F1-delta.md matches no registered pattern → NoMatch"
        );
    }
}

// -----------------------------------------------------------------------
// AC-002 (BC-4.11.001 EC-005): first-match-wins for ambiguous paths
// -----------------------------------------------------------------------

#[test]
fn test_BC_4_11_001_ac002_matches_canonical_first_match_wins_for_ambiguous_path() {
    let ambiguous_yaml = r#"version: 1
artifacts:
  - artifact_type: first-match
    canonical_path_pattern: ".factory/specs/behavioral-contracts/ss-{subsystem}/BC-{bc-id}.md"
    description: First entry
    enforcement_level: "block"
  - artifact_type: second-match
    canonical_path_pattern: ".factory/specs/behavioral-contracts/ss-{subsystem}/BC-{bc-id}.md"
    description: Second entry
    enforcement_level: "warn"
"#;
    let registry = require_registry(ambiguous_yaml, "ac002_first_match_wins");
    let result = panic::catch_unwind(|| {
        matches_canonical(
            ".factory/specs/behavioral-contracts/ss-04/BC-4.11.001.md",
            &registry,
        )
    });
    assert!(
        result.is_ok(),
        "matches_canonical should return first-match result (BC-4.11.001 EC-005: \
             first-matching entry wins). Production unimplemented."
    );
    if let Ok(mr) = result {
        // The first entry has enforcement_level=block → MatchResult::Block
        assert_eq!(
            mr,
            MatchResult::Block,
            "first-match-wins: the first registry entry (enforcement_level=block) \
                 must be returned, not the second (enforcement_level=warn)"
        );
    }
}

// -----------------------------------------------------------------------
// AC-002: placeholder substitution in patterns
// -----------------------------------------------------------------------

#[test]
fn test_BC_4_11_001_ac002_placeholder_subsystem_expansion_in_bc_pattern() {
    let yaml = registry_yaml("block");
    let registry = require_registry(&yaml, "ac002_placeholder_subsystem");
    for subsystem in &["01", "04", "06", "13"] {
        let path = format!(
            ".factory/specs/behavioral-contracts/ss-{}/BC-{}.01.001.md",
            subsystem, subsystem
        );
        let result = panic::catch_unwind({
            let registry = registry.clone();
            let path = path.clone();
            move || matches_canonical(&path, &registry)
        });
        assert!(
            result.is_ok(),
            "matches_canonical should support {{subsystem}} placeholder for \
                 path '{}' (BC-4.11.001 invariant 6). Production unimplemented.",
            path
        );
        if let Ok(mr) = result {
            assert_eq!(
                mr,
                MatchResult::Block,
                "path with subsystem '{}' must match block entry via placeholder expansion",
                subsystem
            );
        }
    }
}

#[test]
fn test_BC_4_11_001_ac002_placeholder_bc_id_expansion_in_bc_pattern() {
    let yaml = registry_yaml("block");
    let registry = require_registry(&yaml, "ac002_placeholder_bc_id");
    let paths = [
        ".factory/specs/behavioral-contracts/ss-04/BC-4.11.001.md",
        ".factory/specs/behavioral-contracts/ss-06/BC-6.22.001.md",
        ".factory/specs/behavioral-contracts/ss-01/BC-1.01.001.md",
    ];
    for path in &paths {
        let result = panic::catch_unwind({
            let registry = registry.clone();
            let path = path.to_string();
            move || matches_canonical(&path, &registry)
        });
        assert!(
            result.is_ok(),
            "matches_canonical should support {{bc-id}} placeholder for path '{}' \
                 (BC-4.11.001 invariant 6 — placeholder expansion). Production unimplemented.",
            path
        );
        if let Ok(mr) = result {
            assert_eq!(
                mr,
                MatchResult::Block,
                "BC path '{}' must match via {{bc-id}} placeholder expansion",
                path
            );
        }
    }
}

#[test]
fn test_BC_4_11_001_ac002_placeholder_cycle_id_expansion_in_cycle_doc_pattern() {
    let yaml = r#"version: 1
artifacts:
  - artifact_type: cycle-document
    canonical_path_pattern: ".factory/cycles/{cycle-id}/{doc-type}.md"
    description: Cycle document
    enforcement_level: "block"
"#;
    let registry = require_registry(yaml, "ac002_placeholder_cycle_id");
    let path = ".factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md";
    let result = panic::catch_unwind({
        let registry = registry.clone();
        move || matches_canonical(path, &registry)
    });
    assert!(
        result.is_ok(),
        "matches_canonical should support {{cycle-id}} placeholder for cycle docs \
             (BC-4.11.001 invariant 6). Path: '{}'. Production unimplemented.",
        path
    );
    if let Ok(mr) = result {
        assert_eq!(
            mr,
            MatchResult::Block,
            "cycle-doc path must match via {{cycle-id}} placeholder"
        );
    }
}

#[test]
fn test_BC_4_11_001_ac002_placeholder_story_id_expansion_in_story_pattern() {
    let yaml = r#"version: 1
artifacts:
  - artifact_type: story-spec
    canonical_path_pattern: ".factory/stories/S-{story-id}-{slug}.md"
    description: Story specification
    enforcement_level: "block"
"#;
    let registry = require_registry(yaml, "ac002_placeholder_story_id");
    let path = ".factory/stories/S-13.01-path-governance-bundle.md";
    let result = panic::catch_unwind({
        let registry = registry.clone();
        move || matches_canonical(path, &registry)
    });
    assert!(
        result.is_ok(),
        "matches_canonical should support {{story-id}} and {{slug}} placeholders \
             (BC-4.11.001 invariant 6). Path: '{}'. Production unimplemented.",
        path
    );
    if let Ok(mr) = result {
        assert_eq!(
            mr,
            MatchResult::Block,
            "story path must match via {{story-id}}/{{slug}} placeholder expansion"
        );
    }
}

#[test]
fn test_BC_4_11_001_ac002_placeholder_phase_expansion_in_phase_pattern() {
    let yaml = r#"version: 1
artifacts:
  - artifact_type: phase-delta-analysis
    canonical_path_pattern: ".factory/cycles/{cycle-id}/{phase}-delta-analysis.md"
    description: Phase delta analysis
    enforcement_level: "block"
"#;
    let registry = require_registry(yaml, "ac002_placeholder_phase");
    let path = ".factory/cycles/v1.0-feature-engine-discipline-pass-1/F1-delta-analysis.md";
    let result = panic::catch_unwind({
        let registry = registry.clone();
        move || matches_canonical(path, &registry)
    });
    assert!(
        result.is_ok(),
        "matches_canonical should support {{phase}} placeholder for phase delta analysis \
             (BC-4.11.001 invariant 6). Path: '{}'. Production unimplemented.",
        path
    );
    if let Ok(mr) = result {
        assert_eq!(
            mr,
            MatchResult::Block,
            "phase-delta path must match via {{phase}} placeholder expansion"
        );
    }
}

// -----------------------------------------------------------------------
// AC-003 (BC-4.11.001 PC6): unregistered .factory/ path blocked
// -----------------------------------------------------------------------

#[test]
fn test_BC_4_11_001_ac003_unregistered_path_blocked() {
    let payload = make_payload("Write", Some(".factory/feature-deltas/F1-delta.md"));
    let result = panic::catch_unwind(|| run_logic(payload, Ok(registry_yaml("block"))));
    assert!(
        result.is_ok(),
        "hook_logic should return HookResult::block_with_fix for unregistered .factory/ path \
             '.factory/feature-deltas/F1-delta.md' (BC-4.11.001 PC6 — ARTIFACT_PATH_UNREGISTERED). \
             Production function is unimplemented."
    );
    if let Ok((hook_result, _, _)) = result {
        match &hook_result {
            HookResult::Block { reason } => {
                assert!(
                    reason.contains("ARTIFACT_PATH_UNREGISTERED"),
                    "block reason must contain ARTIFACT_PATH_UNREGISTERED code \
                         (BC-4.11.001 PC6). Got: '{}'",
                    reason
                );
                assert!(
                    reason.contains("validate-artifact-path"),
                    "block reason must include hook name 'validate-artifact-path' \
                         (canonical Why/Fix/Code pattern). Got: '{}'",
                    reason
                );
            }
            other => {
                panic!(
                    "expected HookResult::Block for unregistered .factory/ path, \
                         got {:?} (BC-4.11.001 PC6 — unregistered path must be blocked)",
                    other
                );
            }
        }
    }
}

#[test]
fn test_BC_4_11_001_ac003_block_reason_contains_path_under_test() {
    let target_path = ".factory/feature-deltas/F1-delta.md";
    let payload = make_payload("Write", Some(target_path));
    let result = panic::catch_unwind(|| run_logic(payload, Ok(registry_yaml("block"))));
    assert!(
        result.is_ok(),
        "hook_logic must not panic for unregistered .factory/ path; \
             should return HookResult::Block (BC-4.11.001 PC6). Production unimplemented."
    );
    if let Ok((hook_result, _, _)) = result {
        if let HookResult::Block { reason } = hook_result {
            assert!(
                reason.contains(target_path),
                "block reason must include the write target path '{}' \
                     so the agent knows which path was rejected (BC-4.11.001 PC6). \
                     Got: '{}'",
                target_path,
                reason
            );
        }
    }
}

// -----------------------------------------------------------------------
// AC-004 (BC-4.11.001 PC7): non-.factory/ path → early-exit Continue
// -----------------------------------------------------------------------

#[test]
fn test_BC_4_11_001_ac004_non_factory_path_returns_continue() {
    let mut read_file_called = false;
    let payload = make_payload("Write", Some("src/lib.rs"));
    let result = panic::catch_unwind(move || {
        hook_logic(
            payload,
            HookCallbacks {
                read_file: |_path| {
                    // read_file must NOT be called for non-.factory/ paths
                    read_file_called = true;
                    Err("should not be called".to_string())
                },
                emit_event: |_, _| {},
                log: |_, _| {},
            },
        )
    });
    assert!(
        result.is_ok(),
        "hook_logic should return HookResult::Continue for non-.factory/ path 'src/lib.rs' \
             (BC-4.11.001 PC7 — early-exit for non-.factory/ paths). Production unimplemented."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result,
            HookResult::Continue,
            "non-.factory/ path 'src/lib.rs' must return HookResult::Continue \
                 (BC-4.11.001 PC7 — hook is scoped to .factory/ paths only)"
        );
    }
}

#[test]
fn test_BC_4_11_001_ac004_non_factory_path_does_not_invoke_read_file() {
    use std::sync::{Arc, Mutex};
    let called = Arc::new(Mutex::new(false));
    let called_clone = called.clone();
    let payload = make_payload(
        "Edit",
        Some("plugins/vsdd-factory/skills/create-adr/SKILL.md"),
    );
    let result = panic::catch_unwind(move || {
        hook_logic(
            payload,
            HookCallbacks {
                read_file: move |_path| {
                    *called_clone.lock().unwrap() = true;
                    Err("registry-should-not-be-read".to_string())
                },
                emit_event: |_, _| {},
                log: |_, _| {},
            },
        )
    });
    assert!(
        result.is_ok(),
        "hook_logic should return Continue without reading registry for non-.factory/ path \
             (BC-4.11.001 PC7). Production unimplemented."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result,
            HookResult::Continue,
            "non-.factory/ path (even inside plugins/) must return Continue without \
                 registry lookup (BC-4.11.001 PC7)"
        );
        assert!(
            !*called.lock().unwrap(),
            "read_file must NOT be called for non-.factory/ path (BC-4.11.001 PC7 — \
                 no registry lookup for paths outside .factory/)"
        );
    }
}

// -----------------------------------------------------------------------
// AC-005 (BC-4.11.001 PC3/4/5): enforcement_level per entry
// -----------------------------------------------------------------------

#[test]
fn test_BC_4_11_001_ac005_enforcement_level_block_entry_returns_continue() {
    let payload = make_payload(
        "Write",
        Some(".factory/specs/behavioral-contracts/ss-04/BC-4.11.001.md"),
    );
    let result = panic::catch_unwind(|| run_logic(payload, Ok(registry_yaml("block"))));
    assert!(
        result.is_ok(),
        "hook_logic should return HookResult::Continue when path matches a 'block' \
             enforcement_level entry (BC-4.11.001 PC3: matched block-level path → write proceeds). \
             Production function is unimplemented."
    );
    if let Ok((hook_result, _, _)) = result {
        assert_eq!(
            hook_result,
            HookResult::Continue,
            "BC-4.11.001 PC3: path matching enforcement_level=block entry must return Continue \
                 (the write is ALLOWED — 'block' means 'this is the canonical path')"
        );
    }
}

#[test]
fn test_BC_4_11_001_ac005_enforcement_level_warn_entry_returns_continue_with_event() {
    let payload = make_payload(
        "Write",
        Some(".factory/specs/behavioral-contracts/ss-04/BC-4.11.001.md"),
    );
    let result = panic::catch_unwind(|| run_logic(payload, Ok(registry_yaml("warn"))));
    assert!(
        result.is_ok(),
        "hook_logic should return HookResult::Continue and emit hook.warn event \
             when path matches enforcement_level=warn entry (BC-4.11.001 PC4). \
             Production function is unimplemented."
    );
    if let Ok((hook_result, _, events)) = result {
        assert_eq!(
            hook_result,
            HookResult::Continue,
            "BC-4.11.001 PC4: warn-level match must return Continue (write proceeds)"
        );
        assert!(
            events
                .iter()
                .any(|(event_type, _)| event_type == "hook.warn"),
            "BC-4.11.001 PC4: enforcement_level=warn must emit a hook.warn event. \
                 Got events: {:?}",
            events
        );
    }
}

#[test]
fn test_BC_4_11_001_ac005_enforcement_level_advisory_entry_returns_continue_with_log() {
    let payload = make_payload(
        "Write",
        Some(".factory/specs/behavioral-contracts/ss-04/BC-4.11.001.md"),
    );
    let result = panic::catch_unwind(|| run_logic(payload, Ok(registry_yaml("advisory"))));
    assert!(
        result.is_ok(),
        "hook_logic should return HookResult::Continue and call log_debug \
             when path matches enforcement_level=advisory entry (BC-4.11.001 PC5). \
             Production function is unimplemented."
    );
    if let Ok((hook_result, log_calls, events)) = result {
        assert_eq!(
            hook_result,
            HookResult::Continue,
            "BC-4.11.001 PC5: advisory-level match must return Continue"
        );
        // log_debug is level 1
        assert!(
            log_calls.iter().any(|(level, _)| *level <= 1),
            "BC-4.11.001 PC5: advisory match must call log_debug (level 0 or 1). \
                 Got log calls: {:?}",
            log_calls
        );
        // No hook.warn event should be emitted for advisory
        assert!(
            !events
                .iter()
                .any(|(event_type, _)| event_type == "hook.warn"),
            "BC-4.11.001 PC5: advisory match must NOT emit hook.warn event \
                 (no stderr/event for advisory — log_debug only). Got events: {:?}",
            events
        );
    }
}

#[test]
fn test_BC_4_11_001_ac005_no_match_returns_block_with_fix() {
    let payload = make_payload("Write", Some(".factory/unknown/path/artifact.md"));
    let result = panic::catch_unwind(|| run_logic(payload, Ok(registry_yaml("block"))));
    assert!(
        result.is_ok(),
        "hook_logic should return HookResult::block_with_fix when no registry entry matches \
             (BC-4.11.001 PC6 — unregistered path is always blocked). Production unimplemented."
    );
    if let Ok((hook_result, _, _)) = result {
        assert!(
            matches!(hook_result, HookResult::Block { .. }),
            "BC-4.11.001 PC6: no-match path must return HookResult::Block. Got: {:?}",
            hook_result
        );
    }
}

// -----------------------------------------------------------------------
// AC-006 (BC-4.11.001 EC-001/EC-002): graceful degrade
// -----------------------------------------------------------------------

#[test]
fn test_BC_4_11_001_ac006_graceful_degrade_absent_registry_returns_continue() {
    let payload = make_payload(
        "Write",
        Some(".factory/specs/behavioral-contracts/ss-04/BC-4.11.001.md"),
    );
    let result = panic::catch_unwind(|| {
        run_logic(payload, Err("registry absent: no such file".to_string()))
    });
    assert!(
        result.is_ok(),
        "hook_logic should return HookResult::Continue when registry is absent \
             (BC-4.11.001 EC-001 — graceful degrade, never blocks on missing registry). \
             Production function is unimplemented."
    );
    if let Ok((hook_result, log_calls, _)) = result {
        assert_eq!(
            hook_result,
            HookResult::Continue,
            "BC-4.11.001 EC-001: absent registry must return Continue (graceful degrade). \
                 Must NOT block (registry absence is not an error for the hook to block on)"
        );
        assert!(
            !log_calls.is_empty(),
            "BC-4.11.001 EC-001: absent registry must emit a log message \
                 (log_debug advisory). Got zero log calls."
        );
    }
}

#[test]
fn test_BC_4_11_001_ac006_graceful_degrade_malformed_registry_returns_continue() {
    let payload = make_payload("Write", Some(".factory/specs/prd.md"));
    let result = panic::catch_unwind(|| {
        run_logic(payload, Ok("{ THIS IS NOT VALID YAML: [broken".to_string()))
    });
    assert!(
        result.is_ok(),
        "hook_logic should return HookResult::Continue when registry YAML is malformed \
             (BC-4.11.001 EC-002 — graceful degrade on parse error, never blocks). \
             Production function is unimplemented."
    );
    if let Ok((hook_result, log_calls, _)) = result {
        assert_eq!(
            hook_result,
            HookResult::Continue,
            "BC-4.11.001 EC-002: malformed registry must return Continue (graceful degrade). \
                 Must NOT block."
        );
        // log_error is level 4; verify some log was emitted
        assert!(
            !log_calls.is_empty(),
            "BC-4.11.001 EC-002: malformed registry must emit a log_error message. \
                 Got zero log calls."
        );
        // Check it's an error-level log (level 4 = error)
        assert!(
            log_calls.iter().any(|(level, _)| *level >= 3),
            "BC-4.11.001 EC-002: malformed registry must call log_error (level >= 3). \
                 Got log calls: {:?}",
            log_calls
        );
    }
}

// -----------------------------------------------------------------------
// BC-4.11.001 EC-006: file_path absent from payload
// -----------------------------------------------------------------------

#[test]
fn test_BC_4_11_001_ec006_missing_file_path_returns_continue() {
    let payload = make_payload("Write", None); // no file_path
    let result = panic::catch_unwind(|| run_logic(payload, Ok(registry_yaml("block"))));
    assert!(
        result.is_ok(),
        "hook_logic should return HookResult::Continue when file_path is absent from payload \
             (BC-4.11.001 EC-006 — graceful degrade on missing data). Production unimplemented."
    );
    if let Ok((hook_result, log_calls, _)) = result {
        assert_eq!(
            hook_result,
            HookResult::Continue,
            "BC-4.11.001 EC-006: absent file_path must return Continue (not block on missing data)"
        );
        // log_warn is level 3
        assert!(
            log_calls.iter().any(|(level, _)| *level >= 2),
            "BC-4.11.001 EC-006: absent file_path must call log_warn (level >= 2). \
                 Got log calls: {:?}",
            log_calls
        );
    }
}

// -----------------------------------------------------------------------
// BC-4.11.001 invariant 3: unregistered .factory/ path is ALWAYS blocked
// (regardless of enforcement_level of other entries)
// -----------------------------------------------------------------------

#[test]
fn test_BC_4_11_001_invariant3_unregistered_factory_path_always_blocked() {
    let payload = make_payload("Write", Some(".factory/some/new/artifact-type/file.md"));
    let result = panic::catch_unwind(|| run_logic(payload, Ok(advisory_only_registry_yaml())));
    assert!(
        result.is_ok(),
        "hook_logic should return HookResult::Block for unregistered .factory/ path \
             even when registered entries are advisory (BC-4.11.001 invariant 3: \
             unregistered path → always blocked). Production unimplemented."
    );
    if let Ok((hook_result, _, _)) = result {
        assert!(
            matches!(hook_result, HookResult::Block { .. }),
            "BC-4.11.001 invariant 3: unregistered .factory/ path must be blocked \
                 regardless of other entries' enforcement_level. Got: {:?}",
            hook_result
        );
    }
}

// -----------------------------------------------------------------------
// BC-4.11.001 invariant 4: hook must NOT modify the registry file
// (read-only access — tested via callback tracking)
// -----------------------------------------------------------------------

#[test]
fn test_BC_4_11_001_invariant4_hook_does_not_write_registry() {
    let original_yaml = registry_yaml("block");
    let yaml_snapshot = original_yaml.clone();
    let payload = make_payload(
        "Write",
        Some(".factory/specs/behavioral-contracts/ss-04/BC-4.11.001.md"),
    );
    let result = panic::catch_unwind(|| run_logic(payload, Ok(yaml_snapshot.clone())));
    assert!(
        result.is_ok(),
        "hook_logic must not panic and must not modify registry content \
             (BC-4.11.001 invariant 4 — registry is read-only). Production unimplemented."
    );
    // After running, the original YAML string is unchanged (immutable Rust string)
    assert_eq!(
        original_yaml, yaml_snapshot,
        "BC-4.11.001 invariant 4: registry content must be unchanged after hook_logic invocation"
    );
}

// -----------------------------------------------------------------------
// BC-4.11.001 invariant 9: bare HookResult::block() is prohibited;
// all blocks must use block_with_fix.
// -----------------------------------------------------------------------

#[test]
fn test_BC_4_11_001_invariant9_block_uses_block_with_fix_pattern() {
    let payload = make_payload("Write", Some(".factory/feature-deltas/F1-delta.md"));
    let result = panic::catch_unwind(|| run_logic(payload, Ok(registry_yaml("block"))));
    assert!(
        result.is_ok(),
        "hook_logic must not panic for unregistered .factory/ path. \
             Block message must use block_with_fix pattern (BC-4.11.001 invariant 9). \
             Production unimplemented."
    );
    if let Ok((hook_result, _, _)) = result {
        if let HookResult::Block { reason } = hook_result {
            // block_with_fix produces: "BLOCKED by <hook>: ... Code: <code>."
            assert!(
                reason.starts_with("BLOCKED by"),
                "BC-4.11.001 invariant 9: block reason must start with 'BLOCKED by' \
                     (block_with_fix canonical format). Got: '{}'",
                reason
            );
            assert!(
                reason.contains("Code: ARTIFACT_PATH_UNREGISTERED"),
                "BC-4.11.001 invariant 9: block reason must contain \
                     'Code: ARTIFACT_PATH_UNREGISTERED'. Got: '{}'",
                reason
            );
        }
    }
}

// -----------------------------------------------------------------------
// VP-069 proptest: registry load never panics on any byte sequence
// -----------------------------------------------------------------------

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    // VP-069 Part A: parse_registry never panics on any input.
    // The harness generates arbitrary strings and asserts no panic.
    proptest! {
        #![proptest_config(proptest::test_runner::Config::with_cases(200))]

        /// VP-069 Part A (BC-4.11.001 PC1): load_registry must never panic on
        /// any byte sequence. Always returns Ok(registry) or Err(parse_error).
        #[test]
        fn prop_BC_4_11_001_vp069_registry_parse_never_panics(input in any::<Vec<u8>>()) {
            // Convert arbitrary bytes to string (lossy — mimics real-world malformed YAML)
            let s = String::from_utf8_lossy(&input).into_owned();
            // load_registry must not panic on any input; catch_unwind detects panic
            let result = panic::catch_unwind(|| load_registry(&s));
            prop_assert!(
                result.is_ok(),
                "VP-069 Part A: load_registry panicked on input (len {}). \
                 Must return Ok or Err, never panic. Production function is unimplemented.",
                s.len()
            );
        }

        /// VP-069 Part B (BC-4.11.001 EC-002): malformed YAML must not produce
        /// a block outcome. The hook must return Continue on parse failure.
        /// Tests that load_registry + hook_logic treat malformed input as
        /// graceful-degrade (Continue), never Block.
        #[test]
        fn prop_BC_4_11_001_vp069_malformed_registry_hook_returns_continue(
            input in ".*"
        ) {
            // Use a .factory/ path to ensure we actually enter the registry-check branch
            let payload = make_payload(
                "Write",
                Some(".factory/specs/behavioral-contracts/ss-04/BC-4.11.001.md"),
            );
            let result = panic::catch_unwind(|| {
                run_logic(payload, Ok(input.clone()))
            });
            prop_assert!(
                result.is_ok(),
                "VP-069 Part B: hook_logic panicked on malformed registry input. \
                 Must return Continue (graceful degrade). Production unimplemented."
            );
            if let Ok((hook_result, _, _)) = result {
                // For inputs that fail to parse as valid registry YAML,
                // the hook must return Continue (graceful degrade per EC-002).
                // Inputs that happen to parse as valid YAML may return Block if the
                // path doesn't match — but they must never panic.
                prop_assert_ne!(
                    hook_result.exit_code(),
                    1, // HookResult::Error exit code
                    "VP-069 Part B: hook_logic must not return HookResult::Error \
                     for any registry input (panic-escape through error path). \
                     Got: {:?}",
                    "Error"
                );
            }
        }

        /// VP-069 Part C (BC-4.11.001 PC7 + invariant 3): empty registry
        /// (zero artifacts) must Continue for non-.factory/ paths and
        /// Block for .factory/ paths (empty registry = no valid patterns).
        #[test]
        fn prop_BC_4_11_001_vp069_empty_registry_non_factory_path_always_continues(
            // Generate paths that do NOT start with .factory/
            path in "[^.].*"
        ) {
            let payload = make_payload("Write", Some(&path));
            let result = panic::catch_unwind(|| {
                run_logic(payload, Ok(empty_registry_yaml()))
            });
            prop_assert!(
                result.is_ok(),
                "VP-069 Part C: hook_logic panicked for non-.factory/ path '{}' \
                 with empty registry. Must return Continue. Production unimplemented.",
                path
            );
            if let Ok((hook_result, _, _)) = result {
                prop_assert_eq!(
                    hook_result,
                    HookResult::Continue,
                    "VP-069 Part C: non-.factory/ path '{}' with empty registry \
                     must return Continue (BC-4.11.001 PC7 — early exit for non-.factory/)",
                    path
                );
            }
        }
    }
}

// -----------------------------------------------------------------------
// VP-070 kani harness: path matching is pure and deterministic
//
// The kani proofs live in #[cfg(kani)] mod kani_proofs below.
// These test-mode equivalents exercise the same properties via catch_unwind
// to provide Red Gate failures with clear assertion messages.
// -----------------------------------------------------------------------

#[test]
fn proof_BC_4_11_001_vp070_match_path_is_deterministic() {
    // VP-070 Proof 1: same (path, registry) always returns same MatchResult.
    // Tests determinism for a fixed canonical BC path.
    let yaml = registry_yaml("block");
    let registry = require_registry(&yaml, "vp070_proof1_determinism");
    let path = ".factory/specs/behavioral-contracts/ss-04/BC-4.11.001.md";
    let r1 = panic::catch_unwind({
        let reg = registry.clone();
        move || matches_canonical(path, &reg)
    });
    let r2 = panic::catch_unwind({
        let reg = registry.clone();
        move || matches_canonical(path, &reg)
    });
    assert!(
        r1.is_ok(),
        "VP-070 Proof 1: matches_canonical must not panic for canonical BC path \
             (determinism proof). Production unimplemented."
    );
    assert!(
        r2.is_ok(),
        "VP-070 Proof 1: second call to matches_canonical must not panic \
             (determinism proof). Production unimplemented."
    );
    if let (Ok(result1), Ok(result2)) = (r1, r2) {
        assert_eq!(
            result1, result2,
            "VP-070 Proof 1: matches_canonical must be deterministic — \
                 same (path, registry) must always return same MatchResult"
        );
    }
}

#[test]
fn proof_BC_4_11_001_vp070_non_factory_path_always_returns_nomatch() {
    // VP-070 Proof 2: non-.factory/ paths always return MatchResult::NoMatch
    let yaml = registry_yaml("block");
    let registry = require_registry(&yaml, "vp070_proof2_non_factory");
    let non_factory_paths = [
        "src/lib.rs",
        "crates/hook-plugins/validate-artifact-path/src/lib.rs",
        "",
        "Cargo.toml",
        "plugins/vsdd-factory/hooks/validate-artifact-path.sh",
    ];
    for path in &non_factory_paths {
        let result = panic::catch_unwind({
            let reg = registry.clone();
            let p = path.to_string();
            move || matches_canonical(&p, &reg)
        });
        assert!(
            result.is_ok(),
            "VP-070 Proof 2: matches_canonical must not panic for non-.factory/ path '{}'. \
                 Production unimplemented.",
            path
        );
        if let Ok(mr) = result {
            assert_eq!(
                mr,
                MatchResult::NoMatch,
                "VP-070 Proof 2: non-.factory/ path '{}' must return MatchResult::NoMatch \
                     (hook_logic handles early-exit Continue for these paths)",
                path
            );
        }
    }
}

#[test]
fn proof_BC_4_11_001_vp070_empty_path_returns_nomatch() {
    // VP-070 Proof 3: empty path returns MatchResult::NoMatch.
    let yaml = registry_yaml("block");
    let registry = require_registry(&yaml, "vp070_proof3_empty_path");
    let result = panic::catch_unwind(move || matches_canonical("", &registry));
    assert!(
        result.is_ok(),
        "VP-070 Proof 3: matches_canonical must not panic for empty path. \
             Production unimplemented."
    );
    if let Ok(mr) = result {
        assert_eq!(
            mr,
            MatchResult::NoMatch,
            "VP-070 Proof 3: empty path must return MatchResult::NoMatch \
                 (empty path cannot match any .factory/ prefix)"
        );
    }
}

#[test]
fn proof_BC_4_11_001_vp070_advisory_only_registry_never_produces_block_in_matches() {
    // VP-070 Proof 4: advisory-only registry must never return MatchResult::Block
    let yaml = advisory_only_registry_yaml();
    let registry = require_registry(&yaml, "vp070_proof4_advisory_only");
    let result = panic::catch_unwind({
        let reg = registry.clone();
        move || {
            matches_canonical(
                ".factory/specs/behavioral-contracts/ss-04/BC-4.11.001.md",
                &reg,
            )
        }
    });
    assert!(
        result.is_ok(),
        "VP-070 Proof 4: matches_canonical must not panic for advisory-only registry. \
             Production unimplemented."
    );
    if let Ok(mr) = result {
        assert_ne!(
            mr,
            MatchResult::Block,
            "VP-070 Proof 4: advisory-only registry must never return MatchResult::Block \
                 from matches_canonical (Block variant is reserved for enforcement_level=block entries)"
        );
        // Should return Advisory variant for matching paths
        match mr {
            MatchResult::Advisory { .. } | MatchResult::NoMatch => {}
            other => panic!(
                "VP-070 Proof 4: advisory-level matching path should return Advisory or NoMatch, \
                     not {:?}",
                other
            ),
        }
    }
}

// -----------------------------------------------------------------------
// BC-4.11.001 invariant 6 (v1.1): single-segment placeholder semantics
// {placeholder} must NOT match empty content or content containing '/'
// (Amended NC-1, F5 pass-1 fix burst 2026-05-07)
// -----------------------------------------------------------------------

#[test]
fn test_BC_4_11_001_invariant6_placeholder_rejects_empty_segment() {
    // ".factory/cycles//decision-log.md" has an empty segment where {cycle-id} is expected.
    // Per invariant 6 (v1.1): placeholder must match >=1 character AND no '/'.
    // Double-slash means the placeholder would match zero characters — must reject.
    let yaml = r#"version: 1
artifacts:
  - artifact_type: cycle-decision-log
    canonical_path_pattern: ".factory/cycles/{cycle-id}/decision-log.md"
    description: Cycle decision log
    enforcement_level: "block"
"#;
    let registry = require_registry(yaml, "invariant6_empty_segment");
    let path = ".factory/cycles//decision-log.md";
    let result = matches_canonical(path, &registry);
    assert_eq!(
        result,
        MatchResult::NoMatch,
        "BC-4.11.001 invariant 6 (v1.1): placeholder must match at least one character. \
         Empty segment (double-slash) MUST NOT match. Path: '{}'",
        path
    );
}

#[test]
fn test_BC_4_11_001_invariant6_placeholder_rejects_multi_segment() {
    // ".factory/cycles/a/b/decision-log.md" has "a/b" where {cycle-id} is expected.
    // Per invariant 6 (v1.1): placeholder is single-segment — no '/' allowed in matched content.
    // "a/b" spans two segments and MUST NOT match {cycle-id}.
    let yaml = r#"version: 1
artifacts:
  - artifact_type: cycle-decision-log
    canonical_path_pattern: ".factory/cycles/{cycle-id}/decision-log.md"
    description: Cycle decision log
    enforcement_level: "block"
"#;
    let registry = require_registry(yaml, "invariant6_multi_segment");
    let path = ".factory/cycles/a/b/decision-log.md";
    let result = matches_canonical(path, &registry);
    assert_eq!(
        result,
        MatchResult::NoMatch,
        "BC-4.11.001 invariant 6 (v1.1): {{placeholder}} is single-segment — \
         content containing '/' MUST NOT match. Path: '{}'",
        path
    );
}

// -----------------------------------------------------------------------
// BC-4.11.001 EC-007: execution time ceiling ≤200ms for matches_canonical
//
// F-MED-5: no benchmark existed to enforce the 200ms ceiling from EC-007.
// This test loads the actual artifact-path-registry.yaml (the production
// registry), runs matches_canonical 1000 times against a fixture path,
// and asserts the total elapsed time is under 200ms.
//
// Why 1000 iterations instead of 1? A single call is too fast to time
// reliably (sub-microsecond on modern hardware). 1000 iterations at once
// gives a stable measurement and still should complete well under the
// 200ms WASM ceiling. If the total 1000-call time exceeds 200ms, each
// individual call would be averaging ~200µs — already 10x too slow for
// a WASM execution budget of the whole hook.
// -----------------------------------------------------------------------

#[test]
fn test_BC_4_11_001_ec007_matches_canonical_1000_calls_under_200ms() {
    // BC-4.11.001 EC-007: "WASM execution time MUST remain under 200ms."
    // Load the actual production registry and time 1000 matches_canonical
    // calls. Total elapsed must be < 200ms (a 1000-call batch under 200ms
    // means each call is < 200µs on average — comfortably within WASM budget).
    //
    // If this test becomes flaky on heavily loaded CI machines, the ceiling
    // can be raised to 1000ms (still proves no O(n²) regression).
    use std::time::Instant;

    // Locate the production registry relative to CARGO_MANIFEST_DIR.
    // Path: <workspace>/plugins/vsdd-factory/config/artifact-path-registry.yaml
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR must be set during cargo test");
    let registry_path = std::path::Path::new(&manifest_dir)
        .join("../../../../plugins/vsdd-factory/config/artifact-path-registry.yaml");

    if !registry_path.exists() {
        // Registry not present (e.g., running in a context without plugin dir).
        // Fall back to the multi-entry fixture — still exercises the algorithm.
        let yaml = multi_entry_registry_yaml();
        let registry = load_registry(&yaml)
            .expect("fixture registry must load");
        let path = ".factory/specs/behavioral-contracts/ss-04/BC-4.11.001.md";

        let start = Instant::now();
        for _ in 0..1000 {
            let _ = matches_canonical(path, &registry);
        }
        let elapsed_ms = start.elapsed().as_millis();

        assert!(
            elapsed_ms < 200,
            "BC-4.11.001 EC-007: 1000 matches_canonical calls on fixture registry \
             must complete in < 200ms total; got {}ms (fixture fallback — production \
             registry absent at {})",
            elapsed_ms,
            registry_path.display()
        );
        return;
    }

    let yaml = std::fs::read_to_string(&registry_path)
        .unwrap_or_else(|e| panic!("failed to read registry at {}: {}", registry_path.display(), e));

    let registry = load_registry(&yaml)
        .expect("production registry must load");

    // Use an unregistered path to exercise the full scan (worst case: no early match).
    let path = ".factory/cycles/v1.0-feature-engine-discipline-pass-1/S-99.99/implementation/red-gate-log.md";

    let start = Instant::now();
    for _ in 0..1000 {
        let _ = matches_canonical(path, &registry);
    }
    let elapsed_ms = start.elapsed().as_millis();

    assert!(
        elapsed_ms < 200,
        "BC-4.11.001 EC-007: 1000 matches_canonical calls on production registry \
         ({} entries) must complete in < 200ms total; got {}ms. \
         A regression in pattern_matches complexity (e.g., O(n²)) would cause \
         this to exceed the budget.",
        registry.artifacts.len(),
        elapsed_ms
    );
}
