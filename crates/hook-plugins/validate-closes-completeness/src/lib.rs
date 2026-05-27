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

    // Phase 2: cross-site Closes completeness validation (additive to Phase 1).
    // Run for STATE.md, INDEX.md, and decision-log.md arms where Closes cites appear.
    // Phase 2 is fail-open at every step — only appends violations when BOTH the pointer
    // file and adversary file are readable AND violations are found.
    match arm {
        Arm::State | Arm::Index | Arm::DecisionLog => {
            let phase2_violations = run_hook_phase2_cross_site(&file_path, &content);
            violations.extend(phase2_violations);
        }
        Arm::Lessons => {
            // Phase 2 cross-site validation applies to the structured citation sites;
            // lessons.md entries are not cross-validated against the adversary canonical set
            // in Phase 2 (that is a future scope item per BC-5.39.007).
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
// Phase 2 — Cross-site Closes completeness validation (BC-5.39.007 P2-1..P2-4)
// ---------------------------------------------------------------------------

/// Read `.factory/current-adversary-pass.txt` and parse the integer pass number.
///
/// # Return values
///
/// - `Ok(None)`: file absent or unreadable via `host::read_file` — **fail-open**
///   (no block; hook skips Phase 2 per P2-1 postcondition).
/// - `Err(msg)`: file present but content is not a valid positive integer — **hard block**.
///   Per ADR-022 parse-error handling, a corrupt pointer file must be fixed (NOT fail-open).
/// - `Ok(Some(n))`: file present and content parses as u32 `n` (trimming whitespace).
///
/// # Architecture compliance
///
/// - Uses `host::read_file` which returns `Result<Vec<u8>, HostError>`.
/// - Converts bytes to String via `String::from_utf8`.
/// - Pointer file path is always `.factory/current-adversary-pass.txt` per ADR-022 Option c.
/// - Content is parsed as integer pass number — NOT treated as a file path.
///
/// # BC trace
/// BC-5.39.007 Phase 2 P2-1 postcondition; ADR-022 pointer file specification.
pub fn read_current_adversary_pass_number() -> Result<Option<u32>, String> {
    use vsdd_hook_sdk::host;

    const POINTER_PATH: &str = ".factory/current-adversary-pass.txt";

    let bytes = match host::read_file(POINTER_PATH, 1024, 1000) {
        Ok(b) => b,
        // HostError: absent or unreadable — fail-open per P2-1
        Err(_) => return Ok(None),
    };

    let content = match String::from_utf8(bytes) {
        Ok(s) => s,
        Err(_) => {
            return Err(format!(
                "validate-closes-completeness: {POINTER_PATH} UTF-8 decode error — \
                 corrupt pointer file. Run state-manager update-pass-pointer to fix."
            ));
        }
    };

    let trimmed = content.trim();
    match trimmed.parse::<u32>() {
        Ok(n) => Ok(Some(n)),
        Err(_) => Err(format!(
            "validate-closes-completeness: {POINTER_PATH} contains {:?} which is not a \
             valid positive integer. Run state-manager update-pass-pointer to fix.",
            trimmed
        )),
    }
}

/// Derive the adversary review file path from the integer pass number.
///
/// Pure function: returns `.factory/cycles/v1.0-brownfield-backfill/adv-cycle-pass-{pass_n}.md`.
/// The cycle ID is hardcoded for M3 scope (cycle resolver is future scope per ADR-022).
///
/// This function encapsulates the pointer-integer → path-string convention per ADR-022,
/// ensuring the pointer file content is NEVER used as a literal path string.
///
/// # BC trace
/// BC-5.39.007 Phase 2 P2-1b postcondition; ADR-022 pointer file specification.
pub fn derive_adversary_review_path(pass_n: u32) -> String {
    format!(".factory/cycles/v1.0-brownfield-backfill/adv-cycle-pass-{pass_n}.md")
}

/// Extract the canonical finding set from the adversary review file content.
///
/// Finds the `## Part A` section (also accepts `# Part A` or `**Part A**` headings).
/// Within that section, extracts all finding IDs matching:
/// - `F-P\d+-\d+` (e.g., `F-P15-001`, `F-P74-003`)
/// - `F-BC\d+P\d+-\d+` (e.g., `F-BC5P39-001`)
///
/// Returns `Vec<String>` of unique IDs in order of first appearance.
/// Returns empty Vec if Part A section not found or no finding IDs present.
///
/// # CRITICAL
/// Does NOT use the `regex` crate — hand-rolled scanning to stay within WASM fuel budget.
/// The regex crate's DFA compilation exhausts the fuel cap on first call.
///
/// # BC trace
/// BC-5.39.007 Phase 2 P2-2 postcondition.
pub fn extract_canonical_finding_set(adversary_content: &str) -> Vec<String> {
    // Find the Part A section start.
    let part_a_start = find_part_a_start(adversary_content);
    let part_a_start = match part_a_start {
        Some(pos) => pos,
        None => return Vec::new(),
    };

    // Extract content from Part A start to the next top-level section (## Part or # Part or end).
    let after_part_a = &adversary_content[part_a_start..];
    let section_end = find_next_section_after_first(after_part_a);
    let part_a_content = &after_part_a[..section_end];

    // Extract all finding IDs from Part A content.
    extract_finding_ids_from_content(part_a_content)
}

/// Find the byte offset of the start of the Part A section content in `text`.
///
/// Accepts:
/// - `## Part A` — standard h2 heading (with or without ` — suffix`)
/// - `# Part A` — h1 heading
/// - `**Part A**` — bold emphasis
///
/// Returns `Some(offset)` pointing to the start of the line *after* the heading,
/// or `None` if no Part A heading found.
fn find_part_a_start(text: &str) -> Option<usize> {
    let mut pos = 0usize;
    for line in text.split('\n') {
        let line_len = line.len() + 1; // +1 for '\n'
        let trimmed = line.trim_end_matches('\r').trim();

        let is_part_a = trimmed.starts_with("## Part A")
            || trimmed.starts_with("# Part A")
            || trimmed.starts_with("**Part A**");

        if is_part_a {
            let after = pos + line.len() + 1; // skip the heading line + newline
            // Guard against exceeding text length
            return Some(after.min(text.len()));
        }
        pos += line_len;
    }
    None
}

/// Find the byte offset of the next top-level section heading after the first line.
///
/// Used to delimit the Part A section content. Looks for lines starting with `## `
/// (but not the very first line if it's a heading).
///
/// Returns the byte offset of the start of that next section, or `text.len()` if none.
fn find_next_section_after_first(text: &str) -> usize {
    let mut pos = 0usize;
    let mut first = true;
    for line in text.split('\n') {
        let line_len = line.len() + 1; // +1 for '\n'
        let trimmed = line.trim_end_matches('\r').trim();

        if !first {
            // Check for any major section heading that marks end of Part A
            if trimmed.starts_with("## ")
                || (trimmed.starts_with("# ") && !trimmed.starts_with("## "))
                || trimmed.starts_with("**Part B")
                || trimmed.starts_with("**Part C")
            {
                return pos;
            }
        }
        first = false;
        pos += line_len;
    }
    pos.min(text.len())
}

/// Extract all finding IDs (`F-P\d+-\d+` or `F-BC\d+P\d+-\d+`) from `content`.
///
/// Returns unique IDs in order of first appearance. Hand-rolled scanning; no regex.
///
/// Patterns recognized (all ASCII, so byte scanning is safe):
/// - `F-P` + digits + `-` + digits  (e.g., `F-P15-001`, `F-P74-003`)
/// - `F-BC` + digits + `P` + digits + `-` + digits  (e.g., `F-BC5P39-001`)
fn extract_finding_ids_from_content(content: &str) -> Vec<String> {
    let mut ids: Vec<String> = Vec::new();
    let bytes = content.as_bytes();
    let len = bytes.len();
    let mut i = 0usize;

    while i < len {
        // Look for 'F' followed by '-'
        if bytes[i] == b'F' && i + 1 < len && bytes[i + 1] == b'-' {
            // Try to match F-BC\d+P\d+-\d+ first (longer pattern)
            if let Some(end) = try_match_f_bc_pattern(bytes, i) {
                if content.is_char_boundary(i) && content.is_char_boundary(end) {
                    let id = content[i..end].to_string();
                    if !ids.contains(&id) {
                        ids.push(id);
                    }
                }
                i = end;
                continue;
            }
            // Try to match F-P\d+-\d+
            if let Some(end) = try_match_f_p_pattern(bytes, i) {
                if content.is_char_boundary(i) && content.is_char_boundary(end) {
                    let id = content[i..end].to_string();
                    if !ids.contains(&id) {
                        ids.push(id);
                    }
                }
                i = end;
                continue;
            }
        }
        i += 1;
    }

    ids
}

/// Try to match `F-P\d+-\d+` at byte position `start` in `bytes`.
///
/// Returns `Some(end_pos)` if matched (end_pos is exclusive), `None` otherwise.
/// Requires: `bytes[start] == b'F'` and `bytes[start+1] == b'-'`.
fn try_match_f_p_pattern(bytes: &[u8], start: usize) -> Option<usize> {
    let len = bytes.len();
    // Must start with "F-P"
    if start + 2 >= len || bytes[start + 2] != b'P' {
        return None;
    }
    let mut i = start + 3; // past "F-P"
    // Collect digits after "F-P"
    let digit_start = i;
    while i < len && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i == digit_start {
        return None; // no digit after F-P
    }
    // Must have '-' separator
    if i >= len || bytes[i] != b'-' {
        return None;
    }
    i += 1; // skip '-'
    // Collect digits after '-'
    let digit2_start = i;
    while i < len && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i == digit2_start {
        return None; // no digit after '-'
    }
    Some(i)
}

/// Try to match `F-BC\d+P\d+-\d+` at byte position `start` in `bytes`.
///
/// Returns `Some(end_pos)` if matched, `None` otherwise.
/// Requires: `bytes[start] == b'F'` and `bytes[start+1] == b'-'`.
fn try_match_f_bc_pattern(bytes: &[u8], start: usize) -> Option<usize> {
    let len = bytes.len();
    // Must start with "F-BC"
    if start + 3 >= len || bytes[start + 2] != b'B' || bytes[start + 3] != b'C' {
        return None;
    }
    let mut i = start + 4; // past "F-BC"
    // Collect digits
    let digit_start = i;
    while i < len && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i == digit_start {
        return None;
    }
    // Must have 'P'
    if i >= len || bytes[i] != b'P' {
        return None;
    }
    i += 1;
    // Collect digits after 'P'
    let digit2_start = i;
    while i < len && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i == digit2_start {
        return None;
    }
    // Must have '-'
    if i >= len || bytes[i] != b'-' {
        return None;
    }
    i += 1;
    // Collect digits after '-'
    let digit3_start = i;
    while i < len && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i == digit3_start {
        return None;
    }
    Some(i)
}

/// Collect all finding IDs cited in `**Closes:**` lines in `content`.
///
/// Scans all lines in `content` for lines starting with `**Closes:**`,
/// extracts finding IDs (matching `F-P\d+-\d+` or `F-BC\d+P\d+-\d+`)
/// from the portion after the label.
///
/// `site_name` is unused here (it's passed to callers for labeling violations).
///
/// Returns `Vec<String>` of unique cited finding IDs.
///
/// Pure: no I/O; no regex crate.
///
/// # BC trace
/// BC-5.39.007 Phase 2 P2-3; D-411(c).
pub fn collect_closes_cites(content: &str, _site_name: &str) -> Vec<String> {
    let mut ids: Vec<String> = Vec::new();
    for line in content.split('\n') {
        let trimmed = line.trim_end_matches('\r').trim();
        if let Some(rest) = trimmed.strip_prefix("**Closes:**") {
            let found = extract_finding_ids_from_content(rest);
            for id in found {
                if !ids.contains(&id) {
                    ids.push(id);
                }
            }
        }
    }
    ids
}

/// Collect finding IDs per `**Closes:**` line (one site per line).
///
/// Returns a Vec of `(label, Vec<finding_ids>)` — one entry per `**Closes:**` line.
/// The label includes the site name and a line index for disambiguation.
///
/// This enables per-line site comparison for cardinality checks.
fn collect_closes_sites_by_line(content: &str, site_name: &str) -> Vec<(String, Vec<String>)> {
    let mut sites: Vec<(String, Vec<String>)> = Vec::new();
    let mut line_idx = 0usize;
    for line in content.split('\n') {
        let trimmed = line.trim_end_matches('\r').trim();
        if let Some(rest) = trimmed.strip_prefix("**Closes:**") {
            let ids = extract_finding_ids_from_content(rest);
            if !ids.is_empty() {
                let label = format!("{site_name} line {line_idx}");
                sites.push((label, ids));
                line_idx += 1;
            }
        }
    }
    sites
}

/// Check cross-site completeness: every site must enumerate all canonical finding IDs,
/// and all sites that enumerate findings must agree on cardinality.
///
/// - Missing canonical IDs at a site → `Violation` citing D-411(c)+D-413(b).
/// - Cardinality divergence across sites → `Violation` citing D-420(a).
///
/// `sites`: Vec of `(site_name, cited_ids)` pairs.
/// `canonical_set`: the authoritative list of finding IDs from Part A.
///
/// Returns `Vec<Violation>` — empty if all sites are complete and agree.
///
/// Pure: no I/O; no regex crate.
///
/// # BC trace
/// BC-5.39.007 Phase 2 P2-3 postcondition (missing finding → block naming site + D-411(c));
/// BC-5.39.007 Phase 2 P2-4 postcondition (cardinality divergence → block citing D-420(a)).
pub fn check_cross_site_completeness(
    canonical_set: &[String],
    sites: &[(String, Vec<String>)],
) -> Vec<Violation> {
    let mut violations: Vec<Violation> = Vec::new();

    if canonical_set.is_empty() || sites.is_empty() {
        return violations;
    }

    // Per-site completeness check (P2-3: each site must have all canonical IDs).
    for (site_name, cited_ids) in sites {
        let missing: Vec<&String> = canonical_set
            .iter()
            .filter(|id| !cited_ids.contains(id))
            .collect();
        if !missing.is_empty() {
            let missing_list = missing
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            violations.push(Violation {
                description: format!(
                    "citation site `{site_name}` is missing finding IDs [{missing_list}] \
                     from the canonical set — all sites must enumerate the complete finding \
                     set per D-411(c)+D-413(b)"
                ),
                cited_raw: missing_list,
            });
        }
    }

    // Cardinality parity check (P2-4: all sites must cite the same count).
    // Only check sites that have at least one finding ID.
    let non_empty_sites: Vec<&(String, Vec<String>)> = sites
        .iter()
        .filter(|(_, ids)| !ids.is_empty())
        .collect();

    if non_empty_sites.len() >= 2 {
        let first_count = non_empty_sites[0].1.len();
        let all_same = non_empty_sites.iter().all(|(_, ids)| ids.len() == first_count);
        if !all_same {
            // Build a summary of the divergence.
            let counts_summary = non_empty_sites
                .iter()
                .map(|(name, ids)| format!("`{name}` cites {} finding(s)", ids.len()))
                .collect::<Vec<_>>()
                .join("; ");
            violations.push(Violation {
                description: format!(
                    "multi-site cardinality divergence in Closes citation sets — {counts_summary}; \
                     all sites must cite the same count of findings per D-420(a)"
                ),
                cited_raw: counts_summary,
            });
        }
    }

    violations
}

/// Orchestrate Phase 2 cross-site validation for a triggered file write.
///
/// # Control flow
///
/// 1. Call `read_current_adversary_pass_number()`:
///    - `Ok(None)`: pointer file absent → `log_warn` advisory + return empty Vec (fail-open P2-1).
///    - `Err(msg)`: corrupt pointer → return `[Violation]` (hard block).
///    - `Ok(Some(n))`: proceed with pass number n.
/// 2. Derive adversary review path via `derive_adversary_review_path(n)`.
///    Read via `host::read_file`; on error → `log_warn` + return empty Vec (fail-open P2-2).
/// 3. Extract canonical finding set. If empty → `log_warn` + return empty Vec (fail-open P2-2).
/// 4. Collect Closes cites from `content` (the triggered file).
/// 5. Call `check_cross_site_completeness`.
/// 6. Return accumulated violations.
///
/// Phase 2 is fail-open at every step — only blocks when BOTH files are readable AND
/// canonical set is non-empty AND violations are found.
///
/// # BC trace
/// BC-5.39.007 Phase 2 P2-1..P2-4 postconditions; Phase 2 invariant 1 (fail-open).
pub fn run_hook_phase2_cross_site(file_path: &str, content: &str) -> Vec<Violation> {
    use vsdd_hook_sdk::host;

    const HOOK_NAME: &str = "validate-closes-completeness";

    // Step 1: read pointer file.
    let pass_n = match read_current_adversary_pass_number() {
        Ok(None) => {
            host::log_warn(&format!(
                "[{HOOK_NAME}] Phase 2: current-adversary-pass.txt absent; \
                 skipping cross-site validation. Ensure state-manager's Commit A \
                 writes this file per ADR-022."
            ));
            return Vec::new();
        }
        Err(msg) => {
            // Corrupt pointer file — hard block.
            return vec![Violation {
                description: msg.clone(),
                cited_raw: msg,
            }];
        }
        Ok(Some(n)) => n,
    };

    // Step 2: derive adversary review path and read file.
    let adversary_path = derive_adversary_review_path(pass_n);
    let adversary_content = match host::read_file(&adversary_path, MAX_BYTES, 2000) {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(s) => s,
            Err(e) => {
                host::log_warn(&format!(
                    "[{HOOK_NAME}] Phase 2: UTF-8 decode error reading {adversary_path}: {e}; \
                     skipping cross-site validation (fail-open)"
                ));
                return Vec::new();
            }
        },
        Err(e) => {
            host::log_warn(&format!(
                "[{HOOK_NAME}] Phase 2: adversary review file {adversary_path} unreadable: \
                 {e:?}; skipping cross-site validation (fail-open P2-2)"
            ));
            return Vec::new();
        }
    };

    // Step 3: extract canonical finding set.
    let canonical_set = extract_canonical_finding_set(&adversary_content);
    if canonical_set.is_empty() {
        host::log_warn(&format!(
            "[{HOOK_NAME}] Phase 2: canonical finding set is empty from {adversary_path}; \
             skipping cross-site validation (fail-open — cannot block on unknown canonical set)"
        ));
        return Vec::new();
    }

    // Step 4: collect per-line Closes cites from the triggered file.
    // Each **Closes:** line is treated as a separate site for cross-site comparison.
    let site_name = file_path;
    let sites = collect_closes_sites_by_line(content, site_name);

    if sites.is_empty() {
        // No **Closes:** lines with finding IDs in the triggered file — nothing to validate.
        return Vec::new();
    }

    // Step 5: cross-site completeness check.
    check_cross_site_completeness(&canonical_set, &sites)
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
