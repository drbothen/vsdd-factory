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
bc_id: BC-6.20.004
section: "6.20"
---

# BC-6.20.004: create-adr writes frontmatter with status=proposed (always at creation)

## Description

When the skill scaffolds a new ADR file from `adr-template.md`, it always sets the `status` frontmatter field to `proposed` regardless of any other arguments. The skill never writes `accepted`, `superseded`, or any other lifecycle status at creation time. This enforces the ADR lifecycle convention that new decisions start as proposals requiring human review before acceptance.

## Preconditions

1. The skill has successfully allocated an ADR-NNN ID (BC-6.20.001 or `--id` override accepted).
2. The skill is about to render the new ADR file from the template.

## Postconditions

1. The written ADR file contains `status: proposed` in frontmatter.
2. No other status value (`accepted`, `superseded`, `rejected`, `deprecated`) appears in the `status` field of the newly created file.
3. The `adr_id` field is set to the allocated ADR-NNN value.
4. The `date` field is set to today's ISO-8601 date (YYYY-MM-DD format, UTC).
5. The `superseded_by` field is set to `null` in the new file (never set at creation).

## Invariants

1. `status: proposed` is immutable at creation — the skill has no `--status` flag and no code path that writes a different status.
2. `superseded_by: null` in the new ADR file is always the creation-time value; only the skill's supersession patch step (BC-6.20.007) can set this field, and only on the *old* ADR, never on the new one.
3. `date` always reflects the system date at invocation time, not a user-supplied value.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | User attempts to pass `--status accepted` (no such flag) | Skill ignores unknown flags or errors; `status: proposed` is always written |
| EC-002 | System clock is unavailable or returns unexpected format | Skill exits non-zero with "cannot determine today's date" before writing |
| EC-003 | Template `adr-template.md` contains a different default status value | Skill overrides the template value with `proposed`; template default is irrelevant |

## Canonical Test Vectors

| Input | Expected `status` in Output File | Category |
|-------|----------------------------------|----------|
| Normal invocation | `proposed` | happy-path |
| Invocation with `--supersedes ADR-013` | `proposed` (new file); old ADR gets `superseded_by` patch separately | happy-path |
| Any invocation | `superseded_by: null` in new file | happy-path |
| Date = 2026-04-25 (system) | `date: 2026-04-25` in frontmatter | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (none allocated — covered by integration test suite) | status=proposed invariant at creation | integration-test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-001 |
| L2 Domain Invariants | none directly |
| Architecture Module | plugins/vsdd-factory/skills/create-adr/SKILL.md |
| Stories | S-6.01 |
| Source AC | S-6.01 §AC-2 (tests: `test_frontmatter_status_always_proposed`, `test_frontmatter_date_is_today`) |
| FR | FR-041 |

## Related BCs

- BC-6.20.005 — sibling frontmatter validation (subsystems_affected field)
- BC-6.20.006 — sibling frontmatter validation (supersedes field)
- BC-6.20.007 — supersession patch sets superseded_by on the *old* ADR, never on the new one

## Architecture Anchors

- `plugins/vsdd-factory/skills/create-adr/SKILL.md` — skill entry point
- `plugins/vsdd-factory/templates/adr-template.md` — template read as scaffold source

## Story Anchor

S-6.01

## VP Anchors

- (none allocated — covered by integration test)
