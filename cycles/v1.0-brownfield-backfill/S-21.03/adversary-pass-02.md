---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-07-24T00:00:00Z
phase: 5
inputs: []
input-hash: "[live-state]"
traces_to: ".factory/stories/STORY-S-21.03-pr-manager-orphan-merge.md"
pass: 2
previous_review: "adversary-pass-01.md"
story: S-21.03
cycle: v1.0-brownfield-backfill
verdict: NOT-CLEAN
reviewed_head: 5569a7ee
reviewed_branch: feature/S-21.03-pr-manager-orphan-merge
base_commit: a4a79f09
date: 2026-07-24
---

# S-21.03 LOCAL Adversary Pass-2 — NOT-CLEAN

**Date:** 2026-07-24
**Story:** S-21.03 — PR-manager orphan-merge handling
**Pass:** 2 of BC-5.39.001 cascade
**Result:** NOT-CLEAN — streak 0/3
**Severity breakdown:** B0 / H1 / M2 / L0 / NITPICK0 / OBS0
**Total findings:** 3 findings
**Reviewed diff:** HEAD 5569a7ee on feature/S-21.03-pr-manager-orphan-merge vs base a4a79f09

---

## Finding ID Convention

Finding IDs for this story's local cascade use the format: `F-S2103-P<PASS>-<SEQ>`

- `F`: Fixed prefix for factory local adversary findings
- `S2103`: Story identifier (S-21.03 compact form)
- `P<PASS>`: Pass number (e.g., `P1`, `P2`)
- `<SEQ>`: Three-digit sequence within the pass (e.g., `001`)

Examples: `F-S2103-P2-001` (MEDIUM finding, pass 2, first finding), `F-S2103-P2-002` (HIGH finding, pass 2, second finding).

---

## Part A — Prior Pass Finding Resolution

### F-S2103-P1-001 — Load-bearing ancestry assertion placed after destructive branch deletion

- **Pass-1 verdict:** MEDIUM
- **Pass-2 resolution:** PARTIALLY-FIXED
- **Evidence:** Structural relocation is confirmed — the ancestry assertion now executes before branch deletion steps 8b/8c/8d. However, the `--delete-branch` flag at merge time (Step 8-pre-B) deletes the remote head at the point GitHub executes the merge, falsifying the "intact remote head" recovery promise that the assertion relocation was intended to preserve. The structural reorder is real, but the recovery affordance is defeated.
- **Carry-forward finding:** F-S2103-P2-001 (MEDIUM)

---

### F-S2103-P1-002 — Harness is divergent parallel algorithm: direction, message-body, and consequence parity not tested

- **Pass-1 verdict:** MEDIUM
- **Pass-2 resolution:** PARTIALLY-FIXED
- **Evidence:** PC2 leg (direction assertions: ALLOW/BLOCK based on ancestry) confirmed fixed. PC3 leg defeated by relocation: the doc-parity grep tests for the presence of the ancestry assertion by searching for "Step 9" back-references (the historical placement). After relocation to Step 8-post-A, the Step 9 greps still hit the historical back-reference strings ("Step 9 → Step 8-post-A" changelog cites), not the relocated mandate itself. The harness therefore passes on a salted copy where the load-bearing Step 8-post-A mandate is deleted — a false-green on a P0 guard.
- **Carry-forward finding:** F-S2103-P2-002 (HIGH; TD-VSDD-059 class)

---

### F-S2103-P1-003 — Harness JSON regex requires no-space-after-colon; real gh output pretty-prints with spaces

- **Pass-1 verdict:** LOW
- **Pass-2 resolution:** CONFIRMED-FIXED
- **Evidence:** Fixture stubs now use realistic pretty-printed JSON matching actual `gh` output format; parser accepts whitespace-tolerant input.

---

### OBS-1 — Trunk hardcoded to `develop`; BC says "configured trunk"

- **Pass-1 verdict:** LOW (informational)
- **Pass-2 resolution:** CONFIRMED-FIXED
- **Evidence:** Trunk now resolved from `git config vsdd.trunk-branch` with fallback to `develop`; aligned with BC phrasing and Merge-Strategy Gate coherence.

---

### OBS-2 — POLICY 21 detection is .sh-extension-based; new extensionless shims satisfy letter but evade detector [process-gap]

