//! validate-dispatch-advance — PostToolUse WASM hook plugin.
//!
//! Blocks any Edit/Write to `STATE.md` whose `current_step:` frontmatter field
//! contains structural violations, and any Edit/Write to `INDEX.md` containing
//! adversary-pass rows with the wrong column count.
//!
//! # Validation gates (STATE.md arm)
//!
//! 1. **Forbidden meta-commentary** (D-440(a)+D-441(a)+D-442(a)): patterns
//!    `META-LEVEL-\d+ WATCH`, `self-app TEST`, `expected verdict` must not
//!    appear in the `current_step:` frontmatter value.
//!
//! 2. **4-index version citations** (D-439(b)): all four literal substrings
//!    `BC-INDEX v`, `VP-INDEX v`, `STORY-INDEX v`, `ARCH-INDEX v` must be
//!    present in `current_step:`.
//!
//! 3. **Trajectory-tail LENGTH=4** (D-451(c)): the global regex `→(\d+)` must
//!    match exactly 4 times in the `current_step:` value.
//!
//! 4. **D-chain cite currency** (D-443(a)): `current_step:` must contain at
//!    least one `D-\d+` reference. The maximum D-NNN integer found in
//!    `current_step:` must be >= the maximum D-NNN integer visible anywhere
//!    in STATE.md. The literal `D-382..D-` range prefix is NOT required;
//!    prose forms like `D-chain cite D-476` are accepted (BC-5.39.006 v1.1).
//!
//! # Validation gate (INDEX.md arm)
//!
//! 5. **5-column adversary-pass rows** (D-441(b)/D-442(b)): within the
//!    `## Adversarial Reviews` h2 section only, rows must have 6 pipe characters
//!    (1 leading + 4 internal + 1 trailing = 5 columns). Rows under other h2
//!    headings are not validated. Separator rows excluded. Historical 4-column
//!    sections (5-pipe header) are grandfathered and skipped entirely.
//!
//! # Behavioral Contracts
//!
//! - BC-5.39.006: blocks dispatch-advance structural violations.
//!
//! # D-NNN closures
//!
//! - D-440(a): forbidden meta-commentary prohibition.
//! - D-441(a): verbatim-strict `current_step:` — no meta-commentary injection.
//! - D-442(a): prescribed clause order — forbidden-pattern gate prevents markers.
//! - D-443(a): D-chain cite currency gate.
//! - D-439(b): all 4 index version cite patterns required.
//! - D-441(b)/D-442(b): 5-column INDEX.md adversary-pass row schema (6 pipes) within
//!   `## Adversarial Reviews` h2 section; historical 4-col rows grandfathered.
//! - D-451(c): trajectory-tail LENGTH=4.
//!
//! # Architecture compliance
//!
//! - HOST_ABI_VERSION = 1 (no new host functions introduced).
//! - Fail-open on every `host::read_file` error (BC-5.39.006 invariant 9).
//! - No `println!` — use `host::log_*` for all diagnostic output.
//! - No `unwrap()` or `expect()` in production paths.
//! - No `regex` crate: hand-rolled pattern scanning to stay within WASM fuel budget.
//! - File-path enforcement via in-plugin guard using `Path::file_name` (Q5/Q6 canonical
//!   pattern; NOT `ends_with("STATE.md")` or `ends_with("INDEX.md")` — false-positive
//!   on paths like `xSTATE.md` or `xINDEX.md`).
//! - `tool = "Edit|Write"` is the canonical Q5/Q6 form for this hook's registry entry.
//! - All byte-index slice expressions with possible multi-byte UTF-8 input MUST use
//!   `is_char_boundary()` guards (BC-5.39.006 invariant 10; S-15.11 F-P4-001 lesson).

use vsdd_hook_sdk::{HookPayload, HookResult};

/// HOST_ABI_VERSION declares the ABI contract version this plugin was built
/// against. Must remain 1.
pub const HOST_ABI_VERSION: u32 = 1;

/// Maximum bytes to read from STATE.md or INDEX.md via `host::read_file`.
///
/// Set to 512 KiB (524288 bytes) — parity with BC-5.39.005/BC-5.39.006
/// cap and the validate-state-structure sibling crate (F-P5-002 fix).
/// Real STATE.md as of the F-P5-002 fix burst is 95185 bytes (95 KiB);
/// INDEX.md is typically smaller. The 4x growth runway accommodates future
/// expansion without requiring a further cap raise.
///
/// BC-5.39.006 precondition 4.
pub const MAX_BYTES: u32 = 524_288;

// ---------------------------------------------------------------------------
// Violation type
// ---------------------------------------------------------------------------

/// A structural violation found in the write target content.
///
/// Carries both a human-readable `description` (used verbatim in the block
/// message) and the `cited_raw` body-literal form of the offending text.
/// Structural plumbing per TD-VSDD-059 paper-fix avoidance: block messages
/// quote the exact string the author wrote, not a paraphrase.
#[derive(Debug, Clone)]
pub struct Violation {
    /// Human-readable description of the violation, used in the block message.
    pub description: String,
    /// The raw source text that triggered the violation.
    /// Populated via `.trim_end().to_string()` at every violation site.
    /// Required per TD-VSDD-059 paper-fix avoidance (structural plumbing
    /// from day 1 per S-15.07+S-15.11 cascade lessons).
    pub cited_raw: String,
}

// ---------------------------------------------------------------------------
// File-path guards
// ---------------------------------------------------------------------------

/// Returns `true` if `file_path` names a file whose `file_name` component is
/// exactly `STATE.md`.
///
/// Uses path-component-strict matching (`Path::file_name`) rather than
/// `ends_with`, preventing false-positive fires on paths like
/// `/some/dir/xSTATE.md`.
///
/// Returns `false` if the path has no file-name component (e.g. `/`).
///
/// # BC trace
/// BC-5.39.006 precondition 1 — hook STATE.md arm activates only on exact-name match.
/// BC-5.39.006 invariant 3 — path-component-strict matching is mandatory.
/// AC-16 / AC-17 — `xSTATE.md` and `xINDEX.md` must not trigger validation.
pub fn is_state_md_target(file_path: &str) -> bool {
    std::path::Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        == Some("STATE.md")
}

