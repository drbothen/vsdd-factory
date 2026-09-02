// Test files use .expect()/.unwrap()/.panic!() for failure reporting.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
//! BC-10.13.001 PC5 (lossless changelog rotation) + EC-004 (below-threshold
//! no-op) + EC-005 (destination cycle dir auto-created) + Invariant 2
//! (rotation idempotency: re-running immediately after a successful
//! rotation, before the threshold is exceeded again, is a verified-clean
//! no-op).
//!
//! # Archive-path resolution — tested via the function's OWN report, not a
//! pre-guessed path
//!
//! `rotate_changelog(path: &Path, cycle_name: &str, keep_recent: usize,
//! mode: MigrationMode)` has no separate `factory_root` parameter (unlike
//! `migrate_all`), so exactly how it resolves `.factory/cycles/<cycle_name>/`
//! relative to an arbitrary `path` is an implementer decision the BC text
//! does not fully pin down. Rather than hardcode a specific resolution
//! scheme and risk over-constraining a choice the BC leaves open (which
//! would make this a false Red Gate — failing the implementer for a
//! reasonable design the BC never mandated), these tests locate the archive
//! via `RotationReport::archive_path` — the value the function ITSELF
//! reports — while still pinning everything the BC text DOES mandate
//! explicitly: the naming convention `<file-basename>-changelog-archive.md`
//! (PC5's own literal wording) and that it lands under a `cycles/<cycle_name>`
//! path segment.

mod common;

use std::path::Path;

use last_amended_migrate::frontmatter::parse_frontmatter;
use last_amended_migrate::migrate::MigrationMode;
use last_amended_migrate::rotate::rotate_changelog;

const KEEP_RECENT: usize = 3;

/// Build a `.factory`-rooted fixture (so a `.factory/cycles/...` archive
/// destination has somewhere sensible to land) with 8 `changelog:` items,
/// newest-first, item text `"item-N"` where N counts down from the newest
/// (item-8 newest .. item-1 oldest) so pre/post rotation membership is easy
/// to assert unambiguously.
fn build_rotation_fixture(tmp: &Path) -> std::path::PathBuf {
    let items: Vec<String> = (1..=8)
        .rev()
        .map(|n| common::changelog_item_block(&format!("2026-08-{n:02}"), &format!("item-{n}")))
        .collect();
    let last_amended = common::clean_current_entry("2026-09-02", "v5.41", "current entry");
    let content = common::frontmatter_file(
        "behavioral-contract-index",
        "5.41",
        &last_amended,
        Some(&items),
        "# Fixture BC-INDEX\n",
    );
    common::write_file(
        tmp,
        ".factory/specs/behavioral-contracts/BC-INDEX.md",
        &content,
    )
}

/// PC5: rotating a fixture whose `changelog:` sequence (8 items) exceeds
/// `keep_recent` (3) moves the 5 oldest items out verbatim, leaves the 3
/// newest in the source, and the moved items land in the reported archive
/// file — verbatim, undamaged.
#[test]
fn test_BC_10_13_001_PC5_rotate_changelog_moves_oldest_items_verbatim() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = build_rotation_fixture(dir.path());

    let before = parse_frontmatter(&path).expect("parse_frontmatter on the fixture");
    assert_eq!(before.changelog_items_raw.len(), 8, "fixture sanity");

    let report = rotate_changelog(&path, "test-cycle", KEEP_RECENT, MigrationMode::Apply)
        .expect("rotate_changelog must succeed");

    assert!(report.mutated);
    assert_eq!(
        report.items_moved, 5,
        "8 items - keep_recent 3 = 5 items moved"
    );
    assert!(
        report
            .archive_path
            .to_string_lossy()
            .contains("cycles/test-cycle"),
        "archive must land under a cycles/<cycle_name> path segment: {:?}",
        report.archive_path
    );
    assert!(
        report
            .archive_path
            .to_string_lossy()
            .ends_with("BC-INDEX-changelog-archive.md"),
        "PC5's own literal naming convention '<file-basename>-changelog-archive.md': {:?}",
        report.archive_path
    );

    // Source: exactly the 3 NEWEST items remain, byte-for-byte, in order.
    let after = parse_frontmatter(&path).expect("parse_frontmatter after rotation");
    assert_eq!(after.changelog_items_raw.len(), KEEP_RECENT);
    for (i, expected_n) in [8, 7, 6].into_iter().enumerate() {
        assert!(
            after.changelog_items_raw[i].contains(&format!("item-{expected_n}")),
            "position {i} must hold item-{expected_n}: {:?}",
            after.changelog_items_raw[i]
        );
    }

    // Archive: the 5 OLDEST items, verbatim (no reserialize/mangling — a
    // plain substring check on the archive file's raw bytes is the correct
    // "verbatim" proof, since transplantation must not alter the text).
    let archive_content = common::read_file(&report.archive_path);
    for n in 1..=5 {
        assert!(
            archive_content.contains(&format!("item-{n}")),
            "archive must contain the moved item-{n} verbatim: {archive_content:?}"
        );
    }
    for n in 6..=8 {
        assert!(
            !archive_content.contains(&format!("item-{n}")),
            "archive must NOT contain a retained (non-moved) item-{n}"
        );
    }
}

