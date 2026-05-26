//! validate-closes-completeness — PostToolUse WASM hook plugin.
//!
//! Blocks any Edit/Write to `lessons.md`, `STATE.md`, `INDEX.md`, or
//! `decision-log.md` that contains Closes annotation structural violations:
//!
//! 1. **Missing `**Closes:**` line** (D-448(b)): every `## L-` or `## PG-` h2
//!    lesson entry in `lessons.md` MUST contain a `**Closes:**` bold-prefix line
//!    with non-empty content. `### Closes` h3 headings are NOT valid.
//!
//! 2. **Forbidden per-mechanism annotation** (D-419(c)+D-420(e)): patterns like
//!    `(per D-413(b) completeness mandate)` and `N items per D-413(b)` are
//!    forbidden in `**Closes:**` lines.
//!
//! 3. **Bare umbrella cite without sample-vs-exhaustive flag** (D-441(c)+D-442(c)):
//!    `D-\d+\.\.D-\d+` patterns MUST carry one of: `(sample)`, `(exhaustive)`,
//!    `sample-vs-exhaustive`, or `see decision-log.md for full range`.
//!
//! 4. **Undeclared documentary-historical exemption** (D-443(b)): lessons.md entries
//!    that lack `**Closes:**` MUST have an explicit `(documentary-historical)` or
//!    `(pre-D-448(b) exemption)` declaration.
//!
//! # Behavioral Contracts
//!
//! - BC-5.39.007: blocks Closes annotation structural violations.
//!
//! # D-NNN closures
//!
//! - D-419(c): Closes cite ID format — structured IDs required.
//! - D-420(e): per-finding mechanism annotations forbidden in Closes blocks.
//! - D-441(c): sample-vs-exhaustive flag required on umbrella citation sites.
//! - D-442(c): retroactive sweep for sample-vs-exhaustive flag.
//! - D-443(b): documentary-historical exemption must be explicit.
//! - D-448(b): lessons.md entries MUST have `**Closes:**` bold-prefix line.
//!
//! # Architecture compliance
//!
//! - HOST_ABI_VERSION = 1 (no new host functions introduced).
//! - Fail-open on every `host::read_file` error (BC-5.39.007 invariant 9).
//! - No `println!` — use `host::log_*` for all diagnostic output.
//! - No `unwrap()` or `expect()` in production paths.
//! - No `regex` crate: hand-rolled pattern scanning to stay within WASM fuel budget.
//! - File-path enforcement via in-plugin guard using `Path::file_name` (Q5/Q6 canonical
//!   pattern; NOT `ends_with` — false-positive on `xSTATE.md`).
//! - `tool = "Edit|Write"` is the canonical Q5/Q6 form for this hook's registry entry.
//! - All byte-index slice expressions MUST use `is_char_boundary()` guards where
//!   multi-byte UTF-8 is possible (BC-5.39.007 invariant 10).

use vsdd_hook_sdk::{HookPayload, HookResult};

/// HOST_ABI_VERSION declares the ABI contract version this plugin was built against.
pub const HOST_ABI_VERSION: u32 = 1;

/// Maximum bytes to read from any target file via `host::read_file`.
///
/// Set to 512 KiB (524288 bytes) — parity with sibling hooks; comfortably
/// above lessons.md soft cap (3500 lines × ~250 bytes/line ≈ 875 KiB worst case,
/// but real content is ~800 KB within budget). BC-5.39.007 precondition 4.
pub const MAX_BYTES: u32 = 524_288;

// ---------------------------------------------------------------------------
// Violation type
// ---------------------------------------------------------------------------

/// A structural violation found in the file content.
///
/// Carries both a human-readable `description` (used verbatim in the block message)
/// and the `cited_raw` body-literal form of the offending text. Structural plumbing
/// per TD-VSDD-059 paper-fix avoidance: block messages quote the exact string the
/// author wrote, not a paraphrase.
#[derive(Debug, Clone)]
pub struct Violation {
    /// Human-readable description of the violation, used in the block message.
    pub description: String,
    /// The raw source text that triggered the violation.
    /// Populated at every violation site per TD-VSDD-059.
    pub cited_raw: String,
}

// ---------------------------------------------------------------------------
// File-path guards (path-component-strict — BC-5.39.007 invariant 3)
// ---------------------------------------------------------------------------

/// Returns `true` if `file_path` names a file whose `file_name` component is
/// exactly `lessons.md`.
///
/// Uses path-component-strict matching (`Path::file_name`) rather than
/// `ends_with`, preventing false-positive fires on paths where the name
/// component differs from `lessons.md`.
///
/// # BC trace
/// BC-5.39.007 precondition 1; invariant 3 — path-component-strict matching.
pub fn is_lessons_md_target(file_path: &str) -> bool {
    std::path::Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        == Some("lessons.md")
}

/// Returns `true` if `file_path` names a file whose `file_name` component is
/// exactly `STATE.md`.
///
/// Uses path-component-strict matching — does NOT use `ends_with`.
///
/// # BC trace
/// BC-5.39.007 precondition 1; invariant 3 — xSTATE.md must NOT match.
pub fn is_state_md_target(file_path: &str) -> bool {
    std::path::Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        == Some("STATE.md")
}

/// Returns `true` if `file_path` names a file whose `file_name` component is
/// exactly `INDEX.md`.
///
/// Uses path-component-strict matching — does NOT use `ends_with`.
///
/// # BC trace
/// BC-5.39.007 precondition 1; invariant 3.
pub fn is_index_md_target(file_path: &str) -> bool {
    std::path::Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        == Some("INDEX.md")
}

