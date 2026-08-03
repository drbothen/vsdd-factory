//! arm_d.rs — Class D: finding-ID namespace format advisory.
//!
//! Pure-core module (ADR-035 §Decision 1): operates on already-read content.
//! ALWAYS returns `HookResult::Continue` — Class D is advisory-only (BC-5.39.010
//! invariant 6). Advisories are emitted via `host::log_warn` in `lib.rs`.
//!
//! # Behavior
//! Fires on cycle artifact writes (burst-log.md, lessons.md, INDEX.md). Extracts
//! ONLY the scope-limited region of the artifact (positional anchor, NOT pattern
//! match). Scans for tokens matching the "finding-like" shape:
//!   `[A-Za-z][A-Za-z0-9-]*[0-9]+` (starts with letter, ends with digit)
//! that appear in `Closes:` / `Refs:` lines. Tokens in the exclusion list and
//! tokens starting with `F-` are not flagged.
//!
//! # Scope-limited extraction (BC-5.39.010 precondition 30)
//! - `burst-log.md`: LAST H2 section (from last `^## ` heading to EOF).
//! - `lessons.md`: LAST `^L-EDP1-[0-9]+-[0-9]+:` anchor block; fallback last 200 lines.
//! - `INDEX.md`: `## Adversarial Reviews` section ONLY.
//!
//! Exclusion is by STRUCTURAL POSITION (last section only), NOT by pattern-matching
//! on section names. Using a section-name pattern as exclusion would create an escape
//! channel — the failure mode from pass-29 `F-S2104-P29-H02`.
//!
//! # Known exclusion list (BC-5.39.010 precondition 32)
//! Prefixes: `D-`, `S-`, `BC-`, `VP-`, `R-`, `L-`, `ADR-`, `EC-`, `NFR-`, `ASM-`, `FM-`
//!
//! # Honest Gap (Story AC-013 §Gap)
//! Class D checks syntactic format (does the token start with `F-`?) ONLY. It does NOT
//! verify semantic existence (does `F-S2104-P99-H99` name a real finding in any pass
//! record?). A well-formed but fabricated ID passes Class D cleanly. Semantic existence
//! validation is routed to bats Tier 3 per ADR-035 §Decision 4.
//!
//! # BC trace
//! BC-5.39.010 preconditions 28-33; postconditions 16-18; invariant 6.

use crate::Advisory;
use crate::dispatch::CycleArtifactKind;

/// The known-safe prefix exclusion list (BC-5.39.010 precondition 32).
///
/// Tokens that start with any of these prefixes are NOT finding-like and MUST NOT
/// trigger an advisory, even if they otherwise match the `[A-Za-z][A-Za-z0-9-]*[0-9]+` shape.
pub const EXCLUDED_PREFIXES: &[&str] = &[
    "D-", "S-", "BC-", "VP-", "R-", "L-", "ADR-", "EC-", "NFR-", "ASM-", "FM-",
];

