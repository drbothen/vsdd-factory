//! arm_a1.rs — Class A Arm1: BC frontmatter version vs BC-INDEX.md row version.
//!
//! Effectful-shell module (ADR-035 §Decision 1): reads BC-INDEX.md via
//! `host::read_file` as a secondary read.
//!
//! # Behavior
//! Fires when a BC file is written. Reads BC-INDEX.md and compares the version
//! cell in the table row for this BC's ID against the BC frontmatter `version:`
//! field. Blocks if the INDEX row is stale (present but different), advisory-only
//! if the BC is new (v1.0 and not yet in the INDEX).
//!
//! # Fail-closed (BC-5.39.008 v1.6 / BC-5.39.010 invariant 4/5)
//! - `CapabilityDenied` on BC-INDEX.md → Block (sandbox misconfiguration).
//! - `NotFound` on BC-INDEX.md → Advisory + Continue (bootstrap ordering).
//! - Primary target (BC file) read failure → Block always (invariant 4).
//!   Primary target failure is handled in `lib.rs` before dispatching here.
//!
//! # BC trace
//! BC-5.39.010 preconditions 1-8; postconditions 1-6; invariants 4-5.

use crate::{Advisory, Violation};
use vsdd_hook_sdk::host::HostError;

/// Read cap for BC-INDEX.md secondary read.
/// BC-5.39.010 AC-019 precondition: `max_bytes = 1048576`, `timeout_ms = 3000`.
pub const BC_INDEX_MAX_BYTES: u32 = 1_048_576;
pub const BC_INDEX_TIMEOUT_MS: u32 = 3_000;

/// Read cap for BC files (primary target and secondary BC reads in Arm A2).
/// BC-5.39.010 AC-019 precondition: `max_bytes = 524288`, `timeout_ms = 3000`.
pub const BC_MAX_BYTES: u32 = 524_288;
pub const BC_TIMEOUT_MS: u32 = 3_000;

/// Derive a deterministic BC file path from a BC ID string.
///
/// Algorithm (BC-5.39.010 precondition 11 `derive_bc_path`):
///   `BC-<section>.<subsection>.<seq>` → `.factory/specs/behavioral-contracts/ss-<section_zero_padded>/<BC-ID>.md`
///
/// Example: `BC-5.39.010` → `.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md`
///
/// No `read_dir`, no `glob`, no filesystem enumeration — deterministic derivation only.
///
/// # BC trace
/// BC-5.39.010 precondition 11; Architecture Compliance Rule (derive_bc_path).
pub fn derive_bc_path(bc_id: &str) -> String {
    // Algorithm: BC-<section>.<subsection>.<seq>
    // → .factory/specs/behavioral-contracts/ss-<section_zero_padded>/<BC-ID>.md
    // Example: BC-5.39.010 → .factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md
    //          BC-6.26.001 → .factory/specs/behavioral-contracts/ss-06/BC-6.26.001.md

    // Parse the section number from "BC-<section>.<rest>"
    let section_str = bc_id
        .strip_prefix("BC-")
        .and_then(|s| s.split('.').next())
        .unwrap_or("00");

    // Zero-pad section number to 2 digits
    let section_num: u32 = section_str.parse().unwrap_or(0);
    let section_padded = format!("{section_num:02}");

    format!(".factory/specs/behavioral-contracts/ss-{section_padded}/{bc_id}.md")
}

