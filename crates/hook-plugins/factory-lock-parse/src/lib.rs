//! factory-lock-parse — shared library crate (D15, S-17.04).
//!
//! Exports the parse primitives extracted from `verify-factory-lock::lib`:
//!   - `LockState` — parsed `factory_lock` block (holder, locked_at, expires_at)
//!   - `LockParseError` — error variants from lock-block parsing
//!   - `parse_factory_lock(content: &str) -> Result<Option<LockState>, LockParseError>`
//!   - `extract_yaml_string_value(line: &str, key: &str) -> Option<String>`
//!   - `parse_iso8601(s: &str) -> Result<chrono::DateTime<chrono::Utc>, LockParseError>`
//!
//! Both `verify-factory-lock` and `verify-state-timestamp-refresh` depend on this
//! crate instead of each maintaining independent frontmatter scanners.
//!
//! # Architecture compliance
//!
//! - No `serde_yaml` / `serde_norway` — manual line-by-line scan (Architecture
//!   Compliance Rule 4 / ADR-025 Decision 12 §12.4).
//! - No `regex` crate — manual tokenisation only.
//! - `chrono` (workspace dep) is the only non-std dependency.
//! - Pure library: no I/O, no host calls. All functions are deterministic given input.
//!
//! # Red Gate status (S-17.04 T-1)
//!
//! STUB: function bodies are unimplemented (`todo!()`). The unit tests in this file
//! MUST FAIL until the implementer fills in the real logic in T-2 (D15).

// Allow the BC-based test naming convention (non_snake_case is workspace-allowed).
#![cfg_attr(not(kani), allow(unexpected_cfgs))]

// ---------------------------------------------------------------------------
// Error variants
// ---------------------------------------------------------------------------

/// Error variants from lock-block parsing.
///
/// Mirrors `LockCheckError` in `verify-factory-lock` but scoped to parse-only
/// concerns — no `StateReadError` or `IdentityResolutionFailed` (those are
/// plugin-level, not parse-level).
#[derive(Debug, Clone, PartialEq)]
pub enum LockParseError {
    /// The `factory_lock` block is present but malformed (missing field, empty
    /// field, unexpected inline value, or absent closing `---` delimiter).
    MalformedLockBlock(String),
}

// ---------------------------------------------------------------------------
// Parsed lock state
// ---------------------------------------------------------------------------

/// A successfully-parsed `factory_lock` block from STATE.md frontmatter.
///
/// All three sub-fields (`holder`, `locked_at`, `expires_at`) are required;
/// absence of any field routes to `MalformedLockBlock`.
#[derive(Debug, Clone, PartialEq)]
pub struct LockState {
    /// Email of the current lock holder.
    pub holder: String,
    /// ISO-8601 timestamp when the lock was acquired.
    pub locked_at: String,
    /// ISO-8601 datetime when the lock auto-expires.
    pub expires_at: String,
}

// ---------------------------------------------------------------------------
// Public API — STUB bodies (implementer fills these in T-2 / D15)
// ---------------------------------------------------------------------------

/// Scan the YAML frontmatter of STATE.md content for the `factory_lock:` block.
///
/// Reads only the region between the first and second `---\n` delimiters.
/// Uses a line-by-line scan (no YAML parser; no `regex` crate).
/// Sub-fields are indented with exactly 2 spaces under `factory_lock:`.
///
/// Returns:
/// - `Ok(None)` if the `factory_lock` key is absent (unlocked path).
/// - `Ok(Some(LockState))` if all three sub-fields are present and non-empty.
/// - `Err(MalformedLockBlock)` if the block is present but malformed.
///
/// # Red Gate
///
/// STUB — `todo!()`. Tests that call this function MUST fail until the
/// implementer fills in the real body.
pub fn parse_factory_lock(content: &str) -> Result<Option<LockState>, LockParseError> {
    let _ = content;
    todo!("D15 implementer task: extract parse_factory_lock from verify-factory-lock")
}

