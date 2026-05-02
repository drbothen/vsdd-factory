//! pr-manager-completion-guard — SubagentStop WASM hook plugin.
//!
//! Ports `plugins/vsdd-factory/hooks/pr-manager-completion-guard.sh` to
//! native WASM (wasm32-wasip1). Implements FM4 detection: blocks a
//! pr-manager subagent stop when fewer than 8 of the 9 required PR
//! lifecycle steps have been completed.
//!
//! # Behavior (BC-7.03.045 / BC-7.03.046 / BC-7.03.047 / BC-7.03.048)
//!
//! Fires on SubagentStop. Resolves agent identity and result text via the
//! BC-2.02.012 canonical typed-projection fallback chains, then:
//!
//! - **Non-pr-manager agent:** exits 0 immediately (hook does not apply).
//!   (BC-7.03.045)
//! - **STEP_COMPLETE count >= 8:** exits 0 (all steps complete; no block).
//!   (BC-7.03.046)
//! - **BLOCKED result at line start:** exits 0 (legitimate early exit).
//!   (BC-7.03.047)
//! - **Otherwise (< 8 steps, not BLOCKED):** emits `hook.block` event,
//!   writes multi-line stderr injection with verbatim hint line
//!   `"CONTINUE TO STEP N NOW: <hint>"`, exits 2. (BC-7.03.048)
//! - **Malformed / missing JSON:** graceful exit 0. (BC-7.03.045 invariant 2)
//!
//! # BC-2.02.012 typed-projection usage
//!
//! **Agent identity (BC-2.02.012 Postcondition 5 canonical fallback chain):**
//! ```rust,ignore
//! // Per BC-2.02.012 Postcondition 5:
//! let agent: &str = payload.agent_type.as_deref()
//!     .or(payload.subagent_name.as_deref())
//!     .unwrap_or("unknown");
//! ```
//!
//! **Result text (BC-2.02.012 Postcondition 6 canonical 2-stage chain):**
//! ```rust,ignore
//! // Per BC-2.02.012 Postcondition 6:
//! let result: &str = payload.last_assistant_message.as_deref()
//!     .or(payload.result.as_deref())
//!     .unwrap_or("");
//! ```
//!
//! # Architecture compliance notes
//!
//! - `host::emit_event` replaces all `bin/emit-event` subprocess calls.
//!   `bin/emit-event` is NOT removed (E-8 D-10; deferred to S-8.29).
//! - No dependency on `legacy-bash-adapter` — forbidden per E-8 D-10.
//! - HOST_ABI_VERSION = 1 unchanged (additive projection per D-6 Option A).
//! - `on_error = "block"` in the registry means the dispatcher blocks the
//!   SubagentStop event if this plugin itself panics — this is the
//!   dispatcher-level crash behavior. The hook's own hard-block signal is
//!   HookResult::Block (exit 2).
//! - `host::exec_subprocess` is NOT required: the bash version does not call
//!   `gh` — it reads stdin JSON and writes to stderr only. The `gh` entry in
//!   `binary_allow` is a dormant allowlist entry; the WASM port preserves
//!   this no-gh behavior.

use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// Step hint table (BC-7.03.048 / AC-005)
// ---------------------------------------------------------------------------

/// Select the verbatim continuation hint string for `next_step`.
///
/// Mirrors the 10-arm `case $NEXT_STEP in` block in the bash source
/// (pr-manager-completion-guard.sh lines 65-76). The wildcard arm fires
/// for NEXT_STEP >= 10 (or any value not in 1..=9).
///
/// These strings MUST match the bash source verbatim — any wording drift
/// will be caught by the bats parity tests (AC-006).
pub fn hint_for_step(next_step: u64) -> &'static str {
    unimplemented!("T-6: hint table not yet implemented (stub only)")
}

// ---------------------------------------------------------------------------
// Pure logic helpers (testable without wasmtime)
// ---------------------------------------------------------------------------

/// Return true if `agent` is a pr-manager variant.
///
/// Matches: contains `"pr-manager"` OR contains `"pr_manager"`.
/// Mirrors bash glob `*pr-manager*|*pr_manager*` (AC-003 / T-3 / EC-001).
pub fn is_pr_manager(agent: &str) -> bool {
    unimplemented!("T-3: pr-manager scoping not yet implemented (stub only)")
}

