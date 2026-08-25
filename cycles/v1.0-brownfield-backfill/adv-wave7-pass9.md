# Wave-7 Pass-9 / S-21.19-R8 — Fresh-Context Adversarial Review Record

**Cycle:** v1.0-brownfield-backfill
**Date:** 2026-08-24
**Spec state reviewed:** BC-1.03.017 v1.27, BC-1.03.018 v1.6, ADR-044 v1.3, ADR-039 v1.16
**Parent commit:** (D-1080 factory-artifacts HEAD — see `git -C .factory log -1`)

---

## Verdicts

| Story | Pass | Verdict | Streak | Severity |
|-------|------|---------|--------|----------|
| S-21.22 | pass-9 | **CLEAN** | 0/3→1/3 | — |
| S-21.19 | R8 | NOT-CLEAN | 0/3 | HIGH + MED |
| S-21.20 | pass-9 | NOT-CLEAN | 0/3 | MED + LOW |
| S-21.21 | pass-9 | NOT-CLEAN | 0/3 | HIGH + MED [process-gap] |
| S-21.23 | pass-9 | NOT-CLEAN | 0/3 | HIGH + LOW [process-gap] |

> **REMEDIATION STATUS: NOT REMEDIATED THIS BURST.** Pass-9 findings are recorded in full. The pipeline PIVOTED to external research → ADR-045 (stable-anchor cross-reference architecture proposal) rather than executing another manual remediation sweep. Wave-7 pre-TDD cascade is **HELD** pending human ratification of ADR-045 via POLICY 22 channel. All four finding classes are instances of the same structural defect (version-pin propagation churn / detector blindness) that ADR-045 addresses by construction.

---

## Findings

### S-21.22 (CLEAN — streak 0/3→1/3)

No findings. Story body and BC-table citations are consistent with BC-1.03.017 v1.27 and BC-1.03.018 v1.6 as of D-1080.

Streak **ADVANCES 0/3→1/3**.

---

### S-21.19 (NOT-CLEAN — streak 0/3)

**F-S2119-R8-001 HIGH** — Story body contains a line-wrapped `BC-1.03.017 v1.26` cite in the BC traceability narrative. The BC ID (`BC-1.03.017`) appears on one physical line and the version token (`v1.26`) appears on the next physical line as a continuation of the paragraph. The POLICY 8 / D-1080 grep pattern `grep -nE '\| *BC-1\.03\.017 *\| *v[0-9]'` is table-cell-aware but reads single physical lines; a line-wrapped cite spanning two physical lines is invisible to this detector. The target version must be updated to v1.27 (current per D-1080). Route: story-writer (locate and update the line-wrapped cite; verify with a normalised tr-collapse multiline sweep). **NOT REMEDIATED — deferred to ADR-045 stable-anchor migration.**

**F-S2119-R8-002 MED** — Story body retains `ADR-039 AMD-002` pin as a bare version identifier in the implementation-notes rationale section. POLICY 19 extended scope (story bodies) applies; AMD-002 is a version qualifier and must be stripped to stable form `ADR-039 Amendment 2` or the functional anchor (the relevant decision clause title). Route: story-writer (strip AMD-002 version token). **NOT REMEDIATED — deferred to ADR-045 stable-anchor migration.**

Streak **REMAINS 0/3**.

---

### S-21.20 (NOT-CLEAN — streak 0/3)

**F-S2120-P9-001 MED** — Story body AC-022 implementation narrative uses a mislabelling phrase that over-scopes the AC's delivery boundary: the narrative attributes behavior to "all six wave-7 stories" when AC-022's traceability anchor is scoped to S-21.20 alone per the D-1057 split-seam assignments. The over-scoping framing conflicts with the split-topology SoT in BC-1.03.017 v1.27 §Architecture Anchors. Route: story-writer (narrow AC-022 narrative to S-21.20 split-seam scope). **NOT REMEDIATED — deferred to ADR-045 stable-anchor migration.**

**F-S2120-P9-002 LOW** — Story body uses `[[hook]]` where `[[hooks]]` (plural) is the canonical term per BC-1.03.017 v1.27 §Ubiquitous Language and the hooks-registry.toml header. Single-character editorial discrepancy; not load-bearing for AC delivery. Route: story-writer (correct to `[[hooks]]`). **NOT REMEDIATED — deferred to ADR-045 stable-anchor migration.**

Streak **REMAINS 0/3**.

---

### S-21.21 (NOT-CLEAN — streak 0/3)

