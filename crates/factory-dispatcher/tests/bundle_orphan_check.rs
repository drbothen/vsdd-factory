// Test files use .expect()/.unwrap()/.panic!() for failure reporting.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
//! AC-006 + AC-007 Rust workspace tests for S-19.04 + S-21.09 bundle hygiene and
//! declared-set gate stories.
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
//! | T-012 | AC-006 S-21.09 | GREEN (Red-Gate-by-design) | Declared-set ⊆ tracked-set gate via `check_declared_subset_tracked()`; step 1: registry inventory; step 2: per-registry floors (hooks ≥ 30, resolvers ≥ 1); step 3: declared − tracked = ∅; step 4: no STAGED-NOT-COMMITTED |
//! | T-013 | AC-006 S-21.09 | GREEN | BLOCKER-1 nospace control: `plugin="hook-plugins/ghost-guard-nospace.wasm"` (no spaces around =) is parsed as declared by toml-crate parser; proves false-negative gap closed |
//! | T-014 | AC-006 S-21.09 | GREEN | BLOCKER-1 dotslash control: `plugin = "./hook-plugins/ghost-guard-dotslash.wasm"` (leading ./) is parsed as declared by toml-crate parser after ./ normalization |
//! | T-015 | AC-006 S-21.09 | GREEN | Declared-but-untracked: calls `check_declared_subset_tracked()` with synthetic fixtures (30 hooks + 1 resolver, missing one); asserts "MISSING: hooks-only.wasm" identifier |
//! | T-016 | AC-006 S-21.09 | GREEN | PASS arm: calls `check_declared_subset_tracked()` with all declared tracked → Ok (no false positives) |
//! | T-017 | AC-006 S-21.09 | GREEN | Registry-inventory UNEXPECTED arm (hyphen form): tmpdir + `metrics-registry.toml` → "UNEXPECTED: metrics-registry.toml" outcome identifier confirmed |
//! | T-018 | AC-006 S-21.09 | GREEN | Registry-inventory MISSING arm: empty tmpdir → "MISSING: hooks-registry.toml" and "MISSING: resolvers-registry.toml" identifiers confirmed |
//! | T-019 | AC-006 S-21.09 | GREEN | Hooks floor control: calls `check_declared_subset_tracked()` with 1-entry hooks; `#[should_panic]` locks "T-012: hooks registry declared set has only 1 entries" |
//! | T-020 | AC-006 S-21.09 | GREEN | EC-005 control: calls `check_declared_subset_tracked()` with empty tracked; `#[should_panic]` locks "T-012 EC-005" identifier |
//! | T-021 | AC-006 S-21.09 | GREEN | Staged-not-committed: calls `check_declared_subset_tracked()` with staged artifact → Err containing "STAGED-NOT-COMMITTED: staged-plugin.wasm" identifier |
//! | T-022 | AC-006 S-21.09 | GREEN | Resolvers floor control: calls `check_declared_subset_tracked()` with empty resolvers; `#[should_panic]` locks "T-012: resolvers registry declared set is empty" |
//! | T-023 | AC-006 S-21.09 | GREEN | MEDIUM-1 boundary polarity (corrected pass 4): bare plugin path `ghost-bare.wasm` (no `hook-plugins` component after normalization) → excluded from declared; traversal/absolute forms now INCLUDED via lexical normalisation |
//! | T-024 | AC-006 S-21.09 | GREEN | BLOCKER-2 underscore mutant: `metrics_registry.toml` (underscore, previously missed by `-registry.toml` filter) caught by fail-closed `*.toml` inventory → "UNEXPECTED: metrics_registry.toml" |
//! | T-025 | AC-006 S-21.09 | GREEN | F-1 traversal control: `plugin = "hooks/../hook-plugins/ghost-traversal.wasm"` is parsed as declared (resolves `..` relative to registry parent, lands inside `hook-plugins/`) |
//! | T-026 | AC-006 S-21.09 | GREEN | MEDIUM-2 (revised): absolute-form plugin `/abs/hook-plugins/ghost-absolute.wasm` is EXCLUDED — production passes absolute paths unchanged; artifact lives outside repo |
//! | T-027 | AC-006 S-21.09 | GREEN | F-2 floor boundary (29 fires): 29-entry hooks set fires hooks floor — `#[should_panic]` locks "T-012: hooks registry declared set has only 29 entries"; pins the threshold so mutating `< 30` to `< 2` is caught |
//! | T-028 | AC-006 S-21.09 | GREEN | F-3a narrowing proof (non-recursive, SAFE): subdirectory `config/hooks-registry.toml` invisible to `fs::read_dir`; safe because production only loads top-level registries |
//! | T-029 | AC-006 S-21.09 | GREEN | F-3b narrowing proof (case-sensitive): `metrics-registry.TOML` (uppercase) invisible to `.ends_with(".toml")`; safe because production loads lowercase-named registries |
//! | T-030 | AC-006 S-21.09 | GREEN | F-9 wiring control: `run_t012_gate` integrates both `check_registry_inventory` (phase A) and `check_declared_subset_tracked` (phase B via git fixture); removing either call breaks a phase |
//! | T-031 | AC-006 S-21.09 | GREEN | MEDIUM-4(a) case-variant control: `plugin = "Hook-Plugins/foo.wasm"` is parsed as declared (case-insensitive `hook-plugins` component match) |
//! | T-032 | AC-006 S-21.09 | GREEN | MEDIUM-1 nested-subdir control: `plugin = "hook-plugins/sub/nested.wasm"` yields `nested.wasm` (last component); proves non-flat declarations are not silently mis-named |
//!
//! † T-009 is a STANDING GREEN GATE — passes immediately on any clean checkout where no
//! orphan WASMs are tracked in git, and remains green on contaminated worktrees because
//! local build artifacts (untracked per .gitignore) are excluded from the git-tracked set.
//!
//! †† T-012 is a RED GATE at design time — before the S-21.09 AC-001 fix committed
//! `validate-factory-path-staging.wasm`, `declared − tracked` was non-empty and the test
//! failed; after the artifact was committed, `declared − tracked` became empty and the test
//! turned GREEN. The `GREEN (Red-Gate-by-design)` status documents the test's
//! designed-as-Red-Gate role while making the current GREEN run state unambiguous.
//!
//! ## Hermetic Design for T-009
//!
//! `plugins/vsdd-factory/hook-plugins/` is listed in `.gitignore`. Running
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
//! Stories: S-19.04 (T-006..T-011), S-21.09 (T-012..T-032)
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

