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
extracted_from: .factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md:273
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

# Behavioral Contract BC-5.01.003: Step taxonomy: `type:` enumerated as `skill`, `agent`, `gate`, `loop`, `human-approval`, `sub-workflow`, `parallel`, `compound`

## Description

Every step in `workflow.steps[]` declares a `type:` field constrained to one of
exactly 8 enumerated values. Each type carries a specific shape: `skill` invokes
a SKILL.md file; `agent` dispatches a sub-agent; `gate` asserts pass criteria;
`loop` performs bounded iteration; `human-approval` pauses for operator sign-off;
`sub-workflow` invokes another `.lobster` file; `parallel` fans out via `for_each`;
`compound` sequences sub-steps under a shared `depends_on` parent.

## Preconditions

1. A step entry exists in `workflow.steps[]`.
2. The step declares a `type:` field.

## Postconditions

1. `type` value is exactly one of: `skill`, `agent`, `gate`, `loop`, `human-approval`,
   `sub-workflow`, `parallel`, `compound`.
2. The step's other fields conform to the shape implied by its type:
   - `skill` → `skill: "skills/<name>/SKILL.md"`
   - `agent` → `agent: <name>`, `task: "..."`
   - `gate` → `gate: { criteria: [...], fail_action: block }`
   - `loop` → `loop: { max_iterations: N, exit_condition: "...", steps: [...] }`
   - `human-approval` → `approval: { prompt, artifacts, timeout }`
   - `sub-workflow` → `sub_workflow: "<file>.lobster"`
   - `parallel` → `for_each` and inner `steps[]`
   - `compound` → sequence of sub-steps sharing a `depends_on` parent

## Invariants

1. The 8-value enum is closed — no other types are accepted.
2. Each type uniquely binds to its shape contract.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Step declares unknown `type:` value | Parse / validation error |
| EC-002 | Step shape mismatches declared type | Validation error |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `greenfield.lobster` containing all 8 types | Parses; each step typed correctly | happy-path |
| Step with `type: skil` (typo) | Validation error | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every step's `type:` value is in the closed enum | manual |
| VP-TBD | Every step's body conforms to the type's documented shape | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/workflows/*.lobster` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.01.001 — composes with (workflow envelope)
- BC-5.01.004 — composes with (depends_on topological resolution)
- BC-5.01.007 — composes with (loop bounds)
- BC-5.01.008 — composes with (human-approval shape)
- BC-5.01.011 — composes with (sub-workflow invocation)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#step-taxonomy`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `greenfield.lobster:51, 74, 81-82, 110, 117, 132, 174, 234, 261, 296, 336, 411, 437, 466, 568, 638, 645, 651-655, 711-720, 736, 798-799, 891-892, 1001, 1062, 1192, 1280`; `code-delivery.lobster:38-40, 65-71, 73-80, 102-106, 144-145`; `brownfield.lobster:144-150` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: every type appears multiple times in greenfield; confirmed across three sampled workflows

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | none (data shape) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (data) |

#### Refactoring Notes

No refactoring needed — schema-level enum invariant.
