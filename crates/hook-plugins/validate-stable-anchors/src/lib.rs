//! validate-stable-anchors — PreToolUse WASM hook plugin.
//!
//! Enforces TD-VSDD-091 stable-anchor convention for spec files under
//! `.factory/specs/**/*.md`. Blocks writes whose body text contains volatile
//! `*.<ext>:NNN` line-cite patterns (e.g., `main.rs:416`, `setup.bats:42`,
//! `Cargo.toml:17`, `install.sh:99`).
//!
//! # Rationale
//!
//! TD-031 (escalated P2→P1, fix-burst-15, O-P16-001) documents a recurrent
//! convergence loop: the same fix-bursts that codify TD-031 introduce new
//! TD-031 violations because there was no mechanical enforcement. This hook
//! breaks the loop by blocking `*.<ext>:NNN` patterns at write time.
//!
//! # Exemption zones
//!
//! Three zones are exempt from the lint (see `is_exempt_line`):
//!
//! 1. Lines within `## Amendment …` historical sections (POLICY 1 changelog).
//! 2. Lines within `## Changelog` tables.
//! 3. Lines within the VP-079 Scenario 6 bash code-fence SITES array
//!    (operationally required per F-P17-001 — line ranges drive sed mutations).
//!
//! # Behavioral Contracts
//!
//! None formally registered yet; this hook implements enforcement for TD-031.
//! A BC will be registered as part of S-15.01 convergence gate (TD-031 closure).
//!
//! # Architecture compliance
//!
//! - HOST_ABI_VERSION = 1 (no new host functions introduced).
//! - Pure `fn hook_logic(...)` takes all host I/O as injectable closures.
//!   Unit tests exercise every branch without a WASM runtime.
//! - WASM-migration rule (Decision 5): native WASM Rust crate, NOT bash.
//!
//! # References
//!
//! - TD-031 (tech-debt-register.md)
//! - TD-VSDD-091 stable-anchor convention
//! - F-P16-001 / F-P16-002 / F-P17-001

// Allow `#[cfg(kani)]` without triggering unexpected_cfgs warning.
#![cfg_attr(not(kani), allow(unexpected_cfgs))]

use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// ABI version constant
// ---------------------------------------------------------------------------

/// HOST_ABI_VERSION declares the ABI contract version this plugin was built
/// against. The dispatcher reads this before any host call. Must remain 1.
pub const HOST_ABI_VERSION: u32 = 1;

// ---------------------------------------------------------------------------
// Pattern detection
// ---------------------------------------------------------------------------

/// Human-readable description of the source-file line-cite pattern.
///
/// Matches `<word_char>.<1-8 lowercase letters>:<digit>` — any
/// `*.<ext>:NNN` cite where `<ext>` is a source, config, or test file
/// extension (rs, sh, py, go, ts, js, md, toml, yaml, yml, bats, json, …).
///
/// Rationale for broad match (not just `.rs`): TD-VSDD-091 bars ALL volatile
/// source-file line citations, not only Rust. Known offenders in the spec
/// corpus include `.bats:NNN-NNN` (F-P17-003), `.toml:NNN`, `.sh:NNN`.
/// Locking the lint to a single extension would recreate the recurrence gap.
pub const SOURCE_LINE_CITE_PATTERN: &str =
    ".<ext>:NNN (word_char before dot; 1-8 lowercase letters; digit after colon; not a URL)";

