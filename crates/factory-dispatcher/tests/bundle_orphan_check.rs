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
//! | ID    | Story AC | Status   | Description |
//! |-------|----------|----------|-------------|
//! | T-006 | AC-006   | GREEN    | fixture-a: resolvers-registry-only WASM → non-orphan (dual-registry regression gate) |
//! | T-007 | AC-006   | GREEN    | fixture-c: neither-registry WASM → orphan, ORPHAN: <name> line confirmed |
//! | T-008 | AC-006   | GREEN    | fixture-d: negative-control (F-P2-010) — resolvers-only WASM is orphan when only hooks-registry used |
//! | T-009 | AC-006   | GREEN†   | Hermetic real-bundle gate: enumerates GIT-TRACKED set (`git ls-files`) against both real registries; asserts zero tracked orphans (EAC-005 standing regression gate) |
//! | T-010 | AC-007   | GREEN    | Bundle-simulation: stages fixture with underscore-named WASMs through `stage_release_bundle`; asserts staged artifact has zero orphans per real registries and proves underscore-glob semantics (RED at 298389b0 via todo!(); GREEN since d9502701) |
//!
//! † T-009 is a STANDING GREEN GATE — passes immediately on any clean checkout where no
//! orphan WASMs are tracked in git, and remains green on contaminated worktrees because
//! local build artifacts (untracked per .gitignore) are excluded from the git-tracked set.
//!
//! ## Hermetic Design for T-009
//!
//! `plugins/vsdd-factory/hook-plugins/` is listed in `.gitignore` (~L64). Running
//! `cargo build --target wasm32-wasip1 --workspace` deposits underscore-named lib-target
//! stub WASMs there as untracked files. A raw `fs::read_dir` enumeration would find
//! these build artifacts and false-fail the orphan assertion on any post-build dev machine,
//! making `cargo test --workspace` (the documented pre-push gate) unreliable. T-009
//! instead calls `git ls-files plugins/vsdd-factory/hook-plugins/` which returns ONLY
//! the tracked set — the explicitly committed, versioned plugin binaries that release.yml
//! stages from a clean checkout. Untracked build artifacts are invisible to git ls-files
//! and cannot contaminate the result.
//!
//! ## Staging Semantics for T-010
//!
//! The release.yml steps "Stage artifact directory" and "Stage wasm plugins" use an
//! underscore-glob case arm as the governing exclusion rule:
//!
//! ```yaml
//! *_*.wasm)
//!   case "$name" in
//!     vsdd_context_resolvers.wasm|wasm_resolver_export.wasm) echo "skip stale..."; continue ;;
//!     *) echo "skip lib-target stub: $name"; continue ;;
//!   esac ;;
//! ```
//!
//! The outer `*_*.wasm` arm matches ANY WASM whose basename contains an underscore.
//! The inner arms are legacy documentation of specific stale artifacts; they do not limit
//! the outer glob — all underscore-named WASMs are skipped, including future lib-target
//! stubs not enumerated in the inner case. Non-underscore (hyphen-named) WASMs pass through.
//!
//! `hello-hook.wasm` has NO underscore → staging logic would copy it if it appeared in
//! build output. Its exclusion is via BUILD-OMISSION: `cargo build --example hello-hook`
//! was removed from release.yml in AC-001; hello-hook.wasm is never produced by the
//! release build. T-009 (tracked-set gate) provides the secondary guarantee if it were
//! accidentally committed. T-010 fixture therefore does NOT include hello-hook.wasm,
//! faithfully representing post-build-omission staging inputs.
//!
//! Reciprocal anchor: `stage_release_bundle` mirrors the workflow underscore-glob staging
//! logic. The inverse anchor
//! `# Test gate: crates/factory-dispatcher/tests/bundle_orphan_check.rs::stage_release_bundle`
//! is present at both release.yml staging steps (satisfied at d9502701):
//!   - release.yml "Stage artifact directory" step
//!   - release.yml "Stage wasm plugins" step (commit-binaries job)
//! ci.yml staging consumers additionally carry the `# Test gate:` comment pointing here
//! (one-directional; this test does not reference ci.yml step names — see S-19.04 T-010).
//!
//! ## Fixture Layout
//!
//! Fixture TOML sources are canonical at:
//!   `crates/factory-dispatcher/tests/fixtures/bundle-orphan/hooks-registry-fixture.toml`
//!   `crates/factory-dispatcher/tests/fixtures/bundle-orphan/resolvers-registry-fixture.toml`
//!
//! Embedded at compile time via `include_str!()` — the .toml files are the single source
//! of truth; edits must be made there, not to the constants.
//!
//! Story: S-19.04
//! VP Trace: — (AC-006 wires EAC-005 as load-bearing leg; no BC mapping)

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
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

