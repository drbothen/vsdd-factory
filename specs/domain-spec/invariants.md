---
document_type: domain-spec-section
level: L2
section: invariants
version: "1.5"
status: accepted
producer: business-analyst
timestamp: 2026-04-25T00:00:00
last_amended: 2026-05-07
phase: 1.3
inputs:
  - .factory/phase-0-ingestion/pass-2-domain-model.md
  - .factory/phase-0-ingestion/pass-8-final-synthesis.md
input-hash: "a6c6f62"
traces_to: L2-INDEX.md
---

# Domain Invariants

> **Sharded L2 section (DF-021).** Navigate via `L2-INDEX.md`.
> Source: pass-2-domain-model.md §2b Business rules (17 invariants).
> Each DI-NNN is a business invariant — a domain rule that must always hold,
> independent of implementation. Each traces to its BR-NN rule in business-rules.md
> and the SS-NN subsystem that enforces it.

## Execution Invariants

**DI-001 — Tiers execute sequentially; plugins within a tier execute in parallel**
A tier N+1 may not begin until all plugins in tier N have produced a result (Ok, Timeout, or Crashed). Within a tier, plugins have no ordering guarantee.
Enforcement owner: SS-01 (executor.rs). BC range: BC-1.
Justification: DI-001 is a business invariant because priority ordering is the domain's mechanism for expressing dependency — a pre-check plugin (priority 100) must complete before an execution plugin (priority 500). Source: pass-2 §BR-Parallel.

**DI-002 — A plugin crash or timeout does not block sibling plugins**
The failure of any single plugin produces a `PluginResult::Crashed` or `PluginResult::Timeout` for that entry; sibling entries in the same tier continue to completion.
Enforcement owner: SS-01 (executor.rs, tokio panic isolation). BC range: BC-1.
Justification: DI-002 is a business invariant because the dispatcher must never let one misbehaving plugin deny service to other hooks. Source: pass-2 §BR-Plugin-failure.

**DI-003 — `block_intent` is an aggregate; the tier runs to completion regardless**
Recording `block_intent = true` does not abort the current tier mid-flight. All plugins in the tier complete; the exit code 2 is emitted only after the tier's last plugin resolves.
Enforcement owner: SS-01 (executor.rs::plugin_requests_block). BC range: BC-1.
Justification: DI-003 is a business invariant because partial-tier execution would leave audit events missing for hooks that never got to run. Source: pass-2 §BR-on_error.

## Security Invariants

