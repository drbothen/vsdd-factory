# Adversarial Review — S-12.04 Pass 1

## Metadata
- Story: S-12.04 WASM Resolver Loading, Lifecycle, and Error Isolation
- Branch SHA reviewed: 4605fa8c
- Pass: 1
- Reviewer: adversary (fresh context)
- Classification: **CRITICAL**
- Within-story finding count: 14
- Recommendation: **PROCEED_TO_FIX**

---

## Executive Summary

The implementation of S-12.04 delivers scaffolding and partial wiring for WASM resolver loading but leaves two load-bearing paths as `todo!()` stubs and ships empty Kani harnesses. The story's acceptance criteria require end-to-end resolver invocation and formal verification coverage — neither is satisfied. Additionally, TOML schema drift (field `path` vs `plugin`) and a missing `context_key` field in the loader call create silent misconfiguration paths. Six HIGH findings compound the CRITICAL baseline. The code is not ready for convergence.

---

## Findings

### F-S12.04-P1-001 — CRITICAL: `invoke_resolver_wasm` is `todo!()`

**File:** `crates/engine/src/resolver/wasm.rs`

**Observation:** The primary dispatch function `invoke_resolver_wasm` contains only `todo!("WASM invocation not yet implemented")`. Every call site that routes through this function will panic at runtime. The story's acceptance criterion AC-S12.04-3 ("resolver invocation succeeds end-to-end") is entirely unmet.

**Evidence:**
```rust
pub fn invoke_resolver_wasm(
    ctx: &ResolverContext,
    module: &WasmModule,
) -> Result<ResolverOutput, ResolverError> {
    todo!("WASM invocation not yet implemented")
}
```

**Impact:** Any request that reaches a WASM resolver will panic, taking down the engine thread. Error isolation is not tested because invocation never executes.

**Required fix:** Implement the Wasmtime call sequence: obtain a `Store`, instantiate the pre-compiled `Module`, look up the `"resolve"` export, call it with the serialized `ResolverInput`, deserialize the result. The skeleton in `wasm_lifecycle.rs` already pre-compiles modules — wire the store creation and call here.

---

### F-S12.04-P1-002 — CRITICAL: Kani harnesses are empty

**File:** `crates/engine/src/resolver/kani_proofs.rs`

**Observation:** All three Kani harness functions (`verify_resolver_load_lifecycle`, `verify_error_isolation`, `verify_resource_limits`) have empty bodies — no symbolic inputs, no assertions, no property checks. The story requires formal verification of lifecycle and error isolation properties (AC-S12.04-6). Empty harnesses trivially pass without verifying anything.

**Evidence:**
```rust
#[cfg(kani)]
#[kani::proof]
fn verify_resolver_load_lifecycle() {
    // TODO: implement
}

#[cfg(kani)]
#[kani::proof]
fn verify_error_isolation() {
    // TODO: implement
}

#[cfg(kani)]
#[kani::proof]
fn verify_resource_limits() {
    // TODO: implement
}
```

**Impact:** Formal verification gate passes vacuously. Soundness guarantee claimed in the spec is not established.

**Required fix:** Each harness must construct symbolic inputs via `kani::any()`, call the relevant function under test, and assert the postconditions from the story's verification properties (VP-S12.04-1 through VP-S12.04-3).

---

### F-S12.04-P1-003 — HIGH: TOML field `path` vs `plugin` schema drift

**File:** `crates/engine/src/resolver/loader.rs` vs `docs/resolver-config.toml` (sample)

**Observation:** The loader parses `resolver.plugin` from TOML config, but the sample configuration file and the story spec (S-12.04 §3.2) both use `resolver.path`. Operators following the documented schema will produce configs that silently fail to load the resolver — the field deserializes to `None`, the loader skips initialization, and no error is surfaced.

**Evidence (loader):**
```rust
let plugin_path = config.resolver.plugin
    .as_ref()
    .ok_or(ResolverError::MissingConfig("plugin"))?;
```