/// Extract the version cell from BC-INDEX.md for the given BC ID.
///
/// Scans the BC-INDEX.md body table for a row whose **first pipe-cell** contains
/// `bc_id`, then extracts the last version token (`vN.N` or `vN.NN`) across all
/// pipe-cells in that row. Returns `None` if no matching first-cell row is found
/// (new BC not yet registered).
///
/// First-cell anchoring (F-P2-002): matching on any cell (`line.contains(bc_id)`)
/// would pick up cross-reference rows whose non-first cells mention `bc_id` (e.g.,
/// a "depends on BC-X.YY.ZZZ" note in the Title column). Combined with last-wins
/// semantics, that overwrites the correct own-row version. The fix uses `splitn(3,
/// '|')` to extract the first cell and checks `first_cell.contains(bc_id)`, which
/// handles both plain `BC-1.17.001` and markdown-linked `[BC-1.17.001](path)` forms.
///
/// Pure: operates on already-read bytes. Called from `run_arm_a1_with_index_result`.
///
/// # BC trace
/// BC-5.39.010 postconditions 1-4 (version cell matching logic);
/// F-P2-002 (first-cell anchor — cross-reference row must not win over own row).
pub fn extract_bc_index_version(bc_id: &str, index_content: &[u8]) -> Option<String> {
    // Scan BC-INDEX.md body table for a row containing bc_id and extract the version cell.
    // Version cell format: `vN.N` or `vN.NN` token in a pipe-delimited table row.
    //
    // F-S2107-P1B-007: use starts_with('|') so YAML frontmatter lines (changelog entries
    // containing bc_id and '|') are skipped. Sibling precedent: arm_b::parse_story_index_catalog_hash.
    //
    // F-S2107-P1B-006: return the LAST version token found across all cells, not the first.
    // Production BC-INDEX rows use escaped-pipe chains (`v1.3 \| v1.4 \| ... \| v1.6`);
    // split('|') yields each segment, and the current version is always the LAST token.
    let content = std::str::from_utf8(index_content).unwrap_or("");
    let mut last_version: Option<String> = None;

    for line in content.lines() {
        // Only check body table rows (starts_with '|' — skips frontmatter lines)
        if !line.starts_with('|') {
            continue;
        }
        // F-P2-002: anchor on the FIRST pipe-cell only.
        // `line.contains(bc_id)` would match any row that cites bc_id anywhere
        // (e.g., a cross-reference in the Title or Depends column). Combined with
        // LAST-wins semantics, that overrides the correct own-row version.
        // splitn(3, '|') on "| BC-1.17.001 | ..." yields:
        //   ["", " BC-1.17.001 ", " rest..."]
        // so segments[1].trim() is the first-cell content.
        // Handles both plain "BC-1.17.001" and markdown-linked "[BC-1.17.001](path)" forms
        // by using contains() rather than equality.
        let mut segments = line.splitn(3, '|');
        let _ = segments.next(); // leading empty segment (before first '|')
        let first_cell = segments.next().map(|s| s.trim()).unwrap_or("");
        if !first_cell.contains(bc_id) {
            continue;
        }
        // Iterate ALL pipe-separated segments; last version token wins.
        // (Escaped-pipe version chains: "v1.3 \| v1.4 \| v1.7" — last is current.)
        for cell in line.split('|') {
            let cell = cell.trim();
            if let Some(version) = extract_version_token(cell) {
                last_version = Some(version);
            }
        }
    }
    last_version
}

/// Extract a `vN.N` or `vN.NN` version token from a string.
/// Returns the version number without the leading `v`.
///
/// F-P1B-014: leading word-boundary check (parity with arm_a2 sibling
/// `extract_version_token_from_table_row`). Tokens like `rev1.5` or `Nov1.6`
/// must NOT yield a version because the preceding character is alphanumeric.
fn extract_version_token(text: &str) -> Option<String> {
    // Hand-rolled: find occurrence of 'v' at word boundary followed by digits.dot.digits
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'v' && i + 1 < bytes.len() && bytes[i + 1].is_ascii_digit() {
            // Word boundary at start: preceding char must not be alphanumeric
            let prev_ok = i == 0 || !bytes[i - 1].is_ascii_alphanumeric();
            if !prev_ok {
                i += 1;
                continue;
            }
            // Found 'v' at word boundary followed by digit — extract vN.N pattern
            let start = i + 1; // skip 'v'
            let mut end = start;
            // Consume digits
            while end < bytes.len() && bytes[end].is_ascii_digit() {
                end += 1;
            }
            // Check for '.' followed by digits
            if end < bytes.len() && bytes[end] == b'.' {
                end += 1; // skip '.'
                if end < bytes.len() && bytes[end].is_ascii_digit() {
                    while end < bytes.len() && bytes[end].is_ascii_digit() {
                        end += 1;
                    }
                    // Check boundary: next char must be non-alphanumeric (word boundary)
                    let next_ok = end >= bytes.len() || !bytes[end].is_ascii_alphanumeric();
                    if next_ok {
                        // version_start..end is ASCII (digits and dot), safe byte slice
                        return Some(text[start..end].to_string());
                    }
                }
            }
        }
        i += 1;
    }
    None
}

