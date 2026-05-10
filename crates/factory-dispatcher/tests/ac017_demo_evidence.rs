// Test files use .expect()/.unwrap()/.panic!() for failure reporting.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
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

use regex::Regex;
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

/// AC-016 + AC-017: latency-canary.md must record a parseable p95 value ≤ 1500ms
/// and name the canonical test command.
///
/// # Why numeric parse, not string-contains (F-P2-014 process-gap codification)
///
/// The previous guard (`content.contains("p95")`) was structurally weak: a file
/// containing "p95 = 99999ms — FAIL" would have passed. This version:
///
/// 1. Parses the canonical p95 table row via regex and extracts the numeric value
/// 2. Asserts the parsed value is ≤ 1500ms (AC-016 budget per ADR-020 Class A)
/// 3. Asserts the methodology section names the canonical test command
///
/// The canonical format is a Markdown table row:
///   `| p95 | 1050ms | <= 1500ms (Class A, ADR-020) | **PASS** |`
///
/// The regex matches: `| p95 | <N>ms |` where <N> is the numeric value.
///
/// # AC-016 budget per ADR-020 (Class A — cold-start dispatch)
///
/// Original 500ms revised after F5 pass-1 finding F-P1-003 + F-P1-009.
///
/// RED until demo-recorder phase produces the measurement file.
#[test]
fn test_BC_1_14_001_ac016_latency_canary_md_contains_p95_value() {
    /// AC-016 Class A budget (ADR-020): p95 ≤ 1500ms.
    const AC016_BUDGET_MS: u64 = 1500;

    /// Canonical test command that must appear in the methodology section.
    const CANONICAL_CMD: &str = "cargo test --release -p factory-dispatcher --test latency_canary";

    let canary_path = evidence_dir().join("latency-canary.md");

    // AC-016 budget per ADR-020 (Class A — cold-start dispatch). Original 500ms revised after F5 pass-1 finding F-P1-003 + F-P1-009.
    if !canary_path.exists() {
        panic!(
            "test_BC_1_14_001_ac016_latency_canary_md_contains_p95_value: \
             AC-016/AC-017 FAIL — docs/demo-evidence/S-15.01/latency-canary.md does not exist. \
             Demo-recorder phase (F4 Step 4) must produce this file with p95 ≤ 1500ms per ADR-020 §Class A."
        );
    }

    let content = std::fs::read_to_string(canary_path).expect("latency-canary.md must be readable");

    // --- Guard 1: parse the canonical p95 table row and assert ≤ 1500ms ---
    //
    // Matches the canonical Markdown table row:
    //   | p95 | 1050ms | <= 1500ms (Class A, ADR-020) | **PASS** |
    //
    // Regex: `\|\s*[Pp]95\s*\|\s*(\d+)\s*ms` — captures the numeric value.
    // If multiple p95 rows appear (e.g., debug vs release tables), we assert
    // the canonical-test value (the first match) is ≤ 1500ms. Debug-mode jitter
    // is informational and not asserted here (ADR-020 §Class A applies to release).
    let p95_re = Regex::new(r"\|\s*[Pp]95\s*\|\s*(\d+)\s*ms").expect("p95 regex must compile");

    let p95_ms: u64 = match p95_re.captures(&content) {
        Some(caps) => caps[1]
            .parse()
            .expect("p95 capture group must be a valid integer"),
        None => panic!(
            "test_BC_1_14_001_ac016_latency_canary_md_contains_p95_value: \
             AC-016 FAIL — latency-canary.md does not contain a parseable p95 table row. \
             Expected format: '| p95 | <N>ms | ...' per the canonical demo-evidence template. \
             The file must record the p95 sync_group latency (≤ {}ms per ADR-020 §Class A).",
            AC016_BUDGET_MS
        ),
    };

    assert!(
        p95_ms <= AC016_BUDGET_MS,
        "test_BC_1_14_001_ac016_latency_canary_md_contains_p95_value: \
         AC-016 FAIL — latency-canary.md records p95 = {}ms which EXCEEDS the AC-016 Class A budget \
         of {}ms per ADR-020 §Class A. \
         The demo-recorder must re-run the canary and update latency-canary.md with a conforming measurement.",
        p95_ms,
        AC016_BUDGET_MS
    );

    // --- Guard 2: methodology section must name the canonical test command ---
    //
    // Ensures the demo file documents HOW the measurement was taken, not just the result.
    // The canonical command is:
    //   cargo test --release -p factory-dispatcher --test latency_canary -- --ignored
    //
    // We check for the prefix (without flags) to allow for minor flag variation
    // (e.g., --nocapture) without false negatives.
    assert!(
        content.contains(CANONICAL_CMD),
        "test_BC_1_14_001_ac016_latency_canary_md_contains_p95_value: \
         AC-017 FAIL — latency-canary.md does not reference the canonical test command. \
         The methodology section must name '{}' so that the measurement is reproducible. \
         A p95 value without provenance cannot serve as demo evidence (AC-017).",
        CANONICAL_CMD
    );
}
