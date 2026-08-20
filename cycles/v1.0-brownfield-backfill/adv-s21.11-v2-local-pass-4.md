---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-19T00:00:00Z
phase: 3
inputs:
  - .factory/stories/S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.03.017.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.03.018.md
  - .factory/specs/architecture/decisions/ADR-039-validator-failure-policy-resource-exhaustion-fail-closed.md
input-hash: "832164e"
traces_to: S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md
pass: 4
cascade: S-21.11-v2
previous_review: adv-s21.11-v2-local-pass-3.md
---

# Adversarial Review — S-21.11 v2 (LOCAL cascade, pass 4)

> **Persistence note (state-manager, this burst):** content below is transcribed verbatim from the
> orchestrator-relayed adversary dispatch per POLICY 22 (orchestrator-transcribed; the live
> adversary session ran in a separate context this burst resumes from), following the
> `adv-s21.11-v2-local-pass-N.md` convention established at pass-3.

Artifacts reviewed: story `S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md`
v2.3 (input-hash `97029a5`); `BC-1.03.017.md` v1.16; `BC-1.03.018.md` v1.1; `ADR-039` v1.13
(§Erratum E-005, `status: ratified`); factory-artifacts bundle HEAD `041446a4` (D-1043 commit).
Rubric: full `.factory/policies.yaml` (POLICY 1-22).

## Verdict: NOT-CLEAN

1 MEDIUM finding (streak-resetting). 1 non-resetting LOW/ADVISORY observation. 2 grounding
confirmations.

## Finding ID Convention

Finding IDs for the S-21.11 v2 cascade use the format `F-S2111V2-P<PASS>-<SEQ>`, established at
pass-1 (F-S2111V2-P1-001) and continued at pass-2 (F-S2111V2-P2-001..005) and pass-3
(F-S2111V2-P3-001).

## Part A — Fix Verification (pass >= 2 only)

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| F-S2111V2-P3-001 | HIGH | RESOLVED | ADR-039 v1.13 §Erratum E-005 narrows the "Precise rule (normative)" paragraph to the two-condition form (architect, D-1043); BC-1.03.017 v1.16 §Architecture Anchors + §Traceability swept to match (product-owner, D-1043) — re-verified this pass, no residual at the BC-body layer. |

## Part B — New Findings (or all findings for pass 1)

### MEDIUM

#### F-S2111V2-P4-001 (MEDIUM, streak-resetting)

**Location:** story `S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md`
frontmatter `behavioral_contracts:` array + body `## Behavioral Contracts` table BC-1.03.017
Version cell + 56 body narrative/AC/Task/EC citation sites.

**Defect:** D-1043's remediation bumped BC-1.03.017 v1.15→v1.16 (the §AMD-003 fail-closed
narrow-predicate correction), landing correctly in `BC-1.03.017.md` itself, in `BC-INDEX.md`, and
in the `STORY-INDEX.md` S-21.11 catalog row — but the version bump was never swept into the story
file that actually cites BC-1.03.017 throughout its body. The story remained pinned at `BC-1.03.017
v1.15` in its frontmatter `behavioral_contracts:` array and across every body citation site,
violating POLICY 8 (`bc_array_changes_propagate_to_body_and_acs`) and POLICY 3
(citation-staleness parity). Severity is MEDIUM rather than HIGH because BC-1.03.017 v1.16 was a
corrective-only wording change (§Erratum E-005's narrow-predicate formalization) — the *semantics*
the story's own Task #19b already implemented were already correct (per pass-3's Part A
verification); only the *version-pin citations* were stale, not the operative content.

This is the FOURTH instance in the S-21.11 v2 cascade of the
version-cite-propagates/algorithm-content-does-not defect class first codified for the S-21.07
cascade (`L-BB-version-cite-propagation-must-include-algorithm-content-not-just-version-numbers`,
D-1006; recurred a third time at D-1043/pass-3 in the opposite direction — story→BC — and now a
fourth time here in the BC→story direction), confirming the class recurs bidirectionally across
independently-cascading spec artifacts within the same story.

**Routed:** story-writer (BC-1.03.017 v1.15→v1.16 cite-parity sweep across frontmatter + body).

