---
document_type: adversary-pass-report
level: ops
title: "S-15.09 LOCAL Adversary Cascade — Pass 5"
producer: adversary
timestamp: 2026-05-17T00:00:00Z
phase: per-story-step-4-local-adversary-cascade
story: S-15.09
pass: 5
verdict: LOW
finding_count: { critical: 0, high: 0, medium: 0, low: 4, nitpick: 1, process_gap: 0 }
streak_3_clean: "0/3"
---

# S-15.09 LOCAL Adversary Cascade — Pass 5

## Part A — Findings

### F-P5-001 [LOW] Story Precondition 4 contradicts BC Precondition 4 on host::read_file timeout (5000 vs 2000)

- **Location:** Story `.factory/stories/S-15.09-validate-state-structure-phase-1.md:178`; BC `.factory/specs/behavioral-contracts/ss-05/BC-5.39.005.md:73`; impl `crates/hook-plugins/validate-state-structure/src/lib.rs:767`.
- **Evidence:** Story says "5000 ms timeout"; BC + implementation say `timeout_ms = 2000`.
- **Issue:** Spec drift. Story conflates registry-level hook timeout (5000) with per-call host::read_file timeout (2000).
- **Recommendation:** Update story Precondition 4 to distinguish the two timeouts.

### F-P5-002 [LOW — ORCHESTRATOR-ELEVATED TO CRITICAL] max_bytes=65536 below real STATE.md size (95,185 bytes); validator was silently INERT

- **Location:** `lib.rs:767` `host::read_file(&file_path, 65536, 2000)`.
- **Evidence:** orchestrator verified real STATE.md = 95,185 bytes (145% of 64 KiB cap); implementer confirmed `host::read_file` returns `Err(HostError::OutputTooLarge)` on oversize → hook fail-opens (Continue) silently → validator was inert against real production target.
- **Issue:** META-LEVEL-24 silent-inert-validator false-green class. SOUL.md #4 violation. The hook shipped (in pass-4 bats green) but never blocked a real STATE.md edit.
- **Recommendation:** Raise max_bytes to ≥256 KiB with growth runway; add load-bearing length-budget sentinel + regression test that exercises oversize content.

### F-P5-003 [LOW] extract_banner_line_count whole-document scope inconsistent with banner-block-scoped validate_dual_margin

- **Location:** `lib.rs:112-161` vs `lib.rs:234-273`.
- **Issue:** Asymmetric scope. Future state-manager fix-burst that adds body prose like "(wc-l)" could silently shift banner-wc to body-derived value.
- **Recommendation:** Constrain extract_banner_line_count to operate within extract_banner_block.

### F-P5-004 [LOW] BC input-hash 5af355e may be stale vs current story df9db17

- **Location:** BC-5.39.005 frontmatter `input-hash`; story frontmatter `input-hash`.
- **Issue:** POLICY 18 ambiguity — input-hash convention is "frozen at authoring" or "recompute on changes"?
- **Recommendation:** Recompute via `bin/compute-input-hash` OR codify project convention.

### F-P5-005 [NITPICK] Visibility asymmetry — extract_banner_block + has_adjacent_arrow_digit_run private; sibling extractors pub

- **Location:** `lib.rs:299` + `lib.rs:523`.
- **Issue:** Cosmetic asymmetry.
- **Recommendation:** Promote to `pub` for consistency with sibling helpers.

## Part B — Production-Grade Default Audit

- F-P5-002 is the standout: a CRITICAL-class silent-inert validator that passed pass-4 only because the bats test fail-opens silently when `host::read_file` returns Err. Production-grade lens demands in-scope fix.
- F-P5-001 + F-P5-003 are spec/scope drift — mechanical fixes in scope.
- F-P5-004 is POLICY 18 mechanical verification.
- F-P5-005 is cosmetic in-scope cleanup.
- No MVP rationalizations, no paper-fix smells, no "TODO for architect" placeholders observed.

## Part C — Self-Application Audit (META-LEVEL)

- Iron Law respected: no prior pass reports read.
- Fresh-context value: pass-5 examined NEW axes (file size budget, scope asymmetry, spec drift across artifacts, input-hash convention) that pass-1 through pass-4 did not focus on.
- Orchestrator independently verified F-P5-002 with `wc -c .factory/STATE.md` = 95,185 bytes. Severity elevation from LOW to CRITICAL based on production-impact analysis: hook was silently inert against the very target it validates.
- POLICY 13 + POLICY 15 compliance: all findings cite verbatim file:line evidence.
- NOVELTY: HIGH. Pass-5 surfaced a silent-failure class invisible to passes 1-4 due to the synthetic-test-suite-passes pattern.

## Verdict & Streak

- Pass-5 verdict: **LOW** (4L + 1N).
- Streak: 1/3 (post-pass-4) → **0/3** (≥LOW resets).
- Mandatory fix-burst required.

## Fix-burst routing (orchestrator-routed; complete)

- implementer @ `44b308a8` — F-P5-002 (CRITICAL-class) + F-P5-003 + F-P5-005 closed. WASM 173 KB. 48 unit + 24 bats green. host::read_file oversize behavior documented (Err → fail-open silent). "Test the test" verification done.
- story-writer @ `4c93c920` factory-artifacts — F-P5-001 closed (story v1.3→v1.4; 4 occurrences updated; `<IMPL>` placeholder pending state-manager reconciliation).
- state-manager — this persistence commit + `<IMPL>` placeholder reconciliation + F-P5-004 decision.

## SIBLING-CRATE SPILLOVER (surfaced by implementer, NOT in-scope for S-15.09 fix-burst)

Implementer's investigation revealed that `validate-burst-log` and `validate-index-cite-refresh` (shipped M2 wave-1+2; merged to develop) likely have the SAME silent-inert defect:

- `validate-burst-log` reads `.factory/cycles/v.../burst-log.md` (608,723 bytes = 9.4x the 65536 cap) — likely SILENTLY INERT
- `validate-index-cite-refresh` reads multiple files including `lessons.md` (119,730 bytes = 1.8x the cap) — likely SILENTLY INERT

This is a cross-story integration finding. The fixes belong in follow-up stories (the silent-inert defects are already on develop). Orchestrator should surface to human for prioritization — both stories were claimed-converged but their pass-N adversary cascades did NOT catch this class because they didn't independently verify against the real production-target file sizes. This is precisely the META-LEVEL-24 false-green pattern recurring at the cross-story scope.

## Closure verification

F-P5-002 closure tested via deliberate STATE.md mutation: hook now produces `exit_code=2 block_reason="banner claims 1427 lines but actual line count is 428"`. Pre-fix, the same mutation would have produced `exit_code=0` (fail-open silent inert). The validator is now LOAD-BEARING for the first time in the cascade.
