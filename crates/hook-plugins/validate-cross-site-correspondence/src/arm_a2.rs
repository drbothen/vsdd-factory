//! arm_a2.rs — Class A Arm2: story body BC-version citations vs BC frontmatter.
//!
//! Effectful-shell module (ADR-035 §Decision 1): reads cited BC files via
//! `host::read_file` using the deterministic `derive_bc_path` path derivation
//! (no `host::list_dir`, no filesystem enumeration).
//!
//! # Behavior
//! Fires when a story file is written. For each BC listed in `behavioral_contracts:`,
//! extracts version tokens from table rows in the story body that cite that BC ID.
//! Compares each cited version against the BC frontmatter `version:` field (read
//! from the deterministically-derived path). Blocks if any cited version is stale.
//! Cascades: all stale citations are collected and reported in ONE combined block.
//!
//! # Key design invariants
//! - Path derivation: ONLY `derive_bc_path` (from arm_a1); NO `read_dir`/`glob`.
//! - Version token extraction: `\bv([0-9]+\.[0-9]+)\b` — table rows only.
//! - Skip (not block) on: missing citations in table rows, NotFound BC files.
//! - Block on: present-but-stale citations, CapabilityDenied on BC reads.
//! - Cascade: single combined block when multiple BCs have stale citations.
//!
//! # BC trace
//! BC-5.39.010 preconditions 9-15; postconditions 7-11; invariant 1 (no enumeration).

use crate::{Advisory, Violation};
use vsdd_hook_sdk::host::HostError;

/// Extract all table-row version citations for a given BC ID in story body content.
///
/// Scans `content` for pipe-delimited table rows (`|...|`) that contain both
/// the `bc_id` token and a version token matching `\bv([0-9]+\.[0-9]+)\b`.
/// Returns a `Vec<(location, version)>` where `location` is a human-readable
/// row identifier and `version` is the cited version string (e.g., `"1.17"`).
///
/// Only rows that contain BOTH the BC ID and a version token are included.
/// Prose mentions of the BC ID (without a version token in the same row) are NOT
/// included — this is the skip-not-block semantic for absent citations
/// (BC-5.39.010 postcondition 8).
///
/// # BC trace
/// BC-5.39.010 §Architecture Anchors `extract_story_bc_version_citations`;
/// preconditions 12-13 (table row detection + version token regex).
pub fn extract_story_bc_version_citations(content: &str, bc_id: &str) -> Vec<(String, String)> {
    // Scan content for pipe-delimited table rows (|...|) that contain both
    // the bc_id token AND a version token matching \bv([0-9]+\.[0-9]+)\b.
    // Returns Vec<(row_location, version)>.
    let mut citations = Vec::new();
    for (line_num, line) in content.lines().enumerate() {
        if !line.contains('|') {
            continue;
        }
        if !line.contains(bc_id) {
            continue;
        }
        // Line contains both bc_id and pipe chars (table row)
        // Now look for a version token vN.N in this row
        if let Some(version) = extract_version_token_from_table_row(line) {
            let location = format!("table row {}", line_num + 1);
            citations.push((location, version));
        }
    }
    citations
}

/// Extract a version token `vN.N` or `vN.NN` from a table row string.
/// Hand-rolled — no regex crate.
/// Returns the version number without the leading `v`.
fn extract_version_token_from_table_row(line: &str) -> Option<String> {
    let bytes = line.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'v' && i + 1 < bytes.len() && bytes[i + 1].is_ascii_digit() {
            // Check previous char is not alphanumeric (word boundary at start)
            let prev_ok = i == 0 || !bytes[i - 1].is_ascii_alphanumeric();
            if !prev_ok {
                i += 1;
                continue;
            }
            let start = i + 1; // skip 'v'
            let mut end = start;
            while end < bytes.len() && bytes[end].is_ascii_digit() {
                end += 1;
            }
            if end < bytes.len() && bytes[end] == b'.' {
                end += 1;
                if end < bytes.len() && bytes[end].is_ascii_digit() {
                    while end < bytes.len() && bytes[end].is_ascii_digit() {
                        end += 1;
                    }
                    // Word boundary at end
                    let next_ok = end >= bytes.len() || !bytes[end].is_ascii_alphanumeric();
                    if next_ok {
                        // Pure ASCII (digits and dot) — safe byte slice
                        return Some(line[start..end].to_string());
                    }
                }
            }
        }
        i += 1;
    }
    None
}

