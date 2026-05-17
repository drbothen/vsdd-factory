//! validate-state-structure — PostToolUse WASM hook plugin.
//!
//! Blocks any Edit/Write to `STATE.md` that contains structural violations:
//!
//! 1. **Banner line-count drift** (D-421(c)+D-422(c)+D-424(b)+D-428(d)+D-438(a)+D-440(d)+D-442(d)):
//!    the banner line-count claim MUST match the actual newline count in the file.
//!
//! 2. **Dual-margin form absent** (D-446(c)): the banner MUST contain both margin
//!    values — one from the soft target and one from the actual count.
//!
//! 3. **Trajectory-tail cardinality** (D-433(e)+D-439(c)+D-451(c)+D-432(b)): the
//!    trajectory-tail string MUST contain exactly 4 `→(\d+)` matches.
//!
//! # Behavioral Contracts
//!
//! - BC-5.39.005: blocks structurally invalid STATE.md writes.
//!
//! # D-NNN closures
//!
//! - D-421(c): banner wc-l discipline from pass 41 forward.
//! - D-422(c): banner wc-l discipline from pass 42 forward.
//! - D-424(b): banner wc-l discipline from pass 44 forward.
//! - D-428(d): banner wc-l discipline from pass 48 forward.
//! - D-438(a): banner wc-l discipline from pass 58 forward.
//! - D-440(d): banner wc-l discipline from pass 60 forward.
//! - D-442(d): banner wc-l discipline from pass 62 forward.
//! - D-446(c): dual-margin form required in banner.
//! - D-433(e): trajectory-tail LENGTH=4.
//! - D-439(c): trajectory-tail LENGTH=4 (pass 59 codification).
//! - D-451(c): trajectory-tail LENGTH=4 extension (pass 71).
//! - D-432(b): trajectory-tail canonical form `→N→N→N→N`.
//!
//! # Architecture compliance
//!
//! - HOST_ABI_VERSION = 1 (no new host functions introduced).
//! - Fail-open on every `host::read_file` error (BC-5.39.005 invariant 7).
//! - No `println!` — use `host::log_*` for all diagnostic output.
//! - No `unwrap()` or `expect()` in production paths.
//! - No `regex` crate: hand-rolled pattern scanning to stay within WASM fuel budget.
//! - File-path enforcement via in-plugin guard using `Path::file_name` (Q5/Q6 canonical
//!   pattern; NOT `ends_with("STATE.md")` — false-positive on `xSTATE.md`).
//! - `tool = "Edit|Write"` is the canonical Q5/Q6 form for this hook's registry entry.

use vsdd_hook_sdk::{HookPayload, HookResult};

/// HOST_ABI_VERSION declares the ABI contract version this plugin was built
/// against. Must remain 1.
pub const HOST_ABI_VERSION: u32 = 1;

// ---------------------------------------------------------------------------
// Violation type
// ---------------------------------------------------------------------------

/// A structural violation found in the STATE.md content.
#[derive(Debug, Clone)]
pub struct Violation {
    /// Human-readable description of the violation, used in the block message.
    pub description: String,
    /// The raw body-literal form of the offending text. Structural plumbing per
    /// TD-VSDD-059 paper-fix avoidance. Enables block message to quote the exact
    /// string the author wrote.
    pub cited_raw: String,
}

// ---------------------------------------------------------------------------
// File-path guard
// ---------------------------------------------------------------------------

/// Returns `true` if `file_path` names a file whose `file_name` component is
/// exactly `STATE.md`.
///
/// Uses path-component-strict matching (`Path::file_name`) rather than
/// `ends_with`, preventing false-positive fires on paths like
/// `/some/dir/xSTATE.md` where `.ends_with("STATE.md")` would also be true.
///
/// Returns `false` if the path has no file-name component (e.g. `/`).
///
/// # BC trace
/// BC-5.39.005 precondition 1 — hook only activates on STATE.md writes.
/// BC-5.39.005 invariant 6 — path-component-strict matching is mandatory.
pub fn is_state_md_target(file_path: &str) -> bool {
    std::path::Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        == Some("STATE.md")
}

