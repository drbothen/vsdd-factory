//! warn-pending-wave-gate — Stop lifecycle WASM hook plugin.
//!
//! At session end, reads `.factory/wave-state.yaml` via `host::read_file`,
//! scans all waves for `gate_status: pending`, and if any are found emits a
//! `hook.block severity=warn` event and writes a WAVE GATE REMINDER to stderr.
//! Always exits 0 (advisory only — never blocks session end).
//!
//! BCs: BC-7.03.091 (identity & registry binding), BC-7.03.092 (stderr warning).
//!
//! # Schema alignment (F-P3-001 fix)
//!
//! The canonical producer (`update-wave-state-on-merge`) writes wave-state.yaml
//! in SEQUENCE form:
//!
//! ```yaml
//! waves:
//!   - wave: "W-15"
//!     gate_status: pending
//! ```
//!
//! This hook previously read the MAPPING form (waves: { W-15: { ... } }),
//! making it operationally inert in production (TD-073 — now resolved).
//! Fixed in F-P3-001 fix-burst: use `WaveEntry` / `WaveState` structs via
//! serde_yaml to parse the canonical SEQUENCE form.
//!
//! Porting note: the bash source used python3 for YAML parsing. This crate
//! uses serde_yaml 0.9.34 instead — no subprocess, no python3 dependency.

use serde::Deserialize;
use vsdd_hook_sdk::{HookPayload, HookResult};

/// Represents a single wave entry in `.factory/wave-state.yaml`.
///
/// Matches the canonical SEQUENCE schema produced by `update-wave-state-on-merge`
/// (same struct layout as `vsdd-context-resolvers::wave_context::WaveEntry`).
#[derive(Debug, Clone, Deserialize)]
struct WaveEntry {
    /// Wave identifier (e.g., "W-15", "F4").
    pub wave: String,
    /// Gate status — None means not yet set (wave is not in a terminal state).
    #[serde(default)]
    pub gate_status: Option<String>,
}

/// Top-level `.factory/wave-state.yaml` structure (canonical SEQUENCE form).
#[derive(Debug, Clone, Default, Deserialize)]
struct WaveState {
    /// All waves in the pipeline, in order from earliest to latest.
    #[serde(default)]
    pub waves: Vec<WaveEntry>,
}

/// Top-level hook logic. Reads wave-state.yaml, finds pending waves, and
/// emits the advisory warning if any are found.
///
/// Separated from `main()` so unit tests can drive it without a WASM runtime.
///
/// BC-7.03.091 postcondition 2: all early-exit paths are silent (no stderr,
/// no emit_event, exit 0).
/// BC-7.03.092 postcondition 1: pending waves found → emit hook.block
/// severity=warn + stderr WAVE GATE REMINDER.
pub fn warn_pending_wave_gate_logic(
    _payload: HookPayload,
    read_wave_state: impl FnOnce() -> Option<String>,
    emit: impl FnOnce(&str, &[(&str, &str)]),
    write_stderr: impl FnOnce(&str),
) -> HookResult {
    // AC-004(a): wave-state.yaml absent → exit 0, no output.
    let yaml_content = match read_wave_state() {
        Some(content) => content,
        None => return HookResult::Continue,
    };

    // AC-004(b): YAML parse fails or `waves` key absent → exit 0, no output.
    // Parse into WaveState (canonical SEQUENCE form per F-P3-001 fix).
    // serde_yaml returns Err on malformed YAML, and WaveState::default() gives
    // empty waves list for missing-key or null-waves cases via #[serde(default)].
    let wave_state: WaveState = match serde_yaml::from_str(&yaml_content) {
        Ok(ws) => ws,
        Err(_) => return HookResult::Continue,
    };

    // Scan for waves with gate_status == "pending".
    // EC-008: gate_status is Option<String>; None means absent (not pending).
    let mut pending: Vec<String> = Vec::new();
    for entry in &wave_state.waves {
        if entry.gate_status.as_deref() == Some("pending") {
            pending.push(entry.wave.clone());
        }
    }

    // AC-004(c): no pending waves → exit 0, no output.
    if pending.is_empty() {
        return HookResult::Continue;
    }

    // AC-003: pending waves found — emit hook.block severity=warn.
    let comma_joined = pending.join(",");
    emit(
        "hook.block",
        &[
            ("hook", "warn-pending-wave-gate"),
            ("matcher", "Stop"),
            ("reason", "pending_wave_gate_at_session_end"),
            ("severity", "warn"),
            ("pending_waves", &comma_joined),
        ],
    );

    // AC-003: write WAVE GATE REMINDER to stderr.
    // Format matches bash source exactly (blank lines, indentation, hint line).
    let mut msg = String::new();
    msg.push('\n');
    msg.push_str("WAVE GATE REMINDER:\n");
    for wave in &pending {
        msg.push_str(&format!(
            "  - {} gate is pending. Run the gate before starting the next wave.\n",
            wave
        ));
    }
    msg.push('\n');
    msg.push_str("  Invoke /vsdd-factory:wave-gate or update .factory/wave-state.yaml\n");
    msg.push_str(
        "  with gate_status: passed (after running checks) or deferred (with rationale).\n",
    );
    write_stderr(&msg);

    // Always exit 0 (BC-7.03.091 postcondition 2 — advisory only).
    HookResult::Continue
}
