---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-04-26T00:00:00
phase: 1a
inputs: [.factory/stories/S-7.03-tdd-discipline-hardening.md]
input-hash: "a361f34"
traces_to: .factory/stories/S-7.03-tdd-discipline-hardening.md
origin: brownfield
extracted_from: ".factory/stories/S-7.03-tdd-discipline-hardening.md#AC-010"
subsystem: "SS-06"
capability: "CAP-016"
lifecycle_status: active
introduced: v1.0-brownfield-backfill
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-6.21.002
section: "6.21"
---

# BC-6.21.002: mutation kill rate floor is 80%; surviving mutants must be addressed via test, dead-code confirmation, or explicit waiver

## Description

After `cargo mutants` runs for a facade-mode crate (BC-6.21.001), the kill rate (killed / total) must be ≥ 80%. If the kill rate is below 80%, the wave gate blocks and every surviving mutant must be dispositioned through one of exactly three options: (A) a new test is written that kills the mutant, (B) the mutant is confirmed dead-code-equivalent with explanation, or (C) the mutant is explicitly waived with rationale in the PR description. Undispositioned surviving mutants above the 20% allowance block wave gate merge.

## Preconditions

1. `cargo mutants` has been run per BC-6.21.001 and produced a mutation report.
2. The mutation report is available with total mutants, killed mutants, and surviving mutants list.
3. Kill rate = killed / total is computed.

## Postconditions

**If kill rate ≥ 80%:**
1. Wave gate mutation check passes.
2. Surviving mutants (≤20%) are noted in the mutation report but do not block.

**If kill rate < 80%:**
1. Wave gate BLOCKS.
2. For each surviving mutant above the 20% allowance, one of three dispositions must be applied:
   - **Disposition A (new test):** A new test is written that kills the mutant. The test must be committed and cargo mutants re-run to confirm the mutant is now killed.
   - **Disposition B (dead-code-equivalent):** The surviving mutant modifies code that is unreachable in all real execution paths OR produces an equivalent behavior under all inputs. A written explanation is required in the PR description.
   - **Disposition C (explicit waiver):** The surviving mutant is accepted as-is with a written rationale in the PR description acknowledging the coverage gap and its acceptable risk.
3. All surviving mutants must have a documented disposition before wave gate proceeds.

## Invariants

1. Kill rate floor is 80% (integer arithmetic: `killed * 100 / total >= 80`). No float precision grace.
2. Disposition B (dead-code-equivalent) requires a specific explanation, not a blanket "this code is unreachable." The explanation must cite the execution condition under which the code is unreachable.
3. Disposition C (explicit waiver) must name the specific mutant (source file, line, mutation type) — a blanket "all surviving mutants waived" is not acceptable.
4. After Disposition A (new test), cargo mutants must be re-run to confirm the mutant is killed. The re-run results replace the original in the mutation report.
5. Undispositioned surviving mutants (no A/B/C assigned) are equivalent to a kill rate violation and block wave gate.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Kill rate = 79% (1 mutant short of 80%) | BLOCKS. One additional test or disposition required. |
| EC-002 | Kill rate = 80% exactly | PASSES (≥ 80% inclusive). |
| EC-003 | Total mutants = 0 (crate has no mutable logic) | Kill rate = 100% vacuously. Wave gate passes with note "no mutants generated." |
| EC-004 | Kill rate = 0% (all mutants survive — catastrophic test gap) | BLOCKS with severity HIGH. All surviving mutants need disposition. Automated agents should flag for human review rather than attempting mass waivers. |
| EC-005 | Disposition A produces a regression in another test | The new test has a defect; fix the test. Do not waive the mutant to avoid writing a correct test. |

## Canonical Test Vectors

| Total Mutants | Killed | Surviving | Kill Rate | Decision |
|---------------|--------|-----------|-----------|----------|
| 50 | 42 | 8 | 84% | PASS (≥80%) |
| 50 | 39 | 11 | 78% | BLOCK (1 above 20% allowance) |
| 50 | 40 | 10 | 80% | PASS (boundary) |
| 10 | 0 | 10 | 0% | BLOCK HIGH-severity |
| 0 | 0 | 0 | 100% vacuous | PASS with note |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-064 | Kill rate threshold check is applied at wave gate and blocks correctly | manual (procedural verification) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-016 |
| Capability Anchor Justification | CAP-016 ("Drive TDD delivery with red/green/refactor gate enforcement") per capabilities.md §CAP-016 — this BC operationalizes the quality threshold for mutation testing that CAP-016's compensating control requires: the 80% floor ensures facade-mode stories maintain meaningful test coverage assurance comparable to a genuine red/green cycle. |
| L2 Domain Invariants | none |
| Architecture Module | plugins/vsdd-factory/skills/wave-gate/SKILL.md |
| Stories | S-7.03 |
| Source AC | S-7.03 §AC-010 |
| FR | FR-043 |

## Related BCs

- BC-6.21.001 — parent (execution of cargo mutants; this BC governs the threshold enforcement)
- BC-8.30.002 — depends on (facade mode that creates the mutation testing obligation)
- BC-8.29.003 — depends on (Option B creates the same obligation through a different path)

## Architecture Anchors

- `plugins/vsdd-factory/skills/wave-gate/SKILL.md` — kill rate threshold and disposition protocol

## Story Anchor

S-7.03

## VP Anchors

- VP-064 — facade-mode mutation gate enforcement
