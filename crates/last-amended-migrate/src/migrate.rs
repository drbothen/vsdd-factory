//! Migration subcommand orchestration (BC-10.13.001 PC1-PC4, PC6).

use crate::changelog::ChangelogMutation;
use crate::eligibility::Eligibility;
use crate::error::MigrateError;
use std::path::{Path, PathBuf};

/// The 5 ADR-049-governed files this tool's migration subcommand targets,
/// relative to a supplied `.factory/` root (BC-10.13.001 Precondition 1,
/// §Description; D-1149).
pub const TARGET_FILES: [&str; 5] = [
    "stories/STORY-INDEX.md",
    "specs/behavioral-contracts/BC-INDEX.md",
    "specs/architecture/ARCH-INDEX.md",
    "specs/verification-properties/VP-INDEX.md",
    "STATE.md",
];

/// Whether a migration/rotation invocation reports violations only
/// (`Check`, mirroring `compute-input-hash`'s `--check`) or performs the
/// write (`Apply`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MigrationMode {
    /// Report only; `migrate_file`/`migrate_all`/`rotate_changelog` MUST NOT
    /// write to any target file in this mode.
    Check,
    /// Perform the write.
    Apply,
}

/// Per-file migration outcome.
#[derive(Debug, Clone)]
pub struct FileMigrationReport {
    pub path: PathBuf,
    pub eligibility: Eligibility,
    pub changelog_mutation: ChangelogMutation,
    pub escape_fixed: bool,
    /// Count of historical entries PC7's full-recovery split relocated from
    /// an inline `[Prior: ...]` chain into `changelog:`, newest-first.
    /// `0` when `eligibility` is `Eligibility::CurrentEntryOnly` (no split
    /// occurred — including the PC4/PC7-step-8 no-op re-run case after a
    /// prior split has already resolved the chain).
    pub entries_recovered: usize,
    /// `true` iff any of the above resulted in an actual file write —
    /// always `false` in `MigrationMode::Check`, by definition.
    pub mutated: bool,
}

/// Aggregate report across every file a migration invocation touched.
#[derive(Debug, Clone, Default)]
pub struct MigrationReport {
    pub files: Vec<FileMigrationReport>,
}

impl MigrationReport {
    /// Count of files this report recorded an actual mutation for.
    ///
    /// # BC-5.38.002 GREEN-BY-DESIGN
    ///
    /// Pure fold over already-computed `FileMigrationReport::mutated` flags:
    /// zero branching beyond the closure's boolean field read (no
    /// if/match/?/unwrap), no I/O, no non-trivial helper calls
    /// (`Iterator::filter`/`count` are primitive standard-library
    /// operations, not domain logic), body is 3 lines. All four BC-5.38.002
    /// criteria hold — see the stub commit report's GREEN-BY-DESIGN table.
    pub fn total_mutated(&self) -> usize {
        self.files.iter().filter(|f| f.mutated).count()
    }
}

/// `path`'s final component is literally `STATE.md` (ADR-049 Decision 4 /
/// EC-006 — the file `ensure_changelog_field` must always skip).
fn is_state_file(path: &Path) -> bool {
    path.file_name().and_then(|n| n.to_str()) == Some("STATE.md")
}

/// PC7 step 3: split `tail` (which starts with the marker itself) into its
/// individual historical-entry raw texts, newest-of-the-priors-first,
/// prefix/suffix-stripped of the ` [Prior:`/`]` bracket syntax. A single
/// forward-scanning cursor loop — each `str::find` call searches only the
/// REMAINING slice from `cursor` onward (never re-scanning from index 0), so
/// the total work across every iteration is bounded by `O(n)` in `tail`'s
/// length, satisfying Invariant 3 even for a chain of many entries.
fn split_tail_entries(tail: &str) -> Vec<String> {
    use crate::eligibility::CHAIN_MARKER;

    let mut marker_starts = Vec::new();
    let mut cursor = 0usize;
    while let Some(rel) = tail[cursor..].find(CHAIN_MARKER) {
        marker_starts.push(cursor + rel);
        cursor += rel + CHAIN_MARKER.len();
    }

    let last_index = marker_starts.len().saturating_sub(1);
    let mut entries = Vec::with_capacity(marker_starts.len());
    for (i, &start) in marker_starts.iter().enumerate() {
        let end = marker_starts.get(i + 1).copied().unwrap_or(tail.len());
        let raw_entry = &tail[start..end];
        let stripped = raw_entry
            .strip_prefix(CHAIN_MARKER)
            .unwrap_or(raw_entry)
            .trim_start_matches(' ');
        // Only the LAST (innermost/oldest) entry carries the whole chain's
        // accumulated trailing `]` characters — by construction (each
        // nesting level's own closing bracket is appended immediately after
        // the fully-expanded inner content, so all of them land consecutively
        // at the true end of the raw value), every non-last entry's text
        // already ends exactly where the next marker begins, with no
        // trailing bracket of its own to strip.
        let text = if i == last_index {
            stripped.trim_end_matches(']')
        } else {
            stripped
        };
        entries.push(text.to_string());
    }
    entries
}

