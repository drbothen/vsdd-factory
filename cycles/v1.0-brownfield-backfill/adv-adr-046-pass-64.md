# ADR-046 Spec-Convergence Adversary Review — Pass 64

**Date:** 2026-08-27
**Reviewer:** vsdd-factory:adversary (fresh-context; reads prior-pass Part A only)
**Scope:** ADR-046 "fix-state-writes" + companion BCs BC-4.17.001 / BC-5.40.001 / BC-7.07.001
**Frozen set:** ADR-046 v1.23 + BC-4.17.001 v1.26 + BC-5.40.001 v1.21 + BC-7.07.001 v1.39
**Streak entering this pass:** 1/3 (pass-63 CLEAN; post-pass-62-reset sequence)

---

## Part A — Blocking Findings

**VERDICT: CLEAN — zero blocking findings at any severity.**

No BLOCKER, HIGH, or MEDIUM findings exist in this pass. The spec artifact set is internally consistent and faithfully describes the source code behavior as verified by independent re-derivation.

---

## Part B — Substantive Verification

### Scope Confirmation Table

| Artifact | Version | input-hash |
|----------|---------|------------|
| ADR-046 | v1.23 | 3335ad4 |
| BC-4.17.001 | v1.26 | 6b0b35c |
| BC-5.40.001 | v1.21 | 6a9cc08 |
| BC-7.07.001 | v1.39 | e73bc01 |

All four frozen spec artifacts confirmed at the expected versions and hashes. None edited this pass.

### Ground-Truth Code Verification (all 17 checks re-derived independently)

Every behavioral claim in the frozen set independently re-derived against source (`crates/factory-lock-parse/src/lib.rs`, `crates/factory-lock/src/lib.rs`, `plugins/vsdd-factory/hooks/precompact-flush.sh`, `plugins/vsdd-factory/hooks/verify-state-timestamp-refresh.sh`):

1. **`parse_factory_lock` empty-holder → `Err(Malformed)`:** confirmed. `crates/factory-lock-parse/src/lib.rs` lines 219-227: empty string holder returns `Err(MalformedLockBlock)`. ADR-046 §0th-case table + BC-4.17.001 PC2 + BC-7.07.001 Inv3b all correctly specify `Err(Malformed)`. MATCH.

2. **`parse_factory_lock` absent-holder-with-siblings → `Err(Malformed)`:** confirmed. Same code block: a `factory_lock:` key with no `holder:` sub-key (but with other sub-keys present) returns `Err(MalformedLockBlock)`. Spec: "absent holder with siblings = case-1 = Err(Malformed)". MATCH.

3. **`Ok(None)` only for fully-absent or null block:** confirmed. `Ok(None)` is returned only when the `factory_lock:` top-level key is absent entirely, or when `factory_lock: null`. Not returned for empty/absent holder when siblings present. ADR-046 Decision-3 + BC-4.17.001 PC2 five-case table accurate. MATCH.

4. **`renew_lock_with_now` opaque-String `expires_at`:** confirmed. `crates/factory-lock/src/lib.rs` `renew_lock_with_now` computes `expires_at` as a formatted ISO-8601 string via `now + TTL_SECONDS` arithmetic, stores as opaque `String`, never date-parses the result. ADR-046 case-1 re-derived accurately. MATCH.

5. **`renew_lock_with_now` byte-compare / never date-parses:** confirmed. The `renew_lock_with_now` function writes a new `expires_at` string directly from `Duration` arithmetic; no parsing of the outgoing value. MATCH.

6. **`parse_iso8601` used for case-1 `is_expired` check:** confirmed. `is_expired` calls `parse_iso8601` on the stored `expires_at` field to compare against `now`. ADR-046 case-1 description accurate. MATCH.

7. **`is_expired`: `now >= expires_at`:** confirmed. Boundary condition: `now` equal to `expires_at` is expired (>=). Spec boundary "now >= expires_at" MATCH.

8. **`trim_git_email` uses `trim_end`:** confirmed. `crates/factory-lock/src/lib.rs` `trim_git_email`: trailing-whitespace stripped with `trim_end()`. BC-5.40.001 §Implementation Note accurate. MATCH.

