//! WASI command entry point for validate-per-story-adversary-convergence.
//!
//! Reads the SubagentStop JSON payload from stdin via the SDK's
//! `__internal::run` trampoline, calls `on_hook`, and exits.
//!
//! Deserialization failures (malformed JSON, missing required fields) are
//! caught by the trampoline: the hook logs a best-effort warning and exits 0
//! (graceful degradation — BC-4.10.002).
//!
//! Unit-testable logic lives in `src/lib.rs` (`hook_logic`);
//! this file wires the pure logic to real host function calls.
//!
//! # Compliance notes
//! - HOST_ABI_VERSION = 1 (BC-4.10.001 invariant 2; AC-011).
//! - No new host functions: only `host::read_file`, `host::log_*`,
//!   `host::emit_event` (ABI v1 surfaces).
//! - No host write operations — hook is read-only (AC-012).
//! - S-12.08: RealCallbacks no longer holds a `stories` field; story list is
//!   extracted from `plugin_config["wave_context"]["stories"]` inside `hook_logic`
//!   via `extract_stories_from_wave_context`. The old `list_stories` callback
//!   method and `extract_stories_from_config` helper have been removed (AC-010).

use validate_per_story_adversary_convergence::hook_logic;
use vsdd_hook_sdk::{HookPayload, HookResult};

/// HOST_ABI_VERSION declaration for this plugin (BC-4.10.001 invariant 2; AC-011).
///
/// Must equal 1 — no new host functions are introduced.
pub const HOST_ABI_VERSION: u32 = vsdd_hook_sdk::HOST_ABI_VERSION;

/// WASM entry point: wires real host functions to `hook_logic`.
///
/// The `RealCallbacks` struct implements `HookCallbacks` using the real
/// `vsdd_hook_sdk::host::*` bindings. All host I/O routes through
/// `hook_logic`'s injectable callbacks, preserving testability (AC-010).
///
/// S-12.08: `RealCallbacks` no longer holds a `stories` field. The story list
/// is read by `hook_logic` from `payload.plugin_config["wave_context"]["stories"]`
/// (injected by WaveContextResolver before dispatch). `RealCallbacks` only wires
/// `read_file`, `log_*`, and `emit_event`.
fn on_hook(payload: HookPayload) -> HookResult {
    use validate_per_story_adversary_convergence::HookCallbacks;
    use validate_per_story_adversary_convergence::IoError;
    use vsdd_hook_sdk::host;

    /// Production callback implementation.
    ///
    /// S-12.08: `stories` field removed. The story list comes from
    /// `plugin_config["wave_context"]["stories"]` (WaveContextResolver path).
    /// `read_file` is still used to read per-story adversary-convergence-state.json.
    struct RealCallbacks;

    impl HookCallbacks for RealCallbacks {
        fn read_file(&self, path: &str) -> Result<Option<String>, IoError> {
            // Use host::read_file with a generous cap (64 KiB) and 5s timeout.
            // Returns Ok(None) when the file is absent (HostError maps to None
            // for capability-denied / not-found; other errors surface as Err).
            match host::read_file(path, 65536, 5000) {
                Ok(bytes) => {
                    if bytes.is_empty() {
                        Ok(None)
                    } else {
                        match String::from_utf8(bytes) {
                            Ok(s) => Ok(Some(s)),
                            Err(e) => Err(IoError(format!("utf8 decode error: {}", e))),
                        }
                    }
                }
                Err(vsdd_hook_sdk::host::HostError::InvalidArgument) => Ok(None),
                Err(vsdd_hook_sdk::host::HostError::CapabilityDenied) => Ok(None),
                Err(e) => Err(IoError(format!("host read_file error: {:?}", e))),
            }
        }

        fn log_debug(&self, msg: &str) {
            // SDK exposes log_info as the closest equivalent to log_debug
            // (no separate log_debug in HOST_ABI v1; BC-4.10.002 PC3 amended v1.1).
            host::log_info(msg);
        }

        fn log_error(&self, msg: &str) {
            host::log_error(msg);
        }

        fn emit_event(&self, event_type: &str, fields: &[(&str, &str)]) {
            // Emit structured event via host ABI (BC-4.10.001 observability mandate;
            // BC-7.03.075 hook.block event pattern; F-CRIT-4 fix).
            // HOST_ABI_VERSION = 1 exposes host::emit_event (ABI v1 surface).
            host::emit_event(event_type, fields);
        }
    }

    // S-12.08: hook_logic now extracts story list from payload.plugin_config["wave_context"]["stories"]
    // via extract_stories_from_wave_context (called inside hook_logic). No pre-extraction here.
    hook_logic(&payload, &RealCallbacks)
}

fn main() {
    vsdd_hook_sdk::__internal::run(on_hook);
}