**DI-004 — Capability denial always produces both a return code AND an audit event**
Every capability-gated host function (exec_subprocess, read_file, env) atomically returns `codes::CAPABILITY_DENIED (-1)` to the WASM caller AND emits `internal.capability_denied` to the internal log. One without the other is a bug.
Enforcement owner: SS-01 (host/*.rs). BC range: BC-1.
Justification: DI-004 is a business invariant because the audit trail is the security property; the return code alone is insufficient for compliance. Source: pass-2 §BR-Capability-denial.

**DI-005 — Shell interpreters require explicit `shell_bypass_acknowledged`**
The set {bash, sh, zsh, pwsh, fish, csh, tcsh, ksh} is refused by exec_subprocess unless the capability entry provides a non-empty `shell_bypass_acknowledged` string. No default value bypasses this.
Enforcement owner: SS-01 (host/exec_subprocess.rs::is_shell). BC range: BC-1.
Justification: DI-005 is a business invariant because allowing arbitrary shell invocation without acknowledgment negates the WASM sandbox. Source: pass-2 §BR-shell_bypass.

**DI-006 — Setuid/setgid binaries are refused unconditionally on Unix**
No capability configuration permits execution of a binary with setuid or setgid bits. This refusal is not configurable.
Enforcement owner: SS-01 (host/exec_subprocess.rs::refuse_setuid). BC range: BC-1.
Justification: DI-006 is a business invariant because privilege escalation via WASM plugin is a categorical security violation. Source: pass-2 §BR-setuid.

## Observability Invariants

**DI-007 — Dispatcher self-telemetry is always-on**
`dispatcher-internal-YYYY-MM-DD.jsonl` is written for every invocation, independent of `observability-config.toml`. No configuration disables it.
Enforcement owner: SS-03 (internal_log.rs). BC range: BC-3.
Justification: DI-007 is a business invariant because the self-telemetry is the fallback debugging path when all user-configured sinks fail. Source: pass-2 §BR-Always-on.
**Amended by:** ADR-015 D-15.1 — the debug stream (dispatcher-internal-*.jsonl) is opt-in via `VSDD_DEBUG_LOG=1` env var or `debug_log_enabled = true` in `observability-config.toml`; env var dominates when present (12-factor override semantics, D-311). The amended invariant is: "the debug stream is always-on **when opted in**." When not opted in, only `events-*.jsonl` is written. See BC-1.12.002 for the enforcing behavioral contract.

**DI-008 — Internal log filenames are derived from event timestamps, not wall clock**
`dispatcher-internal-YYYY-MM-DD.jsonl` date is taken from `event.ts[..10]`. Test suites may inject past timestamps and get deterministic file names.
Enforcement owner: SS-03 (internal_log.rs::date_stamp). BC range: BC-3.
Justification: DI-008 is a business invariant because test reproducibility depends on timestamp-derived names, not process-time. Source: pass-2 §BR-Daily-rotation.
**Reaffirmed by:** ADR-015 D-15.1 — the debug filename pattern `dispatcher-internal-YYYY-MM-DD.jsonl` is unchanged; timestamp-derived naming applies equally to the opt-in debug stream.

**DI-009 — Internal logs are pruned to 30 days at dispatcher start**
`prune_old(30)` executes during dispatcher startup. Operators cannot extend retention without modifying the dispatcher (a `DEFAULT_RETENTION_DAYS` constant, not configurable via TOML).
Enforcement owner: SS-03 (internal_log.rs). BC range: BC-3.
Justification: DI-009 is a business invariant because unbounded log growth is a reliability risk on developer machines. Source: pass-2 §InternalLog.

**DI-010 — Plugin stderr is capped at 4 KiB with truncation marker**
`STDERR_CAP_BYTES = 4096`. If a plugin writes more, the captured string ends with `(stderr truncated)`. This cap is not configurable per registry entry.
Enforcement owner: SS-01 (invoke.rs). BC range: BC-1.
Justification: DI-010 is a business invariant because unbounded stderr capture would allow a plugin to exhaust dispatcher memory. Source: pass-2 §PluginResult.

## Sink Invariants

**DI-011 — Sink `submit` must not block the dispatcher**
Sink drivers use bounded mpsc + `try_send`. A full queue drops the event (recorded as SinkFailure) rather than blocking. This preserves dispatcher latency for the user's tool call.
Enforcement owner: SS-03 (sink-core Sink trait). BC range: BC-3.
Justification: DI-011 is a business invariant because sink slowness must not degrade the user's Claude Code tool call response time. Source: pass-2 §BR-Sink-submit.
**Status:** SUPERSEDED by ADR-015 D-15.1 — the single-sink FileSink design eliminates the mpsc+try_send indirection; FileSink::write is synchronous-by-design under the BC-1.11.002 partial-write contract. The non-blocking guarantee is now satisfied structurally (no queue) rather than via the Sink trait submit abstraction.

**DI-012 — A sink failure affects only that sink**
Failures are recorded in `Mutex<Vec<SinkFailure>>` per driver. Other sinks continue processing. S-4.4 will surface these as `internal.sink_error` events.
Enforcement owner: SS-03 (sinks/mod.rs). BC range: BC-3.
Justification: DI-012 is a business invariant because a single bad sink config must not eliminate observability for all other sinks. Source: pass-2 §BR-Sink-failure.
**Status:** SUPERSEDED by ADR-015 D-15.1 — single-sink design; per-sink failure isolation is moot when there is only one sink. FileSink write failures are governed by the BC-1.11.002 partial-write recovery contract.

**DI-013 — Unknown sink driver types are non-fatal**
If `observability-config.toml` names an unrecognized driver type (forward-compatible for S-4.x), the dispatcher emits a warning to stderr and continues loading other sinks.
Enforcement owner: SS-03 (sinks::from_config warn-and-skip). BC range: BC-3.
Justification: DI-013 is a business invariant because operators upgrading configs ahead of binary versions must not lose all observability. Source: pass-2 §BR-Unknown-driver.
**Refined by:** ADR-015 D-15.1 (single-stream + multi-sink retirement decision) — the warn-and-skip behavior for unknown `observability-config.toml` v2 keys is specified in **BC-3.05.004 Postcondition 7** (the v2 schema validation contract that resolves OQ-1 in `SS-03-event-emission.md`). The forward-compatibility spirit is preserved: unknown keys in the v2 schema are non-fatal warnings, not hard errors (hard errors apply only to schema_version mismatch per DI-014).

## Configuration Invariants

**DI-014 — Schema version mismatch is a hard load error**
`REGISTRY_SCHEMA_VERSION = 2` (post-ADR-019; was 1 pre-2026-05-07), `INTERNAL_EVENT_SCHEMA_VERSION = 1`, and `schema_version = 2` in hooks-registry.toml (and `schema_version = 2` in observability config) must match their respective expected versions. Any mismatch emits `internal.dispatcher_error` (registry mismatch: `dispatcher.schema_mismatch`; observability mismatch: `internal.dispatcher_error`) and exits non-zero.
Enforcement owner: SS-01 (registry.rs::validate), SS-03 (sinks::from_config). BC range: BC-1, BC-3, BC-7.
Justification: DI-014 is a business invariant because silently processing a mismatched schema would produce incorrect behavior with no error signal. Source: pass-2 §BR-Schema-version.
**Updated per ADR-015 (D-314):** `observability-config.toml` schema_version=2 is the ADR-015 target format. BC-3.05.004 Postcondition 4 hard-errors on schema_version=1 (old format no longer accepted post-migration) and accepts schema_version=2. The invariant extends to v2: any schema_version value other than 2 in a post-Wave-2 deployment emits `internal.dispatcher_error` and exits. DI-014 remains active and its spirit (hard error on mismatch, never silent) is preserved.
**Updated per ADR-019 (F2 2026-05-07):** `REGISTRY_SCHEMA_VERSION` in `registry.rs` bumped from 1 to 2. The BC-1 enforcement arm (BC-1.14.001) and BC-7 enforcement arm (BC-7.06.001) both cite DI-014. Schema-version mismatch for `hooks-registry.toml` is explicitly fail-closed (exit code 2), not fail-open (exit 0) — this is the named exception to BC-1.08.001 fail-open policy. The BC range is: BC-1, BC-3 (and BC-7.06.001 by ID-prefix retention per POLICY 1; authoritative subsystem is SS-01 per F-P1-006 reanchor — the ID prefix "BC-7" is preserved for append-only continuity, not as a subsystem designation).
**Amendment note (F2 pass-2 2026-05-07, F-P2-014):** "BC-7" in the BC range was misleading because BC-7.06.001's frontmatter `subsystem` was reanchored to SS-01 during pass-1. The BC range now explicitly notes this ID-prefix retention to avoid implying the BC is subsystem-SS-07-owned.

**DI-015 — Per-project activation is required before the dispatcher can run**
`hooks.json` is gitignored. Without activation, no `hooks.json` exists, so Claude Code cannot invoke the dispatcher. Activation is the gate — not install.
Enforcement owner: SS-09 (activate skill). BC range: BC-9.01.004 (CI matrix), BC-9.01.005 (hooks.json gitignore — gate artifact).
Justification: DI-015 is a business invariant because the binary path is platform-specific; shipping a pre-written hooks.json would point to the wrong binary on most machines. Source: pass-2 §BR-Activation.

## Plugin Isolation Invariants

**DI-016 — Each registry entry sees only its own `plugin_config`**
The executor splices the entry's `config` block as `plugin_config` into the payload clone for that entry. The value from a different registry entry is never visible.
Enforcement owner: SS-01 (executor.rs). BC range: BC-1.
Justification: DI-016 is a business invariant because configuration leakage between plugin instances (e.g., two `legacy-bash-adapter` entries with different `script_path`) would cause incorrect behavior. Source: pass-2 §BR-per-plugin-config.

**DI-017 — `trace_id` is present on every emitted event**
Every `InternalEvent` carries the UUID v4 generated from the stdin envelope. No event is emitted without it, enabling full causal reconstruction of a single hook invocation.
Enforcement owner: SS-01 (main.rs, executor.rs, emit_event host fn). BC range: BC-1.
Justification: DI-017 is a business invariant because the trace ID is the audit correlation key — an event without it cannot be attributed to its invoking tool call. Source: pass-2 §BR-trace_id.
**Renamed by ADR-015 v1.7 changelog:** `dispatcher_trace_id` → `trace_id` (canonicalized in D-15.2.e). The invariant is identical; only the field name changed. All BCs and code must use `trace_id`. Any reference to `dispatcher_trace_id` in existing code or specs is a drift artifact to be corrected.

> **Note:** DI-018 was proposed in Phase 1d pass-1 to address the concurrent self-modification risk for vsdd-factory's dogfooding (engine and product are the same repo). Pass-2 review (F-018, F-021) flagged that the proposed enforcement claim was aspirational without an actual enforcing BC. The risk is now captured as a known limitation (KL-005) rather than a domain invariant.
>
> See `.factory/specs/prd.md` §10.4 KL-005 and §11 DRIFT-011 for the current treatment.

## Dispatcher Timing Invariants

**DI-019 — ASYNC_DRAIN_WINDOW_MS = 100 (milliseconds, runtime constant)**
After `sync_group` plugin execution completes, the dispatcher waits up to `ASYNC_DRAIN_WINDOW_MS` milliseconds for spawned async-group tasks to emit terminal events to FileSink before forcibly terminating them and exiting. The constant bounds the dispatcher's user-facing latency tail and ensures bounded-but-reliable async telemetry emission.

**Statement:** `ASYNC_DRAIN_WINDOW_MS = 100` ms. This is the canonical default value. The total dispatcher wall-clock latency upper bound is therefore: `max(sync_plugin_durations_within_slowest_tier) + ASYNC_DRAIN_WINDOW_MS + bounded_overhead`.

**Scope:** Applies to all dispatcher invocations across all hook event types (PreToolUse, PostToolUse, Stop, SubagentStop, SessionStart, SessionEnd, WorktreeCreate, WorktreeRemove, PostToolUseFailure, PermissionRequest). Not configurable per event; applies uniformly.

**Configurability:** Future may permit env-var override (e.g., `VSDD_ASYNC_DRAIN_WINDOW_MS`); this is a deferred decision. The value 100 ms is the canonical baseline and the only value with specified behavior.

**Rationale:** 100 ms is long enough for in-flight tokio tasks to complete a sub-millisecond FileSink append; short enough to be imperceptible to a human user after `sync_group` finishes. Async plugins requiring longer drain (e.g., network I/O) must be redesigned — the drain is for terminal-event flush only, not for completing arbitrary async work.

**Enforcement owner:** SS-01 (dispatcher runtime — `crates/factory-dispatcher/src/engine.rs`).
**BC range:** BC-1.14.001 (PC4 partition contract — async-task drain window), BC-3.08.001 (event-type catalog; events that depend on the drain window being open to emit cleanly).
**Cited by:** BC-1.14.001 (Traceability L2 Domain Invariants — DI-019), BC-3.08.001 (Traceability L2 Domain Invariants — DI-019), VP-079 (event-emission verification; timing assertions must account for 100 ms drain), ADR-019 (latency-budget rationale; drain window as part of the async-semantics design).

Justification: DI-019 is a domain invariant because the drain-window constant directly bounds user-facing latency for every Claude Code tool call that triggers the dispatcher. Inlining the constant in a single BC file (BC-1.14.001) would make it invisible to sibling BCs (e.g., BC-3.08.001) and VPs (VP-079) that depend on its value for fixture timing. Lifting it to a domain invariant makes the constraint enforceable across all dispatcher subsystems.

## CHANGELOG

| Version | Date | Change |
|---------|------|--------|
| v1.0 | 2026-04-25 | Initial authoring from domain spec crystallization (Phase 1.3). 17 invariants (DI-001–DI-017). |
| v1.1 | 2026-05-06 | D-314 F-4 fix: DI-007/008/011/012/013/014/017 amended/refined/superseded per ADR-015. DI-007 amended (debug stream is opt-in). DI-008 reaffirmed (filename pattern unchanged). DI-011 superseded (single-sink eliminates mpsc+try_send). DI-012 superseded (single-sink; per-sink isolation moot). DI-013 refined (warn-and-skip extended to v2 unknown keys per BC-3.05.004). DI-014 updated (schema_version=2 target; hard error on mismatch preserved). DI-017 renamed dispatcher_trace_id → trace_id per ADR-015 v1.7 canonicalization. BC-side L2 citation work (adding DI references to BC-1.12.002/003/004 and BC-3.05.004) deferred to D-315 (PO). |
| v1.2 | 2026-05-07 | F2 pass-1 fix burst: DI-014 amended — `REGISTRY_SCHEMA_VERSION` updated from 1 to 2 (post-ADR-019); BC range extended to include BC-7 (BC-7.06.001 is the BC-7 enforcement arm). DI-014 prose now explicitly notes the fail-closed (exit 2) exception to BC-1.08.001 fail-open for registry schema mismatch. |
| v1.3 | 2026-05-07 | F2 pass-2 fix burst: DI-014 amendment note added per F-P2-014 — BC-7.06.001 ID-prefix retention clarification (BC-7 prefix preserved for append-only continuity; subsystem is SS-01 post-reanchor). DI-015 added — per-project activation gate invariant. |
| v1.4 | 2026-05-07 | F2 pass-3 fix burst (BC-1.14.001 v1.3): BC-1.14.001 v1.3 inlined `ASYNC_DRAIN_WINDOW_MS = 100` as a Constant Definitions table. State after pass-3 fix burst; constant was in BC, not yet lifted to DI. |
| v1.5 | 2026-05-07 | F2 pass-3 user-correction: DI-019 authored — `ASYNC_DRAIN_WINDOW_MS = 100 ms` lifted from BC-1.14.001 Constant Definitions table to a domain invariant. BC-1.14.001 v1.3 → v1.4 refactored to cite DI-019 by reference; constant value removed from BC inline definition. BC-3.08.001 v1.1 → v1.2 updated to cite DI-019 in L2 Domain Invariants. New section "Dispatcher Timing Invariants" added. |

## Amendment 2026-05-07 (v1.4 → v1.5 — F2 pass-3 user-correction)

**Structural correction requested by user after reviewing BC-1.14.001 v1.3.**

BC-1.14.001 v1.3 (authored in the pass-3 fix burst to resolve adversary findings F-P3-002 and F-P3-007) placed `ASYNC_DRAIN_WINDOW_MS = 100` as an inline constant table inside the BC file. The user identified this placement as architecturally incorrect: constants that bound cross-cutting dispatcher behavior belong in the domain invariants, not inside a single BC's body.

**Changes in this amendment:**

1. **DI-019 authored** (`ASYNC_DRAIN_WINDOW_MS = 100 ms`) — new invariant in the "Dispatcher Timing Invariants" section above. This is the canonical single source of truth for the constant value.
2. **BC-1.14.001 v1.3 → v1.4** — the "Constant Definitions" section's inline `= 100` value was removed; replaced with a reference to DI-019. PC4 prose now cites DI-019 as canonical with the value in parentheses as a reading aid. DI-019 added to Traceability L2 Domain Invariants alongside DI-014.
3. **BC-3.08.001 v1.1 → v1.2** — DI-019 added to Traceability L2 Domain Invariants. The async-path event types (`plugin.timeout`, `plugin.async_block_discarded`) are emitted during the drain window governed by DI-019.

**This is a structural correction, not a semantic change.** The value (100 ms), the drain-window mechanism, and all behavioral postconditions are unchanged. Only the placement of the canonical constant definition changed (BC → DI).

**Cross-burst dependencies (architect):**
- VP-079 must update its timing-assertion anchor to reference DI-019 (not the now-removed BC-1.14.001 constant table).
- ADR-019 §Consequences should cite DI-019 as the latency-budget invariant for the drain window.