**Evidence (spec/sample config):**
```toml
[resolver]
path = "resolvers/my_resolver.wasm"
```

**Impact:** Silent misconfiguration. Resolver silently absent in production; ops team sees no error, only missing resolution results.

**Required fix:** Align field name to `path` in the struct and deserializer, or rename the spec. Add a deprecation alias if migration is needed. Add a config validation test that loads the sample TOML and asserts the path is populated.

---

### F-S12.04-P1-004 — HIGH: Missing `context_key` in loader invocation

**File:** `crates/engine/src/resolver/loader.rs`

**Observation:** The `ResolverContext` struct requires a `context_key` field (documented in `crates/engine/src/resolver/types.rs` and required by the WASM ABI contract). The loader constructs `ResolverContext` without populating `context_key`, leaving it as `Default::default()` (empty string). The WASM module's `resolve` export uses `context_key` to scope cache lookups — an empty key collapses all cache entries to a single slot.

**Evidence:**
```rust
let ctx = ResolverContext {
    input: serialized_input,
    config: resolver_config.clone(),
    // context_key not set — defaults to ""
};
```

**Impact:** Cache collision across all concurrent resolver calls. Under load, one request's cached result overwrites another's. Non-deterministic resolution results.

**Required fix:** Populate `context_key` from the request correlation ID or a UUID generated at call site. Update the loader signature to accept the key or derive it from the surrounding `RequestContext`.

---

### F-S12.04-P1-005 — HIGH: Engine instance mismatch between loader and invoker

**File:** `crates/engine/src/resolver/loader.rs`, `crates/engine/src/resolver/wasm.rs`

**Observation:** `loader.rs` creates a `wasmtime::Engine` with a custom `Config` (fuel metering enabled). `wasm.rs`'s `invoke_resolver_wasm` (when implemented) is expected to receive a pre-compiled `Module` — but `Module` is compiled against a specific `Engine` instance, and `Store` must be created from the same `Engine`. The current code passes the `Module` across a boundary where the engine instance is not threaded through, making it impossible to create a compatible `Store` at invocation time without reconstructing the engine (losing the fuel config).

**Evidence:**
```rust
// loader.rs
let engine = Engine::new(&config)?;
let module = Module::new(&engine, &wasm_bytes)?;
// engine is dropped here; only module is stored

// wasm.rs (future implementation site)
// No engine reference available to create Store
let store = Store::new(/* ??? */, state);
```

**Impact:** Either the engine gets reconstructed without fuel metering (security regression — unbounded execution), or invocation panics trying to create an incompatible Store.

**Required fix:** Store the `Engine` alongside the compiled `Module` in the resolver registry struct (`LoadedResolver`). Thread the engine reference into `invoke_resolver_wasm`.

---

### F-S12.04-P1-006 — HIGH: Loader return value discarded at call site

**File:** `crates/engine/src/engine.rs`

**Observation:** The call to `load_resolver` in the engine initialization path discards the returned `LoadedResolver` — the result is bound to `_` and dropped. The resolver registry is never populated. Subsequent invocation attempts find an empty registry and fall through to a "no resolver configured" error path, making the feature entirely inoperative even after F-001 is fixed.

**Evidence:**
```rust
let _ = load_resolver(&config)?;  // LoadedResolver dropped immediately
```

**Impact:** Resolver loading appears to succeed (no error) but the loaded module is immediately freed. All resolver invocations will fail with "resolver not found."

**Required fix:** Bind the result and insert it into `self.resolver_registry` (or equivalent store). If no registry exists yet, that is a separate gap — create one.

---

### F-S12.04-P1-007 — HIGH: Capability wiring gap — WASM module receives no capability grants

**File:** `crates/engine/src/resolver/wasm.rs`, `crates/engine/src/resolver/loader.rs`

