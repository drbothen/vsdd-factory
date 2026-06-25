//! validate-burst-log — PostToolUse WASM hook plugin.
//!
//! Fires on two triggers (merged into one plugin entry per ADR-029):
//!
//! **Trigger A — PostToolUse Edit|Write (existing)**: blocks any write to a
//! `burst-log.md` file that leaves a structurally incomplete latest burst entry.
//! Validates three structural properties:
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
//! **Trigger B — PostToolUse Bash (ADR-029 §Decision 1)**: on Bash git-commit
//! events, reads `git_context` from `payload.extra` (injected by the dispatcher
//! host layer) and runs the MULTI_COMMIT_CHAIN_NOT_ALLOWED detector. This is
//! exec-free (BC-5.41.003 PC1 + ADR-029 §Decision 3): no `host::exec_subprocess`
//! call is made for commit context acquisition. Fail-open: if `git_context` is
//! absent or all-empty, the check is skipped.
//!
//! # Behavioral Contracts
//!
//! - BC-5.39.004: blocks structurally incomplete burst-log entries.
//! - BC-5.41.003: PreCompact flush exemption + MULTI_COMMIT_CHAIN_NOT_ALLOWED detector.
//! - BC-1.16.001: git_context 4-field injection contract (dispatcher side).
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
//! - HOST_ABI_VERSION = 1 (no new host functions introduced; git_context rides extra map).
//! - Fail-open on every `host::read_file` error (BC-5.39.004 invariant 5).
//! - Fail-open on absent/empty git_context (BC-5.41.003 Invariant 5; BC-1.16.001 INV3).
//! - No `println!` — use `host::log_*` for all diagnostic output.
//! - No `unwrap()` or `expect()` in production paths.
//! - No `regex` crate: hand-rolled pattern scanning to stay within WASM fuel budget.
//! - No `exec_subprocess` for commit-context acquisition (ADR-029 §Decision 3; BC-5.41.003 PC1).
//! - File-path enforcement via in-plugin guard (Q5/Q6 canonical pattern);
//!   registry entry does NOT include a `file_pattern` field.

use vsdd_hook_sdk::{HookPayload, HookResult};

/// HOST_ABI_VERSION declares the ABI contract version this plugin was built
/// against. Must remain 1.
pub const HOST_ABI_VERSION: u32 = 1;