/// Check whether `line` contains a volatile source-file line citation starting
/// at byte position `dot_pos` (the position of the `.` in `.<ext>:`).
///
/// Returns `true` if the sequence at `line[dot_pos..]` matches:
///
/// ```text
/// <word_char> . <1-8 lowercase ASCII letters> : <ASCII digit>
/// ```
///
/// where `<word_char>` is the character immediately before the dot, AND the
/// three bytes before the dot are NOT `://` (URL exclusion guard).
///
/// This is a pure function: no I/O, no globals.
pub fn is_source_line_cite(line: &[u8], dot_pos: usize) -> bool {
    // -- 1. Collect the extension: 1-8 lowercase ASCII letters after the dot --
    let after_dot = &line[dot_pos + 1..];
    let ext_len = after_dot
        .iter()
        .take(8)
        .take_while(|&&b| b.is_ascii_lowercase())
        .count();
    if ext_len == 0 {
        return false;
    }

    // -- 2. Verify the character after the extension is ':' --
    let colon_pos = dot_pos + 1 + ext_len;
    if line.get(colon_pos) != Some(&b':') {
        return false;
    }

    // -- 3. Verify at least one ASCII digit follows ':' --
    let after_colon_pos = colon_pos + 1;
    if !line
        .get(after_colon_pos)
        .map(|b| b.is_ascii_digit())
        .unwrap_or(false)
    {
        return false;
    }

    // -- 4. Verify a word character immediately precedes the dot --
    if dot_pos == 0 {
        return false;
    }
    let before_byte = line[dot_pos - 1];
    let is_word_char = before_byte.is_ascii_alphanumeric() || before_byte == b'_';
    if !is_word_char {
        return false;
    }

    // -- 5. URL exclusion guard: reject if "://" appears anywhere before the dot --
    // e.g. `https://example.com:8080` or `http://localhost.dev:3000` — the dot
    // is in a hostname and the scheme indicator "://" appears before it.
    // We scan the entire prefix before dot_pos (bounded by the line start).
    let lookbehind = &line[..dot_pos];
    if lookbehind.windows(3).any(|w| w == b"://") {
        return false;
    }

    true
}

/// Scan a single line for `*.<ext>:NNN` volatile source-file line-cite patterns.
///
/// Detects any `<word_char>.<1-8 lowercase letters>:<digit>` sequence.
/// Returns `true` if at least one such pattern is found.
/// This is a pure function.
pub fn scan_line(line: &str) -> bool {
    let bytes = line.as_bytes();
    for dot_pos in memchr_iter(b'.', bytes) {
        if is_source_line_cite(bytes, dot_pos) {
            return true;
        }
    }
    false
}

/// Simple byte-by-byte iterator over positions of `needle` in `haystack`.
///
/// Used by `scan_line` to locate all `.` bytes without pulling in a dependency.
fn memchr_iter(needle: u8, haystack: &[u8]) -> impl Iterator<Item = usize> + '_ {
    haystack
        .iter()
        .enumerate()
        .filter_map(move |(i, &b)| if b == needle { Some(i) } else { None })
}

// ---------------------------------------------------------------------------
// Exemption state machine
// ---------------------------------------------------------------------------

/// Tracks the current exemption zone while scanning a spec file line-by-line.
///
/// The state machine transitions on section headings:
///
/// - `## Amendment …` or `## Changelog` → `Changelog` (all lines exempt)
/// - Any other `## ` heading → `Body` (violations enforced)
/// - Opening ` ``` ` inside an exempt zone while in SITES context → `SitesFence`
/// - Closing ` ``` ` while in `SitesFence` → back to prior exempt state
///
/// The VP-079 Scenario 6 SITES array exemption uses a simpler heuristic:
/// within any code fence (` ``` `) that follows a line containing "SITES=(",
/// all content is exempt. This covers the operational line ranges without
/// requiring VP-079-specific identification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExemptZone {
    /// Normal spec body — lint is active.
    Body,
    /// Inside a `## Amendment …` or `## Changelog` section — lint is inactive.
    Changelog,
    /// Inside a bash code fence that contains `SITES=(` — lint is inactive.
    /// This covers the VP-079 Scenario 6 SITES array per F-P17-001.
    SitesFence,
}

/// Determine whether `line` opens a new top-level (`## `) section.
///
/// Returns `Some(true)` if this is an Amendment/Changelog heading (exempt zone),
/// `Some(false)` if this is any other `## ` heading (enforce zone),
/// `None` if the line is not a `## ` heading.
pub fn classify_heading(line: &str) -> Option<bool> {
    if !line.starts_with("## ") {
        return None;
    }
    let rest = &line[3..];
    let exempt =
        rest.starts_with("Amendment") || rest.to_ascii_lowercase().starts_with("changelog");
    Some(exempt)
}