**F-S2121-P9-001 HIGH** — Story body contains 6 ADR-039 version pins that survived all prior POLICY 19 sweeps. All 6 cites follow the pattern `ADR-039 §Decision N vM.NN` embedded within multi-level list items. Each cite has an anchor reference (`{#adr039-dN}`) interposed between the ADR ID and the version token on the same logical citation, i.e., `ADR-039 §Decision 3 {#adr039-d3} v1.10`. The D-1079 multiline-normalising sweep (`tr -s ' \n' ' '`) collapses physical-line wraps but does NOT strip inline anchor spans; the regex `ADR-039 §Decision [0-9]+ v[0-9]` therefore fails to match citations with anchor interposition. This is a detector-architecture gap, not a coverage gap. The underlying spec cite should be stripped to stable form `ADR-039 §Decision N`. Route: story-writer (strip all 6 `vM.NN` version tokens from the anchor-interposed cites). **NOT REMEDIATED — deferred to ADR-045 stable-anchor migration.**

**F-S2121-P9-002 MED [process-gap]** — The D-1079 detector regex cannot match anchor-interposed pins of the form `ADR-039 §Decision N {#anchor} vM.NN` because the inline anchor span breaks the regex's contiguity assumption. This is the same detector-architecture gap exploited by F-S2121-P9-001. Classification: `[process-gap]` — the finding is an instance of the gap that ADR-045 §Decision 2 (AST-based suspect-link validator) targets. Route: ADR-045 ratification + validator implementation. **NOT REMEDIATED — anchored ADR-045 / S-15.03 PRIORITY-A.**

Streak **REMAINS 0/3**.

---

### S-21.23 (NOT-CLEAN — streak 0/3)

**F-S2123-P9-001 HIGH** — Story body BC-1.03.018 Invariant 6 citation contains a line-wrapped `ADR-039 §Decision 3 v1.10` pin: `ADR-039 §Decision 3` on one physical line, `v1.10` on the next line as a continuation. Single-line grep (including the POLICY 19 `tr`-normalised multiline sweep that only collapses blank lines, not mid-paragraph line-wraps produced by editor auto-wrap) is blind to this two-line form. The version token must be stripped. Route: story-writer (locate and strip the wrapped version token; verify with a paragraph-level normalise-then-grep sweep). **NOT REMEDIATED — deferred to ADR-045 stable-anchor migration.**

**F-S2123-P9-002 LOW [process-gap]** — The current single-line-grep approach used by POLICY 19 validators cannot detect version pins that span physical lines due to editor auto-wrap. This is a detector coverage gap, not a story defect per se. Classification: `[process-gap]` — the finding is an instance of the line-wrap blindness gap that ADR-045 §Decision 2 targets. Route: ADR-045 ratification + validator implementation. **NOT REMEDIATED — anchored ADR-045 / S-15.03 PRIORITY-A.**

Streak **REMAINS 0/3**.

---

## Note

All pass-9 findings are instances of two root-cause classes:

1. **Version-pin propagation churn (HIGH × 3):** Line-wrapped cites (`S-21.19 R8-001`, `S-21.23 pass-9-001`) and anchor-interposed cites (`S-21.21 pass-9-001`) are structurally invisible to the current grep-based detector suite. Manual remediation of these three HIGH findings would advance the version pin again (requiring a new BC or ADR version), regenerating new cohort-wide cites, and repeating the cycle. This is the structural floor identified at passes 4–9 (D-1075–D-1081). Remediation by additional manual sweeps would not converge; the root cause is the detector architecture, not insufficient sweep effort.

2. **Detector-architecture [process-gap] (MED × 1, LOW × 2):** `F-S2121-P9-002`, `F-S2123-P9-002` identify specific detector blindness modes. These are not story defects; they are gaps in the validation tooling that allow real defects to survive sweeps.

**Disposition:** The pipeline PIVOTED at D-1081 from manual remediation to architectural intervention. ADR-045 (stable-anchor cross-reference architecture) proposes eliminating load-bearing version pins by construction, making the detector-blindness classes moot. Human ratification of ADR-045 is required before the corpus-migration epic can begin. Until ratification, Wave-7 pre-TDD cascade remains **HELD** (not accepted, not converged).

S-21.22's CLEAN verdict (streak 1/3) and S-21.25's 3/3 CONVERGED status are unaffected by the HELD state.

---

## Remediation Burst Routing (pass-9)

**NOT INITIATED.** Findings deferred to ADR-045 stable-anchor migration epic pending human ratification via POLICY 22 channel.

If ratification is rejected and manual remediation is chosen instead:

1. **story-writer (①):** S-21.19 v1.11→v1.12 (line-wrapped v1.26 cite + AMD-002 pin strip); S-21.20 v1.9→v1.10 (AC-022 narrative narrow + [[hook]]→[[hooks]] fix); S-21.21 v1.10→v1.11 (6 anchor-interposed ADR-039 pin strip); S-21.23 v1.8→v1.9 (line-wrapped ADR-039 v1.10 pin strip).
2. **state-manager (②):** adv-wave7-pass9.md status update + STORY-INDEX version bump + STATE.md advance → pass-10/R9 dispatch.
