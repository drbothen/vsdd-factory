// Test files use .expect()/.unwrap()/.panic!() for failure reporting.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
//! VP-078 Harness 3 — Positive telemetry-plugin classification.
//!
//! BC-7.06.001 Invariant 6: the 9 named telemetry-only plugins MUST be
//! present in the live registry AND carry `async = true`.
//!
//! AC-009 falsifiable test: `vp_078_harness_3_telemetry_positive_classification()`
//! loads the live registry from `plugins/vsdd-factory/hooks-registry.toml`
//! and asserts all 9 named plugins are present AND have `async_flag == true`.
//!
//! # F-P1-008 fix: (name, event) tuple iteration
//!
//! The original implementation used `registry.hooks.iter().find(|e| e.name == name)`,
//! which returned only the FIRST matching entry by name. With multiple entries sharing
//! the same name across different events (e.g., `worktree-hooks` appears once with
//! `event = "WorktreeCreate"` and once with `event = "WorktreeRemove"`), this masked
//! misclassification of the second entry.
//!
//! Fix: `REQUIRED_ASYNC_PLUGINS` is now a `&[(&str, &str)]` slice of (name, event)
//! tuples. Each tuple binds a specific (name, event) pair. The test asserts
//! `async_flag == true` for EACH specific (name, event) tuple entry — not just
//! the first match by name.
//!
//! # Red Gate
//!
//! RED until T-3h: `hooks-registry.toml` has not yet been updated to
//! `schema_version = 2` or the 9 telemetry plugins classified `async = true`.
//! This test panics at `Registry::load()` with a schema-version error (E-REG-001)
//! once T-3a raises `REGISTRY_SCHEMA_VERSION = 2`, or fails the `async_flag`
//! assertions before T-3h classifies the 9 plugins.
//!
//! # BC traces
//!
//! - BC-7.06.001 v1.7 Invariant 6: nine telemetry-only plugins must be async=true
//! - BC-7.06.001 v1.7 Invariant 7: (name, event, tool) tuple is unique per registry
//! - VP-078 v1.8 Harness 3: positive-classification integration test
//! - AC-009 (S-15.01 v1.6)

use factory_dispatcher::registry::{Registry, RegistryEntry};
use std::path::PathBuf;

/// Resolve the path to the live hooks-registry.toml from CARGO_MANIFEST_DIR.
///
/// `CARGO_MANIFEST_DIR` points to `crates/factory-dispatcher`; walk two
/// levels up to reach the workspace root, then into the registry file.
/// Matches the precedent in `loads_legacy_registry.rs` and `bc_7_03_079_080_parity.rs`.
fn registry_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates/")
        .parent()
        .expect("workspace root")
        .join("plugins")
        .join("vsdd-factory")
        .join("hooks-registry.toml")
}

/// The 9 telemetry-only plugins that MUST be classified `async = true`, expressed
/// as (name, event, tool) tuples per BC-7.06.001 v1.7 Invariant 7.
///
/// Using (name, event) tuples — not name alone — ensures that ALL entries for a
/// given name are individually verified.  `worktree-hooks` appears twice in the
/// registry (WorktreeCreate + WorktreeRemove); the original name-only `find()`
/// returned only the first match, masking misclassification of the second entry
/// (F-P1-008 fix).
///
/// Canonical list: BC-7.06.001 v1.7 Invariant 6 / VP-078 v1.8 Harness 3.
/// Do NOT modify this list without a BC amendment.
///
/// Verified against `plugins/vsdd-factory/hooks-registry.toml` as of S-15.01 v1.7.
const REQUIRED_ASYNC_PLUGINS: &[(&str, &str)] = &[
    ("session-start-telemetry", "SessionStart"),
    ("session-end-telemetry", "SessionEnd"),
    ("worktree-hooks", "WorktreeCreate"),
    ("worktree-hooks", "WorktreeRemove"),
    ("tool-failure-hooks", "PostToolUseFailure"),
    ("capture-commit-activity", "PostToolUse"),
    ("capture-pr-activity", "PostToolUse"),
    ("track-agent-start", "PreToolUse"),
    ("track-agent-stop", "SubagentStop"),
    // Note: session-learning (Stop) and convergence-tracker/purity-check (PostToolUse)
    // are intentionally omitted below because they are not in BC-7.06.001 Invariant 6
    // per the canonical list. If Invariant 6 is amended, update this list.
];

