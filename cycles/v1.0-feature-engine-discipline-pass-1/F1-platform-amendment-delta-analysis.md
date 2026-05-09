---
document_type: feature-delta-analysis
cycle_id: v1.0-feature-engine-discipline-pass-1
phase: F1-amendment
amendment_type: mid-cycle-scope-expansion
status: draft
created: 2026-05-07
author: architect
input-source: F-P2-001 + user architectural directive 2026-05-07
---

<!-- L-P25-002 carve-out: This is a Phase F1-amendment (architect-proposal, mid-cycle scope expansion) artifact preserved as historical audit record. Pseudocode symbols within are PRE-MERGE PLANNING VOCABULARY. Downstream specs MUST replace with merged-code symbols per L-P21-001 + L-P24-002. This file itself is exempt from fabricated-symbol sweeps under the L-P25-002 F1-architect-proposal carve-out (extends L-P24-001 brownfield Phase 0 carve-out). The F1-amendment phase is structurally an amendment to a Phase F1 architect proposal and is treated as in-scope of L-P25-002. Codified at lessons.md L-P25-002. -->

# F1-Amendment Delta Analysis — WASM-Plugin Context Resolver Platform

## 1. Scope Summary

### What the platform IS

A **factory-agnostic, WASM-plugin-based context-resolver mechanism** that enables the
dispatcher to dynamically inject runtime context into hook plugin invocations. The
dispatcher core (`crates/factory-dispatcher/`) gains a `ContextResolver` trait and a
`ResolverRegistry` that it calls before each hook dispatch. Hooks declare which
resolvers they need via `needs_context = [...]` in `hooks-registry.toml`. The
dispatcher invokes only the declared resolvers, merges their outputs into
`plugin_config` under each resolver's key, then dispatches the hook with the
enriched payload.

Resolvers are WASM plugins — same ABI model as hooks — compiled separately and
loaded at dispatcher startup. Each resolver reads its allowed inputs
(event envelope, `agent_type`, `project_dir`, selected filesystem paths) and
returns `Option<serde_json::Value>`. Resolvers run with capability-restricted
filesystem access declared per-resolver in `resolvers-registry.toml`.

The platform is **completely factory-agnostic at the dispatcher layer**: the
dispatcher core knows nothing about vsdd-factory's domain, waves, or STATE.md.
Factory-specific resolver logic lives in a separate per-factory crate
(`crates/vsdd-context-resolvers/`), compiled to `.wasm`, and referenced by
`resolvers-registry.toml` entries.

### What it IS NOT

- Not a specific resolver. The `WaveContextResolver` (the first concrete resolver)
  is a per-factory concern in `crates/vsdd-context-resolvers/`. The dispatcher
  crate is unaware of it.
- Not a vsdd-specific feature. Any factory using `factory-dispatcher` can ship
  its own resolver crate and register it.
- Not a one-off fix for F-P2-001. F-P2-001 is the triggering finding, but the
  platform is the correct architectural solution. S-12.08 (the terminal story)
  delivers the F-P2-001 fix as a consumer of the platform.
- Not a compile-time resolver registry. Resolvers are dynamically loaded WASM
  artifacts, not Rust functions linked at compile time.
- Not a hook permission expansion. Resolvers do NOT execute arbitrary host
  functions; they are sandboxed identically to hooks, with declared
  `path_allow` entries covering only the files they must read.

### Triggering Finding

F-P2-001 (adversarial review pass 2, 2026-05-07): the convergence hook
(`validate-per-story-adversary-convergence`) is operationally inert in production
because no mechanism populates `plugin_config.stories` at dispatch time. The
dispatcher splices only the **static** `hooks-registry.toml` config into
`plugin_config`. The hook needs the active wave's story list, which lives in
`wave-state.yaml` — a runtime artifact that is not present in the registry.

Three fix options were considered and the architectural directive selected option
γ-WASM-plugin resolvers as the maximum-ambition path: generic infrastructure that
solves the data-injection problem for all hooks, not just the convergence hook.

---

## 2. Impact Boundary

### New Files and Crates

| Path | Category | Purpose |
|------|----------|---------|
| `/Users/jmagady/Dev/vsdd-factory/crates/vsdd-context-resolvers/` | NEW-CRATE | Per-factory resolver crate; `WaveContextResolver` reads `wave-state.yaml` + `STATE.md`; compiles to `vsdd-context-resolvers.wasm` |
| `/Users/jmagady/Dev/vsdd-factory/crates/vsdd-context-resolvers/src/lib.rs` | NEW | WaveContextResolver entry point; `#[resolver]` macro; reads `plugin_config.project_dir`; parses wave-state; returns `wave_context` Value |
| `/Users/jmagady/Dev/vsdd-factory/crates/vsdd-context-resolvers/src/wave_context.rs` | NEW | Wave-state parsing logic; produces `WaveContext { stories, wave_id, cycle_id }` |
| `/Users/jmagady/Dev/vsdd-factory/crates/vsdd-context-resolvers/Cargo.toml` | NEW | Crate manifest; WASM target; depends on `hook-sdk` (resolver traits) |
| `/Users/jmagady/Dev/vsdd-factory/crates/vsdd-context-resolvers/tests/` | NEW | Unit tests for WaveContextResolver parsing + output shape |
| `/Users/jmagady/Dev/vsdd-factory/crates/factory-dispatcher/src/resolver.rs` | NEW | `ContextResolver` trait; `ResolverRegistry` loading + caching; resolver invocation; error isolation; merge-into-plugin_config logic |
| `/Users/jmagady/Dev/vsdd-factory/crates/factory-dispatcher/src/resolver_loader.rs` | NEW | Load `.wasm` resolver artifacts; same mtime-cache pattern as `plugin_loader.rs`; returns `ResolverModule` |
| `/Users/jmagady/Dev/vsdd-factory/plugins/vsdd-factory/resolvers-registry.toml` | NEW | Resolver registration file; TOML; `[[resolvers]]` table; declares `name`, `plugin`, `path_allow`; separate from `hooks-registry.toml` |
| `/Users/jmagady/Dev/vsdd-factory/plugins/vsdd-factory/hook-plugins/vsdd-context-resolvers.wasm` | NEW | Built artifact; output of `crates/vsdd-context-resolvers/` cross-compile |
| `/Users/jmagady/Dev/vsdd-factory/crates/hook-sdk/src/resolver.rs` | NEW | `ResolverInput`, `ResolverOutput`, `#[resolver]` proc-macro glue; host ABI extension for resolver context accessors |
| `/Users/jmagady/Dev/vsdd-factory/crates/hook-sdk-macros/src/resolver_macro.rs` | NEW | `#[resolver]` macro implementation; parallel to existing `#[hook]` macro |

