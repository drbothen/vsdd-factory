# Wave-7 Pass-6 / S-21.19-R5 — Fresh-Context Adversarial Review Record

**Cycle:** v1.0-brownfield-backfill
**Date:** 2026-08-24
**Spec state reviewed:** BC-1.03.017 v1.25, BC-1.03.018 v1.5, ADR-044 v1.3, ADR-039 v1.16
**Parent commit:** daecbbdb (D-1077)

---

## Verdicts

| Story | Pass | Verdict | Streak | Severity |
|-------|------|---------|--------|----------|
| S-21.19 | R5 | NOT-CLEAN | 0/3 | MED |
| S-21.20 | pass-6 | CLEAN | 1/3→2/3 | LOW (non-resetting ×2) |
| S-21.21 | pass-6 | NOT-CLEAN | 0/3 | HIGH |
| S-21.22 | pass-6 | NOT-CLEAN | 0/3 | MED |
| S-21.23 | pass-6 | NOT-CLEAN | 0/3 | MED+LOW+LOW |

---

## Findings

### S-21.19 (NOT-CLEAN — streak 0/3)

**F-S2119-R5-001 MED** — Story body cites `ADR-044 v1.3` at 14 sites as the live split-topology ADR, but the Task 10a durable-gate context requires citing `ADR-044 v1.1` (the version that first defined the ADDITIVE-not-replacement split topology, which is the invariant being verified). Using v1.3 throughout conflates the flip-capstone ownership clarification (v1.3) with the foundational split-topology invariant (v1.1), violating POLICY 19 split-cite precision discipline. Route: story-writer (sweep all 14 `ADR-044 v1.3` occurrences → `ADR-044 v1.1` where the cite is for the split-topology invariant; retain v1.3 where the cite is for the flip-capstone ownership). **FIXED** (story-writer: S-21.19 v1.8→v1.9).

---

### S-21.20 (CLEAN — streak 1/3→2/3)

2 LOW (non-resetting) findings:

- **F-S2120-P6-001 LOW (POLICY 18):** `ADR-044` absent from story frontmatter `inputs:` array despite being a load-bearing cite in Tasks 1/2 and multiple AC references (split-topology foundation governs PC13/Invariant 11 preconditions). Route: state-manager (POLICY 18 frontmatter/input-hash correction — does NOT bump story version). **FIXED** (state-manager step ④ this burst: ADR-044 added to `inputs:`; input-hash recomputed via compute-input-hash --update).

- **F-S2120-P6-002 LOW (editorial, deferred):** DAG diagram label for S-21.19 still reads `"(REOPENED — ADR-044 v1.2 function-split reconvergence in flight)"`. Now that ADR-044 v1.3 is ratified and the R4/R5 remediation cycle is in progress, the parenthetical is stale-historical context. Non-load-bearing (functional dependency and STOP-if-not-available discipline in Task 1 unchanged). **DEFERRED** — anchor: next S-21.20 story touch.

Streak **ADVANCES 1/3→2/3**.

---

### S-21.21 (NOT-CLEAN — streak 0/3)

**F-S2121-P6-001 HIGH** — Task 10a as written executes the bash-adapter frozen-corpus sufficiency check AFTER Task 10's one-time calibration confirm step, but the ordering is hazardous: if Task 10 (one-time confirm) has already run in a prior CI session and its snapshot is committed, Task 10a's durable gate should be positioned as a CONDITIONAL — only emit the durable gate if the one-time snapshot was committed in THIS session (flip-conditional guard). Without the conditional, Task 10a may re-emit the durable gate redundantly on re-runs, silently overwriting a previously committed frozen corpus. Route: architect (§8.8 flip-conditional addendum: Task 10a guard clause `if snapshot was committed this session`) + story-writer (Task 10a body: add conditional preamble). **FIXED** (architect: decomposition-plan §8.8 flip-conditional; story-writer: S-21.21 v1.7→v1.8 Task 10a conditional preamble).

---

### S-21.22 (NOT-CLEAN — streak 0/3)

