//! validate-pr-review-posted — SubagentStop WASM hook plugin.
//!
//! Scoped to `pr-reviewer` and `pr-review-triage` subagents only.
//! Runs three independent validation checks against the result text:
//!
//! - Check 1 (BC-7.04.042): pr-review.md was written to .factory/code-delivery/
//! - Check 2 (BC-7.04.043): no `gh pr comment` fallback used
//! - Check 3a/3b (BC-7.04.044): formal `gh pr review` posted with a verdict
//!
//! Errors are accumulated and emitted together via a `hook.block` event plus
//! stderr message. Always returns `HookResult::Continue` (advisory block-mode:
//! emit the warning event + stderr rather than returning HookResult::Block).
//! Exits 0 immediately for non-pr-reviewer agents (BC-7.04.041).
//!
//! Agent identity resolved via BC-2.02.012 Postcondition 5 canonical chain.
//! Result content resolved via BC-2.02.012 Postcondition 6 canonical chain.

use vsdd_hook_sdk::{HookPayload, HookResult};

/// Top-level hook logic (testable without wasmtime).
///
/// `emit_block`: called with `(subagent: &str)` when checks fail — emits the
///   `hook.block` event.
/// `write_stderr`: called with the formatted error + remediation text when checks fail.
/// `print_stdout`: called with `{"outcome":"block","reason":"pr_review_invalid"}` to
///   signal advisory block to the dispatcher. Dispatcher reads this stdout line
///   regardless of `on_error` (W-15 gate fix CRIT-PR59-001).
///
/// Returns `HookResult::Continue` in all cases — the hook communicates the block
/// via the `hook.block` warning event, stdout outcome line, and stderr message
/// (advisory block-mode); `on_error=continue` in the registry governs dispatcher
/// crash semantics only.
pub fn validate_pr_review_logic<E, W, P>(
    payload: HookPayload,
    emit_block: E,
    write_stderr: W,
    print_stdout: P,
) -> HookResult
where
    E: FnOnce(&str),
    W: FnOnce(&str),
    P: FnOnce(&str),
{
    // BC-2.02.012 Postcondition 5: canonical agent identity fallback chain.
    // Mirrors validate-pr-review-posted.sh:21:
    //   AGENT=$(echo "$INPUT" | jq -r '.agent_type // .subagent_name // "unknown"')
    let agent: &str = payload
        .agent_type
        .as_deref()
        .or(payload.subagent_name.as_deref())
        .unwrap_or("unknown");

    // BC-7.04.041: scope to pr-reviewer / pr-review-triage only.
    // Matching mechanism: substring containment (bash case *pr-reviewer*|...).
    // Dotted variants (pr.reviewer) are NOT canonical and must NOT match.
    if !agent.contains("pr-reviewer")
        && !agent.contains("pr_reviewer")
        && !agent.contains("pr-review-triage")
    {
        return HookResult::Continue;
    }

    // BC-2.02.012 Postcondition 6: canonical assistant-message fallback chain.
    // Mirrors validate-pr-review-posted.sh:22:
    //   RESULT=$(echo "$INPUT" | jq -r '.last_assistant_message // .result // empty')
    let result: &str = payload
        .last_assistant_message
        .as_deref()
        .or(payload.result.as_deref())
        .unwrap_or("");

    let mut errors: Vec<&str> = Vec::new();

    // Check 1 (BC-7.04.042): pr-review.md was written.
    // Raw string: `\.` is literal backslash-dot — regex crate matches literal `.`
    // (parity with bash grep -qE "pr-review\.md|...").
    // All patterns are case-sensitive (Rust regex default = bash grep -E default).
    let check1_re =
        regex::Regex::new(r"pr-review\.md|wrote.*review|review.*written|Write.*pr-review")
            .expect("check1 regex is valid");
    if !check1_re.is_match(result) {
        errors.push("pr-review.md may not have been written to .factory/code-delivery/");
    }

    // Check 2 (BC-7.04.043): no gh pr comment fallback.
    if result.contains("gh pr comment") {
        errors.push(
            "Used 'gh pr comment' instead of 'gh pr review' \
             — findings won't show as a formal review verdict",
        );
    }

    // Check 3a (BC-7.04.044): formal review posted.
    let check3a_re = regex::Regex::new(
        r"gh pr review|pr review.*posted|review.*posted.*GitHub|APPROVE|REQUEST_CHANGES",
    )
    .expect("check3a regex is valid");
    if !check3a_re.is_match(result) {
        errors.push("No evidence that a formal GitHub review was posted via 'gh pr review'");
    }

    // Check 3b (BC-7.04.044): formal review posted but no verdict detected.
    // Gate: literal `gh pr review` token only — NOT a re-test of Check 3a's
    // full disjunction. Check 3b is independent of Check 3a.
    let verdict_re = regex::Regex::new(r"approve|request-changes|APPROVE|REQUEST_CHANGES")
        .expect("verdict regex is valid");
    if result.contains("gh pr review") && !verdict_re.is_match(result) {
        errors.push("Review posted but no verdict (--approve or --request-changes) detected");
    }

    if !errors.is_empty() {
        emit_block(agent);
        // Advisory block signal to dispatcher (W-15 gate fix CRIT-PR59-001).
        // Dispatcher reads this stdout line regardless of on_error setting.
        print_stdout(r#"{"outcome":"block","reason":"pr_review_invalid"}"#);

        // Build formatted error list + remediation instructions
        // (mirrors bash lines 61-67).
        let mut msg = String::from("PR REVIEW POSTING INCOMPLETE:\n");
        for e in &errors {
            msg.push_str(&format!("  - {e}\n"));
        }
        msg.push_str("  pr-reviewer MUST: (1) write pr-review.md, (2) spawn github-ops with\n");
        msg.push_str(
            "  'gh pr review --approve' or 'gh pr review --request-changes --body-file'.\n",
        );
        msg.push_str("  NEVER use 'gh pr comment' for review verdicts.\n");

        write_stderr(&msg);
    }

    HookResult::Continue
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

    /// BC-7.04.041: non-pr-reviewer agent → HookResult::Continue, no emit, no stderr.
    #[test]
    fn test_BC_7_04_041_non_pr_reviewer_exits_immediately() {
        let payload = make_payload(&base_subagentstop(
            r#""agent_type":"code-reviewer","last_assistant_message":"wrote pr-review.md and gh pr review --approve""#,
        ));
        let mut emitted = false;
        let mut warned = false;
        let result = validate_pr_review_logic(
            payload,
            |_| {
                emitted = true;
            },
            |_| {
                warned = true;
            },
            |_| {},
        );
        assert_eq!(result, HookResult::Continue);
        assert!(!emitted, "non-pr-reviewer must not emit event");
        assert!(!warned, "non-pr-reviewer must not write stderr");
    }

    /// AC-003 g.1: agent_type present, subagent_name absent — primary arm of fallback chain.
    #[test]
    fn test_BC_2_02_012_agent_type_primary_arm() {
        let payload = make_payload(&base_subagentstop(
            r#""agent_type":"pr-reviewer","last_assistant_message":"wrote pr-review.md and gh pr review --approve""#,
        ));
        let mut emitted = false;
        validate_pr_review_logic(
            payload,
            |_| {
                emitted = true;
            },
            |_| {},
            |_| {},
        );
        assert!(
            !emitted,
            "all-pass with pr-reviewer via agent_type must not emit block"
        );
    }

    /// AC-003 g.2: agent_type absent, subagent_name present — fallback arm exercised.
    #[test]
    fn test_BC_2_02_012_subagent_name_fallback_arm() {
        let payload = make_payload(&base_subagentstop(
            r#""subagent_name":"pr-reviewer","last_assistant_message":"wrote pr-review.md and gh pr review --approve""#,
        ));
        let mut emitted = false;
        validate_pr_review_logic(
            payload,
            |_| {
                emitted = true;
            },
            |_| {},
            |_| {},
        );
        assert!(
            !emitted,
            "all-pass via subagent_name fallback must not emit block"
        );
    }

    /// BC-7.04.041: pr-review-triage agent matches containment check.
    #[test]
    fn test_BC_7_04_041_pr_review_triage_matches() {
        // pr-review-triage with a bad result (no review.md) → should emit block
        let payload = make_payload(&base_subagentstop(
            r#""agent_type":"pr-review-triage","last_assistant_message":"done nothing""#,
        ));
        let mut emitted = false;
        validate_pr_review_logic(
            payload,
            |_| {
                emitted = true;
            },
            |_| {},
            |_| {},
        );
        assert!(emitted, "pr-review-triage must apply checks");
    }

    /// BC-7.04.042: Check 1 — pr-review.md not written → accumulate error.
    #[test]
    fn test_BC_7_04_042_check1_pr_review_md_not_written() {
        let payload = make_payload(&base_subagentstop(
            r#""agent_type":"pr-reviewer","last_assistant_message":"gh pr review --approve posted""#,
        ));
        let mut stderr_msg: Option<String> = None;
        validate_pr_review_logic(
            payload,
            |_| {},
            |msg| {
                stderr_msg = Some(msg.to_string());
            },
            |_| {},
        );
        let msg = stderr_msg.expect("should have written stderr");
        assert!(
            msg.contains("pr-review.md may not have been written"),
            "check1 error expected"
        );
    }

    /// BC-7.04.042: Check 1 passes via `wrote.*review` pattern.
    #[test]
    fn test_BC_7_04_042_check1_wrote_review_pattern_passes() {
        let payload = make_payload(&base_subagentstop(
            r#""agent_type":"pr-reviewer","last_assistant_message":"wrote my review notes and gh pr review --approve""#,
        ));
        let mut emitted = false;
        validate_pr_review_logic(
            payload,
            |_| {
                emitted = true;
            },
            |_| {},
            |_| {},
        );
        assert!(!emitted, "wrote.*review must satisfy check 1");
    }

    /// BC-7.04.043: Check 2 — gh pr comment detected → accumulate error.
    #[test]
    fn test_BC_7_04_043_check2_gh_pr_comment_detected() {
        let payload = make_payload(&base_subagentstop(
            r#""agent_type":"pr-reviewer","last_assistant_message":"wrote pr-review.md and ran gh pr comment --body findings""#,
        ));
        let mut stderr_msg: Option<String> = None;
        validate_pr_review_logic(
            payload,
            |_| {},
            |msg| {
                stderr_msg = Some(msg.to_string());
            },
            |_| {},
        );
        let msg = stderr_msg.expect("should have written stderr");
        assert!(msg.contains("gh pr comment"), "check2 error expected");
    }

    /// BC-7.04.044: Check 3a — no formal review → accumulate error.
    #[test]
    fn test_BC_7_04_044_check3a_no_formal_review() {
        let payload = make_payload(&base_subagentstop(
            r#""agent_type":"pr-reviewer","last_assistant_message":"wrote pr-review.md and submitted findings""#,
        ));
        let mut stderr_msg: Option<String> = None;
        validate_pr_review_logic(
            payload,
            |_| {},
            |msg| {
                stderr_msg = Some(msg.to_string());
            },
            |_| {},
        );
        let msg = stderr_msg.expect("should have written stderr");
        assert!(
            msg.contains("No evidence that a formal GitHub review"),
            "check3a error expected"
        );
    }

    /// BC-7.04.044: Check 3b — gh pr review with no verdict → accumulate error.
    #[test]
    fn test_BC_7_04_044_check3b_review_posted_no_verdict() {
        // Case (e) from story spec: gh pr review --no-body (no approve/request-changes)
        let payload = make_payload(&base_subagentstop(
            r#""agent_type":"pr-reviewer","last_assistant_message":"ran gh pr review --no-body""#,
        ));
        let mut stderr_msg: Option<String> = None;
        validate_pr_review_logic(
            payload,
            |_| {},
            |msg| {
                stderr_msg = Some(msg.to_string());
            },
            |_| {},
        );
        let msg = stderr_msg.expect("should have written stderr for check 3b");
        assert!(
            msg.contains("no verdict"),
            "check3b error expected; got: {msg}"
        );
        // Check 3a must NOT have fired (gh pr review matched 3a)
        assert!(
            !msg.contains("No evidence"),
            "check3a must not fire when gh pr review is present"
        );
    }

    /// All three checks pass → no emit, no stderr, HookResult::Continue.
    #[test]
    fn test_all_checks_pass_no_output() {
        let payload = make_payload(&base_subagentstop(
            r#""agent_type":"pr-reviewer","last_assistant_message":"wrote pr-review.md and posted gh pr review --approve""#,
        ));
        let mut emitted = false;
        let mut warned = false;
        let result = validate_pr_review_logic(
            payload,
            |_| {
                emitted = true;
            },
            |_| {
                warned = true;
            },
            |_| {},
        );
        assert_eq!(result, HookResult::Continue);
        assert!(!emitted, "all-pass must not emit event");
        assert!(!warned, "all-pass must not write stderr");
    }

    /// Multiple checks fail → all errors accumulated, single emit + single stderr call.
    #[test]
    fn test_multiple_checks_fail_accumulated() {
        // No pr-review.md + uses gh pr comment + no formal review → 3 errors
        let payload = make_payload(&base_subagentstop(
            r#""agent_type":"pr-reviewer","last_assistant_message":"ran gh pr comment --body findings""#,
        ));
        let mut emit_count = 0usize;
        let mut stderr_msg: Option<String> = None;
        validate_pr_review_logic(
            payload,
            |_| {
                emit_count += 1;
            },
            |msg| {
                stderr_msg = Some(msg.to_string());
            },
            |_| {},
        );
        assert_eq!(
            emit_count, 1,
            "should emit exactly once even with multiple errors"
        );
        let msg = stderr_msg.expect("stderr should have been written");
        assert!(
            msg.contains("pr-review.md"),
            "check1 error in multi-failure"
        );
        assert!(
            msg.contains("gh pr comment"),
            "check2 error in multi-failure"
        );
        assert!(
            msg.contains("No evidence"),
            "check3a error in multi-failure"
        );
    }

    /// Remediation block is always included in stderr when errors exist.
    #[test]
    fn test_remediation_block_present_on_error() {
        let payload = make_payload(&base_subagentstop(
            r#""agent_type":"pr-reviewer","last_assistant_message":"done nothing""#,
        ));
        let mut stderr_msg: Option<String> = None;
        validate_pr_review_logic(
            payload,
            |_| {},
            |msg| {
                stderr_msg = Some(msg.to_string());
            },
            |_| {},
        );
        let msg = stderr_msg.expect("stderr expected");
        assert!(
            msg.contains("pr-reviewer MUST"),
            "remediation line 1 present"
        );
        assert!(
            msg.contains("gh pr review --approve"),
            "remediation line 2 present"
        );
        assert!(
            msg.contains("NEVER use 'gh pr comment'"),
            "remediation line 3 present"
        );
    }

    /// BC-7.04.044 EC-003: result contains `gh pr review --approve` → all pass.
    #[test]
    fn test_BC_7_04_044_review_with_approve_verdict_all_pass() {
        let payload = make_payload(&base_subagentstop(
            r#""agent_type":"pr-reviewer","last_assistant_message":"wrote pr-review.md; ran gh pr review --approve""#,
        ));
        let mut emitted = false;
        validate_pr_review_logic(
            payload,
            |_| {
                emitted = true;
            },
            |_| {},
            |_| {},
        );
        assert!(!emitted, "gh pr review --approve should satisfy all checks");
    }

    /// HookResult is always Continue (never Block or Error).
    #[test]
    fn test_hook_always_returns_continue() {
        let cases = [
            // all pass
            base_subagentstop(
                r#""agent_type":"pr-reviewer","last_assistant_message":"wrote pr-review.md and gh pr review --approve""#,
            ),
            // check failures
            base_subagentstop(
                r#""agent_type":"pr-reviewer","last_assistant_message":"done nothing""#,
            ),
            // non-pr-reviewer
            base_subagentstop(r#""agent_type":"implementer","last_assistant_message":"""#),
        ];
        for json in &cases {
            let payload = make_payload(json);
            let result = validate_pr_review_logic(payload, |_| {}, |_| {}, |_| {});
            assert_eq!(result, HookResult::Continue, "must always return Continue");
        }
    }
}
