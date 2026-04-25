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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:765
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

# Behavioral Contract BC-5.02.001: orchestrator: never writes any files — delegates all writes

## Description

The orchestrator is a coordinator with read-only file access. It MUST delegate
every file write to a specialist agent via the Agent tool with `subagent_type` set.
Direct use of Write, Edit, apply_patch, exec, or process is forbidden.

## Preconditions

1. The orchestrator session has begun.
2. A pipeline step requires a file write.

## Postconditions

1. The orchestrator's effective tool profile excludes write/edit/apply_patch/exec/process.
2. Every file write is performed by a specialist agent dispatched via the Agent tool.
3. The dispatching Agent call sets `subagent_type` explicitly to a specialist agent.

## Invariants

1. The deny list is verified in agent frontmatter or runtime config.
2. The orchestrator never bypasses the deny list.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Orchestrator attempts Write directly | Tool denied at runtime |
| EC-002 | No specialist agent matches the required write | Halt and escalate |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Orchestrator dispatch session | Tool profile excludes write/edit/apply_patch/exec/process | happy-path |
| Orchestrator attempting Write | Denied by tool profile | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Orchestrator effective tool profile excludes Write | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/orchestrator/orchestrator.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.02.002 — composes with (orchestrator never delegates to itself)
- BC-5.10.002 — composes with (state-manager owns state writes)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#orchestrator-tool-profile`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/orchestrator/orchestrator.md:102, 372-377` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit denial in orchestrator agent body and Tool Access section

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads only |
| **Global state access** | reads pipeline state |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell (coordinator) |

#### Refactoring Notes

No refactoring needed — the constraint is enforced at the tool-profile layer.
