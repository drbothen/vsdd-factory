//! `env` host function: read a single environment variable.
//!
//! Capability: the variable name must be listed in the plugin's
//! `Capabilities::env_allow`. Names not on the allow-list return
//! [`codes::CAPABILITY_DENIED`] and emit
//! `internal.capability_denied` so operators can see what the plugin
//! tried to read.
//!
//! Unset but permitted variables return `0` (zero bytes written) —
//! the SDK wrapper surfaces that as `Ok(None)` rather than an error.

use serde_json::{Map, Value};
use wasmtime::Linker;

use super::memory::{read_wasm_string, write_wasm_bytes};
use super::{HostCallError, HostCaller, HostContext, codes};

pub fn register(linker: &mut Linker<HostContext>) -> Result<(), HostCallError> {
    linker
        .func_wrap(
            "vsdd",
            "env",
            |mut caller: HostCaller<'_>,
             name_ptr: u32,
             name_len: u32,
             out_ptr: u32,
             out_cap: u32|
             -> i32 {
                let name = match read_wasm_string(&mut caller, name_ptr, name_len) {
                    Ok(s) => s,
                    Err(_) => return codes::INVALID_ARGUMENT,
                };
                let ctx = caller.data();
                if !env_allowed(ctx, &name) {
                    emit_denial(ctx, &name, "env_not_on_allow_list");
                    return codes::CAPABILITY_DENIED;
                }
                let value = ctx.env_view.get(&name).cloned();
                match value {
                    None => 0,
                    Some(v) => {
                        let bytes = v.into_bytes();
                        match write_wasm_bytes(&mut caller, out_ptr, out_cap, &bytes) {
                            Ok(n) => n as i32,
                            Err(_) => codes::INVALID_ARGUMENT,
                        }
                    }
                }
            },
        )
        .map_err(|e| HostCallError::Linker(e.to_string()))?;
    Ok(())
}

fn env_allowed(ctx: &HostContext, name: &str) -> bool {
    ctx.capabilities.env_allow.iter().any(|n| n == name)
}

fn emit_denial(ctx: &HostContext, name: &str, reason: &str) {
    let mut details = Map::new();
    details.insert("variable".to_string(), Value::String(name.to_string()));
    let ev = ctx.denial_event("env", reason, details);
    ctx.emit_internal(ev);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::test_support::*;

    #[test]
    fn allow_list_grants_access() {
        let mut ctx = context_with_caps(crate::registry::Capabilities {
            env_allow: vec!["FOO".to_string()],
            ..Default::default()
        });
        ctx.env_view.insert("FOO".to_string(), "bar".to_string());
        assert!(env_allowed(&ctx, "FOO"));
        assert!(!env_allowed(&ctx, "SECRET"));
    }

    #[test]
    fn denial_for_missing_allow_list() {
        let ctx = bare_context();
        assert!(!env_allowed(&ctx, "ANYTHING"));
    }
}