/// Return the git-tracked WASM basenames under `plugins/vsdd-factory/hook-plugins/`.
///
/// Runs `git ls-files plugins/vsdd-factory/hook-plugins/` from `root`, filters for
/// `.wasm` extension, and returns bare filenames (no directory prefix).
///
/// # Hermetic contract
///
/// `plugins/vsdd-factory/hook-plugins/` is gitignored (.gitignore ~L64). Local cargo
/// builds deposit underscore-named lib-target stub WASMs there as untracked files.
/// Using `git ls-files` instead of `fs::read_dir` ensures only the committed, versioned
/// set is examined — identical to what `release.yml` stages from a clean `git checkout`.
/// Untracked build artifacts cannot contaminate the result on any post-build dev machine.
fn git_tracked_wasm_names(root: &Path) -> Vec<String> {
    let output = Command::new("git")
        .args(["ls-files", "plugins/vsdd-factory/hook-plugins/"])
        .current_dir(root)
        .output()
        .expect("git ls-files must execute; ensure git is on PATH in the test environment");

    assert!(
        output.status.success(),
        "git ls-files exited with status {}: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout)
        .expect("git ls-files output must be valid UTF-8")
        .lines()
        .filter(|line| line.ends_with(".wasm"))
        .map(|line| {
            Path::new(line)
                .file_name()
                .expect("every git ls-files path must have a filename component")
                .to_string_lossy()
                .into_owned()
        })
        .collect()
}

// ---------------------------------------------------------------------------
// AC-007 staging simulation stub
// ---------------------------------------------------------------------------

/// Simulate the release.yml artifact-staging underscore-glob exclusion logic.
///
/// Mirrors the outer case arm from release.yml steps "Stage artifact directory" and
/// "Stage wasm plugins" (commit-binaries job):
///
/// ```yaml
/// *_*.wasm)          # <- outer arm: skip any WASM with underscore in basename
///   case "$name" in
///     vsdd_context_resolvers.wasm|wasm_resolver_export.wasm) ... continue ;;
///     *) echo "skip lib-target stub: $name"; continue ;;
///   esac ;;
/// ```
///
/// **Semantics: skip any `.wasm` whose basename contains an underscore (`'_'`).**
/// The outer `*_*.wasm` glob is the governing rule. The inner named arms document
/// specific stale artifacts but do not restrict the outer glob — any new underscore-named
/// lib-target stub is skipped automatically. Non-underscore (hyphen-named) WASMs are
/// copied from `src_dir/` to `dst_dir/`.
///
/// # `hello-hook.wasm` exclusion mechanism
///
/// `hello-hook.wasm` contains no underscore — the staging logic would copy it if
/// present in build output. Its exclusion from release bundles is via BUILD-OMISSION
/// (`cargo build --example hello-hook` removed from release.yml in AC-001). The T-010
/// fixture does not include hello-hook.wasm; it represents post-build-omission inputs.
///
/// # Reciprocal anchor
///
/// The inverse anchor `# Test gate: ...::stage_release_bundle` is present at both
/// release.yml staging steps (applied at d9502701): "Stage artifact directory" and
/// "Stage wasm plugins" (commit-binaries job). ci.yml staging consumers additionally
/// carry the `# Test gate:` comment pointing here (one-directional; this test does not
/// reference ci.yml step names — see S-19.04 T-010).
/// (Was IMPLEMENTER FLAG at 2bd3c898; satisfied at d9502701.)
///
/// AC-007 / EAC-005: `dst_dir` must contain zero dual-registry-orphan WASMs after staging.
fn stage_release_bundle(src_dir: &Path, dst_dir: &Path) {
    // Mirrors the *_*.wasm outer case arm from release.yml steps "Stage artifact
    // directory" and "Stage wasm plugins":
    //
    //   *_*.wasm)            <- outer arm: skip any WASM basename containing '_'
    //     case "$name" in
    //       vsdd_context_resolvers.wasm|wasm_resolver_export.wasm) ... continue ;;
    //       *) echo "skip lib-target stub: $name"; continue ;;
    //     esac ;;
    //
    // The outer *_*.wasm glob is the governing rule. The inner named arms in
    // release.yml document specific stale artifacts but do not restrict the outer
    // glob — all underscore-named WASMs are skipped, including future lib-target
    // stubs not enumerated in the inner case (proven by some_new_stub_lib.wasm
    // fixture in T-010 assertion 3). Non-underscore (hyphen-named) WASMs are copied.
    //
    // hello-hook.wasm exclusion is via BUILD-OMISSION (AC-001: the --example hello-hook
    // build step was removed from release.yml); it has no underscore so the staging
    // logic would copy it if present. The T-010 fixture omits hello-hook.wasm to
    // faithfully represent post-build-omission staging inputs.

    let entries = fs::read_dir(src_dir)
        .unwrap_or_else(|e| panic!("failed to read src_dir {}: {}", src_dir.display(), e));
    for entry in entries {
        let entry = entry.expect("dir entry must be readable");
        let filename = entry.file_name().to_string_lossy().into_owned();
        if !filename.ends_with(".wasm") {
            continue;
        }
        // Outer *_*.wasm glob: skip any .wasm whose basename contains an underscore.
        if filename.contains('_') {
            continue;
        }
        let dst = dst_dir.join(&filename);
        fs::copy(entry.path(), &dst)
            .unwrap_or_else(|e| panic!("failed to copy {} to {}: {}", filename, dst.display(), e));
    }
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
// Regression gate for EC-003: vsdd-context-resolvers.wasm (hyphen) was falsely classified
// as orphan in v1.0 because only hooks-registry.toml was checked.
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

    assert!(
        orphans.contains(&"neither-registry.wasm".to_string()),
        "T-007 AC-006: neither-registry.wasm must be classified as orphan \
         (referenced by neither hooks-registry.toml nor resolvers-registry.toml); \
         got orphans: {:?}",
        orphans
    );

    // Verify the ORPHAN: <name> format required by AC-006 spec clause (d)
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
//   Confirms the dual-registry check is LOAD-BEARING: omitting the resolvers-registry
//   check produces a false-positive orphan. Regression gate for EC-003 v1.0 defect.
// ---------------------------------------------------------------------------
#[test]
fn test_S_19_04_ac006_T008_negative_control_resolvers_only_is_orphan_with_hooks_only_detection() {
    let tmp = tempdir().expect("tempdir must create successfully");
    let hook_plugins = tmp.path().join("hook-plugins");
    fs::create_dir_all(&hook_plugins).expect("hook-plugins dir must be created");

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
         as orphan when only hooks-registry is used — confirms dual-registry check in \
         collect_orphans_dual is load-bearing, not advisory (EC-003); \
         got orphans: {:?}",
        orphans
    );
}

// ---------------------------------------------------------------------------
// T-009 — AC-006 EAC-005: hermetic tracked-bundle zero-orphan standing gate
//
// Enumerates the GIT-TRACKED set under `plugins/vsdd-factory/hook-plugins/` via
// `git ls-files` and asserts every tracked WASM is referenced by at least one registry.
//
// HERMETIC DESIGN: `hook-plugins/` is gitignored. Local `cargo build --target
// wasm32-wasip1` deposits underscore-named lib-target stub WASMs there as UNTRACKED
// files. Using `git ls-files` (not `fs::read_dir`) ensures only the committed set is
// examined — identical to what release.yml stages from a clean git checkout. Untracked
// build artifacts (hello-hook.wasm, vsdd_context_resolvers.wasm, wasm_resolver_export.wasm
// and any lib-target stubs) are invisible to git ls-files and cannot false-fail this test
// on any post-build dev machine, making `cargo test --workspace` reliable as a pre-push gate.
//
// CONTAMINATION RESISTANCE: the enumeration source is the stdout of `git ls-files`, not
// the filesystem dirlist. A worktree containing extra untracked orphan files passes this
// test unchanged.
//
// STANDING GREEN GATE: passes immediately in the current clean checkout (all git-tracked
// WASMs are referenced by hooks-registry or resolvers-registry as of S-19.04 implementation).
// Catches future regressions where an unreferenced WASM is accidentally committed to git.
// ---------------------------------------------------------------------------
#[test]
fn test_S_19_04_ac006_T009_hermetic_tracked_bundle_zero_orphans() {
    let root = workspace_root();
    let hooks_registry = root.join("plugins/vsdd-factory/hooks-registry.toml");
    let resolvers_registry = root.join("plugins/vsdd-factory/resolvers-registry.toml");

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

    // Enumerate ONLY git-tracked WASMs (hermetic: untracked build artifacts excluded)
    let tracked_names = git_tracked_wasm_names(&root);

    assert!(
        !tracked_names.is_empty(),
        "T-009: git ls-files returned no tracked WASMs under \
         plugins/vsdd-factory/hook-plugins/ — check workspace_root() detection"
    );

    let hooks_refs = parse_plugin_refs(&hooks_registry);
    let resolvers_refs = parse_plugin_refs(&resolvers_registry);

    let mut orphans: Vec<&str> = tracked_names
        .iter()
        .filter(|name| !hooks_refs.contains(*name) && !resolvers_refs.contains(*name))
        .map(String::as_str)
        .collect();
    orphans.sort();

    assert!(
        orphans.is_empty(),
        "T-009 AC-006 EAC-005: zero tracked-orphan WASMs expected in \
         plugins/vsdd-factory/hook-plugins/ (git-tracked set via git ls-files); \
         found {} orphan(s) — these tracked WASMs must be removed from git or \
         added to the appropriate registry:\n{}",
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
// Simulates the release.yml "Stage artifact directory" step against a fixture
// pre-staging directory representing build output AFTER the AC-001 fix (hello-hook
// build step removed). Calls `stage_release_bundle(pre_staging, artifact)` and asserts
// the staged artifact has zero orphans per the real registries.
//
// Fixture pre-staging contents (representing post-build-omission cargo output):
//   vsdd_context_resolvers.wasm   — underscore → outer *_*.wasm glob skips it
//   wasm_resolver_export.wasm     — underscore → outer *_*.wasm glob skips it
//   some_new_stub_lib.wasm        — underscore, NOT in inner named denylist → proves
//                                   the outer glob governs, not a hardcoded name list
//   vsdd-context-resolvers.wasm   — hyphen, no underscore → copied (keep-assertion)
//
// NOTE: hello-hook.wasm is ABSENT from the fixture. Its exclusion is via BUILD-OMISSION
// (AC-001: cargo build --example hello-hook removed), not the staging glob. The staging
// logic would copy it if present (no underscore). The fixture faithfully represents
// post-build-omission inputs; T-009 provides the secondary tracked-file guarantee.
//
// TDD history: RED at 298389b0 (todo!() stub); GREEN since d9502701 (underscore-glob impl).
// stage_release_bundle skips underscore names, copies vsdd-context-resolvers.wasm →
//   - zero orphans per real registries (EAC-005)
//   - vsdd-context-resolvers.wasm present in artifact (keep-assertion i)
//   - some_new_stub_lib.wasm NOT in artifact (glob-semantics proof)
//   - resolvers-registry still references vsdd-context-resolvers.wasm (keep-assertion ii)
//
// DISTINCT FROM T-009:
//   T-009 verifies TRACKED SOURCE STATE — regression gate for accidental commit of orphans
//   T-010 verifies STAGING LOGIC CORRECTNESS — the underscore-glob exclusion mechanism
//         itself; exercises stage_release_bundle against fixture inputs
// ---------------------------------------------------------------------------
#[test]
fn test_S_19_04_ac007_T010_release_staging_underscore_glob_excludes_orphans() {
    // Build the pre-staging fixture: underscore-named orphans + live hyphen WASM.
    // Represents cargo build output after hello-hook build step removal (AC-001).
    let tmp = tempdir().expect("tempdir must create successfully");
    let pre_staging = tmp.path().join("pre-staging");
    fs::create_dir_all(&pre_staging).expect("pre-staging dir must be created");

    // Underscore-named WASMs: outer *_*.wasm glob must skip all of these.
    //   vsdd_context_resolvers.wasm, wasm_resolver_export.wasm — the two named stale artifacts.
    //   some_new_stub_lib.wasm — new underscore name NOT in inner denylist; proves glob scope.
    for name in &[
        "vsdd_context_resolvers.wasm",
        "wasm_resolver_export.wasm",
        "some_new_stub_lib.wasm",
    ] {
        fs::write(pre_staging.join(name), b"")
            .unwrap_or_else(|e| panic!("write fixture {}: {}", name, e));
    }

    // Hyphen-named live WaveContextResolver: no underscore → must be copied to artifact/.
    fs::write(pre_staging.join("vsdd-context-resolvers.wasm"), b"")
        .expect("vsdd-context-resolvers.wasm fixture must be written");

    let artifact = tmp.path().join("artifact");
    fs::create_dir_all(&artifact).expect("artifact dir must be created");

    // Implemented since d9502701: underscore names excluded; vsdd-context-resolvers.wasm copied.
    stage_release_bundle(&pre_staging, &artifact);

    // Zero orphans in staged bundle per real registries (EAC-005)
    let root = workspace_root();
    let hooks_reg = root.join("plugins/vsdd-factory/hooks-registry.toml");
    let resolvers_reg = root.join("plugins/vsdd-factory/resolvers-registry.toml");
    let orphans = collect_orphans_dual(&artifact, &hooks_reg, &resolvers_reg);

    assert!(
        orphans.is_empty(),
        "T-010 AC-007 EAC-005: staged artifact must contain zero dual-registry-orphan \
         WASMs; found {} orphan(s) — staging logic is not applying underscore-glob \
         exclusion correctly:\n{}",
        orphans.len(),
        orphans
            .iter()
            .map(|n| format!("  ORPHAN: {}", n))
            .collect::<Vec<_>>()
            .join("\n")
    );

    // AC-007 keep-assertion (i): live WaveContextResolver must survive staging.
    // vsdd-context-resolvers.wasm has no underscore → must be copied to artifact/.
    assert!(
        artifact.join("vsdd-context-resolvers.wasm").exists(),
        "T-010 AC-007 keep-assertion (i): vsdd-context-resolvers.wasm must be present \
         in staged artifact/ — hyphen-named, no underscore, must not be excluded \
         by the *_*.wasm glob (EC-003)"
    );

    // Glob-semantics proof: some_new_stub_lib.wasm has underscore but is NOT named in
    // the legacy inner denylist (vsdd_context_resolvers.wasm|wasm_resolver_export.wasm).
    // Must NOT appear in artifact/ — proving the outer *_*.wasm glob governs all
    // underscore-named WASMs, not merely those enumerated in the inner case arms.
    // A hardcoded-denylist implementation would erroneously copy this file (test would fail).
    assert!(
        !artifact.join("some_new_stub_lib.wasm").exists(),
        "T-010 AC-007 glob-semantics proof: some_new_stub_lib.wasm must NOT be in \
         staged artifact/ — the outer *_*.wasm underscore glob must skip ALL \
         underscore-named WASMs, not only the two in the inner legacy denylist; \
         a hardcoded-denylist implementation would incorrectly copy this file"
    );

    // AC-007 keep-assertion (ii): real resolvers-registry.toml must still reference
    // vsdd-context-resolvers.wasm (registry reference must remain intact).
    let resolvers_reg_content = fs::read_to_string(&resolvers_reg)
        .expect("resolvers-registry.toml must be readable for keep-assertion (ii)");
    assert!(
        resolvers_reg_content.contains("hook-plugins/vsdd-context-resolvers.wasm"),
        "T-010 AC-007 keep-assertion (ii): resolvers-registry.toml must contain \
         'hook-plugins/vsdd-context-resolvers.wasm' — registry reference must remain intact"
    );
}
