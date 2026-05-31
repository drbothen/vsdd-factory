//! validate-trajectory-tail-cell-completeness — PostToolUse WASM hook plugin.
//!
//! Closes ADV-EDP1-P75-HIGH-002 / META-LEVEL-30 route (b): codified-without-
//! runtime-gate-permits-silent-degradation. Enforces per-cell presence of the
//! canonical trajectory_tail arrow-sequence at PostToolUse on writes to the
//! D-453(d)-prescribed sites.
//!
//! # Severity arms
//! - STATE.md (sites 1–5): Block severity — cascade all missing sites in one Block.
//! - INDEX.md / burst-log.md / lessons.md (sites 6–9): advisory severity (log_warn + Continue).
//!
//! # Architecture compliance (BC-5.39.009 v1.8)
//! - PostToolUse only; read-only validator (no write capability).
//! - `max_bytes = 524_288` on ALL `host::read_file` calls (invariant 7).
//! - Path-component-strict basename detection via `Path::file_name` (invariant 3).
//! - STATE.md parent-guard: path-component-walk for `.factory` (Precondition 4).
//! - INDEX.md cycle-path guard: dynamic `current_cycle:` resolution (Precondition 4).
//! - LENGTH=4 STRICT equality (inv-4): count == 4, NOT >= 4 (TD-VSDD-059 paper-fix).
//! - is_char_boundary() guards on byte-index slicing (invariant 11).
//! - No regex crate: manual byte-walk scanner (U6).
//! - Fail-open on every host error / UTF-8 decode failure (inv-10, EC-020).

use vsdd_hook_sdk::{HookPayload, HookResult};

/// Maximum bytes to read from any target file via `host::read_file`.
pub const MAX_BYTES: u32 = 524_288;

/// HOST_ABI_VERSION declares the ABI contract version this plugin was built against.
pub const HOST_ABI_VERSION: u32 = 1;

/// The canonical trajectory-tail marker prefix (with trailing space) per inv-4 Step 1.
const MARKER: &str = "trajectory-tail ";

// ---------------------------------------------------------------------------
// Target arm detection
// ---------------------------------------------------------------------------

/// Which target file arm a write event maps to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetArm {
    /// `.factory/STATE.md` — Block severity (sites 1–5).
    State,
    /// active-cycle `INDEX.md` — Advisory severity (sites 6–7).
    Index,
    /// `burst-log.md` — Advisory severity (site 8).
    BurstLog,
    /// `lessons.md` — Advisory severity (site 9, OUT-OF-SCOPE PC10).
    Lessons,
}

/// Detect which target arm a `file_path` maps to (path-component-strict).
///
/// Returns `None` for any non-target file (caller emits `HookResult::Continue`).
///
/// # BC trace
/// BC-5.39.009 invariant 3; Precondition 4 (STATE.md parent-guard).
pub fn target_arm(file_path: &str) -> Option<TargetArm> {
    let path = std::path::Path::new(file_path);
    let basename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    // PATH-COMPONENT-WALK for `.factory` parent guard (BC v1.8 Precondition 4 / F-SP3-008).
    // NOT `starts_with` / `contains` string checks — those are not robust to path
    // normalization (Windows backslash, `./` prefix). The `.factory` component must be
    // present as a discrete path component.
    let is_factory_path = path.components().any(|c| c.as_os_str() == ".factory");
    match basename {
        // STATE.md arm fires only when BOTH basename == "STATE.md" AND a `.factory`
        // path component is present (EC-019 / AC-23 non-factory fail-open).
        "STATE.md" if is_factory_path => Some(TargetArm::State),
        // INDEX.md: dynamic cycle-path guard is applied INSIDE the arm (secondary STATE.md
        // read); here we only require the `.factory` component. Resolution is NOT hardcoded
        // to any specific cycle name (F-SP3-001 + F-SP4-012).
        "INDEX.md" if is_factory_path => Some(TargetArm::Index),
        // burst-log.md / lessons.md match by basename only — unambiguous across cycles.
        "burst-log.md" => Some(TargetArm::BurstLog),
        "lessons.md" => Some(TargetArm::Lessons),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Trajectory_tail arrow counting (LENGTH=4 STRICT — inv-4)
// ---------------------------------------------------------------------------

/// Count `→(\d+)` arrow-segments in an ALREADY-SCOPED segment.
///
/// The caller MUST first locate the `trajectory-tail ` marker and pass only the
/// substring from marker-end to first `;` (or end-of-text).
///
/// Counts EVERY valid `→\d+` segment (returns 5 for a LENGTH=5 sequence); the
/// equality decision lives in [`has_trajectory_tail`].
///
/// # BC trace
/// BC-5.39.009 inv-4 (LENGTH=4 STRICT equality; counting helper).
pub fn count_trajectory_arrows(text: &str) -> usize {
    // `→` is [0xE2, 0x86, 0x92] — 3 bytes UTF-8. `bytes[i..].starts_with(arrow_bytes)`
    // reads complete bytes from `i`, so there is no partial multi-byte boundary risk.
    let arrow_bytes = "→".as_bytes();
    let bytes = text.as_bytes();
    let mut count = 0usize;
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i..].starts_with(arrow_bytes) {
            i += arrow_bytes.len();
            let digit_start = i;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
            if i > digit_start {
                count += 1; // valid →\d+ segment
            }
            // Arrow not followed by a digit is malformed — skip without counting.
        } else {
            i += 1;
        }
    }
    count
}

/// Returns `true` iff the scoped `segment` contains exactly 4 `→\d+` arrow-segments.
///
/// EQUALITY semantics per inv-4: LENGTH=3 AND LENGTH=5 both return `false`.
///
/// # BC trace
/// BC-5.39.009 inv-4.
pub fn has_trajectory_tail(segment: &str) -> bool {
    count_trajectory_arrows(segment) == 4
}

/// Apply the inv-4 two-step marker-prefix check to an extracted text region.
///
/// Step 1: locate the `trajectory-tail ` marker; if absent → `false` (missing site).
/// Step 2: scope to `[marker-end, first ';' or first '\n')` and require `count == 4`.
///
/// The canonical D-453(d) marker form is an inline token whose arrow-sequence
/// immediately follows the marker on the SAME line (e.g.,
/// `trajectory-tail →9→9→9→11;`). The segment therefore terminates at the first
/// `;` OR the first newline — whichever comes first. A newline boundary is
/// required because multi-line YAML block-scalar / multi-row values are joined
/// before this check runs: without it, a prose mention of the marker on one line
/// (`trajectory-tail marker is ABSENT`) would falsely consume arrows from a later
/// line, defeating EC-017's marker-absent detection. The `;`-only form in the BC
/// PC1/PC2/PC4/PC5 prose is a single-line shorthand; the production marker never
/// spans a newline, so the newline boundary preserves all valid cases while
/// rejecting cross-line false matches (EC-017 fail-marker-absent-multi-line).
///
/// `MARKER` is ASCII (16 bytes), so `marker_pos + MARKER.len()` is always on a char
/// boundary; `;` and `\n` are ASCII (1 byte). An `is_char_boundary` guard is
/// asserted defensively before slicing (invariant 11).
fn marker_prefix_check(text: &str) -> bool {
    let marker_pos = match text.find(MARKER) {
        Some(pos) => pos,
        None => return false, // Step 1: marker absent → site missing
    };
    let after_start = marker_pos + MARKER.len();
    if !text.is_char_boundary(after_start) {
        // Defensive: should never happen since MARKER is ASCII.
        return false;
    }
    let after_marker = &text[after_start..];
    // Segment terminates at the first `;` OR the first newline, whichever is earlier.
    let semi = after_marker.find(';');
    let nl = after_marker.find('\n');
    let segment_end = match (semi, nl) {
        (Some(s), Some(n)) => s.min(n),
        (Some(s), None) => s,
        (None, Some(n)) => n,
        (None, None) => after_marker.len(),
    };
    if !after_marker.is_char_boundary(segment_end) {
        return false;
    }
    let segment = &after_marker[..segment_end];
    has_trajectory_tail(segment)
}

// ---------------------------------------------------------------------------
// Frontmatter helpers
// ---------------------------------------------------------------------------

