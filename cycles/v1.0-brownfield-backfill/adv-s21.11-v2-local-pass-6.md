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
input-hash: "9231819"
traces_to: S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md
pass: 6
cascade: S-21.11-v2
previous_review: adv-s21.11-v2-local-pass-5.md
---

# Adversarial Review — S-21.11 v2 (LOCAL cascade, pass 6)

> **Persistence note (state-manager, this burst):** content below is transcribed verbatim from the
> orchestrator-relayed adversary dispatch per POLICY 22 (orchestrator-transcribed; the live
> adversary session ran in a separate context this burst resumes from), following the
> `adv-s21.11-v2-local-pass-N.md` convention established at pass-3.

Artifacts reviewed: story `S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md`
v2.5 (input-hash `97029a5`); `BC-1.03.017.md` v1.17; `BC-1.03.018.md` v1.1; `ADR-039` v1.13
(§Erratum E-005, `status: ratified`); factory-artifacts bundle HEAD `64460671` (D-1045 commit).
Rubric: full `.factory/policies.yaml` (POLICY 1-22).

## Verdict: NOT-CLEAN

1 HIGH finding (streak-resetting). 2 non-resetting ADVISORY observations. Multiple grounding
confirmations.

## Finding ID Convention

Finding IDs for the S-21.11 v2 cascade use the format `F-S2111V2-P<PASS>-<SEQ>`, established at
pass-1 (F-S2111V2-P1-001) and continued at pass-2 (F-S2111V2-P2-001..005), pass-3
(F-S2111V2-P3-001), pass-4 (F-S2111V2-P4-001), and pass-5 (F-S2111V2-P5-001).

## Part A — Fix Verification (pass >= 2 only)

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| F-S2111V2-P5-001 | HIGH | RESOLVED | product-owner's BC-1.03.017 v1.16→v1.17 Invariant 10 rewrite (+ 4 sibling sites) re-verified this pass, no residual STRICT-SUPERSET/`exactly as before` framing anywhere in BC-1.03.017; story-writer's 58-cite `v1.17` sweep re-verified, no residual `v1.16` cite live in the story. |

## Part B — New Findings (or all findings for pass 1)

### HIGH

#### F-S2111V2-P6-001 (HIGH, streak-resetting)

**Location:** S-21.11 story `## Edge Cases` table, EC-011 row's "Post-wiring-fix" clause, plus 2
further residual sites found on exhaustive sweep: AC-013's own body (the embedded "EC-011"
paragraph immediately preceding "see Edge Cases EC-011 below"), and AC-013b's opening paragraph's
axis-grouping label.

