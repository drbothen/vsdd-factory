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
bc_id: BC-6.20.006
section: "6.20"
---

# BC-6.20.006: create-adr validates --supersedes ADR-NNN exists before proceeding

## Description

When the user supplies `--supersedes ADR-NNN`, the skill verifies that the referenced ADR exists as a file matching `decisions/ADR-NNN-*.md` on the filesystem before writing any new file. If the referenced ADR does not exist, the skill refuses and exits non-zero. This prevents dangling `supersedes` references in the new ADR's frontmatter.

## Preconditions

1. The user has supplied `--supersedes ADR-NNN`.
2. The skill is about to begin the file-write phase.

## Postconditions

1. **On validation success:** `decisions/ADR-NNN-*.md` exists; the `supersedes: ADR-NNN` field is written into the new ADR's frontmatter; skill proceeds to supersession patch (BC-6.20.007).
2. **On validation failure:** `decisions/ADR-NNN-*.md` does not exist; skill exits non-zero with message "ADR-NNN does not exist at decisions/ADR-NNN-*.md. Cannot set supersedes."; no file is written.

## Invariants

1. Validation is filesystem-based (file existence), not ARCH-INDEX-based — the file must be present on disk.
2. The refusal happens before any write to the new ADR file or any patch to ARCH-INDEX.
3. A valid `--supersedes` reference is written as `supersedes: ADR-NNN` (not the filename slug, just the ID).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `--supersedes ADR-NNN` where ADR-NNN is already superseded (has `superseded_by` set) | Skill warns "ADR-NNN is already superseded by ADR-MMM" but does NOT block — double-supersession is valid during reorganization; proceeds |
| EC-002 | `--supersedes` not supplied | `supersedes: null` written in new ADR frontmatter; supersession patch step (BC-6.20.007) is skipped |
| EC-003 | `--supersedes ADR-000` (zero-padded edge case, likely non-existent) | Treated as normal existence check; fails if file not found |
| EC-004 | Filesystem check succeeds but ADR-NNN is absent from ARCH-INDEX | Skill proceeds (filesystem is authoritative for this check); inconsistency may be flagged by BC-6.20.003 during the initial scan |

## Canonical Test Vectors

| Input | Expected Behavior | Category |
|-------|------------------|----------|
| `--supersedes ADR-013` where `decisions/ADR-013-*.md` exists | Proceeds; `supersedes: ADR-013` written | happy-path |
| `--supersedes ADR-099` where ADR-099 does not exist | Exit non-zero; error message names ADR-099 | error |
| No `--supersedes` flag | Proceeds; `supersedes: null` in frontmatter | happy-path |
| `--supersedes ADR-013` where ADR-013 has `superseded_by: ADR-010` | Proceeds with warning (double-supersession allowed) | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (none allocated — covered by integration test suite) | Dangling supersedes reference always rejected | integration-test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-017 |
| Capability Anchor Justification | Anchored to CAP-017 (Create and manage formal ADR records) per capabilities.md §CAP-017 — literal match for ADR scaffolding. |
| L2 Domain Invariants | none directly |
| Architecture Module | plugins/vsdd-factory/skills/create-adr/SKILL.md |
| Stories | S-6.01 |
| Source AC | S-6.01 §AC-2 (test: `test_supersedes_validated_to_exist`) |
| FR | FR-041 |

## Related BCs

- BC-6.20.004 — sibling frontmatter scaffold (status/date fields)
- BC-6.20.005 — sibling frontmatter validation (subsystems_affected)
- BC-6.20.007 — supersession patch step that follows this validation

## Architecture Anchors

- `plugins/vsdd-factory/skills/create-adr/SKILL.md` — skill entry point
- `.factory/specs/architecture/decisions/` — filesystem existence check

## Story Anchor

S-6.01

## VP Anchors

- (none allocated — covered by integration test)
