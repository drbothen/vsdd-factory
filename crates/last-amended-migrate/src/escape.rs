//! D-1144 YAML double-quote escape remediation (BC-10.13.001 PC3).
//!
//! # S-15.03 SEC-001 remediation — control-character escaping
//!
//! A literal `"` is not the only byte sequence that is significant inside a
//! YAML double-quoted scalar: a raw (unescaped) newline, carriage return,
//! tab, or any other C0 control character (`U+0000`..`U+001F`) is ALSO
//! illegal there and breaks strict `safe_load` parsing exactly like an
//! unescaped quote does (CWE-116, improper output neutralization). Both
//! `needs_escaping` and `escape_value` therefore treat "needs escaping" as
//! "contains an unescaped `\"` OR a raw control character below `U+0020`" —
//! not just the quote case D-1144 originally named.

/// Whether `value` contains at least one literal `"` that is not already
/// escaped as `\"`, OR at least one raw (unescaped-by-construction) control
/// character below `U+0020` (`\n`, `\r`, `\t`, and anything else in that
/// range) — the full D-1144/SEC-001 defect class that breaks strict YAML
/// `safe_load` on `BC-INDEX.md`/`ARCH-INDEX.md`/`STATE.md`'s current
/// `last_amended` entries and on any `changelog:` item text this tool emits.
///
/// Must be idempotent-safe: an already-fully-escaped value returns `false`
/// (PC4) — this function is the detection half of that guarantee, and MUST
/// agree with `escape_value` on what counts as "already escaped." Control
/// characters need no odd/even backslash-run reasoning: `escape_value`
/// always converts a raw control byte into a multi-character printable
/// escape sequence (e.g. `\n` becomes the two characters `\` and `n`), so a
/// second pass over already-escaped output never observes a raw control
/// byte again — idempotency holds by construction, not by a parity check.
///
/// Implementation note (quote case only): a quote is "already escaped" iff
/// it is preceded by an ODD run of literal backslashes (each adjacent PAIR
/// of backslashes is one literal escaped backslash character, so an odd
/// trailing run means exactly one of them escapes the quote that follows). A
/// single forward pass tracking the current backslash-run length — never a
/// look-behind re-scan — correctly distinguishes `\"` (escaped, run length
/// 1) from `\\"` (an escaped backslash followed by an UNESCAPED quote, run
/// length 2).
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
        if is_raw_control(c) {
            return true;
        }
        backslash_run = 0;
    }
    false
}

/// Whether `c` is a raw C0 control character (`U+0000`..=`U+001F`) that is
/// illegal, unescaped, inside a YAML double-quoted scalar — covers `\n`
/// (`U+000A`), `\r` (`U+000D`), `\t` (`U+0009`), and every other control
/// character in that range.
fn is_raw_control(c: char) -> bool {
    (c as u32) < 0x20
}

/// Escape every unescaped literal `"` in `value` to `\"`, and every raw C0
/// control character to its standard YAML double-quoted-scalar escape
/// sequence (`\n`, `\r`, `\t` for those three; `\xHH` for any other control
/// byte below `U+0020`), preserving every other character verbatim
/// (BC-10.13.001 PC3; S-15.03 SEC-001). Idempotent: calling this on an
/// already-escaped value must not double-escape (PC4) — uses the same
/// odd/even backslash-run test as `needs_escaping` for the quote case, and
/// control characters are idempotent by construction (see `needs_escaping`'s
/// doc comment) — so a naive `value.replace('"', "\\\"")`'s double-escape
/// defect cannot occur here.
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
            '\n' => {
                result.push_str("\\n");
                backslash_run = 0;
            }
            '\r' => {
                result.push_str("\\r");
                backslash_run = 0;
            }
            '\t' => {
                result.push_str("\\t");
                backslash_run = 0;
            }
            c if is_raw_control(c) => {
                result.push_str(&format!("\\x{:02X}", c as u32));
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
