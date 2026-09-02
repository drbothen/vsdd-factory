// Test-support files use .expect()/.unwrap()/.panic!() for failure reporting.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
// This module is `mod common;`-included by many separate `tests/bc_*.rs`
// integration-test binaries (Cargo compiles each `tests/*.rs` file as its
// own crate); no single one of them uses every helper here, so per-binary
// dead-code warnings are expected and not a real defect — CI runs
// `cargo clippy --workspace --all-targets -- -D warnings`, which would
// otherwise fail the build on this intentional, shared-fixture-module
// sharing pattern (mirrors how other workspace test-support modules handle
// the same "not every consumer uses every helper" shape).
#![allow(dead_code)]
//! Shared fixture-construction helpers for `last-amended-migrate`'s Red Gate
//! test suite (BC-10.13.001, BC-5.45.001 PC2, BC-4.18.001).
//!
//! `tests/common/mod.rs` is Cargo's special-cased "not a test binary" path
//! (mirrors `crates/policy15-attestation-gate`'s pattern of a private
//! `Repo` helper, but shared across multiple `tests/bc_*.rs` files here via
//! `mod common;` rather than duplicated, since none of these fixtures need
//! to differ per-file).
//!
//! # BC-5.38.001 / POLICY 11 discipline
//!
//! This module builds INPUT fixtures and reads OUTPUT files back from disk;
//! it never reimplements `last-amended-migrate`'s own parsing, eligibility,
//! escaping, or write logic. Every test that needs to inspect a mutated
//! file's structure calls the crate's own real `parse_frontmatter` (or
//! constructs a `FrontmatterDoc` directly via its public fields, which is
//! itself a POLICY-11-compliant use of the real public type) rather than a
//! hand-rolled parser here.

use std::fs;
use std::path::{Path, PathBuf};

use last_amended_migrate::migrate::TARGET_FILES;

/// Write `content` to `dir`/`rel_path`, creating parent directories as
/// needed. Returns the absolute path written.
pub fn write_file(dir: &Path, rel_path: &str, content: &str) -> PathBuf {
    let full = dir.join(rel_path);
    if let Some(parent) = full.parent() {
        fs::create_dir_all(parent).expect("create_dir_all fixture parent");
    }
    fs::write(&full, content).expect("write fixture file");
    full
}

pub fn read_file(path: &Path) -> String {
    fs::read_to_string(path).expect("read fixture file back")
}

/// One `changelog:` sequence item block, newest-first convention (matching
/// every real ADR-049-governed file's corpus shape — see
/// `.factory/specs/behavioral-contracts/BC-INDEX.md`'s own `changelog:`
/// sequence for the shape this mirrors).
pub fn changelog_item_block(date: &str, change_text: &str) -> String {
    format!("  - date: {date}\n    change: \"{change_text}\"\n")
}

/// Build one of the 5 ADR-049-governed files' frontmatter shape.
///
/// `last_amended_inner` is written VERBATIM between the outer double quotes
/// — callers control escaping precisely (including deliberately-broken
/// D-1144-defect fixtures), matching `FrontmatterDoc::last_amended_raw`'s own
/// documented "unescaped/unquoted" (i.e. literal-source-text-between-the-
/// quotes) semantics.
///
/// `changelog_items` is `None` for a file that has no `changelog:` key at
/// all (the `STORY-INDEX.md`-before-migration / `STATE.md` shape); `Some(&[])`
/// for a present-but-empty sequence; `Some(items)` for a populated one,
/// newest-first.
pub fn frontmatter_file(
    document_type: &str,
    version: &str,
    last_amended_inner: &str,
    changelog_items: Option<&[String]>,
    body: &str,
) -> String {
    let mut s = String::new();
    s.push_str("---\n");
    s.push_str(&format!("document_type: {document_type}\n"));
    s.push_str(&format!("version: \"{version}\"\n"));
    s.push_str(&format!("last_amended: \"{last_amended_inner}\"\n"));
    if let Some(items) = changelog_items {
        s.push_str("changelog:\n");
        for item in items {
            s.push_str(item);
        }
    }
    s.push_str("---\n\n");
    s.push_str(body);
    s
}

/// A `last_amended` current-entry-only value with NO D-1144 escape defect —
/// the fully-compliant post-ADR-049 shape (BC-10.13.001 EC-001 verified-
/// clean-no-op case).
pub fn clean_current_entry(date: &str, version: &str, summary: &str) -> String {
    format!("{date} ({version}) — {summary}")
}

