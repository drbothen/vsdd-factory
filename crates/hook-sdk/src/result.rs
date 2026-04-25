//! Hook result — the value a plugin's `#[hook]` function returns.

use serde::{Deserialize, Serialize};

/// What a plugin tells the dispatcher after running.
///
/// Maps directly onto the dispatcher's exit-code contract for blocking
/// hooks (PreToolUse, PermissionRequest):
///
/// - `Continue`  → exit code `0` (allow)
/// - `Block`     → exit code `2` (block; reason surfaced to Claude Code)
/// - `Error`     → exit code `1` (plugin failed; non-blocking by default)
///
/// The dispatcher serializes this enum to JSON and to the appropriate
/// process exit code; plugin authors don't manage exit codes themselves.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum HookResult {
    /// Allow the tool call (or non-blocking event) to proceed normally.
    Continue,

    /// Block the tool call. The `reason` is surfaced to Claude Code so
    /// the user understands why; keep it short and actionable.
    Block { reason: String },

    /// The plugin failed. The `message` is logged to the dispatcher's
    /// internal log and emitted as an `internal.host_function_panic` /
    /// `plugin.crashed` event but does **not** block the call by default
    /// — operators set `on_error = "block"` in `hooks-registry.toml` if
    /// they want plugin failures to be hard stops.
    Error { message: String },
}

impl HookResult {
    /// Convenience constructor: `HookResult::block("reason")`.
    pub fn block(reason: impl Into<String>) -> Self {
        HookResult::Block {
            reason: reason.into(),
        }
    }

    /// Convenience constructor: `HookResult::error("message")`.
    pub fn error(message: impl Into<String>) -> Self {
        HookResult::Error {
            message: message.into(),
        }
    }

    /// Process exit code that the runtime will use after serializing.
    /// Public so the dispatcher (and tests) can rely on the same mapping.
    pub fn exit_code(&self) -> i32 {
        match self {
            HookResult::Continue => 0,
            HookResult::Block { .. } => 2,
            HookResult::Error { .. } => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn continue_serializes_with_outcome_tag() {
        let s = serde_json::to_string(&HookResult::Continue).unwrap();
        assert_eq!(s, r#"{"outcome":"continue"}"#);
    }

    #[test]
    fn block_serializes_with_reason() {
        let s = serde_json::to_string(&HookResult::block("policy 9 violation")).unwrap();
        assert_eq!(s, r#"{"outcome":"block","reason":"policy 9 violation"}"#);
    }

    #[test]
    fn error_serializes_with_message() {
        let s = serde_json::to_string(&HookResult::error("disk full")).unwrap();
        assert_eq!(s, r#"{"outcome":"error","message":"disk full"}"#);
    }

    #[test]
    fn round_trip_block() {
        let original = HookResult::block("nope");
        let json = serde_json::to_string(&original).unwrap();
        let back: HookResult = serde_json::from_str(&json).unwrap();
        assert_eq!(original, back);
    }

    #[test]
    fn exit_codes_match_blocking_contract() {
        assert_eq!(HookResult::Continue.exit_code(), 0);
        assert_eq!(HookResult::block("x").exit_code(), 2);
        assert_eq!(HookResult::error("y").exit_code(), 1);
    }
}
