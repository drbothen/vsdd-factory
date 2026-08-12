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
//! | T-020 | AC-006 S-21.09 | GREEN | EC-005b control: calls `check_declared_subset_tracked()` with empty tracked; `#[should_panic]` locks "T-012 EC-005b" identifier |
//! | T-021 | AC-006 S-21.09 | GREEN | Staged-not-committed: calls `check_declared_subset_tracked()` with staged artifact → Err containing "STAGED-NOT-COMMITTED: staged-plugin.wasm" identifier |
//! | T-022 | AC-006 S-21.09 | GREEN | Resolvers floor control: calls `check_declared_subset_tracked()` with empty resolvers; `#[should_panic]` locks "T-012: resolvers registry declared set is empty" |
//! | T-023 | AC-006 S-21.09 | GREEN | MEDIUM-1 boundary polarity (corrected pass 4): bare plugin path `ghost-bare.wasm` (no `hook-plugins` component after normalization) → excluded from declared; traversal/absolute forms now INCLUDED via lexical normalisation |
//! | T-024 | AC-006 S-21.09 | GREEN | BLOCKER-2 underscore mutant: `metrics_registry.toml` (underscore, previously missed by `-registry.toml` filter) caught by fail-closed `*.toml` inventory → "UNEXPECTED: metrics_registry.toml" |
//! | T-025 | AC-006 S-21.09 | GREEN | F-1 traversal control: `plugin = "hooks/../hook-plugins/ghost-traversal.wasm"` is parsed as declared (resolves `..` relative to registry parent, lands inside `hook-plugins/`) |
//! | T-026 | AC-006 S-21.09 | GREEN | MEDIUM-2 + HIGH-1 pass-6: absolute-form excluded; depth-matched sub-test proves prefix-verification loop is load-bearing (not just the length check) |
//! | T-027 | AC-006 S-21.09 | GREEN | F-2 floor boundary (29 fires): 29-entry hooks set fires hooks floor — `#[should_panic]` locks "T-012: hooks registry declared set has only 29 entries"; pins the threshold so mutating `< 30` to `< 2` is caught |
//! | T-028 | AC-006 S-21.09 | GREEN | F-3a narrowing proof (non-recursive, SAFE): subdirectory `config/hooks-registry.toml` invisible to `fs::read_dir`; safe because production only loads top-level registries |
//! | T-029 | AC-006 S-21.09 | GREEN | F-3b narrowing proof (case-sensitive): `metrics-registry.TOML` (uppercase) invisible to `.ends_with(".toml")`; safe because production loads lowercase-named registries |
//! | T-030 | AC-006 S-21.09 | GREEN | F-9 wiring control: `run_t012_gate` integrates both `check_registry_inventory` (phase A) and `check_declared_subset_tracked` (phase B via git fixture); removing either call breaks a phase |
//! | T-031 | AC-006 S-21.09 | GREEN | MEDIUM-4(a) case-variant admission control: `plugin = "Hook-Plugins/foo.wasm"` enters declared as `"Hook-Plugins/foo.wasm"` (verbatim; gate-3 eq_ignore_ascii_case admits; no lowercasing, per pass-9.1) |
//! | T-032 | AC-006 S-21.09 | GREEN | MEDIUM-1 nested-subdir control: `plugin = "hook-plugins/sub/nested.wasm"` yields `nested.wasm` (last component); proves non-flat declarations are not silently mis-named |
//! | T-033 | AC-006 S-21.09 | GREEN | MEDIUM-3 minimum-length lower boundary: `plugin = "hook-plugins"` (directory path, no filename) is excluded; pins `expected_depth + 2` constant |
//! | T-034 | AC-006 S-21.09 | GREEN | MEDIUM-4 `-r` flag control: `git ls-tree -r` finds a WASM committed under `hook-plugins/sub/`; dropping `-r` misses nested files |
//! | T-035 | AC-006 S-21.09 | GREEN | HIGH-1 pass-7: gate-3 (hook-plugins component check) isolated control: `plugin = "other-dir/evil-probe.wasm"` passes gates 1+2 (depth+2 length, parent prefix matches) but is excluded by gate 3 (`other-dir` ≠ `hook-plugins`) |
//! | T-036 | AC-006 S-21.09 | GREEN | HIGH-1 pass-8 M15 killer: git fixture with `.gitignore` excluding `hook-plugins/`, 30 hooks force-committed, `gitignored-probe.wasm` declared AND on disk but NOT force-added; `MISSING: hook-plugins/gitignored-probe.wasm` via `run_t012_gate`; MEDIUM-2 `git check-ignore` load-bearing check; LOW-2 non-WASM filter control |
//! | T-037 | AC-006 S-21.09 | GREEN | HIGH-2 pass-8 M18+M16 killer: git fixture — `git add -f` `staged-probe.wasm` (declared, on disk) without committing; `STAGED-NOT-COMMITTED: hook-plugins/staged-probe.wasm` via `run_t012_gate` |
//! | T-038 | AC-006 S-21.09 | GREEN | HIGH-3 pass-8 UNGATED-DECLARATION: `other-dir/evil-probe.wasm` passes gates 1+2 but fails gate 3; `run_t012_gate` emits `UNGATED-DECLARATION: other-dir/evil-probe.wasm` before reaching git calls |
//! | T-039 | AC-006 S-21.09 | GREEN | HIGH-1 pass-9 probe control: 30 hooks declared at `hook-plugins/sub/hNN.wasm`, committed flat at `hook-plugins/hNN.wasm`; `MISSING: hook-plugins/sub/h00.wasm` (false-negative closed by full-path tracking) |
//! | T-040 | AC-006 S-21.09 | GREEN | MEDIUM-1 pass-9 resolvers arm: ungated declaration in `resolvers-registry.toml` fires `UNGATED-DECLARATION: other-dir/evil-resolver.wasm` |
//! | T-041 | AC-006 S-21.09 | GREEN | MEDIUM-3 pass-9 git failure path: valid-inventory non-git dir; `run_t012_gate` panics with `git ls-files exited with status` |
//! | T-042 | AC-006 S-21.09 | GREEN | pass-9.1 case-variant MISSING outcome: `Hook-Plugins/ghost-missing.wasm` declared verbatim, lowercase `hook-plugins/ghost-missing.wasm` tracked; `check_declared_subset_tracked` returns `MISSING: Hook-Plugins/ghost-missing.wasm` (pins end-to-end Linux-CI behavior) |
//! | T-043 | AC-006 S-21.09 | GREEN | pass-10 bare-name UNGATED control: `ghost-bare.wasm` resolves to `registry_parent/ghost-bare.wasm`; passes containment check (inside worktree root); NOT correctly under hook-plugins/; fires `UNGATED-DECLARATION: ghost-bare.wasm` |
//! | T-044 | AC-006 S-21.09 | GREEN | pass-10 `../registry-parent/` UNGATED control: `../vsdd-factory/ghost-updir.wasm` resolves back to `registry_parent/ghost-updir.wasm`; inside root; fires `UNGATED-DECLARATION: ../vsdd-factory/ghost-updir.wasm` |
//! | T-045 | AC-006 S-21.09 | GREEN | pass-10.1 one-level-up UNGATED: `../ghost.wasm` resolves to `plugins/ghost.wasm` inside root; containment passes, NOT under hook-plugins/; fires `UNGATED-DECLARATION: ../ghost.wasm` |
//! | T-046 | AC-006 S-21.09 | GREEN | pass-10.1 two-levels-up UNGATED: `../../ghost.wasm` resolves to `<root>/ghost.wasm` inside root; containment passes (root_parts.len()+1 > root_parts.len()); fires `UNGATED-DECLARATION: ../../ghost.wasm` |
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
//! Stories: S-19.04 (T-006..T-011), S-21.09 (T-012..T-042)
//! VP Trace: — (AC-006 wires EAC-005 as load-bearing leg; no BC mapping)

use std::collections::HashSet;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::Command;
use tempfile::tempdir;

// ---------------------------------------------------------------------------
// Module-level constants
// ---------------------------------------------------------------------------

/// Git-root-relative prefix stripped from `git ls-files` / `git ls-tree` paths to
/// produce registry-parent-relative declared-set identifiers.
///
/// `git_tracked_wasm_names()` and `git_committed_wasm_names()` return paths like
/// `"plugins/vsdd-factory/hook-plugins/foo.wasm"`.  `run_t012_gate()` and T-009 strip
/// this prefix to get `"hook-plugins/foo.wasm"` — matching what `parse_plugin_refs()`
/// returns via `extract_hook_plugin_name()`.
///
/// The constant is declared once here (TD-VSDD-060 sibling-site discipline) so that
/// a mutation to one copy is caught by ALL callers.  A wrong value (e.g.,
/// `"nonexistent-prefix/"`) causes every `strip_prefix` call to fail loud via
/// `.expect()`, rather than silently shrinking the domain.
const REGISTRY_PARENT_PREFIX: &str = "plugins/vsdd-factory/";

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

/// Lexically normalise `path`: resolve `..` (pop), skip `.`, clear on root prefix.
///
/// Shared by `extract_hook_plugin_name` and `detect_ungated_declarations`.
///
/// **`RootDir | Prefix(_) => parts.clear()` is provably a no-op** (4th defensive
/// unreachable arm): on an absolute path, the root component always fires FIRST,
/// before any `Normal` component has pushed to `parts`, so `parts` is always empty
/// when this arm executes.  The `clear()` has no observable effect.  Recorded as the
/// same defensive category as the three unreachable `?` exits in
/// `extract_hook_plugin_name`.
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
                parts.clear(); // defensive no-op: parts always empty when root fires
            }
        }
    }
    parts
}

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
/// | Standard | `registry_parent/hook-plugins/foo.wasm` | `hook-plugins/foo.wasm` |
/// | Leading `./` | `registry_parent/hook-plugins/foo.wasm` | `hook-plugins/foo.wasm` |
/// | Traversal (into hook-plugins) | `registry_parent/hook-plugins/foo.wasm` | `hook-plugins/foo.wasm` |
/// | Case variant | `registry_parent/Hook-Plugins/foo.wasm` | `Hook-Plugins/foo.wasm` (verbatim; MISSING on lowercase git tree) |
/// | Nested subdir | `registry_parent/hook-plugins/sub/nested.wasm` | `hook-plugins/sub/nested.wasm` |
/// | Absolute path | `/abs/hook-plugins/foo.wasm` (stays absolute, ≠ registry_parent) | `None` |
/// | Traversal (cancels hook-plugins) | `registry_parent/ghost.wasm` | `None` |
/// | `../` prefix | `parent(registry_parent)/hook-plugins/foo.wasm` | `None` |
/// | Bare | `registry_parent/ghost-bare.wasm` | `None` |
///
/// **Why divergent-prefix absolute paths are excluded by gate-2:** `lex_norm`'s
/// `RootDir` arm clears `parts` (a provable no-op since `parts` is always empty when
/// root fires), then builds from the normal components as usual.  An absolute path
/// `/plugins/vsdd-factory/hook-plugins/foo.wasm` therefore has the same `lex_norm`
/// output as the relative path `plugins/vsdd-factory/hook-plugins/foo.wasm` and
/// IS admitted past gate-2.  In contrast, an absolute path whose first component
/// differs from the registry parent (e.g., `/usr/local/lib/evil.wasm`) fails gate-2
/// because `joined_parts[0]` = `"usr"` ≠ `parent_parts[0]` = `"plugins"`.
/// Production's `resolve_plugin_paths()` passes absolute plugin paths through
/// unchanged, so they load from outside the repo; the gate is silent on them only
/// if they have a divergent prefix.  Absolute paths matching the registry parent
/// prefix are admitted into `declared` and compared against the git-tracked set.
///
/// **Case-insensitive `hook-plugins` match (gate 3):** `eq_ignore_ascii_case` accepts
/// `Hook-Plugins/x.wasm` past gate 3, admitting it into `declared` as `"Hook-Plugins/x.wasm"`
/// (verbatim — NOT lowercased).  This is necessary: a case-sensitive check (`c == "hook-plugins"`)
/// would return `None`, leaving `declared` empty and `declared ⊆ tracked` trivially true — a
/// silent false negative.  The admitted path is then compared case-sensitively against the
/// git-tracked set.  On Linux CI (case-sensitive FS), `"Hook-Plugins/x.wasm"` ≠
/// `"hook-plugins/x.wasm"` → `MISSING: Hook-Plugins/x.wasm` fires.  On macOS
/// (case-insensitive FS) the same MISSING fires, conservatively flagging a declaration
/// that does not match the tracked path — operators should use lowercase `hook-plugins/`.
/// `git ls-files` preserves committed case verbatim on both platforms (confirmed by probe).
///
/// **Registry-relative path returned verbatim:** `joined_parts[expected_depth..].join("/")`
/// is returned as-is (e.g., `"hook-plugins/foo.wasm"`, `"hook-plugins/sub/nested.wasm"`,
/// or `"Hook-Plugins/foo.wasm"` for a case-variant declaration).  This preserves
/// subdirectory structure so that a declaration at `hook-plugins/sub/h00.wasm` is distinct
/// from a WASM committed flat at `hook-plugins/h00.wasm` — the basename-only approach was
/// HIGH-1 (pass-9), a correctness bug where both paths reduced to `h00.wasm` producing a
/// false negative.  The `hook-plugins` component is **NOT** lowercased: lowercasing would
/// cause `Hook-Plugins/foo.wasm` to map to `hook-plugins/foo.wasm`, masking a case
/// mismatch on Linux CI where production genuinely cannot load the artifact.
///
/// **Diffing against git output:** `git_tracked_wasm_names()` and
/// `git_committed_wasm_names()` return raw git-root-relative paths (e.g.,
/// `"plugins/vsdd-factory/hook-plugins/foo.wasm"`).  `run_t012_gate()` strips the
/// `"plugins/vsdd-factory/"` prefix before calling `check_declared_subset_tracked()`,
/// producing registry-parent-relative paths (`"hook-plugins/foo.wasm"`) that match the
/// declared-set identifiers returned here.
///
/// **Gate structure:** three reachable exclusion gates plus four defensive unreachable
/// arms. Reachable gates: (1) minimum-length (`joined_parts.len() < expected_depth + 2`
/// → `return None`); (2) registry-parent prefix loop (`joined_parts[i] != parent_parts[i]`
/// → `return None`); (3) `hook-plugins` component check
/// (`!hook_comp.eq_ignore_ascii_case("hook-plugins")` → `return None`). Defensive
/// unreachable arms: `registry_path.parent()` `?` exit, `joined_parts.get(i)` (inside the
/// gate-2 loop) `?` exit, `joined_parts.get(expected_depth)` (before gate-3) `?` exit —
/// the latter two are unreachable because gate-1 establishes `len >= expected_depth + 2` —
/// and the `RootDir | Prefix(_) => parts.clear()` arm in `lex_norm` (provably a no-op:
/// `parts` is always empty when a root component fires; see `lex_norm` doc).
///
/// **Repo-internal non-`hook-plugins/` declarations are detected by `detect_ungated_declarations`:**
/// `extract_hook_plugin_name` (this function) returns `None` for declarations that fail
/// gate 1 (bare names, `../`-prefix forms that cancel to `registry_parent`) or gate 2
/// (divergent prefix), and `None` for gate-3 failures — so they do not appear in
/// `declared`.  For gate-3 failures (`other-dir/evil-probe.wasm`) this was a silent
/// false-negative before pass-10.  Starting pass-10, `detect_ungated_declarations()` uses
/// a lower gate-1 threshold (`expected_depth + 1`) to catch ALL declarations that resolve
/// inside the registry parent but outside `hook-plugins/`, including bare names and
/// `../registry_parent/` forms.  They surface as `UNGATED-DECLARATION: <path>` in
/// `run_t012_gate()` before any git calls.  See T-038 (gate-3 escape), T-043 (bare-name),
/// T-044 (`../registry-parent/` prefix).
///
/// See T-025 (traversal-into proof), T-026 (depth-matched absolute + divergent-prefix),
/// T-023 (traversal-cancels + bare forms), T-031 (case-variant), T-032 (nested-subdir),
/// T-033 (min-length lower boundary), T-038 (ungated-declaration), T-043 (bare-name
/// UNGATED), T-044 (`../registry-parent/` UNGATED).
fn extract_hook_plugin_name(registry_path: &Path, plugin_path: &str) -> Option<String> {
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

    // Return the registry-parent-relative path starting at the hook-plugins component,
    // verbatim (no case normalisation).
    // For flat declarations (`hook-plugins/foo.wasm`) this is `"hook-plugins/foo.wasm"`.
    // For nested declarations (`hook-plugins/sub/nested.wasm`) this is
    // `"hook-plugins/sub/nested.wasm"` — preserving the subdirectory path so that a
    // declaration at `hook-plugins/sub/h00.wasm` is distinct from a committed flat
    // `hook-plugins/h00.wasm` (HIGH-1 correctness fix, pass-9).
    // Case-variant declarations (e.g., `Hook-Plugins/foo.wasm`) are returned verbatim;
    // they will fire `MISSING:` against a lowercase git tree on both macOS and Linux
    // (pass-9.1 correctness fix: lowercasing masked Linux-CI false negatives).
    Some(joined_parts[expected_depth..].join("/"))
}