/// A violation found in a spec file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Violation {
    /// 1-based line number within the spec file.
    pub line_number: usize,
    /// The full text of the offending line.
    pub line_text: String,
}

/// Scan the text content of a single spec file for TD-031 violations.
///
/// Returns a list of `Violation` entries (empty = no violations).
///
/// Exemption logic:
/// 1. `## Amendment …` and `## Changelog` sections: all lines exempt.
/// 2. Bash code fences (` ``` `) that contain `SITES=(`: all lines within
///    the fence are exempt (VP-079 Scenario 6 per F-P17-001).
///
/// This function is pure: no I/O.
pub fn scan_spec(content: &str) -> Vec<Violation> {
    let mut violations = Vec::new();
    let mut zone = ExemptZone::Body;
    let mut in_fence = false;

    for (idx, line) in content.lines().enumerate() {
        let line_num = idx + 1;

        // Track code fence transitions (``` markers).
        let trimmed = line.trim();
        let is_fence_marker = trimmed.starts_with("```");

        if is_fence_marker {
            if !in_fence {
                // Opening a new fence.
                in_fence = true;
                // zone transitions to SitesFence only when we see
                // "SITES=(" *inside* an already-open fence. We handle that below.
            } else {
                // Closing an existing fence.
                in_fence = false;
                if zone == ExemptZone::SitesFence {
                    // Restore to Changelog if we were in one, else Body.
                    // Since SITES fences appear within Changelog sections in VP-079,
                    // we conservatively restore to Changelog. If the fence was
                    // outside a Changelog section (unlikely but possible), this
                    // would over-exempt. Acceptable: SITES fences only appear in
                    // changelog-type sections in practice.
                    zone = ExemptZone::Changelog;
                }
                continue;
            }
        }

        // Inside a fence: check for SITES=( to activate SitesFence exemption.
        if in_fence && trimmed.contains("SITES=(") {
            zone = ExemptZone::SitesFence;
        }

        // Heading transition (only outside fences).
        if !in_fence && let Some(is_exempt) = classify_heading(line) {
            zone = if is_exempt {
                ExemptZone::Changelog
            } else {
                ExemptZone::Body
            };
            continue;
        }

        // Skip exempt zones.
        if zone != ExemptZone::Body {
            continue;
        }

        // Skip fences in Body zone (code blocks in spec body are not linted —
        // they are illustrative; only prose text carries normative weight).
        if in_fence {
            continue;
        }

        // Lint this line.
        if scan_line(line) {
            violations.push(Violation {
                line_number: line_num,
                line_text: line.to_string(),
            });
        }
    }

    violations
}

// ---------------------------------------------------------------------------
// Hook callback surface (injectable pattern)
// ---------------------------------------------------------------------------

/// All side-effecting callbacks injected into `hook_logic` for testability.
pub struct HookCallbacks<R, E, L>
where
    R: FnOnce(&str) -> Result<String, String>,
    E: FnMut(&str, &[(&str, &str)]),
    L: FnMut(u8, &str),
{
    /// Read a file by path; returns `Ok(contents)` or `Err(message)` if absent/error.
    pub read_file: R,
    /// Emit an event (type, fields).
    pub emit_event: E,
    /// Log a message at the given level (0=trace, 1=debug, 2=info, 3=warn, 4=error).
    pub log: L,
}

