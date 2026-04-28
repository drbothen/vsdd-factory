//! S-4.08: 1.0.0-rc.1 release gate — RED gate tests.
//!
//! Tests for the testable-now ACs of the rc.1 release gate story.
//!
//! Deferred ACs (require live CI / GH / shakedown completion — cannot be
//! tested in this unit/integration pass):
//!   AC-2  (cargo publish --dry-run): needs network + crate state
//!   AC-3  (bats suite): live run against CI environment
//!   AC-4  (commit.made events under load): production smoke test
//!   AC-6  (Windows smoke test): cross-platform gate
//!   AC-7  (GH pre-release with 5-platform tarballs): requires tag + workflow
//!   AC-8  (E-3+E-4 story closure): GitHub API live state
//!   AC-11 (multi-sink dogfood): environment gate
//!   AC-12 (release-notes artifact): created by release engineer at cut time
//!   AC-15 (GH prerelease: true flag): Release.yml + GH API
//!   AC-Q1 (Semgrep SAST): CI tool gate
//!   AC-Q2 (VP-INDEX snapshot green): VP-INDEX live review
//!   AC-Q3 (latency budget): informational only; no benchmark harness yet
//!
//! Testable-now ACs covered by this file:
//!   AC-1  (BC-9.01.004): ci/platforms.yaml declares exactly 5 canonical platforms
//!   AC-5  (BC-9.01.005): 5 hooks.json.<platform> variants committed; hooks.json gitignored
//!   AC-9  (BC-9.01.006 PC1,PC2): check-shakedown-window.sh exit-code contract
//!   AC-10 (BC-9.01.006 PC4): check-shakedown-window.sh --stories exit-code contract
//!   AC-13 (process-gap): check-changelog-monotonicity.sh exit-code contract
//!   AC-14 (BC-9.01.001): bump-version.sh accepts 1.0.0-rc.N prerelease format
//!   AC-Q4 (process-gap): LICENSE present (no extension); deny.toml present
//!   AC-Q5 (process-gap): README.md, CHANGELOG.md, CONTRIBUTING.md present

#![allow(non_snake_case)]

use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Resolve the repository root from the test binary's location.
///
/// The test binary sits at `<repo>/target/...` so we walk up until we
/// find `Cargo.toml` at the root with `[workspace]` content, or fall
/// back to a compile-time env var injected by the test harness.
fn repo_root() -> PathBuf {
    // Walk from the manifest dir (set at compile time by Cargo).
    let start = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // CARGO_MANIFEST_DIR = <repo>/crates/factory-dispatcher
    // So the repo root is two levels up.
    start
        .parent()
        .expect("crate parent dir")
        .parent()
        .expect("repo root")
        .to_path_buf()
}

// ---------------------------------------------------------------------------
// AC-1 — BC-9.01.004: ci/platforms.yaml declares exactly 5 canonical platforms
// ---------------------------------------------------------------------------

/// BC-9.01.004 PC2: ci/platforms.yaml must exist.
#[test]
fn test_BC_9_01_004_platforms_yaml_exists() {
    let path = repo_root().join("ci/platforms.yaml");
    assert!(
        path.exists(),
        "FAIL AC-1 (BC-9.01.004 PC2): ci/platforms.yaml must exist but was not found at {:?}",
        path
    );
}

/// BC-9.01.004 invariant 1 + PC2: ci/platforms.yaml must declare exactly 5 platforms.
///
/// Counts `platform:` keys — a simple textual parse that is stable against
/// the YAML structure used in this file.
#[test]
fn test_BC_9_01_004_platforms_yaml_declares_exactly_five_platforms() {
    let path = repo_root().join("ci/platforms.yaml");
    let content = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Cannot read ci/platforms.yaml: {e}"));

    // Count `  - platform:` lines (leading spaces + dash).
    let count = content
        .lines()
        .filter(|l| l.trim_start().starts_with("- platform:"))
        .count();

    assert_eq!(
        count,
        5,
        "FAIL AC-1 (BC-9.01.004 invariant 1): ci/platforms.yaml must declare exactly 5 \
         platforms, found {count}. \
         Expected: darwin-arm64, darwin-x64, linux-x64, linux-arm64, windows-x64"
    );
}

