# Wave-7 Pass-5 / S-21.19-R4 — Fresh-Context Adversarial Review Record

**Cycle:** v1.0-brownfield-backfill
**Date:** 2026-08-23
**Spec state reviewed:** BC-1.03.017 v1.23, BC-1.03.018 v1.5, ADR-044 v1.3, ADR-039 v1.16
**Parent commit:** 196ba85e (D-1075)

---

## Verdicts

| Story | Pass | Verdict | Streak | Severity |
|-------|------|---------|--------|----------|
| S-21.19 | R4 | NOT-CLEAN | 0/3 | HIGH+LOW |
| S-21.20 | pass-5 | CLEAN | 0/3→1/3 | — |
| S-21.21 | pass-5 | NOT-CLEAN | 0/3 | HIGH+MED+LOW |
| S-21.22 | pass-5 | NOT-CLEAN | 1/3→0/3 reset | MED |
| S-21.23 | pass-5 | NOT-CLEAN | 0/3 | HIGH |

---

## Findings

### S-21.19 (NOT-CLEAN — streak 0/3)

**F-S2119-R4-001 HIGH** — Decomposition-plan §8.7 framed the additive-then-migrate step as a literal-replacement step for S-21.21, contradicting ADR-044 v1.3 ADDITIVE-then-migrate invariant (retained 2-arg call must stand through wave 7; literal-replacement interpretation opens the Invariant 12 fail-open window). Route: architect (§8.7 rewrite + new §8.8 calibration-ownership matrix). FIXED.

**F-S2119-R4-002 LOW (non-resetting)** — ADR-044 v1.3 cite pairing in story Task 5a lacks explicit ADDITIVE-not-replacement context anchor for the OR-combined call site. Route: story-writer. FIXED.

---

### S-21.20 (CLEAN — streak 0/3→1/3)

2 LOW (non-resetting) findings folded into story-writer pass-5 sweep:
- Title-cell ceil() abbreviation note (brevity-only context, non-resetting).
- ADR-044 v1.2 historical-descriptor phrase (historical record preserved, non-resetting).

Streak **ADVANCES 0/3→1/3**.

---

### S-21.21 (NOT-CLEAN — streak 0/3)

**F-S2121-P5-001 HIGH** — BC-1.03.017 v1.23 PC6 asserts BOTH calibration checks (Task 4 one-time confirmation AND Task 5a durable CI gate) are OWNED BY S-21.22 for ALL SIX §Decision 2 validators. Architecturally impossible under the decomposition plan: S-21.21 covers the five legacy-bash-adapter.wasm-hosted plugins; routing those five-plugin sufficiency checks to S-21.22 creates a false sequential DAG dependency and inflates S-21.22's scope beyond its native-WASM-only mandate. Route: architect (new §8.8) + product-owner (BC-1.03.017 v1.23→v1.24 PC6 split-ownership correction) + story-writer. FIXED.

**F-S2121-P5-002 MED** — Task 11 bash-adapter calibration predicate uses un-ceil'd `observed_max×1.5` formula while BC-1.03.017 v1.23 adopted `ceil(observed_max×1.5)` throughout all live predicate occurrences (TD-VSDD-060 sibling-sweep miss from pass-4 ceil() rollout). Route: story-writer. FIXED.

**F-S2121-P5-003 LOW (non-resetting)** — Title-cell ceil() abbreviation alignment (consistency note). Route: story-writer (folded). FIXED.

---

### S-21.22 (NOT-CLEAN — streak 1/3→0/3 reset)

**F-S2122-P5-001 MED** — Token Budget section cites ADR-044 v1.1; current ratified version is v1.3 (POLICY 19 stale-cite). Route: story-writer. FIXED.

**F-S2122-P5-002 MED** — Title-cell description does not reflect the ceil() formula suffix adopted by F-S2122-P4-001; Task 5a body already uses ceil() but the title-cell abbreviation lags (TD-VSDD-060 sibling-sweep miss). Route: story-writer. FIXED.

Streak: **RESETS 1/3→0/3**.

---

### S-21.23 (NOT-CLEAN — streak 0/3)

**F-S2123-P5-001 HIGH** — AC-022 control-letter schema misaligned to BC-1.03.018 v1.5 PC9 seven-partition: story has (e)=COMMENT-ONLY, (f)=LIVE-TREE; BC-1.03.018 v1.5 defines (f)=COMMENT-ONLY, (g)=LIVE-TREE (v1.5 expanded the partition with a new (g) LIVE-TREE entry; story was only updated through (f), missing the new (g) position). CWE-636: LIVE-TREE control at wrong position would fail the live-tree false-green fixture. Route: story-writer. FIXED.

---

## Note

All five NOT-CLEAN findings were partial-fix/sibling-sweep gaps left by the pass-4 remediation (TD-VSDD-059/060 recurrence-class; no NEW codification needed — already-codified pattern).

---

## Next Remediation Burst (routing order enforced)

1. **architect:** decomposition-plan §8.7 rewrite (additive-not-replacement framing explicit) + new §8.8 calibration-ownership matrix (five-plugin bash-adapter checks OWNED BY S-21.21; native-WASM `validate-cross-site-correspondence` checks OWNED BY S-21.22).
2. **product-owner AFTER architect:** BC-1.03.017 v1.23→v1.24 (PC6 split-ownership correction per §8.8; H1 enriched per POLICY 7).
3. **story-writer AFTER PO:** S-21.19 ADR-cite pairing anchor + S-21.21 ceil() Task 11 sweep + S-21.22 ADR-044 v1.3 token-budget cite + title-cell ceil() + S-21.23 (e)/(f)/(g) reposition to BC-1.03.018 v1.5 schema; all BC-1.03.017 stories re-anchor to v1.24; fold LOWs.
4. **state-manager commit;** then dispatch pass-6/R5 (5 fresh cascades against v1.24/v1.5).
