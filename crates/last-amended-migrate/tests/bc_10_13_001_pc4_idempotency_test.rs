// Test files use .expect()/.unwrap()/.panic!() for failure reporting.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
//! BC-10.13.001 PC4 (migration idempotency) + Invariant 2 (idempotent on
//! every subcommand) + EC-001 (already-fully-compliant file is a
//! verified-clean no-op).
//!
//! Complements the idempotency assertions embedded in the PC3 escape-test
//! file (which covers the escape-specific idempotency angle) with the
//! general contract across a variety of starting shapes: a file that was
//! ALREADY fully compliant before the first call ever runs, and the
//! "second run after a real mutation" angle from a `changelog:`-bootstrap
//! starting point rather than an escape-fix starting point.

mod common;

use last_amended_migrate::migrate::MigrationMode;
use last_amended_migrate::migrate_file;

/// PC4 / EC-001: a fixture that is ALREADY fully compliant (has
/// `changelog:`, clean current-entry-only `last_amended`, no D-1144 defect)
/// produces a verified-clean report — zero mutations — on the FIRST call,
/// and the file content is byte-for-byte unchanged.
#[test]
fn test_BC_10_13_001_EC001_already_compliant_file_is_noop_on_first_run() {
    let dir = tempfile::tempdir().expect("tempdir");
    let last_amended = common::clean_current_entry("2026-09-02", "v4.11", "already compliant");
    let content = common::frontmatter_file(
        "architecture-index",
        "4.11",
        &last_amended,
        Some(&[common::changelog_item_block("2026-08-01", "an older entry")]),
        "# Fixture ARCH-INDEX\n",
    );
    let path = common::write_file(dir.path(), "ARCH-INDEX.md", &content);
    let before = common::read_file(&path);

    let report = migrate_file(&path, MigrationMode::Apply).expect("migrate_file must succeed");

    assert!(!report.mutated, "EC-001: already-compliant file is a no-op");
    assert!(!report.escape_fixed);
    let after = common::read_file(&path);
    assert_eq!(before, after, "content must be byte-for-byte unchanged");
}

/// PC4: running migration a SECOND time (after a REAL first-run mutation
/// that added `changelog:`) is a verified-clean no-op — the tool "detects
/// the already-compliant shape and takes no destructive or duplicating
/// action (it never double-adds a `changelog:` field ...)".
#[test]
fn test_BC_10_13_001_PC4_second_run_after_changelog_bootstrap_is_noop() {
    let dir = tempfile::tempdir().expect("tempdir");
    let last_amended = common::clean_current_entry("2026-09-02", "v4.430", "bootstrap fixture");
    let content = common::frontmatter_file(
        "story-index",
        "4.430",
        &last_amended,
        None,
        "# Fixture STORY-INDEX\n",
    );
    let path = common::write_file(dir.path(), "STORY-INDEX.md", &content);

    let first = migrate_file(&path, MigrationMode::Apply).expect("first migrate_file call");
    assert!(first.mutated, "first run adds changelog: — a real mutation");
    let after_first = common::read_file(&path);

    let second = migrate_file(&path, MigrationMode::Apply).expect("second migrate_file call");
    assert!(
        !second.mutated,
        "PC4: second run must be a verified-clean no-op (0 mutations)"
    );

    let after_second = common::read_file(&path);
    assert_eq!(
        after_first, after_second,
        "PC4: second run must produce byte-identical output to the first \
         run's result — no double-add, no re-wrap"
    );
}

/// Invariant 2 extends the idempotency guarantee to `Check` mode too:
/// running `Check` against an already-migrated file must also report zero
/// mutations (and, being `Check` mode, must never write regardless).
#[test]
fn test_BC_10_13_001_invariant2_check_mode_after_apply_reports_zero_mutations() {
    let dir = tempfile::tempdir().expect("tempdir");
    let last_amended = common::clean_current_entry("2026-09-02", "v4.430", "bootstrap fixture");
    let content = common::frontmatter_file(
        "story-index",
        "4.430",
        &last_amended,
        None,
        "# Fixture STORY-INDEX\n",
    );
    let path = common::write_file(dir.path(), "STORY-INDEX.md", &content);

    let apply = migrate_file(&path, MigrationMode::Apply).expect("apply call");
    assert!(apply.mutated);
    let after_apply = common::read_file(&path);

    let check = migrate_file(&path, MigrationMode::Check).expect("check call");
    assert!(
        !check.mutated,
        "Check mode after a real Apply must report zero mutations (already compliant)"
    );

    let after_check = common::read_file(&path);
    assert_eq!(
        after_apply, after_check,
        "Check mode must NEVER write, by definition"
    );
}
