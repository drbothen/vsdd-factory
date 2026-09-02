// Test files use .expect()/.unwrap()/.panic!() for failure reporting.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
//! BC-10.13.001 PC3 (D-1144 unescaped-double-quote remediation on
//! `BC-INDEX.md`/`ARCH-INDEX.md`/`STATE.md`) + Invariant 4 / BC-5.45.001
//! Invariant 3 (every value the tool emits parses under strict YAML
//! `safe_load`) + PC4 idempotency of the escape step specifically.
//!
//! Backslash/quote sequences are constructed via `\u{5c}`/`\u{22}` escapes
//! rather than nested Rust string-escaping, to keep the exact byte sequence
//! under test unambiguous (this is precisely the class of off-by-one the
//! module under test — `src/escape.rs` — exists to get right, so the test's
//! own fixture construction must not be a second source of ambiguity).

mod common;

use last_amended_migrate::escape::{escape_value, needs_escaping};
use last_amended_migrate::migrate::MigrationMode;
use last_amended_migrate::migrate_file;

const BACKSLASH: char = '\u{5c}';
const QUOTE: char = '\u{22}';

// ── Direct `needs_escaping`/`escape_value` unit tests ───────────────────────

#[test]
fn test_BC_10_13_001_PC3_needs_escaping_true_on_unescaped_quote() {
    let raw = format!("fixed the {QUOTE}quoted term{QUOTE} defect");
    assert!(needs_escaping(&raw));
}

#[test]
fn test_BC_10_13_001_PC3_needs_escaping_false_on_plain_text() {
    let raw = "fixed the quoted term defect (no quotes at all)";
    assert!(!needs_escaping(raw));
}

/// Already-escaped `\"` (one backslash, then quote) must NOT be flagged —
/// this is the idempotency half of PC4 for the escape step specifically.
#[test]
fn test_BC_10_13_001_PC3_needs_escaping_false_on_already_escaped_quote() {
    let raw = format!("abc{BACKSLASH}{QUOTE}def");
    assert!(
        !needs_escaping(&raw),
        "an already-escaped \\\" must not be re-flagged: {raw:?}"
    );
}

/// The tricky case the module doc calls out explicitly: `\\"` (an ESCAPED
/// BACKSLASH — two backslashes — immediately followed by an UNESCAPED
/// quote) IS an unescaped quote, not a double-escape. A naive
/// look-behind-one-char check would get this wrong.
#[test]
fn test_BC_10_13_001_PC3_needs_escaping_true_after_escaped_backslash() {
    let raw = format!("abc{BACKSLASH}{BACKSLASH}{QUOTE}def");
    assert!(
        needs_escaping(&raw),
        "an escaped backslash (\\\\) followed by a bare quote must still \
         count as an unescaped quote: {raw:?}"
    );
}

#[test]
fn test_BC_10_13_001_PC3_escape_value_escapes_unescaped_quotes() {
    let raw = format!("fixed the {QUOTE}quoted term{QUOTE} defect");
    let escaped = escape_value(&raw);
    assert!(!needs_escaping(&escaped), "result must be fully escaped");
    assert_eq!(
        escaped,
        format!("fixed the {BACKSLASH}{QUOTE}quoted term{BACKSLASH}{QUOTE} defect")
    );
}

/// PC4 idempotency: escaping an already-escaped value must not double-
/// escape it (a naive `value.replace('"', "\\\"")` fails this).
#[test]
fn test_BC_10_13_001_PC4_escape_value_is_idempotent() {
    let raw = format!("fixed the {QUOTE}quoted term{QUOTE} defect");
    let once = escape_value(&raw);
    let twice = escape_value(&once);
    assert_eq!(
        once, twice,
        "escaping an already-escaped value must be a no-op (PC4)"
    );
}

/// The `\\"` case's escape output: only the bare trailing quote gains an
/// escape; the pre-existing escaped backslash is preserved verbatim.
#[test]
fn test_BC_10_13_001_PC3_escape_value_preserves_escaped_backslash() {
    let raw = format!("abc{BACKSLASH}{BACKSLASH}{QUOTE}def");
    let escaped = escape_value(&raw);
    assert!(!needs_escaping(&escaped));
    assert_eq!(
        escaped,
        format!("abc{BACKSLASH}{BACKSLASH}{BACKSLASH}{QUOTE}def"),
        "must preserve the pre-existing escaped backslash (2 chars) and add \
         exactly one new escaping backslash before the formerly-bare quote"
    );
}

// ── `migrate_file` end-to-end integration + strict YAML round trip ─────────