### Modified Files

| Path | Category | Change |
|------|----------|--------|
| `/Users/jmagady/Dev/vsdd-factory/crates/factory-dispatcher/src/executor.rs` | MODIFIED | Call `ResolverRegistry::resolve_context(entry, &payload)` before plugin dispatch; merge result into `plugin_config` before `invoke_plugin` |
| `/Users/jmagady/Dev/vsdd-factory/crates/factory-dispatcher/src/main.rs` | MODIFIED | Load `resolvers-registry.toml` at startup alongside `hooks-registry.toml`; pass `ResolverRegistry` into executor |
| `/Users/jmagady/Dev/vsdd-factory/crates/factory-dispatcher/src/registry.rs` | MODIFIED | `RegistryEntry` gains `needs_context: Vec<String>` optional field (defaults empty; backward-compatible) |
| `/Users/jmagady/Dev/vsdd-factory/crates/factory-dispatcher/src/host/mod.rs` | MODIFIED | Wire resolver-specific host functions (context read-only accessors) into `Linker<HostContext>` for resolver execution |
| `/Users/jmagady/Dev/vsdd-factory/crates/factory-dispatcher/Cargo.toml` | MODIFIED | No new external deps (wasmtime already present); resolver.rs + resolver_loader.rs are internal modules |
| `/Users/jmagady/Dev/vsdd-factory/crates/hook-sdk/src/lib.rs` | MODIFIED | Re-export `resolver` module; keep hook API unchanged |
| `/Users/jmagady/Dev/vsdd-factory/crates/hook-sdk/Cargo.toml` | MODIFIED | Add `resolver-authoring` feature flag gating resolver traits (optional; avoids hook crates pulling in unused types) |
| `/Users/jmagady/Dev/vsdd-factory/crates/hook-sdk-macros/src/lib.rs` | MODIFIED | Export `#[resolver]` macro alongside `#[hook]` macro |
| `/Users/jmagady/Dev/vsdd-factory/plugins/vsdd-factory/hooks-registry.toml` | MODIFIED | Schema extension: `needs_context = ["wave-context"]` added to `validate-per-story-adversary-convergence` entry; existing entries unmodified (backward-compat; absent field defaults to `[]`) |
| `/Users/jmagady/Dev/vsdd-factory/crates/hook-sdk/HOST_ABI.md` | MODIFIED | Major extension: §Context Injection Contract — resolver input shape, output merging semantics, `needs_context` field, resolver ABI version, failure isolation guarantees |
| `/Users/jmagady/Dev/vsdd-factory/crates/hook-plugins/validate-per-story-adversary-convergence/src/lib.rs` | MODIFIED | `RealCallbacks::list_stories` reads `plugin_config.wave_context.stories` (injected by WaveContextResolver) instead of falling back to absent static config; removes graceful-degrade Continue path for missing stories |
| `/Users/jmagady/Dev/vsdd-factory/plugins/vsdd-factory/hooks-registry.toml` | MODIFIED | See above (`needs_context` field on convergence hook entry) |
| `/Users/jmagady/Dev/vsdd-factory/crates/factory-dispatcher/tests/integration_test.rs` | MODIFIED | Extend with resolver-loading integration tests; end-to-end: registry loads resolver → hook receives merged plugin_config |

### Spec Artifacts

| Path | Category | Change |
|------|----------|--------|
| `/Users/jmagady/Dev/vsdd-factory/.factory/specs/architecture/ARCH-INDEX.md` | MODIFIED | +ADR-018; SS-04 BC count update; SS-01 module table extension |
| `/Users/jmagady/Dev/vsdd-factory/.factory/specs/architecture/SS-01-hook-dispatcher.md` | MODIFIED | +`resolver.rs`, `resolver_loader.rs` in module table; resolver lifecycle in dispatch flow |
| `/Users/jmagady/Dev/vsdd-factory/.factory/specs/architecture/SS-04-plugin-ecosystem.md` | MODIFIED | +`vsdd-context-resolvers/` crate; resolver WASM artifact; resolver capability model |
| `/Users/jmagady/Dev/vsdd-factory/.factory/specs/architecture/SS-02-hook-sdk.md` | MODIFIED | +resolver authoring API; `#[resolver]` macro; `resolver-authoring` feature flag |
| `/Users/jmagady/Dev/vsdd-factory/.factory/specs/architecture/decisions/ADR-018-wasm-plugin-context-resolvers.md` | NEW | Codifies the WASM-plugin resolver design decision |
| `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.13.001.md` | NEW | Resolver-registry loading and context-injection pre-dispatch contract |
| `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.12.001.md` | NEW | Resolver lifecycle invariant |
| `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.12.002.md` | NEW | Resolver ABI and payload schema |
| `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.12.003.md` | NEW | Resolver capability model |
| `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.12.004.md` | NEW | Resolver error and crash isolation semantics |
| `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.12.005.md` | NEW | Context-injection merging contract |
| `/Users/jmagady/Dev/vsdd-factory/.factory/specs/verification-properties/VP-073.md` | NEW | Resolver-load purity |
| `/Users/jmagady/Dev/vsdd-factory/.factory/specs/verification-properties/VP-074.md` | NEW | Resolver-error isolation |
| `/Users/jmagady/Dev/vsdd-factory/.factory/specs/verification-properties/VP-075.md` | NEW | Context-injection determinism |
| `/Users/jmagady/Dev/vsdd-factory/.factory/specs/verification-properties/VP-076.md` | NEW | Resolver-capability confinement |
| `/Users/jmagady/Dev/vsdd-factory/.factory/specs/verification-properties/VP-INDEX.md` | MODIFIED | +4 VPs (VP-073..076); total 72→76 |
| `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/BC-INDEX.md` | MODIFIED | +6 BCs; total 1937→1943; SS-01 114→115; SS-04 34→39 |
| `/Users/jmagady/Dev/vsdd-factory/.factory/specs/prd.md` | MODIFIED | +1 FR (platform-agnostic runtime context injection); subsystem registry counts updated |

