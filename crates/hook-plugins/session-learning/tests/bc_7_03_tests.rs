//! Red Gate test suite for S-8.06: session-learning WASM hook port.
//!
//! Every test in this file MUST FAIL until `session_learning_logic` and
//! `format_utc_now` are implemented in T-3.
//!
//! Behavioral contracts under test:
//!   BC-7.03.076 — identity & registry binding (Stop, priority=910, on_error=continue)
//!   BC-7.03.077 — appends timestamped marker to .factory/sidecar-learning.md;
//!                 creates file with header if absent; append-only invariant
//!   BC-7.03.078 — skips (exit 0 / HookResult::Continue) when .factory/ absent
//!
//! Story ACs covered:
//!   AC-001 (BC-7.03.076): hook always returns Continue (exit 0)
//!   AC-002 (BC-7.03.077): .factory/ present + file absent → header + marker created
//!   AC-003 (BC-7.03.078): .factory/ absent → Continue immediately, no file created
//!   AC-004 (BC-7.03.077): append-only — no duplicate header on repeat invocations
//!
//! Edge cases from S-8.06 story spec:
//!   EC-001: .factory/ not writable → non-zero / HookResult::Error (parity with bash set -e)
//!   EC-002: sidecar-learning.md exists with partial header → append only, no header rewrite
//!   EC-003: consecutive invocations (< 1s apart) → each produces a distinct line
//!   EC-004: very large sidecar-learning.md → append-only, no full-file buffering
//!   EC-005: large stdin envelope → plugin completes cleanly (tested via bats)

use session_learning::{MARKER_FORMAT, SIDECAR_HEADER, format_utc_now, session_learning_logic};
use vsdd_hook_sdk::HookResult;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Return a fixed deterministic timestamp for injection in tests.
fn fixed_ts() -> String {
    "2026-01-15T10:30:00Z".to_string()
}

/// Return a second distinct timestamp for consecutive-invocation tests.
fn fixed_ts2() -> String {
    "2026-01-15T10:30:01Z".to_string()
}

/// Build the expected marker line for a given timestamp.
fn expected_marker(ts: &str) -> String {
    MARKER_FORMAT.replace("{}", ts)
}

// ---------------------------------------------------------------------------
// BC-7.03.078 — skip when .factory/ absent
// ---------------------------------------------------------------------------

/// AC-003 traces to BC-7.03.078 postcondition 1.
/// Precondition: .factory/ directory does NOT exist in fs_root.
/// Postcondition: returns HookResult::Continue immediately; no files created.
#[test]
fn test_BC_7_03_078_exits_continue_when_factory_absent() {
    let tmp = tempfile::tempdir().unwrap();
    // Do NOT create .factory/ inside tmp — it must be absent.
    let result = session_learning_logic(fixed_ts, tmp.path().to_str().unwrap());
    // Must return Continue (exit 0) per BC-7.03.078 postcondition 1.
    assert_eq!(result, HookResult::Continue);
    // Must not create any file.
    assert!(
        !tmp.path().join(".factory").exists(),
        ".factory/ must not be created when it was absent"
    );
}

/// AC-003 invariant: no sidecar file created when .factory/ is absent.
#[test]
fn test_BC_7_03_078_no_sidecar_created_when_factory_absent() {
    let tmp = tempfile::tempdir().unwrap();
    let _ = session_learning_logic(fixed_ts, tmp.path().to_str().unwrap());
    let sidecar = tmp.path().join(".factory").join("sidecar-learning.md");
    assert!(
        !sidecar.exists(),
        "sidecar-learning.md must not be created when .factory/ is absent"
    );
}

// ---------------------------------------------------------------------------
// BC-7.03.077 — creates file with header when absent
// ---------------------------------------------------------------------------

/// AC-002 traces to BC-7.03.077 postcondition 1.
/// Precondition: .factory/ exists; sidecar-learning.md does NOT exist.
/// Postcondition: file created with SIDECAR_HEADER then one marker line; exits 0.
#[test]
fn test_BC_7_03_077_creates_file_with_header_when_absent() {
    let tmp = tempfile::tempdir().unwrap();
    let factory_dir = tmp.path().join(".factory");
    std::fs::create_dir_all(&factory_dir).unwrap();

    let result = session_learning_logic(fixed_ts, tmp.path().to_str().unwrap());
    assert_eq!(result, HookResult::Continue);

    let sidecar = factory_dir.join("sidecar-learning.md");
    assert!(sidecar.exists(), "sidecar-learning.md must be created");

    let content = std::fs::read_to_string(&sidecar).unwrap();
    assert!(
        content.starts_with(SIDECAR_HEADER),
        "file must start with SIDECAR_HEADER; got: {:?}",
        &content[..content.len().min(120)]
    );
}