/// Class A Arm2 check for a single BC, with the BC file read result as a seam.
///
/// Pure-seam entry point for unit testing: the caller provides the host read
/// result for the BC file rather than performing a live `host::read_file` call.
///
/// Returns `(violations, advisories)`:
/// - If `citations` is empty for this BC: returns empty (skip — postcondition 8).
/// - If BC read returns `NotFound`: advisory only, Continue (postcondition 10).
/// - If BC read returns `CapabilityDenied`: blocking violation.
/// - If BC frontmatter version matches all citation versions: empty.
/// - If any citation has a stale version: blocking violation per stale cite.
///
/// # BC trace
/// BC-5.39.010 postconditions 7-11; preconditions 14-15.
pub fn run_arm_a2_for_bc_with_result(
    story_id: &str,
    bc_id: &str,
    citations: &[(String, String)],
    bc_read_result: Result<Vec<u8>, HostError>,
) -> (Vec<Violation>, Vec<Advisory>) {
    // Empty citations → skip (postcondition 8)
    if citations.is_empty() {
        return (vec![], vec![]);
    }

    match bc_read_result {
        Err(HostError::NotFound) => {
            // NotFound → advisory + Continue (postcondition 10)
            let advisory = Advisory {
                message: format!(
                    "validate-cross-site-correspondence [Class A Arm2] advisory: \
                    BC file '{bc_id}' not found when checking story '{story_id}' — \
                    bootstrap ordering or deleted BC."
                ),
            };
            (vec![], vec![advisory])
        }
        Err(other) => {
            // CapabilityDenied or other → block (fail-closed)
            let violation = Violation {
                description: format!(
                    "validate-cross-site-correspondence [Class A Arm2]: \
                    host error reading BC '{bc_id}' for story '{story_id}': {other:?}. \
                    Verify read_file path_allow includes '.factory/specs/behavioral-contracts/'."
                ),
            };
            (vec![violation], vec![])
        }
        Ok(bc_bytes) => {
            let bc_content = std::str::from_utf8(&bc_bytes).unwrap_or("");
            let bc_version = crate::frontmatter::extract_frontmatter_field(bc_content, "version")
                .unwrap_or_default();

            let mut violations = Vec::new();
            for (location, cited_version) in citations {
                if *cited_version != bc_version {
                    violations.push(Violation {
                        description: format!(
                            "validate-cross-site-correspondence [Class A Arm2]: \
                            story '{story_id}' cites '{bc_id}' at version v{cited_version} \
                            in {location}, but BC frontmatter says version {bc_version}. \
                            Update the story's BC-table citation to v{bc_version}. POLICY 14 leg 3."
                        ),
                    });
                }
            }
            (violations, vec![])
        }
    }
}

