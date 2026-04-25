//! Sink driver contract shared by every observability sink (S-1.8).
//!
//! This crate defines the trait and config types that every sink driver
//! (`sink-file`, and the HTTP / OTel / Datadog / Honeycomb drivers landing
//! in S-1.9 and S-4.x) implements. It intentionally has no cyclic
//! dependency on `factory-dispatcher`: the dispatcher's `InternalEvent`
//! will grow a `From<InternalEvent> for SinkEvent` conversion in the
//! integration story that wires this pipeline up — see the TODO in
//! `factory-dispatcher::sinks::mod.rs`. Keeping the event shape owned
//! by `sink-core` means drivers only need this crate, not the
//! dispatcher's internal plumbing.
//!
//! Design choices:
//!
//! - [`SinkEvent`] is a field bag (`serde_json::Map<String, Value>`)
//!   rather than a named struct so reserved fields (`type`, `ts`,
//!   `ts_epoch`, `schema_version`) and event-specific extras coexist in
//!   a single flat JSON object — exactly the shape
//!   `InternalEvent` already serializes on the wire.
//! - Trait methods are synchronous at the call site. `submit` MUST be
//!   non-blocking (the dispatcher runs it from synchronous code on the
//!   hook hot path); driver implementations push onto an internal queue
//!   and do the actual I/O on a background worker.
//! - [`RoutingFilter`] is allow-list-first then deny-list — the most
//!   common operator need ("only send these event types to this sink")
//!   is the default, and deny is a fallback for broader filters.

#![deny(missing_docs)]

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::BTreeMap;
use thiserror::Error;

/// A single event flowing through the sink pipeline.
///
/// Producers (the dispatcher's `emit_event` host call) populate the
/// reserved top-level fields — `type`, `ts`, `ts_epoch`,
/// `schema_version` — plus any event-specific extras. Sinks treat the
/// whole payload as opaque JSON; only routing inspects `type`.
///
/// The struct is intentionally a thin wrapper around a
/// `serde_json::Map` so it serializes flat, matching the
/// `InternalEvent` shape already written by
/// `crates/factory-dispatcher/src/internal_log.rs`. Once the
/// dispatcher-side integration lands, `InternalEvent` will gain a
/// `From<InternalEvent> for SinkEvent` conversion that simply
/// flattens the struct into this map.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct SinkEvent {
    /// The flat field bag. Reserved names: `type`, `ts`, `ts_epoch`,
    /// `schema_version`. Drivers MAY inspect additional fields for
    /// routing / batching keys but MUST NOT mutate them before the
    /// write.
    pub fields: Map<String, Value>,
}

impl SinkEvent {
    /// Construct an empty event. Producers chain [`Self::insert`] to
    /// build the shape up.
    pub fn new() -> Self {
        Self { fields: Map::new() }
    }

    /// Wrap an existing map. Used by the dispatcher when converting
    /// from `InternalEvent` (the pending integration).
    pub fn from_map(fields: Map<String, Value>) -> Self {
        Self { fields }
    }

    /// Read the `type` reserved field as a `&str`.
    ///
    /// Returns `None` for events that haven't had `type` populated yet
    /// (producer bug) or where the value isn't a string (drivers may
    /// treat this as a hard drop — routing can't be applied).
    pub fn event_type(&self) -> Option<&str> {
        self.fields.get("type").and_then(Value::as_str)
    }

    /// Insert a field, overwriting on collision. Chainable for builder
    /// ergonomics from tests.
    #[must_use]
    pub fn insert(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.fields.insert(key.into(), value.into());
        self
    }
}

/// Per-sink configuration shared by every driver.
///
/// Driver-specific config (e.g. `FileSinkConfig`'s `path_template`) is
/// defined in the driver crate and composes this struct.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SinkConfigCommon {
    /// Operator-assigned name; must be unique within the config file.
    /// Used for `internal.sink_*` event correlation and file-name
    /// template substitution (`{name}`).
    pub name: String,

    /// When false, the sink is constructed but never receives events.
    /// Used to keep a misbehaving sink in the config while an operator
    /// debugs (`enabled = false` rather than deleting the stanza).
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Optional routing filter; when `None`, the sink accepts every
    /// event. See [`RoutingFilter`] for semantics.
    #[serde(default)]
    pub routing_filter: Option<RoutingFilter>,

    /// Static tags enriched onto every event before the driver writes
    /// it out. Operators use these for per-sink source attribution
    /// (`env:prod`, `team:factory`). Non-colliding keys: producers
    /// still emit `type`/`ts`; tag keys that collide with reserved
    /// names are ignored by drivers.
    #[serde(default)]
    pub tags: BTreeMap<String, String>,
}

