# ADR-046 Adversarial Spec-Convergence Review — Pass 58

**Reviewed artifact set (frozen):** ADR-046 v1.23 + BC-4.17.001 v1.25 + BC-5.40.001 v1.20 + BC-7.07.001 v1.39
**Review date:** 2026-08-27
**Verdict:** FINDINGS (1 MED) + 2 OBS — F-P58-001, FIXED
**Gate:** BC-5.39.001 3-CLEAN spec-convergence (ADR-046 fix-state-writes)
**Streak effect:** 1/3 → **RESETS to 0/3** — the 8th reset this session
**D-chain:** D-1115

## Part A — Finding Set (frozen set: ADR-046 v1.23 + BC-4.17.001 v1.25 + BC-5.40.001 v1.20 + BC-7.07.001 v1.39)

**HIGH (0):** none this pass.

**MEDIUM (1):**

### F-P58-001 (MED, POLICY 4 internal-consistency)

BC-4.17.001's §Description ADR-046-coverage sentence and §Traceability ADR Reference row enumerated
this BC's ADR-046 coverage as "Decisions 1, 2, and 4" only — **omitting Decision 5** — despite this
same BC's own body containing multiple explicit "MIGRATED … per ADR-046 §Decision 5" annotations:

- Precondition 4 (`STATE_MD_MAX_BYTES` read-cap + mandatory `extract_frontmatter` use)
- Invariant 7 (`extract_frontmatter`-exclusive operation)
- Invariant 8 (`state_md_approaching_cap` soft-warn)
- EC-015 (`OutputTooLarge` fail-open)
- VP-TBD-7, VP-TBD-8, VP-TBD-9

All of these were migrated into BC-4.17.001 from BC-5.40.001's now-historical Precondition
6/Invariant 7/Invariant 8/EC-010/S-19.08 T-001/T-004/T-005/T-007, per ADR-046 §Decision 5's
guard-read reconciliation (originally sourced at F-P4-002, v1.4) and per ADR-046's own File-Change
Plan + Companion Amendment 1 item (vi), which directs that migration explicitly into this BC. A BC
that is the designated migration TARGET of an ADR Decision, and that carries five separate live-body
annotations to that effect, asserting its own top-level coverage summary as "1, 2, 4" is an internal
self-contradiction — the coverage sentence and the ADR-Reference traceability row are both
undercounting the BC's own documented scope.

**Ground truth:** ADR-046's `## Decision` list (flat, 6 items) — Decision 5 is "retire
`verify-state-timestamp-refresh`; migrate its read-cap/`extract_frontmatter`/soft-warn guard-read
logic into `stamp-state-timestamp` (this BC) and `precompact-flush` (BC-7.07.001)." BC-4.17.001 is a
named migration target of Decision 5, not merely an incidental cross-reference.

**Disposition:** FIXED by product-owner (BC-4.17.001 v1.25→**v1.26**):
- §Description's ADR-046-coverage sentence now states this BC covers Decision 5's guard-read/cap
  migration alongside Decisions 1/2/4.
- §Traceability ADR Reference row now lists `§Decision 5` with a short summary (migrated
  read-cap/`extract_frontmatter`/soft-warn/`OutputTooLarge` guard-read reconciliation from
  BC-5.40.001's retired `verify-state-timestamp-refresh`).

No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1).

**Classification note:** this is the same under-inclusive-ADR-Decision-enumeration defect CLASS as
O-P48-001 (a prior BC coverage-enumeration gap, previously FIXED under governance election) — not a
new discipline, an instance of an existing one, re-surfacing at a different BC/Decision pairing.

**LOW/BLOCKING (0):** no additional blocking findings.

Two LOW, non-blocking observations recorded below (O-P58-001, O-P58-002) — both adjudicated
NON-DEFECT, tracked, not counted against the streak (the streak reset is driven exclusively by
F-P58-001).

## Part B — Verified-Clean Observations (adversary-confirmed, no additional blocking findings)

Fresh-context adversary review, information-asymmetric per the Iron Law (no visibility into
prior-pass narrative, decision-log, lessons.md, or burst-log — reviewed the frozen artifact set on
its own merits only). Beyond F-P58-001, this pass re-applied all seventeen previously-codified
convergence-technique disciplines and found no regression:

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
  remains ACCEPTED-tracked, untouched.
- **BC-4.17.001 EC-011 `holder: null` documentation asymmetry (O-P57-001):** re-examined — remains
  ACCEPTED-tracked, untouched by this pass (different locus from F-P58-001).

### O-P58-001 (LOW, provenance-ID cross-check, adversary-adjudicated NON-DEFECT)

