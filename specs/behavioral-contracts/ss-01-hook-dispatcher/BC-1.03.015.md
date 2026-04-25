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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-rust-tests.md:519"
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

# Behavioral Contract BC-1.03.015: factory-dispatcher::executor (integration)::multi_tier_runs_in_priority_order — tier 10 plugin executes before tier 100 plugins

## Description

Integration test: 3 plugins — `late-a` (priority=100), `early` (priority=10), `late-b` (priority=100). After `execute_tiers`, `summary.per_plugin_results[0].plugin_name == "early"`; remaining slots contain `late-a` and `late-b` (order within tier unspecified). Tier ordering is observable in the per_plugin_results vec.

## Preconditions

1. 3 plugins with mixed priorities (10 / 100 / 100).

## Postconditions

1. First result is `early` (priority=10).
2. Subsequent slots contain `late-a` and `late-b` (order within same-priority unspecified).

## Invariants

1. Tier order is reflected in result vec order; same-priority within-tier ordering is unspecified.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | (No edge cases captured in Phase 0 extraction; to be added in Phase 1.5/test-writer pass) | TBD |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Plugins late-a (100), early (10), late-b (100) | First result is `early`; rest contains both `late-a` and `late-b` | happy-path |
| TBD | TBD | edge-case |
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
| Architecture Module | SS-01 — `crates/factory-dispatcher/tests/executor_integration.rs` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/factory-dispatcher/tests/executor_integration.rs::multi_tier_runs_in_priority_order` (lines 217–248) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-deep-rust-tests.md` line `519` |

#### Evidence Types Used

- assertion (integration test)

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
