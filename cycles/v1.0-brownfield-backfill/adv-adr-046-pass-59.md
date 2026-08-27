# ADR-046 Adversarial Spec-Convergence Review — Pass 59

**Reviewed artifact set (frozen):** ADR-046 v1.23 + BC-4.17.001 v1.26 + BC-5.40.001 v1.20 + BC-7.07.001 v1.39
**Review date:** 2026-08-27
**Verdict:** FINDINGS (1 MED) — F-P59-001, FIXED
**Gate:** BC-5.39.001 3-CLEAN spec-convergence (ADR-046 fix-state-writes)
**Streak effect:** 0/3 → **STAYS 0/3** (already at floor from pass-58; this is a fix burst, not an additional reset)
**D-chain:** D-1116

## Part A — Finding Set (frozen set: ADR-046 v1.23 + BC-4.17.001 v1.26 + BC-5.40.001 v1.20 + BC-7.07.001 v1.39)

**HIGH (0):** none this pass.

**MEDIUM (1):**

### F-P59-001 (MED, POLICY 4 internal-consistency)

BC-5.40.001's §Traceability ADR Reference row and §Description named ADR-046 coverage only for
§Decision 1(b) (PC4 mid-burst renewal actor reassignment), **omitting Decision 5** — despite this
same BC's own body containing multiple explicit MIGRATED/RETAINED-AS-HISTORICAL annotations under
§Decision 5's guard-read reconciliation:

- Precondition 6 (`STATE_MD_MAX_BYTES` read-cap + mandatory `extract_frontmatter` use — MIGRATED)
- Invariant 7 (`extract_frontmatter`-exclusive operation — MIGRATED)
- Invariant 8 (`state_md_approaching_cap` soft-warn — MIGRATED)
- EC-010 (`OutputTooLarge` fail-open — MIGRATED)
- §Verification Properties / §VP Anchors T-001..T-007 (RETAINED AS HISTORICAL/DORMANT)

This is the **mirror-image gap of BC-4.17.001's own F-P58-001** (fixed at pass-58, v1.26, on the
migration TARGET side — BC-4.17.001 is where the guard-read contract migrated TO). BC-5.40.001 is
the migration SOURCE — the BC the contract migrated FROM — and this source-side coverage gap was
never itself swept when the pass-58 fix landed only on the target.

**Ground truth:** ADR-046's `## Decision` list (flat, 6 items) — Decision 5 is "retire
`verify-state-timestamp-refresh`; migrate its read-cap/`extract_frontmatter`/soft-warn guard-read
logic into `stamp-state-timestamp` (BC-4.17.001) and `precompact-flush` (BC-7.07.001)," with
BC-5.40.001 as the historical origin BC whose Precondition 6/Invariant 7/Invariant 8/EC-010 the
migration moved out of. A BC that carries five separate live-body MIGRATED/RETAINED annotations
citing §Decision 5, but whose own top-level ADR-coverage summary omits §Decision 5 entirely, is an
internal self-contradiction of the same class as F-P58-001.

**Disposition:** FIXED by product-owner (BC-5.40.001 v1.20→**v1.21**):
- §Description gains a new sentence stating ADR-046 §Decision 5 reconciles the guard-read contract
  originally specified here (Precondition 6/Invariants 7-8/EC-010), migrated out to BC-4.17.001,
  retained here only as a historical/dormant record per §Decision 5's crate-retention clause.
- §Traceability ADR Reference row now cites `ADR-046 §Decision 1(b)/§Decision 5` with a §Decision 5
  summary (guard-read/cap reconciliation migrated-out to BC-4.17.001; T-001..T-007 retained-as-
  historical per §Decision 5's crate-retention clause).

No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1).

**Mandatory cluster-wide ADR-Decision-coverage audit (in-scope, this pass, per the D-1115-codified
discipline):** every `ADR-046 §Decision N` / `ADR-025 §Decision N` token in the live bodies of
BC-4.17.001 and BC-7.07.001 was enumerated and cross-checked against each BC's own §Traceability
ADR Reference row:

