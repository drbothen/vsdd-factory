//! `vsdd-context-resolvers` — per-factory WASM context resolver crate.
//!
//! Provides the `WaveContextResolver`: a `#[resolver]`-annotated function that
//! reads `.factory/wave-state.yaml` and injects `wave_context` into
//! `plugin_config` at dispatch time.
//!
//! # Architecture
//! - **Effectful entry point:** `resolve_impl` — calls `host::read_file`, then
//!   delegates to the pure layer.
//! - **Pure computation layer:** `resolve_wave_context_pure` — takes a
//!   pre-parsed `WaveState`; used by the VP-075 proptest (AC-008).
//!
//! # Panic-free guarantee
//! No `unwrap()` or `expect()` anywhere in this crate (AC-010 / BC-4.12.004
//! INV1). All error paths return `ResolverOutput { key: ..., value: None }`.
//!
//! # Factory-agnostic invariant
//! `factory-dispatcher` has ZERO compile-time dependency on this crate
//! (BC-1.13.001 INV1 / ADR-018). The WASM artifact is loaded dynamically.

pub mod wave_context;

pub use wave_context::{WaveContext, WaveState};

use vsdd_hook_sdk::resolver as resolver_macro;
use vsdd_hook_sdk::resolver::{ResolverInput, ResolverOutput};

/// Effectful entry point for the `wave-context` resolver.
///
/// Reads `.factory/wave-state.yaml` via `host::read_file`, parses into
/// `WaveState`, then delegates to `resolve_wave_context_pure`.
///
/// Error policy (BC-4.12.004 PC3):
/// - Missing file  → `ResolverOutput { key: "wave-context", value: None }`
/// - Malformed YAML → same
/// - No active wave → same
/// Never traps. Never panics.
///
/// The `#[resolver_macro]` attribute generates the WASM `resolve()` export
/// (gated to `#[cfg(target_arch = "wasm32")]`) and a `fn main() {}` no-op
/// (BC-4.12.002 PC5).
#[allow(dead_code)]
#[resolver_macro]
pub fn resolve_impl(_input: ResolverInput) -> ResolverOutput {
    todo!()
}

/// Pure computation layer — deterministic given the same inputs (VP-075).
///
/// Does NOT call `host::read_file` or perform any I/O. Takes a pre-parsed
/// `WaveState` (the caller is responsible for file reading and parsing).
///
/// Returns `ResolverOutput { key: "wave-context", value: Some({...}) }` when
/// the `wave_state` contains an active wave with all required fields, or
/// `ResolverOutput { key: "wave-context", value: None }` otherwise.
///
/// BC-5.38.005 self-check: "If I include this real implementation, will the
/// test for this function pass trivially without any implementer work?" — Yes;
/// this function's logic determines whether AC-001 through AC-004 pass.
/// Body must remain `todo!()`.
pub fn resolve_wave_context_pure(
    _input: &ResolverInput,
    _wave_state: &WaveState,
) -> ResolverOutput {
    todo!()
}
