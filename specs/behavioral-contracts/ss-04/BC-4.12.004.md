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
bc_id: BC-4.12.004
section: "4.12"
last_amended: 2026-05-07
---

# BC-4.12.004: Resolver crashes (panic, trap, timeout, ABI violation) MUST NOT propagate to the dispatcher process and MUST produce a `resolver.error` telemetry event with fail-loud semantics for declared resolvers

## Description

The WASM sandbox provides hard isolation between a resolver's execution and the dispatcher
process. A resolver panic, WASM trap, execution timeout (fuel/epoch budget exceeded), or
ABI violation (e.g., invalid output JSON) MUST NOT crash the dispatcher or propagate an
unhandled error to the dispatch stack. The dispatcher wraps each resolver call in the same
trap-classification logic used by `invoke.rs` for hooks (`classify_resolver_trap`). On any
resolver error, the dispatcher: (1) emits a `resolver.error` telemetry event with the
resolver name and error detail; (2) does NOT merge the failed resolver's output into
`plugin_config`; (3) proceeds to hook dispatch WITHOUT the missing context. The hook is then
responsible for treating a missing `plugin_config[key]` as an error if that context is
required for correctness — the dispatcher does not make this judgment on the hook's behalf.

## Preconditions

1. A resolver invocation is in progress inside a `Store<HostContext>` (per BC-4.12.001
   lifecycle).
2. The resolver's `resolve()` function has been called and has encountered one of the
   following conditions:
   a. WASM trap (e.g., unreachable instruction, out-of-bounds memory access, integer overflow)
   b. Panic (Rust `panic!` compiled to WASM trap via `panic = "abort"`)
   c. Fuel/epoch budget exhausted (execution timeout)
   d. ABI violation (the `resolve()` return value cannot be decoded as valid `ResolverOutput` JSON)
   e. `CapabilityDenied` from a host function (per BC-4.12.003 — this is a structured error, not a trap)
3. The dispatcher is executing inside `ResolverRegistry::invoke_resolver(name, input)`.

## Postconditions

1. **Dispatcher process is unaffected:** The resolver error MUST be caught within
   `invoke_resolver` and returned as `Result<ResolverOutput, ResolverError>`. The `?`
   operator MUST NOT propagate a resolver error past the dispatch boundary. The dispatcher
   continues executing normally after handling the error.
2. **`resolver.error` telemetry event:** The dispatcher MUST emit a structured telemetry
   event with:
   - `event_type`: `"resolver.error"`
   - `resolver_name`: the name of the failed resolver (from the registry)
   - `error_kind`: one of `"trap"`, `"timeout"`, `"abi_violation"`, `"capability_denied"`,
     `"not_found"`, `"load_error"`
   - `error_detail`: a human-readable string describing the specific error
   - `hook_event_name`: the name of the hook dispatch context that triggered this resolver
3. **Failed resolver output NOT merged:** The dispatcher MUST NOT write any data into
   `plugin_config` for a failed resolver. No partial output, no null value, no default value
   — the key is simply absent from `plugin_config` after a resolver failure.
4. **Dispatch proceeds without the missing context:** The hook dispatch continues after
   resolver failure. The hook receives a `plugin_config` that lacks the failed resolver's
   key. The hook is responsible for deciding whether to block or continue based on the
   missing key.
5. **No silent degrade for declared resolvers:** If the hook declared the resolver's key in
   `needs_context` and the resolver fails, the key is absent from `plugin_config` — this is
   visible to the hook. The dispatcher does NOT inject an empty `{}` value or any default to
   mask the failure. Fail-loud at the hook's observation point.
6. **Error isolation per resolver:** If multiple resolvers are declared in `needs_context` and
   one fails, the remaining resolvers STILL execute. A failure in resolver A does not skip
   resolver B. Each resolver invocation is independently isolated.
7. **Dispatcher log entry:** In addition to the telemetry event, the dispatcher writes a
   log entry to `.factory/logs/` (or the configured log path) at error level documenting
   the resolver crash with the same fields as the telemetry event.

## Invariants