---

## 3. Affected Specs

### Proposed Behavioral Contracts

Six new BCs spanning SS-01 (dispatcher contract for the new pre-dispatch resolver
invocation step) and SS-04 (plugin ecosystem contracts for resolver behavior).

**BC-1.13.001 — SS-01: Resolver-registry loading and pre-dispatch context injection**

The dispatcher loads `resolvers-registry.toml` at startup (same mtime-cache
pattern as `hooks-registry.toml`). Before each hook dispatch, it inspects
`entry.needs_context`; for each declared resolver name, it invokes the registered
WASM resolver with a `ResolverInput` struct containing the event envelope,
`agent_type`, and `project_dir`. The dispatcher merges each resolver's
`Option<Value>` output into `plugin_config` under the resolver's key. If
`needs_context` is empty or absent, no resolver is called. Context injection
occurs before `invoke_plugin`; the hook sees the merged `plugin_config`.

Rationale: this contract lives in SS-01 because the dispatcher core owns the
pre-dispatch sequence; resolver lifecycle details live in SS-04.

**BC-4.12.001 — SS-04: Resolver lifecycle invariant**

Resolvers are loaded at dispatcher startup (not per-dispatch). A resolver WASM
artifact is compiled into a `Module` at load time; per-dispatch invocation creates
a fresh `Store` per resolver call (same pattern as hooks, except the module
compilation is amortized). Resolver module cache is keyed by path + mtime (same
invalidation logic as `plugin_loader.rs`). All resolvers listed in
`resolvers-registry.toml` are loaded at startup; if any fail to load, the
dispatcher emits `resolver.load_error` and exits with the same non-blocking
exit-0 semantics as registry errors, unless `fail_closed = true` is set in the
resolver's registry entry.

**BC-4.12.002 — SS-04: Resolver ABI and payload schema**

Resolvers expose a single exported function `resolve(input_ptr: i32, input_len: i32) -> i64`
returning a ptr+len pair encoding a JSON-serialized `ResolverOutput` (distinct from
`HookResult`; resolvers do NOT return block/continue exit codes). Input is a JSON-
serialized `ResolverInput { event_type: String, hook_event_name: String, agent_type: Option<String>, project_dir: String, plugin_config: Value }`.
Output is `ResolverOutput { key: String, value: Option<Value> }` where `key` is the
resolver's registry name and `value` is the context payload (or `None` if resolver
has no output for this dispatch context). This is Resolver ABI v1, versioned
independently from Hook ABI v1.

**BC-4.12.003 — SS-04: Resolver capability model**

Each resolver entry in `resolvers-registry.toml` declares `path_allow = [...]`
listing the path prefixes the resolver may read via `host::read_file`. No
`write_file`, `exec_subprocess`, or `emit_event` host functions are available to
resolvers (resolvers are read-only by design). Resolvers may call `host::log` for
diagnostics. Capability enforcement uses the same deny-by-default mechanism as
hooks (DI-004). A resolver attempting to read an unallowed path receives
`CapabilityDenied` (return code, not panic); this is surfaced as
`ResolverError::CapabilityDenied` in the dispatcher's resolver invocation result.

**BC-4.12.004 — SS-04: Resolver error and crash isolation semantics**

A resolver panic, trap, timeout, or `CapabilityDenied` MUST NOT propagate to the
dispatcher process or crash the hook dispatch flow. The dispatcher wraps each
resolver call in the same trap-classification logic used by `invoke.rs` for hooks.
On any resolver error, the dispatcher: (1) emits `resolver.error` event with
resolver name and error detail; (2) does NOT merge the failed resolver's output
into `plugin_config`; (3) proceeds to hook dispatch WITHOUT the missing context
(fail-loud for declared resolvers: the hook will see an absent field and must
handle it explicitly — no silent degrade for non-optional context).

Distinction from graceful-degrade: the hook is responsible for treating a missing
`wave_context` key as a hard error if that context is required for its correctness
guarantee. The dispatcher does not decide on the hook's behalf.

**BC-4.12.005 — SS-04: Context-injection merging contract**

The dispatcher merges resolver outputs into `plugin_config` under each resolver's
`key` field from the registry. Merge is additive: existing `plugin_config` fields
from the static registry entry are preserved; resolver output keys are overlaid
(resolver output wins on key collision, with a `resolver.merge_collision` event
emitted). The final `plugin_config` value seen by the hook equals the static
registry config merged with all resolver outputs, in registry declaration order
(first resolver declared wins on collision). Merge is pure JSON value overlay —
no deep merge; the resolver's `value` replaces the key wholesale.

### Proposed ADR

**ADR-018 — WASM-plugin Context Resolvers — Design and Layering**

Next-available ID: ADR-018 (last accepted is ADR-017 at
`decisions/ADR-017-per-story-adversary-phasing.md`).

The ADR codifies:

- **Decision:** Use sandboxed WASM-plugin resolvers for runtime context injection.
  The dispatcher core (`factory-dispatcher`) exposes a `ContextResolver` trait
  and a `ResolverRegistry`. Per-factory context logic lives in separate WASM
  resolver crates. Hooks opt in per-entry via `needs_context = [...]`.
- **Factory-agnostic layering:** `factory-dispatcher` has zero awareness of
  vsdd-factory's domain. `crates/vsdd-context-resolvers/` is the per-factory
  resolver crate; it is compiled to WASM and loaded dynamically.
- **Opt-in mechanism:** `needs_context = ["wave-context"]` in
  `hooks-registry.toml`; entries without this field are unaffected.
- **Separate registry:** `resolvers-registry.toml` is a distinct file from
  `hooks-registry.toml` (rationale: resolvers have a different lifecycle — they
  are not dispatched per event; they are invoked as data providers pre-dispatch;
  conflating them with hook entries would require a schema extension that mixes
  two conceptually distinct registration concerns).
