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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:613
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

# Behavioral Contract BC-5.07.021: formal-verifier: mutation kill rate enforced per module-criticality tier

## Description

Mutation testing per module MUST meet or exceed `module-criticality.md`
thresholds: CRITICAL ≥95%, HIGH ≥90%, MEDIUM ≥80%, LOW ≥70%. Surviving mutants
below threshold trigger added tests.

## Preconditions

1. formal-verifier running mutation tests.

## Postconditions

1. `mutation-results/` per-module report shows kill rate ≥ threshold for each module's criticality tier.
2. Surviving mutants below threshold trigger added tests.

## Invariants

1. Per-tier thresholds are mandatory.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Module slightly below threshold | Add tests; re-run |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| CRITICAL module 96% kill rate | PASS | happy-path |
| CRITICAL module 92% | FAIL; add tests | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every module's kill rate ≥ threshold for its tier | manual |

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
| **Path** | `plugins/vsdd-factory/agents/formal-verifier.md:60-62, 152-156` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit thresholds + cargo mutants enforcement

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (mutation-results) |
| **Global state access** | none |
| **Deterministic** | yes (given fixed seed) |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
