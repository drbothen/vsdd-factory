---
document_type: adr
adr_id: ADR-010
status: accepted
date: 2026-04-26
subsystems_affected: [SS-01, SS-02]
supersedes: null
superseded_by: null
---

# ADR-010: StoreData-Typed Linker for Host Functions (invoke.rs Pattern)

## Context

wasmtime's `Linker<T>` type is generic over the store data type `T`. Every host
function registered with a linker has access to the store's data through a
`Caller<'_, T>` argument. This creates a binding constraint: the linker's type
parameter must match the store's type parameter.

The v1.0 dispatcher has two sources of host state that must coexist in a single
wasmtime `Store` during plugin invocation:

1. `HostContext` — the dispatcher-level per-invocation state (plugin name, session ID,
   capabilities, event queue, internal log reference). Defined in
   `crates/factory-dispatcher/src/host/mod.rs`. This is the type used by the host
   function implementations in `crates/factory-dispatcher/src/host/` submodules
   (`emit_event`, `exec_subprocess`, `read_file`, `log`, `env`, etc.).

2. `WasiP1Ctx` — the WASI preview 1 context (stdin/stdout/stderr pipes, preopened
   dirs, env vars). Required by `wasmtime_wasi::p1::add_to_linker_sync`, which takes
   a closure mapping `&mut T` to `&mut WasiP1Ctx`.

wasmtime does not support a linker built over one data type being used with a store
over a different data type. Both `HostContext` and `WasiP1Ctx` must live in the same
store data struct, but the host function submodules in `host/` are written against
`HostContext`, not the combined type.

## Decision

The `invoke.rs` module introduces `StoreData`, a wrapper struct that contains both
`HostContext` (as `host`) and `WasiP1Ctx` (as `wasi`). All linker registrations at
invocation time operate on `Linker<StoreData>`, accessing host state via
`caller.data().host` or `caller.data_mut().host`. The host function submodules
(`host/emit_event.rs`, `host/log.rs`, etc.) continue to operate on `Linker<HostContext>`
for integration-test purposes; the `proxy_host_imports` / `setup_host_on_store_data`
functions in `invoke.rs` re-register the same semantics against `StoreData`.
No global state is used; all host function state flows through the `Store`.

## Rationale

The `StoreData` pattern is the idiomatic wasmtime solution for combining WASI
context with custom host state. The wasmtime documentation and community examples
(including the Bytecode Alliance's wasi-preview2-prototype) use exactly this pattern:
a wrapper struct with named fields for each state slice, and closures in linker
registrations that dereference the appropriate field.

An alternative approach would be to make `HostContext` itself contain `WasiP1Ctx`
as a field and pass `HostContext` directly as the store data type. This was rejected
because it couples the host-function API type (`HostContext`) to the WASI lifecycle
machinery, making it harder to test host functions in isolation (integration tests
for `host/` submodules can instantiate `HostContext` without building a full WASI
context). The current split keeps these concerns separate.

A second alternative was to use global or thread-local state for `HostContext` and
register host functions as closures that capture a `Arc<Mutex<HostContext>>`. This
is explicitly rejected by the design principles of the dispatcher: the module doc
for `host/mod.rs` states that host function state flows through the `Store` and every
pointer is bounds-checked against wasm memory size on every call. Thread-local state
would break the per-invocation isolation guarantee — two plugins running in parallel
`spawn_blocking` tasks (ADR-008) would share thread-local state.

The comment in `invoke.rs:275–283` is candid about the implementation: wasmtime
does not support cloning `Func` between different `Store` types, so
`proxy_host_imports` effectively rebuilds the linker from scratch via
`setup_host_on_store_data`. The `_host_linker_reference` parameter is kept in the
signature for readability — it documents the intent (bridge from
`Linker<HostContext>` to `Linker<StoreData>`) even though the implementation
re-registers directly.

## Consequences

### Positive

- Per-invocation isolation is guaranteed: each `spawn_blocking` task creates its own
  `Store<StoreData>`, and no state leaks between concurrent plugin invocations.
- Integration tests for individual host functions can instantiate `HostContext`
  directly without needing a full WASI context or a compiled WASM module.
- `StoreData` fields are named (`host`, `wasi`), making the access pattern in linker
  closures unambiguous.
- The pattern is idiomatic wasmtime; new contributors familiar with the ecosystem
  will recognize it immediately.

### Negative / Trade-offs

- Host function semantics are registered twice: once against `Linker<HostContext>`
  (in `host/` submodules, for tests) and once against `Linker<StoreData>` (in
  `invoke.rs`, for production). This duplication must be kept in sync manually.
- The `setup_host_on_store_data` function in `invoke.rs` is the largest function in
  the dispatcher and grows with each new host function added to the ABI.

### Status as of v1.0.0-beta.5

IN-EFFECT. `StoreData { host: HostContext, wasi: WasiP1Ctx }` is defined in
`crates/factory-dispatcher/src/invoke.rs`. The `invoke_plugin` function constructs
a `Store<StoreData>`, builds a `Linker<StoreData>` via `setup_host_on_store_data`,
adds WASI via `p1::add_to_linker_sync(&mut linker, |d: &mut StoreData| &mut d.wasi)`,
then instantiates the plugin module. No global state is used at any point in the
invocation path.

## Alternatives Considered

- **HostContext contains WasiP1Ctx:** Make `HostContext` the store data type directly
  by embedding WASI context as a field. Rejected: couples the testable host-function
  API type to WASI lifecycle machinery; breaks unit-testability of `host/` submodules.
- **Global/thread-local HostContext:** Share host context through a thread-local
  `Arc<Mutex<HostContext>>` captured in linker closures. Rejected: breaks
  per-invocation isolation; two parallel `spawn_blocking` tasks would share state
  across plugin invocations.
- **Single Linker<HostContext> with WASI embedded in HostContext:** Identical to
  the first alternative but phrased differently. Same rejection reason.

## Source / Origin

- **Code as-built:** `crates/factory-dispatcher/src/invoke.rs:136` (`StoreData`
  struct construction), `invoke.rs:150` (`Linker<StoreData>` creation),
  `invoke.rs:156` (`p1::add_to_linker_sync` with `|d: &mut StoreData| &mut d.wasi`).
- **Code as-built:** `crates/factory-dispatcher/src/invoke.rs:275–294`
  (`proxy_host_imports` comment explaining the bridge pattern).
- **Code as-built:** `crates/factory-dispatcher/src/invoke.rs:297–549`
  (`setup_host_on_store_data` implementing all host imports against `StoreData`).
- **Code as-built:** `crates/factory-dispatcher/src/host/mod.rs:38–68`
  (`HostContext` struct definition and doc explaining it does not contain WASI context).
