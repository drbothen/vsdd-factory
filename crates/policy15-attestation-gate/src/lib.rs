//! POLICY 15 ATTESTATION-LOCATION GATE — ADR-040 §Decisions 7-10 (v1.17)
//!
//! # Why this exists
//!
//! D-912 added a gate to prevent fix waves from pushing assertion-site changes
//! without co-updating `red-gate-log.md`. Across six bash-in-markdown iterations
//! every defect was found only by manual extraction-and-execution — none by code
//! review. POLICY 21 prohibits new `.sh` files; this crate is the POLICY 21-compliant
//! replacement. Controls are `#[test]` functions, making defect class 5
//! (four outcomes sharing two exit codes → control matches wrong exit code) impossible.
//!
//! # Four outcomes
//!
//! | Outcome | Exit code |
//! |---------|-----------|
//! | `Fail` | 2 |
//! | `PassWithActivations(N ≥ 1)` | 0 |
//! | `PassZeroActivations` | 0 |
//! | `EmptyOrUnreachable(_)` | 2 |
//!
//! # Usage
//!
//! ```no_run
//! use policy15_attestation_gate::{run_gate, GateOutcome};
//! use std::path::Path;
//!
//! let outcome = run_gate(Path::new("."), "develop").expect("gate error");
//! println!("{}", outcome.identifier());
//! std::process::exit(outcome.exit_code());
//! ```

use std::path::Path;
use std::process::Command;

// ── Pinned constants (ADR-040 §Decision 8 Ruling 8(a)) ───────────────────────

/// Pinned crate path for the S-21.07 story.
pub const PLUGIN_CRATE: &str = "crates/hook-plugins/validate-cross-site-correspondence";

/// Pinned evidence log path for the S-21.07 story.
pub const RED_GATE_LOG: &str =
    "crates/hook-plugins/validate-cross-site-correspondence/docs/red-gate-log.md";

// ── Outcome types ─────────────────────────────────────────────────────────────

/// Why a commit failed the obligation.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FailReason {
    /// The pinned log (`red-gate-log.md`) was absent at the activating commit.
    LogAbsent,
    /// The log was present but contained no `^### .*assertion-site attestation (<parent>)` heading.
    AttestationMissing,
}

/// A commit that violated the POLICY 15 obligation.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FailedCommit {
    /// Full 40-character commit SHA.
    pub commit: String,
    /// Why this commit failed.
    pub reason: FailReason,
}

/// Why the gate could not reach a measurable result.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UnreachableCause {
    /// The pinned crate path is absent from the HEAD git tree (not the filesystem).
    ///
    /// Possible cause: crate renamed or restructured without updating the pin constant.
    StalePin,
    /// `merge_base(HEAD, base_branch)..HEAD` returned zero commits.
    ///
    /// Possible cause: `fetch-depth: 0` not set, wrong MERGE_BASE, or
    /// the base branch ref could not be resolved.
    EmptyRange,
    /// A commit in the range produced an empty changed-file set.
    ///
    /// Possible cause: `git commit --allow-empty`, shallow-clone boundary, or
    /// measurement tool failure.
    UnmeasurableDiff {
        /// The commit that produced the empty diff.
        commit: String,
    },
}

/// The four gate outcomes (ADR-040 §Decision 8 Ruling 8(c)).
///
/// Mapping to process exit codes happens only in the binary (`src/main.rs`).
/// Tests assert on the variant, which makes defect class 5
/// (four outcomes / two exit codes) structurally impossible.
// NOTE: #[non_exhaustive] is deliberately absent so that exhaustive match at
// every call site is a compile error when a new outcome is added — the same
// property that guards PluginResult in the dispatcher crate.
#[derive(Debug, PartialEq, Eq)]
pub enum GateOutcome {
    /// One or more commits violated the POLICY 15 obligation.
    Fail(Vec<FailedCommit>),
    /// Gate activated for N ≥ 1 commits; all were compliant with the obligation.
    PassWithActivations(usize),
    /// Scope target exists, commit range non-empty, all diffs measurable, zero activations.
    ///
    /// Legitimate for docs-only PRs. Named (not anonymous exit-0) so it is observable
    /// across CI runs — analogous to SystemVerilog `cover property (A)` pairing.
    PassZeroActivations,
    /// Structural prerequisite failed — CI setup defect or stale pin.
    EmptyOrUnreachable(UnreachableCause),
}

impl GateOutcome {
    /// Returns `true` for outcomes that map to exit code 0.
    pub fn is_pass(&self) -> bool {
        matches!(
            self,
            GateOutcome::PassWithActivations(_) | GateOutcome::PassZeroActivations
        )
    }

    /// Process exit code for this outcome.
    ///
    /// | Variant | Exit code |
    /// |---------|-----------|
    /// | `Fail` | 2 |
    /// | `PassWithActivations` | 0 |
    /// | `PassZeroActivations` | 0 |
    /// | `EmptyOrUnreachable` | 2 |
    pub fn exit_code(&self) -> i32 {
        if self.is_pass() { 0 } else { 2 }
    }

