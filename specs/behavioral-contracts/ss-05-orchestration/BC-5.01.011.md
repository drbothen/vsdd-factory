---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [.factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md]
input-hash: "a022087"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
origin: brownfield
extracted_from: .factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md:341
subsystem: SS-05
capability: CAP-TBD
lifecycle_status: active
introduced: v1.0.0-beta.4
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# Behavioral Contract BC-5.01.011: Sub-workflow invocation: `type: sub-workflow` with `sub_workflow: "<filename>.lobster"`

## Description

Reusable workflow logic is invoked via `type: sub-workflow` with `sub_workflow:
"<file>.lobster"`. The orchestrator parses and inlines the referenced workflow's
steps; inputs flow via the parent workflow's variable scope.

## Preconditions

1. A `.lobster` file declares reusable workflow logic suitable for inclusion.

## Postconditions

1. Step declares `type: sub-workflow`.
2. Step declares `sub_workflow: "<file>.lobster"` referencing another `.lobster` file.
3. The orchestrator parses and inlines the referenced workflow's steps at this point.
4. Inputs flow via the parent's variable scope.

## Invariants

1. Sub-workflow files reside in the same `.lobster` namespace as the parent.
2. Sub-workflow inclusion is non-cyclic.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `sub_workflow` references missing file | Parse error |
| EC-002 | Cyclic sub-workflow references | Parse error |
| EC-003 | Sub-workflow declares variables that conflict with parent scope | TBD — shadowing semantics |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `greenfield.lobster:98-100` (planning.lobster sub-workflow) | planning steps inlined | happy-path |
| `greenfield.lobster:907-909` (code-delivery.lobster invoked from wave-integration-fix loop) | code-delivery steps inlined per wave fix | happy-path |
| `greenfield.lobster:1237-1239` (code-delivery as ui-fix-delivery) | code-delivery reused for UI fix | happy-path |
| `brownfield.lobster:336-338` (greenfield.lobster invoked as sub-workflow) | greenfield steps inlined | happy-path |
| `brownfield.lobster:359-362` (multi-repo.lobster sub-workflow) | multi-repo steps inlined | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every `sub_workflow:` reference resolves to an existing `.lobster` file | manual |
| VP-TBD | The sub-workflow inclusion graph is acyclic | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | orchestrator sub-workflow inliner |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.01.003 — composes with (step taxonomy: sub-workflow)
- BC-5.31 — composes with (Code Delivery workflow as common sub-workflow)
- BC-5.34 — composes with (Multi-Repo workflow as sub-workflow)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#sub-workflows`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `greenfield.lobster:98-100, 907-909, 1237-1239`; `brownfield.lobster:336-338, 359-362` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit `sub_workflow:` references in workflow YAML

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (file load of sub-workflow) |
| **Global state access** | reads parent variable scope |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (data + read) |

#### Refactoring Notes

No refactoring needed.
