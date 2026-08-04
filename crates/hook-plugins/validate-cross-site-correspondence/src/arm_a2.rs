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

/// Returns `true` if `heading` (the text after `## `) identifies a target section
/// that must be scanned for BC-version citations.
///
/// A target section matches iff the heading starts with the section prefix AND
/// the next character (if any) is a word boundary: space or `(`. This implements
/// `^## Behavioral Contracts\b` and `^## Token Budget\b` per PC13 (amended, v1.4).
///
/// Admitted examples:
/// - `Behavioral Contracts`, `Behavioral Contracts Table`, `Behavioral Contracts (BC Count: N)`
/// - `Token Budget`, `Token Budget Estimate`, `Token Budget Estimate (MANDATORY)`
///
/// Excluded by word-boundary guard:
/// - `Behavioral Contracting` (not `starts_with("Behavioral Contracts")` — position 19 is 'i' vs 's')
///
/// # BC trace
/// BC-5.39.010 PC13 (amended v1.4): word-boundary prefix predicate; non-conformance note
/// explicitly forbids `heading == "..."` string equality.
fn is_target_heading(heading: &str) -> bool {
    is_section_prefix(heading, "Behavioral Contracts") || is_section_prefix(heading, "Token Budget")
}

/// Returns `true` if `heading` starts with `prefix` followed by a word boundary
/// (space, `(`, or end-of-string). Does NOT admit `prefix` followed by an alphanumeric
/// continuation (e.g., `Token Budgeting` would not match `Token Budget`).
fn is_section_prefix(heading: &str, prefix: &str) -> bool {
    if heading == prefix {
        return true;
    }
    if let Some(rest) = heading.strip_prefix(prefix) {
        // Word boundary: next char must be ' ' or '('
        matches!(rest.chars().next(), Some(' ') | Some('('))
    } else {
        false
    }
}

/// Extract all table-row version citations for a given BC ID in story body content.
///
/// Scans `content` for pipe-delimited table rows (`|...|`) that contain both
/// the `bc_id` token and a version token matching `\bv?([0-9]+\.[0-9]+)\b`.
/// Returns a `Vec<(location, version)>` where `location` is a human-readable
/// row identifier and `version` is the cited version string (e.g., `"1.17"`).
///
/// Only rows that contain BOTH the BC ID and a version token are included.
/// Prose mentions of the BC ID (without a version token in the same row) are NOT
/// included — this is the skip-not-block semantic for absent citations
/// (BC-5.39.010 postcondition 8).
///
/// # Section bounding (PC13 amended v1.4, F-S2107-P1B-001)
/// The scan is bounded to named sections that carry BC-table citations.
/// A `## ` heading switches the scanner to skip mode UNLESS the heading matches
/// the word-boundary prefix predicate (see `is_target_heading`):
/// - `^## Behavioral Contracts\b` — admits `Behavioral Contracts`, `Behavioral Contracts Table`, etc.
/// - `^## Token Budget\b` — admits `Token Budget`, `Token Budget Estimate`, `Token Budget Estimate (MANDATORY)`, etc.
///
/// Using exact string equality is non-conforming per PC13 v1.4 non-conformance note.
///
/// Content that has no `## ` headings at all (e.g., simple unit-test fixtures) is
/// scanned without restriction — the scanner starts in "no active section" state
/// which is treated as scannable.
///
/// # BC trace
/// BC-5.39.010 §Architecture Anchors `extract_story_bc_version_citations`;
/// preconditions 12-13 (table row detection + version token regex); PC13 (amended v1.4:
/// word-boundary prefix predicate, optional `v` prefix, last/rightmost token).
pub fn extract_story_bc_version_citations(content: &str, bc_id: &str) -> Vec<(String, String)> {
    let mut citations = Vec::new();
    // skip_section: true when inside a named ## section that is NOT a target section.
    //
    // F-P2-001 fix: initialize to `true` unconditionally.
    // BC-5.39.010 PC13 (v1.3+): scan confined to ^## Behavioral Contracts and
    // ^## Token Budget ONLY. Content before the first target heading — YAML frontmatter
    // body, preamble prose, any line outside a scoped section — must NOT be scanned.
    // A file with zero ## headings contains zero scannable sections; correct output is
    // zero citations. Initializing to false inverts this guarantee and reintroduces the
    // unbounded-scan regression PC13 v1.3 was introduced to prevent.
    let mut skip_section = true;

    for (line_num, line) in content.lines().enumerate() {
        // Detect ## heading and update section context
        if let Some(rest) = line.strip_prefix("## ") {
            let heading = rest.trim();
            skip_section = !is_target_heading(heading);
            continue;
        }

        if skip_section {
            continue;
        }

        if !line.contains('|') {
            continue;
        }
        if !line.contains(bc_id) {
            continue;
        }

        // Row is in a scannable section and contains the BC ID: extract version
        if let Some(version) = extract_version_token_from_table_row(line) {
            let location = format!("table row {}", line_num + 1);
            citations.push((location, version));
        }
    }
    citations
}