9. **Three TTL literals all 2700 (incl. `u64`):** confirmed. `TTL_SECONDS: u64 = 2700` in `crates/factory-lock/src/lib.rs`; `LOCK_RENEWAL_TTL_SECS: u64 = 2700` in `plugins/vsdd-factory/hooks/precompact-flush.sh`; `TTL_SECONDS=2700` in ADR-046 §Implementation. All three match. MATCH. The "MUST NOT be overridden" comment verified present. MATCH.

10. **Precompact-flush Step-4 identity-blind renew_lock:** confirmed. `plugins/vsdd-factory/hooks/precompact-flush.sh` Step 4 calls `renew_lock_with_now` unconditionally without inspecting holder identity. ADR-046 §Precompact-flush integration description accurate. MATCH.

11. **`FactoryLock` vs `LockState` distinction:** confirmed. `crates/factory-lock-parse/src/lib.rs` uses `FactoryLock` as the public type; `LockState` is the internal wire form. Spec uses `FactoryLock` throughout as the public API type; no confusion of the two. MATCH.

12. **`extract_yaml_string_value` holder `null` → literal `"null"`:** confirmed. `extract_yaml_string_value` on a YAML scalar `null` returns `Some("null")` (string). EC-011 in BC-5.40.001 (holder → literal string `"null"`) accurate. MATCH.

13. **`verify-state-timestamp-refresh` Steps 4-7/8 (F-P54-001 fix):** confirmed. `plugins/vsdd-factory/hooks/verify-state-timestamp-refresh.sh` module doc: Steps numbered 4-7 body check + Step 8 final verdict, matching ADR-046 step citations. MATCH.

14. **Five-case table byte-consistent across ADR/BC-4.17.001 PC2/BC-7.07.001 Inv3b:** confirmed. The five cases (fully-absent, null-block, well-formed, empty-holder, absent-holder-with-siblings) are enumerated identically across the three spec documents. No divergence detected. MATCH.

15. **Decision-5 migration reconciled both ends:** confirmed. BC-4.17.001 v1.26 §Description/§Traceability now includes the Decision-5 coverage (F-P58-001 fix, pass-58). BC-5.40.001 v1.21 §Description/§Traceability now includes the Decision-5 coverage (F-P59-001 fix, pass-59). Both migration ends confirmed complete, symmetric per Decision-5 (MIGRATED/RETAINED-AS-HISTORICAL). MATCH.

16. **POLICY 4/6/19 compliance:** confirmed. No intra-document inconsistency detected. ARCH-INDEX ADR-046 row subsystem name `SS-05` confirmed against ARCH-INDEX §Subsystems table (POLICY 6). No load-bearing ADR version pins in any live BC body (POLICY 19). MATCH.

17. **Sibling-sweep — no unswept holder:null straggler:** confirmed. Corpus-wide check: `BC-4.17.001`, `BC-5.40.001`, `BC-7.07.001` all handle the `holder: null` case consistently. ADR-046 treatment of null-block confirmed correct. F-P62-001 structural fix (ARCH-INDEX ADR-046 row headline) re-confirmed holding — "current version per ADR-046 frontmatter (tail records bump history)" form present; recurrence mechanism eliminated; O-P28-002 falsification durably closed. MATCH.

### Internal Consistency and Cross-Artifact Reconciliation

- **BC-4.17.001 v1.26:** §Description, §Traceability, §Preconditions, §Invariants, §Examples, §VP Anchors all internally consistent. Decision-5 enumeration complete (F-P58-001 fix). No stale version pins in live body.
- **BC-5.40.001 v1.21:** §Description gains Decision-5 reconciliation sentence (F-P59-001 fix); §Traceability ADR Reference row includes §Decision 5 summary; §Preconditions PC4/PC6, §Invariants 7/8, EC-010, §VP Anchors T-001..T-007 all correctly annotated MIGRATED/RETAINED-AS-HISTORICAL under §Decision 5's guard-read reconciliation. `trim_git_email` cross-reference non-normative (O-P60-002 adjudicated NON-DEFECT). No stale version pins in live body.
- **BC-7.07.001 v1.39:** Invariant 3b five-case table accurate; §Traceability clean; EC-011 holder:null accurate. No §Decision 5 participant (confirmed correct). No stale version pins.
- **ADR-046 v1.23:** §Decisions 1-5, §File-Change Plan, §Companion Amendment 1 items (i)-(vi) internally consistent. Step numbering (4-7/8) correct (F-P54-001 fix). §0th-case table accurate. No load-bearing volatile version pins in live body (POLICY 19 satisfied).

