//! track-agent-start — PreToolUse:Agent WASM hook plugin.
//!
//! Emits `agent.start` telemetry on every Agent subagent dispatch per BC-7.03.080.
//!
//! Plugin-set fields (strict E-8 D-2 bash parity — lines 43-44 of track-agent-start.sh):
//!   - `hook`       — literal "track-agent-start"
//!   - `matcher`    — literal "Agent"
//!   - `subagent`   — `tool_input.subagent_type` (default: "unknown" per EC-001)
//!   - `story_id`   — optional; extracted from `tool_input.prompt` via two-pattern cascade:
//!     pattern 1: `S-[0-9]+\.[0-9]+` (e.g., S-8.03); pattern 2: `STORY-[0-9]+`
//!     (e.g., STORY-042); omitted if neither matches (EC-003).
//!
//! FORBIDDEN FIELDS (E-8 D-2 strict parity):
//!   - NO `agent_id` field
//!   - NO `tool_name` field
//!   - NO other additive fields beyond the bash source
//!
//! Best-effort semantics (BC-7.03.079 invariant 2 + AC-006):
//!   - JSON parse failure → handled by __internal::run; hook sees HookPayload (Continue)
//!   - tool_name != "Agent" → exit 0 immediately (EC-004)
//!   - host::emit_event failure → silently swallowed via `let _ = ...` (EC-006)
//!   - Missing prompt → treat as "" (EC-002)
//!   - Missing subagent_type → use "unknown" (EC-001)
//!
//! 4 host-enriched fields are auto-injected by `emit_event` (BC-1.05.012):
//!   `dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`.
//! 4 construction-time fields set by the dispatcher: `ts`, `ts_epoch`, `schema_version`, `type`.
//!
//! RESERVED_FIELDS the plugin MUST NOT set (8 total):
//!   Host-enriched: `dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`
//!   Construction-time: `ts`, `ts_epoch`, `schema_version`, `type`

use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// Story-id extraction (pure, testable without WASM runtime)
// ---------------------------------------------------------------------------

/// Extract story_id from a prompt string using the two-pattern cascade.
///
/// Pattern 1 (preferred): `S-[0-9]+\.[0-9]+` — e.g., S-8.03, S-6.07.
/// Pattern 2 (fallback):  `STORY-[0-9]+`     — e.g., STORY-042.
///
/// Returns the first match of pattern 1, or if absent, the first match of
/// pattern 2. Returns `None` if neither pattern matches (EC-003: omit field).
///
/// Per EC-003: if both patterns match, pattern 1 wins (S-N.NN first).
pub fn extract_story_id(prompt: &str) -> Option<String> {
    // Pattern 1: S-N.NN (e.g., S-8.03)
    let re1 = regex::Regex::new(r"S-[0-9]+\.[0-9]+").expect("static regex is valid");
    if let Some(m) = re1.find(prompt) {
        return Some(m.as_str().to_string());
    }

    // Pattern 2 (fallback): STORY-NNN (e.g., STORY-042)
    let re2 = regex::Regex::new(r"STORY-[0-9]+").expect("static regex is valid");
    if let Some(m) = re2.find(prompt) {
        return Some(m.as_str().to_string());
    }

    None
}

// ---------------------------------------------------------------------------
// Core hook logic (injectable emit callback — testable without WASM runtime)
// ---------------------------------------------------------------------------

