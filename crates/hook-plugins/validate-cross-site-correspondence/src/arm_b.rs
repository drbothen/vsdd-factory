//! arm_b.rs — Class B: STORY-INDEX.md three-way input-hash correspondence.
//!
//! Effectful-shell module (ADR-035 §Decision 1): reads STORY-INDEX.md via
//! `host::read_file` when processing story file writes (Arm B1).
//!
//! # Two sub-arms
//!
//! ## B1 — story file write trigger
//! Fires when a story file is written. Extracts B1 (story frontmatter `input-hash:`),
//! reads STORY-INDEX.md to extract B2 (catalog row hash) and B3 (blockquote hash).
//! Blocks if any pair disagrees. Advisory if B2/B3 are absent (new story, not yet
//! registered in STORY-INDEX.md).
//!
//! ## B2 — STORY-INDEX.md write trigger
//! Fires when STORY-INDEX.md is written. Parses all story IDs and hash tokens from
//! the catalog rows and aggregation blockquote within the file itself. Blocks if any
//! catalog hash disagrees with the corresponding blockquote hash (cascade).
//!
//! # Block message invariant (BC-5.39.010 invariant 11)
//! The B1 block message MUST include a provenance note distinguishing:
//! - `stale` — B2/B3 likely just need a `--update` sweep
//! - `fabricated` — B1 disagrees with B3 but agrees with B2, suggesting B1 was
//!   manually edited without updating the story file
//!
//! # BC trace
//! BC-5.39.010 preconditions 16-25; postconditions 12-15; invariant 11.

use crate::{Advisory, Violation};
use vsdd_hook_sdk::host::HostError;

/// Read cap for STORY-INDEX.md secondary read (B1 story write trigger).
/// BC-5.39.010 AC-019: `max_bytes = 1048576`, `timeout_ms = 3000`.
pub const STORY_INDEX_B1_MAX_BYTES: u32 = 1_048_576;
pub const STORY_INDEX_B1_TIMEOUT_MS: u32 = 3_000;

/// Read cap for STORY-INDEX.md when it is the primary target (B2 write trigger).
/// BC-5.39.010 AC-019: `max_bytes = 2097152`, `timeout_ms = 5000`.
pub const STORY_INDEX_B2_MAX_BYTES: u32 = 2_097_152;
pub const STORY_INDEX_B2_TIMEOUT_MS: u32 = 5_000;

/// Extract the `input-hash:` value from a story file's frontmatter.
///
/// Returns `None` if the field is absent (Arm B1 skips entirely per
/// BC-5.39.010 precondition 18 second clause).
///
/// Pure: no I/O.
///
/// # BC trace
/// BC-5.39.010 precondition 18 (input-hash: field presence check).
pub fn parse_story_input_hash(story_content: &str) -> Option<String> {
    crate::frontmatter::extract_frontmatter_field(story_content, "input-hash")
}

/// Extract the input-hash from the STORY-INDEX.md catalog row for `story_id`.
///
/// Scans the catalog table in STORY-INDEX.md for a row containing `story_id`
/// and extracts the `input-hash <hash>` token from that row.
/// Returns `None` if no row is found for this story ID (new story not yet indexed).
///
/// Pure: operates on already-read bytes.
///
/// # BC trace
/// BC-5.39.010 precondition 19 (B2 catalog row extraction).
pub fn parse_story_index_catalog_hash(index_content: &[u8], story_id: &str) -> Option<String> {
    // F-S2107-P1B-008: naive `contains(story_id)` matches rows WHERE story_id appears
    // in depends_on/blocks columns (e.g. S-18.00's row contains "[S-18.01]"), returning
    // the wrong row's hash. PC16: catalog lookup must anchor on the FIRST pipe-delimited
    // cell — the row whose first cell is exactly the story_id.
    let content = std::str::from_utf8(index_content).ok()?;
    for line in content.lines() {
        // Must be a table row (starts with |)
        if !line.starts_with('|') {
            continue;
        }
        // First cell must be exactly the story_id (trim surrounding whitespace)
        let mut cells = line.split('|');
        cells.next(); // skip leading empty segment before first '|'
        let first_cell = cells.next().map(|c| c.trim()).unwrap_or("");
        if first_cell != story_id {
            continue;
        }
        // Extract `input-hash <hash>` from this row
        if let Some(hash) = extract_input_hash_token(line) {
            return Some(hash);
        }
    }
    None
}

