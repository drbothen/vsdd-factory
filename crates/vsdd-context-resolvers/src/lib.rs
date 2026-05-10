//! `vsdd-context-resolvers` — per-factory WASM context resolver crate.
//!
//! Provides the `WaveContextResolver`: a `#[resolver]`-annotated function that
//! reads `.factory/wave-state.yaml` and `.factory/STATE.md`, then injects
//! `wave_context` into `plugin_config` at dispatch time.
//!
//! # Architecture
//! - **Effectful entry point:** `resolve_impl` — calls `host::read_file` for both
//!   `.factory/wave-state.yaml` (wave data) and `.factory/STATE.md` (cycle ID),
//!   then delegates to the pure layer.
//! - **Pure computation layer:** `resolve_wave_context_pure` — takes pre-parsed
//!   `WaveState` and `cycle_id`; used by the VP-075 proptest (AC-008).
//!
//! # Key naming: `wave_context` (underscore)
//! The canonical key is `"wave_context"` (underscore), per BC-4.12.005 PC7,
//! ADR-018 §Context Key, and S-12.08 AC-001.
//!
//! # Panic-free guarantee
//! AC-010 / BC-4.12.004 INV1: No fallible unwrap or panic-on-error calls anywhere
//! in this crate's production source. All error paths return
//! `ResolverOutput { key: ..., value: None }`.
//!
//! # Factory-agnostic invariant
//! `factory-dispatcher` has ZERO compile-time dependency on this crate
//! (BC-1.13.001 INV1 / ADR-018). The WASM artifact is loaded dynamically.

pub mod wave_context;

pub use wave_context::{WaveEntry, WaveState};

/// Maximum bytes read from `.factory/wave-state.yaml` or `.factory/STATE.md`.
///
/// 1 MiB is generous for both files. The host `read_file` call is capped at this
/// budget to bound memory use inside the WASM linear memory.
const MAX_STATE_FILE_BYTES: u32 = 1024 * 1024; // 1 MiB

/// Read timeout in milliseconds for `host::read_file` calls.
///
/// 1 000 ms is the wall-clock budget per file read. If the host takes longer
/// it returns a timeout error, which is handled identically to `FileNotFound`
/// (resolver returns `value: None`).
const READ_TIMEOUT_MS: u32 = 1000;

// Dual import: `resolver_macro` is the TYPE path used by `#[resolver_macro]`
// attribute syntax; `resolver` is the MODULE path for ResolverInput/ResolverOutput
// types. The macro crate and the type module share the `vsdd_hook_sdk::resolver`
// namespace — the alias disambiguates the proc-macro invocation from the type import.
use vsdd_hook_sdk::resolver as resolver_macro;
use vsdd_hook_sdk::resolver::{ResolverInput, ResolverOutput};

