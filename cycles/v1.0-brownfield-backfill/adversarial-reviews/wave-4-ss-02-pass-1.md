---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-26T00:00:00Z
phase: 2-re-anchor
inputs:
  - .factory/stories/S-1.03-hook-sdk-crate.md
  - .factory/stories/S-2.05-hook-sdk-publish.md
  - .factory/specs/prd.md
  - .factory/specs/domain-spec/capabilities.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/specs/verification-properties/VP-INDEX.md
  - .factory/specs/verification-properties/VP-038.md
input-hash: "4bdaf5a"
traces_to: ".factory/specs/prd.md#FR-009"
cycle: v1.0-brownfield-backfill
sub_cycle: wave-4-ss-02-re-anchor
pass: 1
previous_review: null
po_commit_reviewed: 4bdaf5a
verdict: FINDINGS_REMAIN
finding_count: 7
convergence_step: 0_of_3
---

# Adversarial Review — Wave 4 SS-02 Re-anchor — Pass 1

## Finding ID Convention

Finding IDs use the format: `ADV-W4SS02-P1-<SEV>-<SEQ>`.

## Part B — New Findings (7 total: 1 CRIT, 3 HIGH, 3 MED, 0 LOW)

### ADV-W4SS02-P1-CRIT-001 [CRITICAL] — VP-038 body Traceability dropped S-3.03 anchor (POLICY 1 append-only violation)

- **Severity:** CRITICAL
- **Category:** spec-fidelity
- **Policy:** POLICY 1 (append_only_numbering / anchor preservation)
- **Location:** `.factory/specs/verification-properties/VP-038.md:95`
- **Description:** Wave 4 state-manager VP propagation (commit 4bdaf5a) overwrote rather than appended the Stories field in VP-038.md, losing the S-3.03 anchor established in Wave 3.
- **Evidence:** VP-038.md:95 reads `Stories: S-1.03 ...` only. S-3.03.md:23-24 declares `verification_properties: [VP-038]`. VP-INDEX.md:145 also shows only S-1.03.
- **Proposed Fix:** VP-038.md:95 → `Stories: S-3.03 (Wave 3 SS-04 baseline; BC-2.01.002 anchor for block-ai-attribution exit-code), S-1.03 (hook-sdk-crate, Wave 4 SS-02 baseline; bcs BC-2.01.001-003 in 22-BC anchor set)`. Preserves both per POLICY 1.

### ADV-W4SS02-P1-HIGH-001 [HIGH] — 22 SS-02 BC files retain CAP-TBD frontmatter and TBD Traceability after BC-INDEX update

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Policy:** POLICY 8 (bc_array_changes_propagate_to_body_and_acs); POLICY 4
- **Location:** all 22 SS-02 BC files in `.factory/specs/behavioral-contracts/ss-02/`
- **Description:** PO baseline 3c50b6f flipped BC-INDEX rows but BC files still claim CAP-TBD. Three contradictory signals downstream (BC-INDEX vs frontmatter vs body).
- **Evidence (sampled):**
  - BC-2.01.001.md:15 — `capability: "CAP-TBD"` (BC-INDEX shows CAP-009)
  - BC-2.01.001.md:71-74 — Traceability `L2 Capability TBD`, `Stories TBD`
  - BC-2.02.005.md, BC-2.04.005.md, BC-2.05.003.md — same pattern
- **Proposed Fix:** Update each of 22 BC files: frontmatter `capability:` `"CAP-TBD"` → `"CAP-009"`; body Traceability `L2 Capability TBD` → `CAP-009`; `Stories TBD` → `S-1.03` (and `S-3.03` for BC-2.01.002).

### ADV-W4SS02-P1-HIGH-002 [HIGH] — Bidirectional dep violation: S-3.01/S-3.02/S-3.03 missing S-1.03 in depends_on

