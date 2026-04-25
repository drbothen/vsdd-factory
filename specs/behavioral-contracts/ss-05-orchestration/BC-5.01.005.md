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
extracted_from: .factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md:295
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

# Behavioral Contract BC-5.01.005: Steps SHALL declare `condition:` for conditional execution; condition is a string expression evaluated against scoped context

## Description

Steps that should run conditionally declare `condition: "<expr>"` referencing
upstream-step results, config keys, or feature flags. The expression language
supports `==`, `!=`, `in [...]`, `OR`, `AND`, function calls like `file_exists(...)`,
and negation `!`.

## Preconditions

1. The step is conditionally activated (i.e., should not run unconditionally).

## Postconditions

1. Step declares `condition: "<expr>"`.
2. The expression references upstream-step results, config keys, or feature flags.
3. Operator semantics: `==`, `!=`, `in [...]`, `OR`, `AND` are supported.
4. The orchestrator evaluates the condition against scoped context before dispatch.

## Invariants

1. Conditions are pure string expressions (no shell-out, no side effects).
2. Unsatisfied conditions cause the step to be skipped (not failed).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Condition references undefined symbol | TBD — error or evaluate to false |
| EC-002 | Condition syntactically invalid | Parse error |
| EC-003 | Step has no `condition:` | Always runs (when dependencies satisfied) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `condition: "feature_type in ['ui', 'full-stack']"` | Step runs only when feature_type matches | happy-path |
| `condition: "architect.verdict == 'request-changes'"` | Step runs only on architect rework verdict | happy-path |
| `condition: "human_approved_multi_repo == true"` | Step runs only after human approval | happy-path |
| `condition: "!file_exists('CLAUDE.md')"` | Step runs only when file is absent | happy-path |
| `condition: "!= 'multi-service' OR human_approved_multi_repo == false"` | Compound expression evaluation | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every `condition:` value parses successfully against the documented grammar | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | orchestrator condition evaluator |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.01.003 — composes with (step taxonomy)
- BC-5.01.004 — composes with (topological ordering)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#step-conditions`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `greenfield.lobster:74` (`!file_exists`); `:128` (`architect.verdict ==`); `:176` (`feature_type in [...]`); `:393` (`human_approved_multi_repo == true`); `:421` (compound expression) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: condition expressions documented inline at each conditional step

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads only (file_exists check) |
| **Global state access** | reads scoped context (upstream results, config) |
| **Deterministic** | yes (given fixed context) |
| **Thread safety** | unknown |
| **Overall classification** | pure (with controlled context reads) |

#### Refactoring Notes

No refactoring needed — condition evaluator is pure given scoped context.
