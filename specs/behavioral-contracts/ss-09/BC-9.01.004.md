---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [bc-id-mapping.md, pass-3-behavioral-contracts.md]
input-hash: "dd75ae2"
traces_to: bc-id-mapping.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:522"
subsystem: "SS-09"
capability: "CAP-007"
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

# Behavioral Contract BC-9.01.004: 5-platform CI matrix is the build matrix; drift gated by check-platforms-drift.py

## Description

The CI build matrix is exactly five platforms (darwin-arm64, darwin-x64, linux-x64, linux-arm64, windows-x64), pinned in `ci/platforms.yaml`. A `check-platforms-drift.py` gate enforces that the matrix and any per-platform variant artifacts stay in sync.

## Preconditions

1. CI run in progress on a release-bearing branch / tag.
2. `ci/platforms.yaml` exists and declares the canonical 5-platform list.

## Postconditions

1. All 5 platforms are green in CI.
2. The 5 platforms are pinned in `ci/platforms.yaml`.
3. The drift gate enforces: any divergence between `ci/platforms.yaml` and downstream artifact lists fails CI.

## Invariants

1. The 5 platforms enumerated in `ci/platforms.yaml` are the single source of truth for the build matrix.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | A new platform variant added to one place but not the other | check-platforms-drift.py fails CI |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| CI run on a branch with consistent platform manifests | All 5 platforms green; drift gate passes | happy-path |
| TBD | TBD | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-007 ("Deploy and activate the plugin on any supported platform") per capabilities.md §CAP-007 |
| L2 Domain Invariants | DI-015 (per-project activation gate prerequisite — the 5-platform CI matrix produces the gate-ready artifacts) |
| Architecture Module | SS-09 — `ci/platforms.yaml`, `ci/check-platforms-drift.py` |
| Stories | S-0.03 |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `ci/platforms.yaml` (and surrounding `ci/` directory) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `522` |

#### Evidence Types Used

- documentation (CHANGELOG v1.0.0-beta.1)

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