/// Class A Arm1 check with the BC-INDEX.md read result provided as a seam.
///
/// This is the pure-seam entry point for unit testing: the caller provides the
/// host read result rather than performing a live `host::read_file` call. The
/// effectful wrapper `run_arm_a1` (used in `on_post_tool_use`) calls this after
/// performing the real read.
///
/// Returns `(violations, advisories)`:
/// - Block violation: INDEX row present with stale version.
/// - Block violation: BC version > 1.0 and no INDEX row (previous registration
///   existed but was dropped — postcondition 4).
/// - Advisory: BC version == "1.0" and no INDEX row (new BC not yet registered —
///   postcondition 3).
/// - Block violation: `CapabilityDenied` on BC-INDEX.md (invariant 5).
/// - Advisory: `NotFound` on BC-INDEX.md (precondition 8, invariant 5 second clause).
///
/// # BC trace
/// BC-5.39.010 postconditions 1-6; preconditions 1-8; invariants 4-5.
pub fn run_arm_a1_with_index_result(
    bc_id: &str,
    bc_version: &str,
    _bc_file_path: &str,
    index_read_result: Result<Vec<u8>, HostError>,
) -> (Vec<Violation>, Vec<Advisory>) {
    match index_read_result {
        Err(HostError::CapabilityDenied) => {
            // BC-INDEX.md CapabilityDenied → block (invariant 5)
            let violation = Violation {
                description: format!(
                    "validate-cross-site-correspondence [Class A Arm1]: \
                    CapabilityDenied reading BC-INDEX.md for '{bc_id}' — \
                    sandbox misconfiguration. Verify read_file path_allow includes \
                    '.factory/specs/behavioral-contracts/'. POLICY 14 leg 5."
                ),
            };
            (vec![violation], vec![])
        }
        Err(HostError::NotFound) => {
            // BC-INDEX.md NotFound → advisory + Continue (bootstrap ordering)
            let advisory = Advisory {
                message: format!(
                    "validate-cross-site-correspondence [Class A Arm1] advisory: \
                    BC-INDEX.md not found when validating '{bc_id}' — \
                    bootstrap ordering: INDEX may not exist yet."
                ),
            };
            (vec![], vec![advisory])
        }
        Err(other) => {
            // Other host errors → block (fail-closed)
            let violation = Violation {
                description: format!(
                    "validate-cross-site-correspondence [Class A Arm1]: \
                    host error reading BC-INDEX.md for '{bc_id}': {other:?}. \
                    POLICY 14 leg 5."
                ),
            };
            (vec![violation], vec![])
        }
        Ok(index_bytes) => {
            match extract_bc_index_version(bc_id, &index_bytes) {
                None => {
                    // BC not in INDEX
                    // postcondition 3: v1.0 → advisory
                    // postcondition 4: v > 1.0 → block (previous registration dropped)
                    let is_v1_0 = bc_version == "1.0";
                    if is_v1_0 {
                        let advisory = Advisory {
                            message: format!(
                                "validate-cross-site-correspondence [Class A Arm1] advisory: \
                                '{bc_id}' (v{bc_version}) not yet registered in BC-INDEX.md — \
                                new BC, bootstrap ordering."
                            ),
                        };
                        (vec![], vec![advisory])
                    } else {
                        let violation = Violation {
                            description: format!(
                                "validate-cross-site-correspondence [Class A Arm1]: \
                                '{bc_id}' (v{bc_version}) has no row in BC-INDEX.md — \
                                previous registration appears to have been dropped. \
                                Add or restore the INDEX row. POLICY 14 leg 5."
                            ),
                        };
                        (vec![violation], vec![])
                    }
                }
                Some(index_version) => {
                    // Compare index version against BC frontmatter version
                    if index_version == bc_version {
                        (vec![], vec![])
                    } else {
                        let violation = Violation {
                            description: format!(
                                "validate-cross-site-correspondence [Class A Arm1]: \
                                BC-INDEX.md row for '{bc_id}' cites version v{index_version} \
                                but BC frontmatter says version {bc_version}. \
                                Update the BC-INDEX.md row to v{bc_version}. POLICY 14 leg 5."
                            ),
                        };
                        (vec![violation], vec![])
                    }
                }
            }
        }
    }
}

