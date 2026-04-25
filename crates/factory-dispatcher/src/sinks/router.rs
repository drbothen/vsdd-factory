//! Thin wrapper around [`SinkRegistry`] that gives future S-4.x work
//! (routing filters, retry/circuit-breaker, batching) a stable
//! extension point without having to change call sites. Today it just
//! delegates to the registry.
//!
//! The integration story that wires the dispatcher's event loop into
//! the sink fleet calls [`Router::submit`] for every event drained
//! from `HostContext.events`. Currently no call sites exist — see the
//! `TODO(integration)` in `sinks::mod.rs`.

use sink_core::SinkEvent;

use super::SinkRegistry;

/// Stable extension point for pre-submit event processing. Holds a
/// registry and forwards to it; S-4.x will graft retry / circuit-
/// breaker / batching behavior in at this layer without touching the
/// call sites or the driver implementations.
pub struct Router {
    registry: SinkRegistry,
}

impl Router {
    /// Wrap a registry. The caller typically builds the registry via
    /// [`SinkRegistry::load`] or constructs one programmatically for
    /// tests.
    pub fn new(registry: SinkRegistry) -> Self {
        Self { registry }
    }

    /// Fan an event out to every accepting sink. Delegates to
    /// [`SinkRegistry::submit_all`].
    pub fn submit(&self, event: SinkEvent) {
        self.registry.submit_all(event);
    }

    /// Flush every underlying sink. Delegates to
    /// [`SinkRegistry::flush_all`].
    pub fn flush(&self) -> anyhow::Result<()> {
        self.registry.flush_all()
    }

    /// Shut down every underlying sink. Delegates to
    /// [`SinkRegistry::shutdown_all`].
    pub fn shutdown(&self) {
        self.registry.shutdown_all();
    }

    /// Borrow the underlying registry for tests that want to inspect
    /// individual sinks.
    pub fn registry(&self) -> &SinkRegistry {
        &self.registry
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn router_delegates_to_empty_registry() {
        let router = Router::new(SinkRegistry::empty());
        router.submit(SinkEvent::new().insert("type", "plugin.invoked"));
        router.flush().unwrap();
        router.shutdown();
    }
}
