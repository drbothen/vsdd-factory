//! `factory_dispatcher` — library surface for the vsdd-factory v1.0
//! dispatcher. The binary at [`main.rs`](src/main.rs) is a thin
//! orchestrator over these modules; integration tests consume the
//! library directly.
//!
//! S-1.2 owns the I/O boundary, registry loading, and routing
//! decisions. Execution (wasmtime instantiation, tokio scheduling,
//! fuel enforcement) is filled in by S-1.4–S-1.6.

pub mod host;
pub mod internal_log;
pub mod payload;
pub mod registry;
pub mod routing;
pub mod sinks;

pub use host::{HostCallError, HostContext, setup_linker};
pub use internal_log::{
    DEFAULT_RETENTION_DAYS, DISPATCHER_SHUTTING_DOWN, DISPATCHER_STARTED,
    INTERNAL_CAPABILITY_DENIED, INTERNAL_DISPATCHER_ERROR, INTERNAL_EVENT_SCHEMA_VERSION,
    INTERNAL_HOST_FUNCTION_PANIC, INTERNAL_SINK_CIRCUIT_CLOSED, INTERNAL_SINK_CIRCUIT_OPENED,
    INTERNAL_SINK_ERROR, INTERNAL_SINK_QUEUE_FULL, InternalEvent, InternalLog, PLUGIN_COMPLETED,
    PLUGIN_CRASHED, PLUGIN_INVOKED, PLUGIN_LOAD_FAILED, PLUGIN_LOADED, PLUGIN_TIMEOUT,
};
pub use payload::{HookPayload, PayloadError};
pub use registry::{
    Capabilities, ExecSubprocessCaps, OnError, ReadFileCaps, Registry, RegistryDefaults,
    RegistryEntry, RegistryError,
};
pub use routing::{PluginResultStub, group_by_priority, match_plugins};

/// ABI version the dispatcher speaks. Kept in lock-step with
/// `vsdd_hook_sdk::HOST_ABI_VERSION`; diverging is a breaking change.
pub const HOST_ABI_VERSION: u32 = 1;

/// Generate a fresh v4 UUID to use as the per-invocation
/// `dispatcher_trace_id`. Extracted so tests can substitute a fixed
/// value.
pub fn new_trace_id() -> String {
    uuid::Uuid::new_v4().to_string()
}
