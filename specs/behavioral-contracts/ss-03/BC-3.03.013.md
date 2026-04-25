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
  - .factory/phase-0-ingestion/pass-3-deep-rust-tests.md
input-hash: "2c315c8"
traces_to: .factory/specs/behavioral-contracts/bc-id-mapping.md
origin: brownfield
extracted_from: "pass-3-deep-rust-tests.md:1090"
subsystem: "SS-03"
capability: "TBD"
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

# Behavioral Contract BC-3.03.013: sink-otel-grpc::resource_attributes_merge_defaults_with_config: operator overrides win over auto-

> Section: OTLP gRPC sink batching and lifecycle
> Source BC (audit ID): BC-AUDIT-2389

## Description

Given Operator overrides include `service.name="custom-svc"`, `deployment.env="dev"`. When `build_resource_attributes(&overrides)`.. Then service.name=="custom-svc" (override wins); deployment.env=="dev" (operator-only); host.name is auto-populated from gethostname() and non-empty.

## Preconditions

1. Operator overrides include `service.name="custom-svc"`, `deployment.env="dev"`.

## Postconditions

1. service.name=="custom-svc" (override wins); deployment.env=="dev" (operator-only); host.name is auto-populated from gethostname() and non-empty.

## Invariants

1. Resource attributes blend operator config with sensible defaults.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | TBD — Phase 1.6b will enumerate edge cases from source review | TBD |

## Canonical Test Vectors

> Test vectors will be pulled from the pinned test case in Phase 1.6b.

| Input | Expected Output | Category |
|-------|----------------|----------|
| Operator overrides include `service.name="custom-svc"`, `deployment.env="dev"`. | service.name=="custom-svc" (override wins); deployment.env=="dev" (operator-only); host.name is auto-populated from geth | happy-path |
| TBD | TBD | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Given Operator overrides include `service.name="custom-svc"`, `deployment.env="dev"`. When `build_resource_attributes(&o | manual (existing test: `crates/sink-otel-grpc/src/lib.rs::tests::resource_attributes_merge_defaults_with_config`) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD (subsystem L2 spec pending) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-03 (OTLP gRPC sink batching and lifecycle) |
| Stories | TBD (Phase 2 story-writer pass) |

## Related BCs (Recommended)

- TBD — cross-references will be filled in Phase 1.6b after all per-BC files exist.

## Architecture Anchors (Recommended)

- `architecture/SS-03-observability-sinks.md` — section: OTLP gRPC sink batching and lifecycle

---

### Brownfield-Specific Sections

> This BC was extracted during Phase 0d brownfield ingestion (BC-AUDIT pass) and migrated to canonical one-per-file BC-S.SS.NNN format in Phase 1.4b.

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/sink-otel-grpc/src/lib.rs` |
| **Source line(s)** | 916–943 |
| **Confidence** | high |
| **Original audit ID** | BC-AUDIT-2389 |
| **Pass-3 source** | `pass-3-deep-rust-tests.md:1090` |
| **Extraction Date** | 2026-04-25 |
| **Pinned test** | ``crates/sink-otel-grpc/src/lib.rs::tests::resource_attributes_merge_defaults_with_config`` |
| **Test type** | unit |

#### Evidence Types Used

- **assertion**: pinned by Rust unit/integration test

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