| BC | Body cites | ADR Reference row cites | Verdict |
|----|-----------|--------------------------|---------|
| BC-4.17.001 (v1.26) | §Decision 1/1(b)/2/4/5 | §Decision 1/1(b)/2/4/5 (fixed at pass-58) | **COMPLETE — no gap** |
| BC-5.40.001 (v1.20→v1.21, this pass) | §Decision 1(b)/5 (five migrated/retained annotations) | §Decision 1(b) only (v1.20) | **GAP — F-P59-001, fixed to v1.21** |
| BC-7.07.001 (v1.39) | §Decision 1(b)/3/4 only | §Decision 1(b)/3/4 | **CLEAN — not a §Decision 5 participant** |

BC-7.07.001's two `ADR-025 §Decision 11` body citations (factory-lock-write.sh availability
context) and BC-4.17.001's/BC-5.40.001's `ADR-025 §Decision 14` cap-sourcing citations are passing
supporting-citation footnotes, not migration/retention/coverage-class annotations under the
established F-P58-001/F-P59-001 convention, and are symmetrically already omitted from all three
BCs' ADR Reference rows — confirmed not a fresh drift, no action required. BC-5.40.001's separate
"covers ADR-025 Decisions 2, 3, 5, 8, and 10" sentence is ADR-025-scoped and untouched by this fix
(not conflated, out of finding scope).

**Result of audit:** BC-5.40.001 was the LAST remaining gap in the cluster-wide
ADR-Decision-coverage-enumeration discipline (codified D-1115); after this fix, all three companion
BCs' ADR Reference rows are confirmed complete against their own live-body migration/retention
annotations.

**LOW/BLOCKING (0):** no additional blocking findings; no non-blocking observations recorded this
pass (the two pass-58 observations, O-P58-001/O-P58-002, were re-checked and remain
ACCEPTED-tracked/untouched — see Part B).

## Part B — Verified-Clean Observations (adversary-confirmed, no additional blocking findings)

Fresh-context adversary review, information-asymmetric per the Iron Law (no visibility into
prior-pass narrative, decision-log, lessons.md, or burst-log — reviewed the frozen artifact set on
its own merits only). Beyond F-P59-001, this pass re-applied all seventeen previously-codified
convergence-technique disciplines plus the eighteenth (ADR-Decision-coverage-enumeration, D-1115)
and found no regression:

