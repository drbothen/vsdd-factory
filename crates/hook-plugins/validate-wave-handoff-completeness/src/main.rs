//! WASI command entry point for validate-wave-handoff-completeness.
//!
//! Reads the PostToolUse JSON payload from stdin via the SDK's
//! `__internal::run` trampoline, calls `on_post_tool_use`, and exits.
//!
//! Deserialization failures (malformed JSON, missing required fields) are
//! caught by the trampoline: the hook logs a best-effort warning and exits 0
//! (graceful degradation — `on_error = "continue"` per BC-4.14.001 PC6).
//!
//! Unit-testable logic lives in `src/lib.rs` (`check_handoff_completeness`
//! and `on_post_tool_use`); this file wires the pure logic to real host
//! function calls via the SDK trampoline.
//!
//! # Compliance notes (BC-4.14.001)
//! - `on_error = "continue"` in the registry: WASM crash → fail-open Continue.
//! - HOST_ABI_VERSION = 1 (no new host functions introduced).
//! - Uses only host::read_file, host::log_* (ABI v1).
//! - No dependency on factory-dispatcher or other workspace crates (forbidden).
//! - Pure-parse: no filesystem access, no process spawning (INV1).

use validate_wave_handoff_completeness::on_post_tool_use;

fn main() {
    vsdd_hook_sdk::__internal::run(on_post_tool_use);
}
