---
document_type: perimeter-audit
audit_id: perimeter-audit-adr-046-3clean
cycle: v1.0-brownfield-backfill
date: 2026-08-27
auditor: consistency-validator (fresh-context)
trigger: BC-5.39.001 3-CLEAN ACHIEVED (passes 63/64/65) — pre-gate-closure perimeter check
verdict: PERIMETER-GAPS
---

# Perimeter Audit: ADR-046 3-CLEAN Gate Closure Check

**Date:** 2026-08-27
**Trigger:** ADR-046 BC-5.39.001 literal 3-CLEAN achieved (passes 63/64/65, D-1123). Pre-closure
perimeter audit performed by fresh-context consistency-validator to verify whether the surrounding
story, BC, and index parity is sound before the gate is formally closed.

**Audit scope:** Fresh-context read of the frozen spec set (ADR-046 v1.23 + BC-4.17.001 v1.26 +
BC-5.40.001 v1.21 + BC-7.07.001 v1.39) PLUS the implementing story S-17.05 v1.1, the four index
files (ARCH-INDEX v3.94, BC-INDEX v5.18, VP-INDEX v2.79, STORY-INDEX v4.393), and the code at
`crates/factory-lock/`, `crates/factory-lock-parse/`, `crates/hook-plugins/precompact-flush/`.

---

## VERDICT: PERIMETER-GAPS

**All gaps are in the IMPLEMENTING STORY S-17.05 — NOT in the frozen spec set.**

The frozen spec set (ADR-046 v1.23 + BC-4.17.001 v1.26 + BC-5.40.001 v1.21 + BC-7.07.001 v1.39)
is internally consistent. The adversary 3-CLEAN result (passes 63/64/65) is VALID.

The perimeter audit identified 3 BLOCKS-CLOSURE gaps, 2 ADVISORYs, and 2 SANCTIONED-DEFERRALs in
the implementing story S-17.05. The spec-convergence gate on the adversary axis is SATISFIED; the
story-scope gap is a separate implementation-readiness issue that must be resolved via story
decomposition before S-17.05 TDD entry.

---

## Gap Registry

### Gap A — BLOCKS-CLOSURE (story-writer)

**Finding:** S-17.05 has no task for the ADR-046 File-Change-Plan `factory-lock` shared-function
additions.

**Locus:** S-17.05 v1.1 `target_module`, `library_requirements`, `file_structure`, `tasks` — all
omit `crates/factory-lock/` entirely.

**ADR-046 mandates (File-Change-Plan, Companion Amendment 1):**
- `renew_lock_if_holder(state: &str, holder: &str) -> Result<String, LockError>` — new
  shared function in `crates/factory-lock/`
