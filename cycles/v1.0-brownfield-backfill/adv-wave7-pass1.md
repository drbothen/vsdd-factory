---
document_type: adversarial-review-record
level: cycle
cycle: v1.0-brownfield-backfill
wave: 7
pass: 1
scope: [S-21.20, S-21.21, S-21.22, S-21.23]
producer: state-manager
created: 2026-08-22
last_amended: "2026-08-22 (D-1070) — story-layer remediation disposition appended"
---

# Wave-7 Pre-TDD Adversary Pass-1 — Compact Record

> Compact record per D-1069/D-1070 task scope: finding ID · severity · one-line ·
> disposition. NOT a verbatim reproduction of the fresh-context adversary
> transcripts. Full narrative detail: STATE.md Decisions Log D-1069 row +
> D-1070 row (this cycle file's sibling STORY-INDEX/story-file annotations).

Four Wave-7-ready stories (S-21.20/S-21.21/S-21.22/S-21.23, split seams 2/3/4/5
of 6 from the superseded S-21.11) were dispatched fresh-context, information-
asymmetric, against BC-1.03.017 v1.19 / BC-1.03.018 v1.1 (pre-remediation spec
state) at D-1068 READY. Verdicts below.

## S-21.20 — PC13 full-registry coverage — **CLEAN**

LOCAL BC-5.39.001 streak: 0/3 → **1/3**.

| Finding | Severity | One-line | Disposition |
|---------|----------|----------|--------------|
| F-S2120-P1-001 | MEDIUM | (folded to Phase-3 test-design notes) | Folded into story's Phase-3 test-design notes (D-1070); non-resetting |
| F-S2120-P1-002 | MEDIUM | (folded to Phase-3 test-design notes) | Folded into story's Phase-3 test-design notes (D-1070); non-resetting |
| F-S2120-P1-003 | LOW | cosmetic | Deferred, non-resetting |
| F-S2120-P1-004 | LOW | cosmetic | Deferred, non-resetting |
| F-S2120-P1-005 | LOW | cosmetic | Deferred, non-resetting |

No streak-resetting findings. Story re-anchored to BC-1.03.017 v1.20 at D-1070
(non-resetting maintenance re-anchor); streak carries forward unaffected.
Pass-2 (fresh context) dispatched against the v1.20 spec state — next action.

## S-21.21 — AMD-002 bash-adapter wiring calibration — **NOT-CLEAN**

LOCAL BC-5.39.001 streak: 0/3 (remediation does not advance streak).

| Finding | Severity | One-line | Disposition |
|---------|----------|----------|--------------|
| F-S2121-P1-001 | HIGH | Calibration-statistic contradiction (`fuel_cap` target cited `measured_p99×1.5` vs body's own worked example) | Fixed — ADR-039 §Decision 4 target corrected `measured_p99×1.5`→`observed_max×1.5` (D-1069 Q2, architect); story Task 11 swept (D-1070) |
| F-S2121-P1-002 | HIGH | PC13 live-wiring ordering — ADR-044 v1.0 assigned the error-exit axis to S-21.24 (STRICTLY LAST capstone), but S-21.21's own AMD-002 wiring task needs it live at wave 7, before S-21.24 exists | Fixed — architect adjudication Q1=Option A (human-ratified): reopen S-21.19, new Task 6 splits off standalone `plugin_fail_closed_on_error_exit`; ADR-044 v1.1 Addendum (D-1069); S-21.21 new Task 5a live-wires the fn (D-1070) |
| F-S2121-P1-003 | MEDIUM | Timing tolerance was two-sided, permitting an implausibly-fast false-pass | Fixed — one-sided timing tolerance adopted in S-21.21's calibration harness (D-1070) |

Routing: architect (ADR-039/ADR-044 amendment, Q1/Q2 adjudication) + story-writer
(Task 5a/Task 11 story-layer application). Spec-layer landed D-1069; story-layer
landed D-1070. Streak remains 0/3 — pass-2 (fresh context, against v1.20/v1.1
spec + S-21.19 v1.4 Task-6 split) is next action.

## S-21.22 — Native-WASM calibration and flip — **NOT-CLEAN**

LOCAL BC-5.39.001 streak: 0/3 (remediation does not advance streak).

| Finding | Severity | One-line | Disposition |
|---------|----------|----------|--------------|
| F-S2122-P1-001 | HIGH | CWE-636 "inert until S-21.24" overclaim — story asserted the flip was fully inert pre-capstone without qualifying which axis | Fixed — inert-until-S-21.24 caveat added, scoped explicitly to the exhaustion axis only (S-21.21/S-21.19 own the error-exit axis, live at wave 7) (D-1070) |
| F-S2122-P1-002 | HIGH | Calibration-sufficiency gate missing — no machine-checkable precondition that the six ADR-039 §Decision 2 fail-closed validators were actually calibrated before the flip | Fixed — new BC-1.03.017 PC6 sufficiency gate authored (D-1069, product-owner); PC6 + calibration harness AC wired into S-21.22 (D-1070) |
| F-S2122-P1-003 | MEDIUM | `PRACTICAL_FUEL_CEILING` mis-cited (stale/absent upper-bound value) | Fixed — `PRACTICAL_FUEL_CEILING = 500_000_000` cited (ADR-039 §Erratum E-007, D-1069; story-layer cite D-1070) |
| F-S2122-P1-004 | MEDIUM | S-21.13's `depends_on` still pointed at the superseded S-21.11 instead of S-21.22, which now owns `validate-cross-site-correspondence` | Fixed — S-21.13 `depends_on` redirect [S-21.10,S-21.11]→[S-21.10,S-21.22] executed (D-1070, discharges D-1057 carry-forward) |
| F-S2122-P1-005 | MEDIUM | Calibration harness underspecified (no named fixture/procedure) | Fixed — calibration harness specified in S-21.22 body (D-1070) |

Routing: product-owner (BC-1.03.017 PC6), architect (ADR-039 erratum), story-writer
(harness spec + S-21.13 redirect). Spec-layer landed D-1069; story-layer landed
D-1070. Streak remains 0/3 — pass-2 (fresh context, against v1.20 spec) is next
action.

## S-21.23 — Break-glass override mechanism — **NOT-CLEAN**

LOCAL BC-5.39.001 streak: 0/3 (remediation does not advance streak).

| Finding | Severity | One-line | Disposition |
|---------|----------|----------|--------------|
| F-S2123-P1-001 | HIGH | Audit-emission-failure silent-bypass — the break-glass override could be applied even if the mandatory `break_glass.activated` audit write silently failed, defeating the audit trail | Fixed — new ADR-039 §AMD-004 (D-1069, architect, Q3 adjudication): override MUST NOT apply unless the audit write is confirmed durably written via new `InternalLog::write_checked` tri-state method; new BC-1.03.018 PC11 + Invariant 7 (D-1069, product-owner); story AC-042/AC-043 audit fail-closed authored (D-1070) |
| F-S2123-P1-002 | MEDIUM | `NotUnicode` edge case unhandled in the bypass-env-var parse path | Fixed — edge-case cross-map added, EC-007 covers `NotUnicode` (D-1069 BC-1.03.018 v1.2; D-1070 story cross-map) |
| F-S2123-P1-003 | MEDIUM | Env-only-sourcing negative AC missing (no test asserting the gate does NOT activate from a non-env source) | Fixed — negative AC added via the edge-case cross-map (D-1070) |
| F-S2123-P1-004 | MEDIUM | EC-table 6-edge-case enumeration incomplete/miscounted | Fixed — EC-table enumeration corrected to the full 6-edge-case set (D-1070) |
| F-S2123-P1-005 | LOW | cosmetic | Fixed in scope (D-1070) |
| F-S2123-P1-006 | LOW | cosmetic | Fixed in scope (D-1070) |

**Observation O-4 (non-severity-numbered, STORY-INDEX metadata defect, not a
story-content finding):** S-21.23's own STORY-INDEX catalog row cited
`input-hash cbbc8dd`, but `cbbc8dd` was never S-21.23's own genuinely-computed
hash — it was a stale copy of S-21.20's PRE-D-1068 hash value, carried forward
in error at row-creation time (2026-08-20, D-1061 blockquote population).
S-21.20's hash has since moved on (`cbbc8dd`→`86952d4`→`33ca0c4`); S-21.23's
row never independently reflected its own file's real hash. **Fixed at D-1070**
— S-21.23's STORY-INDEX row and the D-1057 blockquote enumeration both
corrected to S-21.23's true, independently-computed hash (`33ca0c4`, which
genuinely coincides with S-21.20's current hash because both stories now
declare an identical two-file `inputs:` list — a real coincidence, not a
copy-paste artifact).

Routing: architect (ADR-039 §AMD-004), product-owner (BC-1.03.018 PC11/Invariant
7/EC-007), story-writer (AC-042/043 + edge-case cross-map), state-manager
(STORY-INDEX hash correction). Spec-layer landed D-1069; story-layer + index
correction landed D-1070. Streak remains 0/3 — pass-2 (fresh context, against
v1.2 spec) is next action.

## Cross-Story Adjudication (D-1069 architect memo, human-ratified)

| Question | Resolution | Consequence |
|----------|-----------|-------------|
| Q1 (F-S2121-P1-002 scope split) | **Option A** — reopen S-21.19 (CONVERGED 3-CLEAN at D-1065) to wire PC13's error-exit axis live at wave 7, owned by S-21.21/S-21.19 jointly, not deferred to S-21.24 | S-21.19 REOPENED at D-1070, new Task 6 (standalone `plugin_fail_closed_on_error_exit`), BC-5.39.001 streak 3/3→0/3, pre-TDD cascade restarts from pass-1; **Wave 6 no longer COMPLETE** |
| Q2 (F-S2122-P1-002 target statistic) | `fuel_cap` target `measured_p99×1.5`→`observed_max×1.5` (erratum); new `PRACTICAL_FUEL_CEILING=500_000_000`; new BC-1.03.017 PC6 sufficiency gate | ADR-039 v1.16, BC-1.03.017 v1.20 (D-1069) |
| Q3 (F-S2123-P1-001 audit fail-closed) | New ADR-039 §AMD-004 — break-glass override gated on confirmed-durable audit write via `InternalLog::write_checked` | ADR-039 v1.16, BC-1.03.018 v1.2 (D-1069) |

## Disposition Summary

- **Spec layer** (ADR-039 v1.16, ADR-044 v1.1, BC-1.03.017 v1.20, BC-1.03.018
  v1.2): landed D-1069 (`.factory` commit `a3bfa1af`).
- **Story layer** (S-21.19/20/21/22/23/24 re-anchor, S-21.19 Task-6 reopen,
  S-21.13/S-21.16 depends_on redirects, STORY-INDEX hash corrections): landed
  D-1070 (this commit).
- **Wave 6 status:** S-21.25 remains CONVERGED-AWAITING-TDD (3-CLEAN,
  UNCHANGED). S-21.19 REOPENED — Wave 6 is **NO LONGER COMPLETE**.
- **Wave 7 status:** S-21.20 streak 1/3 (CLEAN pass-1, re-anchor non-resetting).
  S-21.21/S-21.22/S-21.23 streak 0/3 (NOT-CLEAN pass-1, remediated). All four
  now at BC-1.03.017 v1.20 / BC-1.03.018 v1.2. Pass-2 (fresh context) for all
  four is the next action.
