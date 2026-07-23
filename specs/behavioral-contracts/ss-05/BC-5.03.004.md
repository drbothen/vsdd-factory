---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [.factory/phase-0-ingestion/pass-3-deep-agents.md]
input-hash: "595f07d"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
origin: brownfield
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:55
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

# Behavioral Contract BC-5.03.004: accessibility-auditor: automated tools run before manual review

## Description

The accessibility-auditor MUST execute axe-core / lighthouse / pa11y (web) or
jsx-a11y (React) before performing manual review. Output JSON files must be
present before the markdown audit is composed.

## Preconditions

1. Audit target has a UI surface.

## Postconditions

1. Tool JSON outputs (`accessibility-report.json`, `lighthouse-a11y.json`,
   `pa11y-report.json`) exist under `.factory/cycles/**/hardening/`.
2. Audit markdown is composed only after the JSON outputs exist.
3. Markdown report references the JSON files by path.

## Invariants

1. Tool-first workflow is mandatory.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Some tools fail to run | Document gap; proceed with available outputs |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Web app audit | accessibility-report.json + lighthouse-a11y.json + pa11y-report.json present | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Tool JSON files exist before audit markdown | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/accessibility-auditor.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.03.001 — composes with (WCAG citation)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#accessibility-auditor`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/accessibility-auditor.md:102, 257` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit tool-first rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (tool execution + JSON output) |
| **Global state access** | none |
| **Deterministic** | yes (given fixed UI) |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
