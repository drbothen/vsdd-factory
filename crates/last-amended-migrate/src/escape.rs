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
/// Implementation note: a quote is "already escaped" iff it is preceded by
/// an ODD run of literal backslashes (each adjacent PAIR of backslashes is
/// one literal escaped backslash character, so an odd trailing run means
/// exactly one of them escapes the quote that follows). A single forward
/// pass tracking the current backslash-run length — never a look-behind
/// re-scan — correctly distinguishes `\"` (escaped, run length 1) from
/// `\\"` (an escaped backslash followed by an UNESCAPED quote, run length 2).
pub fn needs_escaping(value: &str) -> bool {
    let mut backslash_run = 0usize;
    for c in value.chars() {
        if c == '\\' {
            backslash_run += 1;
            continue;
        }
        if c == '"' && backslash_run.is_multiple_of(2) {
            return true;
        }
        backslash_run = 0;
    }
    false
}

/// Escape every unescaped literal `"` in `value` to `\"`, preserving every
/// other character verbatim (BC-10.13.001 PC3). Idempotent: calling this on
/// an already-escaped value must not double-escape (PC4) — uses the same
/// odd/even backslash-run test as `needs_escaping`, so a naive
/// `value.replace('"', "\\\"")`'s double-escape defect cannot occur here.
pub fn escape_value(value: &str) -> String {
    let mut result = String::with_capacity(value.len() + 8);
    let mut backslash_run = 0usize;
    for c in value.chars() {
        match c {
            '\\' => {
                backslash_run += 1;
                result.push(c);
            }
            '"' => {
                if backslash_run.is_multiple_of(2) {
                    result.push('\\');
                }
                result.push('"');
                backslash_run = 0;
            }
            _ => {
                result.push(c);
                backslash_run = 0;
            }
        }
    }
    result
}
