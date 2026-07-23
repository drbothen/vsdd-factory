//! WASI command entry point for validate-factory-path-staging.
//!
//! Reads the PreToolUse JSON payload from stdin via the SDK's
//! `__internal::run` trampoline, calls `on_pre_tool_use`, and exits.
//!
//! Deserialization failures (malformed JSON, missing required fields) are
//! caught by the trampoline: the hook logs a best-effort warning and exits 0
//! (graceful degradation — on_error=continue per BC-4.16.001 Invariant 2).
//!
//! Unit-testable logic lives in `src/lib.rs` (`hook_logic`);
//! this file wires the pure logic to real host function calls.
//!
//! # Compliance notes (BC-4.16.001)
//! - HOST_ABI_VERSION = 1 (no new host functions introduced).
//! - Uses only host::exec_subprocess, host::log_*, host::emit_event (ABI v1).
//! - Block messages use HookResult::block_with_fix (canonical Why/Fix/Code).
//! - No dependency on factory-dispatcher or other workspace crates (forbidden).
//! - ADR-031 §Decision 3: distinct crate from validate-artifact-path.

use validate_factory_path_staging::on_pre_tool_use;

fn main() {
    vsdd_hook_sdk::__internal::run(on_pre_tool_use);
}