fn default_true() -> bool {
    true
}

impl Default for SinkConfigCommon {
    fn default() -> Self {
        Self {
            name: String::new(),
            enabled: true,
            routing_filter: None,
            tags: BTreeMap::new(),
        }
    }
}

/// Allow/deny routing for event `type` values.
///
/// Semantics:
///
/// 1. If `event_types_allow` is non-empty, the event's type MUST be in
///    the allow list; otherwise the event is rejected.
/// 2. If `event_types_deny` contains the event's type, the event is
///    rejected regardless of the allow list.
/// 3. Missing / non-string `type` is rejected when either list is
///    non-empty (drivers can't route what they can't classify).
/// 4. Both lists empty = pass-through (matches the "no routing_filter"
///    case).
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoutingFilter {
    /// When non-empty, only these event types pass.
    #[serde(default)]
    pub event_types_allow: Vec<String>,
    /// Always-blocked event types. Applied after the allow check.
    #[serde(default)]
    pub event_types_deny: Vec<String>,
}

impl RoutingFilter {
    /// Apply the filter to a raw event type name. Returns `true` when
    /// the sink should accept the event.
    pub fn accepts(&self, event_type: &str) -> bool {
        if event_type.is_empty() {
            // Empty type is a producer bug; only pass-through filters
            // accept it (matches "no filter" semantics).
            return self.event_types_allow.is_empty() && self.event_types_deny.is_empty();
        }
        if !self.event_types_allow.is_empty()
            && !self.event_types_allow.iter().any(|t| t == event_type)
        {
            return false;
        }
        if self.event_types_deny.iter().any(|t| t == event_type) {
            return false;
        }
        true
    }
}

/// The contract every sink driver implements.
///
/// `submit` is the hot-path entry: it MUST be non-blocking because the
/// dispatcher calls it from synchronous code with the hook payload in
/// hand. Drivers push onto an internal bounded queue and do the I/O on
/// a background worker.
///
/// `flush` is called at tier boundaries (per
/// `.factory/specs/2026-04-24-v1.0-factory-plugin-kit-design.md`). It
/// MAY block, but should respect a reasonable timeout so a stuck sink
/// doesn't stall the dispatcher.
///
/// `shutdown` drains the queue and releases I/O resources. After
/// shutdown, `submit` is a no-op.
pub trait Sink: Send + Sync {
    /// The operator-assigned sink name (also used in `internal.sink_*`
    /// event correlation).
    fn name(&self) -> &str;

    /// Does this sink want this event? Called synchronously from the
    /// producer thread — MUST NOT block on I/O.
    fn accepts(&self, event: &SinkEvent) -> bool;

    /// Non-blocking enqueue. Overflow behavior is driver-specific but
    /// MUST NOT block the caller.
    fn submit(&self, event: SinkEvent);

    /// Drain the queue and wait for in-flight writes to complete.
    fn flush(&self) -> anyhow::Result<()>;

    /// Close the driver; drain remaining events; release resources.
    fn shutdown(&self);
}

/// Errors a sink driver can surface (outside of the infallible
/// hot-path `submit`). Reserved for `flush` / `shutdown` and for the
/// driver's own integration with `internal.sink_error` reporting.
#[derive(Debug, Error)]
pub enum SinkError {
    /// Underlying I/O error; typically wraps `std::io::Error` from the
    /// driver's write path.
    #[error("sink I/O error: {0}")]
    Io(String),

    /// `serde_json::to_string` failed on an event. Should be unreachable
    /// given the `Map<String, Value>` shape, but surfaced for
    /// defense-in-depth.
    #[error("sink serialization error: {0}")]
    SerializeFailed(String),

    /// The driver dropped an event because its internal queue was
    /// full. The driver's own metric counter increments separately.
    #[error("sink backpressure drop on '{sink_name}'")]
    BackpressureDrop {
        /// Name of the sink that dropped the event.
        sink_name: String,
    },