/// Extract the scope-limited region from a cycle artifact's content.
///
/// Returns a sub-slice of `content` representing only the region that should be
/// scanned by Arm D. Extraction is by STRUCTURAL POSITION, not by pattern match.
///
/// - `BurstLog`: returns content from the start of the LAST `^## ` heading to EOF.
/// - `Lessons`: returns the LAST `^L-EDP1-[0-9]+-[0-9]+:` anchor block; if absent,
///   returns the last 200 lines.
/// - `CycleIndex`: returns the `## Adversarial Reviews` section (from that heading
///   to the next `## ` heading or EOF).
///
/// Returns an empty string if the structural anchor is not found and no fallback applies.
///
/// # BC trace
/// BC-5.39.010 precondition 30 (scope-limited extraction); AC-014.
pub fn extract_scope_limited_region(content: &str, kind: CycleArtifactKind) -> &str {
    match kind {
        CycleArtifactKind::BurstLog => {
            // Return content from start of LAST `## ` heading to EOF.
            // Structural position: last H2 heading, not pattern-match on section name.
            let mut last_h2_byte_offset: Option<usize> = None;
            let mut search_pos = 0;
            while let Some(rel) = content[search_pos..].find("\n## ") {
                last_h2_byte_offset = Some(search_pos + rel + 1); // +1 to skip '\n'
                search_pos += rel + 1;
            }
            // Also check if content starts with ## at position 0
            if content.starts_with("## ") {
                // There might be no '\n## ' but the very beginning is an H2
                // We already handle this if we look for "## " at start
                if last_h2_byte_offset.is_none() {
                    last_h2_byte_offset = Some(0);
                }
                // If there IS a '\n## ' later, we already found it; no need to override
            }
            match last_h2_byte_offset {
                Some(offset) if content.is_char_boundary(offset) => &content[offset..],
                _ => "",
            }
        }
        CycleArtifactKind::Lessons => {
            // Return LAST line that contains `L-EDP1-[0-9]+-[0-9]+:` anchor block.
            // Handles both bare `L-EDP1-NNN-NNN:` and heading `## L-EDP1-NNN-NNN:` forms.
            // Fallback: last 200 lines.
            let mut last_anchor_offset: Option<usize> = None;

            // Scan all lines for L-EDP1 anchor pattern
            let mut byte_offset: usize = 0;
            for line in content.lines() {
                if let Some(marker_pos) = line.find("L-EDP1-") {
                    // Found "L-EDP1-" on this line — validate [0-9]+: format
                    // Actual anchor format: L-EDP1-NNN: (e.g. L-EDP1-001:)
                    let after_prefix = &line[marker_pos + 7..]; // skip "L-EDP1-"
                    if let Some(colon_pos) = after_prefix.find(':') {
                        let between = &after_prefix[..colon_pos];
                        // Accept: one or more digit groups separated by dashes
                        // e.g. "001" or "001-062"
                        let all_digit_groups = between.split('-').all(|part| {
                            !part.is_empty() && part.bytes().all(|b| b.is_ascii_digit())
                        });
                        if all_digit_groups && !between.is_empty() {
                            last_anchor_offset = Some(byte_offset);
                        }
                    }
                }
                byte_offset += line.len() + 1; // +1 for '\n'
            }

            match last_anchor_offset {
                Some(offset) => {
                    if content.is_char_boundary(offset) {
                        &content[offset..]
                    } else {
                        ""
                    }
                }
                None => {
                    // Fallback: last 200 lines
                    let lines: Vec<&str> = content.lines().collect();
                    let total = lines.len();
                    if total <= 200 {
                        content
                    } else {
                        let skip = total - 200;
                        // Find byte offset of the line at `skip`
                        let mut byte_offset = 0;
                        for (i, line) in content.lines().enumerate() {
                            if i == skip {
                                break;
                            }
                            byte_offset += line.len() + 1; // +1 for '\n'
                        }
                        if byte_offset <= content.len() && content.is_char_boundary(byte_offset) {
                            &content[byte_offset..]
                        } else {
                            content
                        }
                    }
                }
            }
        }
        CycleArtifactKind::CycleIndex => {
            // Return `## Adversarial Reviews` section only (from that heading
            // to the next `## ` heading or EOF).
            let target = "## Adversarial Reviews";
            let start = if let Some(pos) = content.find(target) {
                pos
            } else if let Some(pos) = content.find("\n## Adversarial Reviews") {
                pos + 1 // skip the leading \n
            } else {
                return "";
            };

            // Find the next ## heading after start
            let after_start = start + target.len();
            match content[after_start..].find("\n## ") {
                Some(rel_end) => {
                    let end = after_start + rel_end;
                    if content.is_char_boundary(start) && content.is_char_boundary(end) {
                        &content[start..end]
                    } else {
                        ""
                    }
                }
                None => {
                    if content.is_char_boundary(start) {
                        &content[start..]
                    } else {
                        ""
                    }
                }
            }
        }
    }
}

/// Returns `true` if `token` starts with one of the EXCLUDED_PREFIXES.
///
/// Per BC-5.39.010 precondition 32, tokens starting with excluded prefixes are
/// known-safe namespace tokens and MUST NOT trigger advisories.
///
/// Pure: no I/O; all ASCII prefix comparisons.
///
/// # BC trace
/// BC-5.39.010 precondition 32 (exclusion list); postcondition 16 (known-safe tokens).
pub fn is_excluded_namespace(token: &str) -> bool {
    EXCLUDED_PREFIXES
        .iter()
        .any(|prefix| token.starts_with(prefix))
}

