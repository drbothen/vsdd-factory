//! track-agent-stop — SubagentStop WASM hook plugin.
//!
//! Emits `agent.stop` telemetry on every SubagentStop event per BC-7.03.082.
//!
//! Plugin-set fields (strict bash parity — track-agent-stop.sh):
//!   - `hook`        — literal "track-agent-stop"
//!   - `matcher`     — literal "SubagentStop"
//!   - `subagent`    — agent identity resolved via BC-2.02.012 Postcondition 5
//!     fallback chain: `agent_type` → `subagent_name` → "unknown"
//!   - `exit_class`  — one of: "empty" | "blocked" | "ok"
//!   - `result_len`  — non-whitespace Unicode codepoint count of resolved assistant
//!     message (W-15 gate fix HIGH-W15-002: aligned with handoff-validator)
//!
//! Agent identity fallback chain (BC-2.02.012 Postcondition 5):
//!   `payload.agent_type.as_deref().or(payload.subagent_name.as_deref()).unwrap_or("unknown")`
//!
//! Assistant-message fallback chain (BC-2.02.012 Postcondition 6):
//!   `payload.last_assistant_message.as_deref().or(payload.result.as_deref()).unwrap_or("")`
//!
//! EXIT_CLASS classification (BC-7.03.082 postcondition 1):
//!   - `empty`  : trimmed non-whitespace Unicode codepoint count == 0
//!   - `blocked`: result matches BLOCKED regex at line start (multiline `(?m)`)
//!   - `ok`     : all other cases
//!
//! BLOCKED regex (shared with pr-manager-completion-guard per bash comment):
//!   `(?m)^(Status:\s*|##?\s*)?\s*BLOCKED`
//!
//! RESULT_LEN Unicode semantics: `.chars().filter(|c| !c.is_whitespace()).count()`
//! W-15 gate fix HIGH-W15-002: aligned with handoff-validator (chars, not bytes).
//!
//! Best-effort semantics (on_error = "continue"):
//!   - JSON parse failure → handled by __internal::run; hook sees HookPayload (Continue)
//!   - host::emit_event failure → silently swallowed via `let _ = ...`
//!   - Always exits 0 (HookResult::Continue on every path)
//!
//! RESERVED_FIELDS the plugin MUST NOT set (8 total):
//!   Host-enriched: `dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`
//!   Construction-time: `ts`, `ts_epoch`, `schema_version`, `type`

use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// BLOCKED regex pattern (shared with pr-manager-completion-guard)
// ---------------------------------------------------------------------------

/// BLOCKED regex: matches at start of any line (multiline `(?m)` flag so `^`
/// matches start-of-line, mirroring bash `grep` per-line semantics per AC-003).
///
/// Pattern is shared with pr-manager-completion-guard for consistency
/// (per bash comment: "Matches the BLOCKED detection used by
/// pr-manager-completion-guard for consistency.").
const BLOCKED_PATTERN: &str = r"(?m)^(Status:\s*|##?\s*)?\s*BLOCKED";

// ---------------------------------------------------------------------------
// Exit class classification (pure, testable without WASM runtime)
// ---------------------------------------------------------------------------

/// Classify the exit condition of the assistant message.
///
/// - `empty`  : `result_len` == 0 (no non-whitespace Unicode codepoints)
/// - `blocked`: result matches BLOCKED pattern at start of any line
/// - `ok`     : all other cases
///
/// `result_len` is the non-whitespace Unicode codepoint count computed via
/// `.chars().filter(|c| !c.is_whitespace()).count()`.
/// Whitespace counted as Unicode codepoints (is_whitespace()); aligned across
/// handoff-validator + track-agent-stop per W-15 gate fix HIGH-W15-002.
pub fn classify_exit(result: &str) -> (&'static str, usize) {
    // RESULT_LEN: non-whitespace Unicode codepoint count (W-15 gate fix HIGH-W15-002).
    // Aligned with handoff-validator which uses chars().filter(|c| !c.is_whitespace()).
    // Note: is_whitespace() covers ASCII whitespace + Unicode whitespace (e.g., U+00A0
    // non-breaking space), unlike the previous is_ascii_whitespace() implementation.
    let result_len = result.chars().filter(|c| !c.is_whitespace()).count();

    if result_len == 0 {
        return ("empty", 0);
    }

    let re = regex::Regex::new(BLOCKED_PATTERN).expect("static BLOCKED regex is valid");
    if re.is_match(result) {
        ("blocked", result_len)
    } else {
        ("ok", result_len)
    }
}

