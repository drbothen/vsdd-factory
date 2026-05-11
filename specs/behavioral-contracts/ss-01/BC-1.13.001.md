---
document_type: behavioral-contract
level: L3
version: "1.2"
status: draft
producer: product-owner
timestamp: 2026-05-07T00:00:00Z
phase: 1a
inputs:
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/F1-platform-amendment-delta-analysis.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md
input-hash: "[pending-recompute]"
traces_to: .factory/cycles/v1.0-feature-engine-discipline-pass-1/F1-platform-amendment-delta-analysis.md
origin: greenfield
extracted_from: null
subsystem: "SS-01"
capability: "CAP-002"
lifecycle_status: active
introduced: v1.0-feature-engine-discipline-pass-1
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-1.13.001
section: "1.13"
last_amended: 2026-05-10
---

# BC-1.13.001: Dispatcher MUST load `resolvers-registry.toml` at startup and inject resolver context into `plugin_config` before each hook dispatch

## Description

The dispatcher (`crates/factory-dispatcher/`) loads `resolvers-registry.toml` at startup
using the same mtime-based cache invalidation pattern as `plugin_loader.rs`. Before each
hook dispatch, the dispatcher inspects the hooks-registry entry's `needs_context` field; for
each declared resolver name, it invokes the corresponding WASM resolver module with a
`ResolverInput` struct, merges the returned `Option<Value>` output into `plugin_config` under
the resolver's `context_key`, and then dispatches the hook with the enriched payload. If
`resolvers-registry.toml` is absent, the dispatcher starts with zero resolvers configured —
this is an expected, non-error operational state for factories that have not adopted resolvers.

## Preconditions

