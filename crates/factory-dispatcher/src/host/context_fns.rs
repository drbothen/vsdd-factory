//! Context-read host functions: `session_id`, `dispatcher_trace_id`,
//! `plugin_root`, `plugin_version`, `cwd`.
//!
//! All five share the `(out_ptr, out_cap) -> bytes_written` shape. If
//! the caller's buffer is too small, the host returns the **required
//! capacity** without writing, so the SDK re-allocates and retries
//! (see `vsdd_hook_sdk::host::read_string`).

use wasmtime::Linker;

use super::memory::write_wasm_bytes;
use super::{HostCallError, HostCaller, HostContext};

pub fn register(linker: &mut Linker<HostContext>) -> Result<(), HostCallError> {
    linker
        .func_wrap(
            "vsdd",
            "session_id",
            |mut caller: HostCaller<'_>, out_ptr: u32, out_cap: u32| -> u32 {
                write_context_string(&mut caller, out_ptr, out_cap, |c| c.session_id.clone())
            },
        )
        .map_err(|e| HostCallError::Linker(e.to_string()))?;

    linker
        .func_wrap(
            "vsdd",
            "dispatcher_trace_id",
            |mut caller: HostCaller<'_>, out_ptr: u32, out_cap: u32| -> u32 {
                write_context_string(&mut caller, out_ptr, out_cap, |c| {
                    c.dispatcher_trace_id.clone()
                })
            },
        )
        .map_err(|e| HostCallError::Linker(e.to_string()))?;

    linker
        .func_wrap(
            "vsdd",
            "plugin_root",
            |mut caller: HostCaller<'_>, out_ptr: u32, out_cap: u32| -> u32 {
                write_context_string(&mut caller, out_ptr, out_cap, |c| {
                    c.plugin_root.to_string_lossy().into_owned()
                })
            },
        )
        .map_err(|e| HostCallError::Linker(e.to_string()))?;

    linker
        .func_wrap(
            "vsdd",
            "plugin_version",
            |mut caller: HostCaller<'_>, out_ptr: u32, out_cap: u32| -> u32 {
                write_context_string(&mut caller, out_ptr, out_cap, |c| c.plugin_version.clone())
            },
        )
        .map_err(|e| HostCallError::Linker(e.to_string()))?;

    linker
        .func_wrap(
            "vsdd",
            "cwd",
            |mut caller: HostCaller<'_>, out_ptr: u32, out_cap: u32| -> u32 {
                write_context_string(&mut caller, out_ptr, out_cap, |c| {
                    c.cwd.to_string_lossy().into_owned()
                })
            },
        )
        .map_err(|e| HostCallError::Linker(e.to_string()))?;

    Ok(())
}

/// Shared body for the five `(out_ptr, out_cap) -> u32` host fns.
fn write_context_string<F>(
    caller: &mut HostCaller<'_>,
    out_ptr: u32,
    out_cap: u32,
    extract: F,
) -> u32
where
    F: FnOnce(&HostContext) -> String,
{
    let s = extract(caller.data());
    let bytes = s.into_bytes();
    // On bounds error we treat the call as if it wrote zero bytes; the
    // SDK wrapper will surface that as an empty string. Nothing here is
    // security-critical — these are context getters, not capability-gated.
    write_wasm_bytes(caller, out_ptr, out_cap, &bytes).unwrap_or_default()
}
