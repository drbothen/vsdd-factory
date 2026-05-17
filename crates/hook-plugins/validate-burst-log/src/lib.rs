//! validate-burst-log — PostToolUse WASM hook plugin.
//!
//! Blocks any Edit/Write to a `burst-log.md` file that leaves a structurally
//! incomplete latest burst entry. Validates three structural properties:
//!
//! 1. **h2 heading format** (D-421(e)+D-438(d)+D-439(a)): the latest h2 heading
//!    must match `## Burst: <description> (YYYY-MM-DD)`.
//!
//! 2. **9-block completeness** (D-444(c)+D-446(a)): all 9 required bold-heading
//!    block types must be present in the latest burst entry.
//!
//! 3. **Dim-1 cardinality parity** (D-432(e)+D-448(d)(i)): the integer in the
//!    Dim-1 headline must equal the count of files in the Dim-1 list.
//!
//! # Behavioral Contracts
//!
//! - BC-5.39.004: blocks structurally incomplete burst-log entries.
//!
//! # D-NNN closures
//!
//! - D-421(e): burst-log h2 heading form `## Burst: .+ (YYYY-MM-DD)` enforced.
//! - D-438(d): canonical h2 form from pass 38 forward (same regex as D-421(e)).
//! - D-439(a): h2 enforcement per D-421(e).
//! - D-444(c): 9 required block types in every burst entry.
//! - D-446(a): own-burst entry completeness gate.
//! - D-432(e): Dim-1 headline integer must equal enumerated list count.
//! - D-448(d)(i): Dim-1 cardinality parity source-attestation gate.
//! - D-443(e)(ii): own-burst h2 present at Commit A (real-time gate).
//!
//! # Architecture compliance
//!
//! - HOST_ABI_VERSION = 1 (no new host functions introduced).
//! - Fail-open on every `host::read_file` error (BC-5.39.004 invariant 5).
//! - No `println!` — use `host::log_*` for all diagnostic output.
//! - No `unwrap()` or `expect()` in production paths.
//! - No `regex` crate: hand-rolled pattern scanning to stay within WASM fuel budget.
//! - File-path enforcement via in-plugin guard (Q5/Q6 canonical pattern);
//!   registry entry does NOT include a `file_pattern` field.

use vsdd_hook_sdk::{HookPayload, HookResult};

/// HOST_ABI_VERSION declares the ABI contract version this plugin was built
/// against. Must remain 1.
pub const HOST_ABI_VERSION: u32 = 1;

// ---------------------------------------------------------------------------
// Required block types (D-444(c))
// ---------------------------------------------------------------------------

/// The 9 required block type bold-heading tokens per D-444(c).
/// Dim-2/5/6/7 use prefix-match (the check uses `contains("**Dim-N")`) to
/// accommodate attestation-suffix variants like `**Dim-2 Attestation**`.
const REQUIRED_BLOCK_TOKENS: &[&str] = &[
    "**Parent-commit",
    "**Adversary verdict",
    "**Files touched (Dim-1)",
    "**Codifications",
    "**Dim-2",
    "**Dim-5",
    "**Dim-6",
    "**Dim-7",
    "**Closes",
];

/// Human-readable names corresponding to `REQUIRED_BLOCK_TOKENS`, used in
/// violation messages.
const REQUIRED_BLOCK_NAMES: &[&str] = &[
    "Parent-commit",
    "Adversary verdict",
    "Files touched (Dim-1)",
    "Codifications",
    "Dim-2",
    "Dim-5",
    "Dim-6",
    "Dim-7",
    "Closes",
];

// ---------------------------------------------------------------------------
// Violation types
// ---------------------------------------------------------------------------

/// A structural violation found in the latest burst entry.
#[derive(Debug, Clone)]
pub struct Violation {
    /// Human-readable description of the violation, used in the block message.
    pub description: String,
    /// The raw body-literal form of the offending text (e.g. malformed h2 line,
    /// Dim-1 headline text). Structural plumbing per TD-VSDD-059 paper-fix avoidance.
    /// Enables block message to quote the exact string the author wrote.
    pub cited_raw: String,
}

