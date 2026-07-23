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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:861
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

# Behavioral Contract BC-5.02.012: orchestrator: heartbeat is read-only (no spawning, no writes)

## Description

The HEARTBEAT companion reads STATE.md, cost-summary.md, and `.factory/.git`
existence and reports alerts. It MUST NOT spawn sub-agents or write any files.
Replies `HEARTBEAT_OK` if all checks pass.

## Preconditions

1. A heartbeat session is initiated.

## Postconditions

1. Heartbeat performs only Read-tool calls.
2. Heartbeat reports either `HEARTBEAT_OK` or specific alerts.
3. Zero Write or Agent tool calls are made.

## Invariants

1. Heartbeat is lightweight by design — no side effects.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | STATE.md missing | Alert; do not spawn agent to fix |
| EC-002 | `.factory/.git` missing | Alert; do not spawn agent to fix |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Healthy session | `HEARTBEAT_OK` | happy-path |
| STATE.md missing | Alert (read-only diagnosis) | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Heartbeat session shows zero Write or Agent tool calls | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/orchestrator/HEARTBEAT.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.02.001 — composes with (orchestrator never writes)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#heartbeat`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/orchestrator/HEARTBEAT.md:48-51` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit read-only rule in HEARTBEAT.md

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads only |
| **Global state access** | reads STATE.md, cost-summary.md, .factory/.git |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (read-only diagnostic) |

#### Refactoring Notes

No refactoring needed.
