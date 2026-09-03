// Test files use .expect()/.unwrap()/.panic!() for failure reporting.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
//! S-15.03 pr-reviewer B2 — PC7 full-recovery split on `STATE.md`.
//!
//! # Why this file exists
//!
//! No prior test in this crate exercised `migrate_file`'s PC7
//! (`Eligibility::PriorChainSplit`) path against a `STATE.md`-shaped
//! fixture carrying an inline `[Prior: ...]` chain. `tests/
//! bc_10_13_001_pc1_changelog_bootstrap_test.rs`'s EC-006 coverage only
//! exercises the `CurrentEntryOnly` path via `ensure_changelog_field`
//! directly — it never constructs a chained `last_amended` and never calls
//! `migrate_file`. This file closes that gap.
//!
//! # Is dropping the recovered entries a bug?
//!
//! A pr-reviewer flagged `migrate_file`'s STATE.md branch (`src/migrate.rs`,
//! the `if !is_state { ... }` guard around `prepend_changelog_item`) as
//! "silent data loss": `entries_recovered` is reported nonzero while the
//! actual chained-entry text is never written anywhere.
//!
//! This is investigated and found to be **spec-conformant, not a defect**.
//! BC-10.13.001 v1.1's own EC-006 row is unambiguous and was human-ratified
//! specifically for this case:
//!
//! > "Migration invoked against `STATE.md`" -> "`changelog:` field is never
//! > added (PC1); only `last_amended` current-entry-only confirmation or
//! > full-recovery split (PC2/PC7) and, if applicable, D-1144 escape
//! > remediation (PC3) apply. `STATE.md` has no `changelog:` field, so a
//! > chain found on `STATE.md` is split with the recovered entries
//! > SUPERSEDED BY ITS BODY-LEVEL DECISIONS LOG rather than relocated to a
//! > frontmatter sequence, per ADR-049 Decision 4 / BC-5.45.001 PC3."
//!
//! The `state-burst` skill's own `last_amended` Write-Path Discipline
//! (`plugins/vsdd-factory/skills/state-burst/SKILL.md` §3) independently
//! documents the same design: every burst that touches one of the 5
//! D-1149 files is required to write STATE.md's substantive history into
//! its body-level `## Decisions Log`/`## Phase Progress` sections AS PART
//! OF THE SAME BURST, separately from `last_amended` — so by the time a
//! chain ever accumulates on STATE.md's `last_amended` (a discipline
//! violation `migrate --path` exists to recover from, not a source of
//! otherwise-unrecorded information), the substantive content of every
//! chained entry already lives in the body. `entries_recovered` on
//! STATE.md therefore means "N redundant chained summaries were stripped
//! from the frontmatter scalar," not "N entries were preserved into a new
//! location" — a materially different, but still accurate and
//! non-misleading, semantics from the other 4 files' `entries_recovered`.
//!
//! This test suite pins that exact spec-mandated behavior end-to-end via
//! `migrate_file`, so any future change to this behavior shows up as a
//! test failure requiring a deliberate, reviewed decision — not a silent
//! regression.

mod common;

use last_amended_migrate::eligibility::Eligibility;
use last_amended_migrate::migrate::MigrationMode;
use last_amended_migrate::migrate_file;