/// Effectful entry point for the `wave_context` resolver.
///
/// Reads `.factory/wave-state.yaml` and `.factory/STATE.md` via `host::read_file`,
/// parses both into their respective types, then delegates to `resolve_wave_context_pure`.
///
/// Error policy (BC-4.12.004 PC3):
/// - Missing wave-state.yaml → `ResolverOutput { key: "wave_context", value: None }`
/// - Malformed YAML → same
/// - Missing STATE.md → resolver uses `None` cycle_id → pure fn returns `value: None`
/// - No active wave → same
/// Never traps. Never panics.
///
/// The `#[resolver_macro]` attribute generates the WASM `resolve()` export
/// (gated to `#[cfg(target_arch = "wasm32")]`) and a `fn main() {}` no-op
/// (BC-4.12.002 PC5).
///
/// Per EC-002: this resolver returns `wave_context` regardless of `event_type`.
/// `input.event_type` is intentionally not consulted.
#[resolver_macro]
pub fn resolve_impl(input: ResolverInput) -> ResolverOutput {
    // Per EC-002: resolver returns wave_context regardless of event_type.
    // input.event_type is intentionally not consulted.
    let _ = &input.event_type;

    // AC-007: use top-level `input.project_dir` (NOT `plugin_config`) per spec.
    let wave_state_path = format!("{}/.factory/wave-state.yaml", input.project_dir);
    let state_md_path = format!("{}/.factory/STATE.md", input.project_dir);

    // Read wave-state.yaml. Missing/unreadable → value: None (AC-002, AC-005).
    // Log unexpected read errors at warn level to aid operator diagnosis.
    // HostError::Other(-N) codes include file-not-found; those are expected and
    // produce no log output. CapabilityDenied / Timeout get a warn-level log.
    let wave_bytes = match vsdd_hook_sdk::host::read_file(
        &wave_state_path,
        MAX_STATE_FILE_BYTES,
        READ_TIMEOUT_MS,
    ) {
        Ok(b) => b,
        Err(e) => {
            match &e {
                vsdd_hook_sdk::host::HostError::Other(_) => {
                    // Likely file-not-found — expected on fresh projects.
                    // No log output for this case.
                }
                _ => {
                    vsdd_hook_sdk::host::log_warn(&format!(
                        "vsdd-context-resolvers: read_file {} failed: {:?}",
                        wave_state_path, e
                    ));
                }
            }
            return ResolverOutput {
                key: "wave_context".to_string(),
                value: None,
            };
        }
    };

    let wave_yaml = match std::str::from_utf8(&wave_bytes) {
        Ok(s) => s,
        Err(_) => {
            return ResolverOutput {
                key: "wave_context".to_string(),
                value: None,
            };
        }
    };

    // Malformed YAML → WaveState::default() (empty waves) → pure fn returns value: None.
    // Symmetric with read_file error observability above — log at warn level so operators
    // can diagnose malformed wave-state.yaml without having to reproduce the failure.
    let wave_state = match wave_context::parse_wave_state(wave_yaml) {
        Ok(ws) => ws,
        Err(e) => {
            vsdd_hook_sdk::host::log_warn(&format!(
                "vsdd-context-resolvers: parse_wave_state failed: {} — falling back to empty waves",
                e
            ));
            wave_context::WaveState::default()
        }
    };

    // Read STATE.md for cycle_id. Missing/unreadable → None cycle_id.
    // If cycle_id is None the pure fn returns value: None (required field for output).
    let cycle_id: Option<String> =
        vsdd_hook_sdk::host::read_file(&state_md_path, MAX_STATE_FILE_BYTES, READ_TIMEOUT_MS)
            .ok()
            .and_then(|bytes| String::from_utf8(bytes).ok())
            .and_then(|text| wave_context::parse_cycle_id_from_state_md(&text));

    resolve_wave_context_pure(&input, &wave_state, cycle_id.as_deref())
}

/// Pure computation layer — deterministic given the same inputs (VP-075).
///
/// Does NOT call `host::read_file` or perform any I/O. Takes a pre-parsed
/// `WaveState` (canonical schema with `waves: Vec<WaveEntry>`) and an optional
/// `cycle_id` extracted from STATE.md frontmatter.
///
/// Returns `ResolverOutput { key: "wave_context", value: Some({...}) }` when:
/// - An active wave exists (last wave with `gate_status` not in TERMINAL_STATES
///   per BC-8.14.009: "passed" | "deferred" | "failed" | "completed")
/// - That wave has at least one story in its `stories` list
/// - `cycle_id` is `Some`
///
/// Returns `ResolverOutput { key: "wave_context", value: None }` in all other
/// cases (AC-002b, AC-003, AC-004).
///
/// Per VP-075: this function is deterministic — same `(input, wave_state,
/// cycle_id)` triple always produces identical output. No I/O, no randomness,
/// no side effects.
/// Per BC-4.12.004 INV1: no fallible unwrap or panic-on-error calls.
/// Per EC-002: `input.event_type` is intentionally not consulted.
pub fn resolve_wave_context_pure(
    _input: &ResolverInput,
    wave_state: &WaveState,
    cycle_id: Option<&str>,
) -> ResolverOutput {
    // AC-002b / EC-002: cycle_id required; if absent return None.
    let cycle = match cycle_id {
        Some(c) if !c.is_empty() => c,
        _ => {
            return ResolverOutput {
                key: "wave_context".to_string(),
                value: None,
            };
        }
    };

    // Find active wave: last entry with gate_status not in TERMINAL_STATES
    // (per BC-8.14.009: "passed" | "deferred" | "failed" | "completed").
    let active = match wave_context::find_active_wave(wave_state) {
        Some(w) => w,
        None => {
            return ResolverOutput {
                key: "wave_context".to_string(),
                value: None,
            };
        }
    };

    // AC-003: empty stories list → value: None.
    if active.stories.is_empty() {
        return ResolverOutput {
            key: "wave_context".to_string(),
            value: None,
        };
    }

    ResolverOutput {
        key: "wave_context".to_string(),
        value: Some(serde_json::json!({
            "cycle_id": cycle,
            "wave_id": active.wave,
            "stories": active.stories,
        })),
    }
}
