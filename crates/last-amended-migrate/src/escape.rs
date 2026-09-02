//! D-1144 YAML double-quote escape remediation (BC-10.13.001 PC3).

/// Whether `value` contains at least one literal `"` that is not already
/// escaped as `\"` — the D-1144 defect class that breaks strict YAML
/// `safe_load` on `BC-INDEX.md`/`ARCH-INDEX.md`/`STATE.md`'s current
/// `last_amended` entries.
///
/// Must be idempotent-safe: an already-fully-escaped value returns `false`
/// (PC4) — this function is the detection half of that guarantee, and MUST
/// agree with `escape_value` on what counts as "already escaped."
///
/// # BC-5.38.001 compliance
///
/// NON-TRIVIAL: distinguishing an unescaped `"` from an already-escaped `\"`
/// requires a look-behind/branching scan over the preceding byte(s), and
/// correct handling of an escaped backslash immediately before a quote
/// (`\\"`, which IS an unescaped quote following a literal escaped
/// backslash, not a double-escape). Body is `todo!()`.
///
/// # Self-Check (BC-5.38.005 invariant 1)
///
/// "If I include this real implementation, will the test for this function
/// pass trivially without any implementer work?" — No; the `\\"` vs `\"`
/// distinction is exactly the kind of edge case a naive one-line check gets
/// wrong. Therefore: `todo!()`.
pub fn needs_escaping(value: &str) -> bool {
    todo!(
        "detect an unescaped literal double-quote in a {}-byte value \
        (BC-10.13.001 PC3), correctly distinguishing an unescaped `\"` from \
        an already-escaped `\\\"` and from `\\\\\"`",
        value.len()
    )
}

/// Escape every unescaped literal `"` in `value` to `\"`, preserving every
/// other character verbatim (BC-10.13.001 PC3). Idempotent: calling this on
/// an already-escaped value must not double-escape (PC4).
///
/// # BC-5.38.001 compliance
///
/// NON-TRIVIAL: same look-behind/branching requirement as `needs_escaping`,
/// plus the PC3 obligation to alter nothing but the escape insertions
/// themselves. Body is `todo!()`.
///
/// # Self-Check (BC-5.38.005 invariant 1)
///
/// "If I include this real implementation, will the test for this function
/// pass trivially without any implementer work?" — No; a naive
/// `value.replace('"', "\\\"")` would double-escape on a second run and
/// violate PC4's idempotency guarantee. Therefore: `todo!()`.
pub fn escape_value(value: &str) -> String {
    todo!(
        "escape unescaped double-quotes only, without double-escaping \
        already-escaped sequences, in a {}-byte value (BC-10.13.001 PC3 + \
        PC4 idempotency)",
        value.len()
    )
}
