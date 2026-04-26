---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-04-26T00:00:00
phase: 1a
inputs: [.factory/stories/S-7.03-tdd-discipline-hardening.md]
input-hash: ""
traces_to: .factory/stories/S-7.03-tdd-discipline-hardening.md
origin: brownfield
extracted_from: ".factory/stories/S-7.03-tdd-discipline-hardening.md#AC-010"
subsystem: "SS-08"
capability: "CAP-016"
lifecycle_status: active
introduced: v1.0-brownfield-backfill
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-8.30.001
section: "8.30"
---

# BC-8.30.001: story template must include tdd_mode field with strict|facade enum and strict default

## Description

The story template (`plugins/vsdd-factory/templates/story-template.md`) must include a `tdd_mode:` frontmatter field with an explicit two-value enum (`strict` or `facade`). If a story file omits this field, the pipeline treats it as `tdd_mode: strict`. The field was absent from the template prior to S-7.03, which meant the pipeline had no contract-driven way to distinguish stories that legitimately combine scaffolding with implementation (DTU facade work) from stories that must follow the full TDD Iron Law cycle.

## Preconditions

1. S-7.03 delivery is in progress.
2. The story template file (`plugins/vsdd-factory/templates/story-template.md`) is being modified.
3. Existing stories that do not have `tdd_mode:` are in the wild (they retroactively default to `strict`).

## Postconditions

1. `plugins/vsdd-factory/templates/story-template.md` frontmatter block contains:
   ```yaml
   tdd_mode: strict  # strict | facade. strict = full TDD Iron Law enforced; facade = scaffold+impl combined, mutation testing at wave gate
   ```
2. The comment documents both valid values and their semantic difference.
3. The story-writer agent's prompt references this field as part of the required frontmatter checklist.
4. Any pipeline step that reads `tdd_mode` treats a missing field as `strict` (not `facade`).

## Invariants

1. Valid values are exactly: `strict`, `facade`. No other values. An unknown value is treated as `strict` with a warning.
2. The default-to-strict rule is a safety default: stories that forget to set `tdd_mode` get the stricter enforcement, not the relaxed path.
3. `tdd_mode: facade` must only be set by the decompose-stories skill or with explicit human justification. Story-writer must not set it autonomously without a BC or AC requiring it.
4. Once a story's `tdd_mode` is set, it cannot be changed after Step 2 without an explicit amendment process.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Existing story files in `.factory/stories/` lack `tdd_mode` | All treated as `strict`. No backfill needed — the default handles it. |
| EC-002 | `tdd_mode: facade` set on an algorithmic-core story (not a DTU clone) | Valid syntax but semantically suspect. Adversary must flag as a finding if the story is not a structural facade or DTU clone. |
| EC-003 | `tdd_mode: invalid_value` | Treated as `strict` with a WARNING log entry identifying the unknown value. |

## Canonical Test Vectors

| Story Frontmatter | Effective tdd_mode | Step 2 Behavior |
|-------------------|--------------------|-----------------|
| `tdd_mode: strict` | strict | todo!() discipline enforced; Red Gate density gate applies |
| `tdd_mode: facade` | facade | scaffold+impl combined allowed; mutation testing at wave gate |
| `tdd_mode:` absent | strict (default) | Same as explicit strict |
| `tdd_mode: blah` | strict (warning) | Pipeline warns, applies strict |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (static-check) | Story template contains tdd_mode field with comment | grep-based static check |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-016 |
| Capability Anchor Justification | CAP-016 ("Drive TDD delivery with red/green/refactor gate enforcement") per capabilities.md §CAP-016 — this BC creates the contract-level mechanism by which stories communicate their TDD mode to the pipeline, enabling CAP-016's gate to be applied with the right semantics per story type. |
| L2 Domain Invariants | none |
| Architecture Module | plugins/vsdd-factory/templates/story-template.md, plugins/vsdd-factory/agents/story-writer.md |
| Stories | S-7.03 |
| Source AC | S-7.03 §AC-010 |
| FR | FR-043 |

## Related BCs

- BC-8.30.002 — sibling (facade-mode semantics that this field unlocks)
- BC-8.29.001 — depends on (strict-mode Red Gate density check gate)
- BC-5.38.001 — depends on (strict-mode stub discipline)

## Architecture Anchors

- `plugins/vsdd-factory/templates/story-template.md` — frontmatter field to add
- `plugins/vsdd-factory/agents/story-writer.md` — checklist for required frontmatter

## Story Anchor

S-7.03

## VP Anchors

(static check — adversarial coverage sufficient)