/// Returns `true` if `file_path` names a file whose `file_name` component is
/// exactly `decision-log.md`.
///
/// Uses path-component-strict matching — does NOT use `ends_with`.
///
/// # BC trace
/// BC-5.39.007 precondition 1; invariant 3.
pub fn is_decision_log_target(file_path: &str) -> bool {
    std::path::Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        == Some("decision-log.md")
}

// ---------------------------------------------------------------------------
// Lesson entry detection (BC-5.39.007 precondition 5)
// ---------------------------------------------------------------------------

/// A single lesson entry in `lessons.md` — one `## L-` or `## PG-` h2 section.
#[derive(Debug, Clone)]
pub struct LessonEntry {
    /// The h2 heading text (e.g., `## L-EDP1-007 — description`).
    pub heading: String,
    /// The body content from after the heading line to the next h2 or end-of-content.
    pub body: String,
    /// The line number (0-indexed) of the h2 heading.
    pub start_line: usize,
}

/// Find all lesson entries in `lessons.md` content.
///
/// A lesson entry begins at any line starting with `## L-` or `## PG-` and
/// runs to the next `## ` heading (any h2) or end-of-content. The body content
/// is the text between the h2 heading line (exclusive) and the next h2 (exclusive).
///
/// Returns a `Vec<LessonEntry>` with one entry per h2 heading found.
/// If no lesson entries exist, returns an empty Vec.
///
/// Pure: no I/O; no regex crate.
///
/// # BC trace
/// BC-5.39.007 precondition 5 — h2-heading-based lesson entry detection.
/// BC-5.39.007 invariant 6 — `## L-` or `## PG-` begins each entry.
pub fn find_lesson_entries(content: &str) -> Vec<LessonEntry> {
    let mut entries: Vec<LessonEntry> = Vec::new();
    let lines: Vec<&str> = content.split('\n').collect();

    let mut i = 0usize;
    while i < lines.len() {
        let line = lines[i].trim_end_matches('\r');
        // Check if this line is a lesson entry h2 heading.
        if line.starts_with("## L-") || line.starts_with("## PG-") {
            let heading = line.to_string();
            let start_line = i;
            // Collect body lines until the next h2 (any `## ` prefix) or end.
            let body_start = i + 1;
            let mut body_end = lines.len();
            let mut j = body_start;
            while j < lines.len() {
                let body_line = lines[j].trim_end_matches('\r');
                if body_line.starts_with("## ") {
                    body_end = j;
                    break;
                }
                j += 1;
            }
            // Build body string from lines[body_start..body_end].
            let body = lines[body_start..body_end]
                .iter()
                .map(|l| l.trim_end_matches('\r'))
                .collect::<Vec<_>>()
                .join("\n");
            entries.push(LessonEntry {
                heading,
                body,
                start_line,
            });
            // Continue from the next line after the heading.
            // The next iteration will find the next h2 if there is one.
            i = body_start;
        } else {
            i += 1;
        }
    }
    entries
}

// ---------------------------------------------------------------------------
// Closes-block presence check (BC-5.39.007 postcondition 2; D-448(b))
// ---------------------------------------------------------------------------

