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
fn on_hook(payload: HookPayload) -> HookResult {
    use validate_per_story_adversary_convergence::HookCallbacks;
    use validate_per_story_adversary_convergence::IoError;
    use validate_per_story_adversary_convergence::extract_stories_from_config;
    use vsdd_hook_sdk::host;

    /// Production callback implementation.
    ///
    /// `stories` holds the story list extracted from `plugin_config.stories`
    /// (F-HIGH-3 fix). When the dispatcher populates `plugin_config.stories`
    /// before the SubagentStop event, `list_stories` returns those IDs rather
    /// than always returning Err (which previously made the hook operationally
    /// inert in every wave-gate dispatch).
    ///
    /// When `plugin_config.stories` is absent or not a string array,
    /// `list_stories` returns `Err(IoError(...))` and `hook_logic` gracefully
    /// degrades to Continue (BC-4.10.002 invariant 3).
    struct RealCallbacks {
        stories: Result<Vec<String>, IoError>,
    }

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

        fn list_stories(&self, _cycle_id: &str) -> Result<Vec<String>, IoError> {
            // F-HIGH-3 fix: return the story list extracted from plugin_config.stories.
            // The wave-gate dispatcher must populate plugin_config.stories in the
            // registry [hooks.config] table before triggering SubagentStop.
            // If absent, Err triggers graceful degrade in hook_logic (BC-4.10.002 inv-3).
            match &self.stories {
                Ok(v) => Ok(v.clone()),
                Err(e) => Err(IoError(e.0.clone())),
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

    // TODO(S-12.08 Step 3): rewire to extract_stories_from_wave_context.
    // Replace the call below with:
    //   let stories = extract_stories_from_wave_context(&payload.plugin_config);
    // and update RealCallbacks.stories type to Result<Vec<String>, WaveContextError>.
    // The new path must NOT gracefully degrade on WaveContextError::Missing —
    // it must emit HookResult::Block with code WAVE_CONTEXT_MISSING (AC-002, AC-010).
    //
    // Extract story list from plugin_config.stories before constructing callbacks.
    // If absent, RealCallbacks.stories holds Err → graceful degrade (F-HIGH-3 fix).
    let stories = extract_stories_from_config(&payload.plugin_config);
    hook_logic(&payload, &RealCallbacks { stories })
}

fn main() {
    vsdd_hook_sdk::__internal::run(on_hook);
}