/// Returns `true` if `token` matches the finding-like shape.
///
/// A token is finding-like (per BC-5.39.010 precondition 31) if ALL of:
/// 1. First character is ASCII letter.
/// 2. Last character is ASCII digit.
/// 3. Contains only `[A-Za-z0-9-]` characters.
///
/// Bare numerics (`001`), all-alpha tokens, and tokens with punctuation other
/// than `-` do NOT match.
///
/// Pure: no I/O; hand-rolled ASCII check (no regex).
///
/// # BC trace
/// BC-5.39.010 precondition 31 (finding-like shape); postcondition 17.
pub fn is_finding_like(token: &str) -> bool {
    if token.is_empty() {
        return false;
    }
    let bytes = token.as_bytes();
    // First character must be ASCII letter
    if !bytes[0].is_ascii_alphabetic() {
        return false;
    }
    // Last character must be ASCII digit
    if !bytes[bytes.len() - 1].is_ascii_digit() {
        return false;
    }
    // All characters must be [A-Za-z0-9-]
    bytes
        .iter()
        .all(|b| b.is_ascii_alphanumeric() || *b == b'-')
}

/// Advisory message for a non-F- finding-like token.
///
/// Returns the canonical advisory message text as specified in BC-5.39.010
/// postcondition 17. The message must match exactly:
/// `"validate-cross-site-correspondence [Class D] advisory: non-canonical
/// finding-ID token '<token>' on line '<line>' in <section> of <file>.
/// Finding IDs must start with 'F-'. Verify this is not a phantom ID or
/// retracted reference."`
///
/// # BC trace
/// BC-5.39.010 postcondition 17 (exact advisory message text).
pub fn class_d_advisory_message(token: &str, line: &str, section: &str, file: &str) -> String {
    format!(
        "validate-cross-site-correspondence [Class D] advisory: non-canonical \
        finding-ID token '{token}' on line '{line}' in {section} of {file}. \
        Finding IDs must start with 'F-'. Verify this is not a phantom ID or \
        retracted reference."
    )
}