1. **No `unwrap()` on resolver result:** The `invoke_resolver` function MUST NOT use
   `.unwrap()` or `.expect()` on its result. The kani harness (`classify_resolver_trap`) MUST
   verify that the pure error-classification logic handles all `TrapCode` variants without
   panicking. No `unreachable!()` in the trap classifier.
2. **Isolation applies to ALL error types:** The error containment invariant applies to traps,
   panics, timeouts, and ABI violations equally. There is no error type that causes a resolver
   failure to propagate to the dispatcher.
3. **`resolver.error` is always emitted on failure:** If `invoke_resolver` returns
   `Err(ResolverError::*)`, the dispatcher MUST emit `resolver.error` before proceeding to
   hook dispatch. There is no silent failure path.
4. **CapabilityDenied is an error, not a trap:** A resolver that receives `CapabilityDenied`
   from `host::read_file` (per BC-4.12.003) has a structured error return code available to
   its Rust code. If the resolver correctly handles the error and returns a valid
   `ResolverOutput` (possibly with `value: None`), this is NOT a resolver failure — it is
   correct error-handling behavior. Only if the resolver panics or traps as a result of the
   capability denial is BC-4.12.004 invoked.
5. **Hook decides on missing context:** The dispatcher never interprets the semantic meaning
   of a missing resolver key. If the hook needs `wave_context` and it is absent, only the
   hook can decide whether to return `HookResult::Block` or `HookResult::Continue`. The
   dispatcher's role ends at emitting the `resolver.error` event and leaving the key absent.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Resolver panics (Rust `panic!`)| `panic = "abort"` compiles to WASM trap. Trap caught by `invoke_resolver`. `resolver.error` emitted with `error_kind: "trap"`. Dispatch continues without resolver key. |
| EC-002 | Resolver enters infinite loop (fuel budget exhausted) | Wasmtime fuel/epoch limit triggers. `resolver.error` emitted with `error_kind: "timeout"`. Dispatch continues. |
| EC-003 | Resolver returns invalid JSON in output buffer | `ResolverOutput` deserialization fails. `resolver.error` emitted with `error_kind: "abi_violation"`. Dispatch continues without key. |
| EC-004 | Resolver returns valid JSON but with wrong type (e.g., `value` is a string where a JSON object is expected) | From the dispatcher's perspective, `value` is `Option<serde_json::Value>` — any valid JSON value is accepted. Type checking is the hook's responsibility when it reads `plugin_config[key]`. No `abi_violation` error for valid JSON. |
| EC-005 | Resolver A fails; resolver B succeeds | B's output is merged normally. A's key is absent. `resolver.error` emitted for A only. |
| EC-006 | Resolver times out after 25ms (conservative budget) | Fuel/epoch budget set to 25% of hook budget per S-12.04 spec. Timeout treated as `error_kind: "timeout"`. |
| EC-007 | Resolver calls `host::write_file` (which is not in resolver linker) | Module instantiation fails at startup (linker error), not at runtime. Treated as `resolver.load_error` (BC-1.13.001 PC2), not a runtime `resolver.error`. |

## Canonical Test Vectors