/// Resolve `plugin_path` relative to the registry file's parent directory and
/// return the bare WASM filename if the result lands inside the sibling
/// `hook-plugins/` directory.
///
/// This function mirrors the production dispatcher's `resolve_plugin_paths()` semantics:
/// relative paths are joined to the registry parent directory; absolute paths replace the
/// base entirely (Rust `Path::join` / POSIX resolution — confirmed by
/// `resolve_plugin_paths_is_idempotent_for_absolute_paths` in `registry.rs` tests).
/// After joining, the path is lexically normalised (`..` pops, `.` skips, root prefix
/// clears the accumulator), and the result is checked for a `hook-plugins` component
/// case-insensitively immediately after the registry-parent prefix.
///
/// | Input form | Resolution | Result |
/// |-----------|------------|--------|
/// | Standard | `registry_parent/hook-plugins/foo.wasm` | `foo.wasm` |
/// | Leading `./` | `registry_parent/hook-plugins/foo.wasm` | `foo.wasm` |
/// | Traversal (into hook-plugins) | `registry_parent/hook-plugins/foo.wasm` | `foo.wasm` |
/// | Case variant | `registry_parent/Hook-Plugins/foo.wasm` | `foo.wasm` |
/// | Nested subdir | `registry_parent/hook-plugins/sub/nested.wasm` | `nested.wasm` |
/// | Absolute path | `/abs/hook-plugins/foo.wasm` (stays absolute, ≠ registry_parent) | `None` |
/// | Traversal (cancels hook-plugins) | `registry_parent/ghost.wasm` | `None` |
/// | `../` prefix | `parent(registry_parent)/hook-plugins/foo.wasm` | `None` |
/// | Bare | `registry_parent/ghost-bare.wasm` | `None` |
///
/// **Why absolute paths are excluded:** production's `resolve_plugin_paths()` passes
/// absolute plugin paths through unchanged, so they load from outside the repo.
/// Including them in `declared` would demand git-tracking for a file that lives
/// outside the repository — a false-positive MISSING outcome.  Excluding them is
/// consistent with the gate's scope (artifacts under `plugins/vsdd-factory/hook-plugins/`).
///
/// **Case-insensitive `hook-plugins` match:** on macOS's case-insensitive default
/// filesystem, `Hook-Plugins/x.wasm` and `hook-plugins/x.wasm` are the same directory.
/// Using `eq_ignore_ascii_case` prevents a case-variant declaration from silently
/// escaping the declared-set gate (MEDIUM-4(a)).
///
/// **Last component as filename:** `joined_parts.last()` is returned rather than
/// `joined_parts[hook-plugins_pos + 1]`.  For nested declarations
/// (`hook-plugins/sub/nested.wasm`), `pos + 1` would yield `sub` — not the artifact
/// filename — causing a false MISSING identifier (MEDIUM-1).  `git ls-files` is
/// recursive, so the tracked-set contains filenames; using `last()` keeps parity.
///
/// See T-025 (traversal-into proof), T-026 (absolute exclusion), T-023 (traversal-
/// cancels + bare exclusion), T-031 (case-variant), T-032 (nested-subdir).
fn extract_hook_plugin_name(registry_path: &Path, plugin_path: &str) -> Option<String> {
    use std::path::Component;

    // Lexically normalise `path`: resolve `..` (pop), skip `.`, clear on root prefix.
    fn lex_norm(path: &Path) -> Vec<String> {
        let mut parts: Vec<String> = Vec::new();
        for comp in path.components() {
            match comp {
                Component::Normal(c) => {
                    if let Some(s) = c.to_str() {
                        parts.push(s.to_owned());
                    }
                }
                Component::ParentDir => {
                    parts.pop();
                }
                Component::CurDir => {}
                Component::RootDir | Component::Prefix(_) => {
                    parts.clear();
                }
            }
        }
        parts
    }

    let registry_parent = registry_path.parent()?;

    // Join registry_parent with plugin_path.
    // If plugin_path is absolute, Rust's Path::join replaces the base entirely —
    // matching POSIX resolution semantics and the production dispatcher's
    // resolve_plugin_paths() behaviour (absolute paths pass through unchanged).
    let joined = registry_parent.join(plugin_path);

    let joined_parts = lex_norm(&joined);
    let parent_parts = lex_norm(registry_parent);

    // The resolved path must land at registry_parent/hook-plugins/<...>/filename:
    //   joined_parts == [parent_parts..., "hook-plugins" (case-insensitive), ..., filename]
    // Minimum length: parent_parts.len() + 2 (hook-plugins component + at least one filename).
    let expected_depth = parent_parts.len();
    if joined_parts.len() < expected_depth + 2 {
        return None;
    }

    // Verify the registry_parent prefix matches exactly.
    for (i, p) in parent_parts.iter().enumerate() {
        if joined_parts.get(i)? != p {
            return None;
        }
    }

    // Verify the component immediately after registry_parent is `hook-plugins`.
    // Case-insensitive: handles `Hook-Plugins/x.wasm` on macOS case-insensitive FS.
    let hook_comp = joined_parts.get(expected_depth)?;
    if !hook_comp.eq_ignore_ascii_case("hook-plugins") {
        return None;
    }

    // Return the LAST component (the filename).
    // For flat declarations (`hook-plugins/foo.wasm`) this is `foo.wasm`.
    // For nested declarations (`hook-plugins/sub/nested.wasm`) this is `nested.wasm`,
    // matching `git ls-files` basename output (git ls-files is recursive).
    joined_parts.last().cloned()
}

/// Parse all `plugin = "hook-plugins/<name>"` references from a TOML registry file,
/// resolving each plugin path relative to the registry file's parent directory.
///
/// Uses the `toml` crate (the same parser the dispatcher uses via `registry.rs`) so that
/// any TOML-legal spelling of the `plugin` key is handled identically to production:
///
/// - `plugin = "hook-plugins/foo.wasm"` — standard form
/// - `plugin="hook-plugins/foo.wasm"` — no spaces around `=` (TOML-legal; missed by v1)
/// - `plugin = "./hook-plugins/foo.wasm"` — leading `./` (TOML-legal; missed by v1)
/// - `plugin = "hooks/../hook-plugins/foo.wasm"` — traversal into hook-plugins (F-1; T-025)
/// - `plugin = "/abs/hook-plugins/foo.wasm"` — absolute path, excluded (MEDIUM-2; T-026)
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
///
/// The v2 implementation used `trim_start_matches("./").strip_prefix("hook-plugins/")`,
/// which missed traversal forms (`hooks/../hook-plugins/foo`). Replaced in pass 4 (F-1).
///
/// The v3 (pass-4) implementation used a root-clearing lexical normalisation on the raw
/// plugin path, which incorrectly included absolute paths and returned only the component
/// immediately after `hook-plugins` (incorrect for nested subdirs). Replaced in pass 5
/// (MEDIUM-1/MEDIUM-2/MEDIUM-4(a)) by `extract_hook_plugin_name(registry, plugin_path)`
/// which resolves relative to the registry parent and uses `last()` as the filename.
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
                    // Resolve plugin_path relative to the registry file's parent
                    // directory; include it only if the result lands inside the
                    // sibling hook-plugins/ directory.  Absolute paths are excluded
                    // (they don't resolve to hook-plugins/ relative to registry_parent).
                    // See `extract_hook_plugin_name()` for the full resolution table.
                    if let Some(name) = extract_hook_plugin_name(registry, plugin_path) {
                        refs.insert(name);
                    }
                }
            }
        }
    }

    refs
}

/// Verify the set of `*.toml` files under `plugins_vsdd_factory_dir` equals
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
/// `resolvers-registry.toml`.  If a third registry were added, its declared names
/// would never enter `declared`, the `declared − tracked` difference would be
/// unaffected, and T-012 would stay GREEN while that registry's artifacts were
/// entirely ungated.  By asserting the inventory first, anyone who adds a registry
/// is forced to update T-012's registry list before T-012 will pass.
///
/// ## Fail-closed filter (`*.toml`, not `*-registry.toml`)
///
/// The filter enumerates ALL `.toml` files, not just files ending in
/// `-registry.toml`.  The open-ended form (`*-registry.toml`) is an open
/// enumeration over a naming convention — `metrics_registry.toml` (underscore)
/// or `metrics.registry.toml` (dot) would slip through and bypass detection.
/// `plugins/vsdd-factory/` currently contains exactly two `.toml` files; the
/// fail-closed form asserts exact set equality, so any addition fires.
/// T-024 proves the underscore form is caught; T-017 proves the hyphen form.
///
/// ## Narrowing: non-recursive (`fs::read_dir`)
///
/// `fs::read_dir` is non-recursive; only top-level `.toml` files in
/// `plugins_vsdd_factory_dir` are enumerated.  This is intentional and PROVABLY SAFE:
/// the production dispatcher resolves registries exclusively at the plugin root level —
/// `${CLAUDE_PLUGIN_ROOT}/hooks-registry.toml` and
/// `${CLAUDE_PLUGIN_ROOT}/resolvers-registry.toml` (see `resolve_registry_path()` in
/// `main.rs` and `resolve_plugin_paths()` in `registry.rs`).  A registry placed under a
/// subdirectory is never loaded by production; its declared artifacts are architecturally
/// inert and never required to be git-tracked.
///
/// Subdirectory `.toml` files do exist in the workspace.  As of S-21.09 (grounded by
/// literal `find` output):
///
/// ```text
/// $ find plugins/vsdd-factory -mindepth 2 -name '*.toml'
/// plugins/vsdd-factory/fixtures/smoke-project/Cargo.toml
/// plugins/vsdd-factory/tests/fixtures/registry-tool-filter/anchored.toml
/// plugins/vsdd-factory/tests/fixtures/registry-tool-filter/comment-inject.toml
/// plugins/vsdd-factory/tests/fixtures/registry-tool-filter/intent-comment.toml
/// plugins/vsdd-factory/tests/fixtures/registry-tool-filter/prefix-only-anchor.toml
/// plugins/vsdd-factory/tests/fixtures/registry-tool-filter/unanchored.toml
/// plugins/vsdd-factory/tests/fixtures/validate-policies-schema/fail-nonexistent-plugin/hooks-registry.toml
/// plugins/vsdd-factory/tests/fixtures/validate-policies-schema/pass-namespaced-lint-hook/hooks-registry.toml
/// plugins/vsdd-factory/tests/fixtures/validate-policies-schema/pass-valid-lint-hook/hooks-registry.toml
/// $ find plugins/vsdd-factory -mindepth 1 -maxdepth 1 -name '*.toml'
/// plugins/vsdd-factory/hooks-registry.toml
/// plugins/vsdd-factory/resolvers-registry.toml
/// ```
///
/// `plugins/vsdd-factory/config/` holds `artifact-path-registry.yaml` (a YAML file —
/// no `.toml` files).  The three `hooks-registry.toml` files visible to a recursive walk
/// are test fixtures under `tests/fixtures/`, not production registries.
///
/// Boundary-polarity proof: T-028 shows that `config/hooks-registry.toml` (a subdirectory
/// file) is invisible to the non-recursive enumeration.  The false-negative class (a new
/// plugin registry placed in a subdirectory) is SAFE: production never loads it, so its
/// artifacts are never executed and never require git tracking — no security gap exists.
///
/// ## Narrowing: case-sensitive extension filter
///
/// `.ends_with(".toml")` is a Rust string comparison — case-sensitive on all
/// platforms.  A file named `metrics-registry.TOML` (uppercase extension) would
/// not be caught.  This is acceptable: all production registries use lowercase
/// filenames by convention, and CI runs on a case-sensitive filesystem (Linux).
///
/// Boundary-polarity proof: T-029 shows that `metrics-registry.TOML` is invisible
/// to the `.ends_with(".toml")` filter even when the file exists in the directory.
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
        .filter(|name| name.ends_with(".toml"))
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
         {{hooks-registry.toml, resolvers-registry.toml}} — any other .toml file is \
         either an ungated plugin registry (its declared artifacts bypass T-012's \
         declared−tracked gate) or an unintended tooling configuration file \
         (rustfmt.toml, taplo.toml, etc.); to resolve: if it is a new registry, \
         add it to T-012's registry list; otherwise remove it:\n{}",
        plugins_vsdd_factory_dir.display(),
        lines.join("\n")
    ))
}

