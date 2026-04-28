---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-04-28T00:00:00
phase: 1a
inputs:
  - .factory/stories/S-5.01-session-start-hook.md
  - .factory/specs/domain-spec/capabilities.md
input-hash: "9865e16"
traces_to: .factory/specs/prd.md#FR-046
origin: greenfield
extracted_from: null
subsystem: "SS-04"
capability: "CAP-002"
lifecycle_status: active
introduced: v1.0.0-rc.1
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-4.04.002: session-start plugin invokes factory-health subprocess; emits session.started even if check fails

## Description

The session-start plugin invokes `factory-health --brief` via the `exec_subprocess` host function immediately after receiving a `SessionStart` event. It captures the subprocess stdout and exit code and maps the result to the `factory_health` field (`"healthy"` | `"warnings"` | `"errors"`) in the `session.started` event payload. If the subprocess fails to execute for any reason (binary missing, permission denied, timeout), the plugin still emits `session.started` with `factory_health = "unknown"` — fail-open semantics per EC-001 in S-5.01.

## Preconditions

1. A `SessionStart` event has been routed to the session-start plugin (BC-4.04.004).
2. The `exec_subprocess` host function is available in the plugin's WASM runtime context.

## Postconditions

1. The plugin invokes `exec_subprocess("factory-health", ["--brief"])` before computing the `factory_health` field. The subprocess timeout is **5000ms (5 seconds)**.
2. The `session.started` event is emitted with `factory_health` set to exactly one of:
   - `"healthy"` — exit 0 AND stdout contains no line matching `^WARN(ING)?:` (case-sensitive prefix)
   - `"warnings"` — exit 0 AND stdout contains at least one line matching `^WARN(ING)?:` (case-sensitive prefix); all other exit-0 stdout that does not match is also mapped to `"healthy"`
   - `"errors"` — exit non-zero
   - `"unknown"` — subprocess execution failed (binary not found, permission denied, timeout, or other invocation error)
3. A subprocess execution failure (binary not found, permission denied, timeout) does NOT prevent `session.started` from being emitted.
4. The plugin returns `HookResult::Ok` regardless of whether `factory-health` succeeded or failed.

## Invariants

1. `session.started` is always emitted — subprocess failure is never a blocking condition.
2. `factory_health` field is always present in the emitted payload; it is never absent or null.
3. The `factory-health --brief` subprocess is invoked at most once per `SessionStart` event.
4. The `factory-health --brief` subprocess timeout is bounded at 5000ms; SessionStart plugin latency must not exceed this timeout plus plugin overhead (i.e., the plugin does not wait indefinitely for a slow subprocess).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `factory-health` binary is not found on PATH | `exec_subprocess` returns error; plugin sets `factory_health = "unknown"`; `session.started` emitted normally |
| EC-002 | `factory-health` exits with non-zero exit code | Plugin maps to `factory_health = "errors"`; `session.started` emitted normally |
| EC-003 | `factory-health --brief` subprocess exceeds 5000ms timeout | `exec_subprocess` returns timeout error; plugin sets `factory_health = "unknown"`; `session.started` still emitted within timeout + plugin_overhead bound |
| EC-004 | `factory-health` exits 0 but stdout contains a line matching `^WARN(ING)?:` | Plugin maps to `factory_health = "warnings"`; `session.started` emitted normally |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `SessionStart` event; `factory-health --brief` exits 0, stdout has no `^WARN(ING)?:` lines | `session.started` emitted with `factory_health = "healthy"`; `exec_subprocess` mock invocation count ≥ 1 | happy-path |
| `SessionStart` event; `factory-health` binary not found (`exec_subprocess` returns `Err(NotFound)`) | `session.started` emitted with `factory_health = "unknown"` | error |
| `SessionStart` event; `factory-health --brief` exits 1 (errors detected) | `session.started` emitted with `factory_health = "errors"` | edge-case |
| `SessionStart` event; `factory-health --brief` mock delays > 5000ms (timeout) | `exec_subprocess` returns timeout error; `session.started` emitted with `factory_health = "unknown"` | timeout |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-065 | Session-Start Plugin Surface Invariant — All BC-4.04.* Postconditions Hold in Integration Test | integration |

## Related BCs

- **BC-4.04.001** — composes with (provides the `factory_health` field value that BC-4.04.001 includes in the `session.started` payload)
- **BC-4.04.003** — composes with (idempotency guard prevents redundant subprocess invocations on duplicate events)
- **BC-4.04.004** — depends on (hooks.json.template registration triggers this plugin)

## Architecture Anchors

- SS-04 — `crates/hook-plugins/session-start-telemetry/src/lib.rs` (`exec_subprocess` call + `factory_health` mapping logic + 5000ms timeout constant)

## Story Anchor

S-5.01

## VP Anchors

VP-065

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-002 |
| Capability Anchor Justification | CAP-002 ("Hook Claude Code tool calls with sandboxed WASM plugins") per capabilities.md §CAP-002 |
| L2 Domain Invariants | DI-004 (capability denial emits audit event — exec_subprocess capability gate for factory-health must emit internal.capability_denied if denied); DI-011 (sink submit must not block — subprocess invocation bounded by 5000ms timeout to preserve dispatcher latency) |
| Architecture Module | SS-04 — `crates/hook-plugins/session-start-telemetry/src/lib.rs` |
| Stories | S-5.01 |
| Functional Requirement | FR-046 |