- **Rejected alternatives:**
  - α — Hook reads `wave-state.yaml` directly via `host::read_file`: works for
    this one hook but does not generalize; every future hook needing runtime state
    must duplicate the parsing logic; no isolation between data-provider and
    behavior logic.
  - β — Skill writes a manifest before SubagentStop fires: requires skill-layer
    coupling; orchestration-layer fix for a dispatcher-layer problem; brittle to
    skill dispatch ordering; no WASM isolation.
  - γ-compile-time — Resolvers as Rust functions registered at compile time in
    `factory-dispatcher`: couples the generic dispatcher binary to factory domain
    code; breaks factory-agnostic invariant; requires rebuild to add a resolver.
  - γ-config-driven — Resolvers as TOML-declarative config (e.g., `read_file =
    "wave-state.yaml"; jq_query = ".waves[-1].stories"`): insufficiently expressive
    for complex resolution logic; no error isolation; no capability sandboxing.
- **Consequences:** `factory-dispatcher` gains two new modules
  (`resolver.rs`, `resolver_loader.rs`); `hook-sdk` gains a `resolver-authoring`
  feature flag; `HOST_ABI.md` gains a resolver contract section; future factories
  can ship their own resolver crates without touching the dispatcher.

### Proposed Verification Properties

**VP-073 — Resolver-load purity**
- Type: invariant
- Proof method: unit-test (integration test of resolver module compilation)
- Scope: SS-01, SS-04
- Property: loading a `.wasm` resolver artifact (`Module::from_file`) is
  deterministic and has no observable side effects at load time. Given the same
  `.wasm` bytes, two sequential `get_or_compile` calls return structurally
  identical modules. The load operation does not call any host functions, does
  not read `resolvers-registry.toml` again, and does not modify global state.
- Feasibility: achievable via unit test; module compilation is deterministic in
  wasmtime. Kani is not applicable (OS I/O boundary); integration test is
  sufficient.

**VP-074 — Resolver-error isolation**
- Type: safety
- Proof method: kani (pure logic branch) + integration test (trap injection)
- Scope: SS-01, SS-04
- Property: a resolver panic, trap, or timeout does not propagate to the
  dispatcher process. The dispatcher's `invoke_resolver` function returns
  `Result<ResolverOutput, ResolverError>` in all cases; the error variant is
  handled without unwinding the dispatch stack. The kani harness verifies the
  pure error-classification logic (`classify_resolver_trap`); the integration
  test injects a deliberately-panicking resolver and verifies the hook dispatch
  proceeds normally with an absent context key and a `resolver.error` event
  emitted.
- Feasibility: kani-provable for the classification function; integration test
  for the wasmtime trap boundary (out of kani scope).

**VP-075 — Context-injection determinism**
- Type: postcondition
- Proof method: proptest
- Scope: SS-01, SS-04
- Property: given identical inputs (`ResolverInput { event_type, hook_event_name,
  agent_type, project_dir, plugin_config }`), a resolver returns identical
  `ResolverOutput` (same key, same value). The proptest strategy generates
  arbitrary `ResolverInput` structs (bounded string lengths) and calls the
  resolver's pure `resolve_wave_context` function twice with the same input;
  asserts outputs are identical. Applicable only to the pure computation portion
  of the resolver (file I/O is mocked via in-memory test fixtures).
- Feasibility: proptest is the right tool; determinism is trivially provable once
  file I/O is mocked. 200 trials, 5s timeout.

**VP-076 — Resolver-capability confinement**
- Type: safety
- Proof method: integration (capability-confinement integration test)
- Scope: SS-04
- Property: a resolver attempting to read a filesystem path outside its declared
  `path_allow` entries receives `CapabilityDenied` from `host::read_file` and
  does NOT observe the file contents. The integration test creates a resolver
  that attempts to read `/etc/passwd` (outside any `path_allow`); verifies the
  resolver receives an error code; verifies the dispatcher emits
  `resolver.capability_denied`; verifies no file contents appear in
  `plugin_config`.
- Feasibility: achievable via the existing capability-enforcement infrastructure
  in `crates/factory-dispatcher/src/host/read_file.rs`; the same test pattern
  used for hook capability confinement applies directly.

### PRD Delta

One new functional requirement, added to the PRD FR section and Subsystem
Registry counts:

**FR-RESOLVER-001: Factory-agnostic runtime context injection for hooks via
sandboxed WASM-plugin resolvers**

The dispatcher MUST support a resolver registry (`resolvers-registry.toml`)
containing zero or more WASM resolver plugins. When a hooks-registry entry
declares `needs_context = [resolver-name, ...]`, the dispatcher MUST invoke the
named resolvers before hook dispatch and merge their outputs into `plugin_config`.
Resolvers MUST run with capability-restricted filesystem access. A resolver
crash or error MUST NOT prevent hook dispatch. The dispatcher core MUST have
zero compile-time dependency on any per-factory resolver crate.

Subsystem Registry impact: SS-01 BC count 114→115 (+BC-1.13.001); SS-04 BC count
34→39 (+BC-4.12.001 through BC-4.12.005). Total BCs 1937→1943.

---

## 4. Affected Stories — Proposed Decomposition

### Story List

| Story ID | Title | Subsystems | Depends On |
|----------|-------|-----------|-----------|
| S-12.03 | ContextResolver trait + ResolverRegistry (in-memory) | SS-01, SS-02 | — |
| S-12.04 | WASM resolver loading + lifecycle + error isolation | SS-01, SS-04 | S-12.03 |
| S-12.05 | hook-sdk resolver-authoring extensions | SS-02 | S-12.03 |
| S-12.06 | HOST_ABI.md context-injection contract section | SS-02 | S-12.03, S-12.05 |
| S-12.07 | `vsdd-context-resolvers` crate + WaveContextResolver | SS-04 | S-12.04, S-12.05 |
| S-12.08 | Migrate convergence hook to consume `wave_context`; closes F-P2-001 | SS-04 | S-12.07 |

Total: 6 stories, IDs S-12.03 through S-12.08, appended to Epic E-12
(Engine Governance).

### Dependency Chain

The dependency structure is nearly linear with one fork:

```
S-12.06 (HOST_ABI docs — establishes the contract spec all impl stories follow)
  |
  +--- (parallel) S-12.03 (ContextResolver trait + in-memory registry)
  |                  |
  |              S-12.04 (WASM loading, depends on S-12.03)
  |                  |
  +--- (parallel) S-12.05 (hook-sdk resolver traits/macros)
                      |
                  (join at) S-12.07 (vsdd-context-resolvers crate, depends on S-12.04 + S-12.05)
                                |
                            S-12.08 (convergence hook migration, depends on S-12.07)
```

