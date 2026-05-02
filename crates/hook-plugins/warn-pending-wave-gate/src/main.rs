//! WASI command entry point for warn-pending-wave-gate.
//!
//! Reads `.factory/wave-state.yaml` via `host::read_file`, finds any waves
//! with `gate_status: pending`, and if found emits a `hook.block severity=warn`
//! event and writes a WAVE GATE REMINDER to stderr. Always exits 0.
//!
//! BCs: BC-7.03.091 (identity & registry binding), BC-7.03.092 (stderr warning).

use vsdd_hook_sdk::{HookPayload, HookResult};
use warn_pending_wave_gate::warn_pending_wave_gate_logic;

/// Path to the wave state file (relative to project root).
const WAVE_STATE_PATH: &str = ".factory/wave-state.yaml";

/// Maximum bytes to read from wave-state.yaml (64 KB; file is expected <10 KB).
/// `host::read_file` accepts `u32` for max_bytes.
const MAX_BYTES: u32 = 65536;

/// Timeout in milliseconds for the read_file host call.
const TIMEOUT_MS: u32 = 1000;

fn on_hook(payload: HookPayload) -> HookResult {
    warn_pending_wave_gate_logic(
        payload,
        || {
            // AC-001 / AC-004(a): read wave-state.yaml via host::read_file.
            // Returns None if absent or CapabilityDenied (EC-009).
            match vsdd_hook_sdk::host::read_file(WAVE_STATE_PATH, MAX_BYTES, TIMEOUT_MS) {
                Ok(bytes) => String::from_utf8(bytes).ok(),
                Err(_) => None,
            }
        },
        |event_type, fields| {
            vsdd_hook_sdk::host::emit_event(event_type, fields);
        },
        |msg| {
            eprint!("{}", msg);
        },
    )
}

fn main() {
    vsdd_hook_sdk::__internal::run(on_hook);
}
