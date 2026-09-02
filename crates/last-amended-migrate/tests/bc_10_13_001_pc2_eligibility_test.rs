// Test files use .expect()/.unwrap()/.panic!() for failure reporting.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
//! BC-10.13.001 PC2 (current-entry-only confirmation) + Precondition 2 /
//! EC-003 (out-of-scope `[Prior: ...]` chain classification) + Invariant 3
//! (bounded-resource safety on arbitrarily long input).
//!
//! # Reconciliation note — instruction vs. BC/stub design
//!
//! The dispatching prompt for this Red Gate pass asked for a test proving
//! that, given a fixture whose `last_amended` is an inline `[Prior: ...]`
//! chain, `migrate_file` SPLITS it (current entry retained in
//! `last_amended`, displaced tail moved to `changelog:`/a sidecar). That is
//! NOT what BC-10.13.001 or the stub architecture specify:
//!
//! - BC-10.13.001 **Precondition 2**: "the target file's current
//!   `last_amended` value is already in a D-1149-slim current-entry form
//!   ... the tool is a shape-completion and escape-remediation utility, NOT
//!   a general-purpose mega-line splitter (that surgery was D-1149's
//!   one-time human-authorized exception and is out of this tool's scope)."
//! - BC-10.13.001 **EC-003**: a file whose `last_amended` still contains a
//!   nested `[Prior: ...]` chain is "**Out of scope** for this tool per
//!   Precondition 2 — the tool reports the file as NOT eligible for
//!   migration ... and does not attempt the large-scale bracket-splitting
//!   surgery."
//! - `src/eligibility.rs`'s own doc comment: "MUST NOT perform or begin any
//!   bracket-splitting surgery — this function only classifies."
//! - `src/migrate.rs`'s own doc comment: "Returns `Err(MigrateError::NotEligible)`
//!   per EC-003 rather than attempting any bracket-splitting surgery."
//! - Both BC-10.13.001 canonical test vectors show `last_amended`
//!   unaffected/rewritten-in-place only — never split into two locations.
//!
//! Per this agent's own operating discipline ("Never blindly implement...
//! The BC defines what's correct — not the reviewer's [or dispatching
//! prompt's] intuition"), this file tests the BC-CORRECT behavior
//! (NOT-ELIGIBLE classification, no mutation attempted, bounded-time/-memory
//! even on a synthetic mega-line) rather than a splitting behavior the BC
//! explicitly forbids this tool from performing. See this story's final
//! report for the routing note back to product-owner/architect flagging the
//! discrepancy for confirmation.

mod common;

use std::time::Instant;

use last_amended_migrate::eligibility::{Eligibility, check_eligibility};
use last_amended_migrate::migrate::MigrationMode;
use last_amended_migrate::{MigrateError, migrate_file};

/// Sanity/positive-classification control: a genuinely current-entry-only
/// value (no `[Prior: ` marker at all) classifies as `CurrentEntryOnly`.
#[test]
fn test_BC_10_13_001_PC2_current_entry_only_no_marker_classifies_eligible() {
    let raw = common::clean_current_entry("2026-09-02", "v1.0", "some entry text");
    assert_eq!(check_eligibility(&raw), Eligibility::CurrentEntryOnly);
}

/// BC-10.13.001 Precondition 2: a trailing `[Prior history →
/// <file>-amendment-history.md]` POINTER NOTE (distinct text — "history →",
/// not "Prior: ") must NOT be misclassified as a `[Prior: ...]` chain
/// marker. This is the exact distinction Precondition 2 draws ("a single
/// dated entry, optionally with a trailing `[Prior history → ...]` pointer
/// note").
#[test]
fn test_BC_10_13_001_PC2_trailing_pointer_note_is_not_a_chain_marker() {
    let raw = "2026-09-02 (v1.0) — some entry text [Prior history → BC-INDEX-amendment-history.md]"
        .to_string();
    assert_eq!(
        check_eligibility(&raw),
        Eligibility::CurrentEntryOnly,
        "a `[Prior history \u{2192} ...]` pointer note is NOT a `[Prior: ...]` \
         chain marker and must classify as eligible"
    );
}

/// BC-10.13.001 EC-003 / Precondition 2: a genuine `[Prior: <date> (vX.Y) —
/// ...]` chain classifies as `NotEligiblePriorChain`.
#[test]
fn test_BC_10_13_001_PC2_prior_chain_classifies_not_eligible() {
    let raw = common::prior_chain_last_amended("2026-09-02", "v1.0");
    assert_eq!(check_eligibility(&raw), Eligibility::NotEligiblePriorChain);
}

