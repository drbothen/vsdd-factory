//! VP-096 proptest harness for `extract_frontmatter` purity (T-010; S-19.02).
//!
//! Property: `extract_frontmatter(bytes)` output byte-equals the input prefix
//! `bytes[0..delimiter_start_offset]` for whichever delimiter form is present
//! (LF-inline `\n---\n`, CRLF-inline `\r\n---\r\n`, CRLF-EOF `\r\n---`, or
//! LF-EOF `\n---`), or the full input when no delimiter is present. Two
//! invocations on the same input must produce byte-identical results (determinism).
//!
//! # Red Gate
//!
//! `extract_frontmatter` is a `todo!()` stub in S-19.02 Task 10. All tests
//! in this file will panic until the implementation is complete.
//!
//! # BC Traces
//! - BC-4.13.001 v1.15 Phase-A Invariant 9 (frontmatter-only-parsing mandate;
//!   CRLF forms added in v1.15 / EC-017)
//! - VP-096: extract_frontmatter Purity — output byte-equals file prefix up to
//!   (excluding) the second `---` delimiter line; deterministic for any input.
//! - AC-005 (byte-exact boundary; parity-with-full-file-parse FORBIDDEN per F-P2-011)

use factory_lock_parse::extract_frontmatter;
use proptest::prelude::*;

/// Find the byte offset of the first recognized delimiter in `input`, modelling
/// all four forms that `extract_frontmatter` recognizes per BC-4.13.001 v1.15.
///
/// Returns `Some(offset)` where `offset` is the byte index of the leading
/// byte of the delimiter sequence (i.e. the first byte NOT included in the
/// returned frontmatter prefix). Returns `None` if no delimiter form is found.
///
/// Precedence mirrors the implementation exactly (BC-4.13.001 v1.15 §Search-Order):
///   1. LF-inline  `\n---\n`   — most common; inline delimiter, trailing newline.
///   2. CRLF-inline `\r\n---\r\n` — Windows autocrlf checkout (EC-017).
///   3. CRLF-EOF  `\r\n---` at end of input — CRLF file, no trailing newline.
///      Checked BEFORE LF-EOF: `\r\n---` ends with `\n---`; checking LF-EOF
///      first would yield wrong offset and leave a stray `\r` in the prefix.
///   4. LF-EOF    `\n---` at end of input — LF file, no trailing newline.
///
/// This is an independent oracle: it does NOT call `extract_frontmatter`.
/// Using the same primitives (window-scan, ends_with) but expressed separately
/// avoids POLICY 11 tautology while guaranteeing the same contract semantics.
fn find_delimiter_offset(input: &[u8]) -> Option<usize> {
    // 1. LF-inline \n---\n
    let lf_inline = b"\n---\n";
    if let Some(pos) = input.windows(lf_inline.len()).position(|w| w == lf_inline) {
        return Some(pos);
    }
    // 2. CRLF-inline \r\n---\r\n
    let crlf_inline = b"\r\n---\r\n";
    if let Some(pos) = input
        .windows(crlf_inline.len())
        .position(|w| w == crlf_inline)
    {
        return Some(pos);
    }
    // 3. CRLF-EOF \r\n--- at end of input (checked before LF-EOF).
    let crlf_eof = b"\r\n---";
    if input.ends_with(crlf_eof) {
        return Some(input.len() - crlf_eof.len());
    }
    // 4. LF-EOF \n--- at end of input.
    let lf_eof = b"\n---";
    if input.ends_with(lf_eof) {
        return Some(input.len() - lf_eof.len());
    }
    None
}

proptest! {
    /// VP-096 Property 1: output is a byte-exact prefix bounded by the delimiter.
    ///
    /// For any arbitrary byte input, the oracle searches for the first delimiter
    /// in precedence order (LF-inline → CRLF-inline → CRLF-EOF → LF-EOF):
    /// - If a delimiter is found at offset `i`, then `extract_frontmatter(input)`
    ///   must byte-equal `input[0..i]`.
    /// - If no delimiter form is present, `extract_frontmatter(input)` must
    ///   byte-equal the full input.
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
                    "Delimiter found at offset {}; extracted must equal input[0..{}]",
                    offset,
                    offset
                );
            }
            None => {
                prop_assert_eq!(
                    extracted,
                    input.as_slice(),
                    "No delimiter found; extracted must byte-equal full input"
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