/// Verify T-012 steps 2–4 over caller-supplied per-registry declared sets, git-tracked
/// set, and git-committed set.
///
/// Extracted from the T-012 `#[test]` body so that fixture-driven controls
/// (T-015/T-016/T-019/T-020/T-021/T-022) call the REAL gate function rather than
/// logic replicas — replicas cannot detect the defect they purport to guard
/// (TD-VSDD-059 paper-fix detection).
///
/// Returns `Ok(())` when all checks pass. Returns `Err(message)` containing
/// a D-970 Codification 1 outcome identifier on the FIRST failure:
///
/// | Check | Outcome identifier prefix |
/// |-------|--------------------------|
/// | Step 2a: hooks floor | `T-012: hooks registry declared set has only N entries` |
/// | Step 2b: resolvers floor | `T-012: resolvers registry declared set is empty` |
/// | EC-005: tracked empty | `T-012 EC-005` |
/// | Step 3: declared − tracked | `  MISSING: <name>` per artifact |
/// | Step 4: staged-not-committed | `  STAGED-NOT-COMMITTED: <name>` per artifact |
fn check_declared_subset_tracked(
    hooks_declared: &HashSet<String>,
    resolvers_declared: &HashSet<String>,
    tracked: &HashSet<String>,
    committed: &HashSet<String>,
) -> Result<(), String> {
    // Step 2a: hooks registry non-vacuity floor.
    // A hooks parse collapse from 35 to e.g. 1 entry would vacuously pass step 3;
    // the floor catches it.  Floor of 30 mirrors the release.yml sibling gate.
    if hooks_declared.len() < 30 {
        return Err(format!(
            "T-012: hooks registry declared set has only {} entries (expected >= 30); \
             this almost certainly indicates a parse failure in parse_plugin_refs() — \
             check hooks-registry.toml path and TOML format",
            hooks_declared.len()
        ));
    }

    // Step 2b: resolvers registry non-vacuity floor.
    // The resolvers registry currently contributes exactly 1 entry
    // (vsdd-context-resolvers.wasm).  An empty resolvers parse is a parse failure;
    // a union floor of >= 30 cannot distinguish this from a valid state where the
    // hooks registry alone contributes 30+.
    if resolvers_declared.is_empty() {
        return Err(
            "T-012: resolvers registry declared set is empty (expected >= 1); \
             this almost certainly indicates a parse failure in parse_plugin_refs() — \
             check resolvers-registry.toml path and TOML format"
                .to_string(),
        );
    }

    // Build union for step 3.
    let declared: HashSet<String> = hooks_declared
        .iter()
        .chain(resolvers_declared.iter())
        .cloned()
        .collect();

    // EC-005: an empty tracked set would make every declared artifact appear "missing",
    // producing noisy false failures.  Convert this scenario into a clearly-named error.
    if tracked.is_empty() {
        return Err(
            "T-012 EC-005: git ls-files returned no tracked WASMs under \
             plugins/vsdd-factory/hook-plugins/ — verify git is on PATH, workspace_root() \
             resolves correctly, and the test is run inside a git repository"
                .to_string(),
        );
    }

    // Step 3: declared − tracked.
    let mut missing: Vec<&str> = declared
        .iter()
        .filter(|name| !tracked.contains(*name))
        .map(String::as_str)
        .collect();
    missing.sort();

    if !missing.is_empty() {
        return Err(format!(
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
        ));
    }

    // Step 4: staged-not-committed supplementary check.
    // A file staged (`git add -f`) but not yet committed passes the index check above
    // yet is absent from a fresh CI checkout.
    let mut staged_not_committed: Vec<&str> = tracked
        .iter()
        .filter(|name| !committed.contains(*name))
        .map(String::as_str)
        .collect();
    staged_not_committed.sort();

    if !staged_not_committed.is_empty() {
        return Err(format!(
            "T-012 AC-006 S-21.09 BC-4.16.001 Precondition 3 (supplementary): {} WASM \
             artifact(s) are in the git index but NOT committed to HEAD — absent on a \
             fresh CI checkout; commit each before pushing:\n{}",
            staged_not_committed.len(),
            staged_not_committed
                .iter()
                .map(|n| format!("  STAGED-NOT-COMMITTED: {}", n))
                .collect::<Vec<_>>()
                .join("\n")
        ));
    }

    Ok(())
}

/// T-012 gate sequence: all four declared-set ⊆ tracked-set checks against a
/// given workspace `root`.
///
/// Extracted from the `#[test]` body so T-030 can call it against a fixture root,
/// pinning the wiring for BOTH `check_registry_inventory` and
/// `check_declared_subset_tracked` (F-9 wiring control):
///
/// - Phase A (T-030): a fixture root with an unexpected `.toml` proves
///   `check_registry_inventory` is called — removing the call changes the error
///   from "UNEXPECTED: taplo.toml" to a hooks-floor error.
/// - Phase B (T-030): a git fixture root with a missing declared WASM proves
///   `check_declared_subset_tracked` is called — removing the call causes
///   `run_t012_gate` to return `Ok` where `Err` is expected.
///
/// Steps:
///   1. Registry inventory (`check_registry_inventory`): asserts `plugins/vsdd-factory/`
///      contains exactly `{hooks-registry.toml, resolvers-registry.toml}`.
///   2. EC-004: hard-fail on missing registry file.
///   3. Parse per-registry refs via `parse_plugin_refs()`.
///   4-6. Per-registry floors, declared−tracked, staged-not-committed via
///      `check_declared_subset_tracked()`.
///
/// Returns `Ok(())` when all checks pass. Returns `Err(message)` on inventory or
/// declared-subset failure. Panics on git command failure or EC-004.
fn run_t012_gate(root: &Path) -> Result<(), String> {
    // Step 1 (AC-006 step 1): registry-inventory gate.
    // Assert plugins/vsdd-factory/ contains exactly {hooks-registry.toml,
    // resolvers-registry.toml}.  Must run BEFORE declared-set aggregation so
    // that adding a third registry fires here, forcing the caller to update T-012's
    // registry list rather than silently leaving the new registry's artifacts ungated.
    let plugins_vsdd_factory = root.join("plugins/vsdd-factory");
    check_registry_inventory(&plugins_vsdd_factory)?;

    let hooks_registry = root.join("plugins/vsdd-factory/hooks-registry.toml");
    let resolvers_registry = root.join("plugins/vsdd-factory/resolvers-registry.toml");

    // EC-004: hard-fail on missing registry file — never silently pass.
    if !hooks_registry.exists() {
        panic!(
            "T-012 EC-004: plugins/vsdd-factory/hooks-registry.toml not found under \
             workspace root {}; registry file is mandatory — test must fail explicitly, \
             not silently pass",
            root.display()
        );
    }
    if !resolvers_registry.exists() {
        panic!(
            "T-012 EC-004: plugins/vsdd-factory/resolvers-registry.toml not found under \
             workspace root {}; registry file is mandatory — test must fail explicitly, \
             not silently pass",
            root.display()
        );
    }

    // Parse per-registry refs separately so check_declared_subset_tracked() can apply
    // per-registry floors (HIGH-2: a union floor cannot detect a resolvers-only collapse).
    let hooks_refs = parse_plugin_refs(&hooks_registry);
    let resolvers_refs = parse_plugin_refs(&resolvers_registry);

    // Git-tracked set (index) and committed set (HEAD tree).
    // `git_tracked_wasm_names()` panics on non-zero exit so failure is explicit, not silent.
    let tracked: HashSet<String> = git_tracked_wasm_names(root).into_iter().collect();
    let committed: HashSet<String> = git_committed_wasm_names(root).into_iter().collect();

    // Steps 2-4: per-registry floors, declared⊆tracked, staged-not-committed.
    // Delegated to check_declared_subset_tracked() so fixture-driven controls
    // (T-015/T-016/T-019/T-020/T-021/T-022) call the real gate, not logic replicas.
    check_declared_subset_tracked(&hooks_refs, &resolvers_refs, &tracked, &committed)
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
/// `plugins/vsdd-factory/hook-plugins/` is gitignored. Local cargo
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
            "-r",
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
    // Full gate sequence is delegated to run_t012_gate() so that T-030 can call
    // the same function against a fixture workspace root, pinning both
    // check_registry_inventory (phase A) and check_declared_subset_tracked (phase B)
    // as load-bearing wiring (F-9 wiring control).
    //
    // vsdd-context-resolvers.wasm: declared in resolvers-registry, also tracked in git.
    // Using both registries matches the dual-registry scope of T-009 and avoids a
    // registry-scope mismatch between the two directions.
    run_t012_gate(&workspace_root()).unwrap_or_else(|msg| panic!("{}", msg));
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
// This test proves extract_hook_plugin_name() normalises the leading `./`
// (CurDir component → no-op in lex_norm) by resolving the path relative to the
// registry parent so that `parse_plugin_refs()` on the dotslash fixture MUST return
// a set containing "ghost-guard-dotslash.wasm".
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
         extract_hook_plugin_name() resolves the path relative to the registry parent, \
         so CurDir ('.') is a no-op and the path lands at registry_parent/hook-plugins/...; \
         got refs: {:?}",
        refs
    );
}