/// Extract the input-hash from the STORY-INDEX.md aggregation blockquote for `story_id`.
///
/// Scans the aggregation blockquote (`> S-21.07=47a65c9`) in STORY-INDEX.md for
/// an entry matching `story_id`. Returns `None` if not found.
///
/// Pure: operates on already-read bytes.
///
/// # BC trace
/// BC-5.39.010 precondition 20 (B3 blockquote extraction).
pub fn parse_story_index_blockquote_hash(index_content: &[u8], story_id: &str) -> Option<String> {
    // F-S2107-P1B-003: production STORY-INDEX blockquote is ONE prose line with all hashes
    // embedded as `S-XX.YY=HHHHHHH` tokens (not one line per story). PC21 specifies a
    // WITHIN-line search for `\b<id>=([0-9a-f]{7,40})\b` on `^> ` lines.
    let content = std::str::from_utf8(index_content).ok()?;
    let needle = format!("{}=", story_id);

    for line in content.lines() {
        if !line.starts_with("> ") {
            continue;
        }
        // Search for `story_id=HHHHHHH` anywhere within the line, with word boundary before.
        let mut search_start = 0;
        while search_start < line.len() {
            let search_in = &line[search_start..];
            if let Some(rel_pos) = search_in.find(&needle) {
                let abs_pos = search_start + rel_pos;
                // Word boundary before story_id: preceding char must not be alphanumeric
                let wb_ok = abs_pos == 0 || {
                    let prev = line[..abs_pos].chars().last().unwrap_or('\0');
                    !prev.is_ascii_alphanumeric()
                };
                if wb_ok {
                    let hash_start = abs_pos + needle.len();
                    if hash_start <= line.len() {
                        // Extract hex-only token bounded to 7..=40 chars (PC21).
                        // PC21 specifies [0-9a-f]{7,40} — lowercase only. This is
                        // deliberate and spec-mandated (BC-5.39.010 PC21); do not
                        // widen to is_ascii_hexdigit().
                        let hash: String = line[hash_start..]
                            .chars()
                            .take_while(|c| matches!(c, '0'..='9' | 'a'..='f'))
                            .collect();
                        if hash.len() >= 7 && hash.len() <= 40 {
                            return Some(hash);
                        }
                    }
                }
                search_start = abs_pos + 1;
            } else {
                break;
            }
        }
    }
    None
}

/// Arm B1 check with the STORY-INDEX.md read result as a seam.
///
/// Pure-seam entry point for unit testing.
///
/// Returns `(violations, advisories)`:
/// - Block: B2 ≠ B1 (catalog disagrees with story frontmatter).
/// - Block: B3 ≠ B1 (blockquote disagrees with story frontmatter).
/// - Block: CapabilityDenied on STORY-INDEX.md read (invariant 5 — sandbox
///   misconfiguration on secondary target is blocking per BC-5.39.010 PC26).
/// - Advisory: NotFound on STORY-INDEX.md (bootstrap ordering).
/// - Advisory: B2 or B3 absent (new story not yet in STORY-INDEX.md).
///
/// # BC trace
/// BC-5.39.010 postconditions 12-13; preconditions 16-21, 26; invariant 11.
pub fn run_arm_b1_with_index_result(
    story_id: &str,
    story_hash: &str,
    index_read_result: Result<Vec<u8>, HostError>,
) -> (Vec<Violation>, Vec<Advisory>) {
    let mut violations = Vec::new();
    let mut advisories = Vec::new();

    match index_read_result {
        Err(HostError::CapabilityDenied) => {
            // BC-5.39.010 PC26: CapabilityDenied on secondary target → block
            // Invariant 5: sandbox misconfiguration is never legitimate
            violations.push(Violation {
                description: format!(
                    "validate-cross-site-correspondence [Class B] POLICY 18: \
                    STORY-INDEX.md read denied (CapabilityDenied) for story {story_id} \
                    — sandbox misconfiguration must be investigated \
                    (stale registry entry or capability not granted)"
                ),
            });
        }
        Err(HostError::NotFound) => {
            // STORY-INDEX.md not found — bootstrap ordering, advisory only
            advisories.push(Advisory {
                message: format!(
                    "validate-cross-site-correspondence [Class B] advisory: \
                    STORY-INDEX.md not found during B1 check for {story_id} \
                    — bootstrap ordering: index may not exist yet"
                ),
            });
        }
        Err(other) => {
            // Other host errors → block (fail-closed)
            violations.push(Violation {
                description: format!(
                    "validate-cross-site-correspondence [Class B] POLICY 18: \
                    STORY-INDEX.md read error ({other:?}) for story {story_id} \
                    — cannot verify hash correspondence (stale read or host fault)"
                ),
            });
        }
        Ok(ref bytes) => {
            let catalog_hash = parse_story_index_catalog_hash(bytes, story_id);
            let blockquote_hash = parse_story_index_blockquote_hash(bytes, story_id);

            match (catalog_hash, blockquote_hash) {
                (None, None) => {
                    // Story not yet registered in STORY-INDEX.md — advisory only (new story)
                    advisories.push(Advisory {
                        message: format!(
                            "validate-cross-site-correspondence [Class B] advisory: \
                            story {story_id} has input-hash in frontmatter but is not yet \
                            registered in STORY-INDEX.md — run compute-input-hash --update \
                            after indexing"
                        ),
                    });
                }
                (Some(b2), Some(b3)) => {
                    let b2_match = b2 == story_hash;
                    let b3_match = b3 == story_hash;

                    if !b2_match || !b3_match {
                        let provenance = classify_provenance(story_hash, &b2, &b3);
                        violations.push(Violation {
                            description: format!(
                                "validate-cross-site-correspondence [Class B] POLICY 18: \
                                input-hash mismatch for story {story_id} \
                                — story={story_hash} catalog={b2} blockquote={b3} \
                                — {provenance}"
                            ),
                        });
                    }
                }
                (Some(b2), None) => {
                    // Only catalog row present — no blockquote entry yet
                    if b2 != story_hash {
                        violations.push(Violation {
                            description: format!(
                                "validate-cross-site-correspondence [Class B] POLICY 18: \
                                input-hash mismatch for story {story_id} \
                                — story={story_hash} catalog={b2} blockquote=absent \
                                — stale — update both STORY-INDEX.md catalog and blockquote"
                            ),
                        });
                    } else {
                        advisories.push(Advisory {
                            message: format!(
                                "validate-cross-site-correspondence [Class B] advisory: \
                                story {story_id} catalog row present but blockquote entry absent \
                                in STORY-INDEX.md — run compute-input-hash --update"
                            ),
                        });
                    }
                }
                (None, Some(b3)) => {
                    // Only blockquote entry present — no catalog row yet
                    if b3 != story_hash {
                        violations.push(Violation {
                            description: format!(
                                "validate-cross-site-correspondence [Class B] POLICY 18: \
                                input-hash mismatch for story {story_id} \
                                — story={story_hash} catalog=absent blockquote={b3} \
                                — stale — update both STORY-INDEX.md catalog and blockquote"
                            ),
                        });
                    } else {
                        advisories.push(Advisory {
                            message: format!(
                                "validate-cross-site-correspondence [Class B] advisory: \
                                story {story_id} blockquote entry present but catalog row absent \
                                in STORY-INDEX.md — run compute-input-hash --update"
                            ),
                        });
                    }
                }
            }
        }
    }

    (violations, advisories)
}

