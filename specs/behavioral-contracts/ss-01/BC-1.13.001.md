---
document_type: behavioral-contract
level: L3
version: "1.3"
status: draft
producer: product-owner
timestamp: 2026-05-07T00:00:00Z
phase: 1a
inputs:
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/F1-platform-amendment-delta-analysis.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md
  - .factory/specs/architecture/decisions/ADR-024-dispatcher-log-dir-resolution-and-plugin-root-fail-loud.md
input-hash: "4a88bec"
traces_to: .factory/cycles/v1.0-feature-engine-discipline-pass-1/F1-platform-amendment-delta-analysis.md
origin: greenfield
extracted_from: null
subsystem: "SS-01"
capability: "CAP-002"
lifecycle_status: active
introduced: v1.0-feature-engine-discipline-pass-1
modified:
  - "2026-06-22 (v1.3) — S-18.14 spec-evolution (D-676 / ADR-024 v1.3): INV-8 (resolver WASM path resolution base must be TOML parent dir); PC-9 (successful load when artifacts present at TOML-parent-relative path); PC-10 (log_dir field in dispatcher.started payload — placed here because no dedicated dispatcher.started payload BC exists; ADR-024 Decision 5); EC-010 (relative WASM path exists at PLUGIN_ROOT but not CWD → must load successfully). ADR Reference updated to cite ADR-024 v1.3. Changelog and Architecture Anchors extended."
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-1.13.001
section: "1.13"
last_amended: "2026-06-22"
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
9. **PC-9 (Successful load when artifacts present):** When `resolvers-registry.toml` is present
   and valid and the referenced WASM artifacts exist at TOML-parent-relative paths, the dispatcher
   MUST load all declared resolvers with zero `resolver.load_error` events for any declared
   resolver. A `resolver.load_error` for a resolver whose WASM file exists at the TOML-parent-
   relative path is a specification violation. (Anchor: see INV-8 below; root-cause fix for the
   empirically observed 8,560 `resolver.load_error` / 0 successful loads since rc.21.)
