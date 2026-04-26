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
extracted_from: ".factory/stories/S-6.01-create-adr-skill.md#AC-2"
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
bc_id: BC-6.20.005
section: "6.20"
---

# BC-6.20.005: create-adr validates subsystems_affected against ARCH-INDEX Subsystem Registry

## Description

The skill validates every entry in the user-supplied `--subsystems` array against the ARCH-INDEX Subsystem Registry (SS-01 through SS-10) before writing any file. Any entry that does not match a known SS-ID causes the skill to refuse with an error listing the invalid entries and the full set of valid SS-IDs. This prevents ADRs with phantom subsystem references from entering the architecture record.

## Preconditions

1. The user has supplied a `--subsystems` argument containing one or more SS-ID values.
2. The ARCH-INDEX Subsystem Registry section is readable and contains the canonical list of valid SS-IDs.

## Postconditions

1. **On validation success:** all supplied SS-IDs are present in the Subsystem Registry; skill proceeds to file write.
2. **On validation failure:** skill exits non-zero with message identifying the invalid SS-ID(s) and listing valid values; no file is written, no ARCH-INDEX row inserted.

## Invariants

1. Validation always runs against the live ARCH-INDEX Subsystem Registry, not a hardcoded list — so newly added subsystems are automatically accepted.
2. The error message on failure always enumerates both the invalid entries and the full valid set.
3. An empty `--subsystems` array (zero entries) is permitted; the field is written as an empty array in frontmatter.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `--subsystems SS-99` (unknown) | Refuses: "SS-99 is not a registered subsystem. Valid: SS-01, SS-02, ..., SS-10" |
| EC-002 | `--subsystems SS-01,SS-99` (one valid, one invalid) | Refuses — the entire array is rejected; both valid and invalid entries named in message |
| EC-003 | `--subsystems` not supplied | Field written as empty array `[]`; no validation error |
| EC-004 | `--subsystems SS-06,SS-08,SS-10` (all valid) | Proceeds; frontmatter `subsystems_affected: [SS-06, SS-08, SS-10]` |
| EC-005 | ARCH-INDEX Subsystem Registry section is missing | Skill exits non-zero: "Cannot validate subsystems — ARCH-INDEX Subsystem Registry section not found" |

## Canonical Test Vectors

| Input `--subsystems` | Expected Behavior | Category |
|----------------------|------------------|----------|
| `SS-06,SS-08,SS-10` | Proceeds; frontmatter written with array | happy-path |
| (not supplied) | Proceeds; `subsystems_affected: []` | happy-path |
| `SS-99` | Exit non-zero; lists valid IDs | error |
| `SS-01,SS-99` | Exit non-zero; names SS-99 as invalid | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (none allocated — covered by integration test suite) | Unknown SS-IDs always rejected | integration-test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-017 |
| Capability Anchor Justification | Anchored to CAP-017 (Create and manage formal ADR records) per capabilities.md §CAP-017 — literal match for ADR scaffolding. |
| L2 Domain Invariants | none directly |
| Architecture Module | plugins/vsdd-factory/skills/create-adr/SKILL.md |
| Stories | S-6.01 |
| Source AC | S-6.01 §AC-2 (test: `test_subsystems_validated_against_registry`) |
| FR | FR-041 |

## Related BCs

- BC-6.20.004 — sibling frontmatter scaffold (status field)
- BC-6.20.006 — sibling frontmatter validation (supersedes field)

## Architecture Anchors

- `plugins/vsdd-factory/skills/create-adr/SKILL.md` — skill entry point
- `.factory/specs/architecture/ARCH-INDEX.md` — Subsystem Registry (validation source)

## Story Anchor

S-6.01

## VP Anchors

- (none allocated — covered by integration test)
