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
/// Scans for the tolerant pattern `(\d+) lines (wc-l` followed by any non-digit,
/// non-letter terminator byte (`;`, `.`, `,`, `)`, space, etc.). This matches all
/// real-world STATE.md forms:
///
/// - `N lines (wc-l)` — canonical fixture form
/// - `N lines (wc-l; net +N ...)` — line-growth tracker form (real STATE.md)
/// - `N lines (wc-l. rest)` — sentence-end form
/// - `N lines (wc-l, ...)` — comma-separated form
///
/// Returns the **last** matching occurrence. Real STATE.md has many interim
/// `N lines (wc-l;` entries in the line-growth tracker; the last one is the
/// most recent canonical claim (the trailing entry in the tracker sequence).
///
/// Returns `None` if no such pattern is found (banner absent).
///
/// # BC trace
/// BC-5.39.005 invariant 3 — line-count comparison uses newline-character counting.
/// F-P1-001 fix: tolerant terminator; last-occurrence anchor.
pub fn extract_banner_line_count(content: &str) -> Option<usize> {
    // Hand-rolled scan: look for "lines (wc-l" then verify a non-alphanum terminator.
    // We stay on raw bytes to avoid regex crate fuel exhaustion. All pattern bytes
    // are ASCII, so no UTF-8 split risk.
    let prefix = b" lines (wc-l";
    let prefix_len = prefix.len();

    let bytes = content.as_bytes();
    let content_len = bytes.len();

    if content_len < prefix_len {
        return None;
    }

    let mut last_found: Option<usize> = None;
    let mut i = 0usize;

    while i + prefix_len <= content_len {
        if &bytes[i..i + prefix_len] == prefix {
            // Check that the byte immediately after the prefix is a valid terminator:
            // any byte that is NOT an ASCII letter or digit. This accepts `)`, `;`, `.`, `,`,
            // space, `\n`, etc. — but rejects mid-word matches like "lines (wc-lb..." which
            // could occur if terminology diverges.
            let after = i + prefix_len;
            let valid_terminator = after >= content_len  // prefix at end of content
                || {
                    let b = bytes[after];
                    !b.is_ascii_alphabetic() && !b.is_ascii_digit()
                };

            if valid_terminator {
                // Walk backwards from i to collect preceding digits.
                let digit_end = i;
                let mut digit_start = digit_end;
                while digit_start > 0 && bytes[digit_start - 1].is_ascii_digit() {
                    digit_start -= 1;
                }
                if digit_start < digit_end {
                    // Safe: digit range is all ASCII, so byte boundary = char boundary.
                    if let Ok(n) = content[digit_start..digit_end].parse::<usize>() {
                        last_found = Some(n);
                    }
                }
            }
        }
        i += 1;
    }

    last_found
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
/// Returns a `Violation` in two cases:
///
/// 1. **Banner absent** (no `N lines (wc-l...)` pattern found): returns a violation
///    per BC-5.39.005 EC-014 — an absent SIZE BUDGET banner is itself a structural
///    defect. Previously this was silently skipped, which allowed STATE.md files with
///    no banner to pass this check; that was incorrect per the spec.
///
/// 2. **Claimed count diverges from actual count**: returns a count-mismatch violation.
///
/// Returns `None` if banner present and claimed == actual.
///
/// # BC trace
/// BC-5.39.005 postcondition 2; EC-014 (absent banner).
/// F-P1-002 fix: fire violation on absent banner rather than silently returning None.
pub fn validate_banner_wc(content: &str) -> Option<Violation> {
    match extract_banner_line_count(content) {
        None => {
            // EC-014: absent SIZE BUDGET banner is a structural violation.
            Some(Violation {
                description: "no SIZE BUDGET banner found; STATE.md MUST include an HTML comment \
                     banner with 'N lines (wc-l)' claim per \
                     D-421(c)+D-422(c)+D-424(b)+D-428(d)+D-438(a)+D-440(d)+D-442(d)"
                    .to_string(),
                cited_raw: "(none)".to_string(),
            })
        }
        Some(claimed) => {
            let actual = count_newlines(content);
            if claimed != actual {
                Some(Violation {
                    description: format!(
                        "banner claims {claimed} lines but actual line count is {actual} — \
                         reconcile banner wc-l (D-421(c)+D-422(c)+D-424(b)+D-428(d)+\
                         D-438(a)+D-440(d)+D-442(d))"
                    ),
                    cited_raw: format!("{claimed} lines (wc-l)"),
                })
            } else {
                None
            }
        }
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
/// Anchors on the literal `STATE.md SIZE BUDGET` marker — the canonical heading that
/// appears in real STATE.md (line 24 in the actual file). This prevents false matches
/// against other HTML comments that may appear earlier in the document.
///
/// Algorithm:
/// 1. Find `STATE.md SIZE BUDGET` in the content.
/// 2. From that position, scan backwards to find the opening `<!--`.
/// 3. From the opening `<!--`, scan forward to find the closing `-->`.
/// 4. Return the slice between `<!--` and `-->` (exclusive of the delimiters).
///
/// Returns `None` if:
/// - No `STATE.md SIZE BUDGET` marker exists (no size budget block).
/// - No `<!--` is found before the marker.
/// - No `-->` is found after the opening `<!--`.
/// - The extracted slice does not land on valid UTF-8 char boundaries.
///
/// No byte-index slicing on non-ASCII boundaries: `find()` returns byte positions of
/// ASCII delimiters, which are guaranteed ASCII-clean. The `is_char_boundary` guard
/// handles the unlikely case where content bytes are unexpected.
///
/// # F-P1-010 fix
/// Previously anchored on "first HTML comment" — now anchored on `STATE.md SIZE BUDGET`.
fn extract_banner_block(content: &str) -> Option<&str> {
    let budget_marker = "STATE.md SIZE BUDGET";
    let open_marker = "<!--";
    let close_marker = "-->";

    // Find the SIZE BUDGET marker.
    let budget_pos = content.find(budget_marker)?;

    // Scan backwards from budget_pos to find the opening `<!--`.
    // We look in the substring content[0..budget_pos].
    let before_marker = &content[..budget_pos];
    let open_pos = before_marker.rfind(open_marker)?;
    let after_open = open_pos + open_marker.len();

    // Find the closing `-->` after the opening.
    let rel_close = content[after_open..].find(close_marker)?;
    let close_pos = after_open + rel_close;

    // Verify char boundaries (ASCII delimiters, but be defensive).
    if !content.is_char_boundary(after_open) || !content.is_char_boundary(close_pos) {
        return None;
    }

    Some(&content[after_open..close_pos])
}

// ---------------------------------------------------------------------------
// Trajectory-tail cardinality validation
// ---------------------------------------------------------------------------

/// Extract the trajectory-tail line from content.
///
/// The canonical tail line contains `→` (U+2192, RIGHTWARDS ARROW) followed by digits,
/// repeated N times (canonical form: `→N→N→N→N`).
///
/// # Anchoring rule (F-P1-007 fix)
///
/// Real STATE.md prose may contain `→N` sequences in narrative tables, chart rows,
/// and decision-log rows anywhere in the document. To avoid picking up a spurious
/// narrative occurrence, this function first extracts the SIZE BUDGET banner block via
/// `extract_banner_block` and searches ONLY within that block. If the banner block
/// exists and contains a trajectory-tail line, that line is returned.
///
/// If the banner block is absent (or contains no arrow-digit line), falls back to
/// scanning the full document — this preserves compatibility with older fixtures that
/// store the trajectory tail outside the banner comment.
///
/// Returns the first matching line (trimmed) within the banner block, or the first
/// matching line in the full document if no banner block is found.
/// Returns `None` if no arrow-digit sequence is found anywhere.
///
/// # BC trace
/// BC-5.39.005 postcondition 4; invariant 5.
pub fn extract_trajectory_tail_line(content: &str) -> Option<String> {
    // The arrow character is UTF-8: → = U+2192 = 0xE2 0x86 0x92 (3 bytes).

    // Prefer the banner-block-anchored scan (F-P1-007).
    if let Some(block) = extract_banner_block(content) {
        for line in block.split('\n') {
            let trimmed = line.trim_end_matches('\r').trim();
            if contains_arrow_digit_sequence(trimmed) {
                return Some(trimmed.to_string());
            }
        }
    }

    // Fallback: scan full document (for fixtures without a SIZE BUDGET banner).
    for line in content.split('\n') {
        let trimmed = line.trim_end_matches('\r').trim();
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
/// Uses `char_indices()` iteration to match on the Unicode scalar `'→'` (U+2192)
/// directly, which is both correct and explicit about UTF-8 safety.
///
/// # UTF-8 safety (F-P1-009)
///
/// The previous implementation used raw byte scanning with `i += 1` stepping.
/// That was technically safe because U+2192 encodes as `[0xE2, 0x86, 0x92]` in
/// UTF-8 — `0xE2` is a leading byte (4-bit prefix `1110`), and `0x86`/`0x92` are
/// continuation bytes (prefix `10`), so they are never confused with ASCII bytes.
/// However, `char_indices()` is cleaner: it iterates decoded `char` values and
/// their byte positions, explicitly handling all multi-byte sequences.
///
/// The ASCII digit check on `bytes[after]` remains valid: all ASCII bytes have the
/// high bit 0, so they cannot be continuation bytes of any multi-byte sequence.
///
/// # BC trace
/// BC-5.39.005 invariant 5 — `→(\d+)` match count must equal 4.
pub fn count_arrow_digit_matches(s: &str) -> usize {
    let bytes = s.as_bytes();
    let len = bytes.len();
    let arrow_byte_len = '\u{2192}'.len_utf8(); // 3 bytes

    let mut count = 0usize;
    let mut skip_until_byte: usize = 0;

    for (byte_pos, ch) in s.char_indices() {
        if byte_pos < skip_until_byte {
            continue;
        }
        if ch == '\u{2192}' {
            // The byte immediately after the arrow (byte_pos + 3) must be an ASCII digit.
            let after = byte_pos + arrow_byte_len;
            if after < len && bytes[after].is_ascii_digit() {
                count += 1;
                // Skip past the arrow and all following digits to avoid double-counting.
                let mut j = after;
                while j < len && bytes[j].is_ascii_digit() {
                    j += 1;
                }
                skip_until_byte = j;
            }
        }
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
        // Banner must include STATE.md SIZE BUDGET for extract_banner_block to find it (F-P1-010).
        let content = "<!--\n  STATE.md SIZE BUDGET (per D-421(c)):\n  margin from soft-target = 85; margin from actual = 472 (D-446(c))\n-->\n";
        let v = validate_dual_margin(content);
        assert!(v.is_none(), "both margins present should return None");
    }

    #[test]
    fn test_BC_5_39_005_dual_margin_only_soft_target_returns_violation() {
        // EC-003: only soft-target margin present
        // Banner must include STATE.md SIZE BUDGET marker (F-P1-010).
        let content = "<!--\n  STATE.md SIZE BUDGET (per D-421(c)):\n  margin from soft-target = 500 - 415 = 85. 28 lines (wc-l).\n-->\n";
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
        // EC-013: banner narrative with em-dash should not panic (is_char_boundary guard).
        // Banner must include STATE.md SIZE BUDGET marker for extract_banner_block to find it (F-P1-010).
        let content = "<!--\n  STATE.md SIZE BUDGET (per D-421(c)):\n  margin from soft-target = 500\u{2014}415 = 85; margin from actual = 472 (D-446(c))\n-->\nline\n";
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

    // ── F-P1-001: tolerant wc-l terminators (;, ., ,, )) ────────────────────

    /// F-P1-001: extract_banner_line_count must accept semicolon terminator
    /// Real STATE.md uses "N lines (wc-l; net +N ...)" throughout line-growth tracker.
    #[test]
    fn test_BC_5_39_005_extract_banner_line_count_semicolon_terminator() {
        let content = "<!--\n  395 lines (wc-l; net -52 from pass-64).\n-->\nrest\n";
        let result = extract_banner_line_count(content);
        assert_eq!(result, Some(395), "semicolon terminator must be accepted");
    }

    /// F-P1-001: extract_banner_line_count must accept comma terminator
    #[test]
    fn test_BC_5_39_005_extract_banner_line_count_comma_terminator() {
        let content = "<!-- 428 lines (wc-l, net +5) -->\nrest\n";
        let result = extract_banner_line_count(content);
        assert_eq!(result, Some(428), "comma terminator must be accepted");
    }

    /// F-P1-001: extract_banner_line_count must accept close-paren terminator (original form)
    #[test]
    fn test_BC_5_39_005_extract_banner_line_count_paren_terminator() {
        let content = "<!-- 2 lines (wc-l) -->\n\n";
        let result = extract_banner_line_count(content);
        assert_eq!(result, Some(2), "close-paren terminator must be accepted");
    }

    /// F-P1-001: all four terminators accepted in a single-function test
    #[test]
    fn test_BC_5_39_005_extract_banner_line_count_all_terminators() {
        for (terminator, input, expected) in [
            (")", "<!-- 10 lines (wc-l) -->", Some(10usize)),
            (";", "<!-- 20 lines (wc-l; net +2) -->", Some(20)),
            (".", "<!-- 30 lines (wc-l). rest -->", Some(30)),
            (",", "<!-- 40 lines (wc-l, net +4) -->", Some(40)),
        ] {
            let result = extract_banner_line_count(input);
            assert_eq!(
                result, expected,
                "terminator '{terminator}' must be accepted; input={input:?}"
            );
        }
    }

    /// F-P1-001: when multiple wc-l claims exist (line-growth tracker pattern),
    /// extract_banner_line_count MUST return the LAST occurrence (canonical trailing claim).
    #[test]
    fn test_BC_5_39_005_extract_banner_line_count_returns_last_occurrence() {
        // Simulates real STATE.md banner: many interim wc-l; claims, final canonical claim
        let content = concat!(
            "<!--\n",
            "  STATE.md SIZE BUDGET (per D-421(c)):\n",
            "  Line-growth tracker: pass-65 395 lines (wc-l; net -52); pass-66 397 lines (wc-l; net +2); pass-67 399 lines (wc-l; net +2).\n",
            "  Hard cap (500 lines) margin from soft-target = 500 - 415 = 85; margin from actual = 500 - 428 = 72 (D-446(c) dual-margin form).\n",
            "-->\n",
        );
        // The LAST numeric before "lines (wc-l" is 428 (in the margin sentence)
        // But wait — the margin sentence has no `lines (wc-l` — only the tracker entries do.
        // So last occurrence is 399. Verify that:
        let result = extract_banner_line_count(content);
        assert_eq!(result, Some(399), "last wc-l occurrence must be returned");
    }

    /// F-P1-001: real-prose banner (mirrors STATE.md) with semicolon-terminated
    /// wc-l claims must extract the last occurrence correctly.
    #[test]
    fn test_BC_5_39_005_extract_banner_line_count_real_prose_pattern() {
        // Mirror of actual STATE.md line 26 structure (abbreviated)
        let content = concat!(
            "<!--\n",
            "  STATE.md SIZE BUDGET (per D-421(c)):\n",
            "  Soft target: \u{2264}415 lines; margin from soft-target = 500 - 415 = 85; margin from actual = 500 - 428 = 72 (D-446(c) dual-margin form).\n",
            "  Line-growth tracker: pass-49 310 lines; pass-70 435 lines (wc-l; net +30); pass-71 439 lines (wc-l; net +4); S-15.11-post-merge-burst 428 lines (wc-l; net +5 from pass-70).\n",
            "  Hard cap: 500 lines.\n",
            "-->\n",
        );
        let result = extract_banner_line_count(content);
        assert_eq!(
            result,
            Some(428),
            "last wc-l occurrence in real-prose banner must be extracted"
        );
    }

    // ── F-P1-002: banner absent fires banner-wc violation (EC-014) ──────────

    /// F-P1-002: absent banner must produce a banner-wc violation
    /// (BC-5.39.005 EC-014: empty STATE.md => both banner AND tail violations).
    #[test]
    fn test_BC_5_39_005_validate_banner_wc_absent_banner_returns_violation() {
        let content = "# no banner\n\nTrajectory \u{2192}9\u{2192}9\u{2192}9\u{2192}9\n";
        let v = validate_banner_wc(content);
        assert!(
            v.is_some(),
            "absent banner must produce a banner-wc violation"
        );
        let viol = v.unwrap();
        assert!(
            viol.description.contains("no SIZE BUDGET banner"),
            "description must mention missing banner; got: {}",
            viol.description
        );
        assert!(
            viol.description.contains("D-421"),
            "must cite D-421(c) per spec; got: {}",
            viol.description
        );
    }

    /// F-P1-002: truly empty STATE.md fires BOTH banner AND tail violations (EC-014)
    #[test]
    fn test_BC_5_39_005_ec014_empty_state_md_fires_both_violations() {
        let content = "";
        let banner_viol = validate_banner_wc(content);
        let tail_viol = validate_trajectory_tail(content);
        assert!(
            banner_viol.is_some(),
            "empty content must produce banner-wc violation (EC-014)"
        );
        assert!(
            tail_viol.is_some(),
            "empty content must produce trajectory-tail violation (EC-014)"
        );
    }

    // ── F-P1-007+F-P1-010: banner-block anchored on STATE.md SIZE BUDGET ────

    /// F-P1-010: extract_banner_block must anchor on "STATE.md SIZE BUDGET" marker.
    /// If there is an HTML comment before the SIZE BUDGET comment, it must be skipped.
    #[test]
    fn test_BC_5_39_005_extract_banner_block_anchored_on_size_budget_marker() {
        // Content with a leading HTML comment (not the SIZE BUDGET) followed by the real banner
        let content = concat!(
            "<!-- some other comment -->\n",
            "<!--\n",
            "  STATE.md SIZE BUDGET (per D-421(c)):\n",
            "  Hard cap (500 lines) margin from soft-target = 85; margin from actual = 472 (D-446(c)).\n",
            "-->\n",
            "# heading\n",
        );
        let block = extract_banner_block(content);
        assert!(block.is_some(), "should find the SIZE BUDGET banner block");
        let block = block.unwrap();
        assert!(
            block.contains("STATE.md SIZE BUDGET"),
            "extracted block must contain SIZE BUDGET marker; got: {block:?}"
        );
        assert!(
            !block.contains("some other comment"),
            "must not return the first unrelated comment; got: {block:?}"
        );
    }

    /// F-P1-007: trajectory tail extraction must be anchored within the SIZE BUDGET banner block.
    /// Narrative text containing →N outside the banner should NOT be picked up as the tail.
    #[test]
    fn test_BC_5_39_005_trajectory_tail_anchored_in_banner_block() {
        // This content has an →N sequence in a narrative paragraph but the canonical
        // tail is in the SIZE BUDGET block.
        let content = concat!(
            "<!--\n",
            "  STATE.md SIZE BUDGET (per D-421(c)):\n",
            "  Trajectory →9→9→9→9\n",
            "  Hard cap (500 lines) margin from soft-target = 85; margin from actual = 472 (D-446(c)).\n",
            "-->\n",
            "\n",
            "# Pipeline State\n",
            "\n",
            "Some narrative mentioning →5→6→7 axis counts in a table.\n",
            "\n",
        );
        // The banner contains →9→9→9→9 (4 components) — valid.
        // The narrative line has →5→6→7 (3 components) — would be invalid if picked up.
        // With banner-anchored extraction, the banner's 4-component tail must win.
        let v = validate_trajectory_tail(content);
        assert!(
            v.is_none(),
            "banner-anchored →9→9→9→9 must return None; got: {v:?}"
        );
    }

    // ── F-P1-009: count_arrow_digit_matches byte-safety doc ─────────────────
    // No behavioral test needed — F-P1-009 is a doc-comment addition.
    // The existing count_arrow_digit tests cover correctness.

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