/// Returns `true` if `file_path` names a file whose `file_name` component is
/// exactly `INDEX.md`.
///
/// Uses path-component-strict matching (`Path::file_name`) rather than
/// `ends_with`, preventing false-positive fires on paths like
/// `/some/dir/xINDEX.md`.
///
/// Returns `false` if the path has no file-name component (e.g. `/`).
///
/// # BC trace
/// BC-5.39.006 precondition 1 — hook INDEX.md arm activates only on exact-name match.
/// BC-5.39.006 invariant 3 — path-component-strict matching is mandatory.
pub fn is_index_md_target(file_path: &str) -> bool {
    std::path::Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        == Some("INDEX.md")
}

// ---------------------------------------------------------------------------
// current_step: extraction
// ---------------------------------------------------------------------------

/// Extract the `current_step:` frontmatter value from YAML frontmatter.
///
/// YAML frontmatter is bounded by `---` delimiters. Scans for the line
/// starting with `current_step:` within the first frontmatter block and
/// returns the rest of that line as the value (trimming leading whitespace).
///
/// Byte-index slice expressions guard `is_char_boundary()` before slicing
/// to avoid panics on multi-byte UTF-8 content (em-dash, en-dash, NBSP in
/// STATE.md narrative text). BC-5.39.006 invariant 10; S-15.11 F-P4-001.
///
/// Returns `None` if the frontmatter block is absent, the delimiter is not
/// found, or `current_step:` is not present in frontmatter.
///
/// # BC trace
/// BC-5.39.006 invariant 4 — validation is anchored to `current_step:` only.
fn extract_current_step(content: &str) -> Option<&str> {
    // Find the start of YAML frontmatter: content must begin with `---`.
    // Skip leading whitespace/newlines to be tolerant of BOM-free UTF-8.
    let content = content.trim_start_matches('\u{feff}'); // strip BOM if present
    let after_open = content.strip_prefix("---")?;
    // Skip the newline after the opening delimiter.
    let body_start = after_open
        .strip_prefix('\n')
        .or_else(|| after_open.strip_prefix("\r\n"))
        .unwrap_or(after_open);

    // Find the closing `---` delimiter.
    // Use line-by-line search so we don't match `---` inside values.
    let mut fm_end = None;
    let mut offset = 0usize;
    for line in body_start.lines() {
        if line == "---" {
            fm_end = Some(offset);
            break;
        }
        // Advance offset by line length + newline.
        // We use safe `get` to avoid non-boundary slicing (invariant 10).
        offset += line.len() + 1; // +1 for '\n'; \r\n handled by lines()
    }
    let fm_body = match fm_end {
        Some(end) => {
            // Guard: ensure `end` is on a char boundary before slicing.
            if body_start.is_char_boundary(end) {
                &body_start[..end]
            } else {
                // Fallback: scan from start of body_start to the unsafe index
                // using the safe .get() accessor. If boundary is invalid, bail.
                body_start.get(..end)?
            }
        }
        None => body_start, // No closing delimiter; scan full remainder.
    };

    // Scan frontmatter body for the `current_step:` key.
    for line in fm_body.lines() {
        let trimmed = line.trim_start();
        let prefix = "current_step:";
        if let Some(rest) = trimmed.strip_prefix(prefix) {
            // Value is the rest of the line, trimmed of leading whitespace and quotes.
            let value = rest.trim();
            // Strip optional surrounding single or double quotes.
            let value = value
                .strip_prefix('\'')
                .and_then(|v| v.strip_suffix('\''))
                .or_else(|| value.strip_prefix('"').and_then(|v| v.strip_suffix('"')))
                .unwrap_or(value);
            return Some(value);
        }
    }
    None
}

// ---------------------------------------------------------------------------
// STATE.md validation checks
// ---------------------------------------------------------------------------

/// Check for forbidden meta-commentary patterns in `current_step:` value.
///
/// Scans for any of:
/// - `META-LEVEL-\d+ WATCH` (hand-rolled: `META-LEVEL-` prefix + digit run + ` WATCH`)
/// - `self-app TEST` (literal substring)
/// - `expected verdict` (literal substring)
///
/// If matched: returns `Some(Violation)` with `cited_raw` set to the matched
/// substring and `decision_refs` embedded in the description citing
/// D-440(a)+D-441(a)+D-442(a).
///
/// # BC trace
/// BC-5.39.006 postcondition 2; D-440(a)+D-441(a)+D-442(a); AC-1/AC-2/AC-3.
fn check_forbidden_meta_commentary(current_step_value: &str) -> Option<Violation> {
    // Hand-rolled scan for `META-LEVEL-\d+ WATCH` (BC-5.39.006 invariant 4).
    // The pattern is: literal "META-LEVEL-" followed by one or more digits,
    // followed by " WATCH".
    let meta_prefix = "META-LEVEL-";
    let meta_suffix = " WATCH";
    if let Some(pos) = current_step_value.find(meta_prefix) {
        let after_prefix = &current_step_value[pos + meta_prefix.len()..];
        // Count digits after the prefix.
        let digit_end = after_prefix
            .char_indices()
            .take_while(|(_, c)| c.is_ascii_digit())
            .last()
            .map(|(i, c)| i + c.len_utf8())
            .unwrap_or(0);
        if digit_end > 0 {
            let after_digits = &after_prefix[digit_end..];
            if after_digits.starts_with(meta_suffix) {
                let matched_end = pos + meta_prefix.len() + digit_end + meta_suffix.len();
                // Guard char boundary before slicing for cited_raw.
                let cited_raw = current_step_value
                    .get(pos..matched_end)
                    .unwrap_or(meta_prefix)
                    .to_string();
                return Some(Violation {
                    description: format!(
                        "forbidden meta-commentary pattern found in current_step: \
                         matched pattern `META-LEVEL-N WATCH`; \
                         cited: \"{cited_raw}\"; \
                         D-440(a)+D-441(a)+D-442(a)"
                    ),
                    cited_raw,
                });
            }
        }
    }

    // Literal substring: `self-app TEST`.
    if current_step_value.contains("self-app TEST") {
        return Some(Violation {
            description: "forbidden meta-commentary pattern found in current_step: \
                          matched pattern `self-app TEST`; \
                          D-440(a)+D-441(a)+D-442(a)"
                .to_string(),
            cited_raw: "self-app TEST".to_string(),
        });
    }

    // Literal substring: `expected verdict`.
    if current_step_value.contains("expected verdict") {
        return Some(Violation {
            description: "forbidden meta-commentary pattern found in current_step: \
                          matched pattern `expected verdict`; \
                          D-440(a)+D-441(a)+D-442(a)"
                .to_string(),
            cited_raw: "expected verdict".to_string(),
        });
    }

    None
}

