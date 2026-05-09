---
document_type: behavioral-contract
level: L3
version: "1.2"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
last_amended: 2026-05-08
phase: 1.4b
inputs: [.factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md]
input-hash: "a022087"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
origin: brownfield
extracted_from: .factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md § "BC-AUDIT-109"
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

# Behavioral Contract BC-5.01.002: Workflow `defaults:` block sets default `on_failure`, `max_retries`, `timeout` for unspecified steps

## Description

Each `.lobster` workflow declares a `defaults:` block at workflow level with three keys —
`on_failure`, `max_retries`, and `timeout` — that apply to any step which does not
override them. Observed values: `on_failure: escalate`, `max_retries: 2`,
`timeout: "2h"` or `"1h"`.

## Preconditions

1. Workflow declares a `defaults:` block at the workflow level.

## Postconditions

1. `defaults.on_failure` is set (universally `escalate` across sampled workflows).
2. `defaults.max_retries` is set (universally `2` across sampled workflows).
3. `defaults.timeout` is set (typical: `"2h"` or `"1h"`).
4. Each step inherits these defaults unless it explicitly overrides them.

## Invariants

1. `defaults:` is non-empty when present.
2. The three keys (`on_failure`, `max_retries`, `timeout`) appear together.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `defaults:` omitted from a workflow | TBD — fall back to engine defaults or reject |
| EC-002 | Step overrides one default but not others | Other defaults still inherited |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `greenfield.lobster::defaults` | `on_failure: escalate, max_retries: 2, timeout: "2h"` | happy-path |
| `brownfield.lobster::defaults` | Same shape | happy-path |
| `code-delivery.lobster::defaults` | Same shape | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every workflow's `defaults:` block contains the three required keys | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/workflows/*.lobster` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.01.001 — composes with (workflow protocol envelope)
- BC-5.01.006 — composes with (failure-handling semantics for `on_failure`)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#workflow-defaults`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `greenfield.lobster::defaults`; `brownfield.lobster::defaults`; `code-delivery.lobster::defaults` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: stated in workflow file headers across three sampled workflows

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | none (data) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (data) |

#### Refactoring Notes

No refactoring needed — schema-level data invariant.

## Changelog

- v1.2 (2026-05-08): TD-VSDD-091 stable-anchor migration sweep (Chunk 3) — 4 cites migrated. `extracted_from` line cite and 3 `.lobster:NNN` cites (Canonical Test Vectors + Source Evidence) replaced with stable symbol anchors (`pass-3-behavioral-contracts-deep-r1.md § "BC-AUDIT-109"`; `greenfield.lobster::defaults`; `brownfield.lobster::defaults`; `code-delivery.lobster::defaults`).
