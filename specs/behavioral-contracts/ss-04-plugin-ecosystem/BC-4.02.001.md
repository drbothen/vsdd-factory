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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md:442"
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

# Behavioral Contract BC-4.02.001: Adapter forwards stdout AND stderr to host log via `host::log_info` / `host::log_warn` (per-stream, non-empty)

## Description

After the bash subprocess completes, the legacy-bash-adapter forwards captured streams to the host log using stream-symmetric severity: stdout → `host::log_info` (debug-level chatter), stderr → `host::log_warn` (operational signal). Empty streams are dropped (no noise). This is the v1.0.0-beta.4 stderr-capture wiring.

## Preconditions

1. The bash subprocess has emitted any stdout or stderr.

## Postconditions

1. If stdout is non-empty → adapter calls `host::log_info(format!("legacy-bash[{path}] stdout: {stdout}"))`.
2. If stderr is non-empty → adapter calls `host::log_warn(format!("legacy-bash[{path}] stderr: {stderr}"))`.
3. Empty streams are not forwarded.

## Invariants

1. Stream-routing severity: stdout → info; stderr → warn. Both lifecycle-event-visible.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Both streams empty | No log_info / log_warn calls |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Bash hook emits "ok" to stdout, "warn" to stderr | log_info("legacy-bash[...] stdout: ok") + log_warn("legacy-bash[...] stderr: warn") | happy-path |
| TBD | TBD | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD (anchor in Phase 1.5) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-04 — `crates/hook-plugins/legacy-bash-adapter/src/lib.rs` (post-exec_subprocess forwarding loop) |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/hook-plugins/legacy-bash-adapter/src/lib.rs:151-158` |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md` line `442` |

#### Evidence Types Used

- assertion (CHANGELOG v1.0.0-beta.4 stderr-capture wiring)

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
