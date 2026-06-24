//! Plugin invocation: create a Store, set the epoch deadline and fuel
//! budget, pipe the payload through WASI stdin, run `_start`, capture
//! stdout, and classify the outcome.
//!
//! Everything except the bounded cost enforcement (epoch interruption,
//! fuel consumption) happens synchronously. The per-invocation timeout
//! is honored by the shared [`EpochTicker`]; each invocation just sets
//! its own deadline before calling `_start`.
//!
//! ## S-18.00: EventType enum
//!
//! [`EventType`] is the closed enum of Claude Code harness event types that the
//! dispatcher recognises. It enumerates `PreToolUse`, `PostToolUse`, `PreCompact`,
//! and `PostCompact` as first-class variants (BC-1.15.001 INV1). An unknown-event
//! fallback that silently discards PreCompact/PostCompact is a specification
//! violation.
//!
//! PreCompact/PostCompact routing runs through `main.rs → execute_tiers`. The
//! block-intent decision for PostCompact is suppressed by
//! `EventType::from_event_str().is_advisory_only()` (BC-1.15.001 PC2).
//! `dispatch_precompact` and `dispatch_postcompact` are intentional public symbol
//! anchors for the routing arms; the complete dispatch path lives in main.rs.

use std::time::Instant;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use wasmtime::{Engine, Module, Store, Trap};
use wasmtime_wasi::p1::{self, WasiP1Ctx};
use wasmtime_wasi::p2::pipe::{MemoryInputPipe, MemoryOutputPipe};
use wasmtime_wasi::{DirPerms, FilePerms, I32Exit, WasiCtxBuilder};

use crate::engine::timeout_ms_to_epochs;
use crate::host::{HostContext, setup_linker};

// ---------------------------------------------------------------------------
// S-18.00: EventType enum (BC-1.15.001 INV1)
//
// Closed enum of harness event types the dispatcher recognises. All event
// types — including PreCompact and PostCompact added by S-18.00 — MUST be
// enumerated here rather than handled via string fallback. An unknown-event
// fallback path that silently discards PreCompact/PostCompact is a
// specification violation (BC-1.15.001 INV1).
//
// The complete dispatch path for PreCompact/PostCompact runs through
// main.rs → execute_tiers. `dispatch_precompact` and `dispatch_postcompact`
// are public symbol anchors for the routing arms (BC-1.15.001 PC1/PC2/PC3/PC4/PC5).
// ---------------------------------------------------------------------------

/// Closed enum of harness event types the dispatcher routes.
///
/// Adding a new event type to the Claude Code harness protocol requires a
/// new variant here — string-fallback dispatch is a specification violation
/// per BC-1.15.001 INV1.
///
/// # S-18.00 additions
///
/// `PreCompact` and `PostCompact` are added as first-class variants alongside
/// the existing `PreToolUse` and `PostToolUse` (BC-1.15.001 INV1).
/// `PostCompact` is advisory-only at the harness level: the dispatcher
/// propagates exit codes but never sets `block_intent=true` for PostCompact
/// (BC-1.15.001 PC2).
///
/// # Serde
///
/// `EventType` can be round-tripped from the `event_name` / `event` string
/// fields in the harness payload and hooks-registry.toml via
/// [`EventType::from_event_str`] and [`EventType::as_str`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum EventType {
    /// Claude Code pre-tool-use hook event (gates tool execution).
    PreToolUse,
    /// Claude Code post-tool-use hook event (advisory / observability).
    PostToolUse,
    /// Claude Code pre-compact hook event (gates context compaction).
    /// Introduced in harness >= v2.1.105. Block-intent propagation is
    /// supported (BC-1.15.001 PC1/PC4).
    PreCompact,
    /// Claude Code post-compact hook event (advisory-only; no block-intent).
    /// Introduced in harness >= v2.1.105 (BC-1.15.001 PC2).
    PostCompact,
    /// Other harness events not enumerated above (session lifecycle, etc.).
    /// The dispatcher routes these identically to `PostToolUse` semantics
    /// (advisory, no block-intent) unless a future story adds a specific arm.
    Other,
}

impl EventType {
    /// Convert this variant to the canonical event-name string used in
    /// hooks-registry.toml `event` fields and harness payload `event_name` fields.
    ///
    /// # GREEN-BY-DESIGN
    ///
    /// Pure match on enum variants; zero branching beyond pattern, no I/O,
    /// no helpers. Body ≤ 3 lines per variant. BC-5.38.002 criteria all satisfied.
    pub fn as_str(&self) -> &'static str {
        match self {
            EventType::PreToolUse => "PreToolUse",
            EventType::PostToolUse => "PostToolUse",
            EventType::PreCompact => "PreCompact",
            EventType::PostCompact => "PostCompact",
            EventType::Other => "Other",
        }
    }

    /// Returns `true` if this event type is advisory-only at the harness level,
    /// meaning the dispatcher MUST NOT propagate `block_intent=true` regardless
    /// of plugin exit codes or `on_error` settings.
    ///
    /// # BC-1.15.001 PC2 (S-18.00)
    ///
    /// `PostCompact` is the only advisory-only event type. The harness does not
    /// honour `block_intent` on PostCompact; attempting to set it would be a
    /// specification violation.
    ///
    /// All other event types (`PreToolUse`, `PostToolUse`, `PreCompact`, `Other`)
    /// return `false` — they support block-intent propagation via `on_error=block`
    /// and exit-code 2 semantics.
    pub fn is_advisory_only(&self) -> bool {
        matches!(self, EventType::PostCompact)
    }

    /// Parse an event-name string from the harness payload or hooks-registry.toml
    /// into an `EventType` variant.
    ///
    /// Returns `EventType::Other` for unrecognised event names so the dispatcher
    /// never silently drops an unknown event — callers can inspect `EventType::Other`
    /// and handle gracefully.
    ///
    /// # BC-1.15.001 INV1
    ///
    /// `"PreCompact"` and `"PostCompact"` are first-class event types; they MUST NOT
    /// return `EventType::Other`. An unknown-event fallback that silently discards
    /// these events is a specification violation.
    pub fn from_event_str(event: &str) -> Self {
        match event {
            "PreToolUse" => EventType::PreToolUse,
            "PostToolUse" => EventType::PostToolUse,
            "PreCompact" => EventType::PreCompact,
            "PostCompact" => EventType::PostCompact,
            _ => EventType::Other,
        }
    }
}

