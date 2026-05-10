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
pub fn classify_resolver_trap(resolver_name: &str, trap: wasmtime::Trap) -> ResolverError {
    // All wasmtime Trap variants are mapped to ResolverError::Trap.
    //
    // VP-074 / BC-4.12.004 INV1 totality requirement: this function must
    // return ResolverError::Trap for every possible TrapCode input without
    // panicking. The test harness verifies this by iterating all valid
    // byte values from Trap::from_u8 and asserting the Trap variant.
    //
    // The detail field carries the human-readable description from
    // wasmtime::Trap's Display impl, which is always non-empty.
    ResolverError::Trap {
        name: resolver_name.to_string(),
        detail: format!("{trap}"),
    }
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

// ---------------------------------------------------------------------------
// AC-012 unit tests — classify_resolver_trap is total
//
// BC-4.12.004 invariant 1 / VP-074:
// `classify_resolver_trap` must handle every variant of wasmtime::Trap
// without panicking. This is a pure function — no I/O, no side effects.
//
// The kani harness (above, Step 3) proves this formally. These Rust tests
// verify concrete trap instances from the wasmtime runtime.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// test_BC_4_12_004_classify_resolver_trap_is_total
    ///
    /// Calls `classify_resolver_trap` with every known `wasmtime::Trap` variant
    /// constructible via `Trap::from_u8`. Asserts:
    /// 1. The function returns `ResolverError::Trap` for every input
    ///    (not `NotFound` — BC-4.12.004 INV1).
    /// 2. The `detail` field is non-empty for every variant.
    /// 3. The function does NOT panic for any variant (totality — VP-074).
    ///
    /// `wasmtime::Trap` in version 44 is a C-like enum with variants accessible
    /// via `Trap::from_u8(byte)`. We iterate byte values 0..=255 and test every
    /// `Some(trap)` that `from_u8` returns.
    ///
    /// Red Gate: fails because `classify_resolver_trap` is `todo!()` in
    /// the Step 3 stub — any call panics before implementation.
    #[test]
    fn test_BC_4_12_004_classify_resolver_trap_is_total() {
        let resolver_name = "test-resolver";
        let mut tested_count = 0usize;

        // Iterate all possible byte values; from_u8 returns Some for each
        // valid TrapCode variant and None for invalid bytes.
        for byte in 0u8..=u8::MAX {
            let Some(trap) = wasmtime::Trap::from_u8(byte) else {
                continue;
            };
            tested_count += 1;

            // classify_resolver_trap must not panic (totality — VP-074).
            let result = classify_resolver_trap(resolver_name, trap);

            // Result must always be the Trap variant (BC-4.12.004 INV1).
            match &result {
                ResolverError::Trap { name, detail } => {
                    assert_eq!(
                        name.as_str(),
                        resolver_name,
                        "AC-012 / VP-074: classify_resolver_trap must preserve \
                         resolver_name in Trap variant (byte={byte})"
                    );
                    assert!(
                        !detail.is_empty(),
                        "AC-012 / VP-074: Trap.detail must be non-empty for \
                         byte={byte}"
                    );
                }
                ResolverError::NotFound { .. } => {
                    panic!(
                        "AC-012 / VP-074: classify_resolver_trap must NEVER return \
                         ResolverError::NotFound (byte={byte})"
                    );
                }
                other => {
                    panic!(
                        "AC-012 / VP-074: classify_resolver_trap must return \
                         ResolverError::Trap for every TrapCode (byte={byte}), \
                         got {:?}",
                        other
                    );
                }
            }
        }

        assert!(
            tested_count > 0,
            "AC-012: at least one wasmtime::Trap variant must be testable via \
             Trap::from_u8 — wasmtime API may have changed"
        );
    }

    /// test_BC_4_12_004_classify_resolver_trap_detail_carries_trap_description
    ///
    /// Verifies that the `detail` field in the returned `ResolverError::Trap`
    /// is non-empty and carries a human-readable description of the trap.
    /// Uses `wasmtime::Trap::StackOverflow` as a concrete, stable variant.
    ///
    /// Red Gate: fails because `classify_resolver_trap` is `todo!()`.
    #[test]
    fn test_BC_4_12_004_classify_resolver_trap_detail_carries_trap_description() {
        // StackOverflow is byte 0 and is always a valid TrapCode.
        let trap = wasmtime::Trap::from_u8(0)
            .expect("StackOverflow (byte 0) must be a valid wasmtime::Trap");

        let result = classify_resolver_trap("my-resolver", trap);

        match result {
            ResolverError::Trap { detail, .. } => {
                assert!(
                    !detail.is_empty(),
                    "AC-012: Trap.detail must be non-empty — must carry a \
                     human-readable description of the wasmtime trap variant"
                );
            }
            other => {
                panic!(
                    "AC-012: expected ResolverError::Trap for StackOverflow, \
                     got {:?}",
                    other
                );
            }
        }
    }
}
