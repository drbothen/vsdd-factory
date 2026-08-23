# Wave-7 Pass-4 / S-21.19-R3 — Fresh-Context Adversarial Review Record

**Cycle:** v1.0-brownfield-backfill
**Date:** 2026-08-23
**Spec state reviewed:** BC-1.03.017 v1.22, BC-1.03.018 v1.4, ADR-044 v1.3, ADR-039 v1.16
**Parent commit:** c47c913f (D-1074)

---

## Verdicts

| Story | Pass | Verdict | Streak | Severity |
|-------|------|---------|--------|----------|
| S-21.19 | R3 | NOT-CLEAN | 1/3→0/3 reset | HIGH |
| S-21.20 | pass-4 | NOT-CLEAN | 1/3→0/3 reset | MED |
| S-21.21 | pass-4 | NOT-CLEAN | 0/3 | HIGH+MED |
| S-21.22 | pass-4 | CLEAN | 0/3→1/3 | — |
| S-21.23 | pass-4 | NOT-CLEAN | 0/3 | 3 MED |

---

## Findings

### S-21.19 (NOT-CLEAN — streak 1/3→0/3 reset)

**F-S2119-R3-001 HIGH** — Story frames 2-arg `plugin_fail_closed` as "retired"/superseded; contradicts ADR-044 v1.3 ADDITIVE-then-migrate (2-arg RETAINED through wave 7, removed atomically at S-21.24); reopens CWE-636 Timeout+on_error=Block fail-open window. Route: story-writer.

**F-S2119-R3-002 HIGH** — BC-1.03.017 v1.21→v1.22 re-anchor missing (57 sites, POLICY 8/17). Route: story-writer.

**F-S2119-R3-003 HIGH** — ADR-044 v1.1/v1.2→v1.3 model+cite. Route: story-writer.

**F-S2119-R3-004 MED** — Stale input-hash. Route: state-manager.

**F-S2119-R3-005 LOW (non-resetting)** — Invariant 12 coverage-continuity narrative missing in ACs. Route: story-writer.

---

### S-21.20 (NOT-CLEAN — streak 1/3→0/3 reset)

**F-S2120-P4-001 MED** — BC-table Title cell inserted "and" breaks POLICY-7 verbatim-subset with v1.22 H1. Route: story-writer.

**F-S2120-P4-002 LOW (non-resetting)** — ADR-044 v1.1 stale pin. Route: story-writer.

_(F-S2120-P3-001 STORY-INDEX drift CONFIRMED fixed.)_

---

### S-21.21 (NOT-CLEAN — streak 0/3)

**F-S2121-P4-001 HIGH** — BC-1.03.017 v1.22 EC-011 "pre-wiring-fix" clause asserts no on_error=Block enforcement, contradicting its own Invariant 12 + story's corrected EC-011 (TD-VSDD-060 sibling-sweep miss). Route: product-owner.

**F-S2121-P4-002 MED** — S-21.21 AC-007/Task 10 frame PC6 regression against LIVE corpus; v1.22 PC6 split into (i) one-time-live-confirm vs (ii) frozen-snapshot standing gate; content half not propagated. Route: story-writer (+PO confirm S-21.21-vs-S-21.22 standing-gate ownership).

**F-S2121-P4-003 LOW (non-resetting)** — PSI cite-scope looseness. Route: story-writer.

_(Story-side additive wiring, regression fixture, v1.22/v1.3 cites all CONFIRMED correct.)_

---

### S-21.22 (CLEAN — streak 0/3→1/3)

**F-S2122-P4-001 LOW (non-resetting)** — STOP-gate uses ceil(observed_max×1.5) vs BC un-ceil'd; strictly more conservative. Route: product-owner decision or story-writer align.

**F-S2122-P4-002 LOW (non-resetting)** — Mixed ADR-044 v1.1/v1.3 cite (POLICY 19). Route: story-writer.

---

### S-21.23 (NOT-CLEAN — streak 0/3)

**F-S2123-P4-001 MED** — BC-1.03.018 PC9 vs story AC-022 control-letter drift (BC (g)=COMMENT-ONLY vs story (g)=LIVE-TREE); reader defers wrong control. Route: product-owner (align BC letters to story's LIVE-TREE=(g) partition).

**F-S2123-P4-002 MED** — PC9 detector false-green: commented-out full call passes substring detector; mandate comment-stripping + add full-call-in-comment MUST-NOT-satisfy fixture (CWE-636). Route: product-owner + story-writer.

**F-S2123-P4-003 MED** — Stale ADR-044 v1.2 cite in Position-in-DAG. Route: story-writer.

**F-S2123-P4-004 LOW (non-resetting)** — AC-045 all+non-named-alone sub-case not separately exercised. Route: story-writer (optional).

**F-S2123-P4-005 LOW (non-resetting)** — `all`-as-trimmed-comma-token drift vs BC PC3. Route: product-owner confirm.

_(AC-045 all×non-named negative control CONFIRMED correct.)_

---

## Orchestration Note

S-21.19 was omitted from the D-1074 Step-1 re-anchor sweep (3a=S-21.21/22/24, 3b=S-21.23; state-manager mechanically re-anchored only S-21.20). Fresh-context R3 surfaced the omission — process worked as designed.

---

## Next Remediation Burst (routing order enforced)

1. **product-owner:** BC-1.03.017 v1.22→v1.23 (EC-011 sweep to match Invariant 12 + ceil() decision) + BC-1.03.018 v1.4→v1.5 (PC9 letter-align (g)=LIVE-TREE + detector comment-stripping + all-token note).
2. **story-writer AFTER PO:** S-21.19 additive-model rewrite + full re-anchor; S-21.20 title-cell; S-21.21 PC6 (i)/(ii); S-21.23 fixtures+ADR cite; all-stories re-anchor to v1.23/v1.5; fold S-21.22 LOWs.
3. **state-manager commit;** then dispatch pass-5/R4 (5 fresh cascades).