    /// Submit / flush called after `shutdown`.
    #[error("sink '{sink_name}' is shut down")]
    Shutdown {
        /// Name of the sink that rejected the call.
        sink_name: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routing_filter_default_accepts_everything() {
        let f = RoutingFilter::default();
        assert!(f.accepts("plugin.invoked"));
        assert!(f.accepts("internal.sink_error"));
        assert!(f.accepts("commit.made"));
    }

    #[test]
    fn routing_filter_allow_list_only_accepts_listed() {
        let f = RoutingFilter {
            event_types_allow: vec!["commit.made".into(), "pr.merged".into()],
            event_types_deny: vec![],
        };
        assert!(f.accepts("commit.made"));
        assert!(f.accepts("pr.merged"));
        assert!(!f.accepts("plugin.invoked"));
        assert!(!f.accepts("internal.sink_error"));
    }

    #[test]
    fn routing_filter_deny_list_only_rejects_listed() {
        let f = RoutingFilter {
            event_types_allow: vec![],
            event_types_deny: vec!["internal.sink_error".into()],
        };
        assert!(f.accepts("plugin.invoked"));
        assert!(f.accepts("commit.made"));
        assert!(!f.accepts("internal.sink_error"));
    }

    #[test]
    fn routing_filter_both_lists_allow_first_then_deny() {
        let f = RoutingFilter {
            event_types_allow: vec!["plugin.invoked".into(), "plugin.completed".into()],
            event_types_deny: vec!["plugin.completed".into()],
        };
        // In allow + not denied.
        assert!(f.accepts("plugin.invoked"));
        // In allow but also denied — deny wins.
        assert!(!f.accepts("plugin.completed"));
        // Not in allow at all.
        assert!(!f.accepts("commit.made"));
    }

    #[test]
    fn routing_filter_empty_event_type_rejected_when_filtered() {
        let pass = RoutingFilter::default();
        assert!(pass.accepts(""), "no-filter accepts empty");

        let allow_only = RoutingFilter {
            event_types_allow: vec!["commit.made".into()],
            event_types_deny: vec![],
        };
        assert!(!allow_only.accepts(""));

        let deny_only = RoutingFilter {
            event_types_allow: vec![],
            event_types_deny: vec!["commit.made".into()],
        };
        assert!(!deny_only.accepts(""));
    }

    #[test]
    fn routing_filter_allow_case_sensitive() {
        // Event type names are case-sensitive by contract (lowercase
        // with dot-separated namespaces per the spec's catalog).
        let f = RoutingFilter {
            event_types_allow: vec!["Commit.Made".into()],
            event_types_deny: vec![],
        };
        assert!(!f.accepts("commit.made"));
        assert!(f.accepts("Commit.Made"));
    }

    #[test]
    fn sink_event_event_type_accessor_reads_type_field() {
        let ev = SinkEvent::new().insert("type", "commit.made");
        assert_eq!(ev.event_type(), Some("commit.made"));
    }

    #[test]
    fn sink_event_event_type_missing_returns_none() {
        let ev = SinkEvent::new();
        assert_eq!(ev.event_type(), None);
    }

    #[test]
    fn sink_event_event_type_non_string_returns_none() {
        let ev = SinkEvent::new().insert("type", serde_json::json!(42));
        assert_eq!(ev.event_type(), None);
    }

    #[test]
    fn sink_event_serializes_as_flat_object() {
        let ev = SinkEvent::new()
            .insert("type", "plugin.invoked")
            .insert("plugin_name", "capture-commit-activity");
        let s = serde_json::to_string(&ev).unwrap();
        // No `fields` wrapper — transparent serde makes the map the
        // top-level object.
        let parsed: Value = serde_json::from_str(&s).unwrap();
        assert!(parsed.get("fields").is_none());
        assert_eq!(parsed["type"], "plugin.invoked");
        assert_eq!(parsed["plugin_name"], "capture-commit-activity");
    }

    #[test]
    fn sink_config_common_defaults_enabled_true() {
        let toml_src = r#"
            name = "local-events"
        "#;
        let cfg: SinkConfigCommon = toml::from_str(toml_src).unwrap();
        assert_eq!(cfg.name, "local-events");
        assert!(cfg.enabled);
        assert!(cfg.routing_filter.is_none());
        assert!(cfg.tags.is_empty());
    }
}