/// Returns `true` if `path` is a volatile factory path whose content changes
/// frequently enough that a stable input-hash cannot be maintained.
///
/// Volatile paths (BC-5.39.010 v1.6 PC40):
/// - `.factory/STATE.md` — direct child only (not subdirectory STATE.md files)
/// - `.factory/**/BC-INDEX.md` — anywhere under `.factory/`
/// - `.factory/**/VP-INDEX.md` — anywhere under `.factory/`
/// - `.factory/**/STORY-INDEX.md` — anywhere under `.factory/`
/// - `.factory/cycles/**` — any file under the cycles tree
///
/// Pure: no I/O.
///
/// # BC trace
/// BC-5.39.010 v1.6 PC40: volatile-input precondition.
pub fn is_volatile_path(path: &str) -> bool {
    use std::path::{Component, Path};
    let p = Path::new(path);
    let components: Vec<_> = p.components().collect();

    let has_factory = components
        .iter()
        .any(|c| matches!(c, Component::Normal(s) if *s == ".factory"));
    if !has_factory {
        return false;
    }

    let filename = p.file_name().and_then(|f| f.to_str()).unwrap_or("");

    // .factory/STATE.md — direct child only (parent dir must be exactly ".factory")
    if filename == "STATE.md" {
        let parent_is_factory = p
            .parent()
            .and_then(|parent| parent.file_name())
            .and_then(|f| f.to_str())
            .map(|f| f == ".factory")
            .unwrap_or(false);
        if parent_is_factory {
            return true;
        }
    }

    // .factory/**/BC-INDEX.md, VP-INDEX.md, STORY-INDEX.md
    if matches!(filename, "BC-INDEX.md" | "VP-INDEX.md" | "STORY-INDEX.md") {
        return true;
    }

    // .factory/cycles/** — any file under the cycles tree
    components
        .iter()
        .any(|c| matches!(c, Component::Normal(s) if *s == "cycles"))
}

/// Extract the `inputs:` YAML sequence from a story file's frontmatter.
///
/// Returns the list of input paths declared in the frontmatter `inputs:` field.
/// Returns an empty `Vec` if the field is absent or the frontmatter is malformed.
///
/// Delegates to `crate::frontmatter::extract_frontmatter_sequence` which handles
/// both inline (`inputs: [a, b]`) and block (`inputs:\n  - a\n  - b`) YAML forms.
///
/// Pure: no I/O.
///
/// # BC trace
/// BC-5.39.010 v1.6 PC40: volatile-input precondition.
pub fn parse_story_volatile_inputs(content: &str) -> Vec<String> {
    crate::frontmatter::extract_frontmatter_sequence(content, "inputs")
}

/// Arm B1 effectful entry point.
///
/// Reads STORY-INDEX.md via `host::read_file` (`max_bytes = 1048576`,
/// `timeout_ms = 3000`), then delegates to `run_arm_b1_with_index_result`.
///
/// PC40 (BC-5.39.010 v1.6): if the story's `inputs:` list contains any volatile
/// paths (STATE.md, INDEX files, cycles/ artifacts), emits advisory + Continue
/// without performing the three-way hash comparison.
///
/// Called from `on_post_tool_use` when a story file write is detected.
///
/// # BC trace
/// BC-5.39.010 preconditions 17-21 (STORY-INDEX.md read + hash comparison).
/// BC-5.39.010 v1.6 PC40: volatile-input precondition.
pub fn run_arm_b1(story_id: &str, story_content: &str) -> (Vec<Violation>, Vec<Advisory>) {
    let story_hash = match parse_story_input_hash(story_content) {
        Some(h) => h,
        None => return (vec![], vec![]), // No input-hash: skip (PC18)
    };

    // PC40: if any declared input is volatile, skip the three-way comparison.
    // Volatile paths (STATE.md, INDEX files, cycles/**) change too frequently to
    // maintain stable hashes; emitting a block would be a false positive.
    let inputs = parse_story_volatile_inputs(story_content);
    let volatile_found: Vec<&str> = inputs
        .iter()
        .filter(|p| is_volatile_path(p))
        .map(|p| p.as_str())
        .collect();
    if !volatile_found.is_empty() {
        let advisory = Advisory {
            message: format!(
                "validate-cross-site-correspondence [Class B] advisory: story {story_id} \
                has volatile inputs {volatile_found:?} — skipping three-way input-hash \
                comparison per BC-5.39.010 v1.6 PC40. Volatile paths do not produce \
                stable hashes. Update input-hash manually when non-volatile inputs change."
            ),
        };
        return (vec![], vec![advisory]);
    }

    let index_result = vsdd_hook_sdk::host::read_file(
        ".factory/stories/STORY-INDEX.md",
        STORY_INDEX_B1_MAX_BYTES,
        STORY_INDEX_B1_TIMEOUT_MS,
    );
    run_arm_b1_with_index_result(story_id, &story_hash, index_result)
}

