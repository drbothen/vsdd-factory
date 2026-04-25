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
}
