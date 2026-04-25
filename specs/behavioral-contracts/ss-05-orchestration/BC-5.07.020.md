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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:597
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

# Behavioral Contract BC-5.07.020: formal-verifier: cannot see adversarial reviews (information wall)

## Description

Formal verification MUST be driven by the Provable Properties Catalog and
specification, not adversarial findings. The agent MUST NOT load any file under
`.factory/cycles/**/adversarial-reviews/`.

## Preconditions

1. formal-verifier dispatched.

## Postconditions

1. Tool-call audit shows zero Read calls against `.factory/cycles/**/adversarial-reviews/`.

## Invariants

1. Verification is spec-driven, not finding-driven.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Verifier needs adversary's known issues | Derive independently from spec |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| formal-verifier session | Zero Reads on adversarial-reviews | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Tool-call audit confirms info wall | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/formal-verifier.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.019 — composes with (proof completion)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#formal-verifier`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/formal-verifier.md:207-220` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Information Asymmetry Wall

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (allowed paths) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
