---
document_type: adversarial-review
wave: 15
gate_run: rerun-1
producer: adversary
date: 2026-05-02
branch_under_review: 1ab1d6f
verdict: FINDINGS
input_hash: rerun-1-1ab1d6f
---

# W-15 Wave Gate Re-run #1 — Adversary Findings

**Branch:** develop @ 1ab1d6f (post PR #59 fix-burst)

## Re-run #1 verdict: FINDINGS — BLOCKING rc.3

Original 5 CRIT + 6 HIGH all CLOSED by PR #59 EXCEPT one regression introduced:

### CRIT-PR59-001 — Advisory block-mode pattern is inert in production dispatcher

| Field | Value |
|-------|-------|
| Severity | CRITICAL |
| Files | `crates/factory-dispatcher/src/executor.rs:89`; `crates/hook-sdk/HOST_ABI.md:146-184`; `plugins/vsdd-factory/hooks-registry.toml:859,870` |

The fix-burst's "canonical advisory-block-mode" pattern is broken at the dispatcher boundary. The dispatcher's gate is:

```rust
if matches!(outcome.on_error, OnError::Block) && plugin_requests_block(&outcome.result)
```

This is an AND. PR #59 flipped handoff-validator + pr-manager-completion-guard to `on_error="continue"`, so the second clause cannot fire. The new HOST_ABI.md prose claims "If outcome=block, dispatcher exits with non-zero status" but the code disagrees. Two of the three "canonical block-mode" plugins (handoff-validator, validate-pr-review-posted) don't even emit the stdout JSON line — only `hook.block` events. pr-manager-completion-guard's `println!(r#"{{"outcome":"block","reason":"..."}}"#)` at lib.rs:262 is dead code from the dispatcher's perspective. FM4 (the entire reason for the plugin) is silently no-op.

**Recommended fix:** Drop `on_error == OnError::Block` precondition in executor.rs:89; add stdout `println!` to handoff-validator + validate-pr-review-posted; add integration test wiring on_error=continue + stdout=outcome:block end-to-end.

### Original CRIT findings — all closed

- CRIT-W15-001 (release pipeline): CLOSED — release.yml + ci.yml build all 16 wasm + count-verification step
- CRIT-W15-002/HIGH-W15-003 (block-mode pattern): NOT-CLOSED — introduces CRIT-PR59-001
- CRIT-W15-003/SEC-001 (WASI preopen): CLOSED as documentation-only
- CRIT-W15-004 (regex): CLOSED — `merged|squash.*merge` + regression tests
- CRIT-W15-005 (Tier-1-only): CLOSED — docs/E-8-tier1-native-audit.md qualifies as Tier 1

### Original HIGH findings — all closed

- HIGH-W15-002 (whitespace): CLOSED — chars-based filter
- HIGH-W15-004 (default features): CLOSED — `default = []`
- 2 clippy errors: CLOSED — `cargo clippy -p track-agent-stop -- -D warnings` passes
- SEC-003 (VSDD_SINK_FILE): CLOSED — `#[cfg(debug_assertions)]` gate + path-traversal rejection

## Verdict

**FINDINGS** — BLOCKING rc.3. Recommended fix-burst #2 to address CRIT-PR59-001.
