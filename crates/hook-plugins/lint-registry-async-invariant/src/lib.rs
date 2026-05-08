//! lint-registry-async-invariant — PreToolUse WASM hook plugin.
//!
//! CI lint enforcement for BC-7.06.001:
//! - Registry `schema_version` MUST be 2; v1 (or absent) is rejected.
//! - For every `[[hooks]]` entry, if `on_error = "block"` then `async` MUST be
//!   absent or `async = false`. Any entry violating this invariant is an
//!   `E-REG-002` hard error.
//!
//! This is a native WASM Rust crate per Decision 5 (WASM-migration rule).
//! Do NOT implement via legacy-bash-adapter.
//!
//! # Behavioral Contracts
//!
//! - BC-7.06.001 v1.3: hooks-registry.toml schema_version 2 — per-plugin
//!   `async: bool` field with CI lint invariant `on_error = "block"` implies
//!   `async = false`. Three-layer lint defense.
//!
//! # Architecture compliance
//!
//! - HOST_ABI_VERSION = 1 (no new host functions introduced).
//! - Pure `fn lint_logic(...)` takes all host I/O as injectable closures.
//!   Unit tests exercise every branch without a WASM runtime.
//! - ASYNC_DRAIN_WINDOW_MS referenced via DI-019; do NOT hardcode the value.

// Allow `#[cfg(kani)]` without triggering unexpected_cfgs warning.
#![cfg_attr(not(kani), allow(unexpected_cfgs))]

use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// ABI version constant
// ---------------------------------------------------------------------------

/// HOST_ABI_VERSION declares the ABI contract version this plugin was built
/// against. The dispatcher reads this before any host call. Must remain 1.
pub const HOST_ABI_VERSION: u32 = 1;

/// Path to the hooks-registry.toml (relative to CLAUDE_PROJECT_DIR).
pub const REGISTRY_PATH: &str = "plugins/vsdd-factory/hooks-registry.toml";

// ---------------------------------------------------------------------------
// Error codes (BC-7.06.001)
// ---------------------------------------------------------------------------

/// Hard error code for schema_version mismatch (schema_version != 2).
/// Emits `dispatcher.schema_mismatch` event.
pub const E_REG_001: &str = "E-REG-001";

/// Hard error code for on_error=block AND async=true coexistence.
/// Emits `dispatcher.registry_invalid` event.
pub const E_REG_002: &str = "E-REG-002";

// ---------------------------------------------------------------------------
// Lint result type
// ---------------------------------------------------------------------------

/// Result of running the lint checks against a registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LintResult {
    /// All invariants satisfied.
    Pass,
    /// schema_version field is missing or not equal to 2 (E-REG-001).
    SchemaMismatch { got: Option<u32> },
    /// At least one entry has on_error=block AND async=true (E-REG-002).
    InvariantViolation { plugin_name: String },
}

// ---------------------------------------------------------------------------
// Pure core lint function (injectable-callback pattern)
// ---------------------------------------------------------------------------

/// All side-effecting callbacks injected into `lint_logic` for testability.
pub struct LintCallbacks<R, E, L>
where
    R: FnOnce(&str) -> Result<String, String>,
    E: FnMut(&str, &[(&str, &str)]),
    L: FnMut(u8, &str),
{
    /// Read a file by path; returns `Ok(contents)` or `Err(message)` if absent/error.
    pub read_file: R,
    /// Emit an event (type, fields).
    pub emit_event: E,
    /// Log a message at the given level (0=trace, 1=debug, 2=info, 3=warn, 4=error).
    pub log: L,
}

/// Core lint-registry-async-invariant hook logic.
///
/// Reads hooks-registry.toml, asserts schema_version == 2, then checks that
/// no entry combines on_error = "block" with async = true. On violation,
/// returns `HookResult::block_with_fix` with the appropriate error code.
///
/// All host I/O is injected via `callbacks` so unit tests can exercise every
/// branch without a WASM runtime.
///
/// # BC traces
/// - BC-7.06.001 Invariant 1: on_error=block implies async=false
/// - BC-7.06.001 postcondition 1: schema_version=2 required
/// - BC-7.06.001 postcondition 2: per-plugin async field with serde-default
/// - DI-019: ASYNC_DRAIN_WINDOW_MS — cite by reference, do NOT hardcode value
pub fn lint_logic<R, E, L>(
    _payload: HookPayload,
    _callbacks: LintCallbacks<R, E, L>,
) -> HookResult
where
    R: FnOnce(&str) -> Result<String, String>,
    E: FnMut(&str, &[(&str, &str)]),
    L: FnMut(u8, &str),
{
    todo!("T-3i: implement lint invariant checks — read hooks-registry.toml, assert schema_version=2, check on_error=block implies async=false (BC-7.06.001)")
}

/// Parse the hooks-registry TOML and return the raw lint result.
///
/// This is the pure analysis layer. No I/O — caller provides the raw TOML text.
/// Kani-compatible: no I/O, no globals, no branching beyond match on parsed fields.
///
/// # BC traces
/// - BC-7.06.001 postcondition 1 + Invariant 1
/// - VP-078 Harness 1: lint-invariant
pub fn run_lint(toml_text: &str) -> LintResult {
    todo!("T-3f/T-3i: parse hooks-registry TOML text, check schema_version=2, check on_error=block implies async=false")
}

// ---------------------------------------------------------------------------
// Top-level entry point (wired to real host fns in main.rs)
// ---------------------------------------------------------------------------

/// Called from the WASI entry point in `main.rs`.
///
/// Wires the real vsdd_hook_sdk host functions to the injectable-callback
/// surface of `lint_logic`.
pub fn on_pre_tool_use(payload: HookPayload) -> HookResult {
    use vsdd_hook_sdk::host;

    lint_logic(
        payload,
        LintCallbacks {
            read_file: |path| match host::read_file(path, 65536, 5000) {
                Ok(bytes) => String::from_utf8(bytes).map_err(|e| e.to_string()),
                Err(e) => Err(format!("{:?}", e)),
            },
            emit_event: |event_type, fields| {
                host::emit_event(event_type, fields);
            },
            log: |level, msg| match level {
                0..=2 => host::log_info(msg),
                3 => host::log_warn(msg),
                _ => host::log_error(msg),
            },
        },
    )
}
