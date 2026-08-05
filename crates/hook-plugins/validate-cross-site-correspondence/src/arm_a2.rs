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
/// Content with no `## ` headings — or no headings matching either target predicate —
/// yields **zero citations**. The scanner initializes in skip mode (`skip_section = true`)
/// and only enters a scannable state upon encountering a target `## ` heading. Absent
/// any such heading, no rows are inspected. This is the correct PC13 (v1.3) behavior:
/// the unbounded-scan regression that PC13 was introduced to prevent is exactly the
/// case where content before the first target section is treated as scannable.
/// (F-P2-001: prior `skip_section = false` initialization admitted preamble content.)
///
/// # BC trace
/// BC-5.39.010 §Architecture Anchors `extract_story_bc_version_citations`;
/// preconditions 12-13 (table row detection + version token regex); PC13 (amended v1.4:
/// word-boundary prefix predicate, optional `v` prefix, last/rightmost token);
/// F-P2-001 (skip_section initialization — preamble must not be scanned).
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
        if !line_contains_bc_id_at_boundary(line, bc_id) {
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

/// Returns `true` if `line` contains `bc_id` as a standalone token.
///
/// `line.contains(bc_id)` has a prefix-collision defect (F-S2107-P3-004):
/// "BC-5.39.0101" contains "BC-5.39.010" as a substring. The word-boundary
/// check requires that the character immediately AFTER bc_id in the line is
/// NOT alphanumeric (digit or ASCII letter).
///
/// Hand-rolled — no regex crate (ADR-035 §Decision 5 fuel-budget constraint).
///
/// # BC trace
/// BC-5.39.010 v1.9 PC13: word-boundary bc_id token test.
/// F-S2107-P3-004: `line.contains(bc_id)` prefix-collision fix.
fn line_contains_bc_id_at_boundary(line: &str, bc_id: &str) -> bool {
    let mut search_start = 0;
    while search_start < line.len() {
        let Some(rel) = line[search_start..].find(bc_id) else {
            return false;
        };
        let abs = search_start + rel;
        let end = abs + bc_id.len();
        // Word boundary at end: char after bc_id must not be alphanumeric.
        let boundary_ok = line[end..]
            .chars()
            .next()
            .map(|c| !c.is_ascii_alphanumeric())
            .unwrap_or(true); // end-of-string is always a boundary
        if boundary_ok {
            return true;
        }
        // Not a boundary: advance and try again
        search_start = abs + 1;
    }
    false
}

/// Check if `s` is a pure-version string `^v?[0-9]+\.[0-9]+$`.
///
/// Returns the version string WITHOUT a leading `v`.
/// Admits "1.7", "v1.7", "1.3", "v1.3", "1.10", "v1.10".
/// Rejects "BC-5.39.010", "S-4.07", "~4,000", "DEFERRED v1.6 text".
fn parse_pure_version_field(s: &str) -> Option<String> {
    let inner = s.strip_prefix('v').unwrap_or(s);
    let bytes = inner.as_bytes();
    if bytes.is_empty() || !bytes[0].is_ascii_digit() {
        return None;
    }
    let mut i = 0;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i >= bytes.len() || bytes[i] != b'.' {
        return None;
    }
    i += 1;
    if i >= bytes.len() || !bytes[i].is_ascii_digit() {
        return None;
    }
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    // Must consume the entire string (pure-version field, not a substring)
    if i == inner.len() {
        Some(inner.to_string())
    } else {
        None
    }
}

/// Find the rightmost `\bv([0-9]+\.[0-9]+)\b` token in a single field.
///
/// The `v` prefix is mandatory (Phase 2 of PC13). Returns the version without `v`.
/// Scans left-to-right and keeps updating `last_match` — callers receive the
/// rightmost occurrence.
fn extract_mandatory_v_inline(s: &str) -> Option<String> {
    let bytes = s.as_bytes();
    let mut last_match: Option<String> = None;
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'v' {
            let prev_ok = i == 0 || !bytes[i - 1].is_ascii_alphanumeric();
            if prev_ok && i + 1 < bytes.len() && bytes[i + 1].is_ascii_digit() {
                let digit_start = i + 1;
                let mut j = digit_start;
                while j < bytes.len() && bytes[j].is_ascii_digit() {
                    j += 1;
                }
                if j < bytes.len() && bytes[j] == b'.' {
                    let post_dot = j + 1;
                    if post_dot < bytes.len() && bytes[post_dot].is_ascii_digit() {
                        let mut k = post_dot;
                        while k < bytes.len() && bytes[k].is_ascii_digit() {
                            k += 1;
                        }
                        let next_ok = k >= bytes.len() || !bytes[k].is_ascii_alphanumeric();
                        if next_ok {
                            last_match = Some(s[digit_start..k].to_string());
                            i = k;
                            continue;
                        }
                    }
                }
            }
        }
        i += 1;
    }
    last_match
}