**Observation:** The story spec (S-12.04 §4.1) requires that WASM resolvers run under a capability model: only explicitly granted capabilities (network, filesystem, env) are accessible. The current implementation does not configure `wasmtime_wasi::WasiCtxBuilder` with any capability restrictions — it either uses a default WASI context (full ambient authority) or omits WASI entirely. No capability grant list is read from config or passed to the store.

**Impact:** WASM resolvers run with full WASI authority by default, defeating the isolation model. A malicious or buggy resolver can access arbitrary filesystem paths and environment variables.

**Required fix:** Read capability grants from `ResolverConfig::capabilities`. Build a `WasiCtxBuilder` that grants only those capabilities. If `capabilities` field is missing from `ResolverConfig`, add it and update the TOML schema accordingly.

---

### F-S12.04-P1-008 — HIGH: `path_allow` resolution uses process working directory as base

**File:** `crates/engine/src/resolver/loader.rs`

**Observation:** When resolving the WASM module path from config, the loader joins `path_allow` entries against `std::env::current_dir()`. In production, the working directory is not guaranteed to be the config file's directory or the plugin root. Paths that are valid relative to the config file become invalid or resolve to unintended locations.

**Evidence:**
```rust
let base = std::env::current_dir()?;
let resolved = base.join(&config.resolver.path_allow[0]);
```

**Impact:** WASM module fails to load in any deployment where the process is not started from the expected directory. Ops teams will see `No such file or directory` with no actionable context.

**Required fix:** Resolve relative paths against the config file's parent directory, not `current_dir()`. Require the config file path to be available in the loading context, or require all paths to be absolute.

---

### F-S12.04-P1-009 — MEDIUM: No test for loader failure modes (corrupt WASM, wrong magic bytes)

**File:** `crates/engine/src/resolver/loader.rs` (test section)

**Observation:** The test suite covers successful load from a valid `.wasm` fixture. There are no tests for: (a) file not found, (b) file present but not valid WASM (wrong magic bytes), (c) WASM module present but missing the required `resolve` export. The error isolation requirement (AC-S12.04-5) is partly behavioral — the engine must not crash on bad resolver input. Without these tests, the error paths are unverified.

**Required fix:** Add three negative-path unit tests using temp files: one absent, one with `b"not wasm"` content, one with a valid WASM module that exports no `resolve` function. Assert the correct `ResolverError` variant is returned in each case.

---

### F-S12.04-P1-010 — MEDIUM: `ResolverError` does not implement `std::error::Error`

**File:** `crates/engine/src/resolver/types.rs`

**Observation:** `ResolverError` derives `Debug` and `thiserror::Error` is not used — the enum manually implements `Display` but not `std::error::Error`. This prevents `?` propagation into `anyhow::Error` chains and breaks integration with the engine's top-level error handling, which expects `dyn std::error::Error`.

**Evidence:**
```rust
#[derive(Debug)]
pub enum ResolverError {
    MissingConfig(&'static str),
    LoadFailed(String),
    InvocationFailed(String),
    // ...
}

impl fmt::Display for ResolverError { ... }
// No `impl std::error::Error for ResolverError`
```

**Required fix:** Add `impl std::error::Error for ResolverError {}` or switch to `#[derive(thiserror::Error)]` with `#[error("...")]` attributes on each variant.

---

### F-S12.04-P1-011 — MEDIUM: Lifecycle drop not verified — `Drop` impl missing for `LoadedResolver`

**File:** `crates/engine/src/resolver/wasm.rs`

**Observation:** The story requires verified resource cleanup on resolver unload (AC-S12.04-4: "resolver resources are released on unload"). `LoadedResolver` holds a compiled `Module` and (implicitly) an `Engine`. There is no `Drop` implementation, no test that verifies the module is freed, and no integration with the engine's shutdown sequence. Wasmtime's `Module` does drop its internal state on `Drop`, but there is no store or instance to clean up — the question is whether the engine-level registry clears entries on shutdown. No evidence of this exists.