/// Core validate-stable-anchors hook logic.
///
/// Called on PreToolUse Edit|Write events. Reads `tool_input.file_path`; if the
/// target is a spec file under `.factory/specs/**/*.md`, reads its prospective
/// content and scans for TD-031 violations. Blocks if any are found.
///
/// Graceful degrade: file unreadable → Continue + log_warn (don't block on I/O error).
///
/// # Error codes
///
/// - `TD_031_STABLE_ANCHOR_VIOLATION`: spec body text contains `*.<ext>:NNN` line cite
///   (any source, config, or test file extension).
pub fn hook_logic<R, E, L>(
    payload: HookPayload,
    mut callbacks: HookCallbacks<R, E, L>,
) -> HookResult
where
    R: FnOnce(&str) -> Result<String, String>,
    E: FnMut(&str, &[(&str, &str)]),
    L: FnMut(u8, &str),
{
    // Extract file_path from tool_input.
    let file_path = match payload.tool_input.get("file_path").and_then(|v| v.as_str()) {
        Some(p) => p.to_string(),
        None => {
            (callbacks.log)(
                1,
                "[validate-stable-anchors] no file_path in payload — skipping",
            );
            return HookResult::Continue;
        }
    };

    // Only lint spec files: `.factory/specs/**/*.md`
    if !is_spec_target(&file_path) {
        return HookResult::Continue;
    }

    (callbacks.log)(
        2,
        &format!(
            "[validate-stable-anchors] checking spec file: {}",
            file_path
        ),
    );

    // For Write: content is in tool_input["content"].
    // For Edit: content is the result of reading the existing file (the edit
    // may not yet be applied). We read the file via the host to get current
    // content, then also check the new_string field for Edit calls, since the
    // hook fires PreToolUse (before the write is applied).
    //
    // Strategy: check both the existing file content (if readable) and any
    // new content fields from the tool_input payload. This catches violations
    // whether introduced by a fresh Write or an Edit that inserts a new cite.

    // Collect content to scan from tool_input fields.
    let mut content_to_scan: Vec<(String, String)> = Vec::new(); // (label, text)

    // For Write: check the "content" field (the new file body).
    if let Some(content) = payload.tool_input.get("content").and_then(|v| v.as_str()) {
        content_to_scan.push(("write_content".to_string(), content.to_string()));
    }

    // For Edit: check the "new_string" field (the replacement text).
    if let Some(new_str) = payload
        .tool_input
        .get("new_string")
        .and_then(|v| v.as_str())
    {
        // new_string is a fragment, not the full file. Scan it directly.
        // Exemption zones can't be tracked for fragments, but Edit's new_string
        // that contains *.<ext>:NNN is almost certainly a violation if it reaches
        // spec body text. We apply simple scan_line per line (no exemption state).
        let fragment_violations: Vec<Violation> = new_str
            .lines()
            .enumerate()
            .filter_map(|(i, line)| {
                if scan_line(line) {
                    Some(Violation {
                        line_number: i + 1,
                        line_text: line.to_string(),
                    })
                } else {
                    None
                }
            })
            .collect();

        if !fragment_violations.is_empty() {
            let violation_list = format_violations(&fragment_violations, &file_path);
            (callbacks.emit_event)(
                "td031.violation",
                &[
                    ("hook", "validate-stable-anchors"),
                    ("file", &file_path),
                    ("violation_count", &fragment_violations.len().to_string()),
                ],
            );
            return HookResult::block_with_fix(
                "validate-stable-anchors",
                format!(
                    "TD-031 violation in new_string for '{}': `*.<ext>:NNN` volatile line cites found. {}",
                    file_path, violation_list
                ),
                "Replace `*.<ext>:NNN` line citations with stable symbol anchors per TD-VSDD-091. \
                 Example: instead of `main.rs:416`, use the function name `emit_plugin_async_block_discarded` \
                 or qualified path `factory_dispatcher::main::run`. \
                 Amendment/Changelog sections and VP-079 Scenario 6 SITES arrays are exempt.",
                "TD_031_STABLE_ANCHOR_VIOLATION",
            );
        }

        // No violation in new_string — also read the current file content for
        // full-document validation (catches pre-existing violations surfaced
        // by a neighboring edit, but primarily this is belt-and-suspenders).
        // Read the full file to scan for context. Use callback.
        // NOTE: read_file is FnOnce so we do it here after new_string check.
        match (callbacks.read_file)(&file_path) {
            Ok(existing) => {
                content_to_scan.push(("existing_file".to_string(), existing));
            }
            Err(_) => {
                // File not yet created or unreadable — skip full-file scan.
                (callbacks.log)(
                    1,
                    &format!(
                        "[validate-stable-anchors] could not read '{}' for full-file scan — skipping",
                        file_path
                    ),
                );
            }
        }
    } else if content_to_scan.is_empty() {
        // Neither content nor new_string — read the existing file.
        match (callbacks.read_file)(&file_path) {
            Ok(existing) => {
                content_to_scan.push(("existing_file".to_string(), existing));
            }
            Err(_) => {
                (callbacks.log)(
                    1,
                    &format!(
                        "[validate-stable-anchors] could not read '{}' — skipping",
                        file_path
                    ),
                );
                return HookResult::Continue;
            }
        }
    }

    // Scan all collected content with full exemption-zone awareness.
    for (label, content) in &content_to_scan {
        let violations = scan_spec(content);
        if !violations.is_empty() {
            let violation_list = format_violations(&violations, &file_path);
            (callbacks.emit_event)(
                "td031.violation",
                &[
                    ("hook", "validate-stable-anchors"),
                    ("file", &file_path),
                    ("source", label.as_str()),
                    ("violation_count", &violations.len().to_string()),
                ],
            );
            return HookResult::block_with_fix(
                "validate-stable-anchors",
                format!(
                    "TD-031 violation in '{}' ({}): `*.<ext>:NNN` volatile line cites found. {}",
                    file_path, label, violation_list
                ),
                "Replace `*.<ext>:NNN` line citations with stable symbol anchors per TD-VSDD-091. \
                 Example: instead of `main.rs:416`, use the function name `emit_plugin_async_block_discarded` \
                 or qualified path `factory_dispatcher::main::run`. \
                 Amendment/Changelog sections and VP-079 Scenario 6 SITES arrays are exempt.",
                "TD_031_STABLE_ANCHOR_VIOLATION",
            );
        }
    }

    (callbacks.log)(
        2,
        &format!(
            "[validate-stable-anchors] PASS — no TD-031 violations in '{}'",
            file_path
        ),
    );
    HookResult::Continue
}

