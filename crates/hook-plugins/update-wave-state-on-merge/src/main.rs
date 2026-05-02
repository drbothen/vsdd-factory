//! WASI command entry point for update-wave-state-on-merge.
//!
//! Two compilation modes are supported:
//!
//! ## Production mode (no default features, `--no-default-features`)
//!
//! Uses the full dispatcher host-function ABI:
//! - `read_yaml`:  `vsdd_hook_sdk::host::read_file(".factory/wave-state.yaml", 65536, 1000)`
//! - `write_yaml`: `vsdd_hook_sdk::host::write_file(".factory/wave-state.yaml", &bytes, 65536, 10000)`
//!   (4-param form per S-8.10 v1.1 AC-1; `WriteFileCaps` capability block required
//!   in hooks-registry.toml)
//! - `emit`:       `vsdd_hook_sdk::host::emit_event("hook.action", &[...])`
//!   with fields: hook, matcher, reason, story_id, wave, total, merged, gate_transitioned
//!
//! ## Standalone / test mode (default features, `feature = "standalone"`)
//!
//! Uses WASI std::fs file I/O for testing with plain `wasmtime run` (no dispatcher).
//! File path is resolved by:
//!   1. `VSDD_WAVE_STATE_PATH` env var (if passed via `--env`)
//!   2. Enumerating WASI preopened directories (fd 3+) for `wave-state.yaml`
//!   3. Production fallback `.factory/wave-state.yaml`
//!
//! The bats parity tests use:
//!   `env VSDD_WAVE_STATE_PATH="${wave_state}" wasmtime run --dir "${BATS_TEST_TMPDIR}" "${WASM_BIN}"`
//!
//! wasmtime 44 does NOT forward process env vars to WASM via the `env` prefix. The
//! preopened-dir enumeration approach (step 2) finds `wave-state.yaml` in BATS_TEST_TMPDIR
//! by trying `${preopen_dir}/wave-state.yaml` for each preopened dir.
//!
//! # BC trace
//! BC-7.03.083 postcondition 2: always exit 0 (advisory, on_error=continue).

use update_wave_state_on_merge::{WaveStateOutcome, wave_state_hook_logic};
use vsdd_hook_sdk::HookPayload;
#[cfg(not(feature = "standalone"))]
use vsdd_hook_sdk::HookResult;

// ---------------------------------------------------------------------------
// Production entry point (no standalone feature)
// ---------------------------------------------------------------------------

#[cfg(not(feature = "standalone"))]
fn on_hook(payload: HookPayload) -> HookResult {
    wave_state_hook_logic(
        payload,
        // read_yaml: read .factory/wave-state.yaml via host read_file
        || match vsdd_hook_sdk::host::read_file(".factory/wave-state.yaml", 65536, 1000) {
            Ok(bytes) => Some(String::from_utf8_lossy(&bytes).into_owned()),
            Err(vsdd_hook_sdk::host::HostError::CapabilityDenied) => {
                vsdd_hook_sdk::host::log_warn(
                    "update-wave-state-on-merge: read_file capability denied",
                );
                None
            }
            Err(_) => None,
        },
        // write_yaml: write updated YAML back via host write_file (EC-005: advisory)
        |yaml_str: String| {
            let bytes = yaml_str.into_bytes();
            if let Err(e) =
                vsdd_hook_sdk::host::write_file(".factory/wave-state.yaml", &bytes, 65536, 10000)
            {
                vsdd_hook_sdk::host::emit_event(
                    "hook.error",
                    &[
                        ("hook", "update-wave-state-on-merge"),
                        ("reason", "write_failed"),
                        ("gate_transitioned", "false"),
                        ("error", &format!("{e:?}")),
                    ],
                );
                eprintln!("update-wave-state-on-merge: write_file failed: {e:?}");
            }
        },
        // emit: emit hook.action event with merge/gate fields (T-6)
        |outcome: &WaveStateOutcome, story_id: &str| {
            if let WaveStateOutcome::Appended {
                wave,
                total,
                merged,
                gate_transitioned,
            } = outcome
            {
                if *gate_transitioned {
                    eprintln!(
                        "update-wave-state-on-merge: all stories in {} merged. \
                         gate_status → pending.\n  Run the wave integration gate \
                         before starting the next wave.",
                        wave
                    );
                }
                vsdd_hook_sdk::host::emit_event(
                    "hook.action",
                    &[
                        ("hook", "update-wave-state-on-merge"),
                        ("matcher", "SubagentStop"),
                        ("reason", "wave_merge_recorded"),
                        ("story_id", story_id),
                        ("wave", wave.as_str()),
                        ("total", &total.to_string()),
                        ("merged", &merged.to_string()),
                        ("gate_transitioned", &gate_transitioned.to_string()),
                    ],
                );
            }
        },
    )
}

#[cfg(not(feature = "standalone"))]
fn main() {
    vsdd_hook_sdk::__internal::run(on_hook);
}

// ---------------------------------------------------------------------------
// Standalone / test entry point (standalone feature = default for debug builds)
//
// Does NOT reference any vsdd host functions — no vsdd::* imports in binary.
// Uses WASI std::fs / std::env / std::io for file I/O so plain `wasmtime run`
// works without a dispatcher. Satisfies bats parity tests (AC-006).
// ---------------------------------------------------------------------------