**Required fix:** Add a `drop` test or integration test that loads a resolver, drops the registry, and verifies (via a reference count or a mock drop counter) that the module is released. Document the cleanup contract in the code.

---

### F-S12.04-P1-012 — MEDIUM: Fuel exhaustion returns generic error, not `ResolverError::ResourceLimitExceeded`

**File:** `crates/engine/src/resolver/wasm.rs`

**Observation:** When fuel is exhausted during WASM execution, Wasmtime returns a `Trap` with `TrapCode::OutOfFuel`. The (future) invocation path should map this to `ResolverError::ResourceLimitExceeded` for observability and policy enforcement. The current stub returns a generic `InvocationFailed` variant (implied by the `todo!()`). No match arm for `OutOfFuel` exists anywhere in the codebase.

**Required fix:** In the invocation error mapping, match `TrapCode::OutOfFuel` and return `ResolverError::ResourceLimitExceeded`. Add a unit test that pre-configures a module to exhaust fuel and asserts the correct variant.

---

### F-S12.04-P1-013 — LOW: WASM fixture in tests is checked in as binary, not generated

**File:** `crates/engine/tests/fixtures/test_resolver.wasm`

**Observation:** The WASM test fixture is a pre-compiled binary blob committed to the repository. If the fixture's source (`test_resolver.wat` or equivalent) diverges from the binary, tests may pass against a stale artifact. No build step regenerates the fixture.

**Required fix:** Either (a) add a `build.rs` that compiles `test_resolver.wat` to `test_resolver.wasm` using `wat` crate, or (b) add a test that validates the fixture's exports match the expected ABI contract so stale binaries are caught. Option (a) is preferred.

---

### F-S12.04-P1-014 — LOW: No structured log fields on resolver load events

**File:** `crates/engine/src/resolver/loader.rs`

**Observation:** The loader emits `tracing::info!("Resolver loaded")` without structured fields (resolver path, module hash, load duration, export count). The observability story for this feature requires operators to be able to correlate resolver load events with specific module versions. Plain text log lines are not queryable in structured log systems (Loki, CloudWatch Insights).

**Required fix:** Use `tracing::info!(path = %resolved_path, hash = %module_hash, duration_ms = %elapsed.as_millis(), "resolver loaded")` pattern. Compute a `blake3` hash of the WASM bytes at load time and include it as a field.

---

## Finding Summary

| ID | Severity | Title |
|----|----------|-------|
| F-S12.04-P1-001 | CRITICAL | `invoke_resolver_wasm` is `todo!()` |
| F-S12.04-P1-002 | CRITICAL | Kani harnesses are empty |
| F-S12.04-P1-003 | HIGH | TOML field `path` vs `plugin` schema drift |
| F-S12.04-P1-004 | HIGH | Missing `context_key` in loader invocation |
| F-S12.04-P1-005 | HIGH | Engine instance mismatch between loader and invoker |
| F-S12.04-P1-006 | HIGH | Loader return value discarded at call site |
| F-S12.04-P1-007 | HIGH | Capability wiring gap — no capability grants |
| F-S12.04-P1-008 | HIGH | `path_allow` resolved from process cwd |
| F-S12.04-P1-009 | MEDIUM | No loader failure-mode tests |
| F-S12.04-P1-010 | MEDIUM | `ResolverError` missing `std::error::Error` |
| F-S12.04-P1-011 | MEDIUM | No `Drop` verification for `LoadedResolver` |
| F-S12.04-P1-012 | MEDIUM | Fuel exhaustion maps to wrong error variant |
| F-S12.04-P1-013 | LOW | WASM fixture committed as binary, no regeneration step |
| F-S12.04-P1-014 | LOW | No structured log fields on loader events |

**Recommendation: PROCEED_TO_FIX** — resolve CRITICAL and HIGH findings before Pass 2. MEDIUM and LOW may be batched or deferred per architect discretion.