/// AC-002: header trailing blank line is REQUIRED (bats byte-identical comparison).
/// The header must end with "\n\n" (two newlines after last sentence).
#[test]
fn test_BC_7_03_077_header_has_trailing_blank_line() {
    // Verify the constant itself ends with the required trailing blank line.
    assert!(
        SIDECAR_HEADER.ends_with("\n\n"),
        "SIDECAR_HEADER must end with trailing blank line (\\n\\n); got: {:?}",
        &SIDECAR_HEADER[SIDECAR_HEADER.len().saturating_sub(4)..]
    );

    let tmp = tempfile::tempdir().unwrap();
    let factory_dir = tmp.path().join(".factory");
    std::fs::create_dir_all(&factory_dir).unwrap();

    let _ = session_learning_logic(fixed_ts, tmp.path().to_str().unwrap());
    let content = std::fs::read_to_string(factory_dir.join("sidecar-learning.md")).unwrap();
    // Content must contain the full header including trailing blank line.
    assert!(
        content.contains(SIDECAR_HEADER),
        "sidecar-learning.md must contain exact SIDECAR_HEADER with trailing blank line"
    );
}

/// AC-002: marker line format is `- Session ended at <TS> (awaiting /session-review)\n`.
#[test]
fn test_BC_7_03_077_appends_marker_line_with_correct_format() {
    let tmp = tempfile::tempdir().unwrap();
    let factory_dir = tmp.path().join(".factory");
    std::fs::create_dir_all(&factory_dir).unwrap();

    let result = session_learning_logic(fixed_ts, tmp.path().to_str().unwrap());
    assert_eq!(result, HookResult::Continue);

    let content = std::fs::read_to_string(factory_dir.join("sidecar-learning.md")).unwrap();
    let marker = expected_marker("2026-01-15T10:30:00Z");
    assert!(
        content.contains(&marker),
        "sidecar-learning.md must contain marker line {:?}; got content: {:?}",
        marker,
        content
    );
}

/// AC-002: exactly one marker line on first invocation.
#[test]
fn test_BC_7_03_077_exactly_one_marker_line_on_first_invocation() {
    let tmp = tempfile::tempdir().unwrap();
    let factory_dir = tmp.path().join(".factory");
    std::fs::create_dir_all(&factory_dir).unwrap();

    let _ = session_learning_logic(fixed_ts, tmp.path().to_str().unwrap());

    let content = std::fs::read_to_string(factory_dir.join("sidecar-learning.md")).unwrap();
    let marker_count = content
        .lines()
        .filter(|l| l.starts_with("- Session ended at "))
        .count();
    assert_eq!(
        marker_count, 1,
        "first invocation must produce exactly one marker line; got {marker_count}"
    );
}

// ---------------------------------------------------------------------------
// BC-7.03.077 — append-only invariant (AC-004)
// ---------------------------------------------------------------------------

/// AC-004 traces to BC-7.03.077 postcondition 1 (append-only invariant).
/// Precondition: .factory/ exists; sidecar-learning.md pre-exists with prior content.
/// Postcondition: one NEW marker line appended; no duplicate header; prior content intact.
#[test]
fn test_BC_7_03_077_append_only_no_duplicate_header() {
    let tmp = tempfile::tempdir().unwrap();
    let factory_dir = tmp.path().join(".factory");
    std::fs::create_dir_all(&factory_dir).unwrap();
    let sidecar = factory_dir.join("sidecar-learning.md");

    // Pre-populate with header + one prior marker (simulate a previous session).
    let prior_content = format!(
        "{}{}",
        SIDECAR_HEADER, "- Session ended at 2026-01-01T00:00:00Z (awaiting /session-review)\n"
    );
    std::fs::write(&sidecar, &prior_content).unwrap();

    let result = session_learning_logic(fixed_ts, tmp.path().to_str().unwrap());
    assert_eq!(result, HookResult::Continue);

    let content = std::fs::read_to_string(&sidecar).unwrap();

    // Must still begin with the original header (no header overwrite).
    assert!(
        content.starts_with(SIDECAR_HEADER),
        "pre-existing header must be preserved unchanged"
    );

    // Header must appear exactly ONCE.
    let header_occurrences = content.matches("# Sidecar Learning").count();
    assert_eq!(
        header_occurrences, 1,
        "SIDECAR_HEADER must appear exactly once; found {header_occurrences} occurrences"
    );

    // New marker line must be present.
    let new_marker = expected_marker("2026-01-15T10:30:00Z");
    assert!(
        content.contains(&new_marker),
        "new marker line must be appended; content: {:?}",
        content
    );

    // Prior marker must still be present (content not overwritten).
    assert!(
        content.contains("2026-01-01T00:00:00Z"),
        "prior marker must be preserved; content: {:?}",
        content
    );
}