/// Extract a version token `v?N.N` or `v?N.NN` from a table row string.
///
/// Implements `\bv?([0-9]+\.[0-9]+)\b` semantics (PC13 amended): the `v` prefix
/// is optional. Returns the version number without any leading `v`.
///
/// Per PC13 (LAST/rightmost pipe-field token): scans the entire line and returns
/// the LAST matching token, not the first. This prevents spurious matches from
/// BC ID fragments like "BC-5.39.010" (which contains "5.39") from masking the
/// actual version column that appears later in the row.
///
/// Hand-rolled — no regex crate (ADR-035 §Decision 5 fuel-budget constraint).
fn extract_version_token_from_table_row(line: &str) -> Option<String> {
    let bytes = line.as_bytes();
    let mut last_match: Option<String> = None;
    let mut i = 0;
    while i < bytes.len() {
        // Word boundary check: preceding char must not be alphanumeric (F-S2107-P1B-002)
        let prev_ok = i == 0 || !bytes[i - 1].is_ascii_alphanumeric();
        if !prev_ok {
            i += 1;
            continue;
        }

        // Determine start of digit run (skip optional 'v' prefix per PC13)
        let digit_start =
            if bytes[i] == b'v' && i + 1 < bytes.len() && bytes[i + 1].is_ascii_digit() {
                i + 1 // optional 'v' prefix; digit run starts at i+1
            } else if bytes[i].is_ascii_digit() {
                i // bare digit: digit run starts at i
            } else {
                i += 1;
                continue;
            };

        // Scan the integer part of the version
        let mut end = digit_start;
        while end < bytes.len() && bytes[end].is_ascii_digit() {
            end += 1;
        }

        // Require exactly one dot followed by at least one digit (N.N shape)
        if end < bytes.len() && bytes[end] == b'.' {
            let post_dot = end + 1;
            if post_dot < bytes.len() && bytes[post_dot].is_ascii_digit() {
                end = post_dot;
                while end < bytes.len() && bytes[end].is_ascii_digit() {
                    end += 1;
                }
                // Word boundary at end: next char must not be alphanumeric
                let next_ok = end >= bytes.len() || !bytes[end].is_ascii_alphanumeric();
                if next_ok {
                    // All bytes in digit_start..end are ASCII digits or '.': safe slice
                    last_match = Some(line[digit_start..end].to_string());
                    i = end;
                    continue;
                }
                // Not a word boundary: advance past the matched digit run's start
                i = digit_start + 1;
                continue;
            }
        }

        // No valid N.N token starting here; advance one byte
        i = digit_start + 1;
    }
    last_match
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
            // F-S2107-P1B-016: surface UTF-8 decode failure as a distinct violation;
            // unwrap_or("") silently turns a corrupt BC file into bc_version="" which
            // mismatches every citation and directs the fixer toward a version bump.
            let bc_content = match std::str::from_utf8(&bc_bytes) {
                Ok(s) => s,
                Err(_) => {
                    return (
                        vec![Violation {
                            description: format!(
                                "validate-cross-site-correspondence [Class A Arm2]: \
                                BC file '{bc_id}' content is not valid UTF-8 — \
                                cannot verify version citations in story '{story_id}'. \
                                Fix: ensure the BC file is saved as UTF-8."
                            ),
                        }],
                        vec![],
                    );
                }
            };
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

    // -----------------------------------------------------------------------
    // T-048 / F-S2107-P1B-002: bare version token (no `v` prefix) must be detected.
    //
    // The real S-21.07 Behavioral Contracts table row is:
    //   | BC-5.39.010 | <title> | 1.3 | AC-001 through AC-021 |
    //
    // BC-5.39.010 v1.3 AC-017 (amended PC13) explicitly requires the version
    // column in the Behavioral Contracts table to be treated as authoritative even
    // without a `v` prefix. The production story file S-21.07 uses bare "1.3".
    //
    // Bug: `extract_version_token_from_table_row` only checks `bytes[i] == b'v'`.
    // A bare "1.3" cell has no `v` byte at position 0, so the digit sequence is
    // invisible → function returns None → zero citations for the story's own BC.
    //
    // This means the arm fires NO version check against BC-5.39.010 v1.3 when the
    // story body references it — a complete silent bypass of the Arm A2 gate for
    // the governing BC of this very plugin.
    //
    // After fix (optional `v?` prefix: check for bare digit start in addition to
    // `b'v'` prefix): detect "1.3" → citation added → 1 citation returned.
    //
    // RED GATE: extract_version_token_from_table_row only matches b'v'; bare "1.3"
    // returns None → citations.len() == 0 ≠ 1 → assertion FAILS.
    // -----------------------------------------------------------------------

    /// T-048 / F-S2107-P1B-002: bare version '1.3' in BC table row must be detected.
    ///
    /// RED GATE: extract_story_bc_version_citations returns 0 citations (bare version
    /// invisible to extract_version_token_from_table_row). assert_eq!(len, 1) FAILS.
    #[test]
    fn test_BC_5_39_010_arm_a2_bare_version_bc_table_row_detected() {
        // Production-shaped: matches the real S-21.07 Behavioral Contracts table row
        // structure. Version cell is "1.3" with no leading `v`.
        let content = "---\n\
            behavioral_contracts: [BC-5.39.010]\n\
            ---\n\
            \n\
            ## Behavioral Contracts\n\
            \n\
            | BC ID | Title | Version | ACs |\n\
            | --- | --- | --- | --- |\n\
            | BC-5.39.010 | WASM hook cross-site correspondence gate | 1.3 | AC-001 through AC-021 |\n";

        let citations = extract_story_bc_version_citations(content, "BC-5.39.010");

        assert_eq!(
            citations.len(),
            1,
            "bare version '1.3' in Behavioral Contracts table row must be detected. \
            F-S2107-P1B-002: extract_version_token_from_table_row only checks bytes[i]==b'v', \
            silently skipping bare version cells. Current citations: {:?}",
            citations
        );
        if citations.len() == 1 {
            assert_eq!(
                citations[0].1, "1.3",
                "extracted version must be '1.3' (without v prefix)"
            );
        }
    }

    // -----------------------------------------------------------------------
    // T-049 / F-S2107-P1B-001: scan must be bounded to the Behavioral Contracts
    // section; Edge Cases table rows must NOT be scanned.
    //
    // Current bug: `extract_story_bc_version_citations` scans ALL lines that
    // contain both bc_id and a pipe character. In the real S-21.07 story file,
    // the Edge Cases table contains rows like:
    //
    //   EC-002: "BC bumped v1.17→v1.18; INDEX says v1.17" in col 2, "BC-5.39.010 EC-002" in col 4
    //   EC-015: "BC written `version: "1.33"`, `last_amended: "... (v1.31) ..."` " in col 2,
    //           "BC-5.39.010 EC-015" in col 4
    //   EC-017: "modified: ["2026-05-14", "2026-05-18", "2026-05-20 (v1.3)"]" in col 2,
    //           "BC-5.39.010 EC-017" in col 4
    //
    // All three rows pass the "contains bc_id" check (last cell has "BC-5.39.010 EC-NNN").
    // All three rows have version tokens in other columns (v1.17, v1.31, v1.3 respectively).
    // The unbounded scan therefore picks up spurious "stale version" citations from
    // descriptive content, leading to false-positive blocking violations.
    //
    // The fix (amended PC13): scan is bounded to the `## Behavioral Contracts`
    // and `## Token Budget` sections only; scanning stops at the next `##` heading.
    //
    // Additionally the v? prefix fix (T-048) is needed to detect the actual BC table
    // row — in this test we use a v-prefixed version "v1.3" in the BC table row
    // to isolate the section-bounding issue from the bare-version issue.
    //
    // After fix (section-bounded + optional v?):
    //   - Only the `## Behavioral Contracts` section is scanned
    //   - BC table row (v1.3) → 1 citation
    //   - EC-002, EC-015, EC-017 rows NOT scanned → 0 spurious citations
    //   - Total: exactly 1 citation
    //
    // RED GATE: unbounded scan hits BC table + EC-002 + EC-015 (all contain BC-5.39.010
    // and a version token) → 3 citations → assert_eq!(citations.len(), 1) FAILS.
    // -----------------------------------------------------------------------

    // -----------------------------------------------------------------------
    // F-P2-001 (BLOCKER): extract_story_bc_version_citations — skip_section leading-edge.
    //
    // Bug: arm_a2.rs line 96 initializes `skip_section = false`. Under this init,
    // ALL content before the first `## ` heading (YAML frontmatter body, preamble
    // prose) is in the scan window. Any preamble line that contains a pipe, the
    // bc_id, and a version token is spuriously picked up as a BC version citation.
    //
    // Adversary F-P2-001 orch-verified: S-21.04 has 20 occurrences of BC-6.26.001
    // in frontmatter; line 11 of S-21.04 contains a pipe → false citation → block.
    //
    // BC-5.39.010 PC13: scan must be confined to ^## Behavioral Contracts and
    // ^## Token Budget sections only. skip_section MUST be initialized to TRUE.
    // -----------------------------------------------------------------------

    /// F-P2-001 (BLOCKER): preamble before any ## heading must not be scanned.
    ///
    /// RED GATE: `skip_section = false` → preamble line is in scan window →
    /// "Cross-reference note | BC-5.39.010 | see version v1.0 |" yields citation "1.0" →
    /// total 2 citations → `assert_eq!(citations.len(), 1)` FAILS.
    /// After fix (skip_section = true): preamble NOT scanned → 1 citation (BC section only).
    #[test]
    fn test_BC_5_39_010_arm_a2_frontmatter_preamble_not_scanned_skip_section_true() {
        // Content with a preamble line (before any ## heading) that contains
        // a pipe + bc_id + version token. Should produce ZERO preamble citations.
        let content = concat!(
            "---\n",
            "behavioral_contracts: [BC-5.39.010]\n",
            "version: 1.5\n",
            "---\n",
            "\n",
            // Preamble line: has pipe + BC-5.39.010 + version token v1.0
            // skip_section=false (BUG): SCANNED → spurious citation "1.0"
            // skip_section=true  (FIX): NOT scanned → no citation from preamble
            "Cross-reference note | BC-5.39.010 | see version v1.0 for prior schema |\n",
            "\n",
            "## Behavioral Contracts\n",
            "\n",
            "| BC-5.39.010 | WASM hook gate | v1.5 | AC-001 |\n",
        );
        let citations = extract_story_bc_version_citations(content, "BC-5.39.010");
        // After fix: exactly 1 citation from ## Behavioral Contracts section row.
        // Currently (skip_section=false): 2 citations — preamble "1.0" + BC section "1.5".
        assert_eq!(
            citations.len(),
            1,
            "preamble lines before any ## heading must NOT produce BC citations. \
            BC-5.39.010 PC13: skip_section must be initialized to true (F-P2-001). \
            RED GATE: skip_section=false → {} citations (preamble + BC section). \
            Expected: 1 (BC section only). Citations: {:?}",
            citations.len(),
            citations
        );
        if citations.len() == 1 {
            assert_eq!(
                citations[0].1, "1.5",
                "only the Behavioral Contracts table row version (1.5) must be returned"
            );
        }
    }

    /// T-049 / F-S2107-P1B-001: Edge Cases rows must not be scanned.
    ///
    /// RED GATE: unbounded scan returns ≥3 citations (BC table + EC-002 + EC-015).
    /// assert_eq!(citations.len(), 1) FAILS.
    #[test]
    fn test_BC_5_39_010_arm_a2_edge_cases_rows_not_scanned_section_bounded() {
        // Production-shaped content from S-21.07 — v-prefixed version in the BC table
        // row to isolate the section-bounding defect from the bare-version defect (T-048).
        // EC-002 and EC-015 rows carry BC-5.39.010 in the last column (Source cell)
        // plus incidental version tokens in the scenario description column.
        let content = "---\n\
            behavioral_contracts: [BC-5.39.010]\n\
            ---\n\
            \n\
            ## Behavioral Contracts\n\
            \n\
            | BC ID | Title | Version | ACs |\n\
            | --- | --- | --- | --- |\n\
            | BC-5.39.010 | WASM hook cross-site correspondence gate | v1.3 | AC-001 through AC-021 |\n\
            \n\
            ## Edge Cases\n\
            \n\
            | ID | Scenario | Expected | Source |\n\
            | --- | --- | --- | --- |\n\
            | EC-002 | BC bumped v1.17\u{2192}v1.18; INDEX says v1.17 | Block: Class A Arm1 | BC-5.39.010 EC-002 |\n\
            | EC-015 | BC written `version: \"1.33\"`, `last_amended: \"... (v1.31) ...\"` | Block: Class E1 | BC-5.39.010 EC-015 |\n\
            | EC-017 | modified: [\"2026-05-14\", \"2026-05-18\", \"2026-05-20 (v1.3)\"] | Part E passes | BC-5.39.010 EC-017 |\n";

        let citations = extract_story_bc_version_citations(content, "BC-5.39.010");

        // After fix: exactly 1 citation from the ## Behavioral Contracts section row.
        // Currently (unbounded): BC table row (v1.3) + EC-002 (v1.17) + EC-015 (v1.31)
        // + EC-017 (v1.3) = 4 citations. assert_eq! FAILS.
        assert_eq!(
            citations.len(),
            1,
            "Edge Cases table rows must NOT produce BC version citations. \
            F-S2107-P1B-001: unbounded scan yields spurious citations from EC-002 (v1.17), \
            EC-015 (v1.31), and EC-017 (v1.3). Current citations: {:?}",
            citations
        );
        if citations.len() == 1 {
            assert_eq!(
                citations[0].1, "1.3",
                "only the Behavioral Contracts table row citation must be returned"
            );
        }
    }
}
