---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-05-03T00:00:00Z
phase: 1.4b
inputs: [gap-analysis-w16-subprocess.md]
input-hash: "[pending-recompute]"
traces_to: gap-analysis-w16-subprocess.md
origin: brownfield
extracted_from: ".factory/architecture/gap-analysis-w16-subprocess.md:Section 5"
subsystem: "SS-01"
capability: "CAP-TBD"
lifecycle_status: active
introduced: v1.0.0
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# Behavioral Contract BC-1.05.036: factory-dispatcher::host::exec_subprocess::emits_completed_event_on_success — host.exec_subprocess.completed event on every successful subprocess completion

## Description

On successful subprocess completion (any exit code, including non-zero), the dispatcher MUST emit a `host.exec_subprocess.completed` event through the normal sink chain. This closes an observability gap identified in gap-analysis-w16-subprocess.md Section 5: `exec_subprocess.rs:285-288` currently has no emit call on success. The new event is success-path only; existing denial-path events are preserved unchanged.

## Preconditions

1. Plugin calls `vsdd::exec_subprocess`.
2. Capability check passes (binary is on allow-list, no traversal, no shell bypass violation).
3. Subprocess spawns and completes (any exit code).

## Postconditions

1. Exactly one `host.exec_subprocess.completed` event is emitted via `ctx.emit_internal`.
2. Event payload includes all 8 fields: `{plugin_id: String, binary: String /* canonicalized full path */, args_count: u32, exit_code: i32, duration_ms: u64, stdout_bytes: u64, stderr_bytes: u64, truncated: bool}`.
3. `duration_ms` is measured from `Instant::now()` at `Command::spawn()` to process exit; the deadline `Instant` already present in `execute_bounded` (exec_subprocess.rs:270) is the reference.
4. Event is routed through the normal `ctx.emit_internal` sink chain (file/datadog/honeycomb per config), the same path as the existing `emit_denial` call.
5. On error paths (capability denied, timeout, output too large), the existing distinct events continue to fire; `host.exec_subprocess.completed` is NOT emitted on error paths.

## Invariants

1. Success-path telemetry is emitted for every subprocess completion regardless of the subprocess's own exit code (exit code 0 and non-zero both trigger the event).

## Related BCs

- BC-1.05.001 — exec_subprocess capability check (depends on: this event fires only after the capability check passes)
- BC-1.05.032 — timeout enforcement (sibling: timeout path emits a different event; this event is NOT emitted on timeout)
- BC-1.05.005 — OUTPUT_TOO_LARGE path (sibling: output-too-large path emits a different event; this event is NOT emitted)
- BC-1.05.035 — path canonicalization guard (sibling extension from same gap analysis)

## Architecture Anchors

- `crates/factory-dispatcher/src/host/exec_subprocess.rs:285-288` — current success path with no emit call; `emit_event!("host.exec_subprocess.completed", ...)` added here
- `crates/factory-dispatcher/src/host/exec_subprocess.rs:304-309` — existing `emit_denial` call; new event follows the same pattern
- `.factory/architecture/gap-analysis-w16-subprocess.md` Section 5 — authority for this extension

## Story Anchor

S-9.07 (validate-wave-gate-prerequisite WASM port) — implementation task

## VP Anchors

(TBD — to be assigned in Phase 1.6b)

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Subprocess exits 0 | `host.exec_subprocess.completed` emitted with `exit_code=0` |
| EC-002 | Subprocess exits non-zero (e.g., 1) | `host.exec_subprocess.completed` emitted with `exit_code=1` |
| EC-003 | Capability check fails | `internal.capability_denied` emitted; `host.exec_subprocess.completed` NOT emitted |
| EC-004 | Subprocess times out | Timeout error event emitted; `host.exec_subprocess.completed` NOT emitted |
| EC-005 | Subprocess output exceeds cap | `OUTPUT_TOO_LARGE` path; `host.exec_subprocess.completed` NOT emitted |
| EC-006 | Payload field type check | All 8 fields present with declared types (`plugin_id: String`, `binary: String`, `args_count: u32`, `exit_code: i32`, `duration_ms: u64`, `stdout_bytes: u64`, `stderr_bytes: u64`, `truncated: bool`) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Capability passes; subprocess exits 0 | Exactly one `host.exec_subprocess.completed` event; `exit_code=0` | happy-path |
| Capability passes; subprocess exits 1 | Exactly one `host.exec_subprocess.completed` event; `exit_code=1` | happy-path |
| Capability check fails | `internal.capability_denied` emitted; `host.exec_subprocess.completed` NOT emitted | error |
| Subprocess timeout | Timeout event emitted; `host.exec_subprocess.completed` NOT emitted | error |
| Subprocess output exceeds cap | OutputTooLarge path; `host.exec_subprocess.completed` NOT emitted | error |
| Successful completion | Event payload contains all 8 fields with correct types | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD (anchor in Phase 1.5) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/host/exec_subprocess.rs` |
| Stories | S-9.07 |
| Capability Anchor Justification | CAP-TBD — capability anchor to be confirmed in Phase 1.5; this BC governs exec_subprocess success-path observability in factory-dispatcher |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `.factory/architecture/gap-analysis-w16-subprocess.md` Section 5 item 2; `crates/factory-dispatcher/src/host/exec_subprocess.rs:285-288` |
| **Confidence** | HIGH (gap explicitly identified by architect gap analysis; analogous pattern exists in emit_denial at lines 304–309) |
| **Extraction Date** | 2026-05-03 |
| **Extracted from** | `.factory/architecture/gap-analysis-w16-subprocess.md` Section 5 |

#### Evidence Types Used

- gap-analysis (architect-identified missing emit call at exec_subprocess.rs:285–288)
- assertion (analogous emit_denial pattern at exec_subprocess.rs:304–309 confirms implementation pattern)

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | YES — emits event via ctx.emit_internal sink chain (non-blocking try_send) |
| **Global state access** | No |
| **Deterministic** | YES — event emission is deterministic given subprocess completion |
| **Thread safety** | YES — follows same pattern as existing emit_denial |
| **Overall classification** | Deterministic with best-effort sink I/O |

#### Refactoring Notes

(TBD — to be assessed in Phase 1.6b verification properties pass)
