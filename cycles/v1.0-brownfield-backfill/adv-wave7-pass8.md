# Wave-7 Pass-8 / S-21.19-R7 — Fresh-Context Adversarial Review Record

**Cycle:** v1.0-brownfield-backfill
**Date:** 2026-08-24
**Spec state reviewed:** BC-1.03.017 v1.26, BC-1.03.018 v1.6, ADR-044 v1.3, ADR-039 v1.16
**Parent commit:** (D-1079 factory-artifacts HEAD — see `git -C .factory log -1`)

---

## Verdicts

| Story | Pass | Verdict | Streak | Severity |
|-------|------|---------|--------|----------|
| S-21.19 | R7 | NOT-CLEAN | 0/3 | MED |
| S-21.20 | pass-8 re-confirm | NOT-CLEAN | 3/3→0/3 (RESET — BC-table-cell miss) | MED |
| S-21.21 | pass-8 | NOT-CLEAN | 0/3 | MED |
| S-21.22 | pass-8 | NOT-CLEAN | 0/3 | MED |
| S-21.23 | pass-8 | NOT-CLEAN | 0/3 | MED+LOW |

---

## Findings

### S-21.19 (NOT-CLEAN — streak 0/3)

**F-S2119-R7-001 MED** — Story BC traceability table retains monolithic-S-21.11 framing in the PC ownership column: the traceability row still attributes all PCs to S-21.11 v2.11 as the sole delivery vehicle, which was superseded by the six-story split topology at D-1057. The split-topology framing is now authoritative per BC-1.03.017 v1.26 §Architecture Anchors split-ownership section; the story traceability row is the downstream citation and must reflect the split-owned PC assignment. Route: story-writer (update BC traceability ownership framing to reference the split-seam PC assignments per BC-1.03.017 v1.26 split-topology SoT). **FIXED** (story-writer + product-owner: BC-1.03.017 v1.26→v1.27 split-topology re-anchor of ALL live monolithic-S-21.11 sites; S-21.19 v1.10→v1.11).

Streak **REMAINS 0/3**.

---

### S-21.20 (NOT-CLEAN — streak 3/3→0/3 RESET)

**F-S2120-R7-001 MED** — Story body BC table version cell shows v1.25 (the cell was NOT updated to v1.26 at D-1079 despite the D-1079 Changelog entry claiming "POLICY 8 full propagation"). This is the straggler class first identified at F-S2120-P8-001: the pipe-delimited BC-table Version column (`| BC-1.03.017 | v1.25 |`) was not reached by the D-1079 sweep, which checked frontmatter `behavioral_contracts`, H1 cite, and body narrative cites, but did not run a table-cell-aware grep (`grep -nE '\| *BC-1\.03\.017 *\| *v1\.[0-9]+'`) to isolate the Version column cell. Route: story-writer (update BC table version cell to v1.26; run table-cell-aware parity gate to confirm). **FIXED** (story-writer + product-owner: BC-1.03.017 v1.26→v1.27 split-topology re-anchor; S-21.20 BC table cell v1.25→v1.27 confirmed via table-cell-aware grep; story v1.8→v1.9).

Streak **RESET 3/3→0/3** (BC-table-cell miss constitutes a POLICY 8 finding at MED severity; streak resets per BC-5.39.001).

---

### S-21.21 (NOT-CLEAN — streak 0/3)

**F-S2121-P8-001 MED** — Story body BC table version cell shows v1.25 (same straggler class as F-S2120-R7-001). The D-1079 re-anchor for S-21.21 swept frontmatter `behavioral_contracts` + H1 + body narrative cites but left the pipe-delimited BC-table Version column cell at v1.25. Route: story-writer (update BC table version cell to current version; run table-cell-aware parity gate). **FIXED** (story-writer + product-owner: BC-1.03.017 v1.26→v1.27 re-anchor; S-21.21 BC table cell v1.25→v1.27; story v1.9→v1.10).

**F-S2121-P8-002 MED** — Story body retains multiple ADR version pins in narrative prose that survived the D-1079 ADR-pin-strip pass. The POLICY 19 extended scope (story bodies) was operative at D-1079, but two sites were missed: ADR-044 v1.3 context anchor in Task 5a's live-wiring narrative and `§Decision 4 v1.16` in the calibration-sufficiency rationale. Route: story-writer (strip remaining version tokens; verify via tr-normalized multiline sweep). **FIXED** (story-writer: S-21.21 v1.9→v1.10 ADR-pin residual sweep).

Streak **REMAINS 0/3**.

---

### S-21.22 (NOT-CLEAN — streak 0/3)

