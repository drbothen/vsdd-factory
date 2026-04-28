---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: "1.4b"
inputs:
  - .factory/specs/behavioral-contracts/bc-id-mapping.md
  - .factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md
input-hash: "88162d7"
traces_to: .factory/specs/behavioral-contracts/bc-id-mapping.md
origin: brownfield
extracted_from: "pass-3-behavioral-contracts-deep-r1.md:389"
subsystem: "SS-03"
capability: "CAP-003"
lifecycle_status: fulfilled
introduced: v1.0.0-beta.4
modified: [v1.0.0-rc.1]
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# Behavioral Contract BC-3.04.002: Router exists as the future extension point for S-4.x retry / circuit-breaker / batching / routing

> Section: Sink router pass-through and extension point
> Source BC (audit ID): BC-AUDIT-124

## Description

The Router module documents the intended extension surface: "Stable extension point that S-4.x will graft retry / circuit-breaker / batching behavior in at this layer without touching the call sites or the driver implementations." Rust `pub fn registry(&self) -> &SinkRegistry` is exposed for tests but is a public surface that suggests future inspection use.

## Preconditions

1. Tier E stories S-4.4 through S-4.6 haven't shipped.

## Postconditions

1. The Router module documents the intended extension surface: "Stable extension point that S-4.x will graft retry / circuit-breaker / batching behavior in at this layer without touching the call sites or the driver implementations." Rust `pub fn registry(&self) -> &SinkRegistry` is exposed for tests but is a public surface that suggests future inspection use.

## Invariants

1. TBD — invariants not explicitly stated in source pass-3 entry.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | TBD — Phase 1.6b will enumerate edge cases from source review | TBD |

## Canonical Test Vectors

> Test vectors will be pulled from the pinned test case in Phase 1.6b.

| Input | Expected Output | Category |
|-------|----------------|----------|
| TBD | TBD | happy-path |
| TBD | TBD | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | TBD — Phase 1.6b will identify formal verification properties | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD (subsystem L2 spec pending) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-03 (Sink router pass-through and extension point) |
| Stories | TBD (Phase 2 story-writer pass) |

## Related BCs (Recommended)

- TBD — cross-references will be filled in Phase 1.6b after all per-BC files exist.

## Architecture Anchors (Recommended)

- `architecture/SS-03-observability-sinks.md` — section: Sink router pass-through and extension point

---

### Brownfield-Specific Sections

> This BC was extracted during Phase 0d brownfield ingestion (BC-AUDIT pass) and migrated to canonical one-per-file BC-S.SS.NNN format in Phase 1.4b.

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/factory-dispatcher/src/sinks/router.rs` |
| **Source line(s)** | TBD |
| **Confidence** | high |
| **Original audit ID** | BC-AUDIT-124 |
| **Pass-3 source** | `pass-3-behavioral-contracts-deep-r1.md:389` |
| **Extraction Date** | 2026-04-25 |

**Evidence (from pass-3):**

> `sinks/router.rs:1-9, 17-22` (docstring + struct-level comment). Pass-6 DRIFT-002 / DRIFT-005 / story-coverage S-4.4..S-4.6 corroborate.

#### Evidence Types Used

- **inferred**: from pass-3 narrative; no explicit assertion captured

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | TBD — Phase 1.6b will classify |
| **Global state access** | TBD |
| **Deterministic** | TBD |
| **Thread safety** | TBD |
| **Overall classification** | TBD |

#### Refactoring Notes

TBD — Phase 1.6b will produce refactoring guidance.