- **Pass-1 verdict:** LOW (informational; process-gap)
- **Pass-2 resolution:** Deferred to merge gate (human decision required). Not a story-scope blocker. Governance question remains open; queued for human at S-21.03 merge gate per pass-1 orchestrator disposition.

---

### OBS-3 — Step 9 stale claim about remote branch deletion verification (pre-existing, in-scope)

- **Pass-1 verdict:** LOW (informational; pre-existing)
- **Pass-2 resolution:** CONFIRMED-FIXED
- **Evidence:** Step 9 narrative updated to accurately describe Step 8c `--exit-code` semantics; explicit clauses added for protection-rejected and fork-PR cases.

---

## Part B — New Findings

### HIGH

#### F-S2103-P2-002 — PC3 doc-parity greps salt Step 9 back-reference, not the relocated mandate; false-green on P0 guard

- **Severity:** HIGH
- **Category:** coverage-gap
- **Location:** bats test file (PC3 doc-parity assertions) / TD-VSDD-059 class
- **Description:** After the Step 9 → Step 8-post-A relocation, the doc-parity grep assertions in the bats harness test for the presence of the ancestry assertion by pattern-matching against "Step 9" strings. Following relocation, the script contains legitimate "Step 9" back-reference strings (changelog annotations such as "Step 9 → Step 8-post-A") that satisfy the grep, regardless of whether the actual Step 8-post-A mandate is present. The harness therefore produces a false-green when the load-bearing Step 8-post-A block is deleted from the script: the changelog back-reference "Step 9 → Step 8-post-A" keeps the grep passing, but the operative guard is absent. This is a TD-VSDD-059 class paper-fix: renaming/annotation-only assertions that pass even when the structural implementation is removed.
- **Evidence:** A salted copy of pr-manager.sh with the Step 8-post-A block deleted was verified to pass T-003, T-004, and T-005 (the doc-parity assertions), while the Step 8-post-A mandate was absent. The greps match the historical back-reference string, not the structural mandate.
- **Fix:** Repoint doc-parity greps from "Step 9" pattern to the actual Step 8-post-A structural marker (e.g., `_extract_step8_post_a_section` extraction function or an ordering assertion that verifies the ancestry check line appears before the deletion invocation). Add a MUTATION-VERIFIED regression property: a salted copy with Step 8-post-A deleted must fail T-003/T-004/T-005.
- **Routing:** test-writer (repoint doc-parity greps + ordering assertion + mutation verification)
- **Status:** FIXED at 9bd75ab3 — `_extract_step8_post_a_section` extraction function added; ordering assertion verifies ancestry check precedes deletion invocation; MUTATION-VERIFIED: salted-copy with Step 8-post-A deleted fails T-003/T-004/T-005.

---

### MEDIUM

#### F-S2103-P2-001 — `--delete-branch` at Step 8-pre-B deletes remote head at merge time, defeating recovery affordance

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** pr-manager.sh Step 8-pre-B (gh pr merge invocation)
- **Description:** The `gh pr merge` call at Step 8-pre-B includes the `--delete-branch` flag. This flag instructs GitHub to delete the remote head branch at the point of merge execution — before the subsequent Steps 8b/8c/8d explicit deletion sequence. The F-P1-001 fix relocated the ancestry assertion to execute before branch deletion, preserving the "intact remote head" recovery affordance. However, `--delete-branch` at Step 8-pre-B defeats this affordance: the remote head is gone by the time the ancestry check executes, because GitHub processes the deletion as part of the merge event. The operator recovery path documented in the orphan-merge handling block (which assumes the remote head is intact for `git reset` and force-push recovery) is therefore unavailable.
- **Evidence:** Verified via `gh pr merge` documentation: `--delete-branch` causes the remote branch to be deleted server-side at merge time. The explicit deletion steps (8b/8c/8d) execute after merge confirmation, but the remote branch is already absent from origin. Feasibility verified: `enforce-merge-strategy.sh` accepts `--delete-branch` as an optional residual argument; dropping it from the invocation is a minimal-scope fix.
- **Fix:** Drop `--delete-branch` from the Step 8-pre-B `gh pr merge` invocation. Gate branch deletion solely through Steps 8b/8c/8d, which execute after the Step 8-post-A ancestry check. This preserves the recovery affordance: if Step 8-post-A fires BLOCK, the remote head is still intact and the operator can follow the documented recovery path.
- **Routing:** implementer (drop `--delete-branch` from Step 8-pre-B; gate deletion on Step 8-post-A pass)
- **Status:** FIXED at ad4e2bc0 — feasibility verified (`--delete-branch` is an optional residual arg in `enforce-merge-strategy.sh`); invocation drops `--delete-branch`; Steps 8b/8c/8d are the sole deletion mechanism and execute only after Step 8-post-A passes.

