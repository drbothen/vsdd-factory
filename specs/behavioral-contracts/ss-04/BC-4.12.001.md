---
document_type: behavioral-contract
level: L3
version: "1.1"
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
subsystem: "SS-04"
capability: "CAP-009"
lifecycle_status: active
introduced: v1.0-feature-engine-discipline-pass-1
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-4.12.001
section: "4.12"
last_amended: 2026-05-07
---

# BC-4.12.001: Resolver WASM modules MUST be loaded once at dispatcher startup and held in-process for the lifetime of the dispatcher session with mtime-based cache invalidation

## Description

Resolver WASM artifacts are compiled into `Module` objects once at dispatcher startup via
`crates/factory-dispatcher/src/resolver_loader.rs`. The module cache is keyed by file path
plus mtime (the same cache-invalidation pattern as `plugin_loader.rs`). Each hook dispatch
that requires a resolver creates a fresh `Store` per resolver invocation — module compilation
is amortized across dispatches, but store creation is per-call to ensure isolation. If a
resolver's `.wasm` artifact mtime changes while the dispatcher is running, the module is
recompiled on the next invocation that needs it.

## Preconditions

1. The dispatcher has loaded `resolvers-registry.toml` and identified all registered resolver
   entries (per BC-1.13.001 startup contract).
2. Each resolver entry specifies a `plugin` path pointing to a `.wasm` artifact on disk.
3. The wasmtime runtime is available and configured identically to the hook plugin runtime
   (same `Engine` configuration, same fuel/epoch settings used for resolvers).

## Postconditions

1. **Load-once semantics:** Each resolver `.wasm` artifact is compiled into a `Module` exactly
   once per startup (or once per mtime change). Subsequent dispatches that invoke the same
   resolver reuse the cached `Module`; they do NOT re-read or re-compile the WASM file.
2. **Per-dispatch Store isolation:** Each resolver invocation creates a fresh `Store<HostContext>`
   (not a fresh `Module`). The `Store` is created, used for one `resolve()` invocation, and
   dropped. No state persists between invocations via the `Store`.
3. **Mtime-based invalidation:** The cache entry for a resolver module is keyed by
   `(canonical_path, mtime)`. On each dispatch, the resolver_loader checks the current mtime
   of the `.wasm` file. If the mtime has changed since the module was cached, the old module
   is evicted and the artifact is recompiled into a new `Module`.
4. **Determinism across dispatches:** Given the same `.wasm` bytes (same mtime), invoking a
   resolver with identical `ResolverInput` MUST produce identical `ResolverOutput`. The
   module is deterministic; per-call non-determinism is limited to I/O performed via host
   functions (which are themselves deterministic for the same filesystem state).
5. **Dispatcher log at startup:** The count of successfully compiled resolver modules is
   emitted to the dispatcher log at startup level (e.g., `"Compiled N resolver modules from
   resolvers-registry.toml"`).
6. **Failed module compilation:** If a resolver `.wasm` artifact fails to compile
   (`Module::from_file` error), the dispatcher emits `resolver.load_error` with the resolver
   name and the compilation error detail, then fails startup (unless the resolver entry has
   `fail_closed = false`, in which case the entry is skipped with a warning and the
   dispatcher starts with the remaining resolvers).

## Invariants

1. **Compilation is startup-time only (not per-dispatch):** Under no circumstances is a
   resolver module compiled during the dispatch hot path. Any code path that compiles a WASM
   resolver inside `executor.rs`'s dispatch loop (rather than in `resolver_loader.rs` at
   startup or on-mtime-change) violates this invariant.
2. **Store-per-call isolation:** Resolver invocations MUST NOT share a `Store` across
   calls. Shared store would allow cross-invocation state leakage through WASM memory.
3. **Same Engine instance:** Resolver modules MUST be compiled with the same `Engine`
   instance used for hook plugins (or a structurally identical `Engine` config). Mixing
   engine configurations is a correctness risk.
4. **Mtime check is on every dispatch:** The resolver_loader performs the mtime check on
   every dispatch that needs the resolver, not on a periodic timer. This ensures that a
   resolver update takes effect on the next dispatch after the file changes, not after an
   arbitrary interval.
5. **No session-level caching of `ResolverOutput`:** Resolver output is NOT cached between
   invocations (OD-4). A resolver producing the same output for the same dispatch context is
   the resolver's own property (VP-075), not a caching guarantee.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Resolver `.wasm` file is deleted after startup | Next dispatch that needs this resolver: mtime check fails (file not found). Emit `resolver.load_error`. Dispatch proceeds without that resolver's context (hook sees absent key). |
| EC-002 | Resolver `.wasm` file mtime changes between two dispatches | Old module evicted from cache; artifact recompiled on next dispatch. If compilation fails, emit `resolver.load_error`. |
| EC-003 | Two concurrent hook dispatches need the same resolver | Both dispatches hit the module cache concurrently. Cache lookup must be thread-safe (Mutex or Arc). Each dispatch creates its own `Store` independently. |
| EC-004 | Resolver `.wasm` artifact is 0 bytes or truncated | `Module::from_file` returns an error. `resolver.load_error` emitted. Startup fails (or entry skipped if `fail_closed = false`). |
| EC-005 | Resolver WASM binary exceeds memory limit | Configured engine fuel/epoch budget limits resolver execution. Module compilation itself is not fuel-limited; execution is. |
| EC-006 | Same resolver artifact referenced by two registry entries | Each registry entry creates its own cache key (path + mtime). Deduplication is implementation-defined; functionally they produce separate modules (or the same module if the loader deduplicates on path). |

