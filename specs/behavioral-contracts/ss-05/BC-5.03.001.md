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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:31
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

# Behavioral Contract BC-5.03.001: accessibility-auditor: WCAG criterion citation is mandatory for every finding

## Description

The accessibility-auditor MUST attach a specific WCAG 2.1 criterion ID (e.g., 1.4.3)
and a concrete file/component location to every finding emitted; generic advice
is forbidden. The audit report's table must summarize per-principle counts
(Perceivable / Operable / Understandable / Robust).

## Preconditions

1. accessibility-auditor is dispatched against a UI-bearing product.

## Postconditions

1. Every entry under `### [SEVERITY] Finding Title` in `accessibility-audit.md` has
   a non-empty `WCAG Criterion:` line.
2. Every entry has a non-empty `Location:` line (file/component).
3. Findings missing either field are linted as policy violations.
4. The report includes a per-principle summary (Perceivable / Operable / Understandable / Robust).

## Invariants

1. WCAG citation is non-optional.
2. Location specificity is non-optional.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Generic advice without WCAG ID | Linted as policy violation |
| EC-002 | Finding with WCAG ID but no Location | Linted as policy violation |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Finding with WCAG 1.4.3 + Location | Accepted | happy-path |
| Finding without WCAG ID | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every accessibility-audit.md finding has both WCAG Criterion and Location | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/accessibility-auditor.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.03.002 — composes with (read-only constraint)
- BC-5.03.003 — composes with (skip when no UI)

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
| **Path** | `plugins/vsdd-factory/agents/accessibility-auditor.md:100-104, 252-258` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Constraints and Rules sections

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (audit tool output) + writes (audit report) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
