---
document_type: consistency-review
wave: 15
gate_run: rerun-2
producer: consistency-validator
date: 2026-05-02
branch_under_review: c4dc842
verdict: FINDINGS
input_hash: rerun-2-c4dc842
---

# W-15 Wave Gate Re-run #2 — Consistency Findings

**Branch:** develop @ c4dc842 (post PR #60 fix-burst)

## Verdict: FINDINGS (doc-only; not code defects)

### CRIT-CONS-001: CLOSED

Typed BC-2.02.012 projection applied in update-wave-state-on-merge. Production scenario (SubagentStop with null tool_input) now correctly triggers merge tracking. Verified by new test `test_pr_manager_works_with_null_tool_input`.

### LOW-CONS-002: CLOSED

Doc comment at track-agent-stop:65 corrected to `!c.is_whitespace()`. Stale "byte-filter" reference at line 184 updated to "char-filter".

### NEW-CONS-001 (Major): HOST_ABI.md plugin bullets incomplete

**File:** `crates/hook-sdk/HOST_ABI.md:173,177`

The handoff-validator and validate-pr-review-posted plugin entries in HOST_ABI.md do not mention that these plugins now emit `{"outcome":"block","reason":"..."}` to stdout when the advisory block condition fires. After PR #60's behavioral change, the HOST_ABI.md description of these two plugins is incomplete — readers see the old contract (hook.block event only) without the new stdout emission.

**Required fix:** Update HOST_ABI.md:173 and :177 to document stdout block emission for both plugins.

### NEW-CONS-002 (Minor): HOST_ABI.md on_error reserved-behavior sentence misleading

**File:** `crates/hook-sdk/HOST_ABI.md:163-165`

The sentence describing `on_error="block"` reserved behavior implies that setting `on_error="block"` is required for advisory blocking. After executor.rs:90 change, the canonical pattern is `on_error="continue"` plus stdout `{"outcome":"block"}` emission. The reserved-behavior sentence should be updated to align with the actual executor implementation at executor.rs:15-17.

**Required fix:** Rewrite HOST_ABI.md:163-165 to describe `on_error="block"` as reserved/legacy and `on_error="continue"` + stdout block emission as the canonical advisory pattern.

## Summary

Both findings are doc-only. No code correctness issues remain. Addressed in fix-burst #3 (PR #61).
