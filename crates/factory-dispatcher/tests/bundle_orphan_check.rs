// Test files use .expect()/.unwrap()/.panic!() for failure reporting.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
//! AC-006 + AC-007 Rust workspace tests for S-19.04 bundle hygiene story.
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
//! | T-009 | AC-006   | Real-bundle gate: enumerates real `plugins/vsdd-factory/hook-plugins/*.wasm` against both real registries; asserts zero orphans (EAC-005 load-bearing gate) |
//! | T-010 | AC-007   | Bundle-simulation: stages fixture pre-staging dir (3 known orphans + live WASM); asserts staged artifact has zero orphans per real registries (EAC-005 post-staging assertion) |
//!
//! ## Red Gate Status
//!
//! T-006, T-007, T-008: GREEN — [`collect_orphans_dual`] and [`collect_orphans_hooks_only`]
//! are implemented; fixture-based tests pass against the inline fixture content.
//!
//! T-009: RED — real `hook-plugins/` contains orphan WASMs (hello-hook.wasm,
//! vsdd_context_resolvers.wasm, wasm_resolver_export.wasm); assert fails listing each
//! `ORPHAN: <name>`. Implementer must delete/exclude the 3 orphan files to green this test.
//!
//! T-010: RED — [`stage_release_bundle`] contains `todo!()` stub; test panics at runtime.
//! Implementer replaces the stub with logic mirroring the fixed release.yml exclusion.
//!
//! ## Fixture Layout
//!
//! Fixture TOML sources are canonical at:
//!   `crates/factory-dispatcher/tests/fixtures/bundle-orphan/hooks-registry-fixture.toml`
//!   `crates/factory-dispatcher/tests/fixtures/bundle-orphan/resolvers-registry-fixture.toml`
//!
//! These files are embedded at compile time via `include_str!()` into `HOOKS_REGISTRY_FIXTURE`
//! and `RESOLVERS_REGISTRY_FIXTURE`. The fixture .toml files are the single source of truth;
//! any edits must be made there, not to the constants.
//!
//! Each test creates a temporary directory containing:
//! - `hook-plugins/` directory with fixture `.wasm` files (empty; only filename matters)
//! - A `hooks-registry.toml` written from the fixture content
//! - A `resolvers-registry.toml` written from the fixture content
//!
//! Story: S-19.04
//! VP Trace: — (AC-006 wires EAC-005 as load-bearing leg; no BC mapping)

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::tempdir;

// ---------------------------------------------------------------------------
// Fixture TOML content (canonical source: fixtures/bundle-orphan/*.toml)
// ---------------------------------------------------------------------------

/// Hooks-registry fixture: references only `hooks-only.wasm`.
/// `resolvers-only.wasm` and `neither-registry.wasm` are absent.
///
/// Canonical source: `crates/factory-dispatcher/tests/fixtures/bundle-orphan/hooks-registry-fixture.toml`
const HOOKS_REGISTRY_FIXTURE: &str =
    include_str!("fixtures/bundle-orphan/hooks-registry-fixture.toml");

/// Resolvers-registry fixture: references only `resolvers-only.wasm`.
/// `hooks-only.wasm` and `neither-registry.wasm` are absent.
///
/// Canonical source: `crates/factory-dispatcher/tests/fixtures/bundle-orphan/resolvers-registry-fixture.toml`
const RESOLVERS_REGISTRY_FIXTURE: &str =
    include_str!("fixtures/bundle-orphan/resolvers-registry-fixture.toml");

// ---------------------------------------------------------------------------
// Detection helpers
// ---------------------------------------------------------------------------

/// Parse all `plugin = "hook-plugins/<name>"` references from a TOML registry file.
///
/// Scans every line for the pattern `plugin = "hook-plugins/<filename>"` and extracts
/// the bare filename (e.g., `"hooks-only.wasm"`).  Works for both `hooks-registry.toml`
/// (which uses `[[hooks]]` sections) and `resolvers-registry.toml` (which uses
/// `[[resolvers]]` sections) because both registries use the same `plugin =` key form.
fn parse_plugin_refs(registry: &Path) -> HashSet<String> {
    let content = fs::read_to_string(registry)
        .unwrap_or_else(|e| panic!("failed to read registry {}: {}", registry.display(), e));
    let mut refs = HashSet::new();
    for line in content.lines() {
        let line = line.trim();
        // Match:  plugin = "hook-plugins/<name>"
        // Also handles single-quote TOML values.
        if let Some(rest) = line.strip_prefix("plugin = ") {
            let value = rest.trim_matches(|c| c == '"' || c == '\'');
            if let Some(filename) = value.strip_prefix("hook-plugins/") {
                refs.insert(filename.to_string());
            }
        }
    }
    refs
}

