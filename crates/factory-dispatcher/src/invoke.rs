//! Plugin invocation: create a Store, set the epoch deadline and fuel
//! budget, pipe the payload through WASI stdin, run `_start`, capture
//! stdout, and classify the outcome.
//!
//! Everything except the bounded cost enforcement (epoch interruption,
//! fuel consumption) happens synchronously. The per-invocation timeout
//! is honored by the shared [`EpochTicker`]; each invocation just sets
//! its own deadline before calling `_start`.

use std::time::Instant;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use wasmtime::{Engine, Module, Store, Trap};
use wasmtime_wasi::p1::{self, WasiP1Ctx};
use wasmtime_wasi::p2::pipe::{MemoryInputPipe, MemoryOutputPipe};
use wasmtime_wasi::{I32Exit, WasiCtxBuilder};

use crate::engine::timeout_ms_to_epochs;
use crate::host::{HostContext, setup_linker};

/// Outcome of a single `invoke_plugin` call.
///
/// `fuel_consumed` is always populated so operators can see how close
/// normal plugins came to their budget. `elapsed_ms` is wall-clock from
/// just before `_start` to just after the result is classified.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PluginResult {
    Ok {
        /// Process exit code the plugin's `fn main()` returned.
        exit_code: i32,
        /// Raw stdout (UTF-8 `HookResult` JSON in well-behaved plugins).
        stdout: String,
        elapsed_ms: u64,
        fuel_consumed: u64,
    },
    Timeout {
        cause: TimeoutCause,
        elapsed_ms: u64,
        fuel_consumed: u64,
    },
    Crashed {
        trap_string: String,
        elapsed_ms: u64,
        fuel_consumed: u64,
    },
}

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

    let wasi_ctx = WasiCtxBuilder::new()
        .stdin(MemoryInputPipe::new(payload_json.to_vec()))
        .stdout(stdout.clone())
        .stderr(stderr.clone())
        .build_p1();

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
            Ok(PluginResult::Ok {
                exit_code: 0,
                stdout: out,
                elapsed_ms,
                fuel_consumed,
            })
        }
        Err(err) => classify_trap(anyhow::Error::from(err), &stdout, elapsed_ms, fuel_consumed),
    }
}

fn classify_trap(
    err: anyhow::Error,
    stdout: &MemoryOutputPipe,
    elapsed_ms: u64,
    fuel_consumed: u64,
) -> Result<PluginResult, InvokeError> {
    // WASI `exit(n)` propagates as an `I32Exit` in wasmtime-wasi's
    // preview-1 glue; non-zero exit is still "Ok" from our POV since
    // the plugin ran to a controlled finish.
    if let Some(exit) = err.downcast_ref::<I32Exit>() {
        let out = stdout_to_string(stdout);
        return Ok(PluginResult::Ok {
            exit_code: exit.0,
            stdout: out,
            elapsed_ms,
            fuel_consumed,
        });
    }
    if let Some(trap) = err.downcast_ref::<Trap>() {
        return Ok(match trap {
            Trap::Interrupt => PluginResult::Timeout {
                cause: TimeoutCause::Epoch,
                elapsed_ms,
                fuel_consumed,
            },
            Trap::OutOfFuel => PluginResult::Timeout {
                cause: TimeoutCause::Fuel,
                elapsed_ms,
                fuel_consumed,
            },
            other => PluginResult::Crashed {
                trap_string: other.to_string(),
                elapsed_ms,
                fuel_consumed,
            },
        });
    }
    // Any other wasmtime error surfaces as Crashed — preserves the
    // exact diagnostic for operators.
    Ok(PluginResult::Crashed {
        trap_string: format!("{err:#}"),
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
                    if [
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

    // read_file isn't reachable by any in-tree plugin yet. Register a
    // stub that returns CAPABILITY_DENIED so a misbehaving plugin fails
    // loudly rather than traps on a missing import.
    linker
        .func_wrap(
            "vsdd",
            "read_file",
            |_caller: Caller<'_, StoreData>,
             _p1: u32,
             _p2: u32,
             _p3: u32,
             _p4: u32,
             _p5: u32,
             _p6: u32|
             -> i32 { codes::CAPABILITY_DENIED },
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
             result_ptr_out: u32,
             result_len_out: u32|
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

                // Write the envelope at offset 0 of guest memory and
                // tell the guest where to find it via the out pointers.
                // Same protocol as host/exec_subprocess.rs — see that
                // module for the trade-off note about offset 0.
                if write_wasm_u32_sd(&mut caller, result_ptr_out, 0).is_err() {
                    return codes::INVALID_ARGUMENT;
                }
                if write_wasm_u32_sd(&mut caller, result_len_out, envelope.len() as u32).is_err() {
                    return codes::INVALID_ARGUMENT;
                }
                let written =
                    match write_wasm_bytes_sd(&mut caller, 0, envelope.len() as u32, &envelope) {
                        Ok(n) => n,
                        Err(_) => return codes::INVALID_ARGUMENT,
                    };
                if written != envelope.len() as u32 {
                    return codes::OUTPUT_TOO_LARGE;
                }
                codes::OK
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

fn write_wasm_u32_sd(
    caller: &mut Caller<'_, StoreData>,
    out_ptr: u32,
    value: u32,
) -> Result<(), HostCallError> {
    let bytes = value.to_le_bytes();
    let written = write_wasm_bytes_sd(caller, out_ptr, bytes.len() as u32, &bytes)?;
    if written != bytes.len() as u32 {
        return Err(HostCallError::OutOfBounds {
            ptr: out_ptr,
            len: bytes.len() as u32,
            memory_size: 0,
        });
    }
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
