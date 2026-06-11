//! WASI command entry point for verify-factory-lock.
//!
//! Reads the PreToolUse JSON payload from stdin via the SDK's
//! `__internal::run` trampoline, calls `on_pre_tool_use`, and exits.
//!
//! Deserialization failures (malformed JSON, missing required fields) are
//! caught by the trampoline: the hook logs a best-effort warning and exits 0
//! (graceful degradation — on_error=continue per BC-4.13.001 PC8).
//!
//! Unit-testable logic lives in `src/lib.rs` (`guard_logic`);
//! this file wires the pure logic to real host function calls.
//!
//! # Compliance notes (BC-4.13.001)
//! - HOST_ABI_VERSION = 1 (no new host functions introduced; ADR-025 Decision 1).
//! - Uses only host::read_file, host::exec_subprocess, host::log_warn (ABI v1).
//! - Block messages use HookResult::Block (block_intent = true; exit code 2).
//! - async = false REQUIRED in both registry entries — see ADR-019 + ADR-025.
//! - No dependency on factory-dispatcher or other workspace crates (forbidden).

use verify_factory_lock::on_pre_tool_use;

fn main() {
    vsdd_hook_sdk::__internal::run(on_pre_tool_use);
}