/// Check that all 4 required index version citation patterns are present in
/// `current_step:` value.
///
/// Required literal substrings (each followed by a version token):
/// - `BC-INDEX v`
/// - `VP-INDEX v`
/// - `STORY-INDEX v`
/// - `ARCH-INDEX v`
///
/// If any are absent: returns `Some(Violation)` naming each missing cite,
/// citing D-439(b).
///
/// # BC trace
/// BC-5.39.006 postcondition 3; D-439(b); AC-4; invariant 5.
fn check_index_version_cites(current_step_value: &str) -> Option<Violation> {
    const REQUIRED: &[&str] = &["BC-INDEX v", "VP-INDEX v", "STORY-INDEX v", "ARCH-INDEX v"];
    let missing: Vec<&str> = REQUIRED
        .iter()
        .copied()
        .filter(|pat| !current_step_value.contains(pat))
        .collect();

    if missing.is_empty() {
        return None;
    }

    let missing_list = missing.join(", ");
    Some(Violation {
        description: format!(
            "missing required index version citation(s) in current_step: {missing_list}; \
             all 4 patterns (BC-INDEX v, VP-INDEX v, STORY-INDEX v, ARCH-INDEX v) \
             must be present per D-439(b)"
        ),
        cited_raw: missing_list,
    })
}

/// Check that trajectory-tail in `current_step:` contains exactly 4 `→(\d+)` matches.
///
/// Uses hand-rolled scanning for the `→` character (U+2192, 3 UTF-8 bytes)
/// followed immediately by one or more ASCII digits. Counts the number of
/// such sequences globally in `current_step_value`.
///
/// If count != 4: returns `Some(Violation)` naming actual count and required
/// count (4), citing D-451(c).
///
/// # UTF-8 note
/// The arrow `→` is U+2192, encoded as `\xE2\x86\x92` in UTF-8. Hand-rolled
/// scanning uses `char_indices()` iteration on `&str` chars to avoid byte-index
/// boundary issues. No raw byte slicing performed here.
///
/// # BC trace
/// BC-5.39.006 postcondition 4; D-451(c); AC-5/AC-6; invariant 6.
fn check_trajectory_tail_length(current_step_value: &str) -> Option<Violation> {
    let count = count_arrow_digit_matches(current_step_value);
    if count == 4 {
        return None;
    }
    Some(Violation {
        description: format!(
            "trajectory-tail in current_step: has {count} `→N` component(s); \
             required LENGTH=4 per D-451(c); \
             canonical form is `→N→N→N→N`"
        ),
        cited_raw: format!("found {count} arrow-digit matches"),
    })
}

