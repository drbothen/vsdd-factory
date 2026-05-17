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

/// Maximum bytes to read from STATE.md via `host::read_file`.
///
/// Set to 512 KiB (524288 bytes) — comfortably above the BC-5.39.005 hard cap
/// of 500 lines × ~250 bytes/line average = ~125 KiB, with 4x growth runway.
/// Real STATE.md as of the F-P5-002 fix burst is 95185 bytes (95 KiB), which was
/// 145% of the old 65536-byte cap. The old cap caused `host::read_file` to return
/// `Err(HostError::OutputTooLarge)` (-3), which triggered the fail-open path and
/// rendered the validator silently inert against the real production target.
///
/// F-P5-002: raise from 65536 to 524288.
pub const MAX_BYTES_STATE_MD: u32 = 524_288;

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
/// Scans **only within the SIZE BUDGET banner block** (the `<!-- STATE.md SIZE BUDGET ...
/// -->` HTML comment) to avoid false matches on body-prose mentions of line counts —
/// e.g., Phase Progress rows or historical compaction notes that use the same
/// `N lines (wc-l)` pattern. Within the banner, returns the **last** matching
/// occurrence, since real STATE.md has many interim `N lines (wc-l;` entries in
/// the line-growth tracker and the last one is the most recent canonical claim.
///
/// Returns `None` if no banner block is found, or if no matching pattern is found
/// within the banner block.
///
/// # BC trace
/// BC-5.39.005 invariant 3 — line-count comparison uses newline-character counting.
/// F-P1-001 fix: tolerant terminator; last-occurrence anchor.
/// F-P5-003 fix: banner-block-scoped scan (prevents body-prose false matches).
pub fn extract_banner_line_count(content: &str) -> Option<usize> {
    // Scope to the SIZE BUDGET banner block (F-P5-003). If no banner block is found,
    // return None immediately — the caller (validate_banner_wc) will produce the
    // appropriate "absent banner" violation.
    let scan_target: &str = extract_banner_block(content).unwrap_or("");

    scan_for_last_wc_l(scan_target)
}

