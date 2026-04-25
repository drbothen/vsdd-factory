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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:39
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

# Behavioral Contract BC-5.03.002: accessibility-auditor: read-only — never modifies source

## Description

Despite having `apply_patch`/`exec` in profile, the accessibility-auditor MUST
limit writes to `.factory/cycles/**/hardening/accessibility-audit.md` and the
JSON tool outputs. It MUST NOT modify `src/`, tests, or design system files.

## Preconditions

1. accessibility-auditor is running.

## Postconditions

1. Git diff after the agent runs touches no path outside `.factory/cycles/**/hardening/`.
2. Audit reports + tool JSON outputs are the only writes.

## Invariants

1. Source code, tests, and design system files are never modified by this agent.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Agent attempts to apply a fix to source | Self-blocked |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Audit run on UI app | Writes only under `.factory/cycles/**/hardening/` | happy-path |
| Attempt to edit `src/Button.tsx` | Self-blocked | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Git diff after agent runs has zero entries outside hardening directory | manual |

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
| **Path** | `plugins/vsdd-factory/agents/accessibility-auditor.md:99, 251` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit "report findings only" rules

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (scoped to hardening/) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
