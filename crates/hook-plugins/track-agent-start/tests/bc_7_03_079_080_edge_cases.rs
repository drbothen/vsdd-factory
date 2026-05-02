//! BC-7.03.079 / BC-7.03.080 edge-case tests for track-agent-start (S-8.08).
//!
//! **Red Gate scope:** these tests cover payload edge cases NOT exercised by the
//! stub-architect's 17 in-crate unit tests.  Specifically:
//!
//! - EC-001 (null JSON subagent_type): explicit `null` in the JSON object must
//!   be treated as absent → default "unknown".  The stub's EC-001 test covers
//!   *absent key* but NOT *explicit JSON null*.
//!
//! - EC-002 (null JSON prompt): explicit `null` must be treated as absent → "".
//!   Same gap as EC-001: absent key vs. explicit null are different code paths
//!   in `.and_then(|v| v.as_str())`.
//!
//! - EC-004 (subagent_type is a non-string JSON type — integer): `.as_str()`
//!   returns None on integers; hook must still default to "unknown" and emit.
//!
//! - EC-004 / non-Agent tool_name: tool_input may be an empty JSON object `{}`
//!   — the guard fires before any tool_input access; confirms no panic.
//!
//! - Invariant: `track_agent_start_logic` always returns `HookResult::Continue`
//!   regardless of the emit_fn panicking (best-effort AC-006 boundary test).
//!   (Note: in practice emit_fn never panics in production; this test documents
//!   the behavioral boundary.)
//!
//! All tests that PASS in the current stub state are labeled `[regression gate]`.
//! Tests that should fail until implementation changes are labeled `[red gate]`.
//! In practice ALL pass here because the logic is already correct in the stub —
//! the Red Gate for S-8.08 is primarily the registry/filesystem tests in the
//! factory-dispatcher test suite.
//!
//! BC: BC-7.03.079, BC-7.03.080
//! Story: S-8.08

use track_agent_start::track_agent_start_logic;
use vsdd_hook_sdk::{HookPayload, HookResult};

fn make_payload(tool_name: &str, tool_input: serde_json::Value) -> HookPayload {
    serde_json::from_value(serde_json::json!({
        "event_name": "PreToolUse",
        "tool_name": tool_name,
        "session_id": "test-session",
        "dispatcher_trace_id": "test-trace",
        "tool_input": tool_input
    }))
    .expect("fixture must deserialize")
}

/// Collect (event_type, fields) tuples from a logic run.
#[allow(clippy::type_complexity)]
fn run(payload: HookPayload) -> (HookResult, Vec<(String, Vec<(String, String)>)>) {
    let mut calls = Vec::new();
    let result = track_agent_start_logic(payload, |event_type, fields| {
        calls.push((
            event_type.to_string(),
            fields
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        ));
    });
    (result, calls)
}

// ---------------------------------------------------------------------------
// EC-001: explicit JSON null for subagent_type
// ---------------------------------------------------------------------------