// ---------------------------------------------------------------------------
// T-015 — Declared-but-untracked fixture control: outcome identifier confirmed
//
// Positive control for the declared − tracked detection arm.  Uses SYNTHETIC fixtures:
// 29 filler hooks + "hooks-only.wasm" (30 total, passes hooks floor), plus one resolver.
// The tracked set contains the 29 fillers and the resolver but NOT "hooks-only.wasm",
// simulating the pre-`git add -f` state where that artifact was never committed.
//
// The test verifies:
//   (a) "hooks-only.wasm" is absent from tracked → declared − tracked is non-empty.
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
    // Synthetic fixture: 29 filler hooks + "hooks-only.wasm" (30 total, passes hooks
    // floor), plus one resolver (passes resolvers floor).  The tracked set contains
    // the 29 fillers and the resolver but NOT "hooks-only.wasm" — simulating the
    // pre-`git add -f` state where that artifact was never committed.
    //
    // Calls check_declared_subset_tracked() directly so that a mutation to the real
    // "  MISSING: {}" format string breaks this test, not merely a replica.
    let mut hooks_declared: HashSet<String> =
        (0..29).map(|i| format!("filler-{:02}.wasm", i)).collect();
    hooks_declared.insert("hooks-only.wasm".to_string()); // 30 total

    let resolvers_declared: HashSet<String> = ["resolver.wasm".to_string()].into_iter().collect();

    // tracked = all 29 fillers + resolver, NOT "hooks-only.wasm"
    let tracked: HashSet<String> = (0..29)
        .map(|i| format!("filler-{:02}.wasm", i))
        .chain(["resolver.wasm".to_string()])
        .collect();
    let committed = tracked.clone(); // same as tracked: no staged-not-committed noise

    let result =
        check_declared_subset_tracked(&hooks_declared, &resolvers_declared, &tracked, &committed);

    // (a) Must return Err when hooks-only.wasm is declared but not tracked.
    assert!(
        result.is_err(),
        "T-015 AC-006: check_declared_subset_tracked must return Err when a declared \
         artifact is absent from the tracked set; got Ok"
    );

    let msg = result.unwrap_err();

    // (b) Outcome identifier per D-970 Codification 1 — must name the artifact.
    assert!(
        msg.contains("MISSING: hooks-only.wasm"),
        "T-015 AC-006 D-970 Codification 1: error message must contain \
         'MISSING: hooks-only.wasm'; got: {}",
        msg
    );
}

