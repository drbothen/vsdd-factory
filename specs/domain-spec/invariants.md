---
document_type: domain-spec-section
level: L2
section: invariants
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

**DI-008 — Internal log filenames are derived from event timestamps, not wall clock**
`dispatcher-internal-YYYY-MM-DD.jsonl` date is taken from `event.ts[..10]`. Test suites may inject past timestamps and get deterministic file names.
Enforcement owner: SS-03 (internal_log.rs::date_stamp). BC range: BC-3.
Justification: DI-008 is a business invariant because test reproducibility depends on timestamp-derived names, not process-time. Source: pass-2 §BR-Daily-rotation.

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

**DI-012 — A sink failure affects only that sink**
Failures are recorded in `Mutex<Vec<SinkFailure>>` per driver. Other sinks continue processing. S-4.4 will surface these as `internal.sink_error` events.
Enforcement owner: SS-03 (sinks/mod.rs). BC range: BC-3.
Justification: DI-012 is a business invariant because a single bad sink config must not eliminate observability for all other sinks. Source: pass-2 §BR-Sink-failure.

**DI-013 — Unknown sink driver types are non-fatal**
If `observability-config.toml` names an unrecognized driver type (forward-compatible for S-4.x), the dispatcher emits a warning to stderr and continues loading other sinks.
Enforcement owner: SS-03 (sinks::from_config warn-and-skip). BC range: BC-3.
Justification: DI-013 is a business invariant because operators upgrading configs ahead of binary versions must not lose all observability. Source: pass-2 §BR-Unknown-driver.

## Configuration Invariants

**DI-014 — Schema version mismatch is a hard load error**
`REGISTRY_SCHEMA_VERSION = 1`, `INTERNAL_EVENT_SCHEMA_VERSION = 1`, and `schema_version = 1` in observability config must match. Any mismatch emits `internal.dispatcher_error` and exits.
Enforcement owner: SS-01 (registry.rs::validate), SS-03 (sinks::from_config). BC range: BC-1, BC-3.
Justification: DI-014 is a business invariant because silently processing a mismatched schema would produce incorrect behavior with no error signal. Source: pass-2 §BR-Schema-version.

**DI-015 — Per-project activation is required before the dispatcher can run**
`hooks.json` is gitignored. Without activation, no `hooks.json` exists, so Claude Code cannot invoke the dispatcher. Activation is the gate — not install.
Enforcement owner: SS-09 (activate skill). BC range: BC-9.
Justification: DI-015 is a business invariant because the binary path is platform-specific; shipping a pre-written hooks.json would point to the wrong binary on most machines. Source: pass-2 §BR-Activation.

## Plugin Isolation Invariants

**DI-016 — Each registry entry sees only its own `plugin_config`**
The executor splices the entry's `config` block as `plugin_config` into the payload clone for that entry. The value from a different registry entry is never visible.
Enforcement owner: SS-01 (executor.rs). BC range: BC-1.
Justification: DI-016 is a business invariant because configuration leakage between plugin instances (e.g., two `legacy-bash-adapter` entries with different `script_path`) would cause incorrect behavior. Source: pass-2 §BR-per-plugin-config.

**DI-017 — `dispatcher_trace_id` is present on every emitted event**
Every `InternalEvent` carries the UUID v4 generated from the stdin envelope. No event is emitted without it, enabling full causal reconstruction of a single hook invocation.
Enforcement owner: SS-01 (main.rs, executor.rs, emit_event host fn). BC range: BC-1.
Justification: DI-017 is a business invariant because the trace ID is the audit correlation key — an event without it cannot be attributed to its invoking tool call. Source: pass-2 §BR-trace_id.

### DI-018 — Hook Self-Modification Deferral

**Statement:** Modifications to hook-routing artifacts (hooks-registry.toml, hook script files, or hooks.json variants) made during an active dispatcher invocation MUST be deferred to the next dispatcher invocation. The currently-loaded hook routing table and script paths remain in effect until the dispatcher process exits and re-reads its inputs.

**Rationale:** vsdd-factory is its own product. Phase 3 TDD edits hook scripts. The dispatcher fires on tool calls during agent work. Without this invariant, a story modifying `hooks-registry.toml` mid-burst could observe inconsistent routing.

**Enforcement:** PluginCache mtime-based invalidation (BC-1.09.NNN) applies at plugin-load time. Registry changes between invocations are picked up on next dispatcher start.

**Verification:** VP-NEW-018 (manual; could be promoted to integration test that edits a hook file mid-dispatch and verifies the in-flight dispatcher uses the old script_path).

**Source:** Phase 1d adversary pass 1 finding F-014.
