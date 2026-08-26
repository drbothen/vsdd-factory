# ADR-046 Adversarial Spec-Convergence Review — Pass 30

**Reviewed artifact set (frozen):** ADR-046 v1.14 + BC-4.17.001 v1.14 + BC-7.07.001 v1.31 + BC-5.40.001 v1.12
**Review date:** 2026-08-26
**Verdict:** FINDINGS (2: 1 HIGH, 1 MED), 0 LOW observations
**Gate:** BC-5.39.001 3-CLEAN spec-convergence (ADR-046 fix-state-writes)
**Streak effect:** 0/3 → **REMAINS 0/3** (already reset at pass-25; a finding does not reset an already-0/3 streak further)
**D-chain:** D-1087

## Part A — Finding Set (frozen set: ADR-046 v1.14 + BC-4.17.001 v1.14 + BC-7.07.001 v1.31 + BC-5.40.001 v1.12)

**HIGH (1):**

- **F-P30-001 (HIGH, POLICY 14/17, `modified:`/Changelog array-ordering parity)** — BC-4.17.001's
  and BC-5.40.001's `modified:` frontmatter arrays were both ordered ASCENDING (oldest entry at the
  top, newest at the bottom) while their own `## Changelog` tables below were correctly ordered
  DESCENDING (newest row at the top) — a POLICY 14 parity mismatch between the two
  required-to-agree legs. F-P29-003 (pass 29) fixed the identical defect class on sibling
  BC-7.07.001 but the fix was never swept to these two siblings, which carried the same defect the
  whole time (not a new regression — a pre-existing, previously-undetected instance of the same
  class).
  **Disposition: FIXED.** Product-owner reordered BOTH arrays to strict descending-chronological
  (newest at top), matching each BC's own Changelog table. A full 3-BC cluster parity audit
  (version / Changelog-head / modified-head / last_amended-prefix parity, `inputs:` completeness,
  §Story-Anchor↔§Traceability-§Stories cardinality) was run across BC-4.17.001, BC-5.40.001, and
  BC-7.07.001 — BC-7.07.001 confirmed already clean on all five legs (fixed at its own v1.31,
  F-P29-003), no edit required there. Dated HISTORICAL entry text (unchanged in both BCs) — only
  array position corrected, per POLICY 1 append-only numbering. BC-4.17.001 v1.14→v1.15;
  BC-5.40.001 v1.12→v1.13.

**MEDIUM (1):**