/// Returns true if `file_path` is a spec file that should be linted.
///
/// Target: `.factory/specs/**/*.md` (any Markdown file anywhere under the specs directory).
pub fn is_spec_target(file_path: &str) -> bool {
    file_path.starts_with(".factory/specs/") && file_path.ends_with(".md")
}

/// Format a list of violations into a compact human-readable string.
fn format_violations(violations: &[Violation], file_path: &str) -> String {
    let lines: Vec<String> = violations
        .iter()
        .take(5) // show at most 5 to keep block message readable
        .map(|v| format!("  {}:{}: {}", file_path, v.line_number, v.line_text.trim()))
        .collect();
    let suffix = if violations.len() > 5 {
        format!(" (and {} more)", violations.len() - 5)
    } else {
        String::new()
    };
    format!("Violations:\n{}{}", lines.join("\n"), suffix)
}

// ---------------------------------------------------------------------------
// Top-level entry point (wired to real host fns in main.rs)
// ---------------------------------------------------------------------------

/// Called from the WASI entry point in `main.rs`.
///
/// Wires the real vsdd_hook_sdk host functions to the injectable-callback
/// surface of `hook_logic`.
pub fn on_pre_tool_use(payload: HookPayload) -> HookResult {
    use vsdd_hook_sdk::host;

    hook_logic(
        payload,
        HookCallbacks {
            read_file: |path| match host::read_file(path, 131072, 5000) {
                Ok(bytes) => String::from_utf8(bytes).map_err(|e| e.to_string()),
                Err(e) => Err(format!("{:?}", e)),
            },
            emit_event: |event_type, fields| {
                host::emit_event(event_type, fields);
            },
            log: |level, msg| match level {
                0..=2 => host::log_info(msg),
                3 => host::log_warn(msg),
                _ => host::log_error(msg),
            },
        },
    )
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests;