/// Return the YAML frontmatter region (text between the first and second `---` line),
/// or `None` if the document does not open with a `---` delimiter / has no closing one.
fn frontmatter_region(content: &str) -> Option<&str> {
    let mut lines = content.split_inclusive('\n');
    // First line must be a `---` delimiter (trim trailing newline/whitespace).
    let first = lines.next()?;
    if first.trim_end() != "---" {
        return None;
    }
    // Compute byte offset where the body after the first delimiter begins.
    let start = first.len();
    let rest = &content[start..];
    // Find the closing `---` line within `rest`.
    let mut offset = 0usize;
    for line in rest.split_inclusive('\n') {
        if line.trim_end() == "---" {
            return Some(&rest[..offset]);
        }
        offset += line.len();
    }
    None
}

/// Extract the value of a single-line `key:` from a frontmatter region.
///
/// If the value is a YAML block-scalar (`|` or `>`), the indented continuation lines
/// are read and joined (newline for `|`, space for `>`).
///
/// # Folded-`>` blank-line handling (F-007)
///
/// This is a MARKER-DETECTION-oriented join, NOT a spec-complete YAML 1.2 block-scalar
/// parser. For a folded (`>`) scalar, YAML 1.2 folds single newlines to spaces but
/// preserves blank lines as paragraph breaks (a blank line becomes a literal newline in
/// the folded result). Here, a blank line inside the block is collected as an empty
/// string and joined with the surrounding space separator, so a folded blank line
/// collapses to a doubled space rather than a newline. This is intentional and harmless
/// for this hook's sole consumer: `marker_prefix_check` scans the joined value for the
/// `trajectory-tail ` marker and counts `→\d+` segments up to the first `;` or `\n`.
/// Collapsing folded blank lines to spaces neither hides a present marker nor fabricates
/// an absent one — the arrow-sequence is always on the marker's own line, which is never
/// a blank line. A literal (`|`) scalar preserves blank lines via newline joins, which
/// `marker_prefix_check`'s `\n` segment boundary already handles correctly.
fn extract_frontmatter_scalar(region: &str, key: &str) -> Option<String> {
    let key_prefix = format!("{key}:");
    let mut lines = region.lines().peekable();
    while let Some(line) = lines.next() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix(&key_prefix) {
            let value = rest.trim();
            // Block-scalar forms: `|` (literal) or `>` (folded), possibly with chomp
            // indicators (`|-`, `>+`, etc.). The inline value is the indicator only.
            let is_block = matches!(value, "|" | ">" | "|-" | "|+" | ">-" | ">+");
            if is_block {
                let join = if value.starts_with('>') { ' ' } else { '\n' };
                let mut collected: Vec<String> = Vec::new();
                while let Some(next) = lines.peek() {
                    // Continuation lines are indented; a non-indented, non-empty line ends
                    // the block scalar.
                    if next.trim().is_empty() {
                        // Preserve blank lines inside literal blocks; consume.
                        let n = lines.next().unwrap_or("");
                        collected.push(n.trim().to_string());
                        continue;
                    }
                    let leading_ws = next.len() - next.trim_start().len();
                    if leading_ws == 0 {
                        break; // dedented → end of block scalar
                    }
                    let n = lines.next().unwrap_or("");
                    collected.push(n.trim().to_string());
                }
                return Some(collected.join(&join.to_string()));
            }
            return Some(value.to_string());
        }
    }
    None
}

// ---------------------------------------------------------------------------
// STATE.md section extractors (5 sites — Block severity)
// ---------------------------------------------------------------------------

/// Extract the frontmatter `current_step:` value (single- or multi-line scalar).
///
/// # BC trace
/// BC-5.39.009 PC1 + EC-017 (multi-line block-scalar).
pub fn extract_frontmatter_current_step(content: &str) -> Option<String> {
    let region = frontmatter_region(content)?;
    extract_frontmatter_scalar(region, "current_step")
}

/// Extract the `current_cycle:` frontmatter value (used by INDEX.md arm guard).
///
/// Handles bare, single-quoted, double-quoted, trailing-comment, and multi-line
/// block-scalar YAML forms. Empty value → `None`.
///
/// # BC trace
/// BC-5.39.009 §Architecture Anchors (extract_current_cycle).
pub fn extract_current_cycle(content: &str) -> Option<String> {
    let region = frontmatter_region(content)?;
    let raw = extract_frontmatter_scalar(region, "current_cycle")?;
    let raw = raw.trim();
    // F-008 fix: UNQUOTE FIRST, then strip a trailing `#`-comment only on the BARE form.
    // Per BC §Architecture Anchors, the trailing-comment strip applies only when the `#`
    // is "not inside quotes". The prior order (strip `#` then unquote) truncated a quoted
    // value that legitimately contained a `#` (e.g., `current_cycle: "v1.0#hotfix"`),
    // because the `#`-split fired before the quote-aware parse. A YAML quoted scalar's
    // value is the literal content between the quotes (comments after the closing quote
    // are out of scope and dropped); a bare scalar's value ends at the first `#`.
    let value = if let Some(inner) = raw
        .strip_prefix('"')
        .and_then(|s| s.strip_suffix('"'))
        .or_else(|| raw.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')))
    {
        // Quoted form: the value is the literal quoted content; do NOT strip `#`.
        inner.trim().to_string()
    } else {
        // Bare form: strip a trailing `#`-comment, then trim.
        match raw.find('#') {
            Some(pos) => raw[..pos].trim().to_string(),
            None => raw.to_string(),
        }
    };
    if value.is_empty() { None } else { Some(value) }
}

/// Extract the second pipe-delimited column of a markdown table row, trimmed.
fn table_cell_second_column(row: &str) -> Option<String> {
    // Row form: `| col1 | col2 | ... |`. Split on unescaped `|`; the cell is the
    // segment between the 1st and 2nd `|` boundaries (index 1 after leading split).
    let parts: Vec<&str> = row.split('|').collect();
    // parts[0] is the text before the first `|` (usually empty); parts[1] is col1.
    parts.get(2).map(|c| c.trim().to_string())
}

/// Extract the "Last Updated" table-cell value.
///
/// Scans for the `| **Last Updated** |` row and returns the 2nd pipe-column.
///
/// # BC trace
/// BC-5.39.009 PC2 (table cell — NOT a heading).
pub fn extract_last_updated_cell(content: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("| **Last Updated** |") {
            return table_cell_second_column(trimmed);
        }
    }
    None
}

/// Returns `true` if a markdown table row is a separator row (`|---|---|`,
/// `| :--- | ---: |`, etc.).
///
/// F-006 fix: the prior predicate accepted any row whose inner characters were drawn
/// from `{-, |, :, whitespace}` — which mis-classified an all-whitespace/empty data row
/// (`|   |   |`) as a separator and silently DROPPED it. Dropping a thin data row could
/// make `extract_*_latest_row` return an older row (or `None`), producing a
/// false-negative on a Block-severity STATE.md site. A genuine separator row now MUST
/// (a) contain at least one `-`, and (b) have EVERY pipe-delimited cell match the
/// canonical separator-cell grammar `:?-+:?` (optional leading colon, one-or-more
/// dashes, optional trailing colon) after trimming whitespace. An empty cell or a cell
/// containing any other character disqualifies the row.
fn is_separator_row(row: &str) -> bool {
    let trimmed = row.trim();
    // Must be a pipe-delimited row.
    if !trimmed.starts_with('|') {
        return false;
    }
    let inner = trimmed.trim_matches('|');
    if !inner.contains('-') {
        return false;
    }
    // Split into cells on `|`; every cell must be a valid separator cell.
    inner.split('|').all(|cell| is_separator_cell(cell.trim()))
}

/// Returns `true` if `cell` matches the canonical separator-cell grammar `:?-+:?`.
///
/// An empty cell returns `false` (a separator cell must contain at least one dash).
fn is_separator_cell(cell: &str) -> bool {
    let bytes = cell.as_bytes();
    let mut i = 0;
    let n = bytes.len();
    // Optional leading colon.
    if i < n && bytes[i] == b':' {
        i += 1;
    }
    // One-or-more dashes (mandatory).
    let dash_start = i;
    while i < n && bytes[i] == b'-' {
        i += 1;
    }
    if i == dash_start {
        return false; // no dashes → not a separator cell
    }
    // Optional trailing colon.
    if i < n && bytes[i] == b':' {
        i += 1;
    }
    i == n // entire cell consumed
}

