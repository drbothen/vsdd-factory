---
document_type: adr
adr_id: ADR-018
status: accepted
accepted_date: 2026-05-07
date: 2026-05-07
cycle: v1.0-feature-engine-discipline-pass-1
subsystems_affected: [SS-01, SS-04]
supersedes: null
superseded_by: null
---

# ADR-018: WASM-Plugin Context Resolvers — Design and Layering

## Context

### The operational gap that triggered this decision

F-P2-001 (adversarial review pass 2, 2026-05-07) found that
`validate-per-story-adversary-convergence` (shipped in S-12.02) is inert in
production. The hook reads `plugin_config.wave_context.stories` to determine
which stories require convergence clearance, but nothing in the dispatch
pipeline writes that value. The dispatcher splices only the static
`hooks-registry.toml` config into `plugin_config`; `wave-state.yaml` (the
runtime artifact that holds the active wave's story list) is never consulted at
dispatch time.

F5 pass-1 (B3 fix burst) added consumer-side plumbing to the hook to read the
key, but this only moved the problem: the producer side — something that reads
`wave-state.yaml` and injects its contents into `plugin_config` — does not exist.

### Three fix options considered

**Option α — hook reads `wave-state.yaml` directly via `host::read_file`.**
Works for this one hook. Does not generalize: every future hook that needs runtime
state must embed its own reading and parsing logic. There is no isolation between
data-provision and behavioral logic. Violates the single-responsibility principle.
The hook ABI is designed for behavioral decision-making, not data aggregation.

**Option β — skill writes a manifest before `SubagentStop` fires.**
Couples the fix to the orchestration layer. The wave-gate skill would need to
predict dispatcher behavior and pre-seed a sidecar file. This is brittle: the
manifest may be stale, missing, or inconsistent with the actual dispatch context.
No WASM isolation for the data-provision step. Side-channel coordination between
an SS-06 skill and an SS-04 hook is an architectural smell.

**Option γ — factory-agnostic WASM-plugin resolvers (user-authorized).**
The dispatcher core gains a `ContextResolver` trait and a `ResolverRegistry`.
Resolver logic lives in separate WASM artifacts, sandboxed identically to hooks.
Hooks opt in to resolver invocations per entry via `needs_context = [...]` in
`hooks-registry.toml`. The dispatcher invokes only the declared resolvers before
each matched hook dispatch, merges their outputs into `plugin_config`, then
dispatches the hook with the enriched payload. Per-factory resolver crates ship
the domain-specific context logic; the dispatcher core has zero awareness of
vsdd-factory's domain vocabulary (waves, stories, cycle IDs, article drafts, etc.).

User authorized option γ as the correct maximum-ambition architectural path per
D-361 (2026-05-07).

### Factory-agnostic invariant

A central requirement (user-directed, D-361) is that `crates/factory-dispatcher/`
must be usable by all factory types — vsdd-factory, a hypothetical content-factory,
a PR-review-factory — without modification. This means the dispatcher core must
have zero compile-time dependency on any per-factory resolver crate. Per-factory
resolvers ship as separately-compiled WASM artifacts and are referenced only via
`resolvers-registry.toml` entries at runtime.

---

## Decision

### Core architecture

A `ContextResolver` trait and a `ResolverRegistry` struct are added to
`crates/factory-dispatcher/src/resolver.rs`. The registry is loaded at dispatcher
startup alongside `hooks-registry.toml`. Before each hook dispatch, the dispatcher
inspects `entry.needs_context`; for each declared resolver name, it invokes the
registered WASM resolver and merges the output into `plugin_config` under the
resolver's declared key. If `needs_context` is absent or empty, the resolver
invocation path is a zero-cost no-op.

### Six operational design decisions (OD-1 through OD-6)

All six decisions were locked per the F1-amendment analysis and user authorization
(D-361). They are encoded here as binding ADR content.

**OD-1 — Resolver lifecycle: load once at dispatcher startup.**

Resolver WASM artifacts are compiled into `Module` instances at dispatcher startup
(not per-dispatch). Per-dispatch invocation creates a fresh `Store` per resolver
call — same pattern as hooks. Module compilation is amortized across all dispatches
within a process lifetime. Cache is keyed by resolver WASM path + mtime (same
invalidation logic as `plugin_loader.rs`). This matches the proven pattern in
`crates/factory-dispatcher/src/plugin_loader.rs`.

Rejected alternative (per-dispatch compilation): simpler lifecycle but adds
wasmtime compilation overhead to every hook invocation that has `needs_context`.
Not acceptable for a dispatcher that is invoked frequently during agent sessions.

**OD-2 — Resolver registration: separate `resolvers-registry.toml`.**

Resolvers are registered in `plugins/vsdd-factory/resolvers-registry.toml`, a
distinct file from `hooks-registry.toml`. Resolvers are not hooks: they have a
different lifecycle (pre-dispatch data providers, not event handlers), a different
ABI (`resolve()` export returning data, not `block/continue` decisions), and
different capability profiles (read-only by design; no `emit_event`,
`exec_subprocess`, or `write_file`). Conflating them in `hooks-registry.toml`
would create schema ambiguity and force readers to mentally separate two conceptually
distinct registration concerns from one file. A separate file also enables
independent schema versioning.

Rejected alternative (extend `hooks-registry.toml`): one file to load at startup
is operationally simpler. Rejected because the conceptual distinction is
load-bearing for the factory-agnostic invariant and for the ABI versioning
independence requirement.

**OD-3 — Resolver ABI: distinct `ResolverInput`/`ResolverOutput` types, `RESOLVER_ABI_VERSION = 1`.**

Resolvers expose a single exported function:

```
resolve(input_ptr: i32, input_len: i32) -> i64
```

The i64 return encodes a (ptr, len) pair for the output buffer. Input is a
JSON-serialized `ResolverInput { event_type, hook_event_name, agent_type,
project_dir, plugin_config }`. Output is a JSON-serialized `ResolverOutput
{ key: String, value: Option<serde_json::Value> }`.

`ResolverInput` and `ResolverOutput` are distinct types from `HookPayload` and
`HookResult`. Resolvers have no block/continue semantics; they return data, not
decisions. `RESOLVER_ABI_VERSION = 1` is versioned independently from
`HOST_ABI_VERSION = 1` to allow resolver ABI evolution without coupling to hook
ABI version bumps.

Rejected alternative (reuse `HookPayload`/`HookResult`): saves type definitions
but carries irrelevant fields (`tool_input`, `tool_output`) and creates semantic
confusion. Resolver ABI changes would be forced to track hook ABI changes.

**OD-4 — No resolver output caching initially.**

Resolver invocations are fast WASM function calls (microseconds to low milliseconds
for file-read plus parse). No caching layer is added in the initial implementation.
Adding a cache introduces invalidation complexity (cache key, TTL, agent_type
sensitivity) with no measured performance need. Per-dispatch re-invocation is the
safe default. Caching can be added if profiling identifies resolver invocation as a
bottleneck.

Rejected alternative (per-dispatch cache keyed on resolver name + event context):
compelling only when multiple hooks in one dispatch share the same resolver. Current
dispatch model is one hook event per process invocation (dispatcher is not a daemon);
the optimization opportunity is minimal.

**OD-5 — Flat resolver list; no inter-resolver composition (DAG).**

Resolvers are invoked as a flat list. There are no declared dependencies between
resolvers; resolver A's output cannot feed resolver B's input in the initial
implementation. DAG-based composition requires topological sort, cycle detection, and
output-to-input threading. The first concrete resolver (`WaveContextResolver`) has
no dependencies. DAG composition is deferred to a future cycle when a concrete use
case requires it.

Rejected alternative (DAG composition): powerful for multi-stage enrichment but
adds architectural complexity that is not yet warranted.

**OD-6 — Explicit registration only; no auto-discovery.**

All resolvers must be listed in `resolvers-registry.toml` with their WASM path and
`path_allow` declarations. The dispatcher does not scan directories for WASM artifacts
to load as resolvers. Explicit registration is preferred because: (a) capability
declarations (`path_allow`) cannot be auto-inferred from artifact presence alone;
(b) auto-discovery introduces load-order ambiguity; (c) the factory-agnostic invariant
requires that capability grants be a human-auditable, version-controlled decision.

Rejected alternative (auto-discover from `hook-plugins/resolvers/`): less
configuration; resolvers just need to be present in a directory. Rejected because
capability declarations are a security-critical decision that must be explicit.

### Factory-agnostic / per-factory layering

The layering is strict:

- **`crates/factory-dispatcher/`** — resolver infrastructure only. Zero awareness
  of vsdd-factory's domain. Contains: `ContextResolver` trait, `ResolverRegistry`,
  `ResolverInput`, `ResolverOutput`, `ResolverError`, `resolver_loader.rs`,
  `resolver_classify_trap.rs`. The dispatcher binary has no compile-time link to
  any per-factory resolver crate.

- **`crates/hook-sdk/` (with `resolver-authoring` feature flag)** — resolver
  authoring API. Exports `ResolverInput`, `ResolverOutput`, and the `#[resolver]`
  proc-macro (via `crates/hook-sdk-macros/`). The feature flag ensures hook crates
  that are NOT resolvers do not pull in the extra types.

- **`crates/vsdd-context-resolvers/`** — per-factory resolver crate for vsdd-factory.
  Contains `WaveContextResolver`: reads `.factory/wave-state.yaml`, parses the active
  wave's story list, returns `ResolverOutput { key: "wave-context", value: Some(...) }`.
  Compiles to `plugins/vsdd-factory/hook-plugins/vsdd-context-resolvers.wasm`.
  Registered in `plugins/vsdd-factory/resolvers-registry.toml` with
  `path_allow = [".factory/"]`.

This three-layer structure means any future factory (content-factory,
PR-review-factory) can ship its own resolver crate and `resolvers-registry.toml`
entries without touching the dispatcher binary.

### Hook opt-in mechanism

The `RegistryEntry` struct in `crates/factory-dispatcher/src/registry.rs` gains an
optional field `needs_context: Vec<String>` (defaults to `[]`; serde default).
All existing registry entries are backward-compatible: absent `needs_context` is
semantically equivalent to `needs_context = []` and incurs zero resolver invocations.

The convergence hook entry in `plugins/vsdd-factory/hooks-registry.toml` gains:

```toml
needs_context = ["wave-context"]
```

This is the only modification to `hooks-registry.toml` for the initial deployment.

### Absent `resolvers-registry.toml`

If `resolvers-registry.toml` does not exist at the path configured at dispatcher
startup, the dispatcher treats this as zero resolvers and proceeds normally. This
is NOT a startup error. This ensures factories that have not yet authored any
resolvers are unaffected by the new infrastructure.

---

## Rationale

The γ-WASM-plugin approach is the correct path because:

1. **Generality.** The dispatcher serves multiple factory types. A generic mechanism
   solves the data-injection problem for all hooks, not just the convergence hook.
   A one-off fix for `wave-state.yaml` reading would be a stopgap that every future
   hook needing runtime state would have to duplicate.

2. **Isolation.** WASM sandboxing with `path_allow` declarations provides the same
   capability-isolation guarantees for data-provision code as for hook behavior code.
   A resolver that is buggy or compromised cannot affect the dispatcher process beyond
   its declared capability grants.

3. **Separation of concerns.** Resolvers are data providers; hooks are decision makers.
   The ABI distinction (`ResolverOutput` vs `HookResult`) enforces this at the type
   level. A resolver cannot accidentally block a dispatch; it can only inject or
   withhold context.

4. **Maintainability.** Factory-specific resolver logic is versioned and deployed
   independently of the dispatcher binary. Updating the `WaveContextResolver` to
   handle a new `wave-state.yaml` schema does not require a dispatcher rebuild.

5. **Test surface.** The pure computation portion of each resolver (the part that
   processes parsed data) is testable with proptest determinism proofs. The file I/O
   boundary is exercised by integration tests with real WASM execution.

---

## Subsystem Assignments

**SS-01 (Hook Dispatcher Core):** Referencing SS-01 because the pre-dispatch
resolver invocation step (`resolver.rs`, `resolver_loader.rs`) is a new module in
the `crates/factory-dispatcher/` executor pipeline. The `RegistryEntry.needs_context`
field extension lives in `registry.rs` (SS-01). The pre-dispatch sequence is an
SS-01 concern per the Subsystem Registry: "crates/factory-dispatcher/src/{main,
registry,routing,executor,invoke,engine,plugin_loader,payload}.rs".