/// Count lines in `text` that contain `STEP_COMPLETE:`.
///
/// Mirrors `grep -c "STEP_COMPLETE:"` — one line with multiple tokens
/// counts as 1, not 2 (line-occurrence semantics per BC-7.03.046 /
/// AC-003). (T-4 body contract)
pub fn count_step_complete_lines(text: &str) -> u64 {
    unimplemented!("T-4: STEP_COMPLETE line counting not yet implemented (stub only)")
}

/// Extract the highest `step=N` number from STEP_COMPLETE lines in `text`,
/// or 0 if none are present.
///
/// Mirrors the bash `grep -oE "STEP_COMPLETE: step=[0-9]+" | grep -oE "[0-9]+$" | sort -n | tail -1`
/// pipeline. (T-6)
pub fn last_step_from_text(text: &str) -> u64 {
    unimplemented!("T-6: last_step extraction not yet implemented (stub only)")
}

/// Return true if `result` matches the BLOCKED pattern at line start.
///
/// Pattern: `^(Status:|##?\s*)?\s*BLOCKED`
///
/// Note: Rust regex crate uses ERE-style alternation — `|` is unescaped
/// alternation; `\|` is a parse error or literal. (AC-004 / T-5)
pub fn is_blocked(result: &str) -> bool {
    unimplemented!("T-5: BLOCKED detection not yet implemented (stub only)")
}

// ---------------------------------------------------------------------------
// Core hook logic with injectable callbacks
// ---------------------------------------------------------------------------