S-12.06 should be authored first as it defines the canonical contract that all
implementation stories reference. S-12.03 and S-12.05 can execute in parallel
after S-12.06. S-12.04 waits for S-12.03. S-12.07 waits for both S-12.04 and
S-12.05. S-12.08 waits for S-12.07.

### Story Summaries

**S-12.03 — ContextResolver trait + ResolverRegistry (in-memory)**

Introduce the abstract `ContextResolver` trait in `crates/factory-dispatcher/src/resolver.rs`.
Define `ResolverInput`, `ResolverOutput`, `ResolverError`, `ResolverRegistry`. This
story uses an in-memory registry backed by a `HashMap<String, Box<dyn ContextResolver>>` —
no WASM loading yet. Extend `RegistryEntry` in `registry.rs` to parse
`needs_context: Vec<String>` (optional; defaults to `[]`). Extend `executor.rs` to
call `ResolverRegistry::resolve_context(entry, &payload)` and merge into
`plugin_config`. Unit tests verify: (a) missing `needs_context` → zero resolver
calls; (b) declared resolver → output merged into plugin_config under correct key;
(c) resolver returning `None` → key absent from plugin_config; (d) unknown resolver
name → `ResolverError::NotFound` surfaced as `resolver.error` event.

Anchors: BC-1.13.001 (pre-dispatch injection), BC-4.12.001 (lifecycle), BC-4.12.005
(merge contract), VP-075 (determinism — proptest over in-memory resolver).

**S-12.04 — WASM resolver loading + lifecycle + error isolation**

Replace the in-memory registry stub from S-12.03 with WASM module loading via
wasmtime. Add `crates/factory-dispatcher/src/resolver_loader.rs`. Load
`resolvers-registry.toml` at dispatcher startup; compile each `.wasm` resolver
artifact into a `Module`; cache keyed by path + mtime. Per-dispatch: create fresh
`Store` per resolver call; invoke `resolve()` exported function; classify traps;
surface `ResolverError::Trap { cause }`. Unit tests: trap injection via
deliberately-malformed resolver WASM; timeout enforcement (fuel/epoch budget on
resolver invocations, conservatively set to 25% of hook budget). Integration test:
load real WaveContextResolver WASM (built from S-12.07) and verify round-trip.

Anchors: BC-4.12.001 (lifecycle), BC-4.12.004 (crash isolation), VP-073 (load
purity), VP-074 (error isolation kani harness + integration).

**S-12.05 — hook-sdk resolver-authoring extensions**

Add `crates/hook-sdk/src/resolver.rs`: exports `ResolverInput`, `ResolverOutput`,
`ResolverError` types with serde derives. Add `crates/hook-sdk-macros/src/resolver_macro.rs`:
`#[resolver]` macro that generates the WASI-compatible `resolve()` export from a
user-defined `fn resolve_impl(input: ResolverInput) -> ResolverOutput` function.
Add `resolver-authoring` feature flag to `hook-sdk/Cargo.toml` so hook crates
that are NOT resolvers do not pull in the extra types. Document usage in
`hook-sdk/README.md`. Unit tests: macro expansion (via `trybuild`); input/output
serde round-trip.

Anchors: BC-4.12.002 (resolver ABI), VP-075 (determinism — proptest on serde
round-trip).

**S-12.06 — HOST_ABI.md context-injection contract section**

Extend `crates/hook-sdk/HOST_ABI.md` with a new §Context Injection Contract
section. Contents: (1) what `needs_context` means — hook's declaration of
required resolver keys; (2) resolver input shape (`ResolverInput` fields); (3)
resolver output merging semantics (additive overlay, collision resolution rule);
(4) resolver capability model (`path_allow` declarations; no write_file,
exec_subprocess, emit_event); (5) failure semantics (absent key on resolver error,
no silent degrade for hook); (6) resolver ABI version (`RESOLVER_ABI_VERSION = 1`
constant; versioned independently from `HOST_ABI_VERSION`); (7) relationship to
hook `plugin_config` (resolver output is consumed-before-dispatch, not a
hook-return side channel). Cross-reference BC-4.12.001–005 and ADR-018.

This story is documentation-only. No Rust changes. Bats test: validate that
HOST_ABI.md is present, contains the §Context Injection Contract section, and
references `RESOLVER_ABI_VERSION`.

Anchors: BC-4.12.002 (ABI schema), BC-4.12.003 (capability model), BC-4.12.005
(merge contract).

**S-12.07 — `vsdd-context-resolvers` crate + WaveContextResolver**

