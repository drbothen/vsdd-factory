# ADR-046 Adversarial Spec-Convergence Review — Pass 28

**Reviewed artifact set (frozen):** ADR-046 v1.12 + BC-4.17.001 v1.12 + BC-7.07.001 v1.29 + BC-5.40.001 v1.11
**Review date:** 2026-08-26
**Verdict:** FINDINGS (2: 1 HIGH, 1 MED), 2 LOW observations
**Gate:** BC-5.39.001 3-CLEAN spec-convergence (ADR-046 fix-state-writes)
**Streak effect:** 0/3 → **REMAINS 0/3** (already reset at pass-25; a finding does not reset an already-0/3 streak further)
**D-chain:** D-1085

## Part A — Finding Set (frozen set: ADR-046 v1.12 + BC-4.17.001 v1.12 + BC-7.07.001 v1.29 + BC-5.40.001 v1.11)

**HIGH (1):**

- **F-P28-001 (HIGH, POLICY 18)** — Root cause: the pass-27 (D-1084) fixes landed on BC-7.07.001
  without sweeping siblings, creating both an inputs-omission straggler AND two FALSE recorded
  premises (this finding covers the omission; F-P28-002 below covers one of the false premises).
  **(a) ADR-046 + BC-4.17.001 `inputs:` omission.** Neither ADR-046's nor BC-4.17.001's `inputs:`
  frontmatter array cited `crates/factory-lock-parse/src/lib.rs`, despite both artifacts making
  heavily load-bearing claims against that crate: ADR-046's own F-P25-001/v1.11 correction rests
  entirely on `flp::parse_factory_lock`'s `LockState` return type; BC-4.17.001's Precondition 4
  cites `factory_lock_parse::STATE_MD_MAX_BYTES` as "the single canonical declaration" and mandates
  `factory_lock_parse::extract_frontmatter` use, and VP-TBD-7/VP-TBD-8/§Architecture Anchors all
  cite the crate directly. **(b) BC-7.07.001 v1.29 false cross-reference.** BC-7.07.001's own
  F-P27-003/v1.29 disposition falsely claimed its `inputs:` addition of the same file was "mirroring
  sibling BC-4.17.001's input set" — BC-4.17.001's `inputs:` array did NOT contain the file at that
  time (per (a) above); the claim was false when written.
  **Disposition: FIXED.** Architect added `crates/factory-lock-parse/src/lib.rs` to ADR-046's
  `inputs:` (a sanity sweep of the rest of ADR-046's `inputs:` against every crate/file it makes
  concrete claims against found one further genuine omission, `.factory/specs/behavioral-
  contracts/ss-07/BC-7.07.001.md`, already named in ADR-046's own Source/Origin § alongside three
  sibling BCs that ARE in `inputs:` — added for the same reason). Product-owner independently added
  the same crate to BC-4.17.001's `inputs:`, justified against BC-4.17.001's own load-bearing
  claims (NOT derivative of BC-7.07.001's false mirroring claim). Product-owner corrected
  BC-7.07.001's v1.29 disposition text IN PLACE (folded into the v1.30 bump rather than a separate
  erratum row, since a full version bump was already warranted by F-P28-002 in the same finding
  set) to cite POLICY 18's own requirement directly, not the false BC-4.17.001-mirroring premise.
  ADR-046 v1.12→v1.13; BC-4.17.001 v1.12→v1.13; BC-7.07.001 v1.29→v1.30 (values unchanged from
  v1.29, prose-only correction for this leg).

**MEDIUM (1):**

