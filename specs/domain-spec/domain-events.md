---
document_type: domain-spec-section
level: L2
section: domain-events
version: "1.0"
status: accepted
producer: business-analyst
timestamp: 2026-04-25T00:00:00
phase: 1.3
inputs:
  - .factory/phase-0-ingestion/pass-2-domain-model.md
  - .factory/phase-0-ingestion/pass-8-final-synthesis.md
input-hash: "08db1f1"
traces_to: L2-INDEX.md
---

# Domain Events

> **Sharded L2 section (DF-021).** Navigate via `L2-INDEX.md`.
> Source: pass-2-domain-model.md §Domain events catalog (22 event types).
> Events use dot-namespaced lowercase naming. All carry `dispatcher_trace_id`
> when emitted through the dispatcher. "Declared" = constant exists in
> `internal_log.rs` but not yet emitted (DRIFT-007/008/006).

## Dispatcher Lifecycle Events (SS-01)

| DE-ID | Event type | Producer | Consumers | Status | Key fields |
|-------|-----------|----------|-----------|--------|------------|
| DE-001 | `dispatcher.started` | SS-01 `main.rs` | SS-03 (sinks), operators | Shipped | `dispatcher_version`, `host_abi_version`, `platform`, `pid`, `registry_path`, `loaded_plugin_count` |
| DE-002 | `dispatcher.shutting_down` | SS-01 `main.rs` | SS-03 (sinks) | Declared, not emitted (DRIFT-007) | reserved |

## Plugin Lifecycle Events (SS-01)

| DE-ID | Event type | Producer | Consumers | Status | Key fields |
|-------|-----------|----------|-----------|--------|------------|
| DE-003 | `plugin.loaded` | SS-01 `plugin_loader` | SS-03 | Declared, not emitted (DRIFT-008) | `wasm_path`, `plugin_name` |
| DE-004 | `plugin.load_failed` | SS-01 `plugin_loader` | SS-03 | Declared, not emitted (DRIFT-008) | `wasm_path`, `error` |
| DE-005 | `plugin.invoked` | SS-01 `executor::emit_invoked` | SS-03 | Shipped | `event`, `plugin_name`, `plugin_version` |
| DE-006 | `plugin.completed` | SS-01 `executor::emit_lifecycle` | SS-03 | Shipped | `exit_code`, `elapsed_ms`, `fuel_consumed`, `stderr` (if non-empty) |
| DE-007 | `plugin.timeout` | SS-01 `executor::emit_lifecycle` | SS-03 | Shipped | `cause` (epoch\|fuel), `elapsed_ms`, `fuel_consumed`, `stderr` |
| DE-008 | `plugin.crashed` | SS-01 `executor::emit_lifecycle` | SS-03 | Shipped | `trap`, `elapsed_ms`, `fuel_consumed`, `stderr` |
| DE-009 | `plugin.log` | SS-01 `host::log` + `invoke.rs::log` shim | SS-03, operator consoles | Shipped | `level` (trace\|debug\|info\|warn\|error), `message` |

## Internal Audit Events (SS-01)

| DE-ID | Event type | Producer | Consumers | Status | Key fields |
|-------|-----------|----------|-----------|--------|------------|
| DE-010 | `internal.capability_denied` | SS-01 `host/*` on cap denial | SS-03, security audit | Shipped | `function`, `reason`, `command`\|`variable`\|`path` (per cap type) |
| DE-011 | `internal.host_function_panic` | SS-01 `host` | SS-03 | Declared | `function`, `message` |
| DE-012 | `internal.dispatcher_error` | SS-01 `main.rs::emit_dispatcher_error` | SS-03 | Shipped | `message` |

## Sink Health Events (SS-03, Planned for S-4.4)

| DE-ID | Event type | Producer | Consumers | Status | Key fields |
|-------|-----------|----------|-----------|--------|------------|
| DE-013 | `internal.sink_error` | SS-03 sinks (S-4.4 pending) | operator alerts | Not shipped (DRIFT-002) | `sink_name`, `reason` |
| DE-014 | `internal.sink_queue_full` | SS-03 sinks (S-4.4 pending) | operator alerts | Not shipped (DRIFT-002) | `sink_name`, `dropped_event_type` |
| DE-015 | `internal.sink_circuit_opened` | SS-03 sinks (S-4.4 pending) | operator alerts | Not shipped (DRIFT-002) | `sink_name`, `failure_count` |
| DE-016 | `internal.sink_circuit_closed` | SS-03 sinks (S-4.4 pending) | operator alerts | Not shipped (DRIFT-002) | `sink_name`, `recovery_ts` |

## Bash Hook Activity Events (SS-07, via `bin/emit-event`)

| DE-ID | Event type | Producer | Consumers | Status | Key fields |
|-------|-----------|----------|-----------|--------|------------|
| DE-017 | `commit.made` | SS-07 `capture-commit-activity.sh` (PostToolUse:Bash) | SS-03, audit | Shipped | `commit_sha`, `branch`, `message` |
| DE-018 | `pr.merged` | SS-07 `capture-pr-activity.sh` | SS-03, audit | Shipped | `pr_number`, `repo` |
| DE-019 | `hook.block` | SS-07 `block-*.sh` / `protect-*.sh` (PreToolUse) | SS-03, operator | Shipped | `hook`, `matcher`, `reason`, `command` |
| DE-020 | `tool.error` | SS-07 (declared in design Phase 5) | SS-03 | Not wired (DRIFT-006) | `tool`, `exit_code`, `error` |

## Session and Worktree Events (SS-01/SS-07, Tier G, S-5.1–5.3)

| DE-ID | Event type | Producer | Consumers | Status | Key fields |
|-------|-----------|----------|-----------|--------|------------|
| DE-021 | `session.started` | SS-01/SS-07 (S-5.1 pending) | SS-03, session-reviewer | Not wired (DRIFT-006) | `session_id`, `agent`, `dependency_checks` |
| DE-022 | `session.ended` | SS-01/SS-07 (S-5.2 pending) | SS-03 | Not wired (DRIFT-006) | `duration`, `cost` |

Note: `worktree.removed` (S-5.3) is declared in design Phase 5 but not yet assigned a DE-NNN — it would be DE-023 when wired.

## Event Emission Paths

Two distinct emission paths exist today (see DRIFT in pass-8 §L-P0-004):

1. **Native WASM path**: Plugin calls `vsdd::emit_event(type, fields)` host fn → dispatcher event queue → `InternalLog::write` + `SinkRegistry::submit_all`. Used by: any WASM plugin.
2. **Bash shell path**: Hook script calls `${CLAUDE_PLUGIN_ROOT}/bin/emit-event type key=val ...` → shell tool constructs JSON → writes to `.factory/logs/`. Used by: all 44 current bash hooks.

The native path is correctly typed and carries `dispatcher_trace_id`. The bash path does not invoke the dispatcher's event normalization pipeline. S-3.4 (PARTIAL) aims to close this gap.

## Schema Note

All events carry `InternalEvent` base fields: `type_`, `ts` (ISO-8601), `ts_epoch` (i64), `schema_version = 1`. Additional fields are flattened into the `fields` map. Reserved field names (`type`, `ts`, `ts_epoch`, `schema_version`) are filtered out if a plugin attempts to overwrite them via `emit_event`.