/// BC-10.13.001 EC-006 + PC7: a `STATE.md`-shaped fixture whose
/// `last_amended` carries a 2-entry inline `[Prior: ...]` chain is SPLIT
/// (not refused, not errored) — `last_amended` becomes current-entry-only,
/// `entries_recovered` accurately reports 2, `changelog:` is NEVER added,
/// and the file remains valid, parseable, and structurally sound
/// (its `# STATE` body content is untouched).
#[test]
fn test_BC_10_13_001_EC006_migrate_file_splits_state_md_chain_without_adding_changelog() {
    let dir = tempfile::tempdir().expect("tempdir");
    let last_amended = common::chain_last_amended(
        ("2026-09-02", "v9.65", "current burst summary text"),
        &[
            ("2026-09-01", "v9.64", "previous burst summary text"),
            ("2026-08-31", "v9.63", "earlier burst summary text"),
        ],
    );
    let content = common::frontmatter_file(
        "pipeline-state",
        "9.65",
        &last_amended,
        None,
        "# Pipeline State: vsdd-factory\n\n## Decisions Log\n\n\
         (the substantive history for every chained entry already lives \
         here, per the state-burst skill's Write-Path Discipline)\n",
    );
    let path = common::write_file(dir.path(), "STATE.md", &content);
    assert!(!content.contains("changelog:"), "fixture sanity");

    let report = migrate_file(&path, MigrationMode::Apply)
        .expect("migrate_file must SPLIT a STATE.md chain, not error");

    assert!(report.mutated, "a PC7 split is a mutation");
    assert_eq!(report.eligibility, Eligibility::PriorChainSplit);
    assert_eq!(
        report.entries_recovered, 2,
        "both chained entries must be counted as recovered — this reports \
         how many redundant chained summaries were stripped from \
         last_amended, per EC-006 (the substantive content is understood \
         to already live in STATE.md's own body, not relocated by this \
         tool)"
    );

    let after = common::read_file(&path);
    assert!(
        !after.contains(" [Prior:"),
        "last_amended must be current-entry-only after the split — no \
         inline chain marker may remain: {after:?}"
    );
    assert!(
        after.contains("current burst summary text"),
        "the current entry's text must be preserved verbatim: {after:?}"
    );
    assert!(
        !after.contains("changelog:"),
        "STATE.md must NEVER gain a changelog: field, even via a PC7 \
         split (ADR-049 Decision 4 / EC-006): {after:?}"
    );
    assert!(
        after.contains("## Decisions Log"),
        "STATE.md's body content must be left untouched by the split: {after:?}"
    );

    // BC-4.18.001 PC1 (fuel relief): the split's whole purpose is a bounded
    // last_amended — confirm it actually holds for STATE.md too, not just
    // the 4 changelog:-bearing files.
    let doc = last_amended_migrate::frontmatter::parse_frontmatter(&path)
        .expect("parse post-split STATE.md");
    let new_last_amended = doc
        .last_amended_raw
        .expect("STATE.md must have last_amended after the split");
    assert!(
        new_last_amended.len() < 2_000,
        "post-split last_amended must be short — {} bytes is not bounded \
         relief: {new_last_amended:?}",
        new_last_amended.len()
    );

    // Invariant 4 / PC3: the resulting file still parses cleanly under
    // strict YAML `safe_load`.
    let parsed = common::strict_yaml_parse(&after)
        .expect("post-split STATE.md frontmatter must parse under strict YAML safe_load");
    assert!(
        parsed.changelog.is_none(),
        "strict-YAML view must also confirm no changelog: key exists on STATE.md"
    );
}

/// BC-10.13.001 EC-006 / Invariant 3 at STATE.md's own D-1149 calibration
/// scale: a mega-line STATE.md chain still splits successfully (not merely
/// refuses), staying consistent with the EC-009 mega-line vector already
/// proven for the other 4 files.
#[test]
fn test_BC_10_13_001_EC006_migrate_file_splits_mega_line_state_md_chain() {
    let dir = tempfile::tempdir().expect("tempdir");
    let last_amended = common::mega_line_prior_chain(350_000);
    let content = common::frontmatter_file(
        "pipeline-state",
        "9.65",
        &last_amended,
        None,
        "# Pipeline State: vsdd-factory\n\n## Decisions Log\n",
    );
    let path = common::write_file(dir.path(), "STATE.md", &content);
    let before = common::read_file(&path);
    assert!(before.len() > 323_499, "fixture sanity");

    let report = migrate_file(&path, MigrationMode::Apply)
        .expect("mega-line STATE.md chain must SPLIT successfully, not error");

    assert!(report.mutated);
    assert_eq!(report.eligibility, Eligibility::PriorChainSplit);
    assert_eq!(report.entries_recovered, 1);

    let after = common::read_file(&path);
    assert_ne!(before, after, "the mega-line STATE.md file must be mutated");
    assert!(!after.contains(" [Prior:"));
    assert!(
        !after.contains("changelog:"),
        "STATE.md must never gain changelog: even at mega-line scale: {after:?}"
    );
}