/// AC-004: second invocation adds exactly one additional marker line.
#[test]
fn test_BC_7_03_077_second_invocation_appends_exactly_one_more_line() {
    let tmp = tempfile::tempdir().unwrap();
    let factory_dir = tmp.path().join(".factory");
    std::fs::create_dir_all(&factory_dir).unwrap();

    // First invocation — creates file.
    let _ = session_learning_logic(fixed_ts, tmp.path().to_str().unwrap());
    // Second invocation — must append, not overwrite.
    let _ = session_learning_logic(fixed_ts2, tmp.path().to_str().unwrap());

    let content = std::fs::read_to_string(factory_dir.join("sidecar-learning.md")).unwrap();
    let marker_count = content
        .lines()
        .filter(|l| l.starts_with("- Session ended at "))
        .count();
    assert_eq!(
        marker_count, 2,
        "two invocations must produce exactly 2 marker lines; got {marker_count}"
    );
}

// ---------------------------------------------------------------------------
// BC-7.03.076 — always exits Continue (exit 0)
// ---------------------------------------------------------------------------

/// AC-001 traces to BC-7.03.076 postcondition 2 (exit code 0 always, on_error=continue).
/// The hook must always return HookResult::Continue on success paths.
#[test]
fn test_BC_7_03_076_always_returns_continue_when_factory_present() {
    let tmp = tempfile::tempdir().unwrap();
    let factory_dir = tmp.path().join(".factory");
    std::fs::create_dir_all(&factory_dir).unwrap();

    let result = session_learning_logic(fixed_ts, tmp.path().to_str().unwrap());
    assert_eq!(
        result,
        HookResult::Continue,
        "hook must return Continue (exit 0) when .factory/ present"
    );
}

/// BC-7.03.076 postcondition 2: no emit_event calls, no subprocess invocations.
/// Structural test: format_utc_now must produce ISO-8601 UTC format when implemented.
/// (This test fails now because format_utc_now panics with unimplemented!().)
#[test]
fn test_BC_7_03_076_format_utc_now_matches_iso8601_utc_format() {
    // When implemented, format_utc_now() must return a string matching
    // the pattern YYYY-MM-DDTHH:MM:SSZ (same as bash `date -u +"%Y-%m-%dT%H:%M:%SZ"`).
    let ts = format_utc_now();
    // Must be exactly 20 characters: 2026-01-15T10:30:00Z
    assert_eq!(ts.len(), 20, "UTC timestamp must be 20 chars; got {:?}", ts);
    // Must end with 'Z'.
    assert!(
        ts.ends_with('Z'),
        "UTC timestamp must end with 'Z'; got {:?}",
        ts
    );
    // Must match the pattern with hyphens and T separator.
    assert!(
        ts.contains('T'),
        "UTC timestamp must contain 'T' separator; got {:?}",
        ts
    );
    // Validate format with a simple regex-like check.
    let chars: Vec<char> = ts.chars().collect();
    assert!(chars[4] == '-', "position 4 must be '-'");
    assert!(chars[7] == '-', "position 7 must be '-'");
    assert!(chars[10] == 'T', "position 10 must be 'T'");
    assert!(chars[13] == ':', "position 13 must be ':'");
    assert!(chars[16] == ':', "position 16 must be ':'");
}

// ---------------------------------------------------------------------------
// Edge Case EC-001: .factory/ not writable → non-zero exit
// ---------------------------------------------------------------------------

