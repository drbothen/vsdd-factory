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
input-hash: "2f50188"
traces_to: .factory/specs/prd.md#FR-046
origin: greenfield
extracted_from: null
subsystem: "SS-04"
capability: "CAP-002"
lifecycle_status: active
introduced: v1.0.0-rc.1
modified: [v1.0-pass-1, v1.0-pass-2]
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-4.04.005: hooks-registry.toml registers SessionStart event-name routing to session-start-telemetry.wasm with once:true and exec_subprocess capability

## Description

The dispatcher reads `plugins/vsdd-factory/hooks-registry.toml` (owned by SS-07 Hook Bash Layer) to map event names to WASM plugin paths. The `SessionStart` entry in that file MUST exist with `event = "SessionStart"`, `plugin = "session-start-telemetry.wasm"`, `once = true`, `capabilities = ["exec_subprocess"]`, and `epoch_budget_ms = 8000`. This is Layer 2 of the dual-routing-tables pattern per ADR-011 — the dispatcher-side counterpart to BC-4.04.004's `hooks.json.template` entry. The `once = true` flag aligns with BC-4.04.003's idempotency contract (now enforced dispatcher-side per BC-1.10.002); the `exec_subprocess` capability declaration is required for the factory-health subprocess invocation described in BC-4.04.002. The `hooks-registry.toml` entry is added by **direct edit** per the file's own header comment ("Human-edited source of truth as of v1.0.0. The v0.79.x→v1.0 migration generator has been retired; edit this file directly."). The legacy `generate-registry-from-hooks-json.sh` script is retired and MUST NOT be modified for this purpose.

## Preconditions

1. `plugins/vsdd-factory/hooks-registry.toml` is the dispatcher-side routing source of truth (per ADR-011 dual-hook-routing-tables, SS-07 ownership).
2. The `hooks-registry.toml` file is syntactically valid TOML and passes dispatcher schema validation (`schema_version = 1`).
3. The `session-start-telemetry.wasm` binary is registered in the dispatcher's plugin registry (or will be loaded from the plugin directory at dispatch time).

## Postconditions

1. `hooks-registry.toml` contains a `[[hooks]]` entry (or equivalent TOML structure) with `event_name = "SessionStart"`.
2. The `SessionStart` entry specifies `plugin_path = "session-start-telemetry.wasm"`.
3. The `SessionStart` entry has `once = true`.
4. The `SessionStart` entry declares `capabilities = ["exec_subprocess"]` (required for BC-4.04.002 factory-health subprocess invocation).
5. The `SessionStart` entry has `epoch_budget_ms = 8000`. Justification: the plugin's 5000ms subprocess timeout (BC-4.04.002 Invariant 4) plus plugin startup, payload parse, host-fn calls, and `exec_subprocess` setup overhead exceeds the 5000ms global epoch default (CAP-011). The 8000ms value gives 3000ms headroom above the subprocess timeout and prevents `Timeout{Epoch}` from killing the plugin before it can map a subprocess timeout to `factory_health = "unknown"` (per BC-4.04.002 EC-003).
6. Dispatcher successfully loads the entry without error at startup.

## Invariants

1. The `hooks-registry.toml` `SessionStart` entry must remain registered through all v1.0 releases — removal requires a deprecation pass.
2. `once = true` is immutable for `SessionStart` in this file — changing to `once = false` requires a new BC and explicit justification.
3. `exec_subprocess` must remain in the capabilities list as long as BC-4.04.002 requires it; removing the capability would cause the dispatcher to refuse the factory-health subprocess call (per DI-004).
4. `plugin_path` must stay in sync with the canonical crate name `session-start-telemetry.wasm`; any rename propagates to this entry in the same commit.
5. Entries declaring `exec_subprocess` capability MUST declare `epoch_budget_ms` exceeding the longest expected subprocess wait. For the `SessionStart` entry, this means `epoch_budget_ms > 5000` (the subprocess timeout per BC-4.04.002 Invariant 4); `8000` is the required minimum.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `exec_subprocess` capability missing from the `SessionStart` entry | Dispatcher refuses to invoke `factory-health` subprocess; emits `internal.capability_denied` per DI-004; plugin sets `factory_health = "unknown"` (fail-open per BC-4.04.002); `session.started` still emitted |
| EC-002 | `SessionStart` entry is duplicated in `hooks-registry.toml` | Dispatcher uses the first matching entry; logs a warning for the duplicate; behavior is otherwise per the first entry's fields |
| EC-003 | `hooks-registry.toml` contains a TOML syntax error | Dispatcher fails to start; emits `internal.dispatcher_error` with structured error message including file path and parse error; dispatcher exits non-zero |
| EC-004 | `epoch_budget_ms` field is absent from the `SessionStart` entry | Dispatcher uses CAP-011 global default (5000ms); the plugin's 5000ms subprocess timeout (BC-4.04.002 Invariant 4) plus plugin overhead causes `Timeout{Epoch}` before the plugin can map a subprocess timeout to `factory_health = "unknown"` (EC-003 in BC-4.04.002 becomes unreachable); integration tests that rely on the timeout path will fail. This is a defect, not a fail-open — the missing field must be treated as a required field validation error. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Parse `hooks-registry.toml` and inspect entries | `SessionStart` entry present with `plugin_path = "session-start-telemetry.wasm"`, `once = true`, `capabilities` includes `"exec_subprocess"` | happy-path |
| `hooks-registry.toml` `SessionStart` entry with `capabilities = []` (empty) | Dispatcher refuses factory-health subprocess call; `internal.capability_denied` emitted; `session.started` still emitted with `factory_health = "unknown"` | error (capability denied) |
| `hooks-registry.toml` with duplicate `SessionStart` entries (first valid, second malformed) | Dispatcher uses first entry; logs warning about duplicate; `session.started` emitted normally | edge-case (duplicate entry) |
| Parse `hooks-registry.toml` `SessionStart` entry; inspect `epoch_budget_ms` field | `epoch_budget_ms = 8000` present; value is integer ≥ 8000 | happy-path (epoch budget) |
| `hooks-registry.toml` `SessionStart` entry missing `epoch_budget_ms` | Dispatcher applies 5000ms default; plugin invocations requiring subprocess timeout path (`EC-003` in BC-4.04.002) are killed before mapping timeout; integration test for timeout path fails | error (missing required field) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-065 | Session-Start Plugin Surface Invariant — All BC-4.04.* Postconditions Hold in Integration Test | integration |

