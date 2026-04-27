---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: product-owner
timestamp: 2026-04-26T00:00:00Z
phase: 1.4b
inputs: [bc-id-mapping.md, pass-3-behavioral-contracts.md]
input-hash: "ff7795e"
traces_to: bc-id-mapping.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:412"
subsystem: "SS-01"
capability: "CAP-002"
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

# Behavioral Contract BC-1.07.004: registry-generation script is idempotent

## Description

Re-running `scripts/generate-registry-from-hooks-json.sh` against an existing registry produced from the current `hooks.json` produces no diff. 6 generate-registry bats tests cover this.

## Preconditions

1. An existing registry produced from the current `hooks.json` is present.

## Postconditions

1. Re-running the generator produces a byte-identical registry (no diff).

## Invariants

1. Generator is purely deterministic given a fixed `hooks.json`.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | (No edge cases captured in Phase 0 extraction; to be added in Phase 1.5/test-writer pass) | TBD |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Re-run generator on existing registry | No diff | happy-path |
| TBD | TBD | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-049 | Generated hooks-registry.toml round-trips through Registry::load (idempotent generation co-anchor) | integration |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-002 — registry-generation is the bridge from hooks.json (SS-09) to Registry::load (SS-01) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-01 + SS-09 — `scripts/generate-registry-from-hooks-json.sh`; cross-cuts SS-01 dispatcher Registry::load (frontmatter subsystem:SS-01 reflects BC ID convention; script lives in scripts/ which is SS-09-tooled) |
| Stories | S-2.02 |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | 6 generate-registry bats tests; CHANGELOG v1.0.0-beta.1 |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `412` |

#### Evidence Types Used

- assertion (bats tests)
- documentation (CHANGELOG)

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
