---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-20T00:00:00Z
phase: 3
inputs:
  - .factory/stories/S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.03.017.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.03.018.md
  - .factory/specs/architecture/decisions/ADR-039-validator-failure-policy-resource-exhaustion-fail-closed.md
input-hash: "8a3f0c9"
traces_to: S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md
pass: 12
cascade: S-21.11-v2
previous_review: adv-s21.11-v2-local-pass-11.md
---

# Adversarial Review — S-21.11 v2 (LOCAL cascade, pass 12)

> **Persistence note (state-manager, this burst):** content below is transcribed verbatim from the
> orchestrator-relayed adversary dispatch per POLICY 22 (orchestrator-transcribed; the live
> adversary session ran in a separate context this burst resumes from), following the
> `adv-s21.11-v2-local-pass-N.md` convention established at pass-3.

Artifacts reviewed: story `S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md`
v2.10 (input-hash `97029a5`); `BC-1.03.017.md` v1.18; `BC-1.03.018.md` v1.1; `ADR-039` v1.13
(§Erratum E-005, `status: ratified`); factory-artifacts bundle HEAD `06bdde56` (D-1051 commit) — the
SAME bundle reviewed at pass-11's post-remediation state, unchanged per the BC-5.39.001 3-CLEAN
protocol. Rubric: full `.factory/policies.yaml` (POLICY 1-22).

## Verdict: CLEAN

Zero BLOCKER/HIGH/MEDIUM streak-resetting findings. This is the FIRST clean pass since the pass-11
reset — BC-5.39.001 streak ADVANCES **0/3 → 1/3**.

## Finding ID Convention

Finding IDs for the S-21.11 v2 cascade use the format `F-S2111V2-P<PASS>-<SEQ>`, established at
pass-1 (F-S2111V2-P1-001) and continued through pass-11 (F-S2111V2-P11-001..003). This pass's 3
LOW/ADVISORY observations use `F-S2111V2-P12-001..003`.

## Part A — Fix Verification (pass >= 2 only)

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| F-S2111V2-P11-001 | MEDIUM | RESOLVED | Re-verified the Routing Proposals "BC authoring routing — RESOLVED" opening sentence now reads `BC-1.03.017 (\`factory-dispatcher::executor::failure_policy enforcement\`, v1.18)`, consistent with the same paragraph's own "v1.18's new Invariant 11" reference two lines later — no intra-paragraph contradiction. A fresh backtick-tolerant sweep (`BC-1\.03\.017[^v]{0,120}v1\.[0-9]+`) of the story confirms zero non-v1.18 live residue. |
| F-S2111V2-P11-002 | MEDIUM | RESOLVED | Re-verified new Task #20a (Node (D), immediately after Task #20, strictly before Node (E)/Phase 4c) authors `test_all_six_validator_class_plugins_are_fail_closed` with an explicit CONFIRM-RED instruction; Task #29 now reads as a CONFIRM-GREEN verification step post-flip, not an authoring step; Task #16's pointer correctly reads "(Task #20a below)". Task sequencing independently re-derived: Task #20a precedes Tasks #26–#28 (the Phase-4c fail-closed-flip) in the DAG, restoring a genuine first-run RED observation for AC-009. |
| F-S2111V2-P11-003 | MEDIUM | RESOLVED | Re-verified AC-010's test authoring is present at Task #18 alongside its fuel-axis exhaustion siblings, and Task #20's RED-gate verification enumeration now reads "AC-002 through AC-005 and AC-010" — AC-010 is no longer absent from either the authoring DAG or the verification enumeration. |

## Part B — New Findings (none; CLEAN pass)

No BLOCKER, HIGH, or MEDIUM findings. No streak-resetting findings.

### Independently re-derived converged axes (all CLEAN, fresh-context)

- TDD-ordering layer: Task #20a authors AC-009's red-first gate test strictly before the Phase-4c
  fail-closed-flip Tasks #26–#28; Task #16's pointer to Task #20a resolves correctly; Task #29
  correctly reads as a post-flip GREEN-confirmation step, not an authoring step; no other task in
  the DAG references AC-009's test by a stale task number.
- AC-010 coverage: authoring task present at Task #18 (alongside AC-002/AC-003/AC-004/AC-005); Task
  #20's verification enumeration names AC-010 explicitly; no other RED-gate-bearing AC found absent
  from both an authoring task and the verification enumeration on a full re-derivation of every AC
  in the story.
- Task cross-reference integrity: every task-number pointer in the story (DAG captions, "(Task #N
  below)" notes, Node headers) resolves to an actual task at that number; no dangling or
  off-by-one pointer found; no numbering collision introduced by Task #20a's insertion (sub-lettered
  correctly, not renumbering any existing task).
- Version-cite parity (backtick-tolerant AND whitespace-normalized contiguous forms both run):
  every live `BC-1.03.017` cite in the story reads `v1.18`; every live `BC-1.03.018` cite reads
  `v1.1`; every live `BC-1.01.016` cite reads `v1.3`. Zero residue under either detector form.
