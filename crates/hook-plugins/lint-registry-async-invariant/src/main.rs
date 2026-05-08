//! WASI command entry point for lint-registry-async-invariant.
//!
//! Reads the PreToolUse JSON payload from stdin via the SDK's
//! `__internal::run` trampoline, calls `on_pre_tool_use`, and exits.
//!
//! Deserialization failures (malformed JSON, missing required fields) are
//! caught by the trampoline: the hook logs a best-effort warning and exits 0
//! (graceful degradation).
//!
//! Unit-testable logic lives in `src/lib.rs` (`lint_logic`);
//! this file wires the pure logic to real host function calls.
//!
//! # Compliance notes (BC-7.06.001)
//! - HOST_ABI_VERSION = 1 (no new host functions introduced).
//! - Uses only host::read_file, host::log_*, host::emit_event (ABI v1).
//! - All block messages use HookResult::block_with_fix (canonical Why/Fix/Code).
//! - No dependency on factory-dispatcher or other workspace crates (forbidden).
//! - WASM-migration rule (Decision 5): native WASM Rust crate, NOT bash via
//!   legacy-bash-adapter.

use lint_registry_async_invariant::on_pre_tool_use;

fn main() {
    vsdd_hook_sdk::__internal::run(on_pre_tool_use);
}