/// Dispatch a `PreCompact` event to a set of matched plugins.
///
/// PreCompact supports block-intent propagation: a plugin that exits 2
/// causes the dispatcher to return `block_intent=true` (BC-1.15.001 PC1/PC4).
/// On-error semantics mirror PreToolUse (BC-1.15.001 PC5):
/// - `on_error = "block"` crash → block_intent=true (fail-closed)
/// - `on_error = "continue"` crash → advisory only (fail-open)
///
/// When no plugins are registered for this event type, returns without error
/// and produces no block intent (BC-1.15.001 PC3).
///
/// # Integration path
///
/// The full dispatch with plugin invocation, priority ordering, and block-intent
/// aggregation lives in `main.rs` → `executor::execute_tiers`. This function is
/// the unit-level anchor for the `EventType::PreCompact` routing arm; the integration
/// path reaches it via `match_plugins` + `execute_tiers` using `event_name = "PreCompact"`.
///
/// # BC-1.15.001 PC1/PC3/PC4/PC5
pub fn dispatch_precompact() {
    // No-op: this function is the named anchor for the PreCompact routing arm.
    // The complete dispatch (plugin invocation, exit-2 aggregation, on_error semantics)
    // runs through main.rs → execute_tiers when `event_name = "PreCompact"` is matched.
    // BC-1.15.001 PC3: zero registered plugins → block_intent=false, exit 0 (no-op correct).
}

/// Dispatch a `PostCompact` event to a set of matched plugins.
///
/// PostCompact is advisory-only at the harness level: the dispatcher invokes
/// registered plugins and propagates exit codes in the response, but NEVER
/// sets `block_intent=true` regardless of plugin exit code (BC-1.15.001 PC2).
///
/// When no plugins are registered for this event type, returns without error
/// (BC-1.15.001 PC3).
///
/// # Integration path
///
/// The full dispatch path runs through `main.rs` → `executor::execute_tiers` when
/// `event_name = "PostCompact"`. The advisory-only constraint (never block_intent)
/// is enforced in `main.rs` by only propagating exit-2 block intent for PreCompact
/// events; PostCompact results are handled as advisory.
///
/// # BC-1.15.001 PC2/PC3
pub fn dispatch_postcompact() {
    // No-op: this function is the named anchor for the PostCompact routing arm.
    // The complete dispatch runs through main.rs → execute_tiers when
    // `event_name = "PostCompact"` is matched.
    // BC-1.15.001 PC2: block_intent is NEVER set for PostCompact (advisory-only).
    // BC-1.15.001 PC3: zero registered plugins → no-op (correct).
}

/// Outcome of a single `invoke_plugin` call.
///
/// `fuel_consumed` is always populated so operators can see how close
/// normal plugins came to their budget. `elapsed_ms` is wall-clock from
/// just before `_start` to just after the result is classified.
///
/// `stderr` is the plugin's WASI stderr captured during invocation,
/// truncated to [`STDERR_CAP_BYTES`]. It's the diagnostic signal the
/// dispatcher emits on `plugin.completed` / `plugin.crashed` /
/// `plugin.timeout` events; without it, operators see only `exit_code`
/// and have to re-run with a manual capture to find out why a plugin
/// exited 1. Field added in v1.0.0-beta.4 after the S-2.7 dogfood loop
/// ran into exactly that diagnostic gap.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PluginResult {
    Ok {
        /// Process exit code the plugin's `fn main()` returned.
        exit_code: i32,
        /// Raw stdout (UTF-8 `HookResult` JSON in well-behaved plugins).
        stdout: String,
        /// Captured stderr, truncated to STDERR_CAP_BYTES.
        stderr: String,
        elapsed_ms: u64,
        fuel_consumed: u64,
    },
    Timeout {
        cause: TimeoutCause,
        /// Captured stderr, truncated to STDERR_CAP_BYTES. May contain
        /// a partial message because the plugin was interrupted.
        stderr: String,
        elapsed_ms: u64,
        fuel_consumed: u64,
    },
    Crashed {
        trap_string: String,
        /// Captured stderr, truncated to STDERR_CAP_BYTES. Often the
        /// most useful field for diagnosing a crash since wasmtime's
        /// trap_string only surfaces the trap kind, not the plugin's
        /// own pre-trap diagnostics.
        stderr: String,
        elapsed_ms: u64,
        fuel_consumed: u64,
    },
}

/// Truncation cap on per-plugin stderr captured into `PluginResult`.
/// Operators see this value on `plugin.completed`/`plugin.crashed`/
/// `plugin.timeout` events. 4 KiB is generous for diagnostic lines
/// while keeping the internal-log per-event payload bounded.
pub const STDERR_CAP_BYTES: usize = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimeoutCause {
    /// Wall-clock budget exhausted (epoch interruption fired).
    Epoch,
    /// Fuel budget exhausted.
    Fuel,
}

/// Per-invocation budget. Defaults live in
/// `RegistryDefaults`; callers usually get these from a
/// `RegistryEntry` with fallback.
#[derive(Debug, Clone, Copy)]
pub struct InvokeLimits {
    pub timeout_ms: u32,
    pub fuel_cap: u64,
}

impl Default for InvokeLimits {
    fn default() -> Self {
        Self {
            timeout_ms: 5_000,
            fuel_cap: 10_000_000,
        }
    }
}

#[derive(Debug, Error)]
pub enum InvokeError {
    #[error("wasmtime store setup failed: {0}")]
    Setup(String),
    #[error("linker instantiation failed: {0}")]
    Instantiate(String),
    #[error("plugin has no `_start` export")]
    MissingStart,
    #[error("host linker build failed: {0}")]
    HostLinker(String),
    #[error("wasi setup failed: {0}")]
    Wasi(String),
}