// ---------------------------------------------------------------------------
// Core hook logic (injectable emit callback — testable without WASM runtime)
// ---------------------------------------------------------------------------

/// Top-level track-agent-stop hook logic with injectable emit callback.
///
/// Resolves agent identity and assistant-message content via the canonical
/// BC-2.02.012 typed-projection fallback chains (Postconditions 5 and 6),
/// classifies exit condition, and emits `agent.stop` via `emit_fn`.
///
/// All host function dependencies are injected so unit/integration tests can
/// drive every branch without a WASM runtime.
///
/// Best-effort: returns `HookResult::Continue` (exit 0) on every code path.
///
/// # Arguments
/// - `payload`: deserialized HookPayload from the SubagentStop envelope.
/// - `emit_fn`: called with (event_type, fields) to emit `agent.stop`.
pub fn track_agent_stop_logic<Emit>(payload: HookPayload, emit_fn: Emit) -> HookResult
where
    Emit: FnOnce(&str, &[(&str, &str)]),
{
    // BC-2.02.012 Postcondition 5: canonical agent identity fallback chain.
    let agent: &str = payload
        .agent_type
        .as_deref()
        .or(payload.subagent_name.as_deref())
        .unwrap_or("unknown");

    // BC-2.02.012 Postcondition 6: canonical assistant-message fallback chain.
    // Note: bash track-agent-stop.sh:23 uses `// ""` literal where other hooks
    // use `// empty` filter; both yield empty-string under `jq -r`, matching
    // Rust's `unwrap_or("")` exactly.
    let result: &str = payload
        .last_assistant_message
        .as_deref()
        .or(payload.result.as_deref())
        .unwrap_or("");

    // Classify exit condition (BC-7.03.082 postcondition 1).
    let (exit_class, result_len) = classify_exit(result);
    let result_len_str = result_len.to_string();

    // Emit agent.stop. Silently swallow emit errors (best-effort, on_error=continue).
    // "type" is a construction-time RESERVED_FIELD set by the dispatcher —
    // we pass it as the event_type argument, not as a field tuple.
    emit_fn(
        "agent.stop",
        &[
            ("hook", "track-agent-stop"),
            ("matcher", "SubagentStop"),
            ("subagent", agent),
            ("exit_class", exit_class),
            ("result_len", &result_len_str),
        ],
    );

    HookResult::Continue
}

// ---------------------------------------------------------------------------
// Top-level entry point called from main.rs (no callbacks — uses host fns).
// ---------------------------------------------------------------------------