- Predicate coherence / axes-independence hold across every site restating PC13's additive-only
  concept and PC6's epoch-parity concept in both BC-1.03.017 and the story — no regression from the
  pass-9/pass-10/pass-11 remediations.
- Erratum E-005's re-ratification-not-required disposition remains legitimate — the narrowed
  two-leg predicate form is unchanged since D-1043 and does not require a fresh ratification cycle.
- Numeric-magnitude parity CLEAN — `timeout_ms` uniformly `_000`-scale, `fuel_cap` uniformly
  M-scale except the documented AC-002 low-fuel fixture (`fuel_cap = 100`) — consistent with
  ADR-039 §Decision 4's formulas; no regression from pass-8.
- AC↔PC↔EC↔Invariant cross-reference integrity and counts re-confirmed unchanged: BC-1.03.017
  13 PCs / 11 Invariants / 11 ECs / 5 Preconditions; BC-1.03.018 10 PCs / 6 Invariants / 6 ECs;
  PC13's Coverage Set table 18 rows matching AC-024..AC-041 exactly.
- Index parity CLEAN under POLICY 4/6/7/8/18: BC H1 titles match BC-INDEX title cells verbatim;
  bc-array/body-table/AC propagation intact; story input-hash `97029a5` three-way parity
  re-confirmed (STORY-INDEX cite, story frontmatter, story body content).
- Token Budget cell: 60,000 tokens = 30.0% of the story's stated ceiling, arithmetic re-verified.
- BC-1.03.018 depth (10 PCs / 6 Invariants / 6 ECs, full break-glass coverage) re-confirmed
  adequate against its governing ACs.
- `VP-TBD` on BC-1.03.017/BC-1.03.018 (`[F-007]`) remains a sanctioned, previously-disclosed
  deferral (POLICY 9) — re-observed unchanged, not a new finding.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 0 |
| LOW | 1 |
| ADVISORY | 2 |

**Overall Assessment:** pass-clean
**Convergence:** streak ADVANCES 0/3 → **1/3**. Two further FRESH CONSECUTIVE CLEAN passes
(pass-13, pass-14) required for BC-5.39.001 3-CLEAN convergence.
**Readiness:** no action required this pass; bundle should remain STABLE for passes 13 and 14.

## Observations (non-resetting)

- **F-S2111V2-P12-001 (LOW):** Task #22 (Node E) reads "AC-014 through AC-021 (all **nine**
  break-glass/hardening ACs)" — an off-by-one adjectival count; AC-014 through AC-021 inclusive is
  8 ACs (matching BC-1.03.018 PC1–PC8), not nine. The explicit enumeration itself
  ("AC-014 through AC-021") is correct and complete, so there is no implementer misdirection or
  coverage gap — only the parenthetical adjective is wrong. **Disposition: DOCUMENTED, routed to the
  convergence-close consolidated cosmetic sweep (story-writer): "nine" → "eight".**
- **F-S2111V2-P12-002 (ADVISORY):** the story's Node (D) header describes itself as "independent of
  B/C," which is slightly loose given Task #20a's soft Node-B prerequisite (AC-013 must be GREEN
  before AC-009's test can meaningfully assert against the registry state Node B establishes). Task
  SEQUENCE itself is correct — Node B (Tasks #6–10) precedes Node D in the DAG regardless of the
  header's wording — so this is a label-scope precision nit, not a sequencing defect.
  **Disposition: DOCUMENTED, no change required this pass; may be tightened at convergence-close if
  the bundle is touched for other reasons.**
- **F-S2111V2-P12-003 (ADVISORY):** the story's DAG Node-(D) illustrative box omits AC-009 from its
  caption. Independently re-derived: this is CONFIRMED INTENTIONAL, not a partial-propagation miss
  — the box deliberately scopes to decision-function-behavior ACs and excludes the registry-scan
  gate ACs (both AC-001 and AC-009 are absent from the box on the same basis; AC-001 was never
  listed either, before or after any prior remediation). No change needed.

## Novelty Assessment

Novelty LOW — a CLEAN pass following full re-verification of all three pass-11 remediations (Task
#20a red-first insertion, AC-010 authoring/enumeration, backtick-tolerant cite fix), plus 3
cosmetic LOW/ADVISORY observations (an adjectival off-by-one count, a header label-scope nit, and a
confirmed-intentional DAG-box scoping question). None of the three observations touch predicate
content, index parity, numeric magnitudes, cite propagation, or task-DAG structure. No new defect
class. All 3 observations are deliberately left unremediated this burst — per explicit dispatch
scoping — to keep the reviewed bundle byte-identical for passes 13 and 14, consistent with the
BC-5.39.001 3-CLEAN protocol's requirement that consecutive clean passes review a STABLE artifact.
F-S2111V2-P12-001 is routed to the same single consolidated cosmetic sweep at convergence-close
already holding pass-10's 3 folded-in items' successors (none currently outstanding — those closed
at D-1051); F-S2111V2-P12-002 and F-S2111V2-P12-003 require no action (label-precision nit and
confirmed-intentional scoping, respectively).