/// Capture the table DATA rows that follow a heading (prefix-matched) until the next
/// `## ` heading.
///
/// The canonical markdown table form is `header-row`, `separator-row` (`|---|---|`),
/// then zero or more `data-row`s. This helper drops BOTH the separator row(s) AND the
/// header row that precedes the first separator, returning only the data rows in
/// document order.
///
/// F-003 fix: a prior version's doc-comment claimed it skipped the header row, but the
/// code only skipped separator rows — it pushed the header as a data row. That made the
/// header eligible for bottommost-row selection in single-data-row sections (where the
/// header and the lone data row could be confused). Header detection is now explicit: a
/// `|`-row is a header IFF it appears before the section's first separator row. Once a
/// separator row is seen, every subsequent non-separator `|`-row is a genuine data row.
/// Sections with no separator row at all (malformed / non-tabular) yield no data rows.
fn rows_after_heading<'a>(content: &'a str, heading_prefix: &str) -> Vec<&'a str> {
    let mut rows: Vec<&str> = Vec::new();
    let mut in_section = false;
    // Tracks whether the current section's separator row (`|---|---|`) has been seen.
    // `|`-rows before the separator are header rows and are skipped.
    let mut seen_separator = false;
    for line in content.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("## ") || trimmed == "##" {
            if in_section && trimmed.starts_with(heading_prefix) {
                // re-entered same heading? treat as continuation — unlikely; keep scanning.
                continue;
            }
            if trimmed.starts_with(heading_prefix) {
                in_section = true;
                seen_separator = false;
                continue;
            } else if in_section {
                break; // next `## ` heading ends the section
            }
        }
        if !in_section || !trimmed.starts_with('|') {
            continue;
        }
        if is_separator_row(trimmed) {
            seen_separator = true; // header/data boundary
            continue;
        }
        // Non-separator `|`-row: it is the header iff no separator has been seen yet.
        if seen_separator {
            rows.push(trimmed);
        }
    }
    rows
}

/// Extract the bottommost non-archived/non-compacted Phase Progress row.
///
/// # BC trace
/// BC-5.39.009 PC3 (single-row text; ONE-tail-per-region).
pub fn extract_phase_progress_latest_row(content: &str) -> Option<String> {
    let rows = rows_after_heading(content, "## Phase Progress");
    for row in rows.iter().rev() {
        let upper = row.to_uppercase();
        if upper.contains("ARCHIVED") || upper.contains("COMPACTED") {
            continue;
        }
        return Some((*row).to_string());
    }
    None
}

/// Extract the bottommost active Concurrent Cycles row.
///
/// # BC trace
/// BC-5.39.009 PC4 (single-row text; skip CLOSED/COMPACTED/ARCHIVED).
pub fn extract_concurrent_cycles_latest_row(content: &str) -> Option<String> {
    let rows = rows_after_heading(content, "## Concurrent Cycles");
    for row in rows.iter().rev() {
        let upper = row.to_uppercase();
        if upper.contains("CLOSED") || upper.contains("COMPACTED") || upper.contains("ARCHIVED") {
            continue;
        }
        return Some((*row).to_string());
    }
    None
}

/// Extract the Session Resume Checkpoint §1 body.
///
/// Prefix-matches `## Session Resume Checkpoint` (tolerates parenthetical suffix), then
/// captures the `### §1.` body up to the next `###` or `##` heading.
///
/// # BC trace
/// BC-5.39.009 PC5 (prefix-match heading; §1 body).
pub fn extract_session_resume_section_1(content: &str) -> Option<String> {
    let mut in_checkpoint = false;
    let mut in_section_1 = false;
    let mut body: Vec<&str> = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("## Session Resume Checkpoint") {
            in_checkpoint = true;
            continue;
        }
        if in_checkpoint && !in_section_1 {
            if trimmed.starts_with("### §1.") {
                in_section_1 = true;
                continue;
            }
            // Another `## ` heading before reaching §1 ends the checkpoint search.
            if trimmed.starts_with("## ") {
                in_checkpoint = false;
            }
            continue;
        }
        if in_section_1 {
            if trimmed.starts_with("###") || trimmed.starts_with("## ") {
                break; // next sub-section / section ends §1
            }
            body.push(line);
        }
    }
    if in_section_1 {
        Some(body.join("\n"))
    } else {
        None
    }
}

/// Extract the bottommost `### Dim-7` block from burst-log.md.
///
/// Block = text from the bottommost `### Dim-7` heading to next `## ` / `### `.
///
/// # BC trace
/// BC-5.39.009 PC9 (extract_burst_log_latest_dim7).
pub fn extract_burst_log_latest_dim7(content: &str) -> Option<String> {
    let lines: Vec<&str> = content.lines().collect();
    // Find the index of the bottommost `### Dim-7` heading.
    let mut last_dim7: Option<usize> = None;
    for (idx, line) in lines.iter().enumerate() {
        if line.trim_start().starts_with("### Dim-7") {
            last_dim7 = Some(idx);
        }
    }
    let start = last_dim7?;
    let mut block: Vec<&str> = Vec::new();
    for line in &lines[start + 1..] {
        let trimmed = line.trim_start();
        if trimmed.starts_with("## ") || trimmed.starts_with("### ") || trimmed == "##" {
            break;
        }
        block.push(line);
    }
    Some(block.join("\n"))
}

// ---------------------------------------------------------------------------
// Missing-site accumulator + cascade check
// ---------------------------------------------------------------------------

/// A STATE.md site that is missing its trajectory_tail (cascade element).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissingStateSite {
    /// Human-readable site name used verbatim in the cascade Block message.
    pub site_name: &'static str,
}

/// An advisory warning for a non-STATE.md site (log_warn payload).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdvisoryWarning {
    /// Human-readable advisory message.
    pub message: String,
}

/// Run all 5 STATE.md extractors; return ALL missing sites (cascade — inv-8).
///
/// A site is "missing" when its extractor returns `None` OR the extracted text fails
/// the inv-4 two-step marker-prefix check.
///
/// # BC trace
/// BC-5.39.009 PC1–PC5 + PC6 (cascade) + invariant 8.
pub fn check_state_md(content: &str) -> Vec<MissingStateSite> {
    let mut missing: Vec<MissingStateSite> = Vec::new();

    let site_present = |text: Option<String>| -> bool {
        match text {
            Some(t) => marker_prefix_check(&t),
            None => false,
        }
    };

    if !site_present(extract_frontmatter_current_step(content)) {
        missing.push(MissingStateSite {
            site_name: "STATE.md frontmatter current_step",
        });
    }
    if !site_present(extract_last_updated_cell(content)) {
        missing.push(MissingStateSite {
            site_name: "STATE.md Last Updated cell",
        });
    }
    if !site_present(extract_phase_progress_latest_row(content)) {
        missing.push(MissingStateSite {
            site_name: "STATE.md Phase Progress rows",
        });
    }
    if !site_present(extract_concurrent_cycles_latest_row(content)) {
        missing.push(MissingStateSite {
            site_name: "STATE.md Concurrent Cycles row",
        });
    }
    if !site_present(extract_session_resume_section_1(content)) {
        missing.push(MissingStateSite {
            site_name: "STATE.md Session Resume Section 1",
        });
    }

    missing
}