**F-S2122-P6-001 MED** — Task 4 regression assertion header incorrectly attributes the sufficiency predicate to `BC-1.03.017 PC6(ii)` — which is the native-WASM `validate-cross-site-correspondence` ownership clause (S-21.22's own mandate) — when the correct cite for Task 4's bash-adapter-class fuel-cap regression is `BC-1.03.017 PC6(i)` (bash-adapter-class calibration sufficiency, owned by S-21.21). The mis-attribution creates an incorrect ownership claim: S-21.22 owns `validate-cross-site-correspondence` only (per §8.8 split-ownership, PC6(ii)); the bash-adapter regression check described in Task 4 actually belongs to S-21.21 (PC6(i)). This is either a cross-story scope leak or a stale pre-§8.8 cite that was not swept at D-1077. Route: story-writer (correct Task 4 cite from PC6(ii)→PC6(i), or confirm Task 4 is correctly in S-21.22 scope with appropriate rationale). **FIXED** (story-writer: S-21.22 v1.7→v1.8 Task 4 cite corrected).

---

### S-21.23 (NOT-CLEAN — streak 0/3)

**F-S2123-P6-001 MED** — Decomposition-plan §4 descriptor for S-21.23 still references the generic "break-glass PC1–PC12 coverage" framing without distinguishing the `std::env::var_os` accessor constraint introduced in BC-1.03.018 v1.5/v1.6's PC9 detector-precision update. The descriptor language predates the accessor-specific narrowing; story tasks correctly use `var_os` but the plan-level descriptor is adrift from the BC (TD-VSDD-060 plan-descriptor sweep miss). Route: architect (§S-21.23 descriptor update to reference `std::env::var_os`-exclusive detector precision). **FIXED** (architect: decomposition-plan §S-21.23 / §8 descriptor updated).

**F-S2123-P6-002 LOW (POLICY 19 HIGH-floor risk)** — BC-1.03.018 PC9 body cites `ADR-039 §Decision 3 v1.10 minimum-viable definition` with an embedded version pin (`v1.10`). POLICY 19 forbids version-pinned ADR-section references in BC body text (spec content drifts silently when ADR versions advance; ADR §Decision section numbers are stable, version pins are not). Route: product-owner (strip `v1.10` pin from the cited phrase → `ADR-039 §Decision 3 minimum-viable definition`). **FIXED** (product-owner: BC-1.03.018 v1.5→v1.6, strip `v1.10`; BC-INDEX v4.94→v4.95 title-cell update per POLICY 7).

**F-S2123-P6-003 LOW (POLICY 19)** — BC-1.03.018 PC9 detector-precision examples use `std::env::var` (the string-typed accessor) in addition to `std::env::var_os`, while BC-1.03.018 body text mandates `std::env::var_os` exclusively per Task 3/AC-014/EC-015. The example inconsistency may mislead implementers into accepting either accessor. Route: product-owner (update detector-precision examples to use `std::env::var_os` exclusively; add accessor-agnostic clarification note). **FIXED** (product-owner: BC-1.03.018 v1.5→v1.6, update detector-precision examples and add clarification sentence).

---

## Note

Floor-break audit (D-1077) succeeded — all OLD residue classes clean at pass-6 (consistency-validator confirmed zero residue from the D-1077 full-perimeter sweep). The five new findings above are novel, decaying-severity items (TD-VSDD-059/060 recurrence-class: F-S2119-R5-001 split-cite sweep; F-S2121-P6-001 flip-conditional guard; F-S2122-P6-001 stale-pre-§8.8 cite; F-S2123-P6-001/P6-002/P6-003 plan-descriptor + POLICY-19 hygiene). No new codification needed — already-codified patterns. D-1077 floor-break remediation confirmed effective.

---

## Next Remediation Burst (routing order enforced)

1. **architect:** decomposition-plan §8.8 flip-conditional addendum for Task 10a (Task 10a guard clause: only emit durable gate if snapshot committed this session) + §S-21.23 descriptor update (`std::env::var_os`-exclusive detector precision).
2. **product-owner AFTER architect:** BC-1.03.018 v1.5→v1.6 (POLICY-19 ADR-version-pin strip `v1.10` from PC9 cite; `std::env::var_os`-exclusive detector-precision examples + accessor-agnostic clarification; H1 enriched per POLICY 7).
3. **story-writer AFTER PO:** S-21.19 ADR-044 v1.3→v1.1 split-cite sweep (14 sites, F-S2119-R5-001); S-21.21 Task 10a conditional preamble (F-S2121-P6-001); S-21.22 Task 4 cite PC6(ii)→PC6(i) (F-S2122-P6-001); S-21.23/S-21.24 BC-1.03.018 v1.6 re-anchor.
4. **state-manager commit ④:** adv-wave7-pass6.md + POLICY-18 S-21.20 inputs: ADR-044 fix + input-hash reconciliation + BC-INDEX v4.95 + STORY-INDEX v4.389 + D-1078 + STATE.md advance → pass-7/R6 dispatch.
