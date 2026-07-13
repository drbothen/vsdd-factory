// Test files use .expect()/.unwrap()/.panic!() for failure reporting.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
//! AC-006 Rust workspace test for S-19.04 bundle hygiene story.
//!
//! Verifies the dual-registry orphan detection invariant (EC-003): a WASM present
//! under `hook-plugins/` is considered non-orphan if referenced by EITHER
//! `hooks-registry.toml` OR `resolvers-registry.toml`. Checking only one registry
//! produces a false-positive orphan classification (root cause of the v1.0 story
//! defect for `vsdd-context-resolvers.wasm`, corrected in S-19.04 v1.1).
//!
//! ## Test Plan
//!
//! | ID    | Story AC | Description |
//! |-------|----------|-------------|
//! | T-006 | AC-006   | fixture-a: resolvers-registry-only WASM → non-orphan (dual-registry regression gate) |
//! | T-007 | AC-006   | fixture-c: neither-registry WASM → orphan, ORPHAN: <name> line confirmed |
//! | T-008 | AC-006   | fixture-d: negative-control (F-P2-010) — resolvers-only WASM is orphan when only hooks-registry used |
//!
//! ## Red Gate Status
//!
//! All three tests FAIL at Red Gate because [`collect_orphans_dual`] and
//! [`collect_orphans_hooks_only`] contain `todo!()` stubs. The implementer
//! replaces these stubs with real detection logic; all three tests must then pass.
//!
//! ## Fixture Layout
//!
//! Each test creates a temporary directory containing:
//! - `hook-plugins/` directory with fixture `.wasm` files (empty; only filename matters)
//! - A `hooks-registry.toml` referencing only `hooks-only.wasm`
//! - A `resolvers-registry.toml` referencing only `resolvers-only.wasm`
//!
//! Fixture registry sources live at:
//! `crates/factory-dispatcher/tests/fixtures/bundle-orphan/`
//!
//! Story: S-19.04
//! VP Trace: — (AC-006 wires EAC-005 as load-bearing leg; no BC mapping)

use std::fs;
use std::path::Path;
use tempfile::tempdir;

// ---------------------------------------------------------------------------
// Detection helpers (RED GATE stubs — implementer fills these in)
// ---------------------------------------------------------------------------

/// Enumerate orphan WASMs from `hook_plugins_dir` using DUAL-registry detection.
///
/// A WASM at `hook_plugins_dir/<name>.wasm` is an orphan if its filename does not
/// appear as a `plugin = "hook-plugins/<name>.wasm"` value in EITHER
/// `hooks_registry` OR `resolvers_registry`.
///
/// Returns a `Vec<String>` of orphan WASM base-names (e.g., `"neither-registry.wasm"`).
/// Returns an empty Vec when every WASM in the directory is referenced by at least one
/// registry (EAC-005: zero-orphan assertion passes).
///
/// # RED GATE
/// Contains `todo!()` — implementation pending (S-19.04 implementer task).
/// All tests that call this function fail at Red Gate with "not yet implemented".
fn collect_orphans_dual(
    hook_plugins_dir: &Path,
    hooks_registry: &Path,
    resolvers_registry: &Path,
) -> Vec<String> {
    let _ = (hook_plugins_dir, hooks_registry, resolvers_registry);
    todo!("S-19.04 AC-006: implement dual-registry orphan detection — \
           enumerate hook_plugins_dir/*.wasm, parse both registry TOMLs for \
           plugin = \"hook-plugins/<name>\" references, return names absent from both")
}

/// Enumerate orphan WASMs from `hook_plugins_dir` using HOOKS-ONLY detection.
///
/// Same as [`collect_orphans_dual`] but WITHOUT consulting `resolvers-registry.toml`.
/// A WASM not referenced by `hooks_registry` is classified as orphan even if it would
/// be referenced by a resolvers registry.
///
/// Used for T-008 negative-control (F-P2-010): calling this function with only the
/// hooks-registry data must classify a resolvers-only WASM as orphan, confirming
/// the dual-registry check in [`collect_orphans_dual`] is load-bearing, not advisory.
///
/// # RED GATE
/// Contains `todo!()` — implementation pending (S-19.04 implementer task).
fn collect_orphans_hooks_only(hook_plugins_dir: &Path, hooks_registry: &Path) -> Vec<String> {
    let _ = (hook_plugins_dir, hooks_registry);
    todo!("S-19.04 AC-006: implement hooks-only orphan detection — \
           enumerate hook_plugins_dir/*.wasm, parse only hooks_registry TOML for \
           plugin = \"hook-plugins/<name>\" references, return names absent from it")
}

