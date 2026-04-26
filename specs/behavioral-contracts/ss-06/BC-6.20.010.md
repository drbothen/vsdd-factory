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
extracted_from: ".factory/stories/S-6.01-create-adr-skill.md#AC-6"
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
bc_id: BC-6.20.010
section: "6.20"
---

# BC-6.20.010: create-adr annotates Source/Origin section under --brownfield or implicit-brownfield

## Description

When the `--brownfield` flag is supplied, or when `--supersedes ADR-NNN` is supplied (which implies an established design is being formalised), the skill appends a non-skippable warning comment immediately after the `## Source / Origin` section header in the scaffolded ADR file. This comment blocks template-compliance validation from passing until implementation evidence is cited. When neither flag applies, no annotation is added.

## Preconditions

1. The new ADR file has been scaffolded with verbatim template content (BC-6.20.009 complete).
2. At least one of the following is true: `--brownfield` flag was supplied, OR `--supersedes ADR-NNN` was supplied.

## Postconditions

1. The `## Source / Origin` section in the scaffolded file is immediately followed by the annotation:
   ```markdown
   <!-- BROWNFIELD: You MUST cite implementation evidence (file:line from crates/ or
        legacy-design-docs/) before this ADR can be accepted. Omitting evidence is a
        template-compliance failure. -->
   ```
2. No other section of the file is modified by this step.
3. When neither `--brownfield` nor `--supersedes` is present, the `## Source / Origin` section contains only the verbatim template placeholder — no annotation is added.

## Invariants

1. The annotation is always the exact text specified above — no paraphrasing, no variation.
2. The annotation is placed immediately after the `## Source / Origin` header line, before any template placeholder body text.
3. Implicit brownfield (via `--supersedes`) triggers the same annotation as explicit `--brownfield` — the trigger condition is a union, not exclusive.
4. The annotation is a Markdown HTML comment — it does not appear in rendered output but is visible in raw file content.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Both `--brownfield` and `--supersedes` supplied | Annotation written once (not duplicated) |
| EC-002 | `--supersedes` supplied but the superseded ADR is greenfield (has no brownfield evidence itself) | Annotation is still added — the supersession implies formalising an existing design decision |
| EC-003 | Template's `## Source / Origin` section header is renamed or missing | Skill exits non-zero: "adr-template.md missing '## Source / Origin' section — cannot add brownfield annotation" |
| EC-004 | Neither `--brownfield` nor `--supersedes` supplied | No annotation; Source/Origin section contains only template placeholder |

## Canonical Test Vectors

| Input Flags | Expected Source/Origin Content | Category |
|-------------|-------------------------------|----------|
| `--brownfield` | Section header followed by BROWNFIELD comment then template body | happy-path |
| `--supersedes ADR-013` | Section header followed by BROWNFIELD comment then template body | happy-path |
| `--brownfield --supersedes ADR-013` | Annotation appears exactly once | edge-case |
| (no flags) | Section header followed by template body; no comment | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (none allocated — covered by integration test suite) | Brownfield annotation present iff trigger condition met | integration-test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-017 |
| Capability Anchor Justification | Anchored to CAP-017 (Create and manage formal ADR records) per capabilities.md §CAP-017 — literal match for ADR scaffolding. |
| L2 Domain Invariants | none directly |
| Architecture Module | plugins/vsdd-factory/skills/create-adr/SKILL.md |
| Stories | S-6.01 |
| Source AC | S-6.01 §AC-6 (tests: `test_brownfield_flag_injects_source_annotation`, `test_supersedes_implies_brownfield_annotation`, `test_no_brownfield_flag_no_annotation`) |
| FR | FR-041 |

## Related BCs

- BC-6.20.006 — supersedes validation precedes annotation decision
- BC-6.20.009 — verbatim template copy precedes this annotation step
- BC-6.20.011 — validation gate runs after annotation is written

## Architecture Anchors

- `plugins/vsdd-factory/skills/create-adr/SKILL.md` — skill entry point
- `plugins/vsdd-factory/templates/adr-template.md` — template read to locate Source/Origin header

## Story Anchor

S-6.01

## VP Anchors

- (none allocated — covered by integration test)