/// Check whether a lesson entry has a valid `**Closes:**` bold-prefix line.
///
/// Returns a `Vec<Violation>` (0 or 1 entries) for this entry:
/// - If the entry body contains `(documentary-historical)` or
///   `(pre-D-448(b) exemption)`: exempt — returns empty Vec (BC-5.39.007 invariant 7).
/// - If the body contains a `**Closes:**` bold-prefix line with non-empty content
///   after the colon: returns empty Vec (valid).
/// - If the body contains `### Closes` (h3 heading, wrong format): returns one
///   Violation citing D-448(b) (EC-018 path — wrong format).
/// - If `**Closes:**` is present but the content after the colon is empty or
///   whitespace-only: returns one Violation citing D-448(b).
/// - If `**Closes:**` is entirely absent (and no exemption): returns one Violation
///   citing D-448(b).
///
/// Pure: no I/O; no regex crate.
///
/// # BC trace
/// BC-5.39.007 postcondition 2; EC-001..EC-003; EC-018; EC-022; D-448(b).
pub fn check_closes_present(entry: &LessonEntry) -> Vec<Violation> {
    let body = &entry.body;

    // Exemption check (BC-5.39.007 invariant 7; D-443(b); EC-012).
    // The exemption declaration may appear in the heading OR the body of the lesson entry
    // (common pattern: heading embeds "(pre-D-448(b) exemption)" as part of the title).
    if entry.heading.contains("(documentary-historical)")
        || entry.heading.contains("(pre-D-448(b) exemption)")
        || body.contains("(documentary-historical)")
        || body.contains("(pre-D-448(b) exemption)")
    {
        return Vec::new();
    }

    // Check for `### Closes` h3 heading (wrong format — EC-018).
    // This takes precedence over the "missing **Closes:**" check so the message is specific.
    let has_h3_closes = body
        .split('\n')
        .any(|line| line.trim_end_matches('\r').trim() == "### Closes");

    // Check for `**Closes:**` bold-prefix line.
    let mut closes_content: Option<&str> = None;
    for line in body.split('\n') {
        let trimmed = line.trim_end_matches('\r').trim();
        if let Some(rest) = trimmed.strip_prefix("**Closes:**") {
            closes_content = Some(rest);
            break;
        }
    }

    match closes_content {
        Some(rest) => {
            // `**Closes:**` found. Check content after colon (EC-002; EC-022).
            let content_after = rest.trim();
            if content_after.is_empty() {
                // Empty or whitespace-only content after `**Closes:**`.
                return vec![Violation {
                    description: format!(
                        "lesson entry `{}` has `**Closes:**` label but no content after \
                         the colon — empty Closes line is equivalent to absent; \
                         add structured cite IDs per D-448(b)",
                        entry.heading
                    ),
                    cited_raw: "**Closes:**".to_string(),
                }];
            }
            // Non-empty content: valid. (cite ID format checked separately.)
            Vec::new()
        }
        None => {
            // No `**Closes:**` line found.
            if has_h3_closes {
                // EC-018: `### Closes` h3 heading used instead of `**Closes:**`.
                vec![Violation {
                    description: format!(
                        "lesson entry `{}` uses `### Closes` h3 heading instead of \
                         `**Closes:**` bold-prefix line — wrong format per D-448(b); \
                         the canonical form is `**Closes:** <cite-list>` on a single line",
                        entry.heading
                    ),
                    cited_raw: "### Closes".to_string(),
                }]
            } else {
                // No `**Closes:**` line at all and no exemption (EC-001; EC-013).
                vec![Violation {
                    description: format!(
                        "lesson entry `{}` is missing a `**Closes:**` bold-prefix line; \
                         every lesson entry must include `**Closes:** <cite-list>` per D-448(b); \
                         if this entry predates D-448(b), add `(pre-D-448(b) exemption)` per D-443(b)",
                        entry.heading
                    ),
                    cited_raw: entry.heading.clone(),
                }]
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Forbidden annotation check (BC-5.39.007 postcondition 3; D-420(e))
// ---------------------------------------------------------------------------

/// Check a `**Closes:**` line for forbidden per-mechanism annotations.
///
/// Scans for two forbidden patterns per D-420(e):
/// 1. `(per D-413(b) completeness mandate)` — with optional "completeness "
/// 2. `N items per D-413(b)` — aggregate shorthand (any digit count)
///
/// Returns `Some(Violation)` citing D-420(e) if either pattern is found,
/// `None` if the line is clean.
///
/// Hand-rolled scanning — no regex crate.
///
/// # BC trace
/// BC-5.39.007 postcondition 3; invariant 5; EC-004; EC-005; D-420(e).
pub fn check_forbidden_annotations(line: &str) -> Option<Violation> {
    // Pattern 1: `(per D-413(b) completeness mandate)` or `(per D-413(b) mandate)`
    // The optional group is `completeness `.
    // Hand-rolled: look for "(per D-413(b) " then check for "completeness mandate)"
    // or "mandate)".
    let marker1 = "(per D-413(b) ";
    if let Some(pos) = line.find(marker1) {
        let after = &line[pos + marker1.len()..];
        if after.starts_with("mandate)") || after.starts_with("completeness mandate)") {
            let end = if after.starts_with("completeness mandate)") {
                pos + marker1.len() + "completeness mandate)".len()
            } else {
                pos + marker1.len() + "mandate)".len()
            };
            let cited =
                if end <= line.len() && line.is_char_boundary(pos) && line.is_char_boundary(end) {
                    line[pos..end].to_string()
                } else {
                    marker1.to_string()
                };
            return Some(Violation {
                description: format!(
                    "Closes line contains forbidden per-mechanism annotation `{cited}`; \
                     aggregate annotations are forbidden per D-420(e) — \
                     cite each finding individually by structured ID"
                ),
                cited_raw: cited,
            });
        }
    }

    // Pattern 2: `N items per D-413(b)` — digit(s) followed by " items per D-413(b)"
    // Hand-rolled: scan for " items per D-413(b)" and check that there's a digit before it.
    let shorthand_suffix = " items per D-413(b)";
    if let Some(pos) = line.find(shorthand_suffix) {
        // Check if there's a digit immediately before the " items" part.
        if pos > 0 {
            // Walk backwards to find the last non-digit before pos.
            let before = &line[..pos];
            if before.ends_with(|c: char| c.is_ascii_digit()) {
                // Rebuild the matched segment: find the digit run before pos.
                let digit_start = before
                    .rfind(|c: char| !c.is_ascii_digit())
                    .map(|i| i + 1)
                    .unwrap_or(0);
                let end_pos = pos + shorthand_suffix.len();
                let cited_raw =
                    if line.is_char_boundary(digit_start) && line.is_char_boundary(end_pos) {
                        line[digit_start..end_pos].to_string()
                    } else {
                        shorthand_suffix.trim().to_string()
                    };
                return Some(Violation {
                    description: format!(
                        "Closes line contains forbidden aggregate-shorthand annotation `{cited_raw}`; \
                         `N items per D-413(b)` shorthand is forbidden per D-420(e) — \
                         list each finding individually by structured ID"
                    ),
                    cited_raw,
                });
            }
        }
    }

    None
}

// ---------------------------------------------------------------------------
// Cite ID format check (BC-5.39.007 postcondition 8; D-419(c))
// ---------------------------------------------------------------------------

/// Prefixes where the character after the prefix MUST be a digit.
///
/// Per BC-5.39.007 postcondition 8:
/// - `D-\d+` — e.g. `D-444`
/// - `F-P\d+` — e.g. `F-P39-001`
/// - `TD-VSDD-\d+` — e.g. `TD-VSDD-059`
/// - `L-EDP1-\d+` — e.g. `L-EDP1-052`
const DIGIT_REQUIRED_PREFIXES: &[&str] = &["D-", "F-P", "TD-VSDD-", "L-EDP1-"];

/// Prefixes where the character after the prefix may be any ASCII alphanumeric.
///
/// Per BC-5.39.007 postcondition 8:
/// - `PG-[A-Za-z0-9-]+` — e.g. `PG-S-15.14`
/// - `ADV-EDP1-P\d+` — e.g. `ADV-EDP1-P39-FINDING-001` (starts with `P`, a letter)
const ALPHA_OK_PREFIXES: &[&str] = &["PG-", "ADV-EDP1-"];

/// Check cite IDs in a `**Closes:**` line for malformed (non-structured) IDs.
///
/// Extracts content after `**Closes:**`. Splits by commas. For each
/// comma-separated token:
/// - Trims whitespace
/// - Checks `has_valid_id_in_token`
/// - If a token contains a valid ID prefix (the whole token may have trailing
///   descriptive text), it passes
/// - Freeform prose tokens with no recognized ID prefix → Violation citing D-419(c)
///
/// Cross-site staleness (correctly formatted ID referencing nonexistent D-NNN)
/// is Phase 2 only — emit log_warn advisory and do NOT add to violations.
///
/// Returns `Vec<Violation>` (empty if clean).
///
/// Pure: no I/O; no regex crate.
///
/// # BC trace
/// BC-5.39.007 postcondition 8; invariant 8; EC-010; EC-011; EC-019; D-419(c).
pub fn check_cite_id_format(closes_line: &str) -> Vec<Violation> {
    // Extract content after `**Closes:**`.
    let closes_prefix = "**Closes:**";
    let content = match closes_line.find(closes_prefix) {
        Some(pos) => {
            let start = pos + closes_prefix.len();
            if closes_line.is_char_boundary(start) {
                &closes_line[start..]
            } else {
                return Vec::new();
            }
        }
        None => return Vec::new(),
    };

    let content = content.trim();
    if content.is_empty() {
        // Empty content — this is caught by check_closes_present, not here.
        return Vec::new();
    }

    let mut violations = Vec::new();

    // Split by comma and check each token.
    for raw_token in content.split(',') {
        let token = raw_token.trim();

        if token.is_empty() {
            continue;
        }

        // Check if this token contains a valid ID anywhere (token may have trailing text).
        // Strategy: check if any prefix of the token (starting from position 0) matches
        // a valid ID prefix. We scan the token for the FIRST recognized ID prefix occurrence.
        let token_has_valid_id = has_valid_id_in_token(token);

        if !token_has_valid_id {
            // Freeform prose — no structured ID (EC-011; D-419(c)).
            violations.push(Violation {
                description: format!(
                    "Closes line contains malformed cite `{token}` — no recognized structured \
                     ID pattern (expected D-NNN, F-PNN-NNN, TD-VSDD-NNN, PG-NNN, L-EDP1-NNN, \
                     or ADV-EDP1-...); freeform prose is forbidden per D-419(c)"
                ),
                cited_raw: token.to_string(),
            });
        }
        // Note: cross-site staleness (valid format, nonexistent D-NNN) is Phase 2 only.
        // We do NOT check whether D-999 exists. The caller emits log_warn for advisory.
    }

    violations
}

/// Returns `true` if `token` contains a recognized structured ID anywhere within it.
///
/// This handles cases like `D-444 codified` (starts with valid prefix `D-` + digit)
/// or `F-P39-001` (valid), or `fixed the thing` (no valid prefix).
///
/// Algorithm:
/// 1. For each digit-required prefix, check if the token starts with that prefix
///    followed by at least one ASCII digit (not any alphanumeric).
/// 2. For each alphanumeric-ok prefix, check if the token starts with that prefix
///    followed by at least one ASCII alphanumeric character.
/// 3. Additionally accept parenthetical-only and bold-markdown tokens as annotations.
///
/// Per BC-5.39.007 postcondition 8:
/// - `D-`, `F-P`, `TD-VSDD-`, `L-EDP1-` require digit after prefix.
/// - `PG-`, `ADV-EDP1-` allow any ASCII alphanumeric after prefix.
fn has_valid_id_in_token(token: &str) -> bool {
    let t = token.trim();

    if t.is_empty() || t == "---" {
        return true;
    }

    // Parenthetical-only tokens are annotations, not standalone cites.
    // But we don't want to accept them as "having a valid ID" — they should only
    // appear adjacent to a valid cite. However, if they're the only thing in a
    // comma-separated slot, they may be descriptive context.
    // For now: treat parenthetical-only as not-a-violation (annotation context).
    if t.starts_with('(') && t.ends_with(')') {
        return true;
    }

    // Check for markdown bold tokens.
    if t.starts_with("**") {
        return true;
    }

    // Check digit-required prefixes: character after prefix MUST be a digit.
    for &prefix in DIGIT_REQUIRED_PREFIXES {
        if let Some(rest) = t.strip_prefix(prefix)
            && rest
                .chars()
                .next()
                .map(|c| c.is_ascii_digit())
                .unwrap_or(false)
        {
            return true;
        }
    }

    // Check alphanumeric-ok prefixes: character after prefix may be any ASCII alphanumeric.
    for &prefix in ALPHA_OK_PREFIXES {
        if let Some(rest) = t.strip_prefix(prefix)
            && rest
                .chars()
                .next()
                .map(|c| c.is_ascii_alphanumeric())
                .unwrap_or(false)
        {
            return true;
        }
    }

    false
}

// ---------------------------------------------------------------------------
// Umbrella-flag check (BC-5.39.007 postcondition 4-7; D-441(c)+D-442(c))
// ---------------------------------------------------------------------------

/// Check for bare umbrella citation patterns `D-NNN..D-NNN` without adjacent
/// sample-vs-exhaustive flags.
///
/// Scans content line-by-line. For each line containing a `D-\d+\.\.D-\d+`
/// pattern, checks that the SAME LINE also contains one of:
/// - `(sample)` — literal substring
/// - `(exhaustive)` — literal substring
/// - `sample-vs-exhaustive` — literal substring
/// - `see decision-log.md for full range` — literal substring
///
/// If none of these flags are present on the same line as a bare umbrella cite,
/// a `Violation` is added citing D-441(c)+D-442(c).
///
/// Returns `Vec<Violation>` — all bare umbrella cites on flagless lines.
///
/// Pure: no I/O; no regex crate. Hand-rolled D-range detection.
///
/// # BC trace
/// BC-5.39.007 postconditions 4-7; precondition 6; invariant 4; EC-006..EC-009; D-441(c)+D-442(c).
pub fn check_umbrella_flag(content: &str) -> Vec<Violation> {
    let mut violations = Vec::new();

    for line in content.split('\n') {
        let trimmed = line.trim_end_matches('\r');

        // Find all umbrella cite patterns on this line.
        let umbrella_ranges = find_umbrella_cites(trimmed);
        if umbrella_ranges.is_empty() {
            continue;
        }

        // Check if the sample-vs-exhaustive flag is present on this same line.
        let has_flag = trimmed.contains("(sample)")
            || trimmed.contains("(exhaustive)")
            || trimmed.contains("sample-vs-exhaustive")
            || trimmed.contains("see decision-log.md for full range");

        if !has_flag {
            // Each umbrella cite on this line is a violation.
            for (start, end) in umbrella_ranges {
                let cited = if start <= trimmed.len()
                    && end <= trimmed.len()
                    && trimmed.is_char_boundary(start)
                    && trimmed.is_char_boundary(end)
                {
                    trimmed[start..end].to_string()
                } else {
                    "D-NNN..D-NNN".to_string()
                };
                violations.push(Violation {
                    description: format!(
                        "bare umbrella citation `{cited}` found without sample-vs-exhaustive flag; \
                         umbrella ranges MUST carry `(sample)`, `(exhaustive)`, \
                         `sample-vs-exhaustive`, or `see decision-log.md for full range` \
                         per D-441(c)+D-442(c)"
                    ),
                    cited_raw: cited,
                });
            }
        }
    }

    violations
}

/// Find all `D-NNN..D-NNN` umbrella cite patterns in `line`.
///
/// Returns a Vec of `(start_byte, end_byte)` pairs for each match, where
/// the slice `line[start..end]` is the full matched pattern.
///
/// Hand-rolled scanning — no regex crate. All pattern characters are ASCII,
/// so byte-position scanning is safe.
///
/// Algorithm:
/// 1. Find `D-` followed by one or more ASCII digits.
/// 2. Check that the next two characters are `..`.
/// 3. Check that after `..`, there is `D-` followed by one or more ASCII digits.
/// 4. If all match, record the range.
/// 5. Advance past the match.
fn find_umbrella_cites(line: &str) -> Vec<(usize, usize)> {
    let bytes = line.as_bytes();
    let len = bytes.len();
    let mut results = Vec::new();
    let mut i = 0usize;

    while i + 2 < len {
        // Look for 'D' followed by '-'.
        if bytes[i] == b'D' && i + 1 < len && bytes[i + 1] == b'-' {
            let d1_start = i;
            let mut j = i + 2; // past "D-"

            // Collect digits for first D-NNN.
            while j < len && bytes[j].is_ascii_digit() {
                j += 1;
            }
            let d1_digit_end = j;

            // Must have at least one digit after "D-".
            if d1_digit_end == i + 2 {
                i += 1;
                continue;
            }

            // Check for ".." separator.
            if j + 1 < len && bytes[j] == b'.' && bytes[j + 1] == b'.' {
                let after_dots = j + 2;

                // Check for "D-" after "..".
                if after_dots + 1 < len
                    && bytes[after_dots] == b'D'
                    && bytes[after_dots + 1] == b'-'
                {
                    let mut k = after_dots + 2; // past second "D-"

                    // Collect digits for second D-NNN.
                    while k < len && bytes[k].is_ascii_digit() {
                        k += 1;
                    }
                    let d2_digit_end = k;

                    // Must have at least one digit after second "D-".
                    if d2_digit_end > after_dots + 2 {
                        // Valid umbrella cite: D-NNN..D-NNN from d1_start to d2_digit_end.
                        results.push((d1_start, d2_digit_end));
                        i = d2_digit_end;
                        continue;
                    }
                }
            }
        }
        i += 1;
    }

    results
}

// ---------------------------------------------------------------------------
// Block message formatter
// ---------------------------------------------------------------------------

/// Format a list of violations into a single `HookResult::block_with_fix`.
///
/// Enumerates all violations in a single block message per BC-5.39.007
/// postcondition 9.
fn emit_block(hook_name: &str, violations: &[Violation]) -> HookResult {
    let reason = violations
        .iter()
        .enumerate()
        .map(|(i, v)| format!("{}. {}", i + 1, v.description))
        .collect::<Vec<_>>()
        .join("\n");

    let recommendation = "Fix all listed violations before re-writing the file. \
        Ensure every lessons.md entry has a **Closes:** bold-prefix line with structured IDs, \
        umbrella citation ranges carry (sample)/(exhaustive)/sample-vs-exhaustive flags, \
        and no Closes lines contain forbidden aggregate annotations."
        .to_string();

    let code = violations
        .iter()
        .map(|v| format!("  cited_raw: {:?}", v.cited_raw))
        .collect::<Vec<_>>()
        .join("\n");

    HookResult::block_with_fix(hook_name, &reason, &recommendation, &code)
}

// ---------------------------------------------------------------------------
// Main entry point
// ---------------------------------------------------------------------------

/// PostToolUse entry point called by `vsdd_hook_sdk::__internal::run`.
///
/// Dispatches to the appropriate arm based on path-component-strict guards,
/// runs the applicable checks, and returns a single `BlockWithFix` if any
/// violations are found, or `Continue` if the file is clean.
///
/// # Control flow
///
/// 1. Extract `file_path` from `payload.tool_input`. If absent: Continue + log_warn.
/// 2. Route via `is_*_target()` guards:
///    - `lessons.md`: run lesson-entry detection → per-entry closes checks → umbrella-flag check.
///    - `STATE.md`: run umbrella-flag check only (no lesson-entry checks).
///    - `INDEX.md`: run umbrella-flag check only.
///    - `decision-log.md`: run umbrella-flag check only.
///    - else: Continue (not our file).
/// 3. Read file via `host::read_file(path, 524288, 2000)`. On error: Continue + log_warn.
/// 4. If file is empty (zero bytes): Continue + log_warn advisory (EC-021).
/// 5. Accumulate violations. If empty: Continue. If non-empty: single BlockWithFix.
///
/// # BC trace
/// BC-5.39.007 postconditions 1-10; invariants 1-10; EC-001..EC-022.
pub fn on_post_tool_use(payload: HookPayload) -> HookResult {
    use vsdd_hook_sdk::host;

    const HOOK_NAME: &str = "validate-closes-completeness";

    // Step 1: Extract file_path from tool_input.
    let file_path = match payload.tool_input.get("file_path").and_then(|v| v.as_str()) {
        Some(p) => p.to_string(),
        None => {
            host::log_warn(
                "[validate-closes-completeness] file_path absent from tool_input — graceful degrade",
            );
            return HookResult::Continue;
        }
    };

    // Step 2: Route via path guards.
    let arm = if is_lessons_md_target(&file_path) {
        Arm::Lessons
    } else if is_state_md_target(&file_path) {
        Arm::State
    } else if is_index_md_target(&file_path) {
        Arm::Index
    } else if is_decision_log_target(&file_path) {
        Arm::DecisionLog
    } else {
        // Not a target file — continue without action.
        return HookResult::Continue;
    };

    // Step 3: Read file via host::read_file (fail-open per invariant 9).
    let content = match host::read_file(&file_path, MAX_BYTES, 2000) {
        Ok(bytes) => {
            // Step 4: Empty file check (EC-021).
            if bytes.is_empty() {
                host::log_warn(&format!(
                    "[{HOOK_NAME}] {file_path} is empty (zero bytes) — no violations possible \
                     (EC-021); advisory only"
                ));
                return HookResult::Continue;
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

    // Step 5: Run arm-specific checks.
    let mut violations: Vec<Violation> = Vec::new();

    match arm {
        Arm::Lessons => {
            // lessons.md arm: lesson-entry detection + closes checks + umbrella-flag.
            let entries = find_lesson_entries(&content);
            for entry in &entries {
                // Check **Closes:** presence.
                violations.extend(check_closes_present(entry));

                // Check **Closes:** lines for forbidden annotations and cite format.
                for line in entry.body.split('\n') {
                    let trimmed = line.trim_end_matches('\r').trim();
                    if trimmed.starts_with("**Closes:**") {
                        // Forbidden annotation check.
                        if let Some(v) = check_forbidden_annotations(trimmed) {
                            violations.push(v);
                        }
                        // Cite ID format check (Phase 1: format only, not cross-site).
                        let cite_violations = check_cite_id_format(trimmed);
                        violations.extend(cite_violations);
                    }
                }
            }
            // Umbrella-flag check on full content.
            violations.extend(check_umbrella_flag(&content));
        }
        Arm::State | Arm::Index | Arm::DecisionLog => {
            // Umbrella-flag check only (no lesson-entry checks).
            violations.extend(check_umbrella_flag(&content));

            // Also check **Closes:** lines in STATE.md/INDEX.md/decision-log.md
            // for cite ID format (postcondition 8 — all arms).
            // NOTE: check_forbidden_annotations does NOT apply to non-lessons arms
            // per BC-5.39.007 postcondition 3; that check is scoped to lessons.md only.
            for line in content.split('\n') {
                let trimmed = line.trim_end_matches('\r').trim();
                if trimmed.starts_with("**Closes:**") {
                    let cite_violations = check_cite_id_format(trimmed);
                    violations.extend(cite_violations);
                }
            }
        }
    }

    if violations.is_empty() {
        HookResult::Continue
    } else {
        emit_block(HOOK_NAME, &violations)
    }
}

/// Dispatch arm for the four file targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Arm {
    Lessons,
    State,
    Index,
    DecisionLog,
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used)]
mod tests {
    use super::*;

    // ---- path guards ----

    #[test]
    fn test_is_lessons_md_target() {
        assert!(is_lessons_md_target(".factory/cycles/v1.0/lessons.md"));
        assert!(is_lessons_md_target("lessons.md"));
        assert!(!is_lessons_md_target(".factory/cycles/v1.0/xlessons.md"));
        assert!(!is_lessons_md_target(".factory/STATE.md"));
    }

    #[test]
    fn test_is_state_md_target() {
        assert!(is_state_md_target(".factory/STATE.md"));
        assert!(is_state_md_target("STATE.md"));
        // Path-component-strict: xSTATE.md must NOT match.
        assert!(!is_state_md_target(".factory/xSTATE.md"));
        assert!(!is_state_md_target(".factory/cycles/STATE.md.bak"));
    }

    #[test]
    fn test_is_index_md_target() {
        assert!(is_index_md_target(".factory/cycles/v1.0/INDEX.md"));
        assert!(is_index_md_target("INDEX.md"));
        assert!(!is_index_md_target(".factory/cycles/v1.0/xINDEX.md"));
    }

    #[test]
    fn test_is_decision_log_target() {
        assert!(is_decision_log_target(
            ".factory/cycles/v1.0/decision-log.md"
        ));
        assert!(is_decision_log_target("decision-log.md"));
        assert!(!is_decision_log_target(
            ".factory/cycles/v1.0/xdecision-log.md"
        ));
    }

    // ---- find_lesson_entries ----

    #[test]
    fn test_find_lesson_entries_empty() {
        let entries = find_lesson_entries("");
        assert!(entries.is_empty());
    }

    #[test]
    fn test_find_lesson_entries_single() {
        let content =
            "# Header\n\n## L-EDP1-007 — Some lesson\n\nBody text here.\n\n**Closes:** D-444";
        let entries = find_lesson_entries(content);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].heading, "## L-EDP1-007 — Some lesson");
        assert!(entries[0].body.contains("Body text here."));
        assert!(entries[0].body.contains("**Closes:** D-444"));
    }

    #[test]
    fn test_find_lesson_entries_multiple() {
        let content = "\
## L-EDP1-007 — Lesson one

Body 1.

**Closes:** D-444

## L-EDP1-008 — Lesson two

Body 2.

**Closes:** D-445
";
        let entries = find_lesson_entries(content);
        assert_eq!(entries.len(), 2);
        assert!(entries[0].heading.contains("007"));
        assert!(entries[1].heading.contains("008"));
        // Entry 1 body should not include entry 2 heading
        assert!(!entries[0].body.contains("Lesson two"));
    }

    #[test]
    fn test_find_lesson_entries_pg_heading() {
        let content = "## PG-S-15.14 — Some guideline\n\nBody.\n\n**Closes:** D-446";
        let entries = find_lesson_entries(content);
        assert_eq!(entries.len(), 1);
        assert!(entries[0].heading.starts_with("## PG-"));
    }

    // ---- check_closes_present ----

    #[test]
    fn test_closes_present_valid() {
        let entry = LessonEntry {
            heading: "## L-EDP1-007 — Test".to_string(),
            body: "Some body.\n\n**Closes:** D-444 codified".to_string(),
            start_line: 0,
        };
        assert!(check_closes_present(&entry).is_empty());
    }

    #[test]
    fn test_closes_present_missing() {
        let entry = LessonEntry {
            heading: "## L-EDP1-007 — Test".to_string(),
            body: "Some body without closes.".to_string(),
            start_line: 0,
        };
        let violations = check_closes_present(&entry);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].description.contains("D-448(b)"));
    }

    #[test]
    fn test_closes_present_empty_label() {
        let entry = LessonEntry {
            heading: "## L-EDP1-007 — Test".to_string(),
            body: "Some body.\n\n**Closes:**".to_string(),
            start_line: 0,
        };
        let violations = check_closes_present(&entry);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].description.contains("D-448(b)"));
    }

    #[test]
    fn test_closes_present_h3_heading() {
        let entry = LessonEntry {
            heading: "## L-EDP1-010 — Test".to_string(),
            body: "Some body.\n\n### Closes\n\nD-444".to_string(),
            start_line: 0,
        };
        let violations = check_closes_present(&entry);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].description.contains("D-448(b)"));
        assert!(violations[0].cited_raw.contains("### Closes"));
    }

    #[test]
    fn test_closes_present_exemption_documentary_historical() {
        let entry = LessonEntry {
            heading: "## L-EDP1-003 — Old lesson".to_string(),
            body: "This entry predates D-448(b) (documentary-historical).".to_string(),
            start_line: 0,
        };
        assert!(check_closes_present(&entry).is_empty());
    }

    #[test]
    fn test_closes_present_exemption_pre_d448() {
        let entry = LessonEntry {
            heading: "## L-EDP1-003 — Old lesson".to_string(),
            body: "This entry predates D-448(b) (pre-D-448(b) exemption).".to_string(),
            start_line: 0,
        };
        assert!(check_closes_present(&entry).is_empty());
    }

    // ---- check_forbidden_annotations ----

    #[test]
    fn test_forbidden_annotation_completeness_mandate() {
        let line = "**Closes:** F-P39-001, F-P39-002 (per D-413(b) completeness mandate)";
        let result = check_forbidden_annotations(line);
        assert!(result.is_some());
        let v = result.unwrap();
        assert!(v.description.contains("D-420(e)"));
    }

    #[test]
    fn test_forbidden_annotation_mandate_no_completeness() {
        let line = "**Closes:** F-P39-001 (per D-413(b) mandate)";
        let result = check_forbidden_annotations(line);
        assert!(result.is_some());
        let v = result.unwrap();
        assert!(v.description.contains("D-420(e)"));
    }

    #[test]
    fn test_forbidden_annotation_shorthand() {
        let line = "**Closes:** 5 items per D-413(b)";
        let result = check_forbidden_annotations(line);
        assert!(result.is_some());
        let v = result.unwrap();
        assert!(v.description.contains("D-420(e)"));
    }

    #[test]
    fn test_forbidden_annotation_clean() {
        let line = "**Closes:** D-444, F-P39-001";
        assert!(check_forbidden_annotations(line).is_none());
    }

    // ---- check_cite_id_format ----

    #[test]
    fn test_cite_id_valid_d_nnn() {
        let line = "**Closes:** D-444";
        assert!(check_cite_id_format(line).is_empty());
    }

    #[test]
    fn test_cite_id_valid_d_nnn_with_descriptive_text() {
        let line = "**Closes:** D-444 codified";
        assert!(check_cite_id_format(line).is_empty());
    }

    #[test]
    fn test_cite_id_valid_f_p() {
        let line = "**Closes:** F-P39-001";
        assert!(check_cite_id_format(line).is_empty());
    }

    #[test]
    fn test_cite_id_valid_td_vsdd() {
        let line = "**Closes:** TD-VSDD-059";
        assert!(check_cite_id_format(line).is_empty());
    }

    #[test]
    fn test_cite_id_valid_l_edp1() {
        let line = "**Closes:** L-EDP1-052";
        assert!(check_cite_id_format(line).is_empty());
    }

    #[test]
    fn test_cite_id_malformed_freeform() {
        let line = "**Closes:** fixed the thing";
        let violations = check_cite_id_format(line);
        assert!(!violations.is_empty());
        assert!(violations[0].description.contains("D-419(c)"));
    }

    #[test]
    fn test_cite_id_phase1_advisory_d999() {
        // D-999 is correctly formatted — should NOT produce a violation.
        // Cross-site staleness is Phase 2 only.
        let line = "**Closes:** D-999";
        assert!(check_cite_id_format(line).is_empty());
    }

    // ---- has_valid_id_in_token: digit-required vs alphanumeric-ok prefix split ----

    #[test]
    fn test_has_valid_id_d_digits_valid() {
        // D- followed by digit: valid
        assert!(has_valid_id_in_token("D-123"));
    }

    #[test]
    fn test_has_valid_id_d_alpha_invalid() {
        // D- followed by letter: INVALID — digit required after D-
        assert!(!has_valid_id_in_token("D-abc"));
    }

    #[test]
    fn test_has_valid_id_pg_alpha_valid() {
        // PG- followed by letter: valid — alphanumeric OK
        assert!(has_valid_id_in_token("PG-something"));
    }

    #[test]
    fn test_has_valid_id_pg_digits_valid() {
        // PG- followed by digit: also valid (alphanumeric OK includes digits)
        assert!(has_valid_id_in_token("PG-001"));
    }

    #[test]
    fn test_has_valid_id_adv_edp1_valid() {
        // ADV-EDP1- followed by 'P' (letter): valid — alphanumeric OK
        assert!(has_valid_id_in_token("ADV-EDP1-P39"));
    }

    #[test]
    fn test_has_valid_id_l_edp1_digits_valid() {
        // L-EDP1- followed by digit: valid
        assert!(has_valid_id_in_token("L-EDP1-052"));
    }

    #[test]
    fn test_has_valid_id_l_edp1_alpha_invalid() {
        // L-EDP1- followed by letter: INVALID — digit required after L-EDP1-
        assert!(!has_valid_id_in_token("L-EDP1-abc"));
    }

    #[test]
    fn test_has_valid_id_td_vsdd_digits_valid() {
        // TD-VSDD- followed by digit: valid
        assert!(has_valid_id_in_token("TD-VSDD-059"));
    }

    #[test]
    fn test_has_valid_id_td_vsdd_alpha_invalid() {
        // TD-VSDD- followed by letter: INVALID — digit required
        assert!(!has_valid_id_in_token("TD-VSDD-xyz"));
    }

    #[test]
    fn test_has_valid_id_freeform_invalid() {
        // No recognized prefix at all
        assert!(!has_valid_id_in_token("fixed the thing"));
    }

    // ---- check_umbrella_flag ----

    #[test]
    fn test_umbrella_flag_bare_range_no_flag() {
        let content = "Range D-389..D-480 covers all decisions.\n";
        let violations = check_umbrella_flag(content);
        assert!(!violations.is_empty());
        assert!(violations[0].description.contains("D-441(c)"));
    }

    #[test]
    fn test_umbrella_flag_with_sample_flag() {
        let content = "Range D-389..D-480 (sample; see decision-log.md for full range).\n";
        assert!(check_umbrella_flag(content).is_empty());
    }

    #[test]
    fn test_umbrella_flag_with_exhaustive_flag() {
        let content = "Range D-401..D-454 (exhaustive).\n";
        assert!(check_umbrella_flag(content).is_empty());
    }

    #[test]
    fn test_umbrella_flag_with_sample_vs_exhaustive() {
        let content = "See sample-vs-exhaustive discussion for D-401..D-454.\n";
        assert!(check_umbrella_flag(content).is_empty());
    }

    #[test]
    fn test_umbrella_flag_no_range() {
        let content = "Single cite D-444 is fine.\n";
        assert!(check_umbrella_flag(content).is_empty());
    }

    // ---- find_umbrella_cites ----

    #[test]
    fn test_find_umbrella_cites_single() {
        let line = "Range D-389..D-480 here.";
        let found = find_umbrella_cites(line);
        assert_eq!(found.len(), 1);
        let (s, e) = found[0];
        assert_eq!(&line[s..e], "D-389..D-480");
    }

    #[test]
    fn test_find_umbrella_cites_none() {
        let line = "Single D-444 cite.";
        assert!(find_umbrella_cites(line).is_empty());
    }

    #[test]
    fn test_find_umbrella_cites_multiple() {
        let line = "D-389..D-400 and D-401..D-454 both.";
        let found = find_umbrella_cites(line);
        assert_eq!(found.len(), 2);
    }
}