- `IdentityResolution` enum (`Holder` / `Blank` / `SkipReason(String)`)
- `SkipReason` struct/enum for skip-path classification
- `classify_identity_resolution(holder: Option<&str>, git_email: Option<&str>) -> IdentityResolution`
- `trim_git_email(raw: &str) -> String` promotion from
  `crates/hook-plugins/verify-factory-lock/` to `crates/factory-lock-parse/` (Rule 9 "direct
  crate reference")

**Code status:** NONE of these functions exist in any crate. `crates/factory-lock/src/lib.rs` has
no `renew_lock_if_holder`; `crates/factory-lock-parse/src/lib.rs` has no `trim_git_email` at the
promoted path.

**Severity:** BLOCKS-CLOSURE — S-17.05 cannot close the ADR-046 Rollout Note ("all parts ship in
the SAME release") without these functions.

**Owner:** story-writer (S-17.05 re-scope or new companion story S-17.06).

---

### Gap B — BLOCKS-CLOSURE (story-writer)

**Finding:** S-17.05 has no task for the precompact-flush Step-4 identity-gate amendment.

**Locus:** S-17.05 v1.1 tasks (T-1 through T-8) — no task touches
`crates/hook-plugins/precompact-flush/src/lib.rs`. No companion story exists.

**ADR-046 mandates (Rollout Note, explicitly):** All parts MUST ship in the SAME release:
- precompact-flush Step-4 call-site amended: `renew_lock` → `renew_lock_if_holder`
- 4-outcome tests: `Holder` (renew), `Blank` (skip), `SkipReason(...)` (skip with reason),
  `Error` (propagate)

**Code status:** `crates/hook-plugins/precompact-flush/src/lib.rs` line ~518 still calls
identity-blind `renew_lock`. The `renew_lock_if_holder` function doesn't exist yet (Gap A). No
4-outcome tests exist.

**ADR-046 Rollout Note atomicity:** The wave gate is the enforcing mechanism; all three stories
(S-17.05, S-17.06, S-17.07) MUST be in the same wave/release.

**Severity:** BLOCKS-CLOSURE — the ADR-046 atomic-rollout requirement cannot be met while this
task is missing from all stories.

**Owner:** story-writer (new companion story S-17.07).

---

### Gap C — BLOCKS-CLOSURE (story-writer)

**Finding:** BC-7.07.001 is absent from S-17.05 `behavioral_contracts` frontmatter.

**Locus:** S-17.05 v1.1 frontmatter `behavioral_contracts` array; S-17.05 acceptance criteria
(none trace to BC-7.07.001 PC3, Inv3, or Inv3b).

**Spec requirement:** VSDD Criteria 67 (bidirectional-citation) and 69 (story↔BC completeness)
both require that any BC whose requirements are implemented by a story appears in that story's
`behavioral_contracts` frontmatter AND that the story has ACs tracing to the BC's key clauses.

**BC-7.07.001 v1.39 relevance:** PC3 (identity-gate: only renew if lock holder matches git email)
and Inv3 / Inv3b (identity-gate invariants) are directly exercised by the precompact-flush
Step-4 amendment — which is Gap B's companion story S-17.07's core scope. BC-7.07.001 should be
re-anchored to S-17.07 (its owning story per human decomposition decision D-1124), not added to
S-17.05.

**Resolution:** Human decision D-1124 re-anchors BC-7.07.001 to S-17.07. Gap C is resolved by
the decomposition — S-17.07 will carry the BC-7.07.001 citation.

**Severity:** BLOCKS-CLOSURE (pre-decomposition) — RESOLVED by D-1124 wave decomposition.

**Owner:** story-writer (anchor to S-17.07 per D-1124).

---

### Gap D — ADVISORY (story-writer)

**Finding:** S-17.05 `verification_properties` comment cites "VP-TBD-1..4" but BC-4.17.001 v1.26
also has VP-TBD-7/8/9 (Decision-5 migration, per §VP Anchors).

**Locus:** S-17.05 v1.1 frontmatter `verification_properties` comment/field.

**Detail:** The VP-TBD-7/8/9 anchors were added to BC-4.17.001 at v1.26 (D-1115, pass-58 fix). The
S-17.05 story predates this addition; its VP comment was not updated. Not a blocking gap (VPs are
formal-verifier scope per POLICY 9 / Gap F), but the stale count is a documentation accuracy issue.

**Severity:** ADVISORY — does not block closure; should be updated when S-17.05 is re-scoped.

**Owner:** story-writer (update VP comment during re-scope).

---

### Gap E — ADVISORY (story-writer)

**Finding:** `trim_git_email` promotion path ambiguous in S-17.05 T-2.

**Locus:** S-17.05 v1.1 Task T-2, Rule 9 "direct crate reference" — the mechanism by which
`trim_git_email` moves from `crates/hook-plugins/verify-factory-lock/` to the promoted location
is described only as "direct crate reference," which is undefined in the story context.

**Detail:** The only coherent path (consistent with the ADR-046 File-Change-Plan and the existing
crate architecture) is promotion to `crates/factory-lock-parse/`, where other low-level string
parsing utilities live. This should be spelled out explicitly.

**Severity:** ADVISORY — ambiguous but not blocking; the correct path is inferable. Should be
clarified in S-17.06 (the story that owns this function per D-1124).

**Owner:** story-writer (clarify in S-17.06 task description).

---

### Gap F — SANCTIONED-DEFERRAL

**Finding:** VP-TBD-7/8/9 not yet in VP-INDEX.

**Detail:** BC-4.17.001 §VP Anchors cites VP-TBD-7, VP-TBD-8, VP-TBD-9 as PENDING. These have
not been allocated in VP-INDEX.md.

**Resolution:** POLICY 9 sanctioned VP-TBD deferral — formal-verifier scope. Expected state. No
action required at this stage.

**Severity:** SANCTIONED-DEFERRAL.

---

### Gap G — SANCTIONED-DEFERRAL

**Finding:** `verify-state-timestamp-refresh` crate source deletion deferred.

**Detail:** ADR-046 File-Change-Plan includes deletion of the `crates/hook-plugins/verify-state-timestamp-refresh/`
crate source (the WASM guard is replaced by the stamp-state-timestamp hook in S-17.05). This
deletion is deferred per human direction and ADR-anchored sequencing.

**Resolution:** Human-directed deferral, ADR-anchored. Expected state.

**Severity:** SANCTIONED-DEFERRAL.

---

## Index Parity Coverage Matrix

All index cells checked against live artifact frontmatter versions.

| Index File | Cell / Row | Expected | Actual | Status |
|-----------|------------|---------|--------|--------|
| ARCH-INDEX v3.94 | ADR-046 row version | "current version per ADR-046 frontmatter" (stable reference post-F-P62-001) | matches | PASS |
| BC-INDEX v5.18 | BC-4.17.001 row | v1.26 | v1.26 | PASS |
| BC-INDEX v5.18 | BC-5.40.001 row | v1.21 | v1.21 | PASS |
| BC-INDEX v5.18 | BC-7.07.001 row | v1.39 | v1.39 | PASS |
| STORY-INDEX v4.393 | S-17.05 row | v1.1, hash 4702970 | v1.1, hash 4702970 | PASS |
| VP-INDEX v2.79 | VP-TBD-7/8/9 | not allocated (POLICY 9 deferred) | absent | PASS (Gap F — sanctioned) |

**All cells PASS.** No index drift. The 3-CLEAN result is validated from an index-parity perspective.

---

## Per-Question Answers (fresh-context)

**Q1: Is the frozen spec set (ADR-046 v1.23 + BC-4.17.001 v1.26 + BC-5.40.001 v1.21 + BC-7.07.001 v1.39) internally consistent?**
**A: YES.** All cross-references, version cites, and behavioral claims within the four frozen
artifacts are mutually consistent. The adversary 3-CLEAN is valid.

**Q2: Does the implementing story S-17.05 v1.1 have correct scope to implement the entire ADR-046 File-Change-Plan?**
**A: NO.** S-17.05 is under-scoped. Gaps A and B identify two significant implementation areas
(factory-lock shared functions and precompact-flush Step-4 amendment) absent from its task list.
Gap C identifies a missing BC citation (resolved by decomposition to S-17.07 per D-1124).

**Q3: Does the ADR-046 Rollout Note atomicity requirement hold with S-17.05 alone?**
**A: NO.** The Rollout Note requires all parts in the SAME release. With Gap A and Gap B both
unscoped, the atomicity requirement cannot be met by S-17.05 alone. The human wave-decomposition
decision (D-1124) creates S-17.06 + S-17.07 in the same wave to restore atomicity.

**Q4: Is S-17.05 ready for TDD entry?**
**A: NO.** S-17.05 is blocked on the decomposition cascade completing: architect decomposition
design → product-owner BC re-anchoring → story-writer new stories + re-scope → state-manager
indexing. TDD entry for E-17 Wave-5 is gated on this cascade.

---

## Human Decision (D-1124, 2026-08-27): Wave Decomposition

The human directed remediation of the S-17.05 under-scoping via wave decomposition:

- **S-17.05** (stamp-state-timestamp plugin + TTL constant) — RETAINED, re-scoped to its
  narrower original intent.
- **S-17.06** (factory-lock shared functions + identity resolution) — NEW story, owning:
  `renew_lock_if_holder`, `IdentityResolution`, `SkipReason`, `classify_identity_resolution`,
  `trim_git_email` promotion.
- **S-17.07** (precompact-flush Step-4 identity-gate amendment + 4-outcome tests) — NEW story,
  owning the call-site amendment and test suite. BC-7.07.001 re-anchored here.

All three stories MUST be in the same wave/release (ADR-046 Rollout Note atomicity preserved via
the wave gate).

**Gap C resolved:** BC-7.07.001 re-anchored to S-17.07 (its natural owning story) rather than
added to S-17.05.

---

## Conclusion

- **ADR-046 spec-convergence gate (adversary axis):** CLOSED. 3-CLEAN validated (63/64/65).
  Frozen spec set internally consistent.
- **S-17.05 TDD readiness:** NOT READY. Blocked on decomposition cascade (S-17.05 re-scope +
  S-17.06 + S-17.07 creation) completing.
- **Immediate next work:** architect decomposition design → product-owner BC re-anchoring →
  story-writer story creation + S-17.05 re-scope → state-manager indexing.
- **Wave gate:** E-17 Wave-5 TDD entry gated on all three stories (S-17.05/06/07) reaching
  implementation-ready state AND the wave gate passing.