/// Lookup helper: find the specific registry entry matching (name, event) tuple.
///
/// Unlike `find(|e| e.name == name)`, this function searches by BOTH name and event,
/// correctly handling duplicate names across different events (e.g., worktree-hooks).
/// Returns all matching entries (should be exactly one per BC-7.06.001 Invariant 7).
fn find_entry_by_name_event<'a>(
    registry: &'a factory_dispatcher::registry::Registry,
    name: &str,
    event: &str,
) -> Vec<&'a RegistryEntry> {
    registry
        .hooks
        .iter()
        .filter(|e| e.name == name && e.event == event)
        .collect()
}

/// VP-078 Harness 3: all 9 BC-7.06.001 Invariant 6 telemetry plugins must
/// be present in the live registry AND carry `async_flag == true`.
///
/// Uses (name, event) tuple lookup to correctly handle multi-event plugins
/// (e.g., worktree-hooks appearing under WorktreeCreate AND WorktreeRemove).
/// This ensures that BOTH worktree-hooks entries are individually verified,
/// not just the first match (F-P1-008 regression fix).
///
/// RED: will fail at `Registry::load()` with schema_version error (E-REG-001)
/// until T-3h sets `schema_version = 2` in hooks-registry.toml; then fails
/// `async_flag` assertions until T-3h sets `async = true` for all 9 plugins.
///
/// # AC coverage
///
/// AC-009 (telemetry plugin positive classification)
#[test]
fn test_BC_7_06_001_vp078_harness3_telemetry_positive_classification() {
    let registry_path = registry_path();

    // RED: Registry::load() calls validate_async_block_invariant() which is todo!()
    // until T-3f. Before T-3f this panics. After T-3f + T-3h this test exercises
    // the assertion loop.
    let registry = Registry::load(registry_path.as_path()).unwrap_or_else(|e| {
        panic!(
            "test_BC_7_06_001_vp078_harness3_telemetry_positive_classification: \
             Registry::load failed — AC-009 FAIL. Error: {:?}. \
             This is expected RED-state until T-3h sets schema_version=2 and \
             classifies all 9 telemetry plugins as async=true.",
            e
        )
    });

    for (plugin_name, event_name) in REQUIRED_ASYNC_PLUGINS {
        // Use (name, event) tuple lookup — NOT find(|e| e.name == name) — to avoid
        // the F-P1-008 find-first bug where a second entry with the same name but
        // different event is silently skipped.
        let entries = find_entry_by_name_event(&registry, plugin_name, event_name);

        assert!(
            !entries.is_empty(),
            "test_BC_7_06_001_vp078_harness3_telemetry_positive_classification: \
             BC-7.06.001 Invariant 6 violation — ('{}', '{}') missing from live registry. \
             T-3h must add this (name, event) entry with async=true.",
            plugin_name,
            event_name
        );

        // BC-7.06.001 Invariant 7: (name, event) tuple must be unique.
        // If multiple entries match, the registry has a uniqueness violation.
        assert_eq!(
            entries.len(),
            1,
            "test_BC_7_06_001_vp078_harness3_telemetry_positive_classification: \
             BC-7.06.001 Invariant 7 violation — ('{}', '{}') appears {} times in live registry. \
             (name, event) tuples must be unique.",
            plugin_name,
            event_name,
            entries.len()
        );

        let entry = entries[0];
        assert!(
            entry.async_flag,
            "test_BC_7_06_001_vp078_harness3_telemetry_positive_classification: \
             BC-7.06.001 Invariant 6 violation — ('{}', '{}') must be async=true (telemetry-only). \
             Current: async_flag={}. T-3h must set async=true in hooks-registry.toml.",
            plugin_name, event_name, entry.async_flag
        );
    }
}