/// BC-10.13.001 PC3 canonical edge-case test vector: `BC-INDEX.md`-shaped
/// fixture with `changelog:` present and a D-1144-defective `last_amended`.
/// After migration: `changelog:` UNCHANGED; `last_amended` rewritten with
/// the `"` escaped to `\"`; report says 1 file mutated, 1 escape fix.
#[test]
fn test_BC_10_13_001_PC3_migrate_file_fixes_bc_index_quote_defect() {
    let dir = tempfile::tempdir().expect("tempdir");
    let last_amended = common::quote_defect_current_entry("2026-09-02", "v5.41");
    let content = common::frontmatter_file(
        "behavioral-contract-index",
        "5.41",
        &last_amended,
        Some(&[common::changelog_item_block("2026-08-01", "an older entry")]),
        "# Fixture BC-INDEX\n",
    );
    let path = common::write_file(dir.path(), "BC-INDEX.md", &content);

    // Fixture sanity: the PRE-fix content genuinely reproduces the D-1144
    // defect — it must NOT parse under strict YAML.
    assert!(
        common::strict_yaml_parse(&content).is_err(),
        "fixture sanity: an unescaped embedded quote must break strict YAML \
         parsing before the fix is applied"
    );

    let report = migrate_file(&path, MigrationMode::Apply).expect("migrate_file must succeed");

    assert!(report.mutated, "escaping a defective entry is a mutation");
    assert!(report.escape_fixed, "report must record the escape fix");

    let after = common::read_file(&path);
    assert_eq!(
        after.matches("changelog:").count(),
        1,
        "changelog: must remain UNCHANGED per the canonical edge-case vector"
    );
    assert!(
        after.contains("an older entry"),
        "pre-existing changelog item must survive untouched: {after:?}"
    );

    // Invariant 4 / BC-5.45.001 Invariant 3: the POST-fix content parses
    // cleanly under strict YAML.
    let parsed = common::strict_yaml_parse(&after)
        .expect("post-fix frontmatter must parse under strict YAML safe_load");
    assert!(
        parsed.last_amended.contains("quoted term"),
        "the entry's substantive text must be preserved through the fix: {:?}",
        parsed.last_amended
    );
    assert!(
        parsed.last_amended.contains('"'),
        "the YAML-decoded value must contain the literal quote character \
         (semantics preserved, only the raw escaping changed): {:?}",
        parsed.last_amended
    );
}

/// PC4: running the migration a second time on an already-escaped file is a
/// verified-clean no-op — byte-identical output.
#[test]
fn test_BC_10_13_001_PC4_migrate_file_escape_fix_is_idempotent_across_runs() {
    let dir = tempfile::tempdir().expect("tempdir");
    let last_amended = common::quote_defect_current_entry("2026-09-02", "v4.11");
    let content = common::frontmatter_file(
        "architecture-index",
        "4.11",
        &last_amended,
        Some(&[common::changelog_item_block("2026-08-01", "an older entry")]),
        "# Fixture ARCH-INDEX\n",
    );
    let path = common::write_file(dir.path(), "ARCH-INDEX.md", &content);

    let first = migrate_file(&path, MigrationMode::Apply).expect("first migrate_file call");
    assert!(first.mutated && first.escape_fixed);
    let after_first = common::read_file(&path);

    let second = migrate_file(&path, MigrationMode::Apply).expect("second migrate_file call");
    assert!(
        !second.mutated,
        "PC4: a second run against an already-migrated file must report zero mutations"
    );
    assert!(!second.escape_fixed);

    let after_second = common::read_file(&path);
    assert_eq!(
        after_first, after_second,
        "PC4: second run must produce byte-identical output"
    );
}

/// PC3 scope: `VP-INDEX.md`/`STORY-INDEX.md` are explicitly UNAFFECTED by
/// the D-1144 defect set — a clean fixture for either must never be
/// (falsely) flagged as needing an escape fix.
#[test]
fn test_BC_10_13_001_PC3_migrate_file_clean_entry_never_flags_escape_fixed() {
    let dir = tempfile::tempdir().expect("tempdir");
    let last_amended = common::clean_current_entry("2026-09-02", "v2.99", "no quotes here");
    let content = common::frontmatter_file(
        "verification-property-index",
        "2.99",
        &last_amended,
        Some(&[common::changelog_item_block("2026-08-01", "an older entry")]),
        "# Fixture VP-INDEX\n",
    );
    let path = common::write_file(dir.path(), "VP-INDEX.md", &content);

    let report = migrate_file(&path, MigrationMode::Apply).expect("migrate_file must succeed");

    assert!(!report.escape_fixed);
    assert!(!report.mutated, "fully compliant file is a verified no-op");
}
