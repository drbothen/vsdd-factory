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
/// Scans the BC-INDEX.md body table for a row containing `bc_id` and extracts
/// the version cell (the cell containing a `vN.N` or `vN.NN` token). Returns
/// `None` if no row is found for this BC ID (new BC not yet registered).
///
/// Pure: operates on already-read bytes. Called from `run_arm_a1_with_index_result`.
///
/// # BC trace
/// BC-5.39.010 postconditions 1-4 (version cell matching logic).
pub fn extract_bc_index_version(bc_id: &str, index_content: &[u8]) -> Option<String> {
    // Scan BC-INDEX.md body table for a row containing bc_id and extract the version cell.
    // Version cell format: `vN.N` or `vN.NN` token in a pipe-delimited table row.
    let content = std::str::from_utf8(index_content).unwrap_or("");

    for line in content.lines() {
        // Only check pipe-delimited table rows containing bc_id
        if !line.contains('|') || !line.contains(bc_id) {
            continue;
        }
        // Extract cells from table row
        for cell in line.split('|') {
            let cell = cell.trim();
            // Look for a token matching vN.N or vN.NN (version token)
            if let Some(version) = extract_version_token(cell) {
                return Some(version);
            }
        }
    }
    None
}

/// Extract a `vN.N` or `vN.NN` version token from a string.
/// Returns the version number without the leading `v`.
fn extract_version_token(text: &str) -> Option<String> {
    // Hand-rolled: find first occurrence of 'v' followed by digits.dot.digits
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'v' && i + 1 < bytes.len() && bytes[i + 1].is_ascii_digit() {
            // Found 'v' followed by digit — extract vN.N pattern
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
}
