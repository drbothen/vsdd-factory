---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-04-25T00:00:00
phase: 1a
inputs: [.factory/stories/S-7.01-agent-prompt-discipline.md]
input-hash: ""
traces_to: .factory/stories/S-7.01-agent-prompt-discipline.md
origin: greenfield
extracted_from: ".factory/stories/S-7.01-agent-prompt-discipline.md#AC-003"
subsystem: "SS-05"
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
bc_id: BC-5.36.006
section: "5.36"
---

# BC-5.36.006: adversary checks fix propagation to bodies, sibling files, and prose — not just frontmatter

## Description

The adversary's Partial-Fix Regression axis must explicitly require checking three distinct propagation surfaces: (a) the body of files whose frontmatter was changed, (b) sibling files of the same type in the same subsystem or directory, and (c) prose sections in index or reference documents. A fix that updates only one of these surfaces when all three were affected constitutes incomplete propagation.

## Preconditions

1. A prior-pass finding has been marked closed after a fix was applied.
2. The fix involved modifying frontmatter, a specific body section, or a reference document.
3. The adversary is performing a regression propagation check.

## Postconditions

1. The adversary verifies propagation to all three surfaces: body (of frontmatter-changed files), sibling files, and prose references.
2. Any surface that was required but not updated is reported as a finding with the specific file:line that was missed.
3. The adversary prompt explicitly names these three surfaces in the Partial-Fix Regression axis wording.

## Invariants

1. Frontmatter changes always require a body consistency check (e.g., if frontmatter `bcs:` array is updated, the story body's BC table must also be updated).
2. Sibling files are defined as other files of the same document_type in the same directory or subsystem.
3. Prose references include count tables, traceability matrices, and index sections in ARCH-INDEX.md, PRD.md, or BC-INDEX.md.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Frontmatter `bcs:` array was updated but story body BC table was not | Adversary reports: body propagation gap — frontmatter and body are out of sync |
| EC-002 | One BC in ss-06 was fixed but five sibling BCs in ss-06 have identical gaps | Adversary reports: sibling propagation gap for each unfixed sibling, grouped by pattern |
| EC-003 | Count in PRD.md was updated but same count in ARCH-INDEX.md was not | Adversary reports: prose propagation gap in ARCH-INDEX.md |
| EC-004 | Fix was applied to a template file but existing generated files using that template were not regenerated | Adversary notes that template-derived files may be stale and flags for implementer review. This is an advisory finding, not a blocking one (regeneration requires re-delivery). |

## Canonical Test Vectors

| Input State | Expected Adversary Output | Category |
|-------------|--------------------------|----------|
| `bcs: [BC-5.36.001]` added to story frontmatter; story body BC table still shows `BC-TBD` | Adversary reports body gap: frontmatter and body BC table inconsistent | negative |
| BC-5.36.001 added anchor justification; BC-5.36.002 through BC-5.36.005 still missing it | Adversary reports sibling gap: 4 sibling BCs need same fix | negative |
| PRD.md FR-042 row added; ARCH-INDEX FR count not updated | Adversary reports prose gap: ARCH-INDEX FR count stale | negative |
| Fix applied to all three surfaces; adversary checks all three | No finding raised | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-061 | Three-surface propagation check (body, sibling, prose) explicitly listed in adversary.md | static-check (peer review) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-001 |
| Capability Anchor Justification | Anchored to CAP-001 ("Run a self-orchestrating LLM-driven SDLC pipeline") per capabilities.md §CAP-001 — the adversary's three-surface propagation check is a quality gate in the self-orchestrating pipeline's convergence loop described by CAP-001. |
| L2 Domain Invariants | none |
| Architecture Module | plugins/vsdd-factory/agents/adversary.md |
| Stories | S-7.01 |
| Source AC | S-7.01 §AC-003 |
| FR | FR-042 |

## Related BCs

- BC-5.36.005 — depends on (this BC refines the scope of the regression check defined in BC-5.36.005)

## Architecture Anchors

- `plugins/vsdd-factory/agents/adversary.md` — agent prompt file to be modified

## Story Anchor

S-7.01

## VP Anchors

- VP-061 — three-surface propagation check presence