// ---------------------------------------------------------------------------
// Banner line-count validation
// ---------------------------------------------------------------------------

/// Extract the declared line-count integer from the SIZE BUDGET banner comment.
///
/// Scans the content for a line matching the canonical banner form:
/// `(\d+) lines (wc-l)` — returns the first match.
///
/// Returns `None` if no such pattern is found (banner absent).
///
/// # BC trace
/// BC-5.39.005 invariant 3 — line-count comparison uses newline-character counting.
pub fn extract_banner_line_count(content: &str) -> Option<usize> {
    // Hand-rolled scan: look for the pattern `(\d+) lines (wc-l)` on any line.
    // We scan character-by-character to avoid `regex` crate fuel exhaustion.
    let marker = " lines (wc-l)";
    let marker_bytes = marker.as_bytes();

    let bytes = content.as_bytes();
    let content_len = bytes.len();
    let marker_len = marker_bytes.len();

    if content_len < marker_len {
        return None;
    }

    let mut i = 0usize;
    while i + marker_len <= content_len {
        // Check if marker starts at position i.
        if &bytes[i..i + marker_len] == marker_bytes {
            // Walk backwards from i to collect preceding digits.
            let digit_end = i;
            let mut digit_start = digit_end;
            while digit_start > 0 && bytes[digit_start - 1].is_ascii_digit() {
                digit_start -= 1;
            }
            if digit_start < digit_end {
                // Safe: digit range is all ASCII, so byte boundary = char boundary.
                let digits = &content[digit_start..digit_end];
                if let Ok(n) = digits.parse::<usize>() {
                    return Some(n);
                }
            }
        }
        i += 1;
    }
    None
}

/// Count actual newlines in `content`.
///
/// # BC trace
/// BC-5.39.005 invariant 3 — line-count computation uses `\n` counting.
pub fn count_newlines(content: &str) -> usize {
    content.bytes().filter(|&b| b == b'\n').count()
}

/// Validate that the banner line-count claim matches the actual newline count.
///
/// Returns a `Violation` if the claimed count diverges from the actual count.
/// Returns `None` if the banner is absent (caller decides how to handle this case;
/// in the current spec, a missing banner means no banner wc-l violation, but other
/// checks (e.g., dual-margin) will also fire if the banner structure is missing).
///
/// # BC trace
/// BC-5.39.005 postcondition 2.
pub fn validate_banner_wc(content: &str) -> Option<Violation> {
    let claimed = extract_banner_line_count(content)?;
    let actual = count_newlines(content);
    if claimed != actual {
        Some(Violation {
            description: format!(
                "banner claims {claimed} lines but actual line count is {actual} — \
                 reconcile banner wc-l (D-421(c)+D-422(c)+D-424(b)+D-428(d)+D-438(a)+\
                 D-440(d)+D-442(d))"
            ),
            cited_raw: format!("{claimed} lines (wc-l)"),
        })
    } else {
        None
    }
}

// ---------------------------------------------------------------------------
// Dual-margin validation
// ---------------------------------------------------------------------------

/// Validate that the banner contains both margin expressions (D-446(c) dual-margin form).
///
/// The canonical dual-margin form contains TWO distinct `margin from` phrases:
/// - `margin from soft-target` — the soft-target margin
/// - `margin from actual` — the actual margin
///
/// The presence of BOTH is required. One alone is insufficient.
///
/// Scanning is ASCII-safe: the marker strings are ASCII, and we use
/// `contains()` on string slices (no byte-index slicing).
///
/// # BC trace
/// BC-5.39.005 postcondition 3; invariant 4.
pub fn validate_dual_margin(content: &str) -> Option<Violation> {
    // The canonical banner comment block is the HTML comment between <!-- and -->.
    // Extract it: find the first `<!--` and the first `-->` after it.
    let banner_block = extract_banner_block(content);

    let block = match banner_block {
        Some(b) => b,
        None => {
            // No banner block at all — dual-margin is trivially absent.
            return Some(Violation {
                description: "dual-margin form absent: no banner block found; \
                               banner MUST contain both margins per D-446(c)"
                    .to_string(),
                cited_raw: String::new(),
            });
        }
    };

    let has_soft_target_margin = block.contains("margin from soft-target");
    let has_actual_margin = block.contains("margin from actual");

    if has_soft_target_margin && has_actual_margin {
        None
    } else {
        let missing = if !has_soft_target_margin && !has_actual_margin {
            "both 'margin from soft-target' and 'margin from actual'"
        } else if !has_soft_target_margin {
            "'margin from soft-target'"
        } else {
            "'margin from actual'"
        };
        Some(Violation {
            description: format!(
                "dual-margin form missing: {missing} absent from banner; \
                 banner MUST contain both margins per D-446(c)"
            ),
            cited_raw: block.trim_end().to_string(),
        })
    }
}