/// Return raw `plugin = "..."` path strings from `registry_path` that resolve **inside
/// the worktree `root`** but **NOT correctly under `registry_parent/hook-plugins/`**.
///
/// ## Detection algorithm (pass-10.1 — containment-based)
///
/// For each `plugin = "..."` value, compute `joined = lex_norm(registry_parent.join(path))`.
///
/// **Containment check (replaces the pass-10 length-based gate-1):**
/// If `joined.len() <= root_parts.len()` OR `joined[0..root_parts.len()] ≠ root_parts`,
/// the resolved path is outside the worktree root — silently skipped.  Production loads
/// these from outside the repo (e.g., absolute paths with a divergent prefix, or `../`
/// chains deep enough to escape the worktree).  This gate does not report them.
///
/// **Correctness check:**
/// A declaration is "correctly targeted" when ALL three hold:
///   (a) `joined.len() >= expected_depth + 2` — has a filename component after `hook-plugins/`
///   (b) `joined[0..expected_depth] == parent_parts` — registry-parent prefix intact
///   (c) `joined[expected_depth].eq_ignore_ascii_case("hook-plugins")` — lands under hook-plugins
///
/// Any in-repo declaration that does NOT satisfy (a)+(b)+(c) is pushed to `ungated`.
///
/// ## What this catches
/// - Gate-3 escapes: `other-dir/evil-probe.wasm` (T-038) — in-repo, wrong subdir
/// - Bare names: `ghost-bare.wasm` → `registry_parent/ghost-bare.wasm` (T-043)
/// - `../registry-parent/` forms: `../vsdd-factory/ghost.wasm` (T-044)
/// - One-level-up: `../ghost.wasm` → `plugins/ghost.wasm` (T-045)
/// - Two-levels-up: `../../ghost.wasm` → `<root>/ghost.wasm` (T-046)
///
/// ## What this does NOT catch
/// Declarations that resolve OUTSIDE the worktree root (deeper `../` chains, absolute
/// paths with divergent prefixes).  Those escape the containment check and are silently
/// dropped.  Production would attempt to load them from outside the repo — that is a
/// separate concern outside this gate's scope.  See T-047 (out-of-repo probe).
///
/// ## Threshold note
/// The old length-based gate-1 (`< expected_depth + 1`) is gone from this function —
/// the containment check subsumes it.  The `expected_depth + 2` that appears in the
/// `is_hook_plugins` positive check (condition a) has the same semantics as the gate-1
/// in `extract_hook_plugin_name`: "the path must have a filename component after
/// `hook-plugins/`".  Both thresholds are now `expected_depth + 2` for the same reason.
///
/// Used by `run_t012_gate()` to emit `UNGATED-DECLARATION: <path>` before git calls.
///
/// See T-038, T-040 (resolvers arm), T-043 (bare-name), T-044 (`../registry-parent/`),
/// T-045 (`../` one level up), T-046 (`../../` two levels up).
fn detect_ungated_declarations(registry_path: &Path, root: &Path) -> Vec<String> {
    let registry_parent = match registry_path.parent() {
        Some(p) => p,
        None => return Vec::new(),
    };
    let parent_parts = lex_norm(registry_parent);
    let expected_depth = parent_parts.len();
    let root_parts = lex_norm(root);

    let content = fs::read_to_string(registry_path).unwrap_or_default();
    let doc: toml::Value = match content.parse::<toml::Value>() {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    let mut ungated = Vec::new();
    for section in &["hooks", "resolvers"] {
        if let Some(toml::Value::Array(entries)) = doc.get(*section) {
            for entry in entries {
                if let Some(toml::Value::String(plugin_path)) = entry.get("plugin") {
                    let joined_parts = lex_norm(&registry_parent.join(plugin_path.as_str()));

                    // Containment check: the resolved path must land inside the worktree root.
                    // Paths that resolve outside the worktree (deep `../` chains, absolute paths
                    // with a divergent prefix) are silently skipped — not this gate's concern.
                    if joined_parts.len() <= root_parts.len() {
                        continue;
                    }
                    let in_repo = root_parts
                        .iter()
                        .enumerate()
                        .all(|(i, p)| joined_parts.get(i) == Some(p));
                    if !in_repo {
                        continue;
                    }

                    // Correctness check: is this declaration correctly targeting hook-plugins/?
                    // Requires: (a) filename component after hook-plugins/, (b) registry-parent
                    // prefix intact, (c) first component after parent is "hook-plugins".
                    let is_hook_plugins = joined_parts.len() >= expected_depth + 2
                        && parent_parts
                            .iter()
                            .enumerate()
                            .all(|(i, p)| joined_parts.get(i) == Some(p))
                        && joined_parts[expected_depth].eq_ignore_ascii_case("hook-plugins");

                    if !is_hook_plugins {
                        ungated.push(plugin_path.clone());
                    }
                }
            }
        }
    }
    ungated.sort();
    ungated
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
/// Boundary-polarity proof: T-028 uses a synthetic tempdir fixture with a subdirectory
/// `config/hooks-registry.toml`; that file is invisible to the non-recursive enumeration.
/// The false-negative class (a new
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
/// | EC-005b: tracked empty (fixture path) | `T-012 EC-005b` |
/// | Step 3: declared − tracked | `  MISSING: <name>` per artifact |
/// | Step 4: staged-not-committed | `  STAGED-NOT-COMMITTED: <name>` per artifact |
///
/// **Known false-positive class — `enabled = false` entries (LOW-3):** `RegistryEntry`
/// carries an `enabled: bool` field. `parse_plugin_refs()` extracts all `plugin` values
/// from the TOML array regardless of `enabled` state — the `toml` crate sees all array
/// entries equally, and the extraction loop does not filter on `enabled`. A deliberately-
/// disabled entry (`enabled = false`) still contributes its artifact name to `declared`,
/// so the artifact is required to be git-tracked even if production never loads it.
/// Result: a disabled-but-untracked plugin produces a false-positive
/// `MISSING: <name>` outcome. Latent today (`grep -c 'enabled *= *false'
/// plugins/vsdd-factory/hooks-registry.toml` → 0); fail-loud (explicit MISSING
/// identifier), so the outcome is immediately actionable. Recorded per POLICY 13.
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

    // EC-005b: an empty tracked set would make every declared artifact appear "missing",
    // producing noisy false failures.  Convert this scenario into a clearly-named error.
    //
    // In the real gate path (`run_t012_gate`), EC-005a fires first (assert on raw git output)
    // before the tracked set is constructed, so this branch is reachable only in test fixtures
    // that pass an explicitly empty tracked set (e.g., T-020).
    // EC-005a covers the "pathspec returned zero paths" failure; EC-005b covers the fixture path.
    if tracked.is_empty() {
        return Err(
            "T-012 EC-005b: tracked set passed to check_declared_subset_tracked is empty; \
             in the real gate path this is preceded by EC-005a in run_t012_gate; \
             in test fixtures this fires directly when an empty tracked set is passed \
             (e.g., T-020)"
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
///      **EC-004 obligation:** a missing registry file yields `MISSING: <name>` via the
///      inventory step's `?`-propagated `Err` — no separate hard-fail is needed.  The
///      dead-code EC-004 panic blocks that appeared after the inventory call in passes 1–5
///      were unreachable (inventory's `?` always fires first for a missing file) and were
///      removed in pass-6 (MEDIUM-1 closure).
///   2. Parse per-registry refs via `parse_plugin_refs()`.
///   2a/2b/3/4. Per-registry floors, declared−tracked, staged-not-committed via
///      `check_declared_subset_tracked()`.
///
/// Returns `Ok(())` when all checks pass. Returns `Err(message)` on inventory or
/// declared-subset failure. Panics on git command failure.
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

    // EC-004 obligation fulfilled: a missing registry yields Err("...MISSING: hooks-registry.toml"
    // / "...MISSING: resolvers-registry.toml") from step 1 above (propagated by `?`).
    // No separate hard-fail is required — the inventory check fires before parse_plugin_refs
    // is reached. The EC-004 panic blocks present in passes 1–5 were unreachable dead code
    // (removed in pass-6 MEDIUM-1 closure).

    // Parse per-registry refs separately so check_declared_subset_tracked() can apply
    // per-registry floors (HIGH-2: a union floor cannot detect a resolvers-only collapse).
    let hooks_refs = parse_plugin_refs(&hooks_registry);
    let resolvers_refs = parse_plugin_refs(&resolvers_registry);

    // Check for ungated declarations BEFORE calling git (no git required for this step).
    // Any declaration that resolves inside the worktree root but NOT under
    // registry_parent/hook-plugins/ escapes the declared-set gate — a potential
    // reproduction of the S-21.09 failure scenario.
    // Fires first so T-038/T-043/T-044/T-045/T-046 do not need a git fixture.
    let mut ungated: Vec<String> = detect_ungated_declarations(&hooks_registry, root);
    ungated.extend(detect_ungated_declarations(&resolvers_registry, root));
    ungated.sort();
    if !ungated.is_empty() {
        return Err(format!(
            "T-012 AC-006 S-21.09: {} declaration(s) resolve inside the repo but outside \
             hook-plugins/ — these escape the declared-set gate and require investigation:\n{}",
            ungated.len(),
            ungated
                .iter()
                .map(|p| format!("  UNGATED-DECLARATION: {}", p))
                .collect::<Vec<_>>()
                .join("\n")
        ));
    }

    // Git-tracked set (index) and committed set (HEAD tree).
    // `git_tracked_wasm_names()` panics on non-zero exit so failure is explicit, not silent.
    //
    // Strip the "plugins/vsdd-factory/" prefix from raw git paths to obtain
    // registry-parent-relative paths ("hook-plugins/foo.wasm") that match the declared-set
    // identifiers produced by extract_hook_plugin_name() (HIGH-1 correctness fix, pass-9).
    //
    // EC-005a: assert git returned at least one path.  An empty raw set means git ls-files
    // found nothing matching the pathspec 'plugins/vsdd-factory/hook-plugins/' — most likely
    // a case-variant path was force-added under a different capitalisation (e.g., Hook-Plugins/)
    // which silently empties the pathspec on case-sensitive CI.
    // Remedy: git rm -r --cached plugins/vsdd-factory/hook-plugins/ &&
    //         git add -f plugins/vsdd-factory/hook-plugins/
    let tracked_raw = git_tracked_wasm_names(root);
    assert!(
        !tracked_raw.is_empty(),
        "T-012 EC-005a: git ls-files returned no paths matching the pathspec \
         'plugins/vsdd-factory/hook-plugins/' — pathspec is case-sensitive on Linux CI; \
         if a 'git add -f Hook-Plugins/…' re-spelled the index directory, fix with: \
         git rm -r --cached plugins/vsdd-factory/hook-plugins/ && \
         git add -f plugins/vsdd-factory/hook-plugins/"
    );
    let tracked: HashSet<String> = tracked_raw
        .into_iter()
        .map(|p| {
            p.strip_prefix(REGISTRY_PARENT_PREFIX)
                .expect(
                    "T-012: every git-tracked WASM path must start with REGISTRY_PARENT_PREFIX; \
                     a wrong prefix here means git_tracked_wasm_names() returned an unexpected path",
                )
                .to_owned()
        })
        .collect();
    let committed_raw = git_committed_wasm_names(root);
    let committed: HashSet<String> = committed_raw
        .into_iter()
        .map(|p| {
            p.strip_prefix(REGISTRY_PARENT_PREFIX)
                .expect(
                    "T-012: every git-committed WASM path must start with REGISTRY_PARENT_PREFIX; \
                     a wrong prefix here means git_committed_wasm_names() returned an unexpected path",
                )
                .to_owned()
        })
        .collect();

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
///
/// **F8 — Non-recursive `fs::read_dir` (NIT, pre-existing):** `fs::read_dir` is
/// non-recursive; it enumerates only the top-level entries in `hook_plugins_dir`.
/// WASMs in staged bundles that live in subdirectories (e.g., a nested
/// `hook-plugins/sub/nested.wasm`) are invisible to this function and appear as
/// neither orphan nor referenced.  This asymmetry affects only T-010 and T-011
/// (staging simulation using fixture directories) — git-based controls (T-009, T-012)
/// compare registry-parent-relative paths and correctly distinguish flat vs. nested.
/// Scope to fix: S-21.14 (future story); non-recursive enumeration is a known
/// pre-existing limitation on the staging-simulation path only.
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
        // parse_plugin_refs() now returns registry-parent-relative paths
        // ("hook-plugins/foo.wasm"); compare using format!() to build the full key.
        if filename.ends_with(".wasm")
            && !hooks_refs.contains(&format!("hook-plugins/{}", filename))
            && !resolvers_refs.contains(&format!("hook-plugins/{}", filename))
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
        // parse_plugin_refs() now returns registry-parent-relative paths ("hook-plugins/foo.wasm").
        if filename.ends_with(".wasm")
            && !hooks_refs.contains(&format!("hook-plugins/{}", filename))
        {
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

/// Return the git-tracked WASM paths under `plugins/vsdd-factory/hook-plugins/`.
///
/// Runs `git ls-files plugins/vsdd-factory/hook-plugins/` from `root`, filters for
/// `.wasm` extension, and returns full git-root-relative paths (e.g.,
/// `"plugins/vsdd-factory/hook-plugins/foo.wasm"`).
///
/// `run_t012_gate()` strips the `"plugins/vsdd-factory/"` prefix before diffing against
/// the declared set, producing registry-parent-relative paths (`"hook-plugins/foo.wasm"`)
/// that match `extract_hook_plugin_name()` output (HIGH-1 correctness fix, pass-9).
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
        .map(str::to_owned)
        .collect()
}

/// Return the committed (HEAD tree) WASM paths under `plugins/vsdd-factory/hook-plugins/`.
///
/// Runs `git ls-tree --name-only -r HEAD plugins/vsdd-factory/hook-plugins/` from `root`,
/// filters for `.wasm` extension, and returns full git-root-relative paths (e.g.,
/// `"plugins/vsdd-factory/hook-plugins/foo.wasm"`).
///
/// `run_t012_gate()` strips the `"plugins/vsdd-factory/"` prefix before diffing against
/// the declared set (HIGH-1 correctness fix, pass-9; same as `git_tracked_wasm_names()`).
///
/// # Distinction from `git_tracked_wasm_names()`
///
/// `git_tracked_wasm_names()` reads the **git index** (staged + committed) via
/// `git ls-files`.  This function reads only the **HEAD commit tree** via
/// `git ls-tree --name-only -r HEAD`.  A file staged with `git add -f` but not yet
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
        .map(str::to_owned)
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

    // git_tracked_wasm_names() returns full git-root-relative paths (e.g.,
    // "plugins/vsdd-factory/hook-plugins/foo.wasm").  parse_plugin_refs() returns
    // registry-parent-relative paths ("hook-plugins/foo.wasm").  Strip the
    // module-level REGISTRY_PARENT_PREFIX before looking up in refs.
    //
    // Bypass-mutant proof: changing REGISTRY_PARENT_PREFIX to "nonexistent-prefix/"
    // (or any string that does not match the actual paths) causes every `.strip_prefix()`
    // to return `None`.  With `filter_map` that silently empties the domain so `orphans`
    // is always `[]`, and the assert always passes — the gate is invisible.
    // With `.map(|p| p.strip_prefix(…).expect(…))` the mutant panics immediately on the
    // first path, making the bypass loud and detectable.
    let mut orphans: Vec<String> = tracked_names
        .iter()
        .map(|p| {
            p.strip_prefix(REGISTRY_PARENT_PREFIX)
                .expect(
                    "T-009: every git-tracked WASM path must start with REGISTRY_PARENT_PREFIX; \
                     mutate REGISTRY_PARENT_PREFIX to 'nonexistent-prefix/' to verify this \
                     assertion fires — filter_map would have silently emptied the orphan domain",
                )
                .to_owned()
        })
        .filter(|name| {
            !hooks_refs.contains(name.as_str()) && !resolvers_refs.contains(name.as_str())
        })
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

    // HIGH-2 fixture-content assertion: verify the distinguishing syntax is present BEFORE
    // parsing. If the fixture is edited to remove the nospace form, the test becomes trivial
    // (the standard form would still be parsed, but the nospace coverage disappears silently).
    assert!(
        HOOKS_REGISTRY_NOSPACE_FIXTURE.contains("plugin=\"hook-plugins/"),
        "T-013 HIGH-2: HOOKS_REGISTRY_NOSPACE_FIXTURE must contain the nospace form \
         'plugin=\"hook-plugins/' — if this syntax is absent, the test no longer proves \
         the toml-crate handles the no-space-around-equals variant"
    );

    let refs = parse_plugin_refs(&hooks_reg);

    assert!(
        refs.contains("hook-plugins/ghost-guard-nospace.wasm"),
        "T-013 BLOCKER-1: parse_plugin_refs() must extract 'hook-plugins/ghost-guard-nospace.wasm' \
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

    // HIGH-2 fixture-content assertion: verify the distinguishing syntax is present BEFORE
    // parsing. If the fixture is edited to remove the dotslash form, the test becomes trivial.
    assert!(
        HOOKS_REGISTRY_DOTSLASH_FIXTURE.contains("\"./hook-plugins/"),
        "T-014 HIGH-2: HOOKS_REGISTRY_DOTSLASH_FIXTURE must contain the dotslash form \
         '\"./hook-plugins/' — if this syntax is absent, the test no longer proves \
         the parser handles the leading-dotslash variant"
    );

    let refs = parse_plugin_refs(&hooks_reg);

    assert!(
        refs.contains("hook-plugins/ghost-guard-dotslash.wasm"),
        "T-014 BLOCKER-1: parse_plugin_refs() must extract 'hook-plugins/ghost-guard-dotslash.wasm' \
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
    //
    // Values use registry-parent-relative path format ("hook-plugins/...") to match
    // the output of parse_plugin_refs() after the HIGH-1 path-based fix (pass-9).
    let mut hooks_declared: HashSet<String> = (0..29)
        .map(|i| format!("hook-plugins/filler-{:02}.wasm", i))
        .collect();
    hooks_declared.insert("hook-plugins/hooks-only.wasm".to_string()); // 30 total

    let resolvers_declared: HashSet<String> = ["hook-plugins/resolver.wasm".to_string()]
        .into_iter()
        .collect();

    // tracked = all 29 fillers + resolver, NOT "hook-plugins/hooks-only.wasm"
    let tracked: HashSet<String> = (0..29)
        .map(|i| format!("hook-plugins/filler-{:02}.wasm", i))
        .chain(["hook-plugins/resolver.wasm".to_string()])
        .collect();
    let committed = tracked.clone(); // same as tracked: no staged-not-committed noise

    let result =
        check_declared_subset_tracked(&hooks_declared, &resolvers_declared, &tracked, &committed);

    // (a) Must return Err when hook-plugins/hooks-only.wasm is declared but not tracked.
    assert!(
        result.is_err(),
        "T-015 AC-006: check_declared_subset_tracked must return Err when a declared \
         artifact is absent from the tracked set; got Ok"
    );

    let msg = result.unwrap_err();

    // (b) Outcome identifier per D-970 Codification 1 — must name the artifact.
    assert!(
        msg.contains("MISSING: hook-plugins/hooks-only.wasm"),
        "T-015 AC-006 D-970 Codification 1: error message must contain \
         'MISSING: hook-plugins/hooks-only.wasm'; got: {}",
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
    // Values use registry-parent-relative path format ("hook-plugins/...") to match
    // the output of parse_plugin_refs() after the HIGH-1 path-based fix (pass-9).
    let hooks_declared: HashSet<String> = (0..30)
        .map(|i| format!("hook-plugins/filler-{:02}.wasm", i))
        .collect();
    let resolvers_declared: HashSet<String> = ["hook-plugins/resolver.wasm".to_string()]
        .into_iter()
        .collect();
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
    let hooks_declared: HashSet<String> = ["hook-plugins/single-plugin.wasm".to_string()]
        .into_iter()
        .collect(); // 1 entry < 30
    let resolvers_declared: HashSet<String> = ["hook-plugins/resolver.wasm".to_string()]
        .into_iter()
        .collect();
    let tracked: HashSet<String> = HashSet::new();
    let committed: HashSet<String> = HashSet::new();
    // Calls the real function; unwrap panics with the Err message.
    check_declared_subset_tracked(&hooks_declared, &resolvers_declared, &tracked, &committed)
        .unwrap_or_else(|e| panic!("{}", e));
}

// ---------------------------------------------------------------------------
// T-020 — EC-005b control: proves EC-005b fires when tracked set is empty
//
// check_declared_subset_tracked() step EC-005b asserts the tracked set is non-empty.
// An empty tracked set would produce noisy false failures on every declared artifact;
// EC-005b converts this into a clearly-named error.
//
// In the real gate path, EC-005a in run_t012_gate fires first (assert on raw git
// output before the set is built).  EC-005b is reachable only via fixture-empty sets
// (as in this test) or when check_declared_subset_tracked is called directly.
//
// This test calls the REAL function with 30 hooks + 1 resolver (passes floors) but
// an empty tracked set.  `#[should_panic]` locks the "T-012 EC-005b" identifier.
//
// Mutation-proof: removing EC-005b causes the function to proceed to step 3, where
// every declared artifact is "missing".  The panic message becomes a MISSING: chain,
// NOT "T-012 EC-005b", so `#[should_panic(expected = "T-012 EC-005b")]` FAILS.
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
#[should_panic(expected = "T-012 EC-005b")]
fn test_S_21_09_ac006_T020_ec005_fires_on_empty_tracked_set() {
    let hooks_declared: HashSet<String> = (0..30)
        .map(|i| format!("hook-plugins/filler-{:02}.wasm", i))
        .collect();
    let resolvers_declared: HashSet<String> = ["hook-plugins/resolver.wasm".to_string()]
        .into_iter()
        .collect();
    let tracked: HashSet<String> = HashSet::new(); // empty → EC-005b fires
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
    // Values use registry-parent-relative path format ("hook-plugins/...") to match
    // the output of parse_plugin_refs() after the HIGH-1 path-based fix (pass-9).
    let hooks_declared: HashSet<String> = (0..30)
        .map(|i| format!("hook-plugins/filler-{:02}.wasm", i))
        .collect();
    let resolvers_declared: HashSet<String> = ["hook-plugins/resolver.wasm".to_string()]
        .into_iter()
        .collect();

    // tracked includes hook-plugins/staged-plugin.wasm; committed does NOT (simulates
    // `git add -f` without a subsequent commit).
    let mut tracked: HashSet<String> = hooks_declared
        .iter()
        .cloned()
        .chain(resolvers_declared.iter().cloned())
        .collect();
    tracked.insert("hook-plugins/staged-plugin.wasm".to_string());
    let committed: HashSet<String> = tracked
        .iter()
        .filter(|n| *n != "hook-plugins/staged-plugin.wasm")
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
        msg.contains("  STAGED-NOT-COMMITTED: hook-plugins/staged-plugin.wasm"),
        "T-021 AC-006 S-21.09 D-970 Codification 1: error message must contain \
         '  STAGED-NOT-COMMITTED: hook-plugins/staged-plugin.wasm'; got: {}",
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
// EC-005b; the panic message becomes "T-012 EC-005b", NOT "T-012: resolvers registry
// declared set is empty", so `#[should_panic]` FAILS.
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
#[should_panic(expected = "T-012: resolvers registry declared set is empty")]
fn test_S_21_09_ac006_T022_resolvers_floor_fires_on_empty_resolvers_set() {
    let hooks_declared: HashSet<String> = (0..30)
        .map(|i| format!("hook-plugins/filler-{:02}.wasm", i))
        .collect(); // passes hooks floor
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
// **Excluded region (from `declared`; now caught by `detect_ungated_declarations`):**
//   Paths that resolve to a location outside registry_parent/hook-plugins/ —
//   `extract_hook_plugin_name` returns `None`; but `detect_ungated_declarations` catches
//   those that still resolve inside registry_parent:
//   - Bare names: `ghost-bare.wasm` → registry_parent/ghost-bare.wasm (T-043)
//   - Traversal-cancels: `hook-plugins/../ghost.wasm` → registry_parent/ghost.wasm (T-043 class)
//   - `../registry-parent/` prefix: `../vsdd-factory/ghost.wasm` → same (T-044)
//   - Divergent-prefix absolute paths: `/abs/hook-plugins/foo.wasm` (T-026) — fail gate-2,
//     not caught by detect_ungated_declarations (load from outside repo)
//   - `../` prefix escaping registry_parent: `../hook-plugins/foo.wasm` → parent(registry_parent)/hook-plugins/foo.wasm — fail gate-2 in both functions
//
// **Can harmful content occupy the excluded region? (LOW-1 — CLOSED pass-10)**
//   Bare names, traversal-cancels, and `../`-prefix forms resolve WITHIN the repo but
//   outside `hook-plugins/`.  `extract_hook_plugin_name` still returns `None` for them
//   (gate-1 threshold `< expected_depth + 2` unchanged) so they do not appear in
//   `declared`.  But starting pass-10, `detect_ungated_declarations()` uses
//   gate-1 threshold `< expected_depth + 1` to catch ALL declarations whose resolved
//   path lands inside `registry_parent` but outside `registry_parent/hook-plugins/` —
//   covering bare names (T-043), `../registry_parent/` forms (T-044), and gate-3
//   escapes (T-038).  They surface as `UNGATED-DECLARATION: <path>` in `run_t012_gate()`
//   before any git calls.
//   Divergent-prefix absolute paths (`/abs/path/foo.wasm`) fail gate-2 in both
//   `extract_hook_plugin_name` and `detect_ungated_declarations` and remain outside
//   scope (they load from outside the repo).  Absolute paths whose lex_norm components
//   match the registry parent prefix ARE admitted by gate-2; they compare against git
//   the same as relative paths (see updated "Why divergent-prefix absolute paths are
//   excluded" doc).  No covert path class remains after pass-10.
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
         ('ghost-bare.wasm' resolves to expected_depth+1 components — gate 1 \
         minimum-length check fires before a hook-plugins component can be inspected); \
         got refs: {:?}",
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
         normalises to expected_depth+1 components — gate 1 minimum-length check fires; \
         demonstrated: ParentDir pop is load-bearing — with the pop active, hook-plugins \
         is removed and path has depth+1 components; refs_cancels is empty here; \
         removing the pop retains hook-plugins at depth position (depth+2 components, \
         gate 1 passes, gate 3 admits ghost-cancels.wasm); got refs: {:?}",
        refs_cancels
    );

    // (c) `../`-prefix form (LOW-2 execution): `plugin = "../hook-plugins/evil.wasm"` — resolves
    //     to parent(registry_parent)/hook-plugins/evil.wasm, OUTSIDE registry_parent.
    //     lex_norm: RootDir clears, then builds ["tmp", "hook-plugins", "evil.wasm"] (3 components).
    //     parent_parts for registry_parent = ["tmp", "<tmpdir>"] (expected_depth = 2).
    //     Gate 1: len=3 < expected_depth+2=4 → fires, returns None; EXCLUDED.
    let registry_dotdot = tmp.path().join("registry-dotdot.toml");
    fs::write(
        &registry_dotdot,
        concat!(
            "schema_version = 2\n",
            "[[hooks]]\n",
            "name = \"evil\"\n",
            "event = \"PreToolUse\"\n",
            "tool = \"^Bash$\"\n",
            "plugin = \"../hook-plugins/evil.wasm\"\n",
            "timeout_ms = 5000\n",
            "on_error = \"continue\"\n",
        ),
    )
    .expect("../hook-plugins/ registry must be written to tempfile");

    let refs_dotdot = parse_plugin_refs(&registry_dotdot);

    assert!(
        refs_dotdot.is_empty(),
        "T-023(c) LOW-2 `../`-prefix EXCLUDED: `../hook-plugins/evil.wasm` resolves to \
         parent(registry_parent)/hook-plugins/evil.wasm; lex_norm gives depth+1 components \
         (path escapes registry_parent via `..`) — gate 1 minimum-length check fires before \
         any hook-plugins component can be inspected; got refs: {:?}",
        refs_dotdot
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
        refs.contains("hook-plugins/ghost-traversal.wasm"),
        "T-025 F-1: parse_plugin_refs must extract 'hook-plugins/ghost-traversal.wasm' \
         from a registry using the traversal form \
         (plugin = \"hooks/../hook-plugins/ghost-traversal.wasm\") — \
         extract_hook_plugin_name() resolves '..' via ParentDir pop before testing \
         for the hook-plugins component; got refs: {:?}",
        refs
    );
}

// ---------------------------------------------------------------------------
// T-026 — MEDIUM-2 + HIGH-1 pass-6: absolute-form plugin path is EXCLUDED; depth-matched
//         sub-test proves prefix-verification loop is load-bearing (not just length check)
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
// ## Pass-6 HIGH-1: depth-matched fixture proves prefix loop is load-bearing
//
// Sub-test (a) uses `/abs/hook-plugins/ghost-absolute.wasm` — only 2 Normal components
// before `hook-plugins`.  When `expected_depth` is the tempdir depth (≥ 5 on macOS),
// the minimum-length check `joined_parts.len() < expected_depth + 2` rejects this path
// BEFORE the prefix loop runs — the exclusion is depth-coincidental.
//
// Sub-test (b) constructs an absolute path with EXACTLY `expected_depth` Normal
// components before `hook-plugins`, so `len == expected_depth + 2` passes the length
// check.  The PREFIX-VERIFICATION LOOP (comparing `joined_parts[0..expected_depth]`
// against `parent_parts`) is now the operative gate.  Deleting the prefix loop would
// admit this path → `refs_depth.contains("evil.wasm")` fires RED.
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
fn test_S_21_09_ac006_T026_absolute_form_excluded_from_declared() {
    let tmp = tempdir().expect("tempdir must create successfully");

    // ---- (a) Short absolute path — excluded by minimum-length check ----
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
        "T-026(a) MEDIUM-2: absolute-path plugin declarations must NOT be included \
         in declared — production resolve_plugin_paths() passes absolute paths unchanged \
         (loads from /abs/hook-plugins/..., not from registry_parent/hook-plugins/); \
         including them produces false-positive MISSING outcomes; got refs: {:?}",
        refs
    );
    assert!(
        refs.is_empty(),
        "T-026(a): expected empty refs for short absolute-path declaration; got: {:?}",
        refs
    );

    // ---- (b) Depth-matched absolute path — prefix loop is the operative gate ----
    //
    // Build /seg0/seg1/.../seg_{N-1}/hook-plugins/evil.wasm where N == expected_depth.
    // All seg* components differ from the real tempdir components, so the prefix-
    // verification loop rejects them even though len == expected_depth + 2 passes the
    // minimum-length check.  Mutation proof: deleting the prefix loop in
    // extract_hook_plugin_name() admits "evil.wasm" (refs_depth becomes {"evil.wasm"}).
    let expected_depth = tmp
        .path()
        .components()
        .filter(|c| matches!(c, std::path::Component::Normal(_)))
        .count();

    let mut abs_depth = PathBuf::from("/");
    for i in 0..expected_depth {
        abs_depth.push(format!("seg{}", i));
    }
    abs_depth.push("hook-plugins");
    abs_depth.push("evil.wasm");

    let abs_depth_str = abs_depth
        .to_str()
        .expect("depth-matched absolute path must be valid UTF-8");

    let registry_depth = tmp.path().join("registry-depth.toml");
    let depth_content = format!(
        "schema_version = 2\n[[hooks]]\nname = \"evil\"\nevent = \"PreToolUse\"\n\
         tool = \"^Bash$\"\nplugin = \"{}\"\ntimeout_ms = 5000\non_error = \"continue\"\n",
        abs_depth_str
    );
    fs::write(&registry_depth, &depth_content)
        .expect("depth-matched absolute registry must be written to tempfile");

    let refs_depth = parse_plugin_refs(&registry_depth);

    assert!(
        !refs_depth.contains("evil.wasm"),
        "T-026(b) HIGH-1 depth-matched: absolute path /seg0/.../seg{}/hook-plugins/evil.wasm \
         has len == expected_depth + 2, passing the minimum-length check; the \
         prefix-verification loop is the operative gate (joined_parts[0..expected_depth] \
         are seg* components, not the real tempdir components); deleting the prefix loop \
         admits evil.wasm; got refs_depth: {:?}",
        expected_depth - 1,
        refs_depth
    );
    assert!(
        refs_depth.is_empty(),
        "T-026(b): expected empty refs for depth-matched absolute path; got: {:?}",
        refs_depth
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
// check_declared_subset_tracked proceeds to EC-005b (empty tracked → different error);
// `#[should_panic(expected = "... has only 29 entries")]` FAILS — mutation detected.
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
#[should_panic(expected = "T-012: hooks registry declared set has only 29 entries")]
fn test_S_21_09_ac006_T027_hooks_floor_fires_on_29_entry_set() {
    let hooks_declared: HashSet<String> = (0..29)
        .map(|i| format!("hook-plugins/filler-{:02}.wasm", i))
        .collect(); // 29 < 30 → floor fires
    let resolvers_declared: HashSet<String> = ["hook-plugins/resolver.wasm".to_string()]
        .into_iter()
        .collect();
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
        let init_out = Command::new("git")
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
            .output()
            .expect("git init must execute for T-030 phase B fixture");
        assert!(
            init_out.status.success(),
            "T-030 phase B: git init failed: {}",
            String::from_utf8_lossy(&init_out.stderr)
        );

        Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(root)
            .output()
            .expect("git config user.email must execute");
        Command::new("git")
            .args(["config", "user.name", "Test"])
            .current_dir(root)
            .output()
            .expect("git config user.name must execute");

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

        let add_out = Command::new("git")
            .args(["-c", "core.excludesFile=/dev/null", "add", "."])
            .current_dir(root)
            .output()
            .expect("git add must execute for T-030 phase B fixture");
        assert!(
            add_out.status.success(),
            "T-030 phase B: git add failed: {}",
            String::from_utf8_lossy(&add_out.stderr)
        );

        let commit_out = Command::new("git")
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
            .output()
            .expect("git commit must execute for T-030 phase B fixture");
        assert!(
            commit_out.status.success(),
            "T-030 phase B: git commit failed: {}",
            String::from_utf8_lossy(&commit_out.stderr)
        );

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
            result
                .unwrap_err()
                .contains("MISSING: hook-plugins/ctx.wasm"),
            "T-030 phase B D-970 Codification 1: error must contain 'MISSING: hook-plugins/ctx.wasm'"
        );
    }
}

// ---------------------------------------------------------------------------
// T-031 — MEDIUM-4(a) case-variant admission control: `Hook-Plugins/x.wasm` enters declared
//
// On macOS's case-insensitive default filesystem, `Hook-Plugins/x.wasm` and
// `hook-plugins/x.wasm` refer to the same directory.  The pass-4 implementation used
// `c == "hook-plugins"` — a case-sensitive string comparison — so `Hook-Plugins/x.wasm`
// would return None from extract_hook_plugin_name() and not enter `declared` at all,
// leaving `declared ⊆ tracked` trivially true — a silent false negative.
//
// Pass-5 fix: `extract_hook_plugin_name()` uses `eq_ignore_ascii_case("hook-plugins")`
// in gate 3, admitting the case variant INTO `declared` as `"Hook-Plugins/ghost-case.wasm"`
// (verbatim — NOT lowercased, per pass-9.1 correctness fix).
//
// Pass-9.1 fix (this commit): the hook-plugins component is no longer lowercased on return.
// The declared identifier is `"Hook-Plugins/ghost-case.wasm"`.  When compared
// case-sensitively against a lowercase git-tracked set, this fires `MISSING:` — which is
// correct on both platforms: on Linux CI (case-sensitive FS) production genuinely cannot
// load `Hook-Plugins/…`; on macOS the gate conservatively rejects the mismatch, requiring
// operators to use lowercase `hook-plugins/` in declarations.
//
// Mutation-proof: reverting gate 3 to `c == "hook-plugins"` (case-sensitive) causes
// `Hook-Plugins/x.wasm` to return None; `refs.contains("Hook-Plugins/ghost-case.wasm")`
// FAILS (empty refs).  The old lowercasing mutation would return the lowercase form;
// `refs.contains("Hook-Plugins/ghost-case.wasm")` FAILS (wrong case in returned path).
//
// T-042 is the complementary MISSING-outcome control (end-to-end via
// check_declared_subset_tracked).
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
        refs.contains("Hook-Plugins/ghost-case.wasm"),
        "T-031 MEDIUM-4(a): parse_plugin_refs must include 'Hook-Plugins/ghost-case.wasm' \
         (verbatim) from a registry using the case-variant form \
         (plugin = \"Hook-Plugins/ghost-case.wasm\") — extract_hook_plugin_name() uses \
         eq_ignore_ascii_case for gate-3 to admit the variant, then returns verbatim; \
         mutation-proof: c == \"hook-plugins\" (case-sensitive gate-3) returns None, \
         leaving refs empty; lowercasing mutation returns lowercase form, failing this \
         assertion; got refs: {:?}",
        refs
    );
}

// ---------------------------------------------------------------------------
// T-032 — MEDIUM-1 nested-subdir control: `hook-plugins/sub/nested.wasm` yields full path
//
// The pass-4 implementation returned `normalized.get(pos + 1)` — the component
// IMMEDIATELY after `hook-plugins`.  For `hook-plugins/sub/nested.wasm` this returns
// `sub`, not the artifact filename — causing a false MISSING identifier (MEDIUM-1).
//
// Pass-5 fix: `extract_hook_plugin_name()` returns `joined_parts.last()` (the final
// component of the normalised resolved path), which is the filename regardless of depth.
//
// Pass-9 fix (HIGH-1): returns `joined_parts[expected_depth..].join("/")` — the full
// registry-parent-relative path — preserving subdirectory structure so that a declaration
// at `hook-plugins/sub/nested.wasm` is distinct from a WASM committed flat at
// `hook-plugins/nested.wasm`.
//
// Mutation-proof (current): reverting to `normalized.get(pos + 1)` would return
// `Some("sub")`; `refs.contains("hook-plugins/sub/nested.wasm")` FAILS.
// Reverting to `last()` (pass-5 form) would return `Some("nested.wasm")` (bare);
// `refs.contains("hook-plugins/sub/nested.wasm")` FAILS — the full-path form is now required.
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
        refs.contains("hook-plugins/sub/nested.wasm"),
        "T-032 MEDIUM-1: parse_plugin_refs must extract 'hook-plugins/sub/nested.wasm' \
         (full registry-parent-relative path) from a nested declaration \
         (plugin = \"hook-plugins/sub/nested.wasm\") — \
         extract_hook_plugin_name() returns joined_parts[expected_depth..].join(\"/\"); \
         mutation-proof: reverting to pos+1 returns 'sub', reverting to last() returns \
         bare 'nested.wasm'; both cause this assertion to FAIL; \
         got refs: {:?}",
        refs
    );
    // Also assert 'sub' is NOT in refs (wrong component from the pos+1 bug).
    assert!(
        !refs.contains("sub"),
        "T-032 MEDIUM-1: 'sub' must NOT appear in refs — it is a directory component, \
         not the artifact path; got refs: {:?}",
        refs
    );
    // Also assert bare 'nested.wasm' is NOT in refs (the pass-5 last() regression).
    assert!(
        !refs.contains("nested.wasm"),
        "T-032 HIGH-1: bare 'nested.wasm' must NOT appear in refs — the full path \
         'hook-plugins/sub/nested.wasm' must be used (pass-9 path-based fix); \
         got refs: {:?}",
        refs
    );
}

// ---------------------------------------------------------------------------
// T-033 — MEDIUM-3 minimum-length lower boundary: `hook-plugins` (no filename) excluded
//
// The minimum-length check in `extract_hook_plugin_name()`:
//
//   if joined_parts.len() < expected_depth + 2 { return None; }
//
// requires at least TWO components after registry_parent:
//   [registry_parent_parts..., "hook-plugins", "<filename>"]
//
// For `plugin = "hook-plugins"` (the directory itself — no trailing filename):
//   joined_parts = [registry_parent_parts..., "hook-plugins"]
//   len = expected_depth + 1 < expected_depth + 2 → excluded (correct)
//
// The + 3 direction is already caught by T-013/T-014/T-025/T-031:
//   `hook-plugins/foo.wasm` has len = expected_depth + 2, which is NOT < expected_depth + 3,
//   so weakening to `< expected_depth + 3` would exclude those fixtures → those tests fail.
//
// Together this test (lower boundary) + T-013/014/025/031 (upper direction) form the
// boundary pair that constrains the `+ 2` constant per POLICY 13.
//
// Mutation-proof: weakening to `< expected_depth + 1` allows len = expected_depth + 1
// through; the hook-plugins component check passes (component at expected_depth is
// "hook-plugins"); `last()` returns "hook-plugins" as the filename → declared contains
// "hook-plugins" → false `MISSING: hook-plugins`.  Strengthening to
// `< expected_depth + 3` would exclude T-013/014/025/031 (len = expected_depth + 2
// < expected_depth + 3 → all those tests fail).
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
fn test_S_21_09_ac006_T033_minimum_length_hookplugins_dir_only_excluded() {
    let tmp = tempdir().expect("tempdir must create successfully");
    let registry = tmp.path().join("registry.toml");
    fs::write(
        &registry,
        concat!(
            "schema_version = 2\n",
            "[[hooks]]\n",
            "name = \"ghost-dir\"\n",
            "event = \"PreToolUse\"\n",
            "tool = \"^Bash$\"\n",
            "plugin = \"hook-plugins\"\n",
            "timeout_ms = 5000\n",
            "on_error = \"continue\"\n"
        ),
    )
    .expect("directory-only registry must be written to tempfile");

    let refs = parse_plugin_refs(&registry);

    assert!(
        refs.is_empty(),
        "T-033 MEDIUM-3: parse_plugin_refs must exclude a plugin path of exactly \
         'hook-plugins' (directory only, no filename component) — after resolving \
         relative to registry_parent, joined_parts.len() == expected_depth + 1, which \
         is < expected_depth + 2, so the minimum-length check returns None; \
         mutation-proof: 'len < expected_depth + 1' (weaker) passes this through, \
         last() returns 'hook-plugins' as the filename producing a false \
         MISSING: hook-plugins; the + 3 direction is caught by T-013/014/025/031; \
         got refs: {:?}",
        refs
    );
}

// ---------------------------------------------------------------------------
// T-034 — MEDIUM-4 `-r` flag control: git ls-tree -r finds nested committed WASMs
//
// `git_committed_wasm_names()` runs `git ls-tree --name-only -r HEAD
// plugins/vsdd-factory/hook-plugins/` to list committed artifacts.  Without `-r`,
// `git ls-tree` is non-recursive: subdirectory entries appear as tree objects
// (`040000 tree ...`), not as file paths, so they are filtered out by the
// `.ends_with(".wasm")` check.  A nested WASM at `hook-plugins/sub/nested.wasm`
// would be invisible to a non-recursive `ls-tree`.
//
// T-032 proves the DECLARED side: `hook-plugins/sub/nested.wasm` in a registry yields
// `nested.wasm` as the declared artifact name.  This test proves the COMMITTED side:
// a WASM committed at `hook-plugins/sub/nested.wasm` IS visible to
// `git_committed_wasm_names()` (because `-r` is present), making end-to-end
// cross-checking between declared and committed correct for nested declarations.
//
// Mutation-proof: dropping `-r` from `git ls-tree` causes the subdirectory entry
// (`hook-plugins/sub`) to appear as a tree line rather than a file path; the
// `.ends_with(".wasm")` filter excludes it; `committed` is empty;
// `assert!(committed.contains("nested.wasm"))` FAILS.
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
fn test_S_21_09_ac006_T034_git_ls_tree_r_finds_nested_committed_wasm() {
    let tmp = tempdir().expect("tempdir must create successfully");
    let root = tmp.path();

    // Initialise a git repo with -c overrides to prevent global config interference
    // (same discipline as T-030 Phase B).
    let init_out = Command::new("git")
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
        .output()
        .expect("git init must execute for T-034 fixture");
    assert!(
        init_out.status.success(),
        "T-034: git init failed: {}",
        String::from_utf8_lossy(&init_out.stderr)
    );

    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(root)
        .output()
        .expect("git config user.email must execute");
    Command::new("git")
        .args(["config", "user.name", "Test"])
        .current_dir(root)
        .output()
        .expect("git config user.name must execute");

    // Create nested hook-plugins/sub/nested.wasm (simulates a real nested artifact).
    let hook_plugins_sub = root.join("plugins/vsdd-factory/hook-plugins/sub");
    fs::create_dir_all(&hook_plugins_sub).expect("hook-plugins/sub dir must be created");
    fs::write(hook_plugins_sub.join("nested.wasm"), b"wasm")
        .expect("nested.wasm fixture must be written");

    let add_out = Command::new("git")
        .args(["-c", "core.excludesFile=/dev/null", "add", "."])
        .current_dir(root)
        .output()
        .expect("git add must execute for T-034 fixture");
    assert!(
        add_out.status.success(),
        "T-034: git add failed: {}",
        String::from_utf8_lossy(&add_out.stderr)
    );

    let commit_out = Command::new("git")
        .args([
            "-c",
            "core.excludesFile=/dev/null",
            "-c",
            "commit.gpgsign=false",
            "-c",
            "core.hooksPath=/dev/null",
            "commit",
            "-m",
            "T-034 fixture: nested WASM",
        ])
        .current_dir(root)
        .output()
        .expect("git commit must execute for T-034 fixture");
    assert!(
        commit_out.status.success(),
        "T-034: git commit failed: {}",
        String::from_utf8_lossy(&commit_out.stderr)
    );

    // git_committed_wasm_names() uses `git ls-tree -r HEAD` → must surface nested.wasm.
    // Without -r, the subdirectory appears as a tree object and .ends_with(".wasm")
    // excludes it — committed would be empty and the assertion below would FAIL.
    let committed = git_committed_wasm_names(root);

    // git_committed_wasm_names() now returns full git-root-relative paths (HIGH-1 fix, pass-9).
    // The prefix "plugins/vsdd-factory/" is stripped by run_t012_gate(), not here.
    assert!(
        committed.contains(&"plugins/vsdd-factory/hook-plugins/sub/nested.wasm".to_string()),
        "T-034 MEDIUM-4: git_committed_wasm_names must return \
         'plugins/vsdd-factory/hook-plugins/sub/nested.wasm' (full git path) for a WASM \
         committed at hook-plugins/sub/nested.wasm — the -r flag makes ls-tree recursive; \
         mutation-proof: dropping -r causes the nested WASM to be absent from committed; \
         got committed: {:?}",
        committed
    );
}

// ---------------------------------------------------------------------------
// T-035 — HIGH-1 pass-7: gate-3 (hook-plugins component check) isolated control
//
// `plugin = "other-dir/evil-probe.wasm"` resolves relative to registry_parent to
// [registry_parent_parts..., "other-dir", "evil-probe.wasm"].
//
// Gate-by-gate traversal:
//
//   Gate 1 (minimum-length): joined_parts.len() == expected_depth + 2  → PASSES
//     (len == expected_depth + 2 ≥ expected_depth + 2; the `<` check is false)
//   Gate 2 (prefix loop): joined_parts[0..expected_depth] == parent_parts → PASSES
//     ("other-dir" sits at index expected_depth, outside [0..expected_depth])
//   Gate 3 (hook-plugins component): joined_parts[expected_depth] == "other-dir"
//     ≠ "hook-plugins" → EXCLUDED (sole reason for exclusion)
//
// Captured mutation proof (pass-7):
//   Mutant: `if !hook_comp.eq_ignore_ascii_case("hook-plugins") { return None; }` deleted.
//   $ cargo test --package factory-dispatcher --test bundle_orphan_check \
//         -- test_S_21_09_ac006_T035
//   FAILED: T-035 HIGH-1 gate-3 isolated control: parse_plugin_refs must exclude
//   'other-dir/evil-probe.wasm' — path normalises to expected_depth+2 components
//   (passes gate 1) with parent prefix intact (passes gate 2) but
//   'other-dir' != 'hook-plugins' (gate 3 fires); mutation proof: deleting gate 3
//   admits evil-probe.wasm into declared; got refs: {"evil-probe.wasm"}
//   → gate 3 deletion admits evil-probe.wasm; gates 1+2 alone cannot reject it.
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
fn test_S_21_09_ac006_T035_gate3_hookplugins_component_check_isolated_control() {
    let tmp = tempdir().expect("tempdir must create successfully");

    let registry = tmp.path().join("hooks-registry.toml");
    fs::write(
        &registry,
        concat!(
            "schema_version = 2\n",
            "[[hooks]]\n",
            "name = \"evil-probe\"\n",
            "event = \"PreToolUse\"\n",
            "tool = \"^Bash$\"\n",
            "plugin = \"other-dir/evil-probe.wasm\"\n",
            "timeout_ms = 5000\n",
            "on_error = \"continue\"\n",
        ),
    )
    .expect("gate-3 control registry must be written to tempfile");

    let refs = parse_plugin_refs(&registry);

    assert!(
        refs.is_empty(),
        "T-035 HIGH-1 gate-3 isolated control: parse_plugin_refs must exclude \
         'other-dir/evil-probe.wasm' — path normalises to expected_depth+2 components \
         (passes gate 1) with parent prefix intact (passes gate 2) but \
         'other-dir' != 'hook-plugins' (gate 3 fires); \
         mutation proof: deleting gate 3 admits evil-probe.wasm into declared; \
         got refs: {:?}",
        refs
    );
}

// ---------------------------------------------------------------------------
// T-036 — HIGH-1 pass-8 M15 killer: gitignored probe not force-added
//
// `git_tracked_wasm_names()` calls `git ls-files plugins/vsdd-factory/hook-plugins/`.
// M15 mutant replaces this with an `fs::read_dir` filesystem scan.
//
// This fixture mirrors production: `plugins/vsdd-factory/hook-plugins/` is gitignored.
// The 30 hook WASMs are force-added (`git add -f`) and committed. `gitignored-probe.wasm`
// is declared in the registry AND written to disk AFTER the commit, but is never
// force-added — it stays in the gitignored directory, invisible to `git ls-files`.
//
// Correct behaviour (`git ls-files`): tracked = {h00..h29, ctx.wasm}
//   → declared − tracked = {gitignored-probe.wasm} → MISSING: gitignored-probe.wasm.
//
// Captured M15 mutation proof (pass-8):
//   Mutant: git_tracked_wasm_names() replaced with fs::read_dir filesystem scan.
//   $ cargo test --package factory-dispatcher --test bundle_orphan_check \
//         -- test_S_21_09_ac006_T036
//   FAILED: T-036 M15 killer: error must contain 'MISSING: gitignored-probe.wasm';
//   got: "...STAGED-NOT-COMMITTED: gitignored-probe.wasm"
//   → M15 scan includes gitignored-probe.wasm in tracked; declared−tracked is empty;
//     committed (git ls-tree HEAD) lacks it → tracked−committed fires STAGED-NOT-COMMITTED
//     instead; second assertion fails with unexpected outcome identifier.
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
fn test_S_21_09_ac006_T036_gitignored_probe_not_force_added_fires_missing() {
    let tmp = tempdir().expect("tempdir must create successfully");
    let root = tmp.path();

    // Initialise a git repo with -c overrides (same discipline as T-030 Phase B).
    let init_out = Command::new("git")
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
        .output()
        .expect("git init must execute for T-036 fixture");
    assert!(
        init_out.status.success(),
        "T-036: git init failed: {}",
        String::from_utf8_lossy(&init_out.stderr)
    );

    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(root)
        .output()
        .expect("git config user.email must execute");
    Command::new("git")
        .args(["config", "user.name", "Test"])
        .current_dir(root)
        .output()
        .expect("git config user.name must execute");

    let plugins_dir = root.join("plugins/vsdd-factory");
    let hook_plugins_dir = plugins_dir.join("hook-plugins");
    fs::create_dir_all(&hook_plugins_dir).expect("hook-plugins dir must be created");

    // Write .gitignore that mirrors production: hook-plugins/ is gitignored.
    fs::write(
        root.join(".gitignore"),
        "plugins/vsdd-factory/hook-plugins/\n",
    )
    .expect(".gitignore must be written");

    // hooks-registry: 30 hooks h00..h29 (all committed) + gitignored-probe.wasm (declared only).
    let mut hooks_content = String::from("schema_version = 2\n");
    for i in 0..30_u32 {
        hooks_content.push_str(&format!(
            "[[hooks]]\nname = \"h{i:02}\"\nplugin = \"hook-plugins/h{i:02}.wasm\"\n\
             event = \"PreToolUse\"\ntool = \"^Bash$\"\ntimeout_ms = 5000\n\
             on_error = \"continue\"\n",
        ));
    }
    hooks_content.push_str(
        "[[hooks]]\nname = \"gitignored-probe\"\nplugin = \"hook-plugins/gitignored-probe.wasm\"\n\
         event = \"PreToolUse\"\ntool = \"^Bash$\"\ntimeout_ms = 5000\n\
         on_error = \"continue\"\n",
    );
    fs::write(plugins_dir.join("hooks-registry.toml"), &hooks_content)
        .expect("hooks-registry.toml must be written");

    // resolvers-registry: 1 entry (ctx.wasm — also committed).
    fs::write(
        plugins_dir.join("resolvers-registry.toml"),
        "schema_version = 1\n[[resolvers]]\nname = \"ctx\"\n\
         plugin = \"hook-plugins/ctx.wasm\"\n",
    )
    .expect("resolvers-registry.toml must be written");

    // Write h00..h29.wasm + ctx.wasm (NOT gitignored-probe.wasm yet — written after commit).
    // Also write config.yaml to hook-plugins/ as a non-WASM file (LOW-2 filter control).
    for i in 0..30_u32 {
        fs::write(hook_plugins_dir.join(format!("h{i:02}.wasm")), b"wasm")
            .expect("hook wasm must be written");
    }
    fs::write(hook_plugins_dir.join("ctx.wasm"), b"wasm").expect("ctx.wasm must be written");
    // LOW-2: force-add a non-WASM file to hook-plugins/; the .ends_with(".wasm") filter
    // in git_tracked_wasm_names() must exclude it from the tracked set.
    fs::write(hook_plugins_dir.join("config.yaml"), b"key: value")
        .expect("config.yaml must be written for LOW-2 filter control");

    // Stage .gitignore and registries (these are outside hook-plugins/ and not gitignored).
    let add_root_out = Command::new("git")
        .args([
            "-c",
            "core.excludesFile=/dev/null",
            "add",
            ".gitignore",
            "plugins/vsdd-factory/hooks-registry.toml",
            "plugins/vsdd-factory/resolvers-registry.toml",
        ])
        .current_dir(root)
        .output()
        .expect("git add registries must execute for T-036 fixture");
    assert!(
        add_root_out.status.success(),
        "T-036: git add registries failed: {}",
        String::from_utf8_lossy(&add_root_out.stderr)
    );

    // Force-add all WASMs currently in hook-plugins/ (only h00..h29 + ctx.wasm exist now).
    // -f bypasses the .gitignore that covers this directory.
    let add_wasm_out = Command::new("git")
        .args([
            "-c",
            "core.excludesFile=/dev/null",
            "add",
            "-f",
            "plugins/vsdd-factory/hook-plugins/",
        ])
        .current_dir(root)
        .output()
        .expect("git add -f hook-plugins/ must execute for T-036 fixture");
    assert!(
        add_wasm_out.status.success(),
        "T-036: git add -f hook-plugins/ failed: {}",
        String::from_utf8_lossy(&add_wasm_out.stderr)
    );

    let commit_out = Command::new("git")
        .args([
            "-c",
            "core.excludesFile=/dev/null",
            "-c",
            "commit.gpgsign=false",
            "-c",
            "core.hooksPath=/dev/null",
            "commit",
            "-m",
            "T-036 fixture: 30 hooks committed; gitignored-probe not force-added",
        ])
        .current_dir(root)
        .output()
        .expect("git commit must execute for T-036 fixture");
    assert!(
        commit_out.status.success(),
        "T-036: git commit failed: {}",
        String::from_utf8_lossy(&commit_out.stderr)
    );

    // Write gitignored-probe.wasm to disk AFTER the commit — it now exists on the filesystem
    // but was never staged or committed. git ls-files will not see it (gitignored + not forced);
    // M15 (ls-based scan) would see it on disk and falsely suppress the MISSING outcome.
    fs::write(hook_plugins_dir.join("gitignored-probe.wasm"), b"wasm")
        .expect("gitignored-probe.wasm must be written to disk");

    // gitignored-probe.wasm: on disk, declared, but NOT in git index.
    // git ls-files does not see it; M15 (`ls`) would see it and suppress the MISSING.
    let result = run_t012_gate(root);

    assert!(
        result.is_err(),
        "T-036 M15 killer: run_t012_gate must return Err with MISSING: gitignored-probe.wasm; \
         git ls-files excludes gitignored files even when on disk; \
         M15 mutant (ls instead of git ls-files) would include gitignored-probe.wasm in tracked \
         and suppress the MISSING, returning Ok — this assertion would FAIL"
    );

    let err = result.unwrap_err();
    assert!(
        err.contains("MISSING: hook-plugins/gitignored-probe.wasm"),
        "T-036 M15 killer: error must contain 'MISSING: hook-plugins/gitignored-probe.wasm'; \
         got: {:?}",
        err
    );

    // MEDIUM-2: verify the .gitignore pattern is load-bearing — git must confirm
    // gitignored-probe.wasm is actually gitignored; if the pattern is wrong the file
    // would be visible to git ls-files and the MISSING outcome would be suppressed.
    let check_ignore_out = Command::new("git")
        .args([
            "check-ignore",
            "-q",
            "plugins/vsdd-factory/hook-plugins/gitignored-probe.wasm",
        ])
        .current_dir(root)
        .output()
        .expect("git check-ignore must execute for T-036 MEDIUM-2 load-bearing assertion");
    assert!(
        check_ignore_out.status.success(),
        "T-036 MEDIUM-2: git check-ignore must confirm \
         plugins/vsdd-factory/hook-plugins/gitignored-probe.wasm is gitignored; \
         if the .gitignore pattern is wrong, the probe would be visible to git ls-files \
         and the MISSING outcome would be suppressed — making this a false-positive-free test \
         requires the gitignore pattern to actually work"
    );

    // LOW-2: verify the .ends_with(".wasm") filter in git_tracked_wasm_names() is
    // load-bearing — a non-WASM file committed in hook-plugins/ must NOT appear in
    // the tracked set.  Call git_tracked_wasm_names() directly to inspect the raw result.
    let tracked_raw = git_tracked_wasm_names(root);
    assert!(
        !tracked_raw.iter().any(|p| p.ends_with("config.yaml")),
        "T-036 LOW-2: git_tracked_wasm_names must NOT include 'config.yaml' — the \
         .ends_with(\".wasm\") filter must exclude non-WASM files; \
         got tracked_raw: {:?}",
        tracked_raw
    );
}

// ---------------------------------------------------------------------------
// T-037 — HIGH-2 pass-8 M18+M16 killer: staged but not committed
//
// Phase C of the T-030 wiring control series.
//
// `check_declared_subset_tracked()` distinguishes:
//   - `tracked` (git index, `git ls-files`) — includes staged files
//   - `committed` (HEAD tree, `git ls-tree -r HEAD`) — excludes staged-only files
// Step 4: `tracked − committed ≠ ∅` → STAGED-NOT-COMMITTED fires.
//
// Captured M18 mutation proof (pass-8):
//   Mutant: git_committed_wasm_names() uses `git ls-files` instead of `git ls-tree -r HEAD`.
//   $ cargo test --package factory-dispatcher --test bundle_orphan_check \
//         -- test_S_21_09_ac006_T037
//   FAILED: T-037 M18+M16 killer: run_t012_gate must return Err with
//   STAGED-NOT-COMMITTED: staged-probe.wasm; …
//   → M18 makes committed==tracked (ls-files includes staged file); tracked−committed={}
//     → run_t012_gate returns Ok; result.is_err() assertion FAILS.
//
// Captured M16 mutation proof (pass-8):
//   Mutant: check_declared_subset_tracked args swapped: (&committed, &tracked).
//   $ cargo test --package factory-dispatcher --test bundle_orphan_check \
//         -- test_S_21_09_ac006_T037
//   FAILED: T-037 M18+M16 killer: error must contain 'STAGED-NOT-COMMITTED:
//   staged-probe.wasm'; M16 mutant would produce 'MISSING: staged-probe.wasm' instead;
//   got: "...MISSING: staged-probe.wasm"
//   → M16 passes committed (HEAD — no staged-probe) as "tracked"; declared−committed fires
//     MISSING: staged-probe.wasm; second assertion fails with wrong outcome identifier.
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
fn test_S_21_09_ac006_T037_staged_not_committed_fires_staged_not_committed() {
    let tmp = tempdir().expect("tempdir must create successfully");
    let root = tmp.path();

    let init_out = Command::new("git")
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
        .output()
        .expect("git init must execute for T-037 fixture");
    assert!(
        init_out.status.success(),
        "T-037: git init failed: {}",
        String::from_utf8_lossy(&init_out.stderr)
    );

    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(root)
        .output()
        .expect("git config user.email must execute");
    Command::new("git")
        .args(["config", "user.name", "Test"])
        .current_dir(root)
        .output()
        .expect("git config user.name must execute");

    let plugins_dir = root.join("plugins/vsdd-factory");
    let hook_plugins_dir = plugins_dir.join("hook-plugins");
    fs::create_dir_all(&hook_plugins_dir).expect("hook-plugins dir must be created");

    // hooks-registry: 30 hooks h00..h29 + staged-probe.wasm (all declared).
    let mut hooks_content = String::from("schema_version = 2\n");
    for i in 0..30_u32 {
        hooks_content.push_str(&format!(
            "[[hooks]]\nname = \"h{i:02}\"\nplugin = \"hook-plugins/h{i:02}.wasm\"\n\
             event = \"PreToolUse\"\ntool = \"^Bash$\"\ntimeout_ms = 5000\n\
             on_error = \"continue\"\n",
        ));
    }
    hooks_content.push_str(
        "[[hooks]]\nname = \"staged-probe\"\nplugin = \"hook-plugins/staged-probe.wasm\"\n\
         event = \"PreToolUse\"\ntool = \"^Bash$\"\ntimeout_ms = 5000\n\
         on_error = \"continue\"\n",
    );
    fs::write(plugins_dir.join("hooks-registry.toml"), &hooks_content)
        .expect("hooks-registry.toml must be written");

    // resolvers-registry: 1 entry (ctx.wasm — committed).
    fs::write(
        plugins_dir.join("resolvers-registry.toml"),
        "schema_version = 1\n[[resolvers]]\nname = \"ctx\"\n\
         plugin = \"hook-plugins/ctx.wasm\"\n",
    )
    .expect("resolvers-registry.toml must be written");

    // Write all 30 hooks + ctx.wasm + staged-probe.wasm on disk.
    for i in 0..30_u32 {
        fs::write(hook_plugins_dir.join(format!("h{i:02}.wasm")), b"wasm")
            .expect("hook wasm must be written");
    }
    fs::write(hook_plugins_dir.join("ctx.wasm"), b"wasm").expect("ctx.wasm must be written");
    fs::write(hook_plugins_dir.join("staged-probe.wasm"), b"wasm")
        .expect("staged-probe.wasm must be written");

    // Add and commit h00..h29 + ctx.wasm (everything EXCEPT staged-probe.wasm).
    let add_out = Command::new("git")
        .args(["-c", "core.excludesFile=/dev/null", "add", "."])
        .current_dir(root)
        .output()
        .expect("git add must execute for T-037 phase-1 fixture");
    assert!(
        add_out.status.success(),
        "T-037: git add failed: {}",
        String::from_utf8_lossy(&add_out.stderr)
    );

    // Remove staged-probe.wasm from the index before committing (unstage it).
    let rm_out = Command::new("git")
        .args([
            "rm",
            "--cached",
            "plugins/vsdd-factory/hook-plugins/staged-probe.wasm",
        ])
        .current_dir(root)
        .output()
        .expect("git rm --cached must execute for T-037 fixture");
    assert!(
        rm_out.status.success(),
        "T-037: git rm --cached failed: {}",
        String::from_utf8_lossy(&rm_out.stderr)
    );

    let commit_out = Command::new("git")
        .args([
            "-c",
            "core.excludesFile=/dev/null",
            "-c",
            "commit.gpgsign=false",
            "-c",
            "core.hooksPath=/dev/null",
            "commit",
            "-m",
            "T-037 fixture: 30 hooks + ctx committed; staged-probe excluded",
        ])
        .current_dir(root)
        .output()
        .expect("git commit must execute for T-037 fixture");
    assert!(
        commit_out.status.success(),
        "T-037: git commit failed: {}",
        String::from_utf8_lossy(&commit_out.stderr)
    );

    // Now stage staged-probe.wasm (force-add) WITHOUT committing.
    // After this: git ls-files sees staged-probe.wasm; git ls-tree HEAD does NOT.
    let stage_out = Command::new("git")
        .args([
            "-c",
            "core.excludesFile=/dev/null",
            "add",
            "-f",
            "plugins/vsdd-factory/hook-plugins/staged-probe.wasm",
        ])
        .current_dir(root)
        .output()
        .expect("git add -f staged-probe must execute for T-037 fixture");
    assert!(
        stage_out.status.success(),
        "T-037: git add -f staged-probe failed: {}",
        String::from_utf8_lossy(&stage_out.stderr)
    );

    // git ls-files includes staged-probe (in index); git ls-tree HEAD does NOT (not committed).
    // Step 4: tracked − committed = {staged-probe.wasm} → STAGED-NOT-COMMITTED.
    // M18 (ls-files instead of ls-tree in committed): committed == tracked → no outcome → Ok.
    // M16 (swap args): step 3 uses committed (HEAD — no staged-probe) → MISSING fires instead.
    let result = run_t012_gate(root);

    assert!(
        result.is_err(),
        "T-037 M18+M16 killer: run_t012_gate must return Err with STAGED-NOT-COMMITTED: \
         staged-probe.wasm; staged file is in git index but not in HEAD tree; \
         M18 (ls-files in committed) would suppress this, returning Ok; \
         M16 (swapped args) would produce MISSING instead of STAGED-NOT-COMMITTED"
    );

    let err = result.unwrap_err();
    assert!(
        err.contains("STAGED-NOT-COMMITTED: hook-plugins/staged-probe.wasm"),
        "T-037 M18+M16 killer: error must contain \
         'STAGED-NOT-COMMITTED: hook-plugins/staged-probe.wasm'; \
         M16 mutant would produce 'MISSING: hook-plugins/staged-probe.wasm' instead; \
         got: {:?}",
        err
    );
}

// ---------------------------------------------------------------------------
// T-038 — HIGH-3 pass-8 UNGATED-DECLARATION control
//
// A declaration whose resolved path passes gates 1+2 (repo-internal, depth+prefix)
// but fails gate 3 (next component != "hook-plugins") is now detected by
// `detect_ungated_declarations()` and surfaces as `UNGATED-DECLARATION: <path>`
// in `run_t012_gate()` BEFORE git commands are invoked.
//
// This test does NOT require a git fixture: the ungated check fires in `run_t012_gate()`
// before `git_tracked_wasm_names()` / `git_committed_wasm_names()` are called.
// Only registry files in `plugins/vsdd-factory/` are needed.
//
// Fixture: hooks-registry declares 30 valid h00..h29 hooks + 1 ungated entry
// (`other-dir/evil-probe.wasm`). resolvers-registry declares 1 valid ctx.wasm.
// Inventory passes (exactly the expected pair); ungated check fires before git.
//
// Gate-by-gate for `other-dir/evil-probe.wasm` (same analysis as T-035):
//   Gate 1: joined_parts.len() == expected_depth + 2  → PASSES
//   Gate 2: joined_parts[0..expected_depth] == parent_parts  → PASSES
//   Gate 3: hook_comp == "other-dir" ≠ "hook-plugins"  → UNGATED (not EXCLUDED here)
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
fn test_S_21_09_ac006_T038_ungated_declaration_fires_before_git() {
    let tmp = tempdir().expect("tempdir must create successfully");
    let root = tmp.path();

    let plugins_dir = root.join("plugins/vsdd-factory");
    fs::create_dir_all(&plugins_dir).expect("plugins/vsdd-factory dir must be created");

    // hooks-registry: 30 valid hooks + 1 ungated declaration (other-dir/evil-probe.wasm).
    let mut hooks_content = String::from("schema_version = 2\n");
    for i in 0..30_u32 {
        hooks_content.push_str(&format!(
            "[[hooks]]\nname = \"h{i:02}\"\nplugin = \"hook-plugins/h{i:02}.wasm\"\n\
             event = \"PreToolUse\"\ntool = \"^Bash$\"\ntimeout_ms = 5000\n\
             on_error = \"continue\"\n",
        ));
    }
    hooks_content.push_str(
        "[[hooks]]\nname = \"evil-probe\"\nplugin = \"other-dir/evil-probe.wasm\"\n\
         event = \"PreToolUse\"\ntool = \"^Bash$\"\ntimeout_ms = 5000\n\
         on_error = \"continue\"\n",
    );
    fs::write(plugins_dir.join("hooks-registry.toml"), &hooks_content)
        .expect("hooks-registry.toml must be written");

    // resolvers-registry: 1 valid entry (ctx.wasm).
    fs::write(
        plugins_dir.join("resolvers-registry.toml"),
        "schema_version = 1\n[[resolvers]]\nname = \"ctx\"\n\
         plugin = \"hook-plugins/ctx.wasm\"\n",
    )
    .expect("resolvers-registry.toml must be written");

    // No git init required: detect_ungated_declarations fires before git calls.
    let result = run_t012_gate(root);

    assert!(
        result.is_err(),
        "T-038 UNGATED-DECLARATION: run_t012_gate must return Err when a declaration passes \
         gates 1+2 but fails gate 3; detect_ungated_declarations fires before git calls; \
         removing the ungated check from run_t012_gate would cause run_t012_gate to panic \
         on git commands (no git repo) or return Ok — this assertion would FAIL"
    );

    let err = result.unwrap_err();
    assert!(
        err.contains("UNGATED-DECLARATION: other-dir/evil-probe.wasm"),
        "T-038 UNGATED-DECLARATION: error must contain \
         'UNGATED-DECLARATION: other-dir/evil-probe.wasm'; got: {:?}",
        err
    );
}

// ---------------------------------------------------------------------------
// T-039 — HIGH-1 pass-9 probe control: subdirectory-declared vs flat-committed false negative
//
// This is the probe that HIGH-1 was designed to close.  Before pass-9, both
// `extract_hook_plugin_name` and the git functions reduced paths to basenames, so:
//
//   Declared: `hook-plugins/sub/hNN.wasm`  → basename `hNN.wasm`
//   Committed flat: `hook-plugins/hNN.wasm` → basename `hNN.wasm`
//   declared − tracked = ∅  →  run_t012_gate returned Ok (false negative)
//
// After pass-9, `extract_hook_plugin_name` returns the full registry-parent-relative
// path (`hook-plugins/sub/hNN.wasm`) and the git functions return full paths stripped
// of the `plugins/vsdd-factory/` prefix (`hook-plugins/hNN.wasm`).  The declared set
// and tracked set are now distinct → MISSING fires on every `hook-plugins/sub/hNN.wasm`.
//
// Fixture: 30 hooks declared at `hook-plugins/sub/hNN.wasm` (nested) but committed
// flat at `hook-plugins/hNN.wasm`.  The registry also declares ctx.wasm (resolvers)
// committed flat at `hook-plugins/ctx.wasm`.  All ctx + flat hNN are committed; the
// declared sub-paths are never committed.
//
// Expected before fix: Ok (false negative — basename collision).
// Expected after fix:  Err with "MISSING: hook-plugins/sub/h00.wasm".
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
fn test_S_21_09_ac006_T039_subdir_declared_vs_flat_committed_fires_missing() {
    let tmp = tempdir().expect("tempdir must create successfully");
    let root = tmp.path();

    // Initialise a git repo with -c overrides (same discipline as T-030 Phase B).
    let init_out = Command::new("git")
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
        .output()
        .expect("git init must execute for T-039 fixture");
    assert!(
        init_out.status.success(),
        "T-039: git init failed: {}",
        String::from_utf8_lossy(&init_out.stderr)
    );

    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(root)
        .output()
        .expect("git config user.email must execute");
    Command::new("git")
        .args(["config", "user.name", "Test"])
        .current_dir(root)
        .output()
        .expect("git config user.name must execute");

    let plugins_dir = root.join("plugins/vsdd-factory");
    let hook_plugins_dir = plugins_dir.join("hook-plugins");
    fs::create_dir_all(&hook_plugins_dir).expect("hook-plugins dir must be created");

    // hooks-registry: 30 hooks declared at hook-plugins/sub/hNN.wasm (nested sub-path).
    let mut hooks_content = String::from("schema_version = 2\n");
    for i in 0..30_u32 {
        hooks_content.push_str(&format!(
            "[[hooks]]\nname = \"h{i:02}\"\nplugin = \"hook-plugins/sub/h{i:02}.wasm\"\n\
             event = \"PreToolUse\"\ntool = \"^Bash$\"\ntimeout_ms = 5000\n\
             on_error = \"continue\"\n",
        ));
    }
    fs::write(plugins_dir.join("hooks-registry.toml"), &hooks_content)
        .expect("hooks-registry.toml must be written");

    // resolvers-registry: 1 entry (ctx.wasm — committed flat).
    fs::write(
        plugins_dir.join("resolvers-registry.toml"),
        "schema_version = 1\n[[resolvers]]\nname = \"ctx\"\n\
         plugin = \"hook-plugins/ctx.wasm\"\n",
    )
    .expect("resolvers-registry.toml must be written");

    // Commit WASMs FLAT (not in sub/): hook-plugins/hNN.wasm + hook-plugins/ctx.wasm.
    // These do NOT match the declared sub-paths hook-plugins/sub/hNN.wasm.
    for i in 0..30_u32 {
        fs::write(hook_plugins_dir.join(format!("h{i:02}.wasm")), b"wasm")
            .expect("flat wasm must be written");
    }
    fs::write(hook_plugins_dir.join("ctx.wasm"), b"wasm").expect("ctx.wasm must be written");

    let add_out = Command::new("git")
        .args(["-c", "core.excludesFile=/dev/null", "add", "."])
        .current_dir(root)
        .output()
        .expect("git add must execute for T-039 fixture");
    assert!(
        add_out.status.success(),
        "T-039: git add failed: {}",
        String::from_utf8_lossy(&add_out.stderr)
    );

    let commit_out = Command::new("git")
        .args([
            "-c",
            "core.excludesFile=/dev/null",
            "-c",
            "commit.gpgsign=false",
            "-c",
            "core.hooksPath=/dev/null",
            "commit",
            "-m",
            "T-039 fixture: 30 hooks committed flat; declared at sub/",
        ])
        .current_dir(root)
        .output()
        .expect("git commit must execute for T-039 fixture");
    assert!(
        commit_out.status.success(),
        "T-039: git commit failed: {}",
        String::from_utf8_lossy(&commit_out.stderr)
    );

    // Declared: hook-plugins/sub/hNN.wasm.  Tracked (after prefix strip): hook-plugins/hNN.wasm.
    // With basename reduction (pre-pass-9): both collapse to hNN.wasm → Ok (false negative).
    // With full-path tracking (pass-9): hook-plugins/sub/h00.wasm ∉ tracked → MISSING.
    let result = run_t012_gate(root);

    assert!(
        result.is_err(),
        "T-039 HIGH-1 probe: run_t012_gate must return Err when hooks are declared at \
         hook-plugins/sub/hNN.wasm but committed flat at hook-plugins/hNN.wasm; \
         before pass-9 (basename reduction) this returned Ok — a false negative; \
         after pass-9 (full-path tracking) the declared and tracked paths are distinct"
    );

    let err = result.unwrap_err();
    assert!(
        err.contains("MISSING: hook-plugins/sub/h00.wasm"),
        "T-039 HIGH-1 probe: error must contain 'MISSING: hook-plugins/sub/h00.wasm'; \
         the full registry-parent-relative path must appear in the MISSING identifier; \
         got: {:?}",
        err
    );
}

// ---------------------------------------------------------------------------
// T-040 — MEDIUM-1 resolvers arm: ungated declaration in resolvers-registry fires
//
// T-038 proves `detect_ungated_declarations()` fires on an ungated entry in
// hooks-registry.toml.  This test proves the RESOLVERS arm: `run_t012_gate()` calls
// `detect_ungated_declarations()` on BOTH registries; an ungated entry in
// resolvers-registry.toml must also fire `UNGATED-DECLARATION`.
//
// Fixture: hooks-registry declares 30 valid h00..h29 hooks.
//          resolvers-registry declares 1 valid ctx.wasm + 1 ungated entry
//          (`other-dir/evil-resolver.wasm`).
// Inventory passes (exactly the expected pair); ungated check fires before git.
//
// Mutation-proof: removing the `detect_ungated_declarations(&resolvers_registry)` call
// in run_t012_gate() means the resolvers arm is unchecked; run_t012_gate panics on
// git (no repo) or skips the ungated error → result.is_err() FAILS.
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
fn test_S_21_09_ac006_T040_ungated_declaration_in_resolvers_fires() {
    let tmp = tempdir().expect("tempdir must create successfully");
    let root = tmp.path();

    let plugins_dir = root.join("plugins/vsdd-factory");
    fs::create_dir_all(&plugins_dir).expect("plugins/vsdd-factory dir must be created");

    // hooks-registry: 30 valid hooks (no ungated entries).
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

    // resolvers-registry: 1 valid ctx.wasm + 1 ungated declaration
    // (other-dir/evil-resolver.wasm passes gates 1+2 but fails gate 3).
    fs::write(
        plugins_dir.join("resolvers-registry.toml"),
        "schema_version = 1\n\
         [[resolvers]]\nname = \"ctx\"\nplugin = \"hook-plugins/ctx.wasm\"\n\
         [[resolvers]]\nname = \"evil-resolver\"\nplugin = \"other-dir/evil-resolver.wasm\"\n",
    )
    .expect("resolvers-registry.toml must be written");

    // No git init required: detect_ungated_declarations fires before git calls.
    let result = run_t012_gate(root);

    assert!(
        result.is_err(),
        "T-040 MEDIUM-1 resolvers arm: run_t012_gate must return Err when resolvers-registry \
         contains an ungated declaration; detect_ungated_declarations is called on BOTH \
         registries; removing the resolvers call would miss this — result.is_err() FAILS"
    );

    let err = result.unwrap_err();
    assert!(
        err.contains("UNGATED-DECLARATION: other-dir/evil-resolver.wasm"),
        "T-040 MEDIUM-1 resolvers arm: error must contain \
         'UNGATED-DECLARATION: other-dir/evil-resolver.wasm'; got: {:?}",
        err
    );
}

// ---------------------------------------------------------------------------
// T-041 — MEDIUM-3 git failure path: run_t012_gate panics on non-git dir
//
// `git_tracked_wasm_names()` and `git_committed_wasm_names()` assert success on their
// respective git commands.  When called from a directory that is NOT a git repository,
// these assertions fail with the git error message.  This test proves the panic path
// is reachable and the assertion message contains the expected prefix.
//
// Fixture: a tmpdir with valid inventory + 30 valid hooks + 1 resolver, but NO git
// init.  The ungated check passes (all declarations are in hook-plugins/).  The git
// calls fire and panic.
//
// Mutation-proof: removing the `assert!(output.status.success(), ...)` in
// git_tracked_wasm_names() causes the function to silently return an empty vec
// (or garbage); EC-005a fires instead of the git-error panic.  The
// `#[should_panic(expected = "git ls-files exited with status")]` FAILS because
// EC-005a produces "T-012 EC-005a", not the git error prefix.
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
#[should_panic(expected = "git ls-files exited with status")]
fn test_S_21_09_ac006_T041_run_t012_gate_panics_on_non_git_dir() {
    let tmp = tempdir().expect("tempdir must create successfully");
    let root = tmp.path();

    let plugins_dir = root.join("plugins/vsdd-factory");
    fs::create_dir_all(&plugins_dir).expect("plugins/vsdd-factory dir must be created");

    // Valid inventory: exactly the expected pair.
    // hooks-registry: 30 valid hooks.
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

    // resolvers-registry: 1 valid entry.
    fs::write(
        plugins_dir.join("resolvers-registry.toml"),
        "schema_version = 1\n[[resolvers]]\nname = \"ctx\"\n\
         plugin = \"hook-plugins/ctx.wasm\"\n",
    )
    .expect("resolvers-registry.toml must be written");

    // No git init: inventory passes, ungated check passes, then git_tracked_wasm_names()
    // panics on "git ls-files exited with status".
    let _ = run_t012_gate(root);
}

// ---------------------------------------------------------------------------
// T-042 — pass-9.1 case-variant MISSING outcome: end-to-end pin
//
// T-031 proves that `Hook-Plugins/ghost-case.wasm` ENTERS `declared` verbatim.
// T-042 proves what happens downstream: when `check_declared_subset_tracked` compares
// the verbatim declared identifier `"Hook-Plugins/ghost-missing.wasm"` against a
// lowercase git-tracked set containing `"hook-plugins/ghost-missing.wasm"`, the
// case-sensitive comparison fails and `MISSING: Hook-Plugins/ghost-missing.wasm` fires.
//
// This pins the full chain: gate-3 eq_ignore_ascii_case admittance (T-031) → verbatim
// identifier in declared → case-sensitive mismatch against tracked → MISSING output.
// The outcome is correct on both platforms:
//   - Linux CI (case-sensitive FS): production cannot load `Hook-Plugins/…`;
//     MISSING is accurate.
//   - macOS (case-insensitive FS): production could load it, but the gate conservatively
//     rejects the case mismatch, requiring lowercase `hook-plugins/` in declarations.
//
// Mutation-proof: if extract_hook_plugin_name() re-introduces `to_ascii_lowercase()`
// on result_parts[0], the declared identifier becomes `"hook-plugins/ghost-missing.wasm"`,
// which IS in the tracked set → check_declared_subset_tracked returns Ok → this test
// fails because result.is_err() is false.
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
fn test_S_21_09_ac006_T042_case_variant_declared_fires_missing_against_lowercase_tracked() {
    // Declared: 29 fillers (lowercase) + 1 case-variant hook + 1 resolver (30 hooks total).
    // Values use registry-parent-relative path format to match parse_plugin_refs() output.
    let mut hooks_declared: HashSet<String> = (0..29)
        .map(|i| format!("hook-plugins/filler-{:02}.wasm", i))
        .collect();
    hooks_declared.insert("Hook-Plugins/ghost-missing.wasm".to_string()); // 30 total; case variant

    let resolvers_declared: HashSet<String> = ["hook-plugins/resolver.wasm".to_string()]
        .into_iter()
        .collect();

    // Tracked: all 29 fillers + resolver + LOWERCASE ghost-missing (not the variant).
    // Simulates a git tree where the artifact is committed as "hook-plugins/ghost-missing.wasm"
    // while the registry declares "Hook-Plugins/ghost-missing.wasm" — case mismatch.
    let tracked: HashSet<String> = (0..29)
        .map(|i| format!("hook-plugins/filler-{:02}.wasm", i))
        .chain([
            "hook-plugins/resolver.wasm".to_string(),
            "hook-plugins/ghost-missing.wasm".to_string(), // lowercase — does NOT match variant
        ])
        .collect();
    let committed = tracked.clone();

    let result =
        check_declared_subset_tracked(&hooks_declared, &resolvers_declared, &tracked, &committed);

    // (a) Must return Err: the verbatim declared identifier is not in the tracked set.
    assert!(
        result.is_err(),
        "T-042 pass-9.1: check_declared_subset_tracked must return Err when a case-variant \
         declared path ('Hook-Plugins/ghost-missing.wasm') is compared case-sensitively \
         against a lowercase tracked set; got Ok"
    );

    let msg = result.unwrap_err();

    // (b) MISSING must cite the verbatim declared path, not the lowercase form.
    assert!(
        msg.contains("MISSING: Hook-Plugins/ghost-missing.wasm"),
        "T-042 pass-9.1: error message must contain \
         'MISSING: Hook-Plugins/ghost-missing.wasm' (verbatim declared form); \
         mutation-proof: if to_ascii_lowercase() is re-introduced, declared becomes \
         'hook-plugins/ghost-missing.wasm' which IS in tracked → Ok → this assertion \
         is never reached; got: {}",
        msg
    );
}

// ---------------------------------------------------------------------------
// T-043 — pass-10 UNGATED-DECLARATION: bare-name form (containment-based catch)
//
// A bare plugin path (`ghost-bare.wasm`) resolves to `registry_parent/ghost-bare.wasm`.
// With the pass-10.1 containment-based algorithm:
//   joined = lex_norm(registry_parent.join("ghost-bare.wasm"))
//          = [...root_components..., "plugins", "vsdd-factory", "ghost-bare.wasm"]
//   Containment: len = root_parts.len()+3 > root_parts.len() ✓
//                joined[0..root_parts.len()] == root_parts ✓  → in-repo
//   is_hook_plugins: joined[expected_depth] = "ghost-bare.wasm" ≠ "hook-plugins" → UNGATED
//
// Fixture: 30 valid hooks + bare-name declaration. No git init needed.
//
// Mutation-proof: if containment check is removed (or the is_hook_plugins check is
// removed), the ungated Vec is empty; run_t012_gate proceeds to git calls and panics
// on the non-git tmpdir — `err.contains("UNGATED-DECLARATION")` FAILS.
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
fn test_S_21_09_ac006_T043_bare_name_ungated_declaration_fires() {
    let tmp = tempdir().expect("tempdir must create successfully");
    let root = tmp.path();

    let plugins_dir = root.join("plugins/vsdd-factory");
    fs::create_dir_all(&plugins_dir).expect("plugins/vsdd-factory dir must be created");

    // hooks-registry: 30 valid hooks + 1 bare-name declaration (ghost-bare.wasm).
    let mut hooks_content = String::from("schema_version = 2\n");
    for i in 0..30_u32 {
        hooks_content.push_str(&format!(
            "[[hooks]]\nname = \"h{i:02}\"\nplugin = \"hook-plugins/h{i:02}.wasm\"\n\
             event = \"PreToolUse\"\ntool = \"^Bash$\"\ntimeout_ms = 5000\n\
             on_error = \"continue\"\n",
        ));
    }
    hooks_content.push_str(
        "[[hooks]]\nname = \"ghost-bare\"\nplugin = \"ghost-bare.wasm\"\n\
         event = \"PreToolUse\"\ntool = \"^Bash$\"\ntimeout_ms = 5000\n\
         on_error = \"continue\"\n",
    );
    fs::write(plugins_dir.join("hooks-registry.toml"), &hooks_content)
        .expect("hooks-registry.toml must be written");

    // resolvers-registry: 1 valid entry.
    fs::write(
        plugins_dir.join("resolvers-registry.toml"),
        "schema_version = 1\n[[resolvers]]\nname = \"ctx\"\n\
         plugin = \"hook-plugins/ctx.wasm\"\n",
    )
    .expect("resolvers-registry.toml must be written");

    // No git init required: detect_ungated_declarations fires before git calls.
    let result = run_t012_gate(root);

    assert!(
        result.is_err(),
        "T-043 UNGATED-DECLARATION: run_t012_gate must return Err when a bare-name \
         declaration ('ghost-bare.wasm') passes containment check (inside worktree root) \
         but is NOT correctly under hook-plugins/; detect_ungated_declarations fires \
         before git calls; removing the containment check would cause git panic on \
         non-git tmpdir — UNGATED-DECLARATION would NOT appear in error; got Ok"
    );

    let err = result.unwrap_err();
    assert!(
        err.contains("UNGATED-DECLARATION: ghost-bare.wasm"),
        "T-043 UNGATED-DECLARATION: error must contain \
         'UNGATED-DECLARATION: ghost-bare.wasm'; \
         mutation-proof: reverting gate-1 to expected_depth+2 causes git panic — \
         error contains git message not UNGATED-DECLARATION; got: {:?}",
        err
    );
}

// ---------------------------------------------------------------------------
// T-044 — pass-10 UNGATED-DECLARATION: `../registry-parent/` prefix form (containment)
//
// `../vsdd-factory/ghost-updir.wasm` joined to registry_parent resolves back to
// `registry_parent/ghost-updir.wasm` (the ParentDir pops vsdd-factory, then it is
// pushed again).  Containment check passes (still inside root); is_hook_plugins fails
// (no hook-plugins component) → UNGATED.
//
// Gate-by-gate (pass-10.1 containment algorithm) for `../vsdd-factory/ghost-updir.wasm`:
//   joined = [...root_components..., "plugins", "vsdd-factory", "ghost-updir.wasm"]
//   Containment: len = root_parts.len()+3 > root_parts.len() ✓
//                joined[0..root_parts.len()] == root_parts ✓  → in-repo
//   is_hook_plugins: joined[expected_depth] = "ghost-updir.wasm" ≠ "hook-plugins" → UNGATED
//
// Fixture: 30 valid hooks + `../vsdd-factory/` prefix declaration. No git init needed.
//
// Mutation-proof: removing the containment check or is_hook_plugins check causes
// git panic instead → UNGATED-DECLARATION not in error → assertion FAILS.
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
fn test_S_21_09_ac006_T044_traversal_cancel_registry_parent_prefix_fires_ungated() {
    let tmp = tempdir().expect("tempdir must create successfully");
    let root = tmp.path();

    let plugins_dir = root.join("plugins/vsdd-factory");
    fs::create_dir_all(&plugins_dir).expect("plugins/vsdd-factory dir must be created");

    // hooks-registry: 30 valid hooks + 1 `../vsdd-factory/` prefix declaration.
    let mut hooks_content = String::from("schema_version = 2\n");
    for i in 0..30_u32 {
        hooks_content.push_str(&format!(
            "[[hooks]]\nname = \"h{i:02}\"\nplugin = \"hook-plugins/h{i:02}.wasm\"\n\
             event = \"PreToolUse\"\ntool = \"^Bash$\"\ntimeout_ms = 5000\n\
             on_error = \"continue\"\n",
        ));
    }
    hooks_content.push_str(
        "[[hooks]]\nname = \"ghost-updir\"\nplugin = \"../vsdd-factory/ghost-updir.wasm\"\n\
         event = \"PreToolUse\"\ntool = \"^Bash$\"\ntimeout_ms = 5000\n\
         on_error = \"continue\"\n",
    );
    fs::write(plugins_dir.join("hooks-registry.toml"), &hooks_content)
        .expect("hooks-registry.toml must be written");

    // resolvers-registry: 1 valid entry.
    fs::write(
        plugins_dir.join("resolvers-registry.toml"),
        "schema_version = 1\n[[resolvers]]\nname = \"ctx\"\n\
         plugin = \"hook-plugins/ctx.wasm\"\n",
    )
    .expect("resolvers-registry.toml must be written");

    // No git init required: detect_ungated_declarations fires before git calls.
    let result = run_t012_gate(root);

    assert!(
        result.is_err(),
        "T-044 UNGATED-DECLARATION: run_t012_gate must return Err when \
         '../vsdd-factory/ghost-updir.wasm' resolves inside worktree root but \
         NOT under hook-plugins/; detect_ungated_declarations fires before git calls; \
         removing the containment check would cause git panic instead — got Ok"
    );

    let err = result.unwrap_err();
    assert!(
        err.contains("UNGATED-DECLARATION: ../vsdd-factory/ghost-updir.wasm"),
        "T-044 UNGATED-DECLARATION: error must contain \
         'UNGATED-DECLARATION: ../vsdd-factory/ghost-updir.wasm'; \
         mutation-proof: removing containment check causes git panic — \
         error contains git message not UNGATED-DECLARATION; got: {:?}",
        err
    );
}

// ---------------------------------------------------------------------------
// T-045 — pass-10.1 UNGATED-DECLARATION: one-level-up `../ghost.wasm`
//
// `../ghost.wasm` resolves to `plugins/ghost.wasm` — one level above registry_parent
// but still inside the worktree root.  Before pass-10.1, this silently escaped:
// the old pass-10 length-based gate-1 (`< expected_depth+1`) was:
//   joined = [...root, "plugins", "ghost.wasm"], len = root_parts.len()+2
//   expected_depth = root_parts.len()+2 (absolute path components)
//   gate: len < expected_depth+1 = root_parts.len()+3 → root_parts.len()+2 < root_parts.len()+3
//         → TRUE → silently skipped
// Probe confirmed (pass-10.1 run): run_t012_gate panicked at git step, not UNGATED step.
//
// Pass-10.1 containment-based algorithm:
//   joined = [...root_components..., "plugins", "ghost.wasm"]
//   len = root_parts.len()+2 > root_parts.len() ✓
//   joined[0..root_parts.len()] == root_parts ✓  → in-repo
//   is_hook_plugins: joined.len()=root_parts.len()+2 < expected_depth+2=root_parts.len()+4
//                    → FALSE → UNGATED
//
// No git init needed; detect_ungated_declarations fires before git calls.
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
fn test_S_21_09_ac006_T045_one_level_up_ungated_declaration_fires() {
    let tmp = tempdir().expect("tempdir must create successfully");
    let root = tmp.path();

    let plugins_dir = root.join("plugins/vsdd-factory");
    fs::create_dir_all(&plugins_dir).expect("plugins/vsdd-factory dir must be created");

    // hooks-registry: 30 valid hooks + 1 one-level-up declaration (../ghost.wasm).
    let mut hooks_content = String::from("schema_version = 2\n");
    for i in 0..30_u32 {
        hooks_content.push_str(&format!(
            "[[hooks]]\nname = \"h{i:02}\"\nplugin = \"hook-plugins/h{i:02}.wasm\"\n\
             event = \"PreToolUse\"\ntool = \"^Bash$\"\ntimeout_ms = 5000\n\
             on_error = \"continue\"\n",
        ));
    }
    hooks_content.push_str(
        "[[hooks]]\nname = \"ghost-up\"\nplugin = \"../ghost.wasm\"\n\
         event = \"PreToolUse\"\ntool = \"^Bash$\"\ntimeout_ms = 5000\n\
         on_error = \"continue\"\n",
    );
    fs::write(plugins_dir.join("hooks-registry.toml"), &hooks_content)
        .expect("hooks-registry.toml must be written");

    fs::write(
        plugins_dir.join("resolvers-registry.toml"),
        "schema_version = 1\n[[resolvers]]\nname = \"ctx\"\n\
         plugin = \"hook-plugins/ctx.wasm\"\n",
    )
    .expect("resolvers-registry.toml must be written");

    let result = run_t012_gate(root);

    assert!(
        result.is_err(),
        "T-045 UNGATED-DECLARATION: run_t012_gate must return Err when '../ghost.wasm' \
         resolves to plugins/ghost.wasm inside worktree root; containment passes, \
         is_hook_plugins fails; probe confirmed escape before pass-10.1; got Ok"
    );

    let err = result.unwrap_err();
    assert!(
        err.contains("UNGATED-DECLARATION: ../ghost.wasm"),
        "T-045: error must contain 'UNGATED-DECLARATION: ../ghost.wasm'; got: {:?}",
        err
    );
}

// ---------------------------------------------------------------------------
// T-046 — pass-10.1 UNGATED-DECLARATION: two-levels-up `../../ghost.wasm`
//
// `../../ghost.wasm` resolves to `<root>/ghost.wasm` — at the worktree root level,
// still inside the root (by exactly one component).
// Before pass-10.1, the old gate-1:
//   joined = [...root_components..., "ghost.wasm"], len = root_parts.len()+1
//   expected_depth = root_parts.len()+2
//   gate: len < expected_depth+1 = root_parts.len()+3 → root_parts.len()+1 < root_parts.len()+3
//         → TRUE → silently skipped
// Probe confirmed: run_t012_gate panicked at git step.
//
// Pass-10.1 containment-based algorithm:
//   len = root_parts.len()+1 > root_parts.len() ✓ (margin of exactly 1)
//   joined[0..root_parts.len()] == root_parts ✓  → in-repo
//   is_hook_plugins: len = root_parts.len()+1 < expected_depth+2 = root_parts.len()+4 → UNGATED
//
// No git init needed.
//
// Story: S-21.09
// ---------------------------------------------------------------------------
#[test]
fn test_S_21_09_ac006_T046_two_levels_up_ungated_declaration_fires() {
    let tmp = tempdir().expect("tempdir must create successfully");
    let root = tmp.path();

    let plugins_dir = root.join("plugins/vsdd-factory");
    fs::create_dir_all(&plugins_dir).expect("plugins/vsdd-factory dir must be created");

    // hooks-registry: 30 valid hooks + 1 two-levels-up declaration (../../ghost.wasm).
    let mut hooks_content = String::from("schema_version = 2\n");
    for i in 0..30_u32 {
        hooks_content.push_str(&format!(
            "[[hooks]]\nname = \"h{i:02}\"\nplugin = \"hook-plugins/h{i:02}.wasm\"\n\
             event = \"PreToolUse\"\ntool = \"^Bash$\"\ntimeout_ms = 5000\n\
             on_error = \"continue\"\n",
        ));
    }
    hooks_content.push_str(
        "[[hooks]]\nname = \"ghost-root\"\nplugin = \"../../ghost.wasm\"\n\
         event = \"PreToolUse\"\ntool = \"^Bash$\"\ntimeout_ms = 5000\n\
         on_error = \"continue\"\n",
    );
    fs::write(plugins_dir.join("hooks-registry.toml"), &hooks_content)
        .expect("hooks-registry.toml must be written");

    fs::write(
        plugins_dir.join("resolvers-registry.toml"),
        "schema_version = 1\n[[resolvers]]\nname = \"ctx\"\n\
         plugin = \"hook-plugins/ctx.wasm\"\n",
    )
    .expect("resolvers-registry.toml must be written");

    let result = run_t012_gate(root);

    assert!(
        result.is_err(),
        "T-046 UNGATED-DECLARATION: run_t012_gate must return Err when '../../ghost.wasm' \
         resolves to <root>/ghost.wasm (margin of 1 inside root); containment: \
         len=root_parts.len()+1 > root_parts.len() ✓; probe confirmed escape before \
         pass-10.1; got Ok"
    );

    let err = result.unwrap_err();
    assert!(
        err.contains("UNGATED-DECLARATION: ../../ghost.wasm"),
        "T-046: error must contain 'UNGATED-DECLARATION: ../../ghost.wasm'; got: {:?}",
        err
    );
}