/// Class A Arm2 effectful entry point for a single BC.
///
/// Derives the BC path via `derive_bc_path`, reads the BC file via
/// `host::read_file` (`max_bytes = 524288`, `timeout_ms = 3000`), then
/// delegates to `run_arm_a2_for_bc_with_result`.
///
/// # Implementation guide
/// ```text
/// let bc_path = arm_a1::derive_bc_path(bc_id);
/// let bc_result = host::read_file(&bc_path, BC_MAX_BYTES, BC_TIMEOUT_MS);
/// run_arm_a2_for_bc_with_result(story_id, bc_id, citations, bc_result)
/// ```
///
/// # BC trace
/// BC-5.39.010 preconditions 11 (derive_bc_path), 14-15 (read failure semantics).
pub fn run_arm_a2_for_bc(
    story_id: &str,
    bc_id: &str,
    citations: &[(String, String)],
) -> (Vec<Violation>, Vec<Advisory>) {
    use crate::arm_a1::{BC_MAX_BYTES, BC_TIMEOUT_MS, derive_bc_path};
    let bc_path = derive_bc_path(bc_id);
    let bc_result = vsdd_hook_sdk::host::read_file(&bc_path, BC_MAX_BYTES, BC_TIMEOUT_MS);
    run_arm_a2_for_bc_with_result(story_id, bc_id, citations, bc_result)
}

