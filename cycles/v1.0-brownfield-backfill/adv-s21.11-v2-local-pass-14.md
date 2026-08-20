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
input-hash: "da73a95"
traces_to: S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md
pass: 14
cascade: S-21.11-v2
previous_review: adv-s21.11-v2-local-pass-13.md
---

# Adversarial Review — S-21.11 v2 (LOCAL cascade, pass 14)

> **Persistence note (state-manager, this burst):** content below is transcribed verbatim from the
> orchestrator-relayed adversary dispatch per POLICY 22 (orchestrator-transcribed; the live
> adversary session ran in a separate context this burst resumes from), following the
> `adv-s21.11-v2-local-pass-N.md` convention established at pass-3.

Artifacts reviewed: story `S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md`
v2.11 (input-hash `97029a5`); `BC-1.03.017.md` v1.18; `BC-1.03.018.md` v1.1; `ADR-039` v1.13
(§Erratum E-005, `status: ratified`); factory-artifacts bundle HEAD `c922c742` (D-1053 commit) — the
SAME bundle pass-13's post-remediation state left in place, unchanged per the BC-5.39.001 3-CLEAN
protocol. Rubric: full `.factory/policies.yaml` (POLICY 1-22).

## Verdict: CLEAN

Zero BLOCKER/HIGH/MEDIUM streak-resetting findings. This is the FIRST clean pass since the pass-13
reset — BC-5.39.001 streak ADVANCES **0/3 → 1/3**.

## Finding ID Convention

Finding IDs for the S-21.11 v2 cascade use the format `F-S2111V2-P<PASS>-<SEQ>`, established at
pass-1 (F-S2111V2-P1-001) and continued through pass-13 (F-S2111V2-P13-001). This pass's 2
ADVISORY observations use `F-S2111V2-P14-001..002`.

## Part A — Fix Verification (pass >= 2 only)

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| F-S2111V2-P13-001 | HIGH | RESOLVED | Re-verified new Task #16a (Node (D), immediately after Task #16) authors AC-012's test's three pure-function controls (POSITIVE/NEGATIVE/VACUITY) pre-flip, strictly before the Phase-4c fail-closed-flip Tasks #26–#28; Task #29 now reads as a CONFIRM-ONLY step for the fourth control (LIVE-TREE-CONTROL) post-flip, no longer an authoring step; Task #19's ATOMICITY GATE note correctly points at Task #16a and is literally true at Task #19's execution time (the three pure-function controls already exist and would genuinely fail if Task #19 were committed alone); Task #25's cross-reference correctly names both Task #16a (three controls) and Task #29 (LIVE-TREE-CONTROL only). |
| F-S2111V2-P12-001 | LOW (folded, closed D-1053) | RESOLVED | Re-verified Task #22's adjectival count now reads "AC-014 through AC-021 (all **eight** BC-1.03.018 PC1–PC8 behaviors …)", matching the explicit enumeration (8 ACs, 8 PCs). |

**Independent full re-derivation of the AC-001..AC-041 task-ordering sibling-sweep (the mandatory
class-level fix from D-1053(c)).** Rather than accept the story's own v2.11 changelog claim at face
value, this pass independently rebuilt the complete 41-AC authoring-task/referencing-task ordering
table from the live story text (not from the changelog narrative), checking every AC against both
red-first discipline and reference-before-authoring discipline:

