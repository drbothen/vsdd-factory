# ADR-046 Adversarial Spec-Convergence Review — Pass 29

**Reviewed artifact set (frozen):** ADR-046 v1.13 + BC-4.17.001 v1.13 + BC-7.07.001 v1.30 + BC-5.40.001 v1.11
**Review date:** 2026-08-26
**Verdict:** FINDINGS (3: 1 HIGH, 2 MED), 0 LOW observations
**Gate:** BC-5.39.001 3-CLEAN spec-convergence (ADR-046 fix-state-writes)
**Streak effect:** 0/3 → **REMAINS 0/3** (already reset at pass-25; a finding does not reset an already-0/3 streak further)
**D-chain:** D-1086

## Part A — Finding Set (frozen set: ADR-046 v1.13 + BC-4.17.001 v1.13 + BC-7.07.001 v1.30 + BC-5.40.001 v1.11)

**HIGH (1):**

- **F-P29-001 (HIGH, POLICY 4, spec-vs-code home-crate mis-attribution)** — ADR-046 self-contradicted
  on `rewrite_expires_at`'s home crate. F-P10-001's own v1.8 citation (Companion Amendment 2's
  write-composition paragraph, and its own v1.8 Changelog restatement) correctly states
  `rewrite_expires_at` is confirmed at `crates/factory-lock/src/lib.rs`'s `renew_lock_with_now` Step
  5, but two OTHER loci — the Companion Amendment 2 PC4-reconciliation bullet, and the v1.8
  Changelog entry's own closing sentence — described it as "the same mechanism
  `factory-lock-write.sh`'s own `_update_expires_at` and `rewrite_expires_at` already use," wrongly
  locating `rewrite_expires_at` INSIDE the bash script. Inspection of
  `plugins/vsdd-factory/bin/factory-lock-write.sh` confirms it declares only `_epoch_to_iso`,
  `_write_factory_lock_block`, and `_update_expires_at` — no `rewrite_expires_at` function exists in
  that file. BC-4.17.001's PC4 ("'Targeted' is a semantic-scope guarantee" paragraph) carried the
  identical mis-attribution, mirroring ADR-046's error.
  **Disposition: FIXED.** Architect corrected both ADR-046 loci (the Companion Amendment 2
  PC4-reconciliation bullet + the v1.8 Changelog entry's closing sentence) to attribute
  `rewrite_expires_at` to `crates/factory-lock/src/lib.rs`'s `renew_lock_with_now` (Rust) while
  keeping `factory-lock-write.sh`'s `_update_expires_at` (bash) as the correctly-attributed bash-side
  precedent — both mechanisms remain cited together (neither is a byte-range/patch API; both
  serialize the whole file with one region altered), only the file-of-record for
  `rewrite_expires_at` changed; the underlying whole-file-serialize argument is unaffected. A
  full-document sweep for `rewrite_expires_at` confirmed these were the only two mis-attributing loci
  in ADR-046. Product-owner independently corrected BC-4.17.001's PC4 to cite the same two-mechanism
  pairing (bash `_update_expires_at` example + Rust `renew_lock_with_now` example). ADR-046
  v1.13→v1.14; BC-4.17.001 v1.13→v1.14.

**MEDIUM (2):**

- **F-P29-002 (MED, POLICY 18, `inputs:` completeness)** — BC-5.40.001's `inputs:` frontmatter array
  omitted 5 load-bearing code files despite this BC making exact-code-body current-state claims
  against them: PC3's `is_expired` comparison against `verify-factory-lock`; the migrated
  Precondition 6/Invariant 7/Invariant 8/EC-010 `STATE_MD_MAX_BYTES`/`extract_frontmatter` claims;
  PC4's `renew_lock_if_holder`/`TTL_SECONDS` claims; and the `hooks-registry.toml` deregistration
  claim. This BC was de-scoped from the POLICY 18 sweep already applied to BC-7.07.001 (v1.29) and
  BC-4.17.001 (v1.13) — a sibling-sweep straggler of that same class, this BC never itself swept.
  **Disposition: FIXED.** Product-owner added `crates/hook-plugins/verify-factory-lock/src/lib.rs`,
  `crates/factory-lock/src/lib.rs`, `crates/factory-lock-parse/src/lib.rs`,
  `crates/hook-plugins/verify-state-timestamp-refresh/src/lib.rs`, and
  `plugins/vsdd-factory/hooks-registry.toml` to BC-5.40.001's `inputs:`, same path form the sibling
  BCs already use. Not the accepted BC-4.17.001↔BC-7.07.001↔ADR-046 mutual-inputs cyclic-hash TD
  (that concerns only that triple's mutual ADR/BC edges) — these are missing CODE inputs,
  legitimately in-scope and independent of the cyclic-hash class. BC-5.40.001 v1.11→v1.12.

- **F-P29-003 (MED, POLICY 17/14, `modified:` array ordering re-regression)** — BC-7.07.001's
  `modified:` array sequence was `v1.29, v1.30, v1.28, v1.27, ...` — the newest entry (v1.30 at the
  time) sat in the SECOND slot instead of the top of an otherwise strict-descending array. This is a
  RE-REGRESSION of O-P27-001 (pass-27 fixed the identical defect class at that time); the v1.30 edit
  reintroduced it by appending its own new entry directly above the v1.29 entry it was correcting,
  rather than at the true top of the array.
  **Disposition: FIXED.** Product-owner reordered the entire `modified:` array to strict
  descending-chronological (newest at top) — v1.31, v1.30, v1.29, v1.28, ... down to v1.1 — verified
  against the Changelog table (already correctly ordered, newest-row-first), so both parity legs now
  agree on the newest version. Dated HISTORICAL entry text (v1.1 through v1.30) unchanged — only
  array position corrected, per POLICY 1 append-only numbering. BC-7.07.001 v1.30→v1.31.

**LOW (0):** none this pass.

## Part B — Verified-Clean Observations (adversary-confirmed, no findings)

- **Behavioral core (write-composition table, five-outcome table, identity-gating logic,
  event-sourcing struct-variant text):** re-verified clean across all four artifacts this pass — no
  regression of any prior-pass fix.
- **§Story Anchor / Traceability parity (F-P27-001 class):** re-verified clean across all three
  BCs; no regression.
- **Type-provenance (F-P25-001 class — `LockState` vs `FactoryLock`):** re-verified clean; no
  regression, and not confused with the DISTINCT `rewrite_expires_at` home-crate defect found this
  pass (a different symbol, never previously audited).
- **POLICY 19 (anti-volatile-pin):** no new load-bearing inline `ADR-046 vN.N` / `BC-X.YY.NNN vN.N`
  version-pin token found in any normative body prose across the frozen set.
- **O-P28-002 root-cause fix (version-stable ARCH-INDEX directive):** confirmed holding — the
  directive correctly reads ADR-046's live `version:` field and did not require re-patching at this
  pass's v1.13→v1.14 bump.

This pass's finding cluster spans two distinct classes: (1) F-P29-001 is a GENUINELY NEW defect
class — a cross-language home-crate mis-attribution for a symbol (`rewrite_expires_at`) that none of
the prior 28 passes' cross-language attribution audits (`trim_git_email`/`LockState`/`TTL_SECONDS`/
`STATE_MD_MAX_BYTES`) had ever covered; (2) F-P29-002 and F-P29-003 are both PARTIAL-FIX
REGRESSIONS of the immediately-prior pass's own fixes — F-P29-002 is BC-5.40.001's own de-scoping
from the pass-28 POLICY 18 sweep (BC-5.40.001 was excluded from that sweep's scope, not swept
in error), and F-P29-003 is a literal re-regression of O-P27-001 by the v1.30 edit that was
correcting an unrelated defect in the same file. The adversary's overall assessment: **the
behavioral core is verified clean and stable across three consecutive passes (27, 28, 29) — no
regression of any settled behavioral-content fix — but the spec has NOT fully converged; the
metadata/hygiene layer (inputs: completeness, array-ordering discipline, cross-reference accuracy)
continues to shed partial-fix regressions of the immediately-prior burst's own fix, one pass at a
time.**

## Part C — State at Close of Review

ADR-046 **v1.14** (`rewrite_expires_at` home-crate mis-attribution corrected at 2 loci, F-P29-001);
BC-4.17.001 **v1.14** (PC4 mirrored correction, F-P29-001); BC-5.40.001 **v1.12** (`inputs:` +5 code
files, F-P29-002); BC-7.07.001 **v1.31** (`modified:` array reordered, F-P29-003). BC-5.39.001
3-CLEAN streak: **0/3** (REMAINS — already reset at pass-25; this pass's findings do not reset an
already-0/3 streak further). BC-4.17.001 ↔ BC-7.07.001 ↔ ADR-046 mutual `inputs:` cyclic-hash TD
(tracked since `[D-1082]`, extended to 3-way at D-1085) reconfirmed non-convergent again this pass,
and BC-5.40.001 is now CONFIRMED to participate in the same cyclic tangle (it already cited all
three other artifacts in its own `inputs:` prior to this burst; with all three edited this same
burst, BC-5.40.001's hash is unavoidably affected regardless of its own content edit, and — because
ADR-046 and BC-4.17.001 both cite BC-5.40.001 in their own `inputs:` — the cycle now effectively
spans all four artifacts). Settled per this pass's task instruction, cross-referenced against the
existing `[D-1082]` Drift Item, NOT re-opened as a new item. Gate history to date: 29 passes run
against evolving/frozen sets; 14 genuine findings found and fixed prior to this pass
(F-P10-001/F-P13-001/F-P15-001/F-P18-001/F-P27-001/F-P28-001 HIGH, F-P21-001/F-P23-001/F-P25-001/
F-P25-002/F-P26-001/F-P27-002/F-P27-003/F-P28-002 MED); this pass adds F-P29-001 (HIGH, fixed) +
F-P29-002 (MED, fixed) + F-P29-003 (MED, fixed) — 17 genuine findings fixed across 29 passes total,
zero LOW observations this pass.
**NEXT: fresh pass-30** against the newly-frozen set (ADR-046 v1.14 + BC-4.17.001 v1.14 +
BC-7.07.001 v1.31 + BC-5.40.001 v1.12); needs 3 consecutive clean passes (30, 31, 32) for literal
3-CLEAN convergence. S-17.05 TDD implementation remains gated on convergence.