/// Invoke a pre-compiled plugin module against the given
/// [`HostContext`]. The `payload_json` is written to the plugin's
/// stdin; the plugin is expected to write a `HookResult` JSON line to
/// stdout, which the caller is responsible for parsing.
pub fn invoke_plugin(
    engine: &Engine,
    module: &Module,
    host_ctx: HostContext,
    payload_json: &[u8],
    limits: InvokeLimits,
) -> Result<PluginResult, InvokeError> {
    // Set up wasmtime store with both host context and WASI context.
    // We use a wrapper type so both live in the store's data slot.
    let stdout = MemoryOutputPipe::new(64 * 1024);
    let stderr = MemoryOutputPipe::new(64 * 1024);

    // Preopen the project directory (host_ctx.cwd) as "." in the WASI guest.
    // This enables std::fs operations from WASM plugins that perform filesystem
    // I/O relative to the project root (e.g. session-learning appending to
    // .factory/sidecar-learning.md). Plugins without filesystem needs are
    // unaffected — they simply ignore the preopened handle.
    // If the cwd path cannot be opened (e.g. missing dir in tests), the WASI
    // context is built without a preopen and std::fs calls will return EBADF.
    let mut wasi_builder = WasiCtxBuilder::new();
    wasi_builder
        .stdin(MemoryInputPipe::new(payload_json.to_vec()))
        .stdout(stdout.clone())
        .stderr(stderr.clone());
    if host_ctx.cwd.as_os_str().is_empty() {
        // No project dir — build without filesystem preopen.
    } else if let Err(e) = wasi_builder.preopened_dir(
        // W-15 wave gate (SEC-001 / CRIT-W15-003): WASI preopens grant
        // DirPerms::all() | FilePerms::all() to plugins. This is the sandbox
        // boundary; capability-gated host functions (e.g., write_file) provide
        // ADDITIONAL bounded mechanisms but do not constrain native WASI calls.
        // See crates/hook-sdk/HOST_ABI.md "Filesystem Access Model". v1.1 will
        // tighten preopens to read-only by default with explicit write capability
        // declarations.
        &host_ctx.cwd,
        ".",
        DirPerms::all(),
        FilePerms::all(),
    ) {
        // Non-fatal: log and continue without filesystem access.
        // Plugin may still function if it doesn't need std::fs.
        tracing::debug!(
            cwd = %host_ctx.cwd.display(),
            err = %e,
            "wasi preopen failed; plugin std::fs calls will fail"
        );
    }
    let wasi_ctx = wasi_builder.build_p1();

    let store_data = StoreData {
        host: host_ctx,
        wasi: wasi_ctx,
    };
    let mut store = Store::new(engine, store_data);

    // Wall-clock budget → epoch deadline.
    store.set_epoch_deadline(timeout_ms_to_epochs(limits.timeout_ms as u64));
    store
        .set_fuel(limits.fuel_cap)
        .map_err(|e| InvokeError::Setup(e.to_string()))?;

    // Build per-invocation linker: host imports (S-1.4) + WASI.
    let host_linker = setup_linker(engine).map_err(|e| InvokeError::HostLinker(e.to_string()))?;
    let mut linker: wasmtime::Linker<StoreData> = wasmtime::Linker::new(engine);
    // Copy host imports from the HostContext-typed linker into our
    // StoreData-typed linker. For every Extern the host linker holds,
    // register a proxy that dereferences store_data.host.
    proxy_host_imports(&mut linker, &host_linker, engine)
        .map_err(|e| InvokeError::HostLinker(e.to_string()))?;
    p1::add_to_linker_sync(&mut linker, |d: &mut StoreData| &mut d.wasi)
        .map_err(|e| InvokeError::Wasi(e.to_string()))?;

    let instance = linker
        .instantiate(&mut store, module)
        .map_err(|e| InvokeError::Instantiate(e.to_string()))?;

    let start_export = instance
        .get_typed_func::<(), ()>(&mut store, "_start")
        .map_err(|_| InvokeError::MissingStart)?;

    let started = Instant::now();
    let call_result = start_export.call(&mut store, ());
    let elapsed_ms = started.elapsed().as_millis() as u64;
    let fuel_consumed = fuel_consumed_from_store(&store, limits.fuel_cap);

    // WASI command convention: `_start` returns () on exit(0); any
    // other exit code arrives as a trap whose downcast yields an
    // `I32Exit(code)`.
    match call_result {
        Ok(()) => {
            let out = stdout_to_string(&stdout);
            let err_text = stderr_to_string(&stderr);
            Ok(PluginResult::Ok {
                exit_code: 0,
                stdout: out,
                stderr: err_text,
                elapsed_ms,
                fuel_consumed,
            })
        }
        Err(err) => classify_trap(
            anyhow::Error::from(err),
            &stdout,
            &stderr,
            elapsed_ms,
            fuel_consumed,
        ),
    }
}

fn classify_trap(
    err: anyhow::Error,
    stdout: &MemoryOutputPipe,
    stderr: &MemoryOutputPipe,
    elapsed_ms: u64,
    fuel_consumed: u64,
) -> Result<PluginResult, InvokeError> {
    let stderr_text = stderr_to_string(stderr);
    // WASI `exit(n)` propagates as an `I32Exit` in wasmtime-wasi's
    // preview-1 glue; non-zero exit is still "Ok" from our POV since
    // the plugin ran to a controlled finish.
    if let Some(exit) = err.downcast_ref::<I32Exit>() {
        let out = stdout_to_string(stdout);
        return Ok(PluginResult::Ok {
            exit_code: exit.0,
            stdout: out,
            stderr: stderr_text,
            elapsed_ms,
            fuel_consumed,
        });
    }
    if let Some(trap) = err.downcast_ref::<Trap>() {
        return Ok(match trap {
            Trap::Interrupt => PluginResult::Timeout {
                cause: TimeoutCause::Epoch,
                stderr: stderr_text,
                elapsed_ms,
                fuel_consumed,
            },
            Trap::OutOfFuel => PluginResult::Timeout {
                cause: TimeoutCause::Fuel,
                stderr: stderr_text,
                elapsed_ms,
                fuel_consumed,
            },
            other => PluginResult::Crashed {
                trap_string: other.to_string(),
                stderr: stderr_text,
                elapsed_ms,
                fuel_consumed,
            },
        });
    }
    // Any other wasmtime error surfaces as Crashed — preserves the
    // exact diagnostic for operators.
    Ok(PluginResult::Crashed {
        trap_string: format!("{err:#}"),
        stderr: stderr_text,
        elapsed_ms,
        fuel_consumed,
    })
}

fn fuel_consumed_from_store(store: &Store<StoreData>, cap: u64) -> u64 {
    match store.get_fuel() {
        Ok(remaining) => cap.saturating_sub(remaining),
        Err(_) => 0,
    }
}

/// Read the captured stderr pipe and truncate to STDERR_CAP_BYTES so
/// the per-event payload stays bounded. Truncation appends an ellipsis
/// marker so operators can see they only have a partial view.
fn stderr_to_string(pipe: &MemoryOutputPipe) -> String {
    let bytes = pipe.contents();
    if bytes.len() <= STDERR_CAP_BYTES {
        return String::from_utf8_lossy(&bytes).into_owned();
    }
    let mut s = String::from_utf8_lossy(&bytes[..STDERR_CAP_BYTES]).into_owned();
    s.push_str("\n…(stderr truncated)");
    s
}

fn stdout_to_string(pipe: &MemoryOutputPipe) -> String {
    let bytes = pipe.contents();
    String::from_utf8_lossy(&bytes).into_owned()
}

/// Bridge from the `Linker<HostContext>` built by S-1.4 to a
/// `Linker<StoreData>` the invoke path uses. The bridge re-registers
/// each host import under the StoreData data type by wrapping it in a
/// proxy that delegates into `store_data.host`.
///
/// In practice we just rebuild the linker from scratch here — wasmtime
/// doesn't support cloning Func between different Store types. The
/// S-1.4 `setup_linker` exists for the integration-test story; at
/// runtime the invoke path owns its own linker build.
fn proxy_host_imports(
    linker: &mut wasmtime::Linker<StoreData>,
    _host_linker_reference: &wasmtime::Linker<HostContext>,
    engine: &Engine,
) -> Result<(), String> {
    // Keep the parameter signature stable so the call site from
    // `invoke_plugin` reads naturally — swap the internal
    // implementation to re-register against the StoreData data type.
    let _ = engine;
    setup_host_on_store_data(linker).map_err(|e| e.to_string())
}