/// Enumerate orphan WASMs from `hook_plugins_dir` using DUAL-registry detection.
///
/// A WASM at `hook_plugins_dir/<name>.wasm` is an orphan if its filename does not
/// appear as a `plugin = "hook-plugins/<name>.wasm"` value in EITHER
/// `hooks_registry` OR `resolvers_registry`.
///
/// Returns a `Vec<String>` of orphan WASM base-names (e.g., `"neither-registry.wasm"`).
/// Returns an empty Vec when every WASM in the directory is referenced by at least one
/// registry (EAC-005: zero-orphan assertion passes).
fn collect_orphans_dual(
    hook_plugins_dir: &Path,
    hooks_registry: &Path,
    resolvers_registry: &Path,
) -> Vec<String> {
    let hooks_refs = parse_plugin_refs(hooks_registry);
    let resolvers_refs = parse_plugin_refs(resolvers_registry);

    let mut orphans = Vec::new();
    let entries = fs::read_dir(hook_plugins_dir)
        .unwrap_or_else(|e| panic!("failed to read dir {}: {}", hook_plugins_dir.display(), e));
    for entry in entries {
        let entry = entry.expect("dir entry must be readable");
        let filename = entry.file_name().to_string_lossy().into_owned();
        if filename.ends_with(".wasm")
            && !hooks_refs.contains(&filename)
            && !resolvers_refs.contains(&filename)
        {
            orphans.push(filename);
        }
    }
    orphans.sort(); // deterministic order for test assertions
    orphans
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
fn collect_orphans_hooks_only(hook_plugins_dir: &Path, hooks_registry: &Path) -> Vec<String> {
    let hooks_refs = parse_plugin_refs(hooks_registry);

    let mut orphans = Vec::new();
    let entries = fs::read_dir(hook_plugins_dir)
        .unwrap_or_else(|e| panic!("failed to read dir {}: {}", hook_plugins_dir.display(), e));
    for entry in entries {
        let entry = entry.expect("dir entry must be readable");
        let filename = entry.file_name().to_string_lossy().into_owned();
        if filename.ends_with(".wasm") && !hooks_refs.contains(&filename) {
            orphans.push(filename);
        }
    }
    orphans.sort();
    orphans
}

// ---------------------------------------------------------------------------
// Workspace-root helper
// ---------------------------------------------------------------------------

/// Walk up from `CARGO_MANIFEST_DIR` until the workspace root is found.
///
/// The workspace root is identified by the presence of `plugins/vsdd-factory/` — the
/// canonical artifact directory that only exists at the repo root. Walking up from
/// `crates/factory-dispatcher` takes exactly 2 hops; the loop handles structural
/// refactors robustly without hard-coded level counts.
fn workspace_root() -> PathBuf {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    loop {
        if dir.join("plugins").join("vsdd-factory").exists() {
            return dir;
        }
        match dir.parent() {
            Some(parent) => dir = parent.to_path_buf(),
            None => panic!(
                "workspace root not found: walked up from {} without finding \
                 plugins/vsdd-factory/ directory",
                env!("CARGO_MANIFEST_DIR")
            ),
        }
    }
}

// ---------------------------------------------------------------------------
// AC-007 staging simulation stub
// ---------------------------------------------------------------------------

/// Simulate the release.yml artifact-staging inner case-arm exclusion logic.
///
/// Copies every `.wasm` from `src_dir/` into `dst_dir/` EXCEPT the three known orphan
/// files that the fixed `release.yml` excludes:
/// - `hello-hook.wasm`           — dev sample; REMOVED from release.yml build+copy steps (AC-001)
/// - `vsdd_context_resolvers.wasm` — stale underscore artifact; moved to skip path (AC-001)
/// - `wasm_resolver_export.wasm`   — stale resolver export; moved to skip path (AC-001)
///
/// `vsdd-context-resolvers.wasm` (hyphen, live WaveContextResolver) IS copied; it is
/// referenced by `resolvers-registry.toml` and must survive staging (AC-007 keep-assertion (i)).
///
/// **At Red Gate this function panics with `todo!()`.**  The implementer replaces this
/// stub with logic that matches the corrected release.yml inner case-arm behavior.
///
/// AC-007 / EAC-005: the staged `dst_dir` must contain zero dual-registry-orphan WASMs.
fn stage_release_bundle(src_dir: &Path, dst_dir: &Path) {
    // Red Gate stub — implementer writes the exclusion logic here.
    // The implementation must mirror the fixed release.yml inner case-arm:
    //   vsdd_context_resolvers.wasm|wasm_resolver_export.wasm) echo "skip..."; continue ;;
    // and the removed hello-hook build+copy steps.
    let _ = (src_dir, dst_dir); // suppress unused-variable warnings at Red Gate
    todo!(
        "AC-007: implement staging logic that mirrors the fixed release.yml exclusion — \
         skip hello-hook.wasm, vsdd_context_resolvers.wasm, wasm_resolver_export.wasm; \
         copy all other WASMs from src_dir to dst_dir"
    )
}

// ---------------------------------------------------------------------------
// T-006 — AC-006 fixture-a: resolvers-registry-only WASM is non-orphan (EC-003 regression gate)
//
// Scenario:
//   hook-plugins/
//     resolvers-only.wasm      — referenced by resolvers-registry only
//   hooks-registry.toml        — references hooks-only.wasm (NOT resolvers-only.wasm)
//   resolvers-registry.toml    — references resolvers-only.wasm
//
// Expected: collect_orphans_dual returns [] (no orphans).
// resolvers-only.wasm must NOT be flagged as orphan when the dual-registry check is used.
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
    fs::write(&hooks_reg, HOOKS_REGISTRY_FIXTURE).expect("hooks-registry fixture must be written");
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
// Expected:
//   collect_orphans_dual returns ["neither-registry.wasm"].
//   Test verifies the orphan is present AND the ORPHAN: <name> format matches AC-006.
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
    fs::write(&hooks_reg, HOOKS_REGISTRY_FIXTURE).expect("hooks-registry fixture must be written");
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
// Expected:
//   collect_orphans_hooks_only returns ["resolvers-only.wasm"].
//   This confirms the dual-registry check is LOAD-BEARING: when the resolvers-registry
//   check is omitted, a resolvers-only WASM IS classified as orphan (false-positive).
//   The dual-registry check in collect_orphans_dual removes this false positive.
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
    fs::write(&hooks_reg, HOOKS_REGISTRY_FIXTURE).expect("hooks-registry fixture must be written");

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

// ---------------------------------------------------------------------------
// T-009 — AC-006 EAC-005: real-bundle zero-orphan gate
//
// Enumerates the REAL `plugins/vsdd-factory/hook-plugins/*.wasm` against the REAL
// `hooks-registry.toml` AND `resolvers-registry.toml`. Asserts zero orphans.
//
// This is the load-bearing EAC-005 gate: it fails as long as orphan WASM files
// remain in the source directory (hello-hook.wasm, vsdd_context_resolvers.wasm,
// wasm_resolver_export.wasm confirmed present at Red Gate on 2026-07-13).
//
// Red Gate failure message will list:
//   ORPHAN: hello-hook.wasm
//   ORPHAN: vsdd_context_resolvers.wasm
//   ORPHAN: wasm_resolver_export.wasm
//
// Implementer fix: delete the 3 orphan source files from plugins/vsdd-factory/hook-plugins/.
// After fix, this test passes because zero WASMs remain unreferenced by either registry.
//
// Locates workspace root robustly via `workspace_root()` — walks up from
// CARGO_MANIFEST_DIR until `plugins/vsdd-factory/` is found (2 hops for current layout).
// ---------------------------------------------------------------------------
#[test]
fn test_S_19_04_ac006_T009_real_bundle_zero_orphans() {
    let root = workspace_root();
    let hook_plugins_dir = root.join("plugins/vsdd-factory/hook-plugins");
    let hooks_registry = root.join("plugins/vsdd-factory/hooks-registry.toml");
    let resolvers_registry = root.join("plugins/vsdd-factory/resolvers-registry.toml");

    assert!(
        hook_plugins_dir.exists(),
        "T-009: plugins/vsdd-factory/hook-plugins/ not found under workspace root {}; \
         workspace_root() walk may be misconfigured",
        root.display()
    );
    assert!(
        hooks_registry.exists(),
        "T-009: plugins/vsdd-factory/hooks-registry.toml not found under workspace root {}",
        root.display()
    );
    assert!(
        resolvers_registry.exists(),
        "T-009: plugins/vsdd-factory/resolvers-registry.toml not found under workspace root {}",
        root.display()
    );

    let orphans = collect_orphans_dual(&hook_plugins_dir, &hooks_registry, &resolvers_registry);

    assert!(
        orphans.is_empty(),
        "T-009 AC-006 EAC-005: zero orphan WASMs expected in real hook-plugins/ bundle; \
         found {} orphan(s) — implementer must delete these files from \
         plugins/vsdd-factory/hook-plugins/:\n{}",
        orphans.len(),
        orphans
            .iter()
            .map(|n| format!("  ORPHAN: {}", n))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

// ---------------------------------------------------------------------------
// T-010 — AC-007 EAC-005: release.yml bundle-simulation zero-orphan gate
//
// Simulates the release.yml artifact-staging step against a fixture pre-staging
// directory containing the 3 known orphan WASMs + the live vsdd-context-resolvers.wasm.
//
// Calls `stage_release_bundle(pre_staging_dir, artifact_dir)` — the stub that the
// implementer fills with logic mirroring the fixed release.yml inner case-arm exclusion.
//
// Red Gate: `stage_release_bundle` panics with `todo!()` → test FAILS.
// After implementation: orphans are excluded from artifact_dir; the assert passes.
//
// This gate is DISTINCT from T-009:
//   T-009 verifies SOURCE state (no orphan files in source after implementer deletes them)
//   T-010 verifies STAGING LOGIC (release.yml exclusion correctly suppresses orphans even
//         when they are present as inputs — i.e., the staging logic itself is correct)
//
// AC-007 keep-assertion (i): vsdd-context-resolvers.wasm must survive staging (live WASM).
// AC-007 keep-assertion (ii): the real resolvers-registry.toml must still reference it.
//
// EAC-005: staged bundle dual-registry-orphan count must equal 0.
// ---------------------------------------------------------------------------
#[test]
fn test_S_19_04_ac007_T010_release_staging_excludes_orphans() {
    // Build the pre-staging fixture directory: 3 known orphans + 1 live WASM
    let tmp = tempdir().expect("tempdir must create successfully");
    let pre_staging = tmp.path().join("pre-staging");
    fs::create_dir_all(&pre_staging).expect("pre-staging dir must be created");

    // The 3 known orphan WASMs from the rc.22 smoke finding (S-19.04 narrative)
    for name in &[
        "hello-hook.wasm",
        "vsdd_context_resolvers.wasm",
        "wasm_resolver_export.wasm",
    ] {
        fs::write(pre_staging.join(name), b"")
            .unwrap_or_else(|e| panic!("write fixture {}: {}", name, e));
    }
    // The live WaveContextResolver that MUST appear in the staged bundle (keep-assertion (i))
    fs::write(pre_staging.join("vsdd-context-resolvers.wasm"), b"")
        .expect("vsdd-context-resolvers.wasm fixture must be written");

    // Create empty artifact (destination) directory
    let artifact = tmp.path().join("artifact");
    fs::create_dir_all(&artifact).expect("artifact dir must be created");

    // Red Gate: stage_release_bundle panics with todo!() here.
    // After implementation: exclusion logic copies only non-orphan WASMs to artifact/.
    stage_release_bundle(&pre_staging, &artifact);

    // Verify zero orphans in staged bundle against the real registries
    let root = workspace_root();
    let hooks_reg = root.join("plugins/vsdd-factory/hooks-registry.toml");
    let resolvers_reg = root.join("plugins/vsdd-factory/resolvers-registry.toml");
    let orphans = collect_orphans_dual(&artifact, &hooks_reg, &resolvers_reg);

    assert!(
        orphans.is_empty(),
        "T-010 AC-007 EAC-005: staged artifact bundle must contain zero dual-registry-orphan \
         WASMs; found {} orphan(s) — release.yml staging logic is not excluding them:\n{}",
        orphans.len(),
        orphans
            .iter()
            .map(|n| format!("  ORPHAN: {}", n))
            .collect::<Vec<_>>()
            .join("\n")
    );

    // AC-007 keep-assertion (i): live WaveContextResolver must survive staging
    assert!(
        artifact.join("vsdd-context-resolvers.wasm").exists(),
        "T-010 AC-007 keep-assertion (i): vsdd-context-resolvers.wasm must be present in \
         staged artifact/ — the live WaveContextResolver must not be excluded by the \
         staging logic (EC-003)"
    );

    // AC-007 keep-assertion (ii): real resolvers-registry.toml must still reference it
    let resolvers_reg_content = fs::read_to_string(&resolvers_reg)
        .expect("resolvers-registry.toml must be readable for keep-assertion (ii)");
    assert!(
        resolvers_reg_content.contains("hook-plugins/vsdd-context-resolvers.wasm"),
        "T-010 AC-007 keep-assertion (ii): resolvers-registry.toml must contain \
         'hook-plugins/vsdd-context-resolvers.wasm' — registry reference must remain intact"
    );
}