// ---------------------------------------------------------------------------
// Pure logic functions (no I/O)
// ---------------------------------------------------------------------------

/// Find the latest `## Burst:` h2 heading in `content` and return a slice
/// of the content from that heading to the next `## ` heading or end-of-file.
///
/// Returns `None` if no `## Burst:` line is found — the caller treats this
/// as a missing-h2 violation.
///
/// # BC trace
/// BC-5.39.004 invariant 4 — only the latest burst entry is validated.
pub fn extract_latest_burst(content: &str) -> Option<(usize, usize)> {
    // Find the LAST occurrence of a line starting with "## Burst:".
    // Hand-rolled line scan: iterate line-by-line tracking byte offsets.
    let mut last_start: Option<usize> = None;
    let mut pos = 0usize;

    for line in content.split('\n') {
        let line_start = pos;
        pos += line.len() + 1; // +1 for the '\n' separator

        let trimmed = line.trim_end_matches('\r');
        if trimmed.starts_with("## Burst:") {
            last_start = Some(line_start);
        }
    }

    let start = last_start?;

    // Find the end: next `## ` heading after `start`, or EOF.
    // Skip past the first character so we don't re-match the same heading.
    let after_start = start + 1;
    let rest = &content[after_start..];

    // Scan for the next h2 heading boundary by looking at line starts.
    let mut end_offset: Option<usize> = None;
    let mut rest_pos = 0usize;
    for line in rest.split('\n') {
        let trimmed = line.trim_end_matches('\r');
        // A new h2 heading terminates the current entry.
        if trimmed.starts_with("## ") {
            end_offset = Some(after_start + rest_pos);
            break;
        }
        rest_pos += line.len() + 1;
    }

    let end = end_offset.unwrap_or(content.len());
    Some((start, end))
}

/// Validate the h2 heading line against the canonical pattern:
/// `^## Burst: .+ \(\d{4}-\d{2}-\d{2}\)$`
///
/// Returns `true` if the heading is valid, `false` otherwise.
///
/// # BC trace
/// BC-5.39.004 postcondition 2 — malformed h2 emits BlockWithFix.
pub fn validate_h2_heading(h2_line: &str) -> bool {
    let line = h2_line.trim_end_matches('\r').trim_end();

    // Must start with "## Burst: "
    let after_prefix = match line.strip_prefix("## Burst: ") {
        Some(rest) => rest,
        None => return false,
    };

    // Must not be empty after the prefix
    if after_prefix.is_empty() {
        return false;
    }

    // Must end with ` (YYYY-MM-DD)` where YYYY/MM/DD are 4/2/2 digits.
    // Find the last '(' to locate the parenthesized date.
    let last_paren = match after_prefix.rfind('(') {
        Some(i) => i,
        None => return false,
    };

    // The character before '(' should be a space (or the start of content).
    if last_paren > 0 {
        let before = &after_prefix[last_paren - 1..last_paren];
        if before != " " {
            return false;
        }
    }

    let inside = &after_prefix[last_paren + 1..];

    // Must match YYYY-MM-DD)
    // Exactly: 4 digits, '-', 2 digits, '-', 2 digits, ')'
    let bytes = inside.as_bytes();
    if bytes.len() < 11 {
        return false;
    }

    // YYYY
    if !bytes[..4].iter().all(|b| b.is_ascii_digit()) {
        return false;
    }
    if bytes[4] != b'-' {
        return false;
    }
    // MM
    if !bytes[5..7].iter().all(|b| b.is_ascii_digit()) {
        return false;
    }
    if bytes[7] != b'-' {
        return false;
    }
    // DD
    if !bytes[8..10].iter().all(|b| b.is_ascii_digit()) {
        return false;
    }
    if bytes[10] != b')' {
        return false;
    }

    // The description before '(' must be non-empty (at least 1 char + space).
    // `after_prefix[..last_paren]` = description + trailing space.
    // If last_paren == 0 there's no description.
    if last_paren == 0 {
        return false;
    }
    let description_part = after_prefix[..last_paren].trim_end();
    if description_part.is_empty() {
        return false;
    }

    true
}

