//! `classify_resolver_trap` — maps wasmtime `Trap` to `ResolverError`.
//!
//! When a WASM resolver module traps during execution, the host linker
//! catches the `wasmtime::Trap` and calls this function to produce a
//! structured `ResolverError` carrying the trap detail string.
//!
//! F-P3-002: `Trap::Interrupt` (which is what epoch deadline produces) maps to
//! `ResolverError::Timeout` — not `ResolverError::Trap`. Per HOST_ABI semantics,
//! "timeout" is a distinct error_kind from "trap". All other trap variants map
//! to `ResolverError::Trap`.
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
    // F-P3-002: Trap::Interrupt is produced by epoch deadline interruption.
    // Per HOST_ABI semantics, timeout is a distinct error_kind ("timeout") from
    // a WASM execution trap ("trap"). Mapping Interrupt → Timeout preserves
    // the semantic distinction so callers can distinguish deadline from fault.
    //
    // All other Trap variants map to ResolverError::Trap.
    //
    // VP-074 / BC-4.12.004 INV1 totality requirement: this function must
    // return a ResolverError for every possible TrapCode input without panicking.
    // The Kani harness proof_classify_trap_never_returns_not_found still holds:
    // neither Timeout nor Trap is NotFound.
    match trap {
        wasmtime::Trap::Interrupt => ResolverError::Timeout {
            name: resolver_name.to_string(),
        },
        _ => ResolverError::Trap {
            name: resolver_name.to_string(),
            detail: format!("{trap}"),
        },
    }
}

// ---------------------------------------------------------------------------
// VP-074 Kani formal verification harnesses (F-P1-002)
// ---------------------------------------------------------------------------

/// VP-074 Kani formal verification harnesses (F-P1-002).
///
/// **Toolchain compatibility:** kani 0.67.0 requires rustc 1.93.0-nightly
/// but this workspace is pinned to stable 1.95+ (rust-toolchain.toml).
/// `cargo kani -p factory-dispatcher` reports a toolchain version mismatch.
/// The harnesses compile under `cargo check` (using `#[cfg(kani)]` gate).
/// Kani execution is blocked by the toolchain gap, not by harness logic.
///
/// TODO(VP-074): Re-run once kani supports rustc 1.95+ stable, or add a
/// nightly toolchain override for the verifier target.
///
/// Proves that `classify_resolver_trap` is total (no panics for any input)
/// and satisfies its two critical safety properties:
/// 1. Always returns a `ResolverError` variant (no panic, no unwrap).
/// 2. Never returns `ResolverError::NotFound` (which would be semantically
///    wrong — a trap is not a missing-resolver condition).
///
/// Approach: `wasmtime::Trap` is a C-like enum indexed by a `u8`. A symbolic
/// byte (`kani::any::<u8>()`) exercises the full byte space. For bytes that
/// produce `Some(trap)` via `Trap::from_u8`, `classify_resolver_trap` is
/// called and its result is checked. For bytes that produce `None`, the
/// harness verifies that no trap variant exists for that code (no reachable
/// code paths).
///
/// `#[cfg(kani)]` gates compilation to the Kani model checker only —
/// zero compile-time cost in normal builds.
#[cfg(kani)]
mod kani_harnesses {
    use super::*;