Create `crates/vsdd-context-resolvers/` as a new factory-specific resolver crate.
Entry point: `#[resolver] fn resolve_impl(input: ResolverInput) -> ResolverOutput`
reads `input.plugin_config["project_dir"]` (injected by dispatcher from the hook's
registry entry `plugin_config.project_dir`), constructs the path to
`.factory/wave-state.yaml`, reads it via `host::read_file`, parses the active
wave's story list, and returns `ResolverOutput { key: "wave_context", value:
Some(json!({ "stories": [...], "wave_id": "...", "cycle_id": "..." })) }`.
Register in `plugins/vsdd-factory/resolvers-registry.toml` with `name =
"wave-context"`, `plugin = "hook-plugins/vsdd-context-resolvers.wasm"`, and
`path_allow = [".factory/"]`. Unit tests: parse valid wave-state; parse malformed
wave-state (returns `None`); path outside `.factory/` returns CapabilityDenied.
Integration test: capability confinement (VP-076).

Anchors: BC-4.12.001 (lifecycle), BC-4.12.002 (ABI), BC-4.12.003 (capability
model), BC-4.12.004 (error isolation), VP-073, VP-074, VP-075, VP-076.

**S-12.08 — Migrate convergence hook to consume `wave_context`; closes F-P2-001**

Update `crates/hook-plugins/validate-per-story-adversary-convergence/src/lib.rs`.
`RealCallbacks::list_stories` now reads `plugin_config["wave_context"]["stories"]`
(the key injected by WaveContextResolver via S-12.07). Remove the
`extract_stories_from_config` fallback path that returned `Err` on absent field.
Replace with: present + valid array → use stories; absent or wrong type →
`HookResult::Block` with `block_with_fix` message explaining the resolver failed
or `needs_context` is not wired. Update `hooks-registry.toml` entry for
`validate-per-story-adversary-convergence` to add `needs_context = ["wave-context"]`.
Bats integration test (end-to-end): dispatcher loads WaveContextResolver; dispatches
a SubagentStop event; hook receives `wave_context.stories`; returns correct block
decision for an unconverged story. This story formally closes F-P2-001.

Anchors: BC-4.10.001 (per-story adversary hook invariants — add invariant documenting
`needs_context` wiring), BC-4.12.001, BC-4.12.005, VP-071 (block invariant — now
correctly tested with real wave_context injection).

---

## 5. Affected Tests

### New Test Infrastructure Required

**Cargo unit tests — ContextResolver trait + ResolverRegistry (S-12.03)**
- `crates/factory-dispatcher/tests/resolver_registry_test.rs`: in-memory resolver
  mock; verify merge semantics, missing resolver error, None output behavior,
  `needs_context = []` short-circuit.

**Kani harness — VP-074 resolver-error isolation (S-12.04)**
- `crates/factory-dispatcher/src/resolver_classify_trap.rs`: pure function mapping
  `TrapCode → ResolverError`; kani harness proves all `TrapCode` variants return a
  `ResolverError` (no panics, no unreachable!).

**Proptest — VP-075 context-injection determinism (S-12.03, S-12.05)**
- `crates/factory-dispatcher/tests/resolver_determinism_proptest.rs`: proptest
  strategy over `ResolverInput { event_type: String, hook_event_name: String, ...}`;
  calls mock resolver twice with same input; asserts outputs identical. 200 trials.

**Integration tests — resolver WASM loading (S-12.04)**
- `crates/factory-dispatcher/tests/resolver_integration_test.rs`: loads
  `vsdd-context-resolvers.wasm` (built from S-12.07); dispatches synthetic
  `ResolverInput`; verifies output shape and `wave_context` key.

**Bats integration tests — end-to-end resolver → hook (S-12.08)**
- `plugins/vsdd-factory/tests/resolver-integration.bats`: spawns dispatcher with
  real `resolvers-registry.toml` + `hooks-registry.toml`; injects a synthetic
  `SubagentStop` envelope; verifies `validate-per-story-adversary-convergence`
  returns block decision on unconverged story; verifies `Continue` on converged
  story. This is the definitive F-P2-001 closure test.

**Capability-confinement integration tests — VP-076 (S-12.07)**
- `crates/vsdd-context-resolvers/tests/capability_confinement_test.rs`: WaveContext
  resolver attempts to read `/etc/passwd`; verifies `CapabilityDenied` is returned
  (not a panic); verifies no file contents leak into output.

**Trap-injection integration tests — VP-074 (S-12.04)**
- A deliberately-malformed resolver WASM that traps on `resolve()` call; integration
  test verifies dispatcher emits `resolver.error` event, hook dispatch proceeds,
  `wave_context` key absent in `plugin_config`, hook returns Block with
  block_with_fix citing absent context.

---

## 6. Regression Risk Assessment

### Risk Classification: SUBSTANTIAL

This is a platform-level addition to the dispatcher core. The following risks are
identified honestly.

**R-PLAT-001 — Dispatcher regression in hook fleet (HIGH)**
Every hook dispatch goes through `executor.rs`. Adding resolver pre-dispatch
invocation inserts a new synchronous step in the hot path. If the resolver
registry initialization has a bug, it could affect ALL hooks, not just the
convergence hook. Mitigation: S-12.03 uses an in-memory registry first (no WASM
loading); the WASM loading path (S-12.04) is introduced separately with explicit
regression tests covering the zero-resolver case. The `needs_context = []` default
means existing hooks are never affected at dispatch time — the resolver step is a
zero-cost no-op if the entry has no `needs_context` field.

**R-PLAT-002 — WASM resolver loading failure modes (HIGH)**
WASM resolver loading is new infrastructure. Failure modes include: malformed
`.wasm` artifact (compilation error); missing `.wasm` file (path not found);
resolver with ABI mismatch (wrong export signature); resolver that hangs during
compilation. Mitigation: the same mtime-cache + trap-classification patterns used
for hooks apply. The `fail_closed = true` registry option is available for
resolvers where absence should block dispatch. Comprehensive unit tests for each
failure mode in S-12.04. The dispatcher's existing non-blocking exit-0 behavior
covers the cases where `fail_closed` is not set.

**R-PLAT-003 — hooks-registry.toml schema extension backward-compat (MEDIUM)**
Adding `needs_context = ["wave-context"]` to the convergence hook entry requires
that `RegistryEntry` deserialization tolerates an unknown field. Current behavior:
`#[serde(deny_unknown_fields)]` would reject it; `#[serde(default)]` on the new
field handles it. Must verify the actual serde configuration in `registry.rs` and
add an explicit test that existing entries parse without error when the new field
is absent. If the existing code uses `deny_unknown_fields`, this is a breaking
change to the deserialization schema that must be handled with a schema_version
bump in the registry (RESOLVER_REGISTRY_SCHEMA_VERSION = 1 is new; the
`hooks-registry.toml` itself uses `schema_version = 1` already; resolver fields
in hook entries are additive optional, so no schema_version bump is required for
the hook registry — only for `resolvers-registry.toml` which is a new file with
its own versioning).

**R-PLAT-004 — Bootstrap exception flip (LOW but notable)**
S-12.07 and S-12.08 are the first stories that will go through per-story
adversarial convergence (Step 4.5) from the RIGHT side — their worktrees branch
from develop AFTER Step 4.5 went live. S-12.03 through S-12.06 are authored after
Step 4.5 but are not direct consumers of the convergence hook (that hook fires on
SubagentStop, not on wave-gate). All 6 stories require Step 4.5 per the develop
baseline. This is the intended behavior — the bootstrap exception ends with this
cycle. Story-writer must be aware that per-story-delivery for S-12.07 and S-12.08
will invoke the very convergence gate they are implementing.

**R-PLAT-005 — resolver-registry.toml not auto-discovered (LOW)**
If a factory omits `resolvers-registry.toml`, the dispatcher should not error —
it should treat this as zero resolvers. This must be specified explicitly in
BC-1.13.001 and tested. The risk is that an absent file causes a hard startup
error and blocks all hooks.