/// Parse one extracted historical entry's `"{date} ({version}) — {text}"`
/// shape into its components for `ChangelogItem` construction. Falls back to
/// treating the whole entry as `summary` (with the leading whitespace-
/// delimited token as `date`, no `version`) when the structured shape isn't
/// found — this crate's own EC-009 mega-line fixture is deliberately
/// unstructured filler content, and the split's correctness/boundedness
/// never depends on this parse succeeding structurally (only on the overall
/// split completing correctly and quickly).
fn parse_dated_entry(entry_text: &str) -> (String, Option<String>, String) {
    let Some(space_pos) = entry_text.find(' ') else {
        return (entry_text.to_string(), None, String::new());
    };
    let date = entry_text[..space_pos].to_string();
    let rest = &entry_text[space_pos + 1..];

    if let Some(open) = rest.find('(')
        && let Some(close_rel) = rest[open..].find(')')
    {
        let close = open + close_rel;
        let version = rest[open + 1..close].to_string();
        let after_paren = &rest[close + 1..];
        if let Some(dash_pos) = after_paren.find('\u{2014}') {
            let summary_start = dash_pos + '\u{2014}'.len_utf8();
            let summary = after_paren[summary_start..]
                .trim_start_matches(' ')
                .to_string();
            return (date, Some(version), summary);
        }
    }
    (date, None, rest.to_string())
}

