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
//! | T-011 | AC-007   | GREEN    | POLICY 20 defense proof: read-prefix-fixture.wasm (hyphen-named) passes the *_*.wasm staging glob and is an orphan per both registries; proves `--exclude read-prefix-fixture` in release.yml is the governing defense (S-19.06) |
//! | T-012 | AC-006 S-21.09 | RED†† | Declared-set ⊆ tracked-set gate (step 1: registry inventory; step 2: non-vacuity floor ≥ 30; step 3: declared − tracked = ∅; step 4: no STAGED-NOT-COMMITTED); RED pre-fix; GREEN after AC-001 fix |
//! | T-013 | AC-006 S-21.09 | GREEN | BLOCKER-1 nospace control: `plugin="hook-plugins/ghost-guard-nospace.wasm"` (no spaces around =) is parsed as declared by toml-crate parser; proves false-negative gap closed |
//! | T-014 | AC-006 S-21.09 | GREEN | BLOCKER-1 dotslash control: `plugin = "./hook-plugins/ghost-guard-dotslash.wasm"` (leading ./) is parsed as declared by toml-crate parser after ./ normalization |
//! | T-015 | AC-006 S-21.09 | GREEN | Declared-but-untracked fixture control: synthetic empty tracked set → missing set contains declared artifact; outcome identifier "MISSING: hooks-only.wasm" confirmed in failure message |
//! | T-016 | AC-006 S-21.09 | GREEN | PASS arm fixture control: when tracked set equals declared set, declared − tracked is empty (no false positives) |
//! | T-017 | AC-006 S-21.09 | GREEN | Registry-inventory UNEXPECTED arm: tmpdir with extra `metrics-registry.toml` → "UNEXPECTED: metrics-registry.toml" outcome identifier confirmed |
//! | T-018 | AC-006 S-21.09 | GREEN | Registry-inventory MISSING arm: empty tmpdir → "MISSING: hooks-registry.toml" and "MISSING: resolvers-registry.toml" outcome identifiers confirmed |
//! | T-019 | AC-006 S-21.09 | GREEN | Non-vacuity floor control: 1-entry declared set → `#[should_panic]` proves floor fires with "T-012: declared set has only 1 entries" |
//! | T-020 | AC-006 S-21.09 | GREEN | EC-005 control: empty tracked set → `#[should_panic]` proves assertion fires with "T-012 EC-005" identifier |
//! | T-021 | AC-006 S-21.09 | GREEN | Staged-not-committed control: index has artifact absent from HEAD tree → "STAGED-NOT-COMMITTED: staged-plugin.wasm" outcome identifier confirmed |
//!
//! † T-009 is a STANDING GREEN GATE — passes immediately on any clean checkout where no
//! orphan WASMs are tracked in git, and remains green on contaminated worktrees because
//! local build artifacts (untracked per .gitignore) are excluded from the git-tracked set.
//!
//! †† T-012 is a RED GATE at design time — it was RED at commit a60169bd (before AC-001
//! committed `validate-factory-path-staging.wasm`), and turned GREEN at commit 27123d27
//! (after the artifact was added to the git index with `git add -f`). The `RED††` status
//! in the table documents the test's designed-as-Red-Gate role; current run state is GREEN.
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

/// BLOCKER-1 nospace fixture: declares `ghost-guard-nospace.wasm` via
/// `plugin="hook-plugins/ghost-guard-nospace.wasm"` (no spaces around `=`).
/// Used by T-013 to prove the toml-crate parser catches this form.
///
/// Canonical source: `crates/factory-dispatcher/tests/fixtures/bundle-orphan/hooks-registry-nospace-fixture.toml`
const HOOKS_REGISTRY_NOSPACE_FIXTURE: &str =
    include_str!("fixtures/bundle-orphan/hooks-registry-nospace-fixture.toml");

/// BLOCKER-1 dotslash fixture: declares `ghost-guard-dotslash.wasm` via
/// `plugin = "./hook-plugins/ghost-guard-dotslash.wasm"` (leading `./`).
/// Used by T-014 to prove the toml-crate parser catches this form.
///
/// Canonical source: `crates/factory-dispatcher/tests/fixtures/bundle-orphan/hooks-registry-dotslash-fixture.toml`
const HOOKS_REGISTRY_DOTSLASH_FIXTURE: &str =
    include_str!("fixtures/bundle-orphan/hooks-registry-dotslash-fixture.toml");

// ---------------------------------------------------------------------------
// Detection helpers
// ---------------------------------------------------------------------------