/// Top-level track-agent-start hook logic with injectable emit callback.
///
/// Guards on `payload.tool_name == "Agent"`, extracts
/// `tool_input.subagent_type` and `tool_input.prompt`, and emits
/// `agent.start` via `emit_fn`.
///
/// All host function dependencies are injected so unit/integration tests can
/// drive every branch without a WASM runtime — same pattern as worktree-hooks
/// and session-end-telemetry.
///
/// Best-effort: returns `HookResult::Continue` (exit 0) on every code path
/// including missing fields and emit errors (BC-7.03.079 inv 2).
///
/// # Arguments
/// - `payload`: deserialized HookPayload from the PreToolUse:Agent envelope.
/// - `emit_fn`: called with (event_type, fields) to emit `agent.start`.
pub fn track_agent_start_logic<Emit>(payload: HookPayload, emit_fn: Emit) -> HookResult
where
    Emit: FnOnce(&str, &[(&str, &str)]),
{
    // Guard: tool_name must be "Agent" (EC-004).
    if payload.tool_name != "Agent" {
        return HookResult::Continue;
    }

    // Extract tool_input.subagent_type; default "unknown" (EC-001).
    let subagent = payload
        .tool_input
        .get("subagent_type")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();

    // Extract tool_input.prompt; default "" (EC-002).
    let prompt = payload
        .tool_input
        .get("prompt")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    // Extract story_id via two-pattern cascade (EC-003).
    let story_id = extract_story_id(prompt);

    // Build fields: strict E-8 D-2 bash parity (lines 43-44 of track-agent-start.sh).
    // FORBIDDEN: agent_id, tool_name, any additive field.
    let mut fields: Vec<(&str, &str)> = vec![
        ("hook", "track-agent-start"),
        ("matcher", "Agent"),
        ("subagent", subagent.as_str()),
    ];
    let story_id_ref;
    if let Some(ref s) = story_id {
        story_id_ref = s.as_str();
        fields.push(("story_id", story_id_ref));
    }

    // Emit agent.start. EC-006: emit error is silently swallowed (let _ = ...).
    // "type" is a construction-time RESERVED_FIELD set by the dispatcher —
    // we pass it as the event_type argument, not as a field tuple.
    emit_fn("agent.start", &fields);

    HookResult::Continue
}

// ---------------------------------------------------------------------------
// Top-level entry point called from main.rs (no callbacks — uses host fns).
// ---------------------------------------------------------------------------

/// Called from the WASI entry point in `main.rs`.
///
/// Wires the real vsdd_hook_sdk host functions to the injectable-callback
/// surface of `track_agent_start_logic`.
pub fn on_agent_start(payload: HookPayload) -> HookResult {
    track_agent_start_logic(payload, |event_type, fields| {
        vsdd_hook_sdk::host::emit_event(event_type, fields);
    })
}