The adversary independently re-derived the provenance-ID split between F-P27-001 (cited at BC-5.40.001
and BC-7.07.001's §Story Anchor sections) and F-P25-002 (cited at all three companion BCs'
§Traceability §Stories rows), and considered whether this split — combined with BC-4.17.001 citing
**only** F-P25-002 at both loci (§Traceability AND §Story Anchor), rather than the same F-P27-001/
F-P25-002 split its siblings use — is a defect. **Ruled NOT a defect: this is CORRECT provenance.**
F-P25-002 (pass-25) is the origin fix that first resolved the `[pending]` S-17.05 placeholder at
§Traceability across all three BCs. F-P27-001 (pass-27) was a §Story-Anchor-specific sibling-sweep
fix applied only to BC-5.40.001 and BC-7.07.001, because BC-4.17.001's own pass-25 fix had already
touched BOTH its §Traceability row AND its §Story Anchor section in the same burst — there was no
separate §Story-Anchor gap left in BC-4.17.001 for a pass-27 fix to close. BC-4.17.001 citing
F-P25-002 at both loci therefore correctly reflects its own distinct fix history, not an inconsistency
with its siblings.

**Disposition:** CONFIRMED NON-DEFECT. No edit. Tracked as an accepted-correct-provenance item so
pass-59 (and any future pass) does not re-raise this as a fresh finding.

### O-P58-002 (LOW, lifecycle-status cross-check, NON-DEFECT)

BC-4.17.001's `status: draft` (frontmatter) and `lifecycle_status: draft` (frontmatter) fields were
cross-checked against each other and against POL-14 auto-promotion criteria (draft→active on
implementing-story PR merge). Both fields agree (draft/draft) and S-17.05 has not yet merged an
implementing PR, so `draft` is the correct value for both fields. **NON-DEFECT** — noted only, no
edit.

## Part C — State at Close of Review

ADR-046 **v1.23, UNCHANGED**. BC-4.17.001 **v1.25→v1.26** (F-P58-001 fix). BC-5.40.001 **v1.20,
UNCHANGED**. BC-7.07.001 **v1.39, UNCHANGED**. **BC-5.39.001 3-CLEAN streak: 1/3 → RESETS to 0/3** —
the 8th reset this session. Gate history to date: 58 passes run against evolving/frozen sets; 44
genuine BLOCKING findings found and fixed (43 prior + F-P58-001), plus 10 audit-extra stragglers, 1
latent-bracket drain (pass-37, not counted as genuine), 3 ACCEPTED non-blocking observations carried
forward unchanged (O-P42-001, O-P53-DESC-NOOP, O-P57-001) plus 2 NEW ACCEPTED non-blocking
observations this pass (O-P58-001, O-P58-002), and 3 FIXED non-blocking observations (O-P44-001,
O-P48-001, O-P51-001). The behavioral core (write-composition, identity-gating, event-sourcing)
remains independently re-verified CLEAN for the 31st consecutive pass (since pass-27) — this pass's
finding is confined entirely to the ADR-Decision-coverage-enumeration perimeter, not the behavioral
core.

**Index reconciliation:** BC-INDEX v5.16→v5.17 (BC-4.17.001 row version-cell + Changelog cross-ref).
ARCH-INDEX, STORY-INDEX, VP-INDEX all UNCHANGED (only BC-4.17.001 touched this pass).

**Input-hash recompute:** BC-4.17.001 `b7f7213`→`6b0b35c` (cyclic-hash TD [D-1082], settled +
cross-referenced, NOT reopened). ADR-046 `3335ad4`, BC-5.40.001 `a21ce60`, BC-7.07.001 `e73bc01`
remain unchanged (not edited this pass).

**Process note:** the product-owner turn implementing the F-P58-001 fix dropped mid-edit on an API
loss and was resumed to completion by a fresh product-owner dispatch. The resumed turn re-verified the
partial edit state on disk before continuing; final state (BC-4.17.001 v1.26, §Description +
§Traceability both corrected, Changelog row + `modified:` entry both present) was confirmed complete
before this record was persisted. Non-blocking — noted for the record only.

**NEXT: fresh pass-59** against the pass-58-corrected frozen set (ADR-046 v1.23 + **BC-4.17.001
v1.26** + BC-5.40.001 v1.20 + BC-7.07.001 v1.39), applying all seventeen codified convergence-technique
disciplines proactively from the start, plus the F-P58-001 under-inclusive-ADR-Decision-enumeration
class now folded into the standing discipline set. Streak restarts at 0/3 — 3 consecutive clean passes
(59, 60, 61) needed for literal BC-5.39.001 3-CLEAN. The human decision this session remains to
CONTINUE looping toward literal 3-CLEAN (not accept D-386 Option C asymptotic acceptance) —
accept-provisional under D-386 Option C remains available at any time. S-17.05 TDD implementation
remains gated on convergence.