/// Extract the string value from a YAML key-value line like `key: "value"` or `key: value`.
///
/// Returns `Some(value)` if the line starts with `{key}: `, otherwise `None`.
/// Strips surrounding double-quotes from quoted values.
/// Returns `Some("")` for empty quoted values `""`.
///
/// # Red Gate
///
/// STUB — `todo!()`. Tests that call this function MUST fail until the
/// implementer fills in the real body.
pub fn extract_yaml_string_value(line: &str, key: &str) -> Option<String> {
    let _ = line;
    let _ = key;
    todo!("D15 implementer task: extract extract_yaml_string_value from verify-factory-lock")
}

/// Parse an ISO-8601 datetime string into a `chrono::DateTime<chrono::Utc>`.
///
/// Returns `Ok(dt)` on success, `Err(MalformedLockBlock)` if unparseable.
///
/// # Red Gate
///
/// STUB — `todo!()`. Tests that call this function MUST fail until the
/// implementer fills in the real body.
pub fn parse_iso8601(s: &str) -> Result<chrono::DateTime<chrono::Utc>, LockParseError> {
    let _ = s;
    todo!("D15 implementer task: extract parse_iso8601 from verify-factory-lock")
}

// ---------------------------------------------------------------------------
// Unit tests — factory-lock-parse crate (D15 / S-17.04)
//
// These tests exercise the public API of this crate directly.
// ALL tests MUST FAIL before implementation (Red Gate) because every public
// function is a `todo!()` stub.
//
// Test naming: test_BC_5_40_001_xxx() — traces to BC-5.40.001 PC4 (shared
// parse logic is a prerequisite for the timestamp-refresh guard).
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

    use super::*;

    // -----------------------------------------------------------------------
    // Fixtures (inline — no I/O needed for pure parse functions)
    // -----------------------------------------------------------------------

    /// Minimal STATE.md with NO factory_lock block (unlocked baseline).
    fn state_no_lock() -> &'static str {
        "---\ndocument_type: state\nversion: \"0.0.1-test\"\nphase: test\n---\n\n# STATE\n"
    }

    /// STATE.md with a valid factory_lock block (all three sub-fields present).
    fn state_with_valid_lock() -> &'static str {
        concat!(
            "---\n",
            "document_type: state\n",
            "version: \"0.0.1-test\"\n",
            "phase: test\n",
            "factory_lock:\n",
            "  holder: \"holder@example.com\"\n",
            "  locked_at: \"2026-06-10T14:00:00Z\"\n",
            "  expires_at: \"2099-01-01T00:00:00Z\"\n",
            "---\n\n# STATE\n",
        )
    }

    /// STATE.md with a factory_lock block where holder is empty string (malformed).
    fn state_with_empty_holder() -> &'static str {
        concat!(
            "---\n",
            "document_type: state\n",
            "version: \"0.0.1-test\"\n",
            "phase: test\n",
            "factory_lock:\n",
            "  holder: \"\"\n",
            "  locked_at: \"2026-06-10T14:00:00Z\"\n",
            "  expires_at: \"2099-01-01T00:00:00Z\"\n",
            "---\n\n# STATE\n",
        )
    }

    // -----------------------------------------------------------------------
    // parse_factory_lock tests
    // -----------------------------------------------------------------------

    /// parse_factory_lock: valid block → Ok(Some(LockState)) with correct fields.
    ///
    /// RED GATE: parse_factory_lock is todo!() → panics.
    #[test]
    fn test_BC_5_40_001_parse_factory_lock_returns_some_on_valid_block() {
        let result = parse_factory_lock(state_with_valid_lock());
        let lock = result
            .expect("parse must succeed on valid content")
            .expect("lock block must be present");
        assert_eq!(lock.holder, "holder@example.com");
        assert_eq!(lock.locked_at, "2026-06-10T14:00:00Z");
        assert_eq!(lock.expires_at, "2099-01-01T00:00:00Z");
    }

    /// parse_factory_lock: no factory_lock key → Ok(None).
    ///
    /// RED GATE: parse_factory_lock is todo!() → panics.
    #[test]
    fn test_BC_5_40_001_parse_factory_lock_returns_none_when_absent() {
        let result = parse_factory_lock(state_no_lock())
            .expect("parse must not error on valid unlocked content");
        assert!(
            result.is_none(),
            "Absent factory_lock block must return Ok(None)"
        );
    }

    /// parse_factory_lock: malformed block (empty holder) → Err(MalformedLockBlock).
    ///
    /// RED GATE: parse_factory_lock is todo!() → panics.
    #[test]
    fn test_BC_5_40_001_parse_factory_lock_errors_on_empty_holder() {
        let result = parse_factory_lock(state_with_empty_holder());
        match result {
            Err(LockParseError::MalformedLockBlock(_)) => {
                // Correct.
            }
            Ok(Some(lock)) => panic!(
                "Expected MalformedLockBlock for empty holder, got Ok(Some) with holder: '{}'",
                lock.holder
            ),
            Ok(None) => panic!("Expected MalformedLockBlock for empty holder, got Ok(None)"),
        }
    }

    /// parse_factory_lock: CRLF line endings → same result as LF-only.
    ///
    /// RED GATE: parse_factory_lock is todo!() → panics.
    #[test]
    fn test_BC_5_40_001_parse_factory_lock_handles_crlf_line_endings() {
        // CRLF version of state_with_valid_lock.
        let crlf = concat!(
            "---\r\n",
            "document_type: state\r\n",
            "version: \"0.0.1-test\"\r\n",
            "phase: test\r\n",
            "factory_lock:\r\n",
            "  holder: \"holder@example.com\"\r\n",
            "  locked_at: \"2026-06-10T14:00:00Z\"\r\n",
            "  expires_at: \"2099-01-01T00:00:00Z\"\r\n",
            "---\r\n\r\n# STATE\r\n",
        );
        let result = parse_factory_lock(crlf);
        let lock = result
            .expect("CRLF content must parse successfully")
            .expect("lock block must be found in CRLF content");
        assert_eq!(lock.holder, "holder@example.com");
        assert_eq!(lock.expires_at, "2099-01-01T00:00:00Z");
    }

    // -----------------------------------------------------------------------
    // extract_yaml_string_value tests
    // -----------------------------------------------------------------------

    /// extract_yaml_string_value: bare unquoted value → Some("value").
    ///
    /// RED GATE: extract_yaml_string_value is todo!() → panics.
    #[test]
    fn test_BC_5_40_001_extract_yaml_string_value_bare_unquoted() {
        let result = extract_yaml_string_value("holder: user@example.com", "holder");
        assert_eq!(
            result,
            Some("user@example.com".to_string()),
            "Bare unquoted value must be returned as-is"
        );
    }

    /// extract_yaml_string_value: double-quoted value → Some("value") without quotes.
    ///
    /// RED GATE: extract_yaml_string_value is todo!() → panics.
    #[test]
    fn test_BC_5_40_001_extract_yaml_string_value_double_quoted() {
        let result =
            extract_yaml_string_value("expires_at: \"2099-01-01T00:00:00Z\"", "expires_at");
        assert_eq!(
            result,
            Some("2099-01-01T00:00:00Z".to_string()),
            "Double-quoted value must strip surrounding quotes"
        );
    }

    /// extract_yaml_string_value: wrong key → None.
    ///
    /// RED GATE: extract_yaml_string_value is todo!() → panics.
    #[test]
    fn test_BC_5_40_001_extract_yaml_string_value_wrong_key_returns_none() {
        let result = extract_yaml_string_value("holder: user@example.com", "expires_at");
        assert!(result.is_none(), "Non-matching key must return None");
    }

    // -----------------------------------------------------------------------
    // parse_iso8601 tests
    // -----------------------------------------------------------------------

    /// parse_iso8601: valid UTC ISO-8601 string → Ok(DateTime).
    ///
    /// RED GATE: parse_iso8601 is todo!() → panics.
    #[test]
    fn test_BC_5_40_001_parse_iso8601_valid_utc_string_succeeds() {
        let result = parse_iso8601("2026-06-10T14:00:00Z");
        assert!(
            result.is_ok(),
            "Valid ISO-8601 string must parse successfully"
        );
    }

    /// parse_iso8601: invalid string → Err(MalformedLockBlock).
    ///
    /// RED GATE: parse_iso8601 is todo!() → panics.
    #[test]
    fn test_BC_5_40_001_parse_iso8601_invalid_string_errors() {
        let result = parse_iso8601("not-a-timestamp");
        match result {
            Err(LockParseError::MalformedLockBlock(_)) => {
                // Correct.
            }
            Ok(_) => panic!("Invalid timestamp must NOT parse successfully"),
        }
    }
}
