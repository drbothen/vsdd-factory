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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:269
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

# Behavioral Contract BC-5.07.006: codebase-analyzer: never returns inline findings on Write denial

## Description

If Write fails, the agent MUST try at least 2 different formulations (absolute
path, alternate tool) before reporting failure. It MUST NOT pivot to inline
output as a substitute. Inline results are discarded.

## Preconditions

1. codebase-analyzer encounters a Write denial.

## Postconditions

1. On Write denial, agent transcript shows ≥2 retry formulations.
2. After retries fail, agent emits an explicit STOP report — never an inline findings dump.

## Invariants

1. Inline findings are a failure mode, not a fallback.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | All retries fail | Report STOP with specific denial details |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Write succeeds first try | Continue | happy-path |
| Write denied, 2 retries fail | STOP report | error |
| Write denied, fall back to inline | Audit failure | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | On any Write denial, ≥2 retry formulations precede STOP | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/codebase-analyzer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.005 — composes with (6-pass protocol)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#codebase-analyzer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/codebase-analyzer.md:110-117` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit retry-then-stop rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (with retries) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