/// Class A Arm1 effectful entry point.
///
/// Performs the live `host::read_file` for BC-INDEX.md (with `max_bytes = 1048576`,
/// `timeout_ms = 3000`), then delegates to `run_arm_a1_with_index_result`.
///
/// Called from `on_post_tool_use` after the primary BC file content has been read.
///
/// # Implementation guide
/// ```text
/// let index_result = host::read_file(".factory/specs/behavioral-contracts/BC-INDEX.md",
///     BC_INDEX_MAX_BYTES, BC_INDEX_TIMEOUT_MS);
/// run_arm_a1_with_index_result(bc_id, bc_version, bc_file_path, index_result)
/// ```
///
/// # BC trace
/// BC-5.39.010 preconditions 7-8; invariants 4-5 (fail-closed on CapabilityDenied).
pub fn run_arm_a1(
    bc_id: &str,
    bc_version: &str,
    bc_file_path: &str,
) -> (Vec<Violation>, Vec<Advisory>) {
    let index_result = vsdd_hook_sdk::host::read_file(
        ".factory/specs/behavioral-contracts/BC-INDEX.md",
        BC_INDEX_MAX_BYTES,
        BC_INDEX_TIMEOUT_MS,
    );
    run_arm_a1_with_index_result(bc_id, bc_version, bc_file_path, index_result)
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // derive_bc_path — BC-5.39.010 precondition 11
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_5_39_010_arm_a1_bc_path_derivation_ss05() {
        // BC-5.39.010 PC11: BC-5.39.010 → ss-05
        let path = derive_bc_path("BC-5.39.010");
        assert_eq!(
            path,
            ".factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md"
        );
    }

    #[test]
    fn test_BC_5_39_010_arm_a1_bc_path_derivation_ss06() {
        // BC path derivation for single-digit section zero-padded
        let path = derive_bc_path("BC-6.26.001");
        assert_eq!(
            path,
            ".factory/specs/behavioral-contracts/ss-06/BC-6.26.001.md"
        );
    }

    // -----------------------------------------------------------------------
    // run_arm_a1_with_index_result — BC-5.39.010 postconditions 1-6
    // -----------------------------------------------------------------------

    /// AC-001 MUTANT: stale INDEX row blocks (BC-5.39.010 postcondition 2).
    #[test]
    fn test_BC_5_39_010_arm_a1_stale_index_blocks() {
        let index_content = b"| BC-5.39.010 | some title | v1.5 | 2026-07-01 | active |\n";
        let (violations, _) = run_arm_a1_with_index_result(
            "BC-5.39.010",
            "1.6",
            ".factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md",
            Ok(index_content.to_vec()),
        );
        assert!(
            !violations.is_empty(),
            "stale BC-INDEX row must produce a blocking violation"
        );
        let msg = &violations[0].description;
        assert!(
            msg.contains("[Class A Arm1]"),
            "violation must cite [Class A Arm1]"
        );
        assert!(
            msg.contains("v1.5"),
            "violation must cite stale version v1.5"
        );
        assert!(
            msg.contains("1.6"),
            "violation must cite current version 1.6"
        );
        assert!(
            msg.contains("POLICY 14 leg 5"),
            "violation must cite POLICY 14 leg 5"
        );
    }

    /// AC-001 CONTROL: current INDEX row passes (BC-5.39.010 postcondition 1).
    #[test]
    fn test_BC_5_39_010_arm_a1_current_index_passes() {
        let index_content = b"| BC-5.39.010 | some title | v1.6 | 2026-07-01 | active |\n";
        let (violations, _) = run_arm_a1_with_index_result(
            "BC-5.39.010",
            "1.6",
            ".factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md",
            Ok(index_content.to_vec()),
        );
        assert!(
            violations.is_empty(),
            "current INDEX row must produce no violations"
        );
    }

    /// AC-002: v1.0 BC not in INDEX → advisory-only (BC-5.39.010 postcondition 3).
    #[test]
    fn test_BC_5_39_010_arm_a1_new_v1_0_not_in_index_advisory() {
        // BC-INDEX has no row for BC-9.99.001 (new BC)
        let index_content = b"| BC-5.39.010 | existing | v1.6 | 2026-07-01 | active |\n";
        let (violations, advisories) = run_arm_a1_with_index_result(
            "BC-9.99.001",
            "1.0",
            ".factory/specs/behavioral-contracts/ss-09/BC-9.99.001.md",
            Ok(index_content.to_vec()),
        );
        assert!(violations.is_empty(), "v1.0 not-in-INDEX must not block");
        assert!(
            !advisories.is_empty(),
            "v1.0 not-in-INDEX must emit an advisory"
        );
    }

    /// AC-002: v1.1 BC not in INDEX → block (BC-5.39.010 postcondition 4).
    #[test]
    fn test_BC_5_39_010_arm_a1_v1_1_not_in_index_blocks() {
        // Version > 1.0 means previous registration existed but was dropped
        let index_content = b"| BC-5.39.010 | existing | v1.6 | 2026-07-01 | active |\n";
        let (violations, _) = run_arm_a1_with_index_result(
            "BC-9.99.001",
            "1.1",
            ".factory/specs/behavioral-contracts/ss-09/BC-9.99.001.md",
            Ok(index_content.to_vec()),
        );
        assert!(
            !violations.is_empty(),
            "v1.1 not-in-INDEX must produce a blocking violation"
        );
    }

    /// AC-003: BC-INDEX.md CapabilityDenied → block (BC-5.39.010 invariant 5).
    #[test]
    fn test_BC_5_39_010_arm_a1_bc_index_capability_denied_blocks() {
        let (violations, _) = run_arm_a1_with_index_result(
            "BC-5.39.010",
            "1.6",
            ".factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md",
            Err(HostError::CapabilityDenied),
        );
        assert!(
            !violations.is_empty(),
            "CapabilityDenied on BC-INDEX.md must produce a blocking violation"
        );
    }

    /// AC-003: BC-INDEX.md NotFound → advisory + Continue (BC-5.39.010 precondition 8).
    #[test]
    fn test_BC_5_39_010_arm_a1_bc_index_not_found_advisory() {
        let (violations, advisories) = run_arm_a1_with_index_result(
            "BC-5.39.010",
            "1.6",
            ".factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md",
            Err(HostError::NotFound),
        );
        assert!(
            violations.is_empty(),
            "NotFound on BC-INDEX.md must not block"
        );
        assert!(
            !advisories.is_empty(),
            "NotFound on BC-INDEX.md must emit an advisory"
        );
    }

    // -----------------------------------------------------------------------
    // F-S2107-P1B-006: escaped-pipe version chain — extract_bc_index_version
    // splits on '|' (which also splits at `\|` sequences in the raw bytes),
    // then calls extract_version_token on each cell. The first cell has "v1.3"
    // and is returned immediately without scanning later cells for a higher version.
    // Production BC-INDEX rows use `v1.3 \| v1.4 \| ... \| v1.12`; current version
    // is always the LAST token. Current code returns "1.3" → "1.3" ≠ "1.12" → BLOCK.
    //
    // F-S2107-P1B-007: frontmatter changelog pipe false-match — the YAML frontmatter
    // of BC-INDEX.md contains changelog entries that reference BC IDs with `|` chars
    // in the version column. `extract_bc_index_version` scans ALL lines and matches
    // any line containing BOTH '|' AND the bc_id. A frontmatter line like:
    //   `    change: "v4.43: BC-5.39.010 v1.5|v1.6."` → matches before the body row.
    // Result: returns "4.43" (from `v4.43`) instead of "1.6" → BLOCK.
    // -----------------------------------------------------------------------

    /// T-039 (Rust unit test): escaped-pipe chain must use LAST token (F-S2107-P1B-006).
    ///
    /// BC-5.39.010 v1.3 invariant 10: when version_history has escaped-pipe delimiter,
    /// only the FINAL version token is authoritative.
    ///
    /// RED GATE: current code returns "1.3" (first token from split('|') on
    /// `v1.3 \| v1.4 \| v1.5 \| v1.6`). "1.3" ≠ "1.6" → violation → NOT empty.
    /// assert!(violations.is_empty()) FAILS → RED gate.
    /// After fix (scan all tokens, return last version found): "1.6" → empty → PASSES.
    #[test]
    fn test_BC_5_39_010_arm_a1_escaped_pipe_chain_last_token_wins() {
        // Production-shaped INDEX row: `v1.3 \| v1.4 \| v1.5 \| v1.6` (4-version chain)
        // split('|') yields cells: `""`, `" [BC-5.39.010]..."`, ..., `" v1.3 \"`, ...
        // extract_version_token on first cell "v1.3 \" → "1.3" (first match)
        // extract_version_token on last cell " v1.6 " → "1.6"
        // After fix: LAST version token is authoritative → returns "1.6"
        let index = concat!(
            "| [BC-5.39.010](ss-05/BC-5.39.010.md) | title | draft | CAP-032 | S-21.07",
            " | v1.3 \\| v1.4 \\| v1.5 \\| v1.6 |\n",
        );
        let (violations, _) = run_arm_a1_with_index_result(
            "BC-5.39.010",
            "1.6",
            ".factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md",
            Ok(index.as_bytes().to_vec()),
        );
        assert!(
            violations.is_empty(),
            "escaped-pipe version chain `v1.3 \\| v1.4 \\| v1.5 \\| v1.6`: current version \
            '1.6' is the LAST token → must not block. \
            Red Gate: current code extracts first token '1.3' → '1.3' ≠ '1.6' → violation (F-S2107-P1B-006)"
        );
    }

    /// T-039b (Rust unit test): frontmatter changelog line must not be matched as BC body row
    /// (F-S2107-P1B-007).
    ///
    /// BC-5.39.010 v1.3 invariant 10: extract_bc_index_version must scan only table body
    /// rows (after the closing `---` of YAML frontmatter), not frontmatter content.
    ///
    /// RED GATE: current code scans ALL lines. Frontmatter changelog entry
    /// `    change: "v4.43: BC-5.39.010 v1.5|v1.6."` contains both `|` and "BC-5.39.010"
    /// → matched first. extract_version_token on `    change: "v4.43: BC-5.39.010 v1.5`
    /// returns "4.43". "4.43" ≠ "1.6" → violation → violations NOT empty.
    /// assert!(violations.is_empty()) FAILS → RED gate.
    /// After fix (skip lines before body): body row `| v1.6 |` → "1.6" → empty → PASSES.
    #[test]
    fn test_BC_5_39_010_arm_a1_frontmatter_changelog_pipe_not_matched_as_table_row() {
        // YAML frontmatter has changelog line with both `|` and BC-5.39.010 → false match
        // body table row correctly has v1.6 → should pass
        let index = concat!(
            "---\n",
            "document_type: bc-index\n",
            "changelog:\n",
            "  - date: 2026-07-31\n",
            "    change: \"v4.43: BC-5.39.010 v1.5|v1.6.\"\n",
            "---\n\n",
            "| BC ID | Title | Status | Version |\n",
            "|-------|-------|--------|---------|\n",
            "| [BC-5.39.010](ss-05/BC-5.39.010.md) | title | draft | v1.6 |\n",
        );
        let (violations, _) = run_arm_a1_with_index_result(
            "BC-5.39.010",
            "1.6",
            ".factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md",
            Ok(index.as_bytes().to_vec()),
        );
        assert!(
            violations.is_empty(),
            "frontmatter changelog line `change: \"v4.43: BC-5.39.010 v1.5|v1.6.\"` must NOT \
            be matched as BC body table row. Red Gate: current code returns '4.43' from \
            frontmatter → '4.43' ≠ '1.6' → violation (F-S2107-P1B-007)"
        );
    }

    // -----------------------------------------------------------------------
    // F-P2-002 (BLOCKER): extract_bc_index_version — unanchored first-cell lookup.
    //
    // Bug: `line.contains(bc_id)` matches ANY row that mentions bc_id, not just the
    // row whose FIRST cell IS bc_id. Combined with LAST-wins semantics, a later row
    // that merely cites bc_id in a non-first cell (e.g., a cross-reference in the
    // Title or Depends column) overwrites the correct row's version.
    //
    // ORCH-VERIFIED: BC-INDEX row for BC-1.17.001 (own row) has v1.7; row for BC-2.07.001
    // mentions "BC-1.17.001" in its Title column and has v1.6. LAST-wins + contains picks
    // the later row → returns "1.6" → spurious violation on every BC-1.17.001 write.
    //
    // Fix: anchor on first pipe-cell — only rows whose first cell trims to bc_id match.
    // -----------------------------------------------------------------------

    /// F-P2-002 (BLOCKER): own-row version wins over cross-reference in later row.
    ///
    /// RED GATE: `line.contains("BC-1.17.001")` matches BOTH rows.
    /// LAST-wins picks BC-2.07.001 row's last version token "1.6" → returns Some("1.6").
    /// `assert_eq!(result, Some("1.7"))` FAILS.
    /// After fix (first-cell anchor): only BC-1.17.001's own row matches → "1.7" → PASSES.
    #[test]
    fn test_BC_5_39_010_arm_a1_cross_reference_in_later_row_own_row_version_wins() {
        // BC-INDEX with two rows:
        //   row 1: BC-1.17.001 own row — v1.7 in version column (first cell IS bc_id)
        //   row 2: BC-2.07.001 row — mentions "BC-1.17.001" in Title cell (non-first) with v1.6
        // Expected: extract_bc_index_version("BC-1.17.001", ...) → Some("1.7")
        let index = concat!(
            "---\ndocument_type: bc-index\n---\n\n",
            "| BC-1.17.001 | Title A: session-replay gate | draft | CAP-017 | S-14.03 | v1.7 |\n",
            "| BC-2.07.001 | Title B: depends on BC-1.17.001 parity | draft | CAP-018 | S-14.04 | v1.6 |\n",
        );
        let result = extract_bc_index_version("BC-1.17.001", index.as_bytes());
        assert_eq!(
            result,
            Some("1.7".to_string()),
            "extract_bc_index_version must anchor on first cell only. \
            BC-1.17.001 own row is v1.7; later BC-2.07.001 row mentions BC-1.17.001 \
            in a non-first cell with v1.6. LAST-wins + unanchored contains returns \
            '1.6' (WRONG). F-P2-002 RED GATE. Current: {:?}",
            result
        );
    }

    /// AC-019: Read cap constant values match the spec (BC-5.39.010 §AC-019).
    ///
    /// This test verifies the constant VALUES are correct per spec. It does NOT verify
    /// that these constants are actually passed to `host::read_file` (cap-PASSING).
    ///
    /// Cap-PASSING cannot be tested from the pure seam (`run_arm_a1_with_index_result`)
    /// because the host call lives inside `run_arm_a1` (effectful). The implementer MUST
    /// add a bats integration test for cap-passing once the stub is replaced:
    ///   - Fixture: BC-INDEX.md with a small `max_bytes` override in the test registry
    ///   - Verify: the arm handles a truncated or denied read without out-of-bounds panic
    ///
    /// This test PASSES in Red Gate (constants are already correctly set per spec).
    #[test]
    fn test_BC_5_39_010_AC019_bc_index_read_cap_constant_values_match_spec() {
        assert_eq!(
            BC_INDEX_MAX_BYTES, 1_048_576,
            "[AC-019] BC-INDEX max_bytes must be 1 MiB (1_048_576) per BC-5.39.010"
        );
        assert_eq!(
            BC_INDEX_TIMEOUT_MS, 3_000,
            "[AC-019] BC-INDEX timeout_ms must be 3000 per BC-5.39.010"
        );
        assert_eq!(
            BC_MAX_BYTES, 524_288,
            "[AC-019] BC file max_bytes must be 512 KiB (524_288) per BC-5.39.010"
        );
        assert_eq!(
            BC_TIMEOUT_MS, 3_000,
            "[AC-019] BC timeout_ms must be 3000 per BC-5.39.010"
        );
    }

    // -----------------------------------------------------------------------
    // F-S2107-P3-001 (BLOCKER): three-state extractor — RowPresentNoVersion
    //
    // BC-5.39.010 v1.7 PC5 (amended): extract_bc_index_version must distinguish
    // three states, not two:
    //   RowAbsent         — no line for this BC ID in the index
    //   RowPresentNoVersion — row exists but has no version-chain cell (5-column shape)
    //   Version(v)        — row exists and carries a version token
    //
    // The canonical BC-INDEX shape is 5 columns:
    //   | BC ID | Title | Status | Capability | Stories |
    // Version-chain cell (6th column) appears on only ~40 of 1983 rows.
    // The REMAINING ~1943 rows are RowPresentNoVersion — NOT an error.
    //
    // Two-state None conflation: current code returns None for BOTH RowAbsent
    // AND RowPresentNoVersion. In the None branch, version > "1.0" → BLOCK.
    // This silently blocks every write to ~1943 BCs (e.g., BC-1.01.001).
    //
    // Post-fix: RowPresentNoVersion → silent-continue (no violations, no advisory).
    // -----------------------------------------------------------------------

    /// F-S2107-P3-001 RED GATE: 5-column row (RowPresentNoVersion) with version > "1.0"
    /// must NOT block.
    ///
    /// BC-5.39.010 v1.8 PC5: COLUMN-COUNT-ANCHORED classification. After escape-aware
    /// split, count non-empty fields: exactly 5 → RowPresentNoVersion unconditionally —
    /// no token search performed. Current token-search implementation returns None
    /// (no v-prefixed token) which maps to the old RowAbsent → block path.
    /// After fix: RowPresentNoVersion → silent-continue → no violations.
    ///
    /// RED GATE: current None path → is_v1_0=false → block → violations NOT empty.
    #[test]
    fn test_BC_5_39_010_arm_a1_row_present_no_version_cell_not_blocked() {
        // 5-column canonical BC-INDEX shape: no version-chain cell.
        // BC-5.39.010 v1.8 PC5: column count alone determines state — no token search
        // is performed on any field, including story IDs in the Stories column.
        let index_content =
            b"| [BC-9.99.001](ss-09/BC-9.99.001.md) | Some title | draft | CAP-TBD | S-99.01 |\n";
        let (violations, advisories) = run_arm_a1_with_index_result(
            "BC-9.99.001",
            "1.2",
            ".factory/specs/behavioral-contracts/ss-09/BC-9.99.001.md",
            Ok(index_content.to_vec()),
        );
        assert!(
            violations.is_empty(),
            "5-column row (no version-chain cell) with bc_version='1.2' must NOT block. \
            BC-5.39.010 v1.8 PC5: column-count-anchored — exactly 5 fields → RowPresentNoVersion \
            unconditionally; no token search on any field. \
            F-S2107-P3-001 BLOCKER: current None branch treats this as RowAbsent → block. \
            Fix: escape-aware split → count fields → 5 → RowPresentNoVersion → silent-continue. \
            Violations: {:?}",
            violations
        );
        assert!(
            advisories.is_empty(),
            "RowPresentNoVersion must be fully silent — no advisory either. \
            BC-5.39.010 v1.8 PC5. Advisories: {:?}",
            advisories
        );
    }

    /// F-S2107-P3-001 RED GATE: exact BC-1.01.001 INDEX row shape must not block.
    ///
    /// Live corpus evidence (adversary pass-3 verified):
    ///   BC-INDEX row: `| [BC-1.01.001](ss-01/BC-1.01.001.md) | Registry rejects unknown
    ///   schema version | draft | CAP-TBD | S-15.01 |`
    ///   BC-1.01.001.md version: "1.2"
    ///
    /// BC-5.39.010 v1.8 PC5: column-count-anchored — 5 non-empty fields after escape-aware
    /// split → RowPresentNoVersion unconditionally. No token search on any field, including
    /// the Stories column which contains "S-15.01". Current implementation performs token
    /// search and finds no v-prefixed token → None → RowAbsent path → block.
    ///
    /// RED GATE: violations not empty. After fix: RowPresentNoVersion → silent-continue.
    #[test]
    fn test_BC_5_39_010_arm_a1_bc_1_01_001_exact_row_shape_not_blocked() {
        // Exact row shape from live BC-INDEX.md (adversary pass-3 corpus verification).
        // BC-5.39.010 v1.8 PC5: 5 fields → RowPresentNoVersion, no token search performed.
        let index_content = b"| [BC-1.01.001](ss-01/BC-1.01.001.md) | Registry rejects \
            unknown schema version | draft | CAP-TBD | S-15.01 |\n";
        let (violations, _) = run_arm_a1_with_index_result(
            "BC-1.01.001",
            "1.2",
            ".factory/specs/behavioral-contracts/ss-01/BC-1.01.001.md",
            Ok(index_content.to_vec()),
        );
        assert!(
            violations.is_empty(),
            "BC-1.01.001 canonical 5-column INDEX row with bc_version='1.2' must NOT block. \
            BC-5.39.010 v1.8 PC5: 5 fields escape-aware → RowPresentNoVersion unconditionally. \
            F-S2107-P3-001 BLOCKER. Violations: {:?}",
            violations
        );
    }

    /// F-S2107-P3-001 RED GATE (product-owner regression guard): row with S-15.01 in the
    /// Stories column MUST yield RowPresentNoVersion, NOT Version("15.01").
    ///
    /// BC-5.39.010 v1.8 PC5 (corpus stat): 194 of 1,943 five-field rows carry story IDs
    /// whose decimal fragments (e.g., "15.01") resemble version tokens. This is the single
    /// most important test in this burst — it names the exact defect the v1.8 contract was
    /// designed to eliminate. Any extractor that converts "S-15.01" → Version("15.01")
    /// is non-conforming with BC-5.39.010 v1.8 PC5.
    ///
    /// RED GATE: current None path → RowAbsent → block → violations NOT empty.
    #[test]
    fn test_BC_5_39_010_arm_a1_stories_column_s15_01_yields_row_present_no_version() {
        // Same corpus row as bc_1_01_001_exact_row_shape_not_blocked, explicitly named for
        // the S-15.01 regression guard as required by product-owner (BC-5.39.010 v1.8).
        let index_content = b"| [BC-1.01.001](ss-01/BC-1.01.001.md) | Registry rejects \
            unknown schema version | draft | CAP-TBD | S-15.01 |\n";
        let (violations, _) = run_arm_a1_with_index_result(
            "BC-1.01.001",
            "1.2",
            ".factory/specs/behavioral-contracts/ss-01/BC-1.01.001.md",
            Ok(index_content.to_vec()),
        );
        assert!(
            violations.is_empty(),
            "S-15.01 in the Stories column must yield RowPresentNoVersion, NOT Version('15.01'). \
            BC-5.39.010 v1.8 PC5 regression guard: column-count 5 → RowPresentNoVersion, \
            no token search on any field. This is the exact false-positive the v1.8 \
            column-count-anchored contract was designed to close. Violations: {:?}",
            violations
        );
    }

    /// BC-5.39.010 v1.8 PC5 escape-aware split RED GATE: 5-field row where the Stories cell
    /// contains a literal `\|` (escaped pipe) must NOT be inflated to 6+ fields.
    ///
    /// Row shape: `| [BC-9.99.002](...) | Title | active | CAP-TBD | S-1.03 \| S-2.06 |`
    ///
    /// Naive `|` split: 6 segments (the `\|` in the Stories cell creates a phantom boundary).
    /// Escape-aware split (substitute `\|` → placeholder, split on `|`, restore):
    ///   field 1: [BC-9.99.002](...)  field 2: Title  field 3: active
    ///   field 4: CAP-TBD             field 5: S-1.03 \| S-2.06
    ///   → 5 non-empty fields → RowPresentNoVersion.
    ///
    /// Current implementation (naive split + token search) returns None on this row (no
    /// v-prefixed token) → RowAbsent path → block. After fix: escape-aware 5 fields →
    /// RowPresentNoVersion → silent-continue.
    ///
    /// RED GATE: violations NOT empty.
    #[test]
    fn test_BC_5_39_010_arm_a1_escape_aware_5field_stories_pipe_not_a_version_cell() {
        // Stories cell contains `\|` which naive splitting inflates to phantom 6th field.
        // BC-5.39.010 v1.8 PC5: escape-aware split must count this as 5 fields → RowPresentNoVersion.
        let index_content =
            b"| [BC-9.99.002](ss-09/BC-9.99.002.md) | Two linked stories | active | \
            CAP-TBD | S-1.03 \\| S-2.06 |\n";
        let (violations, _) = run_arm_a1_with_index_result(
            "BC-9.99.002",
            "1.2",
            ".factory/specs/behavioral-contracts/ss-09/BC-9.99.002.md",
            Ok(index_content.to_vec()),
        );
        assert!(
            violations.is_empty(),
            "5-field row with \\| in Stories cell must NOT block. \
            BC-5.39.010 v1.8 PC5: escape-aware split → 5 non-empty fields → RowPresentNoVersion. \
            Naive split inflates to 6 fields, converting a RowPresentNoVersion row into a \
            spurious Version check. Violations: {:?}",
            violations
        );
    }

    /// BC-5.39.010 v1.8 PC5 escape-aware split GREEN regression guard: 6-field row where
    /// the Version cell contains a `\|`-separated version chain must still yield the correct
    /// version.
    ///
    /// Row shape:
    ///   `| [BC-9.99.003](...) | Title | active | CAP-TBD | S-99.01 | v1.5 \| v1.6 \| v1.7 |`
    ///
    /// Naive split: 9 segments. Escape-aware split:
    ///   field 1: [BC-9.99.003](...)  field 2: Title  field 3: active
    ///   field 4: CAP-TBD             field 5: S-99.01
    ///   field 6: v1.5 \| v1.6 \| v1.7  → ≥6 fields → extract rightmost vN.N from field 6
    ///
    /// Current token-search over naive-split segments accidentally produces the correct answer
    /// ("v1.7" is found in the rightmost phantom segment). This GREEN test is the regression
    /// guard ensuring the escape-aware implementation continues to extract "1.7" correctly.
    #[test]
    fn test_BC_5_39_010_arm_a1_escape_aware_6field_version_chain_with_pipe_regression() {
        // 6-field row: version chain uses `\|` as delimiter within the version cell.
        // The current code passes; this is a GREEN regression guard for the v1.8 fix.
        let index_content =
            b"| [BC-9.99.003](ss-09/BC-9.99.003.md) | Version chain test | active | \
            CAP-TBD | S-99.01 | v1.5 \\| v1.6 \\| v1.7 |\n";
        let (violations, _) = run_arm_a1_with_index_result(
            "BC-9.99.003",
            "1.7",
            ".factory/specs/behavioral-contracts/ss-09/BC-9.99.003.md",
            Ok(index_content.to_vec()),
        );
        assert!(
            violations.is_empty(),
            "6-field version chain row with \\| separators in version cell must extract 'v1.7'. \
            BC-5.39.010 v1.8 PC5: escape-aware split → 6 fields → Version from field 6 \
            (rightmost vN.N). This is a regression guard — the fix must NOT break this case. \
            Violations: {:?}",
            violations
        );
    }
}