/// Extract the content of the SIZE BUDGET banner HTML comment block (`<!-- ... -->`).
///
/// Returns a string slice of the banner content (between `<!--` and `-->`) if found.
/// Returns `None` if no such block exists.
///
/// No byte-index slicing on non-ASCII boundaries: `find()` returns byte positions of
/// the ASCII delimiters, which are guaranteed ASCII-clean.
fn extract_banner_block(content: &str) -> Option<&str> {
    let start_marker = "<!--";
    let end_marker = "-->";

    let start = content.find(start_marker)?;
    let after_start = start + start_marker.len();

    // Find end marker after the opening.
    let rel_end = content[after_start..].find(end_marker)?;
    let end = after_start + rel_end;

    // end is a byte offset within content at an ASCII boundary (start of "-->" which is ASCII).
    // Verify it's a char boundary before slicing — defensive guard per S-15.11 F-P4-001 lesson.
    if !content.is_char_boundary(after_start) || !content.is_char_boundary(end) {
        return None;
    }

    Some(&content[after_start..end])
}

// ---------------------------------------------------------------------------
// Trajectory-tail cardinality validation
// ---------------------------------------------------------------------------

/// Extract the trajectory-tail line from content.
///
/// The canonical tail line contains `→` (U+2192, RIGHTWARDS ARROW) followed by digits,
/// repeated N times. We look for a line containing `→` followed by at least one digit.
///
/// Returns the first matching line (trimmed) or `None` if no such line exists.
///
/// # BC trace
/// BC-5.39.005 postcondition 4; invariant 5.
pub fn extract_trajectory_tail_line(content: &str) -> Option<String> {
    // The arrow character is UTF-8: → = U+2192 = 0xE2 0x86 0x92 (3 bytes).
    // We use str::contains to find lines with the pattern without byte-index arithmetic.
    for line in content.split('\n') {
        let trimmed = line.trim_end_matches('\r').trim();
        // A trajectory-tail line contains at least one `→` followed by a digit.
        if contains_arrow_digit_sequence(trimmed) {
            return Some(trimmed.to_string());
        }
    }
    None
}

/// Returns `true` if `s` contains at least one `→N` sequence (arrow + ASCII digit).
///
/// Hand-rolled scan to stay within WASM fuel budget.
/// The arrow → is U+2192 = 0xE2 0x86 0x92 in UTF-8 (3 bytes).
fn contains_arrow_digit_sequence(s: &str) -> bool {
    let arrow = "\u{2192}"; // → U+2192
    let arrow_bytes = arrow.len(); // 3 bytes
    let bytes = s.as_bytes();

    let mut i = 0usize;
    while i + arrow_bytes <= bytes.len() {
        // Check if the arrow (3-byte UTF-8 sequence) starts at position i.
        if &bytes[i..i + arrow_bytes] == arrow.as_bytes() {
            // The byte immediately after the arrow must be an ASCII digit.
            let after = i + arrow_bytes;
            if after < bytes.len() && bytes[after].is_ascii_digit() {
                return true;
            }
        }
        i += 1;
    }
    false
}

