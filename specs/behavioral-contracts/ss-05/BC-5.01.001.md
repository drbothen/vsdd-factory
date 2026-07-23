---
document_type: behavioral-contract
level: L3
version: "1.3"
status: draft
producer: codebase-analyzer
timestamp: 2026-05-08T00:00:00
phase: 1.4b
inputs: [.factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md]
input-hash: "a022087"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
origin: brownfield
extracted_from: '.factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md § "GAP-C findings — Workflow .lobster protocol contracts"'
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

# Behavioral Contract BC-5.01.001: A `.lobster` file is YAML at the top level with a single `workflow:` key

## Description

`.lobster` workflow files are YAML at the top level with a single `workflow:` key
containing the workflow definition. Files live under `plugins/vsdd-factory/workflows/`
or `workflows/phases/` and are pure data parsed via `bin/lobster-parse` using
`yq eval --output-format=json | jq`.

## Preconditions

1. File extension is `.lobster`.
2. File path is under `plugins/vsdd-factory/workflows/` or `workflows/phases/`.

## Postconditions

1. Top-level YAML object has exactly one `workflow:` key.
2. The `workflow:` value is a map containing `name`, `description`, `version`,
   `defaults`, `steps[]`, and optionally `inputs[]` and `cost_monitoring{}`.
3. The `version` field uses SemVer-ish strings (e.g., `"2.1.0"`, `"3.0.0"`).
4. `bin/lobster-parse` consumes the file via `yq eval --output-format=json '.' | jq "$EXPR"`.

## Invariants

1. `.lobster` files are pure data (no executable directives at the top level).
2. The top-level structure is stable across all sampled workflow files.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Top-level keys other than `workflow:` | Parse error / rejection |
| EC-002 | Missing `version` field | Validation error at load time |
| EC-003 | Non-SemVer `version` value | TBD — coerce or reject |

## Canonical Test Vectors

> Golden-file test inputs and expected outputs.

| Input | Expected Output | Category |
|-------|----------------|----------|
| `greenfield.lobster` (1,409 LOC) | Parses successfully; `workflow.name`, `version: "2.1.0"`, `cost_monitoring`, `defaults` all present | happy-path |
| `plugins/vsdd-factory/workflows/brownfield.lobster::workflow` | Parses successfully; `workflow.name`, `version: "3.0.0"` | happy-path |
| File with two top-level keys | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | All `.lobster` files in `plugins/vsdd-factory/workflows/` parse to a single-key `workflow:` object | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/workflows/*.lobster`, `bin/lobster-parse` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.01.002 — composes with (defaults block schema)
- BC-5.01.003 — composes with (step taxonomy)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#workflow-protocol` — workflow file structure

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD — `.lobster` schema validation

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/greenfield.lobster::workflow`; `plugins/vsdd-factory/workflows/brownfield.lobster::workflow`; `plugins/vsdd-factory/workflows/code-delivery.lobster::workflow`; `bin/lobster-parse::yq_stderr` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: stated in workflow file headers
- inferred: parser code (`bin/lobster-parse`) consumes the documented shape

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads only (file load by parser) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (data) |

#### Refactoring Notes

No refactoring needed — `.lobster` files are pure data; verification is schema-level.

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.1 | 2026-04-25 | codebase-analyzer | Initial authoring. |
| v1.2 | 2026-05-08 | implementer | TD-VSDD-091 Chunk 4 — migrated 2 line citations to stable anchors: `pass-3-behavioral-contracts-deep-r1.md:259` → `§ "GAP-C findings"` section anchor; `bin/lobster-parse:39-51` → `bin/lobster-parse::yq_stderr`. |
| v1.3 | 2026-05-08 | implementer | TD-VSDD-091 Chunk 6 — migrated 2 body cites: `brownfield.lobster:1-23` → `brownfield.lobster::workflow` (test vector table and Source Evidence Path). `greenfield.lobster:1-30`, `code-delivery.lobster:1-32` → `::workflow` anchors. |