// ---------------------------------------------------------------------------
// T-016 — PASS arm fixture control: empty diff when all declared are tracked
//
// Negative control for the declared − tracked direction.  Uses SYNTHETIC fixtures:
// exactly 30 hooks (filler-00..filler-29) + 1 resolver, with tracked set equal to
// the declared union — simulating the post-fix state where every declared artifact
// has been committed to git.
//
// When tracked == declared, the declared − tracked difference is empty.
// The test asserts Ok — confirming no false positives when every declared artifact
// is present in the tracked set.
//
// F-2 floor boundary (high end): this fixture uses exactly 30 hooks (passes the
// `hooks_declared.len() >= 30` floor at the boundary value).  T-027 covers the
// complementary case (29 hooks → floor fires).  Together T-016 and T-027 form the
// floor boundary pair that constrains the constant `30`.  Mutating `< 30` to `< 2`
// causes T-027 to pass incorrectly (29 >= 2 → floor does not fire → #[should_panic]
// fails), while this test continues to pass — catching the mutation.
//
// This test always GREEN (fixture-driven; does not call git ls-files).
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
fn test_S_21_09_ac006_T016_pass_arm_empty_diff_when_all_declared_are_tracked() {
    // Synthetic fixture: EXACTLY 30 hooks + 1 resolver (passes both floors AT the
    // floor boundary).  tracked = declared union — simulates the post-fix state where
    // every declared artifact is committed.
    //
    // Calls check_declared_subset_tracked() directly so that a mutation that widens the
    // declared − tracked step (always returns Ok) would still be caught if a missing
    // control were removed.  The PASS arm proves no false-positives.
    let hooks_declared: HashSet<String> =
        (0..30).map(|i| format!("filler-{:02}.wasm", i)).collect();
    let resolvers_declared: HashSet<String> = ["resolver.wasm".to_string()].into_iter().collect();
    let tracked: HashSet<String> = hooks_declared
        .iter()
        .cloned()
        .chain(resolvers_declared.iter().cloned())
        .collect();
    let committed = tracked.clone();

    let result =
        check_declared_subset_tracked(&hooks_declared, &resolvers_declared, &tracked, &committed);

    result.unwrap_or_else(|e| {
        panic!(
            "T-016 AC-006: check_declared_subset_tracked must return Ok when every \
             declared artifact is also in the tracked set (no false positives in PASS arm); \
             got Err: {}",
            e
        )
    });
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
// T-019 — Hooks floor control: proves hooks floor fires when hooks set has 1 entry
//
// step 2a of check_declared_subset_tracked() asserts `hooks_declared.len() >= 30`.
// A floor with no control is indistinguishable from a dead assertion (TD-VSDD-059).
// This test calls the REAL function (not a replica) with a 1-entry hooks set and uses
// `#[should_panic]` to verify it fires.  The `expected=` parameter locks the hooks-
// floor outcome identifier.
//
// Mutation-proof: neutralising the hooks floor (`< 30` → `false`) causes
// check_declared_subset_tracked() to return Ok; unwrap_or_else() never panics;
// `#[should_panic]` FAILS — the mutation is detected.
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
#[should_panic(expected = "T-012: hooks registry declared set has only 1 entries")]
fn test_S_21_09_ac006_T019_hooks_floor_fires_on_one_entry_hooks_set() {
    let hooks_declared: HashSet<String> = ["single-plugin.wasm".to_string()].into_iter().collect(); // 1 entry < 30
    let resolvers_declared: HashSet<String> = ["resolver.wasm".to_string()].into_iter().collect();
    let tracked: HashSet<String> = HashSet::new();
    let committed: HashSet<String> = HashSet::new();
    // Calls the real function; unwrap panics with the Err message.
    check_declared_subset_tracked(&hooks_declared, &resolvers_declared, &tracked, &committed)
        .unwrap_or_else(|e| panic!("{}", e));
}

// ---------------------------------------------------------------------------
// T-020 — EC-005 control: proves EC-005 fires when tracked set is empty
//
// check_declared_subset_tracked() step EC-005 asserts the tracked set is non-empty.
// An empty tracked set would produce noisy false failures on every declared artifact;
// EC-005 converts this into a clearly-named error.
//
// This test calls the REAL function with 30 hooks + 1 resolver (passes floors) but
// an empty tracked set.  `#[should_panic]` locks the "T-012 EC-005" identifier.
//
// Mutation-proof: removing EC-005 causes the function to proceed to step 3, where
// every declared artifact is "missing".  The panic message becomes a MISSING: chain,
// NOT "T-012 EC-005", so `#[should_panic(expected = "T-012 EC-005")]` FAILS.
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
#[should_panic(expected = "T-012 EC-005")]
fn test_S_21_09_ac006_T020_ec005_fires_on_empty_tracked_set() {
    let hooks_declared: HashSet<String> =
        (0..30).map(|i| format!("filler-{:02}.wasm", i)).collect();
    let resolvers_declared: HashSet<String> = ["resolver.wasm".to_string()].into_iter().collect();
    let tracked: HashSet<String> = HashSet::new(); // empty → EC-005 fires
    let committed: HashSet<String> = HashSet::new();
    check_declared_subset_tracked(&hooks_declared, &resolvers_declared, &tracked, &committed)
        .unwrap_or_else(|e| panic!("{}", e));
}

// ---------------------------------------------------------------------------
// T-021 — Staged-not-committed control: "STAGED-NOT-COMMITTED: <name>" identifier
//
// check_declared_subset_tracked() step 4 computes `staged_not_committed = tracked − committed`
// and returns Err with "  STAGED-NOT-COMMITTED: <name>" per artifact.
//
// This test calls the REAL function with 30 hooks + 1 resolver (passes floors), all in
// tracked (passes steps 2-3), plus "staged-plugin.wasm" in tracked but absent from
// committed.  Asserts the returned Err contains the outcome identifier.
//
// Mutation-proof: neutralising step 4 (`!staged_not_committed.is_empty()` → `false`)
// causes the function to return Ok; result.is_err() assertion FAILS.
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
fn test_S_21_09_ac006_T021_staged_not_committed_outcome_identifier() {
    // Base sets: 30 hooks + 1 resolver, all tracked and committed.
    let hooks_declared: HashSet<String> =
        (0..30).map(|i| format!("filler-{:02}.wasm", i)).collect();
    let resolvers_declared: HashSet<String> = ["resolver.wasm".to_string()].into_iter().collect();

    // tracked includes staged-plugin.wasm; committed does NOT (simulates `git add -f`
    // without a subsequent commit).
    let mut tracked: HashSet<String> = hooks_declared
        .iter()
        .cloned()
        .chain(resolvers_declared.iter().cloned())
        .collect();
    tracked.insert("staged-plugin.wasm".to_string());
    let committed: HashSet<String> = tracked
        .iter()
        .filter(|n| *n != "staged-plugin.wasm")
        .cloned()
        .collect();

    let result =
        check_declared_subset_tracked(&hooks_declared, &resolvers_declared, &tracked, &committed);

    // (a) Must return Err for the staged-not-committed scenario.
    assert!(
        result.is_err(),
        "T-021 AC-006 S-21.09: check_declared_subset_tracked must return Err when \
         a tracked artifact is absent from committed; got Ok"
    );

    let msg = result.unwrap_err();

    // (b) Outcome identifier per D-970 Codification 1.
    assert!(
        msg.contains("  STAGED-NOT-COMMITTED: staged-plugin.wasm"),
        "T-021 AC-006 S-21.09 D-970 Codification 1: error message must contain \
         '  STAGED-NOT-COMMITTED: staged-plugin.wasm'; got: {}",
        msg
    );
}

// ---------------------------------------------------------------------------
// T-022 — Resolvers floor control: proves resolvers floor fires on empty resolvers set
//
// check_declared_subset_tracked() step 2b asserts resolvers_declared is non-empty.
// A union floor of >= 30 cannot detect a resolvers-only collapse: 35 hooks alone
// clears 30, so a resolvers parse failure would go undetected.  The per-registry
// floor converts this scenario into a clearly-named error.
//
// This test calls the REAL function with 30 hooks (passes hooks floor) but empty
// resolvers.  `#[should_panic]` locks the resolvers-floor outcome identifier.
//
// Mutation-proof: removing the resolvers floor causes the function to proceed to
// EC-005; the panic message becomes "T-012 EC-005", NOT "T-012: resolvers registry
// declared set is empty", so `#[should_panic]` FAILS.
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
#[should_panic(expected = "T-012: resolvers registry declared set is empty")]
fn test_S_21_09_ac006_T022_resolvers_floor_fires_on_empty_resolvers_set() {
    let hooks_declared: HashSet<String> =
        (0..30).map(|i| format!("filler-{:02}.wasm", i)).collect(); // passes hooks floor
    let resolvers_declared: HashSet<String> = HashSet::new(); // empty → resolvers floor fires
    let tracked: HashSet<String> = HashSet::new();
    let committed: HashSet<String> = HashSet::new();
    check_declared_subset_tracked(&hooks_declared, &resolvers_declared, &tracked, &committed)
        .unwrap_or_else(|e| panic!("{}", e));
}

// ---------------------------------------------------------------------------
// T-023 — Boundary polarity: bare path excluded + traversal-that-cancels excluded
//
// parse_plugin_refs() includes ONLY names where `extract_hook_plugin_name()` resolves
// the path relative to the registry parent and finds a `hook-plugins` component
// (case-insensitive) immediately after the parent prefix.
//
// This test exercises two excluded forms:
//
//   (a) Bare name: `plugin = "ghost-bare.wasm"` → resolves to registry_parent/ghost-bare.wasm
//       — no hook-plugins component → excluded.
//
//   (b) Traversal-cancels: `plugin = "hook-plugins/../ghost-cancels.wasm"` → resolves to
//       registry_parent/ghost-cancels.wasm (hook-plugins pushed, then popped by `..`)
//       — no hook-plugins component after registry_parent → excluded.
//
// ## Boundary-polarity record (POLICY 13) — pass-5
//
// **Included region:**
//   Paths that, after resolving relative to registry_parent, have `hook-plugins`
//   (case-insensitive) as the component immediately after registry_parent:
//   - Standard: `hook-plugins/foo.wasm`
//   - Dotslash: `./hook-plugins/foo.wasm`
//   - Traversal INTO hook-plugins: `hooks/../hook-plugins/foo.wasm` (T-025)
//   - Case variant: `Hook-Plugins/foo.wasm` (T-031)
//   - Nested subdir: `hook-plugins/sub/nested.wasm` → `nested.wasm` (T-032)
//
// **Excluded region:**
//   Paths that resolve to a location outside registry_parent/hook-plugins/:
//   - Bare names: `ghost-bare.wasm` → registry_parent/ghost-bare.wasm
//   - Traversal-cancels: `hook-plugins/../ghost.wasm` → registry_parent/ghost.wasm
//   - Absolute paths: `/abs/hook-plugins/foo.wasm` (T-026)
//   - `../` prefix: `../hook-plugins/foo.wasm` → parent(registry_parent)/hook-plugins/foo.wasm
//
// **Can harmful content occupy the excluded region?**
//   Excluded paths resolve outside `registry_parent/hook-plugins/`, which is the
//   gitignored artifact directory.  An artifact at such a path is either not in the
//   gitignored dir (visible to `git status`) or loads from outside the repo entirely
//   (absolute / `../` prefix).  No stealth path exists in the excluded region.
//
// **Traversal-cancels mutation proof (POLICY 13, MEDIUM-4(b)):**
//   If `ParentDir` were NOT popped (bug: `ParentDir => {}` instead of `ParentDir => { pop }`),
//   `hook-plugins/../ghost-cancels.wasm` would normalise to
//   `[..., registry_parent, hook-plugins, ghost-cancels.wasm]` — the `hook-plugins`
//   component survives, and `ghost-cancels.wasm` is falsely admitted.  T-023(b) detects
//   this mutation: `refs_cancels.is_empty()` fails if `..` is ignored.
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
fn test_S_21_09_ac006_T023_boundary_polarity_bare_and_traversal_cancels_excluded() {
    let tmp = tempdir().expect("tempdir must create successfully");

    // (a) Bare name: no hook-plugins component → excluded.
    let registry_bare = tmp.path().join("registry-bare.toml");
    fs::write(
        &registry_bare,
        concat!(
            "schema_version = 2\n",
            "[[hooks]]\n",
            "name = \"ghost\"\n",
            "event = \"PreToolUse\"\n",
            "tool = \"^Bash$\"\n",
            "plugin = \"ghost-bare.wasm\"\n",
            "timeout_ms = 5000\n",
            "on_error = \"continue\"\n"
        ),
    )
    .expect("bare-path registry must be written to tempfile");

    let refs_bare = parse_plugin_refs(&registry_bare);

    assert!(
        refs_bare.is_empty(),
        "T-023(a) boundary-polarity: parse_plugin_refs must exclude bare plugin paths \
         ('ghost-bare.wasm' has no hook-plugins component after resolving relative to \
         the registry parent); got refs: {:?}",
        refs_bare
    );

    // (b) Traversal-cancels: hook-plugins/../ghost-cancels.wasm resolves to
    //     registry_parent/ghost-cancels.wasm (hook-plugins popped by ..) → excluded.
    // Mutation proof: if ParentDir were ignored (not popped), hook-plugins would survive
    // in the component list and ghost-cancels.wasm would be falsely admitted.
    let registry_cancels = tmp.path().join("registry-cancels.toml");
    fs::write(
        &registry_cancels,
        concat!(
            "schema_version = 2\n",
            "[[hooks]]\n",
            "name = \"ghost-cancels\"\n",
            "event = \"PreToolUse\"\n",
            "tool = \"^Bash$\"\n",
            "plugin = \"hook-plugins/../ghost-cancels.wasm\"\n",
            "timeout_ms = 5000\n",
            "on_error = \"continue\"\n"
        ),
    )
    .expect("traversal-cancels registry must be written to tempfile");

    let refs_cancels = parse_plugin_refs(&registry_cancels);

    assert!(
        refs_cancels.is_empty(),
        "T-023(b) MEDIUM-4(b) traversal-cancels: `hook-plugins/../ghost-cancels.wasm` \
         resolves to registry_parent/ghost-cancels.wasm (the `..` pops `hook-plugins`); \
         no hook-plugins component remains → excluded from declared; \
         mutation proof: if ParentDir were not popped, hook-plugins would survive and \
         ghost-cancels.wasm would be falsely admitted; got refs: {:?}",
        refs_cancels
    );
}

// ---------------------------------------------------------------------------
// T-024 — BLOCKER-2 underscore mutant: `metrics_registry.toml` caught by fail-closed
//
// Before the fail-closed fix, check_registry_inventory() filtered with
// `ends_with("-registry.toml")`.  A file named `metrics_registry.toml` (underscore)
// does NOT end with "-registry.toml", so it slipped through — the directory appeared
// to contain only the expected pair, and the inventory check returned Ok even though
// an ungated registry was present.
//
// The fail-closed fix enumerates ALL `*.toml` files.  Any file ending in `.toml` that
// is not in the expected pair fires UNEXPECTED.  This test proves the underscore form
// is now caught; T-017 proves the hyphen form (was already caught by the old filter).
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
fn test_S_21_09_ac006_T024_registry_inventory_underscore_form_caught() {
    let tmp = tempdir().expect("tempdir must create successfully");

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
    // Underscore form: previously invisible to the -registry.toml filter.
    fs::write(
        tmp.path().join("metrics_registry.toml"),
        "schema_version = 1\n",
    )
    .expect("metrics_registry.toml must be written to tempdir");

    let result = check_registry_inventory(tmp.path());

    // (a) Must return Err — the underscore form must not slip through.
    assert!(
        result.is_err(),
        "T-024 BLOCKER-2 fail-closed: check_registry_inventory must return Err when \
         metrics_registry.toml (underscore) is present; got Ok — \
         the *.toml filter must catch all .toml files, not just *-registry.toml"
    );

    let msg = result.unwrap_err();

    // (b) Outcome identifier names the specific file per D-970 Codification 1.
    assert!(
        msg.contains("UNEXPECTED: metrics_registry.toml"),
        "T-024 BLOCKER-2 D-970 Codification 1: error message must contain \
         'UNEXPECTED: metrics_registry.toml'; got: {}",
        msg
    );
}

// ---------------------------------------------------------------------------
// T-025 — F-1 traversal form: `hooks/../hook-plugins/foo.wasm` is parsed as declared
//
// Before pass-4, parse_plugin_refs() used `trim_start_matches("./")` followed by
// `strip_prefix("hook-plugins/")`.  The traversal form `hooks/../hook-plugins/foo.wasm`
// does NOT start with `./` and does NOT start with `hook-plugins/` after stripping `./`,
// so the v2 gate silently excluded it from `declared` — a false negative.
//
// The production dispatcher's resolve_plugin_paths() joins relative paths against the
// registry parent.  `hooks/../hook-plugins/foo.wasm` resolves (via OS path resolution)
// to `hook-plugins/foo.wasm` relative to the registry parent — i.e., the same gitignored
// artifact directory as `hook-plugins/foo.wasm`.  The gate must detect it.
//
// Pass-4/pass-5 fix: `extract_hook_plugin_name(registry, plugin_path)` resolves
// plugin_path relative to registry_parent, then lexically normalises:
// `hooks/../hook-plugins/ghost-traversal.wasm` joined with registry_parent →
// pop `hooks`, push `hook-plugins`, push `ghost-traversal.wasm` → the component
// immediately after registry_parent is `hook-plugins` → `ghost-traversal.wasm` returned.
//
// This test proves the fix: parse_plugin_refs() on the traversal-form registry MUST
// return a set containing "ghost-traversal.wasm".
//
// Mutation-proof: reverting to `strip_prefix("hook-plugins/")` without normalisation
// would exclude the traversal form; `refs.contains(...)` assertion FAILS.
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
fn test_S_21_09_ac006_T025_traversal_form_is_parsed_as_declared() {
    let tmp = tempdir().expect("tempdir must create successfully");
    let registry = tmp.path().join("registry.toml");
    fs::write(
        &registry,
        concat!(
            "schema_version = 2\n",
            "[[hooks]]\n",
            "name = \"ghost-traversal\"\n",
            "event = \"PreToolUse\"\n",
            "tool = \"^Bash$\"\n",
            "plugin = \"hooks/../hook-plugins/ghost-traversal.wasm\"\n",
            "timeout_ms = 5000\n",
            "on_error = \"continue\"\n"
        ),
    )
    .expect("traversal-form registry must be written to tempfile");

    let refs = parse_plugin_refs(&registry);

    assert!(
        refs.contains("ghost-traversal.wasm"),
        "T-025 F-1: parse_plugin_refs must extract 'ghost-traversal.wasm' from a \
         registry using the traversal form \
         (plugin = \"hooks/../hook-plugins/ghost-traversal.wasm\") — \
         extract_hook_plugin_name() resolves '..' via ParentDir pop before testing \
         for the hook-plugins component; got refs: {:?}",
        refs
    );
}

// ---------------------------------------------------------------------------
// T-026 — MEDIUM-2 (pass-5 correction): absolute-form plugin path is EXCLUDED
//
// The pass-4 implementation (root-clearing lexical normalisation on the raw plugin path)
// treated `/abs/hook-plugins/ghost-absolute.wasm` as equivalent to
// `hook-plugins/ghost-absolute.wasm` inside the repo.  This was wrong:
//
//   - Production dispatcher's `resolve_plugin_paths()` passes absolute plugin paths
//     through UNCHANGED (see `resolve_plugin_paths_is_idempotent_for_absolute_paths`
//     in `registry.rs` tests and `resolve_plugin_paths()` source which only calls
//     `base.join(&entry.plugin)` for RELATIVE paths).
//   - An absolute path like `/abs/hook-plugins/ghost-absolute.wasm` loads from
//     that absolute filesystem location — NOT from `${PLUGIN_ROOT}/hook-plugins/`.
//   - Demanding `ghost-absolute.wasm` be git-tracked inside the repo when production
//     loads it from outside the repo produces a false-positive MISSING outcome.
//
// Pass-5 fix: `extract_hook_plugin_name(registry, plugin_path)` resolves plugin_path
// relative to the registry's parent directory using Rust Path::join semantics:
// if plugin_path is absolute, it REPLACES the base (registry_parent is discarded).
// `/abs/hook-plugins/ghost-absolute.wasm` does not start with registry_parent's
// prefix, so the component match fails → returns None.
//
// The EXCLUSION is correct and safe: T-025 continues to prove that traversal forms
// that resolve INTO `hook-plugins/` relative to the registry parent ARE included.
// Absolute paths that resolve outside the repo simply escape the gate (fail closed —
// no security consequence, and no false-positive MISSING outcome).
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
fn test_S_21_09_ac006_T026_absolute_form_excluded_from_declared() {
    let tmp = tempdir().expect("tempdir must create successfully");
    let registry = tmp.path().join("registry.toml");
    fs::write(
        &registry,
        concat!(
            "schema_version = 2\n",
            "[[hooks]]\n",
            "name = \"ghost-absolute\"\n",
            "event = \"PreToolUse\"\n",
            "tool = \"^Bash$\"\n",
            "plugin = \"/abs/hook-plugins/ghost-absolute.wasm\"\n",
            "timeout_ms = 5000\n",
            "on_error = \"continue\"\n"
        ),
    )
    .expect("absolute-form registry must be written to tempfile");

    let refs = parse_plugin_refs(&registry);

    // Absolute paths must NOT enter declared: they resolve outside the registry parent
    // and production loads them from that external location, not from hook-plugins/.
    assert!(
        !refs.contains("ghost-absolute.wasm"),
        "T-026 MEDIUM-2 (pass-5): absolute-path plugin declarations must NOT be included \
         in declared — production resolve_plugin_paths() passes absolute paths unchanged \
         (loads from /abs/hook-plugins/..., not from registry_parent/hook-plugins/); \
         including them produces false-positive MISSING outcomes; got refs: {:?}",
        refs
    );
    assert!(
        refs.is_empty(),
        "T-026: expected empty refs for absolute-path declaration; got: {:?}",
        refs
    );
}

// ---------------------------------------------------------------------------
// T-027 — F-2 floor boundary (29 fires): hooks floor fires on 29-entry set
//
// The non-vacuity floor asserts `hooks_declared.len() >= 30`.  T-019 proves it fires
// for a 1-entry set.  A floor constant without a boundary-pair control is indistinguishable
// from a weaker constant — mutating `< 30` to `< 2` leaves T-019 green (1 < 2 is true).
//
// This test provides the complementary boundary case: 29 entries (= threshold − 1)
// MUST fire the floor.  T-016 provides the other side: exactly 30 entries passes.
// Together they form the floor boundary pair that constrains the constant `30`.
//
// Mutation-proof: mutating `< 30` to `< 2` makes 29 pass the floor (29 >= 2 is true);
// check_declared_subset_tracked proceeds to EC-005 (empty tracked → different error);
// `#[should_panic(expected = "... has only 29 entries")]` FAILS — mutation detected.
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
#[should_panic(expected = "T-012: hooks registry declared set has only 29 entries")]
fn test_S_21_09_ac006_T027_hooks_floor_fires_on_29_entry_set() {
    let hooks_declared: HashSet<String> =
        (0..29).map(|i| format!("filler-{:02}.wasm", i)).collect(); // 29 < 30 → floor fires
    let resolvers_declared: HashSet<String> = ["resolver.wasm".to_string()].into_iter().collect();
    let tracked: HashSet<String> = HashSet::new();
    let committed: HashSet<String> = HashSet::new();
    // Calls the real function; unwrap panics with the Err message.
    check_declared_subset_tracked(&hooks_declared, &resolvers_declared, &tracked, &committed)
        .unwrap_or_else(|e| panic!("{}", e));
}

// ---------------------------------------------------------------------------
// T-028 — F-3a narrowing proof (non-recursive, SAFE): subdirectory .toml invisible
//
// check_registry_inventory() uses `fs::read_dir` (non-recursive).  This is intentional
// and PROVABLY SAFE:
//
//   The production dispatcher resolves registries exclusively at the plugin root level:
//     - `${CLAUDE_PLUGIN_ROOT}/hooks-registry.toml` (via resolve_registry_path() in main.rs)
//     - `${CLAUDE_PLUGIN_ROOT}/resolvers-registry.toml` (via plugin_root.join(...) in main.rs)
//   A registry placed in a subdirectory is NEVER loaded by production.  Its declared
//   artifacts are architecturally inert: they are never executed, never checked against
//   the git index, and never required to exist.  The false-negative class is safe.
//
// Documented narrowing scope:
//   - Gate checks ONLY top-level `.toml` files in `plugins_vsdd_factory_dir`.
//   - Subdirectory `.toml` files are outside the gate's scope.
//   - A registry added under a subdirectory bypasses the inventory check; this is SAFE
//     (not a convention-dependency) because production never loads subdirectory registries.
//
// Literal-shell grounding (from workspace root):
//   $ find plugins/vsdd-factory -mindepth 2 -name '*.toml'
//   plugins/vsdd-factory/fixtures/smoke-project/Cargo.toml
//   plugins/vsdd-factory/tests/fixtures/registry-tool-filter/anchored.toml
//   plugins/vsdd-factory/tests/fixtures/registry-tool-filter/comment-inject.toml
//   plugins/vsdd-factory/tests/fixtures/registry-tool-filter/intent-comment.toml
//   plugins/vsdd-factory/tests/fixtures/registry-tool-filter/prefix-only-anchor.toml
//   plugins/vsdd-factory/tests/fixtures/registry-tool-filter/unanchored.toml
//   plugins/vsdd-factory/tests/fixtures/validate-policies-schema/fail-nonexistent-plugin/hooks-registry.toml
//   plugins/vsdd-factory/tests/fixtures/validate-policies-schema/pass-namespaced-lint-hook/hooks-registry.toml
//   plugins/vsdd-factory/tests/fixtures/validate-policies-schema/pass-valid-lint-hook/hooks-registry.toml
//   (plugins/vsdd-factory/config/ contains artifact-path-registry.yaml — no .toml files)
//
// This test proves the narrowing: a `config/hooks-registry.toml` file is invisible
// to the non-recursive enumeration when the top-level pair is correct.
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
fn test_S_21_09_ac006_T028_subdirectory_toml_not_caught_narrowing_proof() {
    let tmp = tempdir().expect("tempdir must create successfully");

    // Expected top-level pair: inventory should return Ok.
    fs::write(
        tmp.path().join("hooks-registry.toml"),
        "schema_version = 2\n",
    )
    .expect("hooks-registry.toml must be written");
    fs::write(
        tmp.path().join("resolvers-registry.toml"),
        "schema_version = 1\n",
    )
    .expect("resolvers-registry.toml must be written");

    // Subdirectory file: invisible to non-recursive fs::read_dir.
    // Mirrors the real workspace where tests/fixtures/ holds hooks-registry.toml files
    // in subdirectories that must NOT trigger the inventory check.
    let config_dir = tmp.path().join("config");
    fs::create_dir_all(&config_dir).expect("config dir must be created");
    fs::write(
        config_dir.join("hooks-registry.toml"),
        "schema_version = 2\n",
    )
    .expect("config/hooks-registry.toml must be written");

    // Non-recursive: subdirectory file is invisible → inventory is Ok.
    let result = check_registry_inventory(tmp.path());

    assert!(
        result.is_ok(),
        "T-028 F-3a narrowing (non-recursive, SAFE): check_registry_inventory must return \
         Ok when only the top-level pair exists; config/hooks-registry.toml is in a \
         subdirectory and must NOT be detected by the non-recursive fs::read_dir; \
         narrowing is safe because production only loads top-level registries; \
         got Err: {}",
        result.unwrap_err()
    );
}

// ---------------------------------------------------------------------------
// T-029 — F-3b narrowing proof (case-sensitive, SAFE): uppercase .TOML not caught
//
// check_registry_inventory() uses `.ends_with(".toml")` — a Rust string comparison
// that is case-sensitive on all platforms (the comparison is on the string content,
// not the filesystem).  A file named `metrics-registry.TOML` fails the filter.
//
// Documented narrowing scope:
//   - All production registries use lowercase filenames by convention.
//   - The production dispatcher resolves registries using the literal names
//     `hooks-registry.toml` and `resolvers-registry.toml` (lowercase); a file named
//     `hooks-registry.TOML` would not be loaded by production at runtime, so
//     excluding it from the inventory check is safe (same load-path argument as T-028).
//   - CI runs on a case-sensitive filesystem (Linux); macOS HFS+ preserves case in
//     directory listings, so ".ends_with(\".toml\")" still returns false for ".TOML"
//     even on macOS.
//   - An adversarially-placed `ghost_REGISTRY.TOML` would bypass this check; this is
//     safe (production would not load it) and code review is the additional defence.
//
// This test proves the narrowing: a `metrics-registry.TOML` file is invisible to the
// case-sensitive filter even when it exists in the directory.
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
fn test_S_21_09_ac006_T029_uppercase_extension_not_caught_narrowing_proof() {
    let tmp = tempdir().expect("tempdir must create successfully");

    // Expected top-level pair: inventory should return Ok.
    fs::write(
        tmp.path().join("hooks-registry.toml"),
        "schema_version = 2\n",
    )
    .expect("hooks-registry.toml must be written");
    fs::write(
        tmp.path().join("resolvers-registry.toml"),
        "schema_version = 1\n",
    )
    .expect("resolvers-registry.toml must be written");

    // Uppercase extension: "metrics-registry.TOML".ends_with(".toml") == false.
    // The file is visible in the directory listing but invisible to the string filter.
    fs::write(
        tmp.path().join("metrics-registry.TOML"),
        "schema_version = 1\n",
    )
    .expect("metrics-registry.TOML must be written");

    // Case-sensitive filter: uppercase file is invisible → inventory is Ok.
    let result = check_registry_inventory(tmp.path());

    assert!(
        result.is_ok(),
        "T-029 F-3b narrowing (case-sensitive): check_registry_inventory must return Ok \
         when only the top-level pair is present at the .toml extension level; \
         metrics-registry.TOML has an uppercase extension and must NOT be caught by \
         .ends_with(\".toml\"); got Err: {}",
        result.unwrap_err()
    );
}

// ---------------------------------------------------------------------------
// T-030 — F-9 wiring control: run_t012_gate integrates both check calls
//
// check_registry_inventory and check_declared_subset_tracked are each tested by
// dedicated unit controls (T-017/T-018 and T-015/T-016/T-019..T-022 respectively).
// However, deleting either call from run_t012_gate's body leaves all prior tests
// green (the real workspace is clean; T-012 still passes).  This test pins both
// call sites by running run_t012_gate against fixture workspace roots designed to
// fail at a specific call:
//
//   Phase A — inventory wiring:
//     tmpdir with {hooks-registry.toml, resolvers-registry.toml, taplo.toml}.
//     run_t012_gate must return Err with "UNEXPECTED: taplo.toml".
//     If check_registry_inventory were removed: parse_plugin_refs on a minimal
//     registry (0 hooks) → hooks floor fires → error is "has only 0 entries", NOT
//     "UNEXPECTED: taplo.toml" → msg.contains assertion FAILS.
//
//   Phase B — declared-subset wiring (git fixture):
//     git-initialized tmpdir with valid inventory; hooks-registry declares 30 WASMs
//     (all committed); resolvers-registry declares ctx.wasm (NOT committed).
//     run_t012_gate must return Err with "MISSING: ctx.wasm".
//     If check_declared_subset_tracked were removed: run_t012_gate returns Ok →
//     result.is_err() assertion FAILS.
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
fn test_S_21_09_ac006_T030_wiring_control_both_check_calls_are_active() {
    // ---- Phase A: inventory wiring ----
    {
        let tmp = tempdir().expect("tempdir must create successfully");
        let root = tmp.path();
        let plugins_dir = root.join("plugins/vsdd-factory");
        fs::create_dir_all(&plugins_dir).expect("plugins dir must be created");

        // Minimal registries (no [[hooks]] entries → 0 declared hooks).
        fs::write(
            plugins_dir.join("hooks-registry.toml"),
            "schema_version = 2\n",
        )
        .expect("hooks-registry.toml must be written");
        fs::write(
            plugins_dir.join("resolvers-registry.toml"),
            "schema_version = 1\n",
        )
        .expect("resolvers-registry.toml must be written");

        // Unexpected file — what check_registry_inventory must catch BEFORE the floor.
        fs::write(plugins_dir.join("taplo.toml"), "").expect("taplo.toml must be written");

        let result = run_t012_gate(root);

        assert!(
            result.is_err(),
            "T-030 phase A (inventory wiring): run_t012_gate must return Err when \
             taplo.toml is present in plugins/vsdd-factory/; if check_registry_inventory \
             were removed from run_t012_gate, the hooks floor would fire instead (0 hooks); \
             got Ok"
        );

        // The outcome identifier must be the INVENTORY error, not the floor error.
        // This distinguishes the two call sites: inventory fires first and names the file.
        assert!(
            result.unwrap_err().contains("UNEXPECTED: taplo.toml"),
            "T-030 phase A (inventory wiring): error must contain 'UNEXPECTED: taplo.toml' \
             (inventory outcome) — if check_registry_inventory were removed, the error would \
             be 'hooks registry declared set has only 0 entries' (floor outcome) and this \
             assertion would FAIL"
        );
    }

    // ---- Phase B: declared-subset wiring (git fixture) ----
    {
        let tmp = tempdir().expect("tempdir must create successfully");
        let root = tmp.path();

        // Initialise a git repo.
        // -c overrides prevent interference from global git config:
        //   core.excludesFile=/dev/null — prevents a global gitignore from hiding *.wasm
        //   commit.gpgsign=false       — prevents gpg-sign failures if globally enabled
        //   core.hooksPath=/dev/null   — prevents global hooks from rejecting the commit
        //   init.templateDir=          — prevents template-installed hooks
        let status = Command::new("git")
            .args([
                "-c",
                "core.excludesFile=/dev/null",
                "-c",
                "commit.gpgsign=false",
                "-c",
                "core.hooksPath=/dev/null",
                "-c",
                "init.templateDir=",
                "init",
            ])
            .current_dir(root)
            .status()
            .expect("git init must succeed for T-030 phase B fixture");
        assert!(status.success(), "T-030 phase B: git init failed");

        Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(root)
            .status()
            .expect("git config user.email must succeed");
        Command::new("git")
            .args(["config", "user.name", "Test"])
            .current_dir(root)
            .status()
            .expect("git config user.name must succeed");

        let plugins_dir = root.join("plugins/vsdd-factory");
        let hook_plugins_dir = plugins_dir.join("hook-plugins");
        fs::create_dir_all(&hook_plugins_dir).expect("hook-plugins dir must be created");

        // Valid inventory: exactly the expected pair, no extra files.
        // hooks-registry: 30 entries (all declaring h{:02}.wasm → committed below).
        let mut hooks_content = String::from("schema_version = 2\n");
        for i in 0..30_u32 {
            hooks_content.push_str(&format!(
                "[[hooks]]\nname = \"h{i:02}\"\nplugin = \"hook-plugins/h{i:02}.wasm\"\n\
                 event = \"PreToolUse\"\ntool = \"^Bash$\"\ntimeout_ms = 5000\n\
                 on_error = \"continue\"\n",
            ));
        }
        fs::write(plugins_dir.join("hooks-registry.toml"), &hooks_content)
            .expect("hooks-registry.toml must be written");

        // resolvers-registry: 1 entry — ctx.wasm will NOT be committed (triggers MISSING).
        fs::write(
            plugins_dir.join("resolvers-registry.toml"),
            "schema_version = 1\n[[resolvers]]\nname = \"ctx\"\n\
             plugin = \"hook-plugins/ctx.wasm\"\n",
        )
        .expect("resolvers-registry.toml must be written");

        // Commit the 30 hook WASMs but NOT ctx.wasm.
        for i in 0..30_u32 {
            fs::write(hook_plugins_dir.join(format!("h{i:02}.wasm")), b"wasm")
                .expect("hook wasm fixture must be written");
        }

        let status = Command::new("git")
            .args(["-c", "core.excludesFile=/dev/null", "add", "."])
            .current_dir(root)
            .status()
            .expect("git add must succeed for T-030 phase B fixture");
        assert!(status.success(), "T-030 phase B: git add failed");

        let status = Command::new("git")
            .args([
                "-c",
                "core.excludesFile=/dev/null",
                "-c",
                "commit.gpgsign=false",
                "-c",
                "core.hooksPath=/dev/null",
                "commit",
                "-m",
                "T-030 phase B fixture",
            ])
            .current_dir(root)
            .status()
            .expect("git commit must succeed for T-030 phase B fixture");
        assert!(status.success(), "T-030 phase B: git commit failed");

        // Inventory passes (exact pair); hooks floor passes (30 entries);
        // resolvers floor passes (1 entry); declared − tracked fires on ctx.wasm.
        // If check_declared_subset_tracked were removed: run_t012_gate returns Ok →
        // result.is_err() FAILS.
        let result = run_t012_gate(root);

        assert!(
            result.is_err(),
            "T-030 phase B (declared-subset wiring): run_t012_gate must return Err \
             with MISSING: ctx.wasm when ctx.wasm is declared but not committed; \
             if check_declared_subset_tracked were removed from run_t012_gate, \
             run_t012_gate would return Ok and this assertion would FAIL"
        );

        assert!(
            result.unwrap_err().contains("MISSING: ctx.wasm"),
            "T-030 phase B D-970 Codification 1: error must contain 'MISSING: ctx.wasm'"
        );
    }
}

// ---------------------------------------------------------------------------
// T-031 — MEDIUM-4(a) case-variant control: `Hook-Plugins/x.wasm` is parsed as declared
//
// On macOS's case-insensitive default filesystem, `Hook-Plugins/x.wasm` and
// `hook-plugins/x.wasm` refer to the same directory.  The pass-4 implementation used
// `c == "hook-plugins"` — a case-sensitive string comparison — so `Hook-Plugins/x.wasm`
// would fail the check and escape the declared-set gate entirely.
//
// Pass-5 fix: `extract_hook_plugin_name()` uses `eq_ignore_ascii_case("hook-plugins")`
// when checking the component after registry_parent.  This handles case variants while
// remaining deterministic on case-sensitive Linux CI.
//
// Mutation-proof: reverting to `c == "hook-plugins"` (case-sensitive) causes
// `Hook-Plugins/x.wasm` to return None; `refs.contains("ghost-case.wasm")` FAILS.
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
fn test_S_21_09_ac006_T031_case_variant_hook_plugins_included() {
    let tmp = tempdir().expect("tempdir must create successfully");
    let registry = tmp.path().join("registry.toml");
    fs::write(
        &registry,
        concat!(
            "schema_version = 2\n",
            "[[hooks]]\n",
            "name = \"ghost-case\"\n",
            "event = \"PreToolUse\"\n",
            "tool = \"^Bash$\"\n",
            "plugin = \"Hook-Plugins/ghost-case.wasm\"\n",
            "timeout_ms = 5000\n",
            "on_error = \"continue\"\n"
        ),
    )
    .expect("case-variant registry must be written to tempfile");

    let refs = parse_plugin_refs(&registry);

    assert!(
        refs.contains("ghost-case.wasm"),
        "T-031 MEDIUM-4(a): parse_plugin_refs must include 'ghost-case.wasm' from a \
         registry using the case-variant form (plugin = \"Hook-Plugins/ghost-case.wasm\") \
         — extract_hook_plugin_name() uses eq_ignore_ascii_case for the hook-plugins \
         component; on macOS case-insensitive FS this is the same directory; \
         mutation-proof: c == \"hook-plugins\" (case-sensitive) would return None here; \
         got refs: {:?}",
        refs
    );
}

// ---------------------------------------------------------------------------
// T-032 — MEDIUM-1 nested-subdir control: `hook-plugins/sub/nested.wasm` yields filename
//
// The pass-4 implementation returned `normalized.get(pos + 1)` — the component
// IMMEDIATELY after `hook-plugins`.  For `hook-plugins/sub/nested.wasm` this returns
// `sub`, not `nested.wasm`.  `git ls-files` is recursive and returns full paths; the
// gate compares basenames, so the declared set would contain `sub` and `nested.wasm`
// would never match any tracked artifact — a MISSING: sub identifier instead of the
// correct artifact name.
//
// Pass-5 fix: `extract_hook_plugin_name()` returns `joined_parts.last()` (the final
// component of the normalised resolved path), which is the filename regardless of depth.
//
// Mutation-proof: reverting to `normalized.get(pos + 1)` would return `Some("sub")`
// for this input; `refs.contains("nested.wasm")` assertion FAILS.
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
fn test_S_21_09_ac006_T032_nested_subdir_yields_filename() {
    let tmp = tempdir().expect("tempdir must create successfully");
    let registry = tmp.path().join("registry.toml");
    fs::write(
        &registry,
        concat!(
            "schema_version = 2\n",
            "[[hooks]]\n",
            "name = \"nested\"\n",
            "event = \"PreToolUse\"\n",
            "tool = \"^Bash$\"\n",
            "plugin = \"hook-plugins/sub/nested.wasm\"\n",
            "timeout_ms = 5000\n",
            "on_error = \"continue\"\n"
        ),
    )
    .expect("nested-subdir registry must be written to tempfile");

    let refs = parse_plugin_refs(&registry);

    assert!(
        refs.contains("nested.wasm"),
        "T-032 MEDIUM-1: parse_plugin_refs must extract 'nested.wasm' (last component) \
         from a nested declaration (plugin = \"hook-plugins/sub/nested.wasm\") — \
         extract_hook_plugin_name() returns joined_parts.last(), not pos+1; \
         mutation-proof: reverting to pos+1 returns 'sub' and this assertion FAILS; \
         got refs: {:?}",
        refs
    );
    // Also assert 'sub' is NOT in refs (wrong component from the pos+1 bug).
    assert!(
        !refs.contains("sub"),
        "T-032 MEDIUM-1: 'sub' must NOT appear in refs — it is a directory component, \
         not the artifact filename; got refs: {:?}",
        refs
    );
}
