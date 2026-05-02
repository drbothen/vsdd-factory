---
document_type: adversarial-review
wave: 15
gate_run: rerun-2
producer: adversary
date: 2026-05-02
branch_under_review: c4dc842
verdict: CONVERGED
input_hash: rerun-2-c4dc842
---

# W-15 Wave Gate Re-run #2 — Adversary Review

**Branch:** develop @ c4dc842 (post PR #60 fix-burst)

## Verdict: CONVERGED

All blocking findings from re-run #1 are closed. Two residual items are doc-only (not code defects) and are addressed in fix-burst #3.

### CRIT-PR59-001 (dispatcher AND-gate): CLOSED

executor.rs:90 drops the `on_error == OnError::Block` precondition. The gate now fires on `plugin_requests_block(&outcome.result)` alone, regardless of the plugin's registered `on_error` value. handoff-validator and validate-pr-review-posted both emit stdout `{"outcome":"block","reason":"..."}` in all block paths. Integration tests added:
- `advisory_block_fires_with_on_error_continue` — verifies block fires when plugin emits outcome:block and on_error=continue
- `advisory_block_absent_with_on_error_continue_and_no_block_stdout` — verifies non-block path is not affected

FM4 (advisory block-mode gate) is now functional in production.

### CRIT-CONS-001 (update-wave-state-on-merge null tool_input): CLOSED

Typed BC-2.02.012 projection applied at lib.rs:314-331. Plugin now reads `payload.agent_type.as_deref()` with `.or(payload.subagent_name.as_deref())` fallback, and `payload.last_assistant_message.as_deref()` with `.or(payload.result.as_deref())` fallback. New test `test_pr_manager_works_with_null_tool_input` proves the production scenario (SubagentStop envelope with null tool_input) correctly triggers merge tracking.

### LOW-CONS-002 (doc comment inversion): CLOSED

Doc comment corrected to `!c.is_whitespace()` matching the implementation. Stale "byte-filter" wording updated to "char-filter".

### Residual MED-1 (HOST_ABI.md doc drift): NEW-CONS-001 + NEW-CONS-002

Two doc-only findings introduced by PR #60's behavioral changes:

- **NEW-CONS-001 (Major):** HOST_ABI.md:173,177 — handoff-validator + validate-pr-review-posted bullets omit the stdout outcome:block emission they now perform after the PR #60 fix. Readers of HOST_ABI.md see an incomplete contract for these plugins.
- **NEW-CONS-002 (Minor):** HOST_ABI.md:163-165 — the `on_error="block"` reserved-behavior sentence is misleading post-executor.rs:90 change. Should align with the actual executor behavior (on_error=continue + stdout block is the canonical advisory pattern).

These are doc-only defects. Not blocking rc.3. Addressed in fix-burst #3 (PR #61).

### Residual LOW-1 (test scope)

The advisory-block integration test in PR #60 covers the helper function `plugin_requests_block`, not the full `execute_tiers` call path. A future end-to-end test against `execute_tiers` is recommended as a structural lock. Tracked as TD item (non-blocking).

## Verdict

**CONVERGED** — no blocking code defects. Fix-burst #3 closes HOST_ABI.md doc drift before rc.3 tag.
