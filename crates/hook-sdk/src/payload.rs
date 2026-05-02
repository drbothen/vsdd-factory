//! Hook payload — the typed projection of Claude Code's stdin envelope
//! that the dispatcher hands to a plugin's `#[hook]` function.

use serde::{Deserialize, Serialize};

/// The data delivered to a hook on each invocation.
///
/// Mirrors the JSON envelope the dispatcher writes to a plugin's stdin.
/// `tool_input` and `tool_response` are kept as `serde_json::Value`
/// because their schemas vary per tool — plugins that care narrow the
/// shape themselves.
///
/// `tool_response` is `None` for `PreToolUse` (the call has not happened
/// yet) and present for `PostToolUse` and most lifecycle events.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HookPayload {
    /// Claude Code event name — e.g. `"PreToolUse"`, `"PostToolUse"`,
    /// `"SubagentStop"`, `"SessionStart"`.
    pub event_name: String,

    /// Name of the tool being invoked, when applicable. Empty for
    /// session/lifecycle events that aren't tied to a tool.
    #[serde(default)]
    pub tool_name: String,

    /// Claude Code session identifier — same value across every hook
    /// invocation in a single session.
    pub session_id: String,

    /// Per-invocation correlation ID assigned by the dispatcher.
    /// Propagated to every `emit_event` call and to the
    /// dispatcher-internal log so causal chains stay intact.
    pub dispatcher_trace_id: String,

    /// Tool-specific input (the arguments Claude Code passed to the
    /// tool). Schema varies per tool.
    #[serde(default)]
    pub tool_input: serde_json::Value,

    /// Tool-specific response (the value the tool returned). `None` for
    /// `PreToolUse` and other pre-call events. Schema varies per tool.
    #[serde(default)]
    pub tool_response: Option<serde_json::Value>,

    /// Per-plugin configuration sourced from the registry entry's
    /// `[hooks.config]` table. Schema is plugin-defined; the dispatcher
    /// only forwards. Defaults to `Value::Null` when no config block is
    /// present in the registry. Used by the legacy-bash-adapter (S-2.1)
    /// to receive its `script_path`, and available to any future plugin
    /// that needs declarative configuration alongside its registration.
    #[serde(default)]
    pub plugin_config: serde_json::Value,

    // ── SubagentStop top-level fields (BC-2.02.012) ──────────────────────
    // Present only on `event_name == "SubagentStop"` envelopes.  All four
    // use `#[serde(default)]` so non-SubagentStop envelopes deserialize
    // successfully with every field as `None` (BC-2.02.012 Invariant 2).
    // JSON `null` also deserializes to `None` — providing jq-`//`-equivalent
    // null-as-advance semantics (BC-2.02.012 Invariant 3).
    // HOST_ABI_VERSION remains 1; this is an additive extension per D-6
    // Option A and D-183 (BC-2.02.012 Invariant 1).

    /// Agent type identifier carried by a SubagentStop envelope
    /// (e.g. `"product-owner"`, `"pr-reviewer"`).
    ///
    /// Primary arm of the canonical agent identity fallback chain
    /// (BC-2.02.012 Postcondition 1 and 5). Resolve agent identity with:
    ///
    /// ```
    /// # use vsdd_hook_sdk::HookPayload;
    /// # let json = r#"{"event_name":"SubagentStop","session_id":"s","dispatcher_trace_id":"t","agent_type":"pr-reviewer","subagent_name":"fallback"}"#;
    /// # let payload: HookPayload = serde_json::from_str(json).unwrap();
    /// let identity = payload.agent_type.as_deref()
    ///     .or(payload.subagent_name.as_deref())
    ///     .unwrap_or("unknown");
    /// assert_eq!(identity, "pr-reviewer");
    /// ```
    ///
    /// `None` when absent or JSON null. Not populated for non-SubagentStop
    /// events (BC-2.02.012 Postcondition 7).
    #[serde(default)]
    pub agent_type: Option<String>,

    /// Subagent name carried by a SubagentStop envelope
    /// (e.g. `"pr-reviewer-fallback"`).
    ///
    /// Fallback arm of the canonical agent identity chain when
    /// `agent_type` is `None` (BC-2.02.012 Postcondition 2 and 5).
    /// `None` when absent or JSON null.
    #[serde(default)]
    pub subagent_name: Option<String>,

    /// Last assistant message text carried by a SubagentStop envelope.
    ///
    /// Primary arm of the canonical assistant-message fallback chain
    /// (BC-2.02.012 Postcondition 3 and 6). Resolve assistant message with:
    ///
    /// ```
    /// # use vsdd_hook_sdk::HookPayload;
    /// # let json = r#"{"event_name":"SubagentStop","session_id":"s","dispatcher_trace_id":"t","last_assistant_message":null,"result":"fallback-result"}"#;
    /// # let payload: HookPayload = serde_json::from_str(json).unwrap();
    /// let msg = payload.last_assistant_message.as_deref()
    ///     .or(payload.result.as_deref())
    ///     .unwrap_or("");
    /// assert_eq!(msg, "fallback-result");
    /// ```
    ///
    /// `None` when absent or JSON null.
    #[serde(default)]
    pub last_assistant_message: Option<String>,

    /// Result field carried by a SubagentStop envelope.
    ///
    /// Fallback arm of the canonical assistant-message chain when
    /// `last_assistant_message` is `None` (BC-2.02.012 Postcondition 4 and 6).
    /// `None` when absent or JSON null.
    #[serde(default)]
    pub result: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(json: &str) -> HookPayload {
        serde_json::from_str(json).expect("fixture should parse")
    }

    // ── Pre-existing tests (must continue to pass) ───────────────────────────

    #[test]
    fn pretooluse_payload_deserializes() {
        let p = fixture(
            r#"{
                "event_name": "PreToolUse",
                "tool_name": "Bash",
                "session_id": "sess-1",
                "dispatcher_trace_id": "trace-1",
                "tool_input": {"command": "git status"}
            }"#,
        );
        assert_eq!(p.event_name, "PreToolUse");
        assert_eq!(p.tool_name, "Bash");
        assert_eq!(p.session_id, "sess-1");
        assert_eq!(p.dispatcher_trace_id, "trace-1");
        assert!(p.tool_response.is_none());
        assert_eq!(
            p.tool_input.get("command").and_then(|v| v.as_str()),
            Some("git status")
        );
    }

    #[test]
    fn posttooluse_payload_with_response() {
        let p = fixture(
            r#"{
                "event_name": "PostToolUse",
                "tool_name": "Bash",
                "session_id": "sess-1",
                "dispatcher_trace_id": "trace-2",
                "tool_input": {"command": "true"},
                "tool_response": {"exit_code": 0}
            }"#,
        );
        assert_eq!(p.event_name, "PostToolUse");
        let resp = p.tool_response.as_ref().expect("response present");
        assert_eq!(resp.get("exit_code").and_then(|v| v.as_i64()), Some(0));
    }

    #[test]
    fn lifecycle_payload_without_tool_name() {
        let p = fixture(
            r#"{
                "event_name": "SessionStart",
                "session_id": "sess-x",
                "dispatcher_trace_id": "trace-x"
            }"#,
        );
        assert_eq!(p.event_name, "SessionStart");
        assert_eq!(p.tool_name, "");
        assert!(p.tool_input.is_null());
        assert!(p.tool_response.is_none());
    }

    #[test]
    fn payload_round_trip_via_serde() {
        let original = fixture(
            r#"{
                "event_name": "PreToolUse",
                "tool_name": "Edit",
                "session_id": "s",
                "dispatcher_trace_id": "t",
                "tool_input": {"file_path": "src/lib.rs"}
            }"#,
        );
        let json = serde_json::to_string(&original).expect("serialize");
        let round: HookPayload = serde_json::from_str(&json).expect("round-trip");
        assert_eq!(round.event_name, original.event_name);
        assert_eq!(round.session_id, original.session_id);
    }

    #[test]
    fn plugin_config_defaults_to_null_when_missing() {
        let p = fixture(
            r#"{
                "event_name": "PreToolUse",
                "session_id": "s",
                "dispatcher_trace_id": "t"
            }"#,
        );
        assert!(p.plugin_config.is_null());
    }

    #[test]
    fn plugin_config_passes_through_when_present() {
        let p = fixture(
            r#"{
                "event_name": "PreToolUse",
                "session_id": "s",
                "dispatcher_trace_id": "t",
                "plugin_config": {"script_path": "hooks/foo.sh"}
            }"#,
        );
        assert_eq!(
            p.plugin_config.get("script_path").and_then(|v| v.as_str()),
            Some("hooks/foo.sh"),
        );
    }

    // ── BC-2.02.012 SubagentStop field tests (Red Gate) ──────────────────────
    //
    // These tests cover BC-2.02.012 Postconditions 1-7 and Invariants 2-4.
    // They must all FAIL before the implementation is complete.
    // Maps to S-8.30 AC-2, AC-3, AC-4, AC-5, AC-8 and EC-001..EC-008.

    // ── AC-3 / Postconditions 1-4: all four fields populated from envelope ───

    /// BC-2.02.012 Postcondition 1: agent_type is Some when envelope carries
    /// a non-null string.
    /// BC-2.02.012 Postcondition 2: subagent_name is Some when envelope
    /// carries a non-null string.
    /// BC-2.02.012 Postcondition 3: last_assistant_message is Some when
    /// envelope carries a non-null string.
    /// BC-2.02.012 Postcondition 4: result is Some when envelope carries a
    /// non-null string.
    ///
    /// Canonical test vector (BC-2.02.012): happy-path, all four fields
    /// present.
    #[test]
    fn test_BC_2_02_012_subagentstop_all_four_fields_populated() {
        let p = fixture(
            r#"{
                "event_name": "SubagentStop",
                "session_id": "s",
                "dispatcher_trace_id": "t",
                "agent_type": "pr-reviewer",
                "subagent_name": "pr-reviewer-fallback",
                "last_assistant_message": "wrote pr-review.md and posted gh pr review --approve",
                "result": "fallback-result"
            }"#,
        );
        // Postcondition 1
        assert_eq!(p.agent_type, Some("pr-reviewer".to_string()));
        // Postcondition 2
        assert_eq!(p.subagent_name, Some("pr-reviewer-fallback".to_string()));
        // Postcondition 3
        assert_eq!(
            p.last_assistant_message,
            Some("wrote pr-review.md and posted gh pr review --approve".to_string())
        );
        // Postcondition 4
        assert_eq!(p.result, Some("fallback-result".to_string()));
    }

    /// BC-2.02.012 Canonical test vector: happy-path fallback — agent_type
    /// absent, subagent_name present; last_assistant_message absent, result
    /// present.
    #[test]
    fn test_BC_2_02_012_subagentstop_fallback_fields_populated() {
        let p = fixture(
            r#"{
                "event_name": "SubagentStop",
                "session_id": "s",
                "dispatcher_trace_id": "t",
                "subagent_name": "story-writer",
                "result": "Complete."
            }"#,
        );
        // Postcondition 1: agent_type absent → None
        assert_eq!(p.agent_type, None);
        // Postcondition 2: subagent_name present
        assert_eq!(p.subagent_name, Some("story-writer".to_string()));
        // Postcondition 3: last_assistant_message absent → None
        assert_eq!(p.last_assistant_message, None);
        // Postcondition 4: result present
        assert_eq!(p.result, Some("Complete.".to_string()));
    }

    // ── AC-2 / Postcondition 7 / Invariant 2: backward-compat, non-SubagentStop
    // envelope has all four fields = None ─────────────────────────────────────

    /// BC-2.02.012 Postcondition 7 / Invariant 2: PreToolUse envelope
    /// (no SubagentStop fields) deserializes with all four new fields = None.
    /// Verifies #[serde(default)] backward-compat (AC-2).
    ///
    /// Canonical test vector (BC-2.02.012): edge-case (non-SubagentStop).
    #[test]
    fn test_BC_2_02_012_pretooluse_subagentstop_fields_default_to_none() {
        let p = fixture(
            r#"{
                "event_name": "PreToolUse",
                "tool_name": "Bash",
                "session_id": "s",
                "dispatcher_trace_id": "t",
                "tool_input": {}
            }"#,
        );
        assert_eq!(p.agent_type, None, "agent_type must default to None for PreToolUse");
        assert_eq!(p.subagent_name, None, "subagent_name must default to None for PreToolUse");
        assert_eq!(
            p.last_assistant_message, None,
            "last_assistant_message must default to None for PreToolUse"
        );
        assert_eq!(p.result, None, "result must default to None for PreToolUse");
    }

    // ── AC-4 / Postconditions 3, 5, 6 / Invariant 3: JSON null → None,
    // canonical fallback chains ───────────────────────────────────────────────

    /// BC-2.02.012 EC-001 / Invariant 3: JSON null on agent_type and
    /// last_assistant_message → None; fallback chains advance to next
    /// non-null value.
    /// BC-2.02.012 Postcondition 5: canonical agent identity fallback chain.
    /// BC-2.02.012 Postcondition 6: canonical assistant-message fallback chain.
    ///
    /// Canonical test vector (BC-2.02.012): edge-case (JSON null).
    #[test]
    fn test_BC_2_02_012_json_null_fields_deserialize_to_none_and_fallback_chains() {
        let p = fixture(
            r#"{
                "event_name": "SubagentStop",
                "session_id": "s",
                "dispatcher_trace_id": "t",
                "agent_type": null,
                "subagent_name": "pr-reviewer",
                "last_assistant_message": null,
                "result": "actual-result"
            }"#,
        );
        // EC-001 / Invariant 3: JSON null → None
        assert_eq!(p.agent_type, None, "JSON null agent_type must deserialize to None");
        assert_eq!(p.subagent_name, Some("pr-reviewer".to_string()));
        assert_eq!(p.last_assistant_message, None, "JSON null last_assistant_message must be None");
        assert_eq!(p.result, Some("actual-result".to_string()));

        // Postcondition 5: canonical agent identity fallback chain
        let identity = p.agent_type.as_deref().or(p.subagent_name.as_deref()).unwrap_or("unknown");
        assert_eq!(identity, "pr-reviewer", "fallback chain must advance to subagent_name when agent_type is None");

        // Postcondition 6: canonical assistant-message fallback chain
        let msg = p.last_assistant_message.as_deref().or(p.result.as_deref()).unwrap_or("");
        assert_eq!(msg, "actual-result", "fallback chain must advance to result when last_assistant_message is None");
    }

    /// BC-2.02.012 EC-003: all four SubagentStop fields absent.
    /// Identity resolves to "unknown"; message resolves to "".
    ///
    /// Canonical test vector (BC-2.02.012): edge-case (all absent).
    #[test]
    fn test_BC_2_02_012_ec003_all_fields_absent_resolves_to_defaults() {
        let p = fixture(
            r#"{
                "event_name": "SubagentStop",
                "session_id": "s",
                "dispatcher_trace_id": "t"
            }"#,
        );
        assert_eq!(p.agent_type, None);
        assert_eq!(p.subagent_name, None);
        assert_eq!(p.last_assistant_message, None);
        assert_eq!(p.result, None);

        // EC-003: final unwrap_or values
        let identity = p.agent_type.as_deref().or(p.subagent_name.as_deref()).unwrap_or("unknown");
        assert_eq!(identity, "unknown", "all-absent: identity must resolve to \"unknown\"");

        let msg = p.last_assistant_message.as_deref().or(p.result.as_deref()).unwrap_or("");
        assert_eq!(msg, "", "all-absent: message must resolve to empty string");
    }

    // ── AC-5 / Invariant 4: round-trip preservation ──────────────────────────

    /// BC-2.02.012 Invariant 4: field names are canonical and immutable;
    /// serialize + re-deserialize preserves all four SubagentStop field values.
    #[test]
    fn test_BC_2_02_012_invariant4_subagentstop_round_trip_preserves_fields() {
        let original = fixture(
            r#"{
                "event_name": "SubagentStop",
                "session_id": "s",
                "dispatcher_trace_id": "t",
                "agent_type": "product-owner",
                "subagent_name": "po-fallback",
                "last_assistant_message": "Done.",
                "result": "Complete."
            }"#,
        );
        let json = serde_json::to_string(&original).expect("serialize");
        let round: HookPayload = serde_json::from_str(&json).expect("round-trip");

        assert_eq!(round.agent_type, original.agent_type, "agent_type must survive round-trip");
        assert_eq!(round.subagent_name, original.subagent_name, "subagent_name must survive round-trip");
        assert_eq!(
            round.last_assistant_message, original.last_assistant_message,
            "last_assistant_message must survive round-trip"
        );
        assert_eq!(round.result, original.result, "result must survive round-trip");
    }

    // ── AC-8 / Postcondition 7 / Invariant 2: all known event_names
    // deserialize without error; non-SubagentStop have all four fields = None
    // ─────────────────────────────────────────────────────────────────────────

    /// BC-2.02.012 Postcondition 7 / Invariant 2 / AC-8: all known event types
    /// deserialize without error. For non-SubagentStop events the four new
    /// fields must be None.
    ///
    /// Event types enumerated per AC-8: PreToolUse, PostToolUse, SessionStart,
    /// SessionEnd, SubagentStop.  (Stop omitted per AC-8 note — unverified in
    /// bash hooks and BC-2.02.012 evidence.)
    #[test]
    fn test_BC_2_02_012_ac8_all_event_types_deserialize_subagentstop_fields_none_for_non_subagentstop()
    {
        let non_subagentstop_fixtures = [
            (
                "PreToolUse",
                r#"{"event_name":"PreToolUse","tool_name":"Bash","session_id":"s","dispatcher_trace_id":"t","tool_input":{}}"#,
            ),
            (
                "PostToolUse",
                r#"{"event_name":"PostToolUse","tool_name":"Bash","session_id":"s","dispatcher_trace_id":"t","tool_input":{},"tool_response":{}}"#,
            ),
            (
                "SessionStart",
                r#"{"event_name":"SessionStart","session_id":"s","dispatcher_trace_id":"t"}"#,
            ),
            (
                "SessionEnd",
                r#"{"event_name":"SessionEnd","session_id":"s","dispatcher_trace_id":"t"}"#,
            ),
        ];

        for (event, json) in &non_subagentstop_fixtures {
            let p: HookPayload =
                serde_json::from_str(json).unwrap_or_else(|e| panic!("{event} failed to deserialize: {e}"));
            assert_eq!(p.agent_type, None, "{event}: agent_type must be None");
            assert_eq!(p.subagent_name, None, "{event}: subagent_name must be None");
            assert_eq!(p.last_assistant_message, None, "{event}: last_assistant_message must be None");
            assert_eq!(p.result, None, "{event}: result must be None");
        }

        // SubagentStop with all four fields: must deserialize without error
        let p = fixture(
            r#"{"event_name":"SubagentStop","session_id":"s","dispatcher_trace_id":"t","agent_type":"product-owner","last_assistant_message":"Done."}"#,
        );
        assert_eq!(p.event_name, "SubagentStop");
        assert_eq!(p.agent_type, Some("product-owner".to_string()));
        assert_eq!(p.last_assistant_message, Some("Done.".to_string()));
        assert_eq!(p.subagent_name, None);
        assert_eq!(p.result, None);
    }

    // ── EC-007 / type fidelity: wrong-type value returns serde error ──────────

    /// BC-2.02.012 EC-007: if the envelope sends `"agent_type": 123` (integer
    /// instead of string), serde_json must return a deserialization error.
    /// This is an envelope schema violation; no silent coercion should occur.
    #[test]
    fn test_BC_2_02_012_ec007_wrong_type_agent_type_returns_serde_error() {
        let json = r#"{"event_name":"SubagentStop","session_id":"s","dispatcher_trace_id":"t","agent_type":123}"#;
        let result: Result<HookPayload, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "integer agent_type must cause a serde deserialization error, not silent coercion"
        );
    }

    /// BC-2.02.012 EC-007: if the envelope sends `"subagent_name": true`
    /// (boolean instead of string), serde_json must return an error.
    #[test]
    fn test_BC_2_02_012_ec007_wrong_type_subagent_name_returns_serde_error() {
        let json = r#"{"event_name":"SubagentStop","session_id":"s","dispatcher_trace_id":"t","subagent_name":true}"#;
        let result: Result<HookPayload, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "boolean subagent_name must cause a serde deserialization error, not silent coercion"
        );
    }

    /// BC-2.02.012 EC-007: if `last_assistant_message` is an integer, serde
    /// must return an error.
    #[test]
    fn test_BC_2_02_012_ec007_wrong_type_last_assistant_message_returns_serde_error() {
        let json = r#"{"event_name":"SubagentStop","session_id":"s","dispatcher_trace_id":"t","last_assistant_message":42}"#;
        let result: Result<HookPayload, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "integer last_assistant_message must cause a serde deserialization error"
        );
    }

    /// BC-2.02.012 EC-007: if `result` is an array, serde must return an error.
    #[test]
    fn test_BC_2_02_012_ec007_wrong_type_result_returns_serde_error() {
        let json = r#"{"event_name":"SubagentStop","session_id":"s","dispatcher_trace_id":"t","result":[]}"#;
        let result: Result<HookPayload, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "array result must cause a serde deserialization error"
        );
    }

    // ── EC-008: empty-string does NOT advance fallback chain ──────────────────

    /// BC-2.02.012 EC-008: `"agent_type": ""` deserializes to Some(""),
    /// which does NOT advance the fallback chain.  Rust's Option::or
    /// advances only on None, matching jq // which advances on null/false
    /// but NOT on empty-string.
    #[test]
    fn test_BC_2_02_012_ec008_empty_string_does_not_advance_fallback() {
        let p = fixture(
            r#"{
                "event_name": "SubagentStop",
                "session_id": "s",
                "dispatcher_trace_id": "t",
                "agent_type": "",
                "subagent_name": "should-not-be-used"
            }"#,
        );
        assert_eq!(p.agent_type, Some("".to_string()), "empty string must be Some(\"\"), not None");

        // EC-008: Some("") does not advance; identity = ""
        let identity = p.agent_type.as_deref().or(p.subagent_name.as_deref()).unwrap_or("unknown");
        assert_eq!(identity, "", "empty string agent_type must yield \"\" (not \"should-not-be-used\")");
    }

    // ── EC-006: unknown fields are silently ignored ───────────────────────────

    /// BC-2.02.012 EC-006: SubagentStop envelope with additional top-level
    /// fields not in HookPayload must deserialize successfully
    /// (no #[serde(deny_unknown_fields)]).
    #[test]
    fn test_BC_2_02_012_ec006_unknown_fields_silently_ignored() {
        let p = fixture(
            r#"{
                "event_name": "SubagentStop",
                "session_id": "s",
                "dispatcher_trace_id": "t",
                "agent_type": "pr-reviewer",
                "unknown_future_field": "some-value",
                "another_unknown": 99
            }"#,
        );
        // Must not fail — unknown fields ignored
        assert_eq!(p.agent_type, Some("pr-reviewer".to_string()));
    }

    // ── Non-SubagentStop scope: new fields must not leak into PreToolUse ──────

    /// BC-2.02.012 Postcondition 7 / EC-005: a Stop-like (non-SubagentStop)
    /// event JSON round-trips without populating the four new fields.
    /// Verifies the projection is SubagentStop-scoped, not leaking.
    #[test]
    fn test_BC_2_02_012_non_subagentstop_projection_does_not_leak() {
        let p = fixture(
            r#"{
                "event_name": "PreToolUse",
                "session_id": "s",
                "dispatcher_trace_id": "t",
                "tool_input": {"command": "ls"}
            }"#,
        );
        // None of the four SubagentStop fields should be populated
        assert!(
            p.agent_type.is_none() && p.subagent_name.is_none()
                && p.last_assistant_message.is_none() && p.result.is_none(),
            "SubagentStop fields must all be None for non-SubagentStop event"
        );
    }

    // ── HOST_ABI_VERSION invariant (Invariant 1 / AC-7) ──────────────────────

    /// BC-2.02.012 Invariant 1 / AC-7: HOST_ABI_VERSION must remain 1 after
    /// this additive extension (D-6 Option A, D-183).
    #[test]
    fn test_BC_2_02_012_invariant1_host_abi_version_remains_one() {
        assert_eq!(
            crate::HOST_ABI_VERSION,
            1,
            "HOST_ABI_VERSION must remain 1 (BC-2.02.012 Invariant 1, D-6 Option A)"
        );
    }

    // ── RED GATE: HOST_ABI.md SubagentStop documentation (AC-6) ─────────────
    //
    // AC-6 requires HOST_ABI.md to be updated with the SubagentStop envelope
    // schema section.  These tests verify the documentation contract by
    // reading HOST_ABI.md and asserting the required content is present.
    // They FAIL until the implementer updates HOST_ABI.md per AC-6.
    //
    // NOTE: These are the "failing tests" for the Red Gate.  They exercise
    // real file-system behavior (not serde), and they fail because the
    // stub-architect did not update HOST_ABI.md.

    /// AC-6(a)+(e): HOST_ABI.md must contain a SubagentStop section with
    /// the four field names and a BC-2.02.012 cross-reference.
    ///
    /// Fails until HOST_ABI.md is updated per S-8.30 AC-6.
    #[test]
    fn test_BC_2_02_012_ac6_host_abi_md_contains_subagentstop_section() {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR must be set during cargo test");
        let abi_path = std::path::Path::new(&manifest_dir).join("HOST_ABI.md");
        let content = std::fs::read_to_string(&abi_path)
            .unwrap_or_else(|e| panic!("failed to read {}: {}", abi_path.display(), e));

        // AC-6(e): cross-reference to BC-2.02.012 must be present
        assert!(
            content.contains("BC-2.02.012"),
            "HOST_ABI.md must contain a cross-reference to BC-2.02.012 (AC-6(e))"
        );
    }

    /// AC-6(b): HOST_ABI.md must document that SubagentStop fields are present
    /// ONLY on SubagentStop envelopes and absent for other event types.
    ///
    /// Fails until HOST_ABI.md is updated per S-8.30 AC-6.
    #[test]
    fn test_BC_2_02_012_ac6_host_abi_md_documents_subagentstop_presence_semantics() {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR must be set during cargo test");
        let abi_path = std::path::Path::new(&manifest_dir).join("HOST_ABI.md");
        let content = std::fs::read_to_string(&abi_path)
            .unwrap_or_else(|e| panic!("failed to read {}: {}", abi_path.display(), e));

        // AC-6(b): presence semantics — "SubagentStop" must appear in the doc
        assert!(
            content.contains("SubagentStop"),
            "HOST_ABI.md must document SubagentStop envelope presence semantics (AC-6(b))"
        );
    }

    /// AC-6(c): HOST_ABI.md must contain the canonical agent identity fallback
    /// chain expression (BC-2.02.012 Postcondition 5).
    ///
    /// Fails until HOST_ABI.md is updated per S-8.30 AC-6.
    #[test]
    fn test_BC_2_02_012_ac6_host_abi_md_contains_agent_identity_fallback_chain() {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR must be set during cargo test");
        let abi_path = std::path::Path::new(&manifest_dir).join("HOST_ABI.md");
        let content = std::fs::read_to_string(&abi_path)
            .unwrap_or_else(|e| panic!("failed to read {}: {}", abi_path.display(), e));

        // AC-6(c): canonical agent identity chain from BC-2.02.012 Postcondition 5
        assert!(
            content.contains("agent_type.as_deref()"),
            "HOST_ABI.md must contain the canonical agent identity fallback chain (AC-6(c))"
        );
    }

    /// AC-6(c): HOST_ABI.md must contain the canonical assistant-message
    /// fallback chain expression (BC-2.02.012 Postcondition 6).
    ///
    /// Fails until HOST_ABI.md is updated per S-8.30 AC-6.
    #[test]
    fn test_BC_2_02_012_ac6_host_abi_md_contains_assistant_message_fallback_chain() {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR must be set during cargo test");
        let abi_path = std::path::Path::new(&manifest_dir).join("HOST_ABI.md");
        let content = std::fs::read_to_string(&abi_path)
            .unwrap_or_else(|e| panic!("failed to read {}: {}", abi_path.display(), e));

        // AC-6(c): canonical assistant-message chain from BC-2.02.012 Postcondition 6
        assert!(
            content.contains("last_assistant_message.as_deref()"),
            "HOST_ABI.md must contain the canonical assistant-message fallback chain (AC-6(c))"
        );
    }

    /// AC-6(d): HOST_ABI.md must contain an example SubagentStop JSON envelope
    /// matching BC-2.02.012 canonical test vectors.
    ///
    /// Fails until HOST_ABI.md is updated per S-8.30 AC-6.
    #[test]
    fn test_BC_2_02_012_ac6_host_abi_md_contains_subagentstop_example_json() {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR must be set during cargo test");
        let abi_path = std::path::Path::new(&manifest_dir).join("HOST_ABI.md");
        let content = std::fs::read_to_string(&abi_path)
            .unwrap_or_else(|e| panic!("failed to read {}: {}", abi_path.display(), e));

        // AC-6(d): example JSON must include at least "agent_type" key
        assert!(
            content.contains("agent_type"),
            "HOST_ABI.md must contain an example SubagentStop JSON envelope with agent_type (AC-6(d))"
        );
        // AC-6(a): all four field names must be present in the documentation
        assert!(
            content.contains("subagent_name"),
            "HOST_ABI.md must document the subagent_name field (AC-6(a))"
        );
        assert!(
            content.contains("last_assistant_message"),
            "HOST_ABI.md must document the last_assistant_message field (AC-6(a))"
        );
    }
}