/// Locate `wave-state.yaml` using WASI preopen directory enumeration.
///
/// wasmtime `--dir HOST_DIR` maps HOST_DIR at path `HOST_DIR` in the WASI namespace
/// (same absolute path). Relative paths like `wave-state.yaml` don't resolve via CWD=`/`
/// unless the preopened dir IS `/` (i.e., `--dir HOST_DIR::/`).
///
/// Strategy:
/// 1. Try `VSDD_WAVE_STATE_PATH` env var (only set when passed via `--env`).
/// 2. Enumerate WASI preopened directories (starting at fd=3) via raw `fd_prestat_get`
///    and `fd_prestat_dir_name` syscalls. For each preopened dir, construct the absolute
///    path `<preopen_dir>/wave-state.yaml` and check if it exists.
/// 3. Fall back to `.factory/wave-state.yaml` (production path for non-bats scenarios).
///
/// Returns the path string to use for read/write, or None if not found anywhere
/// (in which case process_wave_state will return NoOp gracefully).
#[cfg(feature = "standalone")]
fn resolve_wave_state_path() -> Option<String> {
    // Step 1: env var (only works if wasmtime passes --env VSDD_WAVE_STATE_PATH)
    if let Some(path) = std::env::var("VSDD_WAVE_STATE_PATH")
        .ok()
        .filter(|p| !p.is_empty())
    {
        return Some(path);
    }

    // Step 2: enumerate WASI preopened directories
    // On wasm32-wasip1, preopened dirs start at fd=3 (after stdin=0, stdout=1, stderr=2).
    // We use raw WASI syscalls via the `wasi_snapshot_preview1` import module.
    #[cfg(target_arch = "wasm32")]
    {
        // Raw WASI prestat structures (wasi_snapshot_preview1 ABI)
        #[repr(C)]
        struct WasiPrestatDir {
            tag: u8,
            _pad: [u8; 3],
            pr_name_len: u32,
        }

        // Import WASI fd_prestat_get and fd_prestat_dir_name
        // These are already available via the WASI runtime (WASI stdio is always used).
        #[link(wasm_import_module = "wasi_snapshot_preview1")]
        unsafe extern "C" {
            fn fd_prestat_get(fd: i32, prestat: *mut WasiPrestatDir) -> i32;
            fn fd_prestat_dir_name(fd: i32, path: *mut u8, path_len: u32) -> i32;
        }

        for fd in 3i32..32 {
            let mut prestat = WasiPrestatDir {
                tag: 0xff,
                _pad: [0; 3],
                pr_name_len: 0,
            };
            let rc = unsafe { fd_prestat_get(fd, &mut prestat) };
            if rc != 0 {
                // EBADF (rc=8) means no more preopened fds
                break;
            }
            if prestat.tag != 0 {
                // tag=0 means preopened directory; skip other types
                continue;
            }
            if prestat.pr_name_len == 0 {
                continue;
            }

            let mut name_buf = vec![0u8; prestat.pr_name_len as usize];
            let rc = unsafe { fd_prestat_dir_name(fd, name_buf.as_mut_ptr(), prestat.pr_name_len) };
            if rc != 0 {
                continue;
            }

            let dir = String::from_utf8_lossy(&name_buf).into_owned();
            let dir = dir.trim_end_matches('/');
            let candidate = format!("{dir}/wave-state.yaml");

            if std::path::Path::new(&candidate).exists() {
                return Some(candidate);
            }
        }
    }

    // Step 3: production fallback (works when CWD contains .factory/)
    if std::path::Path::new(".factory/wave-state.yaml").exists() {
        return Some(".factory/wave-state.yaml".to_string());
    }

    // Not found — process_wave_state will return NoOp gracefully
    None
}

/// Writes wave-state.yaml back using the resolved path.
/// For bats tests, the absolute path from `resolve_wave_state_path` is used.
#[cfg(feature = "standalone")]
fn write_wave_state(path: &str, yaml: String) {
    if let Err(e) = std::fs::write(path, yaml.as_bytes()) {
        eprintln!("update-wave-state-on-merge: write failed (standalone mode): {e}");
    }
}

#[cfg(feature = "standalone")]
fn main() {
    use std::io::Read;

    // Read stdin payload
    let mut buf = Vec::new();
    std::io::stdin().read_to_end(&mut buf).unwrap_or_default();

    let payload: HookPayload = match serde_json::from_slice(&buf) {
        Ok(p) => p,
        Err(_) => {
            // Malformed JSON — advisory hook, exit 0 silently
            std::process::exit(0);
        }
    };

    // Resolve the wave-state.yaml path via preopen enumeration.
    // If not found, read_yaml returns None → process_wave_state returns NoOp.
    let wave_state_path = resolve_wave_state_path();

    let result = wave_state_hook_logic(
        payload,
        // read_yaml: WASI std::fs via resolved absolute path
        || {
            wave_state_path
                .as_deref()
                .and_then(|p| std::fs::read_to_string(p).ok())
        },
        // write_yaml: WASI std::fs via resolved absolute path
        |yaml_str: String| {
            if let Some(path) = wave_state_path.as_deref() {
                write_wave_state(path, yaml_str);
            }
        },
        // emit: standalone mode — write gate-transition reminder to stderr
        |outcome: &WaveStateOutcome, _story_id: &str| {
            if let WaveStateOutcome::Appended {
                wave,
                gate_transitioned: true,
                ..
            } = outcome
            {
                eprintln!(
                    "update-wave-state-on-merge: all stories in {} merged. \
                     gate_status → pending.\n  Run the wave integration gate \
                     before starting the next wave.",
                    wave
                );
            }
        },
    );

    // Advisory hook — always exit 0 (BC-7.03.083 postcondition 2)
    let _ = result;
    std::process::exit(0);
}