**Defect:** BC-1.03.017 v1.17 (D-1045, this same cascade's immediately-prior remediation) already
rewrote its OWN EC-011 to the axes-independent per-outcome form — conditioning the `Ok{exit_code !=
0}` sub-case on PC13/`on_error` and the `Timeout` sub-case on PC1/PC6/PC10(b)/`failure_policy=
FailClosed`, explicitly noting the scenario assumes the plugin's steady-state `failure_policy=
FailClosed` annotation (PC9), not `on_error=Block` alone. This fix was never propagated into this
story's OWN sibling EC-011 row, which still stated the post-fix outcome "blocks under `on_error=
Block` regardless of which sub-case surfaces" (AC-013's embedded paragraph) and "MUST produce a
block under `on_error=Block`" — "AC-005/AC-011 already close the `Timeout`/`Crashed` case" (the
EC-011 table row itself). D-1045's own remediation burst ran a defensive semantic check for this
exact predicate-coherence theme, grepping for `strict superset`/`superset of`/`continue to block`/
`exactly as before`/`NOT Ok{exit_code`/`regardless of failure_policy` — a pattern list built from
BC-1.03.017's pre-fix WORDING, not from the CONCEPT. The story's own EC-011 used distinct phrasing
("MUST produce a block under `on_error=Block`" + "AC-005/AC-011 already close the `Timeout`/
`Crashed` case") that the pattern list did not anticipate, so the check reported clean while the
identical defect class survived un-swept. Three concrete errors resulted: (1) `Timeout{Fuel|Epoch}`
blocking was attributed to `on_error=Block` alone, omitting the load-bearing `failure_policy=
FailClosed`/PC9 condition; (2) AC-005(b)'s semantics were inverted — AC-005(b) is a NOT-block
assertion (`Timeout` + `on_error=Block` + `failure_policy=FailOpen` → NOT block), not a
block-closing citation the EC-011 clause could lean on; (3) the story's own EC-009 row was
contradicted (`on_error=block` does not apply to exhaustion outcomes, PC5).

This is the SIXTH instance of the version-cite-propagates/algorithm-content-does-not class in this
cascade (D-1006), and — like pass-5 — a CONTENT-level residue rather than a pure version-citation
staleness gap. Unlike pass-5 (intra-artifact, BC-body→BC-body within the SAME burst), this instance
is cross-artifact and cross-burst: the D-1045 burst fixed the concept in BC-1.03.017's own EC-011
but did not dispatch a story-side sibling-sweep for the story's OWN copy of the same edge case,
because D-1045's defensive-check pattern list was derived from the BC's pre-fix wording, not from
an independent semantic read of the story. This confirms pass-5's own lesson (D-1045(h): sweeps must
be SEMANTIC, enumerating every site that STATES the concept, not literal-string matches against one
artifact's specific phrasing) generalizes across artifact boundaries, not only within one.

**Routed:** story-writer (S-21.11 EC-011 rewrite, mirroring BC-1.03.017 v1.17 EC-011's corrected
structure, plus an exhaustive story-wide semantic sweep — not a narrow phrase list — of every
predicate/axes-stating site in the story).

**RESOLVED this burst:**
- **story-writer** — S-21.11 v2.5→v2.6: rewrote the Edge Cases table's EC-011 row to condition each
  sub-outcome on its own governing axis, mirroring BC-1.03.017 v1.17 EC-011's corrected structure:
  `Ok{exit_code != 0}` → PC13/AC-013b under `on_error=Block` (`failure_policy`-independent);
  `Timeout` → PC1/PC6/PC10(b)/AC-011(b) under `failure_policy=FailClosed` (`on_error`-independent
  for the exhaustion decision), with the steady-state `failure_policy=FailClosed` (PC9/AC-008)
  assumption stated explicitly and an inline cross-reference confirming consistency with EC-009.
  Ran an exhaustive story-wide semantic sweep (all Edge Cases EC-001–EC-014, all Acceptance
  Criteria AC-001–AC-041, all 32 Tasks, the Behavioral Contracts table, Architecture Mapping,
  Purity Classification, Architecture Compliance Rules, Previous Story Intelligence, Library &
  Framework Requirements, File Structure Requirements, and Routing Proposals) for the axes-
  conflation concept regardless of wording, and found 2 further residual sites the table-row fix
  alone would have left behind: AC-013's own embedded "EC-011" paragraph (rewritten to the same
  per-axis breakdown), and AC-013b's opening paragraph, which grouped AC-005/AC-011 together as
  "`on_error`-vs-`Crashed` coverage" — imprecise, since AC-011 is `Timeout`/`failure_policy`
  axes-independence coverage, not `Crashed` coverage (the identical PC4/PC5/PC10-grouping
  imprecision BC-1.03.017 v1.17 already corrected in its own PC13 header) — split into "AC-005(a)'s
  `on_error`-governs-crash coverage" and "AC-011's `failure_policy`-governs-`Timeout`
  axes-independence coverage." EC-009 consistency confirmed (no change needed — already correct).
  All other examined sites verified already correctly axes-independent; the legitimate
  AC-022⊃AC-012 "strict superset" gate-coverage relationship (an unrelated concept — mechanical
  CI-gate ordering coverage, not the predicate) was correctly left untouched.

### ADVISORY (non-resetting)

**ADR-039 §AMD-003 option-(b) closing prose retains "strict superset."** Verified SEMANTICALLY
CORRECT, not a defect: this occurrence is a superset of the pre-existing exit-code-2 stdout
protection (the pre-§AMD-003 baseline behavior), NOT the E-005-rejected "superset of `Crashed |
Timeout`" framing that BC-1.03.017's Invariant 10 and EC-011 both had to be rewritten away from at
D-1043/D-1045. No implementer reading this clause in isolation would build the wrong predicate from
it — it describes a different, narrower relationship than the defect class this cascade has been
chasing. Deliberately NOT churned this burst: bumping ADR-039 to reword a non-defect wording nit
would re-trigger the input-hash-churn→propagation-residue cycle that reset the streak at passes 4
and 5 (a BC/ADR version bump obligates a full cite-parity sweep across every citing artifact),
trading a demonstrated residue-risk cost for a cosmetic clarity gain with zero safety impact.
Disposition: touch when ADR-039 is next legitimately edited for a substantive reason. Also
recorded: BC-1.03.017 v1.17's own changelog row's `last_amended` narrative says "the story declares
this BC as an input" — imprecise; the story's `inputs:` frontmatter array does NOT include
BC-1.03.017.md (only ADR-039, the wasm-fuel-exhaustion-detection.md research doc, and
hooks-registry.toml). Non-load-bearing historical-changelog prose, not churned for the same reason.

**S-21.11 story input-hash `97029a5` re-verified, not stale.** Prior passes flagged this hash as
possibly stale relative to ADR-039 v1.13 without being able to run `compute-input-hash`. This pass
confirms: ADR-039 v1.13's content was already absorbed into `97029a5` at D-1043 (when ADR-039
changed v1.12→v1.13); ADR-039 has not changed since. Definitively CLOSED this burst — see
Grounding confirmations below.

## Confirmations (converged)

- Governing ADR §AMD-003 (Precise Rule + Erratum E-005) and BC-1.03.017 v1.17 (Invariant 10, PC13,
  EC-011, Architecture Anchors, Traceability) now ALL state the narrow additive-only
  axes-independent predicate consistently.
- `Timeout{Fuel|Epoch} + on_error=Block + failure_policy=FailOpen → exit 0 / NOT block` holds at
  PC5, PC10(a), EC-009, Invariant 1, AC-005(b), AC-011, and TC-12(a) — no contradiction anywhere in
  the reviewed bundle.
- 18-entry `on_error="block"` `hooks-registry.toml` plugin enum remains EXACT against AC-024–AC-041
  and PC13's Coverage Set table, row-for-row.
- BC-1.03.017 v1.17 version-cite parity holds across story, BC-INDEX, and STORY-INDEX.
- POLICY 7 (BC H1 ↔ BC-INDEX title cell) and POLICY 8 (`behavioral_contracts` array → body table →
  ACs propagation) parity both hold.
- Erratum E-005's re-ratification-not-required disposition (POLICY-22-exempt erratum category,
  `status: ratified` preserved) remains SOUND — no new information this pass changes that
  assessment.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 1 |
| MEDIUM | 0 |
| LOW | 0 |
| ADVISORY | 2 (non-resetting, RECORDED not churned) |

**Overall Assessment:** pass-with-findings
**Convergence:** findings remain — iterate (streak resets 0/3; pass-7 required)
**Readiness:** requires revision (routed story-writer; RESOLVED this same burst)

## Grounding confirmations (non-findings, independently re-derived)

- **Story input-hash three-way parity confirmed.** `compute-input-hash --check` (operator-
  authoritative marketplace rc.23 binary, per-file, per L-EDP1-073) against the S-21.11 story
  returns clean: computed `97029a5` = stored `97029a5`. `--resolve` confirms all 3 declared inputs
  (ADR-039, `wasm-fuel-exhaustion-detection.md`, `hooks-registry.toml`) resolve. Cross-checked
  against STORY-INDEX catalog row and E-21 delivery blockquote — all three cite `97029a5`. Prior
  passes' "possibly stale vs ADR-039 v1.13" flag is definitively CLOSED: ADR-039 v1.13's content
  was already absorbed into `97029a5` at D-1043; no further ADR-039 change has occurred since.
- **EC-009 remains internally consistent** with the rewritten EC-011 — both now state
  `failure_policy=FailOpen` prevents exhaustion-outcome blocking regardless of `on_error`, with no
  contradiction between the two rows.
- **No other story site echoes the fixed conflation pattern.** The exhaustive story-wide sweep
  (AC-001–AC-041, all 32 Tasks, EC-001–EC-014, Behavioral Contracts table, Architecture Mapping,
  Purity Classification, Architecture Compliance Rules, Previous Story Intelligence, Library &
  Framework Requirements, File Structure Requirements, Routing Proposals) confirmed only the 3
  sites fixed this burst (EC-011 table row, AC-013's embedded paragraph, AC-013b's opening
  paragraph) carried the defect; every other site was already axes-independent.

## Observations (non-resetting)

- **[carry-forward, = known F-007]** BC-1.03.017/BC-1.03.018 both still carry `VP-TBD` under
  POLICY 9 — pre-existing, tracked as `[F-007]` in STATE.md Blocking Issues/Drift Items, anchored
  to a future dedicated VP-authoring pass. Not a new finding; re-observed unchanged.

## Novelty Assessment

Novelty MEDIUM. Sixth instance of the version-cite-propagates/algorithm-content-does-not class in
this cascade, and the second instance (after pass-5) of a CONTENT-level predicate-coherence
residue rather than a pure version-citation staleness gap — but the first instance where the
residue crosses an artifact boundary in the OTHER direction from pass-4 (BC-fixed-first,
story-not-yet-swept, rather than story-fixed-first/BC-not-yet-swept) via a defensive-check pattern
list built from the wrong artifact's wording. Orchestration lesson: D-1045(h)'s "sweeps must be
SEMANTIC" discipline generalizes ACROSS artifact boundaries — when one artifact's predicate-
coherence fix produces a defensive-check pattern list, that list is necessarily anchored to THAT
artifact's specific pre-fix phrasing, and will systematically miss a sibling artifact restating the
identical concept in different words. A cross-artifact sibling-sweep dispatch (not just a
same-artifact defensive check) is required whenever a predicate/invariant is narrowed or corrected
in one of two or more artifacts that jointly state the same concept. This pass's story-wide
exhaustive semantic sweep found and fixed all 3 residual sites in one burst, consistent with pass-5's
demonstrated pattern that an exhaustive semantic sweep (vs. a targeted grep) terminates a given
residue chain in a single pass.