/// Arm B2: check all story hashes within STORY-INDEX.md itself for internal
/// catalog vs blockquote consistency.
///
/// STORY-INDEX.md IS the primary target for Arm B2: any `HostError` on its read
/// blocks (BC-5.39.010 precondition 22 note). The content has already been read
/// by `on_post_tool_use` before dispatch.
///
/// Scans all story IDs in the catalog and compares each with the corresponding
/// blockquote entry. Reports ALL mismatches in ONE combined block (cascade —
/// postcondition 15).
///
/// Pure: operates on already-read content string.
///
/// # BC trace
/// BC-5.39.010 preconditions 22-25; postcondition 15 (cascade); invariant 11.
pub fn run_arm_b2(story_index_content: &str) -> Vec<Violation> {
    let mut violations = Vec::new();

    // Build catalog map: Vec<(story_id, catalog_hash)>
    let mut catalog: Vec<(String, String)> = Vec::new();
    // Build blockquote map: Vec<(story_id, blockquote_hash)>
    let mut blockquote: Vec<(String, String)> = Vec::new();

    for line in story_index_content.lines() {
        if line.starts_with('|') {
            // Catalog table row — extract story_id (first cell) and input-hash token
            if let (Some(story_id), Some(hash)) = (
                extract_story_id_from_table_row(line),
                extract_input_hash_token(line),
            ) {
                catalog.push((story_id, hash));
            }
        } else if line.starts_with("> ") {
            // F-S2107-P1B-004: production STORY-INDEX blockquote is ONE prose line with
            // multiple `S-XX.YY=HHHHHHH` tokens. The current single-`find('=')` approach
            // grabs the whole prose prefix as story_id. Fix: extract ALL id=hash pairs.
            for (story_id, hash) in extract_blockquote_pairs(line) {
                blockquote.push((story_id, hash));
            }
        }
    }

    // Compare catalog→blockquote direction (BC-5.39.010 PC22 note: "scans all story IDs
    // in the catalog"). This direction correctly ignores blockquote entries for stories
    // not in the catalog (they belong to other wave aggregations).
    for (cat_story_id, cat_hash) in &catalog {
        let bq_hash = blockquote
            .iter()
            .find(|(id, _)| id == cat_story_id)
            .map(|(_, h)| h.as_str());

        if let Some(bq_h) = bq_hash
            && bq_h != cat_hash
        {
            violations.push(Violation {
                description: format!(
                    "validate-cross-site-correspondence [Class B] POLICY 18: \
                    STORY-INDEX.md internal parity violation for story {cat_story_id} \
                    — catalog={cat_hash} blockquote={bq_h} \
                    — run `compute-input-hash --update`"
                ),
            });
            // If they match: no violation (correct)
        }
        // If bq_hash is None (story not in blockquote): no violation — blockquote may
        // aggregate multiple waves; absence is not an error for Arm B2.
    }

    violations
}

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

/// Extract `input-hash <hex>` token from a table row or any content line.
///
/// Returns the hex hash string, or `None` if not present or not valid hex.
///
/// F-S2107-P1B-009: PC20 requires `\binput-hash\s+([0-9a-f]{7,40})\b`.
/// - `is_ascii_alphanumeric()` accepted non-hex tokens like "bonus" (has 'o', 'n', 'u', 's').
/// - Single space needle missed multi-space forms (`\s+`).
/// - No retry past a non-conforming first match (live STORY-INDEX has "convention",
///   "mismatch", "bonus", "updated" at 6+ sites).
///
/// Fix: validate hex-only, bound to {7,40}, allow `\s+`, retry past bad matches.
fn extract_input_hash_token(line: &str) -> Option<String> {
    let keyword = "input-hash";
    let mut search_start = 0;

    while search_start < line.len() {
        let search_in = &line[search_start..];
        let Some(rel_pos) = search_in.find(keyword) else {
            break;
        };
        let pos = search_start + rel_pos;

        // Word boundary before "input-hash"
        let wb_before_ok = pos == 0 || {
            let prev = line[..pos].chars().last().unwrap_or('\0');
            !prev.is_ascii_alphanumeric()
        };
        let after_keyword = pos + keyword.len();
        if !wb_before_ok || after_keyword >= line.len() {
            search_start = pos + 1;
            continue;
        }

        // Skip \s+ (at least one whitespace required — PC20)
        let mut hash_start = after_keyword;
        while hash_start < line.len()
            && (line.as_bytes()[hash_start] == b' ' || line.as_bytes()[hash_start] == b'\t')
        {
            hash_start += 1;
        }
        if hash_start == after_keyword {
            // No whitespace found — not a valid "input-hash \s+" pattern
            search_start = pos + 1;
            continue;
        }

        // Extract hex-only token bounded to {7,40} chars
        let hash: String = line[hash_start..]
            .chars()
            .take_while(|c| matches!(c, '0'..='9' | 'a'..='f'))
            .collect();

        if hash.len() >= 7 && hash.len() <= 40 {
            return Some(hash);
        }
        // Non-conforming token: retry past this occurrence
        search_start = hash_start + 1;
    }
    None
}