- AC-012 (Task #16a, pre-flip) — confirmed correctly relocated; Task #19's pointer resolves to
  Task #16a; no dangling reference to the old (Task #29-only) authoring site remains anywhere in
  the story.
- AC-009 (Task #20a, pre-flip, D-1051) — re-confirmed still correctly ordered; no regression
  introduced by the AC-012 insertion (Task #16a and Task #20a are independent insertions at
  different DAG nodes, correctly sub-lettered, no renumbering collision between them).
- AC-010 (Task #18, D-1051) — re-confirmed still correctly ordered and still named in Task #20's
  verification enumeration; unaffected by this burst's changes.
- All remaining 38 ACs (AC-001..AC-008, AC-011, AC-013..AC-041 excluding the four above) —
  independently walked task-by-task; every authoring task precedes every task that cites the AC's
  test as already existing or already GREEN; no third sibling of the authored-after-referenced
  class found anywhere in the 41-AC set. **This pass's own exhaustive re-check corroborates
  D-1053(c)'s claim rather than merely trusting it** — the "AC-012 was the ONLY violation" finding
  is independently reproduced, not taken on faith.

## Part B — New Findings (none streak-resetting; CLEAN pass)

No BLOCKER, HIGH, or MEDIUM findings. No streak-resetting findings. 2 ADVISORY observations (see
below).

### Independently re-derived converged axes (all CLEAN, fresh-context)

- TASK/TDD-ordering layer: the full 41-AC ordering table (rebuilt independently above) confirms
  AC-012 was the only violation and it is now fixed; AC-009's Task #20a and AC-010's Task #18
  remain correctly ordered with no regression from this burst's Task #16a insertion; no numbering
  collision introduced by the co-existence of Task #16a, Task #20a, Task #10b, Task #10c, Task
  #19b, Task #19c, and Task #25b (all prior letter-suffixed insertions independently re-confirmed
  still resolvable and non-colliding).
- Version-cite parity (backtick-tolerant AND whitespace-normalized/multiline detector forms both
  run): every live `BC-1.03.017` cite in the story reads `v1.18`; every live `BC-1.03.018` cite
  reads `v1.1`; every live `BC-1.01.016` cite reads `v1.3`. Zero residue under either detector
  form.
- PC13 Coverage Set table: 18 EXACT rows cross-checked against the live `hooks-registry.toml`
  registry, including the `protect-secrets` `^Bash$`/`^Read$` trigger-pattern split — no drift.
- Predicate coherence / axes-independence hold across every site restating PC13's additive-only
  concept and PC6's epoch-parity concept in both BC-1.03.017 and the story — no regression from
  the pass-9 through pass-13 remediations.
- ADR-039 §Erratum E-005's narrowed two-leg predicate form, §AMD-003's RATIFIED status, and
  §Decision 4's fuel/timeout calibration floors all re-confirmed unchanged and internally
  consistent with the story's own restatements.
- Index parity CLEAN under POLICY 7/8/18: BC H1 titles match BC-INDEX title cells verbatim;
  bc-array/body-table/AC propagation intact; story input-hash `97029a5` three-way parity
  re-confirmed (STORY-INDEX cite, story frontmatter, story body content).
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
| LOW | 0 |
| ADVISORY | 2 |

**Overall Assessment:** pass-clean
**Convergence:** streak ADVANCES 0/3 → **1/3**. Two further FRESH CONSECUTIVE CLEAN passes
(pass-15, pass-16) required for BC-5.39.001 3-CLEAN convergence.
**Readiness:** no action required this pass; bundle should remain STABLE for passes 15 and 16.

## Observations (non-resetting)

- **F-S2111V2-P14-001 (ADVISORY):** Task #19b's illustrative snippet
  `matches!(result, PluginResult::Ok { exit_code } if exit_code != 0)` omits the `..` rest-pattern
  (struct-pattern-exhaustiveness form would read `Ok { exit_code, .. }` if `PluginResult::Ok` has
  fields beyond `exit_code`). This is an illustrative code snippet embedded in task-directive prose,
  not shipped implementation code; the intent is unambiguous and the explicit `if exit_code != 0`
  guard predicate is correct regardless of the omitted rest-pattern. **Disposition: DOCUMENTED,
  routed to the convergence-close consolidated cosmetic sweep (story-writer): add `, ..`.**
- **F-S2111V2-P14-002 (ADVISORY):** the Token Budget section's "This story spec | 22,900" row is
  annotated "updated v2.3" — stale, since v2.4 through v2.11 (8 subsequent versions) added further
  content (task insertions, EC rewrites, DAG scope notes) without a corresponding token-count
  re-estimate. POLICY 8's substantive requirement (BC-count cell tracking the 3-BC
  `behavioral_contracts:` array) is satisfied and unaffected — only the informational prose token
  estimate lags. **Disposition: DOCUMENTED, no action required this pass; routed to the
  convergence-close consolidated cosmetic sweep alongside F-S2111V2-P14-001.**

## Novelty Assessment

Novelty LOW — a CLEAN pass following full independent re-derivation of the pass-13 remediation
(Task #16a pre-flip relocation, Task #29 narrowing, Task #19 note reconciliation) plus a
from-scratch, non-trusting rebuild of the complete 41-AC task-ordering table that corroborates
D-1053(c)'s "AC-012 was the only violation" claim rather than accepting it on the prior burst's
say-so. 2 cosmetic ADVISORY observations (a code-snippet rest-pattern omission, a stale token-budget
annotation) — neither touches predicate content, index parity, numeric magnitudes, cite
propagation, or task-DAG structure. No new defect class. Both observations are deliberately left
unremediated this burst — per explicit dispatch scoping — to keep the reviewed bundle byte-identical
for passes 15 and 16, consistent with the BC-5.39.001 3-CLEAN protocol's requirement that
consecutive clean passes review a STABLE artifact. F-S2111V2-P12-002 and F-S2111V2-P12-003
(ADVISORY, from pass-12, confirmed no-action-required at D-1052/D-1053) remain unchanged and are
not re-listed as new findings here; they are re-observed present, non-actionable, and carried
forward in STATE.md's Drift Items alongside this pass's 2 new ADVISORY items.