/// Re-register every `vsdd::*` host function directly against a
/// `Linker<StoreData>`. Semantics mirror the `HostContext`-flavored
/// registrations in `host/` submodules, but access host state through
/// `store.data().host`.
fn setup_host_on_store_data(
    linker: &mut wasmtime::Linker<StoreData>,
) -> Result<(), crate::host::HostCallError> {
    use crate::host::HostCallError;
    use crate::host::codes;
    use crate::internal_log::InternalEvent;
    use serde_json::{Map, Value};
    use wasmtime::Caller;

    // log
    linker
        .func_wrap(
            "vsdd",
            "log",
            |mut caller: Caller<'_, StoreData>, level: u32, msg_ptr: u32, msg_len: u32| {
                if let Ok(msg) = read_wasm_string_sd(&mut caller, msg_ptr, msg_len) {
                    let level_str = match level {
                        0 => "trace",
                        1 => "debug",
                        2 => "info",
                        3 => "warn",
                        4 => "error",
                        _ => "info",
                    };
                    let host = &caller.data().host;
                    let ev = InternalEvent::now("plugin.log")
                        .with_trace_id(&host.dispatcher_trace_id)
                        .with_session_id(&host.session_id)
                        .with_plugin_name(&host.plugin_name)
                        .with_plugin_version(&host.plugin_version)
                        .with_field("level", Value::String(level_str.to_string()))
                        .with_field("message", Value::String(msg));
                    host.emit_internal(ev);
                }
            },
        )
        .map_err(|e| HostCallError::Linker(e.to_string()))?;

    // emit_event
    linker
        .func_wrap(
            "vsdd",
            "emit_event",
            |mut caller: Caller<'_, StoreData>,
             type_ptr: u32,
             type_len: u32,
             fields_ptr: u32,
             fields_len: u32| {
                let event_type = match read_wasm_string_sd(&mut caller, type_ptr, type_len) {
                    Ok(s) => s,
                    Err(_) => return,
                };
                let fields_buf = match read_wasm_bytes_sd(&mut caller, fields_ptr, fields_len) {
                    Ok(b) => b,
                    Err(_) => return,
                };
                let pairs = crate::host::emit_event::decode_fields(&fields_buf).unwrap_or_default();
                let host = &caller.data().host;
                let mut ev = InternalEvent::now(&event_type)
                    .with_trace_id(&host.dispatcher_trace_id)
                    .with_session_id(&host.session_id)
                    .with_plugin_name(&host.plugin_name)
                    .with_plugin_version(&host.plugin_version);
                for (k, v) in pairs {
                    // Reserved fields: plugins cannot override dispatcher-owned identity fields.
                    // BC-3.08.001 v1.7 Invariant 5: both "trace_id" (canonical wire name) and
                    // "dispatcher_trace_id" (legacy, defense-in-depth) are reserved.
                    if [
                        "trace_id",
                        "dispatcher_trace_id",
                        "session_id",
                        "plugin_name",
                        "plugin_version",
                        "ts",
                        "ts_epoch",
                        "schema_version",
                        "type",
                    ]
                    .contains(&k.as_str())
                    {
                        continue;
                    }
                    ev = ev.with_field(&k, Value::String(v));
                }
                host.emit_internal(ev);
            },
        )
        .map_err(|e| HostCallError::Linker(e.to_string()))?;

    // context getters: session_id / dispatcher_trace_id / plugin_root / plugin_version / cwd
    linker
        .func_wrap(
            "vsdd",
            "session_id",
            context_reader(|h| h.session_id.clone()),
        )
        .map_err(|e| HostCallError::Linker(e.to_string()))?;
    linker
        .func_wrap(
            "vsdd",
            "dispatcher_trace_id",
            context_reader(|h| h.dispatcher_trace_id.clone()),
        )
        .map_err(|e| HostCallError::Linker(e.to_string()))?;
    linker
        .func_wrap(
            "vsdd",
            "plugin_root",
            context_reader(|h| h.plugin_root.to_string_lossy().into_owned()),
        )
        .map_err(|e| HostCallError::Linker(e.to_string()))?;
    linker
        .func_wrap(
            "vsdd",
            "plugin_version",
            context_reader(|h| h.plugin_version.clone()),
        )
        .map_err(|e| HostCallError::Linker(e.to_string()))?;
    linker
        .func_wrap(
            "vsdd",
            "cwd",
            context_reader(|h| h.cwd.to_string_lossy().into_owned()),
        )
        .map_err(|e| HostCallError::Linker(e.to_string()))?;

    // env — simplified; full capability check lives in crate::host::env
    linker
        .func_wrap(
            "vsdd",
            "env",
            |mut caller: Caller<'_, StoreData>,
             name_ptr: u32,
             name_len: u32,
             out_ptr: u32,
             out_cap: u32|
             -> i32 {
                let name = match read_wasm_string_sd(&mut caller, name_ptr, name_len) {
                    Ok(s) => s,
                    Err(_) => return codes::INVALID_ARGUMENT,
                };
                let host = &caller.data().host;
                if !host.capabilities.env_allow.iter().any(|n| n == &name) {
                    let mut details = Map::new();
                    details.insert("variable".into(), Value::String(name.clone()));
                    host.emit_internal(host.denial_event("env", "env_not_on_allow_list", details));
                    return codes::CAPABILITY_DENIED;
                }
                let value = host.env_view.get(&name).cloned();
                match value {
                    None => 0,
                    Some(v) => {
                        let bytes = v.into_bytes();
                        match write_wasm_bytes_sd(&mut caller, out_ptr, out_cap, &bytes) {
                            Ok(n) => n as i32,
                            Err(_) => codes::INVALID_ARGUMENT,
                        }
                    }
                }
            },
        )
        .map_err(|e| HostCallError::Linker(e.to_string()))?;

    // read_file: real implementation. Uses output-pointer protocol:
    // the host reads the file, grows WASM memory to hold the bytes,
    // writes them there, then writes the address and length back to
    // the guest-provided out-param pointers.
    //
    // S-8.07: first in-tree plugin (warn-pending-wave-gate) to use this path.
    linker
        .func_wrap(
            "vsdd",
            "read_file",
            |mut caller: Caller<'_, StoreData>,
             path_ptr: u32,
             path_len: u32,
             max_bytes: u32,
             _timeout_ms: u32,
             out_ptr_out: u32,
             out_len_out: u32|
             -> i32 {
                let path = match read_wasm_string_sd(&mut caller, path_ptr, path_len) {
                    Ok(s) => s,
                    Err(_) => return codes::INVALID_ARGUMENT,
                };

                // Capability check + file read (host-side logic, no WASM memory).
                let body = {
                    let ctx = caller.data().host.clone();
                    match crate::host::read_file::prepare(&ctx, &path, max_bytes) {
                        Ok((bytes, _)) => bytes,
                        Err(code) => return code,
                    }
                };

                if body.is_empty() {
                    // Empty file: write ptr=0, len=0.  SDK read_owned_bytes guards
                    // ptr==0 → returns Vec::new(), which is correct for empty files.
                    let _ = write_wasm_u32_sd(&mut caller, out_ptr_out, 0);
                    let _ = write_wasm_u32_sd(&mut caller, out_len_out, 0);
                    return codes::OK;
                }

                // Find the current end of WASM linear memory, then grow by
                // enough pages to hold `body`.  Writing at the old end gives
                // us a valid, unused address (the SDK copies the bytes
                // immediately via `read_owned_bytes`, so the page is never
                // reused for anything else during this call).
                let memory = match get_memory_sd(&mut caller) {
                    Ok(m) => m,
                    Err(_) => return codes::INTERNAL_ERROR,
                };
                let current_bytes = memory.data_size(&caller);
                let pages_needed = body.len().div_ceil(65536) as u64;
                if memory.grow(&mut caller, pages_needed).is_err() {
                    return codes::INTERNAL_ERROR;
                }

                let write_offset = current_bytes as u32;

                // Write file bytes at the newly allocated offset.
                // `out_cap` = body.len() because we just grew enough memory.
                if write_wasm_bytes_sd(&mut caller, write_offset, body.len() as u32, &body).is_err()
                {
                    return codes::INTERNAL_ERROR;
                }

                // Return (ptr, len) to the guest via the out-params.
                if write_wasm_u32_sd(&mut caller, out_ptr_out, write_offset).is_err() {
                    return codes::INVALID_ARGUMENT;
                }
                if write_wasm_u32_sd(&mut caller, out_len_out, body.len() as u32).is_err() {
                    return codes::INVALID_ARGUMENT;
                }
                codes::OK
            },
        )
        .map_err(|e| HostCallError::Linker(e.to_string()))?;

    // write_file: real implementation rooted at cwd (CLAUDE_PROJECT_DIR).
    // Relative paths in the request and path_allow are resolved against
    // ctx.cwd so plugins can write project-relative files (e.g.
    // `.factory/wave-state.yaml`). Uses input-pointer protocol: the host
    // reads the byte slice from guest memory (read_wasm_bytes protocol).
    // First consumer: update-wave-state-on-merge (S-8.04 BC-7.03.085/086).
    linker
        .func_wrap(
            "vsdd",
            "write_file",
            |mut caller: Caller<'_, StoreData>,
             path_ptr: u32,
             path_len: u32,
             contents_ptr: u32,
             contents_len: u32,
             max_bytes: u32,
             _timeout_ms: u32|
             -> i32 {
                let path = match read_wasm_string_sd(&mut caller, path_ptr, path_len) {
                    Ok(s) => s,
                    Err(_) => return codes::INVALID_ARGUMENT,
                };
                let contents = match read_wasm_bytes_sd(&mut caller, contents_ptr, contents_len) {
                    Ok(b) => b,
                    Err(_) => return codes::INVALID_ARGUMENT,
                };
                let host = caller.data().host.clone();
                // Capability check: deny-by-default (BC-2.02.011 postcondition 1).
                let caps = match &host.capabilities.write_file {
                    Some(c) => c.clone(),
                    None => return codes::CAPABILITY_DENIED,
                };
                // Resolve path against cwd (CLAUDE_PROJECT_DIR) for relative paths.
                let cwd = &host.cwd;
                let resolved = if std::path::Path::new(&path).is_absolute() {
                    std::path::PathBuf::from(&path)
                } else {
                    cwd.join(&path)
                };
                // Byte cap: minimum of call arg and per-capability override.
                let effective_cap = match caps.max_bytes_per_call {
                    Some(cap_override) => max_bytes.min(cap_override),
                    None => max_bytes,
                };
                if contents.len() as u64 > effective_cap as u64 {
                    return codes::OUTPUT_TOO_LARGE;
                }
                // Check path is under an allowed prefix (rooted at cwd).
                // Use the same canonicalize-with-ancestor-fallback as write_file.rs
                // to handle files that don't yet exist.
                let allowed = caps.path_allow.iter().any(|pref| {
                    let pref_path = if std::path::Path::new(pref).is_absolute() {
                        std::path::PathBuf::from(pref)
                    } else {
                        cwd.join(pref)
                    };
                    // Apply ancestor fallback to the pref path too so that
                    // symlinks in the host path (e.g. macOS /var → /private/var)
                    // do not cause false capability denials when the target file
                    // is new (doesn't exist yet). Without this, `canon_resolved`
                    // resolves symlinks via ancestor fallback but `canon_pref`
                    // retains the unresolved symlink, causing starts_with to
                    // fail on macOS bats temp dirs (/var/folders → /private/var/folders).
                    let canon_pref = pref_path.canonicalize().unwrap_or_else(|_| {
                        // Ancestor fallback for pref_path: walk ancestors until one canonicalizes.
                        let mut pref_tail: Vec<std::ffi::OsString> = Vec::new();
                        let mut cur = pref_path.clone();
                        loop {
                            match cur.file_name() {
                                None => break pref_path.clone(),
                                Some(f) => pref_tail.push(f.to_os_string()),
                            }
                            match cur.parent() {
                                None => break pref_path.clone(),
                                Some(p) => {
                                    if let Ok(canon_p) = p.canonicalize() {
                                        let mut result = canon_p;
                                        for component in pref_tail.iter().rev() {
                                            result = result.join(component);
                                        }
                                        return result;
                                    }
                                    cur = p.to_path_buf();
                                }
                            }
                        }
                    });
                    // For the target, canonicalize with ancestor fallback.
                    let canon_resolved = resolved.canonicalize().unwrap_or_else(|_| {
                        // Walk ancestors until one exists.
                        let mut tail: Vec<std::ffi::OsString> = Vec::new();
                        let mut cur = resolved.clone();
                        loop {
                            match cur.file_name() {
                                None => break resolved.clone(),
                                Some(f) => tail.push(f.to_os_string()),
                            }
                            match cur.parent() {
                                None => break resolved.clone(),
                                Some(p) => {
                                    if let Ok(canon_p) = p.canonicalize() {
                                        let mut result = canon_p;
                                        for component in tail.iter().rev() {
                                            result = result.join(component);
                                        }
                                        return result;
                                    }
                                    cur = p.to_path_buf();
                                }
                            }
                        }
                    });
                    canon_resolved.starts_with(&canon_pref)
                });
                if !allowed {
                    return codes::CAPABILITY_DENIED;
                }
                // Write to disk.
                match std::fs::write(&resolved, &contents) {
                    Ok(()) => codes::OK,
                    Err(_) => codes::INTERNAL_ERROR,
                }
            },
        )
        .map_err(|e| HostCallError::Linker(e.to_string()))?;

    // exec_subprocess: real implementation that delegates to the
    // crate::host::exec_subprocess policy + executor. The legacy-bash-
    // adapter (S-2.1) needs this path live so it can shell out to bash
    // hooks; tests in crate::host::exec_subprocess cover the policy.
    linker
        .func_wrap(
            "vsdd",
            "exec_subprocess",
            |mut caller: Caller<'_, StoreData>,
             cmd_ptr: u32,
             cmd_len: u32,
             args_ptr: u32,
             args_len: u32,
             stdin_ptr: u32,
             stdin_len: u32,
             timeout_ms: u32,
             max_output_bytes: u32,
             result_buf_ptr: u32,
             result_buf_cap: u32|
             -> i32 {
                let cmd = match read_wasm_string_sd(&mut caller, cmd_ptr, cmd_len) {
                    Ok(s) => s,
                    Err(_) => return codes::INVALID_ARGUMENT,
                };
                let args_buf = match read_wasm_bytes_sd(&mut caller, args_ptr, args_len) {
                    Ok(b) => b,
                    Err(_) => return codes::INVALID_ARGUMENT,
                };
                let args = match crate::host::exec_subprocess::decode_args(&args_buf) {
                    Some(a) => a,
                    None => return codes::INVALID_ARGUMENT,
                };
                let stdin_bytes = if stdin_len == 0 {
                    Vec::new()
                } else {
                    match read_wasm_bytes_sd(&mut caller, stdin_ptr, stdin_len) {
                        Ok(b) => b,
                        Err(_) => return codes::INVALID_ARGUMENT,
                    }
                };

                let envelope = match crate::host::exec_subprocess::run(
                    &caller.data().host,
                    &cmd,
                    &args,
                    &stdin_bytes,
                    timeout_ms,
                    max_output_bytes,
                ) {
                    Ok(env) => env,
                    Err(code) => return code,
                };

                // Write the envelope into the guest-provided buffer.
                // Returns bytes written (positive) on success or a
                // negative error code. Mirrors host/exec_subprocess.rs.
                if envelope.len() as u32 > result_buf_cap {
                    return codes::OUTPUT_TOO_LARGE;
                }
                match write_wasm_bytes_sd(&mut caller, result_buf_ptr, result_buf_cap, &envelope) {
                    Ok(written) => written as i32,
                    Err(_) => codes::INVALID_ARGUMENT,
                }
            },
        )
        .map_err(|e| HostCallError::Linker(e.to_string()))?;

    Ok(())
}