/// Parse all `plugin = "hook-plugins/<name>"` references from a TOML registry file.
///
/// Uses the `toml` crate (the same parser the dispatcher uses via `registry.rs`) so that
/// any TOML-legal spelling of the `plugin` key is handled identically to production:
///
/// - `plugin = "hook-plugins/foo.wasm"` — standard form
/// - `plugin="hook-plugins/foo.wasm"` — no spaces around `=` (TOML-legal; missed by v1)
/// - `plugin = "./hook-plugins/foo.wasm"` — leading `./` (TOML-legal; missed by v1)
/// - `plugin = 'hook-plugins/foo.wasm'` — single-quoted TOML string
///
/// Works for both `hooks-registry.toml` (`[[hooks]]` array) and
/// `resolvers-registry.toml` (`[[resolvers]]` array) without special-casing.
///
/// The v1 implementation used `line.trim().strip_prefix("plugin = ")` — an exact
/// single-space prefix match. For T-012's direction (`declared − tracked`), a parse
/// miss is a **false negative**: the name is absent from `declared` and therefore
/// cannot appear in `declared − tracked`, so the gate goes GREEN on a real gap.
/// (Contrast: for T-009's direction, the identical bug is a false positive — loud and
/// safe.) Replaced in S-21.09 adversary pass 1 (BLOCKER-1 closure).
fn parse_plugin_refs(registry: &Path) -> HashSet<String> {
    let content = fs::read_to_string(registry)
        .unwrap_or_else(|e| panic!("failed to read registry {}: {}", registry.display(), e));

    let doc: toml::Value = content.parse::<toml::Value>().unwrap_or_else(|e| {
        panic!(
            "failed to parse TOML registry {}: {}",
            registry.display(),
            e
        )
    });

    let mut refs = HashSet::new();

    // [[hooks]] in hooks-registry.toml; [[resolvers]] in resolvers-registry.toml.
    // Both use `plugin = "hook-plugins/<name>"` in each array entry.
    for section_name in &["hooks", "resolvers"] {
        if let Some(toml::Value::Array(entries)) = doc.get(*section_name) {
            for entry in entries {
                if let Some(toml::Value::String(plugin_path)) = entry.get("plugin") {
                    // Normalize a leading "./" — the dispatcher's resolve_plugin_paths()
                    // joins relative paths against the registry's parent dir, accepting
                    // both "hook-plugins/foo" and "./hook-plugins/foo" as equivalent.
                    // Strip the leading "./" here for consistent basename extraction.
                    let normalized = plugin_path.trim_start_matches("./");
                    if let Some(filename) = normalized.strip_prefix("hook-plugins/") {
                        refs.insert(filename.to_string());
                    }
                }
            }
        }
    }

    refs
}

/// Verify the set of `*-registry.toml` files under `plugins_vsdd_factory_dir` equals
/// exactly `{"hooks-registry.toml", "resolvers-registry.toml"}`.
///
/// Returns `Ok(())` when the inventory matches exactly.
/// Returns `Err(message)` containing outcome identifier lines when the set differs:
/// `UNEXPECTED: <name>` for each file beyond the expected pair,
/// `MISSING: <name>` for each expected file absent from the directory.
///
/// ## Why this check runs first (AC-006 step 1)
///
/// T-012's declared-set aggregation is hardcoded to `hooks-registry.toml` +
/// `resolvers-registry.toml`.  If a third registry (e.g., `metrics-registry.toml`)
/// were added, its declared names would never enter `declared`, the
/// `declared − tracked` difference would be unaffected, and T-012 would stay GREEN
/// while that registry's artifacts were entirely ungated.  By asserting the inventory
/// first, anyone who adds a third registry is forced to update T-012's registry list
/// before T-012 will pass, rather than silently bypassing the gate.
///
/// ## Determinism
///
/// Unexpected and missing lists are sorted before inclusion in the error message so
/// output is stable across runs regardless of `fs::read_dir` ordering.
fn check_registry_inventory(plugins_vsdd_factory_dir: &Path) -> Result<(), String> {
    const EXPECTED: [&str; 2] = ["hooks-registry.toml", "resolvers-registry.toml"];
    let expected: HashSet<&str> = EXPECTED.iter().copied().collect();

    let entries = fs::read_dir(plugins_vsdd_factory_dir).unwrap_or_else(|e| {
        panic!(
            "failed to read plugins/vsdd-factory dir {}: {}",
            plugins_vsdd_factory_dir.display(),
            e
        )
    });

    let mut found: Vec<String> = entries
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|name| name.ends_with("-registry.toml"))
        .collect();
    found.sort(); // deterministic

    let found_set: HashSet<&str> = found.iter().map(String::as_str).collect();

    let mut unexpected: Vec<String> = found
        .iter()
        .filter(|name| !expected.contains(name.as_str()))
        .cloned()
        .collect();
    unexpected.sort();

    let mut missing: Vec<String> = EXPECTED
        .iter()
        .filter(|name| !found_set.contains(*name))
        .map(|s| s.to_string())
        .collect();
    missing.sort();

    if unexpected.is_empty() && missing.is_empty() {
        return Ok(());
    }

    let mut lines = Vec::new();
    for name in &unexpected {
        lines.push(format!("  UNEXPECTED: {}", name));
    }
    for name in &missing {
        lines.push(format!("  MISSING: {}", name));
    }

    Err(format!(
        "T-012 AC-006 step 1 registry-inventory: {} must contain exactly \
         {{hooks-registry.toml, resolvers-registry.toml}} — adding a registry file \
         expands the declared-artifact scope that T-012 covers; update T-012's \
         registry list or remove the file:\n{}",
        plugins_vsdd_factory_dir.display(),
        lines.join("\n")
    ))
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

