//! WASI command entry point for validate-artifact-path.
//!
//! Reads the PreToolUse JSON payload from stdin via the SDK's
//! `__internal::run` trampoline, calls `on_pre_tool_use`, and exits.
//!
//! Deserialization failures (malformed JSON, missing required fields) are
//! caught by the trampoline: the hook logs a best-effort warning and exits 0
//! (graceful degradation — on_error=continue per BC-4.11.001).
//!
//! Unit-testable logic lives in `src/lib.rs` (`hook_logic`);
//! this file wires the pure logic to real host function calls.
//!
//! # Compliance notes (BC-4.11.001)
//! - HOST_ABI_VERSION = 1 (no new host functions introduced).
//! - Uses only host::read_file, host::log_*, host::emit_event (ABI v1).
//! - All block messages use HookResult::block_with_fix (canonical Why/Fix/Code).
//! - No dependency on factory-dispatcher or other workspace crates (forbidden).
//!
//! # S-13.01 Step 4 implementer
//! The `on_pre_tool_use` function in lib.rs is `todo!()`. This main.rs wires
//! the entry point to the SDK trampoline. No changes needed here unless the
//! entry-point signature changes.

use validate_artifact_path::on_pre_tool_use;

fn main() {
    vsdd_hook_sdk::__internal::run(on_pre_tool_use);
}