/// EC-001 [regression gate]: explicit JSON `null` for `subagent_type` must be
/// treated identically to an absent key — defaults to "unknown".
///
/// The stub's in-crate test covers the absent-key path:
///   `"prompt": "S-8.08"` (no subagent_type key at all).
/// This test covers the explicit-null path:
///   `"subagent_type": null`
/// Both exercise `.and_then(|v| v.as_str())` but through different branches:
///   - Absent key → `.get("subagent_type")` returns `None`
///   - JSON null  → `.get("subagent_type")` returns `Some(Value::Null)`
///     → `.as_str()` on Null returns `None`
///
/// Both must produce "unknown" (bash: `jq -r '.tool_input.subagent_type // "unknown"'`).
#[test]
fn test_BC_7_03_080_ec001_explicit_null_subagent_type_defaults_to_unknown() {
    let payload = make_payload(
        "Agent",
        serde_json::json!({
            "subagent_type": null,  // explicit null — not absent key
            "prompt": "working on S-8.08"
        }),
    );
    let (result, calls) = run(payload);

    assert!(
        matches!(result, HookResult::Continue),
        "EC-001: null subagent_type must result in Continue (exit 0)"
    );
    assert_eq!(calls.len(), 1, "EC-001: event must be emitted");

    let (event_type, fields) = &calls[0];
    assert_eq!(event_type, "agent.start");

    let field_map: std::collections::HashMap<&str, &str> = fields
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    assert_eq!(
        field_map.get("subagent"),
        Some(&"unknown"),
        "EC-001: explicit JSON null subagent_type must produce subagent=\"unknown\" \
         (same as absent key — bash jq // \"unknown\" fallback)"
    );
    // story_id extracted from "working on S-8.08"
    assert_eq!(
        field_map.get("story_id"),
        Some(&"S-8.08"),
        "EC-001: story_id should still be extracted from prompt even when subagent_type is null"
    );
}

// ---------------------------------------------------------------------------
// EC-002: explicit JSON null for prompt
// ---------------------------------------------------------------------------

/// EC-002 [regression gate]: explicit JSON `null` for `prompt` must be treated
/// as absent — no story_id extracted, event emitted without story_id field.
///
/// The stub's in-crate test covers the absent-key path (no `prompt` key at all).
/// This test covers the explicit-null path: `"prompt": null`.
#[test]
fn test_BC_7_03_080_ec002_explicit_null_prompt_treated_as_empty_no_story_id() {
    let payload = make_payload(
        "Agent",
        serde_json::json!({
            "subagent_type": "reviewer",
            "prompt": null  // explicit null — not absent key
        }),
    );
    let (result, calls) = run(payload);

    assert!(
        matches!(result, HookResult::Continue),
        "EC-002: null prompt must result in Continue (exit 0)"
    );
    assert_eq!(
        calls.len(),
        1,
        "EC-002: event must be emitted (null prompt only)"
    );

    let (_, fields) = &calls[0];
    let field_map: std::collections::HashMap<&str, &str> = fields
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();

    assert!(
        !field_map.contains_key("story_id"),
        "EC-002: null prompt must not produce a story_id field \
         (bash: jq -r '.tool_input.prompt // \"\"' returns empty string for null)"
    );
    assert_eq!(
        field_map.get("subagent"),
        Some(&"reviewer"),
        "EC-002: subagent extracted normally even when prompt is null"
    );
}

// ---------------------------------------------------------------------------
// EC-001 variant: subagent_type is a non-string JSON type (integer)
// ---------------------------------------------------------------------------

/// EC-001 variant [regression gate]: `subagent_type` carrying a non-string JSON
/// value (integer 42) must be treated as absent → default "unknown".
///
/// `.as_str()` returns `None` for JSON integers.  This path is distinct from
/// the null and absent-key paths but must produce the same default.
#[test]
fn test_BC_7_03_080_ec001_non_string_subagent_type_defaults_to_unknown() {
    let payload = make_payload(
        "Agent",
        serde_json::json!({
            "subagent_type": 42,  // integer — not a string
            "prompt": ""
        }),
    );
    let (result, calls) = run(payload);

    assert!(matches!(result, HookResult::Continue));
    assert_eq!(calls.len(), 1);

    let (_, fields) = &calls[0];
    let field_map: std::collections::HashMap<&str, &str> = fields
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    assert_eq!(
        field_map.get("subagent"),
        Some(&"unknown"),
        "EC-001 variant: integer subagent_type must default to \"unknown\" \
         (.as_str() returns None for non-string JSON types)"
    );
}

// ---------------------------------------------------------------------------
// EC-004: tool_input is entirely absent (serde default = Null)
// ---------------------------------------------------------------------------