/// Core hook logic with injectable side-effect callbacks.
///
/// Accepts a `HookPayload` and two callbacks so unit tests can drive every
/// branch without a WASM runtime.
///
/// `emit`: called with `(event_type, fields)` when emitting hook.block.
/// `block_stderr`: called with the multi-line stderr string when blocking.
///
/// Returns `HookResult::Continue` (exit 0) for the pass paths and
/// `HookResult::block(reason)` (exit 2) for the FM4 block path.
pub fn pr_manager_guard_logic<E, B>(payload: HookPayload, emit: E, block_stderr: B) -> HookResult
where
    E: FnOnce(&str, &[(&str, &str)]),
    B: FnOnce(&str),
{
    unimplemented!("T-3/T-4/T-5/T-6: guard logic not yet implemented (stub only)")
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use vsdd_hook_sdk::HookPayload;

    fn make_payload(json: &str) -> HookPayload {
        serde_json::from_str(json).expect("fixture should parse")
    }

    fn base_subagentstop(extra: &str) -> String {
        format!(
            r#"{{"event_name":"SubagentStop","session_id":"s","dispatcher_trace_id":"t"{}}}"#,
            if extra.is_empty() {
                String::new()
            } else {
                format!(",{}", extra)
            }
        )
    }

    // ── BC-7.03.045: identity & registry binding ──────────────────────────

    /// BC-7.03.045 postcondition 1 (AC-001): crate name and hook interface
    /// compile and link. This test verifies the crate is well-formed.
    #[test]
    fn test_BC_7_03_045_crate_compiles_and_links() {
        // If this test file compiles, the crate skeleton is valid.
        // Real behavior is tested by the other tests in this module.
    }

    // ── is_pr_manager (BC-7.03.045 scope / T-3) ──────────────────────────

    /// BC-7.03.045 / AC-003: agent containing "pr-manager" matches.
    #[test]
    #[should_panic(expected = "stub only")]
    fn test_BC_7_03_045_is_pr_manager_hyphen_variant_matches() {
        assert!(is_pr_manager("pr-manager"));
    }

    /// EC-001: agent containing "pr_manager" (underscore variant) matches.
    #[test]
    #[should_panic(expected = "stub only")]
    fn test_BC_7_03_045_ec001_is_pr_manager_underscore_variant_matches() {
        assert!(is_pr_manager("pr_manager"));
    }

    /// BC-7.03.045: non-pr-manager agent does not match.
    #[test]
    #[should_panic(expected = "stub only")]
    fn test_BC_7_03_045_is_pr_manager_other_agent_does_not_match() {
        assert!(!is_pr_manager("product-owner"));
    }

    /// BC-7.03.045: empty string does not match.
    #[test]
    #[should_panic(expected = "stub only")]
    fn test_BC_7_03_045_is_pr_manager_empty_string_does_not_match() {
        assert!(!is_pr_manager(""));
    }

    // ── count_step_complete_lines (BC-7.03.046 / T-4) ────────────────────

    /// BC-7.03.046: empty string → 0 lines.
    #[test]
    #[should_panic(expected = "stub only")]
    fn test_BC_7_03_046_count_step_complete_lines_empty_text_returns_zero() {
        assert_eq!(count_step_complete_lines(""), 0);
    }

    /// BC-7.03.046: 9 lines each containing STEP_COMPLETE: → count 9.
    #[test]
    #[should_panic(expected = "stub only")]
    fn test_BC_7_03_046_count_step_complete_lines_nine_lines_returns_nine() {
        let text = "STEP_COMPLETE: step=1 name=populate status=ok note=\n\
                    STEP_COMPLETE: step=2 name=demo status=na note=\n\
                    STEP_COMPLETE: step=3 name=create-pr status=ok note=\n\
                    STEP_COMPLETE: step=4 name=security status=ok note=\n\
                    STEP_COMPLETE: step=5 name=review status=ok note=\n\
                    STEP_COMPLETE: step=6 name=checks status=ok note=\n\
                    STEP_COMPLETE: step=7 name=deps status=ok note=\n\
                    STEP_COMPLETE: step=8 name=merge status=ok note=\n\
                    STEP_COMPLETE: step=9 name=cleanup status=ok note=";
        assert_eq!(count_step_complete_lines(text), 9);
    }

    /// BC-7.03.046 / EC-005: exactly 8 lines → count 8 (minimum passing threshold).
    #[test]
    #[should_panic(expected = "stub only")]
    fn test_BC_7_03_046_count_step_complete_lines_eight_lines_returns_eight() {
        let text = "STEP_COMPLETE: step=1\n\
                    STEP_COMPLETE: step=2\n\
                    STEP_COMPLETE: step=3\n\
                    STEP_COMPLETE: step=4\n\
                    STEP_COMPLETE: step=5\n\
                    STEP_COMPLETE: step=6\n\
                    STEP_COMPLETE: step=7\n\
                    STEP_COMPLETE: step=8";
        assert_eq!(count_step_complete_lines(text), 8);
    }

    /// BC-7.03.046: line-occurrence semantics — one line with two STEP_COMPLETE:
    /// tokens counts as 1, not 2 (matches grep -c behavior).
    #[test]
    #[should_panic(expected = "stub only")]
    fn test_BC_7_03_046_count_step_complete_lines_one_line_two_tokens_counts_as_one() {
        let text = "STEP_COMPLETE: step=1 STEP_COMPLETE: step=2";
        assert_eq!(count_step_complete_lines(text), 1);
    }

    // ── last_step_from_text (BC-7.03.048 / T-6) ──────────────────────────

    /// BC-7.03.048: no step=N in text → LAST_STEP = 0.
    #[test]
    #[should_panic(expected = "stub only")]
    fn test_BC_7_03_048_last_step_from_text_no_steps_returns_zero() {
        assert_eq!(last_step_from_text("some result text without step numbers"), 0);
    }

    /// BC-7.03.048: STEP_COMPLETE with step=5 → LAST_STEP = 5 (highest).
    #[test]
    #[should_panic(expected = "stub only")]
    fn test_BC_7_03_048_last_step_from_text_returns_highest_step() {
        let text = "STEP_COMPLETE: step=3\nSTEP_COMPLETE: step=5\nSTEP_COMPLETE: step=1";
        assert_eq!(last_step_from_text(text), 5);
    }

    /// EC-003: STEP_COMPLETE present but no step=N format → LAST_STEP = 0.
    #[test]
    #[should_panic(expected = "stub only")]
    fn test_BC_7_03_048_ec003_step_complete_without_step_number_returns_zero() {
        let text = "STEP_COMPLETE: name=populate status=ok";
        assert_eq!(last_step_from_text(text), 0);
    }

    // ── is_blocked (BC-7.03.047 / T-5) ───────────────────────────────────

    /// BC-7.03.047: "Status: BLOCKED" at line start → blocked.
    #[test]
    #[should_panic(expected = "stub only")]
    fn test_BC_7_03_047_is_blocked_status_prefix_matches() {
        assert!(is_blocked("Status: BLOCKED\nsome more text"));
    }

    /// BC-7.03.047: "BLOCKED" at line start → blocked.
    #[test]
    #[should_panic(expected = "stub only")]
    fn test_BC_7_03_047_is_blocked_bare_blocked_at_line_start_matches() {
        assert!(is_blocked("BLOCKED: dependency PR not merged"));
    }

    /// BC-7.03.047: "## BLOCKED" → blocked (markdown heading variant).
    #[test]
    #[should_panic(expected = "stub only")]
    fn test_BC_7_03_047_is_blocked_markdown_heading_matches() {
        assert!(is_blocked("## BLOCKED\ndetails follow"));
    }

    /// BC-7.03.047: BLOCKED not at line start → NOT blocked (embedded in text).
    #[test]
    #[should_panic(expected = "stub only")]
    fn test_BC_7_03_047_is_blocked_mid_line_does_not_match() {
        assert!(!is_blocked("step completed but was BLOCKED by CI"));
    }

    // ── pr_manager_guard_logic: non-pr-manager agent (BC-7.03.045) ───────

    /// BC-7.03.045: non-pr-manager agent → Continue (exit 0), no emit.
    #[test]
    #[should_panic(expected = "stub only")]
    fn test_BC_7_03_045_non_pr_manager_agent_passes_through() {
        let payload = make_payload(&base_subagentstop(
            r#""agent_type":"product-owner","last_assistant_message":"all done""#,
        ));
        let mut emitted = false;
        let result = pr_manager_guard_logic(payload, |_, _| { emitted = true; }, |_| {});
        assert_eq!(result, HookResult::Continue, "non-pr-manager must exit 0");
        assert!(!emitted, "non-pr-manager must not emit any event");
    }

    // ── pr_manager_guard_logic: step count >= 8 (BC-7.03.046) ────────────

    /// BC-7.03.046 / AC-003: 9 STEP_COMPLETE lines → Continue (exit 0).
    #[test]
    #[should_panic(expected = "stub only")]
    fn test_BC_7_03_046_nine_steps_complete_passes() {
        let result_text = "STEP_COMPLETE: step=1\n\
                           STEP_COMPLETE: step=2\n\
                           STEP_COMPLETE: step=3\n\
                           STEP_COMPLETE: step=4\n\
                           STEP_COMPLETE: step=5\n\
                           STEP_COMPLETE: step=6\n\
                           STEP_COMPLETE: step=7\n\
                           STEP_COMPLETE: step=8\n\
                           STEP_COMPLETE: step=9";
        let payload = make_payload(&base_subagentstop(&format!(
            r#""agent_type":"pr-manager","last_assistant_message":{}"#,
            serde_json::to_string(result_text).unwrap()
        )));
        let mut emitted = false;
        let result = pr_manager_guard_logic(payload, |_, _| { emitted = true; }, |_| {});
        assert_eq!(result, HookResult::Continue, "9 steps complete must exit 0");
        assert!(!emitted, "9 steps complete must not emit any event");
    }

    /// BC-7.03.046 / AC-003 / EC-005: exactly 8 STEP_COMPLETE lines → Continue (exit 0).
    #[test]
    #[should_panic(expected = "stub only")]
    fn test_BC_7_03_046_exactly_eight_steps_passes() {
        let result_text = "STEP_COMPLETE: step=1\n\
                           STEP_COMPLETE: step=2\n\
                           STEP_COMPLETE: step=3\n\
                           STEP_COMPLETE: step=4\n\
                           STEP_COMPLETE: step=5\n\
                           STEP_COMPLETE: step=6\n\
                           STEP_COMPLETE: step=7\n\
                           STEP_COMPLETE: step=8";
        let payload = make_payload(&base_subagentstop(&format!(
            r#""agent_type":"pr-manager","last_assistant_message":{}"#,
            serde_json::to_string(result_text).unwrap()
        )));
        let result = pr_manager_guard_logic(payload, |_, _| {}, |_| {});
        assert_eq!(result, HookResult::Continue, "exactly 8 steps must exit 0 (>= 8 threshold)");
    }

    // ── pr_manager_guard_logic: BLOCKED detection (BC-7.03.047) ──────────

    /// BC-7.03.047 / AC-004: BLOCKED result → Continue (exit 0).
    #[test]
    #[should_panic(expected = "stub only")]
    fn test_BC_7_03_047_blocked_result_passes() {
        let payload = make_payload(&base_subagentstop(
            r#""agent_type":"pr-manager","last_assistant_message":"Status: BLOCKED\nDependency PR not merged.""#,
        ));
        let mut emitted = false;
        let result = pr_manager_guard_logic(payload, |_, _| { emitted = true; }, |_| {});
        assert_eq!(result, HookResult::Continue, "BLOCKED result must exit 0");
        assert!(!emitted, "BLOCKED result must not emit any event");
    }

    // ── pr_manager_guard_logic: FM4 block path (BC-7.03.048) ─────────────

    /// BC-7.03.048 / AC-005: 0 steps, pr-manager → Block (exit 2) with
    /// NEXT_STEP=1, hint="populate PR description from template".
    #[test]
    #[should_panic(expected = "stub only")]
    fn test_BC_7_03_048_zero_steps_blocks_with_step_one_hint() {
        let payload = make_payload(&base_subagentstop(
            r#""agent_type":"pr-manager","last_assistant_message":"no steps yet""#,
        ));
        let mut emitted_event: Option<String> = None;
        let mut emitted_fields: Vec<(String, String)> = Vec::new();
        let mut stderr_msg: Option<String> = None;

        let result = pr_manager_guard_logic(
            payload,
            |event_type, fields| {
                emitted_event = Some(event_type.to_string());
                emitted_fields = fields.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
            },
            |msg| { stderr_msg = Some(msg.to_string()); },
        );

        assert!(
            matches!(result, HookResult::Block { .. }),
            "0 steps must exit 2 (Block)"
        );
        assert_eq!(emitted_event.as_deref(), Some("hook.block"), "must emit hook.block");
        let next_step = emitted_fields.iter().find(|(k, _)| k == "next_step").map(|(_, v)| v.as_str());
        assert_eq!(next_step, Some("1"), "NEXT_STEP must be 1 when LAST_STEP=0");
        let msg = stderr_msg.unwrap();
        assert!(
            msg.contains("CONTINUE TO STEP 1 NOW: populate PR description from template"),
            "stderr must contain verbatim hint line for step 1"
        );
    }

    /// BC-7.03.048 / AC-005 / EC-002: 7 steps (NEXT_STEP=8) → hint for step 8.
    #[test]
    #[should_panic(expected = "stub only")]
    fn test_BC_7_03_048_ec002_seven_steps_blocks_with_step_eight_hint() {
        let result_text = "STEP_COMPLETE: step=1\n\
                           STEP_COMPLETE: step=2\n\
                           STEP_COMPLETE: step=3\n\
                           STEP_COMPLETE: step=4\n\
                           STEP_COMPLETE: step=5\n\
                           STEP_COMPLETE: step=6\n\
                           STEP_COMPLETE: step=7";
        let payload = make_payload(&base_subagentstop(&format!(
            r#""agent_type":"pr-manager","last_assistant_message":{}"#,
            serde_json::to_string(result_text).unwrap()
        )));
        let mut stderr_msg: Option<String> = None;
        let result = pr_manager_guard_logic(
            payload,
            |_, _| {},
            |msg| { stderr_msg = Some(msg.to_string()); },
        );
        assert!(
            matches!(result, HookResult::Block { .. }),
            "7 steps must exit 2 (Block)"
        );
        let msg = stderr_msg.unwrap();
        assert!(
            msg.contains("CONTINUE TO STEP 8 NOW: spawn github-ops: gh pr merge --squash --delete-branch (AUTHORIZE_MERGE=yes mode)"),
            "stderr must contain verbatim hint line for step 8"
        );
    }

    // ── hint_for_step table (BC-7.03.048 / AC-005 / AC-006) ──────────────

    /// BC-7.03.048 / AC-006: all 9 step positions produce correct verbatim hints.
    #[test]
    #[should_panic(expected = "stub only")]
    fn test_BC_7_03_048_hint_table_all_nine_steps_verbatim() {
        let expected: &[(u64, &str)] = &[
            (1, "populate PR description from template"),
            (2, "verify demo evidence (or emit status=na for chore PRs)"),
            (3, "create PR via github-ops"),
            (4, "spawn security-reviewer via Agent tool"),
            (5, "spawn pr-reviewer/pr-review-triage via Agent tool; handle findings; converge"),
            (6, "spawn github-ops: gh pr checks --watch"),
            (7, "verify all dependency PRs merged"),
            (8, "spawn github-ops: gh pr merge --squash --delete-branch (AUTHORIZE_MERGE=yes mode)"),
            (9, "confirm branch deletion; write review-findings.md; emit final STEP_COMPLETE"),
        ];
        for &(step, hint) in expected {
            assert_eq!(
                hint_for_step(step),
                hint,
                "hint for step {} must match bash source verbatim",
                step
            );
        }
    }

    /// BC-7.03.048 / AC-006 / EC-006: NEXT_STEP=10 → wildcard arm fires.
    #[test]
    #[should_panic(expected = "stub only")]
    fn test_BC_7_03_048_ec006_wildcard_arm_step_ten() {
        assert_eq!(
            hint_for_step(10),
            "continue the 9-step lifecycle",
            "NEXT_STEP=10 must fire wildcard arm"
        );
    }

    /// BC-7.03.048 / AC-006: NEXT_STEP=99 → wildcard arm fires.
    #[test]
    #[should_panic(expected = "stub only")]
    fn test_BC_7_03_048_wildcard_arm_step_ninetynine() {
        assert_eq!(
            hint_for_step(99),
            "continue the 9-step lifecycle",
            "NEXT_STEP=99 must fire wildcard arm"
        );
    }

    // ── BC-2.02.012 typed projection: agent identity fallback chain ───────

    /// BC-2.02.012 Postcondition 5: agent_type absent, subagent_name used for
    /// agent identity resolution.
    #[test]
    #[should_panic(expected = "stub only")]
    fn test_BC_2_02_012_pr_manager_agent_identity_fallback_to_subagent_name() {
        // subagent_name contains "pr-manager" → hook applies.
        let payload = make_payload(&base_subagentstop(
            r#""subagent_name":"pr-manager-fallback","last_assistant_message":"no steps""#,
        ));
        let mut emitted_fields: Vec<(String, String)> = Vec::new();
        pr_manager_guard_logic(
            payload,
            |_, fields| {
                emitted_fields = fields.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
            },
            |_| {},
        );
        let subagent = emitted_fields.iter().find(|(k, _)| k == "subagent").map(|(_, v)| v.as_str());
        assert_eq!(subagent, Some("pr-manager-fallback"), "must use subagent_name as fallback identity");
    }

    /// BC-2.02.012 Postcondition 6: last_assistant_message absent → result used.
    /// (EC-004 variant: result field present but empty → 0 steps → block.)
    #[test]
    #[should_panic(expected = "stub only")]
    fn test_BC_2_02_012_pr_manager_result_fallback_to_result_field() {
        // result field = "" (empty) → 0 steps → block
        let payload = make_payload(&base_subagentstop(
            r#""agent_type":"pr-manager","result":"""#,
        ));
        let result = pr_manager_guard_logic(payload, |_, _| {}, |_| {});
        assert!(
            matches!(result, HookResult::Block { .. }),
            "missing last_assistant_message with empty result must block (exit 2)"
        );
    }

    // ── EC-004: both result fields absent → empty string → block ─────────

    /// EC-004: both last_assistant_message and result absent → empty result
    /// → STEP_COUNT=0 → NEXT_STEP=1 → block.
    #[test]
    #[should_panic(expected = "stub only")]
    fn test_BC_7_03_048_ec004_both_result_fields_absent_blocks_with_step_one() {
        let payload = make_payload(&base_subagentstop(r#""agent_type":"pr-manager""#));
        let mut stderr_msg: Option<String> = None;
        let result = pr_manager_guard_logic(
            payload,
            |_, _| {},
            |msg| { stderr_msg = Some(msg.to_string()); },
        );
        assert!(
            matches!(result, HookResult::Block { .. }),
            "absent result fields must block (exit 2)"
        );
        let msg = stderr_msg.unwrap();
        assert!(
            msg.contains("CONTINUE TO STEP 1 NOW: populate PR description from template"),
            "absent result must yield NEXT_STEP=1 hint"
        );
    }

    // ── AC-008: malformed JSON → graceful exit 0 ──────────────────────────

    /// BC-7.03.045 invariant 2 / AC-008: malformed JSON on stdin → serde
    /// error (handled in main.rs as graceful exit 0).
    #[test]
    fn test_BC_7_03_045_ac008_malformed_json_deserialize_fails_gracefully() {
        let result: Result<HookPayload, _> = serde_json::from_str("not json {{{");
        assert!(result.is_err(), "malformed JSON must produce a serde error (handled in main.rs)");
    }

    // ── HookResult exit code contract ─────────────────────────────────────

    /// BC-7.03.048: Block result has exit code 2.
    #[test]
    fn test_BC_7_03_048_block_result_has_exit_code_two() {
        let r = HookResult::block("pr_manager_incomplete_lifecycle");
        assert_eq!(r.exit_code(), 2, "Block must have exit code 2");
    }

    // ── emit_event fields completeness (BC-7.03.048 / AC-005) ────────────

    /// BC-7.03.048 / AC-005: hook.block event must carry all required fields.
    #[test]
    #[should_panic(expected = "stub only")]
    fn test_BC_7_03_048_emit_event_carries_all_required_fields() {
        let payload = make_payload(&base_subagentstop(
            r#""agent_type":"pr-manager","last_assistant_message":"STEP_COMPLETE: step=3""#,
        ));
        let mut emitted_event: Option<String> = None;
        let mut emitted_fields: Vec<(String, String)> = Vec::new();
        pr_manager_guard_logic(
            payload,
            |event_type, fields| {
                emitted_event = Some(event_type.to_string());
                emitted_fields = fields.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
            },
            |_| {},
        );
        assert_eq!(emitted_event.as_deref(), Some("hook.block"));
        let field_keys: Vec<&str> = emitted_fields.iter().map(|(k, _)| k.as_str()).collect();
        for required in &["hook", "matcher", "reason", "subagent", "step_count", "last_step", "next_step"] {
            assert!(
                field_keys.contains(required),
                "hook.block event must carry '{}' field (BC-7.03.048 / AC-005)",
                required
            );
        }
        let hook = emitted_fields.iter().find(|(k, _)| k == "hook").map(|(_, v)| v.as_str());
        assert_eq!(hook, Some("pr-manager-completion-guard"));
        let matcher = emitted_fields.iter().find(|(k, _)| k == "matcher").map(|(_, v)| v.as_str());
        assert_eq!(matcher, Some("SubagentStop"));
        let reason = emitted_fields.iter().find(|(k, _)| k == "reason").map(|(_, v)| v.as_str());
        assert_eq!(reason, Some("pr_manager_incomplete_lifecycle"));
    }

    // ── AC-005: subagent="unknown" emitted literally, not omitted ─────────

    /// BC-7.03.048 / AC-005: when fallback chain resolves to "unknown",
    /// emit ("subagent", "unknown") — do NOT omit the field.
    /// This test uses a payload where all identity fields are absent but
    /// result contains "pr-manager" in the agent field to trigger scoping.
    /// (NOTE: In practice the scoping check uses agent identity, not result
    /// content — this test verifies the unknown literal emission path by
    /// using a specially crafted payload where agent_type="pr-manager" but
    /// we verify the "unknown" fallback is reached when both are absent.)
    #[test]
    #[should_panic(expected = "stub only")]
    fn test_BC_7_03_048_unknown_agent_field_emitted_as_literal_not_omitted() {
        // Construct a SubagentStop where agent_type is literally "pr-manager"
        // to trigger scoping, with no subagent_name — identity resolves to
        // "pr-manager" (not "unknown" in this case, since agent_type is present).
        // This verifies the subagent field is always included in emit_event.
        let payload = make_payload(&base_subagentstop(
            r#""agent_type":"pr-manager""#,
        ));
        let mut emitted_fields: Vec<(String, String)> = Vec::new();
        pr_manager_guard_logic(
            payload,
            |_, fields| {
                emitted_fields = fields.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
            },
            |_| {},
        );
        // subagent field must always be present, never omitted
        assert!(
            emitted_fields.iter().any(|(k, _)| k == "subagent"),
            "subagent field must always be emitted, never omitted (AC-005)"
        );
    }
}