// ---------------------------------------------------------------------------
// Unit tests for pure logic functions
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // extract_story_id tests (BC-7.03.080 postcondition 1 — extraction cascade)
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_7_03_080_extract_story_id_pattern1_wins() {
        // AC-004: prompt containing "S-6.07" -> story_id=S-6.07
        assert_eq!(
            extract_story_id("Working on S-6.07 now"),
            Some("S-6.07".to_string())
        );
    }

    #[test]
    fn test_BC_7_03_080_extract_story_id_pattern2_fallback() {
        // AC-004: prompt containing "STORY-042" (no S-N.NN) -> story_id=STORY-042
        assert_eq!(
            extract_story_id("See STORY-042 for details"),
            Some("STORY-042".to_string())
        );
    }

    #[test]
    fn test_BC_7_03_080_extract_story_id_absent() {
        // AC-004: prompt with no story pattern -> None (omit story_id field)
        assert_eq!(extract_story_id("no story reference here"), None);
    }

    #[test]
    fn test_BC_7_03_080_extract_story_id_pattern1_beats_pattern2() {
        // EC-003: both patterns present -> pattern 1 wins (S-N.NN first)
        assert_eq!(
            extract_story_id("S-8.03 and STORY-042"),
            Some("S-8.03".to_string())
        );
    }

    #[test]
    fn test_BC_7_03_080_extract_story_id_empty_prompt() {
        // EC-002: empty prompt -> None
        assert_eq!(extract_story_id(""), None);
    }

    #[test]
    fn test_BC_7_03_080_extract_story_id_first_match_only() {
        // Only first match of pattern 1 is used (bash: grep -oE ... | head -1)
        assert_eq!(
            extract_story_id("S-8.03 and S-8.04"),
            Some("S-8.03".to_string())
        );
    }

    // -----------------------------------------------------------------------
    // track_agent_start_logic tests (BC-7.03.079 + BC-7.03.080)
    // -----------------------------------------------------------------------

    /// Build a minimal HookPayload for testing.
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

    /// Helper: run the logic with a captured-emit callback.
    /// Returns (result, emitted_calls).
    fn run_logic(payload: HookPayload) -> (HookResult, Vec<(String, Vec<(String, String)>)>) {
        let mut calls: Vec<(String, Vec<(String, String)>)> = Vec::new();
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

    #[test]
    fn test_BC_7_03_079_rejects_non_agent_tool_name() {
        // EC-004: tool_name != "Agent" -> exit 0, no event emitted
        let payload = make_payload("Bash", serde_json::json!({"command": "ls"}));
        let (result, calls) = run_logic(payload);
        assert!(matches!(result, HookResult::Continue));
        assert!(calls.is_empty());
    }

    #[test]
    fn test_BC_7_03_079_rejects_empty_tool_name() {
        // tool_name "" (absent/default): not "Agent" -> exit 0, no event
        let payload = make_payload("", serde_json::json!({}));
        let (result, calls) = run_logic(payload);
        assert!(matches!(result, HookResult::Continue));
        assert!(calls.is_empty());
    }

    #[test]
    fn test_BC_7_03_080_emits_agent_start_with_subagent_and_story_id() {
        // AC-003 + AC-004: valid Agent dispatch, subagent=pr-manager, S-6.07 in prompt
        let payload = make_payload(
            "Agent",
            serde_json::json!({
                "subagent_type": "pr-manager",
                "prompt": "Working on S-6.07"
            }),
        );
        let (result, calls) = run_logic(payload);
        assert!(matches!(result, HookResult::Continue));
        assert_eq!(calls.len(), 1);
        let (event_type, fields) = &calls[0];
        assert_eq!(event_type, "agent.start");
        let field_map: std::collections::HashMap<&str, &str> = fields
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        assert_eq!(field_map.get("hook"), Some(&"track-agent-start"));
        assert_eq!(field_map.get("matcher"), Some(&"Agent"));
        assert_eq!(field_map.get("subagent"), Some(&"pr-manager"));
        assert_eq!(field_map.get("story_id"), Some(&"S-6.07"));
        // E-8 D-2 strict parity: FORBIDDEN fields must be absent
        assert!(
            !field_map.contains_key("agent_id"),
            "agent_id must not appear"
        );
        assert!(
            !field_map.contains_key("tool_name"),
            "tool_name must not appear"
        );
    }

    #[test]
    fn test_BC_7_03_080_emits_agent_start_story_id_story_pattern() {
        // AC-004: STORY-042 fallback (no S-N.NN in prompt)
        let payload = make_payload(
            "Agent",
            serde_json::json!({
                "subagent_type": "implementer",
                "prompt": "See STORY-042 for context"
            }),
        );
        let (result, calls) = run_logic(payload);
        assert!(matches!(result, HookResult::Continue));
        assert_eq!(calls.len(), 1);
        let (_, fields) = &calls[0];
        let field_map: std::collections::HashMap<&str, &str> = fields
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        assert_eq!(field_map.get("story_id"), Some(&"STORY-042"));
        assert_eq!(field_map.get("subagent"), Some(&"implementer"));
    }

    #[test]
    fn test_BC_7_03_080_emits_agent_start_no_story_id_when_absent() {
        // AC-004: no story pattern in prompt -> story_id field omitted
        let payload = make_payload(
            "Agent",
            serde_json::json!({
                "subagent_type": "reviewer",
                "prompt": "no story reference"
            }),
        );
        let (result, calls) = run_logic(payload);
        assert!(matches!(result, HookResult::Continue));
        assert_eq!(calls.len(), 1);
        let (_, fields) = &calls[0];
        let field_map: std::collections::HashMap<&str, &str> = fields
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        assert!(
            !field_map.contains_key("story_id"),
            "story_id must be absent"
        );
    }

    #[test]
    fn test_BC_7_03_080_subagent_defaults_to_unknown_when_absent() {
        // EC-001: subagent_type missing -> use "unknown"
        let payload = make_payload(
            "Agent",
            serde_json::json!({
                "prompt": "S-8.08"
            }),
        );
        let (result, calls) = run_logic(payload);
        assert!(matches!(result, HookResult::Continue));
        assert_eq!(calls.len(), 1);
        let (_, fields) = &calls[0];
        let field_map: std::collections::HashMap<&str, &str> = fields
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        assert_eq!(field_map.get("subagent"), Some(&"unknown"));
    }

    #[test]
    fn test_BC_7_03_080_prompt_missing_treated_as_empty() {
        // EC-002: prompt absent -> no story_id extracted, event emitted without story_id
        let payload = make_payload(
            "Agent",
            serde_json::json!({
                "subagent_type": "test-runner"
            }),
        );
        let (result, calls) = run_logic(payload);
        assert!(matches!(result, HookResult::Continue));
        assert_eq!(calls.len(), 1);
        let (_, fields) = &calls[0];
        let field_map: std::collections::HashMap<&str, &str> = fields
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        assert!(!field_map.contains_key("story_id"));
        assert_eq!(field_map.get("subagent"), Some(&"test-runner"));
    }

    #[test]
    fn test_BC_7_03_079_invariant_exit_0_on_any_path() {
        // BC-7.03.079 invariant 2: always Continue (exit 0) — sampled across paths
        let cases = [
            make_payload("Bash", serde_json::json!({})),
            make_payload("", serde_json::json!({})),
            make_payload("Agent", serde_json::json!({})),
            make_payload(
                "Agent",
                serde_json::json!({"subagent_type": "reviewer", "prompt": ""}),
            ),
        ];
        for payload in cases {
            let tool_name = payload.tool_name.clone();
            let (result, _) = run_logic(payload);
            assert!(
                matches!(result, HookResult::Continue),
                "expected Continue for tool_name={tool_name:?}"
            );
        }
    }

    #[test]
    fn test_BC_7_03_079_no_forbidden_fields_in_emission() {
        // AC-002a: agent_id and tool_name must never appear in emitted fields
        let payload = make_payload(
            "Agent",
            serde_json::json!({
                "subagent_type": "pr-manager",
                "prompt": "S-8.08 work"
            }),
        );
        let (_, calls) = run_logic(payload);
        assert_eq!(calls.len(), 1);
        let (_, fields) = &calls[0];
        for (k, _) in fields {
            assert_ne!(k, "agent_id", "forbidden field agent_id found in emission");
            assert_ne!(
                k, "tool_name",
                "forbidden field tool_name found in emission"
            );
        }
    }

    #[test]
    fn test_BC_7_03_079_emit_fields_exact_bash_parity() {
        // AC-002a: emitted field set matches bash source exactly:
        // type=agent.start hook=track-agent-start matcher=Agent subagent=... [story_id=...]
        // Verifies: hook, matcher, subagent present; story_id present when pattern matches.
        let payload = make_payload(
            "Agent",
            serde_json::json!({
                "subagent_type": "implementer",
                "prompt": "implementing S-8.08"
            }),
        );
        let (_, calls) = run_logic(payload);
        assert_eq!(calls.len(), 1);
        let (event_type, fields) = &calls[0];
        assert_eq!(event_type, "agent.start");
        // Exact field count: hook + matcher + subagent + story_id = 4
        assert_eq!(
            fields.len(),
            4,
            "expected exactly 4 fields (hook, matcher, subagent, story_id)"
        );
        let keys: Vec<&str> = fields.iter().map(|(k, _)| k.as_str()).collect();
        assert!(keys.contains(&"hook"));
        assert!(keys.contains(&"matcher"));
        assert!(keys.contains(&"subagent"));
        assert!(keys.contains(&"story_id"));
    }

    #[test]
    fn test_BC_7_03_079_emit_fields_exact_bash_parity_no_story_id() {
        // AC-002a: when no story_id extracted, field count is exactly 3.
        let payload = make_payload(
            "Agent",
            serde_json::json!({
                "subagent_type": "reviewer",
                "prompt": "no story pattern here"
            }),
        );
        let (_, calls) = run_logic(payload);
        assert_eq!(calls.len(), 1);
        let (_, fields) = &calls[0];
        // Exact field count: hook + matcher + subagent = 3
        assert_eq!(
            fields.len(),
            3,
            "expected exactly 3 fields (hook, matcher, subagent)"
        );
    }
}
