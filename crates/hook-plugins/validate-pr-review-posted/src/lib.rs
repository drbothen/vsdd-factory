//! validate-pr-review-posted — SubagentStop WASM hook plugin.
//!
//! Scoped to `pr-reviewer` and `pr-review-triage` subagents only.
//! Runs three independent validation checks against the result text:
//!
//! - Check 1 (BC-7.04.042): pr-review.md was written to .factory/code-delivery/
//! - Check 2 (BC-7.04.043): no `gh pr comment` fallback used
//! - Check 3a/3b (BC-7.04.044): formal `gh pr review` posted with a verdict
//!
//! Errors are accumulated and emitted together; exits 2 if any check fails.
//! Exits 0 immediately for non-pr-reviewer agents (BC-7.04.041).
//!
//! Agent identity resolved via BC-2.02.012 Postcondition 5 canonical chain.
//! Result content resolved via BC-2.02.012 Postcondition 6 canonical chain.

use vsdd_hook_sdk::{HookPayload, HookResult};

/// Top-level hook logic (testable without wasmtime).
///
/// `emit_block`: called with `(subagent: &str)` when errors are present.
/// Returns `HookResult::Continue` — the hook communicates block via exit 2,
/// not via a Block variant (on_error=continue; exit 2 is the block signal).
pub fn validate_pr_review_logic<E>(payload: HookPayload, emit_block: E) -> HookResult
where
    E: FnOnce(&str),
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
    let check1_re = regex::Regex::new(
        r"pr-review\.md|wrote.*review|review.*written|Write.*pr-review",
    )
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
    let verdict_re =
        regex::Regex::new(r"approve|request-changes|APPROVE|REQUEST_CHANGES")
            .expect("verdict regex is valid");
    if result.contains("gh pr review") && !verdict_re.is_match(result) {
        errors.push("Review posted but no verdict (--approve or --request-changes) detected");
    }

    if !errors.is_empty() {
        emit_block(agent);

        // Write formatted error list + remediation instructions to stderr
        // (mirrors bash lines 61-67).
        use std::io::Write as _;
        let stderr = std::io::stderr();
        let mut out = stderr.lock();
        let _ = writeln!(out, "PR REVIEW POSTING INCOMPLETE:");
        for e in &errors {
            let _ = writeln!(out, "  - {e}");
        }
        let _ = writeln!(
            out,
            "  pr-reviewer MUST: (1) write pr-review.md, (2) spawn github-ops with"
        );
        let _ = writeln!(
            out,
            "  'gh pr review --approve' or 'gh pr review --request-changes --body-file'."
        );
        let _ = writeln!(out, "  NEVER use 'gh pr comment' for review verdicts.");

        // Exit 2 — block signal to Claude Code. on_error=continue in registry
        // governs dispatcher crash semantics only; this is the hook's own block.
        std::process::exit(2);
    }

    HookResult::Continue
}
