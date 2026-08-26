# ADR-046 Adversarial Spec-Convergence Review — Pass 27

**Reviewed artifact set (frozen):** ADR-046 v1.12 + BC-4.17.001 v1.12 + BC-7.07.001 v1.28 + BC-5.40.001 v1.10
**Review date:** 2026-08-26
**Verdict:** FINDINGS (3: 1 HIGH, 2 MED), 1 LOW observation
**Gate:** BC-5.39.001 3-CLEAN spec-convergence (ADR-046 fix-state-writes)
**Streak effect:** 0/3 → **REMAINS 0/3** (already reset at pass-25; a finding does not reset an already-0/3 streak further)
**D-chain:** D-1084

## Part A — Finding Set (frozen set: ADR-046 v1.12 + BC-4.17.001 v1.12 + BC-7.07.001 v1.28 + BC-5.40.001 v1.10)

**HIGH (1):**

- **F-P27-001 (HIGH, POLICY 4)** — The v1.10 (BC-5.40.001) / v1.28 (BC-7.07.001) fixes at pass-25
  (F-P25-002) resolved the Traceability §Stories row's `[pending]` placeholder to S-17.05 in BOTH
  companion BCs, but did not sweep the sibling §Story Anchor section in either file. BC-5.40.001's
  §Story Anchor still read "Dual-story anchor: S-17.01; S-19.08" — omitting S-17.05 entirely and
  carrying a now-incorrect cardinality quantifier ("Dual" when a third story is now confirmed).
  BC-7.07.001's §Story Anchor still read only "S-18.04a" — omitting S-17.05 entirely (this BC's
  §Story Anchor never used a cardinality quantifier, so only the omission applies, not a stale
  count word). This is a TD-VSDD-060-class sibling-sweep gap: resolving a `[pending]` implementing-
  story anchor to a real story ID in the Traceability §Stories row is not sufficient — the SAME
  story ID must be swept into every OTHER locus that anchors implementing stories, including any
  cardinality-quantifier prose ("Dual-story anchor", "Tri-story anchor") that would otherwise
  silently contradict the corrected story count.
  **Disposition: FIXED.** Product-owner corrected BC-5.40.001's §Story Anchor to "Tri-story anchor:
  S-17.01; S-19.08; S-17.05" (quantifier word corrected to match the now-three-story count) and
  BC-7.07.001's §Story Anchor to list both S-18.04a and S-17.05. Both BCs bumped: BC-5.40.001
  v1.10→v1.11, BC-7.07.001 v1.28→v1.29.

**MEDIUM (2):**

- **F-P27-002 (MED, POLICY 17)** — BC-7.07.001's frontmatter carried `status: draft` while
  `lifecycle_status: active` and the BC-INDEX status cell already read `active` — a same-file
  status/lifecycle-status contradiction plus an index/file divergence. Adjudicated: the
  precompact-flush plugin this BC governs has shipped (S-18.04a, E-18 EPIC COMPLETE); BC-4.17.001
  and BC-5.40.001 both carry the identical pending-S-17.05-amendment condition under `status:
  active` (spec-leading-code per the VSDD standing rule, not draft); a pending amendment does not
  make an already-shipped base contract draft. **Disposition: FIXED.** `status: draft` → `status:
  active`, reconciling the file to what BC-INDEX already stated. Not escalated to architect — this
  is a mechanical sibling-parity + lifecycle-consistency adjudication answerable in scope per the
  CANONICAL PRINCIPLE (no "pending architect review" deferral warranted).

- **F-P27-003 (MED, POLICY 18)** — BC-7.07.001's `inputs:` frontmatter list was incomplete relative
  to what its own normative body prose depends on: the LOCK_RENEWAL_TTL_SECS / `parse_iso8601` /
  `flp::parse_factory_lock` code claims, Precondition 1's registry stanza, and EC-004's
  BC-4.13.001-EC-009 alignment all cite files never listed in `inputs:`. **Disposition: FIXED.**
  `inputs:` expanded with `.factory/specs/behavioral-contracts/ss-04/BC-4.13.001.md`,
  `crates/factory-lock/src/lib.rs`, `crates/factory-lock-parse/src/lib.rs`,
  `crates/hook-plugins/precompact-flush/src/lib.rs`,
  `crates/hook-plugins/verify-factory-lock/src/lib.rs`, and
  `plugins/vsdd-factory/hooks-registry.toml` — mirroring sibling BC-4.17.001's already-complete
  input set for the same code surface. BC-4.17.001 itself retained UNCHANGED (its mutual
  `inputs:` cite of BC-7.07.001 is the existing, already-settled cyclic-hash TD — see Part C).

