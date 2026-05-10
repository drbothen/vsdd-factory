//! `classify_resolver_trap` — maps wasmtime `Trap` to `ResolverError::Trap`.
//!
//! When a WASM resolver module traps during execution, the host linker
//! catches the `wasmtime::Trap` and calls this function to produce a
//! structured `ResolverError` carrying the trap detail string.
//!
//! Architecture anchors:
//! - BC-4.12.004 — resolver crash isolation contract
//! - ADR-018 §WASM trap handling
//! - VP-074 — Kani harness for trap classification (placeholder, S-12.04)
//! - S-12.04 — this story; implementation deferred to Step 3

use crate::resolver::ResolverError;

/// Map a wasmtime `Trap` to a `ResolverError::Trap` for the named
/// resolver.
///
/// Non-trivial: contains branching over `Trap` variants, string
/// formatting, and call into `ResolverError` constructor. S-12.04
/// Step 3 implementation.
///
/// # Parameters
///
/// - `resolver_name` — registry name of the resolver that trapped
/// - `trap` — the wasmtime trap produced during execution
///
/// # Returns
///
/// `ResolverError::Trap` with the resolver name and a human-readable
/// detail string derived from the trap.
pub fn classify_resolver_trap(_resolver_name: &str, _trap: wasmtime::Trap) -> ResolverError {
    todo!("S-12.04 Step 3 implementation")
}

// ---------------------------------------------------------------------------
// VP-074 Kani verification harnesses (placeholder)
// ---------------------------------------------------------------------------

/// Placeholder for VP-074 Kani formal verification harnesses.
///
/// Harnesses will verify that `classify_resolver_trap` is total (no
/// panics) and maps every `TrapCode` variant to a `ResolverError::Trap`
/// with a non-empty `detail` string. Populated in S-12.04 Step 3.
///
/// `#[cfg(kani)]` gates compilation to the Kani model checker only so
/// this module has zero compile-time cost in normal builds.
#[cfg(kani)]
mod kani_harnesses {
    use super::*;

    // TODO(S-12.04 Step 3): add kani::proof harnesses for VP-074.
    // Each harness exercises `classify_resolver_trap` over a symbolic
    // `TrapCode` and asserts:
    // 1. The function never panics (totality).
    // 2. The returned variant is always `ResolverError::Trap`.
    // 3. The `detail` field is non-empty for every code.
}