    /// proof_classify_trap_is_total (VP-074 property 1)
    ///
    /// Symbolic verification: for any `u8` value that maps to a valid
    /// `wasmtime::Trap` via `Trap::from_u8`, `classify_resolver_trap`
    /// returns a `ResolverError` variant without panicking.
    ///
    /// This proves totality: the function is defined for every valid TrapCode
    /// reachable by a u8 symbolic value.
    #[kani::proof]
    fn proof_classify_trap_is_total() {
        let byte: u8 = kani::any();
        // Only exercise bytes that produce a valid Trap variant.
        let Some(trap) = wasmtime::Trap::from_u8(byte) else {
            return; // not a valid TrapCode — no reachable classify_resolver_trap call
        };
        // classify_resolver_trap must not panic for any valid TrapCode.
        let result = classify_resolver_trap("test-resolver", trap);
        // Must return some ResolverError variant — the match here exhausts
        // all known variants per the non_exhaustive enum definition.
        match result {
            ResolverError::Trap { .. } => {}
            ResolverError::Timeout { .. } => {}
            ResolverError::AbiViolation { .. } => {}
            ResolverError::CapabilityDenied { .. } => {}
            ResolverError::NotFound { .. } => {
                // This arm must be unreachable — proven by the next harness.
                kani::assert(
                    false,
                    "VP-074: classify_resolver_trap must never return NotFound",
                );
            }
            ResolverError::Malformed { .. } => {}
            ResolverError::DuplicateName { .. } => {}
        }
    }

    /// proof_classify_trap_never_returns_not_found (VP-074 property 2)
    ///
    /// Symbolic verification: for any valid `wasmtime::Trap`, the returned
    /// `ResolverError` is NEVER `ResolverError::NotFound`. A trap is a WASM
    /// execution failure, not a missing-resolver condition.
    ///
    /// This is the critical safety property: if `classify_resolver_trap`
    /// returned `NotFound`, the dispatcher would incorrectly emit
    /// `resolver.not_found` for a resolver that actually exists but trapped.
    #[kani::proof]
    fn proof_classify_trap_never_returns_not_found() {
        let byte: u8 = kani::any();
        let Some(trap) = wasmtime::Trap::from_u8(byte) else {
            return;
        };
        let result = classify_resolver_trap("any-resolver", trap);
        kani::assert(
            !matches!(result, ResolverError::NotFound { .. }),
            "VP-074: classify_resolver_trap must NEVER return ResolverError::NotFound \
             for any TrapCode — a trap is a WASM execution failure, not a missing-resolver condition",
        );
    }

    /// proof_classify_trap_never_returns_trap_for_interrupt (VP-074 property 3, F-P3-002)
    ///
    /// Symbolic verification: for `Trap::Interrupt` (the epoch deadline trap),
    /// the returned `ResolverError` is NEVER `ResolverError::Trap`. Per HOST_ABI
    /// semantics, epoch interruption is a timeout condition, not a WASM fault.
    ///
    /// F-P3-002: Trap::Interrupt maps to ResolverError::Timeout. This harness
    /// proves that invariant: Interrupt → Timeout, not Trap.
    ///
    /// For non-Interrupt traps, the mapping remains Trap → Trap (proven by
    /// proof_classify_trap_is_total which accepts Trap variants without assertion).
    #[kani::proof]
    fn proof_classify_trap_interrupt_returns_timeout() {
        // Trap::Interrupt has a stable byte code; use kani::assume to select only it.
        let byte: u8 = kani::any();
        let Some(trap) = wasmtime::Trap::from_u8(byte) else {
            return;
        };
        // Focus only on the Interrupt variant (the epoch-deadline case).
        kani::assume(matches!(trap, wasmtime::Trap::Interrupt));
        let result = classify_resolver_trap("any-resolver", trap);
        kani::assert(
            matches!(result, ResolverError::Timeout { .. }),
            "VP-074 F-P3-002: classify_resolver_trap must return ResolverError::Timeout \
             for Trap::Interrupt — epoch interruption is a timeout, not a WASM fault",
        );
        kani::assert(
            !matches!(result, ResolverError::Trap { .. }),
            "VP-074 F-P3-002: classify_resolver_trap must NOT return ResolverError::Trap \
             for Trap::Interrupt — epoch deadline must classify as Timeout",
        );
    }
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

