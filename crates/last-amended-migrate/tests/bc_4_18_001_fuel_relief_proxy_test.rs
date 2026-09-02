// Test files use .expect()/.unwrap()/.panic!() for failure reporting.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
//! BC-4.18.001 PC1 (`last_amended` byte length is bounded regardless of
//! cumulative burst count) — the testable crate-level PROXY for the
//! fuel-budget-relief regression.
//!
//! # Scope note — what this file does and does NOT cover
//!
//! BC-4.18.001's PC2/PC3 (a synthetic-burst regression run completing
//! within the `legacy-bash-adapter`-hosted WASM validators' fuel budget,
//! and the differential pre-fix/post-fix fuel-exhaustion comparison) require
//! invoking the REAL `legacy-bash-adapter` dispatcher/WASM-plugin
//! infrastructure (`crates/hook-plugins/legacy-bash-adapter/`,
//! `crates/factory-dispatcher/src/invoke.rs`'s `DEFAULT_FUEL_CAP`/fuel
//! accounting) against `plugins/vsdd-factory/hooks-registry.toml`-registered
//! validators. `last-amended-migrate` has NO dependency on that
//! infrastructure (its own `Cargo.toml` doc comment: "NOT a WASM hook
//! plugin ... has no dependency on `vsdd-hook-sdk`"), and BC-4.18.001's own
//! VP candidate row for PC2 explicitly names that surface: "integration:
//! bats/Rust-workspace test invoking the real `legacy-bash-adapter`-hosted
//! validator(s)". That test belongs in
//! `crates/hook-plugins/legacy-bash-adapter/` or a bats suite under
//! `plugins/vsdd-factory/tests/`, dispatched by fuel-accounting
//! infrastructure this crate cannot and should not reach into — writing it
//! here would mean reimplementing (or worse, mocking) the fuel-accounting
//! logic the real regression test must exercise for real, which is exactly
//! the "vacuously true" Red Gate anti-pattern this agent's operating rules
//! forbid. See this story's final report for the explicit routing note.
//!
//! What IS in this crate's scope, and what this file covers: PC1's own
//! structural claim — "the `last_amended` field's own byte length never
//! exceeds a fixed per-entry ceiling — it is always exactly one dated
//! entry, independent of N" — verified against this tool's REAL
//! `migrate_all`/`migrate_file` output on realistic ADR-049-shaped
//! fixtures, including the 3 escape-defect files PC3 rewrites in place.

mod common;

use last_amended_migrate::frontmatter::parse_frontmatter;
use last_amended_migrate::migrate::{MigrationMode, TARGET_FILES, migrate_all};

/// A generous ceiling for a single dated `last_amended` entry — real corpus
/// entries in `.factory/` run from a few dozen to a few hundred bytes; 2,000
/// bytes is comfortably above any legitimate single entry and orders of
/// magnitude below the pre-fix 323,499-char mega-line this BC exists to
/// eliminate.
const LAST_AMENDED_BYTE_CEILING: usize = 2_000;

/// Build the full 5-file `.factory`-rooted fixture tree matching
/// `TARGET_FILES`'s real relative paths, mixing all 3 PC1/PC3 shapes this
/// story's fixtures need to exercise:
///
/// - `STORY-INDEX.md` — clean entry, NO `changelog:` yet (PC1/EC-002).
/// - `BC-INDEX.md` / `ARCH-INDEX.md` / `STATE.md` — D-1144 escape-defective
///   entry (PC3's 3-file target set).
/// - `VP-INDEX.md` — already fully compliant (EC-001 no-op control).
fn build_five_target_fixtures(tmp: &std::path::Path) -> std::path::PathBuf {
    let factory_root = tmp.join(".factory");

    let story_index_last_amended =
        common::clean_current_entry("2026-09-02", "v4.430", "clean bootstrap entry");
    common::write_file(
        &factory_root,
        "stories/STORY-INDEX.md",
        &common::frontmatter_file(
            "story-index",
            "4.430",
            &story_index_last_amended,
            None,
            "# Fixture STORY-INDEX\n",
        ),
    );

    for (rel, doc_type, version) in [
        (
            "specs/behavioral-contracts/BC-INDEX.md",
            "behavioral-contract-index",
            "5.41",
        ),
        (
            "specs/architecture/ARCH-INDEX.md",
            "architecture-index",
            "4.11",
        ),
    ] {
        let last_amended = common::quote_defect_current_entry("2026-09-02", version);
        common::write_file(
            &factory_root,
            rel,
            &common::frontmatter_file(
                doc_type,
                version,
                &last_amended,
                Some(&[common::changelog_item_block("2026-08-01", "an older entry")]),
                "# Fixture\n",
            ),
        );
    }

    let vp_index_last_amended =
        common::clean_current_entry("2026-09-02", "v2.99", "already compliant");
    common::write_file(
        &factory_root,
        "specs/verification-properties/VP-INDEX.md",
        &common::frontmatter_file(
            "verification-property-index",
            "2.99",
            &vp_index_last_amended,
            Some(&[common::changelog_item_block("2026-08-01", "an older entry")]),
            "# Fixture VP-INDEX\n",
        ),
    );

    let state_last_amended = common::quote_defect_current_entry("2026-09-02", "v9.65");
    common::write_file(
        &factory_root,
        "STATE.md",
        &common::frontmatter_file(
            "pipeline-state",
            "9.65",
            &state_last_amended,
            None,
            "# Fixture STATE\n",
        ),
    );

    factory_root
}

