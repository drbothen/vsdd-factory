---
document_type: behavioral-contract
level: L3
version: "1.0"
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
bc_id: BC-4.12.002
section: "4.12"
last_amended: 2026-05-07
---

# BC-4.12.002: Resolver ABI MUST use distinct `ResolverInput` / `ResolverOutput` types versioned independently as Resolver ABI v1 (NOT reusing `HookPayload` / `HookResult`)

## Description

The resolver ABI is a distinct, independently versioned interface from the hook ABI. Resolvers
expose a single exported WASM function `resolve(input_ptr: i32, input_len: i32) -> i64` that
takes a JSON-serialized `ResolverInput` and returns a ptr+len pair encoding a JSON-serialized
`ResolverOutput`. The `ResolverInput` and `ResolverOutput` types are defined in
`crates/hook-sdk/src/resolver.rs` and are NOT the same as `HookPayload` / `HookResult`.
Resolver plugins are authored using the `#[resolver]` macro from the SDK's
`resolver-authoring` feature flag. The resolver ABI is versioned as `RESOLVER_ABI_VERSION = 1`
independently from `HOST_ABI_VERSION`.

## Preconditions

1. The resolver WASM artifact was compiled from a Rust crate using `vsdd-hook-sdk` with the
   `resolver-authoring` feature flag enabled.
2. The resolver crate uses the `#[resolver]` proc-macro (from `hook-sdk-macros`) which
   generates the WASM-compatible `resolve()` export from a user-defined Rust function.
3. The dispatcher has loaded the resolver artifact and holds a compiled `Module` for it
   (per BC-4.12.001 lifecycle contract).
4. A hook dispatch is in progress and the matched hooks-registry entry declares this resolver
   in its `needs_context` field.

## Postconditions

1. **Resolver exported function signature:** The resolver WASM artifact MUST export a single
   function with the signature: `resolve(input_ptr: i32, input_len: i32) -> i64`. The return
   value is a packed `(ptr: i32, len: i32)` pair encoded as `((ptr as i64) << 32) | (len as i64)`.
2. **ResolverInput type (dispatcher → resolver):** The dispatcher serializes the following
   struct as JSON and passes it to the resolver:
   ```
   ResolverInput {
     event_type: String,           // e.g., "SubagentStop"
     hook_event_name: String,      // e.g., "validate-per-story-adversary-convergence"
     agent_type: Option<String>,   // e.g., "wave-gate", null if absent
     project_dir: String,          // absolute path to the factory project root
     plugin_config: Value,         // the hook's static plugin_config from hooks-registry.toml
   }
   ```
3. **ResolverOutput type (resolver → dispatcher):** The resolver serializes the following
   struct as JSON and returns it via the `resolve()` export:
   ```
   ResolverOutput {
     key: String,           // the resolver's registry name (e.g., "wave-context")
     value: Option<Value>,  // the context payload, or null if no output for this dispatch
   }
   ```
4. **Resolver ABI versioning:** The SDK defines `RESOLVER_ABI_VERSION: u32 = 1` as a
   constant in `hook-sdk/src/resolver.rs`. This constant is versioned independently from
   `HOST_ABI_VERSION`. A resolver compiled against Resolver ABI v1 is incompatible with a
   dispatcher expecting Resolver ABI v2 (future); the dispatcher MUST verify the version
   constant during module loading.
5. **`#[resolver]` macro contract:** The `#[resolver]` proc-macro in `hook-sdk-macros`
   generates the `resolve()` export that: (a) reads the input byte slice from WASM memory;
   (b) deserializes it from JSON into `ResolverInput`; (c) calls the user's
   `fn resolve_impl(input: ResolverInput) -> ResolverOutput` function; (d) serializes the
   output to JSON; (e) writes the output to a WASM memory allocation and returns the
   ptr+len pair. The user function MUST be named `resolve_impl` and have this exact signature.
6. **No block/continue semantics:** Resolvers do NOT return `HookResult::Block` or
   `HookResult::Continue`. They return `ResolverOutput { key, value: Option<Value> }` —
   a data payload, not a behavioral decision. A resolver cannot block a hook dispatch.