- **F-P28-002 (MED, POLICY 17/4)** — BC-7.07.001's own F-P27-002/v1.29 status-flip rationale falsely
  stated that "sibling BC-4.17.001/BC-5.40.001 both carry `status: active` + `lifecycle_status:
  active`" — FALSE for BC-4.17.001, which is correctly `status: draft` + `lifecycle_status: draft`
  because its own base deliverable (the `stamp-state-timestamp` hook, story S-17.05) has not
  shipped. BC-4.17.001 and BC-7.07.001 are asymmetric on this axis, not parallel: BC-7.07.001's own
  base (`precompact-flush`) DID ship via S-18.04a — that is the actual, sufficient reason
  BC-7.07.001's `status: active` is correct, independent of any BC-4.17.001 comparison.
  **Disposition: FIXED.** Product-owner corrected the v1.29 disposition text IN PLACE: it now
  stands on BC-7.07.001's own shipped-base-contract grounds, cites BC-5.40.001 ALONE as the
  active-sibling-parity precedent, and explicitly notes BC-4.17.001's draft status is correct and
  unaffected — not implied to need flipping to active. Neither F-P28-001(b) nor this correction
  changes BC-7.07.001's own `status`/`lifecycle_status`/`inputs:` values — all remain exactly as
  v1.29 set them; only the DISPOSITION PROSE justifying them is corrected. BC-7.07.001 v1.29→v1.30
  (same version bump as F-P28-001(b), folded together per POLICY 14's erratum convention).

**LOW (2, non-blocking observations):**

- **O-P28-001 (LOW)** — A stale `FactoryLock` type-name cite (superseded by the F-P25-001/D-1082
  `LockState` correction) survives in PRESERVED HISTORICAL dated changelog entries only (pre-F-P25-001
  rows in ADR-046's and BC-7.07.001's `last_amended` chains); the live body text is correct and
  consistent across all three artifacts. **Disposition: NO FIX NEEDED — accepted per convention.**
  Historical dated changelog rows are immutable audit trail by this repo's standing convention
  (rewriting them to reflect later corrections would falsify the historical record of what each
  revision actually said at the time); this is the same treatment already applied to other
  historical-preservation items in STATE.md's Drift Items table. Left untouched.

- **O-P28-002 (LOW, `[process-gap]`, 3+ RECURRENCE)** — ADR-046's own File-Change Plan carried a
  self-referential version-bump DIRECTIVE (a row instructing ARCH-INDEX what version to cite for
  this ADR) that hard-codes its target version as a literal number — a construction structurally
  guaranteed to go stale on every subsequent ADR revision that does not also remember to edit this
  row. This is the THIRD occurrence of this exact failure mode: F-P25-002 (D-1082) added a new
  File-Change Plan row without sweeping this directive forward; F-P26-001 (D-1083) caught and
  rewrote a stale "v1.10" straggler to "v1.12"; this pass catches it a third time (would again go
  stale the moment this very burst's v1.13 bump lands, if left in literal-directive form).
  **Disposition: ROOT-CAUSE FIXED (not symptomatically patched again).** Architect restructured the
  row to a version-stable instruction: state-manager now reads ADR-046's current frontmatter
  `version:` field at bump time rather than the row embedding a literal number, so the directive
  cannot again fall one revision behind. The historical "Prior (vX.Y) reflected..." disposition
  chain in that row is preserved as-is (legitimate dated history); only the live directive clause
  and its immediately-following summary sentence were restructured. A sweep of the rest of the
  File-Change Plan for other self-version directives found no other locus. This CODIFIES the fix —
  the recurrence class is now structurally prevented, not merely patched a third time. Lesson
  recorded `[codified]` in `lessons.md`.

## Part B — Verified-Clean Observations (adversary-confirmed, no findings)

- **§Story Anchor / Traceability parity (F-P27-001 class):** re-verified clean across all three
  BCs this pass — no regression of the pass-27 fix.
- **BC-5.40.001 (unchanged this pass):** its §Story Anchor (Tri-story anchor, corrected D-1084) and
  Traceability §Stories row remain consistent; no regression, no new sibling-sweep gap.
- **Type-provenance (F-P25-001 class) and event-sourcing struct-variant text:** re-verified clean
  across all three BCs; no regression.
- **POLICY 19 (anti-volatile-pin):** no new load-bearing inline `ADR-046 vN.N` / `BC-X.YY.NNN vN.N`
  version-pin token found in any normative body prose across the frozen set.
- **Anchors, subsystem names, registry facts:** every `§Decision N` cross-reference, every
  `SS-04`/`SS-05`/`SS-07` subsystem assignment, and every BC/story ID cited resolves to a real,
  correctly-named target.

This pass's finding cluster is entirely the pass-27-fix-introduced straggler/false-premise class:
a fix landing on 1 of N siblings carrying an identical claim (BC-7.07.001's F-P27-002/F-P27-003
dispositions) not only left the sibling gap (F-P28-001(a)) but injected two FALSE cross-references
into its own disposition text about siblings it did not actually check (F-P28-001(b), F-P28-002).
No new independent defect class was discovered this pass beyond the recurring self-referential-
directive class (O-P28-002), which is now root-cause closed.

## Part C — State at Close of Review

ADR-046 **v1.13** (`inputs:` completed — factory-lock-parse crate + BC-7.07.001.md added; File-
Change Plan directive restructured version-stable, O-P28-002 root-cause fix); BC-4.17.001 **v1.13**
(`inputs:` completed — factory-lock-parse crate added, independently justified); BC-7.07.001
**v1.30** (F-P28-001(b) false-mirroring-claim + F-P28-002 false-parallel-claim both corrected IN
PLACE, values unchanged from v1.29); BC-5.40.001 **v1.11** (unchanged this pass). BC-5.39.001
3-CLEAN streak: **0/3** (REMAINS — already reset at pass-25; this pass's findings do not reset an
already-0/3 streak further). BC-4.17.001 ↔ BC-7.07.001 mutual `inputs:` cyclic-hash TD now EXTENDS
to a 3-way cycle including ADR-046 (ADR-046 now cites BC-7.07.001.md in its own `inputs:`, and both
companion BCs cite ADR-046.md in theirs) — reconfirmed non-convergent, settled per this pass's task
instruction, cross-referenced against the existing `[D-1082]` Drift Item, NOT re-opened as a new
item. Gate history to date: 28 passes run against evolving/frozen sets; 12 genuine findings found
and fixed prior to this pass (F-P10-001/F-P13-001/F-P15-001/F-P18-001 HIGH, F-P21-001/F-P23-001/
F-P25-001/F-P25-002/F-P26-001/F-P27-002/F-P27-003 MED, F-P27-001 HIGH); this pass adds F-P28-001
(HIGH, fixed) + F-P28-002 (MED, fixed) — 14 genuine findings fixed across 28 passes total, plus
this pass's O-P28-001 (LOW, accepted-per-convention, no fix needed) and O-P28-002 (LOW,
`[process-gap]`, root-cause fixed/codified this burst).
**NEXT: fresh pass-29** against the newly-frozen set (ADR-046 v1.13 + BC-4.17.001 v1.13 +
BC-7.07.001 v1.30 + BC-5.40.001 v1.11); needs 3 consecutive clean passes (29, 30, 31) for literal
3-CLEAN convergence. S-17.05 TDD implementation remains gated on convergence.
