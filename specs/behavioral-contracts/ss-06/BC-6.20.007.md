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
extracted_from: ".factory/stories/S-6.01-create-adr-skill.md#AC-3"
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
bc_id: BC-6.20.007
section: "6.20"
---

# BC-6.20.007: create-adr bidirectionally patches old ADR's superseded_by on supersession

## Description

When `supersedes: ADR-NNN` is set in the new ADR, the skill patches the frontmatter of the referenced old ADR file to set `superseded_by: ADR-<new>`. The new file write and the old-file patch are treated as a single atomic unit: if the patch fails for any reason (file missing after initial check, frontmatter malformed, write permission error), neither file is persisted and the skill rolls back to clean state and exits non-zero. The patch modifies only the `superseded_by` line in the old ADR's frontmatter.

## Preconditions

1. `--supersedes ADR-NNN` was supplied and validated (BC-6.20.006 passed).
2. The new ADR file has been written to disk (or is staged for write within an atomic transaction).
3. `decisions/ADR-NNN-*.md` (the old ADR) is readable and writable.
4. The old ADR's frontmatter contains a `superseded_by:` field (either `null` or an existing value).

## Postconditions

1. **On success:** the old ADR's frontmatter contains `superseded_by: ADR-<new>` where `<new>` is the newly allocated ID. No other content of the old ADR is modified.
2. **On patch failure:** both the new ADR file and any ARCH-INDEX row insertion are reverted; old ADR is left in its pre-invocation state; skill exits non-zero identifying the failure step.

## Invariants

1. The patch modifies exactly one field in the old ADR frontmatter: `superseded_by`.
2. The patch never rewrites section bodies, other frontmatter fields, or whitespace/indentation outside the `superseded_by` line.
3. Atomicity: either both the new file and the patch are applied, or neither is applied. There is no state where only one side of the supersession exists.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Old ADR frontmatter already has `superseded_by: ADR-MMM` (double supersession) | Skill overwrites with new value `superseded_by: ADR-<new>`; proceeds; warned by BC-6.20.006 EC-001 |
| EC-002 | Old ADR frontmatter has no `superseded_by` field at all (malformed) | Patch fails; full rollback triggered (BC-6.20.012); error identifies "frontmatter missing superseded_by field" |
| EC-003 | Old ADR file is read-only (permissions) | Patch fails; full rollback; error identifies file path and permission issue |
| EC-004 | Old ADR file deleted between validation and patch step (race) | Patch fails; full rollback; error identifies missing file |

## Canonical Test Vectors

| Input | Expected Old ADR State After | Expected New ADR State | Category |
|-------|------------------------------|------------------------|----------|
| `--supersedes ADR-013` (exists, writable) | `superseded_by: ADR-014` | `supersedes: ADR-013` | happy-path |
| `--supersedes ADR-013` but ADR-013 missing `superseded_by` field | Old ADR unchanged; new file deleted; exit non-zero | Not written | error |
| No `--supersedes` | Old ADR untouched | `supersedes: null` | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-060 | Bidirectional supersession: `supersedes ↔ superseded_by` is symmetric after skill completion | manual + integration-test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-001 |
| L2 Domain Invariants | none directly |
| Architecture Module | plugins/vsdd-factory/skills/create-adr/SKILL.md |
| Stories | S-6.01 |
| Source AC | S-6.01 §AC-3 (tests: `test_supersession_patches_old_adr_superseded_by`, `test_supersession_atomic_rollback_on_patch_failure`) |
| FR | FR-041 |

## Related BCs

- BC-6.20.006 — prerequisite: supersedes target validated before this step
- BC-6.20.012 — atomicity contract that governs rollback on patch failure
- BC-6.20.008 — ARCH-INDEX insertion happens after this step (same atomic unit)

## Architecture Anchors

- `plugins/vsdd-factory/skills/create-adr/SKILL.md` — skill entry point
- `.factory/specs/architecture/decisions/` — old ADR file patched here

## Story Anchor

S-6.01

## VP Anchors

- VP-060 — bidirectional supersession symmetry