    /// Greppable outcome identifier string emitted to stdout in CI logs.
    ///
    /// The identifier uniquely identifies the outcome within a shared exit code —
    /// controls assert on this string, not the exit code alone (§Decision 10 Req 4).
    pub fn identifier(&self) -> String {
        match self {
            GateOutcome::Fail(_) => "FAIL: obligation violated".to_string(),
            GateOutcome::PassWithActivations(n) => format!("PASS-{n}-activations"),
            GateOutcome::PassZeroActivations => "PASS-zero-activations".to_string(),
            GateOutcome::EmptyOrUnreachable(UnreachableCause::StalePin) => {
                "EMPTY-or-UNREACHABLE: stale pin".to_string()
            }
            GateOutcome::EmptyOrUnreachable(UnreachableCause::EmptyRange) => {
                "EMPTY-or-UNREACHABLE: git range returned no commits".to_string()
            }
            GateOutcome::EmptyOrUnreachable(UnreachableCause::UnmeasurableDiff { .. }) => {
                "EMPTY-or-UNREACHABLE: unmeasurable diff".to_string()
            }
        }
    }
}

// ── Error type ────────────────────────────────────────────────────────────────

/// Hard failure: git not installed, repository inaccessible, or unexpected I/O error.
///
/// All semantic gate outcomes (including "stale pin" and "unresolvable base")
/// are returned as `Ok(GateOutcome)`, not `Err(GateError)`.
#[derive(Debug, thiserror::Error)]
pub enum GateError {
    /// A git command exited non-zero in a context where success was required.
    #[error("git command failed: {0}")]
    GitCommand(String),
    /// An I/O error occurred when spawning git.
    #[error("I/O error spawning git: {0}")]
    Io(#[from] std::io::Error),
    /// The file did not exist at the given commit.
    ///
    /// Used internally to distinguish "file absent at commit" from hard errors.
    #[error("file absent at {commit}:{path}")]
    FileAbsent { commit: String, path: String },
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Run the POLICY 15 ATTESTATION-LOCATION GATE.
///
/// Computes `git merge-base HEAD origin/<base_branch>` as the range start.
/// If the merge base cannot be resolved (no `origin` remote, unknown branch),
/// returns `Ok(EmptyOrUnreachable(EmptyRange))` — fail closed.
///
/// The stale-pin guard runs **before** the merge-base computation, so when both
/// the crate is absent *and* the base is unresolvable, `StalePin` is returned.
pub fn run_gate(repo: &Path, base_branch: &str) -> Result<GateOutcome, GateError> {
    // Guard 1 — stale-pin check BEFORE the origin/<branch> merge-base lookup.
    //
    // This guard is intentionally duplicated from the one in `run_gate_from_merge_base`
    // (guard 2). The duplication establishes a documented ordering invariant: when the
    // pinned crate is absent *and* the base branch is unresolvable, the gate reports
    // `StalePin` (not `EmptyRange`). Without guard 1, a repo where the crate was
    // renamed/deleted *and* the remote was removed would silently report `EmptyRange`,
    // masking the more actionable stale-pin diagnosis.
    //
    // Pinned by: `test_run_gate_guard1_stale_pin_beats_unresolvable_base`.
    if !tree_path_exists(repo, "HEAD", PLUGIN_CRATE)? {
        return Ok(GateOutcome::EmptyOrUnreachable(UnreachableCause::StalePin));
    }

    let remote_ref = format!("origin/{base_branch}");
    let merge_base = match git_merge_base(repo, "HEAD", &remote_ref) {
        Ok(sha) => sha,
        Err(_) => {
            // Unresolvable base → fail closed (not a silent empty range).
            return Ok(GateOutcome::EmptyOrUnreachable(
                UnreachableCause::EmptyRange,
            ));
        }
    };

    run_gate_inner(repo, &merge_base)
}

/// Run the gate with a pre-computed merge-base SHA.
///
/// This is the testable core: test fixtures construct scratch repos and pass the
/// initial commit SHA as `merge_base` (bypassing the `origin/` remote lookup).
///
/// The stale-pin guard is still applied — the `merge_base` parameter only
/// replaces the `git merge-base HEAD origin/<branch>` computation.
pub fn run_gate_from_merge_base(repo: &Path, merge_base: &str) -> Result<GateOutcome, GateError> {
    if !tree_path_exists(repo, "HEAD", PLUGIN_CRATE)? {
        return Ok(GateOutcome::EmptyOrUnreachable(UnreachableCause::StalePin));
    }
    run_gate_inner(repo, merge_base)
}

// ── Gate core ─────────────────────────────────────────────────────────────────

/// Core gate logic (stale-pin guard applied by callers).
fn run_gate_inner(repo: &Path, merge_base: &str) -> Result<GateOutcome, GateError> {
    let commits = git_log_range(repo, merge_base, "HEAD")?;
    if commits.is_empty() {
        return Ok(GateOutcome::EmptyOrUnreachable(
            UnreachableCause::EmptyRange,
        ));
    }

    let mut fail_list: Vec<FailedCommit> = Vec::new();
    let mut empty_diff_commit: Option<String> = None;
    let mut activations: usize = 0;

    for commit_sha in &commits {
        // Skip parentless commits (root commit or shallow-clone boundary).
        if !commit_has_parent(repo, commit_sha)? {
            eprintln!(
                "WARNING: commit {commit_sha} has no first parent (root commit or shallow-clone boundary) — skipping POLICY 15 evaluation for this commit"
            );
            continue;
        }

        // All files changed in this commit vs its first parent.
        let all_changed = git_diff_name_only(repo, &format!("{commit_sha}^1"), commit_sha)?;

        // Empty diff → unmeasurable scope (§Decision 8 trigger 3).
        if all_changed.is_empty() {
            empty_diff_commit = Some(commit_sha.clone());
            continue;
        }

        // Does this commit touch the pinned crate's *.rs or *.bats?
        let crate_prefix = format!("{PLUGIN_CRATE}/");
        let activating = all_changed
            .iter()
            .any(|f| f.starts_with(&crate_prefix) && (f.ends_with(".rs") || f.ends_with(".bats")));

        if !activating {
            continue;
        }

        // Unconditional obligation: gate activates for this commit.
        activations += 1;
        let parent_sha = git_rev_parse(repo, &format!("{commit_sha}^1"))?;

        // FAIL-when-absent: pinned log must exist at this commit (§Decision 8 Ruling 8(b)).
        let log_content = match git_show_file(repo, commit_sha, RED_GATE_LOG) {
            Ok(content) => content,
            Err(GateError::FileAbsent { .. }) => {
                fail_list.push(FailedCommit {
                    commit: commit_sha.clone(),
                    reason: FailReason::LogAbsent,
                });
                continue;
            }
            Err(e) => return Err(e),
        };

        // Count `^### .*assertion-site attestation (<parent_sha>)` headings.
        let count = count_attestation_headings(&log_content, &parent_sha);
        if count != 1 {
            fail_list.push(FailedCommit {
                commit: commit_sha.clone(),
                reason: FailReason::AttestationMissing,
            });
        }
    }

    // Priority order: FAIL > EMPTY-or-UNREACHABLE > PASS.
    if !fail_list.is_empty() {
        return Ok(GateOutcome::Fail(fail_list));
    }
    if let Some(commit) = empty_diff_commit {
        return Ok(GateOutcome::EmptyOrUnreachable(
            UnreachableCause::UnmeasurableDiff { commit },
        ));
    }
    if activations == 0 {
        return Ok(GateOutcome::PassZeroActivations);
    }
    Ok(GateOutcome::PassWithActivations(activations))
}

// ── Attestation-heading matcher ───────────────────────────────────────────────

/// Count lines in `log_content` matching `^### .*assertion-site attestation (<parent_sha>)`.
///
/// The `^### ` anchor (via `starts_with`) prevents prose lines from contributing
/// a false count (ADR-040 §Decision 6 Ruling 6(b)).
fn count_attestation_headings(log_content: &str, parent_sha: &str) -> usize {
    let needle = format!("assertion-site attestation ({parent_sha})");
    log_content
        .lines()
        .filter(|line| line.starts_with("### ") && line.contains(&needle))
        .count()
}

// ── Git helpers ───────────────────────────────────────────────────────────────

fn run_git(repo: &Path, args: &[&str]) -> Result<std::process::Output, GateError> {
    Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .output()
        .map_err(GateError::Io)
}

/// Returns `true` if `<treeish>:<path>` exists in the git object tree.
///
/// Uses `git cat-file -e` — a tree query, NOT a filesystem `is_dir()` check.
/// Prevents defect-3: a directory can exist on disk but be absent from the tree
/// if it was never committed (ADR-040 §Decision 8 stale-pin discussion).
fn tree_path_exists(repo: &Path, treeish: &str, path: &str) -> Result<bool, GateError> {
    let spec = format!("{treeish}:{path}");
    let output = run_git(repo, &["cat-file", "-e", &spec])?;
    Ok(output.status.success())
}

/// Returns the full 40-character SHA of `git merge-base a b`.
fn git_merge_base(repo: &Path, a: &str, b: &str) -> Result<String, GateError> {
    let output = run_git(repo, &["merge-base", a, b])?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(GateError::GitCommand(format!(
            "git merge-base {a} {b} exited {}",
            output.status
        )))
    }
}

/// Returns full SHAs of commits in `base..head`, newest-first.
fn git_log_range(repo: &Path, base: &str, head: &str) -> Result<Vec<String>, GateError> {
    let range = format!("{base}..{head}");
    let output = run_git(repo, &["log", &range, "--format=%H"])?;
    if !output.status.success() {
        return Err(GateError::GitCommand(format!(
            "git log {range} exited {}",
            output.status
        )));
    }
    let shas = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string())
        .collect();
    Ok(shas)
}

