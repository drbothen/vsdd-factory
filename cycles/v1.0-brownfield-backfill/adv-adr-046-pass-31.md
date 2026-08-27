# ADR-046 Adversarial Spec-Convergence Review — Pass 31

**Reviewed artifact set (frozen):** ADR-046 v1.15 + BC-4.17.001 v1.15 + BC-7.07.001 v1.31 + BC-5.40.001 v1.13
**Review date:** 2026-08-26
**Verdict:** FINDINGS (2 MED), 0 HIGH, 0 LOW observations
**Gate:** BC-5.39.001 3-CLEAN spec-convergence (ADR-046 fix-state-writes)
**Streak effect:** 0/3 → **REMAINS 0/3** (already reset at pass-25; a finding does not reset an already-0/3 streak further)
**D-chain:** D-1088

## Part A — Finding Set (frozen set: ADR-046 v1.15 + BC-4.17.001 v1.15 + BC-7.07.001 v1.31 + BC-5.40.001 v1.13)

**HIGH (0):** none this pass.

**MEDIUM (2):**

- **F-P31-001 (MED, POLICY 18, `inputs:` completeness)** — BC-5.40.001's `inputs:` frontmatter array
  omitted BC-4.13.001 and BC-6.23.001 despite this BC's own body citing both as load-bearing
  current-state authorities: BC-4.13.001 (PC2 `is_expired`/`LockExpired` TTL-boundary comparison in
  PC3; PC1's `SchemaViolation` error-variant cite to BC-4.13.001's malformed-block fail-open path,
  which is BC-4.13.001 PC4; PC6's self-held-`Continue` cite, BC-4.13.001 PC3; Invariant 6's
  malformed-block-unlocked cite, BC-4.13.001 PC4; Invariant 8's soft-warn-threshold adjudication
  cite, BC-4.13.001 Invariant 10) and BC-6.23.001 (PC1's acquire-writes-the-block cite; PC2/
  Precondition-4's unlock-clears-the-block cites). Both sibling BCs (BC-4.17.001, BC-7.07.001) and
  ADR-046 already list both files in their own `inputs:` arrays — BC-5.40.001 was never itself
  swept for this specific pair.
  **Disposition: FIXED.** Product-owner added `.factory/specs/behavioral-contracts/ss-04/
  BC-4.13.001.md` and `.factory/specs/behavioral-contracts/ss-06/BC-6.23.001.md` to `inputs:`, same
  path form the sibling BCs already use. This is NOT the accepted BC-4.17.001↔BC-7.07.001↔ADR-046↔
  BC-5.40.001 mutual-inputs cyclic-hash TD tracked since `[D-1082]` — BC-4.13.001 and BC-6.23.001
  are outside that mutual set. BC-5.40.001 v1.13→v1.14.

- **F-P31-002 (MED, POLICY 4, cross-reference accuracy)** — BC-7.07.001's Postcondition 3 (Shared-
  classifier mandate paragraph) cited "matches BC-5.40.001 §Invariant 2" for the
  `YYYY-MM-DDTHH:MM:SSZ` `factory_lock.expires_at` timestamp-format requirement. Verified against
  BC-5.40.001's actual section content: §Invariant 2 ("Default TTL is 45 minutes (2700 seconds)")
  governs the 2700-second TTL VALUE, not the timestamp string format; the format requirement is
  actually stated in BC-5.40.001 §Precondition 3 ("locked_at and expires_at MUST be ISO-8601 UTC
  timestamps (format: YYYY-MM-DDTHH:MM:SSZ)") and restated at BC-5.40.001 PC1.
  **Disposition: FIXED.** Product-owner retargeted the citation from `BC-5.40.001 §Invariant 2` to
  `BC-5.40.001 §Precondition 3`. BC-7.07.001 v1.31→v1.32.

**LOW (0):** none this pass.

## Part A-extra — Audit-Discovered Stragglers (found and fixed same-burst, in-scope per production-grade default; not part of the adversary's flagged 2-finding set, surfaced by the comprehensive cross-anchor/spec-inputs audits the remediation ran in response to F-P31-001/F-P31-002)

- **BC-5.40.001 own-body cross-anchor straggler (found during the comprehensive cross-anchor audit
  run to close F-P31-001):** BC-5.40.001's own Precondition 4 ("When the factory is unlocked... a
  `state-manager` removes the block") and Postcondition 2 both cited "BC-6.23.001 PC3/PC4" as the
  authority for `/factory-unlock` clearing behavior. Verified against BC-6.23.001's actual section
  content: PC3 is "`/factory-lock` foreign lock held: refuse" — an ACQUIRE-path refusal outcome,
  unrelated to `/factory-unlock`. The self-release clearing act BC-5.40.001 describes is BC-6.23.001
  PC4 alone. **FIXED:** both occurrences corrected from "BC-6.23.001 PC3/PC4" to "BC-6.23.001 PC4".
  No PC/Invariant/EC renumbered (append-only numbering preserved — POLICY 1).

- **BC-7.07.001 spec-inputs completeness straggler (found during the comprehensive spec-inputs
  completeness audit run to close F-P31-002):** BC-7.07.001's body makes load-bearing current-state
  claims against five spec files absent from its own `inputs:` array — BC-5.40.001 (the
  just-corrected §Precondition 3 format cite, plus the Related-BCs "depends on" relationship),
  BC-5.41.003 (Invariant 4's/Postcondition 8's `MULTI_COMMIT_CHAIN_NOT_ALLOWED` exemption-mechanism
  and false-positive-block claims), BC-1.15.001 (Precondition 3's dispatcher-routes-PreCompact-
  events/postconditions-satisfied claim and Related BCs cite), BC-2.02.011 (a body cross-reference
  making a specific current-behavior claim), and `.factory/specs/domain-spec/invariants.md` (a
  domain-invariant claim the body cites by content). **FIXED:** all five added to `inputs:`.

Both stragglers are the SAME root pattern the D-1087 convergence-strategy lesson identified:
single-locus spot-fixing of only the adversary-flagged item leaves siblings/adjacent claims
un-swept. This pass's remediation deliberately extended the comprehensive-per-dimension-sweep
technique from D-1087 to a THIRD dimension — not just "sweep every sibling BC sharing the same
defect class" (D-1087's technique) but "open and verify every cross-anchor citation and every
spec-inputs claim inside the SAME BC the flagged finding already touched" — and caught 3 additional
genuine defects the 2 flagged findings alone would have left for pass-32/33 to discover.

## Part B — Verified-Clean Observations (adversary-confirmed, no findings)

- **Behavioral core (write-composition table, five-outcome table, identity-gating logic,
  event-sourcing struct-variant text):** re-verified clean across all four artifacts this pass — no
  regression of any prior-pass fix.
- **F-P30-001-class array-ordering parity (BC-4.17.001, BC-5.40.001, BC-7.07.001 `modified:` vs
  Changelog):** re-verified clean across all three BCs; O-P28-002's version-stable ARCH-INDEX
  directive held, no re-patch required.
- **Type-provenance (F-P25-001 class — `LockState` vs `FactoryLock`):** re-verified clean; no
  regression.
- **POLICY 19 (anti-volatile-pin):** no new load-bearing inline `ADR-046 vN.N` / `BC-X.YY.NNN vN.N`
  version-pin token found in any normative body prose across the frozen set.
- **§Story Anchor / Traceability parity (F-P27-001 class):** re-verified clean across all three
  BCs; no regression.
- **Audit-A — cross-anchor semantic verification:** 23 `BC-X.YY.ZZZ §Section`/`PCn`/`Invariant-N`
  cross-references opened and checked against the cited BC's actual section content across the
  frozen set (BC-4.17.001, BC-5.40.001, BC-7.07.001 bodies). 21 of 23 confirmed CORRECT
  (BC-4.13.001's 6 EC-009 cites in BC-7.07.001 all confirmed correct); 2 confirmed WRONG
  (BC-5.40.001's two "BC-6.23.001 PC3/PC4" unlock-cites, both corrected to PC4-only — see Part
  A-extra) and 1 confirmed WRONG independently as F-P31-002 (BC-7.07.001's "BC-5.40.001 §Invariant
  2" mis-cite, corrected to §Precondition 3) — no further wrong-section anchors found.
- **Audit-B — spec-inputs completeness verification:** every ADR-046-companion BC's `inputs:` array
  checked against its own body's load-bearing citations. BC-5.40.001 found incomplete (F-P31-001,
  +2 files); BC-7.07.001 found incomplete (Part A-extra, +5 files); BC-4.17.001 confirmed already
  complete, no edit required.

**No spec-vs-code contradictions found this pass.** Both flagged findings (F-P31-001, F-P31-002)
and both audit-extra stragglers are pure cross-reference/frontmatter integrity defects — BC-to-BC
citation accuracy and `inputs:` array completeness. Neither touches this ADR/BC cluster's actual
behavioral contract text (write-composition, identity-gating, event-sourcing, TTL semantics all
re-verified clean and unchanged).

**Novelty assessment:** the substantive behavioral spec for this ADR/BC cluster has converged. The
remaining defect surface, five passes running (27 through 31), is entirely cross-reference and
frontmatter integrity — sibling-sweep gaps, array-ordering parity, `inputs:` completeness,
cross-anchor citation accuracy — never logic or spec-vs-code contradiction. This pass is a direct
confirmation, not a refutation, of the D-1087 convergence-strategy hypothesis: the comprehensive
per-dimension-sweep technique, applied THIS pass to a broader scope (cross-anchor + spec-inputs
audits, not just sibling-BC array-ordering), is what surfaced and closed 3 of this pass's 5 total
genuine defects (the 2 audit-extra stragglers) BEFORE a future pass would have had to find them
piecemeal. The 2 originally-flagged findings (F-P31-001, F-P31-002) demonstrate the technique alone
does not yet reach a literal-CLEAN pass by itself — comprehensive audits still surface genuine
residual defects on first application to a given dimension (this is the FIRST pass a full
cross-anchor semantic audit and a full spec-inputs completeness audit were run against BC-7.07.001
and BC-5.40.001 simultaneously) — but the technique's yield (3 extra defects caught same-burst that
a spot-fix would have left for 2-3 more passes) is the accelerant this gate needs to reach 3-CLEAN.

## Part C — State at Close of Review

BC-5.40.001 **v1.14** (`inputs:` +2 files F-P31-001; BC-6.23.001 PC3/PC4→PC4-only cross-anchor
correction, audit-extra); BC-7.07.001 **v1.32** (PC3 cross-reference retarget F-P31-002; `inputs:`
+5 files, audit-extra). ADR-046 **UNCHANGED at v1.15**; BC-4.17.001 **UNCHANGED at v1.15**
(cluster-audited, confirmed clean, no edit). BC-5.39.001 3-CLEAN streak: **0/3** (REMAINS — already
reset at pass-25; this pass's findings do not reset an already-0/3 streak further). Gate history to
date: 31 passes run against evolving/frozen sets; 21 genuine findings found and fixed prior to this
pass; this pass adds F-P31-001 (MED, fixed) + F-P31-002 (MED, fixed) + 2 audit-extra stragglers
(fixed) — 25 genuine findings/stragglers fixed across 31 passes total, zero HIGH and zero LOW
observations this pass.

**NEXT: fresh pass-32** against the newly-frozen set (ADR-046 v1.15 + BC-4.17.001 v1.15 +
BC-5.40.001 v1.14 + BC-7.07.001 v1.32); needs 3 consecutive clean passes (32, 33, 34) for literal
3-CLEAN convergence. S-17.05 TDD implementation remains gated on convergence. The human decision
this session remains to CONTINUE looping toward literal 3-CLEAN convergence (not accept-provisional
under D-386 Option C asymptotic acceptance).
