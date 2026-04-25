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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:63
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

# Behavioral Contract BC-5.03.005: accessibility-auditor: cannot load architecture files

## Description

The accessibility-auditor MUST NOT load any file under `.factory/specs/architecture/` —
this preserves the user-experience perspective. The auditor judges the experience
from the user's view, not the architect's.

## Preconditions

1. accessibility-auditor is dispatched.

## Postconditions

1. Tool-call audit shows zero Read calls against `.factory/specs/architecture/`.

## Invariants

1. The information wall on architecture files is structural — enforced by `context: { exclude }`.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Agent attempts to Read architecture file | Read denied |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Audit session | Zero Reads on `.factory/specs/architecture/` | happy-path |
| Attempted Read on architecture | Denied | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Tool-call audit shows zero Reads against architecture/ | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/accessibility-auditor.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.01.010 — composes with (information-asymmetry walls via context block)

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
| **Path** | `plugins/vsdd-factory/agents/accessibility-auditor.md:77-78` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Information Asymmetry section

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (allowed paths only) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
