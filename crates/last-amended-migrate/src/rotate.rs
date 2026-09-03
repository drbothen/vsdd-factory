//! Rotation subcommand orchestration (BC-10.13.001 PC5, PC6).

use crate::error::MigrateError;
use crate::migrate::MigrationMode;
use std::path::{Path, PathBuf};

/// Tool-default number of most-recent `changelog:` items to retain in the
/// source file when a `--keep-recent` count is not supplied
/// (BC-10.13.001 PC5 "all-but-the-most-recent-K by tool default").
pub const DEFAULT_KEEP_RECENT: usize = 20;

/// Outcome of a single rotation invocation.
#[derive(Debug, Clone)]
pub struct RotationReport {
    pub path: PathBuf,
    pub archive_path: PathBuf,
    /// Number of `changelog:` items moved out of `path` into the archive.
    /// `0` for a below-threshold no-op (EC-004).
    pub items_moved: usize,
    pub mutated: bool,
}

/// Derive the archive destination path: the nearest `.factory/` ancestor
/// directory of `path` (walking upward — `Path::ancestors` yields the most
/// specific ancestor first), joined with
/// `cycles/<cycle_name>/<file-basename>-changelog-archive.md` (PC5's own
/// literal naming convention). Falls back to a `.factory/` sibling of
/// `path`'s own parent directory when no `.factory` ancestor component is
/// found at all (defensive — every real target file lives under `.factory/`,
/// but `rotate_changelog` has no separate `factory_root` parameter to lean
/// on, unlike `migrate_all`).
fn resolve_archive_path(path: &Path, cycle_name: &str) -> Result<PathBuf, MigrateError> {
    let basename = path.file_stem().and_then(|s| s.to_str()).ok_or_else(|| {
        MigrateError::FrontmatterParse {
            path: path.to_path_buf(),
            reason: "cannot derive a file stem for the archive naming convention".to_string(),
        }
    })?;
    let archive_filename = format!("{basename}-changelog-archive.md");

    let factory_root = path
        .ancestors()
        .find(|a| a.file_name().is_some_and(|n| n == ".factory"))
        .map(Path::to_path_buf)
        .unwrap_or_else(|| {
            path.parent()
                .map(Path::to_path_buf)
                .unwrap_or_else(|| PathBuf::from("."))
                .join(".factory")
        });

    Ok(factory_root
        .join("cycles")
        .join(cycle_name)
        .join(archive_filename))
}

/// Remove every line starting with `prefix` from `text` — used to drop a
/// pre-existing `changelog_archive:` discoverability-pointer line before
/// writing a fresh one, so repeated genuine rotations never accumulate
/// duplicate pointer lines.
fn remove_lines_with_prefix(text: &str, prefix: &str) -> String {
    text.split_inclusive('\n')
        .filter(|line| !line.starts_with(prefix))
        .collect()
}

/// Rewrite `raw`'s `changelog:` sequence to hold only `keep_items` (in
/// order), followed by a `changelog_archive:` discoverability pointer line
/// naming `archive_path` (PC5) — every other byte of `raw` is left
/// untouched.
fn rewrite_source_after_rotation(
    path: &Path,
    raw: &str,
    keep_items: &[String],
    archive_path: &Path,
) -> Result<String, MigrateError> {
    let (seq_start, seq_end) =
        crate::frontmatter::changelog_sequence_bounds(raw).ok_or_else(|| {
            MigrateError::FrontmatterParse {
                path: path.to_path_buf(),
                reason: "changelog: sequence not found while rewriting after rotation".to_string(),
            }
        })?;

    let mut new_seq = String::new();
    for item in keep_items {
        new_seq.push_str(item);
    }

    let tail = remove_lines_with_prefix(&raw[seq_end..], "changelog_archive:");
    let pointer_line = format!("changelog_archive: \"{}\"\n", archive_path.display());

    let mut result = String::with_capacity(raw.len() + pointer_line.len());
    result.push_str(&raw[..seq_start]);
    result.push_str(&new_seq);
    result.push_str(&pointer_line);
    result.push_str(&tail);
    Ok(result)
}

/// Rotate `path`'s `changelog:` sequence: move the oldest items past
/// `keep_recent` verbatim into
/// `.factory/cycles/<cycle_name>/<file-basename>-changelog-archive.md`,
/// removing exactly those items from `path` and leaving a discoverability
/// pointer (BC-10.13.001 PC5).
///
/// No-op (EC-004) when the sequence does not exceed `keep_recent`. Creates
/// `.factory/cycles/<cycle_name>/` if it does not already exist (EC-005).
/// Every `changelog:` item's `date:`/`summary:` text is preserved verbatim —
/// only its location (source vs. archive) changes (PC5). Re-running rotation
/// immediately after a successful rotation, before the threshold is
/// exceeded again, is a verified-clean no-op (Invariant 2).
pub fn rotate_changelog(
    path: &Path,
    cycle_name: &str,
    keep_recent: usize,
    mode: MigrationMode,
) -> Result<RotationReport, MigrateError> {
    let doc = crate::frontmatter::parse_frontmatter(path)?;
    let archive_path = resolve_archive_path(path, cycle_name)?;
    let total = doc.changelog_items_raw.len();

    if total <= keep_recent {
        // EC-004: below-threshold no-op.
        return Ok(RotationReport {
            path: path.to_path_buf(),
            archive_path,
            items_moved: 0,
            mutated: false,
        });
    }

    let items_moved = total - keep_recent;
    // `changelog_items_raw` is newest-first: keep the newest `keep_recent`
    // items in the source, move the rest (the oldest) to the archive.
    let (keep_items, move_items) = doc.changelog_items_raw.split_at(keep_recent);

    if mode == MigrationMode::Check {
        return Ok(RotationReport {
            path: path.to_path_buf(),
            archive_path,
            items_moved,
            mutated: false,
        });
    }

    if let Some(parent) = archive_path.parent() {
        std::fs::create_dir_all(parent).map_err(|source| MigrateError::Io {
            path: parent.to_path_buf(),
            source,
        })?;
    }

    let mut archive_content = if archive_path.exists() {
        std::fs::read_to_string(&archive_path).map_err(|source| MigrateError::Io {
            path: archive_path.clone(),
            source,
        })?
    } else {
        String::new()
    };
    for item in move_items {
        archive_content.push_str(item);
    }
    // S-15.03 SEC-001 (BC-10.13.001 Invariant 4): validate the archive's
    // relocated `changelog:` sequence content parses cleanly before writing.
    crate::yaml_guard::validate_changelog_sequence_yaml(&archive_path, &archive_content)?;
    // S-15.03 SEC-003: write-then-rename, not a direct in-place write.
    crate::atomic_write::write_atomic(&archive_path, &archive_content)?;

    let new_raw = rewrite_source_after_rotation(path, &doc.raw, keep_items, &archive_path)?;
    // S-15.03 SEC-001: validate the rewritten source file's frontmatter
    // before writing it back.
    crate::yaml_guard::validate_frontmatter_yaml(path, &new_raw)?;
    // S-15.03 SEC-003: write-then-rename for the source rewrite too.
    crate::atomic_write::write_atomic(path, &new_raw)?;

    Ok(RotationReport {
        path: path.to_path_buf(),
        archive_path,
        items_moved,
        mutated: true,
    })
}