/// Returns `true` if `commit` has a first parent (is not a root or parentless commit).
fn commit_has_parent(repo: &Path, commit: &str) -> Result<bool, GateError> {
    let parent_rev = format!("{commit}^1");
    let output = run_git(repo, &["rev-parse", "--verify", &parent_rev])?;
    Ok(output.status.success())
}

/// Returns the list of files changed between `from` and `to` (one per line, no blanks).
fn git_diff_name_only(repo: &Path, from: &str, to: &str) -> Result<Vec<String>, GateError> {
    let output = run_git(repo, &["diff", "--name-only", from, to])?;
    if !output.status.success() {
        return Err(GateError::GitCommand(format!(
            "git diff --name-only {from} {to} exited {}",
            output.status
        )));
    }
    let files = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string())
        .collect();
    Ok(files)
}

/// Returns the full SHA for a git revision.
fn git_rev_parse(repo: &Path, rev: &str) -> Result<String, GateError> {
    let output = run_git(repo, &["rev-parse", rev])?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(GateError::GitCommand(format!(
            "git rev-parse {rev} exited {}",
            output.status
        )))
    }
}

/// Returns the content of `path` at `commit`.
///
/// Returns `GateError::FileAbsent` if the path does not exist at that commit.
fn git_show_file(repo: &Path, commit: &str, path: &str) -> Result<String, GateError> {
    let spec = format!("{commit}:{path}");
    let output = run_git(repo, &["show", &spec])?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(GateError::FileAbsent {
            commit: commit.to_string(),
            path: path.to_string(),
        })
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use std::fs;
    use std::process::Command;
    use tempfile::TempDir;

    // ── Fixture helpers ───────────────────────────────────────────────────────

    struct Repo {
        dir: TempDir,
    }

    impl Repo {
        /// Create a scratch git repository with one initial empty commit.
        fn new() -> Self {
            let dir = TempDir::new().expect("TempDir::new");
            let r = Repo { dir };
            r.git(&["init"]);
            r.git(&["config", "user.email", "ci@vsdd-test"]);
            r.git(&["config", "user.name", "CI Fixture"]);
            r.git(&["commit", "--allow-empty", "-m", "initial"]);
            r
        }

        fn path(&self) -> &Path {
            self.dir.path()
        }

        fn git(&self, args: &[&str]) -> std::process::Output {
            Command::new("git")
                .current_dir(self.dir.path())
                .args(args)
                .output()
                .expect("git")
        }

        fn head_sha(&self) -> String {
            let out = self.git(&["rev-parse", "HEAD"]);
            String::from_utf8(out.stdout).unwrap().trim().to_string()
        }

        /// Write `content` to `rel_path` (relative to repo root), creating parent dirs.
        fn write(&self, rel_path: &str, content: &str) {
            let full = self.dir.path().join(rel_path);
            if let Some(parent) = full.parent() {
                fs::create_dir_all(parent).expect("create_dir_all");
            }
            fs::write(&full, content).expect("fs::write");
        }

        /// Stage all changes and commit.
        fn commit(&self, message: &str) {
            self.git(&["add", "."]);
            self.git(&["commit", "-m", message]);
        }

        /// Write a file and commit it in one step.
        fn write_and_commit(&self, rel_path: &str, content: &str, message: &str) {
            self.write(rel_path, content);
            self.commit(message);
        }
    }

    // Shared path constants used across all fixtures.
    const PC: &str = "crates/hook-plugins/validate-cross-site-correspondence";
    const LOG: &str = "crates/hook-plugins/validate-cross-site-correspondence/docs/red-gate-log.md";

    // ── Tests ─────────────────────────────────────────────────────────────────

    /// Positive control 1 — crate `.rs` change; evidence file absent → `Fail(LogAbsent)`.
    #[test]
    fn test_positive_1_absent_log() {
        let repo = Repo::new();
        let base = repo.head_sha();

        repo.write_and_commit(
            &format!("{PC}/src/lib.rs"),
            "// assertion site\n",
            "add assertion site, no log",
        );

        let outcome = run_gate_from_merge_base(repo.path(), &base).expect("no gate error");
        assert!(
            matches!(
                &outcome,
                GateOutcome::Fail(v) if v.len() == 1 && v[0].reason == FailReason::LogAbsent
            ),
            "expected Fail(LogAbsent), got: {:?}",
            outcome
        );
        assert_eq!(outcome.identifier(), "FAIL: obligation violated");
        assert_eq!(outcome.exit_code(), 2);
    }

    /// Positive control 2 — crate `.rs` change; log present, no attestation heading
    /// → `Fail(AttestationMissing)`.
    #[test]
    fn test_positive_2_no_attestation_heading() {
        let repo = Repo::new();
        let base = repo.head_sha();

        // Single commit: .rs file + log present but no matching heading.
        repo.write(&format!("{PC}/src/lib.rs"), "// assertion site\n");
        repo.write(
            LOG,
            "# Red Gate Log\n\n## Observations\n\n(no attestation section)\n",
        );
        repo.commit("assertion site + log, no attestation heading");

        let outcome = run_gate_from_merge_base(repo.path(), &base).expect("no gate error");
        assert!(
            matches!(
                &outcome,
                GateOutcome::Fail(v) if !v.is_empty() && v[0].reason == FailReason::AttestationMissing
            ),
            "expected Fail(AttestationMissing), got: {:?}",
            outcome
        );
        assert_eq!(outcome.identifier(), "FAIL: obligation violated");
        assert_eq!(outcome.exit_code(), 2);
    }

    /// Negative control — crate `.rs` change; correct attestation heading for the parent SHA
    /// → `PassWithActivations(1)`.
    #[test]
    fn test_negative_compliant_attestation() {
        let repo = Repo::new();
        let base = repo.head_sha();
        // The commit we are about to make has HEAD (= base) as its parent.
        let parent_sha = base.clone();

        repo.write(&format!("{PC}/src/lib.rs"), "// assertion site\n");
        repo.write(
            LOG,
            &format!(
                "# Red Gate Log\n\n### Pass-1 assertion-site attestation ({parent_sha})\n\nAttestation body.\n"
            ),
        );
        repo.commit("assertion site + correct attestation heading");

        let outcome = run_gate_from_merge_base(repo.path(), &base).expect("no gate error");
        assert!(
            matches!(outcome, GateOutcome::PassWithActivations(1)),
            "expected PassWithActivations(1), got: {:?}",
            outcome
        );
        assert_eq!(outcome.identifier(), "PASS-1-activations");
        assert_eq!(outcome.exit_code(), 0);
        assert!(outcome.is_pass());
    }

    /// Pass-zero control — crate path seeded into tree; docs-only commit outside
    /// the crate → `PassZeroActivations`.
    ///
    /// Note: `mkdir -p` alone is insufficient — git never tracks empty directories.
    /// A non-`*.rs`/`*.bats` file must be committed inside the crate path for
    /// `git cat-file -e HEAD:<CRATE>` to succeed (ADR-040 §Decision 10 note on
    /// fixture construction).
    #[test]
    fn test_pass_zero_activations() {
        let repo = Repo::new();
        let base = repo.head_sha();

        // Seed the crate into the HEAD git tree via a placeholder file.
        repo.write_and_commit(
            &format!("{PC}/docs/placeholder.md"),
            "placeholder\n",
            "seed: crate placeholder in tree",
        );
        // A docs-only commit outside the crate (no *.rs/*.bats changes).
        repo.write_and_commit("docs/README.md", "# README\n", "docs update outside crate");

        let outcome = run_gate_from_merge_base(repo.path(), &base).expect("no gate error");
        assert!(
            matches!(outcome, GateOutcome::PassZeroActivations),
            "expected PassZeroActivations, got: {:?}",
            outcome
        );
        assert_eq!(outcome.identifier(), "PASS-zero-activations");
        assert_eq!(outcome.exit_code(), 0);
        assert!(outcome.is_pass());
    }

    /// Empty-range control — crate in HEAD tree; `MERGE_BASE == HEAD` (no commits in range)
    /// → `EmptyOrUnreachable(EmptyRange)`.
    ///
    /// The crate placeholder must be committed FIRST so the stale-pin guard passes;
    /// HEAD is then captured AFTER the placeholder commit so `merge_base == HEAD`
    /// produces a genuinely empty range.
    #[test]
    fn test_empty_range() {
        let repo = Repo::new();

        // Seed the crate into the tree before capturing HEAD.
        repo.write_and_commit(
            &format!("{PC}/docs/placeholder.md"),
            "placeholder\n",
            "seed: crate in HEAD tree",
        );
        let head = repo.head_sha(); // captured AFTER seeding

        // MERGE_BASE == HEAD → git log head..HEAD returns zero commits.
        let outcome = run_gate_from_merge_base(repo.path(), &head).expect("no gate error");
        assert!(
            matches!(
                outcome,
                GateOutcome::EmptyOrUnreachable(UnreachableCause::EmptyRange)
            ),
            "expected EmptyOrUnreachable(EmptyRange), got: {:?}",
            outcome
        );
        assert_eq!(
            outcome.identifier(),
            "EMPTY-or-UNREACHABLE: git range returned no commits"
        );
        assert_eq!(outcome.exit_code(), 2);
    }

    /// Stale-pin control — crate path absent from HEAD tree → `EmptyOrUnreachable(StalePin)`.
    #[test]
    fn test_stale_pin() {
        let repo = Repo::new();
        let base = repo.head_sha();

        // Non-empty commit range, but nothing inside the pinned crate.
        repo.write_and_commit("docs/CHANGELOG.md", "# Changelog\n", "changelog only");

        let outcome = run_gate_from_merge_base(repo.path(), &base).expect("no gate error");
        assert!(
            matches!(
                outcome,
                GateOutcome::EmptyOrUnreachable(UnreachableCause::StalePin)
            ),
            "expected EmptyOrUnreachable(StalePin), got: {:?}",
            outcome
        );
        assert_eq!(outcome.identifier(), "EMPTY-or-UNREACHABLE: stale pin");
        assert_eq!(outcome.exit_code(), 2);
    }

    /// Unmeasurable-diff control — `--allow-empty` commit in range →
    /// `EmptyOrUnreachable(UnmeasurableDiff)`.
    #[test]
    fn test_unmeasurable_diff() {
        let repo = Repo::new();

        // Seed the crate so stale-pin guard passes.
        repo.write_and_commit(
            &format!("{PC}/docs/placeholder.md"),
            "placeholder\n",
            "seed: crate in HEAD tree",
        );
        let base = repo.head_sha();

        // An --allow-empty commit has a parent but an empty changed-file set.
        repo.git(&[
            "commit",
            "--allow-empty",
            "-m",
            "empty commit (allow-empty)",
        ]);

        let outcome = run_gate_from_merge_base(repo.path(), &base).expect("no gate error");
        assert!(
            matches!(
                outcome,
                GateOutcome::EmptyOrUnreachable(UnreachableCause::UnmeasurableDiff { .. })
            ),
            "expected EmptyOrUnreachable(UnmeasurableDiff), got: {:?}",
            outcome
        );
        assert_eq!(
            outcome.identifier(),
            "EMPTY-or-UNREACHABLE: unmeasurable diff"
        );
        assert_eq!(outcome.exit_code(), 2);
    }

    /// Defect-3 regression test — crate directory created on disk but never committed
    /// → `EmptyOrUnreachable(StalePin)`, NOT `PassZeroActivations`.
    ///
    /// This is the key regression for ADR-040 §Decision 8: `is_dir()` on the filesystem
    /// returns `true`, but `git cat-file -e HEAD:<path>` returns NOT FOUND because the
    /// directory was never committed.  The gate must use the git-tree check.
    #[test]
    fn test_disk_present_tree_absent_is_stale_pin() {
        let repo = Repo::new();
        let base = repo.head_sha();

        // Create the crate directory on disk — but do NOT commit it.
        let crate_dir = repo.path().join(PC);
        fs::create_dir_all(&crate_dir).expect("create_dir_all");

        // A non-crate commit to give a non-empty range.
        repo.write_and_commit("docs/README.md", "# README\n", "docs only, no crate commit");

        let outcome = run_gate_from_merge_base(repo.path(), &base).expect("no gate error");
        assert!(
            matches!(
                outcome,
                GateOutcome::EmptyOrUnreachable(UnreachableCause::StalePin)
            ),
            "expected StalePin (crate on disk but not in git tree), got: {:?}",
            outcome
        );
    }

    /// Unresolvable-base test — bogus base branch ref → non-`Pass` outcome (fail closed).
    ///
    /// When `git merge-base HEAD origin/<branch>` fails (e.g., no `origin` remote,
    /// or branch not found), the gate must not silently yield an empty-range pass.
    #[test]
    fn test_unresolvable_base_fails_closed() {
        let repo = Repo::new();

        // Seed the crate so guard 1 passes (we reach the merge-base step).
        repo.write_and_commit(
            &format!("{PC}/docs/placeholder.md"),
            "placeholder\n",
            "seed: crate in HEAD tree",
        );

        // No `origin` remote — merge-base fails → EmptyOrUnreachable(EmptyRange).
        // Assert the specific variant, not merely !is_pass(): a coarse boolean check
        // would survive a mutation that changed the outcome identifier (level-5 defect).
        let outcome = run_gate(repo.path(), "nonexistent-branch-xyz").expect("no hard I/O error");
        assert!(
            matches!(
                outcome,
                GateOutcome::EmptyOrUnreachable(UnreachableCause::EmptyRange)
            ),
            "expected EmptyOrUnreachable(EmptyRange) for unresolvable base, got: {:?}",
            outcome
        );
    }

    /// Guard-ordering test — crate absent AND range empty → `StalePin` wins over `EmptyRange`.
    ///
    /// ADR-040 §Decision 8/9: the stale-pin guard runs BEFORE the commit-range check.
    /// When both preconditions fail, `StalePin` must be the reported cause.
    #[test]
    fn test_guard_ordering_stale_pin_beats_empty_range() {
        let repo = Repo::new();
        // HEAD is the initial empty commit; crate was never committed.
        let head = repo.head_sha(); // MERGE_BASE == HEAD → would be empty range

        // Neither condition holds: crate absent (→ StalePin) AND range empty (→ EmptyRange).
        let outcome = run_gate_from_merge_base(repo.path(), &head).expect("no gate error");
        assert!(
            matches!(
                outcome,
                GateOutcome::EmptyOrUnreachable(UnreachableCause::StalePin)
            ),
            "expected StalePin to win over EmptyRange, got: {:?}",
            outcome
        );
    }

    /// Guard-1 ordering test (through `run_gate`) — crate absent AND base unresolvable
    /// → `StalePin`, not `EmptyRange`.
    ///
    /// This is the mutation-killing companion to `test_guard_ordering_stale_pin_beats_empty_range`.
    /// That test exercises guard ordering through `run_gate_from_merge_base` (guard 2).
    /// This test exercises it through `run_gate` (guard 1), which fires before the
    /// `git merge-base HEAD origin/<branch>` call. Neutralising guard 1 would cause
    /// the merge-base failure to be reached first, returning `EmptyOrUnreachable(EmptyRange)`
    /// rather than `StalePin` — failing this assertion.
    #[test]
    fn test_run_gate_guard1_stale_pin_beats_unresolvable_base() {
        let repo = Repo::new();
        // Crate is NOT seeded — absent from HEAD tree (StalePin trigger).
        // No origin remote — merge-base would fail (EmptyRange trigger).
        // Guard 1 must fire first → outcome must be StalePin, not EmptyRange.
        let outcome = run_gate(repo.path(), "nonexistent-branch-xyz").expect("no hard I/O error");
        assert!(
            matches!(
                outcome,
                GateOutcome::EmptyOrUnreachable(UnreachableCause::StalePin)
            ),
            "expected StalePin (guard 1 fires before merge-base), got: {:?}",
            outcome
        );
    }

    // ── Additional edge-case tests ────────────────────────────────────────────

    /// A `.bats` file in the crate also activates the gate.
    #[test]
    fn test_bats_file_activates_gate() {
        let repo = Repo::new();
        let base = repo.head_sha();
        let parent_sha = base.clone();

        repo.write(
            &format!("{PC}/tests/it.bats"),
            "#!/usr/bin/env bats\n@test 'x' { true; }\n",
        );
        repo.write(
            LOG,
            &format!(
                "# Red Gate Log\n\n### Pass-1 assertion-site attestation ({parent_sha})\n\nAttestation.\n"
            ),
        );
        repo.commit("add .bats assertion + attestation");

        let outcome = run_gate_from_merge_base(repo.path(), &base).expect("no gate error");
        assert!(
            matches!(outcome, GateOutcome::PassWithActivations(1)),
            "expected PassWithActivations(1) for .bats activation, got: {:?}",
            outcome
        );
    }

    /// A `.rs` file *outside* the pinned crate does not activate the gate.
    #[test]
    fn test_rs_outside_crate_does_not_activate() {
        let repo = Repo::new();
        // Seed the crate so stale-pin passes.
        repo.write_and_commit(
            &format!("{PC}/docs/placeholder.md"),
            "placeholder\n",
            "seed: crate placeholder",
        );
        let base = repo.head_sha();

        // A .rs file in a completely different crate.
        repo.write_and_commit(
            "crates/some-other-crate/src/lib.rs",
            "// other crate\n",
            "add other crate .rs file",
        );

        let outcome = run_gate_from_merge_base(repo.path(), &base).expect("no gate error");
        assert!(
            matches!(outcome, GateOutcome::PassZeroActivations),
            "expected PassZeroActivations (.rs outside crate), got: {:?}",
            outcome
        );
    }

    /// Prose quoting a prior attestation SHA does not cause count > 1 (line-anchor test).
    ///
    /// This verifies the fix for ADR-040 §Decision 6 Ruling 6(b): the grep predicate
    /// is anchored to `^### ` so prose lines that merely contain the SHA do not increment
    /// the count.
    #[test]
    fn test_prose_sha_does_not_cause_false_count() {
        let repo = Repo::new();
        let base = repo.head_sha();
        let parent_sha = base.clone();

        repo.write(&format!("{PC}/src/lib.rs"), "// assertion site\n");
        // Log contains the correct heading PLUS a prose paragraph quoting the same SHA.
        repo.write(
            LOG,
            &format!(
                "# Red Gate Log\n\n### Pass-1 assertion-site attestation ({parent_sha})\n\nSee also `assertion-site attestation ({parent_sha})` in a prior pass.\n"
            ),
        );
        repo.commit("assertion site + attestation + prose quote");

        let outcome = run_gate_from_merge_base(repo.path(), &base).expect("no gate error");
        // Should be PASS-1-activations, not FAIL due to count > 1.
        assert!(
            matches!(outcome, GateOutcome::PassWithActivations(1)),
            "expected PassWithActivations(1) (prose quote must not add to count), got: {:?}",
            outcome
        );
    }

    /// Multiple activating commits, all compliant → `PassWithActivations(2)`.
    #[test]
    fn test_multiple_activations_all_compliant() {
        let repo = Repo::new();
        let base = repo.head_sha();

        // First activating commit.
        let parent1 = base.clone();
        repo.write(&format!("{PC}/src/a.rs"), "// a\n");
        repo.write(
            LOG,
            &format!("# Red Gate Log\n\n### Pass-1 assertion-site attestation ({parent1})\n"),
        );
        repo.commit("commit 1: a.rs + attestation 1");

        // Second activating commit.
        let parent2 = repo.head_sha();
        repo.write(&format!("{PC}/src/b.rs"), "// b\n");
        // Overwrite log with both attestation headings.
        repo.write(
            LOG,
            &format!(
                "# Red Gate Log\n\n### Pass-1 assertion-site attestation ({parent1})\n\n### Pass-2 assertion-site attestation ({parent2})\n"
            ),
        );
        repo.commit("commit 2: b.rs + attestation 2");

        let outcome = run_gate_from_merge_base(repo.path(), &base).expect("no gate error");
        assert!(
            matches!(outcome, GateOutcome::PassWithActivations(2)),
            "expected PassWithActivations(2), got: {:?}",
            outcome
        );
    }

    // ── Coverage-gap fix-pass tests (pre-merge mutant-survival closure) ────────
    //
    // The four groups below (F-1, F-2, F-5, F-6) close per-guard mutant-survival
    // gaps identified in pre-merge validation. F-1's WARNING assertion is
    // EXPECTED RED pending implementer per ADR-040 §Decision 9 Ruling 9(c) — see
    // `tests/binary_integration_test.rs`. F-2 and F-5 below are expected GREEN.

    /// F-2 — exactly-once upper bound (`count != 1` upper arm, lib.rs `run_gate_inner`).
    ///
    /// Existing controls (`test_positive_2_no_attestation_heading`,
    /// `test_negative_compliant_attestation`) only exercise `count == 0` and
    /// `count == 1`. This test drives `count == 2`: a single activating commit
    /// whose pinned `red-gate-log.md` contains TWO
    /// `### <Pass-N> assertion-site attestation (<PARENT-SHA>)` headings for the
    /// SAME parent SHA. The obligation is "exactly one", not "at least one", so
    /// this must still fail with `AttestationMissing`. A plain mutation run on
    /// `count != 1` (e.g. a mutant that flips the comparison to `count == 0`)
    /// cannot be generated from existing fixtures alone — this hand-authored
    /// control is required to kill it.
    ///
    /// Expected: GREEN (existing `count != 1` logic already handles this
    /// correctly; this is coverage, not a behavior change).
    #[test]
    fn test_f2_two_attestation_headings_same_parent_fails() {
        let repo = Repo::new();
        let base = repo.head_sha();
        let parent_sha = base.clone();

        repo.write(&format!("{PC}/src/lib.rs"), "// assertion site\n");
        // TWO headings for the SAME parent SHA within the SAME commit's log.
        repo.write(
            LOG,
            &format!(
                "# Red Gate Log\n\n\
                 ### Pass-1 assertion-site attestation ({parent_sha})\n\n\
                 Attestation body 1.\n\n\
                 ### Pass-2 assertion-site attestation ({parent_sha})\n\n\
                 Attestation body 2 (duplicate heading for the same parent SHA).\n"
            ),
        );
        repo.commit("assertion site + duplicate attestation headings for same parent");

        let outcome = run_gate_from_merge_base(repo.path(), &base).expect("no gate error");
        assert!(
            matches!(
                &outcome,
                GateOutcome::Fail(v) if v.len() == 1 && v[0].reason == FailReason::AttestationMissing
            ),
            "expected Fail(AttestationMissing) for count==2 (exactly-once upper bound), got: {:?}",
            outcome
        );
        assert_eq!(outcome.identifier(), "FAIL: obligation violated");
        assert_eq!(outcome.exit_code(), 2);
    }

    /// F-5 — `git_merge_base` SUCCESS path, driven through `run_gate()`.
    ///
    /// Existing `run_gate()`-level controls (`test_unresolvable_base_fails_closed`,
    /// `test_run_gate_guard1_stale_pin_beats_unresolvable_base`) only exercise the
    /// ERROR arm of `git_merge_base` (no `origin` remote → merge-base command
    /// fails → `EmptyOrUnreachable(EmptyRange)`). Neither test proves the SUCCESS
    /// arm (`Ok(sha)`) is wired correctly.
    ///
    /// This test fakes a `refs/remotes/origin/<branch>` ref via `git update-ref`
    /// — no real `origin` remote or network access required; `git merge-base`
    /// cannot distinguish a synthesized remote-tracking ref from a fetched one —
    /// so `git merge-base HEAD origin/develop` genuinely succeeds. The downstream
    /// outcome (`Fail(LogAbsent)`) is reachable ONLY if the merge-base SHA was
    /// correctly threaded into `run_gate_inner`; the ERROR arm would instead
    /// short-circuit to `EmptyOrUnreachable(EmptyRange)` before any commit is
    /// ever inspected.
    ///
    /// Expected: GREEN.
    #[test]
    fn test_f5_run_gate_resolves_real_merge_base_success_path() {
        let repo = Repo::new();

        // Seed the crate so guard 1 (stale-pin) passes.
        repo.write_and_commit(
            &format!("{PC}/docs/placeholder.md"),
            "placeholder\n",
            "seed: crate in HEAD tree",
        );
        let base = repo.head_sha();

        // Fake the remote-tracking ref WITHOUT a real `origin` remote.
        repo.git(&["update-ref", "refs/remotes/origin/develop", &base]);

        // Activating commit with no red-gate-log.md → Fail(LogAbsent). This
        // outcome is reachable only via the merge-base SUCCESS arm: if
        // `git_merge_base` had errored, the gate would report
        // EmptyOrUnreachable(EmptyRange) instead, before ever inspecting commits.
        repo.write_and_commit(
            &format!("{PC}/src/lib.rs"),
            "// assertion site\n",
            "activate, no log (drives merge-base success path)",
        );

        let outcome = run_gate(repo.path(), "develop").expect("no gate error");
        assert!(
            matches!(
                &outcome,
                GateOutcome::Fail(v) if v.len() == 1 && v[0].reason == FailReason::LogAbsent
            ),
            "expected Fail(LogAbsent) via resolved real merge-base (success path), got: {:?}",
            outcome
        );
        assert_eq!(outcome.identifier(), "FAIL: obligation violated");
        assert_eq!(outcome.exit_code(), 2);
    }

    /// `GateOutcome::identifier` produces strings that are greppable — spot-check.
    #[test]
    fn test_identifier_strings_are_greppable() {
        assert!(
            GateOutcome::PassWithActivations(3)
                .identifier()
                .contains("PASS-3-activations")
        );
        assert!(
            GateOutcome::PassZeroActivations
                .identifier()
                .contains("PASS-zero-activations")
        );
        assert!(GateOutcome::Fail(vec![]).identifier().contains("FAIL"));
        assert!(
            GateOutcome::EmptyOrUnreachable(UnreachableCause::StalePin)
                .identifier()
                .contains("stale pin")
        );
        assert!(
            GateOutcome::EmptyOrUnreachable(UnreachableCause::EmptyRange)
                .identifier()
                .contains("git range returned no commits")
        );
        assert!(
            GateOutcome::EmptyOrUnreachable(UnreachableCause::UnmeasurableDiff {
                commit: "abc".into()
            })
            .identifier()
            .contains("unmeasurable diff")
        );
    }
}