/// BC-4.18.001 PC1: after `migrate_all` runs across all 5 fixtures
/// (bootstrapping `changelog:` on one, escape-fixing 3, no-op on the 5th),
/// every file's `last_amended` byte length stays under the fixed ceiling —
/// the structural proxy for "cannot re-form a mega-line".
#[test]
fn test_BC_4_18_001_PC1_last_amended_byte_length_bounded_across_all_five_files() {
    let dir = tempfile::tempdir().expect("tempdir");
    let factory_root = build_five_target_fixtures(dir.path());

    let report =
        migrate_all(&factory_root, MigrationMode::Apply).expect("migrate_all must succeed");
    assert_eq!(report.files.len(), TARGET_FILES.len());

    for rel in TARGET_FILES {
        let path = factory_root.join(rel);
        let doc = parse_frontmatter(&path)
            .unwrap_or_else(|e| panic!("parse_frontmatter on {path:?} failed: {e}"));
        let last_amended = doc
            .last_amended_raw
            .unwrap_or_else(|| panic!("{path:?} must have a last_amended value after migration"));
        assert!(
            last_amended.len() < LAST_AMENDED_BYTE_CEILING,
            "BC-4.18.001 PC1: {path:?}'s last_amended is {} bytes — must stay \
             far below the {LAST_AMENDED_BYTE_CEILING}-byte ceiling (proxy \
             for 'cannot re-form the pre-fix 323,499-char mega-line')",
            last_amended.len()
        );
    }
}

/// BC-4.18.001 EC-003: a synthetic burst whose entry includes an embedded
/// double-quote (D-1144 class) still stays within the same bound — the
/// fuel-relief property is orthogonal to the YAML-escape correctness
/// property (already covered by BC-10.13.001 PC3's own test file); this
/// test only confirms escaping doesn't materially change payload size.
#[test]
fn test_BC_4_18_001_EC003_quote_defect_entry_still_bounded_after_escape_fix() {
    let dir = tempfile::tempdir().expect("tempdir");
    let factory_root = build_five_target_fixtures(dir.path());
    migrate_all(&factory_root, MigrationMode::Apply).expect("migrate_all must succeed");

    let bc_index_path = factory_root.join("specs/behavioral-contracts/BC-INDEX.md");
    let doc = parse_frontmatter(&bc_index_path).expect("parse post-migration BC-INDEX.md");
    let last_amended = doc
        .last_amended_raw
        .expect("BC-INDEX.md must have last_amended after migration");

    assert!(
        last_amended.len() < LAST_AMENDED_BYTE_CEILING,
        "escaping 2 quotes must not materially change payload size: {} bytes",
        last_amended.len()
    );
    assert!(
        last_amended.contains("quoted term"),
        "escape fix must preserve the entry's substantive text: {last_amended:?}"
    );
}

/// Precondition 1 / EC-006 at the `migrate_all` aggregation level: exactly
/// 4 of the 5 files may ever carry `changelog:` — `STATE.md` never does,
/// even after a full migration run across all 5.
#[test]
fn test_BC_10_13_001_EC006_migrate_all_state_md_never_gains_changelog() {
    let dir = tempfile::tempdir().expect("tempdir");
    let factory_root = build_five_target_fixtures(dir.path());
    migrate_all(&factory_root, MigrationMode::Apply).expect("migrate_all must succeed");

    let state_content = common::read_file(&factory_root.join("STATE.md"));
    assert!(
        !state_content.contains("changelog:"),
        "STATE.md must NEVER gain a changelog: field, even via migrate_all \
         (ADR-049 Decision 4 / EC-006): {state_content:?}"
    );
}
