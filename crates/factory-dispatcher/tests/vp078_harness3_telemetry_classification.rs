//! VP-078 Harness 3 — Positive telemetry-plugin classification.
//!
//! BC-7.06.001 Invariant 6: the 9 named telemetry-only plugins MUST be
//! present in the live registry AND carry `async = true`.
//!
//! AC-009 falsifiable test: `vp_078_harness_3_telemetry_positive_classification()`
//! loads the live registry from `plugins/vsdd-factory/hooks-registry.toml`
//! and asserts all 9 named plugins are present AND have `async_flag == true`.
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
//! - BC-7.06.001 v1.3 Invariant 6: nine telemetry-only plugins must be async=true
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

/// The 9 telemetry-only plugins that MUST be classified `async = true`.
///
/// Canonical list: BC-7.06.001 v1.3 Invariant 6 / VP-078 v1.8 Harness 3.
/// Do NOT modify this list without a BC amendment.
const REQUIRED_ASYNC_PLUGINS: &[&str] = &[
    "capture-commit-activity",
    "capture-pr-activity",
    "session-start-telemetry",
    "session-end-telemetry",
    "worktree-hooks",
    "tool-failure-hooks",
    "track-agent-start",
    "track-agent-stop",
    "session-learning",
];

/// VP-078 Harness 3: all 9 BC-7.06.001 Invariant 6 telemetry plugins must
/// be present in the live registry AND carry `async_flag == true`.
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

    let find_plugin = |name: &str| -> Option<&RegistryEntry> {
        registry.hooks.iter().find(|e| e.name == name)
    };

    for plugin_name in REQUIRED_ASYNC_PLUGINS {
        let entry = find_plugin(plugin_name).unwrap_or_else(|| {
            panic!(
                "test_BC_7_06_001_vp078_harness3_telemetry_positive_classification: \
                 BC-7.06.001 Invariant 6 violation — '{}' missing from live registry. \
                 T-3h must add this plugin with async=true.",
                plugin_name
            )
        });

        assert!(
            entry.async_flag,
            "test_BC_7_06_001_vp078_harness3_telemetry_positive_classification: \
             BC-7.06.001 Invariant 6 violation — '{}' must be async=true (telemetry-only). \
             Current: async_flag={}. T-3h must set async=true in hooks-registry.toml.",
            plugin_name,
            entry.async_flag
        );
    }
}