**Mitigation Summary:**
- Staged delivery: in-memory registry first (S-12.03), WASM loading second (S-12.04)
- Default-empty `needs_context` ensures zero regression for existing hooks
- Comprehensive bats end-to-end test (S-12.08) as the definitive F-P2-001 closure gate
- `resolvers-registry.toml` absent = zero resolvers (not an error) must be in
  BC-1.13.001 precondition

---

## 7. Epic Placement

Recommended placement: **append stories S-12.03 through S-12.08 to existing
Epic E-12 (Engine Governance)**.

Rationale:
- The resolver platform is a direct consequence of the same governance failure
  that motivated E-12: an engine mechanism that existed on paper but was inert
  in production. F-P2-001 is a deeper instance of the same problem (Step 4.5
  per-story adversary, convergence hook) that E-12's S-12.01 and S-12.02 address
  at the workflow and WASM-hook layers.
- Story IDs S-12.03..08 continue the natural E-12 series. No new epic file is
  needed; the existing E-12 epic spec gains a §Resolver Platform section.
- The alternative — a new Epic E-15 Resolver Platform — would fragment the
  engine-discipline narrative. The resolver platform IS engine discipline; it is
  the infrastructure that makes S-12.02's hook actually work.
- Theme coherence: E-12 is already the "close the gap between paper and
  production" epic. The resolver platform closes the final gap (data injection).

No change to E-13 (existing) or E-14 (process-gap stories S-14.01/S-14.02 from
F-P2-001 non-code findings). E-12 scope expands; E-13 and E-14 are unaffected.

---

## 8. F2-amendment / F3-amendment / F4 / F5-resumption Plan

### Phase Structure

**F2-amendment (parallel PO + architect dispatches; ~1.5-2 hr wall-clock)**

- PO writes 5 new BCs: BC-4.12.001 through BC-4.12.005 (SS-04 resolver contracts)
  + BC-1.13.001 (SS-01 pre-dispatch injection contract). PO also writes FR-RESOLVER-001
  in PRD.md.
- Architect writes ADR-018 + VP-073 through VP-076 + updates ARCH-INDEX.md
  (SS-01/SS-04 counts, ADR table), SS-01-hook-dispatcher.md (resolver modules),
  SS-04-plugin-ecosystem.md (resolver crate), SS-02-hook-sdk.md (resolver authoring).
- Architect updates BC-INDEX.md and VP-INDEX.md totals.
- State-manager runs LAST: single atomic commit per burst protocol (TD-VSDD-053);
  integrates all BC/VP/ARCH updates.

**F3-amendment (PO ratifies + story-writer authors; ~1 hr)**

- PO ratifies platform stories under E-12; adds §Resolver Platform section to
  E-12 epic spec. No new epic file.
- Story-writer authors S-12.03 through S-12.08 with full AC, tasks, and VP
  anchors per the decomposition in Section 4. Each story gets a per-story
  adversary-convergence state file initialized in the cycle directory (S-12.03/
  through S-12.08/).
- State-manager LAST integrates STORY-INDEX.

**F4-platform (per-story delivery; ~1-2 days wall-clock)**

Dependency order enforces delivery sequence. Recommended batch structure:
- Batch 1: S-12.06 (docs; no blocking dependencies; can unblock parallel impl)
- Batch 2 (parallel): S-12.03 + S-12.05 (trait + SDK; both depend only on S-12.06)
- Batch 3: S-12.04 (WASM loading; depends on S-12.03)
- Batch 4: S-12.07 (vsdd resolver; depends on S-12.04 + S-12.05)
- Batch 5: S-12.08 (convergence hook migration + F-P2-001 closure; depends on S-12.07)

All 6 stories go through full per-story-delivery: test-writer → implementer →
adversarial convergence (Step 4.5) → demo-recorder → PR → merge. S-12.07 and
S-12.08 are the first stories where Step 4.5 is exercised with the resolver
platform fully operational — the convergence gate is live for its own delivery.

**F5-resumption (pass-2 fix burst + further adversary passes)**

After S-12.08 merges:
- Pass-2 fix burst addresses the 15 pass-2 findings. F-P2-001 is formally closed
  by S-12.08. F-P2-002 (stale VP-071 wording in BC-4.10.001/BC-5.39.001) is a
  spec fix; can be bundled in the F5 fix burst. F-P2-003 through F-P2-015 proceed
  through standard fix routing.
- Pass-3 adversary review runs on the post-fix-burst state.
- Convergence target: 3 NITPICK_ONLY passes per VSDD standard.

**Wall-clock estimate to F5 convergence:**

| Phase | Activity | Estimate |
|-------|----------|---------|
| F1-amendment | This document | 30 min |
| F2-amendment | PO + architect parallel dispatch | 1.5-2 hr |
| F3-amendment | PO ratification + story authoring | 1 hr |
| F4-platform | 6 stories × per-story-delivery | 1-2 days |
| F5-resumption | Pass-2 fix burst (14 findings + F-P2-001 closure) + pass-3+ adversary cycles | 1-2 days |
| **Total** | | **~3-5 days wall-clock** |

Note: the original estimate in the prompt was 5-8 days. This revision narrows to
3-5 days because: (a) S-12.06 is a documentation-only story with no blocking
upstream; (b) S-12.03 and S-12.05 can execute in parallel; (c) the platform is
scoped to additive-only changes with zero behavioral regression for existing hooks.
The 5-8 day estimate assumed more coupling; the staged delivery plan de-risks this.

---

## 9. Open Design Decisions — NEEDS-CLARIFICATION for Human Review

Six decisions require human input before F2-amendment dispatch. Strong recommendations
per option are provided; the user selects.

**OD-1 — Resolver lifecycle: load once at startup vs. per-dispatch?**

Recommendation: **load at startup with mtime-based invalidation** (same as hook
module cache). Resolvers are cheap to call once loaded; loading per-dispatch would
add wasmtime compilation overhead to every hook invocation that has `needs_context`.
The mtime-cache pattern already proven in `plugin_loader.rs` applies directly.