/// Called from the WASI entry point in `main.rs`.
///
/// Wires the real vsdd_hook_sdk host functions to the injectable-callback
/// surface of `track_agent_stop_logic`.
pub fn on_agent_stop(payload: HookPayload) -> HookResult {
    track_agent_stop_logic(payload, |event_type, fields| {
        // Silently swallow emit_event errors (best-effort, on_error=continue).
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
    // classify_exit tests (BC-7.03.082 postcondition 1)
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_7_03_082_classify_exit_empty_string() {
        // EC-001 / AC-004(a): empty result → empty, result_len=0
        let (class, len) = classify_exit("");
        assert_eq!(class, "empty");
        assert_eq!(len, 0);
    }

    #[test]
    fn test_BC_7_03_082_classify_exit_whitespace_only() {
        // EC-001: whitespace-only → empty (RESULT_LEN = 0 after chars-filter)
        let (class, len) = classify_exit("   \t\n  ");
        assert_eq!(class, "empty");
        assert_eq!(len, 0);
    }

    #[test]
    fn test_BC_7_03_082_classify_exit_blocked_status_prefix() {
        // AC-004(b): "Status: BLOCKED" → blocked
        let (class, _) = classify_exit("Status: BLOCKED");
        assert_eq!(class, "blocked");
    }

    #[test]
    fn test_BC_7_03_082_classify_exit_blocked_bare() {
        // AC-004(b): bare "BLOCKED" at line start → blocked
        let (class, _) = classify_exit("BLOCKED");
        assert_eq!(class, "blocked");
    }

    #[test]
    fn test_BC_7_03_082_classify_exit_blocked_h2_prefix() {
        // AC-004(b): "## BLOCKED" at line start → blocked
        let (class, _) = classify_exit("## BLOCKED");
        assert_eq!(class, "blocked");
    }

    #[test]
    fn test_BC_7_03_082_classify_exit_ok() {
        // AC-004(c): non-empty non-blocked result → ok with correct result_len
        let (class, len) = classify_exit("DONE");
        assert_eq!(class, "ok");
        assert_eq!(len, 4);
    }

    #[test]
    fn test_BC_7_03_082_classify_exit_blocked_mid_text_is_ok() {
        // EC-002: BLOCKED not at line start → ok (multiline ^ anchor required)
        let (class, _) = classify_exit("result is BLOCKED by policy");
        assert_eq!(class, "ok");
    }

    #[test]
    fn test_BC_7_03_082_classify_exit_blocked_multiline_second_line() {
        // EC-007: BLOCKED on a non-first line with (?m) → blocked
        let (class, _) = classify_exit("first line of output\nBLOCKED\nsome more");
        assert_eq!(class, "blocked");
    }

    #[test]
    fn test_BC_7_03_082_classify_exit_result_len_char_count() {
        // W-15 gate fix HIGH-W15-002: result_len is now Unicode codepoint count,
        // not byte count. U+1F600 emoji = 1 codepoint (not 4 bytes).
        let (class, len) = classify_exit("\u{1F600}");
        assert_eq!(class, "ok");
        assert_eq!(
            len, 1,
            "emoji must count as 1 Unicode codepoint (W-15 HIGH-W15-002)"
        );
    }

    #[test]
    fn test_BC_7_03_082_classify_exit_result_len_excludes_ascii_whitespace() {
        // AC-003: "hello world" = 10 non-whitespace chars
        let (class, len) = classify_exit("hello world");
        assert_eq!(class, "ok");
        assert_eq!(len, 10);
    }

    /// W-15 gate fix HIGH-W15-002: Unicode whitespace (U+00A0 non-breaking space)
    /// is counted as whitespace by is_whitespace() (not by is_ascii_whitespace()).
    /// A string with only U+00A0 → empty (result_len=0).
    /// A string with U+00A0 + ASCII space → both are whitespace → empty.
    #[test]
    fn test_HIGH_W15_002_unicode_whitespace_counted_as_whitespace() {
        // U+00A0 non-breaking space + ASCII space: both are whitespace
        let input = "\u{00A0} ";
        let (class, len) = classify_exit(input);
        assert_eq!(
            class, "empty",
            "non-breaking space + ASCII space must be empty (both whitespace)"
        );
        assert_eq!(
            len, 0,
            "both chars are whitespace, non-whitespace count must be 0"
        );

        // "a" + U+00A0 + "b": 2 non-whitespace codepoints
        let input2 = "a\u{00A0}b";
        let (class2, len2) = classify_exit(input2);
        assert_eq!(class2, "ok");
        assert_eq!(
            len2, 2,
            "only 'a' and 'b' are non-whitespace; U+00A0 is whitespace"
        );
    }

    // -----------------------------------------------------------------------
    // track_agent_stop_logic tests (BC-7.03.081, BC-7.03.082, BC-2.02.012)
    // -----------------------------------------------------------------------

    /// Build a minimal HookPayload for testing.
    fn make_payload(json: serde_json::Value) -> HookPayload {
        serde_json::from_value(json).expect("fixture must deserialize")
    }

    /// Helper: run the logic with a captured-emit callback.
    /// Returns (result, emitted_calls).
    #[allow(clippy::type_complexity)]
    fn run_logic(payload: HookPayload) -> (HookResult, Vec<(String, Vec<(String, String)>)>) {
        let mut calls: Vec<(String, Vec<(String, String)>)> = Vec::new();
        let result = track_agent_stop_logic(payload, |event_type, fields| {
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

    /// Make a SubagentStop payload fixture with optional fields.
    fn make_subagentstop(
        agent_type: Option<&str>,
        subagent_name: Option<&str>,
        last_assistant_message: Option<&str>,
        result: Option<&str>,
    ) -> HookPayload {
        let mut map = serde_json::json!({
            "event_name": "SubagentStop",
            "session_id": "test-session",
            "dispatcher_trace_id": "test-trace"
        });
        if let Some(v) = agent_type {
            map["agent_type"] = serde_json::json!(v);
        }
        if let Some(v) = subagent_name {
            map["subagent_name"] = serde_json::json!(v);
        }
        if let Some(v) = last_assistant_message {
            map["last_assistant_message"] = serde_json::json!(v);
        }
        if let Some(v) = result {
            map["result"] = serde_json::json!(v);
        }
        make_payload(map)
    }

    // -- BC-7.03.082 postcondition 1: emits agent.stop with correct fields ---

    #[test]
    fn test_BC_7_03_082_emits_agent_stop_happy_path() {
        // AC-004(c): agent_type present, non-empty non-blocked last_assistant_message
        let payload = make_subagentstop(
            Some("pr-reviewer"),
            None,
            Some("DONE — wrote review."),
            None,
        );
        let (result, calls) = run_logic(payload);
        assert!(matches!(result, HookResult::Continue));
        assert_eq!(calls.len(), 1);
        let (event_type, fields) = &calls[0];
        assert_eq!(event_type, "agent.stop");
        let field_map: std::collections::HashMap<&str, &str> = fields
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        assert_eq!(field_map.get("hook"), Some(&"track-agent-stop"));
        assert_eq!(field_map.get("matcher"), Some(&"SubagentStop"));
        assert_eq!(field_map.get("subagent"), Some(&"pr-reviewer"));
        assert_eq!(field_map.get("exit_class"), Some(&"ok"));
    }

    #[test]
    fn test_BC_7_03_082_emits_agent_stop_exit_class_empty() {
        // AC-004(a): empty last_assistant_message → EXIT_CLASS=empty, result_len=0
        let payload = make_subagentstop(Some("implementer"), None, Some(""), None);
        let (result, calls) = run_logic(payload);
        assert!(matches!(result, HookResult::Continue));
        assert_eq!(calls.len(), 1);
        let (_, fields) = &calls[0];
        let field_map: std::collections::HashMap<&str, &str> = fields
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        assert_eq!(field_map.get("exit_class"), Some(&"empty"));
        assert_eq!(field_map.get("result_len"), Some(&"0"));
    }

    #[test]
    fn test_BC_7_03_082_emits_agent_stop_exit_class_blocked() {
        // AC-004(b): result with "Status: BLOCKED" → EXIT_CLASS=blocked
        let payload = make_subagentstop(
            Some("story-writer"),
            None,
            Some("Status: BLOCKED — missing context"),
            None,
        );
        let (result, calls) = run_logic(payload);
        assert!(matches!(result, HookResult::Continue));
        assert_eq!(calls.len(), 1);
        let (_, fields) = &calls[0];
        let field_map: std::collections::HashMap<&str, &str> = fields
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        assert_eq!(field_map.get("exit_class"), Some(&"blocked"));
    }

    // -- BC-2.02.012 Postcondition 5: agent identity fallback chain ----------

    #[test]
    fn test_BC_2_02_012_agent_type_used_when_present() {
        // PC-5: agent_type present → used as subagent
        let payload = make_subagentstop(
            Some("product-owner"),
            Some("fallback-name"),
            Some("done"),
            None,
        );
        let (_, calls) = run_logic(payload);
        let (_, fields) = &calls[0];
        let field_map: std::collections::HashMap<&str, &str> = fields
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        assert_eq!(field_map.get("subagent"), Some(&"product-owner"));
    }

    #[test]
    fn test_BC_2_02_012_subagent_name_fallback_when_agent_type_absent() {
        // AC-004(f) / EC-004: agent_type absent, subagent_name present → use subagent_name
        let payload = make_subagentstop(None, Some("story-writer"), Some("DONE"), None);
        let (_, calls) = run_logic(payload);
        let (_, fields) = &calls[0];
        let field_map: std::collections::HashMap<&str, &str> = fields
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        assert_eq!(field_map.get("subagent"), Some(&"story-writer"));
    }

    #[test]
    fn test_BC_2_02_012_subagent_unknown_when_both_absent() {
        // EC-004b: both agent_type and subagent_name absent → "unknown"
        let payload = make_subagentstop(None, None, Some("DONE"), None);
        let (_, calls) = run_logic(payload);
        let (_, fields) = &calls[0];
        let field_map: std::collections::HashMap<&str, &str> = fields
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        assert_eq!(field_map.get("subagent"), Some(&"unknown"));
    }

    // -- BC-2.02.012 Postcondition 6: assistant-message fallback chain -------

    #[test]
    fn test_BC_2_02_012_last_assistant_message_used_when_present() {
        // PC-6: last_assistant_message present → used for result classification
        let payload = make_subagentstop(
            Some("reviewer"),
            None,
            Some("DONE primary"),
            Some("result field"),
        );
        let (_, calls) = run_logic(payload);
        let (_, fields) = &calls[0];
        let field_map: std::collections::HashMap<&str, &str> = fields
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        // DONE primary = 11 non-whitespace bytes
        assert_eq!(field_map.get("result_len"), Some(&"11"));
    }

    #[test]
    fn test_BC_2_02_012_result_fallback_when_last_assistant_message_absent() {
        // AC-004(d) / EC-003a: last_assistant_message absent, result present → use result
        let payload = make_subagentstop(Some("implementer"), None, None, Some("DONE via result"));
        let (_, calls) = run_logic(payload);
        let (_, fields) = &calls[0];
        let field_map: std::collections::HashMap<&str, &str> = fields
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        assert_eq!(field_map.get("exit_class"), Some(&"ok"));
        // "DONEviaresult" = 13 non-whitespace bytes
        assert_eq!(field_map.get("result_len"), Some(&"13"));
    }

    #[test]
    fn test_BC_2_02_012_empty_when_both_message_fields_absent() {
        // AC-004(e) / EC-003b: both absent → "", RESULT_LEN=0, EXIT_CLASS=empty
        let payload = make_subagentstop(Some("reviewer"), None, None, None);
        let (_, calls) = run_logic(payload);
        let (_, fields) = &calls[0];
        let field_map: std::collections::HashMap<&str, &str> = fields
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        assert_eq!(field_map.get("exit_class"), Some(&"empty"));
        assert_eq!(field_map.get("result_len"), Some(&"0"));
    }

    // -- BC-7.03.081 invariant 1: always exits 0 ----------------------------

    #[test]
    fn test_BC_7_03_081_invariant_exit_0_on_every_path() {
        // BC-7.03.081 invariant 1: always Continue (exit 0) — sampled across paths
        let cases: Vec<HookPayload> = vec![
            make_subagentstop(None, None, None, None),
            make_subagentstop(Some("reviewer"), None, Some(""), None),
            make_subagentstop(Some("reviewer"), None, Some("BLOCKED"), None),
            make_subagentstop(Some("reviewer"), None, Some("DONE"), None),
            make_subagentstop(None, Some("fallback"), Some("ok result"), None),
        ];
        for payload in cases {
            let (result, _) = run_logic(payload);
            assert!(
                matches!(result, HookResult::Continue),
                "expected Continue (exit 0) on all paths"
            );
        }
    }

    // -- Field set and count validation ------------------------------------

    #[test]
    fn test_BC_7_03_082_emits_exactly_five_fields() {
        // AC-003: emitted fields are hook, matcher, subagent, exit_class, result_len (5 total)
        let payload = make_subagentstop(Some("pr-manager"), None, Some("DONE"), None);
        let (_, calls) = run_logic(payload);
        assert_eq!(calls.len(), 1);
        let (_, fields) = &calls[0];
        assert_eq!(fields.len(), 5, "expected exactly 5 fields");
        let keys: Vec<&str> = fields.iter().map(|(k, _)| k.as_str()).collect();
        assert!(keys.contains(&"hook"));
        assert!(keys.contains(&"matcher"));
        assert!(keys.contains(&"subagent"));
        assert!(keys.contains(&"exit_class"));
        assert!(keys.contains(&"result_len"));
    }

    #[test]
    fn test_BC_7_03_082_hook_and_matcher_literals() {
        // AC-003: hook="track-agent-stop", matcher="SubagentStop" (literals, always)
        let payload = make_subagentstop(Some("x"), None, Some("y"), None);
        let (_, calls) = run_logic(payload);
        let (_, fields) = &calls[0];
        let field_map: std::collections::HashMap<&str, &str> = fields
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        assert_eq!(field_map.get("hook"), Some(&"track-agent-stop"));
        assert_eq!(field_map.get("matcher"), Some(&"SubagentStop"));
    }

    // -- Emit exactly once per invocation ----------------------------------

    #[test]
    fn test_BC_7_03_082_emits_exactly_one_event_per_invocation() {
        // AC-003: exactly one agent.stop event per hook invocation
        let payload = make_subagentstop(Some("reviewer"), None, Some("DONE"), None);
        let (_, calls) = run_logic(payload);
        assert_eq!(
            calls.len(),
            1,
            "exactly one event must be emitted per invocation"
        );
        assert_eq!(calls[0].0, "agent.stop");
    }
}
