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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:673
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

# Behavioral Contract BC-5.07.024: holdout-evaluator: cannot read source code, specs, or prior reviews

## Description

The holdout-evaluator MUST evaluate the system as a black-box user. It MUST NOT
read source code, BCs, architecture docs, prior reviews, semport translation
artifacts, or test source. It CAN read `.factory/holdout-scenarios/`,
`.factory/specs/product-brief.md` (high-level only), and observe runtime behavior.

## Preconditions

1. holdout-evaluator dispatched.

## Postconditions

1. Tool-call audit shows zero Read against forbidden paths.
2. Only allowed Reads: `.factory/holdout-scenarios/`, `product-brief.md`,
   plus Bash invocations of the application binary.

## Invariants

1. Black-box evaluation is structural.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Evaluator needs to clarify expected behavior | Read product-brief.md only |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Holdout session | Zero Reads on forbidden paths | happy-path |
| Read on src/ | Denied | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Tool-call audit confirms info wall | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/holdout-evaluator.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.025 — composes with (gate criteria)
- BC-5.07.027 — composes with (read-only — no Write tool)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#holdout-evaluator`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/holdout-evaluator.md:21-29` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Information Asymmetry Wall

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (allowed) + Bash (SUT execution) |
| **Global state access** | none |
| **Deterministic** | depends on SUT |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
