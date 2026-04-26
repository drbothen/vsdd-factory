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
extracted_from: ".factory/stories/S-6.01-create-adr-skill.md#AC-5"
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
bc_id: BC-6.20.009
section: "6.20"
---

# BC-6.20.009: create-adr scaffolds placeholder section bodies verbatim from template (no ghost-writing)

## Description

The skill copies section bodies from `adr-template.md` verbatim into the new ADR file, preserving bracketed prompt text (e.g., `[2-5 paragraphs] Background, forces driving the decision…`) exactly as written. The skill never generates, infers, or substitutes prose in place of these prompts. After writing the file, the skill prints a structured guidance block to stdout naming all sections to flesh out and recommending the architect agent as the next step.

## Preconditions

1. `plugins/vsdd-factory/templates/adr-template.md` is readable.
2. The new ADR file path has been determined (ADR-NNN allocated, slug derived).
3. All frontmatter fields have been validated (BCs 6.20.004–006 passed).

## Postconditions

1. The scaffolded ADR file contains section bodies copied verbatim from the template, including all bracketed prompt text.
2. No section body contains AI-generated or inferred prose — only the exact template placeholder text.
3. The following guidance block is printed to stdout after the file is written:

```
ADR-NNN scaffolded at: .factory/specs/architecture/decisions/ADR-NNN-<slug>.md

Sections to flesh out:
  - Context      (2-5 paragraphs: background, forces, constraints)
  - Decision     (1-3 paragraphs: the choice itself)
  - Rationale    (2-5 paragraphs: why this, not alternatives)
  - Consequences (Positive / Negative sub-headings)
  - Alternatives Considered (top 2-3 options rejected)
  - Source / Origin (MUST cite implementation evidence for brownfield ADRs)

Recommended next step:
  Spawn architect agent: "Flesh out ADR-NNN sections. File: .factory/specs/architecture/decisions/ADR-NNN-<slug>.md"
```

4. The skill does NOT invoke the architect agent — the guidance block is a print-only hand-off.

## Invariants

1. Template placeholder text is never modified, trimmed, or summarised by the skill.
2. The guidance block is always printed when the skill exits 0 (successful creation).
3. The guidance block is never printed when the skill exits non-zero (failure paths).
4. The skill has no `--no-guidance` flag — the guidance block is unconditional on success.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `adr-template.md` is missing or unreadable | Skill exits non-zero: "Template not found at plugins/vsdd-factory/templates/adr-template.md"; no file written |
| EC-002 | `adr-template.md` has been modified to remove a section | Skill scaffolds whatever the template currently contains verbatim; no error |
| EC-003 | stdout is redirected to /dev/null by caller | Guidance block still written to stdout; skill does not error |

## Canonical Test Vectors

| Input | Expected Scaffolded File Contains | Expected stdout | Category |
|-------|----------------------------------|-----------------|----------|
| Normal invocation with ADR-014 | Verbatim template section bodies including all `[...]` prompts | Guidance block with `ADR-014` substituted | happy-path |
| Template missing | Exit non-zero; no file | No guidance block | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (none allocated — covered by integration test suite) | Template placeholders preserved verbatim | integration-test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-001 |
| L2 Domain Invariants | none directly |
| Architecture Module | plugins/vsdd-factory/skills/create-adr/SKILL.md |
| Stories | S-6.01 |
| Source AC | S-6.01 §AC-5 (tests: `test_scaffold_preserves_template_placeholder_text`, `test_stdout_guidance_block_present`) |
| FR | FR-041 |

## Related BCs

- BC-6.20.004 — frontmatter fields populated before section bodies are written
- BC-6.20.010 — brownfield annotation added to Source/Origin section after verbatim copy

## Architecture Anchors

- `plugins/vsdd-factory/skills/create-adr/SKILL.md` — skill entry point
- `plugins/vsdd-factory/templates/adr-template.md` — template source (read-only)

## Story Anchor

S-6.01

## VP Anchors

- (none allocated — covered by integration test)