/// Class D Arm: scan the scope-limited region of a cycle artifact for
/// non-canonical finding-ID tokens in Closes/Refs lines.
///
/// Returns a `Vec<Advisory>` — one advisory per non-canonical token found.
/// ALWAYS results in `HookResult::Continue` (invariant 6): the caller never
/// blocks on Class D findings.
///
/// Algorithm:
/// 1. Scan lines in `scoped_region` for lines containing `Closes:` or `Refs:`.
/// 2. Extract space/comma-separated tokens.
/// 3. For each token:
///    a. Skip if not finding-like.
///    b. Skip if starts with excluded prefix.
///    c. Skip if starts with `F-`.
///    d. Otherwise: append advisory.
///
/// # BC trace
/// BC-5.39.010 preconditions 28-33; postconditions 16-18; invariant 6.
pub fn run_arm_d(scoped_region: &str, file_path: &str) -> Vec<Advisory> {
    let mut advisories = Vec::new();

    for line in scoped_region.lines() {
        // Only process lines that contain Closes: or Refs:
        let lower = line.to_ascii_lowercase();
        if !lower.contains("closes:") && !lower.contains("refs:") {
            continue;
        }

        // Find the colon position and take tokens after it
        // Find the first occurrence of "closes:" or "refs:" (case-insensitive)
        let after_colon = if let Some(pos) = lower.find("closes:") {
            &line[pos + 7..]
        } else if let Some(pos) = lower.find("refs:") {
            &line[pos + 5..]
        } else {
            continue;
        };

        // Split by whitespace and commas to extract tokens
        for raw_token in after_colon.split(|c: char| c == ',' || c.is_whitespace()) {
            let token = raw_token.trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '-');
            if token.is_empty() {
                continue;
            }
            // Skip if not finding-like
            if !is_finding_like(token) {
                continue;
            }
            // Skip if excluded namespace
            if is_excluded_namespace(token) {
                continue;
            }
            // Skip if starts with F-
            if token.starts_with("F-") {
                continue;
            }
            // This token is finding-like, not excluded, and not F- → advisory
            let msg = class_d_advisory_message(token, line, "Closes/Refs section", file_path);
            advisories.push(Advisory { message: msg });
        }
    }
    advisories
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // is_excluded_namespace — BC-5.39.010 precondition 32
    // -----------------------------------------------------------------------

    /// AC-012: D- prefix is excluded → Continue, no advisory.
    #[test]
    fn test_BC_5_39_010_class_d_excluded_namespace_d944_passes() {
        assert!(
            is_excluded_namespace("D-944"),
            "D-944 must be excluded (D- prefix)"
        );
    }

    /// AC-012: S-, BC-, VP-, L-, ADR-, FM- prefixes are all excluded.
    #[test]
    fn test_BC_5_39_010_class_d_excluded_namespace_s_bc_vp_passes() {
        assert!(
            is_excluded_namespace("S-21.03"),
            "S- prefix must be excluded"
        );
        assert!(
            is_excluded_namespace("BC-5.39.010"),
            "BC- prefix must be excluded"
        );
        assert!(
            is_excluded_namespace("VP-091"),
            "VP- prefix must be excluded"
        );
        assert!(
            is_excluded_namespace("L-EDP1-052"),
            "L- prefix must be excluded"
        );
        assert!(
            is_excluded_namespace("ADR-035"),
            "ADR- prefix must be excluded"
        );
        assert!(
            is_excluded_namespace("FM-001"),
            "FM- prefix must be excluded"
        );
    }

    #[test]
    fn test_BC_5_39_010_class_d_non_excluded_prefix_not_excluded() {
        assert!(!is_excluded_namespace("B01"), "B01 has no excluded prefix");
        assert!(
            !is_excluded_namespace("P45-001"),
            "P45 has no excluded prefix"
        );
    }

    // -----------------------------------------------------------------------
    // is_finding_like — BC-5.39.010 precondition 31
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_5_39_010_class_d_finding_like_b01() {
        assert!(is_finding_like("B01"), "B01 is finding-like");
    }

    #[test]
    fn test_BC_5_39_010_class_d_finding_like_f_prefixed() {
        assert!(
            is_finding_like("F-S2104-P29-H01"),
            "F-S2104-P29-H01 is finding-like"
        );
    }

    #[test]
    fn test_BC_5_39_010_class_d_bare_numeric_not_finding_like() {
        // EC-012: bare numeric like "001" starts with digit → not finding-like
        assert!(
            !is_finding_like("001"),
            "bare numeric must not be finding-like"
        );
    }

    #[test]
    fn test_BC_5_39_010_class_d_all_alpha_not_finding_like() {
        assert!(
            !is_finding_like("Closes"),
            "all-alpha token must not be finding-like"
        );
    }

    // -----------------------------------------------------------------------
    // extract_scope_limited_region — BC-5.39.010 precondition 30
    // -----------------------------------------------------------------------

    /// AC-014: historical section excluded — only last H2 section scanned in burst-log.
    #[test]
    fn test_BC_5_39_010_class_d_historical_section_excluded() {
        // lessons.md with old L-EDP1-001 containing P45-001 (non-F-)
        // and a NEW L-EDP1-062 block with clean Closes: F-xxx lines
        let content = "## L-EDP1-001: old lesson\n\
            **Closes:** P45-001\n\n\
            ## L-EDP1-062: new lesson\n\
            **Closes:** F-S2104-P30-H01\n";
        let region = extract_scope_limited_region(content, CycleArtifactKind::Lessons);
        // The scoped region must be the last L-EDP1 block only (L-EDP1-062)
        // P45-001 in the old block must NOT appear in the scoped region
        assert!(
            !region.contains("P45-001"),
            "historical section with P45-001 must NOT be in scoped region"
        );
        assert!(
            region.contains("F-S2104-P30-H01"),
            "latest L-EDP1 block must be in scoped region"
        );
    }

    #[test]
    fn test_BC_5_39_010_class_d_burst_log_last_h2_extracted() {
        let content = "## Pass 29 Fix Burst\n\
            Some content with P-OLD-001\n\n\
            ## Pass 30 Fix Burst\n\
            **Closes:** F-S2104-P30-H01\n";
        let region = extract_scope_limited_region(content, CycleArtifactKind::BurstLog);
        assert!(
            !region.contains("P-OLD-001"),
            "previous H2 content must not be in scoped region"
        );
        assert!(
            region.contains("F-S2104-P30-H01"),
            "last H2 content must be in scoped region"
        );
    }

    // -----------------------------------------------------------------------
    // run_arm_d — BC-5.39.010 postconditions 16-18
    // -----------------------------------------------------------------------

    /// AC-013 MUTANT: non-F- token B01 triggers advisory (not block — invariant 6).
    #[test]
    fn test_BC_5_39_010_class_d_non_f_token_b01_advisory() {
        // Closes: B01, F-S2104-P29-H01 → advisory for B01, Continue for F- token
        let region = "**Closes:** B01, F-S2104-P29-H01\n";
        let advisories = run_arm_d(region, "burst-log.md");
        assert!(
            !advisories.is_empty(),
            "non-F- token B01 must produce an advisory"
        );
        assert!(
            advisories[0].message.contains("B01"),
            "advisory must cite the offending token"
        );
        assert!(
            advisories[0].message.contains("[Class D]"),
            "advisory must cite [Class D]"
        );
        // Exactly 1 advisory — F- token must NOT trigger one
        assert_eq!(
            advisories.len(),
            1,
            "only B01 must produce an advisory; F- token must not"
        );
    }

    /// AC-013 CONTROL: only F- prefix tokens → Continue, no advisory.
    #[test]
    fn test_BC_5_39_010_class_d_all_f_prefix_passes() {
        let region = "**Closes:** F-S2104-P29-H01, F-S2104-P29-H02\n";
        let advisories = run_arm_d(region, "burst-log.md");
        assert!(
            advisories.is_empty(),
            "all F- tokens must produce no advisories"
        );
    }

    /// EC-024: D-944 in Refs → Continue, no advisory (D- excluded).
    #[test]
    fn test_BC_5_39_010_class_d_refs_d944_no_advisory() {
        let region = "**Refs:** D-944\n";
        let advisories = run_arm_d(region, "burst-log.md");
        assert!(
            advisories.is_empty(),
            "D-944 in Refs must produce no advisory"
        );
    }

    /// Over-broad exclusion mutant for Class D scope-limited extraction (lessons.md).
    ///
    /// A bad implementation might use pattern-match exclusion on section names
    /// (e.g., "skip ## Changelog sections") instead of positional anchoring on
    /// the LAST L-EDP1 entry. Such an implementation would miss a non-F- token
    /// in the LATEST L-EDP1 block if that block happened to follow something
    /// that matched the exclusion pattern.
    ///
    /// The LATEST L-EDP1 block MUST always be in the scoped region.
    ///
    /// BC-5.39.010 precondition 30: scope by structural position (last L-EDP1 anchor).
    /// Derived from canonical test vector: D — phantom + historical-excluded rows.
    #[test]
    fn test_BC_5_39_010_class_d_latest_lessons_entry_non_f_advisory() {
        // Historical L-EDP1-001 block (excluded positionally) contains P45-001.
        // Latest L-EDP1-062 block (in scope) contains B01 (non-F- → must fire advisory).
        let content = "L-EDP1-001:\n\
            **Closes:** F-S2104-P29-H01\n\n\
            L-EDP1-062:\n\
            **Closes:** B01, F-S2104-P30-H01\n";
        let region = extract_scope_limited_region(content, CycleArtifactKind::Lessons);
        let advisories = run_arm_d(region, ".factory/cycles/v1.0-pass-1/lessons.md");
        // extract_scope_limited_region and run_arm_d are todo!() → panics → test FAILS
        // When implemented: region = L-EDP1-062 block only; B01 must trigger advisory
        assert!(
            !advisories.is_empty(),
            "non-F- token B01 in latest L-EDP1 block must produce advisory — \
            latest block is always in scoped region regardless of historical content"
        );
        assert!(
            advisories.iter().any(|a| a.message.contains("B01")),
            "advisory must cite the offending token B01"
        );
        assert!(
            !advisories.iter().any(|a| a.message.contains("P45-001")),
            "historical P45-001 (outside scope window) must NOT produce advisory"
        );
    }

    /// Vacuity guard: prose containing finding-like tokens NOT in Closes:/Refs: lines
    /// must not trigger advisories. Class D scans ONLY lines matching Closes:/Refs:.
    ///
    /// BC-5.39.010 precondition 31 (extraction only from Closes:/Refs: lines).
    #[test]
    fn test_BC_5_39_010_class_d_no_closes_refs_no_advisory() {
        // P45-001 appears in prose and a list item, but NOT in a Closes:/Refs: line
        let region = "## Fix Burst\n\
            This burst closes P45-001 as described above.\n\
            - Detail: P45-001 resolved\n";
        let advisories = run_arm_d(region, ".factory/cycles/v1.0-pass-1/burst-log.md");
        // run_arm_d is todo!() → panics → test FAILS (RED Gate confirmed)
        // When implemented: no Closes:/Refs: lines → no advisories (precondition 31)
        assert!(
            advisories.is_empty(),
            "tokens not in Closes:/Refs: lines must not produce advisories \
            (BC-5.39.010 precondition 31)"
        );
    }
}