**SS-04 (Plugin Ecosystem):** Referencing SS-04 because the resolver WASM artifacts
are new plugin-ecosystem components: `crates/vsdd-context-resolvers/` is a new
plugin crate compiled to `.wasm`, registered in `resolvers-registry.toml`, and
loaded by the dispatcher using the same wasmtime infrastructure as hook plugins.
The resolver capability model (OD-3 ABI, OD-6 explicit registration, OD-2 separate
registry) is an SS-04 behavioral contract surface.

---

## Alternatives Considered

### α — Hook reads `wave-state.yaml` directly via `host::read_file`

Rejected. Conflates data-provision and behavioral-decision logic in one WASM binary.
Every future hook needing runtime state must embed its own parsing logic with no
shared infrastructure. Provides no isolation between the data-provision step and the
hook's core decision function. Does not generalize across factory types.

### β — Skill writes a manifest before `SubagentStop` fires

Rejected. Couples the fix to the orchestration layer (SS-06 skill). The skill must
predict dispatcher dispatch ordering and pre-seed a sidecar file at the right path
before the hook fires. This is brittle to skill execution ordering and to agent
sessions where the skill is not dispatched. No WASM isolation for the data-provision
step. Side-channel coordination between an SS-06 skill and an SS-04 hook violates
layer separation.