**LOW (1, non-blocking observation, no substantive fix needed — cosmetic):**

- **O-P27-001 (LOW)** — BC-7.07.001's `modified:` changelog array had a mis-ordered block: the
  v1.19–v1.23 entries were interleaved out of strict descending-chronological order (a residue of a
  prior burst's append pattern), landing after v1.24 instead of before it.
  **Disposition: FIXED (cosmetic).** Product-owner reordered the array into strict
  descending-chronological sequence. No content changed, only array element order.

## Part B — Verified-Clean Observations (adversary-confirmed, no findings)

- **ADR-046 v1.12 itself:** UNCHANGED and clean this pass — no findings against the ADR document.
  The pass-26 self-referential ARCH-INDEX sync-instruction-row fix (F-P26-001) holds; no regression
  found.
- **§Story Anchor / Traceability parity, all OTHER loci:** every other cross-reference to S-17.01,
  S-19.08, S-17.05, and S-18.04a across the frozen set (Related BCs sections, Architecture Anchors,
  Canonical Test Vectors provenance notes) resolves consistently — F-P27-001 was isolated to the
  two §Story Anchor headings, not a wider class of story-ID drift.
- **BC-4.17.001 (unchanged this pass):** its own §Story Anchor and Traceability §Stories rows
  (already corrected at pass-25/D-1082 for this same S-17.05 class) remain consistent — no
  regression, no sibling-sweep gap found in this file this pass.
- **Type-provenance (F-P25-001 class) and event-sourcing struct-variant text:** re-verified clean
  across all three BCs; no regression.
- **POLICY 19 (anti-volatile-pin):** no new load-bearing inline `ADR-046 vN.N` / `BC-X.YY.NNN vN.N`
  version-pin token found in any normative body prose across the frozen set.
- **Anchors, subsystem names, registry facts:** every `§Decision N` cross-reference, every
  `SS-04`/`SS-05`/`SS-07` subsystem assignment, and every BC/story ID cited resolves to a real,
  correctly-named target.

This pass's finding cluster is entirely the S-17.05-retrofit sibling-sweep class: every finding
traces back to the SAME root event (the pass-25 `[pending]`→S-17.05 resolution) not having swept
every locus that anchors implementing stories or every parity leg the resolution touched
(cardinality-quantifier prose, status/lifecycle consistency, inputs completeness, changelog
ordering). No new independent defect class was discovered this pass.

## Part C — State at Close of Review

BC-5.40.001 **v1.11** (§Story Anchor Tri-story anchor corrected); BC-7.07.001 **v1.29**
(§Story Anchor + status:active + inputs: completed + modified[] reordered); ADR-046 **v1.12**
(unchanged this pass); BC-4.17.001 **v1.12** (unchanged this pass — retained; the BC-4.17.001 ↔
BC-7.07.001 mutual `inputs:` cite cyclic-hash TD is RECONFIRMED non-convergent (BC-4.17.001's
stored input-hash `407e0ff` is now one round behind the freshly-recomputed value, reflecting both
BC-5.40.001 v1.11's and BC-7.07.001 v1.29's new content) — this is the SAME class of expected
residue already settled and cross-referenced at the existing `[D-1082]` Drift Item in STATE.md, NOT
re-opened as a new Drift Item, consistent with this pass's task instruction to settle rather than
reopen. BC-5.39.001 3-CLEAN streak: **0/3** (REMAINS — already reset at pass-25; this pass's
findings do not reset an already-0/3 streak further). Gate history to date: 27 passes run against
evolving/frozen sets; 9 genuine findings found and fixed prior to this pass (F-P10-001/F-P13-001/
F-P15-001/F-P18-001 HIGH, F-P21-001/F-P23-001/F-P25-001/F-P25-002/F-P26-001 MED); this pass adds
F-P27-001 (HIGH, fixed) + F-P27-002/F-P27-003 (MED, both fixed) — 12 genuine findings fixed across
27 passes total, plus this pass's 1 non-blocking LOW cosmetic fix (O-P27-001).
**NEXT: fresh pass-28** against the newly-frozen set (ADR-046 v1.12 + BC-4.17.001 v1.12 +
BC-7.07.001 v1.29 + BC-5.40.001 v1.11); needs 3 consecutive clean passes (28, 29, 30) for literal
3-CLEAN convergence. S-17.05 TDD implementation remains gated on convergence.
