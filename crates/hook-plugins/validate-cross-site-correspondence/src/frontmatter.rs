//! frontmatter.rs — YAML frontmatter parsing utilities.
//!
//! Pure-core module (ADR-035 §Decision 1): no I/O.
//!
//! Extracts individual scalar fields and sequence fields from YAML frontmatter
//! blocks (between the first `---` and second `---` delimiters) using line-scanning.
//! No `serde_norway` or `serde_yaml` is used — hand-rolled line scanning is
//! sufficient for the field extractions needed by this hook (version string,
//! behavioral_contracts sequence, modified sequence, last_amended string).
//!
//! # BC trace
//! BC-5.39.010 §Architecture Anchors:
//!   - `extract_frontmatter_sequence(content, field)` — parses YAML sequence
//!   - `extract_frontmatter_field(content, field)` — single-value scalar extraction
//! BC-5.39.010 invariant 9 — UTF-8 safety: `is_char_boundary()` guards on slicing.

/// Extract a single-value scalar field from YAML frontmatter.
///
/// Scans the leading `---` … `---` region of `content` for the first line
/// matching `^<field>:` and returns the value after the colon, trimmed.
/// Handles bare, single-quoted, and double-quoted YAML values.
/// Returns `None` if the frontmatter region is absent or the field is not found.
///
/// # BC trace
/// BC-5.39.010 §Architecture Anchors `extract_frontmatter_field`; used by
/// arm_a1 (extract `version:`), arm_a2 (extract `story_id:`), arm_e (extract
/// `version:`, `last_amended:`).
pub fn extract_frontmatter_field(content: &str, field: &str) -> Option<String> {
    // Find the frontmatter region: lines between first --- and second ---
    let mut lines = content.lines();
    // First line must be ---
    if lines.next()? != "---" {
        return None;
    }
    for line in lines {
        if line == "---" {
            return None; // End of frontmatter, field not found
        }
        // Check if this line starts with `field:`
        let prefix = format!("{}:", field);
        if line.starts_with(&prefix) {
            let rest = &line[prefix.len()..];
            let trimmed = rest.trim();
            // Strip surrounding quotes (single or double)
            let value = if (trimmed.starts_with('"') && trimmed.ends_with('"'))
                || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
            {
                // Safety: trimmed is at least 2 chars if it has surrounding quotes
                if trimmed.len() >= 2 {
                    let inner = &trimmed[1..trimmed.len() - 1];
                    // Verify char boundary for multi-byte UTF-8
                    if trimmed.is_char_boundary(1) && trimmed.is_char_boundary(trimmed.len() - 1) {
                        inner
                    } else {
                        trimmed
                    }
                } else {
                    trimmed
                }
            } else {
                trimmed
            };
            return Some(value.to_string());
        }
    }
    None
}

/// Extract a YAML sequence field from YAML frontmatter.
///
/// Scans the leading `---` … `---` region of `content` for the first line
/// matching `^<field>:` and returns the items in the sequence as a Vec<String>.
///
/// Handles both inline sequences (`[item1, item2]`) and block sequences
/// (`- item` on subsequent lines).
///
/// Returns an empty Vec if the field is absent, empty, or not a sequence.
///
/// # BC trace
/// BC-5.39.010 §Architecture Anchors `extract_frontmatter_sequence`; used by
/// arm_a2 (extract `behavioral_contracts:`) and arm_e (extract `modified:`).
pub fn extract_frontmatter_sequence(content: &str, field: &str) -> Vec<String> {
    // Find the frontmatter region
    let mut lines = content.lines();
    // First line must be ---
    if lines.next().unwrap_or("") != "---" {
        return vec![];
    }

    let field_prefix = format!("{}:", field);
    let mut found_field = false;
    let mut result = Vec::new();

    for line in lines {
        if line == "---" {
            break;
        }
        if found_field {
            // We're collecting block-sequence items
            let trimmed = line.trim();
            if trimmed.starts_with("- ") {
                // Block sequence item: "  - value" or "- value"
                let item = trimmed[2..].trim();
                result.push(strip_quotes(item));
            } else if !trimmed.is_empty() && !trimmed.starts_with('-') {
                // End of block sequence (next non-empty, non-item line)
                break;
            }
            continue;
        }
        if line.starts_with(&field_prefix) {
            found_field = true;
            let rest = &line[field_prefix.len()..];
            let trimmed = rest.trim();
            if trimmed.is_empty() {
                // Block-sequence style: items on following lines
                continue;
            }
            // Inline sequence: [item1, item2] or []
            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                let inner = &trimmed[1..trimmed.len() - 1];
                if inner.trim().is_empty() {
                    return vec![];
                }
                for item in inner.split(',') {
                    let s = strip_quotes(item.trim());
                    if !s.is_empty() {
                        result.push(s);
                    }
                }
                return result;
            }
        }
    }
    result
}