/// Scan `burst_content` for all 9 required bold-heading block types.
/// Returns a `Vec` of missing block name strings. Empty Vec means all present.
///
/// Uses prefix-match (`contains("**Dim-N")`) for Dim-2/5/6/7 tokens to
/// accommodate `**Dim-2 Attestation**`-style variants per BC-5.39.004 invariant 3.
///
/// # BC trace
/// BC-5.39.004 postcondition 3 — missing blocks named in BlockWithFix message.
pub fn check_block_presence(burst_content: &str) -> Vec<&'static str> {
    let mut missing = Vec::new();
    for (i, &token) in REQUIRED_BLOCK_TOKENS.iter().enumerate() {
        if !burst_content.contains(token) {
            missing.push(REQUIRED_BLOCK_NAMES[i]);
        }
    }
    missing
}

/// Check Dim-1 headline integer against the count of list items in the Dim-1 block body.
///
/// Returns `Some((headline_count, list_count, raw_dim1_line))` if they differ,
/// `None` if equal or if the Dim-1 block is absent.
///
/// Handles list prefixes: `- `, `* `, and `N. ` (numbered lists).
///
/// # BC trace
/// BC-5.39.004 postcondition 4 — Dim-1 mismatch emits BlockWithFix naming both counts.
pub fn check_dim1_cardinality(burst_content: &str) -> Option<(usize, usize, String)> {
    // Find the Dim-1 block: look for a line containing "**Files touched (Dim-1)"
    let dim1_token = "**Files touched (Dim-1)";

    let lines: Vec<&str> = burst_content.split('\n').collect();
    let mut dim1_line_idx: Option<usize> = None;

    for (i, line) in lines.iter().enumerate() {
        if line.contains(dim1_token) {
            dim1_line_idx = Some(i);
            break;
        }
    }

    let dim1_idx = dim1_line_idx?;
    let dim1_line = lines[dim1_idx];

    // Extract the headline integer: find `(\d+) unique files` on the Dim-1 header line
    // or the immediately following line.
    let headline_count = extract_dim1_headline_count(dim1_line).or_else(|| {
        lines
            .get(dim1_idx + 1)
            .and_then(|l| extract_dim1_headline_count(l))
    })?;

    // Count list items in the Dim-1 block body (lines between Dim-1 header and
    // the next bold-header or end-of-burst).
    let list_count = count_dim1_list_items(&lines, dim1_idx + 1);

    if headline_count != list_count {
        Some((headline_count, list_count, dim1_line.trim_end().to_string()))
    } else {
        None
    }
}

/// Extract the integer N from a line containing "N unique files".
/// Hand-rolled: scan for digits immediately before " unique files".
fn extract_dim1_headline_count(line: &str) -> Option<usize> {
    let marker = " unique files";
    let marker_pos = line.find(marker)?;
    // Walk backwards from marker_pos to find digits.
    let before = &line[..marker_pos];
    let digit_end = before.len();
    let mut digit_start = digit_end;
    for b in before.bytes().rev() {
        if b.is_ascii_digit() {
            digit_start -= 1;
        } else {
            break;
        }
    }
    if digit_start == digit_end {
        return None; // No digits found
    }
    let digits = &before[digit_start..digit_end];
    digits.parse::<usize>().ok()
}

