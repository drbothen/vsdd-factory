---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [bc-id-mapping.md, pass-3-behavioral-contracts-deep-r1.md]
input-hash: "[pending-recompute]"
traces_to: bc-id-mapping.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md:473"
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

# Behavioral Contract BC-4.02.005: Adapter resolves relative `script_path` under `${CLAUDE_PLUGIN_ROOT}`; absolute paths bypass the join

## Description

`is_absolute(p)` returns true when `p.starts_with('/')` (Unix root) OR `p[1] == ':'` (Windows drive letter). Absolute → use as-is. Relative → `join_path(plugin_root, script_path)`, which inserts a `/` separator if needed, respects existing trailing separators, and returns `rel` as-is when root is empty.

## Preconditions

1. `script_path` is non-empty.
2. `${CLAUDE_PLUGIN_ROOT}` may be set or empty.

## Postconditions

1. `script_path` matching Unix-absolute (`/foo`) or Windows-drive (`C:/foo`) forms is used as-is.
2. Relative `script_path` resolves under `${CLAUDE_PLUGIN_ROOT}` via `join_path`.
3. `join_path` inserts a separator only when the root has no trailing separator.
4. When `plugin_root` is empty, the relative path is returned as-is.

## Invariants

1. Path joining is purely textual and side-effect-free.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | plugin_root has trailing slash | Separator NOT duplicated |
| EC-002 | Windows-style drive letter `C:/...` | Treated as absolute |
| EC-003 | plugin_root empty + relative path | Returns relative path verbatim |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `script_path = "hooks/foo.sh"`, `plugin_root = "/p"` | `/p/hooks/foo.sh` | happy-path |
| `script_path = "/abs/foo.sh"` | `/abs/foo.sh` | edge-case |
| `script_path = "C:/foo.sh"` | `C:/foo.sh` | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD (anchor in Phase 1.5) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-04 — `crates/hook-plugins/legacy-bash-adapter/src/lib.rs` (run_bash_via_host + is_absolute + join_path) |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/hook-plugins/legacy-bash-adapter/src/lib.rs:135-186`; tests at :347-376 |
| **Confidence** | HIGH (5 unit tests pin Unix root, Windows drive, separator insertion, trailing separator, empty root) |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md` line `473` |

#### Evidence Types Used

- assertion (5 unit tests)

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
