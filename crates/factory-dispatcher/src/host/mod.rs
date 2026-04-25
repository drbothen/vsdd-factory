//! Host function surface (S-1.4).
//!
//! Implements the dispatcher side of the `vsdd` wasm import module
//! documented in [`crates/hook-sdk/HOST_ABI.md`]. Every host function
//! the SDK declares is registered with a wasmtime [`Linker`] through
//! [`setup_linker`], so the wasmtime engine built by S-1.5 can just
//! instantiate any compiled plugin against this linker.
//!
//! Capability enforcement is deny-by-default: a plugin can only call
//! `read_file` / `exec_subprocess` / `env` when its registry entry
//! carries a matching [`Capabilities`] block. Denied calls return a
//! negative error code *and* emit `internal.capability_denied` so
//! operators see exactly what the plugin tried to do.
//!
//! Guest memory access goes through [`memory::read_wasm_string`] and
//! [`memory::write_wasm_bytes`]; every pointer is bounds-checked
//! against the current wasm memory size on every call so a bad guest
//! pointer cannot take the dispatcher down.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use serde_json::{Map, Value};
use thiserror::Error;
use wasmtime::{Caller, Engine, Linker};

use crate::internal_log::{INTERNAL_CAPABILITY_DENIED, InternalEvent};
use crate::registry::Capabilities;

pub mod context_fns;
pub mod emit_event;
pub mod env;
pub mod exec_subprocess;
pub mod log;
pub mod memory;
pub mod read_file;

/// Per-invocation state available to every host function. Lives in the
/// wasmtime [`Store`] for the plugin call and is torn down when the
/// call returns.
pub struct HostContext {
    pub plugin_name: String,
    pub plugin_version: String,
    pub plugin_root: PathBuf,
    pub session_id: String,
    pub dispatcher_trace_id: String,
    pub capabilities: Capabilities,
    pub cwd: PathBuf,

    /// Host-controlled env projection. The dispatcher populates this
    /// from its own process environment, filtered through the plugin's
    /// env allow-list (see [`crate::registry::Capabilities::env_allow`]),
    /// BEFORE the plugin can ever call [`env::env_host`]. Tests can
    /// seed it directly.
    pub env_view: std::collections::HashMap<String, String>,

    /// Stub event sink. `emit_event` pushes enriched events here; the
    /// real consumer is S-1.8's file sink (and the multi-sink router
    /// in S-1.9 / S-4.x). For this story the events live in a
    /// `Mutex<Vec<_>>` that tests can drain.
    pub events: Arc<Mutex<Vec<InternalEvent>>>,

    /// Mirror of the always-on internal log. Used for
    /// `internal.capability_denied` / `internal.host_function_panic`
    /// emission inside host functions so denials are durable even
    /// when the event sink is still stubbed. `None` in tests.
    pub internal_log: Option<Arc<crate::internal_log::InternalLog>>,
}

impl HostContext {
    /// Convenience constructor for tests and S-1.5 plugin instantiation.
    pub fn new(
        plugin_name: impl Into<String>,
        plugin_version: impl Into<String>,
        session_id: impl Into<String>,
        dispatcher_trace_id: impl Into<String>,
    ) -> Self {
        Self {
            plugin_name: plugin_name.into(),
            plugin_version: plugin_version.into(),
            plugin_root: PathBuf::new(),
            session_id: session_id.into(),
            dispatcher_trace_id: dispatcher_trace_id.into(),
            capabilities: Capabilities::default(),
            cwd: PathBuf::new(),
            env_view: std::collections::HashMap::new(),
            events: Arc::new(Mutex::new(Vec::new())),
            internal_log: None,
        }
    }

    /// Drain the stub event queue. Tests + S-1.8 consumer use this.
    pub fn drain_events(&self) -> Vec<InternalEvent> {
        let mut guard = self.events.lock().expect("events mutex poisoned");
        std::mem::take(&mut *guard)
    }