/// Count list items in `lines` starting at `start_idx`, until the next
/// bold-header line (starting with `**`) or the end of the lines slice.
/// Counts lines starting with `- `, `* `, or `\d+. `.
fn count_dim1_list_items(lines: &[&str], start_idx: usize) -> usize {
    let mut count = 0usize;
    for line in lines.iter().skip(start_idx) {
        let trimmed = line.trim_end_matches('\r').trim();
        // Next bold-header terminates the Dim-1 block.
        if trimmed.starts_with("**") && trimmed.ends_with("**") {
            break;
        }
        if trimmed.starts_with("**") {
            break;
        }
        // Count list item prefixes.
        if trimmed.starts_with("- ") || trimmed.starts_with("* ") || is_numbered_list_item(trimmed)
        {
            count += 1;
        }
    }
    count
}

/// Returns true if `s` starts with a numbered list prefix like `1. `, `12. `, etc.
fn is_numbered_list_item(s: &str) -> bool {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i == 0 {
        return false;
    }
    // After digits: expect ". "
    i + 1 < bytes.len() && bytes[i] == b'.' && bytes[i + 1] == b' '
}

// ---------------------------------------------------------------------------
// Block message formatting
// ---------------------------------------------------------------------------

/// Format a list of violations into a `HookResult::block_with_fix`.
fn emit_block(violations: &[Violation]) -> HookResult {
    let lines: Vec<String> = violations
        .iter()
        .map(|v| format!("  - {}", v.description))
        .collect();
    let reason = format!(
        "validate-burst-log: {} violation(s) in latest burst entry:\n{}",
        violations.len(),
        lines.join("\n")
    );
    HookResult::block_with_fix(
        "validate-burst-log",
        reason,
        "Fix the violations listed above before re-writing burst-log.md",
        "BURST_LOG_STRUCTURAL_VIOLATION",
    )
}

// ---------------------------------------------------------------------------
// Hook entry point
// ---------------------------------------------------------------------------

/// Core hook logic for validate-burst-log.
///
/// Called from the WASI entry point in `main.rs` via the SDK trampoline.
///
/// 1. Extracts `file_path` from `tool_input`; early-exit Continue for
///    non-burst-log.md paths (Q5/Q6 in-plugin guard).
/// 2. Reads the written burst-log.md content from the host filesystem via
///    `host::read_file`. On failure, emits Continue + log_warn (fail-open per
///    BC-5.39.004 invariant 5 + postcondition 6).
/// 3. Validates: h2 heading format, 9-block presence, Dim-1 cardinality.
/// 4. Emits `HookResult::block_with_fix` if any violation found, or
///    `HookResult::Continue` if all properties hold.
///
/// # BC trace
/// BC-5.39.004 postconditions 1-6; invariants 1-5.
pub fn on_post_tool_use(payload: HookPayload) -> HookResult {
    use vsdd_hook_sdk::host;

    // Extract file_path from tool_input.
    let file_path = match payload.tool_input.get("file_path").and_then(|v| v.as_str()) {
        Some(p) => p.to_string(),
        None => {
            host::log_warn(
                "[validate-burst-log] file_path absent from tool_input — graceful degrade",
            );
            return HookResult::Continue;
        }
    };

    // In-plugin file-path guard (Q5/Q6 canonical pattern):
    // only act on writes to burst-log.md files.
    if !file_path.ends_with("burst-log.md") {
        return HookResult::Continue;
    }

    // Read the burst-log.md content that was just written.
    // Use the file_path from the envelope directly — it is the canonical path
    // to the file that was just written.
    // On read failure: fail-open (Continue + log_warn) per BC-5.39.004 postcondition 6.
    let content = match host::read_file(&file_path, 65536, 2000) {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(s) => s,
            Err(e) => {
                host::log_warn(&format!(
                    "[validate-burst-log] UTF-8 decode failure reading {file_path}: {e}"
                ));
                return HookResult::Continue;
            }
        },
        Err(e) => {
            host::log_warn(&format!(
                "[validate-burst-log] read_file failed for {file_path}: {e:?}"
            ));
            return HookResult::Continue;
        }
    };

    let mut violations: Vec<Violation> = Vec::new();

    // Find the latest burst entry range.
    match extract_latest_burst(&content) {
        None => {
            // No `## Burst:` heading found — h2 is absent.
            violations.push(Violation {
                description:
                    "h2 heading does not match canonical format '## Burst: <desc> (YYYY-MM-DD)'; \
                     no '## Burst:' heading found in burst-log.md (D-421(e)+D-443(e)(ii))"
                        .to_string(),
                cited_raw: String::new(),
            });
            // Without an h2, we cannot scope to a burst entry, so also report
            // all 9 blocks missing against the full content.
            let missing_blocks = check_block_presence(&content);
            for block_name in &missing_blocks {
                violations.push(Violation {
                    description: format!(
                        "Required block '**{block_name}**' not found in latest burst entry (D-444(c))"
                    ),
                    cited_raw: String::new(),
                });
            }
        }
        Some((burst_start, burst_end)) => {
            let burst_content = &content[burst_start..burst_end];

            // Extract the h2 heading line (first line of the burst entry).
            let h2_line = burst_content
                .split('\n')
                .next()
                .unwrap_or("")
                .trim_end_matches('\r');

            // Validate h2 heading format.
            if !validate_h2_heading(h2_line) {
                violations.push(Violation {
                    description: format!(
                        "h2 heading does not match canonical format '## Burst: <desc> (YYYY-MM-DD)'; \
                         found: '{}' (D-421(e)+D-438(d)+D-439(a))",
                        h2_line.trim_end()
                    ),
                    cited_raw: h2_line.trim_end().to_string(),
                });
            }

            // Check 9-block presence.
            let missing_blocks = check_block_presence(burst_content);
            for block_name in &missing_blocks {
                violations.push(Violation {
                    description: format!(
                        "Required block '**{block_name}**' not found in latest burst entry (D-444(c))"
                    ),
                    cited_raw: String::new(),
                });
            }

            // Check Dim-1 cardinality parity.
            if let Some((headline_count, list_count, dim1_raw)) =
                check_dim1_cardinality(burst_content)
            {
                violations.push(Violation {
                    description: format!(
                        "Dim-1 cardinality mismatch: headline states {headline_count} unique files \
                         but enumerated list has {list_count} items (D-432(e)+D-448(d)(i))"
                    ),
                    cited_raw: dim1_raw,
                });
            }
        }
    }

    if violations.is_empty() {
        HookResult::Continue
    } else {
        emit_block(&violations)
    }
}