1. The dispatcher binary is starting up (or restarting after a WASM artifact mtime change).
2. The hooks-registry.toml file is present and valid (standard dispatcher precondition).
3. `resolvers-registry.toml` MAY or MAY NOT be present at the plugin registry path
   (`plugins/<factory>/resolvers-registry.toml` or the dispatcher's configured registry root).
   **Absence is not an error — see PC1 Critical Constraint below.**
4. A hook dispatch is being processed: the dispatcher has matched an event to a registered
   hooks-registry entry.
5. The matched hooks-registry entry has been deserialized with the `needs_context: Vec<String>`
   field (which defaults to `[]` when absent — backward-compatible with all existing entries).

### PC1 (Critical Constraint — Backward Compatibility)

**If `resolvers-registry.toml` is absent at startup, the dispatcher MUST initialize with zero
resolvers configured. This MUST NOT be a startup error. Existing deployments without the file
behave identically to before this feature.** No error is emitted, no warning is emitted to
stderr, and no hook dispatch is blocked as a result of the absent file. The dispatcher simply
treats the resolver registry as empty. This is the highest-priority backward-compatibility
invariant for the resolver platform: factory deployments that have not adopted resolvers must
be unaffected.

## Postconditions

1. **Registry loading:** The dispatcher loads and compiles all resolver WASM artifacts listed
   in `resolvers-registry.toml` at startup. The count of successfully loaded resolvers is
   written to the dispatcher log at startup (e.g., `"Loaded N context resolvers"`).
2. **Registry parse error (fail-loud):** If `resolvers-registry.toml` is present but
   malformed (TOML parse error, schema validation error, or a referenced `.wasm` path does
   not exist), the dispatcher MUST emit a `resolver.load_error` event with the specific error
   detail and MUST NOT start with a partial resolver set that silently omits the failed entry.
   The dispatcher startup fails loudly; it does not silently degrade to zero resolvers when the
   file exists but is broken.
3. **`needs_context = []` (no-op path):** If the matched hooks-registry entry has
   `needs_context: []` (or the field is absent), the dispatcher skips resolver invocation
   entirely and dispatches the hook with the unmodified `plugin_config`. This is a zero-cost
   path for all existing hooks.
4. **Resolver invocation:** For each resolver name in `needs_context`, the dispatcher invokes
   `ResolverRegistry::invoke_resolver(name, ResolverInput { event_type, hook_event_name,
   agent_type, project_dir, plugin_config })`. The invocation produces `ResolverOutput { key,
   value: Option<Value> }` or a `ResolverError`.
5. **Merge into `plugin_config`:** Each resolver's output is merged into `plugin_config` under
   the resolver's declared `context_key`. The static `plugin_config` from the hooks-registry
   entry is preserved; resolver outputs are overlaid additively (resolver output wins on key
   collision per the merge contract in BC-4.12.005). The hook sees the merged `plugin_config`.
6. **Unknown resolver name (fail-loud at dispatch):** If a `needs_context` entry names a
   resolver that is not registered in `resolvers-registry.toml`, the dispatcher MUST emit a
   `resolver.not_found` event with the hook name and the missing resolver name, and MUST NOT
   silently inject empty context. The hook dispatch proceeds without the missing context; the
   hook is responsible for treating the absent key as an error if the context is required.
7. **Resolver invocation order:** Resolvers in `needs_context` are invoked in declaration order.
   The merge is applied in the same order (first resolver's output is merged first).
8. **Hook receives enriched payload:** The `invoke_plugin` call receives the fully merged
   `plugin_config` — including all resolver outputs — as its `plugin_config` field. The hook
   plugin has no visibility into whether its `plugin_config` was enriched by resolvers or came
   entirely from the static registry config.

## Invariants

1. **Factory-agnostic dispatcher:** The dispatcher core (`crates/factory-dispatcher/`) has
   zero compile-time dependency on any per-factory resolver crate. All resolver logic lives
   in WASM plugins loaded at runtime. The dispatcher knows only the `ResolverInput` /
   `ResolverOutput` ABI (BC-4.12.002), not the semantic meaning of any resolver's output.
2. **Absent registry = zero resolvers (not error):** `resolvers-registry.toml` absent ALWAYS
   yields zero resolvers and NEVER yields a startup error. This invariant is non-negotiable;
   any code path that converts a missing resolver registry file into a hard error violates this
   BC and must be treated as a regression.
3. **`needs_context` defaults to empty:** The `needs_context` field on `RegistryEntry` in
   `registry.rs` MUST use `#[serde(default)]` so that existing `hooks-registry.toml` entries
   without the field parse without error. No `deny_unknown_fields` constraint may be added to
   `RegistryEntry` without a schema_version bump and migration path.
4. **Resolver loading at startup, not per-dispatch:** Resolvers are compiled into `Module`
   objects once per dispatcher lifetime (mtime-cache per BC-4.12.001). Each dispatch creates
   a fresh `Store` per resolver invocation. The compilation cost is amortized; dispatch
   latency for resolver invocation is limited to `Store` creation + WASM function execution.
5. **Context injection precedes `invoke_plugin`:** The resolver invocation and merge step
   MUST complete before `invoke_plugin` is called. Hooks MUST see the fully merged
   `plugin_config`; partial injection is not permitted.
6. **EXPLICIT registry only (no auto-discovery):** The dispatcher MUST NOT scan filesystem
   directories for WASM resolver artifacts. Only resolvers explicitly listed in
   `resolvers-registry.toml` are loaded. This is a load-bearing invariant for the factory-
   agnostic design: auto-discovery would require naming conventions baked into the dispatcher.
7. **Separate registry file:** `resolvers-registry.toml` is a distinct file from
   `hooks-registry.toml`. The two files have different schemas, different lifecycle roles
   (pre-dispatch data providers vs. event handlers), and are versioned independently. The
   dispatcher loads both files at startup.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `resolvers-registry.toml` absent at startup | Zero resolvers configured. Dispatcher starts normally. No error, no warning. All existing hooks function identically to pre-resolver behavior. |
| EC-002 | `resolvers-registry.toml` present but TOML parse error | Fail-loud: emit `resolver.load_error` with parse error detail. Dispatcher startup fails. Do not silently proceed with zero resolvers. |
| EC-003 | `resolvers-registry.toml` present; a `plugin` path does not exist on disk | Fail-loud: emit `resolver.load_error` with the missing path. Startup fails for that resolver entry. |
| EC-004 | Hook entry has `needs_context = ["wave_context"]` but `wave_context` is not in the resolver registry | Emit `resolver.not_found` event at dispatch time. Dispatch proceeds without context injection. Hook sees absent key in `plugin_config`. |
| EC-005 | Hook entry has `needs_context = []` (or field absent) | Resolver invocation skipped entirely. Zero overhead on the dispatch hot path. |
| EC-006 | Resolver WASM mtime changes while dispatcher is running | Mtime-based cache invalidation triggers reload of the changed resolver module on next dispatch that needs it (same pattern as `plugin_loader.rs`). |
| EC-007 | Two hooks in the same dispatch share the same `needs_context` resolver | Each hook's dispatch independently invokes the resolver (no cross-hook caching per OD-4). Each invocation creates a fresh `Store`. |
| EC-008 | `resolvers-registry.toml` has zero `[[resolvers]]` entries | Equivalent to absent file: zero resolvers configured. Valid state; not an error. |
| EC-009 | Resolver returns `None` for its `value` field | Key is NOT written to `plugin_config`. The key is absent from the hook's `plugin_config`. The hook must treat the absent key as appropriate for its logic. |

## Canonical Test Vectors

| Scenario | Registry State | `needs_context` | Expected Behavior |
|----------|---------------|-----------------|-------------------|
| Registry absent | File not found | any | Zero resolvers; dispatcher starts. Hooks dispatch normally. |
| Registry present; no resolvers | `[[resolvers]]` empty | `["wave_context"]` | `resolver.not_found` event; hook proceeds with unmodified `plugin_config`. |
| Registry present; resolver loaded | `wave_context` registered | `["wave_context"]` | Resolver invoked; output merged into `plugin_config["wave_context"]`; hook sees merged config. |
| Registry present; resolver loaded | `wave_context` registered | `[]` | Resolver NOT invoked; hook sees unmodified `plugin_config`. Zero overhead. |
| Resolver returns `None` | `wave_context` registered | `["wave_context"]` | `plugin_config["wave_context"]` key is absent (not null, not empty). |
| Resolver returns value | `wave_context` registered; returns `{stories: [...]}` | `["wave_context"]` | `plugin_config["wave_context"] = {stories: [...]}`. |
| TOML parse error | Malformed TOML | — | Startup fails; `resolver.load_error` emitted. |
| Unknown resolver name | `foo` not registered | `["foo"]` | `resolver.not_found`; dispatch proceeds without context. |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-073 | Resolver-load purity — loading a `.wasm` resolver artifact is deterministic and has no observable side effects at load time | unit-test (integration test of resolver module compilation) |
| VP-074 | Resolver-error isolation — a resolver crash or trap does not propagate to the dispatcher process | kani (pure error-classification logic) + integration test (trap injection) |
| VP-075 | Context-injection determinism — identical `ResolverInput` yields identical `ResolverOutput` | proptest (200 trials, 5s timeout) |
| (unit-test) | Absent `resolvers-registry.toml` yields zero resolvers and no startup error | Rust unit test |
| (unit-test) | `needs_context = []` skips resolver invocation (zero overhead path) | Rust unit test (assert resolver mock not called) |
| (unit-test) | `needs_context` with unknown resolver name emits `resolver.not_found` and does not panic | Rust unit test |
| (unit-test) | Resolver output merged into `plugin_config` under correct key | Rust unit test |
| (unit-test) | Resolver `None` output leaves key absent (not present-but-null) from `plugin_config` | Rust unit test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-002 |
| Capability Anchor Justification | CAP-002 ("Hook Claude Code tool calls and session/worktree lifecycle events with sandboxed WASM plugins") per capabilities.md §CAP-002 — this BC governs the dispatcher core's pre-dispatch context-injection mechanism, which is the runtime wiring that makes WASM hook plugins receive enriched `plugin_config` at dispatch time. CAP-002 defines the full dispatch pipeline from Claude Code lifecycle events through the sandboxed WASM plugin invocation; context injection via resolvers is an extension of that pipeline, operating at the dispatcher layer (SS-01) before `invoke_plugin` is called. The resolver platform is factory-agnostic infrastructure within CAP-002's sandboxed plugin dispatch model. |
| Secondary Capability Reference | CAP-009 ("Author and publish WASM hook plugins using the Rust SDK") per capabilities.md §CAP-009 — resolver plugins are authored using the SDK's `resolver-authoring` feature flag (BC-4.12.002); CAP-009 governs the SDK surface used by resolver authors. |
| L2 Domain Invariants | none |
| Architecture Module | `crates/factory-dispatcher/src/resolver.rs` (ContextResolver trait, ResolverRegistry); `crates/factory-dispatcher/src/resolver_loader.rs` (WASM module loading + mtime-cache); `crates/factory-dispatcher/src/executor.rs` (pre-dispatch resolver invocation); `crates/factory-dispatcher/src/main.rs` (resolvers-registry.toml load at startup); `crates/factory-dispatcher/src/registry.rs` (RegistryEntry.needs_context field) |
| Stories | S-12.03, S-12.04, S-12.06, S-12.08 |
| FR | FR-RESOLVER-001 (factory-agnostic runtime context injection for hooks via sandboxed WASM-plugin resolvers) |
| ADR Reference | ADR-018 (WASM-plugin Context Resolvers — Design and Layering) — codifies the separate registry, factory-agnostic dispatcher, and explicit-registration decisions (OD-1 through OD-6) that this BC encodes as behavioral contracts. |

## Related BCs

- BC-4.12.001 — composes with (resolver lifecycle invariant — loaded once at startup with mtime-cache; this BC describes the startup loading step)
- BC-4.12.002 — composes with (resolver ABI and payload schema — defines `ResolverInput` and `ResolverOutput` types used in resolver invocation)
- BC-4.12.003 — composes with (resolver capability model — capability declarations are read at registry-load time)
- BC-4.12.004 — composes with (resolver error and crash isolation — error handling for failed resolver invocations)
- BC-4.12.005 — composes with (context-injection merging contract — defines how resolver outputs are merged into `plugin_config`)
- BC-1.12.001 — sibling (dispatcher startup and registry loading — this BC extends startup with the resolver registry step)

## Architecture Anchors

- `/Users/jmagady/Dev/vsdd-factory/crates/factory-dispatcher/src/resolver.rs` — ContextResolver trait, ResolverRegistry, ResolverInput, ResolverOutput, ResolverError types
- `/Users/jmagady/Dev/vsdd-factory/crates/factory-dispatcher/src/resolver_loader.rs` — WASM module compilation + mtime-cache
- `/Users/jmagady/Dev/vsdd-factory/crates/factory-dispatcher/src/executor.rs` — pre-dispatch resolver invocation step (between registry lookup and invoke_plugin)
- `/Users/jmagady/Dev/vsdd-factory/crates/factory-dispatcher/src/registry.rs` — RegistryEntry.needs_context field (`#[serde(default)]`)
- `/Users/jmagady/Dev/vsdd-factory/plugins/vsdd-factory/resolvers-registry.toml` — resolver registration file (NEW; distinct from hooks-registry.toml)
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/architecture/decisions/ADR-018-wasm-plugin-context-resolvers.md` — design decision

## Story Anchor

S-12.03 (ContextResolver trait + ResolverRegistry in-memory) and S-12.04 (WASM resolver loading + lifecycle) — v1.0-feature-engine-discipline-pass-1 F3-amendment decomposition.

## VP Anchors

- VP-073 — Resolver-load purity
- VP-074 — Resolver-error isolation
- VP-075 — Context-injection determinism

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.2 | 2026-05-10 | Pass-4 fix-burst: canonical key wave-context → wave_context per BC-4.12.005 PC7 / S-12.07 v1.2 / ADR-018. EC-004 and Canonical Test Vectors truth table (rows 150-154) updated to use underscore form throughout. Added missing `extracted_from: null` frontmatter field (greenfield artifact). |
| 1.1 | 2026-05-09 | F-P45-001 — Traceability Stories row propagated from BC-INDEX v1.57: S-12.03, S-12.04 → S-12.03, S-12.04, S-12.06, S-12.08. BC-INDEX was updated in fix-burst-39 (v1.55) to add S-12.06 + S-12.08; body was not updated in that burst. Refs: F-P45-001, fix-burst-42. |
| 1.0 | 2026-05-07 | Initial authoring (product-owner; F2-amendment phase of v1.0-feature-engine-discipline-pass-1). Encodes architectural decisions OD-1 through OD-6 (user-authorized per D-361). PC1 Critical Constraint explicitly states "absent resolvers-registry.toml = zero resolvers, NOT a startup error" per orchestrator directive and F1-amendment R-PLAT-005 regression risk mitigation. Factory-agnostic dispatcher invariant encodes D-361 generality requirement. |