/// Class A Arm2 full entry point: processes all BCs from a story's frontmatter.
///
/// Reads `behavioral_contracts:` from story frontmatter; for each BC ID, extracts
/// version citations, reads the BC, and checks for staleness. Cascade: all stale
/// citations across all BCs are collected and returned as a single combined block
/// (postcondition 7 last sentence).
///
/// # BC trace
/// BC-5.39.010 postcondition 7 (cascade); preconditions 9-15.
pub fn run_arm_a2(story_id: &str, story_content: &str) -> (Vec<Violation>, Vec<Advisory>) {
    let bc_ids =
        crate::frontmatter::extract_frontmatter_sequence(story_content, "behavioral_contracts");
    if bc_ids.is_empty() {
        return (vec![], vec![]);
    }

    let mut all_violations = Vec::new();
    let mut all_advisories = Vec::new();

    for bc_id in &bc_ids {
        let citations = extract_story_bc_version_citations(story_content, bc_id);
        let (violations, advisories) = run_arm_a2_for_bc(story_id, bc_id, &citations);
        all_violations.extend(violations);
        all_advisories.extend(advisories);
    }

    (all_violations, all_advisories)
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // extract_story_bc_version_citations — BC-5.39.010 PC12-13
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_5_39_010_arm_a2_bc_path_derivation_correct() {
        // Architecture Compliance Rule: no list_dir; derive_bc_path used
        use crate::arm_a1::derive_bc_path;
        let path = derive_bc_path("BC-6.26.001");
        assert_eq!(
            path, ".factory/specs/behavioral-contracts/ss-06/BC-6.26.001.md",
            "Arm A2 must use derive_bc_path to derive BC paths deterministically"
        );
    }

    #[test]
    fn test_BC_5_39_010_arm_a2_version_citation_extracted_from_table_row() {
        let content = "---\nbehavioral_contracts: [BC-6.26.001]\n---\n\
            | BC-6.26.001 | Title | v1.17 | active |\n";
        let result = extract_story_bc_version_citations(content, "BC-6.26.001");
        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0].1, "1.17",
            "version token must be extracted from table row"
        );
    }

    #[test]
    fn test_BC_5_39_010_arm_a2_no_table_row_returns_empty() {
        // AC-008: BC ID present only in prose, no version-citing table row → skip
        let content = "---\nbehavioral_contracts: [BC-6.26.001]\n---\n\
            The BC-6.26.001 contract governs this story.\n";
        let result = extract_story_bc_version_citations(content, "BC-6.26.001");
        assert!(
            result.is_empty(),
            "BC ID in prose only (no version token in row) must return empty"
        );
    }

    // -----------------------------------------------------------------------
    // run_arm_a2_for_bc_with_result — BC-5.39.010 postconditions 7-11
    // -----------------------------------------------------------------------

    /// AC-005 MUTANT: stale Token Budget row blocks (BC-5.39.010 postcondition 7).
    #[test]
    fn test_BC_5_39_010_arm_a2_stale_token_budget_row_blocks() {
        let bc_content = b"---\nversion: \"1.18\"\n---\n# BC-6.26.001\n";
        let citations = vec![("Token Budget row".to_string(), "1.17".to_string())];
        let (violations, _) = run_arm_a2_for_bc_with_result(
            "S-21.07",
            "BC-6.26.001",
            &citations,
            Ok(bc_content.to_vec()),
        );
        assert!(
            !violations.is_empty(),
            "stale citation must produce a blocking violation"
        );
        let msg = &violations[0].description;
        assert!(
            msg.contains("[Class A Arm2]"),
            "violation must cite [Class A Arm2]"
        );
        assert!(
            msg.contains("v1.17"),
            "violation must cite stale version v1.17"
        );
        assert!(
            msg.contains("1.18"),
            "violation must cite current BC version 1.18"
        );
    }

    /// AC-005 CONTROL: current citation passes.
    #[test]
    fn test_BC_5_39_010_arm_a2_current_citation_passes() {
        let bc_content = b"---\nversion: \"1.18\"\n---\n# BC-6.26.001\n";
        let citations = vec![("Token Budget row".to_string(), "1.18".to_string())];
        let (violations, _) = run_arm_a2_for_bc_with_result(
            "S-21.07",
            "BC-6.26.001",
            &citations,
            Ok(bc_content.to_vec()),
        );
        assert!(violations.is_empty(), "current citation must not block");
    }

    /// AC-006: two stale BCs → single combined block (postcondition 7 cascade).
    #[test]
    fn test_BC_5_39_010_arm_a2_two_stale_bcs_combined_block() {
        // Test via run_arm_a2 with story content containing two stale BCs
        let story_content = "---\nbehavioral_contracts: [BC-6.26.001, BC-5.39.008]\n---\n\
            | BC-6.26.001 | Title | v1.17 | active |\n\
            | BC-5.39.008 | Title | v1.5 | active |\n";
        let (violations, _) = run_arm_a2("S-21.07", story_content);
        // Both BCs are stale — combined into one or multiple violations
        // The BC calls will todo!() → panic → test FAILS (RED gate holds)
        assert!(
            !violations.is_empty(),
            "two stale BCs must produce combined violations"
        );
    }

    /// AC-007: empty behavioral_contracts skips Arm A2 (postcondition 9).
    #[test]
    fn test_BC_5_39_010_arm_a2_empty_bcs_skips() {
        let story_content = "---\nbehavioral_contracts: []\n---\nbody\n";
        let (violations, advisories) = run_arm_a2("S-21.07", story_content);
        assert!(
            violations.is_empty(),
            "empty BCs must not produce violations"
        );
        assert!(
            advisories.is_empty(),
            "empty BCs must not produce advisories"
        );
    }

    /// AC-008: no version-citing row for a BC → skip that BC (postcondition 8).
    #[test]
    fn test_BC_5_39_010_arm_a2_no_version_row_skips() {
        let bc_content = b"---\nversion: \"1.18\"\n---\n# BC-6.26.001\n";
        // Empty citations = no table row with version token
        let citations: Vec<(String, String)> = vec![];
        let (violations, _) = run_arm_a2_for_bc_with_result(
            "S-21.07",
            "BC-6.26.001",
            &citations,
            Ok(bc_content.to_vec()),
        );
        assert!(
            violations.is_empty(),
            "no version-citing rows must skip (not block)"
        );
    }

    /// AC-008: BC file NotFound → advisory + Continue (postcondition 10).
    #[test]
    fn test_BC_5_39_010_arm_a2_bc_not_found_advisory() {
        let citations = vec![("Token Budget row".to_string(), "1.17".to_string())];
        let (violations, advisories) = run_arm_a2_for_bc_with_result(
            "S-21.07",
            "BC-6.26.001",
            &citations,
            Err(HostError::NotFound),
        );
        assert!(violations.is_empty(), "NotFound BC must not block");
        assert!(!advisories.is_empty(), "NotFound BC must emit an advisory");
    }
}
