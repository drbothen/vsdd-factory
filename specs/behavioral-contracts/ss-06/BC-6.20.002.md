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
bc_id: BC-6.20.002
section: "6.20"
---

# BC-6.20.002: create-adr refuses explicit --id override that already exists

## Description

When the user supplies an explicit `--id ADR-NNN` flag and that ID already exists (either as a file in `decisions/` or as a row in ARCH-INDEX), the skill refuses with a specific error message and exits non-zero without writing any file or modifying ARCH-INDEX. The refusal message names the conflicting path so the user can take corrective action.

## Preconditions

1. The skill is invoked with an explicit `--id ADR-NNN` override.
2. The named `ADR-NNN` already exists as `decisions/ADR-NNN-*.md` on the filesystem, or already appears as a row in the ARCH-INDEX "Architecture Decisions" table, or both.

## Postconditions

1. The skill exits non-zero.
2. The error message matches: `"ADR-NNN already exists at decisions/ADR-NNN-*.md. Omit --id to auto-allocate or choose a free ID."`
3. No new ADR file is written.
4. ARCH-INDEX is not modified.
5. No supersession patch is applied to any existing ADR.

## Invariants

1. The refusal check happens before any file write or ARCH-INDEX mutation.
2. The error message always names the conflicting artifact path.
3. Exit code is always non-zero on refusal.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `--id` names an ID present in ARCH-INDEX but no file on disk | Skill still refuses — index presence is sufficient to detect collision |
| EC-002 | `--id` names an ID present on disk but absent from ARCH-INDEX | Skill still refuses — filesystem presence is sufficient; inconsistency is also reported (see BC-6.20.003) |
| EC-003 | `--id` names a well-formed but free ID (e.g., ADR-099 where max is ADR-013) | Skill accepts and proceeds with the user-supplied ID |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `--id ADR-013` where ADR-013 exists on disk and in ARCH-INDEX | Exit non-zero; message contains "ADR-013 already exists" | error |
| `--id ADR-013` where ADR-013 exists only in ARCH-INDEX | Exit non-zero; refusal message | error |
| `--id ADR-099` where max existing is ADR-013 | Proceeds normally with ADR-099 | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-059 | ID monotonicity — no duplicate allocation | proptest over fixture state |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-017 |
| Capability Anchor Justification | Anchored to CAP-017 (Create and manage formal ADR records) per capabilities.md §CAP-017 — literal match for ADR scaffolding. |
| L2 Domain Invariants | none directly |
| Architecture Module | plugins/vsdd-factory/skills/create-adr/SKILL.md |
| Stories | S-6.01 |
| Source AC | S-6.01 §AC-1 (test: `test_id_allocation_refuses_duplicate`) |
| FR | FR-041 |

## Related BCs

- BC-6.20.001 — this BC depends on the same dual-source scan described there
- BC-6.20.003 — sibling refusal path (mismatch rather than duplicate)

## Architecture Anchors

- `plugins/vsdd-factory/skills/create-adr/SKILL.md` — skill entry point
- `.factory/specs/architecture/ARCH-INDEX.md` — Architecture Decisions table (collision check)
- `.factory/specs/architecture/decisions/` — filesystem collision check

## Story Anchor

S-6.01

## VP Anchors

- VP-059 — ID monotonicity invariant