---

#### F-S2103-P2-003 — Story and ADR-031 §Decision 8 cite "Step 9 amendment" vs correct Step 8-post-A placement

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** S-21.03 story spec §Architecture Compliance Rules, §Tasks, §Purity Classification, §File Structure; ADR-031 §Decision 8 mandate text
- **Description:** The S-21.03 story spec and ADR-031 §Decision 8 mandate text continued to reference the ancestry assertion as a "Step 9 amendment" and "post-merge Step 9 assertion" after the F-P1-001 fix relocated the assertion to Step 8-post-A (before branch deletion). The specification documents are therefore factually incorrect with respect to the current implementation: the assertion is at Step 8-post-A, not Step 9. This creates a divergence between spec and code that violates BC-6.10.002 PC3 "immediately after state: MERGED" placement mandate — the spec text implies a position that the code no longer implements.
- **Adjudication:** Orchestrator adjudicated as spec-alignment propagation, NOT a code-driven spec change. The canonical placement is BC-6.10.002 PC3 "immediately after state: MERGED" — this mandates Step 8-post-A. The implementation was corrected first (F-P1-001); the spec docs must catch up. Precedent class: ADR-031 v1.6 / OBS-P5-1 (spec-follows-canonical-BC). Routed to story-writer + architect for propagation.
- **Routing:** story-writer (S-21.03 story spec §Architecture Compliance Rules, §Tasks, §Purity Classification, §File Structure); architect (ADR-031 §Decision 8 mandate text)
- **Status:** FIXED at cdc60e8c (story v1.4 — all non-historical Step 9 placement cites corrected to Step 8-post-A; ADR-031 v1.3 §Decision 8 cite updated) + c97faaae (ADR-031 v1.7 — §Decision 8 ancestry-assertion placement Step 8-post-A).

---

### Clean Axes Verified

- **Scope discipline:** Files touched confined to pr-manager.sh, bats test file, story spec, and ADR-031. No lateral scope expansion.
- **TD-VSDD-091:** All spec narrative cites use behavioral anchors (BC-6.10.002 clause identifiers, step labels) rather than line numbers.
- **POLICY 19:** No hardcoded credentials or secrets in fixture stubs.
- **F-P1-003 CONFIRMED-FIXED:** JSON fixture stubs updated to realistic pretty-printed format; parser whitespace-tolerant.
- **OBS-1 CONFIRMED-FIXED:** Trunk resolved dynamically from config; no hardcoded `develop`.
- **OBS-3 CONFIRMED-FIXED:** Step 9 stale claim corrected to Step 8c `--exit-code` semantics.
- **Novelty:** HIGH (second-order consequences of the pass-1 F-001 relocation; three independent failure modes surfaced from the structural reorder).

---

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER  | 0 |
| HIGH     | 1 |
| MEDIUM   | 2 |
| LOW      | 0 |
| NITPICK  | 0 |
| OBS      | 0 |

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Readiness:** requires revision (fix burst dispatched; all findings FIXED prior to this persist)

**Fix burst dispatch (concurrent):**
1. **test-writer** — F-S2103-P2-002 (repoint doc-parity greps + ordering assertion + mutation verification): FIXED at 9bd75ab3
2. **implementer** — F-S2103-P2-001 (drop --delete-branch from Step 8-pre-B): FIXED at ad4e2bc0
3. **story-writer + architect** — F-S2103-P2-003 (story v1.4 + ADR-031 v1.7 Step 8-post-A placement): FIXED at cdc60e8c + c97faaae

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 2 |
| **New findings** | 3 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (3 / (3 + 0)) |
| **Median severity** | HIGH–MEDIUM boundary |
| **Trajectory** | 3→3 |
| **Verdict** | FINDINGS_REMAIN |
| **Novelty class** | HIGH — second-order consequences of pass-1 F-001 relocation; three independent failure modes surfaced from the structural reorder |