// ---------------------------------------------------------------------------
// Fixture TOML strings (canonical; match fixtures/bundle-orphan/ files)
// ---------------------------------------------------------------------------

/// Hooks-registry fixture: references only `hooks-only.wasm`.
/// `resolvers-only.wasm` and `neither-registry.wasm` are absent.
const HOOKS_REGISTRY_FIXTURE: &str = r#"
schema_version = 2

[[hooks]]
name = "hooks-only-hook"
event = "PostToolUse"
tool = "^(Edit|Write)$"
plugin = "hook-plugins/hooks-only.wasm"
timeout_ms = 5000
on_error = "continue"
"#;

/// Resolvers-registry fixture: references only `resolvers-only.wasm`.
/// `hooks-only.wasm` and `neither-registry.wasm` are absent.
const RESOLVERS_REGISTRY_FIXTURE: &str = r#"
schema_version = 1

[[resolvers]]
name = "resolvers_only"
plugin = "hook-plugins/resolvers-only.wasm"
context_key = "resolvers_only"
path_allow = [".factory/wave-state.yaml"]
"#;

// ---------------------------------------------------------------------------
// T-006 — AC-006 fixture-a: resolvers-registry-only WASM is non-orphan (EC-003 regression gate)
//
// Scenario:
//   hook-plugins/
//     resolvers-only.wasm      — referenced by resolvers-registry only
//   hooks-registry.toml        — references hooks-only.wasm (NOT resolvers-only.wasm)
//   resolvers-registry.toml    — references resolvers-only.wasm
//
// Expected (after implementation): collect_orphans_dual returns [] (no orphans).
// resolvers-only.wasm must NOT be flagged as orphan when the dual-registry check is used.
//
// Red Gate: collect_orphans_dual panics with todo!() → test FAILS ✓
//
// This is the regression gate for EC-003: vsdd-context-resolvers.wasm (hyphen) was
// falsely classified as orphan in v1.0 because only hooks-registry.toml was checked.
// ---------------------------------------------------------------------------
#[test]
fn test_S_19_04_ac006_T006_resolvers_registry_only_wasm_is_non_orphan() {
    let tmp = tempdir().expect("tempdir must create successfully");
    let hook_plugins = tmp.path().join("hook-plugins");
    fs::create_dir_all(&hook_plugins).expect("hook-plugins dir must be created");

    // fixture-a: one WASM referenced only by resolvers-registry
    fs::write(hook_plugins.join("resolvers-only.wasm"), b"")
        .expect("resolvers-only.wasm fixture must be written");

    let hooks_reg = tmp.path().join("hooks-registry.toml");
    let resolvers_reg = tmp.path().join("resolvers-registry.toml");
    fs::write(&hooks_reg, HOOKS_REGISTRY_FIXTURE)
        .expect("hooks-registry fixture must be written");
    fs::write(&resolvers_reg, RESOLVERS_REGISTRY_FIXTURE)
        .expect("resolvers-registry fixture must be written");

    // Dual-registry detection: resolvers-only.wasm is in resolvers-registry → not orphan
    let orphans = collect_orphans_dual(&hook_plugins, &hooks_reg, &resolvers_reg);

    assert!(
        !orphans.contains(&"resolvers-only.wasm".to_string()),
        "T-006 AC-006 EC-003: resolvers-only.wasm must NOT be classified as orphan \
         when referenced by resolvers-registry.toml (dual-registry regression gate); \
         got orphans: {:?}",
        orphans
    );

    assert!(
        orphans.is_empty(),
        "T-006 AC-006: expected zero orphans in fixture-a (all WASMs referenced); \
         got: {:?}",
        orphans
    );
}

