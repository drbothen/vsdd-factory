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
    payload: HookPayload,
    mut callbacks: LintCallbacks<R, E, L>,
) -> HookResult
where
    R: FnOnce(&str) -> Result<String, String>,
    E: FnMut(&str, &[(&str, &str)]),
    L: FnMut(u8, &str),
{
    // Only lint when the changed file is hooks-registry.toml (PostToolUse Edit/Write).
    // Edit/Write tool_input uses "file_path" key (not "path").
    let tool_input = payload.tool_input;
    let changed_path = tool_input
        .get("file_path")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if !changed_path.ends_with("hooks-registry.toml") {
        (callbacks.log)(0, "lint-registry-async-invariant: skipping — not hooks-registry.toml");
        return HookResult::Continue;
    }

    (callbacks.log)(2, "lint-registry-async-invariant: checking hooks-registry.toml");

    // Read the registry file via the host read_file capability.
    let toml_text = match (callbacks.read_file)(REGISTRY_PATH) {
        Ok(text) => text,
        Err(e) => {
            (callbacks.log)(3, &format!("lint-registry-async-invariant: cannot read {REGISTRY_PATH}: {e}"));
            // Best-effort: if we can't read the file, don't block (fail-open).
            return HookResult::Continue;
        }
    };

    match run_lint(&toml_text) {
        LintResult::Pass => {
            (callbacks.log)(2, "lint-registry-async-invariant: PASS — registry invariants satisfied");
            HookResult::Continue
        }
        LintResult::SchemaMismatch { got } => {
            let got_str = got.map(|v| v.to_string()).unwrap_or_else(|| "missing".to_string());
            (callbacks.emit_event)(
                "dispatcher.schema_mismatch",
                &[
                    ("found_version", got_str.as_str()),
                    ("expected_version", "2"),
                    ("error_code", E_REG_001),
                ],
            );
            HookResult::block_with_fix(
                "lint-registry-async-invariant",
                format!("hooks-registry.toml has schema_version={got_str}, expected 2 (E-REG-001)"),
                "Update schema_version = 2 in hooks-registry.toml",
                E_REG_001,
            )
        }
        LintResult::InvariantViolation { plugin_name } => {
            (callbacks.emit_event)(
                "dispatcher.registry_invalid",
                &[
                    ("offending_plugin", plugin_name.as_str()),
                    ("violation", "on_error_block_with_async_true"),
                    ("error_code", E_REG_002),
                ],
            );
            HookResult::block_with_fix(
                "lint-registry-async-invariant",
                format!("plugin '{plugin_name}' has on_error=block AND async=true (E-REG-002)"),
                format!("Remove async=true from '{plugin_name}' or change on_error to 'continue'"),
                E_REG_002,
            )
        }
    }
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
    // Parse the TOML text into a dynamic value.
    let table: toml::Table = match toml::from_str(toml_text) {
        Ok(t) => t,
        Err(_) => {
            // Unparseable TOML — treat as schema missing.
            return LintResult::SchemaMismatch { got: None };
        }
    };

    // Check schema_version == 2 (BC-7.06.001 postcondition 1).
    let schema_version = table
        .get("schema_version")
        .and_then(|v| v.as_integer());

    match schema_version {
        Some(2) => {} // OK
        Some(v) => return LintResult::SchemaMismatch { got: Some(v as u32) },
        None => return LintResult::SchemaMismatch { got: None },
    }

    // Check on_error=block implies async=false for all [[hooks]] entries
    // (BC-7.06.001 Invariant 1).
    if let Some(toml::Value::Array(hooks)) = table.get("hooks") {
        for hook in hooks {
            if let toml::Value::Table(entry) = hook {
                let on_error_is_block = entry
                    .get("on_error")
                    .and_then(|v| v.as_str())
                    == Some("block");

                let async_is_true = entry
                    .get("async")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);

                if on_error_is_block && async_is_true {
                    let name = entry
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("<unknown>")
                        .to_string();
                    return LintResult::InvariantViolation { plugin_name: name };
                }
            }
        }
    }

    LintResult::Pass
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
