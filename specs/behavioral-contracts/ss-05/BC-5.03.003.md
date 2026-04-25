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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:47
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

# Behavioral Contract BC-5.03.003: accessibility-auditor: skip cleanly when product has no UI

## Description

When dispatched against a UI-less product (CLI-only, library, API), the
accessibility-auditor emits `N/A — no user interface` and exits; it MUST NOT
fabricate UI findings.

## Preconditions

1. accessibility-auditor is dispatched.
2. Target product has no user interface (CLI-only, library, API).

## Postconditions

1. Audit report contains `N/A — no user interface`.
2. Zero finding entries are produced.
3. Agent exits cleanly.

## Invariants

1. Fabricated UI findings are forbidden.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Mixed CLI + minimal HTML output | TBD — auditor judgment |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| CLI-only Rust crate | `N/A — no user interface`; zero findings | happy-path |
| API-only service | `N/A — no user interface` | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | UI-less audit reports have the canonical N/A line and zero findings | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/accessibility-auditor.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.03.001 — composes with (WCAG citation requirement)

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
| **Path** | `plugins/vsdd-factory/agents/accessibility-auditor.md:131, 256` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit skip rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (N/A report) |
| **Global state access** | reads project type |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
