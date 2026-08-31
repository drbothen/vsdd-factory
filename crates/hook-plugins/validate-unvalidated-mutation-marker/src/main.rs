//! WASI command entry point for validate-unvalidated-mutation-marker.
//!
//! Reads the PreToolUse JSON payload from stdin via the SDK's `__internal::run`
//! trampoline, calls `on_pre_tool_use`, and exits.
//!
//! Both Arm 1 (`^Agent$`) and Arm 2 (`^Bash$` git filter) dispatch to the same
//! `on_pre_tool_use` function because both hooks-registry.toml entries reference
//! this IDENTICAL compiled binary (BC-1.18.002 invariant 1; AC-019).
//!
//! # Compliance notes (BC-1.18.002)
//!
//! - HOST_ABI_VERSION = 1 (no new host functions introduced by S-25.01).
//! - `failure_policy = "fail-open"` for BOTH registry entries (invariant 2).
//!   The gate cannot self-lock: if this plugin itself fuel-exhausts, the gate
//!   is fail-open and the dispatch proceeds (EC-003).
//! - No dependency on factory-dispatcher or other workspace crates (forbidden
//!   for WASM hook plugins).

use validate_unvalidated_mutation_marker::on_pre_tool_use;

fn main() {
    vsdd_hook_sdk::__internal::run(on_pre_tool_use);
}
