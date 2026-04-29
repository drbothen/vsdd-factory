//! Router dispatch layer — per-sink routing filters + tag enrichment (S-4.06).
//!
//! The Router is the single dispatch gate; it applies each sink's
//! [`RoutingFilter`] and tag enrichment before delegating to individual
//! sinks. S-4.06 wires the filter evaluation and enrichment that were
//! previously stubs.
//!
//! Purity classification: **effectful-shell** — Router orchestrates
//! SinkRegistry (which holds boxed Sink trait objects performing I/O);
//! pure filter matching is delegated to `sink-core/src/router_filter.rs`.

use sink_core::SinkEvent;
use tracing::debug;

use super::SinkRegistry;
use crate::internal_log::INTERNAL_EVENT_FILTERED;

/// Stable extension point for pre-submit event processing. Holds a
/// registry and forwards to it after applying per-sink routing filters
/// and tag enrichment (BC-3.04.004).
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

    /// Fan an event out to every accepting sink, applying per-sink
    /// routing filters and tag enrichment before each `submit` call.
    ///
    /// Per BC-3.04.004:
    /// - For each sink: if the sink's RoutingFilter is non-empty and
    ///   the event fails it, that sink's `submit()` is NOT called.
    /// - Tag enrichment from sink `tags` config is applied before
    ///   `submit()` (non-overwrite semantics: producer fields win).
    ///
    /// Per BC-3.04.003:
    /// - When an event fails a sink's filter, no SinkFailure is
    ///   recorded; a debug-level `internal.event_filtered` log entry
    ///   is emitted (one per filtering sink).
    pub fn submit(&self, event: SinkEvent) {
        for sink in self.registry.sinks() {
            // Step 1: Apply per-sink routing filter (BC-3.04.004 PC1).
            if let Some(filter) = sink.routing_filter() {
                if !filter.accepts(&event) {
                    // BC-3.04.003 PC2+PC3: silent drop — no SinkFailure;
                    // emit debug-level INTERNAL_EVENT_FILTERED log entry.
                    debug!(
                        type_ = INTERNAL_EVENT_FILTERED,
                        sink_name = %sink.name(),
                        event_type = %event.event_type().unwrap_or("<unknown>"),
                        "Router silently dropped event that failed routing filter"
                    );
                    continue;
                }
            }

            // Step 2: Tag enrichment (BC-3.04.004 PC3+PC4).
            // Non-overwrite: producer fields win on key collision.
            let enriched = {
                let sink_tags = sink.tags();
                if sink_tags.is_empty() {
                    event.clone()
                } else {
                    let mut ev = event.clone();
                    for (k, v) in sink_tags {
                        if !ev.fields.contains_key(k) {
                            ev.fields
                                .insert(k.clone(), serde_json::Value::String(v.clone()));
                        }
                    }
                    ev
                }
            };

            // Step 3: Delegate to sink (BC-3.04.004 PC2).
            sink.submit(enriched);
        }
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
    use sink_core::{RoutingFilter, Sink, SinkEvent};
    use std::sync::{Arc, Mutex};

    // ── SpySink: mock sink for Router-layer behavioral verification ───────────
    //
    // SpySink's `accepts()` always returns `true` — it never applies the
    // routing filter itself. This simulates the post-Task-16 state where
    // FileSink's `accepts()` no longer performs RoutingFilter evaluation
    // (that responsibility moves to Router::submit). The SpySink records
    // events that actually reach `submit()` so tests can assert the Router
    // did or did not forward them.
    //
    // The `routing_filter` field is readable by the Router via
    // `routing_filter()` — which is the mechanism BC-3.04.004 relies on.

    struct SpySink {
        name: String,
        /// Events that reached submit() — the Router-layer dispatch gate
        /// decides whether to call submit() or silently drop the event.
        submitted: Arc<Mutex<Vec<SinkEvent>>>,
        /// Filter that the Router should read via routing_filter().
        filter: Option<RoutingFilter>,
        /// Tags that the Router should enrich events with.
        tags: std::collections::BTreeMap<String, String>,
    }

    impl SpySink {
        fn new(
            name: &str,
            filter: Option<RoutingFilter>,
            tags: std::collections::BTreeMap<String, String>,
        ) -> (Self, Arc<Mutex<Vec<SinkEvent>>>) {
            let submitted = Arc::new(Mutex::new(Vec::new()));
            let spy = Self {
                name: name.to_string(),
                submitted: Arc::clone(&submitted),
                filter,
                tags,
            };
            (spy, submitted)
        }
    }

    impl Sink for SpySink {
        fn name(&self) -> &str {
            &self.name
        }

        /// Always accepts — does NOT apply routing filter itself.
        /// Routing filter evaluation is the Router's responsibility (BC-3.04.004).
        fn accepts(&self, _event: &SinkEvent) -> bool {
            true
        }

        /// Returns the configured routing filter for Router-layer inspection.
        fn routing_filter(&self) -> Option<&RoutingFilter> {
            self.filter.as_ref()
        }

        /// Returns the configured tags for Router-layer enrichment.
        fn tags(&self) -> &std::collections::BTreeMap<String, String> {
            &self.tags
        }

        fn submit(&self, event: SinkEvent) {
            self.submitted.lock().unwrap().push(event);
        }

        fn flush(&self) -> anyhow::Result<()> {
            Ok(())
        }

        fn shutdown(&self) {}
    }

    // ── BC-3.04.004 PC1: Router filters events that fail allow-list ──────────

    /// AC#1 — BC-3.04.004 PC1: Router::submit reads each sink's routing_filter()
    /// and does NOT call sink.submit() when the event fails the filter.
    ///
    /// RED gate: Router currently delegates to SinkRegistry::submit_all without
    /// reading routing_filter(). The SpySink's accepts() always returns true,
    /// so the only filter gate is Router-layer. After S-4.06 wires the Router,
    /// 'plugin.invoked' must NOT reach the spy's submit().
    #[test]
    fn test_BC_3_04_004_routing_filter_applied_in_dispatch_path() {
        let (spy, submitted) = SpySink::new(
            "Sink-A",
            Some(RoutingFilter {
                event_types_allow: vec!["commit.made".into()],
                event_types_deny: vec![],
                plugin_ids_allow: vec![],
            }),
            Default::default(),
        );

        let registry = SinkRegistry::with_sinks(vec![Box::new(spy)]);
        let router = Router::new(registry);

        // Submit event that does NOT match Sink-A's allow filter.
        router.submit(SinkEvent::new().insert("type", "plugin.invoked"));
        router.flush().unwrap();
        router.shutdown();

        let events = submitted.lock().unwrap();
        assert_eq!(
            events.len(),
            0,
            "BC-3.04.004 PC1: Router must NOT call sink.submit() when event fails filter; \
             got {} events: {:?}",
            events.len(),
            events.iter().map(|e| e.event_type()).collect::<Vec<_>>()
        );
    }

    /// AC#1 — BC-3.04.004 PC2: Router::submit DOES call sink.submit() when
    /// the event passes the filter.
    #[test]
    fn test_BC_3_04_004_accepted_event_reaches_sink() {
        let (spy, submitted) = SpySink::new(
            "Sink-A",
            Some(RoutingFilter {
                event_types_allow: vec!["commit.made".into()],
                event_types_deny: vec![],
                plugin_ids_allow: vec![],
            }),
            Default::default(),
        );

        let registry = SinkRegistry::with_sinks(vec![Box::new(spy)]);
        let router = Router::new(registry);

        router.submit(SinkEvent::new().insert("type", "commit.made"));
        router.flush().unwrap();
        router.shutdown();

        let events = submitted.lock().unwrap();
        assert_eq!(
            events.len(),
            1,
            "BC-3.04.004 PC2: Router must forward event to sink when filter passes; \
             got {} events",
            events.len()
        );
        assert_eq!(
            events[0].event_type(),
            Some("commit.made"),
            "forwarded event must have correct type"
        );
    }

    // ── BC-3.04.004 PC3: static tags merged before submit ────────────────────

    /// AC#2 — BC-3.04.004 PC3: Router applies tag enrichment from sink `tags`
    /// config before calling sink.submit(). The enriched event (with tags
    /// merged) must be what actually arrives at submit().
    ///
    /// RED gate: Router currently passes events through unenriched. After
    /// S-4.06, Router reads each sink's tags config and enriches before submit.
    #[test]
    fn test_BC_3_04_004_static_tags_merged_before_submit() {
        let mut tags = std::collections::BTreeMap::new();
        tags.insert("env".to_string(), "prod".to_string());
        tags.insert("team".to_string(), "factory".to_string());

        let (spy, submitted) = SpySink::new("tagged-sink", None, tags);
        let registry = SinkRegistry::with_sinks(vec![Box::new(spy)]);
        let router = Router::new(registry);

        router.submit(SinkEvent::new().insert("type", "commit.made"));
        router.flush().unwrap();
        router.shutdown();

        let events = submitted.lock().unwrap();
        assert_eq!(
            events.len(),
            1,
            "event must reach sink; got {} events",
            events.len()
        );

        let ev = &events[0];
        assert_eq!(
            ev.fields.get("env").and_then(|v| v.as_str()),
            Some("prod"),
            "BC-3.04.004 PC3: tag 'env' must be in event before submit; event: {:?}",
            ev.fields
        );
        assert_eq!(
            ev.fields.get("team").and_then(|v| v.as_str()),
            Some("factory"),
            "BC-3.04.004 PC3: tag 'team' must be in event before submit; event: {:?}",
            ev.fields
        );
    }

    // ── BC-3.04.004 PC4: tag enrichment does not overwrite producer fields ────

    /// AC#3 — BC-3.04.004 PC4: Tag enrichment MUST NOT overwrite
    /// producer-set fields. Producer fields win on key collision.
    ///
    /// RED gate: Router currently forwards events unenriched, so this test
    /// would pass vacuously (no enrichment = no overwrite). After S-4.06,
    /// Router enriches events but must not overwrite existing fields.
    ///
    /// Note: this test is written as a PASS-after-impl test (non-overwrite
    /// invariant). But it verifies the enrichment path exists AND is safe.
    /// If Router enriches naively (always overwriting), test FAILS.
    #[test]
    fn test_BC_3_04_004_tag_enrichment_does_not_overwrite_producer_fields() {
        let mut tags = std::collections::BTreeMap::new();
        // Config tag "type" must NOT overwrite producer's "type" field.
        tags.insert("type".to_string(), "SHOULD_NOT_WIN".to_string());
        tags.insert("extra_tag".to_string(), "extra_value".to_string());

        let (spy, submitted) = SpySink::new("nooverwrite-sink", None, tags);
        let registry = SinkRegistry::with_sinks(vec![Box::new(spy)]);
        let router = Router::new(registry);

        // Producer sets type="commit.made".
        router.submit(SinkEvent::new().insert("type", "commit.made"));
        router.flush().unwrap();
        router.shutdown();

        let events = submitted.lock().unwrap();
        // After Router enrichment is wired: event must have both fields,
        // with producer's "type" winning.
        assert_eq!(events.len(), 1, "event must reach sink");
        let ev = &events[0];

        // Producer field wins over config tag collision.
        assert_eq!(
            ev.fields.get("type").and_then(|v| v.as_str()),
            Some("commit.made"),
            "BC-3.04.004 PC4: producer 'type' must not be overwritten by config tag; event: {:?}",
            ev.fields
        );
        // Non-colliding config tag must be present (enrichment happened).
        assert_eq!(
            ev.fields.get("extra_tag").and_then(|v| v.as_str()),
            Some("extra_value"),
            "BC-3.04.004 PC4: non-colliding config tag must be added; event: {:?}",
            ev.fields
        );
    }

    // ── BC-3.06.001: default empty filter passes all events ──────────────────

    /// AC#8 — BC-3.06.001 + BC-3.01.004: Sink with no RoutingFilter = all
    /// events pass through.
    #[test]
    fn test_BC_3_06_001_default_empty_filter_passes_all_events_through_router() {
        let (spy, submitted) = SpySink::new("passthrough", None, Default::default());
        let registry = SinkRegistry::with_sinks(vec![Box::new(spy)]);
        let router = Router::new(registry);

        router.submit(SinkEvent::new().insert("type", "plugin.invoked"));
        router.submit(SinkEvent::new().insert("type", "commit.made"));
        router.submit(SinkEvent::new().insert("type", "internal.sink_error"));
        router.flush().unwrap();
        router.shutdown();

        let events = submitted.lock().unwrap();
        assert_eq!(
            events.len(),
            3,
            "BC-3.06.001: all 3 events must pass through default filter; got {}",
            events.len()
        );
    }

    // ── BC-3.06.006: case-sensitive matching in allow-list ───────────────────

    /// AC#6 — BC-3.06.006: 'Commit.Made' allow-list does NOT match
    /// 'commit.made' (case-sensitive).
    #[test]
    fn test_BC_3_06_006_allow_list_case_sensitive_in_router_dispatch() {
        let (spy, submitted) = SpySink::new(
            "case-sensitive",
            Some(RoutingFilter {
                event_types_allow: vec!["Commit.Made".into()],
                event_types_deny: vec![],
                plugin_ids_allow: vec![],
            }),
            Default::default(),
        );

        let registry = SinkRegistry::with_sinks(vec![Box::new(spy)]);
        let router = Router::new(registry);

        // lowercase 'commit.made' must NOT pass the uppercase allow-list.
        router.submit(SinkEvent::new().insert("type", "commit.made"));
        // 'Commit.Made' (exact case) must pass.
        router.submit(SinkEvent::new().insert("type", "Commit.Made"));

        router.flush().unwrap();
        router.shutdown();

        let events = submitted.lock().unwrap();
        assert_eq!(
            events.len(),
            1,
            "BC-3.06.006: only 'Commit.Made' (exact case) must pass; got {} events",
            events.len()
        );
        assert_eq!(
            events[0].event_type(),
            Some("Commit.Made"),
            "surviving event must be 'Commit.Made'"
        );
    }

    // ── BC-3.06.007: plugin_ids_allow field ──────────────────────────────────

    /// AC#5 — BC-3.06.007: Router reads plugin_ids_allow from routing_filter()
    /// and filters events from non-listed plugins.
    #[test]
    fn test_BC_3_06_007_plugin_ids_allow_filters_non_listed_plugins() {
        let (spy, submitted) = SpySink::new(
            "plugin-filter",
            Some(RoutingFilter {
                event_types_allow: vec![],
                event_types_deny: vec![],
                plugin_ids_allow: vec!["capture-commit-activity".into()],
            }),
            Default::default(),
        );

        let registry = SinkRegistry::with_sinks(vec![Box::new(spy)]);
        let router = Router::new(registry);

        // Listed plugin — must pass.
        router.submit(
            SinkEvent::new()
                .insert("type", "plugin.invoked")
                .insert("plugin_id", "capture-commit-activity"),
        );
        // Unlisted plugin — must be filtered.
        router.submit(
            SinkEvent::new()
                .insert("type", "plugin.invoked")
                .insert("plugin_id", "other-plugin"),
        );
        // No plugin_id (BC-3.06.007 EC-001) — must be filtered.
        router.submit(SinkEvent::new().insert("type", "commit.made"));

        router.flush().unwrap();
        router.shutdown();

        let events = submitted.lock().unwrap();
        assert_eq!(
            events.len(),
            1,
            "BC-3.06.007: only the event from 'capture-commit-activity' must pass; got {} events",
            events.len()
        );
        assert_eq!(
            events[0].fields.get("plugin_id").and_then(|v| v.as_str()),
            Some("capture-commit-activity"),
            "surviving event must be from 'capture-commit-activity'"
        );
    }

    // ── BC-3.04.003: silent-drop — no SinkFailure + INTERNAL_EVENT_FILTERED ──

    /// AC#9 — BC-3.04.003 PC1+PC2: Router silently drops filtered events
    /// (no submit() call). Sink-A has allow=['commit.made'].
    /// 'plugin.invoked' must not reach submit().
    #[test]
    fn test_BC_3_04_003_silent_drop_no_sink_failure_on_filtered_event() {
        let (spy, submitted) = SpySink::new(
            "Sink-A",
            Some(RoutingFilter {
                event_types_allow: vec!["commit.made".into()],
                event_types_deny: vec![],
                plugin_ids_allow: vec![],
            }),
            Default::default(),
        );

        let registry = SinkRegistry::with_sinks(vec![Box::new(spy)]);
        let router = Router::new(registry);

        // 'plugin.invoked' should be silently dropped — no submit() call.
        router.submit(SinkEvent::new().insert("type", "plugin.invoked"));
        router.flush().unwrap();
        router.shutdown();

        let events = submitted.lock().unwrap();
        assert_eq!(
            events.len(),
            0,
            "BC-3.04.003 PC1+PC2: silently dropped event must NOT reach submit(); \
             got {} events",
            events.len()
        );
    }

    /// AC#9 — BC-3.04.003 PC3: INTERNAL_EVENT_FILTERED constant must exist
    /// with correct value. This is a compilation-level and value-level check.
    #[test]
    fn test_BC_3_04_003_internal_event_filtered_constant_declared() {
        assert_eq!(
            crate::internal_log::INTERNAL_EVENT_FILTERED,
            "internal.event_filtered",
            "BC-3.04.003 PC3: INTERNAL_EVENT_FILTERED must equal 'internal.event_filtered'"
        );
    }

    // ── Smoke test ────────────────────────────────────────────────────────────

    #[test]
    fn router_delegates_to_empty_registry() {
        let router = Router::new(SinkRegistry::empty());
        router.submit(SinkEvent::new().insert("type", "plugin.invoked"));
        router.flush().unwrap();
        router.shutdown();
    }
}