10. **PC-10 (`log_dir` observability in `dispatcher.started`):** The `dispatcher.started` event
    payload MUST include a `log_dir` string field whose value is the absolute path to the
    dispatcher's internal log directory for this invocation, populated from `InternalLog::log_dir()`
    — the accessor already present on the `InternalLog` struct. The field is emitted
    unconditionally on every startup, is NOT optional, is NOT behind a feature flag, and MUST
    NOT be null or empty. The value is the fully-resolved absolute path produced by the seven-level
    resolution algorithm in ADR-024 Decision 1.
    > **Placement note:** No dedicated `dispatcher.started`-payload BC exists in the SS-01 catalog
    > (BC-1.06.007 and BC-1.06.009 govern brownfield `InternalLog` JSONL envelope shape; BC-1.12.001
    > cites `dispatcher.started` only as a test-vector example of lifecycle routing). This
    > postcondition is placed here because ADR-024 Decision 5 is architecturally coupled to the
    > resolver-platform startup path (same `main.rs` startup sequence), and S-18.14 is the
    > implementing story. If a dedicated `dispatcher.started` payload BC is authored in a future
    > cycle, PC-10 should migrate there; cite this BC as the origin in that migration.

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
8. **INV-8 (Resolver WASM path resolution base):** Relative `plugin` paths in
   `resolvers-registry.toml` (e.g., `plugin = "hook-plugins/vsdd-context-resolvers.wasm"`)
   MUST be resolved against the TOML file's parent directory — which equals `CLAUDE_PLUGIN_ROOT`
   at runtime — NOT against the dispatcher's process working directory (CWD). Absolute `plugin`
   paths pass through unchanged (no re-joining). The resolution MUST be applied in
   `resolver_loader::load_registry` at every call site to `get_or_compile`, for both
   `fail_closed: true` and `fail_closed: false` code paths. A `resolver.load_error` for a
   resolver whose WASM file exists at the correct TOML-parent-relative path is a violation of
   this invariant and MUST be treated as a regression.
   > **Why CWD-relative is wrong:** The dispatcher is invoked by the Claude Code hook
   > infrastructure with CWD set to the host project directory (e.g., `/Users/<user>/project/`).
   > WASM plugin files live under `CLAUDE_PLUGIN_ROOT` (e.g.,
   > `~/.claude/plugins/cache/claude-mp/vsdd-factory/<version>/`). A relative path from the
   > TOML resolves correctly only when the base is the TOML's own parent directory. CWD as
   > base yields a path that does not exist, causing `path.canonicalize()` to return `Err(ENOENT)`
   > — the root cause of 8,560 `resolver.load_error` / 0 successful loads observed since rc.21.
   > (Anchor: `resolver_loader::load_registry` joins `toml_path.parent()` with `entry.plugin`
   > before `get_or_compile`; function-name anchor per TD-VSDD-091.)

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
| EC-010 | `resolvers-registry.toml` present; `plugin` path relative (e.g., `hook-plugins/vsdd-context-resolvers.wasm`); WASM file exists at `CLAUDE_PLUGIN_ROOT/<rel>` but NOT at `<CWD>/<rel>` | Resolver MUST load successfully (TOML-parent-relative resolution per INV-8 wins). Zero `resolver.load_error` events for this resolver. The CWD-relative path's non-existence is irrelevant. A CWD-relative resolution attempt that produces `path.canonicalize()` `Err(ENOENT)` is the bug that INV-8 and PC-9 are designed to prevent. |

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
| Relative WASM path; file at PLUGIN_ROOT, absent at CWD | `wave_context` registered with relative path; WASM exists at TOML-parent dir, not at process CWD | `["wave_context"]` | Resolver loads successfully (zero `resolver.load_error`); context injected into `plugin_config`. CWD non-existence irrelevant. (Witnesses INV-8 / PC-9 / EC-010.) |
| `dispatcher.started` event | Dispatcher starts with valid `resolvers-registry.toml` | — | `dispatcher.started` event payload includes `log_dir` string field (non-empty absolute path) populated from `InternalLog::log_dir()`. (Witnesses PC-10.) |

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
| ADR Reference | ADR-018 (WASM-plugin Context Resolvers — Design and Layering) — codifies the separate registry, factory-agnostic dispatcher, and explicit-registration decisions (OD-1 through OD-6) that this BC encodes as behavioral contracts. ADR-024 v1.3 §Decision 1 Addendum (Resolver WASM plugin path resolution) — establishes that relative `plugin` paths MUST resolve against `toml_path.parent()`, not CWD; functional anchor is `resolver_loader::load_registry` (TD-VSDD-091). ADR-024 v1.3 §Decision 5 (`log_dir` observability) — contracts that `dispatcher.started` payload MUST include `log_dir` from `InternalLog::log_dir()`; see PC-10. |

## Related BCs

- BC-4.12.001 — composes with (resolver lifecycle invariant — loaded once at startup with mtime-cache; this BC describes the startup loading step)
- BC-4.12.002 — composes with (resolver ABI and payload schema — defines `ResolverInput` and `ResolverOutput` types used in resolver invocation)
- BC-4.12.003 — composes with (resolver capability model — capability declarations are read at registry-load time)
- BC-4.12.004 — composes with (resolver error and crash isolation — error handling for failed resolver invocations)
- BC-4.12.005 — composes with (context-injection merging contract — defines how resolver outputs are merged into `plugin_config`)
- BC-1.12.001 — sibling (dispatcher startup and registry loading — this BC extends startup with the resolver registry step; PC-10 `log_dir` in `dispatcher.started` is also anchored here because BC-1.12.001 contracts that lifecycle events route to `events-*.jsonl`)

## Architecture Anchors