7. **No hook-specific fields in ResolverInput:** `ResolverInput` does NOT contain
   `tool_input`, `tool_output`, `tool_response`, or any other hook-execution context fields.
   These fields are hook-ABI-specific and are not available to resolvers. Resolvers receive
   only the event metadata and project context needed to produce their output.
8. **`resolver-authoring` feature flag:** Hook crates that are NOT resolvers MUST NOT pull in
   the `vsdd-hook-sdk/resolver-authoring` feature. The feature flag ensures that
   `ResolverInput`, `ResolverOutput`, and the `#[resolver]` macro are only available to
   crates that explicitly opt in.
9. **ABI contract documented in HOST_ABI.md:** The resolver ABI contract (input/output types,
   `RESOLVER_ABI_VERSION` constant, `#[resolver]` macro usage, and relationship to
   `plugin_config`) MUST be documented in a §Context Injection Contract section in
   `crates/hook-sdk/HOST_ABI.md`.

## Invariants

1. **Distinct types from hook ABI:** `ResolverInput` and `ResolverOutput` are NOT type aliases
   for `HookPayload` and `HookResult`. They are separate structs defined in a separate module
   (`hook-sdk/src/resolver.rs`). Any code that conflates the two ABIs (e.g., passing a
   `HookPayload` where a `ResolverInput` is expected) is a type error.
2. **Resolver ABI versioned independently:** `RESOLVER_ABI_VERSION` evolves independently of
   `HOST_ABI_VERSION`. A bump to `HOST_ABI_VERSION` does not imply a bump to
   `RESOLVER_ABI_VERSION`, and vice versa. The two versioning tracks are decoupled.
3. **`resolve()` is the ONLY exported resolver function:** A resolver WASM module MUST NOT
   export additional functions beyond `resolve()` and the standard WASI-generated exports
   (e.g., `_start`, memory exports). Additional application-specific exports are prohibited.
4. **`plugin_config` in ResolverInput is READ-ONLY:** The resolver receives the static
   `plugin_config` from the hooks-registry entry (before any resolver output has been merged).
   The resolver MUST treat this as read-only data for its own computation; it cannot observe
   or depend on other resolvers' output (OD-5: no inter-resolver dependencies).
5. **JSON-over-WASM memory transport:** The serialization protocol is JSON (serde_json) in
   both directions. The ABI uses the same byte-slice-in-WASM-memory pattern as the hook ABI
   for the `HookPayload`. No binary serialization format (e.g., MessagePack, CBOR) is used.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Resolver returns `value: null` (JSON null) | `ResolverOutput.value` is `None`. Dispatcher does NOT write the key to `plugin_config`. |
| EC-002 | Resolver serializes invalid JSON in its output buffer | Dispatcher receives a deserialization error. Treated as a resolver crash (per BC-4.12.004). `resolver.error` event emitted. |
| EC-003 | Resolver returns a `key` that differs from its registry name | Dispatcher uses the `key` field from `ResolverOutput` for the merge; the registry `context_key` is the authoritative lookup name. If they differ, the merge key is wrong — this is a resolver authoring error. |
| EC-004 | Hook crate accidentally enables `resolver-authoring` feature | SDK types compile in but do nothing unless `#[resolver]` is used. No runtime impact. Should be caught by dependency review. |
| EC-005 | `ResolverInput.plugin_config` contains a key whose value is a large nested JSON object | Serialization still proceeds normally. No size limit on `plugin_config` at the ABI layer (WASM memory is the practical limit). |
| EC-006 | Resolver ignores `event_type` and always returns the same `value` | Valid resolver behavior. The resolver is free to ignore any `ResolverInput` fields it doesn't need. VP-075 determinism still holds. |

## Canonical Test Vectors