## Canonical Test Vectors

| Scenario | Mtime Change | Expected Module Behavior |
|----------|-------------|--------------------------|
| First dispatch after startup | No | Module compiled once; cached; returned from cache on all subsequent dispatches. |
| Second dispatch, same artifact | No | Cache hit; `Module::from_file` NOT called again. |
| Dispatch after artifact mtime change | Yes | Cache miss; artifact recompiled; new module returned. |
| Dispatch after artifact deleted | File gone | `resolver.load_error` emitted; dispatch proceeds without resolver output. |
| Two dispatches with same resolver, concurrent | No | Both get the same cached `Module` (thread-safe read); each creates its own `Store`. |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-073 | Resolver-load purity — `Module::from_file` is deterministic and has no observable side effects at load time; given same `.wasm` bytes, two sequential `get_or_compile` calls return structurally identical modules | unit-test (integration test of resolver module compilation) |
| VP-074 | Resolver-error isolation — resolver trap/crash does not propagate to dispatcher process | kani (pure error-classification logic) + integration test (trap injection) |
| (unit-test) | Module is compiled exactly once per startup (cache hit on second dispatch) | Rust unit test (mock loader; assert compile called once) |
| (unit-test) | Mtime change triggers cache eviction and recompile | Rust unit test (inject mtime change) |
| (unit-test) | Store is created per-invocation, not per-module | Rust unit test (assert separate Store per call) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-009 |
| Capability Anchor Justification | CAP-009 ("Author and publish WASM hook plugins using the Rust SDK") per capabilities.md §CAP-009 — this BC governs the lifecycle of WASM resolver plugins within the dispatcher runtime. Resolver plugins are a class of WASM artifacts authored using the `vsdd-hook-sdk`'s `resolver-authoring` feature (via the `#[resolver]` macro and `ResolverInput`/`ResolverOutput` types defined in `hook-sdk/src/resolver.rs`). CAP-009 defines the SDK as the interface through which plugin authors implement resolver behavior; this BC specifies how those compiled artifacts are loaded, cached, and managed by the dispatcher across invocations. |
| L2 Domain Invariants | none |
| Architecture Module | `crates/factory-dispatcher/src/resolver_loader.rs` (WASM module loading + mtime-cache); `crates/factory-dispatcher/src/resolver.rs` (ResolverRegistry, per-dispatch Store creation) |
| Stories | S-12.04, S-12.06, S-12.07 |
| FR | FR-RESOLVER-001 (factory-agnostic runtime context injection for hooks via sandboxed WASM-plugin resolvers) |
| ADR Reference | ADR-018 (WASM-plugin Context Resolvers — OD-1: load at startup with mtime-based invalidation) |

## Related BCs

- BC-1.13.001 — depends on (dispatcher startup contract; this BC specifies the lifecycle behavior that BC-1.13.001's loading step must fulfill)
- BC-4.12.002 — composes with (resolver ABI — the `resolve()` function invoked per-dispatch using the per-invocation Store)
- BC-4.12.004 — composes with (crash isolation — if a resolver traps during a per-dispatch Store invocation, BC-4.12.004 governs the error handling)

## Architecture Anchors

- `/Users/jmagady/Dev/vsdd-factory/crates/factory-dispatcher/src/resolver_loader.rs` — WASM module loading, mtime-cache, `get_or_compile` function
- `/Users/jmagady/Dev/vsdd-factory/crates/factory-dispatcher/src/resolver.rs` — ResolverRegistry, per-invocation Store creation
- `/Users/jmagady/Dev/vsdd-factory/crates/factory-dispatcher/src/plugin_loader.rs` — reference implementation pattern for mtime-cache (same pattern applied to resolvers)
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/architecture/decisions/ADR-018-wasm-plugin-context-resolvers.md` — OD-1 lifecycle decision

## Story Anchor

S-12.04 — WASM resolver loading + lifecycle + error isolation (v1.0-feature-engine-discipline-pass-1 F3-amendment).

## VP Anchors

- VP-073 — Resolver-load purity
- VP-074 — Resolver-error isolation

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.0 | 2026-05-07 | Initial authoring (product-owner; F2-amendment phase of v1.0-feature-engine-discipline-pass-1). Encodes OD-1 (load-at-startup with mtime-based invalidation). Mtime-cache pattern sourced from `plugin_loader.rs` per F1-amendment delta analysis. No output caching (OD-4). |
| 1.1 | 2026-05-09 | F-P45-001 — Traceability Stories row propagated from BC-INDEX v1.57: S-12.04 → S-12.04, S-12.06, S-12.07. BC-INDEX was updated in fix-burst-39 (v1.55) to add S-12.06 + S-12.07; body was not updated in that burst. Refs: F-P45-001, fix-burst-42. |
