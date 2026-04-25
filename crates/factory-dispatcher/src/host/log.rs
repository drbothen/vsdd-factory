//! `log` host function.
//!
//! Writes a line at `level` into the dispatcher internal log with the
//! plugin name / version / trace id / session id as envelope fields.
//! Best-effort: log failures are swallowed just like every other
//! [`crate::internal_log::InternalLog`] write.

use serde_json::Value;
use wasmtime::Linker;

use super::memory::read_wasm_string;
use super::{HostCallError, HostCaller, HostContext};
use crate::internal_log::InternalEvent;

const EVENT_TYPE: &str = "plugin.log";

pub fn register(linker: &mut Linker<HostContext>) -> Result<(), HostCallError> {
    linker
        .func_wrap(
            "vsdd",
            "log",
            |mut caller: HostCaller<'_>, level: u32, msg_ptr: u32, msg_len: u32| {
                let msg = match read_wasm_string(&mut caller, msg_ptr, msg_len) {
                    Ok(s) => s,
                    Err(_) => return, // best-effort; drop bad strings silently
                };
                let level_str = level_to_str(level);
                let ctx = caller.data();
                let ev = InternalEvent::now(EVENT_TYPE)
                    .with_trace_id(&ctx.dispatcher_trace_id)
                    .with_session_id(&ctx.session_id)
                    .with_plugin_name(&ctx.plugin_name)
                    .with_plugin_version(&ctx.plugin_version)
                    .with_field("level", Value::String(level_str.to_string()))
                    .with_field("message", Value::String(msg));
                ctx.emit_internal(ev);
            },
        )
        .map_err(|e| HostCallError::Linker(e.to_string()))?;
    Ok(())
}

fn level_to_str(level: u32) -> &'static str {
    match level {
        0 => "trace",
        1 => "debug",
        2 => "info",
        3 => "warn",
        4 => "error",
        _ => "info",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_mapping_matches_sdk() {
        assert_eq!(level_to_str(0), "trace");
        assert_eq!(level_to_str(1), "debug");
        assert_eq!(level_to_str(2), "info");
        assert_eq!(level_to_str(3), "warn");
        assert_eq!(level_to_str(4), "error");
    }

    #[test]
    fn unknown_level_falls_back_to_info() {
        assert_eq!(level_to_str(99), "info");
    }
}
