//! WASI command entry point for warn-pending-wave-gate.
//!
//! Reads `.factory/wave-state.yaml` via `host::read_file`, finds any waves
//! with `gate_status: pending`, and if found emits a `hook.block severity=warn`
//! event and writes a WAVE GATE REMINDER to stderr. Always exits 0.
//!
//! BCs: BC-7.03.091 (identity & registry binding), BC-7.03.092 (stderr warning).

use vsdd_hook_sdk::{HookPayload, HookResult};
use warn_pending_wave_gate::warn_pending_wave_gate_logic_with_error_dispatch;

/// Path to the wave state file (relative to project root).
const WAVE_STATE_PATH: &str = ".factory/wave-state.yaml";

/// Maximum bytes to read from wave-state.yaml via `host::read_file`.
///
/// Set to 512 KiB (524288 bytes) — consistent with the sibling cap used by
/// `validate-state-structure` (F-P5-002) and `validate-dispatch-advance`.
/// wave-state.yaml is currently small (<10 KiB) but this constant aligns with
/// the project-wide convention established after F-PASS15 sibling-site sweep.
const MAX_BYTES: u32 = 524_288;

/// Timeout in milliseconds for the read_file host call.
const TIMEOUT_MS: u32 = 1000;

fn on_hook(payload: HookPayload) -> HookResult {
    // S-19.03 (AC-004): use error-dispatch variant to distinguish NotFound (silent
    // Continue) from CapabilityDenied (operator WARN). The old Err(_) => None collapse
    // in warn_pending_wave_gate_logic masked genuine capability misconfigurations.
    warn_pending_wave_gate_logic_with_error_dispatch(
        payload,
        || vsdd_hook_sdk::host::read_file(WAVE_STATE_PATH, MAX_BYTES, TIMEOUT_MS),
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