/// Maximum bytes to read from burst-log.md via `host::read_file`.
///
/// Set to 512 KiB (524288 bytes) — consistent with the cap used by
/// `validate-state-structure` (F-P5-002) and `validate-dispatch-advance`.
/// Burst-log.md grows with every adversarial pass (each entry ~2–5 KiB) and
/// will exceed 64 KiB in long cycles. The old raw literal 65536 was a
/// hardcoded magic number with no named constant, making future audits error-prone.
///
/// F-PASS15-002: introduce named constant and raise from 65536 to 524288.
pub const MAX_BYTES: u32 = 524_288;

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
    // Guard against multi-byte UTF-8 codepoints (em-dash, en-dash, NBSP, etc.)
    // immediately preceding '(': last_paren - 1 may fall inside a multi-byte
    // sequence, causing a panic at the slice boundary. If the byte before '('
    // is not a char boundary, it cannot be an ASCII space — reject immediately.
    if last_paren > 0 {
        if !after_prefix.is_char_boundary(last_paren - 1) {
            // Non-ASCII byte immediately before '(' — not the canonical " " separator.
            return false;
        }
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
    // End-of-line anchor: nothing may follow the closing ')'.
    // Spec pattern has '$' — `inside` must be exactly "YYYY-MM-DD)".
    if inside.len() != 11 {
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
/// Distinguishes three states:
///
/// 1. **Dim-1 block absent** → returns `None`. The `check_block_presence` gate handles
///    this case upstream; no additional violation from this function.
///
/// 2. **Dim-1 block present but headline integer absent** → returns
///    `Some((0, list_count, raw_dim1_line))`. A zero headline count against any positive
///    list count is a structural violation — the author omitted the required integer.
///    This distinguishes the "missing integer" case from the "block absent" case so it
///    is never silently treated as a pass.
///
/// 3. **Dim-1 block present + headline integer present**:
///    - Counts match → returns `None` (no violation).
///    - Counts differ → returns `Some((headline_count, list_count, raw_dim1_line))`.
///
/// Handles list prefixes: `- `, `* `, and `N. ` (numbered lists).
///
/// # BC trace
/// BC-5.39.004 postcondition 4 — Dim-1 mismatch emits BlockWithFix naming both counts.
/// F-S15.11-LOCAL-P2-004 — block-present-with-unparseable-headline must not silently pass.
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

    // State 1: Dim-1 block absent — return None (upstream check_block_presence handles this).
    let dim1_idx = dim1_line_idx?;
    let dim1_line = lines[dim1_idx];

    // Extract the headline integer: find `(\d+) unique files` on the Dim-1 header line
    // or the immediately following line.
    let headline_count_opt = extract_dim1_headline_count(dim1_line).or_else(|| {
        lines
            .get(dim1_idx + 1)
            .and_then(|l| extract_dim1_headline_count(l))
    });

    // Count list items in the Dim-1 block body (lines between Dim-1 header and
    // the next bold-header or end-of-burst).
    let list_count = count_dim1_list_items(&lines, dim1_idx + 1);

    match headline_count_opt {
        // State 2: Dim-1 block present but headline integer is absent.
        // Surface as violation: headline_count=0 vs actual list_count.
        // This prevents silent pass-through of malformed Dim-1 headers.
        None => Some((0, list_count, dim1_line.trim_end().to_string())),
        // State 3: headline integer present.
        Some(headline_count) => {
            if headline_count != list_count {
                // State 3b: mismatch violation.
                Some((headline_count, list_count, dim1_line.trim_end().to_string()))
            } else {
                // State 3a: counts match — no violation.
                None
            }
        }
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
// PreCompact flush exemption (BC-5.41.003 + S-18.04b)
// ---------------------------------------------------------------------------

/// The exact commit subject prefix produced by `precompact-flush.sh`.
///
/// Matches the value of `COMMIT_PREFIX` in `crates/hook-plugins/precompact-flush/src/lib.rs`.
/// Case-sensitive; no substring match; no regex. BC-5.41.003 invariant 3.
pub const PRECOMPACT_FLUSH_PREFIX: &str = "PreCompact flush ";

/// Determine whether a commit is exempt from the MULTI_COMMIT_CHAIN_NOT_ALLOWED
/// detector under the PreCompact flush exemption (BC-5.41.003 PC1 three-case logic).
///
/// Three-case logic (BC-5.41.003 PC1):
///
/// - Case (a): log exists, last line FIELD-4 == `commit`, SHA matches FIELD-2 → exempt.
/// - Case (b): log exists but last line FIELD-4 absent/empty/non-`commit` → treat
///   as stale/corrupted; fall through to case (c).
/// - Case (c): log absent or last line empty → prefix-match alone is sufficient.
///
/// SHA-mismatch with valid FIELD-4=`commit` is NOT exempt (BC-5.41.003 INV1).
///
/// # Parameters
/// - `commit_subject`: the raw commit subject (first line of `git log --format=%s`).
/// - `commit_sha`: the SHA of the commit being evaluated.
/// - `flush_log_content`: `Some(last_line_of_log)` when the log file exists and has
///   at least one line; `None` when the log is absent or empty.
///
/// # Returns
/// `true` if the commit is exempt from chain detection; `false` otherwise.
///
/// # BC trace
/// BC-5.41.003 PC1 cases (a)/(b)/(c); INV1; AC-001/AC-002/AC-003/AC-004; AC-005; AC-008.
///
/// # Self-check (BC-5.38.005 invariant 1)
/// "If I include this real implementation, will the test for this function pass trivially
/// without any implementer work?" — YES: the 3-case logic is non-trivial (branching,
/// field parsing, SHA comparison). Body is `todo!()` per BC-5.38.001.
pub fn is_precompact_flush_exempt(
    commit_subject: &str,
    commit_sha: &str,
    flush_log_content: Option<&str>,
) -> bool {
    // Step 0: prefix match (case-sensitive, exact). BC-5.41.003 INV3.
    if !commit_subject.starts_with(PRECOMPACT_FLUSH_PREFIX) {
        return false;
    }

    // Step 1: examine log content.
    match flush_log_content {
        None => {
            // Case (c): log absent → prefix-match alone is sufficient. AC-002.
            true
        }
        Some(last_line) => {
            // Parse the 4-field log line: <ISO-timestamp> <SHA> <cycle>/<step> <type>
            // Fields are space-separated; we need FIELD-2 (SHA) and FIELD-4 (type token).
            let mut fields = last_line.split_whitespace();
            let _field1 = fields.next(); // ISO-timestamp
            let field2_sha = fields.next(); // SHA
            let _field3 = fields.next(); // cycle/step
            let field4_type = fields.next(); // type token ("commit" or absent/corrupt)

            match field4_type {
                Some("commit") => {
                    // Case (a): log valid, FIELD-4=commit. SHA must match. AC-001/AC-004.
                    match field2_sha {
                        Some(log_sha) => log_sha == commit_sha,
                        None => {
                            // FIELD-4=commit but FIELD-2 absent — treat as corrupted.
                            // Case (b) → case (c): prefix-match-only. AC-003.
                            true
                        }
                    }
                }
                _ => {
                    // Case (b): FIELD-4 absent, empty, or non-"commit" → treat as stale.
                    // Fall through to case (c): prefix-match-only exemption. AC-003.
                    true
                }
            }
        }
    }
}

/// Check whether the HEAD and HEAD^ commit subjects form a `MULTI_COMMIT_CHAIN_NOT_ALLOWED`
/// pattern (TD-VSDD-053), with the PreCompact flush exemption applied (BC-5.41.003).
///
/// A chain violation is triggered when BOTH of the following hold:
/// 1. `head_subject` contains a sentinel word (`backfill`, `Stage 1`, `Stage 2`).
/// 2. `head_parent_subject` contains a sentinel word.
///
/// The exemption: if either commit is exempt under `is_precompact_flush_exempt`, the
/// chain comparison is skipped (no violation). The exemption is checked symmetrically for
/// both HEAD and HEAD^ (BC-5.41.003 INV1 — both hooks must implement identically).
///
/// # Parameters
/// - `head_subject`: commit subject of HEAD.
/// - `head_sha`: SHA of HEAD.
/// - `head_parent_subject`: commit subject of HEAD^.
/// - `head_parent_sha`: SHA of HEAD^.
/// - `flush_log_content`: last line of `.factory/hooks/precompact-flush-log`, or `None`.
///
/// # Returns
/// `Some(Violation)` with `MULTI_COMMIT_CHAIN_NOT_ALLOWED` message if a violation is
/// detected; `None` if no violation (or exemption applies).
///
/// # BC trace
/// BC-5.41.003; BC-5.39.004 INV4; TD-VSDD-053.
///
/// # Self-check (BC-5.38.005 invariant 1)
/// "If I include this real implementation, will the test for this function pass trivially
/// without any implementer work?" — YES: non-trivial branching + sentinel scanning +
/// exemption delegation. Body is `todo!()` per BC-5.38.001.
pub fn check_multi_commit_chain(
    head_subject: &str,
    head_sha: &str,
    head_parent_subject: &str,
    head_parent_sha: &str,
    flush_log_content: Option<&str>,
) -> Option<Violation> {
    // If either commit is exempt under the PreCompact flush exemption,
    // skip the chain comparison entirely. BC-5.41.003 PC1.
    if is_precompact_flush_exempt(head_subject, head_sha, flush_log_content) {
        return None;
    }
    if is_precompact_flush_exempt(head_parent_subject, head_parent_sha, flush_log_content) {
        return None;
    }

    // TD-VSDD-053 chain detector: MULTI_COMMIT_CHAIN_NOT_ALLOWED fires when both
    // HEAD and HEAD^ contain a sentinel word.
    let head_has_sentinel = contains_sentinel(head_subject);
    let parent_has_sentinel = contains_sentinel(head_parent_subject);

    if head_has_sentinel && parent_has_sentinel {
        Some(Violation {
            description: format!(
                "MULTI_COMMIT_CHAIN_NOT_ALLOWED — HEAD and HEAD^ both contain chain-sentinel words; \
                 HEAD: {:?}; HEAD^: {:?}; TD-VSDD-053",
                head_subject, head_parent_subject
            ),
            cited_raw: format!("HEAD={head_subject:?}; HEAD^={head_parent_subject:?}"),
        })
    } else {
        None
    }
}

/// Returns `true` if `subject` contains any TD-VSDD-053 sentinel word.
///
/// Sentinels: "backfill", "Stage 1", "Stage 2" (case-insensitive for "backfill";
/// case-sensitive for "Stage N" per existing convention).
///
/// # BC trace
/// TD-VSDD-053 (single-commit-per-burst chain detector).
fn contains_sentinel(subject: &str) -> bool {
    let lower = subject.to_lowercase();
    lower.contains("backfill") || subject.contains("Stage 1") || subject.contains("Stage 2")
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
// File-path guard
// ---------------------------------------------------------------------------

/// Returns `true` if `file_path` names a file whose `file_name` component is
/// exactly `burst-log.md`.
///
/// Uses path-component-strict matching (`Path::file_name`) rather than
/// `ends_with`, preventing false-positive fires on paths like
/// `/some/dir/xburst-log.md` where `.ends_with("burst-log.md")` is also true.
///
/// Returns `false` if the path has no file-name component (e.g. `/`).
///
/// # BC trace
/// BC-5.39.004 invariant 1 — hook only activates on burst-log.md writes.
pub fn is_burst_log_target(file_path: &str) -> bool {
    std::path::Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        == Some("burst-log.md")
}

// ---------------------------------------------------------------------------
// Hook entry point
// ---------------------------------------------------------------------------

/// Core hook logic for validate-burst-log.
///
/// Called from the WASI entry point in `main.rs` via the SDK trampoline.
///
/// **Dispatch path A — PostToolUse Edit|Write (burst-log.md validation)**:
/// The dispatcher routes PostToolUse `Edit|Write` events to this hook. The
/// in-plugin `burst-log.md` file-name guard filters to the relevant file.
/// Validates h2 heading format, 9-block presence, Dim-1 cardinality.
///
/// **Dispatch path B — PostToolUse Bash (ADR-029 git_context chain detection)**:
/// When `tool_name == "Bash"` and `tool_input.command` contains "git commit",
/// the dispatcher has injected `git_context` into `payload.extra` (per ADR-029
/// §Decision 1+3). This path reads `git_context` from `payload.extra` and runs
/// `check_multi_commit_chain`. Fail-open: if `git_context` is absent or all
/// fields are empty, returns Continue without blocking (BC-1.16.001 INV3).
///
/// The key invariant: NO `host::exec_subprocess` is called for commit-context
/// acquisition. ADR-029 §Decision 3; BC-5.41.003 PC1 addendum.
///
/// # BC trace
/// BC-5.39.004 postconditions 1-6; invariants 1-5.
/// BC-5.41.003 PC1+PC2+PC3; BC-1.16.001 PC1+INV3; ADR-029 §Decision 1+3+5.
pub fn on_post_tool_use(payload: HookPayload) -> HookResult {
    use vsdd_hook_sdk::host;

    // ADR-029 §Decision 1: chain detection fires on PostToolUse Bash git-commit events.
    // Burst-log structural validation fires on PostToolUse Edit|Write events.
    // These are now distinct dispatch paths.
    if payload.tool_name == "Bash" {
        // Dispatch path B: Bash git-commit → chain detection via git_context.
        return check_chain_from_git_context(&payload);
    }

    // Dispatch path A: Edit|Write → burst-log structural validation.

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
    // only act on writes to burst-log.md files. Uses path-component-strict
    // matching to avoid false-positives from suffix-only ends_with (e.g. a
    // path "xburst-log.md" would incorrectly trigger a bare ends_with guard).
    if !is_burst_log_target(&file_path) {
        return HookResult::Continue;
    }

    // Read the burst-log.md content that was just written.
    // Use the file_path from the envelope directly — it is the canonical path
    // to the file that was just written.
    // On read failure: fail-open (Continue + log_warn) per BC-5.39.004 postcondition 6.
    let content = match host::read_file(&file_path, MAX_BYTES, 2000) {
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

/// Dispatch path B — PostToolUse Bash (ADR-029 §Decision 1).
///
/// Reads `git_context` from `payload.extra` (injected by the dispatcher host layer)
/// and runs the MULTI_COMMIT_CHAIN_NOT_ALLOWED detector via `check_multi_commit_chain`.
///
/// Fail-open semantics (BC-1.16.001 INV3; BC-5.41.003 Invariant 5):
/// - `git_context` absent from `payload.extra` → Continue (skip check).
/// - `git_context` present but all four fields empty → Continue (skip check).
/// - `head_parent_subject` is empty → Continue (initial commit; no chain possible).
///
/// The precompact-flush-log is read via `host::read_file` for SHA corroboration
/// (BC-5.41.003 PC1 three-case logic). No `exec_subprocess` is called.
///
/// # BC trace
/// BC-5.41.003 PC1 addendum (ADR-029 wiring); BC-1.16.001 PC1+INV3;
/// ADR-029 §Decision 1+3+5.
fn check_chain_from_git_context(payload: &HookPayload) -> HookResult {
    use vsdd_hook_sdk::host;

    // CR-002: Short-circuit for non-git-commit Bash events per ADR-029 §Decision 1.
    // The dispatcher only injects git_context on qualifying git-commit events, but adding
    // an explicit WASM-level filter avoids unnecessary processing of all other Bash events.
    let command = payload
        .tool_input
        .get("command")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if !command.contains("git") || !command.contains("commit") {
        return HookResult::Continue;
    }

    // Step 1: Extract git_context from payload.extra.
    // Fail-open (Continue) if absent.
    let git_context = match payload.extra.get("git_context") {
        Some(v) => v,
        None => {
            // git_context absent — dispatcher fail-open path or non-qualifying event.
            // BC-1.16.001 INV3: skip chain check, return Continue.
            return HookResult::Continue;
        }
    };

    // Step 2: Extract the 4 required fields.
    let head_subject = git_context
        .get("head_subject")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    let head_sha = git_context
        .get("head_sha")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    let head_parent_subject = git_context
        .get("head_parent_subject")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    let head_parent_sha = git_context
        .get("head_parent_sha")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string();

    // Step 3: Fail-open if all fields empty (dispatcher fail-open path per BC-1.16.001 PC2).
    // SEC-004: emit telemetry warn so silent bypasses are observable.
    if head_subject.is_empty() && head_parent_subject.is_empty() {
        host::log_warn(
            "[validate-burst-log] git_context all-empty — chain check skipped (fail-open per BC-1.16.001 INV3)",
        );
        return HookResult::Continue;
    }

    // Step 4: Fail-open if head_parent_subject is empty (initial commit; no chain possible).
    // BC-5.41.003 Invariant 5 (ADR-029 wiring extension).
    if head_parent_subject.is_empty() {
        return HookResult::Continue;
    }

    // Step 5: Read precompact-flush-log for SHA corroboration.
    // BC-5.41.003 PC1 three-case logic: read via host::read_file (NOT exec_subprocess).
    let flush_log_last_line: Option<String> = {
        let cwd = host::cwd();
        if !cwd.is_empty() {
            let flush_log_path = format!("{cwd}/.factory/hooks/precompact-flush-log");
            match host::read_file(&flush_log_path, 4096, 500) {
                Ok(bytes) => String::from_utf8(bytes).ok().and_then(|s| {
                    s.lines()
                        .rev()
                        .find(|l| !l.trim().is_empty())
                        .map(|l| l.to_string())
                }),
                Err(_) => None,
            }
        } else {
            None
        }
    };

    // Step 6: Run check_multi_commit_chain (pure logic; no I/O).
    match check_multi_commit_chain(
        &head_subject,
        &head_sha,
        &head_parent_subject,
        &head_parent_sha,
        flush_log_last_line.as_deref(),
    ) {
        Some(violation) => emit_block(&[violation]),
        None => HookResult::Continue,
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

    #[test]
    fn test_BC_5_39_004_h2_trailing_content_after_close_paren_returns_false() {
        // F-S15.11-LOCAL-P1-001: trailing content after closing ')' must be rejected.
        // The canonical pattern has a '$' anchor — nothing may follow the ')'.
        // Trailing spaces/CR are normalized by the function before validation (matching
        // how file content is read from disk), so only non-whitespace trailing content
        // constitutes an anchor violation.
        assert!(!validate_h2_heading("## Burst: foo (2026-05-12)abc"));
        assert!(!validate_h2_heading(
            "## Burst: foo (2026-05-12)xyz trailing"
        ));
        // Trailing whitespace is normalized away (CR/LF trim) — not a violation.
        // Canonical positive case still passes.
        assert!(validate_h2_heading("## Burst: foo (2026-05-12)"));
        assert!(validate_h2_heading("## Burst: foo (2026-05-12) ")); // trimmed = canonical
    }

    #[test]
    fn test_BC_5_39_004_h2_with_emdash_before_paren_returns_false_no_panic() {
        // Em-dash U+2014 (3 bytes UTF-8) immediately before '('
        // BEFORE FIX: panicked at byte-index slice (last_paren - 1 inside multi-byte codepoint)
        // AFTER FIX: is_char_boundary guard returns false cleanly without panic
        assert!(!validate_h2_heading("## Burst: foo\u{2014}(2026-05-12)"));
    }

    #[test]
    fn test_BC_5_39_004_h2_with_endash_before_paren_returns_false_no_panic() {
        // En-dash U+2013 (3 bytes UTF-8) immediately before '('
        assert!(!validate_h2_heading("## Burst: foo\u{2013}(2026-05-12)"));
    }

    #[test]
    fn test_BC_5_39_004_h2_with_nbsp_before_paren_returns_false_no_panic() {
        // Non-breaking space U+00A0 (2 bytes UTF-8) immediately before '('
        assert!(!validate_h2_heading("## Burst: foo\u{00A0}(2026-05-12)"));
    }

    #[test]
    fn test_BC_5_39_004_h2_with_canonical_ascii_space_before_paren_returns_true() {
        // Control: canonical ASCII space before '(' must still PASS
        assert!(validate_h2_heading("## Burst: foo (2026-05-12)"));
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

    // ── is_burst_log_target ──────────────────────────────────────────────────

    #[test]
    fn test_BC_5_39_004_file_path_xburst_log_md_does_not_match() {
        // O-P1-002: suffix-only match is a false-positive — xburst-log.md
        // must NOT trigger the hook.
        assert!(!is_burst_log_target("/some/dir/xburst-log.md"));
        assert!(!is_burst_log_target("not-burst-log.md"));
        assert!(!is_burst_log_target("aburst-log.md"));
    }

    #[test]
    fn test_BC_5_39_004_file_path_canonical_burst_log_md_matches() {
        // Canonical path at any depth must match.
        assert!(is_burst_log_target(
            ".factory/cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md"
        ));
        assert!(is_burst_log_target("burst-log.md"));
        assert!(is_burst_log_target("/absolute/path/to/burst-log.md"));
    }

    // ── check_dim1_cardinality: unparseable headline ─────────────────────────

    #[test]
    fn test_BC_5_39_004_dim1_block_present_without_integer_headline_emits_violation() {
        // F-S15.11-LOCAL-P2-004: Dim-1 block present but no integer in headline
        // must surface as a violation (not silently pass).
        let content = concat!(
            "## Burst: Pass-44 test (2026-05-16)\n",
            "**Files touched (Dim-1):**\n",
            "\n",
            "- a.rs\n",
            "- b.rs\n",
            "**Codifications:** done\n",
        );
        // When Dim-1 block exists but headline has no integer, check_dim1_cardinality
        // must return Some(...) to surface a violation (not None which silently passes).
        let result = check_dim1_cardinality(content);
        assert!(
            result.is_some(),
            "Dim-1 block present with no headline integer must surface as a violation, got None"
        );
        let (headline_count, list_count, _raw) = result.unwrap();
        assert_eq!(
            headline_count, 0,
            "headline count must be 0 when integer is absent"
        );
        assert_eq!(list_count, 2, "list count must reflect actual items");
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

    // ── F-PASS15-002: MAX_BYTES cap regression test ──────────────────────────

    /// F-PASS15-002: verifies that MAX_BYTES is set to at least 524288 (512 KiB),
    /// consistent with the sibling cap used by validate-state-structure (F-P5-002)
    /// and validate-dispatch-advance.
    ///
    /// Burst-log.md grows with every adversarial pass. If someone lowers MAX_BYTES
    /// below 512 KiB, this compile-time assertion fails, preventing the silent
    /// regression where burst-log validation becomes functionally dead against
    /// production-sized files in long cycles.
    #[test]
    fn test_BC_5_39_004_max_bytes_cap_at_least_512_kib() {
        // Compile-time assertion: MAX_BYTES must be >= 524288 (512 KiB).
        // This is the load-bearing constant check that closes F-PASS15-002.
        // If someone lowers the cap below 512 KiB, this line fails to compile.
        const _: () = assert!(
            MAX_BYTES >= 524_288,
            "MAX_BYTES must be >= 524288 (512 KiB) per F-PASS15-002"
        );
    }
}