/// EC-001: .factory/ directory exists but sidecar-learning.md is not writable.
/// Expected: HookResult::Error (non-zero exit), matching bash set -euo pipefail behavior.
/// Skipped when running as root (root bypasses Unix permission checks).
#[test]
#[cfg(unix)]
fn test_BC_7_03_077_ec001_write_error_returns_error_result() {
    use std::os::unix::fs::PermissionsExt;

    // Skip if running as root (root bypasses permission checks on most systems).
    // We detect root by trying to read /etc/shadow; if readable we're root.
    // Simpler: create a 0o000 file and attempt to open it — if we can, we're root.
    let root_check_dir = tempfile::tempdir().unwrap();
    let root_probe = root_check_dir.path().join("probe");
    std::fs::write(&root_probe, b"x").unwrap();
    let mut probe_perms = std::fs::metadata(&root_probe).unwrap().permissions();
    probe_perms.set_mode(0o000);
    std::fs::set_permissions(&root_probe, probe_perms).unwrap();
    let is_root = std::fs::read(&root_probe).is_ok();
    if is_root {
        // Running as root — permission bits don't apply; skip test.
        return;
    }

    let tmp = tempfile::tempdir().unwrap();
    let factory_dir = tmp.path().join(".factory");
    std::fs::create_dir_all(&factory_dir).unwrap();

    // Create sidecar-learning.md and make it read-only.
    let sidecar = factory_dir.join("sidecar-learning.md");
    std::fs::write(&sidecar, SIDECAR_HEADER).unwrap();
    let mut perms = std::fs::metadata(&sidecar).unwrap().permissions();
    perms.set_mode(0o444); // read-only
    std::fs::set_permissions(&sidecar, perms).unwrap();

    let result = session_learning_logic(fixed_ts, tmp.path().to_str().unwrap());

    // Must NOT return Continue — must signal error (non-zero exit).
    // Parity with bash: set -euo pipefail causes non-zero exit on >> failure (EC-001).
    assert_ne!(
        result,
        HookResult::Continue,
        "write failure must not silently return Continue; must be Error or propagate"
    );
}

// ---------------------------------------------------------------------------
// Edge Case EC-002: partial header — append only, no rewrite
// ---------------------------------------------------------------------------

/// EC-002: sidecar-learning.md exists with partial header (no trailing blank line).
/// Expected: marker appended without re-adding header; file integrity preserved.
#[test]
fn test_BC_7_03_077_ec002_partial_header_no_rewrite() {
    let tmp = tempfile::tempdir().unwrap();
    let factory_dir = tmp.path().join(".factory");
    std::fs::create_dir_all(&factory_dir).unwrap();
    let sidecar = factory_dir.join("sidecar-learning.md");

    // Write partial header (missing trailing blank line — simulates corruption/truncation).
    let partial = "# Sidecar Learning\n\nSession-end markers for the VSDD factory. Run /session-review to synthesize.\n";
    std::fs::write(&sidecar, partial).unwrap();

    let result = session_learning_logic(fixed_ts, tmp.path().to_str().unwrap());
    assert_eq!(result, HookResult::Continue);

    let content = std::fs::read_to_string(&sidecar).unwrap();

    // Header must appear exactly once (not re-added).
    let header_count = content.matches("# Sidecar Learning").count();
    assert_eq!(
        header_count, 1,
        "header must appear exactly once even with partial pre-existing header; got {header_count}"
    );

    // New marker must be present.
    let marker = expected_marker("2026-01-15T10:30:00Z");
    assert!(
        content.contains(&marker),
        "marker must be appended after partial header; content: {:?}",
        content
    );
}

// ---------------------------------------------------------------------------
// Edge Case EC-003: consecutive invocations produce distinct timestamp lines
// ---------------------------------------------------------------------------

/// EC-003: stop event fires during rapid consecutive sessions (< 1s apart).
/// Each invocation appends a distinct line. No deduplication is performed.
/// ISO-8601 precision to seconds is sufficient.
#[test]
fn test_BC_7_03_077_ec003_consecutive_invocations_distinct_timestamps() {
    let tmp = tempfile::tempdir().unwrap();
    let factory_dir = tmp.path().join(".factory");
    std::fs::create_dir_all(&factory_dir).unwrap();

    let ts_a = "2026-01-15T10:30:00Z";
    let ts_b = "2026-01-15T10:30:01Z"; // 1 second later

    let _ = session_learning_logic(|| ts_a.to_string(), tmp.path().to_str().unwrap());
    let _ = session_learning_logic(|| ts_b.to_string(), tmp.path().to_str().unwrap());

    let content = std::fs::read_to_string(factory_dir.join("sidecar-learning.md")).unwrap();

    let line_a = format!("- Session ended at {ts_a} (awaiting /session-review)");
    let line_b = format!("- Session ended at {ts_b} (awaiting /session-review)");

    assert!(
        content.contains(&line_a),
        "marker for ts_a must be present; content: {:?}",
        content
    );
    assert!(
        content.contains(&line_b),
        "marker for ts_b must be present; content: {:?}",
        content
    );

    // Both lines must be distinct entries (total marker count = 2).
    let count = content
        .lines()
        .filter(|l| l.starts_with("- Session ended at "))
        .count();
    assert_eq!(
        count, 2,
        "two consecutive invocations must produce 2 distinct marker lines; got {count}"
    );
}