- `crates/factory-dispatcher/src/resolver.rs` — ContextResolver trait, ResolverRegistry, ResolverInput, ResolverOutput, ResolverError types
- `crates/factory-dispatcher/src/resolver_loader.rs` — WASM module compilation + mtime-cache; **INV-8 change site**: `load_registry` MUST join `toml_path.parent()` with `entry.plugin` for relative paths before passing to `get_or_compile` and `path.canonicalize()`; applies to ALL `get_or_compile` call sites in this file (both `fail_closed: true` and `fail_closed: false` paths)
- `crates/factory-dispatcher/src/executor.rs` — pre-dispatch resolver invocation step (between registry lookup and invoke_plugin)
- `crates/factory-dispatcher/src/registry.rs` — RegistryEntry.needs_context field (`#[serde(default)]`)
- `crates/factory-dispatcher/src/main.rs` — **PC-10 change site**: `InternalLog::write_started` call MUST include `log_dir` field from `InternalLog::log_dir()` in the `dispatcher.started` payload (ADR-024 Decision 5); also the startup entry point that calls `load_registry` where INV-8 path resolution begins
- `plugins/vsdd-factory/resolvers-registry.toml` — resolver registration file (distinct from hooks-registry.toml)
- `.factory/specs/architecture/decisions/ADR-018-wasm-plugin-context-resolvers.md` — design decision (OD-1 through OD-6)
- `.factory/specs/architecture/decisions/ADR-024-dispatcher-log-dir-resolution-and-plugin-root-fail-loud.md` — v1.3 Decision 1 Addendum (TOML-parent-relative path resolution) and Decision 5 (`log_dir` in `dispatcher.started`)

## Story Anchor

S-12.03 (ContextResolver trait + ResolverRegistry in-memory) and S-12.04 (WASM resolver loading + lifecycle) — v1.0-feature-engine-discipline-pass-1 F3-amendment decomposition.

## VP Anchors

- VP-073 — Resolver-load purity
- VP-074 — Resolver-error isolation
- VP-075 — Context-injection determinism

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.3 | 2026-06-22 | S-18.14 spec-evolution (D-676 / ADR-024 v1.3): (1) INV-8 — Resolver WASM path resolution base MUST be TOML file's parent directory (`CLAUDE_PLUGIN_ROOT` at runtime), NOT process CWD; absolute paths pass through unchanged; applies to ALL `get_or_compile` call sites in `resolver_loader`; root cause of 8,560 `resolver.load_error` / 0 successful loads since rc.21. (2) PC-9 — Successful load when artifacts present at TOML-parent-relative paths; zero `resolver.load_error` for any declared resolver is the spec. (3) PC-10 — `dispatcher.started` event payload MUST include `log_dir` string field from `InternalLog::log_dir()`; unconditional; non-empty; absolute path (ADR-024 Decision 5). Placed here because no dedicated `dispatcher.started`-payload BC exists in the SS-01 catalog; see PC-10 placement note for migration guidance. (4) EC-010 — Relative WASM exists at PLUGIN_ROOT but not CWD → must load successfully. (5) Architecture Anchors updated: path-anchors migrated from absolute user-local paths to repo-relative paths (TD-VSDD-091); `resolver_loader.rs` INV-8 change site and `main.rs` PC-10 change site documented. (6) ADR Reference extended with ADR-024 v1.3 §Decision 1 Addendum and §Decision 5 cites. |
| 1.2 | 2026-05-10 | Pass-4 fix-burst: canonical key wave-context → wave_context per BC-4.12.005 PC7 / S-12.07 v1.2 / ADR-018. EC-004 and Canonical Test Vectors truth table (rows 150-154) updated to use underscore form throughout. Added missing `extracted_from: null` frontmatter field (greenfield artifact). |
| 1.1 | 2026-05-09 | F-P45-001 — Traceability Stories row propagated from BC-INDEX v1.57: S-12.03, S-12.04 → S-12.03, S-12.04, S-12.06, S-12.08. BC-INDEX was updated in fix-burst-39 (v1.55) to add S-12.06 + S-12.08; body was not updated in that burst. Refs: F-P45-001, fix-burst-42. |
| 1.0 | 2026-05-07 | Initial authoring (product-owner; F2-amendment phase of v1.0-feature-engine-discipline-pass-1). Encodes architectural decisions OD-1 through OD-6 (user-authorized per D-361). PC1 Critical Constraint explicitly states "absent resolvers-registry.toml = zero resolvers, NOT a startup error" per orchestrator directive and F1-amendment R-PLAT-005 regression risk mitigation. Factory-agnostic dispatcher invariant encodes D-361 generality requirement. |
