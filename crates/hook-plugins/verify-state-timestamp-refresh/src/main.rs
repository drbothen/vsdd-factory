//! WASI command entry point for verify-state-timestamp-refresh.
//!
//! Reads the PreToolUse JSON payload from stdin via the SDK's
//! `__internal::run` trampoline, calls `on_pre_tool_use`, and exits.
//!
//! Deserialization failures (malformed JSON, missing required fields) are
//! caught by the trampoline: the hook logs a best-effort warning and exits 0
//! (graceful degradation — on_error=continue per ADR-025 Decision 12 §12.3).
//!
//! Unit-testable logic lives in `src/lib.rs` (`guard_logic`);
//! this file wires the pure logic to real host function calls.
//!
//! # Compliance notes (BC-5.40.001 PC4 / ADR-025 Decision 12)
//! - HOST_ABI_VERSION = 1 (no new host functions introduced).
//! - Uses only host::read_file, host::log_warn (no exec_subprocess).
//! - Block messages use HookResult::block_with_fix (canonical actionable format).
//! - async = false REQUIRED in registry entry — see ADR-019 + ADR-025 Decision 12.
//! - No dependency on factory-dispatcher or verify-factory-lock (forbidden).
//! - Trigger: file_path == ".factory/STATE.md" (exact path comparison in WASM).

use verify_state_timestamp_refresh::on_pre_tool_use;

fn main() {
    vsdd_hook_sdk::__internal::run(on_pre_tool_use);
}
