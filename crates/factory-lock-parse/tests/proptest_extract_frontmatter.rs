//! VP-096 proptest harness for `extract_frontmatter` purity (T-010; S-19.02).
//!
//! Three properties are verified:
//!
//! 1. **Structural correctness** (Property 1): the output satisfies three
//!    result-property invariants independent of the implementation's search
//!    algorithm — prefix, delimiter-partition, and no-skipped-delimiter.
//! 2. **Determinism** (Property 2): two invocations on the same input are
//!    byte-identical.
//! 3. **CRLF known-answer** (Property 3): constructed CRLF-delimited inputs
//!    yield exact prefix results per BC-4.13.001 v1.15 / EC-017.
//!
//! # Red Gate
//!
//! `extract_frontmatter` is a `todo!()` stub in S-19.02 Task 10. All tests
//! in this file will panic until the implementation is complete.
//!
//! # BC Traces
//! - BC-4.13.001 v1.15 Phase-A Invariant 9 (frontmatter-only-parsing mandate;
//!   CRLF delimiter forms `\r\n---\r\n` / `\r\n---`-EOF added in v1.15 / EC-017)
//! - VP-096: extract_frontmatter Purity — output byte-equals file prefix up to
//!   (excluding) the second `---` delimiter line; deterministic for any input.
//! - AC-005 (byte-exact boundary; parity-with-full-file-parse FORBIDDEN per F-P2-011)

use factory_lock_parse::extract_frontmatter;
use proptest::prelude::*;

proptest! {
    /// VP-096 Property 1: result-property oracle — independent of impl search order.
    ///
    /// Verifies three structural invariants of the `extract_frontmatter` output
    /// WITHOUT recomputing the expected offset via the implementation's search
    /// algorithm. A shared ordering/offset bug in the impl is invisible to a
    /// mirror oracle; it is caught by Invariant C below.
    ///
    /// **Invariant A — prefix**: `extracted` is always a byte-prefix of `input`
    /// (`input.starts_with(extracted)`). Detects: out-of-bounds or wrong-slice bugs.
    ///
    /// **Invariant B — delimiter partition** (disjunctive):
    ///   - *None case* (`extracted == input`): no delimiter form is present
    ///     anywhere in `input` (all four forms must be absent).
    ///     Detects: impl returns full input when a delimiter IS present.
    ///   - *Some case* (`extracted` is a proper prefix): the bytes immediately
    ///     following `extracted` in `input` begin with one of the four delimiter
    ///     sequences:
    ///       - `\n---\n`   (LF-inline)
    ///       - `\r\n---\r\n` (CRLF-inline)
    ///       - `\n---` exact (LF-EOF: remainder IS the delimiter)
    ///       - `\r\n---` exact (CRLF-EOF: remainder IS the delimiter)
    ///     Detects: wrong cut point, off-by-one, wrong delimiter form selected.
    ///
    /// **Invariant C — minimality (no skipped inline delimiter)**: in the *Some*
    /// case, `extracted` must contain NO inline delimiter (`\n---\n` or
    /// `\r\n---\r\n`). If it does, the impl skipped an earlier delimiter.
    /// Detects: wrong-precedence bugs (e.g., CRLF-inline fires before LF-inline
    /// when both are present), wrong-first-occurrence bugs (e.g., second `\n---\n`
    /// used instead of first).
    ///
    /// Why this is not tautological (POLICY 11): the assertions operate on the
    /// output's structural properties only, never recomputing an expected value.
    /// A shared search-order bug in the impl would cause Invariant C to fail on
    /// any input containing two inline delimiters of the same or different forms.
    ///
    /// RED: extract_frontmatter is a todo!() stub; this test panics until Task 10.
    #[test]
    fn prop_extract_frontmatter_byte_equals_prefix(
        input in proptest::collection::vec(any::<u8>(), 0..=512),
    ) {
        let extracted = extract_frontmatter(&input);

        // Invariant A: extracted must be a prefix of input.
        prop_assert!(
            input.starts_with(extracted),
            "Invariant A failed: extracted is not a byte-prefix of input. \
             extracted.len()={}, input.len()={}",
            extracted.len(),
            input.len()
        );

        if extracted.len() == input.len() {
            // Invariant B (None case): full input returned → no delimiter form must be present.
            let has_lf_inline = input.windows(5).any(|w| w == b"\n---\n");
            let has_crlf_inline = input.windows(7).any(|w| w == b"\r\n---\r\n");
            let has_crlf_eof = input.ends_with(b"\r\n---");
            let has_lf_eof = input.ends_with(b"\n---");
            prop_assert!(
                !has_lf_inline && !has_crlf_inline && !has_crlf_eof && !has_lf_eof,
                "Invariant B (None) failed: full input returned but a delimiter IS present. \
                 has_lf_inline={has_lf_inline}, has_crlf_inline={has_crlf_inline}, \
                 has_crlf_eof={has_crlf_eof}, has_lf_eof={has_lf_eof}"
            );
        } else {
            // Invariant B (Some case): proper prefix returned → remainder must begin with a delimiter.
            let remainder = &input[extracted.len()..];
            let follows_lf_inline = remainder.starts_with(b"\n---\n");
            let follows_crlf_inline = remainder.starts_with(b"\r\n---\r\n");
            let follows_lf_eof = remainder == b"\n---";
            let follows_crlf_eof = remainder == b"\r\n---";
            prop_assert!(
                follows_lf_inline || follows_crlf_inline || follows_lf_eof || follows_crlf_eof,
                "Invariant B (Some) failed: proper prefix returned but bytes after extracted \
                 do not begin with any delimiter form. remainder={remainder:?}"
            );

            // Invariant C: extracted contains no inline delimiter (no earlier delimiter was skipped).
            let extracted_has_lf_inline = extracted.windows(5).any(|w| w == b"\n---\n");
            let extracted_has_crlf_inline = extracted.windows(7).any(|w| w == b"\r\n---\r\n");
            prop_assert!(
                !extracted_has_lf_inline && !extracted_has_crlf_inline,
                "Invariant C failed: extracted contains an inline delimiter — impl skipped \
                 an earlier delimiter (wrong-precedence or wrong-first-occurrence bug). \
                 extracted_has_lf_inline={extracted_has_lf_inline}, \
                 extracted_has_crlf_inline={extracted_has_crlf_inline}. \
                 extracted={extracted:?}"
            );
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