/// Check the 2 INDEX.md advisory sites (Convergence Status + adv-table latest row).
///
/// # BC trace
/// BC-5.39.009 PC7 + PC8 + invariant 6.
pub fn check_index_sites(content: &str) -> Vec<AdvisoryWarning> {
    let mut warnings: Vec<AdvisoryWarning> = Vec::new();

    // Site 6 (PC7): Convergence Status row. Locate the markdown table DATA row whose
    // first cell names "Convergence Status" and check it for a LENGTH=4 tail.
    //
    // F-002 fix: the prior `find(|l| l.contains("Convergence Status"))` matched the
    // `## Convergence Status` HEADING line first (which never carries a tail), so the
    // advisory fired unconditionally regardless of the data row's content — the arm was
    // effectively inert and the false-green advisory tests never caught it. We now skip
    // any `#`-heading line and require a pipe-delimited (`|`) row, so the check binds to
    // the actual Convergence Status table row.
    let convergence = content
        .lines()
        .map(|l| l.trim_start())
        .find(|l| l.starts_with('|') && l.contains("Convergence Status"))
        .map(|l| l.to_string());
    let convergence_ok = match &convergence {
        Some(line) => has_trajectory_tail(line),
        // No Convergence Status DATA row at all → fail-open (treat as present); the
        // section may be absent / formatted as prose on a fresh cycle INDEX.md (inv-10).
        None => true,
    };
    if !convergence_ok {
        warnings.push(AdvisoryWarning {
            message: "INDEX.md Convergence Status row missing LENGTH=4 trajectory_tail".to_string(),
        });
    }

    // Site 7 (PC8): latest adversarial-review summary-table row. Per BC-5.39.009 PC8
    // + §Architecture Anchors, this is the bottommost DATA row WITHIN the
    // `## Adversarial Reviews` table — NOT the global-bottommost `|`-row in the file.
    //
    // F-001 fix: a global `rfind` of the bottommost markdown table row selects the
    // WRONG row in production INDEX.md layout. The canonical cycle INDEX.md places the
    // `## Adversarial Reviews` table BEFORE the `## Convergence Status` section, and
    // other tables (Convergence Status metric tables, sub-cycle tables) follow it. A
    // file-global bottommost `|`-row therefore lands in a later table (e.g., the
    // Convergence Status "Streak" row), defeating PC8. Scope to the
    // `## Adversarial Reviews` table via the same `rows_after_heading` helper used by
    // the STATE.md row extractors (which already strips header + separator rows), then
    // take the bottommost data row within that section.
    let adv_rows = rows_after_heading(content, "## Adversarial Reviews");
    let adv_ok = match adv_rows.last() {
        // No `## Adversarial Reviews` table at all → fail-open (treat as present);
        // the section may not yet exist on a freshly-created cycle INDEX.md (inv-10).
        None => true,
        Some(row) => has_trajectory_tail(row),
    };
    if !adv_ok {
        warnings.push(AdvisoryWarning {
            message: "INDEX.md adv-table latest row missing LENGTH=4 trajectory_tail".to_string(),
        });
    }

    warnings
}

/// Check the burst-log.md advisory site (latest Dim-7 block).
///
/// If no `### Dim-7` block exists at all, the site is treated as PRESENT (fail-open per
/// inv-10) — no advisory is emitted.
///
/// # BC trace
/// BC-5.39.009 PC9 + invariant 6.
pub fn check_burst_log_sites(content: &str) -> Vec<AdvisoryWarning> {
    let mut warnings: Vec<AdvisoryWarning> = Vec::new();
    if let Some(block) = extract_burst_log_latest_dim7(content)
        && !has_trajectory_tail(&block)
    {
        warnings.push(AdvisoryWarning {
            message: "burst-log.md Dim-7 block missing LENGTH=4 trajectory_tail".to_string(),
        });
    }
    // Absent Dim-7 block → present (fail-open); no advisory.
    warnings
}

/// Check the lessons.md advisory site (PC10 OUT-OF-SCOPE — always returns empty).
///
/// PC10 is OUT-OF-SCOPE per BC-5.39.009 v1.8 (F-SP5-003): lessons.md trends are inline
/// prose with no machine-extractable trend-table structure. The lessons.md arm performs
/// no count check and always returns an empty warning list (the caller emits Continue).
///
/// # BC trace
/// BC-5.39.009 PC10 (OUT-OF-SCOPE) + invariant 6.
pub fn check_lessons_sites(_content: &str) -> Vec<AdvisoryWarning> {
    Vec::new()
}

// ---------------------------------------------------------------------------
// Read + decode seam (pure, unit-testable — F-005)
// ---------------------------------------------------------------------------

/// Map a `host::read_file` result through the UTF-8 decode gate to either decoded
/// content or a fail-open advisory message.
///
/// This is the pure, unit-testable seam for the read-and-decode error path (F-005). It
/// closes two findings at once:
///
/// * The read-error branch (any `HostError`, INCLUDING `HostError::OutputTooLarge`)
///   maps to `Err(advisory)` → the caller emits `log_warn` + `HookResult::Continue`
///   (fail-open per inv-10 / PC11). A prior version's bats test claimed a Rust unit test
///   exercised the `OutputTooLarge` variant, but none existed; this seam plus
///   `test_*_output_too_large_maps_to_advisory` makes that claim true.
/// * The UTF-8 decode branch (`String::from_utf8` Err) maps to `Err(advisory)` → same
///   fail-open path (EC-020 / inv-13).
///
/// Monomorphized to the concrete `vsdd_hook_sdk::HostError` type (F-P2-002): the seam
/// statically binds to the SAME error type the production `host::read_file` call returns,
/// so the `OutputTooLarge` unit test exercises the exact production mapping rather than an
/// abstract `E: Debug`. The test constructs a real `HostError::OutputTooLarge` value (no
/// host I/O required), and the production caller passes the SDK's `read_file` result
/// directly. `HostError` implements `Debug` (not `Display`), matching the `{e:?}`
/// formatting of the read-error advisory.
///
/// # BC trace
/// BC-5.39.009 PC11 (uniform HostError fail-open), inv-10, inv-13, EC-020.
pub fn decode_read_result(
    file_path: &str,
    result: Result<Vec<u8>, vsdd_hook_sdk::host::HostError>,
) -> Result<String, String> {
    match result {
        Ok(bytes) => String::from_utf8(bytes).map_err(|e| {
            format!("invalid UTF-8 in {file_path}: {e}; skipping (fail-open per EC-020)")
        }),
        Err(e) => Err(format!(
            "read failed for {file_path}: {e:?}; skipping (fail-open per inv-10)"
        )),
    }
}

// ---------------------------------------------------------------------------
// Main hook orchestration (effectful entry point)
// ---------------------------------------------------------------------------