| Input | Expected ResolverOutput |
|-------|------------------------|
| `ResolverInput { event_type: "SubagentStop", hook_event_name: "validate-...", agent_type: Some("wave-gate"), project_dir: "/repo", plugin_config: {} }` | `ResolverOutput { key: "wave-context", value: Some({"stories": ["S-12.03", "S-12.04"], "wave_id": "wave-1", "cycle_id": "v1.0-..."}) }` |
| Same input but `.factory/wave-state.yaml` absent | `ResolverOutput { key: "wave-context", value: None }` |
| `ResolverInput { event_type: "PreToolUse", ... }` | `ResolverOutput { key: "wave-context", value: None }` (resolver returns None for irrelevant events) |
| Serde round-trip: serialize ResolverInput, deserialize, serialize ResolverOutput, deserialize | Identical structs (VP-075 determinism test vector) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-075 | Context-injection determinism — identical `ResolverInput` yields identical `ResolverOutput` | proptest (200 trials; mock resolver with pure function; assert output identical on two calls with same input) |
| (unit-test) | `ResolverInput` serde round-trip: serialize → deserialize → equal original | Rust unit test |
| (unit-test) | `ResolverOutput { value: None }` does not write key to `plugin_config` | Rust unit test |
| (unit-test) | `#[resolver]` macro expansion compiles and generates correct WASM export | trybuild test |
| (unit-test) | `RESOLVER_ABI_VERSION = 1` is present and exported by `hook-sdk/src/resolver.rs` | Rust unit test (assert constant value) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-009 |
| Capability Anchor Justification | CAP-009 ("Author and publish WASM hook plugins using the Rust SDK") per capabilities.md §CAP-009 — this BC defines the resolver authoring ABI, which is an extension of the SDK's plugin authoring surface. Resolver plugins use the `vsdd-hook-sdk`'s `resolver-authoring` feature flag, the `#[resolver]` proc-macro from `hook-sdk-macros`, and the `ResolverInput`/`ResolverOutput` types. CAP-009 defines the SDK as the interface through which first-party plugin authors implement hook behavior; this BC specifies the distinct resolver sub-surface of that SDK interface. |
| L2 Domain Invariants | none |
| Architecture Module | `crates/hook-sdk/src/resolver.rs` (ResolverInput, ResolverOutput, RESOLVER_ABI_VERSION); `crates/hook-sdk-macros/src/resolver_macro.rs` (#[resolver] macro); `crates/hook-sdk/HOST_ABI.md` (§Context Injection Contract section) |
| Stories | S-12.05 (hook-sdk resolver-authoring extensions), S-12.06 (HOST_ABI.md context-injection contract section) |
| FR | FR-RESOLVER-001 (factory-agnostic runtime context injection for hooks via sandboxed WASM-plugin resolvers) |
| ADR Reference | ADR-018 (WASM-plugin Context Resolvers — OD-3: distinct ResolverInput/ResolverOutput types, versioned independently as Resolver ABI v1) |

## Related BCs

- BC-1.13.001 — depends on (dispatcher pre-dispatch injection contract; this BC defines the ABI types used in that injection step)
- BC-4.12.001 — sibling (resolver lifecycle — this BC defines what the dispatcher invokes during the per-dispatch Store execution)
- BC-4.12.003 — composes with (resolver capability model — capability checks operate within the `resolve()` call defined here)
- BC-4.12.004 — composes with (error isolation — errors from the `resolve()` invocation are handled per BC-4.12.004)
- BC-4.12.005 — depends on (merge contract — uses `ResolverOutput.key` and `ResolverOutput.value` as the merge inputs)

## Architecture Anchors

- `/Users/jmagady/Dev/vsdd-factory/crates/hook-sdk/src/resolver.rs` — ResolverInput, ResolverOutput, RESOLVER_ABI_VERSION, Resolver trait
- `/Users/jmagady/Dev/vsdd-factory/crates/hook-sdk-macros/src/resolver_macro.rs` — `#[resolver]` macro implementation
- `/Users/jmagady/Dev/vsdd-factory/crates/hook-sdk/HOST_ABI.md` — §Context Injection Contract section (to be authored in S-12.06)
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/architecture/decisions/ADR-018-wasm-plugin-context-resolvers.md` — OD-3 ABI decision

## Story Anchor

S-12.05 (hook-sdk resolver-authoring extensions) and S-12.06 (HOST_ABI.md context-injection contract section) — v1.0-feature-engine-discipline-pass-1 F3-amendment.

## VP Anchors

- VP-075 — Context-injection determinism

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.0 | 2026-05-07 | Initial authoring (product-owner; F2-amendment phase of v1.0-feature-engine-discipline-pass-1). Encodes OD-3 (distinct ResolverInput/ResolverOutput types, versioned independently as Resolver ABI v1). Resolver ABI explicitly does NOT reuse HookPayload/HookResult per user-authorized architectural decision D-361. |