### γ-compile-time — Resolvers as Rust functions registered at compile time in `factory-dispatcher`

Rejected. Couples the generic dispatcher binary to vsdd-factory domain code. Any
factory-specific resolver function linked into the dispatcher binary means the
dispatcher binary is no longer factory-agnostic. Adding a resolver requires a
dispatcher rebuild and new binary release. Violates the factory-agnostic invariant
(D-361 user requirement).

### γ-config-driven — Resolvers as TOML-declarative data transforms

Rejected. A config-driven resolver (e.g., `read_file = "wave-state.yaml";
jq_query = ".waves[-1].stories"`) is insufficiently expressive for resolution logic
that must handle schema evolution, missing files, malformed YAML, and edge cases in
the wave-state data model. No WASM isolation; no capability sandboxing. Error
handling and partial-success semantics cannot be expressed declaratively.

---

## Consequences

- **`factory-dispatcher` gains two new modules** (`resolver.rs`,
  `resolver_loader.rs`). The executor pipeline is extended with a pre-dispatch
  resolver invocation step that is a zero-cost no-op for entries without
  `needs_context`.

- **`hook-sdk` gains a `resolver-authoring` feature flag** and the `#[resolver]`
  proc-macro. Hook crates that are not resolvers see no change to their dependency
  graph.

