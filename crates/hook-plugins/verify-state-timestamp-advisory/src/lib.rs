//! verify-state-timestamp-advisory — PostToolUse advisory WASM hook plugin.
//!
//! Emits an advisory on Bash git-commit events where the factory-artifacts HEAD
//! commit included STATE.md but did not advance the `timestamp:` frontmatter field.
//!
//! # Detection mechanism (AC-021)
//!
//! Fires on every PostToolUse Bash event. Reads `git_context` from
//! `payload.extra["git_context"]` (injected by the dispatcher host layer per
//! ADR-032-AC021-prereq). Two pre-conditions gate advisory emission:
//!
//! - **Pre-condition 1** (trigger heuristic): command contains `"git"`, `" commit"`,
//!   and `".factory"`. Spurious fires on develop-branch commits mentioning `.factory`
//!   are possible but bounded: advisory-only, `on_error=continue`.
//! - **Pre-condition 2** (STATE.md modified gate): `state_md_in_commit == "true"`.
//!   Prevents noise on factory-artifacts commits that do not touch STATE.md.
//!
//! When both conditions pass, compares `head_state_timestamp` with
//! `head_parent_state_timestamp`. Byte-identical values → advisory emitted via
//! the injectable `write_advisory` callback.
//!
//! # Behavioral Contracts
//!
//! - BC-5.41.003: exec-free (no `exec_subprocess`).
//! - BC-5.40.001 PC6: fail-open on all error paths.
//!
//! # D-NNN closures
//!
//! - ADR-032 AC-021: PostToolUse per-commit absent-timestamp advisory sentinel.
//!
//! # Architecture compliance
//!
//! - HOST_ABI_VERSION = 1 (no new host functions introduced; `git_context` rides
//!   `extra` map injected by dispatcher host layer).
//! - Fail-open on every error path (BC-5.40.001 PC6; BC-1.16.001 INV3).
//! - No `exec_subprocess` (BC-5.41.003 PC1 unconditionally preserved).
//! - No `unwrap()` or `expect()` in production paths.
//! - Advisory only: always returns `HookResult::Continue`.
//! - No `path_filter` registry field — does not exist in hooks-registry.toml
//!   (ADR-032 §Second deliverable ground-truth verification).

use vsdd_hook_sdk::{HookPayload, HookResult};

/// Advisory text emitted (via stderr) when factory-artifacts HEAD commit
/// included STATE.md but `timestamp:` was not advanced.
///
/// Verbatim from ADR-032 §Second deliverable AC-021 spec.
pub const ADVISORY_MESSAGE: &str = "ADVISORY AC-021: factory-artifacts commit did not advance STATE.md timestamp: \
     \u{2014} verify state-manager burst discipline";

/// HOST_ABI_VERSION declares the ABI contract version this plugin was built against.
pub const HOST_ABI_VERSION: u32 = 1;

/// Returns `true` if `command` qualifies as a factory-artifacts git-commit event
/// under the AC-021 Pre-condition 1 heuristic.
///
/// Requires all three substrings: `"git"`, `" commit"` (with leading space to
/// exclude "commit" used as a noun), and `".factory"`.
pub fn detect_git_commit_event(command: &str) -> bool {
    command.contains("git") && command.contains(" commit") && command.contains(".factory")
}

