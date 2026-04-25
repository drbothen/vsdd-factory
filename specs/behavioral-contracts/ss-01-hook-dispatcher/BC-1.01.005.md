---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [bc-id-mapping.md, pass-3-behavioral-contracts.md]
input-hash: "[pending-recompute]"
traces_to: bc-id-mapping.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:54"
subsystem: "SS-01"
capability: "CAP-TBD"
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

# Behavioral Contract BC-1.01.005: Plugin filter requires event match AND (no tool OR tool regex matches)

## Description

`match_plugins` returns the subset of registry entries where `enabled` AND `event == event_name` AND (`entry.tool is None` OR regex(entry.tool) matches `tool_name`). Disabled entries are omitted regardless of other criteria.

## Preconditions

1. A `HookPayload` has an `event_name` and a `tool_name`.
2. The registry has a mix of tool-bound and tool-free entries.

## Postconditions

1. Returned subset includes only entries where `enabled` is true.
2. Returned subset includes only entries where `event == event_name`.
3. Returned subset includes only entries where `entry.tool` is None OR `regex(entry.tool)` matches `tool_name`.

## Invariants

1. Disabled entries are excluded unconditionally.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Entry has no tool, payload has any tool_name | Match |
| EC-002 | Entry has tool regex with anchoring (`^Edit$`) | Tool name must match the anchored regex |
| EC-003 | Entry uses regex alternation `Edit\|Write` | Either matches |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Registry entry tool `Edit\|Write`, payload tool_name `Edit` | Matches | happy-path |
| Disabled entry, payload matches everything else | Excluded | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD (anchor in Phase 1.5) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/routing.rs` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `routing.rs::tests::{match_filters_by_event_name, match_skips_disabled_entries, match_includes_no_tool_entries_for_any_tool, match_respects_tool_regex_anchoring, match_regex_alternation}` |
| **Confidence** | HIGH (5 distinct test cases pin every branch) |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `54` |

#### Evidence Types Used

- assertion (5 unit tests)

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