/// Hand-rolled scan for the last `(\d+) lines (wc-l<terminator>` pattern within `text`.
///
/// Used by `extract_banner_line_count` to scan the banner block only.
/// Separated into its own function for testability without the banner-extraction layer.
///
/// Scans on raw bytes to avoid regex crate fuel exhaustion. All pattern bytes
/// are ASCII, so no UTF-8 split risk.
fn scan_for_last_wc_l(text: &str) -> Option<usize> {
    let prefix = b" lines (wc-l";
    let prefix_len = prefix.len();

    let bytes = text.as_bytes();
    let text_len = bytes.len();

    if text_len < prefix_len {
        return None;
    }

    let mut last_found: Option<usize> = None;
    let mut i = 0usize;

    while i + prefix_len <= text_len {
        if &bytes[i..i + prefix_len] == prefix {
            // Check that the byte immediately after the prefix is a valid terminator:
            // any byte that is NOT an ASCII letter or digit. This accepts `)`, `;`, `.`, `,`,
            // space, `\n`, etc. — but rejects mid-word matches like "lines (wc-lb..." which
            // could occur if terminology diverges.
            let after = i + prefix_len;
            let valid_terminator = after >= text_len  // prefix at end of text
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
                    if let Ok(n) = text[digit_start..digit_end].parse::<usize>() {
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
/// # F-P5-005 fix
/// Promoted from `fn` to `pub fn` — visibility consistent with sibling extractor helpers
/// (`extract_banner_line_count`, `extract_trajectory_tail_line`, `count_arrow_digit_matches`).
pub fn extract_banner_block(content: &str) -> Option<&str> {
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
/// repeated N times in an ADJACENT run (canonical form: `→N→N→N→N`).
///
/// # Anchoring rule (F-P1-007 fix, tightened in F-P2-001, body-semantics in F-P3-001)
///
/// Real STATE.md prose may contain `→N` sequences in narrative tables, chart rows,
/// and decision-log rows anywhere in the document. To avoid picking up a spurious
/// narrative occurrence, this function first extracts the SIZE BUDGET banner block via
/// `extract_banner_block` and searches ONLY within that block using the
/// `is_trajectory_tail_line` predicate.
///
/// The trajectory tail is identified by a line containing ≥3 arrow-digit components
/// as a TIGHT ADJACENT RUN with a non-digit precursor before the first `→`
/// — distinguishing canonical `→9→9→9→9` from narrative arrows like
/// `(363→310 lines)` (1 component, non-adjacent), table rows like
/// `"Story Status 62→63 merged...65→66"` (multiple components spread across prose), and
/// burst-narrative rows like `Trajectory 11→9→8→7→5` (digit precedes first `→`).
///
/// If the banner block is absent (or contains no qualifying tail line), falls back to
/// scanning the full document — preserving compatibility with fixtures that store the
/// trajectory tail outside the banner comment (e.g., `## Convergence Status` body section).
///
/// **Body fallback uses LAST qualifying line (F-P3-001):**
/// In the real STATE.md document, the body contains both historical trajectory entries
/// with fewer components (e.g., `trajectory →3→3→10` from pass-14) and current
/// entries with the canonical 4-component form (e.g., `trajectory →9→9→9→9` from
/// passes 64-74). Historical entries appear earlier in the document (lower line numbers)
/// than current entries. Using the LAST qualifying line ensures the most recent
/// (canonical current) tail is extracted rather than an earlier historical shorter tail.
///
/// Returns the first qualifying line (trimmed) within the banner block, or the LAST
/// qualifying line in the full document if no banner block match found.
/// Returns `None` if no qualifying trajectory tail is found anywhere.
///
/// # BC trace
/// BC-5.39.005 postcondition 4; invariant 5.
/// F-P2-001: `is_trajectory_tail_line` (adjacent-run discriminator) used instead of
///           weak single-match predicate, preventing false matches on banner narrative arrows.
/// F-P3-001: first-arrow-precursor + last-in-document body-fallback discriminators,
///           preventing body-narrative forms and historical shorter tails from winning.
pub fn extract_trajectory_tail_line(content: &str) -> Option<String> {
    // Prefer the banner-block-anchored scan (F-P1-007).
    if let Some(block) = extract_banner_block(content) {
        for line in block.split('\n') {
            let trimmed = line.trim_end_matches('\r').trim();
            if is_trajectory_tail_line(trimmed) {
                return Some(trimmed.to_string());
            }
        }
    }

    // Fallback: scan full document (for fixtures without a SIZE BUDGET banner,
    // or when real STATE.md stores the trajectory tail in the body section).
    //
    // F-P3-001: Two-pass body scan — canonical-cardinality preferred.
    //
    // Real STATE.md contains both historical shorter-cardinality trajectory entries
    // (e.g., line 81: `trajectory →3→3→10` from pass-14, 3 components) and current
    // canonical 4-component entries (e.g., lines 131-141: `trajectory →9→9→9→9`).
    // The historical entries appear earlier in document order than the current entries.
    //
    // Pass 1: find the FIRST qualifying line with EXACTLY 4 adjacent arrow-digit
    // components. The canonical trajectory tail ALWAYS has exactly 4 components.
    // This skips shorter historical tails without requiring cardinality pre-checking
    // in `validate_trajectory_tail` to distinguish "no tail" from "wrong-cardinality tail".
    //
    // Pass 2 (fallback): if no 4-component line found, return the first qualifying
    // line of any cardinality. This preserves violation reporting for documents that
    // contain ONLY a wrong-cardinality tail (e.g., test fixtures with →9→9→9).
    let mut first_any: Option<String> = None;
    for line in content.split('\n') {
        let trimmed = line.trim_end_matches('\r').trim();
        if is_trajectory_tail_line(trimmed) {
            // Check if this line has exactly 4 adjacent components (canonical cardinality).
            if has_adjacent_arrow_digit_run(trimmed, 4) && count_arrow_digit_matches(trimmed) == 4 {
                // First 4-component canonical tail found — return immediately.
                return Some(trimmed.to_string());
            }
            // Record the first qualifying line of any cardinality for Pass 2 fallback.
            if first_any.is_none() {
                first_any = Some(trimmed.to_string());
            }
        }
    }
    first_any
}

/// Returns `true` if `s` qualifies as a canonical trajectory-tail line.
///
/// # Discriminator (F-P2-001 fix, tightened in F-P3-001)
///
/// The canonical trajectory tail (`→N→N→N→N`) is identified by THREE criteria:
///
/// 1. **Component count ≥ 3**: the line contains at least 3 `→N` sequences.
///    This rejects single-arrow narratives like `(363→310 lines)` (count=1).
///
/// 2. **Adjacent run ≥ 3**: at least 3 of the `→N` sequences appear as a tight
///    consecutive run — each `→N` immediately followed by another `→M` with no
///    intervening non-digit, non-arrow text. This rejects table rows where `→N`
///    sequences are scattered across prose like `"Story Status 62→63 merged...
///    Story Status 66→67"` (count=5 but separated by large amounts of prose text).
///
/// 3. **First-arrow-precursor rule (F-P3-001)**: the FIRST `→` in the qualifying
///    adjacent run must NOT be immediately preceded by an ASCII digit. This is the
///    structural discriminator between:
///    - **Canonical form**: `Trajectory →9→9→9→9` — space before first `→`.
///    - **Narrative form**: `Trajectory 11→9→8→7→5` — digit `1` before first `→`.
///    - **Trend form**: `trend 22→11→16→16` — digit `2` before first `→`.
///
///    The check: when scanning for an adjacent run, the byte immediately before
///    the FIRST arrow of the run must NOT be an ASCII digit (0x30–0x39). If the
///    first arrow is at byte offset 0 (start-of-line), it passes (no preceding byte).
///
/// # Rationale for first-arrow-precursor (F-P3-001)
///
/// The pass-2 fix (adjacent-run discriminator) closed the BANNER-block narrative-arrow
/// class because the banner's D-430(a) compaction line `(363→310 lines)` has only 1
/// adjacent component. However, the BODY-document narrative class (`Trajectory 11→9→8→7→5`,
/// `trend 22→11→16→...`) forms a tight adjacent run of ≥3 components — structurally
/// indistinguishable from canonical `→9→9→9→9` under criterion 2 alone.
///
/// The first-arrow-precursor criterion cleanly separates these classes because:
/// - Canonical trajectory tail ALWAYS starts with a bare `→` (preceded by whitespace
///   after the "Trajectory" keyword, or at line start in bare form `→N→N→N→N`).
/// - Narrative burst/trend forms ALWAYS start with a number (e.g. `11→`, `22→`)
///   because they describe a sequence of COUNTS.
///
/// This is a stronger, context-free structural discriminator that requires no
/// section-anchor dependency.
///
/// # Examples that qualify
/// - `→9→9→9→9` — bare canonical, 4 adjacent, no preceding byte
/// - `Trajectory →9→9→9→9` — prefixed canonical, space before first `→`
/// - `→9→9→9` — 3-component (count wrong for validity, but IS a tail structurally)
///
/// # Examples that do NOT qualify
/// - `(363→310 lines)` — count=1, too few components
/// - `"Story Status 62→63 merged...Story Status 66→67"` — spread arrows
/// - `Trajectory 11→9→8→7→5` — digit `1` precedes first `→` (F-P3-001)
/// - `trend 22→11→16→16→12→2→1→4→5` — digit `2` precedes first `→` (F-P3-001)
pub fn is_trajectory_tail_line(s: &str) -> bool {
    // Quick check: need at least 3 →N matches total.
    if count_arrow_digit_matches(s) < 3 {
        return false;
    }
    // Full check: at least 3 must be adjacent with first-arrow-precursor rule.
    has_adjacent_arrow_digit_run(s, 3)
}

/// Returns `true` if `s` contains a run of at least `min_run` consecutive adjacent
/// `→N` sequences — each directly followed by another `→M` with only digit characters
/// (the trailing digits of the previous match) between them — AND the FIRST arrow of
/// that run is NOT immediately preceded by an ASCII digit (F-P3-001 first-arrow-precursor
/// rule).
///
/// # F-P5-005 fix
/// Promoted from `fn` to `pub fn` — visibility consistent with sibling helpers.
///
/// The canonical trajectory tail `→9→9→9→9` satisfies `min_run=3` (and 4) and the
/// first-arrow-precursor rule (no byte before the first `→`).
///
/// `Trajectory →9→9→9→9` satisfies the rule (space `0x20` precedes first `→`).
///
/// `Trajectory 11→9→8→7→5` does NOT satisfy the rule: digit `0x31` (`1`) precedes
/// the first `→` at `11→`, even though the run is 4 adjacent components.
///
/// A prose table row `"...62→63 merged...65→66..."` does NOT: the arrows are separated
/// by non-digit, non-arrow text (adjacency check fails before precursor check).
///
/// # Algorithm
///
/// Walk the string looking for `→[digits]` tokens. Track the byte position after the
/// last token ends. If the next `→[digits]` starts immediately (byte-adjacent, no gap
/// chars), increment the current run length.
///
/// **First-arrow-precursor (F-P3-001):** When starting a NEW run (current_run == 0,
/// i.e., the first token of a candidate run), check the byte immediately before the
/// arrow (`bytes[i - 1]` if `i > 0`). If that byte is an ASCII digit, this cannot
/// be the start of a canonical trajectory run — skip this token and reset run state.
///
/// A run is broken when any character that is not `→` appears between two `→` sequences
/// (after the digits of the prior match are consumed).
///
/// # UTF-8 safety
/// Arrow `→` is U+2192 = `[0xE2, 0x86, 0x92]`. ASCII digits (0x30–0x39) have the
/// high bit clear and cannot be UTF-8 continuation bytes — safe to check via
/// `bytes[pos].is_ascii_digit()`. The arrow byte-sequence check is identical to
/// `count_arrow_digit_matches`.
///
/// The preceding-byte check (`bytes[i - 1].is_ascii_digit()`) is safe: ASCII digit bytes
/// are single-byte UTF-8 code points, so `bytes[i - 1]` is guaranteed to be the complete
/// preceding scalar value when it is a digit.
///
/// # BC trace
/// F-P3-001: first-arrow-precursor discriminator; closes BODY-document narrative-arrow class.
pub fn has_adjacent_arrow_digit_run(s: &str, min_run: usize) -> bool {
    let arrow_utf8 = "\u{2192}".as_bytes(); // [0xE2, 0x86, 0x92]
    let arrow_byte_len = arrow_utf8.len(); // 3
    let bytes = s.as_bytes();
    let len = bytes.len();

    let mut best_run = 0usize;
    let mut current_run = 0usize;
    let mut i = 0usize;

    while i + arrow_byte_len <= len {
        if &bytes[i..i + arrow_byte_len] == arrow_utf8 {
            // Arrow found at position i.
            let after_arrow = i + arrow_byte_len;
            // Must be followed by at least one ASCII digit.
            if after_arrow < len && bytes[after_arrow].is_ascii_digit() {
                // F-P3-001: first-arrow-precursor rule.
                // If this is the START of a new run (current_run == 0), verify that the
                // byte immediately before this arrow is NOT an ASCII digit.
                // Narrative forms like `11→9→8→7→5` have digit `1` at bytes[i-1].
                // Canonical forms like `→9→9→9→9` or `Trajectory →9→9→9→9` have a
                // non-digit (or no byte) before the first arrow.
                if current_run == 0 && i > 0 && bytes[i - 1].is_ascii_digit() {
                    // First arrow of this candidate run is preceded by a digit —
                    // this is a narrative form, not a canonical trajectory run.
                    // Skip this token entirely and reset run state.
                    // Advance past the arrow and its digits to avoid re-matching.
                    let mut j = after_arrow;
                    while j < len && bytes[j].is_ascii_digit() {
                        j += 1;
                    }
                    current_run = 0;
                    i = j;
                    continue;
                }

                // Consume all trailing digits.
                let mut j = after_arrow;
                while j < len && bytes[j].is_ascii_digit() {
                    j += 1;
                }
                // This `→N` token runs from i to j — valid component of a canonical run.
                current_run += 1;
                if current_run > best_run {
                    best_run = current_run;
                }
                if best_run >= min_run {
                    return true;
                }
                // The run continues only if the NEXT bytes immediately form another →N.
                // Set i to j and loop — the next iteration checks for `→` at j.
                i = j;
                // Do NOT increment i again at the bottom of the loop.
                continue;
            } else {
                // Arrow not followed by digit — not a trajectory component; break run.
                current_run = 0;
            }
        } else {
            // Non-arrow character — any character that breaks adjacency resets the run.
            current_run = 0;
        }
        i += 1;
    }

    best_run >= min_run
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
/// # Mixed-paradigm inner loop (F-P2-005)
///
/// The outer loop uses `char_indices()` (decoded `char` + byte position) for clarity.
/// The inner digit-walk loop uses raw byte indexing: `bytes[j].is_ascii_digit()`.
/// This is deliberate and safe: ASCII digit bytes (0x30–0x39) are single-byte UTF-8
/// code points whose byte values are identical to their Unicode scalar values, so
/// `bytes[j].is_ascii_digit()` is equivalent to `s[j..].chars().next() == Some(c if c.is_ascii_digit())`.
/// Re-decoding UTF-8 in the inner loop would add unnecessary overhead without
/// changing correctness — the digit walk only advances through 0x30–0x39 bytes which
/// are never UTF-8 continuation bytes.
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
///
/// Each violation is formatted as:
/// ```text
///   - {description}
///       cited: "{cited_raw}"
/// ```
/// The `cited_raw` field is included when non-empty, making the block message
/// actionable by quoting the exact offending text the author wrote.
///
/// # F-P2-003 Option A
/// `cited_raw` is wired through `emit_block` rather than being a dead field.
/// The `cited_raw` text appears in the block reason for every violation that
/// populates it. This makes the field load-bearing (TD-VSDD-059 paper-fix avoidance).
fn emit_block(violations: &[Violation]) -> HookResult {
    let lines: Vec<String> = violations
        .iter()
        .map(|v| {
            if v.cited_raw.is_empty() {
                format!("  - {}", v.description)
            } else {
                format!("  - {}\n      cited: \"{}\"", v.description, v.cited_raw)
            }
        })
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
    // F-P5-002: max_bytes raised from 65536 to MAX_BYTES_STATE_MD (512 KiB).
    // The old 64 KiB cap was below real STATE.md size (95 KiB as of F-P5-002 fix
    // burst), causing host::read_file to return Err(OutputTooLarge) (-3) and
    // the validator to fail-open silently — rendering it inert against production.
    // On read failure: fail-open (Continue + log_warn) per BC-5.39.005 postcondition 6.
    let content = match host::read_file(&file_path, MAX_BYTES_STATE_MD, 2000) {
        Ok(bytes) => {
            // Truncation sentinel: if the returned slice is exactly MAX_BYTES_STATE_MD bytes,
            // the host may have truncated. Emit a warn so the truncation is auditable in logs.
            if bytes.len() as u32 == MAX_BYTES_STATE_MD {
                host::log_warn(&format!(
                    "[validate-state-structure] read_file returned exactly {MAX_BYTES_STATE_MD} bytes \
                     for {file_path} — possible truncation at byte budget; \
                     consider raising MAX_BYTES_STATE_MD if STATE.md has grown beyond 512 KiB"
                ));
            }
            match String::from_utf8(bytes) {
                Ok(s) => s,
                Err(e) => {
                    host::log_warn(&format!(
                        "[validate-state-structure] UTF-8 decode failure reading {file_path}: {e}"
                    ));
                    return HookResult::Continue;
                }
            }
        }
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
        // F-P5-003: must include STATE.md SIZE BUDGET marker so extract_banner_block finds it.
        let content = "<!--\n  STATE.md SIZE BUDGET (per D-421(c)):\n  28 lines (wc-l). Hard cap: 500\n-->\nrest\n";
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
        // EC-002: banner claims 28 but file has 4 newlines — mismatch violation.
        // F-P5-003: banner must include STATE.md SIZE BUDGET marker.
        let content = "<!--\n  STATE.md SIZE BUDGET (per D-421(c)):\n  28 lines (wc-l).\n-->\n"; // 4 newlines, claim 28
        // This should produce Some(Violation) because 28 != 4
        let v = validate_banner_wc(content);
        assert!(
            v.is_some(),
            "mismatch (claimed 28, actual 4) should be Some"
        );
    }

    #[test]
    fn test_BC_5_39_005_banner_wc_correct_returns_none() {
        // F-P5-003: banner must include STATE.md SIZE BUDGET marker.
        // Build content where banner line count == actual newline count.
        // The banner block itself has 3 newlines; add 1 more filler line = 4 total.
        // But we need the claimed count to equal total newlines in the full content.
        // Full content: "<!--\n  STATE.md SIZE BUDGET...\n  4 lines (wc-l).\n-->\n" = 4 newlines.
        let content = "<!--\n  STATE.md SIZE BUDGET (per D-421(c)):\n  4 lines (wc-l).\n-->\n";
        // Count newlines: 4 (one per line of the HTML comment block)
        assert_eq!(count_newlines(content), 4);
        let v = validate_banner_wc(content);
        assert!(v.is_none(), "exact match should return None");
    }

    #[test]
    fn test_BC_5_39_005_banner_wc_off_by_one_names_both_counts() {
        // EC-001: banner claims 27 but file has 28 newlines.
        // F-P5-003: banner must include STATE.md SIZE BUDGET marker so extract_banner_block
        // can locate the banner and extract the wc-l claim from it.
        // Banner line: "<!--\n  STATE.md SIZE BUDGET (per D-421(c)):\n  27 lines (wc-l).\n-->\n"
        // = 4 newlines. Add 24 more filler lines = 28 total newlines. Banner claims 27.
        let mut content = String::new();
        content.push_str("<!--\n  STATE.md SIZE BUDGET (per D-421(c)):\n  27 lines (wc-l).\n-->\n");
        for _ in 0..24 {
            content.push_str("line\n");
        }
        // Total: 4 (banner) + 24 (filler) = 28 newlines. Banner claims 27.
        assert_eq!(
            count_newlines(&content),
            28,
            "test precondition: must have 28 newlines"
        );
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
    /// F-P5-003: banner must include STATE.md SIZE BUDGET marker.
    #[test]
    fn test_BC_5_39_005_extract_banner_line_count_semicolon_terminator() {
        let content = "<!--\n  STATE.md SIZE BUDGET (per D-421(c)):\n  395 lines (wc-l; net -52 from pass-64).\n-->\nrest\n";
        let result = extract_banner_line_count(content);
        assert_eq!(result, Some(395), "semicolon terminator must be accepted");
    }

    /// F-P1-001: extract_banner_line_count must accept comma terminator
    /// F-P5-003: banner must include STATE.md SIZE BUDGET marker.
    #[test]
    fn test_BC_5_39_005_extract_banner_line_count_comma_terminator() {
        let content =
            "<!--\n  STATE.md SIZE BUDGET (per D-421(c)):\n  428 lines (wc-l, net +5)\n-->\nrest\n";
        let result = extract_banner_line_count(content);
        assert_eq!(result, Some(428), "comma terminator must be accepted");
    }

    /// F-P1-001: extract_banner_line_count must accept close-paren terminator (original form)
    /// F-P5-003: banner must include STATE.md SIZE BUDGET marker.
    #[test]
    fn test_BC_5_39_005_extract_banner_line_count_paren_terminator() {
        let content = "<!--\n  STATE.md SIZE BUDGET (per D-421(c)):\n  2 lines (wc-l)\n-->\n";
        let result = extract_banner_line_count(content);
        assert_eq!(result, Some(2), "close-paren terminator must be accepted");
    }

    /// F-P1-001: all four terminators accepted in a single-function test.
    /// F-P5-003: banner must include STATE.md SIZE BUDGET marker for extract_banner_block.
    #[test]
    fn test_BC_5_39_005_extract_banner_line_count_all_terminators() {
        for (terminator, input, expected) in [
            (
                ")",
                "<!--\n  STATE.md SIZE BUDGET:\n  10 lines (wc-l)\n-->",
                Some(10usize),
            ),
            (
                ";",
                "<!--\n  STATE.md SIZE BUDGET:\n  20 lines (wc-l; net +2)\n-->",
                Some(20),
            ),
            (
                ".",
                "<!--\n  STATE.md SIZE BUDGET:\n  30 lines (wc-l). rest\n-->",
                Some(30),
            ),
            (
                ",",
                "<!--\n  STATE.md SIZE BUDGET:\n  40 lines (wc-l, net +4)\n-->",
                Some(40),
            ),
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

    // ── F-P2-001: tighter trajectory predicate (canonical-tail discriminator) ──

    /// F-P2-001: a single-narrative arrow `(363→310 lines)` must NOT qualify as a
    /// trajectory tail line. The canonical tail requires adjacent →N→N runs.
    #[test]
    fn test_BC_5_39_005_f_p2_001_narrative_arrow_single_not_trajectory() {
        // Simulates the D-430(a) compaction line in the real STATE.md banner:
        // "D-430(a) compaction authorization: Pass-49 Commit E surgical compaction
        //  (363→310 lines) authorized retroactively per D-430(a)..."
        // This has exactly 1 →N match — must NOT be identified as the trajectory tail.
        let content = concat!(
            "<!--\n",
            "  STATE.md SIZE BUDGET (per D-421(c)):\n",
            "  Hard cap (500 lines) margin from soft-target = 500 - 415 = 85; margin from actual = 500 - 29 = 471 (D-446(c) dual-margin form).\n",
            "  D-430(a) compaction authorization: Pass-49 Commit E surgical compaction (363→310 lines) authorized retroactively.\n",
            "-->\n",
            "\n",
            "# Pipeline State\n",
            "\n",
            "## Convergence Status\n",
            "\n",
            "Trajectory →9→9→9→9\n",
            "\n",
        );
        // With the (363→310) line in banner, the banner scan must NOT pick it up as
        // trajectory tail (only 1 non-adjacent component). Fallback must find the body tail.
        let tail = extract_trajectory_tail_line(content);
        assert!(
            tail.is_some(),
            "should find trajectory tail in body; got None"
        );
        let tail_line = tail.unwrap();
        assert!(
            !tail_line.contains("363"),
            "must NOT return the D-430(a) compaction line as trajectory tail; got: {tail_line:?}"
        );
        // The correct tail has 4 components
        assert_eq!(
            count_arrow_digit_matches(&tail_line),
            4,
            "found tail should have 4 components; got: {tail_line:?}"
        );
    }

    /// F-P2-001: table-row narrative with spread arrows (e.g. "Story Status 62→63 merged ...
    /// Story Status 66→67") must NOT be identified as the trajectory tail even though
    /// it has >=3 →N matches. The discriminator must require ADJACENT →N→N sequences.
    #[test]
    fn test_BC_5_39_005_f_p2_001_spread_table_arrows_not_trajectory() {
        // Simulate the real STATE.md banner tracker line: multiple →N matches but
        // separated by long prose text (not adjacent).
        let spread_line = "Line-growth tracker: pass-49 310 lines; Story Status 62→63 merged + Session Resume Checkpoint refresh net +9). S-15.04-post-merge-burst 480 lines; Story Status 63→64 merged; Story Status 64→65 merged; Story Status 65→66; Story Status 66→67";
        // Count: 5 →N matches, but they are spread far apart
        assert!(
            count_arrow_digit_matches(spread_line) >= 3,
            "test precondition: spread_line must have >=3 →N matches"
        );
        // But is_trajectory_tail_line must reject it
        assert!(
            !is_trajectory_tail_line(spread_line),
            "spread-arrow prose table row must NOT qualify as trajectory tail"
        );
    }

    /// F-P2-001: canonical trajectory forms must qualify as trajectory tail lines.
    #[test]
    fn test_BC_5_39_005_f_p2_001_canonical_trajectory_forms_qualify() {
        // Standard 4-component form
        assert!(
            is_trajectory_tail_line("→9→9→9→9"),
            "→9→9→9→9 must qualify as trajectory tail"
        );
        // With "Trajectory " prefix
        assert!(
            is_trajectory_tail_line("Trajectory →9→9→9→9"),
            "Trajectory →9→9→9→9 must qualify as trajectory tail"
        );
        // 3-component (invalid count but IS a tail line structurally)
        assert!(
            is_trajectory_tail_line("→9→9→9"),
            "→9→9→9 must qualify as trajectory tail (count wrong but is a tail)"
        );
        // 5-component
        assert!(
            is_trajectory_tail_line("→9→9→9→9→9"),
            "→9→9→9→9→9 must qualify as trajectory tail"
        );
        // Multi-digit numbers
        assert!(
            is_trajectory_tail_line("→12→34→56→78"),
            "→12→34→56→78 must qualify as trajectory tail"
        );
    }

    // ── F-P2-002: full-surface validation against real STATE.md ──────────────

    /// F-P2-002: load the actual .factory/STATE.md and assert that ALL THREE
    /// validators (validate_banner_wc, validate_dual_margin, validate_trajectory_tail)
    /// return None. This is the LOAD-BEARING end-to-end proof that the implementation
    /// correctly handles real-world STATE.md content without false-positive blocks.
    ///
    /// The test skips gracefully if STATE.md is not found (isolated build environments).
    #[test]
    fn test_BC_5_39_005_full_validation_against_real_state_md() {
        let state_md_path =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../.factory/STATE.md");

        let content = match std::fs::read_to_string(&state_md_path) {
            Ok(c) => c,
            Err(_) => {
                eprintln!(
                    "[skip] real STATE.md not found at {:?} — skipping full-surface test",
                    state_md_path
                );
                return;
            }
        };

        // Validator 1: banner wc-l
        let banner_viol = validate_banner_wc(&content);
        assert!(
            banner_viol.is_none(),
            "validate_banner_wc must return None for real STATE.md; got: {:?}",
            banner_viol.as_ref().map(|v| &v.description)
        );

        // Validator 2: dual-margin
        let margin_viol = validate_dual_margin(&content);
        assert!(
            margin_viol.is_none(),
            "validate_dual_margin must return None for real STATE.md; got: {:?}",
            margin_viol.as_ref().map(|v| &v.description)
        );

        // Validator 3: trajectory tail cardinality
        let tail_viol = validate_trajectory_tail(&content);
        assert!(
            tail_viol.is_none(),
            "validate_trajectory_tail must return None for real STATE.md; got: {:?} (found tail: {:?})",
            tail_viol.as_ref().map(|v| &v.description),
            extract_trajectory_tail_line(&content)
        );
    }

    // ── F-P3-001: first-arrow-precursor discriminator (body-narrative BODY class) ─

    /// F-P3-001: `Trajectory 11→9→8→7→5` has a DIGIT immediately before the first
    /// `→`. This is the BODY-document narrative-arrow class identified in pass-3.
    /// The current predicate would false-positive-block if this line appeared before
    /// the canonical tail in STATE.md (partial-fix regression from pass-2).
    ///
    /// `is_trajectory_tail_line` MUST return false for this input.
    #[test]
    fn test_BC_5_39_005_f_p3_001_narrative_arrow_burst_not_trajectory() {
        // "Trajectory 11→9→8→7→5" — digit immediately before first →
        // This is verbatim from real STATE.md line 69.
        let narrative_4 = "Trajectory 11\u{2192}9\u{2192}8\u{2192}7\u{2192}5";
        assert!(
            !is_trajectory_tail_line(narrative_4),
            "narrative form with digit-before-first-arrow must NOT qualify as trajectory tail; \
             input: {narrative_4:?}"
        );
    }

    /// F-P3-001: high-count narrative `trend 22→11→16→16→12→2→1→4→5` (9 components,
    /// digit-before-arrow) must NOT be matched as trajectory tail.
    /// Verbatim from real STATE.md line 90+ ("trend" prefix, digit-before-first-arrow).
    #[test]
    fn test_BC_5_39_005_f_p3_001_high_count_narrative_not_trajectory() {
        // "trend 22→11→16→16→12→2→1→4→5" — digit 22 before first →
        // From real STATE.md E-10 pass-9 adversary row
        let narrative_9 =
            "trend 22\u{2192}11\u{2192}16\u{2192}16\u{2192}12\u{2192}2\u{2192}1\u{2192}4\u{2192}5";
        assert!(
            !is_trajectory_tail_line(narrative_9),
            "high-count narrative with digit-before-first-arrow must NOT qualify; \
             input: {narrative_9:?}"
        );
    }

    /// F-P3-001: sibling-site check — `extract_trajectory_tail_line` applied to STATE.md
    /// body content that has `Trajectory 11→9→8→7→5` BEFORE the canonical
    /// `Trajectory →9→9→9→9` must return the canonical tail (not the narrative one).
    ///
    /// Regression-prevention: this tests the full-document fallback path.
    #[test]
    fn test_BC_5_39_005_f_p3_001_body_narrative_before_canonical_finds_canonical() {
        // Simulate body content: narrative line first (line 69 position), canonical second.
        // No SIZE BUDGET banner — forces full-document fallback path.
        let content = concat!(
            "# heading\n",
            "\n",
            "| F5 passes 3-7 | **COMPLETE** | Trajectory 11\u{2192}9\u{2192}8\u{2192}7\u{2192}5; verdict MEDIUM |\n",
            "\n",
            "## Convergence Status\n",
            "\n",
            "Trajectory \u{2192}9\u{2192}9\u{2192}9\u{2192}9\n",
            "\n",
        );
        let tail = extract_trajectory_tail_line(content);
        assert!(
            tail.is_some(),
            "should find the canonical trajectory tail in body"
        );
        let tail_line = tail.unwrap();
        assert!(
            !tail_line.contains("11"),
            "must NOT return the narrative '11→9→8→7→5' line as trajectory tail; \
             got: {tail_line:?}"
        );
        assert_eq!(
            count_arrow_digit_matches(&tail_line),
            4,
            "found tail must have exactly 4 components; got: {tail_line:?}"
        );
    }

    /// F-P3-001: synthetic variant of `test_BC_5_39_005_full_validation_against_real_state_md`
    /// with an injected `Trajectory 11→9→8→7→5` line in the body BEFORE the canonical tail.
    /// This is the regression-prevention test per the finding spec.
    ///
    /// Construct in-memory content: use real STATE.md if available and inject the narrative
    /// line before the canonical tail; otherwise construct a minimal representative fixture.
    #[test]
    fn test_BC_5_39_005_f_p3_001_injected_narrative_does_not_displace_canonical_tail() {
        let state_md_path =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../.factory/STATE.md");

        // Build the content to test against:
        // If real STATE.md is available, prepend a narrative line just before any existing
        // "Trajectory →" line so the narrative form appears first in document order.
        // If not available, construct a minimal representative fixture.
        let injected_narrative = "| injected-test | **COMPLETE** | Trajectory 11\u{2192}9\u{2192}8\u{2192}7\u{2192}5; burst-not-tail |\n";
        let content = match std::fs::read_to_string(&state_md_path) {
            Ok(real_content) => {
                // Find the canonical "Trajectory →" line position in the document and inject
                // the narrative form immediately before it.
                // We search for "Trajectory \u{2192}" (canonical leading space-then-arrow).
                let needle = "Trajectory \u{2192}";
                if let Some(pos) = real_content.find(needle) {
                    // Find the start of the line containing `needle`.
                    let line_start = real_content[..pos].rfind('\n').map(|p| p + 1).unwrap_or(0);
                    let mut modified = real_content[..line_start].to_string();
                    modified.push_str(injected_narrative);
                    modified.push_str(&real_content[line_start..]);
                    modified
                } else {
                    // Canonical form not found — construct minimal fixture.
                    format!(
                        "<!--\n  STATE.md SIZE BUDGET (per D-421(c)):\n  \
                         Hard cap (500 lines) margin from soft-target = 500 - 415 = 85; \
                         margin from actual = 500 - 10 = 490 (D-446(c) dual-margin form).\n  \
                         Trajectory \u{2192}9\u{2192}9\u{2192}9\u{2192}9\n-->\n\
                         {injected_narrative}\
                         Trajectory \u{2192}9\u{2192}9\u{2192}9\u{2192}9\n"
                    )
                }
            }
            Err(_) => {
                // No real STATE.md — construct minimal representative fixture.
                format!(
                    "<!--\n  STATE.md SIZE BUDGET (per D-421(c)):\n  \
                     Hard cap (500 lines) margin from soft-target = 500 - 415 = 85; \
                     margin from actual = 500 - 10 = 490 (D-446(c) dual-margin form).\n  \
                     Trajectory \u{2192}9\u{2192}9\u{2192}9\u{2192}9\n-->\n\
                     {injected_narrative}\
                     Trajectory \u{2192}9\u{2192}9\u{2192}9\u{2192}9\n"
                )
            }
        };

        // The injected narrative line must NOT cause validate_trajectory_tail to fire.
        // If it correctly skips the narrative form, it finds the canonical tail (or None
        // if there truly is no canonical tail in the fixture — but we know there is).
        let tail = extract_trajectory_tail_line(&content);
        assert!(
            tail.is_some(),
            "should still find the canonical trajectory tail after narrative injection"
        );
        let tail_line = tail.unwrap();
        assert!(
            !tail_line.contains("11"),
            "extracted tail must NOT be the injected narrative '11→9→8→7→5' line; \
             got: {tail_line:?}"
        );
        // The canonical tail must have exactly 4 components.
        assert_eq!(
            count_arrow_digit_matches(&tail_line),
            4,
            "canonical tail must have 4 components; got: {tail_line:?}"
        );
    }

    // ── F-P2-003: cited_raw wired into emit_block reason ─────────────────────

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
                // F-P2-003 Option A: cited_raw text must appear in the reason for violations that have it.
                assert!(
                    reason.contains("27 lines (wc-l)"),
                    "cited_raw '27 lines (wc-l)' must appear in block reason; got:\n{reason}"
                );
                assert!(
                    reason.contains("\u{2192}9\u{2192}9\u{2192}9"),
                    "cited_raw arrow sequence must appear in block reason; got:\n{reason}"
                );
            }
            _ => panic!("expected Block result, got {result:?}"),
        }
    }

    // ── Real-STATE.md integration test (F-P1-001 structural closure) ─────────

    /// F-P1-001 structural proof: read the actual .factory/STATE.md from the workspace
    /// root (relative path `../../../../.factory/STATE.md` from the crate directory)
    /// and verify that `extract_banner_line_count` successfully extracts the banner
    /// line-count claim and that it matches the actual newline count.
    ///
    /// This test fails if:
    /// - The real STATE.md has no `lines (wc-l...` pattern (banner absent or stale)
    /// - The extracted count does not match the actual newline count
    ///
    /// The test skips gracefully if the file cannot be read (e.g., running tests from
    /// a different working tree where `.factory/STATE.md` does not exist).
    #[test]
    fn test_BC_5_39_005_f_p1_001_real_state_md_banner_wc_passes() {
        // Path: from crate root (vsdd-factory/crates/hook-plugins/validate-state-structure/)
        // up four levels to workspace root, then into .factory/STATE.md.
        let state_md_path =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../.factory/STATE.md");

        let content = match std::fs::read_to_string(&state_md_path) {
            Ok(c) => c,
            Err(_) => {
                // Skip gracefully — not all build environments have the factory worktree mounted.
                eprintln!(
                    "[skip] real STATE.md not found at {:?} — skipping integration test",
                    state_md_path
                );
                return;
            }
        };

        // F-P1-001: extract_banner_line_count must find a value (tolerant terminator).
        let claimed = extract_banner_line_count(&content).unwrap_or_else(|| {
            panic!(
                "extract_banner_line_count returned None for real STATE.md — \
                 F-P1-001 is NOT closed; banner has no 'N lines (wc-l...)' pattern. \
                 Last 3 wc-l occurrences in file: check grep output."
            )
        });

        let actual = count_newlines(&content);

        assert_eq!(
            claimed, actual,
            "real STATE.md banner claims {claimed} lines but actual count is {actual} — \
             the banner wc-l is stale; update STATE.md banner before committing"
        );
    }

    // ── F-P5-003: banner-scoped extract_banner_line_count ────────────────────

    /// F-P5-003: `extract_banner_line_count` must NOT return a `lines (wc-l` value
    /// from the document body — only from the SIZE BUDGET banner block.
    ///
    /// Regression: if a body row mentions "42 lines (wc-l)" (e.g., a Phase Progress
    /// row or historical note) and that appears AFTER the banner's "28 lines (wc-l)",
    /// the full-document last-occurrence scan would return 42 instead of 28.
    /// Banner-scoped scanning must return 28.
    #[test]
    fn test_BC_5_39_005_f_p5_003_body_wc_l_does_not_displace_banner_wc_l() {
        // Banner claims 28 lines (wc-l). Body contains 42 lines (wc-l) in prose.
        let content = concat!(
            "<!--\n",
            "  STATE.md SIZE BUDGET (per D-421(c)):\n",
            "  Hard cap (500 lines) margin from soft-target = 500 - 415 = 85;\n",
            "  margin from actual = 500 - 28 = 472 (D-446(c) dual-margin form).\n",
            "  28 lines (wc-l).\n",
            "-->\n",
            "\n",
            "# Pipeline State\n",
            "\n",
            "## Phase Progress\n",
            "\n",
            "Historical note: compact from 42 lines (wc-l) at pass-15 was authorized.\n",
            "\n",
        );
        let result = extract_banner_line_count(content);
        assert_eq!(
            result,
            Some(28),
            "extract_banner_line_count must return banner value (28), not body value (42); \
             F-P5-003: banner-scoped scanning required"
        );
    }

    // ── F-P5-002: oversize STATE.md load-bearing regression test ─────────────

    /// F-P5-002: validates that all three validators correctly process content larger
    /// than the OLD 64 KiB `max_bytes` cap (65536 bytes), AND that `MAX_BYTES_STATE_MD`
    /// is set to at least 512 KiB.
    ///
    /// `on_post_tool_use` is not directly testable from unit tests (requires the WASM
    /// host SDK shim). This test:
    ///   1. Builds valid synthetic STATE.md content > 65536 bytes.
    ///   2. Verifies all three validators return None (no false-positive violation).
    ///   3. Verifies `MAX_BYTES_STATE_MD >= 524288` via a compile-time const assert —
    ///      this test would fail to compile if the constant were lowered below 512 KiB.
    ///
    /// The load-bearing bats evidence that the host::read_file path is exercised
    /// (not silently fail-opened) is in `pass-real-state-md-snapshot.bats` (mutation
    /// verified in the fix-burst test-the-test step).
    #[test]
    fn test_BC_5_39_005_f_p5_002_oversize_state_md_full_validation() {
        // Compile-time assertion: MAX_BYTES_STATE_MD must be >= 524288 (512 KiB).
        // This is the load-bearing constant check that closes F-P5-002.
        // If someone lowers the cap below 512 KiB, this line fails to compile.
        const _: () = assert!(
            MAX_BYTES_STATE_MD >= 524_288,
            "MAX_BYTES_STATE_MD must be >= 524288 (512 KiB) per F-P5-002"
        );

        // Build synthetic STATE.md content of 1025 lines (> 65536 bytes at ~77 bytes/line).
        // Strategy: build banner first (7 lines), then fill remaining lines with a known filler.
        // Banner wc-l claim is set equal to the total line count.
        let filler =
            "This is a padding line for the oversize regression test with valid content.\n";
        // Banner lines:
        // 1. <!--
        // 2.   STATE.md SIZE BUDGET (per D-421(c)):
        // 3.   Hard cap (500 lines) margin from soft-target = ... (D-446(c) dual-margin form).
        // 4.   NNNN lines (wc-l).
        // 5.   Trajectory →9→9→9→9
        // 6. -->
        // Total banner newlines = 6.
        let banner_newlines: usize = 6;
        let total_line_count: usize = 1025;
        let filler_count = total_line_count - banner_newlines;

        // Compute sizes: 1025 × 77 bytes/line ≈ 78925 bytes > 65536 (OLD cap).
        let approx_bytes = total_line_count * filler.len();
        assert!(
            approx_bytes > 65536,
            "test precondition: approximate byte size {approx_bytes} must exceed \
             OLD 65536-byte cap"
        );

        let margin = 500usize.saturating_sub(total_line_count);
        let banner = format!(
            "<!--\n\
             STATE.md SIZE BUDGET (per D-421(c)):\n\
             Hard cap (500 lines) margin from soft-target = 500 - 415 = 85; \
             margin from actual = 500 - {total_line_count} = {margin} \
             (D-446(c) dual-margin form).\n\
             {total_line_count} lines (wc-l).\n\
             Trajectory \u{2192}9\u{2192}9\u{2192}9\u{2192}9\n\
             -->\n"
        );
        assert_eq!(
            count_newlines(&banner),
            banner_newlines,
            "test construction: banner must have exactly {banner_newlines} newlines"
        );

        let mut final_content = banner;
        for _ in 0..filler_count {
            final_content.push_str(filler);
        }
        let final_count = count_newlines(&final_content);
        assert_eq!(
            final_count, total_line_count,
            "test construction: final content must have {total_line_count} newlines; \
             got {final_count}"
        );
        assert!(
            final_content.len() > 65536,
            "test construction: final content must exceed OLD 65536-byte cap; \
             actual: {} bytes",
            final_content.len()
        );

        // Run all three validators. They must return None (no violation).
        let banner_viol = validate_banner_wc(&final_content);
        assert!(
            banner_viol.is_none(),
            "validate_banner_wc must return None for oversize valid content; \
             got: {:?}",
            banner_viol.as_ref().map(|v| &v.description)
        );
        let margin_viol = validate_dual_margin(&final_content);
        assert!(
            margin_viol.is_none(),
            "validate_dual_margin must return None for oversize valid content; \
             got: {:?}",
            margin_viol.as_ref().map(|v| &v.description)
        );
        let tail_viol = validate_trajectory_tail(&final_content);
        assert!(
            tail_viol.is_none(),
            "validate_trajectory_tail must return None for oversize valid content; \
             got: {:?}",
            tail_viol.as_ref().map(|v| &v.description)
        );
    }
}
