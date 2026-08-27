---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-08-27T00:00:00Z
phase: 5
inputs:
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/specs/architecture/decisions/ADR-046-posttooluse-hook-authored-statemd-wall-clock-stamping-timestamp-lock-keep-alive.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.17.001.md
  - .factory/specs/behavioral-contracts/ss-07/BC-5.40.001.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.07.001.md
input-hash: "[live-state]"
traces_to: prd.md
pass: 62
previous_review: adv-adr-046-pass-61.md
---

# ADR-046 BC-5.39.001 Spec-Convergence Gate — Adversary Pass 62

**Date:** 2026-08-27
**Reviewer:** vsdd-factory:adversary (fresh-context, independent of all prior passes)
**Scope:** ADR-046 "fix-state-writes" spec-convergence gate — frozen set review
**Frozen set entering this pass:** ADR-046 v1.23 + BC-4.17.001 v1.26 + BC-5.40.001 v1.21 + BC-7.07.001 v1.39
**Streak entering this pass:** 2/3 (from pass-61 CLEAN, D-1118)

---

## PART A: VERDICT

**VERDICT: FINDINGS (1 MEDIUM) — F-P62-001 — streak RESETS 2/3 → 0/3 (9th reset)**

Human adjudication (2026-08-27, human-directed): LITERAL 3-CLEAN standard — pass-62 returned a FINDINGS verdict, so the streak RESETS 2/3 → 0/3 even though the sole finding is outside the frozen spec set. This is the 9th reset this session. The out-of-frozen-set finding still resets per human ruling (literal-3-CLEAN standard).

**Frozen set UNCHANGED.** ADR-046 v1.23 + BC-4.17.001 v1.26 + BC-5.40.001 v1.21 + BC-7.07.001 v1.39.

The four frozen spec artifacts were re-verified and are internally CLEAN and code-faithful:
- All nine spec-vs-code ground-truth behavioral checks MATCH (see PART B)
- All seventeen codified convergence-technique disciplines verified holding
- Zero BLOCKER, HIGH, MEDIUM, or LOW findings within the frozen set perimeter

The sole finding (F-P62-001) is in ARCH-INDEX.md — an index artifact outside the frozen spec set.

---

## Finding ID Convention

Finding IDs for this pass use the format: `F-P62-NNN` (pass-62, sequential).

---

## PART B: SPEC-VS-CODE GROUND-TRUTH VERIFICATION (Confirmed-Clean Frozen Set)

| Claim | Location | Code Site | Verdict |
|-------|----------|-----------|---------|
| `parse_factory_lock` empty/absent-holder → `Err(MalformedLockBlock)` | ADR-046 §Decision 2 / BC-4.17.001 PC2/case-1 / BC-7.07.001 PC2 | `crates/factory-lock-parse/src/lib.rs` lines 207-227 | MATCH |
| `extract_yaml_string_value` no null-special-casing of holder field | ADR-046 §Decision 2 / BC-4.17.001 PC2 | `crates/factory-lock-parse/src/lib.rs` | MATCH |
| `renew_lock_with_now` bare `Duration::seconds(2700)` / byte-guard | ADR-046 §Decision 1/F-004 / BC-5.40.001 Invariant 7 | `crates/factory-lock/src/lib.rs` `renew_lock_with_now` | MATCH |
| `has_factory_lock_key` key-line-only detection | ADR-046 §Decision 1 / BC-4.17.001 PC1 | `crates/factory-lock/src/lib.rs` | MATCH |
| `is_expired` now>=expires_at boundary | ADR-046 §Decision 2 / BC-7.07.001 PC5 | `crates/hook-plugins/verify-factory-lock/src/lib.rs` | MATCH |
| `trim_git_email` holder identity normalization | ADR-046 §Decision 2 / BC-7.07.001 | `crates/hook-plugins/verify-factory-lock/src/lib.rs` | MATCH |
| Step-4 `renew_lock` in `precompact-flush` is identity-blind | ADR-046 §Decision 3 / BC-5.40.001 Invariant 8 | `crates/hook-plugins/precompact-flush/src/lib.rs` Step 4 | MATCH |
| `TTL_SECONDS=2700` (45 min, non-configurable) | ADR-046 §Decision 1/F-004 / BC-5.40.001 Invariant 7 | `plugins/vsdd-factory/bin/factory-lock-write.sh` | MATCH |
| `FactoryLock` vs `LockState` distinction (crate provenance) | ADR-046 §Decision 2 / BC-4.17.001 | `crates/factory-lock-parse/src/lib.rs` vs `crates/factory-lock/src/lib.rs` | MATCH |
| Five-case table byte-identical across ADR-046/BC-4.17.001 PC2/BC-7.07.001 Inv3b | ADR-046 §Decision 2 / BC-4.17.001 PC2 / BC-7.07.001 Inv3b | All three artifacts | MATCH |
| Migration reconciliation BC-5.40.001→BC-4.17.001 bidirectional | ADR-046 §Decision 5 | BC-5.40.001 §Decision 5 + BC-4.17.001 §Decision 5 | MATCH |

