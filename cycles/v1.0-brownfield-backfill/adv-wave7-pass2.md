---
document_type: adversarial-review-record
level: cycle
cycle: v1.0-brownfield-backfill
wave: 7
pass: 2
scope: [S-21.19, S-21.20, S-21.21, S-21.22, S-21.23]
producer: state-manager
created: 2026-08-22
last_amended: "2026-08-22 (D-1072) — story-layer remediation disposition appended"
---

# Wave-7 Pre-TDD Adversary Pass-2 / S-21.19 R1 — Compact Record

> Compact record per D-1071/D-1072 task scope: finding ID · severity · one-line ·
> disposition. NOT a verbatim reproduction of the fresh-context adversary
> transcripts. Full narrative detail: STATE.md Decisions Log D-1071 row +
> D-1072 row (this cycle file's sibling STORY-INDEX/story-file annotations).

Five stories were dispatched fresh-context, information-asymmetric, against the
D-1070-landed spec state (BC-1.03.017 v1.20 / BC-1.03.018 v1.2): S-21.19's
**first remediation round** (R1, its 3-CLEAN convergence having been reset by
the D-1070 Task-6 reopen) and S-21.20/S-21.21/S-21.22/S-21.23's **pass-2**.
Verdicts below.

## S-21.19 — Executor decision-function core — **NOT-CLEAN (R1)**

LOCAL BC-5.39.001 streak: 0/3 (remediation does not advance streak).

| Finding | Severity | One-line | Disposition |
|---------|----------|----------|--------------|
| F-S2119-R1-001 | HIGH | ADR-044 v1.1 Addendum Timeout-scope contradiction — the error-exit leg's `Crashed \| Ok{exit_code!=0}` scope was ambiguously worded, readable as also covering `Timeout` | Fixed — ADR-044 v1.1→v1.2 (Addendum item 2 corrected, `Timeout` explicitly and unconditionally excluded from the error-exit leg) (D-1071, architect); story's own body independently confirmed clean, no story-body defect (D-1072) |
| F-S2119-R1-002 | LOW | Versioned-cite hygiene residual | Fixed — swept in story body (D-1072) |
| F-S2119-R1-003 | LOW | Unwired-exit-axis phrasing imprecision | Fixed — phrasing clarified in story body (D-1072) |

Routing: architect (ADR-044 Addendum correction) + story-writer (LOW hygiene
fixes). Spec-layer landed D-1071 (`8ef46b8a`); story-layer landed D-1072 (this
commit). The HIGH finding was entirely ADR-side — S-21.19's own body was never
the defect source. Streak remains 0/3 — R2 (fresh context, against v1.5/
BC-1.03.017-v1.21) is next action.

## S-21.20 — PC13 full-registry coverage — **NOT-CLEAN (pass-2)**

LOCAL BC-5.39.001 streak: 1/3 (remediation does not advance streak; also does
not reset it — the finding was cross-story/prose-reconciliation class, not a
BLOCKER/HIGH structural defect against this story's own coverage claim).

| Finding | Severity | One-line | Disposition |
|---------|----------|----------|--------------|
| F-S2120-P2-001 | MEDIUM | Step-(b) prose only partially reconciled with the Wave-7-pass-1 (D-1069/D-1070) remediation state | Fixed — step-(b) prose fully reconciled (D-1072) |
| F-S2120-P2-002 | LOW | Missing cross-reference to S-21.19's REOPENED status | Fixed — "S-21.19 REOPENED" cross-reference label added (D-1072) |

Routing: story-writer. Spec-layer BC-1.03.017 v1.21 (PC12 margin fix) landed
D-1071; story-layer fix landed D-1072. Streak remains 1/3 (non-resetting
maintenance-class remediation + re-anchor) — pass-3 (fresh context, against
v1.3/BC-1.03.017-v1.21) is next action.

## S-21.21 — AMD-002 bash-adapter wiring calibration — **NOT-CLEAN (pass-2)**

LOCAL BC-5.39.001 streak: 0/3 (remediation does not advance streak).

| Finding | Severity | One-line | Disposition |
|---------|----------|----------|--------------|
| F-S2121-P2-001 | HIGH | Function-scope contradiction at Task 5a's live-wiring call sites — did not consistently restrict the error-exit leg to `Crashed \| Ok{exit_code!=0}`, echoing the ADR-044 v1.1 ambiguity F-S2119-R1-001 also caught | Fixed — function-scope narrowed to `Crashed \| Ok{exit_code!=0}` throughout Task 5a (D-1072), consistent with ADR-044 v1.2's Addendum correction (D-1071) |
| F-S2121-P2-002 | MEDIUM | Task numbering error — "Task 6a" cited where the story's own task list uses "Task 5a" | Fixed — renumbered Task 6a→Task 5a (D-1072) |
| F-S2121-P2-003 | MEDIUM | Stale `PC3` shorthand cite where the BC uses `Precondition 3` | Fixed — corrected to `Precondition 3` (D-1072) |
| F-S2121-P2-004 | MEDIUM | PC12 timing-tolerance narrative not reconciled with BC-1.03.017 v1.21's new one-sided-floor + CI-jitter-robust model | Fixed — BC-1.03.017 v1.20→v1.21 (PC12 kill-time margin rewritten to a one-sided floor + CI-jitter-robust hundreds-of-ms observed-window model) (D-1071, product-owner); story's PC12 narrative reconciled to the new model (D-1072) |

Routing: product-owner (BC-1.03.017 PC12) + story-writer (function-scope, task
numbering, PC cite fixes). Spec-layer landed D-1071; story-layer landed D-1072.
Streak remains 0/3 — pass-3 (fresh context, against v1.3/BC-1.03.017-v1.21) is
next action.

## S-21.22 — Native-WASM calibration and flip — **NOT-CLEAN (pass-2)**

LOCAL BC-5.39.001 streak: 0/3 (remediation does not advance streak).

| Finding | Severity | One-line | Disposition |
|---------|----------|----------|--------------|
| F-S2122-P2-001 | HIGH | New BC-1.03.017 PC6 calibration-sufficiency gate (D-1069) had no durable home in this story's own test suite — the D-1070 wiring cited the gate but did not give it a persistent, re-runnable test | Fixed — new Task 5a authors a durable frozen-corpus PC6 sufficiency test (D-1072) |
| F-S2122-P2-002 | MEDIUM | Boundary predicate used strict `>` where the gate's own semantics require `>=` | Fixed — boundary corrected to `>=` (D-1072) |
| F-S2122-P2-003 | MEDIUM | Task 6 comment block left inconsistent after the D-1070 PC6 wiring edit | Fixed — Task 6 comment-block swept for consistency (D-1072) |

Routing: story-writer. Spec-layer BC-1.03.017 v1.21 (PC12 margin fix, unrelated
to this story's own PC6 finding) landed D-1071; story-layer fix landed D-1072.
Streak remains 0/3 — pass-3 (fresh context, against v1.3/BC-1.03.017-v1.21) is
next action.

## S-21.23 — Break-glass override mechanism — **NOT-CLEAN (pass-2)**

LOCAL BC-5.39.001 streak: 0/3 (remediation does not advance streak).

| Finding | Severity | One-line | Disposition |
|---------|----------|----------|--------------|
| F-S2123-P2-001 | HIGH | Cross-map over-claim — the D-1070 edge-case cross-map asserted EC-001/EC-002/EC-003 coverage without the tests actually existing as RED-gate-verifiable | Fixed — EC-001/EC-002/EC-003 converted to real RED-gate tests (D-1072) |
| F-S2123-P2-002 | MEDIUM | PC9's single-trigger (`failure_policy`-only) ordering gate did not account for the `on_error`-only activation path | Fixed — BC-1.03.018 v1.2→v1.3 (PC9 rewritten to a dual-trigger `failure_policy`-OR-`on_error` ordering gate, control set 4→6) (D-1071, product-owner); AC-022 gains explicit dual-trigger assertion (D-1072) |
| F-S2123-P2-003 | MEDIUM | AC-022's env-var serialization/ordering across the two self-locking gates left undocumented | Fixed — env-var serialization documented (D-1072) |
| F-S2123-P2-004 | MEDIUM | Mount-gate operational runbook missing (no operator-facing recovery procedure) | Fixed — mount-gate operational runbook added (D-1072) |
| F-S2123-P2-005 | MEDIUM | Break-glass bypass detector left unpinned to a specific version | Fixed — detector version pinned (D-1072) |
| F-S2123-P2-006 | LOW | Audit-loss observability gap — a failed `break_glass.activated` audit write was suppressed (fail-closed on the override) but not independently surfaced to stderr | Fixed — BC-1.03.018 v1.3 new PC12 audit-loss stderr observability + EC-008 + Invariant 8 (D-1071, product-owner); new AC-044 audit-observability leg (D-1072) |

Routing: product-owner (BC-1.03.018 PC9/PC12) + story-writer (RED-gate tests,
AC-022 dual-trigger, runbook, detector pin, AC-044). Spec-layer landed D-1071;
story-layer landed D-1072. Streak remains 0/3 — pass-3 (fresh context, against
v1.2/BC-1.03.018-v1.3) is next action.

## Disposition Summary

- **Spec layer** (ADR-044 v1.1→v1.2, BC-1.03.017 v1.20→v1.21, BC-1.03.018
  v1.2→v1.3): landed D-1071 (`.factory` commit `8ef46b8a`).
- **Story layer** (S-21.19 R1 + S-21.20/21/22/23 pass-2 remediation, all six
  wave-7-adjacent stories re-anchored, `S-21.11-decomposition-plan.md` §1 v1.21
  re-anchor + Precondition 2–6 fix): landed D-1072 (this commit).
- **Streaks:** all five stories (S-21.19/20/21/22/23) remain 0/3 (remediated
  this round, not clean). S-21.20 uniquely carries a non-resetting 1/3 baseline
  from its own D-1069 clean pass-1 — this round's finding was maintenance-class,
  not streak-resetting.
- **Next action:** dispatch S-21.19 R2 + S-21.20/S-21.21/S-21.22/S-21.23 pass-3,
  each fresh-context, against the v1.21/v1.3 re-anchored spec state.
- S-21.24 (Wave 8, capstone) re-anchored to BC-1.03.017 v1.21 / BC-1.03.018 v1.3
  this round (D-1072); its own pre-TDD cascade has not started — STRICTLY LAST,
  awaits Wave 7 (S-21.20/21/22/23 pass-3 + S-21.19 R2) convergence.
