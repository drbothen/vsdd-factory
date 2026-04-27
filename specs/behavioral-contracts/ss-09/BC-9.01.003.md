---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [bc-id-mapping.md, pass-3-behavioral-contracts.md]
input-hash: "455ef24"
traces_to: bc-id-mapping.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:516"
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

# Behavioral Contract BC-9.01.003: release workflow's bot commit atomically writes binaries + plugin.json + marketplace.json

## Description

After a release tag is pushed, the release workflow's bot commit writes the per-platform dispatcher binaries together with `plugin.json` and `marketplace.json` in a single atomic commit. This guarantees that consumers fetching the plugin in any window see a consistent (version=X, matching binaries, matching metadata) tuple.

## Preconditions

1. A release tag has been pushed.
2. CI release workflow is running on the tag.

## Postconditions

1. Consumers fetching in any window see version=X with matching binaries.
2. Plugin cache key never observes mismatched state.
3. Binaries, plugin.json, and marketplace.json land in one atomic commit.

## Invariants

1. (binaries, plugin.json, marketplace.json) tuple is always consistent at any sampled instant after the bot commit lands.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | (No edge cases captured in Phase 0 extraction; to be added in Phase 1.5/test-writer pass) | TBD |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Tag pushed → bot commit | Single commit contains all binaries + plugin.json + marketplace.json updates for that version | happy-path |
| TBD | TBD | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-028 ("Install and update the plugin via Claude Code marketplace") per capabilities.md §CAP-028 — the atomic bot commit guarantees that a marketplace consumer fetching in any window sees a consistent (version=X, matching binaries, matching metadata) tuple |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-09 — release workflow + `plugins/vsdd-factory/.claude-plugin/plugin.json` + marketplace.json |
| Stories | S-2.04, S-2.08 |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `<TBD>` (release workflow YAML) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `516` |

#### Evidence Types Used

- documentation (CHANGELOG v1.0.0-beta.4)

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