**F-S2122-P8-001 MED** — Story body BC table version cell shows v1.25 (same straggler class). The D-1079 re-anchor for S-21.22 did not update the BC-table Version column cell. Route: story-writer (update BC table version cell; run table-cell-aware parity gate). **FIXED** (story-writer + product-owner: BC-1.03.017 v1.26→v1.27 re-anchor; S-21.22 BC table cell v1.25→v1.27; story v1.9→v1.10).

**F-S2122-P8-002 MED** — Story body retains an `ADR-044 §Decision 5 v1.3` pin in the calibration-protocol narrative. POLICY 19 extended scope applies. Route: story-writer (strip version token; stable form `ADR-044 §Decision 5`). **FIXED** (story-writer: S-21.22 v1.9→v1.10 ADR-pin residual sweep).

Streak **REMAINS 0/3**.

---

### S-21.23 (NOT-CLEAN — streak 0/3)

**F-S2123-P8-001 MED** — Story DAG diagram block retains `ADR-044 v1.3 function-split reconvergence` as a Position-in-DAG annotation. POLICY 19 extended scope (story bodies) applies; the version pin is load-bearing as a gating identifier and must be stripped to stable form `ADR-044 function-split reconvergence`. Route: story-writer (strip version pin from DAG annotation). **FIXED** (story-writer: S-21.23 v1.7→v1.8 DAG version-pin strip).

**F-S2123-P8-002 LOW** — Story §Bidirectional Parity Audit Note cross-references decomposition-plan §3 intro paragraph using a stale provenance descriptor that refers to plan state as of D-1069 (before the §3/§4 AC-042–045 architect correction at D-1080). Non-load-bearing for AC delivery; anchor: next S-21.23 touch. Route: story-writer (update provenance descriptor to reference plan §3/§4 corrected at D-1080). **FIXED** (story-writer + architect: decomposition-plan §3/§4 AC-042–045 provenance corrected at D-1080; S-21.23 v1.7→v1.8).

Streak **REMAINS 0/3**.

---

## Note

All five pass-8/R7 findings are BC-table-Version-cell straggler (POLICY 8) or ADR-pin residual (POLICY 19) class — no new failure mode. The BC-table-cell straggler class (F-S2120-R7-001, F-S2121-P8-001, F-S2122-P8-001) is the root cause addressed by D-1080 POLICY 8 codification: re-anchor bursts claiming "full propagation" MUST run and capture a table-cell-aware grep (`grep -nE '\| *BC-ID *\| *vX'`) to isolate the pipe-delimited BC-table Version column BEFORE attesting propagation. An unbacked "full propagation" claim is itself a POLICY 8 finding per D-1080.

Root cause attacked at D-1080: POLICY 8 TABLE-CELL-AWARE PARITY GATE verification step added (closes BC-table-Version-cell straggler class F-S2120/21/22-P8-001). S-21.20's 3/3 CONVERGED PROVISIONAL status RESETS to 0/3 — the D-1079 "full propagation" claim was not backed by a table-cell-aware gate.

---

## Remediation Burst Routing (pass-8/R7)

1. **architect (①, plan):** decomposition-plan §3/§4 AC-042–045 provenance corrected (F-S2123-P8-002 LOW context; split-topology SoT).
2. **product-owner (①, BC-1.03.017 v1.27):** BC-1.03.017 v1.26→v1.27 — split-topology re-anchor of ALL live monolithic-S-21.11 sites (F-S2119-R7-001 MED root-cause fix); all five BC-table-cell straggler sites corrected (F-S2120-R7-001/F-S2121-P8-001/F-S2122-P8-001 MED×3 structural fix at the SoT layer).
3. **story-writer (②) AFTER PO:** S-21.19 v1.10→v1.11 (BC-1.03.017 v1.27 re-anchor; monolithic framing corrected); S-21.20 v1.8→v1.9 (BC-1.03.017 v1.27 re-anchor; BC table cell v1.25→v1.27; streak RESET 3/3→0/3); S-21.21 v1.9→v1.10 (BC table cell v1.25→v1.27 + ADR-pin residual sweep; streak 0/3); S-21.22 v1.9→v1.10 (BC table cell v1.25→v1.27 + ADR-pin residual; streak 0/3); S-21.23 v1.7→v1.8 (DAG ADR-pin strip + provenance; streak 0/3); S-21.24 v1.10→v1.11 (BC-1.03.017 v1.27 re-anchor; STRICTLY LAST).
4. **state-manager (③):** adv-wave7-pass8.md persist + POLICY 8 TABLE-CELL-AWARE PARITY GATE codification (D-1080) + BC-INDEX v4.97 + STORY-INDEX v4.391 + STATE.md advance → pass-9/R8 dispatch.
