---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-04-25T00:00:00
phase: 1a
inputs: [.factory/stories/S-7.02-defensive-sweep-hook-meta-rule.md]
input-hash: ""
traces_to: .factory/stories/S-7.02-defensive-sweep-hook-meta-rule.md
origin: greenfield
extracted_from: ".factory/stories/S-7.02-defensive-sweep-hook-meta-rule.md#AC-005"
subsystem: "SS-07"
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
bc_id: BC-7.05.003
section: "7.05"
---

# BC-7.05.003: validate-template-compliance.sh enforces VP multi-BC source_bc convention

## Description

The existing `validate-template-compliance.sh` must be extended with a check that enforces the VP multi-BC convention: any VP file with more than one entry in its `bcs:` array must also have a non-empty `source_bc:` field whose value is an element of the `bcs:` array. Violations emit a structured warning and cause non-zero exit.

## Preconditions

1. A VP file (`VP-*.md` or `document_type: verification-property`) is being validated.
2. `validate-template-compliance.sh` is invoked (either directly or as a hook).
3. The VP file has a YAML frontmatter block parseable by the script.

## Postconditions

1. If `len(bcs) > 1` and `source_bc:` is empty or absent, the script emits:
   ```
   TEMPLATE COMPLIANCE WARNING: VP-NNN has multiple bcs[] but missing or invalid source_bc field.
   ```
   and exits non-zero.
2. If `len(bcs) > 1` and `source_bc:` is present but not in `bcs[]`, the script emits the same warning.
3. If `len(bcs) == 1` and `source_bc:` is absent, the script exits 0 (single-BC VPs do not require `source_bc:`).
4. If `len(bcs) > 1` and `source_bc:` is present and in `bcs[]`, the script exits 0.
5. The check is scoped to VP files only. Non-VP files must not trigger this check path.

## Invariants

1. The check does not modify any VP file. It is read-only validation.
2. The check is added to the existing script as a new check block — not a replacement of existing checks.
3. The scope guard (VP files only) prevents false positives on story, BC, or architecture files.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | VP file has `bcs: []` (empty array) | Not a multi-BC VP; check skips without warning (0-element array is treated as single or absent). |
| EC-002 | VP file has `bcs: [BC-A, BC-B, BC-C]` and `source_bc: BC-D` where BC-D is not in the array | Fails. source_bc must be IN the bcs[] array. Warning emitted; exit non-zero. |
| EC-003 | Script invoked on a BC file (not a VP file) | Scope guard fires; VP multi-BC check is skipped entirely. Exit 0 (other checks still run). |
| EC-004 | VP file's `bcs:` field is a scalar (not an array) due to authoring error | Script handles this gracefully — emit a different warning: "VP-NNN bcs: field is not an array." |

## Canonical Test Vectors

| Input VP File State | Expected Exit Code | Expected Output | Category |
|--------------------|--------------------|-----------------|----------|
| `bcs: [BC-A, BC-B]`, `source_bc: BC-A` | 0 | (passes) | happy-path |
| `bcs: [BC-A, BC-B]`, no `source_bc:` field | 1 | `TEMPLATE COMPLIANCE WARNING: VP-NNN has multiple bcs[] but missing or invalid source_bc field.` | negative |
| `bcs: [BC-A, BC-B]`, `source_bc: BC-C` (not in array) | 1 | same warning | negative |
| `bcs: [BC-A]`, no `source_bc:` field | 0 | (passes — single BC) | happy-path |
| Non-VP file passed to script | 0 | (VP check not triggered) | scope-guard |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-062 | VP multi-BC source_bc check fires correctly on violation | integration (test harness with fixture VP files) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-001 |
| Capability Anchor Justification | Anchored to CAP-001 ("Run a self-orchestrating LLM-driven SDLC pipeline") per capabilities.md §CAP-001 — enforcing VP structural conventions is a pipeline quality gate that maintains the self-orchestrating SDLC's spec consistency, which is integral to CAP-001. |
| L2 Domain Invariants | none |
| Architecture Module | plugins/vsdd-factory/bin/validate-template-compliance.sh |
| Stories | S-7.02 |
| Source AC | S-7.02 §AC-005 |
| FR | FR-042 |

## Related BCs

- BC-7.05.001 — sibling (both are SS-07 hook/script extensions for spec quality enforcement)

## Architecture Anchors

- `plugins/vsdd-factory/bin/validate-template-compliance.sh` — script to be extended

## Story Anchor

S-7.02

## VP Anchors

- VP-062 — verification property covering compliance hook behavior
