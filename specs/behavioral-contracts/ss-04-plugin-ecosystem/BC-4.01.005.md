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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:386"
subsystem: "SS-04"
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

# Behavioral Contract BC-4.01.005: legacy-bash-adapter caps wall-clock at 60_000ms (backstop only)

## Description

The legacy-bash-adapter declares `BASH_TIMEOUT_MS = 60_000` as a wall-clock backstop. In practice the dispatcher's per-plugin epoch deadline (default 5_000 ms) fires first, so this cap exists only as a defensive backstop in case the dispatcher's epoch interrupt does not fire.

## Preconditions

1. A bash hook is invoked through the adapter.
2. The bash hook hangs.

## Postconditions

1. `BASH_TIMEOUT_MS = 60_000` is the adapter-side cap.
2. The dispatcher's epoch-interruption deadline (default 5_000 ms) is the source of truth and fires first in normal operation.

## Invariants

1. The adapter cap is always >= the dispatcher's epoch deadline (defense in depth).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Dispatcher epoch ticker misconfigured / not running | Adapter cap fires after 60s as a last resort |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Hung bash hook with default 5s dispatcher epoch deadline | Dispatcher times out via epoch first; adapter backstop never fires | happy-path |
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
| Architecture Module | SS-04 — `crates/hook-plugins/legacy-bash-adapter/src/lib.rs` (BASH_TIMEOUT_MS constant) |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/hook-plugins/legacy-bash-adapter/src/lib.rs` (BASH_TIMEOUT_MS constant) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `386` |

#### Evidence Types Used

- type constraint (constant)
- documentation (design comment)

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