/// Extract the story ID from a STORY-INDEX.md table row.
///
/// Table row format: `| S-21.07 | title | ... |`
/// The story ID is in the first pipe-delimited cell.
fn extract_story_id_from_table_row(line: &str) -> Option<String> {
    // Split by | and find the first non-empty cell
    let mut cells = line.split('|');
    cells.next(); // skip leading empty (before first |)
    let first_cell = cells.next()?;
    let id = first_cell.trim().to_string();
    // A story ID starts with "S-"
    if id.starts_with("S-") { Some(id) } else { None }
}

/// Extract all `S-XX.YY=HHHHHHH` pairs from a blockquote line.
///
/// F-S2107-P1B-004: production STORY-INDEX uses ONE prose `> ` line containing all
/// story hashes as semicolon-separated `S-XX.YY=HHHHHHH` tokens. This helper extracts
/// ALL such pairs (not just the first) using a word-boundary-aware scan. Returns only
/// pairs with valid hex hashes bounded to {7,40} chars.
fn extract_blockquote_pairs(line: &str) -> Vec<(String, String)> {
    let mut pairs = Vec::new();
    // Skip the "> " prefix
    let rest = if let Some(r) = line.strip_prefix("> ") {
        r
    } else {
        return pairs;
    };

    let mut search_pos = 0;
    while search_pos < rest.len() {
        // Find next "S-" at a word boundary
        let Some(rel) = rest[search_pos..].find("S-") else {
            break;
        };
        let abs = search_pos + rel;

        // Word boundary: char before "S" must not be alphanumeric
        let wb_ok = abs == 0 || {
            let prev = rest[..abs].chars().last().unwrap_or('\0');
            !prev.is_ascii_alphanumeric()
        };
        if !wb_ok {
            search_pos = abs + 1;
            continue;
        }

        // Parse S-[0-9]+\.[0-9]+ id
        let candidate = &rest[abs..];
        let id_len = parse_story_id_len(candidate);
        if id_len == 0 {
            search_pos = abs + 1;
            continue;
        }

        // Must be followed immediately by '='
        if id_len >= candidate.len() || candidate.as_bytes()[id_len] != b'=' {
            search_pos = abs + id_len;
            continue;
        }

        let story_id = &candidate[..id_len];
        let hash_start = id_len + 1;
        // PC20/PC21 specify [0-9a-f]{7,40} — lowercase only. This is deliberate and
        // spec-mandated (BC-5.39.010 PC20/PC21); do not widen to is_ascii_hexdigit().
        let hash: String = candidate[hash_start..]
            .chars()
            .take_while(|c| matches!(c, '0'..='9' | 'a'..='f'))
            .collect();

        if hash.len() >= 7 && hash.len() <= 40 {
            pairs.push((story_id.to_string(), hash.clone()));
        }
        search_pos = abs + hash_start + hash.len().max(1);
    }
    pairs
}

/// Parse the byte-length of a `S-[0-9]+\.[0-9]+` story ID prefix at the start of `s`.
/// Returns the length of the match, or 0 if no valid story ID starts at position 0.
fn parse_story_id_len(s: &str) -> usize {
    let bytes = s.as_bytes();
    if bytes.len() < 4 || bytes[0] != b'S' || bytes[1] != b'-' {
        return 0;
    }
    let mut i = 2;
    // First digit group
    if i >= bytes.len() || !bytes[i].is_ascii_digit() {
        return 0;
    }
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    // Dot separator
    if i >= bytes.len() || bytes[i] != b'.' {
        return 0;
    }
    i += 1;
    // Second digit group
    if i >= bytes.len() || !bytes[i].is_ascii_digit() {
        return 0;
    }
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    i
}