/// Strip single or double surrounding quotes from a YAML scalar string.
fn strip_quotes(s: &str) -> String {
    if s.len() >= 2 {
        let first = s.as_bytes()[0];
        let last = s.as_bytes()[s.len() - 1];
        if (first == b'"' && last == b'"') || (first == b'\'' && last == b'\'') {
            if s.is_char_boundary(1) && s.is_char_boundary(s.len() - 1) {
                return s[1..s.len() - 1].to_string();
            }
        }
    }
    s.to_string()
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // extract_frontmatter_field — scalar extraction
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_5_39_010_frontmatter_field_bare_value() {
        let content = "---\nversion: 1.6\n---\nbody\n";
        let result = extract_frontmatter_field(content, "version");
        assert_eq!(result, Some("1.6".to_string()));
    }

    #[test]
    fn test_BC_5_39_010_frontmatter_field_quoted_value() {
        let content = "---\nversion: \"1.6\"\n---\nbody\n";
        let result = extract_frontmatter_field(content, "version");
        assert_eq!(result, Some("1.6".to_string()));
    }

    #[test]
    fn test_BC_5_39_010_frontmatter_field_absent_returns_none() {
        let content = "---\nstory_id: S-21.07\n---\nbody\n";
        let result = extract_frontmatter_field(content, "version");
        assert!(result.is_none(), "absent field must return None");
    }

    #[test]
    fn test_BC_5_39_010_frontmatter_field_no_frontmatter_returns_none() {
        let content = "# heading\nbody text\n";
        let result = extract_frontmatter_field(content, "version");
        assert!(result.is_none(), "content without frontmatter must return None");
    }

    #[test]
    fn test_BC_5_39_010_frontmatter_field_last_amended_with_prior_chain() {
        // BC-5.39.010 EC-018: last_amended with [Prior: ...] chain — must return full value
        let content =
            "---\nlast_amended: \"2026-07-30 (v1.6) — Active. [Prior: 2026-07-01 (v1.5) — ...]\"\n---\n";
        let result = extract_frontmatter_field(content, "last_amended");
        assert!(result.is_some(), "last_amended with Prior chain must be extracted");
        let val = result.unwrap();
        assert!(
            val.starts_with("2026-07-30 (v1.6)"),
            "extracted value must start with the outermost date prefix"
        );
    }

    // -----------------------------------------------------------------------
    // extract_frontmatter_sequence — sequence extraction
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_5_39_010_frontmatter_sequence_inline_list() {
        let content = "---\nbehavioral_contracts: [BC-5.39.010, BC-5.39.008]\n---\n";
        let result = extract_frontmatter_sequence(content, "behavioral_contracts");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "BC-5.39.010");
        assert_eq!(result[1], "BC-5.39.008");
    }

    #[test]
    fn test_BC_5_39_010_frontmatter_sequence_block_list() {
        let content = "---\nbehavioral_contracts:\n  - BC-5.39.010\n  - BC-5.39.008\n---\n";
        let result = extract_frontmatter_sequence(content, "behavioral_contracts");
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_BC_5_39_010_frontmatter_sequence_empty_list() {
        let content = "---\nbehavioral_contracts: []\n---\n";
        let result = extract_frontmatter_sequence(content, "behavioral_contracts");
        assert!(result.is_empty(), "empty sequence must return empty Vec");
    }

    #[test]
    fn test_BC_5_39_010_frontmatter_sequence_absent_field() {
        let content = "---\nstory_id: S-21.07\n---\n";
        let result = extract_frontmatter_sequence(content, "behavioral_contracts");
        assert!(result.is_empty(), "absent sequence field must return empty Vec");
    }

    #[test]
    fn test_BC_5_39_010_frontmatter_modified_sequence_with_annotations() {
        // BC-5.39.010 EC-016: modified entries with annotation suffixes
        let content = "---\nmodified:\n  - \"2026-05-14 v1.0\"\n  - \"2026-05-18 (v1.1)\"\n---\n";
        let result = extract_frontmatter_sequence(content, "modified");
        assert_eq!(result.len(), 2);
    }
}