- **F-P58-001 fix (ADR-Decision-coverage-enumeration, eighteenth discipline, D-1115):** re-confirmed
  holding on BC-4.17.001's side; found the mirror-image gap on BC-5.40.001's side — this pass's own
  F-P59-001 finding is the CLUSTER-DRAINING application of the same discipline product-owner's own
  pass-58 disposition prose explicitly anchored ("apply proactively to BC-5.40.001/BC-7.07.001/
  ADR-046 at the next pass" — Drift Items table, D-1115).
- **F-P56-001 fix (0th-case/case-1 boundary, seventeenth discipline):** re-confirmed holding across
  all four artifacts, no regression.
- **STEP-NUMBER CITATION (sixteenth discipline, D-1111):** re-confirmed holding.
- **Illustrative "analogous to T-NNN" enumeration accuracy (ninth discipline):** re-confirmed holding.
- **Behavioral core (write-composition table, identity-gating logic, event-sourcing struct-variant
  text):** re-verified CLEAN — stable since pass-27.
- **AC-attribution cluster-wide drain (eighth/eleventh disciplines):** re-derived, zero mis-anchors.
- **CITATION→INPUT PARITY / catalog-membership-verification (fourteenth/fifteenth disciplines):** no
  new gaps found.
- **BC-7.07.001 §Description "no-op" phrasing (O-P53-DESC-NOOP):** re-examined — remains DEFENSIBLE,
  ACCEPTED-tracked, untouched.
- **BC-5.40.001 `modified:` array v1.1–v1.4 disposition-prose gap (O-P42-001):** re-examined —
  remains ACCEPTED-tracked, untouched by this pass (different locus from F-P59-001).
- **BC-4.17.001 EC-011 `holder: null` documentation asymmetry (O-P57-001):** re-examined — remains
  ACCEPTED-tracked, untouched.
- **F-P27-001/F-P25-002 provenance-ID split (O-P58-001):** re-examined — remains CONFIRMED
  NON-DEFECT, ACCEPTED-tracked, untouched.
- **BC-4.17.001 `status`/`lifecycle_status` draft/draft (O-P58-002):** re-examined — remains
  NON-DEFECT, ACCEPTED-tracked, untouched.

No new non-blocking observations were raised this pass.

## Part C — State at Close of Review

ADR-046 **v1.23, UNCHANGED**. BC-4.17.001 **v1.26, UNCHANGED**. BC-5.40.001 **v1.20→v1.21**
(F-P59-001 fix). BC-7.07.001 **v1.39, UNCHANGED**. **BC-5.39.001 3-CLEAN streak: 0/3 → STAYS 0/3**
(the streak was already at floor entering this pass from pass-58's reset; this fix burst does not
add a further reset — it is the same streak-at-zero state, now with the cluster-wide
ADR-Decision-coverage-enumeration gap fully drained). Gate history to date: 59 passes run against
evolving/frozen sets; 45 genuine BLOCKING findings found and fixed (44 prior + F-P59-001), plus 10
audit-extra stragglers, 1 latent-bracket drain (pass-37, not counted as genuine), 4 ACCEPTED
non-blocking observations carried forward unchanged (O-P42-001, O-P53-DESC-NOOP, O-P57-001,
O-P58-001, plus O-P58-002), and 3 FIXED non-blocking observations (O-P44-001, O-P48-001,
O-P51-001). The behavioral core (write-composition, identity-gating, event-sourcing) remains
independently re-verified CLEAN for the 32nd consecutive pass (since pass-27) — this pass's finding
is confined entirely to the ADR-Decision-coverage-enumeration perimeter, not the behavioral core.

**Index reconciliation:** BC-INDEX v5.17→v5.18 (BC-5.40.001 row version-cell + Changelog
cross-ref). ARCH-INDEX, STORY-INDEX, VP-INDEX all UNCHANGED (only BC-5.40.001 touched this pass).

**Input-hash recompute:** BC-5.40.001 `a21ce60`→`6a9cc08` (cyclic-hash TD [D-1082], settled +
cross-referenced, NOT reopened). ADR-046 `3335ad4`, BC-4.17.001 `6b0b35c`, BC-7.07.001 `e73bc01`
remain unchanged (not edited this pass).

**Process note:** this pass's finding is the SIBLING STRAGGLER of pass-58's F-P58-001 — the
migration-coverage fix at pass-58 touched only the TARGET BC (BC-4.17.001); the SOURCE BC
(BC-5.40.001) reset back to a fresh gap at pass-59 because the pass-58 fix-burst did not sweep both
migration parties at fix time. This is the SAME single-artifact-scoped-fix root cause as the
AC-attribution class drained at D-1104 (eleventh discipline) — reinforced this burst as a standing
process-gap codification (see decision-log D-1116 / lessons.md).

**NEXT: fresh pass-60** against the pass-59-corrected frozen set (ADR-046 v1.23 + BC-4.17.001
v1.26 + **BC-5.40.001 v1.21** + BC-7.07.001 v1.39), applying all eighteen codified
convergence-technique disciplines proactively from the start. Streak stays at 0/3 — 3 consecutive
clean passes (60, 61, 62) needed for literal BC-5.39.001 3-CLEAN. The human decision this session
remains to CONTINUE looping toward literal 3-CLEAN (not accept D-386 Option C asymptotic
acceptance) — accept-provisional under D-386 Option C remains available at any time. S-17.05 TDD
implementation remains gated on convergence.