/// Extract a version token from a table row using the v1.9 two-phase PC13 algorithm.
///
/// **Phase 1 (pure-version field):** split row by `|`; scan fields right-to-left;
/// return the version from the first (rightmost) field whose trimmed content
/// matches `^v?[0-9]+\.[0-9]+$`. The `v` prefix is optional in Phase 1.
///
/// **Phase 2 (mandatory-v fallback):** if Phase 1 finds nothing, scan fields
/// right-to-left for the rightmost `\bv([0-9]+\.[0-9]+)\b` inline token.
/// The `v` prefix is MANDATORY in Phase 2.
///
/// Three collision classes fixed versus the prior optional-v left-to-right scanner:
///   - Class 1: story IDs like "S-4.07" → no pure-version field, no mandatory-v
///     token → no citation (29 rows across 6 stories).
///   - Class 2: "DEFERRED v1.6" in ACs column when Version column is "1.7" →
///     Phase 1 finds "1.7" first (rightmost pure-version field); "v1.6" never reached.
///   - Class 3: "BC-1.13.001" fragments → Phase 1 no pure-version; Phase 2 no
///     mandatory-v → no citation.
///
/// Hand-rolled — no regex crate (ADR-035 §Decision 5 fuel-budget constraint).
///
/// # BC trace
/// BC-5.39.010 v1.9 PC13: two-phase version extraction algorithm.
/// F-S2107-P3-022: reverse-field algorithm (replaces left-to-right last-token).
/// F-S2107-P1B-002: optional-v Phase 1 (detects bare "1.3" version cells).
fn extract_version_token_from_table_row(line: &str) -> Option<String> {
    let fields: Vec<&str> = line.split('|').collect();

    // Phase 1: pure-version field — rightmost field first.
    for field in fields.iter().rev() {
        let trimmed = field.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(v) = parse_pure_version_field(trimmed) {
            return Some(v);
        }
    }

    // Phase 2: mandatory-v inline token — rightmost field first.
    for field in fields.iter().rev() {
        let trimmed = field.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(v) = extract_mandatory_v_inline(trimmed) {
            return Some(v);
        }
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
        // Fixture reflects real corpus shape: BC table row lives under the
        // ## Behavioral Contracts section heading (POLICY 8 / PC13 v1.3+).
        // Without the heading, skip_section starts true and nothing is scanned.
        let content = "---\nbehavioral_contracts: [BC-6.26.001]\n---\n\
            ## Behavioral Contracts\n\n\
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

    /// AC-006: two stale BCs → stale-citation violations with both BC IDs and versions.
    ///
    /// BC-5.39.010 postcondition 7 cascade: all stale citations across all BCs are
    /// collected and returned. Violations must reference both BC IDs and the stale
    /// versions from the story's citation table.
    ///
    /// RED GATE (F-P4-015): current test calls `run_arm_a2` → host::read_file →
    /// CapabilityDenied → fail-closed violations that do NOT contain stale-version info.
    /// The assertions `combined.contains("v1.17")` and `combined.contains("v1.5")` FAIL
    /// against CapabilityDenied messages (which never mention cited versions).
    ///
    /// After fix: implementer must inject BC content via `run_arm_a2_for_bc_with_result`
    /// so that real stale-citation violations are produced. Those violations carry
    /// "v1.17" and "v1.5" from the story's citation table → assertions pass.
    #[test]
    fn test_BC_5_39_010_arm_a2_two_stale_bcs_combined_block() {
        // Fixture reflects real corpus shape: BC table rows live under the
        // ## Behavioral Contracts section heading (POLICY 8 / PC13 v1.3+).
        // Citations are extracted → run_arm_a2_for_bc called → host::read_file
        // returns CapabilityDenied (non-WASM stub: -1) → fail-closed violation.
        // NOTE: CapabilityDenied violations do NOT contain stale-version strings;
        // the RED GATE assertions below fail against this path.
        let story_content = "---\nbehavioral_contracts: [BC-6.26.001, BC-5.39.008]\n---\n\
            ## Behavioral Contracts\n\n\
            | BC-6.26.001 | Title | v1.17 | active |\n\
            | BC-5.39.008 | Title | v1.5 | active |\n";
        let (violations, _) = run_arm_a2("S-21.07", story_content);
        assert!(
            !violations.is_empty(),
            "two stale BCs must produce combined violations"
        );
        // RED GATE assertions (F-P4-015): require stale-citation format, not CapabilityDenied.
        // CapabilityDenied message = "host error reading BC 'BC-6.26.001' … CapabilityDenied."
        // Stale-citation message = "… cites 'BC-6.26.001' at version v1.17 … BC says 1.X."
        // The combined message must reference stale versions v1.17 and v1.5 (from the
        // story's citation table). CapabilityDenied messages omit version info → FAILS.
        let combined: String = violations
            .iter()
            .map(|v| v.description.as_str())
            .collect::<Vec<_>>()
            .join(" | ");
        assert!(
            combined.contains("v1.17"),
            "violations must reference stale citation version v1.17 for BC-6.26.001. \
            CapabilityDenied path does not carry version info → test fails until \
            refactored to inject BC content. F-P4-015 RED GATE. \
            Got combined: {combined:?}"
        );
        assert!(
            combined.contains("v1.5"),
            "violations must reference stale citation version v1.5 for BC-5.39.008. \
            CapabilityDenied path does not carry version info → test fails until \
            refactored to inject BC content. F-P4-015 RED GATE. \
            Got combined: {combined:?}"
        );
        assert!(
            combined.contains("[Class A Arm2]"),
            "violations must cite [Class A Arm2]. \
            BC-5.39.010 postcondition 7. Got combined: {combined:?}"
        );
    }

    /// PC13 bound: a story file with no `## ` headings yields zero citations.
    ///
    /// `skip_section` starts `true` unconditionally per BC-5.39.010 PC13 (amended
    /// v1.3+, F-P2-001). A file that places BC table rows directly after frontmatter
    /// (with no `## Behavioral Contracts` heading) produces zero citations — the
    /// scanner never activates. This pins the heading-free lower bound and validates
    /// that heading-free fixtures are spec-describes-imagined-shape: they do NOT match
    /// real corpus shape (POLICY 8 requires the section heading).
    ///
    /// BC trace: BC-5.39.010 PC13 v1.3+ (skip_section starts true); F-P2-001.
    #[test]
    fn test_BC_5_39_010_arm_a2_heading_free_story_yields_zero_citations() {
        // Fixture: BC table row placed directly after frontmatter close, no ## heading.
        // This shape does NOT exist in the real corpus (POLICY 8 requires the section
        // heading), but confirms that skip_section=true produces zero citations rather
        // than scanning out-of-section content.
        let content = "---\nbehavioral_contracts: [BC-6.26.001]\n---\n\
            | BC-6.26.001 | Title | v1.17 | active |\n";
        let citations = extract_story_bc_version_citations(content, "BC-6.26.001");
        assert_eq!(
            citations.len(),
            0,
            "a heading-free story file must yield zero citations: skip_section starts \
            true (BC-5.39.010 PC13, F-P2-001). No ## Behavioral Contracts heading → \
            scanner never activates → no citations regardless of row content."
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

    // -----------------------------------------------------------------------
    // F-S2107-P3-004 (HIGH carried): PC13 word-boundary token test
    //
    // `line.contains(bc_id)` is the verbatim forbidden construct in PC13.
    // Live consequences: prefix collisions ("BC-5.39.0101" contains "BC-5.39.010")
    // cause the wrong BC's version token to be extracted.
    //
    // After fix: word-boundary token test; "BC-5.39.010" NOT a match in "BC-5.39.0101"
    // because the digit '1' immediately after "010" is NOT a word boundary.
    // -----------------------------------------------------------------------

    /// F-S2107-P3-004 RED GATE: prefix-collision must NOT produce a citation.
    ///
    /// "BC-5.39.0101" contains "BC-5.39.010" as a substring. `line.contains("BC-5.39.010")`
    /// matches it → version "15.01" extracted from S-15.01 → citation produced. WRONG.
    ///
    /// After fix (word-boundary token test): '0' in "010" is followed by '1' in "0101"
    /// — NOT a word boundary → no match → empty citations.
    ///
    /// RED GATE: citations NOT empty (contains prefix-collision citation). Fails now.
    #[test]
    fn test_BC_5_39_010_arm_a2_pc13_prefix_collision_no_citation() {
        // "BC-5.39.0101" is a hypothetical BC whose ID starts with "BC-5.39.010".
        // Current contains("BC-5.39.010") matches → extracts "15.01" from S-15.01.
        let content = concat!(
            "## Behavioral Contracts\n\n",
            "| BC | Version | Status | Stories |\n",
            "|----|---------|--------|--------|\n",
            "| BC-5.39.0101 | Some other contract | draft | S-15.01 |\n",
        );
        let citations = extract_story_bc_version_citations(content, "BC-5.39.010");
        assert!(
            citations.is_empty(),
            "Row '| BC-5.39.0101 | ...' must NOT produce a citation for 'BC-5.39.010'. \
            PC13 (MUST NOT): word-boundary token test required — '0101' has '1' after '010' \
            so 'BC-5.39.010' is NOT a standalone token. \
            F-S2107-P3-004: current `line.contains(\"BC-5.39.010\")` matches → \
            extracts '15.01' from S-15.01 → citation produced. RED GATE. \
            Citations: {:?}",
            citations
        );
    }

    // -----------------------------------------------------------------------
    // F-S2107-P3-022 (MEDIUM): reverse-field algorithm not implemented.
    //
    // PC13 prescribes: split the row by `|` delimiter; iterate fields in REVERSE
    // order (right to left); return the version token from the first (rightmost)
    // field whose stripped content contains a match.
    //
    // Current code scans the ENTIRE LINE left-to-right and returns the LAST token.
    // The docstring claims this "prevents spurious matches from BC ID fragments like
    // 'BC-5.39.010' (which contains '5.39')". It does NOT: a BC-citing row with an
    // EMPTY version cell has no later version token, so "5.39" from the BC ID is the
    // last (and only) match → citation "5.39" produced. Incorrect.
    //
    // After fix: reverse-field algorithm excludes or deprioritises the BC-ID-containing
    // field (or the implementer anchors the version field by cell position), so
    // "5.39" from "BC-5.39.010" is not returned as the version.
    // -----------------------------------------------------------------------

    /// F-S2107-P3-022 RED GATE: BC ID fragment "5.39" in "BC-5.39.010" must NOT be
    /// extracted as the version when the version cell is empty.
    ///
    /// Row: `| BC-5.39.010 | description only | |`
    /// Expected: no citation (empty version cell → nothing to extract).
    /// Current: `extract_version_token_from_table_row` finds "5.39" from the BC ID
    ///          fragment → citation ("table row N", "5.39") produced. WRONG.
    ///
    /// RED GATE: citations NOT empty. Fails now.
    #[test]
    fn test_BC_5_39_010_arm_a2_bc_id_fragment_no_version_citation() {
        // BC ID "BC-5.39.010" contains "5.39" which the left-to-right scanner returns
        // when there is no other version token in the row.
        let content = concat!(
            "## Behavioral Contracts\n\n",
            "| BC | Description | Version |\n",
            "|----|-------------|--------|\n",
            "| BC-5.39.010 | description only | |\n",
        );
        let citations = extract_story_bc_version_citations(content, "BC-5.39.010");
        assert!(
            citations.is_empty(),
            "Row '| BC-5.39.010 | description only | |' (empty version cell) must produce \
            NO citation. 'BC-5.39.010' contains the substring '5.39' which the left-to-right \
            scanner returns when no other version token exists. \
            F-S2107-P3-022: BC ID fragment '5.39' must not be extracted as the version. \
            After fix (reverse-field algorithm): empty version cell → no citation. RED GATE. \
            Citations: {:?}",
            citations
        );
    }

    // -----------------------------------------------------------------------
    // BC-5.39.010 v1.9 PC13: two-phase algorithm — three collision classes.
    //
    // The v1.9 two-phase PC13 algorithm:
    //   Phase 1: scan fields right-to-left for a pure-version field
    //            (`^v?[0-9]+\.[0-9]+$`). Covers standard Version-column rows.
    //   Phase 2 fallback: scan for mandatory-v inline token
    //            (`\bv([0-9]+\.[0-9]+)\b`). Covers Token Budget `~4,000` shape rows.
    //
    // The prior OPTIONAL-v left-to-right last-token form is NON-CONFORMING:
    //   Class 1 (29 rows, 6 stories): story-ID tokens (e.g., "S-4.07") in the
    //            Scope Reason / Trace column match as "4.07" → spurious citation.
    //   Class 2 (1 row, S-21.07): DEFERRED annotation in ACs column contains
    //            "v1.6" which comes after the Version column "1.7" →
    //            last-token returns "1.6" instead of "1.7".
    //   Class 3 (Token Budget rows): bare BC-section IDs like "BC-1.13.001"
    //            → "1.13" extracted from the BC prefix → spurious citation.
    //
    // All three tests below are RED GATE: current optional-v scanner produces
    // wrong output. After two-phase fix, all three pass.
    // -----------------------------------------------------------------------

    /// BC-5.39.010 v1.9 PC13 RED GATE — Collision Class 1 (story-ID Trace column):
    /// story IDs in the Scope Reason cell must NOT be extracted as version citations.
    ///
    /// Corpus shape (S-4.07, BC-3.07.002 BC table row — 3-column reference table):
    ///   `| BC-3.07.002 | sink driver emits internal.sink_error event | S-4.10 emits
    ///    events; S-4.07 integration tests verify (AC-12) |`
    ///
    /// Old optional-v scanner: finds "3.07" (BC-ID), then "4.10" (S-4.10),
    /// then "4.07" (S-4.07) — last-token returns "4.07" → citation ("table row N", "4.07").
    ///
    /// v1.9 two-phase PC13:
    ///   Phase 1 (pure-version field): no field is `^v?[0-9]+\.[0-9]+$` → no match.
    ///   Phase 2 (mandatory-v inline): no `v`-prefixed token in row → no match.
    ///   Result: no citation.
    ///
    /// RED GATE: current code returns citation "4.07". Test FAILS.
    #[test]
    fn test_BC_5_39_010_arm_a2_pc13_class1_story_id_trace_column_not_cited() {
        // Corpus: S-4.07 story file — BC-3.07.002 row in Behavioral Contracts section.
        // The Scope Reason cell contains "S-4.10" and "S-4.07" story-ID references.
        // Old optional-v: "4.07" extracted (last token). v1.9: no citation.
        let content = concat!(
            "## Behavioral Contracts\n\n",
            "| BC ID | Title | Scope Reason |\n",
            "|---|---|---|\n",
            "| BC-3.07.002 | sink driver emits internal.sink_error event on each failure | ",
            "S-4.10 emits internal.sink_error events; S-4.07 integration tests verify ",
            "these events fire from each sink driver under failure conditions (AC-12) |\n",
        );
        let citations = extract_story_bc_version_citations(content, "BC-3.07.002");
        assert!(
            citations.is_empty(),
            "Story-ID tokens in the Scope Reason column (S-4.10, S-4.07) must NOT be \
            extracted as version citations. BC-5.39.010 v1.9 PC13 Class 1: old optional-v \
            last-token finds '4.07' from 'S-4.07' in the Trace column. Two-phase fix: \
            Phase 1 no pure-version field; Phase 2 no mandatory-v token → no citation. \
            29 rows across 6 stories (S-0.03, S-1.03, S-2.06, S-3.01, S-4.07, S-8.09) \
            exhibit this shape. Citations: {:?}",
            citations
        );
    }

    /// BC-5.39.010 v1.9 PC13 RED GATE — Collision Class 2 (ACs-column DEFERRED annotation):
    /// version from Version cell must be returned, NOT the later "v1.6" from ACs cell.
    ///
    /// Corpus shape (S-21.07, BC-5.39.010 BC table row — 4-column row):
    ///   `| BC-5.39.010 | validate-cross-site-correspondence hook | 1.7 |
    ///    AC-001 through AC-021 (DEFERRED v1.6 — Class D) |`
    ///
    /// Old optional-v last-token: scans left-to-right, finds "5.39" (BC-ID), "1.7"
    /// (Version field), then "1.6" from "v1.6" in ACs cell — last-token returns "1.6". WRONG.
    ///
    /// v1.9 two-phase PC13:
    ///   Phase 1 (pure-version field, right-to-left):
    ///     ACs field: not pure-version → skip.
    ///     Version field "1.7": matches `^v?[0-9]+\.[0-9]+$` → return "1.7". CORRECT.
    ///
    /// RED GATE: current code returns citation "1.6". Test asserts "1.7". FAILS.
    #[test]
    fn test_BC_5_39_010_arm_a2_pc13_class2_acs_column_deferred_yields_version_cell() {
        // Corpus: S-21.07 story file — BC-5.39.010 row in Behavioral Contracts section.
        // ACs column contains "DEFERRED v1.6" — old scanner returns "1.6" (last v-prefixed token).
        // v1.9 Phase 1: Version field "1.7" is a pure-version field → citation "1.7".
        let content = concat!(
            "## Behavioral Contracts\n\n",
            "| BC ID | Title | Version | Story ACs |\n",
            "|---|---|---|---|\n",
            "| BC-5.39.010 | validate-cross-site-correspondence WASM hook | 1.7 | ",
            "AC-001 through AC-021 (AC-012/013/014 DEFERRED v1.6 — Class D) |\n",
        );
        let citations = extract_story_bc_version_citations(content, "BC-5.39.010");
        assert_eq!(
            citations.len(),
            1,
            "Row with Version field '1.7' and ACs 'DEFERRED v1.6' must produce exactly 1 \
            citation. BC-5.39.010 v1.9 PC13 Class 2. Citations: {:?}",
            citations
        );
        assert_eq!(
            citations[0].1, "1.7",
            "Citation must be '1.7' (Version field), NOT '1.6' (ACs DEFERRED annotation). \
            BC-5.39.010 v1.9 PC13 Class 2: Phase 1 pure-version field (right-to-left) \
            returns the Version column '1.7' before reaching the 'DEFERRED v1.6' in ACs. \
            Old optional-v last-token incorrectly returns '1.6'. Citation: {:?}",
            citations[0]
        );
    }

    /// BC-5.39.010 v1.9 PC13 RED GATE — Collision Class 3 (Token Budget bare BC-ID):
    /// BC-section-number fragment in "BC-1.13.001" must NOT be extracted as a version.
    ///
    /// Corpus shape (S-12.03, Token Budget section row — 2-column row):
    ///   `| BC-1.13.001 | ~4,000 |`
    ///
    /// Old optional-v scanner: finds "1.13" from "BC-1.13.001" (word boundary at '-'
    /// before '1'; "1.13" matches before the trailing ".001" terminates the match at '.').
    /// Also scans "001" (but ".001" has a leading '.', and "001" starts with prev='.' which
    /// is not alphanumeric → "001" is a bare integer with no '.N' → no version shape).
    /// Final: last_match = "1.13" → citation ("table row N", "1.13"). WRONG.
    ///
    /// v1.9 two-phase PC13:
    ///   Phase 1 (pure-version field): "BC-1.13.001" not pure-version; "~4,000" not
    ///            pure-version → no match.
    ///   Phase 2 (mandatory-v inline): no v-prefixed token in row → no match.
    ///   Result: no citation. CORRECT.
    ///
    /// RED GATE: current code returns citation "1.13". Test FAILS.
    #[test]
    fn test_BC_5_39_010_arm_a2_pc13_class3_token_budget_bc_id_section_number_not_cited() {
        // Corpus: S-12.03 story file — BC-1.13.001 row in Token Budget section.
        // Old optional-v: "1.13" extracted from "BC-1.13.001" BC-section-number fragment.
        // v1.9 two-phase: Phase 1 no pure-version field; Phase 2 no mandatory-v → no citation.
        let content = concat!(
            "## Token Budget Estimate (MANDATORY)\n\n",
            "| Context Source | Estimated Tokens |\n",
            "|---|---|\n",
            "| BC-1.13.001 | ~4,000 |\n",
        );
        let citations = extract_story_bc_version_citations(content, "BC-1.13.001");
        assert!(
            citations.is_empty(),
            "BC-section-number '1.13' from 'BC-1.13.001' in a Token Budget row must NOT \
            be extracted as a version citation. BC-5.39.010 v1.9 PC13 Class 3: old \
            optional-v last-token finds '1.13' (bare digit after '-' word boundary). \
            Two-phase fix: Phase 1 no pure-version field (BC-1.13.001 is not ^v?N.N$); \
            Phase 2 no mandatory-v token → no citation. \
            Corpus: S-12.03 Token Budget row. Citations: {:?}",
            citations
        );
    }
}
