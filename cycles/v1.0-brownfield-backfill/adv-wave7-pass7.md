# Wave-7 Pass-7 / S-21.19-R6 — Fresh-Context Adversarial Review Record

**Cycle:** v1.0-brownfield-backfill
**Date:** 2026-08-24
**Spec state reviewed:** BC-1.03.017 v1.25, BC-1.03.018 v1.6, ADR-044 v1.3, ADR-039 v1.16
**Parent commit:** (D-1078 factory-artifacts HEAD — see `git -C .factory log -1`)

---

## Verdicts

| Story | Pass | Verdict | Streak | Severity |
|-------|------|---------|--------|----------|
| S-21.19 | R6 | NOT-CLEAN | 0/3 | MED+LOW |
| S-21.20 | pass-7 | CLEAN | 2/3→3/3 | LOW (non-resetting ×2) |
| S-21.21 | pass-7 | NOT-CLEAN | 0/3 | HIGH+MED |
| S-21.22 | pass-7 | NOT-CLEAN | 0/3 | MED+LOW+LOW |
| S-21.23 | pass-7 | NOT-CLEAN | 0/3 | MED+MED |

---

## Findings

### S-21.19 (NOT-CLEAN — streak 0/3)

**F-S2119-R6-001 MED** — Story Task 2 contains a line-wrapped `ADR-044 v1.3` cite that survived the D-1078 pass-6/R5 14-site sweep because single-line grep (`grep 'ADR-044 v1.3'`) missed the cite when it spans two physical lines ("`ADR-044` / `v1.3`" wrapped by the story's soft-margin). The D-1078 sweep log claimed 14 sites corrected, but the 15th was obscured by the line-wrap. This is a recurring class (F-S2119-P3-001 first surfaced this exact mechanism; D-1079 codifies the `tr '\n' ' '`-normalized multiline detector discipline). Route: story-writer (sweep via `tr '\n' ' ' | grep -oE 'ADR-044[[:space:]]+v[0-9.]+'` normalized detector; strip any remaining version pin). **FIXED** (story-writer: S-21.19 v1.9→v1.10; multiline detector confirmed clean).

**F-S2119-R6-002 LOW** — Story body retains an ADR sub-version pin (`ADR-044 §Decision 5 v1.3` → the version suffix is load-bearing as a gating identifier per POLICY 19 extended to story bodies at D-1079). Route: story-writer (strip version token; stable form is `ADR-044 §Decision 5`). **FIXED** (story-writer: S-21.19 v1.9→v1.10 sweep).

Streak **RESET 0/3**.

---

### S-21.20 (CLEAN — streak 2/3→3/3)

2 LOW (non-resetting) observations:

- **F-S2120-P7-001 LOW (editorial):** BC-1.03.017 re-anchor from v1.25→v1.26 is mechanical/PC6-orthogonal for this story. The pass-7 CLEAN verdict is provisional — a pass-8 re-confirmation pass MUST run given S-21.20's anchored BC changed (BC-1.03.017 v1.26 flip-conditional enrichment). Non-resetting observation.

- **F-S2120-P7-002 LOW (editorial):** DAG diagram label for S-21.19 still reads `"(REOPENED — ADR-044 v1.2 function-split reconvergence in flight)"`. Now moot. Non-resetting; carried forward from F-S2120-P6-002. Deferred — anchor: next S-21.20 story touch.

Streak **ADVANCES 2/3→3/3 = 3-CLEAN CONVERGENCE ACHIEVED**. Cascade CLOSED subject to pass-8 re-confirmation (BC-1.03.017 v1.26 anchor change is PC6-orthogonal — high confidence CLEAN; re-confirmation is a process gate, not a technical concern).

---

### S-21.21 (NOT-CLEAN — streak 0/3)

**F-S2121-P7-001 HIGH** — BC-1.03.017 Precondition 6 still describes the S-21.21-specific durable gate (`pc6-bash-adapter-sufficiency-snapshot/`) without the flip-conditional guard introduced in decomposition-plan §8.8 Addendum (per F-S2121-P6-001 story-side fix). The BC body (SoT for this gate) still reads as an unconditional assertion over the five bash-adapter plugins — it does not state that the assertion iterates `failure_policy == fail-closed` plugins only and is vacuously GREEN at S-21.21's own merge. Route: product-owner (BC-1.03.017 Precondition 6 S-21.21-specific bullet: add flip-conditional language matching story Task 10a per §8.8 Addendum). **FIXED** (product-owner: BC-1.03.017 v1.25→v1.26 Precondition 6 flip-conditional language added).

**F-S2121-P7-002 MED** — BC-1.03.017 body contains `fuel_consumed × 1.5` assertion occurrences (at ~L526, ~L562, ~L569, and Canonical Test Vectors ~L1327-1328) that do NOT wrap the expression in `ceil(...)`, inconsistent with the `ceil(observed_max×1.5)` formula stated in H1 and in PC3/PC6 (after v1.22 adoption). A TD-VSDD-060 sibling-sweep of the full BC body found these four sites plus two additional occurrences (VP-TBD row and Traceability ADR row). Route: product-owner (ceil() sweep: wrap all six `fuel_consumed × 1.5` expressions in `ceil(...)` throughout BC body). **FIXED** (product-owner: BC-1.03.017 v1.25→v1.26 ceil() sweep at 4+2 body occurrences).

Streak **REMAINS 0/3**.

---

### S-21.22 (NOT-CLEAN — streak 0/3)

**F-S2122-P7-001 MED** — Story body retains a `fuel_consumed × 1.5` expression in Task 4's regression-assertion text that was not swept at D-1077's full-perimeter ceil() audit (C-W7-001..C-W7-003 covered AC-007 body, Test bullet, Task 4's *header*, and Task 5a — but missed the inline formula in Task 4's narrative). Consistent with BC-1.03.017 v1.26 ceil() sweep. Route: story-writer (Task 4 narrative: wrap `fuel_consumed × 1.5` in `ceil(...)`). **FIXED** (story-writer: S-21.22 v1.8→v1.9 Task 4 narrative ceil() applied).

**F-S2122-P7-002 LOW** — STORY-INDEX catalog row header still shows `[BC-1.03.017 v1.25]` for S-21.22 (expected — state-manager step ③ updates indexes; story-writer scope only). Non-resetting. **FIXED** (state-manager step ③: STORY-INDEX updated to BC-1.03.017 v1.26).

**F-S2122-P7-003 LOW** — Story Task 3 cross-references Task 6 in S-21.21 by a specific task number (`S-21.21 Task 6`) that does not match S-21.21 v1.8's current task numbering (S-21.21 split Tasks 6→Tasks 6/10a; the cross-reference is technically stale). Non-load-bearing (the cross-reference is informational; S-21.22's own scope is not gated on the specific task number). **DEFERRED** — anchor: wave-gate pre-merge consistency check per F-S2122-P7-003.