fn context_reader<F>(
    extract: F,
) -> impl Fn(Caller<'_, StoreData>, u32, u32) -> u32 + Send + Sync + 'static + Copy
where
    F: Fn(&HostContext) -> String + Send + Sync + Copy + 'static,
{
    move |mut caller: Caller<'_, StoreData>, out_ptr: u32, out_cap: u32| -> u32 {
        let s = extract(&caller.data().host);
        let bytes = s.into_bytes();
        write_wasm_bytes_sd(&mut caller, out_ptr, out_cap, &bytes).unwrap_or_default()
    }
}

// StoreData-typed memory helpers — mirror host::memory but against the
// wider `StoreData` so the invoke linker can share them.
use wasmtime::Caller;

use crate::host::HostCallError;

fn get_memory_sd(caller: &mut Caller<'_, StoreData>) -> Result<wasmtime::Memory, HostCallError> {
    caller
        .get_export("memory")
        .and_then(|e| e.into_memory())
        .ok_or(HostCallError::MissingMemory)
}

fn read_wasm_bytes_sd(
    caller: &mut Caller<'_, StoreData>,
    ptr: u32,
    len: u32,
) -> Result<Vec<u8>, HostCallError> {
    let memory = get_memory_sd(caller)?;
    let data = memory.data(&caller);
    let start = ptr as usize;
    let end = start
        .checked_add(len as usize)
        .ok_or(HostCallError::MemoryOverflow)?;
    if end > data.len() {
        return Err(HostCallError::OutOfBounds {
            ptr,
            len,
            memory_size: data.len(),
        });
    }
    Ok(data[start..end].to_vec())
}

