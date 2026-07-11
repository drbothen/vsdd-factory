//! VP-096 proptest harness for `extract_frontmatter` purity (T-010; S-19.02).
//!
//! Property: `extract_frontmatter(bytes)` output byte-equals the input prefix
//! `bytes[0..delimiter_start_offset]` when `\n---\n` is present (exclusive
//! boundary), or the full input when absent. Additionally, two invocations on
//! the same input must produce byte-identical results (determinism).
//!
//! # Red Gate
//!
//! `extract_frontmatter` is a `todo!()` stub in S-19.02 Task 10. All tests
//! in this file will panic until the implementation is complete.
//!
//! # BC Traces
//! - BC-4.13.001 v1.14 Phase-A Invariant 9 (frontmatter-only-parsing mandate)
//! - VP-096: extract_frontmatter Purity — output byte-equals file prefix up to
//!   (excluding) the second `---` delimiter line; deterministic for any input.
//! - AC-005 (byte-exact boundary; parity-with-full-file-parse FORBIDDEN per F-P2-011)

use factory_lock_parse::extract_frontmatter;
use proptest::prelude::*;

/// Find the byte offset of the `\n---\n` delimiter in `input`, if present.
///
/// Returns `Some(i)` where `i` is the index of the leading `\n` byte of the
/// `\n---\n` sequence. Returns `None` if no such delimiter is found (the EOF
/// `\n---` form is considered a delimiter only when it appears at the very end
/// of the input, but for proptest simplicity we check only the `\n---\n` form;
/// the EOF form is exercised in unit tests T-008).
fn find_delimiter_offset(input: &[u8]) -> Option<usize> {
    // Search for b"\n---\n" as a contiguous sequence.
    let needle = b"\n---\n";
    input.windows(needle.len()).position(|w| w == needle)
}

proptest! {
    /// VP-096 Property 1: output is a byte-exact prefix bounded by the delimiter.
    ///
    /// For any arbitrary byte input:
    /// - If `\n---\n` is present at offset `i`, then `extract_frontmatter(input)` must
    ///   byte-equal `input[0..i]`.
    /// - If `\n---\n` is absent, `extract_frontmatter(input)` must byte-equal `input`.
    ///
    /// RED: extract_frontmatter is a todo!() stub; this test panics until Task 10.
    #[test]
    fn prop_extract_frontmatter_byte_equals_prefix(input in proptest::collection::vec(any::<u8>(), 0..=512)) {
        let extracted = extract_frontmatter(&input);
        match find_delimiter_offset(&input) {
            Some(offset) => {
                prop_assert_eq!(
                    extracted,
                    &input[..offset],
                    "When \\n---\\n present at offset {}, extracted must equal input[0..{}]",
                    offset,
                    offset
                );
            }
            None => {
                prop_assert_eq!(
                    extracted,
                    input.as_slice(),
                    "When no \\n---\\n delimiter, extracted must byte-equal full input"
                );
            }
        }
    }

    /// VP-096 Property 2: determinism — two invocations produce byte-identical results.
    ///
    /// For any arbitrary byte input, calling `extract_frontmatter` twice must
    /// return slices with identical content.
    ///
    /// RED: extract_frontmatter is a todo!() stub; this test panics until Task 10.
    #[test]
    fn prop_extract_frontmatter_is_deterministic(input in proptest::collection::vec(any::<u8>(), 0..=512)) {
        let first = extract_frontmatter(&input);
        let second = extract_frontmatter(&input);
        prop_assert_eq!(
            first,
            second,
            "extract_frontmatter must be deterministic: two invocations on same input must produce identical slices"
        );
    }

    /// VP-096 Property 3 (CRLF): output is a byte-exact prefix for CRLF-delimited inputs.
    ///
    /// F-S1902-P1-001: BC-4.13.001 v1.14→v1.15 amendment (human approved): extract_frontmatter
    /// MUST recognize the `\r\n---\r\n` CRLF delimiter form in addition to `\n---\n`.
    ///
    /// For any input constructed as `prefix + "\r\n---\r\n" + suffix` where prefix contains
    /// no `\n` bytes:
    ///   - `extract_frontmatter(input)` must byte-equal `input[0..prefix.len()]`
    ///     (exclusive of the `\r\n---\r\n` delimiter itself).
    ///
    /// The prefix is filtered to contain no `\n` bytes so no LF delimiter (`\n---\n`)
    /// can appear in the prefix and accidentally match before the CRLF form. The suffix
    /// is also filtered to contain no `\n` bytes, preventing a spurious LF delimiter
    /// after the injected CRLF delimiter from changing the expected offset.
    ///
    /// RED: extract_frontmatter only recognizes `\n---\n`; the CRLF form `\r\n---\r\n`
    /// is not found → full input returned → byte-exact-prefix property fails.
    #[test]
    fn prop_extract_frontmatter_crlf_byte_equals_prefix(
        prefix in proptest::collection::vec(
            any::<u8>().prop_filter("no-lf", |b| *b != b'\n'),
            0..=100
        ),
        suffix in proptest::collection::vec(
            any::<u8>().prop_filter("no-lf", |b| *b != b'\n'),
            1..=50
        )
    ) {
        // Build the CRLF-delimited input: prefix + "\r\n---\r\n" + suffix.
        let crlf_delimiter = b"\r\n---\r\n";
        let mut input = prefix.clone();
        input.extend_from_slice(crlf_delimiter);
        input.extend_from_slice(&suffix);

        let extracted = extract_frontmatter(&input);
        let expected = &input[..prefix.len()];

        prop_assert_eq!(
            extracted,
            expected,
            "CRLF: extracted must byte-equal input[0..{}] (exclusive of \\r\\n---\\r\\n). \
             Got {} bytes; expected {} bytes. \
             Fix: update extract_frontmatter to recognize \\r\\n---\\r\\n (BC-4.13.001 v1.15).",
            prefix.len(),
            extracted.len(),
            expected.len()
        );
    }
}