Streak **REMAINS 0/3**.

---

### S-21.23 (NOT-CLEAN — streak 0/3)

**F-S2123-P7-P19-001 MED** — Story body retains multiple `ADR-039 §Decision 3 v1.10` and `ADR-039 §Decision 3 v1.9` load-bearing version pins (in body narrative at ~L356, ~L401, ~L720, ~L855, ~L900, ~L942 — 6 sites). POLICY 19 extended at D-1079 to story bodies; these version pins are forbidden. Route: story-writer (strip `v1.10`/`v1.9` suffixes; stable form is `ADR-039 §Decision 3`). **FIXED** (story-writer: S-21.23 v1.6→v1.7 ADR-pin strip at 6 sites).

**F-S2123-P7-P4-002 MED** — Story §Bidirectional Parity Audit Note cites `BC-1.03.018 v1.5` as the provenance source for two AC claims (AC-022 PC9 `all`-wildcard scope and PC9 seven-control partition) that actually first landed in BC-1.03.018 v1.4 (per BC-1.03.018 Changelog — `v1.4 2026-08-22 D-1073: F-S2123-P3-001...PC8 extended...`; `v1.5 2026-08-23 D-1075: F-S2123-P4-001...PC9 control-letter drift corrected`). The parity audit note says `landed at BC-1.03.018 v1.5` but ground truth is v1.4 for PC8-scope and v1.4 for the seven-control count. Route: story-writer (correct provenance at ~L356 and ~L855 — `PC8 all-wildcard + PC9 7-control landed at BC-1.03.018 v1.4`, not v1.5). **FIXED** (story-writer: S-21.23 v1.6→v1.7 provenance correction at 2 sites).

Streak **REMAINS 0/3**.

---

## Note

Substance fully converged — all security/structural axes clean after D-1078. Pass-7/R6 residue is exclusively version-cite/ADR-pin propagation (D-386 Option C class): line-wrapped cite that survived multiline scanning gap (F-S2119-R6-001), sub-version pin (F-S2119-R6-002), BC body missing flip-conditional language (F-S2121-P7-001 HIGH — SoT-level structural gap, not cosmetic), ceil() sweep misses in BC body (F-S2121-P7-002 MED), story Task 4 ceil() formula (F-S2122-P7-001 MED), and story POLICY-19 ADR-pin (F-S2123-P7-P19-001 MED) + provenance correction (F-S2123-P7-P4-002 MED).

Root cause attacked at D-1079: POLICY 19 scope extended to story bodies (closes F-S2123-P7-P19-001 process-gap); multiline version-cite sweep mandate codified in POLICY 5 (closes F-S2119-R6-001 process-gap; generalizes the tr-detector discipline used in D-1078's cited but not-carried-forward fix). S-21.20 achieves 3-CLEAN convergence (pass-8 re-confirmation PENDING per BC anchor change).

---

## Remediation Burst Routing (pass-7/R6)

1. **product-owner (①):** BC-1.03.017 v1.25→v1.26 — Precondition 6 flip-conditional language (F-S2121-P7-001 HIGH) + ceil() sweep at 4+2 body occurrences (F-S2121-P7-002 MED + F-S2122-P7-001 MED); H1 enriched per POLICY 7.
2. **story-writer (②) AFTER PO:** S-21.19 v1.9→v1.10 (F-S2119-R6-001 MED tr-normalized multiline sweep + F-S2119-R6-002 LOW ADR sub-version pin); S-21.20 v1.7→v1.8 (BC-1.03.017 v1.26 re-anchor + story-wide ADR-pin-strip; PC6-orthogonal mechanical); S-21.21 v1.8→v1.9 (BC-1.03.017 v1.26 re-anchor + ADR-pin-strip); S-21.22 v1.8→v1.9 (Task 4 ceil() + BC-1.03.017 v1.26 re-anchor + ADR-pin-strip); S-21.23 v1.6→v1.7 (F-S2123-P7-P19-001 6-site strip + F-S2123-P7-P4-002 provenance + BC re-anchor); S-21.24 v1.9→v1.10 (BC-1.03.017 v1.26 re-anchor + ADR-pin-strip).
3. **state-manager (③):** adv-wave7-pass7.md persist + POLICY-19 scope extension to story-bodies + multiline sweep mandate POLICY-5 codification + D-1079 + BC-INDEX v4.96 + STORY-INDEX v4.390 + STATE.md advance → pass-8/R7 dispatch.