/// PostToolUse advisory entry point.
///
/// Applies Pre-condition 1 + Pre-condition 2 gates, then compares timestamps.
/// Calls `write_advisory` when both conditions pass and timestamps are byte-identical.
///
/// Always returns `HookResult::Continue` — advisory only, never blocks.
///
/// # Injectable callback
///
/// `write_advisory: impl FnOnce(&str)` is injectable for unit-testing without
/// a WASM runtime. The production `main.rs` wires it as `|s| eprintln!("{s}")`.
pub fn on_hook_logic(payload: HookPayload, write_advisory: impl FnOnce(&str)) -> HookResult {
    // Only inspect Bash tool invocations.
    if payload.tool_name != "Bash" {
        return HookResult::Continue;
    }

    // Pre-condition 1: heuristic factory-artifacts git-commit scope filter.
    let command = payload
        .tool_input
        .get("command")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if !detect_git_commit_event(command) {
        return HookResult::Continue;
    }

    // Extract git_context from payload.extra. Fail-open if absent.
    let git_context = match payload.extra.get("git_context") {
        Some(v) => v,
        None => return HookResult::Continue,
    };

    // Pre-condition 2: STATE.md must be present in the factory-artifacts HEAD commit.
    // Absent, empty, or "false" → Continue (no advisory noise on burst-log-only commits).
    let state_md_in_commit = git_context
        .get("state_md_in_commit")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if state_md_in_commit != "true" {
        return HookResult::Continue;
    }

    // Read head_state_timestamp. Fail-open (Continue) if absent or empty.
    let head_ts = git_context
        .get("head_state_timestamp")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if head_ts.is_empty() {
        return HookResult::Continue;
    }

    // Read head_parent_state_timestamp. Fail-open (Continue) if absent or empty.
    // Empty on initial commit (no HEAD^) or when dispatcher prereq not yet deployed.
    let parent_ts = git_context
        .get("head_parent_state_timestamp")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if parent_ts.is_empty() {
        return HookResult::Continue;
    }

    // Emit advisory when timestamps are byte-identical — timestamp was not advanced.
    if head_ts == parent_ts {
        write_advisory(ADVISORY_MESSAGE);
    }

    // Always Continue — advisory only; PostToolUse hooks cannot prevent commits.
    HookResult::Continue
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use vsdd_hook_sdk::HookPayload;

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    fn make_bash_payload_with_git_context(
        command: &str,
        state_md_in_commit: &str,
        head_ts: &str,
        parent_ts: &str,
    ) -> HookPayload {
        let json = serde_json::json!({
            "event_name": "PostToolUse",
            "session_id": "test-session",
            "dispatcher_trace_id": "test-trace",
            "tool_name": "Bash",
            "tool_input": { "command": command },
            "tool_response": {},
            "git_context": {
                "head_subject": "state: advance STATE.md",
                "head_sha": "abc1234",
                "head_parent_subject": "fix: prior commit",
                "head_parent_sha": "def5678",
                "head_state_timestamp": head_ts,
                "head_parent_state_timestamp": parent_ts,
                "state_md_in_commit": state_md_in_commit,
            }
        });
        serde_json::from_value(json).expect("valid PostToolUse payload")
    }

    fn make_bash_payload_no_git_context(command: &str) -> HookPayload {
        let json = serde_json::json!({
            "event_name": "PostToolUse",
            "session_id": "test-session",
            "dispatcher_trace_id": "test-trace",
            "tool_name": "Bash",
            "tool_input": { "command": command },
            "tool_response": {},
        });
        serde_json::from_value(json).expect("valid PostToolUse payload")
    }

    // -----------------------------------------------------------------------
    // AC-021 unit tests (3 required by ADR-032 §Second deliverable)
    // -----------------------------------------------------------------------

    /// AC-021 positive path: Bash command qualifies Pre-condition 1, git_context
    /// present with state_md_in_commit="true" and byte-identical timestamps →
    /// advisory emitted, result is Continue.
    #[test]
    fn ac021_commit_stale_timestamp_emits_advisory() {
        let command = "git -C .factory commit -m 'state: advance STATE.md'";
        let payload = make_bash_payload_with_git_context(
            command,
            "true",
            "2026-01-01T00:00:00Z",
            "2026-01-01T00:00:00Z", // byte-identical → stale
        );

        let mut advisory_captured: Option<String> = None;
        let result = on_hook_logic(payload, |msg| {
            advisory_captured = Some(msg.to_string());
        });

        assert_eq!(
            result,
            HookResult::Continue,
            "AC-021 advisory plugin must always return Continue (never blocks)"
        );
        let emitted = advisory_captured.expect(
            "advisory must be emitted when timestamps are byte-identical (Pre-condition 2 met)",
        );
        assert_eq!(
            emitted, ADVISORY_MESSAGE,
            "advisory text must match the verbatim spec-prescribed message"
        );
    }

    /// AC-021 Pre-condition 2 gate: state_md_in_commit = "false" → no advisory
    /// even when timestamps are identical.
    #[test]
    fn ac021_state_md_not_in_commit_no_advisory() {
        let command = "git -C .factory commit -m 'cycle: burst-log only'";
        let payload = make_bash_payload_with_git_context(
            command,
            "false", // STATE.md NOT in commit — Pre-condition 2 fails
            "2026-01-01T00:00:00Z",
            "2026-01-01T00:00:00Z",
        );

        let mut advisory_emitted = false;
        let result = on_hook_logic(payload, |_| {
            advisory_emitted = true;
        });

        assert_eq!(result, HookResult::Continue);
        assert!(
            !advisory_emitted,
            "no advisory must be emitted when state_md_in_commit is false (Pre-condition 2 gate)"
        );
    }

    /// AC-021 fail-open path: git_context absent from payload.extra → no advisory,
    /// Continue. Covers non-git-commit Bash events and initial commits.
    #[test]
    fn ac021_git_context_absent_no_advisory() {
        let command = "git -C .factory commit -m 'state: update STATE.md'";
        let payload = make_bash_payload_no_git_context(command);

        let mut advisory_emitted = false;
        let result = on_hook_logic(payload, |_| {
            advisory_emitted = true;
        });

        assert_eq!(result, HookResult::Continue);
        assert!(
            !advisory_emitted,
            "no advisory when git_context absent from payload.extra (fail-open per BC-5.40.001 PC6)"
        );
    }
}