/// Count `→N` (arrow followed by one or more ASCII digits) occurrences in `s`.
///
/// Uses `char_indices()` for safe UTF-8 iteration. Arrow U+2192 = 3 bytes.
/// ASCII digit bytes are single-byte and cannot be UTF-8 continuation bytes —
/// safe to check via `bytes[j].is_ascii_digit()` in the inner walk.
///
/// This is the same algorithm used in the validate-state-structure sibling
/// (S-15.09) for trajectory-tail counting; kept consistent for cohesion.
fn count_arrow_digit_matches(s: &str) -> usize {
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
            let after = byte_pos + arrow_byte_len;
            if after < len && bytes[after].is_ascii_digit() {
                count += 1;
                // Skip past all trailing digits to avoid double-counting.
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

/// Check that the D-chain cite in `current_step:` is current.
///
/// BC-5.39.006 v1.1 invariant 7 — D-(\\d+) max-extraction semantics:
///
/// 1. Extract ALL `D-(\d+)` integers from `current_step_value` globally;
///    take the maximum integer found (call it `max_cited`).
/// 2. Extract ALL `D-(\d+)` integers from the full STATE.md `content`;
///    take the maximum integer found (call it `max_in_file`).
/// 3. If `max_cited` is None (no D-NNN reference in current_step): violation.
/// 4. If `max_cited < max_in_file`: stale cite violation.
/// 5. If `max_cited >= max_in_file`: current (or fail-open; no violation).
///
/// The literal prefix `D-382..` is NOT required — production `current_step:`
/// uses prose forms like `D-chain cite D-476 latest brownfield`.
///
/// Fail-open design: when the body has no D-NNN at a higher integer than
/// current_step cites, returns None to avoid blocking in-progress writes.
///
/// # BC trace
/// BC-5.39.006 v1.1 postcondition 5; D-443(a); EC-008/EC-009/EC-010; invariant 7.
fn check_d_chain_currency(content: &str, current_step_value: &str) -> Option<Violation> {
    // Step 1: extract max D-NNN from current_step_value.
    let max_cited = scan_max_d_nnn(current_step_value);

    if max_cited == 0 {
        // No D-NNN reference at all in current_step — violation per invariant 7.
        return Some(Violation {
            description:
                "D-chain cite absent from current_step: no `D-\\d+` reference found; \
                 current_step: must contain a D-NNN cite per D-443(a); \
                 production form: `D-chain cite D-NNN latest brownfield`"
                    .to_string(),
            cited_raw: String::new(),
        });
    }

    // Step 2: extract max D-NNN from full STATE.md content.
    let max_in_file = scan_max_d_nnn(content);

    if max_cited < max_in_file {
        Some(Violation {
            description: format!(
                "D-chain cite in current_step: is stale: max_cited D-{max_cited} \
                 but STATE.md body shows D-{max_in_file} as latest; \
                 update D-chain cite to include D-{max_in_file} per D-443(a)"
            ),
            cited_raw: format!("D-{max_cited}"),
        })
    } else {
        // Current or fail-open per BC-5.39.006 v1.1 invariant 7.
        None
    }
}

/// Scan `s` for all `D-NNN` patterns (where NNN is one or more ASCII digits)
/// and return the maximum integer found. Returns 0 if no match.
///
/// Uses the same hand-rolled scanning strategy as the sibling functions to
/// avoid the regex crate (WASM fuel budget constraint).
///
/// The scan does NOT skip any D-NNN patterns — the caller (check_d_chain_currency)
/// feeds current_step_value and full content separately, so self-reference
/// exclusion is unnecessary: taking the max of current_step's own D-NNNs as
/// `max_cited` is correct by construction.
///
/// # Slice safety (BC-5.39.006 invariant 10)
/// Advances by `pos + 2 + digit_len` bytes, which always lands on an ASCII
/// boundary (digits are single-byte; "D-" is ASCII). Guards `>= len` before
/// slicing to prevent out-of-bounds access.
///
/// # BC trace
/// BC-5.39.006 v1.1 invariant 7 — D-(\d+) max-extraction.
fn scan_max_d_nnn(s: &str) -> u64 {
    let mut max = 0u64;
    let mut search = s;

    while let Some(pos) = search.find("D-") {
        let after = &search[pos + 2..];
        // Extract digits after `D-`.
        let digit_str: String = after.chars().take_while(|c| c.is_ascii_digit()).collect();
        if let Ok(n) = digit_str.parse::<u64>() {
            if n > max {
                max = n;
            }
        }
        // Advance past pos + "D-" + digit_len.
        // `pos + 2 + digit_len` lands on an ASCII digit boundary or the byte
        // after the last digit — always a valid UTF-8 char boundary.
        let advance = pos + 2 + digit_str.len();
        if advance >= search.len() {
            break;
        }
        search = &search[advance..];
    }
    max
}

/// Orchestrate all STATE.md validation checks.
///
/// Extracts `current_step:` value, runs all 4 checks, accumulates non-None
/// results into a `Vec<Violation>`. Returns an empty Vec for a clean write.
///
/// If `current_step:` cannot be extracted (frontmatter absent, truncated, or
/// malformed), returns an empty Vec — fail-open per BC-5.39.006 invariant 7/9.
/// The caller (`on_post_tool_use`) logs a warning via `host::log_warn` so the
/// skip is observable in dispatcher telemetry. This is consistent with the
/// read-error path which also produces Continue + log_warn.
///
/// # BC trace
/// BC-5.39.006 v1.1 postcondition 1/6/7; invariant 7/9; F-P1-004.
pub fn validate_state_md(content: &str) -> Vec<Violation> {
    let current_step = match extract_current_step(content) {
        Some(v) => v,
        None => {
            // current_step: absent or frontmatter malformed — fail-open per invariant 9.
            // Caller (on_post_tool_use) emits log_warn for observability.
            return vec![];
        }
    };

    let mut violations: Vec<Violation> = Vec::new();

    if let Some(v) = check_forbidden_meta_commentary(current_step) {
        violations.push(v);
    }
    if let Some(v) = check_index_version_cites(current_step) {
        violations.push(v);
    }
    if let Some(v) = check_trajectory_tail_length(current_step) {
        violations.push(v);
    }
    if let Some(v) = check_d_chain_currency(content, current_step) {
        violations.push(v);
    }

    violations
}

// ---------------------------------------------------------------------------
// INDEX.md validation
// ---------------------------------------------------------------------------

/// Validate adversary-pass table rows in INDEX.md content.
///
/// BC-5.39.006 v1.1 invariant 8 — h2-section state machine:
///
/// 1. Scan lines for h2 headings (`^## `). Enter `InAdversaryReviews` state
///    when the exact heading `## Adversarial Reviews` is seen. Exit back to
///    `Outside` when ANY next h2 heading appears.
/// 2. Inside `InAdversaryReviews`, find the first non-separator `|`-delimited
///    row — this is the header row. Count its pipes:
///    - pipe_count == 5 (4-column schema): GRANDFATHERED. Skip all remaining
///      rows in this section — no column validation.
///    - pipe_count == 6 (5-column schema): ENFORCE. All subsequent non-separator
///      `|`-rows in this section must have exactly 6 pipes (5 columns).
///    - any other count: treat as 5-col schema enforcement (default to strict).
/// 3. Separator rows (trimmed contains `|---` or `| ---` or `|:---`) are
///    ALWAYS skipped, including when determining header schema.
/// 4. Rows OUTSIDE `## Adversarial Reviews` are NEVER validated.
///
/// Returns all accumulated violations (may be empty for a clean write).
///
/// # Pipe arithmetic (BC-5.39.006 v1.1 invariant 8 corrected)
/// For N columns: 1 leading + (N-1) internal + 1 trailing = N+1 pipes.
/// - 5 columns: 6 pipes. 4 columns: 5 pipes. 6 columns: 7 pipes.
/// The canonical 5-column adversary-pass schema = 6 pipes per D-442(b).
///
/// # BC trace
/// BC-5.39.006 v1.1 postcondition 8/9; D-441(b)/D-442(b); EC-013..EC-016; invariant 8.
pub fn validate_index_md(content: &str) -> Vec<Violation> {
    /// State machine for h2-section tracking.
    #[derive(PartialEq)]
    enum State {
        /// Not inside `## Adversarial Reviews` section.
        Outside,
        /// Inside `## Adversarial Reviews`; header schema not yet detected.
        InSectionNoHeader,
        /// Inside section; header is 4-column (grandfathered; skip validation).
        InSectionGrandfathered,
        /// Inside section; header is 5-column; enforce 6-pipe compliance.
        InSectionEnforce,
    }

    let mut violations: Vec<Violation> = Vec::new();
    let mut state = State::Outside;

    for line in content.lines() {
        let trimmed = line.trim();

        // Detect h2 headings — update section state.
        if trimmed.starts_with("## ") {
            if trimmed == "## Adversarial Reviews" {
                state = State::InSectionNoHeader;
            } else {
                // Any other h2 ends the current section.
                state = State::Outside;
            }
            continue;
        }

        // Only process rows while inside the ## Adversarial Reviews section.
        if state == State::Outside {
            continue;
        }

        // Must be a table row: trimmed starts and ends with `|`.
        if !trimmed.starts_with('|') || !trimmed.ends_with('|') {
            continue;
        }

        // Separator rows are always skipped.
        if trimmed.contains("|---") || trimmed.contains("| ---") || trimmed.contains("|:---") {
            continue;
        }

        let pipe_count = trimmed.chars().filter(|&c| c == '|').count();

        match state {
            State::InSectionNoHeader => {
                // First non-separator row: determine schema from header pipe count.
                if pipe_count == 5 {
                    // 4-column header (5 pipes) — grandfathered; skip validation.
                    state = State::InSectionGrandfathered;
                } else {
                    // 5-column (6 pipes) or other — enforce 6-pipe compliance.
                    state = State::InSectionEnforce;
                    // Validate this header row too if not 6 pipes.
                    // (Header should be 6 pipes for 5-col schema; if it's different
                    // that is unusual but let enforcement below handle it.)
                    if pipe_count != 6 {
                        let actual_cols = pipe_count.saturating_sub(1);
                        let row_preview = safe_truncate(trimmed, 120);
                        violations.push(Violation {
                            description: format!(
                                "INDEX.md ## Adversarial Reviews header row has \
                                 {actual_cols} column(s); required 5 columns per \
                                 D-441(b)/D-442(b); row: `{row_preview}`"
                            ),
                            cited_raw: row_preview,
                        });
                    }
                }
            }
            State::InSectionEnforce => {
                // Enforce 6-pipe (5-column) compliance on all data rows.
                if pipe_count != 6 {
                    let actual_cols = pipe_count.saturating_sub(1);
                    let row_preview = safe_truncate(trimmed, 120);
                    violations.push(Violation {
                        description: format!(
                            "INDEX.md adversary-pass row has {actual_cols} column(s); \
                             required 5 columns per D-441(b)/D-442(b); \
                             row: `{row_preview}`"
                        ),
                        cited_raw: row_preview,
                    });
                }
            }
            State::InSectionGrandfathered => {
                // Grandfathered 4-col section — skip column validation entirely.
            }
            State::Outside => {
                // Unreachable here (handled above), but exhaustive match required.
            }
        }
    }

    violations
}

/// Truncate `s` to at most `max_chars` characters, preserving UTF-8 char boundaries.
///
/// Uses `char_indices()` to find the correct byte boundary (BC-5.39.006 invariant 10).
fn safe_truncate(s: &str, max_chars: usize) -> String {
    let mut byte_end = s.len();
    for (char_count, (byte_pos, _ch)) in s.char_indices().enumerate() {
        if char_count == max_chars {
            byte_end = byte_pos;
            break;
        }
    }
    s[..byte_end].to_string()
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
/// # TD-VSDD-059 compliance
/// `cited_raw` is wired through `emit_block` rather than being a dead field.
fn emit_block(hook_name: &str, violations: &[Violation]) -> HookResult {
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
        "{hook_name}: {} violation(s):\n{}",
        violations.len(),
        lines.join("\n")
    );
    HookResult::block_with_fix(
        hook_name,
        reason,
        "Fix the violations listed above before re-writing this file",
        "DISPATCH_ADVANCE_VIOLATION",
    )
}

// ---------------------------------------------------------------------------
// Hook entry point
// ---------------------------------------------------------------------------

/// PostToolUse hook entry point.
///
/// Called by the SDK trampoline (`__internal::run`) for every Edit/Write
/// PostToolUse event. Dispatches to the STATE.md arm or INDEX.md arm based on
/// path-component-strict guards, then returns the appropriate HookResult.
///
/// # Control flow
///
/// 1. Extract `file_path` from `payload.tool_input`. If absent: Continue
///    (graceful degrade; log_warn).
/// 2. If `is_state_md_target(file_path)`: read via `host::read_file`. On error:
///    Continue + log_warn (fail-open per invariant 9). On success: run
///    `validate_state_md`. If violations non-empty: emit single BlockWithFix
///    enumerating all. Else: Continue.
/// 3. Else if `is_index_md_target(file_path)`: read via `host::read_file`. On
///    error: Continue + log_warn. On success: run `validate_index_md`. If
///    violations non-empty: emit single BlockWithFix enumerating all. Else:
///    Continue.
/// 4. Else: Continue (file not in scope of this hook).
///
/// # BC trace
/// BC-5.39.006 postconditions 1–10; invariants 1–10.
pub fn on_post_tool_use(payload: HookPayload) -> HookResult {
    use vsdd_hook_sdk::host;

    const HOOK_NAME: &str = "validate-dispatch-advance";

    // Step 1: Extract file_path from tool_input.
    let file_path = match payload.tool_input.get("file_path").and_then(|v| v.as_str()) {
        Some(p) => p.to_string(),
        None => {
            host::log_warn(
                "[validate-dispatch-advance] file_path absent from tool_input — graceful degrade",
            );
            return HookResult::Continue;
        }
    };

    if is_state_md_target(&file_path) {
        // STATE.md arm.
        let content = match host::read_file(&file_path, MAX_BYTES, 2000) {
            Ok(bytes) => {
                if bytes.len() as u32 == MAX_BYTES {
                    host::log_warn(&format!(
                        "[{HOOK_NAME}] read_file returned exactly {MAX_BYTES} bytes \
                         for {file_path} — possible truncation; consider raising MAX_BYTES"
                    ));
                }
                match String::from_utf8(bytes) {
                    Ok(s) => s,
                    Err(e) => {
                        host::log_warn(&format!(
                            "[{HOOK_NAME}] UTF-8 decode failure reading {file_path}: {e}"
                        ));
                        return HookResult::Continue;
                    }
                }
            }
            Err(e) => {
                host::log_warn(&format!(
                    "[{HOOK_NAME}] read_file failed for {file_path}: {e:?}"
                ));
                return HookResult::Continue;
            }
        };

        // Fail-open: if current_step: cannot be extracted, skip validation and warn.
        // Consistent with read-error path (invariant 9); aligns with F-P1-004 fix.
        if extract_current_step(&content).is_none() {
            host::log_warn(&format!(
                "[{HOOK_NAME}] current_step: absent or frontmatter malformed in \
                 {file_path} — skipping STATE.md validation (fail-open per invariant 9)"
            ));
            return HookResult::Continue;
        }

        let violations = validate_state_md(&content);
        if violations.is_empty() {
            HookResult::Continue
        } else {
            emit_block(HOOK_NAME, &violations)
        }
    } else if is_index_md_target(&file_path) {
        // INDEX.md arm.
        let content = match host::read_file(&file_path, MAX_BYTES, 2000) {
            Ok(bytes) => {
                if bytes.len() as u32 == MAX_BYTES {
                    host::log_warn(&format!(
                        "[{HOOK_NAME}] read_file returned exactly {MAX_BYTES} bytes \
                         for {file_path} — possible truncation; consider raising MAX_BYTES"
                    ));
                }
                match String::from_utf8(bytes) {
                    Ok(s) => s,
                    Err(e) => {
                        host::log_warn(&format!(
                            "[{HOOK_NAME}] UTF-8 decode failure reading {file_path}: {e}"
                        ));
                        return HookResult::Continue;
                    }
                }
            }
            Err(e) => {
                host::log_warn(&format!(
                    "[{HOOK_NAME}] read_file failed for {file_path}: {e:?}"
                ));
                return HookResult::Continue;
            }
        };

        let violations = validate_index_md(&content);
        if violations.is_empty() {
            HookResult::Continue
        } else {
            emit_block(HOOK_NAME, &violations)
        }
    } else {
        // Not a target path — continue without action.
        HookResult::Continue
    }
}

// ---------------------------------------------------------------------------
// Unit tests (all red until implementer fills in real logic)
// ---------------------------------------------------------------------------

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    // -- Path-guard tests (GREEN-BY-DESIGN: pure Path::file_name delegation) --

    #[test]
    fn test_is_state_md_target_positive() {
        assert!(is_state_md_target(".factory/STATE.md"));
    }

    #[test]
    fn test_xstate_md_is_not_target() {
        assert!(!is_state_md_target("/some/dir/xSTATE.md"));
    }

    #[test]
    fn test_is_index_md_target_positive() {
        assert!(is_index_md_target(
            ".factory/cycles/v1.0-feature-engine-discipline-pass-1/INDEX.md"
        ));
    }

    #[test]
    fn test_xindex_md_is_not_target() {
        assert!(!is_index_md_target("/some/dir/xINDEX.md"));
    }

    // -- scan_max_d_nnn slice-guard safety (F-P1-006; TD-VSDD-060 sibling-site sweep) --

    #[test]
    fn test_scan_max_d_nnn_d_at_end_of_string() {
        // "D-" at the very end of string with no digits — must not panic.
        // Tests the `advance >= search.len()` break guard in scan_max_d_nnn.
        let result = std::panic::catch_unwind(|| {
            // Call check_d_chain_currency which calls scan_max_d_nnn internally.
            let current_step = "BC-INDEX v1.14 VP-INDEX v1.8 STORY-INDEX v1.12 ARCH-INDEX v1.9 D-477";
            let content = "---\ncurrent_step: 'x'\n---\nsuffix D-";
            check_d_chain_currency(content, current_step)
        });
        assert!(result.is_ok(), "scan_max_d_nnn must not panic on 'D-' at end of string");
    }

    #[test]
    fn test_scan_max_d_nnn_multiple_references() {
        // Multiple D-NNN in current_step; max is correctly extracted.
        let current_step =
            "BC-INDEX v1.14 VP-INDEX v1.8 STORY-INDEX v1.12 ARCH-INDEX v1.9 \
             D-382 D-440 D-477 →9→9→9→9";
        let content = "---\ncurrent_step: 'x'\n---\n| D-477 |\n";
        let v = check_d_chain_currency(content, current_step);
        assert!(v.is_none(), "max_cited=477 >= max_in_file=477 — should not violate");
    }

    // -- extract_current_step fail-open (F-P1-004 / BC-5.39.006 v1.1 invariant 7/9) --

    #[test]
    fn test_validate_state_md_fail_open_on_missing_frontmatter() {
        // Content with no frontmatter — extract_current_step returns None.
        // validate_state_md must return vec![] (no violations) — fail-open per invariant 9.
        let content = "# No frontmatter\n\nsome body text D-477\n";
        let violations = validate_state_md(content);
        assert!(
            violations.is_empty(),
            "missing frontmatter must be fail-open (no violations); \
             got: {violations:?}"
        );
    }

    #[test]
    fn test_validate_state_md_fail_open_on_unclosed_frontmatter() {
        // Frontmatter opened but never closed — extract_current_step returns None
        // (no closing `---` found, and current_step: is absent from the partial block).
        let content = "---\nphase: test\n# body without closing ---\n";
        let violations = validate_state_md(content);
        assert!(
            violations.is_empty(),
            "truncated/malformed frontmatter must be fail-open (no violations); \
             got: {violations:?}"
        );
    }

    // -- Forbidden meta-commentary tests --

    #[test]
    fn test_forbidden_meta_commentary_watch() {
        let v = check_forbidden_meta_commentary(
            "BC-INDEX v1.14, VP-INDEX v1.8, STORY-INDEX v1.12, ARCH-INDEX v1.9 \
             META-LEVEL-5 WATCH: self-application →9→9→9→9 D-382..D-477",
        );
        assert!(v.is_some(), "should detect META-LEVEL-5 WATCH pattern");
        let v = v.unwrap();
        assert!(
            v.description.contains("D-440(a)"),
            "block message must cite D-440(a)"
        );
    }

    #[test]
    fn test_forbidden_meta_commentary_self_app() {
        let v = check_forbidden_meta_commentary(
            "BC-INDEX v1.14 VP-INDEX v1.8 STORY-INDEX v1.12 ARCH-INDEX v1.9 \
             self-app TEST example →9→9→9→9 D-382..D-477",
        );
        assert!(v.is_some(), "should detect self-app TEST pattern");
    }

    #[test]
    fn test_forbidden_meta_commentary_expected_verdict() {
        let v = check_forbidden_meta_commentary(
            "BC-INDEX v1.14 VP-INDEX v1.8 STORY-INDEX v1.12 ARCH-INDEX v1.9 \
             expected verdict: PASS →9→9→9→9 D-382..D-477",
        );
        assert!(v.is_some(), "should detect expected verdict pattern");
    }

    // -- Index version cite tests --

    #[test]
    fn test_index_version_cites_all_present() {
        let v = check_index_version_cites(
            "BC-INDEX v1.14, VP-INDEX v1.8, STORY-INDEX v1.12, ARCH-INDEX v1.9 \
             →9→9→9→9 D-382..D-477",
        );
        assert!(v.is_none(), "all 4 cites present — should not violate");
    }

    #[test]
    fn test_index_version_cites_missing_arch() {
        let v = check_index_version_cites(
            "BC-INDEX v1.14, VP-INDEX v1.8, STORY-INDEX v1.12 →9→9→9→9 D-382..D-477",
        );
        assert!(v.is_some(), "missing ARCH-INDEX v cite — should violate");
        let v = v.unwrap();
        assert!(
            v.description.contains("ARCH-INDEX v"),
            "block message must name missing ARCH-INDEX v"
        );
        assert!(
            v.description.contains("D-439(b)"),
            "block message must cite D-439(b)"
        );
    }

    // -- Trajectory-tail length tests --

    #[test]
    fn test_trajectory_tail_length_3() {
        let v = check_trajectory_tail_length(
            "BC-INDEX v1.14, VP-INDEX v1.8, STORY-INDEX v1.12, ARCH-INDEX v1.9 \
             →9→9→9 D-382..D-477",
        );
        assert!(v.is_some(), "tail has 3 components — should violate");
        let v = v.unwrap();
        assert!(
            v.description.contains('3'),
            "block message must name actual count 3"
        );
        assert!(
            v.description.contains("D-451(c)"),
            "block message must cite D-451(c)"
        );
    }

    #[test]
    fn test_trajectory_tail_length_5() {
        let v = check_trajectory_tail_length(
            "BC-INDEX v1.14, VP-INDEX v1.8, STORY-INDEX v1.12, ARCH-INDEX v1.9 \
             →9→9→9→9→9 D-382..D-477",
        );
        assert!(v.is_some(), "tail has 5 components — should violate");
        let v = v.unwrap();
        assert!(
            v.description.contains('5'),
            "block message must name actual count 5"
        );
    }

    #[test]
    fn test_trajectory_tail_length_4() {
        let v = check_trajectory_tail_length(
            "BC-INDEX v1.14, VP-INDEX v1.8, STORY-INDEX v1.12, ARCH-INDEX v1.9 \
             →9→9→9→9 D-382..D-477",
        );
        assert!(
            v.is_none(),
            "tail has exactly 4 components — should not violate"
        );
    }

    // -- D-chain currency tests (BC-5.39.006 v1.1 invariant 7: D-(\d+) max-extraction) --

    #[test]
    fn test_d_chain_absent() {
        // No D-NNN at all in current_step — should violate.
        let content = "---\ncurrent_step: 'BC-INDEX v1.14 →9→9→9→9'\n---\n| D-477 |";
        let v = check_d_chain_currency(
            content,
            "BC-INDEX v1.14, VP-INDEX v1.8, STORY-INDEX v1.12, ARCH-INDEX v1.9 →9→9→9→9",
        );
        assert!(v.is_some(), "absent D-chain cite — should violate");
        let v = v.unwrap();
        assert!(
            v.description.contains("D-443(a)"),
            "block message must cite D-443(a)"
        );
    }

    #[test]
    fn test_d_chain_stale_range_form() {
        // Prose range form D-382..D-476; body shows D-477 → stale.
        let content = "---\ncurrent_step: 'BC-INDEX v1.14 →9→9→9→9 D-382..D-476'\n---\n\
             | D-477 | some row |\n";
        let current_step = "BC-INDEX v1.14, VP-INDEX v1.8, STORY-INDEX v1.12, ARCH-INDEX v1.9 \
             →9→9→9→9 D-382..D-476";
        let v = check_d_chain_currency(content, current_step);
        assert!(
            v.is_some(),
            "stale D-chain cite (max_cited=476 < max_in_file=477) — should violate"
        );
        let v = v.unwrap();
        assert!(
            v.description.contains("476"),
            "block message must name stale max_cited 476"
        );
    }

    #[test]
    fn test_d_chain_stale_prose_form() {
        // Production prose form "D-chain cite D-476"; body shows D-477 → stale.
        let content = "---\ncurrent_step: 'BC-INDEX v1.14 →9→9→9→9 D-chain cite D-476'\n---\n\
             | D-477 | some row |\n";
        let current_step = "BC-INDEX v1.14, VP-INDEX v1.8, STORY-INDEX v1.12, ARCH-INDEX v1.9 \
             →9→9→9→9 D-chain cite D-476";
        let v = check_d_chain_currency(content, current_step);
        assert!(
            v.is_some(),
            "stale prose D-chain cite (max_cited=476 < max_in_file=477) — should violate"
        );
        let v = v.unwrap();
        assert!(
            v.description.contains("476"),
            "block message must name stale max_cited 476"
        );
    }

    #[test]
    fn test_d_chain_current_range_form() {
        // current_step cites D-382..D-477 and body max is also D-477 — current.
        let content = "---\ncurrent_step: 'BC-INDEX v1.14 →9→9→9→9 D-382..D-477'\n---\n\
             | D-477 | some row |\n";
        let current_step = "BC-INDEX v1.14, VP-INDEX v1.8, STORY-INDEX v1.12, ARCH-INDEX v1.9 \
             →9→9→9→9 D-382..D-477";
        let v = check_d_chain_currency(content, current_step);
        assert!(v.is_none(), "D-chain cite is current — should not violate");
    }

    #[test]
    fn test_d_chain_current_prose_form() {
        // Production prose form "D-chain cite D-476"; body max is also 476 — current.
        // This matches the real production STATE.md current_step format.
        let content = "---\ncurrent_step: 'BC-INDEX v1.14 →9→9→9→9 D-chain cite D-476'\n---\n\
             | D-476 | some row |\n";
        let current_step = "BC-INDEX v1.14, VP-INDEX v1.8, STORY-INDEX v1.12, ARCH-INDEX v1.9 \
             →9→9→9→9 D-chain cite D-476 latest brownfield";
        let v = check_d_chain_currency(content, current_step);
        assert!(
            v.is_none(),
            "prose D-chain cite matches body max — should not violate"
        );
    }

    // -- INDEX.md column tests (BC-5.39.006 v1.1: h2-section state machine + header-schema) --

    // Helper: wrap rows in a canonical 5-col-header ## Adversarial Reviews section.
    fn in_5col_section(rows: &str) -> String {
        format!(
            "## Adversarial Reviews\n\n\
             | Pass | Date | Findings Count | Verdict | File |\n\
             |------|------|---------------|---------|------|\n\
             {rows}\n\
             \n## Next Section\n"
        )
    }

    // Helper: wrap rows in a grandfathered 4-col-header ## Adversarial Reviews section.
    fn in_4col_section(rows: &str) -> String {
        format!(
            "## Adversarial Reviews\n\n\
             | Pass | Date | Findings | Status |\n\
             |------|------|----------|--------|\n\
             {rows}\n\
             \n## Next Section\n"
        )
    }

    #[test]
    fn test_index_md_4_col_row_in_5col_section() {
        // EC-013: 4-col row (5 pipes) in 5-col-header section → violation.
        let content = in_5col_section("| col1 | col2 | col3 | col4 |");
        let violations = validate_index_md(&content);
        assert!(
            !violations.is_empty(),
            "4-column row in 5-col section should violate"
        );
        assert!(
            violations[0].description.contains("D-441(b)"),
            "violation must cite D-441(b)"
        );
        assert!(
            violations[0].description.contains('4'),
            "violation must name actual count 4"
        );
    }

    #[test]
    fn test_index_md_6_col_row_in_5col_section() {
        // EC-014: 6-col row (7 pipes) in 5-col-header section → violation.
        let content = in_5col_section("| col1 | col2 | col3 | col4 | col5 | col6 |");
        let violations = validate_index_md(&content);
        assert!(
            !violations.is_empty(),
            "6-column row in 5-col section should violate"
        );
        assert!(
            violations[0].description.contains('6'),
            "violation must name actual count 6"
        );
    }

    #[test]
    fn test_index_md_5_col_row_in_5col_section() {
        // EC-015: 5-col row (6 pipes) in 5-col-header section → pass.
        let content = in_5col_section("| pass1 | 2026-05-17 | 3 | MEDIUM | adv-1.md |");
        let violations = validate_index_md(&content);
        assert!(
            violations.is_empty(),
            "5-column row in 5-col section — should not violate"
        );
    }

    #[test]
    fn test_index_md_4col_section_grandfathered() {
        // 4-col-header section (5 pipes in header) → skip column validation.
        let content = in_4col_section("| 1 | 2026-04-25 | 17 (1C) | substantive |");
        let violations = validate_index_md(&content);
        assert!(
            violations.is_empty(),
            "4-col-header section is grandfathered — no violations expected"
        );
    }

    #[test]
    fn test_index_md_rows_outside_section_skipped() {
        // Rows outside ## Adversarial Reviews h2 must NOT be validated.
        // 3-col row outside section: should not trigger violation.
        let content = "## Story Listing\n\n| S-01 | desc | status |\n|------|------|--------|\n\
             | S-1.01 | foo | done |\n\n## Adversarial Reviews\n\n\
             | Pass | Date | Findings Count | Verdict | File |\n\
             |------|------|---------------|---------|------|\n\
             | 1 | 2026-05-17 | 3 | MEDIUM | adv-1.md |\n\n## More\n";
        let violations = validate_index_md(content);
        assert!(
            violations.is_empty(),
            "out-of-section rows must not be validated; 5-col row inside section passes"
        );
    }

    #[test]
    fn test_index_md_header_row_skipped() {
        // Separator row (|---|---|) within 5-col section must not be flagged.
        let content = in_5col_section(""); // only separator rows in the helpers
        let violations = validate_index_md(&content);
        assert!(
            violations.is_empty(),
            "separator rows must be skipped by validate_index_md"
        );
    }

    #[test]
    fn test_index_md_production_brownfield() {
        // Simulate brownfield INDEX.md — 4-col-header section → grandfathered.
        let content = "\
## Adversarial Reviews\n\n\
| Pass | Date | Findings | Status |\n\
|------|------|----------|--------|\n\
| 1 | 2026-04-25 | 17 (1 CRIT + 7 HIGH) | substantive — fixes applied |\n\
| 2 | 2026-04-25 | 11 (1 CRIT + 4 HIGH) | substantive — fixes applied |\n\n\
## S-6.01 Sub-cycle Adversarial Reviews\n";
        let violations = validate_index_md(content);
        assert!(
            violations.is_empty(),
            "brownfield 4-col rows must be grandfathered; no violations"
        );
    }

    #[test]
    fn test_index_md_production_edp1() {
        // Simulate EDP1 INDEX.md — 5-col-header section; all rows 5-col → pass.
        let content = "\
## Adversarial Reviews\n\n\
> Format note.\n\n\
| Pass | Date | Findings Count | Verdict | File |\n\
|------|------|---------------|---------|------|\n\
| 1 | 2026-05-07 | 29 (4C+14H+6M+5L) | CRITICAL | adv-cycle-pass-1.md |\n\
| 2 | 2026-05-07 | 15 (2C+6H+4M+3L) | CRITICAL | adv-cycle-pass-2.md |\n\n\
## Convergence Status\n";
        let violations = validate_index_md(content);
        assert!(
            violations.is_empty(),
            "EDP1 5-col rows must all pass; no violations"
        );
    }

    // -- UTF-8 / em-dash safety (AC-20) --

    /// AC-20: `current_step:` value containing em-dash (U+2014) or other
    /// multi-byte UTF-8 characters must not cause a panic. This test invokes
    /// `extract_current_step` on content with `—` adjacent to a digit and
    /// asserts the call completes without panicking.
    ///
    /// "If I include this real implementation, will the test for this function
    /// pass trivially without any implementer work?" — No. The extraction
    /// logic must guard is_char_boundary() correctly for this test to pass.
    #[test]
    fn validate_current_step_with_em_dash() {
        let content = "---\ncurrent_step: 'pass 74—D-382..D-477 →9→9→9→9'\n---\nbody\n";
        // Must not panic; correctness of returned value tested by implementer.
        let _ = extract_current_step(content);
    }

    // -- Production false-positive regression (AC-21 preemptive; S-15.09 lesson) --

    /// Load-bearing preemptive test: reads the real `.factory/STATE.md` from
    /// disk and asserts `validate_state_md` emits no violations. This catches
    /// false-positive blocks that would fire against production content before
    /// any bats test runs. Added preemptively per S-15.09 AC-13 cascade lesson.
    ///
    /// Path resolved via `CARGO_MANIFEST_DIR` relative traversal to the
    /// workspace root `.factory/STATE.md`.
    ///
    /// If STATE.md does not exist (e.g., CI environment without factory
    /// worktree mounted), the test is silently skipped rather than failing.
    #[test]
    fn validate_production_state_md_no_false_positive() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let state_md_path = format!("{manifest_dir}/../../../.factory/STATE.md");
        let content = match std::fs::read_to_string(&state_md_path) {
            Ok(c) => c,
            Err(_) => {
                // STATE.md not present in this build environment; skip silently.
                return;
            }
        };
        let violations = validate_state_md(&content);
        assert!(
            violations.is_empty(),
            "validate_state_md must not produce false-positive violations \
             against production STATE.md; violations found: {violations:?}"
        );
    }
}
