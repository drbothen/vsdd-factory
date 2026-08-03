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
    let content = std::str::from_utf8(index_content).ok()?;
    for line in content.lines() {
        // Must be a table row (starts with |) containing story_id
        if !line.starts_with('|') {
            continue;
        }
        if !line.contains(story_id) {
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
    let content = std::str::from_utf8(index_content).ok()?;
    let prefix = format!("> {}=", story_id);
    for line in content.lines() {
        if line.starts_with(&prefix) {
            let rest = &line[prefix.len()..];
            let hash: String = rest
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric())
                .collect();
            if !hash.is_empty() {
                return Some(hash);
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

/// Arm B1 effectful entry point.
///
/// Reads STORY-INDEX.md via `host::read_file` (`max_bytes = 1048576`,
/// `timeout_ms = 3000`), then delegates to `run_arm_b1_with_index_result`.
///
/// Called from `on_post_tool_use` when a story file write is detected.
///
/// # BC trace
/// BC-5.39.010 preconditions 17-21 (STORY-INDEX.md read + hash comparison).
pub fn run_arm_b1(story_id: &str, story_content: &str) -> (Vec<Violation>, Vec<Advisory>) {
    let story_hash = match parse_story_input_hash(story_content) {
        Some(h) => h,
        None => return (vec![], vec![]), // No input-hash: skip (PC18)
    };
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
        } else if let Some(rest) = line.strip_prefix("> ") {
            // Blockquote entry: `> S-21.07=47a65c9`
            if let Some(eq_pos) = rest.find('=') {
                let story_id = rest[..eq_pos].trim().to_string();
                let hash: String = rest[eq_pos + 1..]
                    .chars()
                    .take_while(|c| c.is_ascii_alphanumeric())
                    .collect();
                if !story_id.is_empty() && !hash.is_empty() {
                    blockquote.push((story_id, hash));
                }
            }
        }
    }

    // Compare: for each blockquote entry, find corresponding catalog row
    for (bq_story_id, bq_hash) in &blockquote {
        let cat_hash = catalog
            .iter()
            .find(|(id, _)| id == bq_story_id)
            .map(|(_, h)| h.as_str());

        match cat_hash {
            Some(cat_h) => {
                if cat_h != bq_hash {
                    violations.push(Violation {
                        description: format!(
                            "validate-cross-site-correspondence [Class B] POLICY 18: \
                            STORY-INDEX.md internal parity violation for story {bq_story_id} \
                            — catalog={cat_h} blockquote={bq_hash} \
                            — run `compute-input-hash --update`"
                        ),
                    });
                }
            }
            None => {
                // Blockquote entry without catalog row — orphaned entry → violation
                violations.push(Violation {
                    description: format!(
                        "validate-cross-site-correspondence [Class B] POLICY 18: \
                        STORY-INDEX.md blockquote entry for {bq_story_id} has no corresponding \
                        catalog row — orphaned blockquote entry (stale or manually added)"
                    ),
                });
            }
        }
    }

    violations
}

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

/// Extract `input-hash <hex>` token from a table row or any content line.
/// Returns the hex hash string, or None if not present.
fn extract_input_hash_token(line: &str) -> Option<String> {
    let needle = "input-hash ";
    if let Some(pos) = line.find(needle) {
        let after = &line[pos + needle.len()..];
        let hash: String = after
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric())
            .collect();
        if !hash.is_empty() {
            return Some(hash);
        }
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
}