// ---------------------------------------------------------------------------
// Edge Case EC-004: very large sidecar-learning.md — append-only, no full buffer
// ---------------------------------------------------------------------------

/// EC-004: sidecar-learning.md is very large (many previous sessions).
/// WASM implementation must use OpenOptions::new().append(true) exclusively —
/// must NOT buffer full file into memory before appending.
///
/// This test verifies the behavioral outcome (correct append) for a large file.
/// It cannot enforce "no buffer" at Rust unit-test level — that is an
/// Architecture Compliance Rule enforcement (code review gate).
/// The test documents the required behavior for the implementer.
#[test]
fn test_BC_7_03_077_ec004_large_file_append_only() {
    let tmp = tempfile::tempdir().unwrap();
    let factory_dir = tmp.path().join(".factory");
    std::fs::create_dir_all(&factory_dir).unwrap();
    let sidecar = factory_dir.join("sidecar-learning.md");

    // Write a large pre-existing file (~1 MB of prior session markers).
    let header = SIDECAR_HEADER.to_string();
    let mut large_content = header.clone();
    for i in 0..10_000 {
        large_content.push_str(&format!(
            "- Session ended at 2025-01-{:02}T{:02}:00:00Z (awaiting /session-review)\n",
            (i % 28) + 1,
            i % 24
        ));
    }
    std::fs::write(&sidecar, &large_content).unwrap();

    let size_before = std::fs::metadata(&sidecar).unwrap().len();
    let result = session_learning_logic(fixed_ts, tmp.path().to_str().unwrap());
    let size_after = std::fs::metadata(&sidecar).unwrap().len();

    assert_eq!(result, HookResult::Continue);

    // File must have grown by exactly the marker line length (append-only).
    let expected_marker = expected_marker("2026-01-15T10:30:00Z");
    let expected_growth = expected_marker.len() as u64;
    assert_eq!(
        size_after - size_before,
        expected_growth,
        "file must grow by exactly one marker line; grown by {} bytes, expected {}",
        size_after - size_before,
        expected_growth
    );

    // Header must still appear exactly once.
    let content = std::fs::read_to_string(&sidecar).unwrap();
    assert_eq!(
        content.matches("# Sidecar Learning").count(),
        1,
        "header must appear exactly once in large file"
    );
}

// ---------------------------------------------------------------------------
// Parity test: SIDECAR_HEADER constant matches bash source output byte-for-byte
// ---------------------------------------------------------------------------

/// Behavioral parity: SIDECAR_HEADER must match the exact output of the bash hook's
/// header-creation block (session-learning.sh lines 22-27 via `echo` commands).
///
/// Bash produces:
///   echo "# Sidecar Learning"    → "# Sidecar Learning\n"
///   echo ""                      → "\n"
///   echo "Session-end markers…"  → "Session-end markers for the VSDD factory. Run /session-review to synthesize.\n"
///   echo ""                      → "\n"
///
/// Combined (as > redirect): "# Sidecar Learning\n\nSession-end markers for the VSDD factory. Run /session-review to synthesize.\n\n"
#[test]
fn test_BC_7_03_077_sidecar_header_constant_matches_bash_output() {
    let expected = concat!(
        "# Sidecar Learning\n",
        "\n",
        "Session-end markers for the VSDD factory. Run /session-review to synthesize.\n",
        "\n"
    );
    assert_eq!(
        SIDECAR_HEADER, expected,
        "SIDECAR_HEADER must be byte-identical to bash source output for bats parity"
    );
}

/// Parity: MARKER_FORMAT must use '{}' placeholder exactly once.
#[test]
fn test_BC_7_03_077_marker_format_has_single_placeholder() {
    assert_eq!(
        MARKER_FORMAT.matches("{}").count(),
        1,
        "MARKER_FORMAT must have exactly one '{{}}' placeholder for the timestamp"
    );
    assert!(
        MARKER_FORMAT.starts_with("- Session ended at "),
        "MARKER_FORMAT must start with '- Session ended at '"
    );
    assert!(
        MARKER_FORMAT.contains("(awaiting /session-review)"),
        "MARKER_FORMAT must contain '(awaiting /session-review)'"
    );
    assert!(
        MARKER_FORMAT.ends_with('\n'),
        "MARKER_FORMAT must end with newline"
    );
}