    /// test_classify_resolver_trap_total_byte_iter (F-P1-011, F-P3-002)
    ///
    /// Calls `classify_resolver_trap` with every known `wasmtime::Trap` variant
    /// constructible via `Trap::from_u8`. Asserts:
    /// 1. The function NEVER returns `ResolverError::NotFound` (BC-4.12.004 INV1).
    /// 2. `Trap::Interrupt` (epoch deadline) maps to `ResolverError::Timeout` (F-P3-002).
    /// 3. Non-Interrupt traps map to `ResolverError::Trap` with non-empty `detail`.
    /// 4. The function does NOT panic for any variant (totality — VP-074).
    ///
    /// `wasmtime::Trap` in version 44 is a C-like enum with variants accessible
    /// via `Trap::from_u8(byte)`. We iterate byte values 0..=255 and test every
    /// `Some(trap)` that `from_u8` returns.
    ///
    /// Note: The `proof_` prefix is reserved for Kani harnesses (in kani_harnesses
    /// module). Rust unit tests use `test_` prefix to avoid confusion (F-P1-011).
    #[test]
    fn test_classify_resolver_trap_total_byte_iter() {
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

            // Result must never be NotFound (BC-4.12.004 INV1).
            // F-P3-002: Interrupt maps to Timeout; all others map to Trap.
            match &result {
                ResolverError::Timeout { name } => {
                    // F-P3-002: Interrupt → Timeout is the only valid Timeout mapping.
                    assert_eq!(
                        trap,
                        wasmtime::Trap::Interrupt,
                        "AC-012 / F-P3-002: Timeout must only be returned for Trap::Interrupt, \
                         not for byte={byte} trap={trap:?}"
                    );
                    assert_eq!(
                        name.as_str(),
                        resolver_name,
                        "AC-012 / F-P3-002: Timeout.name must preserve resolver_name (byte={byte})"
                    );
                }
                ResolverError::Trap { name, detail } => {
                    // Non-Interrupt traps → ResolverError::Trap.
                    assert_ne!(
                        trap,
                        wasmtime::Trap::Interrupt,
                        "AC-012 / F-P3-002: Trap::Interrupt must map to Timeout, not Trap \
                         (byte={byte})"
                    );
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
                        "AC-012 / VP-074 / F-P3-002: classify_resolver_trap returned \
                         unexpected variant for byte={byte}: {:?}",
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
    /// F-P3-002: StackOverflow is NOT Interrupt, so it still maps to Trap.
    #[test]
    fn test_BC_4_12_004_classify_resolver_trap_detail_carries_trap_description() {
        // StackOverflow is byte 0 and is always a valid TrapCode.
        let trap = wasmtime::Trap::from_u8(0)
            .expect("StackOverflow (byte 0) must be a valid wasmtime::Trap");
        assert_ne!(
            trap,
            wasmtime::Trap::Interrupt,
            "test assumption: byte 0 must not be Interrupt (Interrupt must map to Timeout per F-P3-002)"
        );

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
                    "AC-012: expected ResolverError::Trap for StackOverflow (non-Interrupt), \
                     got {:?}",
                    other
                );
            }
        }
    }

    /// test_F_P3_002_interrupt_trap_classifies_to_timeout
    ///
    /// Verifies that `Trap::Interrupt` (the epoch deadline trap) maps to
    /// `ResolverError::Timeout`, NOT `ResolverError::Trap`.
    ///
    /// F-P3-002 regression guard: if this test fails, epoch interruption is
    /// being mis-classified as a WASM execution fault.
    #[test]
    fn test_F_P3_002_interrupt_trap_classifies_to_timeout() {
        let result = classify_resolver_trap("epoch-resolver", wasmtime::Trap::Interrupt);

        match result {
            ResolverError::Timeout { name } => {
                assert_eq!(
                    name, "epoch-resolver",
                    "F-P3-002: Timeout.name must equal resolver_name"
                );
            }
            ResolverError::Trap { .. } => {
                panic!(
                    "F-P3-002: Trap::Interrupt must map to ResolverError::Timeout, \
                     not ResolverError::Trap — epoch deadline is a timeout, not a WASM fault"
                );
            }
            other => {
                panic!(
                    "F-P3-002: Trap::Interrupt must map to ResolverError::Timeout, \
                     got {:?}",
                    other
                );
            }
        }
    }
}
