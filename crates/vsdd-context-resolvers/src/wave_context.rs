//! Wave-context YAML parsing types.
//!
//! # Design note (P-005)
//! This is the first resolver module in the `vsdd-context-resolvers` crate.
//! No sibling resolver module exists yet; patterns introduced here become the
//! convention for future resolvers:
//!   - Pure structs for deserialization (`WaveState`) and output (`WaveContext`)
//!   - All fields `Option<T>` to tolerate schema evolution (EC-004 / AC-004)
//!   - A single `pub fn parse_wave_state(yaml: &str) -> Result<WaveState, serde_yaml::Error>`
//!     entry point; callers own error handling
//!   - No `unwrap()` or `expect()` (AC-010 / BC-4.12.004 INV1)
//!
//! # Schema note
//! `WaveState` uses `Option<T>` on every field because `.factory/wave-state.yaml`
//! does not exist yet at stub time (S-12.07 bootstraps it). The implementer
//! (Step 3) MUST read the actual file schema and tighten the struct accordingly.
//! Required fields should graduate from `Option<T>` to `T` with `#[serde(default)]`
//! as the schema stabilises. See story T-1/T-2 and R-PLAT-004.

use serde::Deserialize;

/// Deserialization target for `.factory/wave-state.yaml`.
///
/// All fields are `Option<T>` to survive schema evolution (EC-004 / AC-004).
/// The implementer (Step 3) must align this struct with the actual YAML schema
/// discovered in T-1. Fields may be promoted to non-Optional once the schema is
/// confirmed stable.
// `Default` is GREEN-BY-DESIGN: it is a pure derive with no logic. It is needed
// by AC-002b test (`WaveState::default()` models the post-parse-failure path where
// resolve_impl substitutes an all-None WaveState). No implementer work required.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct WaveState {
    /// Current pipeline cycle identifier (e.g. `"v1.0-feature-engine-discipline-pass-1"`).
    pub current_cycle: Option<String>,
    /// Current active wave identifier (e.g. `"wave-3"`).
    pub current_wave: Option<String>,
    /// Story IDs scheduled in the active wave.
    pub stories: Option<Vec<String>>,
}

/// Pure output type for the `wave_context` JSON payload injected into `plugin_config`.
///
/// Constructed from a valid `WaveState` by `resolve_wave_context_pure`. Serialized
/// to JSON for `ResolverOutput.value`. (BC-4.12.002 PC3, AC-001)
#[derive(Debug, Clone)]
pub struct WaveContext {
    /// Cycle identifier forwarded from `WaveState::current_cycle`.
    pub cycle_id: String,
    /// Wave identifier forwarded from `WaveState::current_wave`.
    pub wave_id: String,
    /// Story list forwarded from `WaveState::stories`.
    pub stories: Vec<String>,
}

/// Parse a YAML string into a `WaveState`.
///
/// Returns `Err(serde_yaml::Error)` on malformed YAML; callers map errors to
/// `ResolverOutput { value: None }` (AC-002, EC-003). Does NOT panic.
/// BC-4.12.004 INV1: no `unwrap()` or `expect()`.
pub fn parse_wave_state(_yaml: &str) -> Result<WaveState, serde_yaml::Error> {
    todo!()
}