/// BC-9.01.004 invariant 1: the 5 platform names must be the canonical set.
#[test]
fn test_BC_9_01_004_platforms_yaml_canonical_platform_names() {
    let path = repo_root().join("ci/platforms.yaml");
    let content = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Cannot read ci/platforms.yaml: {e}"));

    let canonical = [
        "darwin-arm64",
        "darwin-x64",
        "linux-x64",
        "linux-arm64",
        "windows-x64",
    ];

    for name in &canonical {
        assert!(
            content.contains(name),
            "FAIL AC-1 (BC-9.01.004 invariant 1): canonical platform '{}' not found in \
             ci/platforms.yaml",
            name
        );
    }
}

// ---------------------------------------------------------------------------
// AC-5 — BC-9.01.005: 5 hooks.json.<platform> variants committed; hooks.json gitignored
// ---------------------------------------------------------------------------

/// BC-9.01.005 PC3: all 5 per-platform hooks.json variants must exist on disk.
#[test]
fn test_BC_9_01_005_five_hooks_json_platform_variants_committed() {
    let hooks_dir = repo_root().join("plugins/vsdd-factory/hooks");

    let variants = [
        "hooks.json.darwin-arm64",
        "hooks.json.darwin-x64",
        "hooks.json.linux-x64",
        "hooks.json.linux-arm64",
        "hooks.json.windows-x64",
    ];

    for variant in &variants {
        let path = hooks_dir.join(variant);
        assert!(
            path.exists(),
            "FAIL AC-5 (BC-9.01.005 PC3): per-platform variant '{}' must be committed but \
             was not found at {:?}",
            variant,
            path
        );
    }
}

/// BC-9.01.005 PC2: hooks.json.template must exist.
#[test]
fn test_BC_9_01_005_hooks_json_template_committed() {
    let path = repo_root()
        .join("plugins/vsdd-factory/hooks/hooks.json.template");
    assert!(
        path.exists(),
        "FAIL AC-5 (BC-9.01.005 PC2): hooks.json.template must be committed but was not \
         found at {:?}",
        path
    );
}

/// BC-9.01.005 invariant 1: hooks.json must be listed in .gitignore.
///
/// This checks the .gitignore entry that prevents accidental commits of
/// the runtime-written hooks.json.
#[test]
fn test_BC_9_01_005_hooks_json_gitignored() {
    let gitignore_path = repo_root().join(".gitignore");
    let content = fs::read_to_string(&gitignore_path)
        .unwrap_or_else(|e| panic!("Cannot read .gitignore: {e}"));

    // The .gitignore must contain a line that matches hooks.json (without
    // the variant suffix). Accept the full path pattern or bare filename.
    let hooks_json_gitignored = content.lines().any(|line| {
        let stripped = line.trim();
        // Match patterns like: `plugins/vsdd-factory/hooks/hooks.json`
        // or `hooks.json` (bare) — any line that would cause hooks.json
        // (not hooks.json.*) to be ignored.
        (stripped.ends_with("hooks.json") && !stripped.contains('*'))
            || stripped == "hooks.json"
    });

    assert!(
        hooks_json_gitignored,
        "FAIL AC-5 (BC-9.01.005 invariant 1): hooks.json must be listed in .gitignore \
         to prevent accidental commit of the runtime-written file. \
         No matching entry found in .gitignore"
    );
}

// ---------------------------------------------------------------------------
// AC-14 — BC-9.01.001: bump-version.sh accepts 1.0.0-rc.N prerelease format
// ---------------------------------------------------------------------------

