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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md:449"
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

# Behavioral Contract BC-4.02.002: Adapter exit-code mapping: 0 → Continue, 2 → Block (reason=first stderr line OR synthetic), other → Error (message includes script path + code + stderr)

## Description

For every bash exit code N, the legacy-bash-adapter produces a deterministic `HookResult`. N==0 → Continue; N==2 → Block (reason = first non-empty stderr line, or `"legacy bash hook {script_path} blocked"` synthetic if stderr is empty); any other N → Error with message `"legacy bash hook {script_path} exited with code {N}: {stderr}"`.

## Preconditions

1. The bash subprocess has exited with code N.
2. The adapter has captured stderr.

## Postconditions

1. `N == 0` → `HookResult::Continue`.
2. `N == 2` and stderr non-empty → `Block { reason = first non-empty stderr line }`.
3. `N == 2` and stderr empty → `Block { reason = "legacy bash hook {script_path} blocked" }` synthetic.
4. Other N → `HookResult::Error { message = "legacy bash hook {script_path} exited with code {N}: {stderr}" }`.

## Invariants

1. Mapping is total over `i32`.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | exit 2 with multi-line stderr | First non-empty stderr line is used as the reason |
| EC-002 | exit 2 with empty stderr | Synthetic reason `"legacy bash hook {script_path} blocked"` |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `N=0` | Continue | happy-path |
| `N=2`, stderr "denied: foo" | Block { reason: "denied: foo" } | edge-case |
| `N=42`, stderr "boom" | Error { message: "legacy bash hook {path} exited with code 42: boom" } | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD (anchor in Phase 1.5) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-04 — `crates/hook-plugins/legacy-bash-adapter/src/lib.rs` (exit-code match arms) |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/hook-plugins/legacy-bash-adapter/src/lib.rs:103-119` |
| **Confidence** | HIGH (5 distinct test cases pin every branch) |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md` line `449` |

#### Evidence Types Used

- assertion (5 unit tests pin every arm)

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
