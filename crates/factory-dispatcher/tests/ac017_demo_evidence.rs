//! AC-017: Demo evidence completeness check.
//!
//! Verifies that `docs/demo-evidence/S-15.01/` contains all 5 required
//! evidence files after the demo-recorder phase completes.
//!
//! # Red Gate
//!
//! RED until demo-recorder phase (F4 Step 4): the evidence directory and
//! files do not yet exist. All assertions fail with a clear message.
//!
//! # Required files (AC-017)
//!
//! - `before-silent-block.md`    — replay of prism silent-block scenario
//! - `after-visible-block.md`    — dispatcher logs showing block surfaced
//! - `latency-canary.md`         — p95 latency measurement (AC-016)
//! - `schema-mismatch-error.md`  — terminal output of v1 registry rejection
//! - `async-telemetry-drain.md`  — events-*.jsonl excerpt within drain window
//!
//! # BC traces
//!
//! - AC-017 (S-15.01 v1.6): demo evidence completeness
//! - AC-016: p95 latency measurement must appear in latency-canary.md
//! - DI-019: ASYNC_DRAIN_WINDOW_MS referenced in async-telemetry-drain.md

use std::path::PathBuf;

/// The 5 required demo evidence files for S-15.01 (AC-017).
const REQUIRED_DEMO_FILES: &[&str] = &[
    "before-silent-block.md",
    "after-visible-block.md",
    "latency-canary.md",
    "schema-mismatch-error.md",
    "async-telemetry-drain.md",
];

/// Resolve the path to docs/demo-evidence/S-15.01/ from CARGO_MANIFEST_DIR.
/// CARGO_MANIFEST_DIR = crates/factory-dispatcher; walk up 2 to workspace root.
fn evidence_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates/")
        .parent()
        .expect("workspace root")
        .join("docs/demo-evidence/S-15.01")
}

/// AC-017: All 5 demo evidence files must exist in docs/demo-evidence/S-15.01/.
///
/// RED until demo-recorder phase: directory and files do not yet exist.
#[test]
fn test_BC_3_08_001_ac017_demo_evidence_directory_exists() {
    let evidence_dir = evidence_dir();
    assert!(
        evidence_dir.exists(),
        "test_BC_3_08_001_ac017_demo_evidence_directory_exists: \
         AC-017 FAIL — docs/demo-evidence/S-15.01/ does not exist. \
         Demo-recorder phase (F4 Step 4) must create this directory with all 5 evidence files."
    );
}

/// AC-017: Each of the 5 required evidence files must exist.
///
/// RED until demo-recorder phase.
#[test]
fn test_BC_3_08_001_ac017_all_required_demo_files_present() {
    let evidence_dir = evidence_dir();

    // Fail fast if directory is missing — the directory test gives the primary signal.
    if !evidence_dir.exists() {
        // Panic with a clear message listing all missing files.
        let missing: Vec<&str> = REQUIRED_DEMO_FILES.to_vec();
        panic!(
            "test_BC_3_08_001_ac017_all_required_demo_files_present: \
             AC-017 FAIL — demo evidence directory does not exist. \
             Missing all {} files: {:?}. \
             Demo-recorder phase (F4 Step 4) must produce these files.",
            missing.len(),
            missing
        );
    }

    let mut missing = Vec::new();
    for filename in REQUIRED_DEMO_FILES {
        let path = evidence_dir.join(filename);
        if !path.exists() {
            missing.push(*filename);
        }
    }

    assert!(
        missing.is_empty(),
        "test_BC_3_08_001_ac017_all_required_demo_files_present: \
         AC-017 FAIL — missing demo evidence files in docs/demo-evidence/S-15.01/: {:?}. \
         Demo-recorder phase (F4 Step 4) must produce all 5 files.",
        missing
    );
}

/// AC-016 + AC-017: latency-canary.md must contain a p95 value (not blank).
///
/// RED until demo-recorder phase produces the measurement file.
#[test]
fn test_BC_1_14_001_ac016_latency_canary_md_contains_p95_value() {
    let canary_path = evidence_dir().join("latency-canary.md");

    if !canary_path.exists() {
        panic!(
            "test_BC_1_14_001_ac016_latency_canary_md_contains_p95_value: \
             AC-016/AC-017 FAIL — docs/demo-evidence/S-15.01/latency-canary.md does not exist. \
             Demo-recorder phase (F4 Step 4) must produce this file with p95 ≤ 500ms."
        );
    }

    let content = std::fs::read_to_string(canary_path).expect("latency-canary.md must be readable");

    assert!(
        content.contains("p95") || content.contains("P95"),
        "test_BC_1_14_001_ac016_latency_canary_md_contains_p95_value: \
         AC-016 FAIL — latency-canary.md does not contain a 'p95' measurement. \
         The file must record the p95 sync_group latency (≤ 500ms per AC-016)."
    );
}
