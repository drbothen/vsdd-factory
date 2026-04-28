//! Pure-core routing filter for per-sink event dispatch (S-4.06).
//!
//! This module owns the [`RoutingFilter`] struct and its `accepts`
//! method. Extracted from `sink-core/src/lib.rs` so the filter logic is
//! co-located with its type and cleanly separable from the Sink trait
//! and config definitions.
//!
//! Purity classification: **pure-core** — no I/O; all behaviour
//! expressed as pure functions on immutable data. Fully unit-testable
//! without trait objects or filesystem access.

use serde::{Deserialize, Serialize};

use crate::SinkEvent;

/// Allow/deny routing for sink events.
///
/// Semantics (BC-3.01.004, BC-3.06.001, BC-3.06.006, BC-3.06.007):
///
/// 1. `event_types_allow`: if non-empty the event's `type` MUST be in
///    the list; otherwise the event is rejected (whitelist).
/// 2. `event_types_deny`: applied after allow — if the event's `type`
///    is in the deny list the event is rejected regardless of allow.
/// 3. `plugin_ids_allow`: if non-empty the event's `plugin_id` MUST be
///    in the list; otherwise the event is rejected. Empty list = pass.
/// 4. All configured filter fields must pass simultaneously for the
///    event to be forwarded (independent AND semantics).
/// 5. Both allow/deny empty AND plugin_ids_allow empty = pass-through.
/// 6. All matching is case-sensitive (BC-3.06.006).
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoutingFilter {
    /// When non-empty, only these event types pass.
    #[serde(default)]
    pub event_types_allow: Vec<String>,
    /// Always-blocked event types. Applied after the allow check.
    #[serde(default)]
    pub event_types_deny: Vec<String>,
    /// When non-empty, only events from these plugin IDs pass.
    /// Empty list = all plugins pass (pass-through).
    #[serde(default)]
    pub plugin_ids_allow: Vec<String>,
}

