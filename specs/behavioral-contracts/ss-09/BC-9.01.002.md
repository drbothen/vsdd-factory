---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [bc-id-mapping.md, pass-3-behavioral-contracts.md]
input-hash: "3efa098"
traces_to: bc-id-mapping.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:510"
subsystem: "SS-09"
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

# Behavioral Contract BC-9.01.002: chore commit (operator-staged) modifies only CHANGELOG.md

## Description

For the v1.0.0-beta.4 release, the operator-staged "chore" commit must modify ONLY `CHANGELOG.md`. The plugin.json and marketplace.json bumps are written atomically by the bot's binary-bundle commit, not by the operator. This separation is regression-pinned by the very plugin-cache staleness bug that motivated beta.4.

## Preconditions

1. v1.0.0-beta.4 (or any subsequent prerelease) release in progress.
2. Operator is staging a chore commit prior to the bot's binary-bundle commit.

## Postconditions

1. Chore commit's diff includes ONLY changes to CHANGELOG.md.
2. plugin.json and marketplace.json bumps land in the bot's binary-bundle commit, not the operator's chore commit.
3. Operators NEVER stage plugin.json / marketplace.json for the chore commit.

## Invariants

1. The plugin cache key never observes mismatched state between version metadata and binaries.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Operator accidentally stages plugin.json / marketplace.json | Plugin cache staleness recurs (the v1.0.0-beta.4 fix exists to prevent this — release tooling/process gates should refuse) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Operator runs the chore-commit step of the release workflow | git diff includes only CHANGELOG.md | happy-path |
| TBD | TBD | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD (release-pipeline scope; CAP-007 anchor reverted per Wave 5 pass-1 CRIT-002 — no activate-skill story exercises this BC; pending re-anchor to release-pipeline story) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-09 — release workflow + `plugins/vsdd-factory/.claude-plugin/plugin.json`, marketplace.json |
| Stories | TBD (reverted from S-0.03; BC is release-tooling scope, not platform-detection scope) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `<TBD>` (operator-staged release workflow) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `510` |

#### Evidence Types Used

- documentation (CHANGELOG v1.0.0-beta.4 "Plugin-cache staleness" fix; regression-pinned by the bug that drove beta.4)

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