- **`HOST_ABI.md` gains a Context Injection Contract section** (S-12.06) documenting
  `RESOLVER_ABI_VERSION = 1`, `ResolverInput`/`ResolverOutput` shapes, `needs_context`
  semantics, merging rules, capability model, and failure semantics.

- **Future factories can ship their own resolver crates** without touching
  `factory-dispatcher`. The dispatcher binary is resolver-agnostic.

- **`hooks-registry.toml` gains the optional `needs_context` field** on entries. All
  existing entries parse correctly with no change (serde default to `[]`).

- **`resolvers-registry.toml` is a new file type** that must be added to
  `artifact-path-registry.yaml` (ADR-016) as part of story delivery.

- **Resolver crashes and errors do not block hook dispatch.** The dispatcher wraps
  each resolver call in the same trap-classification logic used by `invoke.rs` for
  hooks. A resolver error emits `resolver.error` and proceeds without that resolver's
  context key in `plugin_config`. The hook receives an absent key and must decide
  how to handle it (hard error, graceful degrade, etc. — the dispatcher does not
  decide on the hook's behalf).

---

## Behavioral Contracts

This ADR is implemented by:

- BC-1.13.001 — Resolver-registry loading and pre-dispatch context injection (SS-01)
- BC-4.12.001 — Resolver lifecycle invariant (SS-04)
- BC-4.12.002 — Resolver ABI and payload schema (SS-04)
- BC-4.12.003 — Resolver capability model (SS-04)
- BC-4.12.004 — Resolver error and crash isolation semantics (SS-04)
- BC-4.12.005 — Context-injection merging contract (SS-04)

(BCs authored in parallel by the product-owner in the F2-amendment burst.)

---

## Verification Properties

- VP-073: Resolver-load purity — loading a `.wasm` resolver is deterministic and
  side-effect-free at load time (integration / unit-test, P1)
- VP-074: Resolver-error isolation — a resolver crash does NOT propagate to the
  dispatcher process (kani + integration, P1)
- VP-075: Context-injection determinism — given identical inputs, a resolver returns
  identical output (proptest, P1)
- VP-076: Resolver-capability confinement — a resolver attempting to read an
  out-of-`path_allow` path receives `CapabilityDenied` and does not observe file
  contents (integration, P1)

See `/Users/jmagady/Dev/vsdd-factory/.factory/specs/verification-properties/` for
full VP definitions.

---

## Decision Log Reference

| Decision | ID | Rationale |
|----------|----|-----------|
| Mid-cycle scope expansion to WASM-plugin resolver platform | D-361 | F-P2-001 convergence hook inert in production; γ-generic chosen |
| OD-1 through OD-6 locked | D-361 | F1-amendment recommendations accepted by user |
| F2-amendment dispatch (architect + PO parallel) | D-361 | Structured amendment cycle; not a hot-fix |

## Cross-References

- ADR-002: WASM (wasmtime) plugin ABI — resolver WASM artifacts use the same
  wasmtime runtime; referencing ADR-002 because the resolver loading infrastructure
  reuses wasmtime `Module`/`Store` patterns established there.
- ADR-006: HOST_ABI_VERSION as separate semver constant — `RESOLVER_ABI_VERSION = 1`
  follows the same independently-versioned constant pattern established by ADR-006
  for `HOST_ABI_VERSION`.
- ADR-016: Artifact path registry as single source of truth — `resolvers-registry.toml`
  is a new artifact type that must be registered in `artifact-path-registry.yaml`.
  Referencing ADR-016 because it established the single-source-of-truth principle
  that the resolver's separate registry file honors.
- ADR-017: Per-story adversary phasing — the convergence hook (S-12.02, anchored
  by ADR-017) is the primary consumer of the resolver platform. ADR-018 provides
  the infrastructure that makes ADR-017's hook operationally effective.