- **F-P30-002 (MED, POLICY 18, `inputs:` completeness)** — ADR-046's own `inputs:` frontmatter
  array omitted 6 load-bearing files despite this ADR making exact current-state claims against
  them: `crates/factory-dispatcher/src/invoke.rs` and
  `crates/factory-dispatcher/src/host/exec_subprocess.rs` (§Decision 2/F-005's config-scope-
  equivalence claim and §Context's WASI-clock claim), `plugins/vsdd-factory/tests/verify-state-
  timestamp-refresh.bats` and `plugins/vsdd-factory/tests/validate-state-structure/pass-real-state-
  md-snapshot.bats` (both make specific current-behavior claims in Consequences/Negative and
  Source/Origin "Tests requiring rewrite"), `.factory/stories/S-17.05-stamp-state-timestamp-
  hook.md` (the File-Change Plan's S-17.05 row makes a specific current-content claim about its
  file list), and `.factory/policies.yaml` (Companion Amendments 5/6 quote its current POLICY 19
  `scope:` array and cite a scan of its content).
  **Disposition: FIXED.** Architect ran a MANDATORY complete inputs-completeness audit of the full
  document body (not a spot sweep, per explicit task direction that pass-28's own sweep had missed
  the first two of these six files) and added all six to `inputs:`. Explicitly rejected as
  non-load-bearing padding: `.factory/stories/STORY-INDEX.md`, `.factory/specs/architecture/ARCH-
  INDEX.md` (already covered by `traces_to:`), `.github/workflows/ci.yml` (cited only as a proposed
  future CI-gate location, never a current-content claim). No `modified:` array exists in ADR-046's
  frontmatter (confirmed by inspection), so no reordering finding applies to it. ADR-046
  v1.14→v1.15.

**LOW (0):** none this pass.

## Part B — Verified-Clean Observations (adversary-confirmed, no findings)

- **Behavioral core (write-composition table, five-outcome table, identity-gating logic,
  event-sourcing struct-variant text):** re-verified clean across all four artifacts this pass — no
  regression of any prior-pass fix, including the pass-29 `rewrite_expires_at` home-crate
  correction.
- **§Story Anchor / Traceability parity (F-P27-001 class):** re-verified clean across all three
  BCs; no regression.
- **Type-provenance (F-P25-001 class — `LockState` vs `FactoryLock`):** re-verified clean; no
  regression.
- **POLICY 19 (anti-volatile-pin):** no new load-bearing inline `ADR-046 vN.N` / `BC-X.YY.NNN vN.N`
  version-pin token found in any normative body prose across the frozen set.
- **BC-7.07.001 cluster parity:** all five audited legs (version/Changelog-head/modified-head/
  last_amended-prefix parity, `inputs:` completeness, §Story-Anchor cardinality) confirmed clean —
  no edit required this pass.
- **O-P28-002 root-cause fix (version-stable ARCH-INDEX directive):** confirmed holding — the
  directive correctly reads ADR-046's live `version:` field and did not require re-patching at this
  pass's v1.14→v1.15 bump.

**No spec-vs-code contradictions found this pass.** All substance cross-checks — behavioral core,
write-composition, event-sourcing, type-provenance, §Story Anchor parity, POLICY 19 — re-verified
CLEAN with zero regression. Both findings this pass (F-P30-001, F-P30-002) are pure metadata
parity: array-ordering discipline and `inputs:` frontmatter completeness. Neither touches this
ADR/BC cluster's actual behavioral contract text.

This pass's finding cluster continues the metadata/hygiene-layer churn observed since pass-27:
F-P30-001 is the SAME class as F-P29-003 (array-ordering parity), simply on two DIFFERENT BCs that
were never swept when F-P29-003's fix landed — a sibling-sweep gap, not a regression of the pass-29
fix itself. F-P30-002 is the SAME class as F-P28-001/F-P29-002 (`inputs:` completeness), continuing
to surface on ADR-046 itself this time via a mandatory complete-document audit rather than a spot
check. The remediation approach for this pass switched from single-locus spot-fixes to
COMPREHENSIVE per-dimension sweeps: reordering ALL BC `modified:` arrays cluster-wide (not just the
one flagged) and auditing EVERY ADR-cited file (not just the two initially flagged) — closing the
partial-fix-regression pattern that shed a straggler at each of passes 27, 28, and 29.

## Part C — State at Close of Review

ADR-046 **v1.15** (`inputs:` completed +6 files, F-P30-002); BC-4.17.001 **v1.15** (`modified:`
array reordered, F-P30-001); BC-5.40.001 **v1.13** (`modified:` array reordered, F-P30-001);
BC-7.07.001 **UNCHANGED at v1.31** (confirmed clean by cluster audit, no edit). BC-5.39.001 3-CLEAN
streak: **0/3** (REMAINS — already reset at pass-25; this pass's findings do not reset an
already-0/3 streak further). Gate history to date: 30 passes run against evolving/frozen sets; 17
genuine findings found and fixed prior to this pass; this pass adds F-P30-001 (HIGH, fixed) +
F-P30-002 (MED, fixed) — 19 genuine findings fixed across 30 passes total, zero LOW observations
this pass.

**NEXT: fresh pass-31** against the newly-frozen set (ADR-046 v1.15 + BC-4.17.001 v1.15 +
BC-5.40.001 v1.13 + BC-7.07.001 v1.31); needs 3 consecutive clean passes (31, 32, 33) for literal
3-CLEAN convergence. S-17.05 TDD implementation remains gated on convergence. The human decision
this session is to CONTINUE looping toward literal 3-CLEAN convergence (not accept-provisional
under D-386 Option C asymptotic acceptance).