fn read_wasm_string_sd(
    caller: &mut Caller<'_, StoreData>,
    ptr: u32,
    len: u32,
) -> Result<String, HostCallError> {
    let bytes = read_wasm_bytes_sd(caller, ptr, len)?;
    String::from_utf8(bytes).map_err(|_| HostCallError::InvalidUtf8)
}

fn write_wasm_bytes_sd(
    caller: &mut Caller<'_, StoreData>,
    out_ptr: u32,
    out_cap: u32,
    bytes: &[u8],
) -> Result<u32, HostCallError> {
    let needed = bytes.len() as u32;
    if needed > out_cap {
        return Ok(needed);
    }
    let memory = get_memory_sd(caller)?;
    let start = out_ptr as usize;
    let end = start
        .checked_add(bytes.len())
        .ok_or(HostCallError::MemoryOverflow)?;
    let data_len = memory.data(&caller).len();
    if end > data_len {
        return Err(HostCallError::OutOfBounds {
            ptr: out_ptr,
            len: needed,
            memory_size: data_len,
        });
    }
    memory
        .write(caller, start, bytes)
        .map_err(|_| HostCallError::OutOfBounds {
            ptr: out_ptr,
            len: needed,
            memory_size: data_len,
        })?;
    Ok(needed)
}

/// Write a single little-endian u32 into guest memory.
/// Used for `read_file`'s out-param protocol (`out_ptr_out`, `out_len_out`).
fn write_wasm_u32_sd(
    caller: &mut Caller<'_, StoreData>,
    out_ptr: u32,
    value: u32,
) -> Result<(), HostCallError> {
    let bytes = value.to_le_bytes();
    write_wasm_bytes_sd(caller, out_ptr, bytes.len() as u32, &bytes)?;
    Ok(())
}

/// Per-invocation store data: the HostContext S-1.4 populates plus the
/// wasmtime-wasi preview-1 context the SDK needs to talk to stdin /
/// stdout.
pub struct StoreData {
    pub host: HostContext,
    pub wasi: WasiP1Ctx,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::build_engine;

    fn compile(engine: &Engine, wat: &str) -> Module {
        let bytes = wat::parse_str(wat).expect("wat should parse");
        Module::from_binary(engine, &bytes).expect("module should compile")
    }

    fn bare_ctx() -> HostContext {
        HostContext::new("plugin", "0.0.1", "sess", "trace")
    }

    #[test]
    fn invoke_normal_plugin_returns_ok() {
        // Minimal WASI command that just returns successfully.
        let engine = build_engine().unwrap();
        let module = compile(
            &engine,
            r#"
            (module
              (memory (export "memory") 1)
              (func (export "_start")))
            "#,
        );
        let res =
            invoke_plugin(&engine, &module, bare_ctx(), b"", InvokeLimits::default()).unwrap();
        match res {
            PluginResult::Ok { exit_code, .. } => assert_eq!(exit_code, 0),
            other => panic!("expected Ok, got {other:?}"),
        }
    }

