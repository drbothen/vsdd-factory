# ADR-046 Adversarial Spec-Convergence Review — Pass 25

**Reviewed artifact set (frozen):** ADR-046 v1.10 + BC-4.17.001 v1.11 + BC-7.07.001 v1.27 + BC-5.40.001 v1.9
**Review date:** 2026-08-26
**Verdict:** FINDINGS (2) — both MEDIUM
**Gate:** BC-5.39.001 3-CLEAN spec-convergence (ADR-046 fix-state-writes)
**Streak effect:** 1/3 → **RESET 0/3** (any finding resets the streak; pass-24 was the sole clean pass banked)
**D-chain:** D-1082

This is the first persisted per-pass record for the ADR-046 gate. Passes 1–24 were tracked
narratively in STATE.md / session-checkpoints.md only (no individual pass files were written);
this file establishes the `adv-adr-046-pass-N.md` convention referenced in the task instructions,
mirroring the existing `adv-adr-043-pass-N.md` pattern used for the ADR-043 gate.

## Part A — Finding Set (frozen set: ADR-046 v1.10 + BC-4.17.001 v1.11 + BC-7.07.001 v1.27 + BC-5.40.001 v1.9)

**MEDIUM (2):**

- **F-P25-001 (MED, POLICY 4 spec-vs-code type/function mismatch)** — ADR-046 §Decision 1 /
  File-Change Plan and BC-7.07.001 Invariant 3b annotated the value `renew_lock_if_holder`
  resolves at its holder-present step as `lock_state: FactoryLock`. Ground truth
  (`crates/factory-lock/src/lib.rs` + `crates/factory-lock-parse/src/lib.rs`):
  `renew_lock_if_holder` performs its own independent `flp::parse_factory_lock(content)` parse at
  that step, which returns a `LockState` (crate `factory-lock-parse`) — a field-identical sibling
  struct to `FactoryLock` (crate `factory-lock`), but a distinct type. `FactoryLock` is produced
  only by `factory_lock::parse_lock`, a different function never called on this path. The two
  structs share field shape, which is why the annotation error survived 24 prior passes without
  producing an observable behavioral defect — this is the escalation of the previously-tracked
  **O-P24-001 (LOW)** type-provenance nit (pass-24), now confirmed as a genuine spec-vs-code
  mismatch rather than a cosmetic imprecision, and RESOLVED by this pass's fix.
  **Disposition: FIXED.** ADR-046 and BC-7.07.001 corrected to cite `LockState` /
  `flp::parse_factory_lock`, not `FactoryLock` / `factory_lock::parse_lock`. The canonical model
  is now stated unambiguously: `renew_lock_if_holder` → own `flp::parse_factory_lock` parse →
  `LockState`; `FactoryLock` is produced only by `factory_lock::parse_lock`, a call this hook
  never makes.

- **F-P25-002 (MED, traceability story-anchor conflict)** — ADR-046 named S-17.05 as the
  implementing story in its narrative, while all three companion BCs (BC-4.17.001, BC-7.07.001,
  BC-5.40.001) still carried `[pending]` placeholders in their Traceability §Stories rows and
  §Story Anchor fields — and ADR-046's own "referenced in the File-Change Plan" cross-reference
  to S-17.05 did not resolve (S-17.05 was not yet listed in the File-Change Plan itself).
  **Disposition: FIXED.** Architect added an explicit S-17.05 row to ADR-046's File-Change Plan;
  product-owner cited S-17.05 in all three BCs' Traceability §Stories rows and §Story Anchor
  fields. All four artifacts in the frozen set now agree: S-17.05 (`stamp-state-timestamp-hook`,
  E-17 Wave 5, 8pts, `tdd_mode: strict`) is the confirmed implementing story.

## Part B — Verified-Clean Observations (adversary-confirmed, no findings)

- **POLICY 19 (anti-volatile-pin):** no load-bearing inline `ADR-046 vN.N` / `BC-X.YY.NNN vN.N`
  version-pin tokens found in any of the four frozen-set artifacts' normative body prose (PC/
  Invariant/EC text). All ADR cross-references use the stable `§Decision N` anchor form.
- **Subsystem label:** SS-04/SS-05/SS-07 subsystem assignments across ADR-046 and its three
  companion BCs are internally consistent and match each artifact's own `subsystem:` frontmatter
  field; no cross-file subsystem-label drift.
- **Code-anchored claims:** every `crates/factory-lock*` function-name / constant citation in the
  frozen set (other than the F-P25-001 type-name defect, now fixed) was independently verified
  against `crates/factory-lock/src/lib.rs` and `crates/factory-lock-parse/src/lib.rs` — all
  resolve to real, correctly-named functions/constants.
- **Boundary/idempotency:** the `expires_at` arm's `>=` comparison boundary (same-wall-clock-
  second re-invocation = no-op, not a failure) and the `timestamp:` arm's explicit
  non-idempotency (live clock, unconditional re-stamp) are stated consistently and correctly
  across ADR-046, BC-4.17.001, and BC-7.07.001 — no boundary-condition or idempotency-labeling
  defects found.

## Part C — State at Close of Review

ADR-046 **v1.11** (accepted); BC-4.17.001 **v1.12**; BC-7.07.001 **v1.28**; BC-5.40.001 **v1.10**.
BC-5.39.001 3-CLEAN streak: **0/3** (reset from 1/3 — any finding resets the streak per the
human-ratified literal-3-CLEAN discipline). Gate history to date: 24 passes run against
evolving/frozen sets; 6 genuine bugs found and fixed prior to this pass
(F-P10-001/F-P13-001/F-P15-001/F-P18-001 HIGH, F-P21-001/F-P23-001 MED); this pass adds
F-P25-001/F-P25-002 (both MED, both fixed) — 8 genuine findings fixed across 25 passes total.
**NEXT: fresh pass-26** against the newly-frozen set (ADR-046 v1.11 + BC-4.17.001 v1.12 +
BC-7.07.001 v1.28 + BC-5.40.001 v1.10); needs 3 consecutive clean passes (26, 27, 28) for literal
3-CLEAN convergence. S-17.05 TDD implementation remains gated on convergence.