// ---------------------------------------------------------------------------
// Unit tests — BC-5.39.004
// ---------------------------------------------------------------------------

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    // ── validate_h2_heading ──────────────────────────────────────────────────

    #[test]
    fn test_BC_5_39_004_h2_valid_format_returns_true() {
        assert!(validate_h2_heading(
            "## Burst: Pass-41 fix burst (2026-05-12)"
        ));
        assert!(validate_h2_heading(
            "## Burst: Pass-40 current fix burst (2026-05-16)"
        ));
        assert!(validate_h2_heading("## Burst: some desc (2026-01-01)"));
    }

    #[test]
    fn test_BC_5_39_004_h2_wrong_prefix_returns_false() {
        // EC-002: wrong prefix
        assert!(!validate_h2_heading(
            "## Fix Burst: Pass-44 description without parenthesized date"
        ));
        assert!(!validate_h2_heading("# Burst: single-hash (2026-05-12)"));
        assert!(!validate_h2_heading("### Burst: triple-hash (2026-05-12)"));
    }

    #[test]
    fn test_BC_5_39_004_h2_no_date_parentheses_returns_false() {
        // EC-010: no parenthesized date
        assert!(!validate_h2_heading("## Burst: description"));
        assert!(!validate_h2_heading("## Burst: description 2026-05-12"));
    }

    #[test]
    fn test_BC_5_39_004_h2_malformed_date_returns_false() {
        // Date digits incorrect
        assert!(!validate_h2_heading("## Burst: desc (2026-5-12)"));
        assert!(!validate_h2_heading("## Burst: desc (202X-05-12)"));
        assert!(!validate_h2_heading("## Burst: desc (2026-05)"));
    }

    #[test]
    fn test_BC_5_39_004_h2_empty_description_returns_false() {
        // No description between "## Burst: " and " (YYYY-MM-DD)"
        assert!(!validate_h2_heading("## Burst: (2026-05-12)"));
    }

    // ── check_block_presence ─────────────────────────────────────────────────

    #[test]
    fn test_BC_5_39_004_all_9_blocks_present_returns_empty_vec() {
        let content = concat!(
            "## Burst: test (2026-05-12)\n",
            "**Parent-commit:** abc\n",
            "**Adversary verdict:** NITPICK\n",
            "**Files touched (Dim-1): 1 unique files**\n",
            "- file.rs\n",
            "**Codifications:** D-444(c)\n",
            "**Dim-2 Attestation:** done\n",
            "**Dim-5 Attestation:** done\n",
            "**Dim-6 Attestation:** done\n",
            "**Dim-7 Attestation:** done\n",
            "**Closes:** D-444(c)\n",
        );
        let missing = check_block_presence(content);
        assert!(
            missing.is_empty(),
            "all 9 blocks present; expected empty missing vec, got: {missing:?}"
        );
    }

    #[test]
    fn test_BC_5_39_004_6_blocks_present_names_dim2_dim5_dim6_as_missing() {
        // Fixture: Parent-commit, Adversary verdict, Files touched (Dim-1),
        // Codifications, Dim-7, Closes — missing Dim-2, Dim-5, Dim-6
        let content = concat!(
            "## Burst: test (2026-05-12)\n",
            "**Parent-commit:** abc\n",
            "**Adversary verdict:** HIGH\n",
            "**Files touched (Dim-1): 2 unique files**\n",
            "- a.rs\n",
            "- b.rs\n",
            "**Codifications:** partial\n",
            "**Dim-7 Attestation:** done\n",
            "**Closes:** D-421(e)\n",
        );
        let missing = check_block_presence(content);
        assert!(missing.contains(&"Dim-2"), "expected Dim-2 missing");
        assert!(missing.contains(&"Dim-5"), "expected Dim-5 missing");
        assert!(missing.contains(&"Dim-6"), "expected Dim-6 missing");
        assert_eq!(missing.len(), 3, "expected exactly 3 missing blocks");
    }

    #[test]
    fn test_BC_5_39_004_closes_missing_names_closes_in_vec() {
        // EC-004: 8 of 9 blocks present; Closes absent
        let content = concat!(
            "**Parent-commit:** abc\n",
            "**Adversary verdict:** NITPICK\n",
            "**Files touched (Dim-1): 1 unique files**\n",
            "- file.rs\n",
            "**Codifications:** done\n",
            "**Dim-2 Attestation:** done\n",
            "**Dim-5 Attestation:** done\n",
            "**Dim-6 Attestation:** done\n",
            "**Dim-7 Attestation:** done\n",
        );
        let missing = check_block_presence(content);
        assert!(missing.contains(&"Closes"), "expected Closes in missing");
        assert_eq!(missing.len(), 1);
    }

    // ── check_dim1_cardinality ───────────────────────────────────────────────

    #[test]
    fn test_BC_5_39_004_dim1_headline_5_list_7_returns_mismatch() {
        // EC-005: headline 5, list has 7
        let content = concat!(
            "**Files touched (Dim-1): 5 unique files**\n",
            "- a.rs\n",
            "- b.rs\n",
            "- c.rs\n",
            "- d.rs\n",
            "- e.rs\n",
            "- f.rs\n",
            "- g.rs\n",
            "**Codifications:** done\n",
        );
        let result = check_dim1_cardinality(content);
        assert!(result.is_some(), "expected mismatch");
        let (headline, list, _raw) = result.unwrap();
        assert_eq!(headline, 5);
        assert_eq!(list, 7);
    }

    #[test]
    fn test_BC_5_39_004_dim1_headline_7_list_7_returns_none() {
        // EC-006: headline 7, list 7 — no mismatch
        let content = concat!(
            "**Files touched (Dim-1): 7 unique files**\n",
            "- a.rs\n",
            "- b.rs\n",
            "- c.rs\n",
            "- d.rs\n",
            "- e.rs\n",
            "- f.rs\n",
            "- g.rs\n",
            "**Codifications:** done\n",
        );
        let result = check_dim1_cardinality(content);
        assert!(result.is_none(), "expected no mismatch for equal counts");
    }

    #[test]
    fn test_BC_5_39_004_dim1_headline_3_list_3_returns_none() {
        let content = concat!(
            "**Files touched (Dim-1): 3 unique files**\n",
            "- a.rs\n",
            "- b.rs\n",
            "- c.rs\n",
            "**Codifications:** done\n",
        );
        let result = check_dim1_cardinality(content);
        assert!(result.is_none());
    }

    #[test]
    fn test_BC_5_39_004_dim1_numbered_list_items_counted_correctly() {
        // Numbered list items should also be counted
        let content = concat!(
            "**Files touched (Dim-1): 3 unique files**\n",
            "1. a.rs\n",
            "2. b.rs\n",
            "3. c.rs\n",
            "**Codifications:** done\n",
        );
        let result = check_dim1_cardinality(content);
        assert!(
            result.is_none(),
            "numbered list of 3 with headline 3 = no mismatch"
        );
    }

    // ── extract_latest_burst ─────────────────────────────────────────────────

    #[test]
    fn test_BC_5_39_004_extract_latest_burst_single_entry() {
        let content = "## Burst: Pass-41 fix burst (2026-05-12)\n\n**Parent-commit:** abc\n";
        let result = extract_latest_burst(content);
        assert!(result.is_some());
        let (start, end) = result.unwrap();
        let slice = &content[start..end];
        assert!(slice.starts_with("## Burst:"));
    }

    #[test]
    fn test_BC_5_39_004_extract_latest_burst_two_entries_returns_second() {
        let content = concat!(
            "## Burst: Pass-39 old burst (2026-04-28)\n",
            "**Parent-commit:** old\n",
            "\n",
            "## Burst: Pass-40 new burst (2026-05-16)\n",
            "**Parent-commit:** new\n",
        );
        let result = extract_latest_burst(content);
        assert!(result.is_some());
        let (start, _end) = result.unwrap();
        let slice = &content[start..];
        assert!(slice.contains("Pass-40"), "should return the LATEST burst");
        assert!(
            !slice.contains("Pass-39"),
            "old entry should not be in latest slice"
        );
    }

    #[test]
    fn test_BC_5_39_004_extract_latest_burst_no_h2_returns_none() {
        let content = "**Parent-commit:** abc\n**Closes:** done\n";
        let result = extract_latest_burst(content);
        assert!(result.is_none());
    }

    // ── emit_block ───────────────────────────────────────────────────────────

    #[test]
    fn test_BC_5_39_004_emit_block_names_violations_in_message() {
        let violations = vec![
            Violation {
                description: "Required block '**Dim-2**' not found (D-444(c))".to_string(),
                cited_raw: String::new(),
            },
            Violation {
                description: "Required block '**Dim-5**' not found (D-444(c))".to_string(),
                cited_raw: String::new(),
            },
        ];
        let result = emit_block(&violations);
        match &result {
            HookResult::Block { reason } => {
                assert!(reason.contains("Dim-2"), "reason must mention Dim-2");
                assert!(reason.contains("Dim-5"), "reason must mention Dim-5");
            }
            _ => panic!("expected Block result, got {result:?}"),
        }
    }

    #[test]
    fn test_BC_5_39_004_emit_block_exit_code_2() {
        let violations = vec![Violation {
            description: "test".to_string(),
            cited_raw: String::new(),
        }];
        let result = emit_block(&violations);
        assert_eq!(result.exit_code(), 2);
    }
}
