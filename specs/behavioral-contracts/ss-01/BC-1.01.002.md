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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:36"
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

# Behavioral Contract BC-1.01.002: Registry rejects invalid tool regex at load time

## Description

When a registry entry's `tool` field is not a valid regex, `Registry::parse_str` rejects the registry at load time. Routing-time regex compilation never sees an invalid pattern; the unreachable error branch in `routing.rs::tool_matches` falls back to non-match.

## Preconditions

1. A `RegistryEntry.tool` field is present and is not a valid regex.

## Postconditions

1. `Registry::parse_str` returns `Err(RegistryError::ToolRegex { name, pattern, source })`.
2. Routing-time regex compilation never observes an invalid pattern.

## Invariants

1. Tool-regex validity is verified at load time, not lazily at routing time.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Empty regex `""` | TBD (whether allowed or rejected; not pinned by source) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Entry with `tool = "[invalid"` | `Err(RegistryError::ToolRegex { name, pattern, source })` | error |
| Entry with `tool = "Edit\|Write"` | Ok | happy-path |
| TBD | TBD | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD (anchor in Phase 1.5) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/registry.rs`, `routing.rs` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/factory-dispatcher/src/registry.rs::tests::rejects_invalid_tool_regex` (lines 407–422); `routing.rs::tool_matches` |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `36` |

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