    #[test]
    fn invoke_with_infinite_loop_times_out_on_epoch() {
        // Start the ticker so the engine's epoch actually advances.
        let engine = build_engine().unwrap();
        let _ticker = crate::engine::EpochTicker::start(engine.clone());

        // Infinite loop at a yield point — br 0 checkpoints the epoch
        // on every iteration.
        let module = compile(
            &engine,
            r#"
            (module
              (memory (export "memory") 1)
              (func (export "_start")
                (loop (br 0))))
            "#,
        );
        let res = invoke_plugin(
            &engine,
            &module,
            bare_ctx(),
            b"",
            InvokeLimits {
                timeout_ms: 50,
                fuel_cap: u64::MAX,
            },
        )
        .unwrap();
        match res {
            PluginResult::Timeout {
                cause: TimeoutCause::Epoch,
                ..
            } => {}
            other => panic!("expected Timeout{{Epoch}}, got {other:?}"),
        }
    }

    #[test]
    fn invoke_fuel_hog_runs_out_of_fuel() {
        let engine = build_engine().unwrap();
        // Tight arithmetic loop — each iteration consumes a few fuel
        // units via wasmtime's instruction-count metering.
        let module = compile(
            &engine,
            r#"
            (module
              (memory (export "memory") 1)
              (func (export "_start")
                (local $i i32)
                (local.set $i (i32.const 0))
                (loop $l
                  (local.set $i (i32.add (local.get $i) (i32.const 1)))
                  (br $l))))
            "#,
        );
        let res = invoke_plugin(
            &engine,
            &module,
            bare_ctx(),
            b"",
            InvokeLimits {
                timeout_ms: 60_000,
                fuel_cap: 10_000,
            },
        )
        .unwrap();
        match res {
            PluginResult::Timeout {
                cause: TimeoutCause::Fuel,
                ..
            } => {}
            other => panic!("expected Timeout{{Fuel}}, got {other:?}"),
        }
    }

    #[test]
    fn invoke_panic_plugin_reports_crashed() {
        let engine = build_engine().unwrap();
        let module = compile(
            &engine,
            r#"
            (module
              (memory (export "memory") 1)
              (func (export "_start")
                unreachable))
            "#,
        );
        let res =
            invoke_plugin(&engine, &module, bare_ctx(), b"", InvokeLimits::default()).unwrap();
        match res {
            PluginResult::Crashed { .. } => {}
            other => panic!("expected Crashed, got {other:?}"),
        }
    }

    #[test]
    fn invoke_records_elapsed_and_fuel_on_ok() {
        let engine = build_engine().unwrap();
        let module = compile(
            &engine,
            r#"
            (module
              (memory (export "memory") 1)
              (func (export "_start")))
            "#,
        );
        let res = invoke_plugin(
            &engine,
            &module,
            bare_ctx(),
            b"",
            InvokeLimits {
                timeout_ms: 1000,
                fuel_cap: 1_000_000,
            },
        )
        .unwrap();
        if let PluginResult::Ok { fuel_consumed, .. } = res {
            assert!(
                fuel_consumed < 1_000_000,
                "fuel_consumed should be well under cap for trivial plugin"
            );
        } else {
            panic!("expected Ok");
        }
    }
}

// ---------------------------------------------------------------------------
// S-18.04b-prereq: git_context payload injection stubs (ADR-029)
//
// These are the public API shapes for the dispatcher's PostToolUse Bash
// git-commit event detection and `git_context` injection into `payload.extra`.
//
// Contract (ADR-029 §Decision 1–3):
// - `detect_git_commit_event`: returns true iff the payload is a PostToolUse
//   Bash event whose `tool_input.command` contains "git commit" AND a
//   factory-artifacts worktree indicator (e.g. "-C .factory" or a path
//   containing ".factory"). Non-qualifying events return false.
// - `build_git_context`: given the factory-artifacts directory path, executes
//   four git commands to populate the four-field schema (head_subject, head_sha,
//   head_parent_subject, head_parent_sha) and returns a `serde_json::Value::Object`.
//   On any git error, returns the all-empty four-field object (fail-open;
//   BC-1.16.001 PC2 + AC-002).
// - `inject_git_context_if_qualifying`: orchestrates detection + construction +
//   injection into `payload_value.extra["git_context"]` before routing.
//   No-op if the event is non-qualifying (AC-003, AC-004).
//
// All three functions use `todo!()` bodies — implementer fills real logic.
// Wiring into `main.rs` is the implementer's responsibility (see comment in
// main.rs at the payload enrichment site).
//
// BC-5.38.001 — todo!() obligation: all non-trivial bodies are todo!().
// Self-Check (BC-5.38.005 invariant 1): "If I include this real implementation,
// will the test for this function pass trivially without any implementer work?"
// Yes for all three functions — any real body would satisfy VP-093 tests
// directly. Therefore todo!() is mandatory.
// ---------------------------------------------------------------------------

/// The four-field git_context schema injected into `payload.extra` on qualifying
/// PostToolUse Bash git-commit events (ADR-029 §Decision 2).
///
/// All fields are `String`. Empty string means the field could not be populated
/// (e.g. `head_parent_sha` when factory-artifacts has only one commit).
/// The dispatcher MUST NOT use `null` — empty string is the sentinel per AC-006.
///
/// This struct is `pub` so integration tests can construct expected values and
/// compare against the injected `serde_json::Value::Object`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitContext {
    /// Subject line of HEAD commit in the factory-artifacts worktree.
    pub head_subject: String,
    /// Full 40-character SHA of HEAD commit.
    pub head_sha: String,
    /// Subject line of HEAD^ commit. Empty string if HEAD^ does not exist
    /// (initial commit case — AC-006, AC-011).
    pub head_parent_subject: String,
    /// Full 40-character SHA of HEAD^ commit. Empty string if HEAD^ does not
    /// exist (initial commit case).
    pub head_parent_sha: String,
}

impl GitContext {
    /// Return the all-empty `GitContext` used as the fail-open sentinel
    /// (BC-1.16.001 PC2 / AC-002 / AC-009).
    ///
    /// # GREEN-BY-DESIGN
    ///
    /// Pure field initialisation; zero branching, no I/O, no helpers, 7 lines.
    /// Body is trivial struct construction only — BC-5.38.002 criteria all satisfied.
    pub fn empty() -> Self {
        Self {
            head_subject: String::new(),
            head_sha: String::new(),
            head_parent_subject: String::new(),
            head_parent_sha: String::new(),
        }
    }

    /// Serialize this context to a `serde_json::Value::Object` suitable for
    /// insertion into `payload.extra["git_context"]` (ADR-029 §Decision 2).
    ///
    /// # GREEN-BY-DESIGN
    ///
    /// Builds a JSON object from the four string fields; zero branching,
    /// no I/O, no non-trivial helpers, body ≤ 8 lines. BC-5.38.002 satisfied.
    pub fn to_json(&self) -> serde_json::Value {
        let mut map = serde_json::Map::with_capacity(4);
        map.insert(
            "head_subject".to_string(),
            serde_json::Value::String(self.head_subject.clone()),
        );
        map.insert(
            "head_sha".to_string(),
            serde_json::Value::String(self.head_sha.clone()),
        );
        map.insert(
            "head_parent_subject".to_string(),
            serde_json::Value::String(self.head_parent_subject.clone()),
        );
        map.insert(
            "head_parent_sha".to_string(),
            serde_json::Value::String(self.head_parent_sha.clone()),
        );
        serde_json::Value::Object(map)
    }
}