/// Run the migration subcommand against exactly one target file
/// (BC-10.13.001 v1.1 PC1-PC4, PC6, PC7).
///
/// Orchestration order: parse (`parse_frontmatter`) → classify eligibility
/// (`check_eligibility`, PC2/PC7) → either ensure `changelog:` + D-1144
/// escape remediation on the current entry (`CurrentEntryOnly` path) OR
/// perform the PC7 full-recovery split (`PriorChainSplit` path) → (`Apply`
/// only) write. `Check` mode computes the exact same report but never
/// writes (PC4 verified-clean-report semantics; Invariant 2).
///
/// Returns `Err(MigrateError::NotEligible)` only for the EC-008 case — a
/// `last_amended` field that cannot be located at all in `path`'s
/// frontmatter (a corrupted/unparseable frontmatter delimiter surfaces as
/// `Err(MigrateError::FrontmatterParse)` from the `parse_frontmatter` step
/// instead).
pub fn migrate_file(path: &Path, mode: MigrationMode) -> Result<FileMigrationReport, MigrateError> {
    use crate::changelog::{ChangelogItem, ensure_changelog_field, prepend_changelog_item};
    use crate::eligibility::{CHAIN_MARKER, check_eligibility};
    use crate::escape::{escape_value, needs_escaping};
    use crate::frontmatter::{parse_frontmatter, set_last_amended};

    let mut doc = parse_frontmatter(path)?;
    let raw_last_amended =
        doc.last_amended_raw
            .clone()
            .ok_or_else(|| MigrateError::NotEligible {
                path: path.to_path_buf(),
            })?;

    let eligibility = check_eligibility(&raw_last_amended);
    let is_state = is_state_file(path);
    let mut escape_fixed = false;
    let mut entries_recovered = 0usize;

    let changelog_mutation = match eligibility {
        Eligibility::CurrentEntryOnly => {
            let mutation = ensure_changelog_field(&mut doc, is_state);
            if needs_escaping(&raw_last_amended) {
                let escaped = escape_value(&raw_last_amended);
                set_last_amended(&mut doc, &escaped)?;
                escape_fixed = true;
            }
            mutation
        }
        Eligibility::PriorChainSplit => {
            // PC7 step 1: split at the FIRST marker. `.unwrap_or(len())` is a
            // defensive no-op fallback only — `check_eligibility` already
            // confirmed (via the identical `CHAIN_MARKER` substring check)
            // that this marker is present.
            let marker_pos = raw_last_amended
                .find(CHAIN_MARKER)
                .unwrap_or(raw_last_amended.len());
            let current_text_raw = &raw_last_amended[..marker_pos];
            let tail = &raw_last_amended[marker_pos..];

            // PC7 step 2: current entry stays in last_amended, PC3-escaped.
            let mut new_current = current_text_raw.to_string();
            if needs_escaping(&new_current) {
                new_current = escape_value(&new_current);
                escape_fixed = true;
            }
            set_last_amended(&mut doc, &new_current)?;

            // PC7 step 3/4: parse every historical entry out of the tail.
            let entries = split_tail_entries(tail);
            entries_recovered = entries.len();

            // PC7 step 6: bootstrap changelog: first if absent (no-op for
            // STATE.md, which never gains one — EC-006).
            let mutation = ensure_changelog_field(&mut doc, is_state);

            // PC7 step 5: prepend newest-of-priors-first. EC-006: STATE.md
            // has no changelog: sequence to relocate into — the recovered
            // count is still reported, but the entries themselves are
            // superseded by STATE.md's own body-level Decisions Log rather
            // than written anywhere by this tool.
            if !is_state {
                for entry_text in entries.iter().rev() {
                    let (date, version, mut summary) = parse_dated_entry(entry_text);
                    // S-15.03 SEC-001: `version` is written into its own
                    // YAML double-quoted scalar by `render_item_block`
                    // (`version: "{version}"`) — just like `summary`, it can
                    // in principle carry a control character or an
                    // unescaped quote (a pathologically-shaped legacy
                    // `[Prior: ...]` entry is not guaranteed to match the
                    // `"{date} ({version}) — {text}"` convention exactly),
                    // so it must go through the same escape gate `summary`
                    // already does — this was the one field previously
                    // written unescaped. `date` is deliberately NOT escaped
                    // here: `render_item_block` writes it as a bare, UNQUOTED
                    // plain YAML scalar (`date: {date}`, no surrounding
                    // quotes), so backslash-escape sequences would be
                    // interpreted as literal characters there, not as
                    // escapes — quoting/escaping `date` would require
                    // changing that output shape, which is out of SEC-001's
                    // scope (an output-escaping defect, not an output-shape
                    // change).
                    let version = version.map(|v| {
                        if needs_escaping(&v) {
                            escape_fixed = true;
                            escape_value(&v)
                        } else {
                            v
                        }
                    });
                    if needs_escaping(&summary) {
                        summary = escape_value(&summary);
                        escape_fixed = true;
                    }
                    prepend_changelog_item(
                        &mut doc,
                        ChangelogItem {
                            date,
                            version,
                            summary,
                        },
                    );
                }
            }
            mutation
        }
    };

    let mutated = matches!(eligibility, Eligibility::PriorChainSplit)
        || matches!(changelog_mutation, ChangelogMutation::Added)
        || escape_fixed;

    if mutated && mode == MigrationMode::Apply {
        // S-15.03 SEC-001 (BC-10.13.001 Invariant 4): validate the content
        // this tool is about to write parses under strict YAML `safe_load`
        // BEFORE writing it — never let a corrupt file reach disk.
        crate::yaml_guard::validate_frontmatter_yaml(path, &doc.raw)?;
        std::fs::write(path, &doc.raw).map_err(|source| MigrateError::Io {
            path: path.to_path_buf(),
            source,
        })?;
    }

    Ok(FileMigrationReport {
        path: path.to_path_buf(),
        eligibility,
        changelog_mutation,
        escape_fixed,
        entries_recovered,
        mutated,
    })
}

/// Run the migration subcommand against all 5 `TARGET_FILES`, resolved
/// relative to `factory_root` (BC-10.13.001 §Description, Precondition 1).
pub fn migrate_all(
    factory_root: &Path,
    mode: MigrationMode,
) -> Result<MigrationReport, MigrateError> {
    let mut files = Vec::with_capacity(TARGET_FILES.len());
    for rel in TARGET_FILES {
        files.push(migrate_file(&factory_root.join(rel), mode)?);
    }
    Ok(MigrationReport { files })
}
