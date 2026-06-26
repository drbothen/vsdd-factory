//! WASI command entry point for validate-heavy-op-delegation.
//!
//! Reads the PreToolUse JSON payload from stdin via the SDK's
//! `__internal::run` trampoline, calls `on_pre_tool_use`, and exits.
//!
//! Deserialization failures (malformed JSON, missing required fields) are
//! caught by the trampoline: the hook logs a best-effort warning and exits 0
//! (graceful degradation — `on_error = "continue"` per BC-4.15.001 PC-C).
//!
//! Unit-testable logic lives in `src/lib.rs` (`evaluate_patterns`,
//! `truncate_command_preview`, `build_recommendation_message`, and
//! `on_pre_tool_use`); this file wires the pure logic to real host function
//! calls via the SDK trampoline.
//!
//! # Compliance notes (BC-4.15.001)
//! - `on_error = "continue"` in the registry: WASM crash → fail-open Continue.
//! - HOST_ABI_VERSION = 1 (no new host functions introduced).
//! - Uses only host::log_warn, host::plugin_log (ABI v1; no host::read_file
//!   — pure-parse INV1 requires no filesystem reads).
//! - No dependency on factory-dispatcher or other workspace crates (forbidden).
//! - Pure-parse: no filesystem access, no process spawning (INV1).
//! - Never blocks: block_intent is ALWAYS false (INV2).

use validate_heavy_op_delegation::on_pre_tool_use;

fn main() {
    vsdd_hook_sdk::__internal::run(on_pre_tool_use);
}