/// EC-004: rotation invoked against a file BELOW the size threshold is a
/// no-op — `items_moved == 0`, `mutated == false`, and the source file's
/// `changelog:` sequence is untouched.
#[test]
fn test_BC_10_13_001_EC004_rotate_changelog_below_threshold_is_noop() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = build_rotation_fixture(dir.path());
    let before = common::read_file(&path);

    // keep_recent (10) exceeds the fixture's 8 items — below threshold.
    let report = rotate_changelog(&path, "test-cycle", 10, MigrationMode::Apply)
        .expect("rotate_changelog must succeed even when below threshold");

    assert_eq!(report.items_moved, 0, "EC-004: below threshold — no-op");
    assert!(!report.mutated);

    let after = common::read_file(&path);
    assert_eq!(before, after, "no mutation on a below-threshold rotation");
}

/// EC-005: rotation with a `<cycle-name>` that does not yet exist under
/// `.factory/cycles/` creates the destination directory before writing —
/// the archive file's parent directory must exist afterward even though it
/// did not exist before this call.
#[test]
fn test_BC_10_13_001_EC005_rotate_changelog_creates_missing_cycle_dir() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = build_rotation_fixture(dir.path());
    let cycles_dir = dir.path().join(".factory/cycles/brand-new-cycle");
    assert!(
        !cycles_dir.exists(),
        "fixture sanity: cycle dir must not pre-exist"
    );

    let report = rotate_changelog(&path, "brand-new-cycle", KEEP_RECENT, MigrationMode::Apply)
        .expect("rotate_changelog must create the missing cycle directory");

    assert!(
        report
            .archive_path
            .parent()
            .is_some_and(std::path::Path::exists),
        "the archive file's parent directory must exist after rotation: {:?}",
        report.archive_path
    );
    assert!(
        report.archive_path.exists(),
        "the archive file itself must exist: {:?}",
        report.archive_path
    );
}

/// Invariant 2: re-running rotation immediately after a successful rotation
/// — before the threshold is exceeded again — is a verified-clean no-op.
#[test]
fn test_BC_10_13_001_invariant2_rotate_changelog_reruns_are_noop() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = build_rotation_fixture(dir.path());

    let first = rotate_changelog(&path, "test-cycle", KEEP_RECENT, MigrationMode::Apply)
        .expect("first rotation");
    assert_eq!(first.items_moved, 5);
    let after_first_source = common::read_file(&path);
    let after_first_archive = common::read_file(&first.archive_path);

    let second = rotate_changelog(&path, "test-cycle", KEEP_RECENT, MigrationMode::Apply)
        .expect("second rotation");
    assert_eq!(
        second.items_moved, 0,
        "Invariant 2: immediate re-run before threshold is exceeded again is a no-op"
    );
    assert!(!second.mutated);

    assert_eq!(
        common::read_file(&path),
        after_first_source,
        "source file must be byte-identical after the no-op re-run"
    );
    assert_eq!(
        common::read_file(&second.archive_path),
        after_first_archive,
        "archive file must be byte-identical after the no-op re-run (no \
         duplicate re-append of already-archived items)"
    );
}

/// `MigrationMode::Check` for rotation: reports what WOULD move without
/// writing to either the source or the archive.
#[test]
fn test_BC_10_13_001_rotate_changelog_check_mode_never_writes() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = build_rotation_fixture(dir.path());
    let before = common::read_file(&path);

    let report = rotate_changelog(&path, "test-cycle", KEEP_RECENT, MigrationMode::Check)
        .expect("rotate_changelog Check mode must succeed");

    assert!(!report.mutated, "Check mode must never report a mutation");
    let after = common::read_file(&path);
    assert_eq!(before, after, "Check mode must never write to the source");
    assert!(
        !report.archive_path.exists(),
        "Check mode must never write the archive file either"
    );
}
