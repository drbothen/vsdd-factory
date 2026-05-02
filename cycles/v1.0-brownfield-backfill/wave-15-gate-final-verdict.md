---
document_type: wave-gate-final-verdict
wave: 15
verdict: CONVERGED
date: 2026-05-02
develop_head: d49f33b
fix_bursts: [PR-59, PR-60, PR-61]
---

# W-15 Wave Gate — Final Convergence Verdict

**Wave:** 15 (E-8 Tier 1 native WASM migration)
**Final develop HEAD:** d49f33b
**Date:** 2026-05-02
**Verdict:** CONVERGED

---

## Convergence Narrative

### Original gate run — BLOCKED

Adversary: 5 CRITICAL + 6 HIGH + 7 MEDIUM + 7 LOW. Security: 1 HIGH (SEC-003) + 3 MEDIUM + 2 LOW. Implementer: 2 clippy errors; 19 bats failures (all due to missing WASM artifacts). Findings persisted to factory-artifacts (D-204).

Key original findings:
- CRIT-W15-001: release.yml built only 3 of 16 native WASM plugins — every rc release would produce activation-time LoadFailed for 9 missing plugins
- CRIT-W15-002: advisory block-mode pattern broken — dispatcher AND-gate silently no-ops
- CRIT-W15-003: WASI preopened_dir surface undocumented
- CRIT-W15-004: update-wave-state-on-merge regex false-positive (`merge|squash` matched unrelated strings)
- CRIT-W15-005: "Tier 1 only" claim unsubstantiated
- HIGH-W15-002: handoff-validator vs track-agent-stop whitespace counting (chars vs bytes)
- HIGH-W15-004: update-wave-state-on-merge `default = ["standalone"]` fail-safe inversion
- SEC-003: VSDD_SINK_FILE path injection in production builds

### Fix-burst #1 (PR #59) — 9 commits — develop 3adfe0b → 1ab1d6f

Closed all 5 original CRIT + 5 of 6 original HIGH + SEC-003. Introduced 2 regressions:
- **CRIT-PR59-001:** The "canonical advisory-block-mode" fix itself introduced an AND-gate pattern in executor.rs:89 (`on_error==Block AND stdout-block`). PR #59 had simultaneously set plugins to `on_error="continue"`, making the second clause unreachable. FM4 was silently no-op in production.
- **CRIT-CONS-001:** update-wave-state-on-merge still read agent identity via `tool_input.get("agent_type")`. The dispatcher's SubagentStop envelope sends `tool_input: null`. Merge tracking never fired in production. Test fixtures had masked this by populating tool_input.

### Re-run #1 — BLOCKED

Adversary: FINDINGS (CRIT-PR59-001). Consistency: FINDINGS (CRIT-CONS-001 + LOW-CONS-002). Security: CONVERGED.

### Fix-burst #2 (PR #60) — 3 commits — develop 1ab1d6f → c4dc842

- executor.rs:90: dropped `on_error==Block` precondition — block fires on stdout emission alone
- handoff-validator + validate-pr-review-posted: added stdout `{"outcome":"block","reason":"..."}` emission in all block paths
- update-wave-state-on-merge: replaced tool_input lookups with BC-2.02.012 typed projection (`payload.agent_type.as_deref()` etc.)
- LOW-CONS-002: corrected doc comment inversion in track-agent-stop
- Integration tests: `advisory_block_fires_with_on_error_continue` + `advisory_block_absent_with_on_error_continue_and_no_block_stdout`
- Test: `test_pr_manager_works_with_null_tool_input` proves production SubagentStop scenario

### Re-run #2 — CONVERGED (adversary) + FINDINGS-doc-only (consistency)

Adversary: CONVERGED — all code defects closed. Consistency: FINDINGS on doc-only items only:
- NEW-CONS-001: HOST_ABI.md:173,177 — handoff-validator + validate-pr-review-posted bullets omit stdout block emission they now perform
- NEW-CONS-002: HOST_ABI.md:163-165 — `on_error="block"` reserved-behavior sentence misleading; should reflect canonical `on_error="continue"` + stdout pattern
- Dead test fixture data (tool_input populated in SubagentStop fixtures — misleading given null production behavior)

### Fix-burst #3 (PR #61) — 1 commit — develop c4dc842 → d49f33b

- HOST_ABI.md:173 + :177: updated plugin bullets to document stdout block emission
- HOST_ABI.md:163-165: rewrote reserved-behavior sentence to describe `on_error="continue"` + stdout as canonical advisory pattern
- Removed stale tool_input population from SubagentStop test fixtures

---

## Final State: CONVERGED

All 12 W-15 stories merged to develop. 3 fix-bursts (PR #59/60/61) closed all blocking findings. develop @ d49f33b. v1.0.0-rc.3 release path clear.

---

## Residual Tech Debt (non-blocking, tracked as TD items)

| Item | Description | Source |
|------|-------------|--------|
| TD (integration test) | End-to-end test against `execute_tiers` call path — PR #60 tests cover helper `plugin_requests_block` only | adversary re-run #2 residual LOW |
| TD (plugin version drift) | HIGH-W15-001 from original gate: 1.0.0-rc.1 vs 0.0.1 across native plugin Cargo.toml files | original gate HIGH |
| TD (SEC-002) | split-brain in write_file resolution (production uses invoke.rs; standalone tests use write_file.rs) | original security gate |
| TD (SEC-004) | unbounded HookPayload deserialization (1 MiB cap recommended) | original security gate |
| TD (SEC-005) | binary_allow bare names in hooks-registry.toml | original security gate |
| TD (SEC-006 / TD-014) | Tier 2/3 legacy-bash-adapter retirement (calendar-gated to v1.0 GA close) | original security gate + D-203 |