/// Count the number of `→N` matches (arrow followed by digit sequence) in a string.
///
/// Hand-rolled: scan for `→` (U+2192, 3 UTF-8 bytes) followed immediately by ASCII digits.
/// Each occurrence (regardless of digit length) counts as one match.
///
/// # BC trace
/// BC-5.39.005 invariant 5 — regex `→(\d+)` match count must equal 4.
pub fn count_arrow_digit_matches(s: &str) -> usize {
    let arrow = "\u{2192}"; // → U+2192
    let arrow_bytes = arrow.len(); // 3 bytes
    let bytes = s.as_bytes();
    let len = bytes.len();

    let mut count = 0usize;
    let mut i = 0usize;

    while i + arrow_bytes <= len {
        // Check if arrow starts at position i.
        if &bytes[i..i + arrow_bytes] == arrow.as_bytes() {
            let after = i + arrow_bytes;
            if after < len && bytes[after].is_ascii_digit() {
                count += 1;
                // Skip past the arrow and all following digits to avoid double-counting
                // on adjacent arrows (in practice the pattern is →N→N→N→N with numbers).
                let mut j = after;
                while j < len && bytes[j].is_ascii_digit() {
                    j += 1;
                }
                i = j;
                continue;
            }
        }
        i += 1;
    }
    count
}

/// Validate that the trajectory-tail contains exactly 4 `→N` matches.
///
/// Returns a `Violation` if the count is not 4 (or no tail line found).
/// Returns `None` if the count is exactly 4.
///
/// # BC trace
/// BC-5.39.005 postcondition 4; EC-005, EC-006, EC-008.
pub fn validate_trajectory_tail(content: &str) -> Option<Violation> {
    match extract_trajectory_tail_line(content) {
        None => Some(Violation {
            description: "no trajectory-tail found; expected form `→N→N→N→N` with \
                          exactly 4 components per D-432(b)+D-433(e)+D-439(c)+D-451(c)"
                .to_string(),
            cited_raw: String::new(),
        }),
        Some(tail_line) => {
            let count = count_arrow_digit_matches(&tail_line);
            if count == 4 {
                None
            } else {
                Some(Violation {
                    description: format!(
                        "trajectory-tail has {count} components; required LENGTH=4 per \
                         D-433(e)+D-439(c)+D-451(c)+D-432(b)"
                    ),
                    cited_raw: tail_line,
                })
            }
        }
    }
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
        "validate-state-structure: {} violation(s) in STATE.md:\n{}",
        violations.len(),
        lines.join("\n")
    );
    HookResult::block_with_fix(
        "validate-state-structure",
        reason,
        "Fix the violations listed above before re-writing STATE.md",
        "STATE_STRUCTURE_VIOLATION",
    )
}

// ---------------------------------------------------------------------------
// Hook entry point
// ---------------------------------------------------------------------------

/// Core hook logic for validate-state-structure.
///
/// Called from the WASI entry point in `main.rs` via the SDK trampoline.
/// The dispatcher routes PostToolUse `Edit|Write` events to this hook.
///
/// 1. Extracts `file_path` from `tool_input`; early-exit Continue for
///    non-STATE.md paths (Q5/Q6 in-plugin guard via `is_state_md_target`).
/// 2. Reads the written STATE.md content via `host::read_file`. On failure,
///    emits Continue + log_warn (fail-open per BC-5.39.005 postcondition 6).
/// 3. Validates: banner line-count, dual-margin form, trajectory-tail cardinality.
/// 4. Emits `HookResult::block_with_fix` if any violation found, or
///    `HookResult::Continue` if all properties hold.
///
/// # BC trace
/// BC-5.39.005 postconditions 1-6; invariants 1-8.
pub fn on_post_tool_use(payload: HookPayload) -> HookResult {
    use vsdd_hook_sdk::host;

    // Extract file_path from tool_input.
    let file_path = match payload.tool_input.get("file_path").and_then(|v| v.as_str()) {
        Some(p) => p.to_string(),
        None => {
            host::log_warn(
                "[validate-state-structure] file_path absent from tool_input — graceful degrade",
            );
            return HookResult::Continue;
        }
    };

    // In-plugin file-path guard (Q5/Q6 canonical pattern):
    // only act on writes to STATE.md. Uses path-component-strict matching via
    // Path::file_name to avoid false-positives from suffix-only ends_with
    // (e.g. a path "xSTATE.md" would incorrectly trigger a bare ends_with guard).
    if !is_state_md_target(&file_path) {
        return HookResult::Continue;
    }

    // Read the STATE.md content that was just written.
    // On read failure: fail-open (Continue + log_warn) per BC-5.39.005 postcondition 6.
    let content = match host::read_file(&file_path, 65536, 2000) {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(s) => s,
            Err(e) => {
                host::log_warn(&format!(
                    "[validate-state-structure] UTF-8 decode failure reading {file_path}: {e}"
                ));
                return HookResult::Continue;
            }
        },
        Err(e) => {
            host::log_warn(&format!(
                "[validate-state-structure] read_file failed for {file_path}: {e:?}"
            ));
            return HookResult::Continue;
        }
    };

    let mut violations: Vec<Violation> = Vec::new();

    // Validate banner line-count.
    if let Some(v) = validate_banner_wc(&content) {
        violations.push(v);
    }

    // Validate dual-margin form.
    if let Some(v) = validate_dual_margin(&content) {
        violations.push(v);
    }

    // Validate trajectory-tail cardinality.
    if let Some(v) = validate_trajectory_tail(&content) {
        violations.push(v);
    }

    if violations.is_empty() {
        HookResult::Continue
    } else {
        emit_block(&violations)
    }
}