## Related BCs

- **BC-4.04.004** — counterpart (hooks.json.template is the Claude Code-side registration; this is the dispatcher-side registration; both must exist for full routing)
- **BC-4.04.002** — depends on (exec_subprocess capability in this entry enables the factory-health subprocess call)
- **BC-4.04.003** — aligns with (once = true flag mirrors the idempotency contract)
- **BC-4.04.001** — enables (this routing entry causes the dispatcher to invoke the plugin that emits session.started)
- **BC-1.01.001** — depends on (registry schema validation applies to this toml entry)

## Architecture Anchors

- SS-04 — `plugins/vsdd-factory/hooks-registry.toml` (SessionStart routing entry; routing semantics
  for the session-start-telemetry WASM plugin are SS-04-owned even though the file lives in SS-07
  space; see F-8 ruling below)
- SS-07 — Hook Bash Layer owns `hooks-registry.toml` as the committed, operator-editable dispatcher
  routing table (per ARCH-INDEX.md line 98 and ADR-011). SS-07 contracts the file-as-text; SS-04
  contracts the routing semantics for WASM plugin entries within it. These scopes are non-overlapping.
- SS-09 — `scripts/generate-registry-from-hooks-json.sh` was the v0.79.x→v1.0 migration generator;
  it produced the initial `hooks-registry.toml` from the historical `hooks.json` bash-hook inventory.

**Architectural Notes:**

**F-8 Ruling (subsystem ownership, C1 accepted 2026-04-28):** This BC retains `subsystem: SS-04`
in its frontmatter. Rationale: BC-4.04.005 contracts the *routing semantics* of the SessionStart
WASM plugin entry — which plugin is invoked, with which capabilities, and under which
deduplication policy. SS-04 owns these routing semantics. SS-07 owns `hooks-registry.toml` as
a file-level artifact (schema validation, TOML format, registry loading). The two ownership
claims are at different abstraction layers and are mutually consistent. No renumbering required.

**F-13 Schema Requirement (epoch_budget_ms field, E1 accepted 2026-04-28, RESOLVED in pass-2):** The SessionStart
entry in `hooks-registry.toml` MUST include an `epoch_budget_ms` override greater than the
5000ms global default when the `exec_subprocess` capability is used for `factory-health`.
Rationale: the plugin's 5000ms subprocess timeout (BC-4.04.002 postcondition 1) + plugin
startup/parse overhead exceeds the 5000ms epoch budget (CAP-011), causing `Timeout{Epoch}`
before the plugin can map a subprocess timeout to `factory_health = "unknown"` (EC-003 becomes
unreachable). The required value is `epoch_budget_ms = 8000` (3000ms headroom above the
5000ms subprocess timeout). This field is now specified in Postcondition 5, Invariant 5, EC-004, and Canonical Test Vectors.

**F-17 Generator Workflow (verified 2026-04-28):** `hooks-registry.toml` is **generated-once,
then committed and directly edited** (option b). Evidence: (1) The file header reads "Human-edited
source of truth as of v1.0.0. The v0.79.x→v1.0 migration generator has been retired; edit this
file directly." (2) The generator script (`generate-registry-from-hooks-json.sh`) reads from the
historical git commit `7b4b774^` (v0.79.x hooks.json), not from the current file state — it is a
one-time migration tool, not a continuous code generator. (3) The SessionStart entry for the
session-start-telemetry plugin does NOT appear in the historical v0.79.x hooks.json (which only
covered bash hooks); therefore the generator would never emit it. Story S-5.01 Task 6 must instruct
the implementer to **directly add** the SessionStart `[[hooks]]` entry to `hooks-registry.toml`
rather than updating the generator script. The generator is retired and must not be modified for
this purpose.

## Story Anchor

S-5.01

## VP Anchors

VP-065

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-002 |
| Capability Anchor Justification | CAP-002 ("Hook Claude Code tool calls with sandboxed WASM plugins") per capabilities.md §CAP-002 |
| L2 Domain Invariants | DI-004 (capability denial emits audit event — missing exec_subprocess capability triggers internal.capability_denied); DI-014 (schema version mismatch = hard error — hooks-registry.toml schema_version = 1 must match dispatcher expectation); DI-015 (per-project activation required — this entry is directly added by human edit per the file header; SS-09 generator retired) |
| Architecture Module | SS-04 — `plugins/vsdd-factory/hooks-registry.toml` (SessionStart entry added by direct edit; SS-09 generator retired as of v1.0.0) |
| Stories | S-5.01 |
| Functional Requirement | FR-046 |