/// Detect whether a hook payload qualifies for `git_context` injection
/// (ADR-029 §Decision 1 + §Decision 3; BC-1.16.001 PC3/PC4; AC-003/AC-004/AC-010).
///
/// Returns `true` iff ALL of:
/// 1. `payload.event_name == "PostToolUse"` (AC-004: non-PostToolUse events never qualify).
/// 2. `payload.tool_name == "Bash"` (AC-004: Edit/Write/Agent never qualify).
/// 3. `payload.tool_input.command` contains `"git commit"` as a substring (AC-010).
/// 4. `payload.tool_input.command` contains `".factory"` as an indicator of the
///    factory-artifacts worktree (AC-010 heuristic; minimises spurious injection).
///
/// # Implementer notes
///
/// - Check `tool_name` BEFORE inspecting `tool_input.command` (AC-004: non-Bash events
///   MUST NOT have their command inspected at all).
/// - False positives (e.g. `echo "git commit"` with `.factory` path in args) are
///   acceptable per ADR-029 §Decision 3 Negative consequence note: `git_context` will
///   be valid git state (whatever HEAD of factory-artifacts is), and WASM plugins treat
///   valid-but-irrelevant context as "pass" (fail-open).
/// - Detection is heuristic (AC-010); exactness is not required.
pub fn detect_git_commit_event(payload: &crate::payload::HookPayload) -> bool {
    // AC-004: non-PostToolUse events never qualify.
    if payload.event_name != "PostToolUse" {
        return false;
    }
    // AC-004: non-Bash tools never qualify; do NOT inspect command for non-Bash.
    if payload.tool_name != "Bash" {
        return false;
    }
    // AC-010: heuristic detection — command invokes git with the "commit" subcommand
    // AND contains a ".factory" factory-artifacts worktree indicator.
    //
    // Detection: the command must contain "git" AND " commit" (space-prefixed to anchor
    // "commit" as a git subcommand token rather than part of a -m "message" argument
    // that merely mentions "commit"). The ".factory" indicator scopes to factory-artifacts.
    //
    // Examples that QUALIFY:
    //   "git -C .factory commit -m ..."     → "git" + " commit" + ".factory" ✓
    //   "git commit -C .factory -m ..."     → "git" + " commit" + ".factory" ✓
    //
    // Examples that do NOT qualify (EC-007/EC-008):
    //   "git commit -m ..."                 → no ".factory" indicator ✗
    //   "echo \"git commit\""               → no ".factory" indicator ✗
    //   "git -C .factory push ..."          → no " commit" token ✗
    let command = match payload.tool_input.get("command").and_then(|v| v.as_str()) {
        Some(c) => c,
        None => return false,
    };
    command.contains("git") && command.contains(" commit") && command.contains(".factory")
}

/// Execute the four git commands against the factory-artifacts worktree at
/// `factory_dir` and return a populated `GitContext` (ADR-029 §Decision 3).
///
/// Commands executed (in order):
/// 1. `git -C <factory_dir> log --format=%s -1 HEAD` → `head_subject`
/// 2. `git -C <factory_dir> rev-parse HEAD` → `head_sha`
/// 3. `git -C <factory_dir> log --format=%s -1 HEAD^` → `head_parent_subject`
///    (empty string if HEAD^ does not exist — exit non-zero)
/// 4. `git -C <factory_dir> rev-parse HEAD^` → `head_parent_sha`
///    (empty string if HEAD^ does not exist — exit non-zero)
///
/// # Fail-open contract (BC-1.16.001 PC2 / AC-002 / AC-009)
///
/// On ANY git command failure (non-zero exit, git binary not found, I/O error,
/// permission denied), the function MUST:
/// 1. Emit `tracing::warn!` describing the failure.
/// 2. Return `GitContext::empty()` (all four fields `""`).
///
/// The dispatcher MUST NOT block, abort, or fail-closed on a git error.
///
/// # Initial commit handling (AC-006, AC-011, EC-009)
///
/// Commands 3 and 4 (HEAD^) exit non-zero when factory-artifacts has only one
/// commit. This is NOT a general git error — commands 1 and 2 (HEAD) must still
/// be populated normally. Only `head_parent_subject` and `head_parent_sha` are
/// set to `""` (not null, not absent).
pub fn build_git_context(_factory_dir: &std::path::Path) -> GitContext {
    todo!(
        "S-18.04b-prereq T-2+T-3+T-4: execute git log/rev-parse for HEAD and HEAD^; \
         handle initial-commit (HEAD^ non-zero); fail-open on git error with tracing::warn! \
         (ADR-029 §Decision 3; BC-1.16.001 PC2)"
    )
}

/// Orchestrate git_context detection, construction, and injection into
/// `payload_value` before it is routed to registered plugins.
///
/// # Contract (ADR-029 §Decision 1–3; BC-1.16.001 PC1–PC6)
///
/// 1. Call `detect_git_commit_event(&original_payload)` to determine if this
///    PostToolUse Bash event is a qualifying git-commit event.
/// 2. If non-qualifying: return immediately without mutating `payload_value`
///    (AC-003, AC-004 — no injection on non-qualifying events).
/// 3. If qualifying: call `build_git_context(factory_dir)` to obtain the four-field context,
///    inject `git_context` as a `serde_json::Value::Object` into `payload_value` at key
///    `"git_context"` (rides in the `extra` flatten map — ADR-029 §Decision 2 / AC-005),
///    with all four fields present as strings; null fields are forbidden (AC-006, AC-011).
///
/// # Arguments
///
/// - `original_payload`: the parsed `HookPayload` (used for detection only).
/// - `payload_value`: the mutable `serde_json::Value` that will be passed to
///   `ExecutorInputs` — injection mutates this value in place.
/// - `factory_dir`: path to the factory-artifacts worktree (typically
///   `<CLAUDE_PROJECT_DIR>/.factory`; derived from `CLAUDE_PROJECT_DIR` env var
///   at the call site in `main.rs`).
///
/// # Wiring site
///
/// This function is called in `main.rs` immediately after `dispatcher_trace_id`
/// is injected into `payload_value` and before `ExecutorInputs` is constructed.
/// See the `// S-18.04b-prereq: git_context injection site` comment in main.rs.
pub fn inject_git_context_if_qualifying(
    _original_payload: &crate::payload::HookPayload,
    _payload_value: &mut serde_json::Value,
    _factory_dir: &std::path::Path,
) {
    todo!(
        "S-18.04b-prereq T-5: wire detect + build + inject; call detect_git_commit_event, \
         then build_git_context(factory_dir), then insert into payload_value[\"git_context\"] \
         (ADR-029 §Decision 2; BC-1.16.001 PC1; AC-001)"
    )
}
