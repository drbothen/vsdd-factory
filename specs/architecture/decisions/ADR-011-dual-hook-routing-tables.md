---
document_type: adr
adr_id: ADR-011
status: accepted
date: 2026-04-26
subsystems_affected: [SS-07, SS-09]
supersedes: null
superseded_by: null
---

# ADR-011: Dual hooks.json + hooks-registry.toml During Migration

## Context

v1.0 introduced a clear separation between two routing concerns that were previously
conflated in a single `hooks.json` file:

1. **Claude Code harness wiring** — the JSON file Claude Code reads to know which
   executable to invoke on which hook event. This must be in the format Claude Code
   expects: a JSON object with `hooks` keys mapping event types to arrays of hook
   entries, each with a `command` field pointing to an executable.

2. **Dispatcher internal routing** — the TOML file the compiled dispatcher reads at
   runtime to know which WASM plugins to load, which events/tools they match, their
   priority tiers, timeout overrides, capability grants, and error policies.

In v0.79.x, `hooks.json` served both purposes simultaneously: it was both the
harness wiring document and the implicit routing table. Each entry in `hooks.json`
invoked a distinct bash script, and the script path was the entire routing
specification. With v1.0's single dispatcher binary as the sole `hooks.json`
entry point, a new routing layer was needed. The question was how to manage this
during the transition period when both the old behavior and the new must coexist.

