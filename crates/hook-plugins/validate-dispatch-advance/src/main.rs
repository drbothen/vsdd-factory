//! WASI command entry point for validate-dispatch-advance.
//!
//! Reads the PostToolUse JSON payload from stdin via the SDK's
//! `__internal::run` trampoline, calls `on_post_tool_use`, and exits.
//!
//! Deserialization failures (malformed JSON, missing required fields) are
//! caught by the trampoline: the hook logs a best-effort warning and exits 0
//! (graceful degradation).
//!
//! Unit-testable logic lives in `src/lib.rs` (`on_post_tool_use`);
//! this file wires the pure logic to real host function calls.
//!
//! # Compliance notes (BC-5.39.006)
//! - HOST_ABI_VERSION = 1 (no new host functions introduced).
//! - Uses only host::read_file, host::log_* (ABI v1).
//! - All block messages use HookResult::block_with_fix (canonical Why/Fix/Code).
//! - No dependency on factory-dispatcher or other workspace crates (forbidden).
//! - `tool = "Edit|Write"` is the canonical Q5/Q6 form for this hook's registry entry.

use validate_dispatch_advance::on_post_tool_use;

fn main() {
    vsdd_hook_sdk::__internal::run(on_post_tool_use);
}
