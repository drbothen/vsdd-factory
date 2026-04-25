//! `factory_dispatcher` — library surface for the vsdd-factory v1.0
//! dispatcher. The binary at [`main.rs`](src/main.rs) is a thin
//! orchestrator over these modules; integration tests consume the
//! library directly.
//!
//! S-1.2 owns the I/O boundary, registry loading, and routing
//! decisions. Execution (wasmtime instantiation, tokio scheduling,
//! fuel enforcement) is filled in by S-1.4–S-1.6.

pub mod payload;
pub mod registry;
pub mod routing;

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
