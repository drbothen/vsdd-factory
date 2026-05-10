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
pub mod write_file;

/// Per-invocation state available to every host function. Lives in the
/// wasmtime [`Store`] for the plugin call and is torn down when the
/// call returns.
///
/// `Clone` — cloning is how the per-tier executor (S-1.6) hands each
/// plugin its own context. Per-plugin fields (`plugin_name`,
/// `plugin_version`, `capabilities`, `cwd`) are owned; the event queue
/// and internal log are `Arc`-shared so every clone feeds the same
/// downstream sink pipeline.
#[derive(Clone)]
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

/// Build a `Linker<HostContext>` configured for WASM resolver modules.
///
/// Resolver modules use a subset of the full plugin host ABI: they may
/// not exec subprocesses or write files. This linker wires only the
/// context-injection imports declared by `HOST_ABI.md §Resolver ABI`.
///
/// Non-trivial: registers host functions, applies capability restrictions,
/// and returns the configured linker. S-12.04 Step 3 implementation.
pub fn resolver_linker(_engine: &Engine) -> Linker<HostContext> {
    todo!("S-12.04 Step 3")
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
    write_file::register(&mut linker)?;
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

// ---------------------------------------------------------------------------
// AC-008 / AC-009 unit tests — resolver linker capability enforcement
//
// BC-4.12.003 postconditions 1–3:
// The resolver-specific Linker<HostContext> must NOT expose write_file,
// exec_subprocess, or emit_event host imports. A resolver WASM that references
// any of these must fail at instantiation (linker error), not at runtime.
//
// BC-4.12.003 postcondition 2: the resolver linker MUST expose read_file
// (with path_allow enforcement) and log.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod linker_capability_tests {
    use super::*;
    use crate::engine::build_engine;

    /// Helper: check whether `linker` has a definition for `(module_name, fn_name)`.
    ///
    /// Uses a `Store<HostContext>` so `linker.get(store, ...)` compiles.
    /// Returns `true` if defined, `false` if missing.
    fn linker_has_fn(
        linker: &Linker<HostContext>,
        engine: &Engine,
        module_name: &str,
        fn_name: &str,
    ) -> bool {
        let mut store: wasmtime::Store<HostContext> =
            wasmtime::Store::new(engine, HostContext::new("test", "0.0.0", "s", "t"));
        linker.get(&mut store, module_name, fn_name).is_ok()
    }

    /// test_BC_4_12_003_resolver_linker_excludes_write_file_exec_emit
    ///
    /// Constructs the resolver linker via `resolver_linker(engine)` and asserts
    /// that the following host imports are NOT defined:
    /// - "vsdd::write_file"
    /// - "vsdd::exec_subprocess"
    /// - "vsdd::emit_event"
    ///
    /// These functions are in the full hook linker (setup_linker) but must be
    /// absent from the resolver linker (BC-4.12.003 INV2: resolver sandbox
    /// excludes write, exec, and emit capabilities).
    ///
    /// Also asserts that the following ARE defined:
    /// - "vsdd::read_file"
    /// - "vsdd::log"
    ///
    /// Red Gate: fails because `resolver_linker` is `todo!("S-12.04 Step 3")`.
    #[test]
    fn test_BC_4_12_003_resolver_linker_excludes_write_file_exec_emit() {
        let engine = build_engine()
            .expect("AC-008: build_engine must succeed for resolver linker capability test");

        // This call panics at todo!() before Step 3 — Red Gate.
        let linker = resolver_linker(&engine);

        // Functions that must be ABSENT from the resolver linker.
        assert!(
            !linker_has_fn(&linker, &engine, "vsdd", "write_file"),
            "AC-008 / BC-4.12.003 INV2: resolver linker must NOT expose 'write_file' — \
             resolvers have no write capability"
        );

        assert!(
            !linker_has_fn(&linker, &engine, "vsdd", "exec_subprocess"),
            "AC-008 / BC-4.12.003 INV2: resolver linker must NOT expose 'exec_subprocess' — \
             resolvers have no exec capability"
        );

        assert!(
            !linker_has_fn(&linker, &engine, "vsdd", "emit_event"),
            "AC-008 / BC-4.12.003 INV2: resolver linker must NOT expose 'emit_event' — \
             resolvers must not emit side-channel events"
        );

        // Functions that must be PRESENT in the resolver linker.
        // read_file (with path_allow enforcement) and log are the only allowed imports.
        assert!(
            linker_has_fn(&linker, &engine, "vsdd", "read_file"),
            "AC-009 / BC-4.12.003 PC2: resolver linker MUST expose 'read_file' \
             (subject to path_allow enforcement) — resolvers need read access \
             for their declared capabilities"
        );

        assert!(
            linker_has_fn(&linker, &engine, "vsdd", "log"),
            "AC-009 / BC-4.12.003 PC2: resolver linker MUST expose 'log' — \
             resolvers must be able to emit diagnostic log entries"
        );
    }
}

/// Test helpers shared across host-function unit tests.
#[cfg(test)]
pub(crate) mod test_support {
    use super::*;
    use crate::registry::{Capabilities, ExecSubprocessCaps, ReadFileCaps, WriteFileCaps};

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

    /// Build a `Capabilities` with only the `write_file` block populated.
    /// Parallel to `allow_read` (BC-2.02.011 test support, AC-5 / T-3a).
    pub fn allow_write(paths: &[&str]) -> Capabilities {
        Capabilities {
            write_file: Some(WriteFileCaps {
                path_allow: paths.iter().map(|s| s.to_string()).collect(),
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}
