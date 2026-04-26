---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-04-26T00:00:00
phase: 1a
inputs: [.factory/stories/S-7.03-tdd-discipline-hardening.md]
input-hash: "a3187d9"
traces_to: .factory/stories/S-7.03-tdd-discipline-hardening.md
origin: brownfield
extracted_from: ".factory/stories/S-7.03-tdd-discipline-hardening.md#AC-004"
subsystem: "SS-05"
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
bc_id: BC-5.38.006
section: "5.38"
---

# BC-5.38.006: deliver-story SKILL.md and per-story-delivery.md Step 2 must contain anti-precedent guard text verbatim

## Description

The anti-precedent guard (defined in BC-5.38.004 §Invariants §1) must appear verbatim in both the deliver-story SKILL.md file and the per-story-delivery.md Step 2 section. Verbatim inclusion is required because LLM agents are sensitive to prompt phrasing, and an abstract paraphrase of the guard may not produce the behavioral change observed with the specific SHA-cited evidence. The two files are co-owners of the dispatch context for stub-architect work.

## Preconditions

1. S-7.03 delivery is being executed.
2. The deliver-story SKILL.md and per-story-delivery.md files are being modified per this story's scope.

## Postconditions

1. `plugins/vsdd-factory/skills/deliver-story/SKILL.md` contains the verbatim anti-precedent guard text from BC-5.38.004 §Invariants §1 before the Step 2 scaffold instructions.
2. `plugins/vsdd-factory/workflows/phases/per-story-delivery.md` Step 2 section contains the same verbatim guard text (or an inline cross-reference to the SKILL.md section, acceptable as long as agents loading per-story-delivery.md encounter the guard text).
3. The guard text is not placed in an appendix, footnote, or optional section — it appears in the primary flow of Step 2 instructions.

## Invariants

1. Both files must contain the guard. Having it in only one is insufficient because different dispatch paths may load only one file.
2. The guard text must include the specific SHA commits: `aa706543`, `6d2d005e`, `20b4a12a` (anti-precedents) and `e86d03f2` (model precedent).
3. The word "ANTI-PRECEDENT GUARD:" must appear as a label at the start of the block for machine-extractability.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | per-story-delivery.md includes SKILL.md by reference at load time | Acceptable if the agent's effective context includes the guard. Verify by reviewing loaded context, not just file contents. |
| EC-002 | deliver-story SKILL.md is updated but per-story-delivery.md is forgotten | VIOLATION. Both files must be updated in the same delivery commit (atomicity). |
| EC-003 | Guard text is paraphrased but semantically equivalent | Not acceptable. Verbatim is required. Paraphrase erodes the SHA citation specificity. |

## Canonical Test Vectors

| Input State | Expected Output | Category |
|-------------|----------------|----------|
| Both SKILL.md and per-story-delivery.md contain verbatim guard with all 4 SHAs | Static check passes | happy-path |
| SKILL.md has guard; per-story-delivery.md does not | VIOLATION detected at adversarial review | negative |
| Guard text present but SHA `e86d03f2` omitted | Partial violation — model-precedent SHA required | negative |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (static-check) | Both files contain verbatim guard text including all 4 SHA references | grep-based static check / adversarial review |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-016 |
| Capability Anchor Justification | CAP-016 ("Drive TDD delivery with red/green/refactor gate enforcement") per capabilities.md §CAP-016 — this BC ensures the deliver-story pipeline consistently surfaces the anti-precedent information that agents need to uphold CAP-016's red-gate mandate. |
| L2 Domain Invariants | none |
| Architecture Module | plugins/vsdd-factory/skills/deliver-story/SKILL.md, plugins/vsdd-factory/workflows/phases/per-story-delivery.md |
| Stories | S-7.03 |
| Source AC | S-7.03 §AC-004, AC-005 |
| FR | FR-043 |

## Related BCs

- BC-5.38.004 — parent (defines the required verbatim guard text)
- BC-5.38.005 — sibling (self-check rule in the same delivery context)

## Architecture Anchors

- `plugins/vsdd-factory/skills/deliver-story/SKILL.md` — primary dispatch file
- `plugins/vsdd-factory/workflows/phases/per-story-delivery.md` — per-story delivery workflow

## Story Anchor

S-7.03

## VP Anchors

(static check — adversarial coverage sufficient)
