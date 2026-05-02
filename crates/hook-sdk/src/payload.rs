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
    /// (BC-2.02.012 Postcondition 1 and 5):
    /// ```text
    /// payload.agent_type.as_deref().or(payload.subagent_name.as_deref()).unwrap_or("unknown")
    /// ```
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
    /// (BC-2.02.012 Postcondition 3 and 6):
    /// ```text
    /// payload.last_assistant_message.as_deref().or(payload.result.as_deref()).unwrap_or("")
    /// ```
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
}