**RESOLVED this burst:** story-writer swept every LIVE cite of `BC-1.03.017 v1.15` →
`BC-1.03.017 v1.16`: the frontmatter `behavioral_contracts:` array; the body `## Behavioral
Contracts` table's BC-1.03.017 Version cell (a split-cell `| BC-1.03.017 | v1.15 |` form not
caught by a naive contiguous-string sweep, found and corrected separately); and 56 body
narrative/AC/Task/EC sites (AC-001, AC-007, AC-008, AC-009, AC-011, AC-012, AC-013, AC-013b,
AC-013c, AC-024 through AC-041, EC-004, the Scope Elements DAG hard-edges note, the "PC13
Full-Registry Coverage" section header, Tasks #6/#10b/#19/#19b/#19c, both Architecture Compliance
Rules hazard rows, the Token Budget BC-files line, and the Routing Proposals "BC authoring routing
— RESOLVED" paragraph). Exempted per POLICY 5 v1.3.5: 7 occurrences of `BC-1.03.017 v1.15` inside
the story's OWN historical `## Changelog` table (6 in the v2.3 row, 1 in the v2.1 row) —
historical-by-construction, correctly left unswept. Story `version` 2.3→2.4.

### LOW / ADVISORY (non-resetting)

**Story Token Budget "Context Source" line omitted §AMD-003 + Erratum E-005.** The line describing
ADR-039 as a context source read `| ADR-039 (full text, 6 decisions + §AMD-001 + §AMD-002
amendments) | 7,500 |` — stale, since ADR-039 v1.13 now also carries §AMD-003 and §Erratum E-005.
**FIXED this burst (production-grade default, in-scope):** corrected to `| ADR-039 (full text, 6
decisions + §AMD-001 + §AMD-002 + §AMD-003 amendments + Erratum E-005) | 7,500 |`. No ADR-039
version-pin citation existed on this line to sweep; token estimate unchanged (description-
completeness fix only, no new content added).

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 1 |
| LOW | 1 (advisory, fixed in-scope) |

**Overall Assessment:** pass-with-findings
**Convergence:** findings remain — iterate (streak resets 0/3; pass-5 required)
**Readiness:** requires revision (routed story-writer; RESOLVED this same burst)

## Grounding confirmations (non-findings, independently re-derived)

- **Fail-closed narrow predicate stated identically across all three layers.** ADR-039 §AMD-003
  (Precise rule leg-1/leg-2 + §Erratum E-005), BC-1.03.017 (PC13 body + §Architecture Anchors
  PC13-extension + §Traceability), and the story (AC-013b, Task #19b) all state the identical
  narrow form — `on_error == Block AND result is Ok { exit_code != 0, .. }` — with no drift
  between layers. `Timeout{Fuel|Epoch}+Block+FailOpen→exit 0 / NOT block` confirmed consistent at
  every site (PC5, PC10(a), EC-009, Invariant 1).
- **18-entry `on_error="block"` registry set exact.** `grep -c` against the live
  `hooks-registry.toml` returns 18, matching AC-024..AC-041 and PC13's Coverage Set table
  row-for-row. No drift since pass-3.

## Observations (non-resetting)

- **[carry-forward, = known F-007]** BC-1.03.017/BC-1.03.018 both still carry `VP-TBD` under
  POLICY 9 — pre-existing, tracked as `[F-007]` in STATE.md Blocking Issues/Drift Items, anchored
  to a future dedicated VP-authoring pass. Not a new finding; re-observed unchanged.

## Novelty Assessment

Novelty MEDIUM. Fourth instance of the version-cite-propagates/algorithm-content-does-not class in
this cascade — this time surfacing as a mid-cascade BURST-BOUNDARY gap: the burst that bumped
BC-1.03.017 (D-1043) did not itself dispatch a story-propagation sweep, leaving the story's own
citations stale for exactly one burst cycle until this pass caught it. Orchestration lesson: when a
BC version bumps mid-cascade, the SAME burst must dispatch story-writer to propagate the new
BC-version cite into every citing story, not only into the index rows.