// ---------------------------------------------------------------------------
// T-007 — AC-006 fixture-c: neither-registry WASM is classified as orphan
//
// Scenario:
//   hook-plugins/
//     neither-registry.wasm    — NOT referenced by either registry
//   hooks-registry.toml        — references hooks-only.wasm (absent from hook-plugins)
//   resolvers-registry.toml    — references resolvers-only.wasm (absent from hook-plugins)
//
// Expected (after implementation):
//   collect_orphans_dual returns ["neither-registry.wasm"].
//   Test verifies the orphan is present AND the ORPHAN: <name> format matches AC-006.
//
// Red Gate: collect_orphans_dual panics with todo!() → test FAILS ✓
// ---------------------------------------------------------------------------
#[test]
fn test_S_19_04_ac006_T007_neither_registry_wasm_is_orphan_with_orphan_line() {
    let tmp = tempdir().expect("tempdir must create successfully");
    let hook_plugins = tmp.path().join("hook-plugins");
    fs::create_dir_all(&hook_plugins).expect("hook-plugins dir must be created");

    // fixture-c: one WASM referenced by neither registry
    fs::write(hook_plugins.join("neither-registry.wasm"), b"")
        .expect("neither-registry.wasm fixture must be written");

    let hooks_reg = tmp.path().join("hooks-registry.toml");
    let resolvers_reg = tmp.path().join("resolvers-registry.toml");
    fs::write(&hooks_reg, HOOKS_REGISTRY_FIXTURE)
        .expect("hooks-registry fixture must be written");
    fs::write(&resolvers_reg, RESOLVERS_REGISTRY_FIXTURE)
        .expect("resolvers-registry fixture must be written");

    // Dual-registry detection: neither-registry.wasm is in neither registry → orphan
    let orphans = collect_orphans_dual(&hook_plugins, &hooks_reg, &resolvers_reg);

    // Verify the orphan is detected
    assert!(
        orphans.contains(&"neither-registry.wasm".to_string()),
        "T-007 AC-006: neither-registry.wasm must be classified as orphan \
         (referenced by neither hooks-registry.toml nor resolvers-registry.toml); \
         got orphans: {:?}",
        orphans
    );

    // Verify the ORPHAN: <name> format required by AC-006 spec
    // (EAC-005 assertion would emit: ORPHAN: <name> per AC-006 clause d)
    let orphan_lines: Vec<String> = orphans
        .iter()
        .map(|name| format!("ORPHAN: {}", name))
        .collect();

    assert!(
        orphan_lines.contains(&"ORPHAN: neither-registry.wasm".to_string()),
        "T-007 AC-006: ORPHAN: format must produce 'ORPHAN: neither-registry.wasm'; \
         got lines: {:?}",
        orphan_lines
    );
}

// ---------------------------------------------------------------------------
// T-008 — AC-006 fixture-d (F-P2-010 negative-control)
//
// Scenario:
//   hook-plugins/
//     resolvers-only.wasm      — referenced by resolvers-registry only
//   hooks-registry.toml        — references hooks-only.wasm (NOT resolvers-only.wasm)
//   (resolvers-registry.toml NOT passed to detection function)
//
// Expected (after implementation):
//   collect_orphans_hooks_only returns ["resolvers-only.wasm"].
//   This confirms the dual-registry check is LOAD-BEARING: when the resolvers-registry
//   check is omitted, a resolvers-only WASM IS classified as orphan (false-positive).
//   The dual-registry check in collect_orphans_dual removes this false positive.
//
// Red Gate: collect_orphans_hooks_only panics with todo!() → test FAILS ✓
//
// This test guards against regression to the v1.0 defect (EC-003): if someone
// removes the resolvers-registry check from collect_orphans_dual, it would start
// producing false-positive orphans for vsdd-context-resolvers.wasm.
// ---------------------------------------------------------------------------
#[test]
fn test_S_19_04_ac006_T008_negative_control_resolvers_only_is_orphan_with_hooks_only_detection() {
    let tmp = tempdir().expect("tempdir must create successfully");
    let hook_plugins = tmp.path().join("hook-plugins");
    fs::create_dir_all(&hook_plugins).expect("hook-plugins dir must be created");

    // fixture-d: one WASM referenced only by resolvers-registry
    fs::write(hook_plugins.join("resolvers-only.wasm"), b"")
        .expect("resolvers-only.wasm fixture must be written");

    let hooks_reg = tmp.path().join("hooks-registry.toml");
    fs::write(&hooks_reg, HOOKS_REGISTRY_FIXTURE)
        .expect("hooks-registry fixture must be written");

    // Hooks-only detection (NO resolvers-registry argument):
    // resolvers-only.wasm is NOT in hooks-registry → classified as orphan
    let orphans = collect_orphans_hooks_only(&hook_plugins, &hooks_reg);

    assert!(
        orphans.contains(&"resolvers-only.wasm".to_string()),
        "T-008 AC-006 F-P2-010 negative-control: resolvers-only.wasm MUST be classified \
         as orphan when only hooks-registry is used — this confirms the dual-registry check \
         in collect_orphans_dual is load-bearing, not advisory (EC-003); \
         got orphans: {:?}",
        orphans
    );
}