/// EC-004 [regression gate]: when `tool_input` is absent from the JSON envelope,
/// serde deserializes it as `Value::Null`.  The hook must still guard on
/// `tool_name != "Agent"` correctly and return HookResult::Continue with no event
/// for non-Agent tool_names.
///
/// Verifies that the `payload.tool_input.get("subagent_type")` call does NOT
/// panic when `tool_input` is `Value::Null` (`.get` on Null returns None).
#[test]
fn test_BC_7_03_079_ec004_absent_tool_input_does_not_panic_non_agent() {
    // Simulate a non-Agent PreToolUse with no tool_input at all
    let payload: HookPayload = serde_json::from_value(serde_json::json!({
        "event_name": "PreToolUse",
        "tool_name": "Bash",
        "session_id": "s",
        "dispatcher_trace_id": "t"
        // tool_input absent → serde(default) → Value::Null
    }))
    .expect("fixture must parse");

    let (result, calls) = run(payload);
    assert!(
        matches!(result, HookResult::Continue),
        "EC-004: absent tool_input with non-Agent tool_name must Continue"
    );
    assert!(calls.is_empty(), "no event for non-Agent tool_name");
}

/// EC-004 [regression gate]: when `tool_input` is `Value::Null` but tool_name
/// IS "Agent", the hook must default subagent_type to "unknown" without panicking.
#[test]
fn test_BC_7_03_079_ec004_null_tool_input_with_agent_tool_name_uses_defaults() {
    let payload: HookPayload = serde_json::from_value(serde_json::json!({
        "event_name": "PreToolUse",
        "tool_name": "Agent",
        "session_id": "s",
        "dispatcher_trace_id": "t"
        // tool_input absent → Value::Null; .get() on Null returns None → defaults apply
    }))
    .expect("fixture must parse");

    let (result, calls) = run(payload);
    assert!(matches!(result, HookResult::Continue));
    assert_eq!(
        calls.len(),
        1,
        "Agent dispatch: event emitted even with null tool_input"
    );

    let (event_type, fields) = &calls[0];
    assert_eq!(event_type, "agent.start");
    let field_map: std::collections::HashMap<&str, &str> = fields
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    assert_eq!(
        field_map.get("subagent"),
        Some(&"unknown"),
        "EC-004: null tool_input must default subagent to \"unknown\""
    );
    assert!(
        !field_map.contains_key("story_id"),
        "EC-004: null tool_input → no prompt → no story_id"
    );
}

// ---------------------------------------------------------------------------
// BC-7.03.079 invariant 2: best-effort — never panics
// ---------------------------------------------------------------------------

/// BC-7.03.079 invariant 2 [regression gate]: `track_agent_start_logic` must
/// return `HookResult::Continue` on every code path, including degenerate inputs
/// that could cause a panic in naive implementations.
///
/// Tests the following degenerate inputs that are NOT covered by the stub:
///   - tool_input is a JSON array (not an object)
///   - tool_input is a JSON boolean
///   - tool_input is a JSON string
#[test]
fn test_BC_7_03_079_invariant_2_always_continue_on_degenerate_tool_input_types() {
    let degenerate_inputs: &[serde_json::Value] = &[
        serde_json::json!([1, 2, 3]),  // array — .get() on Array returns None
        serde_json::json!(true),       // boolean
        serde_json::json!("a string"), // string
        serde_json::json!(0),          // integer
    ];

    for input in degenerate_inputs {
        let payload = make_payload("Agent", input.clone());
        let (result, calls) = run(payload);
        assert!(
            matches!(result, HookResult::Continue),
            "BC-7.03.079 invariant 2: must return Continue for degenerate tool_input: {input:?}"
        );
        assert_eq!(
            calls.len(),
            1,
            "BC-7.03.079: event must be emitted even with degenerate tool_input: {input:?}"
        );
        let (_, fields) = &calls[0];
        let field_map: std::collections::HashMap<&str, &str> = fields
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        assert_eq!(
            field_map.get("subagent"),
            Some(&"unknown"),
            "degenerate tool_input must produce subagent=unknown"
        );
    }
}