| Resolver Behavior | Expected Dispatcher Behavior |
|-------------------|------------------------------|
| Resolver returns valid output | Merge into `plugin_config`; no error event |
| Resolver panics (trap) | `resolver.error` with `error_kind: "trap"`; key absent from `plugin_config`; dispatch continues |
| Resolver times out | `resolver.error` with `error_kind: "timeout"`; key absent; dispatch continues |
| Resolver returns invalid JSON | `resolver.error` with `error_kind: "abi_violation"`; key absent; dispatch continues |
| Resolver A fails; resolver B succeeds | `resolver.error` for A; B's key in `plugin_config`; both hooks dispatched |
| Hook needs missing key (resolver failed) | Hook sees absent key in `plugin_config`; hook returns Block with block_with_fix citing absent context |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-074 | Resolver-error isolation — a resolver panic, trap, or timeout does not propagate to the dispatcher process; `invoke_resolver` returns `Result<ResolverOutput, ResolverError>` in all cases | kani (pure logic: `classify_resolver_trap` covers all `TrapCode` variants without panics) + integration test (trap injection: deliberate-panic resolver WASM; assert dispatcher continues; assert `resolver.error` event; assert key absent) |
| (unit-test) | Dispatcher process does not crash when resolver traps | Rust unit test (inject trap via wasmtime `unreachable` WASM; assert invoke_resolver returns Err) |
| (unit-test) | `resolver.error` event emitted with correct fields on trap | Rust unit test (mock telemetry; assert event fields) |
| (unit-test) | Failed resolver key is absent from `plugin_config` (not null, not `{}`) | Rust unit test |
| (unit-test) | Resolver B executes normally after resolver A fails | Rust unit test (two resolvers; A panics; assert B output present) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-009 |
| Capability Anchor Justification | CAP-009 ("Author and publish WASM hook plugins using the Rust SDK") per capabilities.md §CAP-009 — this BC governs the error-isolation semantics of WASM resolver plugins. Resolvers are WASM plugins compiled with the `vsdd-hook-sdk`'s `resolver-authoring` feature; this BC specifies how the dispatcher handles the full range of resolver failure modes within the wasmtime WASM sandbox. CAP-009 defines the SDK and sandbox model; error isolation is a property of that sandbox boundary (the `invoke_resolver` return type `Result<ResolverOutput, ResolverError>` and the `classify_resolver_trap` function). |
| L2 Domain Invariants | none |
| Architecture Module | `crates/factory-dispatcher/src/resolver.rs` (invoke_resolver function; ResolverError enum); `crates/factory-dispatcher/src/resolver_classify_trap.rs` (pure classify_resolver_trap function; kani proof target); `crates/factory-dispatcher/src/executor.rs` (error handling in dispatch loop) |
| Stories | S-12.04, S-12.06, S-12.07 |
| FR | FR-RESOLVER-001 (resolver crash or error MUST NOT prevent hook dispatch) |
| ADR Reference | ADR-018 (WASM-plugin Context Resolvers — crash isolation; same trap-classification pattern as hooks) |

## Related BCs

- BC-1.13.001 — depends on (dispatcher startup and dispatch contract; this BC specifies what happens when resolver invocation fails during the pre-dispatch step)
- BC-4.12.001 — sibling (lifecycle — Store-per-call isolation ensures one resolver crash cannot affect another Store)
- BC-4.12.002 — sibling (ABI — ABI violations in the `resolve()` return value are one of the error kinds handled here)
- BC-4.12.003 — composes with (capability model — CapabilityDenied is a structured error that may or may not result in a trap depending on how the resolver handles it)
- BC-4.12.005 — depends on (merge contract — a failed resolver's output is NOT merged; this BC defines the negative case)

## Architecture Anchors

- `/Users/jmagady/Dev/vsdd-factory/crates/factory-dispatcher/src/resolver.rs` — `invoke_resolver` function, `ResolverError` enum
- `/Users/jmagady/Dev/vsdd-factory/crates/factory-dispatcher/src/resolver_classify_trap.rs` — pure `classify_resolver_trap(TrapCode) -> ResolverError` function (kani proof target per VP-074)
- `/Users/jmagady/Dev/vsdd-factory/crates/factory-dispatcher/src/invoke.rs` — reference implementation pattern for trap classification (hook equivalent)
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/architecture/decisions/ADR-018-wasm-plugin-context-resolvers.md` — crash isolation design

## Story Anchor

S-12.04 — WASM resolver loading + lifecycle + error isolation (v1.0-feature-engine-discipline-pass-1 F3-amendment).

## VP Anchors

- VP-074 — Resolver-error isolation

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.0 | 2026-05-07 | Initial authoring (product-owner; F2-amendment phase of v1.0-feature-engine-discipline-pass-1). Error isolation semantics modeled on hook `invoke.rs` trap-classification pattern. Fail-loud for declared resolvers: absent key is visible to hook; dispatcher does not inject default value. |
| 1.1 | 2026-05-09 | F-P45-001 — Traceability Stories row propagated from BC-INDEX v1.57: S-12.04 → S-12.04, S-12.06, S-12.07. BC-INDEX was updated in fix-burst-39 (v1.55) to add S-12.06 + S-12.07; body was not updated in that burst. Refs: F-P45-001, fix-burst-42. |
