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
extracted_from: ".factory/stories/S-6.01-create-adr-skill.md#AC-4"
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
bc_id: BC-6.20.008
section: "6.20"
---

# BC-6.20.008: create-adr inserts ARCH-INDEX row in numeric order, pipe-aligned

## Description

After writing the new ADR file, the skill inserts one new row into the ARCH-INDEX `## Architecture Decisions` table, positioned immediately after the row for the current highest ADR, maintaining ascending numeric order. The row format is pipe-aligned to match existing table rows, with slug derived from the `--title` argument. If the `## Architecture Decisions` section header is absent from ARCH-INDEX, the skill exits non-zero before writing any file.

## Preconditions

1. The new ADR file has been written successfully.
2. ARCH-INDEX is readable and contains the `## Architecture Decisions` section header.
3. The skill has the ADR-NNN ID, title, subsystems_affected, and slug available.

## Postconditions

1. ARCH-INDEX contains a new row: `| ADR-NNN | <decision-title> | <subsystems_affected joined by ", "> | decisions/ADR-NNN-<slug>.md |`
2. The new row is positioned after the row for the previously highest ADR (ascending order preserved).
3. The pipe characters in the new row are aligned to match the column widths of adjacent rows.
4. The `<slug>` is derived from `--title`: lowercased; whitespace runs replaced with single `-`; all non-`[a-z0-9-]` characters (including non-ASCII) stripped (not transliterated); consecutive `-` runs collapsed to single `-`; leading/trailing `-` trimmed.
5. The `<decision-title>` in the row uses the original unsanitized title (not the slug).

## Invariants

1. The ARCH-INDEX table row count increases by exactly 1 per successful invocation.
2. Numeric order of ADR IDs in the table is always preserved after insertion.
3. The slug in the file path column matches the slug used in the actual filename written to disk.
4. If ARCH-INDEX lacks the `## Architecture Decisions` section, insertion is blocked (no file write proceeds either — see BC-6.20.012 for rollback).
5. Argument validation order: `--title` presence is checked first; subsystem registry validation second; supersedes existence third; ID allocation/override last. Errors at any earlier stage prevent later stages.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `--title "ADR with / special <chars>"` | Slug derivation: lowercase the title; replace whitespace runs with single `-`; strip ALL non-`[a-z0-9-]` characters (including non-ASCII); collapse consecutive `-` runs to a single `-`; trim leading/trailing `-`. UTF-8 input is supported; non-ASCII characters are stripped (NOT transliterated). Example: `Decision <with/special> chars` → `decision-withspecial-chars`. Example: `Café Décision` → `caf-dcision`. Title in ARCH-INDEX row is original unsanitized string. |
| EC-002 | `## Architecture Decisions` section missing from ARCH-INDEX | Skill exits non-zero: "ARCH-INDEX missing '## Architecture Decisions' section. Cannot insert row."; no files written |
| EC-003 | Existing table rows have inconsistent pipe widths | Skill aligns new row to match the widest column in the existing rows |
| EC-004 | Required argument `--title` absent (alone or with other args supplied like `--id` or `--supersedes`) | Skill exits non-zero with usage error BEFORE any other argument is processed; no side-effects. |
| EC-005 | Architecture Decisions section exists with header row but zero data rows (legitimate first-ADR-ever case) | Skill inserts the new row immediately after the header separator (no preceding data row to position after). Postcondition: table now has exactly one data row. |

## Canonical Test Vectors

| Input | Expected ARCH-INDEX Change | Category |
|-------|---------------------------|----------|
| Title "Use wasmtime for plugin sandbox", subsystems SS-01 | Row `\| ADR-014 \| Use wasmtime for plugin sandbox \| SS-01 \| decisions/ADR-014-use-wasmtime-for-plugin-sandbox.md \|` inserted after ADR-013 | happy-path |
| Title "A/B test", subsystems SS-06,SS-08 | Slug: `a-b-test`; row inserted with `SS-06, SS-08` | happy-path |
| ARCH-INDEX missing section header | Exit non-zero; no file written | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (none allocated — covered by integration test suite) | Row insertion preserves numeric order | integration-test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-017 |
| Capability Anchor Justification | Anchored to CAP-017 (Create and manage formal ADR records) per capabilities.md §CAP-017 — literal match for ADR scaffolding. |
| L2 Domain Invariants | none directly |
| Architecture Module | plugins/vsdd-factory/skills/create-adr/SKILL.md |
| Stories | S-6.01 |
| Source AC | S-6.01 §AC-4 (tests: `test_arch_index_row_inserted_in_numeric_order`, `test_arch_index_row_pipe_aligned`, `test_arch_index_slug_derivation`, `test_arch_index_missing_section_blocks`) |
| FR | FR-041 |

## Related BCs

- BC-6.20.007 — supersession patch precedes this step
- BC-6.20.011 — validation gate runs after this step
- BC-6.20.012 — atomicity: ARCH-INDEX insertion is rolled back on downstream failure

## Architecture Anchors

- `plugins/vsdd-factory/skills/create-adr/SKILL.md` — skill entry point
- `.factory/specs/architecture/ARCH-INDEX.md` — Architecture Decisions table (mutated)

## Story Anchor

S-6.01

## VP Anchors

- (none allocated — covered by integration test)