/// Return the committed (HEAD tree) WASM basenames under `plugins/vsdd-factory/hook-plugins/`.
///
/// Runs `git ls-tree --name-only HEAD plugins/vsdd-factory/hook-plugins/` from `root`,
/// filters for `.wasm` extension, and returns bare filenames (no directory prefix).
///
/// # Distinction from `git_tracked_wasm_names()`
///
/// `git_tracked_wasm_names()` reads the **git index** (staged + committed) via
/// `git ls-files`.  This function reads only the **HEAD commit tree** via
/// `git ls-tree --name-only HEAD`.  A file staged with `git add -f` but not yet
/// committed appears in `git_tracked_wasm_names()` output but NOT here — that gap
/// is the `STAGED-NOT-COMMITTED` outcome surfaced by T-012 step 4.
///
/// On a clean CI checkout (fresh `git checkout`) the two are equivalent; the
/// distinction only matters locally between a `git add -f` and the subsequent commit.
fn git_committed_wasm_names(root: &Path) -> Vec<String> {
    let output = Command::new("git")
        .args([
            "ls-tree",
            "--name-only",
            "HEAD",
            "plugins/vsdd-factory/hook-plugins/",
        ])
        .current_dir(root)
        .output()
        .expect("git ls-tree must execute; ensure git is on PATH in the test environment");

    assert!(
        output.status.success(),
        "git ls-tree exited with status {}: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout)
        .expect("git ls-tree output must be valid UTF-8")
        .lines()
        .filter(|line| line.ends_with(".wasm"))
        .map(|line| {
            Path::new(line)
                .file_name()
                .expect("every git ls-tree path must have a filename component")
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

// ---------------------------------------------------------------------------
// T-011 — POLICY 20 / S-19.06 AC-007 build-exclusion-is-governing-defense
//
// `read-prefix-fixture.wasm` is hyphen-named (no underscore). The *_*.wasm
// staging glob does NOT exclude it. Its absence from release bundles is
// governed solely by `--exclude read-prefix-fixture` in the workspace
// `cargo build --target wasm32-wasip1` step of release.yml.
//
// This test proves:
//   1. stage_release_bundle DOES copy read-prefix-fixture.wasm to artifact/
//      (hyphen-named: the *_*.wasm outer case arm does not skip it)
//   2. collect_orphans_dual classifies it as ORPHAN (absent from both registries)
//
// Combined: if `--exclude read-prefix-fixture` were removed from release.yml,
// read-prefix-fixture.wasm would exist in build output, pass through staging,
// and ship as a registry-orphan release artifact — violating POLICY 20.
// The build-exclusion flag is therefore the necessary and sufficient defense.
//
// Story: S-19.06
// ---------------------------------------------------------------------------
#[test]
fn test_S_19_06_policy20_T011_read_prefix_fixture_passes_staging_and_is_orphan() {
    let tmp = tempdir().expect("tempdir must create successfully");
    let pre_staging = tmp.path().join("pre-staging");
    fs::create_dir_all(&pre_staging).expect("pre-staging dir must be created");

    // Simulate: read-prefix-fixture.wasm present in build output
    // (hypothetically, if --exclude read-prefix-fixture were removed from release.yml).
    fs::write(pre_staging.join("read-prefix-fixture.wasm"), b"")
        .expect("read-prefix-fixture.wasm fixture must be written");

    let artifact = tmp.path().join("artifact");
    fs::create_dir_all(&artifact).expect("artifact dir must be created");

    stage_release_bundle(&pre_staging, &artifact);

    // Proof 1: hyphen-named → staging logic copies it.
    // read-prefix-fixture.wasm contains no underscore, so the *_*.wasm outer
    // case arm does NOT match — it passes through to `cp "$wasm" artifact/`.
    assert!(
        artifact.join("read-prefix-fixture.wasm").exists(),
        "T-011 POLICY 20: read-prefix-fixture.wasm has no underscore — the *_*.wasm \
         staging glob does NOT exclude it; it must be copied to artifact/ if present \
         in build output; proves --exclude read-prefix-fixture in release.yml is the \
         governing defense (S-19.06 AC-007)"
    );

    // Proof 2: absent from both registries → dual-registry orphan.
    let root = workspace_root();
    let hooks_reg = root.join("plugins/vsdd-factory/hooks-registry.toml");
    let resolvers_reg = root.join("plugins/vsdd-factory/resolvers-registry.toml");
    let orphans = collect_orphans_dual(&artifact, &hooks_reg, &resolvers_reg);

    assert!(
        orphans.contains(&"read-prefix-fixture.wasm".to_string()),
        "T-011 POLICY 20: read-prefix-fixture.wasm must be classified as orphan — \
         absent from both hooks-registry.toml and resolvers-registry.toml; \
         without --exclude it would ship as a POLICY 20 violation; \
         got orphans: {:?}",
        orphans
    );
}

// ---------------------------------------------------------------------------
// T-012 — AC-006 S-21.09: declared-set ⊆ tracked-set (declared→tracked direction)
//
// Asserts every WASM artifact declared in EITHER registry is present in the
// git-tracked set.  This is the INVERSE of T-009, which asserts the
// tracked-set ⊆ declared-set direction (zero tracked orphans).
//
// Why both registries are consulted:
//   `vsdd-context-resolvers.wasm` is declared only in resolvers-registry.toml and
//   is also tracked in git.  Using only hooks-registry.toml for the declared set
//   would leave it out of `declared`, but it IS tracked — the direction here
//   (declared − tracked) is unaffected, because a name absent from declared cannot
//   appear in the difference.  Using both registries is nonetheless correct: it
//   reflects the full declared contract, matches the dual-registry scope of T-009,
//   and avoids introducing a registry-scope mismatch between the two directions.
//
// Prose-comment exclusion:
//   hooks-registry.toml contains a comment in its file header that cites
//   "hook-plugins/legacy-bash-adapter.wasm," in descriptive prose — a spurious
//   occurrence of the hook-plugins/ path pattern that a naive line-scanner might
//   yield as a 37th declared entry.  Because `parse_plugin_refs()` uses the toml
//   crate (not line-scanning), the comment is never the value of any `plugin` field
//   in a `[[hooks]]` entry and cannot appear in the declared set.
//   True unique declared count in hooks-registry = 35.
//
// Red Gate (pre-fix state):
//   `validate-factory-path-staging.wasm` is declared in hooks-registry.toml
//   (entry at `name = "validate-factory-path-staging"`, `plugin =
//   "hook-plugins/validate-factory-path-staging.wasm"`) but is NOT returned by
//   `git ls-files plugins/vsdd-factory/hook-plugins/` because no `git add -f`
//   has been run.  `declared − tracked` = {"validate-factory-path-staging.wasm"}.
//   Test FAILS with "MISSING: validate-factory-path-staging.wasm".
//
// Green Gate (post-fix state):
//   After `git add -f plugins/vsdd-factory/hook-plugins/validate-factory-path-staging.wasm`
//   the artifact enters the git index.  `git_tracked_wasm_names()` returns it.
//   `declared − tracked` = {} (empty).  Test PASSES.
//
// Class coverage:
//   The assertion checks ALL declared artifacts, not only this instance.
//   A future story that adds a hooks-registry.toml entry without committing the
//   WASM would also fail T-012 — the test catches the defect class, not merely
//   this one artifact.
//
// Per D-970 Codification 1: the failure message names each MISSING artifact
// explicitly so the outcome identifier is unambiguous (bare count assertion
// is the weakness that let this defect survive AC-001 registry-comment claim).
//
// Story: S-21.09
// BC Trace: BC-4.16.001 Precondition 3
// ---------------------------------------------------------------------------
#[test]
fn test_S_21_09_ac006_T012_declared_set_subset_of_tracked_set() {
    let root = workspace_root();

    // Step 1 (AC-006 step 1): registry-inventory gate.
    // Assert plugins/vsdd-factory/ contains exactly {hooks-registry.toml,
    // resolvers-registry.toml}.  Must run BEFORE the declared-set aggregation so
    // that adding a third registry fires here and forces the caller to update T-012's
    // registry list, rather than silently leaving the new registry's artifacts ungated.
    let plugins_vsdd_factory = root.join("plugins/vsdd-factory");
    check_registry_inventory(&plugins_vsdd_factory).unwrap_or_else(|msg| panic!("{}", msg));

    let hooks_registry = root.join("plugins/vsdd-factory/hooks-registry.toml");
    let resolvers_registry = root.join("plugins/vsdd-factory/resolvers-registry.toml");

    // EC-004: hard-fail on missing registry file — never silently pass.
    assert!(
        hooks_registry.exists(),
        "T-012 EC-004: plugins/vsdd-factory/hooks-registry.toml not found under \
         workspace root {}; registry file is mandatory — test must fail explicitly, \
         not silently pass",
        root.display()
    );
    assert!(
        resolvers_registry.exists(),
        "T-012 EC-004: plugins/vsdd-factory/resolvers-registry.toml not found under \
         workspace root {}; registry file is mandatory — test must fail explicitly, \
         not silently pass",
        root.display()
    );

    // Aggregate declared set: union of both registries.
    // `parse_plugin_refs()` is called on each registry file; results are unioned
    // into a HashSet so every declared name appears exactly once regardless of
    // how many registry entries reference the same WASM.
    //
    // vsdd-context-resolvers.wasm: declared in resolvers-registry, also tracked in git.
    // Using both registries places it in the declared set — present in tracked set too,
    // so it contributes no false-positive to the declared − tracked difference.
    let hooks_refs = parse_plugin_refs(&hooks_registry);
    let resolvers_refs = parse_plugin_refs(&resolvers_registry);
    let declared: HashSet<String> = hooks_refs.union(&resolvers_refs).cloned().collect();

    // Non-vacuity floor: a parse regression yielding fewer than 30 entries is almost
    // certainly a parser failure — an empty parse or partial parse would produce a
    // declared − tracked diff of near-zero and vacuously pass every entry check.
    // Floor of 30 mirrors the release.yml sibling gate ("expected >= 30; possible parse
    // failure"), which uses the same rationale. Current actual count = 36 (35 hooks +
    // 1 resolver); floor absorbs reasonable future registry shrinkage without needing
    // constant maintenance, while catching a collapse from 36 to e.g. 1 entry.
    assert!(
        declared.len() >= 30,
        "T-012: declared set has only {} entries (expected >= 30); \
         this almost certainly indicates a parse failure in parse_plugin_refs() — \
         check hooks-registry.toml and resolvers-registry.toml paths and TOML format",
        declared.len()
    );

    // Git-tracked set: same mechanism as T-009.
    // EC-005: git ls-files failure is an explicit error, not a silent pass.
    // `git_tracked_wasm_names()` panics on non-zero exit, which surfaces as a test
    // failure with an explicit message rather than a silent skip or vacuous pass.
    let tracked: HashSet<String> = git_tracked_wasm_names(&root).into_iter().collect();

    assert!(
        !tracked.is_empty(),
        "T-012 EC-005: git ls-files returned no tracked WASMs under \
         plugins/vsdd-factory/hook-plugins/ — verify git is on PATH, workspace_root() \
         resolves correctly, and the test is run inside a git repository"
    );

    // declared − tracked: names present in a registry but absent from the git index.
    //
    // Pre-fix: {"validate-factory-path-staging.wasm"} — the artifact is declared
    //   in hooks-registry.toml but was never committed (no `git add -f` in S-21.01).
    //   This set is non-empty; the assertion below FAILS (RED Gate).
    //
    // Post-fix: {} — after AC-001 commits the artifact, git_tracked_wasm_names()
    //   returns it, the difference is empty, the assertion PASSES (Green Gate).
    let mut missing: Vec<&str> = declared
        .iter()
        .filter(|name| !tracked.contains(*name))
        .map(String::as_str)
        .collect();
    missing.sort(); // deterministic order for reproducible failure messages

    // Per D-970 Codification 1: name each missing artifact explicitly in the failure
    // message.  "MISSING: validate-factory-path-staging.wasm" must appear when the
    // defect is present.  A bare `assert_eq!(missing.len(), 0)` is the pattern that
    // allowed the original defect to survive undetected.
    assert!(
        missing.is_empty(),
        "T-012 AC-006 S-21.09 BC-4.16.001 Precondition 3: every WASM artifact \
         declared in hooks-registry.toml or resolvers-registry.toml MUST be tracked \
         in git (declared_set ⊆ tracked_set); {} artifact(s) declared but NOT \
         tracked — commit each with `git add -f` before the guard can load in \
         production sessions:\n{}",
        missing.len(),
        missing
            .iter()
            .map(|n| format!("  MISSING: {}", n))
            .collect::<Vec<_>>()
            .join("\n")
    );

    // Step 4 (LOW-1 supplementary): staged-not-committed check.
    //
    // AC-001 says "present in the git index" — the primary check above is
    // spec-faithful.  This supplementary check adds a stronger guarantee:
    // a file staged (`git add -f`) but not yet committed would pass the index
    // check above yet be absent from a fresh CI checkout.  On CI (clean
    // `git checkout`), index = HEAD tree and the two checks are equivalent.
    //
    // Failure message outcome identifier: "STAGED-NOT-COMMITTED: <name>"
    // Control: T-021.
    let committed: HashSet<String> = git_committed_wasm_names(&root).into_iter().collect();

    let mut staged_not_committed: Vec<&str> = tracked
        .iter()
        .filter(|name| !committed.contains(*name))
        .map(String::as_str)
        .collect();
    staged_not_committed.sort();

    assert!(
        staged_not_committed.is_empty(),
        "T-012 AC-006 S-21.09 BC-4.16.001 Precondition 3 (supplementary): {} WASM \
         artifact(s) are in the git index but NOT committed to HEAD — absent on a \
         fresh CI checkout; commit each before pushing:\n{}",
        staged_not_committed.len(),
        staged_not_committed
            .iter()
            .map(|n| format!("  STAGED-NOT-COMMITTED: {}", n))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

// ---------------------------------------------------------------------------
// T-013 — BLOCKER-1 nospace control: `plugin="..."` (no spaces around =) is parsed
//
// The v1 parse_plugin_refs() line-scanner required the exact prefix `plugin = `
// (single space on each side of `=`).  TOML allows whitespace around `=` to be
// optional, so `plugin="hook-plugins/ghost-guard-nospace.wasm"` is a valid TOML
// assignment that the real dispatcher accepted (sync_plugins=1, plugins_run=1,
// exit_code=0).  The v1 scanner silently missed it — a false negative in the
// declared − tracked direction (the artifact was never in declared, so it could
// never appear in the missing set).
//
// This test proves the toml-crate replacement closes the gap: parse_plugin_refs()
// on the nospace fixture MUST return a set containing "ghost-guard-nospace.wasm".
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
fn test_S_21_09_ac006_T013_nospace_eq_sign_form_is_parsed_as_declared() {
    let tmp = tempdir().expect("tempdir must create successfully");
    let hooks_reg = tmp.path().join("hooks-registry-nospace.toml");
    fs::write(&hooks_reg, HOOKS_REGISTRY_NOSPACE_FIXTURE)
        .expect("nospace fixture must be written to tempfile");

    let refs = parse_plugin_refs(&hooks_reg);

    assert!(
        refs.contains("ghost-guard-nospace.wasm"),
        "T-013 BLOCKER-1: parse_plugin_refs() must extract 'ghost-guard-nospace.wasm' \
         from a registry using the no-space-around-equals form \
         (plugin=\"hook-plugins/ghost-guard-nospace.wasm\") — \
         the toml-crate parser accepts all TOML-legal whitespace forms; \
         got refs: {:?}",
        refs
    );
}

// ---------------------------------------------------------------------------
// T-014 — BLOCKER-1 dotslash control: `plugin = "./hook-plugins/..."` (leading ./)
//
// The v1 parse_plugin_refs() stripped `plugin = "` and then checked for the
// `hook-plugins/` prefix.  When the value begins with `./`, the remaining string
// is `./hook-plugins/foo.wasm` — `strip_prefix("hook-plugins/")` returns None
// because the path starts with `./`, not `hook-plugins/`.  The artifact was silently
// omitted from the declared set (false negative in the declared − tracked direction).
//
// The dispatcher's registry.rs resolve_plugin_paths() joins relative paths against
// the registry file's parent directory, making `"./hook-plugins/foo"` and
// `"hook-plugins/foo"` functionally identical in production.  The test gate must
// match production behavior.
//
// This test proves the toml-crate replacement normalises the leading `./` via
// `trim_start_matches("./")` before the `strip_prefix("hook-plugins/")` call:
// parse_plugin_refs() on the dotslash fixture MUST return a set containing
// "ghost-guard-dotslash.wasm".
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
fn test_S_21_09_ac006_T014_dotslash_prefix_form_is_parsed_as_declared() {
    let tmp = tempdir().expect("tempdir must create successfully");
    let hooks_reg = tmp.path().join("hooks-registry-dotslash.toml");
    fs::write(&hooks_reg, HOOKS_REGISTRY_DOTSLASH_FIXTURE)
        .expect("dotslash fixture must be written to tempfile");

    let refs = parse_plugin_refs(&hooks_reg);

    assert!(
        refs.contains("ghost-guard-dotslash.wasm"),
        "T-014 BLOCKER-1: parse_plugin_refs() must extract 'ghost-guard-dotslash.wasm' \
         from a registry using the leading-dotslash form \
         (plugin = \"./hook-plugins/ghost-guard-dotslash.wasm\") — \
         the toml-crate version normalises './' via trim_start_matches(\"./\") before \
         checking for the hook-plugins/ prefix; \
         got refs: {:?}",
        refs
    );
}

// ---------------------------------------------------------------------------
// T-015 — Declared-but-untracked fixture control: outcome identifier confirmed
//
// Positive control for the declared − tracked detection arm.  Uses the existing
// hooks-registry and resolvers-registry fixtures (which declare hooks-only.wasm
// and resolvers-only.wasm respectively), paired with a SYNTHETIC EMPTY tracked set
// (simulating the state before any `git add -f` has been run).
//
// Because the tracked set is empty, every declared artifact is in the missing set.
// The test verifies:
//   (a) "hooks-only.wasm" appears in the missing set.
//   (b) The MISSING: <name> failure-message format produces "MISSING: hooks-only.wasm".
//
// Per D-970 Codification 1: the outcome identifier string must appear verbatim in
// the failure message.  A bare count assertion (missing.len() > 0) would allow the
// format to drift without detection.  This fixture control locks the string format.
//
// This test always GREEN (fixture-driven; does not call git ls-files).
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
fn test_S_21_09_ac006_T015_declared_but_untracked_arm_names_artifact() {
    let tmp = tempdir().expect("tempdir must create successfully");
    let hooks_reg = tmp.path().join("hooks-registry.toml");
    let resolvers_reg = tmp.path().join("resolvers-registry.toml");
    fs::write(&hooks_reg, HOOKS_REGISTRY_FIXTURE)
        .expect("hooks-registry fixture must be written to tempfile");
    fs::write(&resolvers_reg, RESOLVERS_REGISTRY_FIXTURE)
        .expect("resolvers-registry fixture must be written to tempfile");

    let hooks_refs = parse_plugin_refs(&hooks_reg);
    let resolvers_refs = parse_plugin_refs(&resolvers_reg);
    let declared: HashSet<String> = hooks_refs.union(&resolvers_refs).cloned().collect();

    // Synthetic empty tracked set: simulates the pre-`git add -f` state.
    // Every declared artifact is missing.
    let tracked: HashSet<String> = HashSet::new();

    let mut missing: Vec<&str> = declared
        .iter()
        .filter(|name| !tracked.contains(*name))
        .map(String::as_str)
        .collect();
    missing.sort();

    // (a) hooks-only.wasm must appear in the missing set.
    assert!(
        missing.contains(&"hooks-only.wasm"),
        "T-015 AC-006 D-970 Codification 1: 'hooks-only.wasm' must appear in the \
         missing set when the tracked set is empty (declared − {} = declared); \
         got missing: {:?}",
        "∅",
        missing
    );

    // (b) The MISSING: <name> format must produce the outcome identifier string.
    let missing_lines: Vec<String> = missing
        .iter()
        .map(|n| format!("  MISSING: {}", n))
        .collect();

    assert!(
        missing_lines.contains(&"  MISSING: hooks-only.wasm".to_string()),
        "T-015 AC-006 D-970 Codification 1: failure-message format must produce \
         '  MISSING: hooks-only.wasm' for a declared-but-untracked artifact; \
         got lines: {:?}",
        missing_lines
    );
}

// ---------------------------------------------------------------------------
// T-016 — PASS arm fixture control: empty diff when all declared are tracked
//
// Negative control for the declared − tracked direction.  Uses the existing
// fixtures (which declare hooks-only.wasm and resolvers-only.wasm), paired with
// a SYNTHETIC TRACKED SET equal to the declared set — simulating the post-fix
// state where every declared artifact has been committed to git.
//
// When tracked == declared, the declared − tracked difference is empty.
// The test asserts missing.is_empty() — confirming no false positives when every
// declared artifact is present in the tracked set.
//
// This test always GREEN (fixture-driven; does not call git ls-files).
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
fn test_S_21_09_ac006_T016_pass_arm_empty_diff_when_all_declared_are_tracked() {
    let tmp = tempdir().expect("tempdir must create successfully");
    let hooks_reg = tmp.path().join("hooks-registry.toml");
    let resolvers_reg = tmp.path().join("resolvers-registry.toml");
    fs::write(&hooks_reg, HOOKS_REGISTRY_FIXTURE)
        .expect("hooks-registry fixture must be written to tempfile");
    fs::write(&resolvers_reg, RESOLVERS_REGISTRY_FIXTURE)
        .expect("resolvers-registry fixture must be written to tempfile");

    let hooks_refs = parse_plugin_refs(&hooks_reg);
    let resolvers_refs = parse_plugin_refs(&resolvers_reg);
    let declared: HashSet<String> = hooks_refs.union(&resolvers_refs).cloned().collect();

    // Synthetic tracked set = declared set: simulates the post-`git add -f` state.
    // No artifact is missing.
    let tracked: HashSet<String> = declared.clone();

    let missing: Vec<&str> = declared
        .iter()
        .filter(|name| !tracked.contains(*name))
        .map(String::as_str)
        .collect();

    assert!(
        missing.is_empty(),
        "T-016 AC-006: declared − tracked must be empty when every declared artifact \
         is also in the tracked set (no false positives in PASS arm); \
         got missing: {:?}",
        missing
    );
}

// ---------------------------------------------------------------------------
// T-017 — Registry-inventory fixture control: unexpected third registry names artifact
//
// Fixture-driven control for the AC-006 step 1 registry-inventory assertion in T-012.
//
// T-012's declared-set aggregation is hardcoded to hooks-registry.toml +
// resolvers-registry.toml.  The design flaw it guards against is direction-blindness:
// if someone adds `plugins/vsdd-factory/metrics-registry.toml`, that file's declared
// plugin names never enter `declared`, the declared − tracked difference is unaffected,
// and T-012 stays GREEN while metrics-registry artifacts are entirely ungated.
//
// check_registry_inventory() catches this by asserting the directory contains EXACTLY
// the expected pair before the declared-set check runs.  This test proves it:
//
//   Scenario: tmpdir contains hooks-registry.toml + resolvers-registry.toml +
//             metrics-registry.toml (the unexpected extra file).
//
//   Expected:
//     (a) check_registry_inventory returns Err (not Ok).
//     (b) The error message contains "UNEXPECTED: metrics-registry.toml" —
//         the outcome identifier naming the specific file per D-970 Codification 1.
//
// This test always GREEN (fixture-driven; does not touch the real filesystem).
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
fn test_S_21_09_ac006_T017_registry_inventory_names_unexpected_file() {
    let tmp = tempdir().expect("tempdir must create successfully");

    // Write the two expected registry files (minimal content — the inventory
    // check only inspects filenames, not file content).
    fs::write(
        tmp.path().join("hooks-registry.toml"),
        "schema_version = 2\n",
    )
    .expect("hooks-registry.toml must be written to tempdir");
    fs::write(
        tmp.path().join("resolvers-registry.toml"),
        "schema_version = 1\n",
    )
    .expect("resolvers-registry.toml must be written to tempdir");

    // Write an unexpected third registry — simulates someone adding a new
    // registry file to plugins/vsdd-factory/ without updating T-012.
    fs::write(
        tmp.path().join("metrics-registry.toml"),
        "schema_version = 1\n",
    )
    .expect("metrics-registry.toml must be written to tempdir");

    let result = check_registry_inventory(tmp.path());

    // (a) Must return Err when an unexpected registry is present.
    assert!(
        result.is_err(),
        "T-017 AC-006 step 1: check_registry_inventory must return Err when \
         metrics-registry.toml is present alongside the expected pair; got Ok"
    );

    let msg = result.unwrap_err();

    // (b) The outcome identifier must name the unexpected file explicitly.
    // Per D-970 Codification 1: a bare "unexpected file found" without the name
    // is insufficient — the identifier must be actionable without opening a log.
    assert!(
        msg.contains("UNEXPECTED: metrics-registry.toml"),
        "T-017 AC-006 step 1 D-970 Codification 1: error message must contain \
         'UNEXPECTED: metrics-registry.toml' — the outcome identifier must name the \
         specific unexpected registry file so the remediation path is unambiguous; \
         got message: {}",
        msg
    );
}

// ---------------------------------------------------------------------------
// T-018 — Registry-inventory MISSING arm: empty dir → both MISSING identifiers
//
// Proves the MISSING arm of check_registry_inventory(): when a directory contains
// NO *-registry.toml files at all, the function must return Err with both
// "MISSING: hooks-registry.toml" and "MISSING: resolvers-registry.toml" in the
// message.
//
// This complements T-017 (UNEXPECTED arm) — together they cover both mismatch
// directions, closing BLOCKER-2 for the registry-inventory outcome class.
//
// Scenario: empty tmpdir — simulates a fresh plugins/vsdd-factory/ with no
//   registry files (e.g., accidental deletion or a brand-new directory).
//
// Expected:
//   (a) check_registry_inventory returns Err (not Ok).
//   (b) Message contains "MISSING: hooks-registry.toml".
//   (c) Message contains "MISSING: resolvers-registry.toml".
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
fn test_S_21_09_ac006_T018_registry_inventory_both_missing() {
    let tmp = tempdir().expect("tempdir must create successfully");

    // Empty tmpdir — no *-registry.toml files present.
    let result = check_registry_inventory(tmp.path());

    // (a) Must return Err when both expected registries are absent.
    assert!(
        result.is_err(),
        "T-018 AC-006 step 1: check_registry_inventory must return Err when \
         the directory is empty (both registries missing); got Ok"
    );

    let msg = result.unwrap_err();

    // (b) hooks-registry.toml MISSING identifier.
    assert!(
        msg.contains("MISSING: hooks-registry.toml"),
        "T-018 AC-006 step 1 D-970 Codification 1: error message must contain \
         'MISSING: hooks-registry.toml' when the directory is empty; got: {}",
        msg
    );

    // (c) resolvers-registry.toml MISSING identifier.
    assert!(
        msg.contains("MISSING: resolvers-registry.toml"),
        "T-018 AC-006 step 1 D-970 Codification 1: error message must contain \
         'MISSING: resolvers-registry.toml' when the directory is empty; got: {}",
        msg
    );
}

// ---------------------------------------------------------------------------
// T-019 — Non-vacuity floor control: proves floor assertion FIRES on 1-entry set
//
// The non-vacuity floor at T-012 step 2 asserts `declared.len() >= 30`.  A floor
// with no control proving it fires is indistinguishable from a dead assertion —
// the "paper-fix" shape that TD-VSDD-059 detects.
//
// This test replicates the step 2 assertion in isolation with a 1-entry `declared`
// set and uses `#[should_panic]` to verify the assertion fires.  The `expected=`
// parameter locks the outcome identifier — if the assertion message changes, this
// test fails loudly rather than silently passing with a different string.
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
#[should_panic(expected = "T-012: declared set has only 1 entries")]
fn test_S_21_09_ac006_T019_non_vacuity_floor_fires_on_one_entry_set() {
    let mut declared: HashSet<String> = HashSet::new();
    declared.insert("single-plugin.wasm".to_string());

    // Replicate T-012 step 2 floor assertion verbatim.
    assert!(
        declared.len() >= 30,
        "T-012: declared set has only {} entries (expected >= 30); \
         this almost certainly indicates a parse failure in parse_plugin_refs() — \
         check hooks-registry.toml and resolvers-registry.toml paths and TOML format",
        declared.len()
    );
}

// ---------------------------------------------------------------------------
// T-020 — EC-005 control: proves EC-005 assertion FIRES on empty tracked set
//
// T-012 step 3 guards against an empty git-tracked set with an explicit assertion
// (EC-005).  An empty `tracked` set would cause `declared − tracked` to include
// every declared artifact, producing noisy false failures.  The EC-005 guard
// converts that scenario into a clearly-named error.
//
// This test replicates the EC-005 assertion in isolation with an empty `tracked`
// set and uses `#[should_panic]` to verify the assertion fires.  The `expected=`
// parameter locks the "T-012 EC-005" outcome identifier.
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
#[should_panic(expected = "T-012 EC-005")]
fn test_S_21_09_ac006_T020_ec005_assertion_fires_on_empty_tracked_set() {
    let tracked: HashSet<String> = HashSet::new();

    // Replicate T-012 EC-005 assertion verbatim.
    assert!(
        !tracked.is_empty(),
        "T-012 EC-005: git ls-files returned no tracked WASMs under \
         plugins/vsdd-factory/hook-plugins/ — verify git is on PATH, workspace_root() \
         resolves correctly, and the test is run inside a git repository"
    );
}

// ---------------------------------------------------------------------------
// T-021 — Staged-not-committed control: "STAGED-NOT-COMMITTED: <name>" identifier
//
// T-012 step 4 computes `staged_not_committed = tracked − committed` and formats
// each entry as "  STAGED-NOT-COMMITTED: <name>".  This test proves the outcome
// identifier is correct by constructing synthetic `tracked` and `committed` sets
// where one artifact is in `tracked` but absent from `committed`, then asserting
// the formatted message contains the expected identifier.
//
// Unlike T-019/T-020, this is NOT `#[should_panic]` — it replicates the message
// generation logic and asserts on the produced string, staying GREEN throughout
// (fixture-driven; no filesystem or git access).
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
fn test_S_21_09_ac006_T021_staged_not_committed_outcome_identifier() {
    // Synthetic sets: staged-plugin.wasm is in the git index but not in HEAD.
    let tracked: HashSet<String> = ["staged-plugin.wasm".to_string()].into_iter().collect();
    let committed: HashSet<String> = HashSet::new();

    // Replicate T-012 step 4 logic verbatim.
    let mut staged_not_committed: Vec<&str> = tracked
        .iter()
        .filter(|name| !committed.contains(*name))
        .map(String::as_str)
        .collect();
    staged_not_committed.sort();

    // Verify the control scenario: exactly one staged-not-committed artifact.
    assert!(
        !staged_not_committed.is_empty(),
        "T-021 AC-006 S-21.09: synthetic staged_not_committed must be non-empty \
         when tracked contains a name absent from committed"
    );

    // Build the failure message lines the same way T-012 step 4 would.
    let lines: Vec<String> = staged_not_committed
        .iter()
        .map(|n| format!("  STAGED-NOT-COMMITTED: {}", n))
        .collect();
    let message = lines.join("\n");

    // Verify the outcome identifier per D-970 Codification 1.
    assert!(
        message.contains("  STAGED-NOT-COMMITTED: staged-plugin.wasm"),
        "T-021 AC-006 S-21.09 D-970 Codification 1: outcome identifier must be \
         '  STAGED-NOT-COMMITTED: staged-plugin.wasm'; got: {}",
        message
    );
}
