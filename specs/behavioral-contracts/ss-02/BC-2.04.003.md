---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [bc-id-mapping.md, pass-3-deep-rust-tests.md]
input-hash: "[pending-recompute]"
traces_to: bc-id-mapping.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-rust-tests.md:754"
subsystem: "SS-02"
capability: "CAP-009"
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

# Behavioral Contract BC-2.04.003: hook-sdk::payload::lifecycle_payload_without_tool_name — SessionStart parses with tool_name="" and tool_input is JSON null

## Description

A SessionStart envelope `{event_name:"SessionStart", session_id:"sess-x", dispatcher_trace_id:"trace-x"}` parses into a `HookPayload` with `tool_name == ""`, `tool_input.is_null() == true`, and `tool_response.is_none()`. SDK lifecycle hooks see no tool fields.

## Preconditions

1. Envelope has no `tool_name` and no `tool_input` keys.

## Postconditions

1. `tool_name == ""`.
2. `tool_input.is_null() == true`.
3. `tool_response.is_none()`.

## Invariants

1. Hooks for SessionStart / SessionEnd / SubagentStop don't fail when no tool is in scope.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | (No edge cases captured in Phase 0 extraction; to be added in Phase 1.5/test-writer pass) | TBD |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `{event_name:"SessionStart", session_id:"sess-x", dispatcher_trace_id:"trace-x"}` | tool_name=="", tool_input null, tool_response None | happy-path |
| TBD | TBD | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-009 |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-02 — `crates/hook-sdk/src/payload.rs` |
| Stories | S-1.03 |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/hook-sdk/src/payload.rs::tests::lifecycle_payload_without_tool_name` (lines 102–115) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-deep-rust-tests.md` line `754` |

#### Evidence Types Used

- assertion (unit test)

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | TBD (Phase 1.6b will refine) |
| **Global state access** | TBD |
| **Deterministic** | TBD |
| **Thread safety** | TBD |
| **Overall classification** | TBD |

#### Refactoring Notes

(TBD — to be assessed in Phase 1.6b verification properties pass)
