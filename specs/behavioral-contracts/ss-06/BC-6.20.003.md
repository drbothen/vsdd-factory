---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-04-25T00:00:00
phase: 1a
inputs: [.factory/stories/S-6.01-create-adr-skill.md]
input-hash: "d81e07b"
traces_to: .factory/stories/S-6.01-create-adr-skill.md
origin: greenfield
extracted_from: ".factory/stories/S-6.01-create-adr-skill.md#AC-1"
subsystem: "SS-06"
capability: "CAP-017"
lifecycle_status: active
introduced: v1.0-brownfield-backfill
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-6.20.003
section: "6.20"
---

# BC-6.20.003: create-adr blocks on filesystem-vs-ARCH-INDEX ID mismatch

## Description

Before allocating a new ADR number, the skill compares the set of ADR IDs found in the `decisions/` filesystem directory against the set of ADR IDs in the ARCH-INDEX "Architecture Decisions" table. If these two sets differ in any way (an ID present in one source but absent from the other), the skill reports the specific inconsistency, exits non-zero, and writes nothing. This check prevents the skill from silently advancing the counter on top of a broken registry state.

## Preconditions

1. The skill has completed scanning both `decisions/ADR-*.md` filenames and ARCH-INDEX table rows.
2. The two ID sets are not equal (symmetric difference is non-empty).

## Postconditions

1. The skill exits non-zero.
2. The error message identifies which IDs are present in one source but absent from the other (e.g., "ADR-013 found in filesystem but missing from ARCH-INDEX").
3. No ADR file is written.
4. ARCH-INDEX is not modified.
5. No supersession patch is applied.

## Invariants

1. The mismatch check always runs before any write operation.
2. The error message always enumerates the specific mismatched IDs (not just "mismatch detected").
3. The skill never auto-heals the mismatch — it always requires manual user reconciliation.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | ARCH-INDEX has ADR-013 row but `decisions/ADR-013-*.md` does not exist | Error: "ADR-013 found in ARCH-INDEX but missing from filesystem. Reconcile manually before proceeding." |
| EC-002 | `decisions/ADR-013-foo.md` exists but ARCH-INDEX has no ADR-013 row | Error: "ADR-013 found in filesystem but missing from ARCH-INDEX. Reconcile manually before proceeding." |
| EC-003 | Multiple mismatches (ADR-009 and ADR-013 both inconsistent) | All mismatched IDs enumerated in one error message; single non-zero exit |
| EC-004 | Both sources empty (no decisions yet) | Not a mismatch — empty == empty; skill proceeds to allocate ADR-001 |

## Canonical Test Vectors

| Input State | Expected Output | Category |
|-------------|----------------|----------|
| Filesystem: {ADR-001..ADR-013}; ARCH-INDEX: {ADR-001..ADR-012} | Exit non-zero; message names ADR-013 as filesystem-only | error |
| Filesystem: {ADR-001..ADR-012}; ARCH-INDEX: {ADR-001..ADR-013} | Exit non-zero; message names ADR-013 as index-only | error |
| Filesystem: {}; ARCH-INDEX: {} | Proceeds; proposes ADR-001 | happy-path |
| Filesystem: {ADR-001..ADR-013}; ARCH-INDEX: {ADR-001..ADR-013} | Proceeds; proposes ADR-014 | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-059 | ID monotonicity requires consistent dual-source state | proptest over fixture state |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-017 |
| Capability Anchor Justification | Anchored to CAP-017 (Create and manage formal ADR records) per capabilities.md §CAP-017 — literal match for ADR scaffolding. |
| L2 Domain Invariants | none directly |
| Architecture Module | plugins/vsdd-factory/skills/create-adr/SKILL.md |
| Stories | S-6.01 |
| Source AC | S-6.01 §AC-1 (test: `test_id_allocation_filesystem_vs_index_mismatch_blocks`) |
| FR | FR-041 |

## Related BCs

- BC-6.20.001 — this BC is triggered when the scan in BC-6.20.001 finds a discrepancy
- BC-6.20.002 — sibling refusal path (duplicate rather than mismatch)

## Architecture Anchors

- `plugins/vsdd-factory/skills/create-adr/SKILL.md` — skill entry point
- `.factory/specs/architecture/ARCH-INDEX.md` — Architecture Decisions table (consistency source)
- `.factory/specs/architecture/decisions/` — filesystem consistency source

## Story Anchor

S-6.01

## VP Anchors

- VP-059 — ID monotonicity invariant
