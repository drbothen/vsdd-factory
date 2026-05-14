---
document_type: behavioral-contract
level: L3
version: "v1.2"
last_amended: 2026-05-13
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: "1.4b"
inputs:
  - .factory/specs/behavioral-contracts/bc-id-mapping.md
  - .factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md
input-hash: "b115391"
traces_to: .factory/specs/behavioral-contracts/bc-id-mapping.md
origin: brownfield
extracted_from: "pass-3-behavioral-contracts-deep-r1.md:382"
subsystem: "SS-03"
capability: "CAP-003"
lifecycle_status: deprecated
introduced: v1.0.0-beta.4
modified: [v1.0.0-rc.1, v1.2-adv-E-10-pass-10]
deprecated: v1.0.0-rc.1
deprecated_by: BC-3.04.004
replacement: BC-3.04.004
retired: null
removed: null
removal_reason: null
---

# Behavioral Contract BC-3.04.001: Router is currently a thin pass-through wrapper around SinkRegistry

> Section: Sink router pass-through and extension point
> Source BC (audit ID): BC-AUDIT-123

## Description

`Router::new(SinkRegistry)` wraps a registry; `submit(SinkEvent)` just delegates to `registry.submit_all(event)`; `flush()` to `registry.flush_all()`; `shutdown()` to `registry.shutdown_all()`. There is no extra logic today. The module's docstring explicitly notes "no call sites exist — see the `TODO(integration)` in `sinks::mod.rs`".

## Preconditions

1. The dispatcher's main does not yet call `Router::submit`.

## Postconditions

1. `Router::new(SinkRegistry)` wraps a registry; `submit(SinkEvent)` just delegates to `registry.submit_all(event)`; `flush()` to `registry.flush_all()`; `shutdown()` to `registry.shutdown_all()`. There is no extra logic today. The module's docstring explicitly notes "no call sites exist — see the `TODO(integration)` in `sinks::mod.rs`".

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
| Architecture Module | SS-03 (Event Emission (OTel-Aligned)) — historically referenced crates/factory-dispatcher/src/sinks/router.rs (retired per ADR-015 D-15.1; BC-3.04.001 deprecated, superseded by BC-3.04.004) |
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
| **Original audit ID** | BC-AUDIT-123 |
| **Pass-3 source** | `pass-3-behavioral-contracts-deep-r1.md:382` |
| **Extraction Date** | 2026-04-25 |

**Evidence (from pass-3):**

> `crates/factory-dispatcher/src/sinks/router.rs § "module doc"` (docstring); `crates/factory-dispatcher/src/sinks/router.rs::Router` (Router::new, submit, flush, shutdown — each is one-line delegation); `crates/factory-dispatcher/src/sinks/mod.rs § "Integration Status"` (TODO(integration) note: main.rs wiring tracked separately).

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



## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.2 | 2026-05-13 | architect | D-346 E-10 pass-10 fix burst — F-3 closure: Architecture Module row corrected from stale `SS-03 (Observability Sinks)` to `SS-03 (Event Emission (OTel-Aligned))`; retired router.rs reference annotated per ADR-015 D-15.1. |
| v1.1 | 2026-05-08 | implementer | TD-VSDD-091 Chunk 6 — migrated 1 body cite: `sinks/router.rs:1-9` + `:33-47` + `sinks/mod.rs:11-21` → `sinks/router.rs § 'module doc'`, `sinks/router.rs::Router`, `sinks/mod.rs § 'Integration Status'`. |