/// A `last_amended` current-entry-only value carrying the D-1144 unescaped-
/// literal-double-quote defect (BC-10.13.001 PC3's 3-file target class:
/// `BC-INDEX.md`/`ARCH-INDEX.md`/`STATE.md`). Contains 2 unescaped `"`.
pub fn quote_defect_current_entry(date: &str, version: &str) -> String {
    format!("{date} ({version}) — fixed the \"quoted term\" defect")
}

/// A `last_amended` value still carrying a nested `[Prior: <date> (vX.Y) —
/// ...]` bracket chain — the pre-D-1149 / non-conforming shape BC-10.13.001
/// Precondition 2 / EC-003 places OUT OF SCOPE (NOT eligible; the tool must
/// NOT attempt to split it — see this module's doc + the eligibility test
/// file's header for the full BC-vs-instruction reconciliation note).
pub fn prior_chain_last_amended(date: &str, version: &str) -> String {
    format!(
        "{date} ({version}) — current entry text [Prior: 2026-08-01 (v0.9) — older entry text [Prior: 2026-07-01 (v0.8) — oldest entry text]]"
    )
}

/// Synthetic MEGA-LINE `last_amended` value reproducing the D-1149
/// 323,499-char `STORY-INDEX.md` calibration ceiling: a single `[Prior:
/// ...]` bracket whose embedded text is padded well past that ceiling
/// (350,000 `x` filler characters — no quotes/colons, so the fixture's
/// *shape* is unambiguous regardless of how the eventual implementation
/// tokenizes it). Proves Invariant 3 (bounded-resource safety on
/// arbitrarily long input) — this is "the whole reason the tool exists":
/// Edit/Write-tool-mediated manual editing cannot safely handle content at
/// this scale (BC-10.13.001 Invariant 3 doc note).
pub fn mega_line_prior_chain(filler_len: usize) -> String {
    let filler = "x".repeat(filler_len);
    format!("2026-09-02 (v1.0) — current entry text [Prior: {filler} (v0.9) — old]")
}

/// The 5 D-1149 sidecar basenames this tool must register (BC-10.13.001
/// PC6 / §Architecture Anchors), derived from the crate's own real
/// `TARGET_FILES` constant rather than hardcoded — stays in lockstep with
/// the crate's own source of truth if `TARGET_FILES` ever changes.
pub fn expected_sidecar_basenames() -> Vec<String> {
    TARGET_FILES
        .iter()
        .map(|rel| {
            let stem = Path::new(rel)
                .file_stem()
                .and_then(|s| s.to_str())
                .expect("TARGET_FILES entry has a file stem");
            format!("{stem}-amendment-history.md")
        })
        .collect()
}

/// Minimal strict-YAML frontmatter shape for the `serde_norway` round-trip
/// check (BC-10.13.001 PC3 / Invariant 4; BC-5.45.001 Invariant 3). Extra
/// frontmatter fields are ignored by default serde struct deserialization
/// (no `deny_unknown_fields`), so this only needs the 2 fields this tool
/// operates on.
#[derive(serde::Deserialize)]
pub struct MinimalFrontmatter {
    pub last_amended: String,
    #[serde(default)]
    pub changelog: Option<Vec<serde_norway::Value>>,
}

/// Extract just the `---`-fenced frontmatter block (without the fences) so
/// `serde_norway::from_str` parses only the YAML region, not the markdown
/// body below it.
pub fn frontmatter_block(file_content: &str) -> &str {
    let after_open = file_content
        .strip_prefix("---\n")
        .expect("fixture must start with a frontmatter fence");
    let end = after_open
        .find("\n---")
        .expect("fixture must have a closing frontmatter fence");
    &after_open[..end]
}

/// Parse a file's frontmatter block under strict YAML `safe_load` semantics
/// (`serde_norway::from_str`), independent of this crate's own hand-rolled
/// production parser — a genuine black-box verification technique, not a
/// reimplementation of `parse_frontmatter`'s logic (POLICY 11).
pub fn strict_yaml_parse(file_content: &str) -> Result<MinimalFrontmatter, serde_norway::Error> {
    serde_norway::from_str(frontmatter_block(file_content))
}