Alternative (per-dispatch): simpler lifecycle (no cache invalidation); acceptable
only if resolver `.wasm` files change rarely and the compilation overhead is
acceptable. Not recommended given the dispatcher's per-invocation short-lived
process model — the cache is in-process and must be rebuilt on every dispatcher
spawn anyway. This means `load at startup` == `load on first dispatcher spawn`
per process lifetime (no daemon). Cache is relevant only when multiple hooks in
the same dispatch share the same resolver module, which is the common case.

**OD-2 — Resolver registration: extend `hooks-registry.toml` OR separate `resolvers-registry.toml`?**

Recommendation: **separate `resolvers-registry.toml`**. Resolvers are not hooks;
they have a different lifecycle (pre-dispatch data providers vs. event handlers),
a different ABI (resolve() vs. on_hook()), and different capability profiles.
Conflating them in `hooks-registry.toml` would create schema ambiguity and force
readers to mentally separate two distinct registration concerns from one file.
A separate file also enables independent schema versioning and easier future
extension.

Alternative (extend hooks-registry.toml): one file to load; simpler startup path.
Not recommended because the conceptual distinction is load-bearing for the
factory-agnostic invariant.

**OD-3 — Resolver ABI surface: reuse HookPayload-like input OR distinct ResolverInput v1?**

Recommendation: **distinct `ResolverInput` / `ResolverOutput` types, versioned as
Resolver ABI v1 alongside Hook ABI v1**. Resolvers have a different lifecycle
(no block/continue semantics; return data not decisions), different return type
(`Option<Value>` not `HookResult`), and different context requirements (they need
`project_dir` and `agent_type` but not `tool_input`/`tool_output`). Reusing
`HookPayload` would carry irrelevant fields and create semantic confusion.
Independent versioning (RESOLVER_ABI_VERSION constant) allows resolver ABI
evolution without coupling to hook ABI version bumps.

Alternative (reuse HookPayload): less code; resolvers could share `HookPayload`
deserialization. Not recommended; type reuse would couple resolver ABI evolution
to hook ABI changes.

**OD-4 — Resolver output caching: per-dispatch caching of resolver output OR always re-invoke?**

Recommendation: **no caching initially**. Resolver invocations are fast WASM
function calls (microseconds to low milliseconds for file-read + parse). Adding a
cache layer introduces invalidation complexity (what's the cache key? what's the
TTL? does agent_type matter?) with no measured need. Add cache if profiling shows
resolver invocation is a bottleneck.

Alternative (cache per dispatch context): reduces redundant `wave-state.yaml` reads
when multiple hooks in one dispatch have `needs_context = ["wave-context"]`. This
is the one compelling case. Note: in the current design, each dispatcher spawn
handles exactly one hook event (dispatcher is per-invocation); the "multiple hooks"
case means multiple plugins matched in the same dispatch. The in-process cache
within a single dispatch is the right granularity if caching is added later.

**OD-5 — Resolver composition: can resolver A's output feed resolver B?**

Recommendation: **NO initially — flat list, no inter-resolver dependencies**.
DAG-based resolver composition is powerful but adds topological sort, cycle
detection, and output-to-input threading complexity. The first use case
(WaveContextResolver) has no dependencies. Defer to a future cycle with measured
need.

Alternative (DAG composition): enables resolvers that enrich each other's outputs.
Not recommended now; implement when a concrete use case requires it.

**OD-6 — Resolver discovery: explicit registration in `resolvers-registry.toml` OR auto-discovery from filesystem?**

Recommendation: **explicit registration** (list each resolver in the TOML file
with its WASM path and `path_allow` declarations). Explicit > implicit; auto-discovery
would require a scanning convention, introduce load-order ambiguity, and make
capability declarations implicit (which resolver gets which `path_allow`?). The
registry is also the place where per-factory customization is declared; auto-
discovery would undermine the factory-agnostic invariant by requiring a naming
convention baked into the dispatcher.

Alternative (auto-discover from a `hook-plugins/resolvers/` directory): less
configuration; resolvers just need to be present. Not recommended; capability
declarations cannot be auto-inferred.

---

## 10. Architectural Concerns About the Mid-Cycle Amendment Approach

The following are honest concerns, not blockers, about the amendment approach.

**Concern A — Spec-implementation gap across the amendment boundary**

Stories S-12.01 and S-12.02 are already delivered (or in delivery). The amendment
introduces 6 new stories under the same epic that depend on platform infrastructure
those stories do not consume. If S-12.01/S-12.02 are merged to develop before
S-12.03 is authored, the BC anchors in S-12.01/S-12.02 (BC-4.10.001, BC-5.39.001)
will have a stale description of VP-071 (F-P2-002 finding). The F-P2-002 fix
(SPEC amendment) must land before S-12.03 is authored so the new stories can
reference the corrected VP-071 semantics.

Action: F-P2-002 spec fix (BC-4.10.001 + BC-5.39.001 VP-071 description update)
should be the FIRST item in the pass-2 fix burst, before any F3-amendment story
authoring begins.

**Concern B — S-12.08 is the convergence gate's own TDD story**

S-12.08's per-story adversary convergence (Step 4.5) will invoke
`validate-per-story-adversary-convergence` with WaveContextResolver wired in (from
S-12.07). This is the first production invocation of the complete platform. If the
resolver has a bug, Step 4.5 will fail during S-12.08's delivery, which is a
recursive failure. Mitigation: the bats end-to-end test in S-12.08 must pass in
the Red Gate before Step 4.5 runs. The test is the forcing function, not Step 4.5.

**Concern C — `resolvers-registry.toml` absent-file handling**

This must be specified as a precondition in BC-1.13.001 before F2-amendment
begins: absent `resolvers-registry.toml` → zero resolvers (not an error). This
is the most likely regression vector for existing deployments. The PO must make
this explicit in BC-1.13.001 PC1.

**Concern D — Hook ABI version impact**

Adding resolver-authoring types to `hook-sdk` constitutes a minor SDK API
addition. BC-2.06.001 (SDK semver bump) contracted that SDK version bumps happen
when the public API surface changes. The addition of a `resolver-authoring` feature
flag and the new `ResolverInput`/`ResolverOutput` types constitute a minor version
bump in `hook-sdk`. The PO should explicitly anchor S-12.05 to BC-2.06.001 and
include the semver bump in S-12.05's acceptance criteria.