impl RoutingFilter {
    /// Apply the filter to a [`SinkEvent`]. Returns `true` when the
    /// sink should accept the event.
    ///
    /// Reads `event.event_type()` for allow/deny checks and
    /// `event.fields["plugin_id"]` for plugin_ids_allow check.
    pub fn accepts(&self, event: &SinkEvent) -> bool {
        // --- event_types_allow / event_types_deny ---
        let event_type = event.event_type().unwrap_or("");
        if event_type.is_empty() {
            // Empty type is a producer bug; only pass-through filters
            // accept it.
            if !self.event_types_allow.is_empty() || !self.event_types_deny.is_empty() {
                return false;
            }
            // Fall through to plugin_ids_allow check below.
        } else {
            if !self.event_types_allow.is_empty()
                && !self.event_types_allow.iter().any(|t| t == event_type)
            {
                return false;
            }
            if self.event_types_deny.iter().any(|t| t == event_type) {
                return false;
            }
        }

        // --- plugin_ids_allow ---
        if !self.plugin_ids_allow.is_empty() {
            let plugin_id = event
                .fields
                .get("plugin_id")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if !self.plugin_ids_allow.iter().any(|p| p == plugin_id) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SinkEvent;

    // ── BC-3.06.001: default (empty) filter accepts every event ──────────────

    #[test]
    fn routing_filter_default_accepts_everything() {
        let f = RoutingFilter::default();
        let ev = |t: &str| SinkEvent::new().insert("type", t);
        assert!(f.accepts(&ev("plugin.invoked")));
        assert!(f.accepts(&ev("internal.sink_error")));
        assert!(f.accepts(&ev("commit.made")));
    }

    // ── BC-3.01.004: allow-list whitelist; deny after allow ──────────────────

    #[test]
    fn routing_filter_allow_list_only_accepts_listed() {
        let f = RoutingFilter {
            event_types_allow: vec!["commit.made".into(), "pr.merged".into()],
            event_types_deny: vec![],
            plugin_ids_allow: vec![],
        };
        let ev = |t: &str| SinkEvent::new().insert("type", t);
        assert!(f.accepts(&ev("commit.made")));
        assert!(f.accepts(&ev("pr.merged")));
        assert!(!f.accepts(&ev("plugin.invoked")));
        assert!(!f.accepts(&ev("internal.sink_error")));
    }

    #[test]
    fn routing_filter_deny_list_only_rejects_listed() {
        let f = RoutingFilter {
            event_types_allow: vec![],
            event_types_deny: vec!["internal.sink_error".into()],
            plugin_ids_allow: vec![],
        };
        let ev = |t: &str| SinkEvent::new().insert("type", t);
        assert!(f.accepts(&ev("plugin.invoked")));
        assert!(f.accepts(&ev("commit.made")));
        assert!(!f.accepts(&ev("internal.sink_error")));
    }

    #[test]
    fn routing_filter_both_lists_allow_first_then_deny() {
        let f = RoutingFilter {
            event_types_allow: vec!["plugin.invoked".into(), "plugin.completed".into()],
            event_types_deny: vec!["plugin.completed".into()],
            plugin_ids_allow: vec![],
        };
        let ev = |t: &str| SinkEvent::new().insert("type", t);
        assert!(f.accepts(&ev("plugin.invoked")));
        assert!(!f.accepts(&ev("plugin.completed")));
        assert!(!f.accepts(&ev("commit.made")));
    }

    // ── BC-3.06.006: case-sensitive allow-list matching ──────────────────────

    #[test]
    fn routing_filter_allow_case_sensitive() {
        let f = RoutingFilter {
            event_types_allow: vec!["Commit.Made".into()],
            event_types_deny: vec![],
            plugin_ids_allow: vec![],
        };
        let ev = |t: &str| SinkEvent::new().insert("type", t);
        assert!(!f.accepts(&ev("commit.made")), "lowercase must not match uppercase allow");
        assert!(f.accepts(&ev("Commit.Made")), "exact case must match");
    }

    // ── BC-3.06.007: plugin_ids_allow filter ─────────────────────────────────

    #[test]
    fn routing_filter_plugin_ids_allow_passes_listed_plugin() {
        let f = RoutingFilter {
            event_types_allow: vec![],
            event_types_deny: vec![],
            plugin_ids_allow: vec!["my-plugin".into()],
        };
        let ev = SinkEvent::new()
            .insert("type", "plugin.invoked")
            .insert("plugin_id", "my-plugin");
        assert!(f.accepts(&ev));
    }

    #[test]
    fn routing_filter_plugin_ids_allow_rejects_unlisted_plugin() {
        let f = RoutingFilter {
            event_types_allow: vec![],
            event_types_deny: vec![],
            plugin_ids_allow: vec!["my-plugin".into()],
        };
        let ev = SinkEvent::new()
            .insert("type", "plugin.invoked")
            .insert("plugin_id", "other-plugin");
        assert!(!f.accepts(&ev));
    }

    #[test]
    fn routing_filter_plugin_ids_allow_empty_list_is_passthrough() {
        let f = RoutingFilter {
            event_types_allow: vec![],
            event_types_deny: vec![],
            plugin_ids_allow: vec![],
        };
        let ev = SinkEvent::new()
            .insert("type", "plugin.invoked")
            .insert("plugin_id", "any-plugin");
        assert!(f.accepts(&ev));
    }

    #[test]
    fn routing_filter_plugin_ids_allow_rejects_missing_plugin_id() {
        // BC-3.06.007 EC-001: event with no plugin_id field when
        // plugin_ids_allow is non-empty → filtered out.
        let f = RoutingFilter {
            event_types_allow: vec![],
            event_types_deny: vec![],
            plugin_ids_allow: vec!["my-plugin".into()],
        };
        let ev = SinkEvent::new().insert("type", "plugin.invoked");
        // No plugin_id field — treated as no match.
        assert!(!f.accepts(&ev));
    }
}
