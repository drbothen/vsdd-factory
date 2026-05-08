//! Pure aggregation of sync_group plugin results into the dispatcher exit code.
//!
//! Per BC-1.14.001 PC5, the dispatcher exit code is computed solely from sync_group
//! plugin results. Async-group results do NOT influence exit code (Invariant 3 +
//! EC-012). This separation is enforced by `aggregate_exit_code` taking
//! `&[PluginResult]` (caller responsible for passing only sync_group results).
//!
//! # BC traces
//! - BC-1.14.001 PC5 — sync_group exit-code aggregation
//! - BC-1.14.001 Invariant 3 — async group excluded from exit-code model
//! - VP-077 H5 — exit-code independence proof (structural: async not accepted)
//! - VP-077 H6 — aggregation correctness proof
//! - ADR-019 — async semantics at registry layer
//! - DI-019 — ASYNC_DRAIN_WINDOW_MS (invariant anchor for async-group exclusion)

use crate::registry::OnError;

/// A sync_group plugin result as produced by the sync dispatch loop.
///
/// This type is intentionally minimal and Kani-friendly: only the two fields
/// that participate in block-intent detection are present. Diagnostic fields
/// (plugin_name, stdout, stderr, etc.) live in [`crate::invoke::PluginResult`]
/// and [`crate::executor::PluginOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginResult {
    /// WASI exit code returned by the plugin process.
    pub exit_code: u8,
    /// Error-handling policy declared in the registry entry for this plugin.
    pub on_error: OnError,
}

/// Aggregate sync_group results into the dispatcher exit code
/// per BC-1.14.001 PC5 and ADR-019 async-semantics invariant.
///
/// Returns `2` if any sync_group plugin returned (`exit_code == 2`, `on_error == Block`).
/// Returns `0` otherwise.
///
/// async_group results are NOT accepted as a parameter and are NOT considered.
/// This structural exclusion is the mechanistic proof of BC-1.14.001 Invariant 3
/// ("Async group plugins are excluded from the exit-code aggregation").
///
/// # Purity contract
///
/// This function is **pure**: no I/O, no globals, no async executor calls,
/// no logging side effects. The function takes a slice and returns a `u8`.
/// Any future modification that introduces side effects must remove the Kani
/// proof harnesses H5/H6 (VP-077) or restructure to maintain a pure inner function.
///
/// # BC traces
/// - BC-1.14.001 PC5 — dispatcher exit code determined by sync_group only
/// - BC-1.14.001 Invariant 3 — async group excluded
/// - VP-077 H5 — exit-code independence (structural)
/// - VP-077 H6 — aggregation correctness
/// - ADR-019 — async semantics at registry layer
/// - DI-019 — ASYNC_DRAIN_WINDOW_MS (drain window constant; cited as invariant
///   source for async-group exclusion semantics)
pub fn aggregate_exit_code(sync_results: &[PluginResult]) -> u8 {
    sync_results
        .iter()
        .any(|r| r.exit_code == 2 && r.on_error == OnError::Block)
        .then_some(2)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::OnError;

    #[test]
    fn empty_sync_results_returns_0() {
        assert_eq!(aggregate_exit_code(&[]), 0);
    }

    #[test]
    fn single_block_result_returns_2() {
        let results = vec![PluginResult {
            exit_code: 2,
            on_error: OnError::Block,
        }];
        assert_eq!(aggregate_exit_code(&results), 2);
    }

    #[test]
    fn exit_2_with_continue_policy_returns_0() {
        // exit_code=2 alone is not a block intent without on_error=Block
        let results = vec![PluginResult {
            exit_code: 2,
            on_error: OnError::Continue,
        }];
        assert_eq!(aggregate_exit_code(&results), 0);
    }

    #[test]
    fn exit_0_with_block_policy_returns_0() {
        // on_error=Block without exit_code=2 is not a block intent
        let results = vec![PluginResult {
            exit_code: 0,
            on_error: OnError::Block,
        }];
        assert_eq!(aggregate_exit_code(&results), 0);
    }

    #[test]
    fn mixed_results_any_block_returns_2() {
        let results = vec![
            PluginResult {
                exit_code: 0,
                on_error: OnError::Continue,
            },
            PluginResult {
                exit_code: 2,
                on_error: OnError::Block,
            },
            PluginResult {
                exit_code: 0,
                on_error: OnError::Block,
            },
        ];
        assert_eq!(aggregate_exit_code(&results), 2);
    }

    #[test]
    fn no_blocking_result_returns_0() {
        let results = vec![
            PluginResult {
                exit_code: 0,
                on_error: OnError::Continue,
            },
            PluginResult {
                exit_code: 1,
                on_error: OnError::Continue,
            },
        ];
        assert_eq!(aggregate_exit_code(&results), 0);
    }

    /// BC-3.08.001 Invariant 5 (trace_id wire format) — zero-occurrence test:
    /// aggregate_exit_code is a pure function; it produces a u8, not a JSON string.
    /// This test asserts the function is not doing any serialization (it cannot
    /// accidentally produce "dispatcher_trace_id" in its output).
    #[test]
    fn aggregate_exit_code_is_pure_u8_no_serialization() {
        let result = aggregate_exit_code(&[PluginResult {
            exit_code: 2,
            on_error: OnError::Block,
        }]);
        // The output is a raw u8 — not a JSON string that could contain field names.
        assert_eq!(result, 2u8);
    }
}
