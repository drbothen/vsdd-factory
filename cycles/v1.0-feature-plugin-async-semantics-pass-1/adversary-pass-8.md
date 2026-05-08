---
document_type: adversary-pass
cycle: v1.0-feature-plugin-async-semantics-pass-1
phase: F2
pass: 8
producer: adversary
producer_model: claude-opus-4-7[1m]
fresh_context: true
verdict: NITPICK_ONLY
finding_count: { high: 0, medium: 0, low: 0, nit: 1 }
adr-013_clock_action: advance
clock: 1_of_3
timestamp: 2026-05-07T00:00:00Z
---

═══════════════════════════════════════
[BEGIN]

# Adversary Pass-8 Findings — F2 spec package

## Verdict
**NITPICK_ONLY** — clock advances 0→1. Trajectory: 19→19→7→6→3→5→4→1 (NIT).

## Counts
HIGH: 0, MEDIUM: 0, LOW: 0, NIT: 1

## Findings

### NIT-P8-001 — ARCH-INDEX cite "per BC-INDEX v1.25" lags actual BC-INDEX v1.26

**File:** `/Users/jmagady/Dev/vsdd-factory/.factory/specs/architecture/ARCH-INDEX.md` line 120
**Evidence:** Line 120 reads `**Total BCs: 1,947 (per BC-INDEX v1.25; …)**` but BC-INDEX line 4 has `version: "1.26"` (bumped by pass-7 fix burst per its changelog at line 15).
**Severity:** NIT (cosmetic — content count `1,947` is correct; only version-string is stale).
**Note:** Recurring drift pattern previously addressed in pass-5 (v1.22→v1.24) and pass-6 (v1.24→v1.25). Process-gap consideration: BC-INDEX-version cite refresh is a habitual omission when burst closes touch BC files but not ARCH-INDEX prose. Recurrence count is at threshold (3); if pass-9 sees it again the codification trigger fires.

## Policy Compliance

All 12 policies satisfied:
- POLICY 1-12: PASS (sampled byte-for-byte where applicable)
- BC-INDEX 1947 sum correct
- VP-INDEX 79 sum correct
- DI-019 lifecycle complete
- POLICY 7 H1↔BC-INDEX byte-for-byte verified for 4 new BCs

## Open Questions

None substantive. Edge-case checklist coverage:
- ASYNC_DRAIN_WINDOW_MS = 0 / negative: DI-019 §Configurability defers to future env-var; current value is "only value with specified behavior" — adequate
- Multiple async plugins racing: BC-1.14.001 Invariant 3 covers
- SIGTERM during drain: implementation/runtime concern outside F2 scope
- schema_version=3: BC-7.06.001 EC-006 covers (hard error; no forward-compat)

## Top Findings

(None substantive — single NIT cited above.)

## Convergence summary

F2 spec package is converged after 7 fix bursts. All 4 user-locked decisions propagate cleanly. Single cosmetic NIT remaining.

[END]
═══════════════════════════════════════