- **Severity:** HIGH
- **Category:** missing-edge-cases
- **Policy:** Story dependency-graph integrity (BC-5.06.015)
- **Location:** S-1.03.md:19, S-3.01.md:19, S-3.02.md:19, S-3.03.md:19
- **Description:** Three stories asymmetric. WASM port plugins directly depend on vsdd-hook-sdk via Cargo.toml — direct S-1.03 dependency. Wave 3 SS-04 pass-6 deferred this as Observation; now compounds.
- **Evidence:** S-1.03.blocks=[S-2.01, S-2.05, S-3.01, S-3.02, S-3.03]; S-3.01-03.depends_on=[S-2.08, S-3.04] (no S-1.03). Compare S-2.01.depends_on=[S-1.03, ...] which IS bidirectional.
- **Proposed Fix:** Add S-1.03 to depends_on of S-3.01, S-3.02, S-3.03. `(pending intent verification)`.

### ADV-W4SS02-P1-HIGH-003 [HIGH] — VP-INDEX §Story Anchors VP-038 row lost S-3.03

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Policy:** POLICY 1 + POLICY 9
- **Location:** `.factory/specs/verification-properties/VP-INDEX.md:145`
- **Description:** Single row `| VP-038 | S-1.03 | Wave 4 SS-02 | ...`. POLICY 1 explicitly stated at VP-INDEX.md:138. Same root cause as CRIT-001 (different file, separate fix locus).
- **Evidence:** VP-INDEX.md:145 shows single anchor row for VP-038 citing only S-1.03.
- **Proposed Fix:** Split into two anchor records:
  ```
  | VP-038 | S-3.03 | Wave 3 SS-04 | S-3.03 anchors BC-2.01.002 for WASM block-ai-attribution exit-code |
  | VP-038 | S-1.03 | Wave 4 SS-02 | VP-038 bcs [BC-2.01.001-003] are in S-1.03's 22-BC anchor set |
  ```

### ADV-W4SS02-P1-MED-001 [MEDIUM] — AC-006 trace gap: missing VP-025 enumerator

- **Severity:** MEDIUM
- **Category:** verification-gaps
- **Policy:** POLICY 4 (semantic_anchoring_integrity)
- **Location:** S-1.03.md:113-116 (AC-006)
- **Description:** AC-006 enumerates 10 host functions but cites only BC-2.02.001/002 (which contract surface visibility, not the function set). VP-025 is the canonical enumerator. AC-014 cites VP-025 correctly.
- **Evidence:** AC-006 body enumerates 10 host functions; BC-2.02.001/002 scope is surface-visibility only; VP-025 is the ABI enumerator; AC-014 already cites VP-025.
- **Proposed Fix:** Append `(traces to VP-025 — Host Function ABI Surface enumerates the 10-fn set)` to AC-006.

### ADV-W4SS02-P1-MED-002 [MEDIUM] — S-1.03 status=merged vs S-2.05 publish=partial; anchor-justification ambiguity

- **Severity:** MEDIUM
- **Category:** ambiguous-language
- **Policy:** POLICY 5
- **Location:** S-1.03.md:7, S-2.05.md:7+46-48
- **Description:** S-1.03 status=merged refers to source-tree merging in beta.4; S-2.05 status=partial because crates.io publish deferred. Same target_module=crates/hook-sdk could confuse readers.
- **Evidence:** Both stories reference crates/hook-sdk; S-1.03 status=merged, S-2.05 status=partial with overlapping module scope.
- **Proposed Fix:** Add note to S-1.03 body: "merged" = source-tree shipping in beta.4, not crates.io distribution (S-2.05 scope). `(pending intent verification)`.

### ADV-W4SS02-P1-MED-003 [MEDIUM] — S-2.05 CAP-009 anchor justification doesn't disclose partial v1.0 coverage