/// PostToolUse entry point called by `vsdd_hook_sdk::__internal::run`.
///
/// Routes by [`target_arm`], reads post-write content via `host::read_file`
/// (`max_bytes = 524_288`), decodes to `&str` (inv-13; UTF-8 failure → fail-open per
/// EC-020), and dispatches to the STATE.md Block arm or one of the advisory arms.
///
/// # BC trace
/// BC-5.39.009 PC1–PC12; invariants 1–13.
pub fn on_post_tool_use(payload: HookPayload) -> HookResult {
    use vsdd_hook_sdk::host;

    const HOOK_NAME: &str = "validate-trajectory-tail-cell-completeness";

    let file_path = match payload.tool_input.get("file_path").and_then(|v| v.as_str()) {
        Some(p) => p.to_string(),
        None => {
            host::log_warn(&format!(
                "[{HOOK_NAME}] file_path absent from tool_input — graceful degrade"
            ));
            return HookResult::Continue;
        }
    };

    let arm = match target_arm(&file_path) {
        Some(a) => a,
        None => return HookResult::Continue,
    };

    // Read bytes then decode to String through the pure F-005 seam. The seam maps EVERY
    // HostError (including HostError::OutputTooLarge) AND the UTF-8 decode Err branch to
    // a single fail-open advisory string (inv-10 / inv-13 / EC-020 / PC11).
    let content = match decode_read_result(&file_path, host::read_file(&file_path, MAX_BYTES, 2000))
    {
        Ok(s) => s,
        Err(advisory) => {
            host::log_warn(&format!("[{HOOK_NAME}] {advisory}"));
            return HookResult::Continue;
        }
    };

    match arm {
        TargetArm::State => {
            let missing = check_state_md(&content);
            if missing.is_empty() {
                HookResult::Continue
            } else {
                let sites = missing
                    .iter()
                    .map(|s| format!("  - {}", s.site_name))
                    .collect::<Vec<_>>()
                    .join("\n");
                HookResult::block_with_fix(
                    HOOK_NAME,
                    format!(
                        "STATE.md trajectory_tail missing from {} prescribed site(s) (D-453(d)):\n{}",
                        missing.len(),
                        sites
                    ),
                    "Add a trajectory_tail arrow-sequence (→N→N→N→N, LENGTH=4 per D-433(e)) to each listed site",
                    "trajectory_tail_site_missing",
                )
            }
        }
        TargetArm::Index => {
            // Dynamic cycle-path guard: secondary read of STATE.md current_cycle: at runtime.
            // Routed through the same F-005 seam: any read error or UTF-8 decode failure
            // on the secondary STATE.md read fails open (skip INDEX.md check).
            let active_cycle = match decode_read_result(
                ".factory/STATE.md",
                host::read_file(".factory/STATE.md", MAX_BYTES, 2000),
            ) {
                Ok(s) => extract_current_cycle(&s).unwrap_or_default(),
                Err(advisory) => {
                    host::log_warn(&format!(
                        "[{HOOK_NAME}] INDEX.md arm cycle-guard: {advisory}"
                    ));
                    return HookResult::Continue;
                }
            };
            // PATH-COMPONENT-WALK (NOT substring contains) for the active-cycle guard.
            let active = !active_cycle.is_empty()
                && std::path::Path::new(&file_path)
                    .components()
                    .any(|c| c.as_os_str() == active_cycle.as_str());
            if !active {
                return HookResult::Continue; // non-active-cycle INDEX.md or cycle unknown
            }
            for w in check_index_sites(&content) {
                host::log_warn(&format!("[{HOOK_NAME}] {}", w.message));
            }
            HookResult::Continue
        }
        TargetArm::BurstLog => {
            for w in check_burst_log_sites(&content) {
                host::log_warn(&format!("[{HOOK_NAME}] {}", w.message));
            }
            HookResult::Continue
        }
        TargetArm::Lessons => {
            for w in check_lessons_sites(&content) {
                host::log_warn(&format!("[{HOOK_NAME}] {}", w.message));
            }
            HookResult::Continue
        }
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // count_trajectory_arrows — BC-5.39.009 inv-4 STRICT equality semantics
    // -----------------------------------------------------------------------

    /// BC-5.39.009 inv-4: LENGTH=0 (empty string) => count=0 (not 4 => absent)
    #[test]
    fn test_BC_5_39_009_invariant_4_count_arrows_empty_segment() {
        // todo!() will panic — that is the Red Gate failure mode for stub functions
        let count = count_trajectory_arrows("");
        assert_eq!(count, 0, "empty segment must count 0 arrows");
    }

    /// BC-5.39.009 inv-4: single arrow →9 => count=1
    #[test]
    fn test_BC_5_39_009_invariant_4_count_arrows_single() {
        let count = count_trajectory_arrows("→9");
        assert_eq!(count, 1);
    }

    /// BC-5.39.009 inv-4: 3 arrows →9→9→9 => count=3 (LENGTH=3 ≠ 4 => absent)
    #[test]
    fn test_BC_5_39_009_invariant_4_count_arrows_length_3() {
        let count = count_trajectory_arrows("→9→9→9");
        assert_eq!(count, 3, "LENGTH=3 must count 3 arrows");
    }

    /// BC-5.39.009 inv-4: 4 arrows →9→9→9→9 => count=4 (LENGTH=4 STRICT => present)
    #[test]
    fn test_BC_5_39_009_invariant_4_count_arrows_length_4() {
        let count = count_trajectory_arrows("→9→9→9→9");
        assert_eq!(count, 4, "LENGTH=4 canonical tail must count 4 arrows");
    }

    /// BC-5.39.009 inv-4: 5 arrows →9→9→9→9→9 => count=5 (LENGTH=5 ≠ 4 => absent per EC-018)
    #[test]
    fn test_BC_5_39_009_invariant_4_count_arrows_length_5() {
        let count = count_trajectory_arrows("→9→9→9→9→9");
        assert_eq!(count, 5, "LENGTH=5 must count 5 arrows");
    }

    /// BC-5.39.009 inv-4: multi-digit values →10→12→11→13 => count=4 (valid per EC-013)
    #[test]
    fn test_BC_5_39_009_invariant_4_count_arrows_multi_digit() {
        let count = count_trajectory_arrows("→10→12→11→13");
        assert_eq!(count, 4, "multi-digit LENGTH=4 tail must count 4 arrows");
    }

    /// BC-5.39.009 inv-4: plain text with no arrows => count=0
    #[test]
    fn test_BC_5_39_009_invariant_4_count_arrows_no_arrows() {
        let count = count_trajectory_arrows("Phase 3 step 42 no tail");
        assert_eq!(count, 0);
    }

    /// BC-5.39.009 inv-4: arrow without digits (malformed) does NOT count
    /// The byte-walk should only count valid →\d+ segments (arrow + at least one digit)
    #[test]
    fn test_BC_5_39_009_invariant_4_count_arrows_arrow_no_digit_not_counted() {
        // "→ abc" — arrow not followed by digit; must not count
        let count = count_trajectory_arrows("→ abc");
        assert_eq!(count, 0, "arrow not followed by digit must not count");
    }

    // -----------------------------------------------------------------------
    // has_trajectory_tail — BC-5.39.009 inv-4 STRICT equality (count == 4)
    // -----------------------------------------------------------------------

    /// BC-5.39.009 inv-4: has_trajectory_tail("→9→9→9→9") == true (LENGTH=4 STRICT)
    #[test]
    fn test_BC_5_39_009_invariant_4_has_tail_length_4_returns_true() {
        let result = has_trajectory_tail("→9→9→9→9");
        assert!(result, "→9→9→9→9 must be recognized as present (count==4)");
    }

    /// BC-5.39.009 inv-4: has_trajectory_tail("→9→9→9") == false (LENGTH=3 ≠ 4)
    #[test]
    fn test_BC_5_39_009_invariant_4_has_tail_length_3_returns_false() {
        let result = has_trajectory_tail("→9→9→9");
        assert!(
            !result,
            "→9→9→9 (LENGTH=3) must NOT be recognized as present"
        );
    }

    /// BC-5.39.009 EC-018 + inv-4: has_trajectory_tail("→9→9→9→9→9") == false (LENGTH=5 ≠ 4)
    /// CRITICAL REGRESSION GUARD: non-anchored (→[0-9]+){4} match would falsely pass on LENGTH=5
    #[test]
    fn test_BC_5_39_009_EC018_has_tail_length_5_returns_false() {
        let result = has_trajectory_tail("→9→9→9→9→9");
        assert!(
            !result,
            "→9→9→9→9→9 (LENGTH=5) must NOT pass — equality semantics required (count==4 not >=4)"
        );
    }

    /// BC-5.39.009 inv-4: has_trajectory_tail("") == false (empty => count=0 ≠ 4)
    #[test]
    fn test_BC_5_39_009_invariant_4_has_tail_empty_returns_false() {
        let result = has_trajectory_tail("");
        assert!(!result, "empty segment must not have tail");
    }

    /// BC-5.39.009 EC-013: multi-digit values →10→12→11→13 pass (count=4)
    #[test]
    fn test_BC_5_39_009_EC013_has_tail_multi_digit_returns_true() {
        let result = has_trajectory_tail("→10→12→11→13");
        assert!(
            result,
            "multi-digit →10→12→11→13 must be recognized as present (count==4)"
        );
    }

    // -----------------------------------------------------------------------
    // target_arm — BC-5.39.009 inv-3 path-component-strict + Precondition 4
    // -----------------------------------------------------------------------

    /// BC-5.39.009 inv-3 + Precondition 4: ".factory/STATE.md" => State arm
    #[test]
    fn test_BC_5_39_009_invariant_3_target_arm_factory_state_md() {
        let arm = target_arm(".factory/STATE.md");
        assert!(
            matches!(arm, Some(TargetArm::State)),
            ".factory/STATE.md must resolve to State arm"
        );
    }

    /// BC-5.39.009 EC-019 + Precondition 4: non-factory STATE.md => None (Continue)
    #[test]
    fn test_BC_5_39_009_EC019_target_arm_non_factory_state_md_returns_none() {
        // Path lacks .factory component — Precondition 4 parent-guard must reject it
        let arm = target_arm("some-other/STATE.md");
        assert!(
            arm.is_none(),
            "STATE.md without .factory component must NOT trigger State arm"
        );
    }

    /// BC-5.39.009 EC-015: /factory-artifacts/STATE.md has basename STATE.md but no .factory component
    #[test]
    fn test_BC_5_39_009_EC015_target_arm_factory_artifacts_path_not_factory_component() {
        // "factory-artifacts" is NOT the ".factory" component — path-component-walk must reject
        let arm = target_arm("factory-artifacts/STATE.md");
        assert!(
            arm.is_none(),
            "factory-artifacts/STATE.md must NOT trigger State arm (no .factory component)"
        );
    }

    /// BC-5.39.009 inv-3: ".factory/cycles/v1.0-brownfield-backfill/INDEX.md" => Index arm
    #[test]
    fn test_BC_5_39_009_invariant_3_target_arm_factory_index_md() {
        let arm = target_arm(".factory/cycles/v1.0-brownfield-backfill/INDEX.md");
        assert!(
            matches!(arm, Some(TargetArm::Index)),
            ".factory/.../INDEX.md must resolve to Index arm"
        );
    }

    /// BC-5.39.009 inv-3: ".factory/cycles/v1.0-brownfield-backfill/burst-log.md" => BurstLog arm
    #[test]
    fn test_BC_5_39_009_invariant_3_target_arm_burst_log_md() {
        let arm = target_arm(".factory/cycles/v1.0-brownfield-backfill/burst-log.md");
        assert!(
            matches!(arm, Some(TargetArm::BurstLog)),
            "burst-log.md must resolve to BurstLog arm"
        );
    }

    /// BC-5.39.009 inv-3: ".factory/cycles/v1.0-brownfield-backfill/lessons.md" => Lessons arm
    #[test]
    fn test_BC_5_39_009_invariant_3_target_arm_lessons_md() {
        let arm = target_arm(".factory/cycles/v1.0-brownfield-backfill/lessons.md");
        assert!(
            matches!(arm, Some(TargetArm::Lessons)),
            "lessons.md must resolve to Lessons arm"
        );
    }

    /// BC-5.39.009 inv-3 + EC-005: ".factory/not-STATE.md" => None (Continue immediately)
    #[test]
    fn test_BC_5_39_009_EC005_target_arm_wrong_basename_returns_none() {
        let arm = target_arm(".factory/not-STATE.md");
        assert!(
            arm.is_none(),
            "not-STATE.md must NOT trigger any arm (basename mismatch)"
        );
    }

    /// BC-5.39.009 inv-3: "STORY-INDEX.md" not in any target set => None
    #[test]
    fn test_BC_5_39_009_invariant_3_target_arm_story_index_returns_none() {
        let arm = target_arm(".factory/stories/STORY-INDEX.md");
        assert!(arm.is_none(), "STORY-INDEX.md must NOT trigger any arm");
    }

    // -----------------------------------------------------------------------
    // extract_frontmatter_current_step — BC-5.39.009 PC1
    // -----------------------------------------------------------------------

    /// BC-5.39.009 PC1: standard inline current_step with trajectory-tail marker => Some(value)
    #[test]
    fn test_BC_5_39_009_PC1_extract_frontmatter_current_step_inline_with_marker() {
        let content = "---\ndocument_type: state\ncurrent_step: \"Phase 3 — trajectory-tail →9→9→9→9; done\"\n---\n\nbody";
        let result = extract_frontmatter_current_step(content);
        assert!(
            result.is_some(),
            "should extract current_step value from frontmatter"
        );
        let val = result.expect("current_step value must be Some after is_some() assertion");
        assert!(
            val.contains("trajectory-tail"),
            "extracted value must contain the trajectory-tail marker"
        );
    }

    /// BC-5.39.009 PC1 / EC-001: current_step key absent => None (treat as missing site)
    #[test]
    fn test_BC_5_39_009_EC001_extract_frontmatter_current_step_absent_key_returns_none() {
        let content = "---\ndocument_type: state\nversion: \"1.0\"\n---\n\nbody";
        let result = extract_frontmatter_current_step(content);
        assert!(
            result.is_none(),
            "absent current_step key must return None (treat as missing site)"
        );
    }

    /// BC-5.39.009 PC1 / EC-016: frontmatter absent (no --- delimiters) => None (fail-open)
    #[test]
    fn test_BC_5_39_009_EC016_extract_frontmatter_no_delimiters_returns_none() {
        let content = "No frontmatter at all — just body text.\nSome content here.";
        let result = extract_frontmatter_current_step(content);
        assert!(
            result.is_none(),
            "absent frontmatter region must return None (fail-open per EC-016)"
        );
    }

    /// BC-5.39.009 PC1 / EC-017: multi-line block-scalar current_step => joined value returned
    #[test]
    fn test_BC_5_39_009_EC017_extract_frontmatter_current_step_multiline_block_scalar() {
        // Using | block scalar with continuation line containing the trajectory-tail marker
        let content = "---\ndocument_type: state\ncurrent_step: |\n  Phase 3 step.\n  trajectory-tail →9→9→9→9; continuation line.\n---\n\nbody";
        let result = extract_frontmatter_current_step(content);
        assert!(
            result.is_some(),
            "multi-line block-scalar current_step must be extracted"
        );
        let val =
            result.expect("multi-line current_step value must be Some after is_some() assertion");
        assert!(
            val.contains("trajectory-tail"),
            "joined multi-line value must contain trajectory-tail marker"
        );
    }

    // -----------------------------------------------------------------------
    // extract_current_cycle — BC-5.39.009 Precondition 4 / F-SP3-001
    // -----------------------------------------------------------------------

    /// BC-5.39.009 Precondition 4: standard current_cycle: value extracted
    #[test]
    fn test_BC_5_39_009_precondition_4_extract_current_cycle_bare_value() {
        let content =
            "---\ndocument_type: state\ncurrent_cycle: v1.0-brownfield-backfill\n---\n\nbody";
        let result = extract_current_cycle(content);
        assert_eq!(
            result,
            Some("v1.0-brownfield-backfill".to_string()),
            "bare current_cycle value must be extracted correctly"
        );
    }

    /// BC-5.39.009 Precondition 4: quoted current_cycle value extracted
    #[test]
    fn test_BC_5_39_009_precondition_4_extract_current_cycle_quoted_value() {
        let content = "---\ncurrent_cycle: \"v1.0-brownfield-backfill\"\n---\n";
        let result = extract_current_cycle(content);
        assert_eq!(
            result,
            Some("v1.0-brownfield-backfill".to_string()),
            "quoted current_cycle value must be extracted (quotes stripped)"
        );
    }

    /// BC-5.39.009 Precondition 4: absent current_cycle key => None
    #[test]
    fn test_BC_5_39_009_precondition_4_extract_current_cycle_absent_returns_none() {
        let content = "---\ndocument_type: state\n---\n\nbody";
        let result = extract_current_cycle(content);
        assert!(
            result.is_none(),
            "absent current_cycle key must return None"
        );
    }

    // -----------------------------------------------------------------------
    // check_state_md — BC-5.39.009 PC1-6 + inv-8 cascade
    // -----------------------------------------------------------------------

    /// BC-5.39.009 PC12: all 5 sites present => empty Vec (no missing sites)
    #[test]
    fn test_BC_5_39_009_PC12_check_state_md_all_sites_present_empty_vec() {
        let content = "\
---
document_type: state
current_step: \"Phase 3 — trajectory-tail →9→9→9→9; done\"
current_cycle: \"v1.0-brownfield-backfill\"
---

| **Last Updated** | 2026-05-28 — trajectory-tail →9→9→9→9; good |

## Phase Progress

| Pass | Status | Notes |
|------|--------|-------|
| P-1  | COMPLETE | trajectory-tail →9→9→9→9 |

## Concurrent Cycles

| Cycle | Status | Notes |
|-------|--------|-------|
| v1.0 | active | trajectory-tail →9→9→9→9 |

## Session Resume Checkpoint (2026-05-28)

### §1. Where We Are

trajectory-tail →9→9→9→9; all sites present.
";
        let missing = check_state_md(content);
        assert!(
            missing.is_empty(),
            "all sites present => no missing sites (PC12 pass case)"
        );
    }

    /// BC-5.39.009 PC1 + PC6: current_step missing tail => Vec with 1 site named "frontmatter current_step"
    #[test]
    fn test_BC_5_39_009_PC1_check_state_md_frontmatter_missing_returns_site() {
        let content = "\
---
document_type: state
current_step: \"Phase 3 step — no tail\"
current_cycle: \"v1.0-brownfield-backfill\"
---

| **Last Updated** | 2026-05-28 — trajectory-tail →9→9→9→9; good |

## Phase Progress

| Pass | Status | Notes |
|------|--------|-------|
| P-1  | COMPLETE | trajectory-tail →9→9→9→9 |

## Concurrent Cycles

| Cycle | Status | Notes |
|-------|--------|-------|
| v1.0 | active | trajectory-tail →9→9→9→9 |

## Session Resume Checkpoint (2026-05-28)

### §1. Where We Are

trajectory-tail →9→9→9→9; session resume OK.
";
        let missing = check_state_md(content);
        assert_eq!(
            missing.len(),
            1,
            "exactly 1 site should be missing (current_step)"
        );
        assert!(
            missing[0].site_name.contains("current_step"),
            "missing site_name must reference current_step"
        );
    }

    /// BC-5.39.009 PC6 + inv-8: multiple missing sites => Vec with all missing (cascade)
    #[test]
    fn test_BC_5_39_009_invariant_8_check_state_md_cascade_accumulates_all_sites() {
        // 3 sites missing: Last Updated (site 2), Concurrent Cycles (site 4), Session Resume (site 5)
        let content = "\
---
document_type: state
current_step: \"Phase 3 — trajectory-tail →9→9→9→9; OK\"
current_cycle: \"v1.0-brownfield-backfill\"
---

| **Last Updated** | 2026-05-28 — no trajectory tail here |

## Phase Progress

| Pass | Status | Notes |
|------|--------|-------|
| P-1  | COMPLETE | trajectory-tail →9→9→9→9 |

## Concurrent Cycles

| Cycle | Status | Notes |
|-------|--------|-------|
| v1.0 | active | no trajectory tail here |

## Session Resume Checkpoint (2026-05-28)

### §1. Where We Are

No trajectory tail in session resume section 1.
";
        let missing = check_state_md(content);
        assert!(
            missing.len() >= 2,
            "cascade must accumulate ALL missing sites (not short-circuit at first); got {} missing",
            missing.len()
        );
    }

    // -----------------------------------------------------------------------
    // EC-018 regression guard — BC-5.39.009 inv-4 LENGTH=5 scoped count
    // -----------------------------------------------------------------------

    /// BC-5.39.009 EC-018: current_step with LENGTH=5 scoped count => site missing (Block)
    /// This is the CRITICAL regression guard: naive (→[0-9]+){4} non-anchored match
    /// would falsely pass on "trajectory-tail →9→9→9→9→9" (matching first 4 of 5 arrows).
    /// Equality count == 4 MUST reject count == 5.
    #[test]
    fn test_BC_5_39_009_EC018_check_state_md_length_5_in_current_step_blocks() {
        let content = "\
---
document_type: state
current_step: \"Phase 3 — trajectory-tail →9→9→9→9→9; LENGTH=5 violation\"
current_cycle: \"v1.0-brownfield-backfill\"
---

| **Last Updated** | 2026-05-28 — trajectory-tail →9→9→9→9; OK |

## Phase Progress

| Pass | Status | Notes |
|------|--------|-------|
| P-1  | COMPLETE | trajectory-tail →9→9→9→9 |

## Concurrent Cycles

| Cycle | Status | Notes |
|-------|--------|-------|
| v1.0 | active | trajectory-tail →9→9→9→9 |

## Session Resume Checkpoint (2026-05-28)

### §1. Where We Are

trajectory-tail →9→9→9→9; session resume OK.
";
        let missing = check_state_md(content);
        assert_eq!(
            missing.len(),
            1,
            "LENGTH=5 in current_step must block (count=5 ≠ 4; equality semantics)"
        );
        assert!(
            missing[0].site_name.contains("current_step"),
            "missing site must be current_step for LENGTH=5 violation"
        );
    }

    // -----------------------------------------------------------------------
    // F-003: rows_after_heading header-row skip (doc-comment now matches code)
    // -----------------------------------------------------------------------

    /// F-003: a single-data-row section returns exactly the DATA row, NOT the header row.
    /// The header (the `|`-row before the separator) must be dropped.
    #[test]
    fn test_F003_rows_after_heading_skips_header_single_data_row() {
        let content = "\
## Phase Progress

| Pass | Status | Notes |
|------|--------|-------|
| P-1 | COMPLETE | the only data row |
";
        let rows = rows_after_heading(content, "## Phase Progress");
        assert_eq!(rows.len(), 1, "exactly one DATA row (header dropped)");
        assert!(
            rows[0].contains("the only data row"),
            "the returned row must be the data row, not the header; got: {}",
            rows[0]
        );
        assert!(
            !rows[0].contains("Pass | Status | Notes"),
            "header row must not be returned as a data row"
        );
    }

    /// F-003: a header-only section (no data rows) returns an EMPTY vec — the header is
    /// never promoted to a data row even when it is the only `|`-row after the separator.
    #[test]
    fn test_F003_rows_after_heading_header_only_section_returns_empty() {
        let content = "\
## Phase Progress

| Pass | Status | Notes |
|------|--------|-------|

## Next Section
";
        let rows = rows_after_heading(content, "## Phase Progress");
        assert!(
            rows.is_empty(),
            "header-only section must yield no data rows; got {} rows",
            rows.len()
        );
    }

    /// F-003 regression guard: the fail-state-phase-progress fixture form (tail in the
    /// HEADER row, no tail in the data row) must still be detected as MISSING — the
    /// bottommost DATA row is what the inv-4 check sees, not the header.
    #[test]
    fn test_F003_phase_progress_tail_in_header_not_data_row_is_missing() {
        let content = "\
---
document_type: state
current_step: \"Phase 3 — trajectory-tail →9→9→9→9; OK\"
current_cycle: \"v1.0-brownfield-backfill\"
---

| **Last Updated** | 2026-05-28 — trajectory-tail →9→9→9→9; good |

## Phase Progress

| Pass | Status | trajectory-tail →9→9→9→9 |
|------|--------|-------|
| P-1  | COMPLETE | no tail in this data row |

## Concurrent Cycles

| Cycle | Status | Notes |
|-------|--------|-------|
| v1.0 | active | trajectory-tail →9→9→9→9 |

## Session Resume Checkpoint (2026-05-28)

### §1. Where We Are

trajectory-tail →9→9→9→9; session resume OK.
";
        let control = content.replace(
            "| P-1  | COMPLETE | no tail in this data row |",
            "| P-1  | COMPLETE | trajectory-tail →9→9→9→9 |",
        );
        eprintln!(
            "DBG_F003 control (tail in data row) missing: {:?}",
            check_state_md(&control)
                .iter()
                .map(|s| s.site_name)
                .collect::<Vec<_>>()
        );
        let missing = check_state_md(content);
        eprintln!(
            "DBG_F003 missing sites: {:?}",
            missing.iter().map(|s| s.site_name).collect::<Vec<_>>()
        );
        // The header row carries a valid tail but the DATA row does not. Because
        // bottommost-DATA-row selection is what the inv-4 check sees (F-003), Phase
        // Progress MUST be reported missing — proving the header is not promoted to a
        // data row.
        assert!(
            missing
                .iter()
                .any(|s| s.site_name.contains("Phase Progress")),
            "tail in header (not data row) must still flag Phase Progress as missing; got: {:?}",
            missing.iter().map(|s| s.site_name).collect::<Vec<_>>()
        );
    }

    // -----------------------------------------------------------------------
    // F-006: is_separator_row must NOT misclassify thin/empty data rows
    // -----------------------------------------------------------------------

    /// F-006: a genuine separator row is classified as a separator.
    #[test]
    fn test_F006_is_separator_row_accepts_canonical_separators() {
        assert!(is_separator_row("|------|--------|-------|"));
        assert!(is_separator_row("| :--- | ---: | :---: |"));
        assert!(is_separator_row("|---|"));
    }

    /// F-006: an all-whitespace/empty data row (`|   |   |`) is NOT a separator and must
    /// therefore survive as a data row (it has no dashes).
    #[test]
    fn test_F006_is_separator_row_rejects_thin_empty_data_row() {
        assert!(
            !is_separator_row("|   |   |"),
            "all-whitespace data row must NOT be classified as a separator"
        );
        assert!(
            !is_separator_row("|  |  |  |"),
            "empty-cell data row must NOT be classified as a separator"
        );
    }

    /// F-006: a thin data row survives `rows_after_heading` (not dropped as a separator),
    /// so it can be selected as the bottommost row by the extractors.
    #[test]
    fn test_F006_thin_data_row_survives_rows_after_heading() {
        let content = "\
## Concurrent Cycles

| Cycle | Status | Notes |
|-------|--------|-------|
|   |   |   |
";
        let rows = rows_after_heading(content, "## Concurrent Cycles");
        assert_eq!(
            rows.len(),
            1,
            "the thin data row must survive (not be dropped as a separator)"
        );
        assert!(
            rows[0].contains('|'),
            "the surviving row is the thin data row"
        );
    }

    /// F-006: a row of text containing dashes but also other chars is NOT a separator.
    #[test]
    fn test_F006_is_separator_row_rejects_text_with_dashes() {
        assert!(
            !is_separator_row("| a-b | c-d |"),
            "cells with non-dash content must not be a separator"
        );
    }

    // -----------------------------------------------------------------------
    // F-008: extract_current_cycle unquotes BEFORE stripping `#`-comment
    // -----------------------------------------------------------------------

    /// F-008: a quoted value containing `#` is NOT truncated (unquote before comment-strip).
    #[test]
    fn test_F008_extract_current_cycle_quoted_value_with_hash_not_truncated() {
        let content = "---\ncurrent_cycle: \"v1.0#hotfix\"\n---\n";
        let result = extract_current_cycle(content);
        assert_eq!(
            result,
            Some("v1.0#hotfix".to_string()),
            "quoted value containing '#' must be preserved intact"
        );
    }

    /// F-008: single-quoted value containing `#` is preserved intact.
    #[test]
    fn test_F008_extract_current_cycle_single_quoted_value_with_hash() {
        let content = "---\ncurrent_cycle: 'v1.0#hotfix'\n---\n";
        let result = extract_current_cycle(content);
        assert_eq!(result, Some("v1.0#hotfix".to_string()));
    }

    /// F-008: a BARE value with a trailing `#`-comment still strips the comment.
    #[test]
    fn test_F008_extract_current_cycle_bare_value_strips_trailing_comment() {
        let content = "---\ncurrent_cycle: v1.0-brownfield-backfill # active cycle\n---\n";
        let result = extract_current_cycle(content);
        assert_eq!(
            result,
            Some("v1.0-brownfield-backfill".to_string()),
            "bare-form trailing '#'-comment must be stripped"
        );
    }

    // -----------------------------------------------------------------------
    // F-001: check_index_sites scopes site 7 to the Adversarial Reviews table
    // -----------------------------------------------------------------------

    /// F-001: when the `## Adversarial Reviews` bottommost data row HAS a tail but a
    /// LATER table (Convergence Status) lacks one, the adv-table site must be considered
    /// PRESENT — proving site 7 is scoped to the Adversarial Reviews table, not the
    /// file-global bottommost `|`-row.
    #[test]
    fn test_F001_index_adv_table_scoped_not_global_bottommost_row() {
        let content = "\
# Cycle Index

## Adversarial Reviews

| Pass | Date | Findings | Status |
|------|------|----------|--------|
| 1 | 2026-05-01 | 9 | early pass no tail |
| 2 | 2026-05-02 | 4 | trajectory-tail →9→9→9→9; latest pass present |

## Convergence Status

| Field | Value |
|-------|-------|
| Convergence Status | 3/3 CONVERGED no tail here at all |
";
        let warnings = check_index_sites(content);
        // The adv-table latest row HAS a tail → no adv-table warning.
        assert!(
            !warnings
                .iter()
                .any(|w| w.message.contains("adv-table latest row")),
            "adv-table site must be PRESENT (scoped to Adversarial Reviews table); \
             warnings={warnings:?}"
        );
        // The Convergence Status DATA row lacks a tail → its own warning IS present.
        assert!(
            warnings
                .iter()
                .any(|w| w.message.contains("Convergence Status")),
            "Convergence Status site must be flagged missing; warnings={warnings:?}"
        );
    }

    /// F-001: when the Adversarial Reviews bottommost data row LACKS a tail, the
    /// adv-table site is flagged missing — even if a later table row has a valid tail.
    #[test]
    fn test_F001_index_adv_table_missing_tail_in_latest_pass_row_flagged() {
        let content = "\
# Cycle Index

## Adversarial Reviews

| Pass | Date | Findings | Status |
|------|------|----------|--------|
| 1 | 2026-05-01 | 9 | trajectory-tail →9→9→9→9; substantive |
| 2 | 2026-05-02 | 4 | NITPICK — no tail here at all |

## Convergence Status

| Field | Value |
|-------|-------|
| Convergence Status | trajectory-tail →9→9→9→9; 3/3 CONVERGED |
";
        let warnings = check_index_sites(content);
        assert!(
            warnings
                .iter()
                .any(|w| w.message.contains("adv-table latest row")),
            "adv-table latest-pass row lacks a tail → must be flagged; warnings={warnings:?}"
        );
        // Convergence Status DATA row HAS a tail → no Convergence Status warning.
        assert!(
            !warnings
                .iter()
                .any(|w| w.message.contains("Convergence Status")),
            "Convergence Status site must be PRESENT; warnings={warnings:?}"
        );
    }

    /// F-001: an INDEX.md with NO `## Adversarial Reviews` table fails open for site 7
    /// (no adv-table warning) — the section may not exist on a fresh cycle index.
    #[test]
    fn test_F001_index_no_adv_table_section_fails_open() {
        let content = "\
# Cycle Index

## Convergence Status

| Metric | Value |
|--------|-------|
| Streak | trajectory-tail →9→9→9→9; 3/3 CONVERGED |
";
        let warnings = check_index_sites(content);
        assert!(
            !warnings
                .iter()
                .any(|w| w.message.contains("adv-table latest row")),
            "absent Adversarial Reviews table → adv-table site fails open; warnings={warnings:?}"
        );
    }

    // -----------------------------------------------------------------------
    // F-005: decode_read_result pure seam (read-error + UTF-8 decode branches)
    // -----------------------------------------------------------------------

    /// F-005: a HostError::OutputTooLarge read result maps to a fail-open advisory.
    /// This is the unit test the bats comment previously CLAIMED existed but did not.
    #[test]
    fn test_F005_decode_read_result_output_too_large_maps_to_advisory() {
        use vsdd_hook_sdk::host::HostError;
        let result: Result<Vec<u8>, HostError> = Err(HostError::OutputTooLarge);
        let mapped = decode_read_result(".factory/STATE.md", result);
        let advisory = mapped.expect_err("OutputTooLarge must map to Err(advisory)");
        assert!(
            advisory.contains("read failed"),
            "advisory must describe a read failure; got: {advisory}"
        );
        assert!(
            advisory.contains("fail-open"),
            "advisory must mention fail-open; got: {advisory}"
        );
    }

    /// F-005: other HostError variants also map to a fail-open advisory.
    #[test]
    fn test_F005_decode_read_result_capability_denied_maps_to_advisory() {
        use vsdd_hook_sdk::host::HostError;
        let result: Result<Vec<u8>, HostError> = Err(HostError::CapabilityDenied);
        assert!(
            decode_read_result(".factory/STATE.md", result).is_err(),
            "CapabilityDenied must map to Err(advisory)"
        );
    }

    /// F-005: invalid UTF-8 bytes map to a fail-open advisory (EC-020 / inv-13).
    #[test]
    fn test_F005_decode_read_result_invalid_utf8_maps_to_advisory() {
        use vsdd_hook_sdk::host::HostError;
        // 0xFF is never valid in UTF-8.
        let result: Result<Vec<u8>, HostError> = Ok(vec![0x2d, 0x2d, 0x2d, 0xFF, 0xFE]);
        let advisory =
            decode_read_result(".factory/STATE.md", result).expect_err("invalid UTF-8 → Err");
        assert!(
            advisory.contains("invalid UTF-8"),
            "advisory must mention invalid UTF-8; got: {advisory}"
        );
        assert!(
            advisory.contains("EC-020"),
            "advisory must cite EC-020 fail-open; got: {advisory}"
        );
    }

    /// F-005: valid UTF-8 bytes decode to the expected String (Ok path).
    #[test]
    fn test_F005_decode_read_result_valid_utf8_decodes_ok() {
        use vsdd_hook_sdk::host::HostError;
        let result: Result<Vec<u8>, HostError> = Ok("hello →9".as_bytes().to_vec());
        let decoded =
            decode_read_result(".factory/STATE.md", result).expect("valid UTF-8 must decode");
        assert_eq!(decoded, "hello →9");
    }
}
