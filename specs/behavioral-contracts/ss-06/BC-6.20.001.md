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
capability: "CAP-001"
lifecycle_status: active
introduced: v1.0-brownfield-backfill
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-6.20.001
section: "6.20"
---

# BC-6.20.001: create-adr allocates next sequential ADR-NNN by scanning filesystem and ARCH-INDEX

## Description

The `create-adr` skill determines the next available ADR number by scanning both the filesystem (`decisions/ADR-*.md` filenames) and the ARCH-INDEX "Architecture Decisions" table rows. It takes the maximum of all found numbers and proposes `ADR-<max+1>` zero-padded to three digits. This dual-source scan is required so that neither source alone can silently advance the counter ahead of the other.

## Preconditions

1. The skill is invoked without an explicit `--id` override.
2. `plugins/vsdd-factory/skills/create-adr/../../../.factory/specs/architecture/decisions/` directory is accessible (may be empty — treated as max=0).
3. `.factory/specs/architecture/ARCH-INDEX.md` is readable and contains an `## Architecture Decisions` section.
4. The filesystem and ARCH-INDEX ID sets are consistent (same set of ADR-NNN IDs present in both).

## Postconditions

1. The skill proposes the ID `ADR-<N+1>` where N is the maximum ADR number found across both sources, zero-padded to three digits (e.g., current max ADR-013 → proposes ADR-014).
2. No file is written and no ARCH-INDEX row is inserted at this step — this is a read-only allocation step.
3. The proposed ID is passed to subsequent steps (file scaffolding, ARCH-INDEX insertion) as the canonical ID for this invocation.

## Invariants

1. The proposed ID is always strictly greater than any existing ADR-NNN found in either source.
2. Zero-padding is always three digits (ADR-001, not ADR-1).
3. If the decisions directory is empty and the ARCH-INDEX table has no rows, the skill proposes ADR-001.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `decisions/` directory does not exist | Skill creates the directory before scanning; treats filesystem count as zero; continues with ARCH-INDEX scan |
| EC-002 | ARCH-INDEX table has no ADR rows (empty table section) | Filesystem max used as sole source; if also empty, proposes ADR-001 |
| EC-003 | Filesystem has ADR-013, ARCH-INDEX table has ADR-012 (mismatch) | ID mismatch — skill reports inconsistency and refuses to proceed; see BC-6.20.003 |

## Canonical Test Vectors

| Input State | Expected Proposed ID | Category |
|-------------|----------------------|----------|
| Filesystem: ADR-001..ADR-013; ARCH-INDEX: ADR-001..ADR-013 | ADR-014 | happy-path |
| Filesystem: empty; ARCH-INDEX: empty | ADR-001 | happy-path |
| Filesystem: ADR-001..ADR-009; ARCH-INDEX: ADR-001..ADR-009 | ADR-010 | happy-path |
| decisions/ directory missing | ADR-001 (directory created) | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-059 | Allocated ADR-NNN is strictly greater than all existing IDs (ID monotonicity) | proptest over fixture state |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-001 |
| L2 Domain Invariants | none directly — correctness invariant local to skill |
| Architecture Module | plugins/vsdd-factory/skills/create-adr/SKILL.md |
| Stories | S-6.01 |
| Source AC | S-6.01 §AC-1 (test: `test_id_allocation_no_collision`) |
| FR | FR-041 |

## Related BCs

- BC-6.20.002 — depends on this BC (duplicate ID refusal uses same scan result)
- BC-6.20.003 — depends on this BC (mismatch detection uses same dual-source scan)

## Architecture Anchors

- `plugins/vsdd-factory/skills/create-adr/SKILL.md` — skill entry point
- `.factory/specs/architecture/ARCH-INDEX.md` — Architecture Decisions table (read)
- `.factory/specs/architecture/decisions/` — filesystem scan target

## Story Anchor

S-6.01

## VP Anchors

- VP-059 — ID monotonicity invariant