/// BC-10.13.001 Invariant 3 — bounded-resource safety on arbitrarily long
/// input, calibrated to (and exceeding) the D-1149 323,499-char ceiling.
/// `check_eligibility` must both (a) correctly classify the mega-line as a
/// chain, and (b) do so in bounded time — a quadratic-backtracking scan
/// would make this test hang rather than fail, which is exactly the defect
/// class this invariant exists to rule out.
#[test]
fn test_BC_10_13_001_invariant3_mega_line_eligibility_scan_is_bounded() {
    let raw = common::mega_line_prior_chain(350_000);
    assert!(
        raw.len() > 323_499,
        "fixture sanity: must exceed the D-1149 calibration ceiling"
    );

    let start = Instant::now();
    let classification = check_eligibility(&raw);
    let elapsed = start.elapsed();

    assert_eq!(classification, Eligibility::NotEligiblePriorChain);
    assert!(
        elapsed.as_secs() < 2,
        "check_eligibility on a {}-byte mega-line took {:?} — Invariant 3 \
         requires a bounded (non-quadratic-backtracking) scan; a hang or \
         multi-second runtime here is the exact D-1149 defect class this \
         tool exists to eliminate",
        raw.len(),
        elapsed
    );
}

/// BC-10.13.001 EC-003 at the `migrate_file` orchestration level: a fixture
/// FILE whose `last_amended` still contains a `[Prior: ...]` chain must be
/// reported `Err(MigrateError::NotEligible)`, and the file on disk must be
/// COMPLETELY UNCHANGED (the tool never attempts the out-of-scope
/// bracket-splitting surgery — no partial mutation either).
#[test]
fn test_BC_10_13_001_EC003_migrate_file_reports_not_eligible_without_mutation() {
    let dir = tempfile::tempdir().expect("tempdir");
    let last_amended = common::prior_chain_last_amended("2026-09-02", "v1.0");
    let content = common::frontmatter_file(
        "behavioral-contract-index",
        "5.41",
        &last_amended,
        Some(&[common::changelog_item_block("2026-08-01", "an older entry")]),
        "# Fixture BC-INDEX\n",
    );
    let path = common::write_file(dir.path(), "BC-INDEX.md", &content);
    let before = common::read_file(&path);

    let result = migrate_file(&path, MigrationMode::Apply);

    match result {
        Err(MigrateError::NotEligible { path: err_path }) => {
            assert_eq!(err_path, path, "NotEligible must name the offending file");
        }
        other => {
            panic!("expected Err(MigrateError::NotEligible {{ .. }}) per EC-003, got: {other:?}")
        }
    }

    let after = common::read_file(&path);
    assert_eq!(
        before, after,
        "EC-003: the tool MUST NOT attempt any bracket-splitting mutation \
         on a NOT-ELIGIBLE file — content must be byte-for-byte unchanged"
    );
}

/// BC-10.13.001 EC-003 + Invariant 3 combined, end-to-end through the real
/// file-I/O path: a mega-line FIXTURE FILE (the actual D-1149 failure mode —
/// "Edit/Write-tool-mediated manual editing cannot safely handle content at
/// this scale") must still be classified NOT-ELIGIBLE, unmutated, and within
/// bounded time when driven through `migrate_file`, not just the in-memory
/// `check_eligibility` unit path.
#[test]
fn test_BC_10_13_001_invariant3_mega_line_file_migrate_file_bounded_and_unmutated() {
    let dir = tempfile::tempdir().expect("tempdir");
    let last_amended = common::mega_line_prior_chain(350_000);
    let content = common::frontmatter_file(
        "story-index",
        "4.430",
        &last_amended,
        None,
        "# Fixture STORY-INDEX\n",
    );
    let path = common::write_file(dir.path(), "STORY-INDEX.md", &content);
    let before = common::read_file(&path);
    assert!(before.len() > 323_499, "fixture sanity");

    let start = Instant::now();
    let result = migrate_file(&path, MigrationMode::Apply);
    let elapsed = start.elapsed();

    assert!(
        matches!(result, Err(MigrateError::NotEligible { .. })),
        "expected Err(NotEligible) on a mega-line prior-chain fixture, got: {result:?}"
    );
    assert!(
        elapsed.as_secs() < 5,
        "migrate_file on a mega-line fixture took {elapsed:?} — must stay bounded (Invariant 3)"
    );

    let after = common::read_file(&path);
    assert_eq!(
        before, after,
        "no mutation attempted on a NOT-ELIGIBLE file"
    );
}