/// BC-9.01.001 PC1 (prerelease format): bump-version.sh exits 0 for 1.0.0-rc.1.
///
/// Runs bump-version.sh against a temp directory with a minimal CHANGELOG.md
/// so we exercise the real script without touching the repo's CHANGELOG.
#[test]
fn test_BC_9_01_001_bump_version_accepts_rc_prerelease_format() {
    let script = repo_root().join("scripts/bump-version.sh");
    assert!(
        script.exists(),
        "PRECONDITION: scripts/bump-version.sh must exist"
    );

    // Create a temp directory with a minimal CHANGELOG.md.
    let tmp = tempfile::tempdir()
        .expect("create temp dir for bump-version test");
    let changelog = tmp.path().join("CHANGELOG.md");
    fs::write(
        &changelog,
        "# Changelog\n\n## 1.0.0-beta.4 — previous entry (2026-04-25)\n\nSome content.\n",
    )
    .expect("write temp CHANGELOG.md");

    // bump-version.sh needs REPO_ROOT to locate CHANGELOG.md.
    // Override by symlinking or invoking with a modified repo root.
    // The script uses REPO_ROOT=$(cd "$(dirname "$0")/.." && pwd), so
    // we set up the expected directory structure in tmp.
    let scripts_dir = tmp.path().join("scripts");
    fs::create_dir(&scripts_dir).expect("create scripts dir");

    // Copy the script into the temp scripts dir.
    let tmp_script = scripts_dir.join("bump-version.sh");
    fs::copy(&script, &tmp_script).expect("copy bump-version.sh");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&tmp_script).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&tmp_script, perms).unwrap();
    }

    // Create stub plugin.json + marketplace.json that jq can read.
    let plugin_dir = tmp.path().join("plugins/vsdd-factory/.claude-plugin");
    fs::create_dir_all(&plugin_dir).expect("create plugin dir");
    fs::write(plugin_dir.join("plugin.json"), r#"{"version":"1.0.0-beta.4"}"#)
        .expect("write plugin.json");
    let mkt_dir = tmp.path();
    fs::write(
        mkt_dir.join(".claude-plugin/marketplace.json"),
        r#"{"plugins":[{"version":"1.0.0-beta.4"}]}"#,
    )
    .ok(); // OK if this fails — script reads it as display-only

    // Run the script with 1.0.0-rc.1 "Release Candidate 1"
    let output = Command::new("bash")
        .arg(&tmp_script)
        .arg("1.0.0-rc.1")
        .arg("Release Candidate 1")
        .current_dir(tmp.path())
        .output()
        .expect("execute bump-version.sh");

    // The script must exit 0 for a valid rc.1 prerelease version.
    assert_eq!(
        output.status.code(),
        Some(0),
        "FAIL AC-14 (BC-9.01.001 PC1): bump-version.sh must exit 0 for '1.0.0-rc.1' \
         prerelease format. \
         stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

/// BC-9.01.001 invariant 1 (rejection): bump-version.sh must reject invalid prerelease format.
///
/// Verifies the regex gate rejects build-metadata suffixes (not accepted per script).
#[test]
fn test_BC_9_01_001_bump_version_rejects_invalid_semver() {
    let script = repo_root().join("scripts/bump-version.sh");
    assert!(
        script.exists(),
        "PRECONDITION: scripts/bump-version.sh must exist"
    );

    let output = Command::new("bash")
        .arg(&script)
        .arg("not-a-semver")
        .output()
        .expect("execute bump-version.sh");

    let exit_code = output.status.code();
    assert!(
        exit_code.map(|c| c != 0).unwrap_or(true),
        "FAIL (BC-9.01.001 invariant 1): bump-version.sh must exit non-zero for invalid \
         version 'not-a-semver', but exited {:?}",
        exit_code
    );
}

/// BC-9.01.001 invariant 1: bump-version.sh must reject when no args given.
#[test]
fn test_BC_9_01_001_bump_version_rejects_missing_version_arg() {
    let script = repo_root().join("scripts/bump-version.sh");
    assert!(
        script.exists(),
        "PRECONDITION: scripts/bump-version.sh must exist"
    );

    let output = Command::new("bash")
        .arg(&script)
        .output()
        .expect("execute bump-version.sh");

    assert_ne!(
        output.status.code(),
        Some(0),
        "FAIL (BC-9.01.001 invariant 1): bump-version.sh must exit non-zero when \
         version arg is missing"
    );
}

// ---------------------------------------------------------------------------
// AC-9 — BC-9.01.006 PC1+PC2: check-shakedown-window.sh exit-code contract
// ---------------------------------------------------------------------------

/// BC-9.01.006 stub existence: check-shakedown-window.sh must exist in scripts/.
#[test]
fn test_BC_9_01_006_check_shakedown_window_script_exists() {
    let path = repo_root().join("scripts/check-shakedown-window.sh");
    assert!(
        path.exists(),
        "FAIL AC-9 (BC-9.01.006): scripts/check-shakedown-window.sh must exist. \
         Create it as part of S-4.08 Task 10."
    );
}

/// BC-9.01.006 PC1+PC2: check-shakedown-window.sh must exit 0 when window is satisfied.
///
/// RED GATE: This test will FAIL until the real implementation is provided.
/// The current stub always exits 1. Implementer must satisfy BC-9.01.006 PC1.
///
/// Simulates: >=14 days elapsed, no open P0 issues. For the RED gate we
/// inject a mock that forces the satisfied path — but the stub does not
/// support this, so the test will fail as required.
#[test]
fn test_BC_9_01_006_shakedown_window_exits_0_when_satisfied() {
    let script = repo_root().join("scripts/check-shakedown-window.sh");
    if !script.exists() {
        panic!(
            "PRECONDITION FAIL: scripts/check-shakedown-window.sh does not exist"
        );
    }

    // We cannot run an actual 14-day clock in a unit test, but we can
    // verify the script contract by injecting a MOCK_SATISFIED=1 env var.
    // The real implementation must honour MOCK_SATISFIED=1 (exit 0) in
    // test mode, or the test vector must be satisfied via a future test
    // harness. For RED gate: current stub ignores this and exits 1.
    let output = Command::new("bash")
        .arg(&script)
        .arg("--tag")
        .arg("v1.0.0-beta.4")
        .arg("--days")
        .arg("14")
        .arg("--p0-query")
        .arg("gh issue list --label P0 --state open --search 'label:beta-shakedown'")
        .env("VSDD_SHAKEDOWN_MOCK_SATISFIED", "1")
        .output()
        .expect("execute check-shakedown-window.sh");

    assert_eq!(
        output.status.code(),
        Some(0),
        "FAIL AC-9 (BC-9.01.006 PC1): check-shakedown-window.sh must exit 0 when \
         VSDD_SHAKEDOWN_MOCK_SATISFIED=1 (window satisfied). \
         Implementer: honour this env var in test mode. \
         Current exit code: {:?}\nstderr: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );
}

/// BC-9.01.006 PC2: check-shakedown-window.sh must exit 1 when P0 issue is open.
///
/// This is the P0-open failure mode. The stub already exits 1, so this
/// test will PASS on the stub — but it must also pass on the real implementation.
/// We keep it here to document the PC2 contract.
#[test]
fn test_BC_9_01_006_shakedown_window_exits_1_when_p0_open() {
    let script = repo_root().join("scripts/check-shakedown-window.sh");
    if !script.exists() {
        panic!(
            "PRECONDITION FAIL: scripts/check-shakedown-window.sh does not exist"
        );
    }

    let output = Command::new("bash")
        .arg(&script)
        .arg("--tag")
        .arg("v1.0.0-beta.4")
        .arg("--days")
        .arg("14")
        .arg("--p0-query")
        .arg("gh issue list --label P0 --state open --search 'label:beta-shakedown'")
        .env("VSDD_SHAKEDOWN_MOCK_P0_OPEN", "1")
        .output()
        .expect("execute check-shakedown-window.sh");

    assert_eq!(
        output.status.code(),
        Some(1),
        "FAIL AC-9 (BC-9.01.006 PC2): check-shakedown-window.sh must exit 1 when \
         P0 issue is open (VSDD_SHAKEDOWN_MOCK_P0_OPEN=1). \
         Current exit code: {:?}",
        output.status.code()
    );
}

/// BC-9.01.006 PC3: check-shakedown-window.sh must exit 2 for non-existent tag.
///
/// RED GATE: stub exits 1 (not 2). Will fail until real implementation.
#[test]
fn test_BC_9_01_006_shakedown_window_exits_2_for_missing_tag() {
    let script = repo_root().join("scripts/check-shakedown-window.sh");
    if !script.exists() {
        panic!(
            "PRECONDITION FAIL: scripts/check-shakedown-window.sh does not exist"
        );
    }

    let output = Command::new("bash")
        .arg(&script)
        .arg("--tag")
        .arg("v999.999.999-nonexistent")
        .arg("--days")
        .arg("14")
        .arg("--p0-query")
        .arg("gh issue list --label P0 --state open")
        .output()
        .expect("execute check-shakedown-window.sh");

    assert_eq!(
        output.status.code(),
        Some(2),
        "FAIL AC-9 (BC-9.01.006 PC3): check-shakedown-window.sh must exit 2 when \
         the given tag does not exist in git. \
         Current exit code: {:?}\nstderr: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );
}

// ---------------------------------------------------------------------------
// AC-10 — BC-9.01.006 PC4: check-shakedown-window.sh --stories flag
// ---------------------------------------------------------------------------

/// BC-9.01.006 PC4: --stories flag invocation for WASM port production exposure.
///
/// RED GATE: stub exits 1 regardless. Will fail until real implementation
/// supports the --stories flag with MOCK_SATISFIED semantics.
#[test]
fn test_BC_9_01_006_shakedown_window_stories_flag_exits_0_when_satisfied() {
    let script = repo_root().join("scripts/check-shakedown-window.sh");
    if !script.exists() {
        panic!(
            "PRECONDITION FAIL: scripts/check-shakedown-window.sh does not exist"
        );
    }

    let output = Command::new("bash")
        .arg(&script)
        .arg("--stories")
        .arg("S-3.01,S-3.02,S-3.03")
        .arg("--days")
        .arg("7")
        .env("VSDD_SHAKEDOWN_MOCK_SATISFIED", "1")
        .output()
        .expect("execute check-shakedown-window.sh");

    assert_eq!(
        output.status.code(),
        Some(0),
        "FAIL AC-10 (BC-9.01.006 PC4): check-shakedown-window.sh --stories must exit 0 \
         when VSDD_SHAKEDOWN_MOCK_SATISFIED=1 (WASM port exposure satisfied). \
         Current exit code: {:?}\nstderr: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );
}

// ---------------------------------------------------------------------------
// AC-13 — check-changelog-monotonicity.sh exit-code contract
// ---------------------------------------------------------------------------

/// check-changelog-monotonicity.sh must exist in scripts/.
#[test]
fn test_BC_9_01_changelog_monotonicity_script_exists() {
    let path = repo_root().join("scripts/check-changelog-monotonicity.sh");
    assert!(
        path.exists(),
        "FAIL AC-13: scripts/check-changelog-monotonicity.sh must exist. \
         Create it as part of S-4.08 File Structure Requirements."
    );
}

/// AC-13: check-changelog-monotonicity.sh must exit 0 for a monotonic CHANGELOG.
///
/// RED GATE: stub always exits 2. Will fail until real implementation.
/// We pass a temp file with a monotonic (descending-date) CHANGELOG.
#[test]
fn test_BC_9_01_changelog_monotonicity_exits_0_for_monotonic_changelog() {
    let script = repo_root().join("scripts/check-changelog-monotonicity.sh");
    if !script.exists() {
        panic!(
            "PRECONDITION FAIL: scripts/check-changelog-monotonicity.sh does not exist"
        );
    }

    let tmp = tempfile::tempdir().expect("create temp dir");
    let changelog = tmp.path().join("CHANGELOG.md");
    // Monotonic: newer date first (2026-04-28 > 2026-04-25)
    fs::write(
        &changelog,
        "# Changelog\n\n\
         ## 1.0.0-rc.1 — Release Candidate 1 (2026-04-28)\n\nSome content.\n\n\
         ## 1.0.0-beta.4 — previous (2026-04-25)\n\nOlder content.\n",
    )
    .expect("write temp CHANGELOG.md");

    let output = Command::new("bash")
        .arg(&script)
        .arg(changelog.to_str().unwrap())
        .output()
        .expect("execute check-changelog-monotonicity.sh");

    assert_eq!(
        output.status.code(),
        Some(0),
        "FAIL AC-13: check-changelog-monotonicity.sh must exit 0 for a monotonic \
         CHANGELOG. \
         Current exit code: {:?}\nstderr: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );
}

/// AC-13: check-changelog-monotonicity.sh must exit 1 for a non-monotonic CHANGELOG.
///
/// RED GATE: stub exits 2 (not 1). Will fail until real implementation.
#[test]
fn test_BC_9_01_changelog_monotonicity_exits_1_for_non_monotonic_changelog() {
    let script = repo_root().join("scripts/check-changelog-monotonicity.sh");
    if !script.exists() {
        panic!(
            "PRECONDITION FAIL: scripts/check-changelog-monotonicity.sh does not exist"
        );
    }

    let tmp = tempfile::tempdir().expect("create temp dir");
    let changelog = tmp.path().join("CHANGELOG.md");
    // Non-monotonic: older date first (2026-04-20 < 2026-04-25 but listed earlier)
    fs::write(
        &changelog,
        "# Changelog\n\n\
         ## 1.0.0-rc.1 — Release Candidate 1 (2026-04-20)\n\nSome content.\n\n\
         ## 1.0.0-beta.4 — previous (2026-04-25)\n\nOlder content.\n",
    )
    .expect("write temp CHANGELOG.md");

    let output = Command::new("bash")
        .arg(&script)
        .arg(changelog.to_str().unwrap())
        .output()
        .expect("execute check-changelog-monotonicity.sh");

    assert_eq!(
        output.status.code(),
        Some(1),
        "FAIL AC-13: check-changelog-monotonicity.sh must exit 1 for non-monotonic \
         CHANGELOG (later entry has older date). \
         Current exit code: {:?}",
        output.status.code()
    );
}

/// AC-13: check-changelog-monotonicity.sh must exit 2 when CHANGELOG.md is missing.
///
/// The stub already exits 2, so this test should pass on the stub AND
/// on the real implementation — documenting the contract.
#[test]
fn test_BC_9_01_changelog_monotonicity_exits_2_for_missing_file() {
    let script = repo_root().join("scripts/check-changelog-monotonicity.sh");
    if !script.exists() {
        panic!(
            "PRECONDITION FAIL: scripts/check-changelog-monotonicity.sh does not exist"
        );
    }

    let output = Command::new("bash")
        .arg(&script)
        .arg("/tmp/nonexistent-changelog-for-s4-08-test.md")
        .output()
        .expect("execute check-changelog-monotonicity.sh");

    assert_eq!(
        output.status.code(),
        Some(2),
        "FAIL AC-13: check-changelog-monotonicity.sh must exit 2 when CHANGELOG.md \
         is missing. \
         Current exit code: {:?}",
        output.status.code()
    );
}

// ---------------------------------------------------------------------------
// AC-Q4 — LICENSE present (no extension); deny.toml present
// ---------------------------------------------------------------------------

/// AC-Q4: LICENSE file must exist at repo root (no extension per OSS convention).
///
/// RED GATE: passes today (LICENSE already present) — verifies gate remains green.
/// This is a regression guard.
#[test]
fn test_BC_9_01_q4_license_file_exists_without_extension() {
    let license = repo_root().join("LICENSE");
    assert!(
        license.exists(),
        "FAIL AC-Q4: LICENSE file must exist at repo root (no extension, per OSS \
         convention). File not found at {:?}",
        license
    );

    // Also verify no LICENSE.md (wrong extension) masquerades as the real one.
    let license_md = repo_root().join("LICENSE.md");
    // We don't FAIL if LICENSE.md also exists (some repos have both), but
    // the canonical check is that bare LICENSE exists.
    let _ = license_md; // no assertion needed
}

/// AC-Q4: deny.toml must exist at repo root.
///
/// RED GATE: fails today (deny.toml not yet created). Implementer must
/// create deny.toml per Task 11 of S-4.08.
#[test]
fn test_BC_9_01_q4_deny_toml_exists_at_repo_root() {
    let deny = repo_root().join("deny.toml");
    assert!(
        deny.exists(),
        "FAIL AC-Q4: deny.toml must exist at repo root to enable 'cargo deny check \
         licenses'. Create it per S-4.08 Task 11 with approved license allow-list \
         (MIT, Apache-2.0, BSD-3-Clause, ISC, MPL-2.0). File not found at {:?}",
        deny
    );
}

// ---------------------------------------------------------------------------
// AC-Q5 — Documentation freeze: README.md, CHANGELOG.md, CONTRIBUTING.md present
// ---------------------------------------------------------------------------

/// AC-Q5: README.md must be present at repo root.
#[test]
fn test_BC_9_01_q5_readme_present() {
    let readme = repo_root().join("README.md");
    assert!(
        readme.exists(),
        "FAIL AC-Q5: README.md must be present at repo root for rc.1 docs freeze. \
         Not found at {:?}",
        readme
    );
}

/// AC-Q5: CHANGELOG.md must be present at repo root.
#[test]
fn test_BC_9_01_q5_changelog_present() {
    let changelog = repo_root().join("CHANGELOG.md");
    assert!(
        changelog.exists(),
        "FAIL AC-Q5: CHANGELOG.md must be present at repo root for rc.1 docs freeze. \
         Not found at {:?}",
        changelog
    );
}

/// AC-Q5: CONTRIBUTING.md must be present at repo root.
#[test]
fn test_BC_9_01_q5_contributing_present() {
    let contributing = repo_root().join("CONTRIBUTING.md");
    assert!(
        contributing.exists(),
        "FAIL AC-Q5: CONTRIBUTING.md must be present at repo root for rc.1 docs freeze. \
         Not found at {:?}",
        contributing
    );
}