### Novelty Assessment

**Novelty: LOW.** The behavioral core has been verified-clean for many consecutive passes (since pass-27, substantively). This pass independently re-derived all 17 claims with zero divergence from prior passes 60-63. The only candidate for fresh attention was the F-P62-001 structural fix — confirmed still holding under independent re-derivation. The converged state is stable.

---

## Part C — Observations (non-blocking; NON-DEFECT — both ALREADY TRACKED, no new action)

### O-P64-001 [NON-DEFECT, documentation-symmetry]

**Class:** Documentation-symmetry asymmetry between BC-4.17.001 and its siblings.

**Detail:** BC-4.17.001 has no explicit `holder: null` illustrative example case (EC) while siblings BC-5.40.001 and BC-7.07.001 do provide such an EC. This is the SAME class as the already-ACCEPTED-tracked **O-P57-001** (D-1114), which was independently adjudicated NON-DEFECT at that time.

**Adversary adjudication:** NON-DEFECT. BC-4.17.001's general five-case PC2 gate already covers the `holder: "null"` case (the literal-string-null return path from `extract_yaml_string_value`) correctly. The absence of a dedicated illustrative EC is a documentation-symmetry choice, not a spec-vs-code defect. Substance is not missing; only the illustrative example is asymmetric.

**Disposition:** ACCEPTED-tracked (recurrence of O-P57-001-class). This is a re-surfacing of the O-P57-001-class item already accepted at D-1114. No new tracked entry needed — the authorial-intent question for product-owner per S-7.01 remains as previously recorded. Does NOT reset streak.

### O-P64-002 [out-of-perimeter → implementer; ALREADY CAPTURED]

**Class:** Stale doc-comments in `crates/factory-lock/src/lib.rs` (implementer-scope).

**Detail:** `crates/factory-lock/src/lib.rs` doc-comments (`renew_lock_with_now` algorithm doc ~line 113, inline comment at Ok(None) arm ~lines 158-160, `parse_lock` doc ~line 318) still describe the pre-F-P56-001 semantics (characterizing empty/absent holder as returning `Ok(None)` when the actual runtime behavior is `Err(MalformedLockBlock)`). Runtime behavior is correct; only doc-comments are stale.

**Disposition:** ALREADY CAPTURED in **S-17.05 v1.1 Task T-8** (story-writer commit `f323b5e2`, D-1120, 2026-08-27). Fix executes when S-17.05 enters TDD implementation. No new action required — noting it as a re-surfacing of the O-P61-001/O-P62-001-class item already captured. Does NOT reset streak.

---

## Summary

| Category | Count |
|----------|-------|
| Blocking findings (BLOCKER/HIGH/MEDIUM) | **0** |
| Non-blocking observations | 2 (both already tracked, no new action) |
| Spec-vs-code checks performed | 17 |
| Spec-vs-code checks: MATCH | 17 |
| Spec-vs-code checks: MISMATCH | 0 |

**Streak:** ADVANCES 1/3 → **2/3** (second consecutive clean pass of the post-pass-62-reset sequence).

**Frozen set:** UNCHANGED — ADR-046 v1.23 + BC-4.17.001 v1.26 + BC-5.40.001 v1.21 + BC-7.07.001 v1.39.

**THIS IS A CLEAN PASS, NOT A FIX BURST** — no spec artifact edited; no version bump; no input-hash recompute; no 4-INDEX version-cell change.

**NEXT:** fresh adversary pass-65 against the SAME unchanged frozen set — one more consecutive clean pass needed for literal BC-5.39.001 3-CLEAN, which unblocks S-17.05 TDD implementation.
