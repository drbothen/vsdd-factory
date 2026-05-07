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
//! - No `host::write_file` calls — hook is read-only (AC-012).

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
    use vsdd_hook_sdk::host;

    struct RealCallbacks;

    impl HookCallbacks for RealCallbacks {
        fn read_file(
            &self,
            path: &str,
        ) -> Result<
            Option<String>,
            validate_per_story_adversary_convergence::IoError,
        > {
            todo!("S-12.02 Step 4 — wire host::read_file to injectable callback")
        }

        fn list_stories(
            &self,
            cycle_id: &str,
        ) -> Result<
            Vec<String>,
            validate_per_story_adversary_convergence::IoError,
        > {
            todo!("S-12.02 Step 4 — enumerate story directories under .factory/cycles/<cycle-id>/")
        }

        fn log_debug(&self, msg: &str) {
            // SDK exposes log_info as the closest equivalent to log_debug
            // (no separate log_debug in HOST_ABI v1).
            host::log_info(msg);
        }

        fn log_error(&self, msg: &str) {
            host::log_error(msg);
        }
    }

    hook_logic(&payload, &RealCallbacks)
}

fn main() {
    vsdd_hook_sdk::__internal::run(on_hook);
}