// ---------------------------------------------------------------------------
// AC-004: story_id extraction — additional canonical test vectors
// ---------------------------------------------------------------------------

/// AC-004 [regression gate]: story_id extraction — numeric-only variants that
/// the BC-7.03.080 canonical test vector table does not include.
///
/// These test the regex boundaries not explicitly called out in the stub's tests.
#[test]
fn test_BC_7_03_080_extract_story_id_boundary_variants() {
    use track_agent_start::extract_story_id;

    // Single-digit section and subsection
    assert_eq!(
        extract_story_id("S-1.2"),
        Some("S-1.2".to_string()),
        "single-digit S-N.NN variant must match"
    );
    // Large numbers
    assert_eq!(
        extract_story_id("S-100.200"),
        Some("S-100.200".to_string()),
        "large-number S-NNN.NNN variant must match"
    );
    // STORY- with single digit
    assert_eq!(
        extract_story_id("STORY-1"),
        Some("STORY-1".to_string()),
        "single-digit STORY-N variant must match"
    );
    // Pattern 1 wins when S-N.NN appears after STORY-NNN in prompt
    assert_eq!(
        extract_story_id("STORY-042 see also S-8.08"),
        Some("S-8.08".to_string()),
        "pattern 1 (S-N.NN) must win even when STORY-NNN appears earlier in prompt (EC-003)"
    );
    // Whitespace-only prompt
    assert_eq!(
        extract_story_id("   \t\n   "),
        None,
        "whitespace-only prompt must return None"
    );
    // Pattern embedded in words: regex must NOT require word boundaries
    // (bash uses `grep -oE` which matches within words too)
    assert_eq!(
        extract_story_id("ticketS-8.08done"),
        Some("S-8.08".to_string()),
        "S-N.NN embedded without whitespace must still match (grep -oE behavior)"
    );
}

// ---------------------------------------------------------------------------
// AC-002a: no forbidden fields on any code path
// ---------------------------------------------------------------------------

/// AC-002a [regression gate]: the forbidden fields (agent_id, tool_name) must
/// be absent in the emitted event on EVERY Agent dispatch path:
///   - with story_id
///   - without story_id
///   - with unknown subagent (null subagent_type)
///
/// This is a comprehensive sweep complementing the in-crate stub test
/// `test_BC_7_03_079_no_forbidden_fields_in_emission` which only covers one case.
#[test]
fn test_BC_7_03_079_ac002a_no_forbidden_fields_on_all_agent_dispatch_paths() {
    let test_cases = [
        (
            "with_story_id",
            make_payload(
                "Agent",
                serde_json::json!({"subagent_type": "pr-manager", "prompt": "S-6.07"}),
            ),
        ),
        (
            "without_story_id",
            make_payload(
                "Agent",
                serde_json::json!({"subagent_type": "reviewer", "prompt": "no pattern"}),
            ),
        ),
        (
            "null_subagent_type",
            make_payload(
                "Agent",
                serde_json::json!({"subagent_type": null, "prompt": "STORY-001"}),
            ),
        ),
        (
            "null_prompt",
            make_payload(
                "Agent",
                serde_json::json!({"subagent_type": "implementer", "prompt": null}),
            ),
        ),
    ];

    for (case_name, payload) in test_cases {
        let (_, calls) = run(payload);
        assert_eq!(calls.len(), 1, "{case_name}: event must be emitted");
        let (_, fields) = &calls[0];
        for (k, _) in fields {
            assert_ne!(
                k, "agent_id",
                "{case_name}: forbidden field agent_id must not appear (E-8 D-2 strict parity)"
            );
            assert_ne!(
                k, "tool_name",
                "{case_name}: forbidden field tool_name must not appear (E-8 D-2 strict parity)"
            );
        }
    }
}
