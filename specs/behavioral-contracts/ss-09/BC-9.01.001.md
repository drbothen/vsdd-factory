---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [bc-id-mapping.md, pass-3-behavioral-contracts.md]
input-hash: "c5cd844"
traces_to: bc-id-mapping.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:504"
subsystem: "SS-09"
capability: "CAP-028"
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

# Behavioral Contract BC-9.01.001: bump-version.sh accepts semver release format (stable N.N.N + prerelease 1.0.0-beta.N / 1.0.0-rc.N)

## Description

bump-version.sh accepts semver release format — stable releases of the form `N.N.N` (e.g., `1.0.0`, `1.1.0`) AND prerelease versions of the form `1.0.0-beta.N` and `1.0.0-rc.N`. This allows the release workflow to bump versions through prerelease cycles AND promote to stable when the rc.N gate passes.

## Preconditions

1. Release workflow run is in progress.
2. A semver prerelease version string (e.g., `1.0.0-beta.N`, `1.0.0-rc.N`) is supplied to the bump tool.

## Postconditions

1. Version bump succeeds.
2. CHANGELOG retains monotonicity (later bump strictly greater than prior).

## Invariants

1. Version numbering increases monotonically across CHANGELOG entries — stable bumps (N.N.N → N.(N+1).N) and prerelease bumps (1.0.0-beta.N → 1.0.0-beta.(N+1) or 1.0.0-rc.M) both maintain ordering.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | (No edge cases captured in Phase 0 extraction; to be added in Phase 1.5/test-writer pass) | TBD |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Story S-0.1 release-cycle invocation through 1.0.0-beta.4 | Version bump succeeds and CHANGELOG monotonicity holds | happy-path |
| TBD | TBD | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-028 ("Install and update the plugin via Claude Code marketplace") per capabilities.md §CAP-028 — bump-version.sh enables version co-stamping across plugin.json, marketplace.json, and CHANGELOG, which is the precondition for every marketplace install reporting the correct version |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-09 — `scripts/bump-version.sh` (prerelease semver validation and version co-stamping) |
| Stories | S-0.01, S-2.08, S-0.02, S-4.08, S-5.07 |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `<TBD>` (bump-version.sh location not cited in source extraction) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `504` |

#### Evidence Types Used

- documentation (CHANGELOG release entries through 1.0.0-beta.4)

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