POLICY spot-checks: POLICY 7/4/5/19/8 — all PASS for the frozen set.

---

## Part B — New Findings (or all findings for pass 1)

### MEDIUM

#### F-P62-001: ARCH-INDEX ADR-046 row headline marker stale by 5 revisions (POLICY 14/17 + POLICY 4)

- **Severity:** MEDIUM
- **Category:** spec-fidelity, upstream-index-version-parity
- **Location:** `.factory/specs/architecture/ARCH-INDEX.md`, `## Architecture Decisions` table, ADR-046 row, Decision-Summary cell headline
- **Description:** The ADR-046 row's Decision-Summary cell opens with the headline marker: `**RATIFIED 2026-08-25; ADR-046 v1.18 as of this row.**` This literal is stale by 5 revisions. The live ADR-046 frontmatter reads `version: "1.23"`, and the SAME cell's own narrative tail records the complete bump history including "Pass-56 … ADR-046 v1.22→v1.23". The hard-coded "v1.18 as of this row" literal self-contradicts the cell's own tail content. This is a NEW LOCUS of the already-codified O-P28-002 recurrence class and **falsifies O-P28-002's "version-stable by construction" claim** — the O-P28-002 fix made the ADR File-Change-Plan instruction row version-stable, but did NOT prevent the ARCH-INDEX output cell's embedded literal from going stale independently.
- **Evidence:** ARCH-INDEX.md ADR-046 row headline: `**RATIFIED 2026-08-25; ADR-046 v1.18 as of this row.**`; ADR-046 frontmatter: `version: "1.23"`; ARCH-INDEX cell tail: `…ADR-046 v1.22→**v1.23**`; Delta: marker says v1.18; reality is v1.23 (stale by 5: v1.19, v1.20, v1.21, v1.22, v1.23).
- **Proposed Fix:** Structural close per TD-VSDD-059 (NOT a paper-patch to v1.23 — that would restale on the next ADR touch). Replace the hard-coded literal with `**RATIFIED 2026-08-25; current version per ADR-046 frontmatter (tail records bump history).**` — this eliminates the sweep-every-touch requirement permanently. Owner: state-manager (ARCH-INDEX per POLICY 6 / Routing Table).

---

## Non-Blocking Observations

### O-P62-001 [out-of-perimeter → implementer, bound to S-17.05]

`crates/factory-lock/src/lib.rs` doc-comments (~lines 113, 158-160, 318) still describe stale pre-F-P56-001 semantics (empty/absent holder → Ok(None)/NoOp). Same locus as O-P61-001. **Disposition:** Update O-P61-001/O-P62-001 Drift Items status to BOUND to S-17.05 (human-directed 2026-08-27). Owner: implementer.

### O-P62-002 [LOW, awareness only]

Finding-ID provenance divergence: BC-4.17.001/BC-7.07.001 label the `classify_identity_resolution` mandate "F-003" while ADR-046 labels the identical decision "F-006". Substance identical; per-document remediation labels, not cross-artifact anchors. NOT a POLICY 4 mis-anchor. Record for awareness; do not re-raise.

### O-P62-003 [process-observation]

O-P28-002's "version-stable by construction" claim is falsified by F-P62-001. The structural fix (replacing the hard-coded literal) is the correct durable close.

---

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 1 (F-P62-001: ARCH-INDEX ADR-046 row headline stale) |
| LOW | 0 |

**Overall Assessment:** block — FINDINGS verdict; fix required before streak can resume.
**Convergence:** findings remain — BC-5.39.001 3-CLEAN streak RESETS 2/3 → 0/3 (9th reset; human-directed literal-3-CLEAN standard 2026-08-27; out-of-frozen-set finding still resets per human ruling). Fresh pass-63 NEXT after fix.
**Readiness:** requires ARCH-INDEX fix (F-P62-001) before fresh pass-63 dispatch.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 62 |
| **New findings** | 1 (F-P62-001: new locus of O-P28-002 class; falsifies O-P28-002 claim) |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 / 1.0 = 1.0 (single finding; new locus) |
| **Median severity** | MEDIUM |
| **Trajectory** | →0→1→0→1 (LENGTH=4, +1 this pass) |
| **Verdict** | FINDINGS_REMAIN |
