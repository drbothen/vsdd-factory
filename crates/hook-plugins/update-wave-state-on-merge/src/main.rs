//! WASI command entry point for update-wave-state-on-merge.
//!
//! The `__internal::run` trampoline reads the payload from stdin, calls
//! `on_hook`, and exits. Unit tests in `src/lib.rs` drive
//! `wave_state_hook_logic` directly without requiring a WASM runtime.
//!
//! # GREEN-phase wiring
//!
//! The GREEN-phase implementer must wire the injectable callbacks to real host
//! functions:
//!
//! - `read_yaml`:  `vsdd_hook_sdk::host::read_file(".factory/wave-state.yaml", 65536, 1000)`
//! - `write_yaml`: `vsdd_hook_sdk::host::write_file(".factory/wave-state.yaml", &bytes, 65536, 10000)`
//!   (4-param form per S-8.10 v1.1 AC-1; `WriteFileCaps` capability block required
//!   in hooks-registry.toml at T-9)
//! - `emit`:       `vsdd_hook_sdk::host::emit_event("hook.action", &[...])`
//!   with fields: hook, matcher, reason, story_id, wave, total, merged, gate_transitioned
//!   (second arg is `&[(&str, &str)]` — NOT a HashMap, NOT JSON per T-6)
//!
//! # BC trace
//! BC-7.03.083 postcondition 2: always exit 0 (advisory, on_error=continue).

use update_wave_state_on_merge::{wave_state_hook_logic, WaveStateOutcome};
use vsdd_hook_sdk::{HookPayload, HookResult};

fn on_hook(payload: HookPayload) -> HookResult {
    wave_state_hook_logic(
        payload,
        // read_yaml: STUB — GREEN-phase: call host::read_file
        || {
            // GREEN-phase implementation:
            //   match vsdd_hook_sdk::host::read_file(".factory/wave-state.yaml", 65536, 1000) {
            //       Ok(bytes) => Some(String::from_utf8_lossy(&bytes).into_owned()),
            //       Err(vsdd_hook_sdk::host::HostError::CapabilityDenied) => {
            //           vsdd_hook_sdk::host::log_warn(
            //               "update-wave-state-on-merge: read_file capability denied"
            //           );
            //           None
            //       }
            //       Err(_) => None,
            //   }
            None
        },
        // write_yaml: STUB — GREEN-phase: call host::write_file
        |_yaml_str: String| {
            // GREEN-phase implementation:
            //   let bytes = yaml_str.into_bytes();
            //   if let Err(e) = vsdd_hook_sdk::host::write_file(
            //       ".factory/wave-state.yaml",
            //       &bytes,
            //       65536,
            //       10000,
            //   ) {
            //       vsdd_hook_sdk::host::emit_event(
            //           "hook.error",
            //           &[
            //               ("hook", "update-wave-state-on-merge"),
            //               ("reason", "write_failed"),
            //               ("gate_transitioned", "false"),
            //               ("error", &format!("{e:?}")),
            //           ],
            //       );
            //       eprintln!("update-wave-state-on-merge: write_file failed: {e:?}");
            //   }
        },
        // emit: STUB — GREEN-phase: call host::emit_event
        |outcome: &WaveStateOutcome, story_id: &str| {
            // GREEN-phase implementation:
            //   if let WaveStateOutcome::Appended { wave, total, merged, gate_transitioned } = outcome {
            //       if *gate_transitioned {
            //           eprintln!(
            //               "update-wave-state-on-merge: all stories in {} merged. \
            //                gate_status → pending.\n  Run the wave integration gate \
            //                before starting the next wave.",
            //               wave
            //           );
            //       }
            //       vsdd_hook_sdk::host::emit_event(
            //           "hook.action",
            //           &[
            //               ("hook", "update-wave-state-on-merge"),
            //               ("matcher", "SubagentStop"),
            //               ("reason", "wave_merge_recorded"),
            //               ("story_id", story_id),
            //               ("wave", wave.as_str()),
            //               ("total", &total.to_string()),
            //               ("merged", &merged.to_string()),
            //               ("gate_transitioned", &gate_transitioned.to_string()),
            //           ],
            //       );
            //   }
            let _ = outcome;
            let _ = story_id;
        },
    )
}

fn main() {
    vsdd_hook_sdk::__internal::run(on_hook);
}