- **Severity:** MEDIUM
- **Category:** ambiguous-language
- **Policy:** POLICY 5
- **Location:** S-2.05.md:138-140
- **Description:** capabilities.md:53-55 declares CAP-009 as "Author **and publish**". S-1.03 covers "author" half (22 BCs); S-2.05 covers "publish" half but with zero v1.0 BCs (deferred to v1.1 BC-2.06.001/002). Justification doesn't make partial coverage explicit.
- **Evidence:** CAP-009 definition = "Author and publish"; S-2.05 bcs:[] (zero v1.0 BCs); v1.1 candidates BC-2.06.001/002 declared.
- **Proposed Fix:** Strengthen S-2.05.md:138-140: "S-2.05 covers the 'publish' half of CAP-009 ('Author and publish'). The 'author' half is fully BC-anchored by S-1.03. The 'publish' half has no v1.0 BC — see v1.1 candidates BC-2.06.001/002. CAP-009 is therefore only partially BC-covered at v1.0."

## Observations

- [process-gap] BC-INDEX↔BC-files propagation discipline gap recurring (Wave 3 deferred, Wave 4 pass-1 finding). Recommend PO sibling-sweep: when BC-INDEX rows updated en masse, same burst MUST update underlying BC files' frontmatter capability + body Traceability cells.
- [process-gap] Bidirectional `depends_on`↔`blocks` symmetry deferred Wave 3 SS-04 pass-6 → Wave 4 pass-1 HIGH. Two consecutive waves = process gap. Recommend state-manager/PO sibling-sweep on `blocks:` modifications.
- [INFO] PRD §8 CAP-009 ↔ capabilities.md ↔ ARCH-INDEX coherent (SS-02 only). Subsystem registry verbatim. F-104 stretch-anchor pattern absent (all 22 BCs directly contracted). Wave 1+2+3 sibling sweep clean (S-1.01.blocks contains S-1.03 preserved; S-3.03.bcs preserved).
- [NIT] S-2.05 Token Budget omits BC count — acceptable since bcs:[] intentional.
- [NIT] S-1.03 Architecture Compliance Rules cites range refs (BC-2.02 SDK host module); style preference; not a defect.

## Sweep Results — Per-Axis

| Policy | Status |
|--------|--------|
| POLICY 1 (append-only) | VIOLATION (CRIT-001, HIGH-003) |
| POLICY 4 (semantic anchoring) | partial violation (HIGH-001, MED-001) |
| POLICY 5 (creators_justify_anchors) | minor gap (MED-003) |
| POLICY 6 (subsystem-registry verbatim) | CLEAN |
| POLICY 7 (BC H1 source-of-truth) | CLEAN (sample) |
| POLICY 8 (frontmatter↔body propagation) | partial violation (HIGH-001) |
| POLICY 9 (VP-INDEX coherence) | partial violation (HIGH-003) |
| POLICY 10 (demo evidence) | N/A |
| Bidirectional dep symmetry | VIOLATION (HIGH-002) |
| CAP-009 ↔ Subsystems | CLEAN |
| FR-009 sole/co-anchored | CLEAN |
| F-104 stretch-anchor | CLEAN |
| Status field appropriateness | MED-002 raised |

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 1 |
| HIGH | 3 |
| MEDIUM | 3 |
| LOW | 0 |

**Overall Assessment:** pass-with-findings
**Convergence:** findings remain — iterate
**Readiness:** requires revision

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 1 |
| **New findings count** | 7 |
| **Duplicate count** | 0 |
| **Novelty score** | 1.0 |
| **Median severity** | HIGH |
| **Severity distribution** | 1 CRIT, 3 HIGH, 3 MED, 0 LOW |
| **Trajectory** | pass-1 = 7 (baseline) |
| **Verdict** | FINDINGS_REMAIN |

## Convergence Status

**0 of 3.** 1 CRIT + 3 HIGH findings block convergence per BC-5.04.003. Expected baseline range was 7-12; actual 7 at low end (smaller subsystem; pre-existing hygiene patterns).

## Trajectory Baseline (Wave 4 SS-02)

| Pass | Total | CRIT | HIGH | MED | LOW |
|------|-------|------|------|-----|-----|
| 1 | 7 | 1 | 3 | 3 | 0 |