DRIFT-004 in the project STATE tracks this dual-table situation as a known,
intentional architectural state. The DRIFT note ("hooks.json + hooks-registry.toml
dual routing tables | MEDIUM-HIGH | cutover before rc.1") records the planned
resolution point rather than an unintended divergence.

## Decision

During the v1.0 migration period (beta releases through rc.1), two routing tables
coexist with distinct, non-overlapping purposes:

- `plugins/vsdd-factory/hooks/hooks.json` (and its `.platform` variants) — Claude
  Code harness wiring only. Contains a single entry pointing to the dispatcher
  binary. Never used by the dispatcher itself. Written by the activation skill
  (ADR-009); gitignored; source of truth is `hooks.json.template`.

- `plugins/vsdd-factory/hooks-registry.toml` — dispatcher routing only. Declares
  every WASM plugin, its event/tool match patterns, priority, timeout, capabilities,
  and error policy. Never read by the Claude Code harness. Committed and operator-editable.

The cutover from dual-table to single-table is planned for rc.1 (DRIFT-004 disposition
"L-P0-002 cutover before rc.1") when the dispatcher assumes full ownership of routing
and `hooks.json` becomes a pure harness wiring artifact with no routing semantics.

## Rationale

The two files have fundamentally different audiences and update cadences:

`hooks.json` is written once at activation time by a skill, not by humans. Its format
is constrained by what Claude Code accepts. Operators never edit it directly; the skill
regenerates it on each activation. It is platform-specific (per ADR-009) and gitignored.

`hooks-registry.toml` is the operator's configuration surface. Operators edit it to
add plugins, adjust priorities, grant capabilities, or change error policies. It is
platform-agnostic (the same TOML works on all platforms because it references
`.wasm` files, not platform binaries). It is committed to the repository.

Merging these into a single file would require either: (a) Claude Code learning to
parse TOML (not feasible), or (b) the dispatcher parsing JSON with TOML semantics
embedded (wrong format for operator authoring — see ADR-004), or (c) maintaining a
dual-format file that serves neither audience well.

The dual-table approach during migration is the correct minimal footprint for the
beta period. It makes the boundaries explicit and allows each file to evolve
independently. The DRIFT-004 tracking ensures the architectural debt is visible and
has a planned resolution, not silently accumulated.

The distinction is confirmed by the design: `hooks.json` is a harness wiring file
(analogous to a systemd unit file pointing to a binary), while `hooks-registry.toml`
is a routing configuration file (analogous to the application's internal config).
These are different layers with different change surfaces.

## Consequences

### Positive

- `hooks.json` format remains exactly what Claude Code expects; no harness
  compatibility risks are introduced by routing changes in `hooks-registry.toml`.
- Operators can add, remove, or reconfigure WASM plugins by editing
  `hooks-registry.toml` alone, with no interaction with the harness-wiring layer.
- The separation makes the migration path to rc.1 safe: the cutover affects only
  how `hooks.json` is generated, not the dispatcher's routing logic.

### Negative / Trade-offs

- Two files with overlapping conceptual territory create operator confusion during
  the beta period. Documentation must clearly explain which file controls what.
- DRIFT-004 carries MEDIUM-HIGH severity because the dual-table state is temporary
  debt that must be resolved before rc.1. If not resolved, future operators may
  attempt to add routing logic to `hooks.json`, breaking the architectural boundary.
- The gitignored status of `hooks.json` means CI cannot lint it directly; the
  `.platform` variants are the only committed artifacts representing the harness
  wiring layer.

### Status as of v1.0.0-beta.5

IN-EFFECT (migration period). Both files are present and serving their distinct
purposes. DRIFT-004 is open with planned resolution at rc.1 (L-P0-002). The
`hooks.json.template` and `.platform` variants are committed; `hooks.json` is
gitignored. `hooks-registry.toml` is committed and routed through by the dispatcher.

## Alternatives Considered

- **Single TOML routing file that also serves as hooks.json:** Requires Claude Code
  to parse TOML. Claude Code does not support TOML for `hooks.json`. Rejected.
- **Single JSON file with dispatcher routing embedded:** Embeds TOML-style registry
  declarations in JSON. Poor operator authoring experience; JSON lacks comment support
  (see ADR-004). Rejected.
- **Migrate to single file immediately at beta.1:** Merge routing into `hooks.json`
  at the start of v1.0. Rejected: requires stabilizing the routing format before
  the dispatcher has been dogfooded; beta period is explicitly for discovering
  format issues.

## Downstream Impact: BC-4.04.004 vs BC-4.04.005 Layer Split

The dual-table architecture creates a two-layer registration requirement for every new lifecycle
plugin (first surfaced in the S-5.01 session-start-telemetry work):

**Layer 1 — hooks.json.template (harness wiring, SS-07-owned file):**
The `SessionStart` entry in `hooks.json.template` routes to the **dispatcher binary**, not to a
WASM plugin filename. The canonical shape is an array-of-objects with a nested `hooks` array
containing a `command` field pointing to the platform-specific dispatcher binary, plus `async: true`
and `once: true` at the outer hook level. BC-4.04.004 contracts this layer. VP-065 harness
assertions for hooks.json.template must assert against this structure (not against a WASM plugin
filename).

**Layer 2 — hooks-registry.toml (dispatcher routing, SS-07-owned file, SS-04-contracted semantics):**
The `SessionStart` entry in `hooks-registry.toml` routes to the WASM plugin
(e.g., `session-start-telemetry.wasm`), with capability declarations (`read_file` +
`exec_subprocess`) and any `timeout_ms` override needed. The entry does NOT carry a `once`
field — `RegistryEntry` has no such field (`deny_unknown_fields` would reject it). Once-discipline
is a Layer 1 concern enforced by `hooks.json.template` (see "Dedup is enforced at Layer 1"
subsection below). BC-4.04.005 contracts this layer. The routing semantics for
this entry are SS-04-owned even though the file lives in SS-07-owned space (see F-8 ruling in
the S-5.01 pass-2 architectural-findings record: Option C1 accepted).

**Rule:** Any BC that contracts a hooks.json.template entry must NOT reference WASM plugin
filenames — that is exclusively BC-4.04.005 / hooks-registry.toml territory. Any BC that
contracts a hooks-registry.toml entry must NOT assert dispatcher binary paths — that is
exclusively BC-4.04.004 / hooks.json.template territory.

## Pass-4 Architectural Simplifications (2026-04-28)

### Dedup is enforced at Layer 1 via `once: true` directive

The dispatcher does NOT enforce per-event dedup at Layer 2. Once-per-session firing
discipline is a Layer 1 concern: Claude Code's `once: true` directive in
`hooks.json.template` (BC-4.04.004 invariant) instructs the Claude Code harness to fire
the `SessionStart` event to the dispatcher exactly once per session. Because the harness
enforces this upstream, the dispatcher receives at most one `SessionStart` invocation per
session — making dispatcher-side dedup redundant.

Pass-4 retired BC-1.10.002 ("Dispatcher suppresses duplicate once:true events by tracking
per-event-name + per-session_id in dispatcher memory") as over-engineering. BC-4.04.003
is revised to reflect that idempotency is delegated to Layer 1; the plugin itself is
unconditionally stateless.

### Plugin file reads use existing `read_file` host fn

Plugins that need to read sandboxed files (e.g., `.claude/settings.local.json` for the
`activated_platform` field) do so by declaring `[hooks.capabilities.read_file]` with an
appropriate `path_allow` list in their `hooks-registry.toml` entry. The production
`read_file` host fn (`crates/factory-dispatcher/src/host/read_file.rs`) with the
`ReadFileCaps.path_allow` enforcement already provides this capability.

Pass-4 retired BC-1.10.001 ("Dispatcher exposes vsdd::activated_platform() host function")
as over-engineering. BC-4.04.005 is updated to declare `[hooks.capabilities.read_file]`
with `path_allow = [".claude/settings.local.json"]` alongside the existing
`[hooks.capabilities.exec_subprocess]` table. BC-4.04.001 is updated to obtain
`activated_platform` via the `read_file` host fn path rather than a bespoke host function.

### Timeout hierarchy

Three timeout budgets govern the `SessionStart` dispatch path. Each layer has headroom
above the layer it contains:

- **Layer 3 — subprocess timeout (5000ms):** The `factory-health` subprocess wait budget
  per BC-4.04.002 Invariant 4. If the subprocess does not return within 5000ms, the plugin
  maps the timeout to `factory_health = "unknown"` and proceeds.
- **Layer 2 — dispatcher `timeout_ms` (8000ms):** The per-call wasmtime epoch budget for
  the `session-start-telemetry` plugin per BC-4.04.005 Postcondition 5. This exceeds the
  subprocess timeout by 3000ms, giving the plugin headroom to handle the timeout and
  complete the `emit_event` call before the dispatcher terminates the WASM execution.
- **Layer 1 — Claude Code harness timeout (10000ms):** The `timeout` field in
  `hooks.json.template` (BC-4.04.004) bounds the entire dispatcher process invocation
  including all subprocess and plugin activity. This exceeds the dispatcher budget by
  2000ms.

Invariant: `5000ms (subprocess) < 8000ms (dispatcher) < 10000ms (harness)`. Each layer
has headroom for the layer below. Implementations MUST NOT violate this ordering; any
change to one timeout must preserve the strict inequality.

## Plugin Wire Format Constraints

Two host-fn behaviors affect all future hook plugin implementations and verification harnesses.
These constraints are not routing concerns (ADR-011's primary topic) but arise in every story that
adds a new lifecycle plugin, so they are recorded here for discoverability.

### emit_event String Coercion

The `emit_event` host fn (`crates/factory-dispatcher/src/host/emit_event.rs:49`) coerces **all**
plugin-supplied field values to `JSON strings` before storing them in the event:

```rust
ev = ev.with_field(&k, Value::String(v));
```

This applies to every field a plugin passes — integers, booleans, objects encoded as strings, and
literal string values all arrive as `Value::String` on the wire. Downstream consumers (file sink
readers, observability dashboards, integration test harnesses) MUST parse string values back to
their semantic types.

**Harness implication:** VP harnesses for any BC that specifies a typed field (integer, object,
boolean) MUST use `.is_string()` assertions, not `.is_number()` / `.is_object()` / `.is_bool()`.
A `.parse::<i64>()` round-trip is the correct way to range-check an integer field.

**Future work:** A typed `emit_event_typed` ABI that preserves JSON types end-to-end is a v1.1
candidate. Until that lands, all plugin-authored fields are strings on the wire.

### emit_event HostContext Enrichment (4 fields)

The `emit_event` host fn automatically injects four fields onto every emitted event from
`HostContext` (per BC-1.05.012 enrichment half):

| Field | Source | emit_event call |
|-------|--------|-----------------|
| `dispatcher_trace_id` | `HostContext.dispatcher_trace_id` | `.with_trace_id(...)` |
| `session_id` | `HostContext.session_id` | `.with_session_id(...)` |
| `plugin_name` | `HostContext.plugin_name` | `.with_plugin_name(...)` |
| `plugin_version` | `HostContext.plugin_version` | `.with_plugin_version(...)` |

These values are populated by the dispatcher's routing layer before the plugin is invoked.
The plugin has no responsibility to set any of these four fields. All four are listed in
`RESERVED_FIELDS` (`emit_event.rs:58-67`); any attempt by a plugin to set them is **silently
dropped** by the filter half of BC-1.05.012.

`session_id` specifically: its value originates from the incoming envelope parsed by
BC-1.02.005 lifecycle-tolerant envelope parsing, which populates `HostContext.session_id`.
When the envelope's `session_id` is missing or empty, BC-1.02.005 sets
`HostContext.session_id = "unknown"`. The plugin does not set `session_id` and must not
include it in its required-fields list.

Plugins MUST NOT include any of these four fields in their required-fields lists. All four are
always present on every event by construction (DI-017 enforcement for `dispatcher_trace_id`;
BC-1.05.012 for all four). Integration harnesses MUST assert these fields at the *host
enrichment layer* — not as plugin-set fields.

**Harness implication:** For any VP that verifies trace correlation, assert
`event["dispatcher_trace_id"].is_string()` (not `event["trace_id"]`). The field name is
`dispatcher_trace_id` — the `trace_id` alias does not exist in the event schema. Similarly,
assert `event["session_id"].is_string()` — this is a host-enriched field, not a plugin-set field.

### InternalEvent::now Construction-Time Fields (4 fields)

Four additional fields are set at event **construction time** by `InternalEvent::now()` —
this is a distinct mechanism from the `emit_event` HostContext enrichment path:

| Field | Source | Mechanism |
|-------|--------|-----------|
| `ts` | Current UTC datetime | `InternalEvent::now()` constructor |
| `ts_epoch` | Current Unix timestamp | `InternalEvent::now()` constructor |
| `schema_version` | Struct constant | `InternalEvent::now()` constructor |
| `type` | Plugin-supplied `event_name` argument | `InternalEvent::now(&event_type)` |

These fields are set before the `emit_event` enrichment loop executes. They are NOT sourced
from `HostContext`. The `type` field is special: it is derived from the plugin's `event_name`
argument (e.g., `"session.started"`), but the plugin cannot use the literal key name `"type"`
in its payload key/value pairs — the RESERVED_FIELDS filter drops it.

All four are listed in `RESERVED_FIELDS` (`emit_event.rs:58-67`); plugin attempts to set
them are silently dropped. Both mechanisms together — HostContext enrichment (4 fields) and
`InternalEvent::now` construction (4 fields) — account for all 8 RESERVED_FIELDS. The
prohibition against plugins setting RESERVED_FIELDS applies uniformly to all 8, but the
VALUE source differs: HostContext per-invocation values vs. construction-time values.

## Source / Origin

- **Master design doc:** `.factory/legacy-design-docs/2026-04-24-v1.0-factory-plugin-kit-design.md`
  lines 88–114 (file tree showing `hooks.json.template`, `.platform` variants, and
  `hooks-registry.toml` as separate files), lines 44–53 (dual-file decision in the
  Decisions section).
- **State tracking:** `.factory/STATE.md` line 119 (DRIFT-004 entry — "hooks.json +
  hooks-registry.toml dual routing tables | MEDIUM-HIGH | L-P0-002 cutover before rc.1").
- **Code as-built:** `plugins/vsdd-factory/hooks/hooks.json.template` (harness wiring
  source of truth), `plugins/vsdd-factory/hooks-registry.toml` (dispatcher routing).
- **Skill documentation:** `plugins/vsdd-factory/skills/activate/SKILL.md` step 6
  (activation writes `hooks.json` from `.platform` variant, never touches `hooks-registry.toml`).