    /// Emit an internal event through the internal log (if wired) and
    /// push it onto the event queue. Used by every host function for
    /// `internal.*` diagnostics.
    pub(crate) fn emit_internal(&self, event: InternalEvent) {
        if let Some(log) = self.internal_log.as_ref() {
            log.write(&event);
        }
        if let Ok(mut events) = self.events.lock() {
            events.push(event);
        }
    }

    /// Build a denial event matching the `internal.capability_denied`
    /// shape in the design doc.
    pub(crate) fn denial_event(
        &self,
        function: &str,
        reason: &str,
        details: Map<String, Value>,
    ) -> InternalEvent {
        let mut ev = InternalEvent::now(INTERNAL_CAPABILITY_DENIED)
            .with_trace_id(&self.dispatcher_trace_id)
            .with_session_id(&self.session_id)
            .with_plugin_name(&self.plugin_name)
            .with_plugin_version(&self.plugin_version)
            .with_field("function", Value::String(function.to_string()))
            .with_field("reason", Value::String(reason.to_string()));
        for (k, v) in details {
            ev = ev.with_field(&k, v);
        }
        ev
    }
}

#[derive(Debug, Error)]
pub enum HostCallError {
    #[error("plugin did not export its memory")]
    MissingMemory,
    #[error("out-of-bounds guest memory access: ptr={ptr}, len={len}, memory_size={memory_size}")]
    OutOfBounds {
        ptr: u32,
        len: u32,
        memory_size: usize,
    },
    #[error("guest bytes were not valid UTF-8")]
    InvalidUtf8,
    #[error("guest memory access overflow: ptr + len would wrap")]
    MemoryOverflow,
    #[error("wasmtime linker error: {0}")]
    Linker(String),
}

/// Register every `vsdd::*` host import with a fresh linker.
///
/// Call once per wasmtime [`Engine`] during dispatcher startup; reuse
/// the returned linker across plugin instantiations.
pub fn setup_linker(engine: &Engine) -> Result<Linker<HostContext>, HostCallError> {
    let mut linker: Linker<HostContext> = Linker::new(engine);

    log::register(&mut linker)?;
    emit_event::register(&mut linker)?;
    read_file::register(&mut linker)?;
    exec_subprocess::register(&mut linker)?;
    env::register(&mut linker)?;
    context_fns::register(&mut linker)?;

    Ok(linker)
}

/// Negative error codes the host returns for bounded calls. Kept in
/// lock-step with `vsdd_hook_sdk::host::HostError::from_code`.
pub mod codes {
    pub const OK: i32 = 0;
    pub const CAPABILITY_DENIED: i32 = -1;
    pub const TIMEOUT: i32 = -2;
    pub const OUTPUT_TOO_LARGE: i32 = -3;
    pub const INVALID_ARGUMENT: i32 = -4;
    pub const INTERNAL_ERROR: i32 = -99;
}

/// Type alias used by every host function entry to keep trait bounds
/// readable.
pub type HostCaller<'a> = Caller<'a, HostContext>;

/// Test helpers shared across host-function unit tests.
#[cfg(test)]
pub(crate) mod test_support {
    use super::*;
    use crate::registry::{Capabilities, ExecSubprocessCaps, ReadFileCaps};

    pub fn bare_context() -> HostContext {
        HostContext::new("p", "0.0.1", "sess", "trace")
    }

    pub fn context_with_caps(caps: Capabilities) -> HostContext {
        let mut ctx = bare_context();
        ctx.capabilities = caps;
        ctx
    }

    pub fn allow_exec(bins: &[&str]) -> Capabilities {
        Capabilities {
            exec_subprocess: Some(ExecSubprocessCaps {
                binary_allow: bins.iter().map(|s| s.to_string()).collect(),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    pub fn allow_read(paths: &[&str]) -> Capabilities {
        Capabilities {
            read_file: Some(ReadFileCaps {
                path_allow: paths.iter().map(|s| s.to_string()).collect(),
            }),
            ..Default::default()
        }
    }
}
