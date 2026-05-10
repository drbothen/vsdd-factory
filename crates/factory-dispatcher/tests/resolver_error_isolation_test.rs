//! Resolver error-isolation tests (S-12.04 AC-004, AC-005).
//!
//! Verifies that a trapping WASM resolver does not abort the dispatch
//! cycle and that a `resolver.error` telemetry event is emitted for each
//! trap (BC-4.12.004 crash isolation contract).
//!
//! WASM test fixtures:
//! - `fixtures/trapping_resolver.wasm` — deferred; to be generated via
//!   wat2wasm by the implementer during Step 3. The canonical WAT source
//!   is a minimal module that exports `resolve` and executes `unreachable`
//!   immediately. Tests reference the path but fail at fixture-not-found
//!   before reaching todo!() — acceptable for Red Gate (BC-5.38.001).
//! - `fixtures/path_escaping_resolver.wasm` — deferred to S-12.07 per
//!   spec File List comment. Not referenced in these stubs.
//!
//! Architecture anchors:
//! - BC-4.12.004 — resolver crash isolation (trapping resolver must not
//!   abort dispatch; trap is caught, converted to `ResolverError::Trap`,
//!   and reported via `emit_resolver_error` callback)
//! - S-12.04 AC-004 (trap isolation), AC-005 (trap event emission)

#[allow(unused_imports)]
use factory_dispatcher::resolver::ResolverRegistry;

/// AC-004: A WASM resolver that traps during `resolve()` does NOT abort
/// dispatch. Remaining resolvers in the registry are invoked normally
/// and the dispatch cycle returns a result.
///
/// BC-5.38.005 self-check: "If I include a real implementation here,
/// will the test pass trivially without implementer work?" — Yes; the real
/// assertion requires a trapping WASM fixture + dispatch invocation +
/// survivor-resolver assertion. todo!() per BC-5.38.001.
#[test]
fn test_trapping_resolver_does_not_abort_dispatch() {
    todo!()
}

/// AC-005: A WASM resolver that traps emits exactly one `resolver.error`
/// telemetry event carrying the trap detail string and the resolver name.
///
/// BC-5.38.005 self-check: real body requires an event-capturing sink,
/// a trapping WASM fixture, dispatch invocation, and event assertion.
/// todo!() per BC-5.38.001.
#[test]
fn test_trapping_resolver_emits_resolver_error_event() {
    todo!()
}