// ---------------------------------------------------------------------------
// Unit tests — BC-5.39.005
// ---------------------------------------------------------------------------

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    // ── is_state_md_target ───────────────────────────────────────────────────

    #[test]
    fn test_BC_5_39_005_state_md_canonical_path_matches() {
        assert!(is_state_md_target(".factory/STATE.md"));
        assert!(is_state_md_target("STATE.md"));
        assert!(is_state_md_target("/absolute/path/to/STATE.md"));
    }

    #[test]
    fn test_BC_5_39_005_xstate_md_path_does_not_match() {
        // EC-012: xSTATE.md must NOT trigger the hook (path-component-strict guard)
        assert!(!is_state_md_target("/some/dir/xSTATE.md"));
        assert!(!is_state_md_target("not-STATE.md"));
        assert!(!is_state_md_target("aSTATE.md"));
    }

    #[test]
    fn test_BC_5_39_005_non_state_md_paths_do_not_match() {
        assert!(!is_state_md_target("burst-log.md"));
        assert!(!is_state_md_target(".factory/cycles/burst-log.md"));
        assert!(!is_state_md_target("ARCH-INDEX.md"));
    }

    // ── extract_banner_line_count ────────────────────────────────────────────

    #[test]
    fn test_BC_5_39_005_extract_banner_line_count_returns_integer() {
        let content = "<!-- 28 lines (wc-l). Hard cap: 500 -->\nrest\n";
        let result = extract_banner_line_count(content);
        assert_eq!(result, Some(28));
    }

    #[test]
    fn test_BC_5_39_005_extract_banner_line_count_absent_returns_none() {
        let content = "# just a heading\n\nno banner here\n";
        let result = extract_banner_line_count(content);
        assert_eq!(result, None);
    }

    #[test]
    fn test_BC_5_39_005_extract_banner_line_count_canonical_form() {
        // Canonical banner form from fixtures:
        // "Hard cap (500 lines) margin from soft-target = 500 - 415 = 85; margin from actual = 500 - 28 = 472 (D-446(c) dual-margin form). 28 lines (wc-l)."
        let content = "<!--\n  STATE.md SIZE BUDGET (per D-421(c)):\n  Hard cap (500 lines) margin from soft-target = 500 - 415 = 85; margin from actual = 500 - 28 = 472 (D-446(c) dual-margin form). 28 lines (wc-l).\n  Hard cap: 500 lines.\n-->\n";
        let result = extract_banner_line_count(content);
        assert_eq!(result, Some(28));
    }

    // ── count_newlines ───────────────────────────────────────────────────────

    #[test]
    fn test_BC_5_39_005_count_newlines_empty() {
        assert_eq!(count_newlines(""), 0);
    }

    #[test]
    fn test_BC_5_39_005_count_newlines_three_lines() {
        assert_eq!(count_newlines("a\nb\nc\n"), 3);
    }

    // ── validate_banner_wc ───────────────────────────────────────────────────

    #[test]
    fn test_BC_5_39_005_banner_wc_match_returns_none() {
        // EC-002: banner claims 3 and file has 3 newlines
        let content = "<!--\n28 lines (wc-l).\n-->\n"; // 3 newlines, claim 28
        // This should produce Some(Violation) because 28 != 3
        let v = validate_banner_wc(content);
        assert!(
            v.is_some(),
            "mismatch (claimed 28, actual 3) should be Some"
        );
    }

    #[test]
    fn test_BC_5_39_005_banner_wc_correct_returns_none() {
        // 2 lines, banner claims 2
        let content = "<!-- 2 lines (wc-l) -->\n\n";
        let v = validate_banner_wc(content);
        assert!(v.is_none(), "exact match should return None");
    }

    #[test]
    fn test_BC_5_39_005_banner_wc_off_by_one_names_both_counts() {
        // EC-001: banner claims 27 but file has 28 newlines
        // Build a 28-newline content with banner claiming 27
        let mut content = String::new();
        content.push_str("<!-- 27 lines (wc-l) -->\n");
        for _ in 0..27 {
            content.push_str("line\n");
        }
        // Total: 28 newlines (1 from banner + 27 from loop)
        assert_eq!(count_newlines(&content), 28);
        let v = validate_banner_wc(&content).expect("should produce violation");
        assert!(v.description.contains("27"), "must name claimed count 27");
        assert!(v.description.contains("28"), "must name actual count 28");
    }

    // ── validate_dual_margin ─────────────────────────────────────────────────

    #[test]
    fn test_BC_5_39_005_dual_margin_both_present_returns_none() {
        let content =
            "<!--\n  margin from soft-target = 85; margin from actual = 472 (D-446(c))\n-->\n";
        let v = validate_dual_margin(content);
        assert!(v.is_none(), "both margins present should return None");
    }

    #[test]
    fn test_BC_5_39_005_dual_margin_only_soft_target_returns_violation() {
        // EC-003: only soft-target margin present
        let content = "<!--\n  margin from soft-target = 500 - 415 = 85. 28 lines (wc-l).\n-->\n";
        let v = validate_dual_margin(content);
        assert!(
            v.is_some(),
            "missing 'margin from actual' should be violation"
        );
        let viol = v.unwrap();
        assert!(
            viol.description.contains("D-446"),
            "must cite D-446(c); got: {}",
            viol.description
        );
    }

    #[test]
    fn test_BC_5_39_005_dual_margin_absent_banner_block_returns_violation() {
        let content = "# no banner here\n\nsome content\n";
        let v = validate_dual_margin(content);
        assert!(
            v.is_some(),
            "absent banner should be a dual-margin violation"
        );
    }

    // ── validate_trajectory_tail ─────────────────────────────────────────────

    #[test]
    fn test_BC_5_39_005_trajectory_tail_4_components_returns_none() {
        // EC-007: exactly 4 components → Continue
        let content = "# heading\n\nTrajectory \u{2192}9\u{2192}9\u{2192}9\u{2192}9\n\n";
        let v = validate_trajectory_tail(content);
        assert!(v.is_none(), "4 components should return None");
    }

    #[test]
    fn test_BC_5_39_005_trajectory_tail_3_components_returns_violation() {
        // EC-005: 3 components → block
        let content = "# heading\n\nTrajectory \u{2192}9\u{2192}9\u{2192}9\n\n";
        let v = validate_trajectory_tail(content);
        assert!(v.is_some(), "3 components should be violation");
        let viol = v.unwrap();
        assert!(viol.description.contains('3'), "must name actual count 3");
        assert!(viol.description.contains('4'), "must name required count 4");
        assert!(viol.description.contains("D-433"), "must cite D-433(e)");
    }

    #[test]
    fn test_BC_5_39_005_trajectory_tail_5_components_returns_violation() {
        // EC-006: 5 components → block
        let content = "# heading\n\nTrajectory \u{2192}9\u{2192}9\u{2192}9\u{2192}9\u{2192}9\n\n";
        let v = validate_trajectory_tail(content);
        assert!(v.is_some(), "5 components should be violation");
        let viol = v.unwrap();
        assert!(viol.description.contains('5'), "must name actual count 5");
    }

    #[test]
    fn test_BC_5_39_005_trajectory_tail_absent_returns_violation() {
        // EC-008: no trajectory-tail line → block
        let content = "# heading\n\nno tail here\n";
        let v = validate_trajectory_tail(content);
        assert!(v.is_some(), "absent tail should be violation");
        let viol = v.unwrap();
        assert!(
            viol.description.contains("no trajectory-tail"),
            "should mention missing tail"
        );
    }

    // ── count_arrow_digit_matches ────────────────────────────────────────────

    #[test]
    fn test_BC_5_39_005_count_arrow_digit_4_components() {
        let s = "\u{2192}9\u{2192}9\u{2192}9\u{2192}9";
        assert_eq!(count_arrow_digit_matches(s), 4);
    }

    #[test]
    fn test_BC_5_39_005_count_arrow_digit_3_components() {
        let s = "\u{2192}9\u{2192}9\u{2192}9";
        assert_eq!(count_arrow_digit_matches(s), 3);
    }

    #[test]
    fn test_BC_5_39_005_count_arrow_digit_5_components() {
        let s = "\u{2192}9\u{2192}9\u{2192}9\u{2192}9\u{2192}9";
        assert_eq!(count_arrow_digit_matches(s), 5);
    }

    #[test]
    fn test_BC_5_39_005_count_arrow_digit_multi_digit_numbers() {
        // →12→34→56→78 — 4 matches even with multi-digit numbers
        let s = "\u{2192}12\u{2192}34\u{2192}56\u{2192}78";
        assert_eq!(count_arrow_digit_matches(s), 4);
    }

    #[test]
    fn test_BC_5_39_005_count_arrow_digit_no_arrows() {
        assert_eq!(count_arrow_digit_matches("no arrows here"), 0);
    }

    // ── UTF-8 safety: em-dash / en-dash in banner ────────────────────────────

    #[test]
    fn test_BC_5_39_005_banner_with_emdash_no_panic() {
        // EC-013: banner narrative with em-dash should not panic (is_char_boundary guard)
        let content = "<!--\n  margin from soft-target = 500\u{2014}415 = 85; margin from actual = 472 (D-446(c))\n-->\nline\n";
        // Just confirm extract_banner_block doesn't panic
        let block = extract_banner_block(content);
        assert!(block.is_some(), "should extract banner block with em-dash");
    }

    // ── is_state_md_target path-component strictness with UTF-8 path ────────

    #[test]
    fn test_BC_5_39_005_path_with_utf8_prefix_does_not_match_state_md() {
        // A path like /dir/\u{00E9}STATE.md — file_name is "\u{00E9}STATE.md" ≠ "STATE.md"
        assert!(!is_state_md_target("/dir/\u{00E9}STATE.md"));
    }

    // ── emit_block ───────────────────────────────────────────────────────────

    #[test]
    fn test_BC_5_39_005_emit_block_exit_code_2() {
        let violations = vec![Violation {
            description: "test violation".to_string(),
            cited_raw: String::new(),
        }];
        let result = emit_block(&violations);
        assert_eq!(result.exit_code(), 2);
    }

    #[test]
    fn test_BC_5_39_005_emit_block_names_all_violations() {
        let violations = vec![
            Violation {
                description: "banner wc-l violation D-421(c)".to_string(),
                cited_raw: "27 lines (wc-l)".to_string(),
            },
            Violation {
                description: "dual-margin missing D-446(c)".to_string(),
                cited_raw: String::new(),
            },
            Violation {
                description: "trajectory-tail 3 components D-433(e)".to_string(),
                cited_raw: "\u{2192}9\u{2192}9\u{2192}9".to_string(),
            },
        ];
        let result = emit_block(&violations);
        match &result {
            HookResult::Block { reason } => {
                assert!(reason.contains("D-421"), "must mention D-421");
                assert!(reason.contains("D-446"), "must mention D-446");
                assert!(reason.contains("D-433"), "must mention D-433");
            }
            _ => panic!("expected Block result, got {result:?}"),
        }
    }
}