/// Classify the provenance of a hash mismatch for invariant 11.
///
/// - stale: B2 == B3 but ≠ B1 → story updated, index needs --update sweep
/// - fabricated: B1 == B2 but ≠ B3 → story frontmatter matches catalog but not blockquote
/// - stale (generic): any other mismatch pattern
fn classify_provenance(b1: &str, b2: &str, b3: &str) -> &'static str {
    if b2 == b3 && b1 != b2 {
        // Index is internally consistent, story disagrees → index is stale
        "stale — STORY-INDEX.md needs `compute-input-hash --update` sweep"
    } else if b1 == b2 && b1 != b3 {
        // Story frontmatter matches catalog but not blockquote → fabricated B3
        "fabricated — blockquote hash disagrees with story frontmatter and catalog row"
    } else {
        // Multiple mismatches
        "stale — multiple hash mismatches; run `compute-input-hash --update`"
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // parse_story_input_hash — BC-5.39.010 precondition 18
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_5_39_010_arm_b_story_hash_extracted_from_frontmatter() {
        let content = "---\ninput-hash: \"47a65c9\"\n---\nbody\n";
        let result = parse_story_input_hash(content);
        assert_eq!(result, Some("47a65c9".to_string()));
    }

    #[test]
    fn test_BC_5_39_010_arm_b_no_input_hash_returns_none() {
        // BC-5.39.010 EC-006 / AC-010: no input-hash field → Arm B1 skips entirely
        let content = "---\nstory_id: S-21.07\n---\nbody\n";
        let result = parse_story_input_hash(content);
        assert!(result.is_none(), "absent input-hash must return None");
    }

    // -----------------------------------------------------------------------
    // run_arm_b1_with_index_result — BC-5.39.010 postconditions 12-13
    // -----------------------------------------------------------------------

    /// AC-009 MUTANT: hash mismatch blocks (BC-5.39.010 postcondition 13).
    #[test]
    fn test_BC_5_39_010_arm_b1_hash_mismatch_blocks() {
        // B1=47a65c9, INDEX has catalog row 4be9d21 → mismatch
        let index_content = b"| S-21.07 | ... | input-hash 4be9d21 | ...\n> S-21.07=4be9d21\n";
        let (violations, _) =
            run_arm_b1_with_index_result("S-21.07", "47a65c9", Ok(index_content.to_vec()));
        assert!(
            !violations.is_empty(),
            "hash mismatch must produce a blocking violation"
        );
        let msg = &violations[0].description;
        assert!(msg.contains("[Class B]"), "violation must cite [Class B]");
        assert!(msg.contains("POLICY 18"), "violation must cite POLICY 18");
        // Provenance note: stale vs fabricated (invariant 11)
        assert!(
            msg.contains("stale") || msg.contains("fabricated"),
            "violation must include provenance note (stale/fabricated)"
        );
    }

    /// AC-009 CONTROL: three-way match passes.
    #[test]
    fn test_BC_5_39_010_arm_b1_hash_match_passes() {
        let index_content = b"| S-21.07 | ... | input-hash 47a65c9 | ...\n> S-21.07=47a65c9\n";
        let (violations, _) =
            run_arm_b1_with_index_result("S-21.07", "47a65c9", Ok(index_content.to_vec()));
        assert!(violations.is_empty(), "three-way hash match must not block");
    }

    /// AC-010: absent secondary sites are advisory-only (postcondition 12).
    #[test]
    fn test_BC_5_39_010_arm_b1_absent_index_sites_advisory() {
        // STORY-INDEX.md exists but has no entry for this story yet
        let index_content = b"| S-21.06 | ... | input-hash aabbcc | ...\n> S-21.06=aabbcc\n";
        let (violations, advisories) =
            run_arm_b1_with_index_result("S-21.07", "47a65c9", Ok(index_content.to_vec()));
        assert!(
            violations.is_empty(),
            "absent secondary sites must not block"
        );
        assert!(
            !advisories.is_empty(),
            "absent secondary sites must emit advisory"
        );
    }

    /// AC-010: no input-hash field → Arm B1 skips entirely.
    #[test]
    fn test_BC_5_39_010_arm_b1_no_input_hash_skips() {
        // This is tested through run_arm_b1 (effectful) — the check is in run_arm_b1
        // before calling run_arm_b1_with_index_result. parse_story_input_hash returns
        // None → skips. Test via parse_story_input_hash which is pure.
        let content = "---\nstory_id: S-21.07\n---\nbody\n";
        let hash = parse_story_input_hash(content);
        assert!(
            hash.is_none(),
            "no input-hash field must return None (B1 skips)"
        );
    }

    /// EC-009 / BC-5.39.010 precondition 26 second clause: STORY-INDEX.md returns
    /// CapabilityDenied on story file write → BLOCK (sandbox misconfiguration is
    /// never legitimate on any secondary target per invariant 5).
    ///
    /// Note: the stub doc-comment for run_arm_b1_with_index_result labels
    /// CapabilityDenied as "Advisory" — that contradicts BC-5.39.010 PC26
    /// ("CapabilityDenied → block"). This test enforces the BC, not the stub comment.
    #[test]
    fn test_BC_5_39_010_arm_b1_story_index_capability_denied_blocks() {
        let (violations, _) =
            run_arm_b1_with_index_result("S-21.07", "47a65c9", Err(HostError::CapabilityDenied));
        assert!(
            !violations.is_empty(),
            "CapabilityDenied on STORY-INDEX.md must produce a blocking violation \
            (BC-5.39.010 PC26; invariant 5 — sandbox misconfiguration on secondary target is blocking)"
        );
    }

    /// Vacuity guard — Arm B2 with no blockquote entries must not pass vacuously.
    /// BC-5.39.010 invariant 8 (B2 cascade). An Arm B2 invocation with no
    /// blockquote entries must invoke the comparison logic (not short-circuit to
    /// "nothing to check → clean pass"). The function must execute — this test
    /// panics on the todo!() body which is the correct Red Gate failure mode.
    #[test]
    fn test_BC_5_39_010_arm_b2_no_blockquote_entries_not_vacuous() {
        // STORY-INDEX.md with catalog rows but NO blockquote (> lines)
        // Expected when implemented: 0 violations (nothing in blockquote to compare).
        // The key is that run_arm_b2 EXECUTES (not skips) — the Red Gate confirms this.
        let content = "| S-21.07 | title | input-hash 47a65c9 | W4 | P1 |\n\
            # No blockquote section here\n";
        let violations = run_arm_b2(content);
        // run_arm_b2 is todo!() → panics → test FAILS (RED Gate confirmed)
        // When implemented: 0 violations (no blockquote entries to compare against catalog)
        assert!(
            violations.is_empty(),
            "absent blockquote entries must not produce violations (nothing to compare)"
        );
    }

    // -----------------------------------------------------------------------
    // run_arm_b2 — BC-5.39.010 postcondition 15 (cascade)
    // -----------------------------------------------------------------------

    /// AC-011 MUTANT: catalog ≠ blockquote → block (cascade).
    #[test]
    fn test_BC_5_39_010_arm_b2_catalog_blockquote_mismatch_blocks() {
        let content = "| S-21.07 | ... | input-hash 47a65c9 | ...\n\
            > S-21.07=4be9d21\n";
        let violations = run_arm_b2(content);
        assert!(
            !violations.is_empty(),
            "catalog/blockquote mismatch must produce violations"
        );
        assert!(
            violations[0].description.contains("[Class B]"),
            "violation must cite [Class B]"
        );
    }

    /// AC-011 CONTROL: catalog and blockquote agree → passes.
    #[test]
    fn test_BC_5_39_010_arm_b2_catalog_blockquote_agree_passes() {
        let content = "| S-21.07 | ... | input-hash 47a65c9 | ...\n\
            > S-21.07=47a65c9\n";
        let violations = run_arm_b2(content);
        assert!(
            violations.is_empty(),
            "matching catalog/blockquote must not block"
        );
    }

    // -----------------------------------------------------------------------
    // F-S2107-P1B-003: parse_story_index_blockquote_hash uses starts_with
    // "> S-21.07=" which only matches synthetic per-story blockquote lines.
    // Production STORY-INDEX.md uses ONE prose line:
    //   > **E-21 delivery:** ... S-21.07=47a65c9. All 7 distinct.
    // This shape never starts with "> S-21.07=" → B3 is always None → advisory
    // (never blocking) even when blockquote hash mismatches → gate is inert.
    // BC-5.39.010 v1.3 §B3 invariant: B3 must be extracted from the prose line.
    // -----------------------------------------------------------------------

    /// F-S2107-P1B-003: production blockquote B3 must be extractable from prose line.
    ///
    /// RED GATE: `starts_with("> S-21.07=")` never matches production prose.
    /// parse_story_index_blockquote_hash returns None → assert_eq fails → RED gate.
    /// After fix (scan embedded `S-21.07=HHHHHHH` token in prose line):
    /// returns Some("47a65c9") → PASSES.
    #[test]
    fn test_BC_5_39_010_arm_b1_production_blockquote_b3_extracted() {
        // Production STORY-INDEX.md blockquote: ONE prose line with all story hashes
        // embedded as `S-XX.YY=HHHHHHH` tokens separated by `;`.
        let content = concat!(
            "| S-21.07 | ... | input-hash 47a65c9 |\n",
            "> **E-21 delivery:** Completed E-21 wave-4 stories.",
            " Input-hashes: S-21.01=32aaccc; S-21.02=11bbddd; S-21.07=47a65c9.",
            " All 7 distinct.\n",
        );
        let b3 = parse_story_index_blockquote_hash(content.as_bytes(), "S-21.07");
        assert_eq!(
            b3,
            Some("47a65c9".to_string()),
            "production blockquote prose line must yield B3 = '47a65c9'. \
            Red Gate: starts_with('> S-21.07=') never matches production shape → None (F-S2107-P1B-003)"
        );
    }

    // -----------------------------------------------------------------------
    // F-S2107-P1B-004: run_arm_b2 parses the blockquote by splitting each "> "
    // line on the first '='. Production prose line starts with "> **E-21 delivery:**"
    // which has an '=' in "delivery" after the '**'. No — actually the first '='
    // in the line is in the embedded `S-XX.YY=HHHHHHH` token. But `rest.find('=')`
    // from position 0 finds the FIRST '=' in the entire line-after-"> ". The story_id
    // parser takes everything before the first '=' as the story_id.
    //
    // For line "> **E-21 delivery:** ... S-21.01=32aaccc; ...",
    // after stripping "> ", rest = "**E-21 delivery:** ... S-21.01=32aaccc; ..."
    // `rest.find('=')` → position of '=' in "S-21.01=32aaccc" → rest[..pos] = garbage
    // → `garbage.starts_with("S-")` → false → orphaned entry → B2 cascade violation.
    //
    // Even if catalog rows agree, run_arm_b2 generates spurious violations for
    // every production blockquote "> " line because story_id extraction fails.
    // -----------------------------------------------------------------------

    /// F-S2107-P1B-004: production blockquote shape must not generate spurious B2 violations.
    ///
    /// RED GATE: run_arm_b2 on production-shaped STORY-INDEX with matching hashes
    /// generates "orphaned blockquote entry" violations. violations NOT empty.
    /// assert!(violations.is_empty()) FAILS → RED gate.
    /// After fix (parse embedded tokens from prose line): violations empty → PASSES.
    #[test]
    fn test_BC_5_39_010_arm_b2_production_blockquote_shape_no_spurious_violations() {
        // Production-shaped STORY-INDEX: catalog row + production prose blockquote.
        // Both agree on S-21.07=47a65c9. Expected: zero violations.
        let content = concat!(
            "| S-21.07 | validate-cross-site-correspondence | E-21 | 11 | P1",
            " | [] | [] | draft | [BC-5.39.010 v1.2] input-hash 47a65c9 |\n",
            "> **E-21 delivery:** Completed E-21 wave-4 stories.",
            " Input-hashes: S-21.01=32aaccc; S-21.07=47a65c9. All 7 distinct.\n",
        );
        let violations = run_arm_b2(content);
        assert!(
            violations.is_empty(),
            "production-shaped STORY-INDEX with catalog=blockquote=47a65c9 must not block. \
            Red Gate: run_arm_b2 generates spurious 'orphaned blockquote entry' from \
            production prose line (F-S2107-P1B-004)"
        );
    }

    // -----------------------------------------------------------------------
    // F-S2107-P1B-009: extract_input_hash_token accepts any 7+ char alphanumeric
    // token after "input-hash ". Non-hex tokens like "bonus" are accepted.
    // BC-5.39.010 v1.3 precondition 17: input-hash value must be hex (0-9a-f only).
    // -----------------------------------------------------------------------

    /// F-S2107-P1B-009: non-hex catalog token must not be accepted as input-hash.
    ///
    /// RED GATE: current code returns Some("bonus") — no hex validation.
    /// assert!(catalog_hash.is_none()) FAILS → RED gate.
    /// After fix (validate hex charset): returns None → PASSES.
    #[test]
    fn test_BC_5_39_010_arm_b_non_hex_catalog_token_not_accepted() {
        // "bonus" contains letters outside 0-9a-f ('o', 'n', 'u', 's') → non-hex
        let content = "| S-21.07 | ... | input-hash bonus | ...\n";
        let catalog_hash = parse_story_index_catalog_hash(content.as_bytes(), "S-21.07");
        assert!(
            catalog_hash.is_none(),
            "non-hex token 'bonus' must not be accepted as input-hash value. \
            BC-5.39.010 PC17: input-hash must be hex (F-S2107-P1B-009). \
            Red Gate: current code returns Some(\"bonus\") → assertion fails"
        );
    }

    // -----------------------------------------------------------------------
    // F-S2107-P1B-008: parse_story_index_catalog_hash uses naive contains(story_id).
    // When STORY-INDEX.md has a row for S-18.00 that includes "S-18.01" in its
    // blocks or depends_on column, `contains("S-18.01")` matches the S-18.00 row
    // FIRST (before the actual S-18.01 row), returning the wrong hash.
    // BC-5.39.010 v1.3 PC16: catalog lookup must match the CANONICAL S-NNN.NNN row
    // (i.e., the row whose FIRST cell is the story_id, not any row mentioning it).
    // -----------------------------------------------------------------------

    /// F-S2107-P1B-008: cross-story catalog lookup must not match wrong row.
    ///
    /// When S-18.00 row contains "S-18.01" in a later column, naive contains("S-18.01")
    /// matches the S-18.00 row first, returning S-18.00's hash instead of S-18.01's.
    ///
    /// RED GATE: parse_story_index_catalog_hash("S-18.01") returns "e5bc551" (S-18.00 hash).
    /// assert_eq!(catalog_hash, Some("1b4ea21")) FAILS → RED gate.
    /// After fix (match only rows where first cell == story_id): returns "1b4ea21" → PASSES.
    #[test]
    fn test_BC_5_39_010_arm_b1_cross_story_catalog_correct_row_matched() {
        // S-18.00 row mentions S-18.01 in a blocks/depends column — comes BEFORE S-18.01 row.
        // Naive contains("S-18.01") hits S-18.00 first → returns S-18.00 hash "e5bc551".
        let index = concat!(
            "| S-18.00 | parent epic | E-18 | ... | [S-18.01] | input-hash e5bc551 |\n",
            "| S-18.01 | child story | E-18 | ... | []        | input-hash 1b4ea21 |\n",
            "> **E-18 delivery:** S-18.00=e5bc551; S-18.01=1b4ea21.\n",
        );
        let catalog_hash = parse_story_index_catalog_hash(index.as_bytes(), "S-18.01");
        assert_eq!(
            catalog_hash,
            Some("1b4ea21".to_string()),
            "S-18.01 catalog lookup must return '1b4ea21' (own row), not 'e5bc551' (S-18.00 row). \
            Red Gate: naive contains('S-18.01') matches S-18.00 row first → wrong hash (F-S2107-P1B-008)"
        );
    }

    // -----------------------------------------------------------------------
    // PC40 (BC-5.39.010 v1.6 amendment): volatile-input precondition.
    //
    // When a story has `input-hash:` AND `inputs:` containing volatile paths
    // (STATE.md, INDEX files, cycles/ artifacts), Arm B1 must emit advisory +
    // Continue rather than proceeding with the three-way comparison.
    //
    // Volatile paths: .factory/STATE.md, .factory/**/BC-INDEX.md,
    // .factory/**/VP-INDEX.md, .factory/**/STORY-INDEX.md, .factory/cycles/**
    //
    // IMPLEMENTATION REQUIREMENT:
    //   1. Add `pub fn is_volatile_path(path: &str) -> bool` to arm_b.rs.
    //   2. Add `pub fn parse_story_volatile_inputs(content: &str) -> Vec<String>`
    //      that extracts the `inputs:` frontmatter YAML list.
    //   3. Modify `run_arm_b1` to check for volatile inputs BEFORE calling
    //      `run_arm_b1_with_index_result`; if any volatile path found, emit
    //      advisory + return Continue (skip the three-way comparison).
    //
    // RED GATE strategy: test calls panic!() stub — always fails until PC40 is
    // implemented and the test is updated to call the real functions.
    // -----------------------------------------------------------------------

    /// PC40: is_volatile_path + parse_story_volatile_inputs smoke test.
    ///
    /// Verifies the two PC40 pure functions and indirectly validates that
    /// run_arm_b1 skips the three-way comparison when volatile inputs are present.
    ///
    /// BC-5.39.010 v1.6 PC40: volatile inputs → advisory + Continue (skip comparison).
    #[test]
    fn test_BC_5_39_010_arm_b1_pc40_volatile_input_detection_required() {
        assert!(
            is_volatile_path(".factory/STATE.md"),
            ".factory/STATE.md must be volatile (PC40)"
        );
        assert!(
            !is_volatile_path(".factory/stories/S-21.07-test.md"),
            "story file must NOT be volatile"
        );
        let story = "---\ninputs: [\".factory/STATE.md\"]\n---\n";
        let inputs = parse_story_volatile_inputs(story);
        assert_eq!(inputs, vec![".factory/STATE.md".to_string()]);
    }
}
