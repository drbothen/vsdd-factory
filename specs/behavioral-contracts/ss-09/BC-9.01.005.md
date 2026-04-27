---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [bc-id-mapping.md, pass-3-behavioral-contracts.md]
input-hash: "993457a"
traces_to: bc-id-mapping.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:528"
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

# Behavioral Contract BC-9.01.005: hooks.json is gitignored; hooks.json.template + per-platform variants are committed

## Description

The runtime `hooks.json` file is gitignored — the activate skill writes it at runtime by copying the appropriate `hooks.json.<platform>` variant. The repository tracks `hooks.json.template` plus 5 per-platform variants. This is per ADR-009 (activation-skill-driven platform binary selection).

## Preconditions

1. A clone of the repository exists with `.gitignore` honored.
2. The activate skill has not yet run on the working copy.

## Postconditions

1. `hooks.json` is NOT tracked in git (gitignored).
2. `hooks.json.template` IS tracked in git.
3. 5 per-platform variants (`hooks.json.<darwin-arm64|darwin-x64|linux-x64|linux-arm64|windows-x64>`) ARE tracked in git.
4. The activate skill writes `hooks.json` at runtime by copying from the platform variant.

## Invariants

1. `hooks.json` is never committed under any circumstance.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Operator stages hooks.json by accident | Should be blocked or reverted; tracked file would diverge from per-platform truth |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Fresh clone | hooks.json absent from working tree until activate runs; .template + 5 variants present | happy-path |
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
| L2 Domain Invariants | TBD |
| Architecture Module | SS-09 — `plugins/vsdd-factory/hooks/hooks.json*`, `.gitignore`, ADR-009 |
| Stories | S-2.06 |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/hooks/hooks.json*`, `.gitignore` |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `528` |

#### Evidence Types Used

- documentation (Story S-0.4; .gitignore Phase 0 step 3)

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
