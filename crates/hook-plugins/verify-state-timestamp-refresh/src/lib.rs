//! verify-state-timestamp-refresh — PreToolUse WASM hook plugin (D16, S-17.04).
//!
//! Enforces BC-5.40.001 PC4 (mid-burst TTL renewal) at write-time.
//!
//! On each invocation the guard:
//!   1. Checks `file_path` in the tool payload (after canonical-path normalisation
//!      per EC-006 / ADR-025 §12.7 R6). If NOT `.factory/STATE.md`: return
//!      `Continue` immediately — zero overhead for all other files (AC-007).
//!   2. Extract the proposed content per tool:
//!      - Write   → `tool_input.content`       (full file body; AC-011)
//!      - Edit    → reconstruct: on-disk + `old_string`→`new_string` apply (AC-012)
//!      - MultiEdit → reconstruct: on-disk + sequential `edits[]` apply (AC-013)
//!   3. Read the on-disk `.factory/STATE.md` via `host::read_file`.
//!      On error (HostError or NotFound): return `Continue` (fail-open per §12.3 / AC-015).
//!      Emit `state_md_approaching_cap` diagnostic warn if bytes_read > 200000
//!      and bytes_read <= 262144 (STATE_MD_MAX_BYTES cap; reads exceeding the
//!      cap fail before reaching this warn path; Invariant 8).
//!      Call `factory_lock_parse::extract_frontmatter` on raw bytes before UTF-8 conversion
//!      (Invariant 7 frontmatter-only mandate; BC-5.40.001 v1.2).
//!   3a. For Edit/MultiEdit: scan new_string value(s) for top-level `timestamp:` and
//!      `factory_lock:` fields (ADR-032 Decision 1+3).
//!       - If neither is set: return Continue (guard_ran payload-neutral). AC-020.
//!       - If only factory_lock: is set: skip Steps 4–7; proceed to Step 8.
//!       - If timestamp: is set (with or without factory_lock:): run full check (Steps 4–8).
//!       For Write: skip this step (full content always checked).
//!   4. Extract `timestamp:` from both proposed content and the on-disk content.
//!   5. If `timestamp:` is absent in proposed content → Block: TimestampStale (AC-008 §12.3 row 6).
//!   6. If `timestamp:` is absent in on-disk content → Continue (first write ever, AC-015/AC-008).
//!   7. If `timestamp:` values are byte-identical → Block: TimestampStale (AC-005/AC-011).
//!   8. If a lock is held in proposed content (`factory_lock.holder` present and
//!      non-empty): compare `factory_lock.expires_at` byte-for-byte.
//!      If byte-identical → Block: LockExpiryStale (AC-006).
//!   9. All other paths → Continue.
//!
//! For Edit/MultiEdit reconstruction (AC-012/AC-013/AC-014):
//!   - Replace first occurrence of `old_string` (or all if `replace_all = true`).
//!   - If `old_string` not found in on-disk content → Continue (fail-open, AC-014).
//!
//! Fail-open error paths (AC-008 / ADR-025 §12.3):
//!   - Proposed content unparseable → Continue
//!   - On-disk read fails (HostError or NotFound) → Continue
//!   - `timestamp:` absent in on-disk → Continue
//!   - Plugin crash (on_error = continue) → Continue
//!
//! # Payload field discipline (ADR-025 §12.1 / Red Gate Test Table)
//!
//! - Write tool: `tool_input.content` (full file body)
//! - Edit tool: `tool_input.old_string` + `tool_input.new_string`
//!   (+ optional `tool_input.replace_all: bool`)
//! - MultiEdit tool: `tool_input.edits[]`
//!   (array of `{old_string, new_string, replace_all?}`)
//! - Path field: `tool_input.file_path` (NOT `tool_input.new_content` — that
//!   field does not exist in Claude Code payloads; 0 occurrences in
//!   5,235+ real dispatcher events per ADR-025 §12.1)
//!
//! # Behavioral Contracts
//!
//! - BC-5.40.001: STATE.md factory_lock schema + TTL + mid-burst renewal + state-burst CAS push.
//!   PC4 (mid-burst renewal) is the primary enforcement target.
//!   PC6 (single-dev zero friction) mandates fail-open on all error paths.
//!
//! # Architecture compliance (ADR-025 Decision 12)
//!
//! - HOST_ABI_VERSION = 1 (no new host functions introduced).
//! - No `serde_yaml` / `serde_norway` — manual line-by-line scan via `factory-lock-parse`.
//! - No `regex` crate — manual tokenisation only.
//! - `async = false` REQUIRED in registry entry (ADR-019; ADR-025 Decision 12).
//! - Guard is read-only: NEVER writes STATE.md.
//! - No `exec_subprocess` — reads proposed content from payload, on-disk via `host::read_file`.
//! - Pure `fn guard_logic(...)` takes all host I/O as injectable callbacks;
//!   unit tests exercise every branch without a WASM runtime.
//! - Trigger (v1.6 env-free): normalised path EQUALS `.factory/STATE.md` OR ends with
//!   `/.factory/STATE.md` — covers both relative and absolute paths from Claude Code
//!   production (bypass-proof per §12.1 / EC-006 / AC-018 / ADR-025 §12.7 R6 v1.6).

// Allow `#[cfg(kani)]` without triggering unexpected_cfgs warning.
#![cfg_attr(not(kani), allow(unexpected_cfgs))]

use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// ABI version constant
// ---------------------------------------------------------------------------

/// HOST_ABI_VERSION declares the ABI contract version this plugin was built
/// against. Must remain 1.
pub const HOST_ABI_VERSION: u32 = 1;

/// Maximum bytes to read from STATE.md via `host::read_file`.
/// 256 KiB (262144 bytes) per BC-5.40.001 v1.2 Precondition 6; parity with
/// `verify-factory-lock` (ADR-025 Decision 12 §12.5; S-19.02 PR #610).
/// This cap exceeds the worst-case observed STATE.md size (<200 KiB under
/// 500-line compaction discipline per D-442(e)), giving ≥25% headroom.
pub const STATE_MD_MAX_BYTES: u32 = 262144;

/// Timeout in milliseconds for the `host::read_file` call.
pub const READ_FILE_TIMEOUT_MS: u32 = 5000;

/// Canonical path of STATE.md — exact string comparison trigger after normalisation.
pub const STATE_MD_PATH: &str = ".factory/STATE.md";

// ---------------------------------------------------------------------------
// Canonical block messages (AC-005 / AC-006 exact text)
// ---------------------------------------------------------------------------

/// Canonical TimestampStale block message constants.
///
/// Full line (per AC-005 / Red Gate Test Table — full-line equality required):
/// `BLOCKED by verify-state-timestamp-refresh: STATE.md timestamp not advanced in
///  this write. Fix: Update 'timestamp:' to the current UTC time before writing
///  STATE.md. Code: TimestampStale.`
///
/// The string is assembled by `canonical_timestamp_stale_message()` by calling
/// `HookResult::block_with_fix(...)` — the canonical, single code path shared with
/// every Block return site in `guard_logic`. This guarantees byte-level parity
/// between the test-visible canonical string and the guard's actual Block output
/// (M1 / single-source-of-truth). No format! duplication.
pub const GUARD_NAME: &str = "verify-state-timestamp-refresh";
pub const TIMESTAMP_STALE_REASON: &str = "STATE.md timestamp not advanced in this write";
pub const TIMESTAMP_STALE_FIX: &str =
    "Update 'timestamp:' to the current UTC time before writing STATE.md";
pub const TIMESTAMP_STALE_CODE: &str = "TimestampStale";

/// Canonical LockExpiryStale block message constants.
///
/// Full line (per AC-006):
/// `BLOCKED by verify-state-timestamp-refresh: factory_lock.expires_at not refreshed
///  in this write while lock is held. Fix: Run: factory-lock-write.sh renew
///  .factory/STATE.md before writing STATE.md. Code: LockExpiryStale.`
pub const LOCK_EXPIRY_STALE_REASON: &str =
    "factory_lock.expires_at not refreshed in this write while lock is held";
pub const LOCK_EXPIRY_STALE_FIX: &str =
    "Run: factory-lock-write.sh renew .factory/STATE.md before writing STATE.md";
pub const LOCK_EXPIRY_STALE_CODE: &str = "LockExpiryStale";

// ---------------------------------------------------------------------------
// Canonical block message strings (for full-line equality assertions in tests)
// ---------------------------------------------------------------------------

/// Full canonical TimestampStale block string.
///
/// Constructed by calling `HookResult::block_with_fix(...)` — the SAME constructor
/// used in every Block return site in `guard_logic`. This is the single source of
/// truth: tests that assert `reason == canonical_timestamp_stale_message()` are
/// asserting equality to the actual output of the guard, not to a separately
/// maintained format string that could drift (M1 fix).
///
/// Tests MUST assert equality to this exact string (not a substring), per the
/// Red Gate Test Table "full-line equality required" mandate (AC-005, M03 fix).
pub fn canonical_timestamp_stale_message() -> String {
    match HookResult::block_with_fix(
        GUARD_NAME,
        TIMESTAMP_STALE_REASON,
        TIMESTAMP_STALE_FIX,
        TIMESTAMP_STALE_CODE,
    ) {
        HookResult::Block { reason } => reason,
        // block_with_fix always returns Block — this arm is unreachable.
        _ => unreachable!("block_with_fix always returns HookResult::Block"),
    }
}

/// Full canonical LockExpiryStale block string.
///
/// Constructed by calling `HookResult::block_with_fix(...)` — the SAME constructor
/// used in every LockExpiryStale Block return site in `guard_logic`. Single source
/// of truth (M1 fix).
///
/// Tests MUST assert equality to this exact string (not a substring), per the
/// Red Gate Test Table "full-line equality required" mandate (AC-006, M03 fix).
pub fn canonical_lock_expiry_stale_message() -> String {
    match HookResult::block_with_fix(
        GUARD_NAME,
        LOCK_EXPIRY_STALE_REASON,
        LOCK_EXPIRY_STALE_FIX,
        LOCK_EXPIRY_STALE_CODE,
    ) {
        HookResult::Block { reason } => reason,
        // block_with_fix always returns Block — this arm is unreachable.
        _ => unreachable!("block_with_fix always returns HookResult::Block"),
    }
}

// ---------------------------------------------------------------------------
// Canonical-path normalisation (EC-006 / ADR-025 §12.7 R6)
// ---------------------------------------------------------------------------

/// Normalise a `file_path` from a tool payload for comparison against `STATE_MD_PATH`.
///
/// Normalisation algorithm per EC-006 / ADR-025 §12.7 R6 (v1.6 — env-free):
///   1. Strip leading `./`
///   2. Collapse `//` → `/` (loop until stable)
///   3. Collapse `/./` → `/` (loop until stable)
///   4. Segment-stack `..` resolution: split on `/`, push normal segments,
///      drop `.` segments, pop on `..` (clamp to empty stack on above-root `..`
///      — discard, do not underflow). Rejoin with `/`. (EC-006 / v1.4 R4)
///
/// The `$CLAUDE_PROJECT_DIR` env-based prefix-strip (v1.5 Step 2) has been REMOVED.
/// `std::env::var` is dead in the WASI sandbox — env vars are never set — so the
/// strip never fired in production, making the guard inert on absolute paths.
/// The v1.6 trigger model uses suffix-match instead: after normalisation, the guard
/// fires when the result EQUALS `.factory/STATE.md` OR ENDS WITH `/.factory/STATE.md`
/// (see `guard_logic` Step 1). No env scaffolding needed. (AC-018 / ADR-025 §12.7 R6 v1.6)
///
/// Fail-open (return the partially-normalised path unchanged) only when an
/// unresolvable encoding (empty result after segment-stack on non-empty input) is
/// encountered — residual misses must never false-block.
pub fn normalise_path(path: &str) -> String {
    // Step 1: strip leading `./`.
    let path = path.strip_prefix("./").unwrap_or(path);

    // Step 2+3: collapse `//` → `/` and `/./` → `/` (loop until stable).
    let mut result = path.to_string();
    loop {
        let next = result.replace("//", "/").replace("/./", "/");
        if next == result {
            break;
        }
        result = next;
    }

    // Step 4: Segment-stack `..` resolution (EC-006 / ADR-025 §12.7 R6 / v1.4 R4).
    //
    // Split the path on `/`, build a stack:
    //   - empty segment (from leading/trailing `/` or `//`) → skip
    //   - `.` segment → skip (already collapsed above, but handle defensively)
    //   - `..` segment → pop last entry if stack non-empty; discard (clamp) if empty
    //   - any other segment → push
    //
    // Rejoin stack entries with `/`.
    // A leading `/` is preserved by detecting it on the input.
    //
    // Fail-open: if the result is empty after segment-stack resolution AND the
    // original path was non-empty, return the pre-stack result unchanged (safe
    // miss — no false-block on pathological input).
    let has_leading_slash = result.starts_with('/');
    let mut stack: Vec<&str> = Vec::new();
    for segment in result.split('/') {
        match segment {
            "" | "." => {} // skip empty and dot segments
            ".." => {
                // Pop last segment if stack is non-empty; clamp (discard) at root.
                if !stack.is_empty() {
                    stack.pop();
                }
                // If stack is empty: above-root `..` is clamped (discarded) — no underflow.
            }
            s => stack.push(s),
        }
    }

    if stack.is_empty() && !result.is_empty() {
        // Segment-stack collapsed everything (e.g., pure `..` input) — fail-open.
        // Return the pre-stack result so we never false-block on degenerate paths.
        return result;
    }

    let rejoined = stack.join("/");
    if has_leading_slash {
        format!("/{rejoined}")
    } else {
        rejoined
    }
}

// ---------------------------------------------------------------------------
// Proposed-content extraction (per-tool reconstruct — AC-011/012/013/014)
// ---------------------------------------------------------------------------

/// The result of extracting the proposed content from a tool payload.
#[derive(Debug)]
pub enum ProposedContent {
    /// Proposed full content string.
    Content(String),
    /// Fail-open: cannot reconstruct (old_string not found, or absent content field).
    FailOpen,
}

/// Extract the proposed full content for a `Write` tool payload.
///
/// Write tool provides `tool_input.content` — the complete new file body.
/// Returns `FailOpen` if the field is absent or not a string (fail-open per AC-008).
///
/// AC-011: the correct field is `content` (NOT `new_content`).
pub fn extract_write_proposed(payload: &HookPayload) -> ProposedContent {
    match payload.tool_input.get("content").and_then(|v| v.as_str()) {
        Some(s) => ProposedContent::Content(s.to_string()),
        None => ProposedContent::FailOpen,
    }
}

/// Extract the proposed full content for an `Edit` tool payload by reconstruction.
///
/// Edit tool provides `tool_input.old_string` + `tool_input.new_string` and optionally
/// `tool_input.replace_all`. The guard reconstructs the proposed full content by
/// applying the substitution to the on-disk content.
///
/// Returns:
/// - `Content(reconstructed)` if `old_string` found in `on_disk_content`.
/// - `FailOpen` if `old_string` not found (fail-open per AC-014).
/// - `FailOpen` if `old_string` or `new_string` is absent from payload.
///
/// AC-012: reconstruct from on-disk + fragment — NOT from fragment alone.
pub fn extract_edit_proposed(payload: &HookPayload, on_disk_content: &str) -> ProposedContent {
    let old_string = match payload
        .tool_input
        .get("old_string")
        .and_then(|v| v.as_str())
    {
        Some(s) => s,
        None => return ProposedContent::FailOpen,
    };
    let new_string = match payload
        .tool_input
        .get("new_string")
        .and_then(|v| v.as_str())
    {
        Some(s) => s,
        None => return ProposedContent::FailOpen,
    };
    let replace_all = payload
        .tool_input
        .get("replace_all")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if replace_all {
        if !on_disk_content.contains(old_string) {
            // old_string not found → fail-open (AC-014).
            return ProposedContent::FailOpen;
        }
        ProposedContent::Content(on_disk_content.replace(old_string, new_string))
    } else {
        // Replace first occurrence only.
        match on_disk_content.find(old_string) {
            None => ProposedContent::FailOpen, // AC-014: old_string not found → fail-open.
            Some(pos) => {
                let mut result = String::with_capacity(
                    on_disk_content.len() - old_string.len() + new_string.len(),
                );
                result.push_str(&on_disk_content[..pos]);
                result.push_str(new_string);
                result.push_str(&on_disk_content[pos + old_string.len()..]);
                ProposedContent::Content(result)
            }
        }
    }
}

/// Extract the proposed full content for a `MultiEdit` tool payload by sequential reconstruction.
///
/// MultiEdit tool provides `tool_input.edits[]` — an array of `{old_string, new_string,
/// replace_all?}`. The guard applies each element in array order to the accumulating content.
///
/// Returns:
/// - `Content(reconstructed)` if all edits applied successfully.
/// - `FailOpen` if any `old_string` is not found in the current content state (AC-014).
/// - `FailOpen` if the `edits` field is absent or not an array.
///
/// AC-013: sequential application, same substitution logic as Edit per element.
pub fn extract_multiedit_proposed(payload: &HookPayload, on_disk_content: &str) -> ProposedContent {
    let edits = match payload.tool_input.get("edits").and_then(|v| v.as_array()) {
        Some(arr) => arr,
        None => return ProposedContent::FailOpen,
    };

    let mut current = on_disk_content.to_string();

    for edit in edits {
        let old_string = match edit.get("old_string").and_then(|v| v.as_str()) {
            Some(s) => s,
            None => return ProposedContent::FailOpen,
        };
        let new_string = match edit.get("new_string").and_then(|v| v.as_str()) {
            Some(s) => s,
            None => return ProposedContent::FailOpen,
        };
        let replace_all = edit
            .get("replace_all")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        if replace_all {
            if !current.contains(old_string) {
                return ProposedContent::FailOpen; // AC-014.
            }
            current = current.replace(old_string, new_string);
        } else {
            match current.find(old_string) {
                None => return ProposedContent::FailOpen, // AC-014.
                Some(pos) => {
                    let mut result =
                        String::with_capacity(current.len() - old_string.len() + new_string.len());
                    result.push_str(&current[..pos]);
                    result.push_str(new_string);
                    result.push_str(&current[pos + old_string.len()..]);
                    current = result;
                }
            }
        }
    }

    ProposedContent::Content(current)
}

// ---------------------------------------------------------------------------
// Helper: extract a top-level YAML frontmatter field value
// ---------------------------------------------------------------------------

/// Result of attempting to extract a top-level frontmatter field.
#[derive(Debug)]
pub enum FieldResult {
    Found(String),
    NotFound,
    Malformed,
}

/// Extract a top-level `key: value` from STATE.md frontmatter content.
///
/// Scans only the region between the first and second `---\n` delimiters
/// (no YAML parser; no `regex` crate — Architecture Compliance Rule 4).
/// Uses `factory_lock_parse::extract_yaml_string_value` for the per-line scan.
///
/// Returns:
/// - `FieldResult::Found(value)` if the key is found in the frontmatter.
/// - `FieldResult::NotFound` if the frontmatter is well-formed but key is absent.
/// - `FieldResult::Malformed` if the frontmatter is unparseable (no closing `---`
///   delimiter). Callers must fail-open on Malformed (AC-008 §12.3 row 1).
pub fn extract_top_level_field(content: &str, key: &str) -> FieldResult {
    // Normalise CRLF.
    let normalised;
    let content = if content.contains('\r') {
        normalised = content.replace("\r\n", "\n");
        normalised.as_str()
    } else {
        content
    };

    // No opening `---\n` — treat as no-frontmatter (not malformed, just absent).
    let after_open = match content.strip_prefix("---\n") {
        Some(rest) => rest,
        None => return FieldResult::NotFound,
    };

    // Find the closing `---` delimiter. Absent → Malformed.
    let frontmatter_end = match after_open.find("\n---\n").or_else(|| {
        if after_open.ends_with("\n---") {
            Some(after_open.len() - 4)
        } else {
            None
        }
    }) {
        Some(pos) => pos,
        None => return FieldResult::Malformed,
    };

    let frontmatter = &after_open[..frontmatter_end];

    for line in frontmatter.lines() {
        // Only scan top-level lines (not indented sub-fields).
        if line.starts_with(' ') || line.starts_with('\t') {
            continue;
        }
        if let Some(value) = factory_lock_parse::extract_yaml_string_value(line, key) {
            return FieldResult::Found(value);
        }
    }
    FieldResult::NotFound
}

// ---------------------------------------------------------------------------
// Helper: new_string_sets_field — ADR-032 Decision 4 payload-scan helper
// ---------------------------------------------------------------------------

/// Return `true` if any non-indented line in `new_string` is a YAML string
/// assignment for `field_key` (ADR-032 Decision 4).
///
/// Iterates `new_string.lines()`. Skips any line whose first byte is a space
/// (0x20) or tab (0x09) — these are YAML sub-fields or list items. For each
/// non-indented line, calls
/// `factory_lock_parse::extract_yaml_string_value(line, field_key)`.
/// Returns `true` on the first `Some(_)` result.
/// Returns `false` if no match found or `new_string` is empty.
///
/// Used by `guard_logic` to determine whether a payload explicitly sets
/// `timestamp:` (Decision 1) — the guard skips timestamp enforcement for
/// Edit/MultiEdit payloads that do NOT set the timestamp field.
///
/// # Notes
/// - The `factory_lock:` block key cannot be detected with this helper because
///   `factory_lock:` has no value on the same line; use the inline
///   `l.starts_with("factory_lock:")` scan for that field (Decision 3).
/// - A false positive (a body line detected as the timestamp field) causes
///   unnecessary enforcement, not bypass — safe failure mode per ADR-032 §Rationale.
pub fn new_string_sets_field(new_string: &str, field_key: &str) -> bool {
    for line in new_string.lines() {
        if line.starts_with(' ') || line.starts_with('\t') {
            continue;
        }
        if factory_lock_parse::extract_yaml_string_value(line, field_key).is_some() {
            return true;
        }
    }
    false
}

// ---------------------------------------------------------------------------
// Helper: extract factory_lock sub-fields independently (AC-016/AC-017)
// ---------------------------------------------------------------------------

/// Extracted `factory_lock` sub-field values from STATE.md frontmatter.
///
/// Both fields are `Option<String>`:
/// - `holder`:     `None` if the sub-field is absent; `Some("")` if present but empty.
/// - `expires_at`: `None` if the sub-field is absent; `Some("")` if present but empty.
///
/// This struct enables independent enforcement decisions:
/// - Lock held  ⟺ `holder` is `Some(h)` where `!h.is_empty()`
/// - ExpiryStale ⟺ lock held AND `expires_at` is `None` OR `Some("")`
///   OR byte-identical to the on-disk value.
///
/// Unlike `parse_factory_lock`, this extractor does NOT error when `expires_at`
/// is absent or empty — it returns `None` for the missing field and lets the
/// caller apply enforcement logic (AC-016/AC-017 / ADR-025 §12.2 revised).
#[derive(Debug)]
pub struct LockSubfields {
    /// Raw `factory_lock.holder` value, or `None` if the sub-field is absent.
    pub holder: Option<String>,
    /// Raw `factory_lock.expires_at` value, or `None` if the sub-field is absent.
    pub expires_at: Option<String>,
}

/// Extract `factory_lock.holder` and `factory_lock.expires_at` independently
/// from STATE.md frontmatter content without failing when either sub-field
/// is absent or empty.
///
/// Returns `None` (fail-open) if:
/// - The frontmatter is malformed (no closing `---` delimiter). Callers must
///   fail-open on malformed content per AC-008 §12.3 row 1.
/// - The `factory_lock:` key is entirely absent (unlocked state).
///
/// Returns `Some(LockSubfields)` when the `factory_lock:` key is present,
/// regardless of whether sub-fields are present, empty, or absent.
/// This is the key difference from `parse_factory_lock` — it does NOT error
/// on absent/empty sub-fields; it returns the raw `Option<String>` for each.
///
/// Architecture: no YAML parser, no `regex` crate (Rule 4). Line-by-line scan
/// over the frontmatter region only (between first and second `---\n`).
pub fn extract_lock_subfields(content: &str) -> Option<LockSubfields> {
    // Normalise CRLF.
    let normalised;
    let content = if content.contains('\r') {
        normalised = content.replace("\r\n", "\n");
        normalised.as_str()
    } else {
        content
    };

    // Extract frontmatter region.
    // None → no frontmatter (unlocked) or no closing delimiter (malformed) — both fail-open.
    let after_open = content.strip_prefix("---\n")?;
    let frontmatter_end = after_open.find("\n---\n").or_else(|| {
        if after_open.ends_with("\n---") {
            Some(after_open.len() - 4)
        } else {
            None
        }
    })?;
    let frontmatter = &after_open[..frontmatter_end];

    // Scan for factory_lock: key and its 2-space-indented sub-fields.
    let mut in_factory_lock = false;
    let mut found_factory_lock = false;
    let mut holder: Option<String> = None;
    let mut expires_at: Option<String> = None;

    for line in frontmatter.lines() {
        if line == "factory_lock:" || line.starts_with("factory_lock:") {
            let after_colon = line["factory_lock:".len()..].trim();
            if after_colon.is_empty() || after_colon == "~" || after_colon == "null" {
                in_factory_lock = true;
                found_factory_lock = true;
            } else {
                // Inline value on factory_lock: — treat as malformed, fail-open.
                return None;
            }
            continue;
        }

        if in_factory_lock {
            // Sub-fields indented with exactly 2 spaces.
            if line.starts_with("  ") && !line.starts_with("   ") {
                let field_line = &line[2..];
                if let Some(v) = factory_lock_parse::extract_yaml_string_value(field_line, "holder")
                {
                    holder = Some(v);
                } else if let Some(v) =
                    factory_lock_parse::extract_yaml_string_value(field_line, "expires_at")
                {
                    expires_at = Some(v);
                }
            } else if !line.is_empty() {
                // Non-indented non-empty line — exited the block.
                in_factory_lock = false;
            }
        }
    }

    if !found_factory_lock {
        return None; // factory_lock key absent — unlocked (no enforcement needed).
    }

    Some(LockSubfields { holder, expires_at })
}

// ---------------------------------------------------------------------------
// Injectable callbacks surface (testable without WASM runtime)
// ---------------------------------------------------------------------------

/// All side-effecting host calls injected into `guard_logic` for testability.
/// In production (`main.rs`), these are wired to real vsdd_hook_sdk host fns.
pub struct GuardCallbacks<R, L, W>
where
    R: FnOnce(&str, u32, u32) -> Result<Vec<u8>, String>,
    L: FnMut(&str),
    W: FnMut(&str),
{
    /// Read a file by path with `(path, max_bytes, timeout_ms)`.
    ///
    /// Returns:
    /// - `Ok(bytes)` on success.
    /// - `Err(msg)` on HostError (including `"NotFound"` when the file does not exist).
    ///   The guard treats ALL `Err(_)` variants as fail-open (AC-008/AC-015).
    pub read_file: R,
    /// Emit a `host::log_warn` message (advisory; non-blocking, internal log only).
    pub log_warn: L,
    /// Write a user-visible message to plugin stderr (relayed to dispatcher stderr).
    ///
    /// Used to emit `guard_ran (continue: <reason>)` on every Continue return path
    /// so bats allow-path tests can assert the guard executed its decision logic
    /// (AC-R5). FnMut because multiple Continue paths exist and only one fires per
    /// invocation — Rust requires FnMut for stored closures that may be called more
    /// than once even if only one call actually occurs at runtime.
    /// In production (`main.rs`) wired to `eprint!`; in tests wired to a noop or
    /// capturing closure.
    pub write_stderr: W,
}

// ---------------------------------------------------------------------------
// Core guard logic (injectable callbacks — testable without WASM runtime)
// ---------------------------------------------------------------------------

/// Core verify-state-timestamp-refresh guard logic.
///
/// All host I/O is injected via `callbacks` so unit tests can exercise every
/// branch without a WASM runtime.
///
/// Decision tree (per ADR-025 Decision 12 / BC-5.40.001 PC4+PC6):
///   1. Normalise `file_path` (EC-006). If NOT `.factory/STATE.md`: return Continue
///      immediately without calling `read_file` (AC-007 / §12.1).
///   2. Extract proposed content per tool:
///      - Write: `tool_input.content` directly (AC-011)
///      - Edit: reconstruct from on-disk + `old_string`/`new_string` (AC-012)
///      - MultiEdit: reconstruct from on-disk + sequential `edits[]` (AC-013)
///
///      On absent/fail-open condition → Continue (AC-008/AC-014).
///   3. Read on-disk STATE.md. On Err (any variant, including NotFound) → Continue
///      (fail-open §12.3 / AC-015).
///   4. Extract `timestamp:` from proposed content.
///      - Absent (NotFound) in proposed → Block: TimestampStale (§12.3 row 6).
///      - Malformed proposed frontmatter → Continue (fail-open, AC-008 §12.3 row 1).
///   5. Extract `timestamp:` from on-disk content.
///      - Absent or Malformed → Continue (first write ever, §12.3 row 5 / EC-004).
///   6. Byte-identical `timestamp:` → Block: TimestampStale (§12.2 / AC-005).
///   7. Lock held in proposed content (factory_lock.holder present + non-empty):
///      extract `factory_lock.expires_at` from proposed and on-disk.
///      Byte-identical → Block: LockExpiryStale (§12.2 / AC-006).
///   8. All other paths → Continue.
///
/// # BC traces
/// - BC-5.40.001 PC4: TimestampStale block / LockExpiryStale block
/// - BC-5.40.001 PC6: fail-open on all error paths
/// - ADR-025 Decision 12 §12.1: file_path trigger (bypass-proof, EC-006 normalise)
/// - ADR-025 Decision 12 §12.2: byte-comparison, not datetime parse
/// - ADR-025 Decision 12 §12.3: fail-open table
/// - AC-011: Write payload uses `content`
/// - AC-012: Edit payload reconstructs from on-disk + fragment
/// - AC-013: MultiEdit payload reconstructs sequentially from on-disk
/// - AC-014: old_string not found → fail-open
/// - AC-015: host::read_file NotFound → fail-open
/// - AC-018: absolute file_path (env-free suffix-match trigger, v1.6 P0 fix)
/// - AC-019: proposed timestamp empty string → Block TimestampStale
/// - AC-020: Edit/MultiEdit payload-neutrality — if no new_string in the payload sets EITHER timestamp: OR factory_lock:, guard returns Continue (payload-neutral; module //! Steps 4–8 skipped (falls through to Step 9 → Continue))
pub fn guard_logic<R, L, W>(
    payload: HookPayload,
    mut callbacks: GuardCallbacks<R, L, W>,
) -> HookResult
where
    R: FnOnce(&str, u32, u32) -> Result<Vec<u8>, String>,
    L: FnMut(&str),
    W: FnMut(&str),
{
    // Step 1: Normalise file_path. If not STATE.md, return Continue immediately (AC-007 / §12.1).
    let file_path = payload
        .tool_input
        .get("file_path")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let normalised = normalise_path(file_path);
    // Trigger check (AC-018 / ADR-025 §12.7 R6 v1.6 — env-free suffix-match model):
    //   - EQUALS ".factory/STATE.md"         (relative path, existing tests a–s)
    //   - ENDS WITH "/.factory/STATE.md"     (absolute path from Claude Code production)
    //
    // The prior env-based prefix-strip (std::env::var("CLAUDE_PROJECT_DIR")) is GONE —
    // it was dead code in the WASI sandbox where env vars are never set, making the
    // guard completely inert for all real Claude Code writes (AC-018 P0 fix).
    //
    // Boundary correctness:
    //   ".factory/STATE.md"                    → exact-eq  → fires
    //   "/abs/path/.factory/STATE.md"          → ends_with → fires
    //   "/abs/path/other/STATE.md"             → neither   → no-fire (correct)
    //   "/abs/path/.factory/STATE.md.bak"      → neither   → no-fire (correct)
    let is_state_md =
        normalised == STATE_MD_PATH || normalised.ends_with(concat!("/", ".factory/STATE.md"));
    if !is_state_md {
        // Not STATE.md — return Continue without reading any file (AC-007 zero-overhead).
        (callbacks.write_stderr)(
            "verify-state-timestamp-refresh: guard_ran (continue: non-state-md)\n",
        );
        return HookResult::Continue;
    }

    // Step 2: Determine proposed content based on tool type.
    // For Write: proposed content is tool_input.content directly (AC-011).
    // For Edit/MultiEdit: we need on-disk content first, then reconstruct.
    // In all cases: read on-disk STATE.md for comparison. Fail-open on any error (AC-008/AC-015).
    let on_disk_bytes =
        match (callbacks.read_file)(STATE_MD_PATH, STATE_MD_MAX_BYTES, READ_FILE_TIMEOUT_MS) {
            Ok(bytes) => bytes,
            Err(_e) => {
                // On-disk read failed (HostError or NotFound) — fail-open (AC-008/AC-015).
                (callbacks.log_warn)(
                    "verify-state-timestamp-refresh: fail-open read-error (STATE.md unreadable)",
                );
                (callbacks.write_stderr)(
                    "verify-state-timestamp-refresh: guard_ran (continue: fail-open read-error)\n",
                );
                return HookResult::Continue;
            }
        };

    // BC-5.40.001 Invariant 8 soft-warning: emit diagnostic when
    // bytes_read > soft_warn_threshold (200000) AND bytes_read <= STATE_MD_MAX_BYTES (262144).
    // Observability-only — never alters the Continue/Block verdict.
    let bytes_read = on_disk_bytes.len();
    if bytes_read > 200_000 && bytes_read <= STATE_MD_MAX_BYTES as usize {
        (callbacks.log_warn)(&format!(
            "state_md_approaching_cap: bytes_read={} cap_bytes={}",
            bytes_read, STATE_MD_MAX_BYTES
        ));
    }

    // BC-5.40.001 Invariant 7 frontmatter-only mandate: extract the YAML
    // frontmatter prefix before field extraction. The guard MUST NOT scan body
    // content when extracting timestamp: or factory_lock: fields (Steps 5 + 7).
    //
    // extract_frontmatter returns bytes[0..delimiter_start_offset] when a
    // closing `\n---\n` or `\n---`-at-EOF delimiter is found, or the full
    // input when absent (VP-096 / AC-005 boundary purity). Calling .to_vec()
    // releases the borrow on on_disk_bytes immediately.
    let frontmatter_owned: Vec<u8> =
        factory_lock_parse::extract_frontmatter(&on_disk_bytes).to_vec();

    let delimiter_found = frontmatter_owned.len() < on_disk_bytes.len();

    // on_disk_field_content: frontmatter-only string for field extraction (Steps 5 + 7).
    //
    // When delimiter found: frontmatter bytes + synthetic `\n---\n` so
    // extract_top_level_field can locate the boundary. Non-UTF-8 frontmatter bytes
    // → fail-open Continue. Body bytes need not be valid UTF-8 for field extraction
    // (T-004: non-UTF-8 body bytes are excluded here; the frontmatter slice is
    // guaranteed valid UTF-8 by the factory write discipline).
    //
    // When delimiter absent: frontmatter_owned is a full-content clone (extract_frontmatter
    // returned the full slice when no delimiter was found). extract_top_level_field returns
    // Malformed → fail-open Continue per AC-008 §12.3 row 1.
    let on_disk_field_content = if delimiter_found {
        let mut parse_input = frontmatter_owned;
        parse_input.extend_from_slice(b"\n---\n");
        match String::from_utf8(parse_input) {
            Ok(s) => s,
            Err(_) => {
                // Non-UTF-8 frontmatter bytes — fail-open.
                (callbacks.log_warn)(
                    "verify-state-timestamp-refresh: fail-open utf8 (STATE.md frontmatter is not valid UTF-8)",
                );
                (callbacks.write_stderr)(
                    "verify-state-timestamp-refresh: guard_ran (continue: fail-open utf8)\n",
                );
                return HookResult::Continue;
            }
        }
    } else {
        // Delimiter absent: frontmatter_owned is the full-content clone.
        // Pass it so extract_top_level_field returns Malformed → fail-open.
        match String::from_utf8(frontmatter_owned) {
            Ok(s) => s,
            Err(_) => {
                // Non-UTF-8 on-disk content (no delimiter) — fail-open.
                (callbacks.log_warn)(
                    "verify-state-timestamp-refresh: fail-open utf8 (STATE.md is not valid UTF-8)",
                );
                (callbacks.write_stderr)(
                    "verify-state-timestamp-refresh: guard_ran (continue: fail-open utf8)\n",
                );
                return HookResult::Continue;
            }
        }
    };

    // on_disk_reconstruction_base: FULL on-disk content for Edit/MultiEdit reconstruction
    // (Step 3 — AC-012/AC-013). The reconstruction base must be the complete file so
    // that edits targeting body content (after the closing --- delimiter) are found.
    //
    // F-P2-001 fix: the prior code passed the frontmatter-only on_disk_field_content as
    // the reconstruction base. Edits whose old_string targets body content were absent
    // from the truncated base → ProposedContent::FailOpen → Continue (guard bypass).
    // Restoring full-content reconstruction preserves pre-diff semantics.
    //
    // When delimiter found: attempt String::from_utf8 on the full on_disk_bytes.
    //   - Succeeds (normal UTF-8 file): full content used for reconstruction.
    //   - Fails (non-UTF-8 body, e.g., T-004 fixture with \xFF\xFE body bytes): fall back
    //     to on_disk_field_content. An Edit/MultiEdit cannot match inside invalid-UTF-8
    //     body bytes; extract_edit_proposed/extract_multiedit_proposed return FailOpen via
    //     AC-014 (old_string absent from frontmatter-only base) → Continue (correct).
    //
    // When delimiter absent: on_disk_field_content already holds the full content (it was
    // constructed from frontmatter_owned which was a full-content clone).
    let on_disk_reconstruction_base: String = if delimiter_found {
        match String::from_utf8(on_disk_bytes) {
            Ok(s) => s,
            Err(_) => {
                // Non-UTF-8 body bytes: full-content conversion fails.
                // Fall back to frontmatter-only base for reconstruction.
                on_disk_field_content.clone()
            }
        }
    } else {
        // No delimiter: on_disk_field_content holds the full content already.
        on_disk_field_content.clone()
    };

    // Step 3: Extract proposed content per tool type.
    let proposed_content: String = match payload.tool_name.as_str() {
        "Write" => match extract_write_proposed(&payload) {
            ProposedContent::Content(s) => s,
            ProposedContent::FailOpen => {
                (callbacks.log_warn)(
                    "verify-state-timestamp-refresh: fail-open extract-write (content field absent)",
                );
                (callbacks.write_stderr)(
                    "verify-state-timestamp-refresh: guard_ran (continue: fail-open extract-write)\n",
                );
                return HookResult::Continue;
            }
        },
        "Edit" => match extract_edit_proposed(&payload, &on_disk_reconstruction_base) {
            ProposedContent::Content(s) => s,
            ProposedContent::FailOpen => {
                (callbacks.log_warn)(
                    "verify-state-timestamp-refresh: fail-open extract-edit (old_string not found or absent)",
                );
                (callbacks.write_stderr)(
                    "verify-state-timestamp-refresh: guard_ran (continue: fail-open extract-edit)\n",
                );
                return HookResult::Continue;
            }
        },
        "MultiEdit" => match extract_multiedit_proposed(&payload, &on_disk_reconstruction_base) {
            ProposedContent::Content(s) => s,
            ProposedContent::FailOpen => {
                (callbacks.log_warn)(
                    "verify-state-timestamp-refresh: fail-open extract-multiedit (edits[] absent or old_string not found)",
                );
                (callbacks.write_stderr)(
                    "verify-state-timestamp-refresh: guard_ran (continue: fail-open extract-multiedit)\n",
                );
                return HookResult::Continue;
            }
        },
        _ => {
            // Unknown tool name — fall back to Write behaviour (content field).
            match extract_write_proposed(&payload) {
                ProposedContent::Content(s) => s,
                ProposedContent::FailOpen => {
                    (callbacks.log_warn)(
                        "verify-state-timestamp-refresh: fail-open extract-unknown-tool (content field absent)",
                    );
                    (callbacks.write_stderr)(
                        "verify-state-timestamp-refresh: guard_ran (continue: fail-open extract-unknown-tool)\n",
                    );
                    return HookResult::Continue;
                }
            }
        }
    };

    // ADR-032 Decision 1+3: Payload scan (inserted after code-inline Step 3).
    //
    // For Edit and MultiEdit, scan new_string value(s) to determine which enforcement
    // checks apply. For Write and unknown tools: both flags are true (full enforcement).
    //
    // sets_timestamp: any new_string sets timestamp: at column 0 (non-indented).
    // sets_factory_lock: any new_string has a non-indented line starting with factory_lock:.
    //
    // Routing (AC-020):
    //   - !sets_timestamp && !sets_factory_lock → payload-neutral → return Continue.
    //   - !sets_timestamp && sets_factory_lock  → skip Steps 4–6; run Step 7 only.
    //   - sets_timestamp (with or without sets_factory_lock) → run Steps 4–7 normally.
    let (sets_timestamp, sets_factory_lock) = match payload.tool_name.as_str() {
        "Edit" => {
            let ns = payload
                .tool_input
                .get("new_string")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let st = new_string_sets_field(ns, "timestamp");
            let sfl = ns.lines().any(|l| {
                !l.starts_with(' ') && !l.starts_with('\t') && l.starts_with("factory_lock:")
            });
            (st, sfl)
        }
        "MultiEdit" => {
            let st = payload
                .tool_input
                .get("edits")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter().any(|e| {
                        new_string_sets_field(
                            e.get("new_string").and_then(|v| v.as_str()).unwrap_or(""),
                            "timestamp",
                        )
                    })
                })
                .unwrap_or(true); // absent edits array: conservative
            let sfl = payload
                .tool_input
                .get("edits")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter().any(|e| {
                        e.get("new_string")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .lines()
                            .any(|l| {
                                !l.starts_with(' ')
                                    && !l.starts_with('\t')
                                    && l.starts_with("factory_lock:")
                            })
                    })
                })
                .unwrap_or(true); // absent edits array: conservative
            (st, sfl)
        }
        _ => {
            // Write + unknown tools: full enforcement (conservative — both true).
            (true, true)
        }
    };

    // AC-020: payload-neutral Edit/MultiEdit → skip all enforcement and return Continue.
    if !sets_timestamp && !sets_factory_lock {
        (callbacks.write_stderr)(
            "verify-state-timestamp-refresh: guard_ran (continue: payload-neutral)\n",
        );
        return HookResult::Continue;
    }

    // Steps 4–6: timestamp enforcement — only when the payload explicitly sets timestamp:.
    if sets_timestamp {
        // Step 4: Extract timestamp: from proposed content.
        let proposed_ts = match extract_top_level_field(&proposed_content, "timestamp") {
            FieldResult::Found(v) => v,
            FieldResult::NotFound => {
                // Absent timestamp: in proposed content is a violation (AC-008 §12.3 row 6 / EC-005).
                return HookResult::Block {
                    reason: canonical_timestamp_stale_message(),
                };
            }
            FieldResult::Malformed => {
                // Malformed proposed frontmatter — fail-open (AC-008 §12.3 row 1).
                (callbacks.log_warn)(
                    "verify-state-timestamp-refresh: fail-open malformed-proposed (frontmatter unparseable)",
                );
                (callbacks.write_stderr)(
                    "verify-state-timestamp-refresh: guard_ran (continue: fail-open malformed-proposed)\n",
                );
                return HookResult::Continue;
            }
        };

        // AC-019: empty or whitespace-only proposed timestamp is equivalent to absent
        // — Block: TimestampStale. `extract_top_level_field` returns `Found("")` for
        // `timestamp: ""` and `Found("   ")` for `timestamp: "   "` (field present but
        // value is empty or whitespace-only). Neither is a valid RFC-3339 timestamp;
        // treat identically to NotFound. ADR-025 §12.2: stale detection must reject
        // empty and whitespace-only values (L4 fix).
        if proposed_ts.trim().is_empty() {
            return HookResult::Block {
                reason: canonical_timestamp_stale_message(),
            };
        }

        // Step 5: Extract timestamp: from on-disk content (frontmatter-only per Invariant 7).
        let on_disk_ts = match extract_top_level_field(&on_disk_field_content, "timestamp") {
            FieldResult::Found(v) => v,
            FieldResult::NotFound | FieldResult::Malformed => {
                // Absent or malformed on-disk timestamp — first write ever (AC-008 §12.3 row 5 / EC-004).
                // Continue — no prior value to compare against.
                (callbacks.log_warn)(
                    "verify-state-timestamp-refresh: fail-open no-disk-timestamp (first write or malformed on-disk)",
                );
                (callbacks.write_stderr)(
                    "verify-state-timestamp-refresh: guard_ran (continue: fail-open no-disk-timestamp)\n",
                );
                return HookResult::Continue;
            }
        };

        // Step 6: Byte-identical timestamp: → Block TimestampStale (AC-005/AC-011/AC-012/AC-013).
        // ADR-025 §12.2: string comparison, not datetime parse.
        if proposed_ts == on_disk_ts {
            return HookResult::Block {
                reason: canonical_timestamp_stale_message(),
            };
        }
    }

    // Step 7: Lock-expiry enforcement — fires when sets_factory_lock OR sets_timestamp.
    //
    // When sets_timestamp=true: a timestamp-advancing Edit under a held lock MUST also
    // renew factory_lock.expires_at in the same payload (PC4 — BC-5.40.001 §PC4).
    // If the proposed content carries a stale expires_at, Block(LockExpiryStale).
    //
    // When sets_factory_lock=true (factory_lock-only Edit): enforce expires_at freshness
    // directly — the payload explicitly modifies the lock block.
    //
    // When neither (payload-neutral): already returned Continue above; unreachable here.
    //
    // "Lock held" = factory_lock.holder present and non-empty in proposed content.
    //
    // Use extract_lock_subfields (not parse_factory_lock) to extract holder and expires_at
    // independently — parse_factory_lock errors on absent/empty expires_at and routes through
    // Err(_) → None → skip, which is wrong when holder is present (AC-016/AC-017).
    //
    // Enforcement matrix (when holder is present and non-empty):
    //   - expires_at absent (None)          → Block: LockExpiryStale (AC-016)
    //   - expires_at empty string ("")      → Block: LockExpiryStale (AC-017)
    //   - expires_at byte-identical to disk → Block: LockExpiryStale (AC-006)
    //   - expires_at different from disk    → Continue (renewal happened)
    //
    // Fail-open cases (no lock enforcement):
    //   - extract_lock_subfields returns None (malformed frontmatter / no factory_lock key)
    //   - holder absent or empty (lock not held in proposed)
    if (sets_factory_lock || sets_timestamp)
        && let Some(proposed_subfields) = extract_lock_subfields(&proposed_content)
    {
        let proposed_holder = proposed_subfields
            .holder
            .as_deref()
            .unwrap_or("")
            .to_string();
        if !proposed_holder.is_empty() {
            // Lock is held in proposed content — enforce expires_at freshness.
            let proposed_expires = proposed_subfields
                .expires_at
                .as_deref()
                .unwrap_or("")
                .to_string();

            if proposed_expires.trim().is_empty() {
                // expires_at absent, empty string, or whitespace-only → Block: LockExpiryStale
                // (AC-016/AC-017). Whitespace-only is not a valid RFC-3339 timestamp and must
                // not slip through as a non-empty renewal (L4 fix / consistency with AC-019).
                return HookResult::Block {
                    reason: canonical_lock_expiry_stale_message(),
                };
            }

            // expires_at present, non-whitespace — compare byte-for-byte with on-disk (AC-006).
            let on_disk_subfields = extract_lock_subfields(&on_disk_field_content);
            let on_disk_expires = on_disk_subfields
                .as_ref()
                .and_then(|sf| sf.expires_at.as_deref())
                .unwrap_or("")
                .to_string();

            if !on_disk_expires.is_empty() && proposed_expires == on_disk_expires {
                // Byte-identical expires_at while lock is held → Block: LockExpiryStale (AC-006).
                return HookResult::Block {
                    reason: canonical_lock_expiry_stale_message(),
                };
            }
            // expires_at present, non-empty, and different from on-disk → renewal happened.
            // Fall through to Continue.
        }
    }

    // Step 8: All checks passed — allow the write (timestamp advanced + lock renewed if held).
    // Every Continue path in this function emits a guard_ran sentinel via write_stderr so
    // bats allow-path tests can assert the guard executed its decision logic (AC-R5).
    // A guard that panics on entry never reaches any emit; combined with exit 0 this
    // proves clean execution (not a silent crash). Block paths prove execution via exit 2.
    // log_warn is wired on every fail-open Continue path (observability parity with
    // verify-factory-lock — L2 fix); the clean success path has no warn to emit.
    (callbacks.write_stderr)("verify-state-timestamp-refresh: guard_ran (continue: advanced)\n");
    HookResult::Continue
}

// ---------------------------------------------------------------------------
// Top-level entry point (wired to real host fns in main.rs)
// ---------------------------------------------------------------------------

/// Called from the WASI entry point in `main.rs`.
///
/// Wires the real vsdd_hook_sdk host functions to the injectable-callback
/// surface of `guard_logic`.
pub fn on_pre_tool_use(payload: HookPayload) -> HookResult {
    use vsdd_hook_sdk::host;

    guard_logic(
        payload,
        GuardCallbacks {
            read_file: |path, max_bytes, timeout_ms| match host::read_file(
                path, max_bytes, timeout_ms,
            ) {
                Ok(bytes) => Ok(bytes),
                Err(e) => Err(format!("{:?}", e)),
            },
            log_warn: |msg| {
                host::log_warn(msg);
            },
            write_stderr: |msg| {
                eprint!("{msg}");
            },
        },
    )
}

// ---------------------------------------------------------------------------
// Unit tests — conformance suite v1.4 (D17 / S-17.04 / AC-005/006/007/008/011-017)
//
// 28 Rust unit tests covering the full AC matrix from S-17.04 v1.4.
// Uses injectable callbacks so no WASM runtime is required.
// All 28 tests pass green against the implemented guard_logic (T-3 + v1.4 T-5 complete).
//
// BLOCK MESSAGE ASSERTIONS: every Block assertion uses FULL canonical equality
// to the `canonical_timestamp_stale_message()` or `canonical_lock_expiry_stale_message()`
// strings — NOT substring contains checks (per ADR-025 finding M03 correction).
//
// PAYLOAD FIELD DISCIPLINE:
//   - Write tests use `tool_input.content` (full file body)
//   - Edit tests use `tool_input.old_string` + `tool_input.new_string`
//   - MultiEdit tests use `tool_input.edits[]`
//   - `tool_input.new_content` NEVER appears — that field does not exist in real payloads.
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    #![allow(
        clippy::expect_used,
        clippy::unwrap_used,
        clippy::panic,
        clippy::assertions_on_constants
    )]

    use super::*;
    use serde_json::json;
    use std::sync::{Arc, Mutex};

    // -----------------------------------------------------------------------
    // Fixture constants
    // -----------------------------------------------------------------------

    const TS_OLD: &str = "2026-06-11T10:00:00Z";
    const TS_NEW: &str = "2026-06-11T11:00:00Z";
    const EXPIRES_OLD: &str = "2026-06-11T10:45:00Z";
    const EXPIRES_NEW: &str = "2026-06-11T11:45:00Z";
    const HOLDER: &str = "dev@example.com";

    // -----------------------------------------------------------------------
    // Fixture builders — STATE.md content strings
    // -----------------------------------------------------------------------

    /// Build STATE.md content with a given timestamp, no lock.
    fn state_md_no_lock(timestamp: &str) -> String {
        format!(
            "---\ndocument_type: state\nversion: \"0.0.1-test\"\ntimestamp: \"{}\"\nphase: test\n---\n\n# STATE\n",
            timestamp
        )
    }

    /// Build STATE.md content with a given timestamp and a lock block.
    fn state_md_with_lock(timestamp: &str, expires_at: &str) -> String {
        format!(
            concat!(
                "---\n",
                "document_type: state\n",
                "version: \"0.0.1-test\"\n",
                "timestamp: \"{ts}\"\n",
                "phase: test\n",
                "factory_lock:\n",
                "  holder: \"{holder}\"\n",
                "  locked_at: \"2026-06-11T10:00:00Z\"\n",
                "  expires_at: \"{exp}\"\n",
                "---\n\n# STATE\n",
            ),
            ts = timestamp,
            holder = HOLDER,
            exp = expires_at,
        )
    }

    /// Build STATE.md content with NO timestamp field (simulates first-ever write).
    fn state_md_no_timestamp() -> String {
        "---\ndocument_type: state\nversion: \"0.0.1-test\"\nphase: test\n---\n\n# STATE\n"
            .to_string()
    }

    /// Build malformed frontmatter content (no closing `---`).
    fn state_md_malformed() -> String {
        "---\ndocument_type: state\nversion: broken".to_string()
    }

    // -----------------------------------------------------------------------
    // Payload builders — real tool payloads (NEVER use new_content)
    // -----------------------------------------------------------------------

    /// Build a `Write` tool HookPayload for `.factory/STATE.md`.
    ///
    /// Write tool payload: `tool_input.content` = full file body (AC-011).
    fn payload_write(content: &str) -> HookPayload {
        serde_json::from_value(json!({
            "event_name": "PreToolUse",
            "tool_name": "Write",
            "session_id": "test-session",
            "dispatcher_trace_id": "test-trace",
            "tool_input": {
                "file_path": ".factory/STATE.md",
                "content": content
            }
        }))
        .expect("fixture HookPayload must deserialize")
    }

    /// Build an `Edit` tool HookPayload for `.factory/STATE.md`.
    ///
    /// Edit tool payload: `tool_input.old_string` + `tool_input.new_string` (AC-012).
    /// The guard reconstructs the proposed content from on-disk + this fragment.
    fn payload_edit(old_string: &str, new_string: &str) -> HookPayload {
        serde_json::from_value(json!({
            "event_name": "PreToolUse",
            "tool_name": "Edit",
            "session_id": "test-session",
            "dispatcher_trace_id": "test-trace",
            "tool_input": {
                "file_path": ".factory/STATE.md",
                "old_string": old_string,
                "new_string": new_string
            }
        }))
        .expect("fixture HookPayload must deserialize")
    }

    /// Build a `MultiEdit` tool HookPayload for `.factory/STATE.md`.
    ///
    /// MultiEdit tool payload: `tool_input.edits[]` array (AC-013).
    /// Each element: `{old_string, new_string, replace_all?}`.
    fn payload_multiedit(edits: Vec<(&str, &str)>) -> HookPayload {
        let edits_json: Vec<serde_json::Value> = edits
            .iter()
            .map(|(old, new)| json!({"old_string": old, "new_string": new}))
            .collect();
        serde_json::from_value(json!({
            "event_name": "PreToolUse",
            "tool_name": "MultiEdit",
            "session_id": "test-session",
            "dispatcher_trace_id": "test-trace",
            "tool_input": {
                "file_path": ".factory/STATE.md",
                "edits": edits_json
            }
        }))
        .expect("fixture HookPayload must deserialize")
    }

    /// Build a payload for a non-STATE.md `Edit` (AC-007).
    fn payload_edit_non_state_md(
        file_path: &str,
        old_string: &str,
        new_string: &str,
    ) -> HookPayload {
        serde_json::from_value(json!({
            "event_name": "PreToolUse",
            "tool_name": "Edit",
            "session_id": "test-session",
            "dispatcher_trace_id": "test-trace",
            "tool_input": {
                "file_path": file_path,
                "old_string": old_string,
                "new_string": new_string
            }
        }))
        .expect("fixture HookPayload must deserialize")
    }

    /// Build a `Write` payload for a canonical-path variant of STATE.md.
    fn payload_write_path_variant(file_path: &str, content: &str) -> HookPayload {
        serde_json::from_value(json!({
            "event_name": "PreToolUse",
            "tool_name": "Write",
            "session_id": "test-session",
            "dispatcher_trace_id": "test-trace",
            "tool_input": {
                "file_path": file_path,
                "content": content
            }
        }))
        .expect("fixture HookPayload must deserialize")
    }

    // -----------------------------------------------------------------------
    // Callback builders
    // -----------------------------------------------------------------------

    /// Build callbacks where read_file returns `on_disk_content` as bytes.
    #[allow(clippy::type_complexity)]
    fn make_callbacks_with_disk(
        on_disk_content: String,
        warn_log: Arc<Mutex<Vec<String>>>,
    ) -> GuardCallbacks<
        impl FnOnce(&str, u32, u32) -> Result<Vec<u8>, String>,
        impl FnMut(&str),
        impl FnMut(&str),
    > {
        let wl = warn_log.clone();
        GuardCallbacks {
            read_file: move |_path, _max, _timeout| Ok(on_disk_content.into_bytes()),
            log_warn: move |msg: &str| {
                wl.lock().unwrap().push(msg.to_string());
            },
            write_stderr: |_msg| {}, // noop in tests — sentinel not asserted here
        }
    }

    /// Build callbacks where read_file returns an error string (covers HostError and NotFound).
    #[allow(clippy::type_complexity)]
    fn make_callbacks_read_error(
        error_msg: &str,
        warn_log: Arc<Mutex<Vec<String>>>,
    ) -> GuardCallbacks<
        impl FnOnce(&str, u32, u32) -> Result<Vec<u8>, String>,
        impl FnMut(&str),
        impl FnMut(&str),
    > {
        let err = error_msg.to_string();
        let wl = warn_log.clone();
        GuardCallbacks {
            read_file: move |_path, _max, _timeout| Err(err),
            log_warn: move |msg: &str| {
                wl.lock().unwrap().push(msg.to_string());
            },
            write_stderr: |_msg| {}, // noop in tests — sentinel not asserted here
        }
    }

    // -----------------------------------------------------------------------
    // AC-005 — WRITE payload: stale timestamp blocks (test_write_payload_stale_timestamp_blocks)
    // Traces: AC-011, AC-005 / ADR-025 D12 §12.2 / BC-5.40.001 PC4
    //
    // Write tool payload: tool_input.content = full stale file body.
    // Expected: Block with FULL canonical TimestampStale message.
    // -----------------------------------------------------------------------

    #[test]
    fn test_write_payload_stale_timestamp_blocks() {
        let on_disk = state_md_no_lock(TS_OLD);
        // Proposed: full file body with SAME timestamp as on-disk (stale).
        let proposed_content = state_md_no_lock(TS_OLD);

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        // Write payload: tool_input.content = full proposed file body (AC-011).
        let payload = payload_write(&proposed_content);

        let result = guard_logic(payload, callbacks);

        let expected_msg = canonical_timestamp_stale_message();
        match result {
            HookResult::Block { reason } => {
                assert_eq!(
                    reason, expected_msg,
                    "test_write_payload_stale_timestamp_blocks: Block message must be the FULL \
                     canonical TimestampStale string (not a substring). \
                     Expected: {expected_msg:?}. Got: {reason:?}"
                );
            }
            HookResult::Continue => panic!(
                "test_write_payload_stale_timestamp_blocks: expected Block(TimestampStale) but got Continue. \
                 Stub returns Continue — RED GATE. \
                 Write tool: tool_input.content contains unchanged timestamp → must Block."
            ),
            other => panic!(
                "test_write_payload_stale_timestamp_blocks: expected Block, got: {:?}",
                other
            ),
        }
    }

    // -----------------------------------------------------------------------
    // AC-005 — WRITE payload: timestamp advanced → Continue
    // Traces: AC-003, AC-011 / BC-5.40.001 PC4 success path
    //
    // GREEN: guard returns Continue for valid advanced-timestamp write.
    // -----------------------------------------------------------------------

    #[test]
    fn test_write_payload_advanced_timestamp_continues() {
        let on_disk = state_md_no_lock(TS_OLD);
        // Proposed: timestamp advanced.
        let proposed_content = state_md_no_lock(TS_NEW);

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_write(&proposed_content);

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "test_write_payload_advanced_timestamp_continues: Write with advanced timestamp must Continue (AC-003)"
        );
    }

    // -----------------------------------------------------------------------
    // AC-005 — timestamp stale, no lock → Block: TimestampStale (FULL canonical message)
    // Traces: AC-005 / ADR-025 D12 §12.2 / BC-5.40.001 PC4
    //
    // Uses Write payload (tool_input.content) — clean single-tool case.
    // GREEN: guard blocks stale timestamp with full canonical TimestampStale message.
    // -----------------------------------------------------------------------

    #[test]
    fn test_timestamp_stale_no_lock_blocks() {
        let on_disk = state_md_no_lock(TS_OLD);
        let proposed = state_md_no_lock(TS_OLD); // same timestamp — stale.

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_write(&proposed);

        let result = guard_logic(payload, callbacks);

        let expected_msg = canonical_timestamp_stale_message();
        match result {
            HookResult::Block { reason } => {
                assert_eq!(
                    reason, expected_msg,
                    "test_timestamp_stale_no_lock_blocks: Block message must be FULL canonical \
                     TimestampStale string. Expected: {expected_msg:?}. Got: {reason:?}"
                );
            }
            HookResult::Continue => panic!(
                "test_timestamp_stale_no_lock_blocks: expected Block(TimestampStale) but got Continue. \
                 Stub returns Continue — RED GATE."
            ),
            other => panic!("Expected Block, got: {:?}", other),
        }
    }

    // -----------------------------------------------------------------------
    // AC-005 — lock held, timestamp stale → Block: TimestampStale (FULL canonical message)
    // Traces: AC-005 / BC-5.40.001 PC4
    //
    // TimestampStale fires before LockExpiryStale (EC-003: both stale → TimestampStale first).
    // expires_at is advanced here to isolate timestamp staleness.
    // GREEN: guard blocks stale timestamp even when lock is held.
    // -----------------------------------------------------------------------

    #[test]
    fn test_timestamp_stale_lock_held_blocks() {
        let on_disk = state_md_with_lock(TS_OLD, EXPIRES_OLD);
        // Proposed: timestamp NOT advanced; expires_at advanced (isolates TimestampStale).
        let proposed = state_md_with_lock(TS_OLD, EXPIRES_NEW);

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_write(&proposed);

        let result = guard_logic(payload, callbacks);

        let expected_msg = canonical_timestamp_stale_message();
        match result {
            HookResult::Block { reason } => {
                assert_eq!(
                    reason, expected_msg,
                    "test_timestamp_stale_lock_held_blocks: Block message must be FULL canonical \
                     TimestampStale string. Expected: {expected_msg:?}. Got: {reason:?}"
                );
            }
            HookResult::Continue => panic!(
                "test_timestamp_stale_lock_held_blocks: expected Block(TimestampStale) but got Continue. \
                 TimestampStale must fire even when lock is held. RED GATE."
            ),
            other => panic!("Expected Block, got: {:?}", other),
        }
    }

    // -----------------------------------------------------------------------
    // AC-006 — lock held, expires_at stale → Block: LockExpiryStale (FULL canonical message)
    // Traces: AC-006 / ADR-025 D12 §12.2 / BC-5.40.001 PC4
    //
    // timestamp IS advanced (to get past TimestampStale), expires_at NOT advanced.
    // GREEN: guard blocks stale lock expiry with full canonical LockExpiryStale message.
    // -----------------------------------------------------------------------

    #[test]
    fn test_lock_expiry_stale_blocks() {
        let on_disk = state_md_with_lock(TS_OLD, EXPIRES_OLD);
        // Proposed: timestamp advanced, expires_at NOT advanced (stale lock expiry).
        let proposed = state_md_with_lock(TS_NEW, EXPIRES_OLD);

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_write(&proposed);

        let result = guard_logic(payload, callbacks);

        let expected_msg = canonical_lock_expiry_stale_message();
        match result {
            HookResult::Block { reason } => {
                assert_eq!(
                    reason, expected_msg,
                    "test_lock_expiry_stale_blocks: Block message must be FULL canonical \
                     LockExpiryStale string. Expected: {expected_msg:?}. Got: {reason:?}"
                );
            }
            HookResult::Continue => panic!(
                "test_lock_expiry_stale_blocks: expected Block(LockExpiryStale) but got Continue. \
                 Lock held + expires_at unchanged must Block. RED GATE."
            ),
            other => panic!("Expected Block, got: {:?}", other),
        }
    }

    // -----------------------------------------------------------------------
    // AC-006 — no lock in proposed content → LockExpiryStale MUST NOT fire
    // Traces: AC-006 / ADR-025 §12.3 row 3 / BC-5.40.001 PC6
    //
    // On-disk has a lock; proposed clears the lock. Timestamp is advanced.
    // Expected: Continue (clearing the lock + advancing timestamp is valid).
    // GREEN: guard returns Continue — no lock in proposed skips LockExpiryStale check.
    // -----------------------------------------------------------------------

    #[test]
    fn test_no_lock_held_skips_expiry_check() {
        let on_disk = state_md_with_lock(TS_OLD, EXPIRES_OLD);
        // Proposed: no lock (clearing the lock), timestamp advanced.
        let proposed = state_md_no_lock(TS_NEW);

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_write(&proposed);

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "test_no_lock_held_skips_expiry_check: No lock in proposed must skip LockExpiryStale \
             and return Continue when timestamp is advanced (AC-006 / ADR-025 §12.3 row 3)"
        );
    }

    // -----------------------------------------------------------------------
    // AC-008 — proposed content unparseable → Continue (fail-open)
    // Traces: AC-008 / ADR-025 §12.3 row 1 / BC-5.40.001 PC6
    //
    // GREEN: guard fails open — malformed proposed frontmatter returns Continue.
    // -----------------------------------------------------------------------

    #[test]
    fn test_proposed_unparseable_continues() {
        let on_disk = state_md_no_lock(TS_OLD);
        let proposed = state_md_malformed(); // No closing `---`.

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_write(&proposed);

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "test_proposed_unparseable_continues: Malformed proposed frontmatter must fail-open (AC-008)"
        );
    }

    // -----------------------------------------------------------------------
    // AC-008 — on-disk read fails (HostError) → Continue (fail-open)
    // Traces: AC-008 / ADR-025 §12.3 row 2 / BC-5.40.001 PC6
    //
    // GREEN: guard fails open — HostError on read_file returns Continue.
    // -----------------------------------------------------------------------

    #[test]
    fn test_on_disk_read_fails_continues() {
        let proposed = state_md_no_lock(TS_NEW);

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_read_error("HostError: Timeout", warn_log.clone());
        let payload = payload_write(&proposed);

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "test_on_disk_read_fails_continues: On-disk read HostError must fail-open (AC-008)"
        );
    }

    // -----------------------------------------------------------------------
    // AC-008 / AC-015 — host::read_file returns NotFound (first-ever write) → Continue
    // Traces: AC-015 / ADR-025 §12.3 row 5 / BC-5.40.001 PC6
    //
    // This tests AC-015 specifically: `host::read_file` returns `NotFound`
    // because the file does not yet exist on disk (first write to the repo).
    //
    // GREEN: guard fails open — NotFound on read_file returns Continue (first-ever write).
    // -----------------------------------------------------------------------

    #[test]
    fn test_read_file_not_found_continues() {
        let proposed = state_md_no_lock(TS_NEW);

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        // Simulate NotFound by returning Err with "NotFound" in the message.
        let callbacks = make_callbacks_read_error(
            "NotFound: .factory/STATE.md does not exist",
            warn_log.clone(),
        );
        let payload = payload_write(&proposed);

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "test_read_file_not_found_continues: host::read_file NotFound must fail-open (AC-015). \
             First write to STATE.md — no prior value to compare against."
        );
    }

    // -----------------------------------------------------------------------
    // AC-008 — timestamp absent in on-disk → Continue (first write ever, EC-004)
    // Traces: AC-008 §12.3 row 5 / BC-5.40.001 PC6
    //
    // On-disk file is well-formed but has NO timestamp field.
    // GREEN: guard fails open — absent on-disk timestamp returns Continue (first-ever write).
    // -----------------------------------------------------------------------

    #[test]
    fn test_timestamp_absent_on_disk_continues() {
        let on_disk = state_md_no_timestamp(); // No timestamp: field at all.
        let proposed = state_md_no_lock(TS_NEW);

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_write(&proposed);

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "test_timestamp_absent_on_disk_continues: Absent on-disk timestamp must Continue \
             (first write ever / AC-008 §12.3 row 5 / EC-004)"
        );
    }

    // -----------------------------------------------------------------------
    // AC-008 — timestamp absent in proposed content → Block: TimestampStale
    // Traces: AC-008 §12.3 row 6 / BC-5.40.001 PC4
    //
    // Absence of timestamp: in proposed content is itself a missing-field violation.
    // GREEN: guard blocks — absent proposed timestamp treated as TimestampStale.
    // -----------------------------------------------------------------------

    #[test]
    fn test_timestamp_absent_in_proposed_blocks() {
        let on_disk = state_md_no_lock(TS_OLD);
        let proposed = state_md_no_timestamp(); // Proposed has NO timestamp: field.

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_write(&proposed);

        let result = guard_logic(payload, callbacks);

        let expected_msg = canonical_timestamp_stale_message();
        match result {
            HookResult::Block { reason } => {
                assert_eq!(
                    reason, expected_msg,
                    "test_timestamp_absent_in_proposed_blocks: Block message must be FULL canonical \
                     TimestampStale string for absent proposed timestamp. \
                     Expected: {expected_msg:?}. Got: {reason:?}"
                );
            }
            HookResult::Continue => panic!(
                "test_timestamp_absent_in_proposed_blocks: expected Block(TimestampStale) but got Continue. \
                 Absence of timestamp: in proposed content is a violation. RED GATE."
            ),
            other => panic!("Expected Block, got: {:?}", other),
        }
    }

    // -----------------------------------------------------------------------
    // AC-007 — non-STATE.md file_path → Continue immediately, NO read_file call
    // Traces: AC-007 / ADR-025 §12.1 / BC-5.40.001 PC6
    //
    // The guard MUST return Continue without calling read_file for non-STATE.md paths.
    // Verified via call-counting mock: if read_file is called, assertion fails.
    //
    // GREEN: guard returns Continue for non-STATE.md without calling read_file.
    // -----------------------------------------------------------------------

    #[test]
    fn test_non_state_md_file_continues_without_read() {
        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let read_call_count = Arc::new(Mutex::new(0u32));
        let read_count_clone = read_call_count.clone();
        let wl = warn_log.clone();

        let callbacks = GuardCallbacks {
            read_file: move |_path, _max, _timeout| {
                *read_count_clone.lock().unwrap() += 1;
                Ok(b"some other file content".to_vec())
            },
            log_warn: move |msg: &str| {
                wl.lock().unwrap().push(msg.to_string());
            },
            write_stderr: |_msg: &str| {}, // noop in tests
        };

        let payload =
            payload_edit_non_state_md(".factory/specs/some-spec.md", "old text", "new text");

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "test_non_state_md_file_continues_without_read: Non-STATE.md must return Continue (AC-007)"
        );

        let calls = *read_call_count.lock().unwrap();
        assert_eq!(
            calls, 0,
            "test_non_state_md_file_continues_without_read: Non-STATE.md must NOT call read_file \
             (zero-overhead path per AC-007). read_file was called {} time(s).",
            calls
        );
    }

    // -----------------------------------------------------------------------
    // AC-012 / ADR-032 Decision 1 — Edit payload: new_string does NOT set timestamp:
    //   or factory_lock: → payload-neutral → Continue
    //
    // Traces: AC-012 / ADR-025 D12 §12.2 / BC-5.40.001 PC4 / ADR-032 Decision 1 (AC-020)
    //
    // Pre-ADR-032 behavior: guard reconstructed the full proposed content and
    // checked whether the timestamp was advanced. A non-timestamp Edit (phase: test
    // → phase: complete) with a stale on-disk timestamp would Block(TimestampStale).
    //
    // ADR-032 Decision 1 supersedes: guard now scans the payload (new_string) for
    // top-level timestamp: and factory_lock: fields BEFORE reconstruction. If neither
    // is set, the guard returns Continue immediately (payload-neutral; AC-020).
    // "phase: complete" sets neither → Continue.
    //
    // GREEN: guard returns Continue — payload-neutral Edit under ADR-032 Decision 1.
    // -----------------------------------------------------------------------

    #[test]
    fn test_edit_payload_reconstruct_phase_change_payload_neutral_continues() {
        let on_disk = state_md_no_lock(TS_OLD);

        // The edit changes something else (the phase field), NOT the timestamp.
        // old_string: the phase line in on-disk content.
        // new_string: different phase value but timestamp unchanged.
        // ADR-032: new_string sets NEITHER timestamp: NOR factory_lock: → payload-neutral.
        let old_str = "phase: test";
        let new_str = "phase: complete";

        // Verify our fixture has the old_string so the test is valid.
        assert!(
            on_disk.contains(old_str),
            "Test fixture must contain old_string: {old_str:?}"
        );

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_edit(old_str, new_str);

        let result = guard_logic(payload, callbacks);

        // ADR-032 Decision 1: payload-neutral Edit → Continue (AC-020).
        // Pre-ADR-032 Block(TimestampStale) behavior is superseded.
        assert_eq!(
            result,
            HookResult::Continue,
            "test_edit_payload_reconstruct_phase_change_payload_neutral_continues: Edit with new_string \
             'phase: complete' (no timestamp: or factory_lock: at col 0) is payload-neutral \
             under ADR-032 Decision 1 (AC-020) → must return Continue. \
             Pre-ADR-032 Block(TimestampStale) behavior is superseded."
        );
    }

    // -----------------------------------------------------------------------
    // AC-012 — Edit payload: reconstruct advances timestamp → Continue
    // Traces: AC-012 / AC-003 / BC-5.40.001 PC4 success path
    //
    // Edit payload: old_string = old timestamp line, new_string = new timestamp line.
    // Guard reconstructs: proposed has advanced timestamp → Continue.
    //
    // GREEN: guard returns Continue — reconstructed Edit proposed has advanced timestamp.
    // -----------------------------------------------------------------------

    #[test]
    fn test_edit_payload_reconstruct_advanced_timestamp_continues() {
        let on_disk = state_md_no_lock(TS_OLD);

        // Edit: update the timestamp line from TS_OLD to TS_NEW.
        let old_str = &format!("timestamp: \"{}\"", TS_OLD);
        let new_str = &format!("timestamp: \"{}\"", TS_NEW);

        assert!(
            on_disk.contains(old_str.as_str()),
            "Test fixture must contain old timestamp line: {old_str:?}"
        );

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_edit(old_str, new_str);

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "test_edit_payload_reconstruct_advanced_timestamp_continues: \
             Edit that advances timestamp must Continue (AC-012 / AC-003)"
        );
    }

    // -----------------------------------------------------------------------
    // AC-013 / ADR-032 Decision 1 — MultiEdit payload: no new_string sets timestamp:
    //   or factory_lock: → payload-neutral → Continue
    //
    // Traces: AC-013 / ADR-025 D12 §12.2 / BC-5.40.001 PC4 / ADR-032 Decision 1 (AC-020)
    //
    // Pre-ADR-032 behavior: guard reconstructed the full proposed content from all
    // edits[] and checked whether the timestamp was advanced. A MultiEdit with no
    // timestamp change would Block(TimestampStale).
    //
    // ADR-032 Decision 1 supersedes: guard scans all new_string values in edits[] for
    // top-level timestamp: / factory_lock: fields. If none is set → payload-neutral →
    // Continue immediately (AC-020). Neither "phase: complete" nor "version: 0.0.2-test"
    // sets timestamp: or factory_lock: → payload-neutral → Continue.
    //
    // GREEN: guard returns Continue — payload-neutral MultiEdit under ADR-032 Decision 1.
    // -----------------------------------------------------------------------

    #[test]
    fn test_multiedit_payload_reconstruct_phase_change_payload_neutral_continues() {
        let on_disk = state_md_no_lock(TS_OLD);

        // Two edits that don't touch the timestamp line or factory_lock block.
        // ADR-032: no new_string sets timestamp: or factory_lock: at col 0 → payload-neutral.
        let edit1_old = "phase: test";
        let edit1_new = "phase: complete";
        let edit2_old = "version: \"0.0.1-test\"";
        let edit2_new = "version: \"0.0.2-test\"";

        assert!(
            on_disk.contains(edit1_old),
            "Fixture must contain: {edit1_old:?}"
        );
        assert!(
            on_disk.contains(edit2_old),
            "Fixture must contain: {edit2_old:?}"
        );

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_multiedit(vec![(edit1_old, edit1_new), (edit2_old, edit2_new)]);

        let result = guard_logic(payload, callbacks);

        // ADR-032 Decision 1: payload-neutral MultiEdit → Continue (AC-020).
        // Pre-ADR-032 Block(TimestampStale) behavior is superseded.
        assert_eq!(
            result,
            HookResult::Continue,
            "test_multiedit_payload_reconstruct_phase_change_payload_neutral_continues: MultiEdit with no \
             new_string setting timestamp: or factory_lock: at col 0 is payload-neutral \
             under ADR-032 Decision 1 (AC-020) → must return Continue. \
             Pre-ADR-032 Block(TimestampStale) behavior is superseded."
        );
    }

    // -----------------------------------------------------------------------
    // AC-013 — MultiEdit payload: reconstruct advances timestamp → Continue
    // Traces: AC-013 / AC-003 / BC-5.40.001 PC4 success path
    //
    // GREEN: guard returns Continue — reconstructed MultiEdit proposed has advanced timestamp.
    // -----------------------------------------------------------------------

    #[test]
    fn test_multiedit_payload_reconstruct_advanced_timestamp_continues() {
        let on_disk = state_md_no_lock(TS_OLD);

        // First edit advances the timestamp.
        let ts_old_line = format!("timestamp: \"{}\"", TS_OLD);
        let ts_new_line = format!("timestamp: \"{}\"", TS_NEW);
        // Second edit changes something else.
        let phase_old = "phase: test";
        let phase_new = "phase: complete";

        assert!(
            on_disk.contains(ts_old_line.as_str()),
            "Fixture must contain old ts line"
        );
        assert!(
            on_disk.contains(phase_old),
            "Fixture must contain phase line"
        );

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_multiedit(vec![
            (ts_old_line.as_str(), ts_new_line.as_str()),
            (phase_old, phase_new),
        ]);

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "test_multiedit_payload_reconstruct_advanced_timestamp_continues: \
             MultiEdit that advances timestamp must Continue (AC-013 / AC-003)"
        );
    }

    // -----------------------------------------------------------------------
    // AC-014 — Edit payload: old_string not found in on-disk → Continue (fail-open)
    // Traces: AC-014 / ADR-025 D12 §12.3 / BC-5.40.001 PC6
    //
    // old_string is not present in the on-disk content. The Edit tool itself will
    // reject this; the guard must fail-open (not block).
    //
    // GREEN: guard fails open — old_string not found in on-disk returns Continue.
    // -----------------------------------------------------------------------

    #[test]
    fn test_edit_old_string_not_found_continues() {
        let on_disk = state_md_no_lock(TS_OLD);

        // old_string that does NOT exist in on-disk content.
        let old_str = "THIS_STRING_DOES_NOT_EXIST_IN_STATE_MD_FIXTURE_12345";
        let new_str = "some replacement";

        assert!(
            !on_disk.contains(old_str),
            "Fixture must NOT contain old_string for this test to be valid"
        );

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_edit(old_str, new_str);

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "test_edit_old_string_not_found_continues: old_string not found in on-disk → fail-open \
             Continue (AC-014). The Edit tool itself will reject the payload with an error."
        );
    }

    // -----------------------------------------------------------------------
    // AC-014 — MultiEdit: first old_string not found → Continue (fail-open)
    // Traces: AC-014 / ADR-025 D12 §12.3 / BC-5.40.001 PC6
    //
    // GREEN: guard fails open — first edit's old_string not found returns Continue.
    // -----------------------------------------------------------------------

    #[test]
    fn test_multiedit_first_old_string_not_found_continues() {
        let on_disk = state_md_no_lock(TS_OLD);

        // First edit's old_string does NOT exist.
        let edit1_old = "THIS_ALSO_DOES_NOT_EXIST_5678";
        let edit1_new = "replacement";

        assert!(
            !on_disk.contains(edit1_old),
            "Fixture must NOT contain edit1 old_string"
        );

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_multiedit(vec![(edit1_old, edit1_new)]);

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "test_multiedit_first_old_string_not_found_continues: first edit's old_string not \
             found → fail-open Continue (AC-014)"
        );
    }

    // -----------------------------------------------------------------------
    // EC-006 — Canonical-path variants: all normalise to STATE.md → guard triggers
    // Traces: EC-006 / ADR-025 §12.7 R6 / AC-007
    //
    // Tests that path variants that should normalise to .factory/STATE.md
    // do NOT escape the guard via fail-open-path-evasion.
    // Each variant uses a stale timestamp so the expected result is Block.
    //
    // Variants tested:
    //   1. `./.factory/STATE.md`  (leading ./)
    //   2. `.factory//STATE.md`   (double slash)
    //   3. `.factory/./STATE.md`  (/./  segment)
    //
    // Note: `$CLAUDE_PROJECT_DIR/.factory/STATE.md` (absolute prefix) is tested
    // separately below since it requires controlling the env var.
    //
    // GREEN: guard normalises all variants to .factory/STATE.md and blocks stale timestamp.
    // -----------------------------------------------------------------------

    #[test]
    fn test_canonical_path_variants_trigger_guard() {
        let on_disk = state_md_no_lock(TS_OLD);
        let stale_proposed = state_md_no_lock(TS_OLD); // stale → would Block if real impl.

        let variants = [
            "./.factory/STATE.md",
            ".factory//STATE.md",
            ".factory/./STATE.md",
        ];

        let expected_msg = canonical_timestamp_stale_message();

        for variant in &variants {
            let warn_log = Arc::new(Mutex::new(Vec::new()));
            let callbacks = make_callbacks_with_disk(on_disk.clone(), warn_log.clone());
            let payload = payload_write_path_variant(variant, &stale_proposed);

            let result = guard_logic(payload, callbacks);

            match result {
                HookResult::Block { ref reason } => {
                    assert_eq!(
                        *reason, expected_msg,
                        "test_canonical_path_variants_trigger_guard: variant {variant:?} — \
                         Block message must be FULL canonical TimestampStale string. \
                         Expected: {expected_msg:?}. Got: {reason:?}"
                    );
                }
                HookResult::Continue => panic!(
                    "test_canonical_path_variants_trigger_guard: variant {variant:?} normalises \
                     to .factory/STATE.md but guard returned Continue instead of Block. \
                     RED GATE: path normalisation + stale-timestamp block must both be implemented. \
                     Variant must NOT escape the guard via path normalisation."
                ),
                other => panic!(
                    "test_canonical_path_variants_trigger_guard: variant {variant:?}: \
                     expected Block, got: {:?}",
                    other
                ),
            }
        }
    }

    // -----------------------------------------------------------------------
    // EC-006 — Quoted timestamp edge case: genuinely-advanced quoted timestamp
    //          must NOT be over-blocked (robust-extraction test)
    // Traces: EC-006 edge / ADR-025 §12.2 (byte-comparison, not datetime parse)
    //
    // Verify that a timestamp in a quoted form that IS genuinely different from
    // the on-disk timestamp (not byte-identical) correctly returns Continue.
    // This guards against over-zealous `extract_yaml_string_value` stripping
    // that might incorrectly compare stripped vs. non-stripped values.
    //
    // GREEN: guard returns Continue — genuinely-advanced quoted timestamp is not over-blocked.
    // -----------------------------------------------------------------------

    #[test]
    fn test_quoted_timestamp_advanced_does_not_over_block() {
        // On-disk has timestamp with quotes.
        let on_disk = state_md_no_lock(TS_OLD); // contains: timestamp: "2026-06-11T10:00:00Z"
        // Proposed has a different (advanced) quoted timestamp.
        let proposed = state_md_no_lock(TS_NEW); // contains: timestamp: "2026-06-11T11:00:00Z"

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_write(&proposed);

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "test_quoted_timestamp_advanced_does_not_over_block: \
             Genuinely-advanced quoted timestamp must Continue (no over-blocking). \
             byte-comparison must compare the extracted value, not the raw line."
        );
    }

    // -----------------------------------------------------------------------
    // Fixture builder — unquoted timestamp variant
    //
    // ADR-025 §12.4 (D17 addition) mandates proving quote-stripping symmetry.
    // The standard `state_md_no_lock` emits `timestamp: "..."` (quoted).
    // This variant emits `timestamp: ...` (unquoted) so mixed-quoting tests can
    // feed an unquoted on-disk value against a quoted proposed value.
    // -----------------------------------------------------------------------

    /// Build STATE.md content with a given timestamp (UNQUOTED), no lock.
    ///
    /// Emits `timestamp: 2026-...` without surrounding double-quotes.
    /// Used by ADR-025 §12.4 mixed-quoting fixtures to prove that
    /// `extract_yaml_string_value` strips quotes symmetrically and does not
    /// produce false blocks or false continues based on quote presence.
    fn state_md_no_lock_unquoted_ts(timestamp: &str) -> String {
        format!(
            "---\ndocument_type: state\nversion: \"0.0.1-test\"\ntimestamp: {}\nphase: test\n---\n\n# STATE\n",
            timestamp
        )
    }

    // -----------------------------------------------------------------------
    // ADR-025 §12.4 — Mixed-quoting test 1: unquoted on-disk, quoted proposed,
    //   DIFFERENT instants → Continue (no false Block from quote mismatch)
    //
    // Traces: ADR-025 §12.4 (D17 addition) / EC-006 edge
    //
    // On-disk:  `timestamp: 2026-06-12T00:00:00Z`   (UNQUOTED)
    // Proposed: `timestamp: "2026-06-12T01:00:00Z"` (QUOTED, DIFFERENT value)
    //
    // The `extract_yaml_string_value` function strips surrounding double-quotes
    // before returning the value. If stripping is symmetric, it must extract
    // `2026-06-12T00:00:00Z` from the unquoted on-disk line AND
    // `2026-06-12T01:00:00Z` from the quoted proposed line, then compare them
    // as different strings → Continue.
    //
    // A bug where quote-stripping is applied only on one side (e.g., proposed
    // value has quotes stripped but on-disk value does not, or vice versa) would
    // compare `2026-06-12T00:00:00Z` against `"2026-06-12T01:00:00Z"` (raw) —
    // these are not byte-identical, so even a buggy guard would Continue here,
    // but the companion test below (quoted-vs-quoted same instant) catches the
    // inverse failure mode. Together they form the §12.4 symmetry proof.
    //
    // GREEN: guard returns Continue (different instants regardless of quoting).
    // -----------------------------------------------------------------------

    #[test]
    fn test_mixed_quoting_different_instant_continues() {
        // On-disk: UNQUOTED timestamp.
        let ts_unquoted = "2026-06-12T00:00:00Z";
        let on_disk = state_md_no_lock_unquoted_ts(ts_unquoted);
        // Proposed: QUOTED timestamp, DIFFERENT instant (advanced by 1 hour).
        let ts_quoted_new = "2026-06-12T01:00:00Z";
        let proposed = state_md_no_lock(ts_quoted_new); // uses standard quoted builder

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_write(&proposed);

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "test_mixed_quoting_different_instant_continues: \
             unquoted on-disk timestamp vs quoted proposed timestamp with a DIFFERENT \
             instant must return Continue. \
             extract_yaml_string_value must strip quotes symmetrically so that \
             '2026-06-12T00:00:00Z' (unquoted) != '2026-06-12T01:00:00Z' (quoted, stripped) \
             → different instants → no false Block. ADR-025 §12.4 load-bearing case."
        );
    }

    // -----------------------------------------------------------------------
    // ADR-025 §12.4 — Mixed-quoting test 2: quoted-vs-quoted, SAME instant → Block
    //
    // Traces: ADR-025 §12.4 (D17 addition) / AC-005 / ADR-025 §12.2
    //
    // On-disk:  `timestamp: "2026-06-12T00:00:00Z"` (QUOTED)
    // Proposed: `timestamp: "2026-06-12T00:00:00Z"` (QUOTED, SAME value — stale)
    //
    // Both sides are quoted with the same value. After symmetric quote-stripping,
    // extracted values are byte-identical → Block: TimestampStale.
    //
    // This confirms stale detection works correctly through quotes (not bypassed
    // because of quote presence). Full canonical block message asserted.
    //
    // GREEN: guard returns Block with full canonical TimestampStale message.
    // -----------------------------------------------------------------------

    #[test]
    fn test_quoted_same_instant_blocks_timestamp_stale() {
        // Both on-disk and proposed use the standard quoted builder with the same timestamp.
        let ts = "2026-06-12T00:00:00Z";
        let on_disk = state_md_no_lock(ts); // `timestamp: "2026-06-12T00:00:00Z"`
        let proposed = state_md_no_lock(ts); // identical — stale write

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_write(&proposed);

        let result = guard_logic(payload, callbacks);

        let expected_msg = canonical_timestamp_stale_message();
        match result {
            HookResult::Block { reason } => {
                assert_eq!(
                    reason, expected_msg,
                    "test_quoted_same_instant_blocks_timestamp_stale: \
                     quoted-vs-quoted same instant must Block with FULL canonical \
                     TimestampStale message. Expected: {expected_msg:?}. Got: {reason:?}"
                );
            }
            HookResult::Continue => panic!(
                "test_quoted_same_instant_blocks_timestamp_stale: \
                 expected Block(TimestampStale) but got Continue. \
                 Quoted-vs-quoted same instant is stale — must Block. \
                 ADR-025 §12.4: stale detection must work through quotes."
            ),
            other => panic!("Expected Block, got: {:?}", other),
        }
    }

    // -----------------------------------------------------------------------
    // EC-006 / v1.6 Red Gate — absolute path with double-slash triggers guard
    //   WITHOUT any env var (env-free suffix-match model)
    //
    // Traces: EC-006 / ADR-025 §12.7 R6 / AC-018 / v1.6 P0
    //
    // This test was previously `test_claude_project_dir_double_slash_path_triggers_guard`
    // which relied on `std::env::set_var("CLAUDE_PROJECT_DIR", ...)`. That approach is
    // now WRONG: the guard runs in the WASI sandbox where std::env::var is dead, so the
    // env-based prefix strip never fired in production — the guard was inert for all
    // absolute paths. The v1.6 fix replaces env-based prefix-stripping with a
    // suffix-match trigger: after all normalizations, fire when the normalized path
    // EQUALS `.factory/STATE.md` OR ENDS WITH `/.factory/STATE.md` (no env var).
    //
    // This rewrite exercises the same double-slash collapse scenario using a concrete
    // absolute path WITHOUT any set_var coupling:
    //   `/Users/alice/project//.factory/STATE.md`
    //   → pre-collapse `//` → `/Users/alice/project/.factory/STATE.md`
    //   → ends_with `/.factory/STATE.md` → guard triggers.
    //
    // MUST FAIL against current impl: current impl uses exact equality
    // (`normalised != STATE_MD_PATH`) so the absolute path → Continue.
    // After the implementer lands the ends_with trigger, this becomes GREEN.
    //
    // NO std::env::set_var / std::env::remove_var anywhere in this test.
    // -----------------------------------------------------------------------

    #[test]
    fn test_absolute_double_slash_path_triggers_guard_no_env() {
        let on_disk = state_md_no_lock(TS_OLD);
        let stale_proposed = state_md_no_lock(TS_OLD); // stale → Block

        // Absolute path with double-slash: pre-collapse turns `//` into `/`,
        // leaving `/Users/alice/project/.factory/STATE.md`.
        // The v1.6 suffix-match trigger: ends_with("/.factory/STATE.md") → fires.
        // No CLAUDE_PROJECT_DIR env var — the suffix-match needs none.
        let path = "/Users/alice/project//.factory/STATE.md";

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_write_path_variant(path, &stale_proposed);

        let result = guard_logic(payload, callbacks);

        let expected_msg = canonical_timestamp_stale_message();
        match result {
            HookResult::Block { reason } => {
                assert_eq!(
                    reason, expected_msg,
                    "test_absolute_double_slash_path_triggers_guard_no_env: \
                     absolute path with double-slash '{path}' must pre-collapse to \
                     '/Users/alice/project/.factory/STATE.md', match via ends_with \
                     '/.factory/STATE.md', and Block on stale timestamp (no env var). \
                     Expected: {expected_msg:?}. Got: {reason:?}"
                );
            }
            HookResult::Continue => panic!(
                "test_absolute_double_slash_path_triggers_guard_no_env: \
                 path '{path}' → after '//' collapse → '/Users/alice/project/.factory/STATE.md' \
                 must trigger the guard via ends_with '/.factory/STATE.md' and Block. \
                 Got Continue. RED GATE v1.6: current impl uses exact equality only \
                 (normalised != STATE_MD_PATH), which misses all absolute paths. \
                 Implementer must add the ends_with('/.factory/STATE.md') suffix-match trigger \
                 (AC-018 / ADR-025 §12.7 R6 v1.6 / P0 fix for WASI sandbox env-var deadness)."
            ),
            other => panic!(
                "test_absolute_double_slash_path_triggers_guard_no_env: \
                 expected Block, got: {:?}",
                other
            ),
        }
    }

    // -----------------------------------------------------------------------
    // AC-018 / v1.6 Red Gate — absolute path without env var triggers guard
    //
    // Traces: AC-018 / ADR-025 §12.7 R6 / BC-5.40.001 PC4 / v1.6 P0
    //
    // This is the EXACT production bypass that was masking the guard: Claude Code
    // emits absolute file_path values like `/Users/alice/project/.factory/STATE.md`
    // through the dispatcher. In the WASI sandbox, std::env::var("CLAUDE_PROJECT_DIR")
    // always returns Err — the env-based prefix strip never fired. The absolute path
    // does NOT equal `.factory/STATE.md` so the exact-equality guard trigger returned
    // Continue unconditionally. The guard was completely inert in production.
    //
    // The v1.6 fix: trigger = (normalised == STATE_MD_PATH) OR
    //               (normalised.ends_with("/.factory/STATE.md"))
    // No env var needed.
    //
    // This test explicitly removes CLAUDE_PROJECT_DIR from the environment (it may
    // have been set by another test in this process) before invoking guard_logic, to
    // confirm the env-free suffix-match path works without any env scaffolding.
    //
    // MUST FAIL against current impl (exact-equality only → Continue for absolute paths).
    // -----------------------------------------------------------------------

    #[test]
    fn test_absolute_path_triggers_guard_without_env() {
        // The v1.6 normalise_path implementation is env-free: CLAUDE_PROJECT_DIR is
        // never read (the env-based prefix strip was removed in P0-H1). No env setup
        // or teardown is needed — the suffix-match trigger fires purely on path content.
        let on_disk = state_md_no_lock(TS_OLD);
        let stale_proposed = state_md_no_lock(TS_OLD); // stale → Block

        // Real absolute path as emitted by Claude Code in production.
        // Normalises to `/Users/alice/project/.factory/STATE.md` (already clean).
        // Suffix-match: ends_with("/.factory/STATE.md") → guard must trigger.
        let path = "/Users/alice/project/.factory/STATE.md";

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_write_path_variant(path, &stale_proposed);

        let result = guard_logic(payload, callbacks);

        let expected_msg = canonical_timestamp_stale_message();
        match result {
            HookResult::Block { reason } => {
                assert_eq!(
                    reason, expected_msg,
                    "test_absolute_path_triggers_guard_without_env: \
                     absolute path '{path}' without CLAUDE_PROJECT_DIR set must trigger \
                     guard via ends_with('/.factory/STATE.md') suffix-match and Block. \
                     Expected: {expected_msg:?}. Got: {reason:?}"
                );
            }
            HookResult::Continue => panic!(
                "test_absolute_path_triggers_guard_without_env: \
                 path '{path}' must trigger guard without CLAUDE_PROJECT_DIR env var. \
                 Got Continue — this is the exact production bypass (P0). \
                 RED GATE v1.6: current impl uses exact equality \
                 (normalised != STATE_MD_PATH); absolute paths never match '.factory/STATE.md'. \
                 Implementer must add ends_with('/.factory/STATE.md') trigger \
                 (AC-018 / ADR-025 §12.7 R6 v1.6)."
            ),
            other => panic!(
                "test_absolute_path_triggers_guard_without_env: \
                 expected Block, got: {:?}",
                other
            ),
        }
    }

    // -----------------------------------------------------------------------
    // AC-018 / v1.6 — non-matching absolute paths must NOT false-trigger (control)
    //
    // Traces: AC-018 suffix-match boundary / AC-007 (non-STATE.md → Continue)
    //
    // The suffix-match trigger `ends_with("/.factory/STATE.md")` must NOT fire
    // on paths that merely contain ".factory/STATE.md" as a substring in the wrong
    // position, or that end with a similar-but-different string.
    //
    // Control paths:
    //   (1) `/Users/alice/project/other/STATE.md`   — ends with `/STATE.md` but
    //       the directory component is `other`, NOT `.factory`.
    //   (2) `/Users/alice/project/.factory/STATE.md.bak` — ends with `.md.bak`
    //       (not `.md`), so the suffix does not match.
    //
    // Both must return Continue (no false-trigger). These are GREEN controls —
    // they should pass against both current impl AND the v1.6 suffix-match impl.
    // Included here so any future change to the trigger boundary has an immediate
    // regression signal.
    // -----------------------------------------------------------------------

    #[test]
    fn test_absolute_non_state_md_paths_do_not_trigger_guard() {
        // Absolute path ending in /other/STATE.md — NOT /.factory/STATE.md.
        {
            let on_disk = state_md_no_lock(TS_OLD);
            let stale = state_md_no_lock(TS_OLD);
            let warn_log = Arc::new(Mutex::new(Vec::new()));
            let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
            let payload = payload_write_path_variant("/Users/alice/project/other/STATE.md", &stale);
            let result = guard_logic(payload, callbacks);
            assert_eq!(
                result,
                HookResult::Continue,
                "test_absolute_non_state_md_paths_do_not_trigger_guard: \
                 '/Users/alice/project/other/STATE.md' must NOT trigger guard — \
                 path ends with '/other/STATE.md', not '/.factory/STATE.md'. \
                 Suffix-match must be precise."
            );
        }

        // Absolute path ending in .factory/STATE.md.bak — NOT .factory/STATE.md.
        {
            let on_disk = state_md_no_lock(TS_OLD);
            let stale = state_md_no_lock(TS_OLD);
            let warn_log = Arc::new(Mutex::new(Vec::new()));
            let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
            let payload =
                payload_write_path_variant("/Users/alice/project/.factory/STATE.md.bak", &stale);
            let result = guard_logic(payload, callbacks);
            assert_eq!(
                result,
                HookResult::Continue,
                "test_absolute_non_state_md_paths_do_not_trigger_guard: \
                 '/Users/alice/project/.factory/STATE.md.bak' must NOT trigger guard — \
                 path ends with '.md.bak', not '/.factory/STATE.md'. \
                 Suffix-match boundary must not over-match."
            );
        }
    }

    // -----------------------------------------------------------------------
    // AC-019 / v1.6 Red Gate — proposed timestamp present but EMPTY → Block
    //
    // Traces: AC-019 / ADR-025 §12.2 / BC-5.40.001 PC4 / v1.6
    //
    // Scenario:
    //   On-disk STATE.md:  `timestamp: "2026-06-11T10:00:00Z"` (real value)
    //   Proposed STATE.md: `timestamp: ""` (empty string — field present but empty)
    //
    // Current behaviour: `extract_yaml_string_value` returns `Some("")` for an
    // empty-quoted value. `extract_top_level_field` returns `FieldResult::Found("")`.
    // In guard_logic Step 4, `proposed_ts = ""`. In Step 6, `"" != on_disk_ts`
    // (they differ) → the guard falls through to Continue.
    //
    // An empty timestamp is NOT a valid advancement. The guard must treat
    // `proposed_ts.is_empty()` as equivalent to `NotFound` — Block: TimestampStale.
    //
    // MUST FAIL against current impl: `"" != on_disk_ts` → Continue.
    // After the implementer adds the empty-check in Step 4/6, this becomes GREEN.
    // -----------------------------------------------------------------------

    #[test]
    fn test_timestamp_empty_string_in_proposed_blocks() {
        // On-disk: real timestamp.
        let on_disk = state_md_no_lock(TS_OLD);
        // Proposed: timestamp field present but with an empty-string value.
        // `extract_yaml_string_value` returns `Some("")` for `timestamp: ""`.
        let proposed = "---\ndocument_type: state\nversion: \"0.0.1-test\"\ntimestamp: \"\"\nphase: test\n---\n\n# STATE\n".to_string();

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_write(&proposed);

        let result = guard_logic(payload, callbacks);

        let expected_msg = canonical_timestamp_stale_message();
        match result {
            HookResult::Block { reason } => {
                assert_eq!(
                    reason, expected_msg,
                    "test_timestamp_empty_string_in_proposed_blocks: \
                     proposed timestamp: \"\" (empty string) must Block: TimestampStale. \
                     An empty timestamp is not a valid advancement. \
                     Expected: {expected_msg:?}. Got: {reason:?}"
                );
            }
            HookResult::Continue => panic!(
                "test_timestamp_empty_string_in_proposed_blocks: \
                 proposed has timestamp: \"\" (empty string) — must Block: TimestampStale. \
                 Got Continue instead. RED GATE v1.6: current impl extracts empty string \
                 via FieldResult::Found(\"\") and then falls through Step 6 \
                 (\"\" != on_disk_ts → differs → Continue). \
                 Implementer must add empty-timestamp check: \
                 if proposed_ts.is_empty() → Block: TimestampStale (AC-019 / ADR-025 §12.2)."
            ),
            other => panic!(
                "test_timestamp_empty_string_in_proposed_blocks: \
                 expected Block, got: {:?}",
                other
            ),
        }
    }

    // -----------------------------------------------------------------------
    // Fixture builders — lock-held STATE.md variants with absent/empty expires_at
    //
    // These fixtures support the AC-016/AC-017 tests added in v1.4.
    // The standard `state_md_with_lock` always emits a full valid lock block.
    // These variants omit or empty the `expires_at` field to drive the
    // expired/absent blocking paths implemented per ADR-025 §12.2 / AC-016.
    // -----------------------------------------------------------------------

    /// Build STATE.md content: lock held (holder present), expires_at LINE ABSENT.
    ///
    /// The lock block has `holder` and `locked_at` but NO `expires_at:` line at all.
    /// `extract_lock_subfields` returns `LockSubfields { holder: Some(..), expires_at: None }`.
    ///
    /// GREEN: guard_logic Step 7 detects `holder` present + `expires_at` None and
    /// returns `Block { reason: canonical_lock_expiry_stale_message() }` (AC-016).
    fn state_md_lock_expires_absent(timestamp: &str) -> String {
        format!(
            concat!(
                "---\n",
                "document_type: state\n",
                "version: \"0.0.1-test\"\n",
                "timestamp: \"{ts}\"\n",
                "phase: test\n",
                "factory_lock:\n",
                "  holder: \"{holder}\"\n",
                "  locked_at: \"2026-06-11T10:00:00Z\"\n",
                // NOTE: NO expires_at line — extract_lock_subfields returns expires_at: None
                "---\n\n# STATE\n",
            ),
            ts = timestamp,
            holder = HOLDER,
        )
    }

    /// Build STATE.md content: lock held (holder present), expires_at EMPTY STRING.
    ///
    /// The lock block has `holder`, `locked_at`, and `expires_at: ""`.
    /// `extract_lock_subfields` returns `LockSubfields { holder: Some(..), expires_at: Some("") }`.
    ///
    /// GREEN: guard_logic Step 7 detects `holder` present + `expires_at` empty string and
    /// returns `Block { reason: canonical_lock_expiry_stale_message() }` (AC-016/AC-017).
    fn state_md_lock_expires_empty(timestamp: &str) -> String {
        format!(
            concat!(
                "---\n",
                "document_type: state\n",
                "version: \"0.0.1-test\"\n",
                "timestamp: \"{ts}\"\n",
                "phase: test\n",
                "factory_lock:\n",
                "  holder: \"{holder}\"\n",
                "  locked_at: \"2026-06-11T10:00:00Z\"\n",
                "  expires_at: \"\"\n", // empty string → extract_lock_subfields returns expires_at: Some("")
                "---\n\n# STATE\n",
            ),
            ts = timestamp,
            holder = HOLDER,
        )
    }

    /// Produces a STATE.md with lock held and `expires_at: "   "` (whitespace-only).
    ///
    /// Used by L4-regression tests: `proposed_expires.trim().is_empty()` must Block
    /// even when the raw field value is non-empty (three spaces). Ensures `.is_empty()`
    /// cannot silently regress back (L4 / BC-5.40.001 / ADR-025 §12.2).
    fn state_md_lock_expires_whitespace(timestamp: &str) -> String {
        format!(
            concat!(
                "---\n",
                "document_type: state\n",
                "version: \"0.0.1-test\"\n",
                "timestamp: \"{ts}\"\n",
                "phase: test\n",
                "factory_lock:\n",
                "  holder: \"{holder}\"\n",
                "  locked_at: \"2026-06-11T10:00:00Z\"\n",
                "  expires_at: \"   \"\n", // whitespace-only → trim().is_empty() → Block
                "---\n\n# STATE\n",
            ),
            ts = timestamp,
            holder = HOLDER,
        )
    }

    // -----------------------------------------------------------------------
    // (t) AC-016 — lock held + expires_at ABSENT → Block LockExpiryStale
    //
    // Traces: AC-016 (v1.4) / ADR-025 §12.2 / BC-5.40.001 PC4
    //
    // Scenario:
    //   - On-disk STATE.md: lock held, valid expires_at (EXPIRES_OLD).
    //   - Proposed STATE.md: lock held (holder present), NO expires_at line.
    //     Timestamp is ADVANCED (TS_NEW) — the Block is specifically about expires_at.
    //   - extract_lock_subfields(proposed) returns
    //     LockSubfields { holder: Some(..), expires_at: None }.
    //
    // Required result: Block with FULL canonical LockExpiryStale message.
    //
    // GREEN: guard_logic Step 7 uses extract_lock_subfields (not parse_factory_lock),
    //   detects holder present + expires_at None → Block: LockExpiryStale.
    //
    // AC-016 rationale: an absent expires_at is MORE dangerous than a stale one —
    // the TTL enforcement window is undefined. Fail-closed is the correct behaviour.
    // -----------------------------------------------------------------------

    #[test]
    fn test_lock_held_expires_at_absent_blocks() {
        // On-disk: valid lock with full expires_at (EXPIRES_OLD).
        let on_disk = state_md_with_lock(TS_OLD, EXPIRES_OLD);
        // Proposed: lock held (holder present), timestamp ADVANCED (not stale),
        // but NO expires_at line at all.
        let proposed = state_md_lock_expires_absent(TS_NEW);

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        // Write payload: proposed content has holder but no expires_at.
        let payload = payload_write(&proposed);

        let result = guard_logic(payload, callbacks);

        let expected_msg = canonical_lock_expiry_stale_message();
        match result {
            HookResult::Block { reason } => {
                assert_eq!(
                    reason, expected_msg,
                    "test_lock_held_expires_at_absent_blocks: \
                     proposed content has lock holder present but NO expires_at line. \
                     The guard must Block with FULL canonical LockExpiryStale message \
                     (absent expires_at is more dangerous than stale — fail-closed). \
                     AC-016 / ADR-025 §12.2 / BC-5.40.001 PC4. \
                     Expected: {expected_msg:?}. Got: {reason:?}"
                );
            }
            HookResult::Continue => panic!(
                "test_lock_held_expires_at_absent_blocks: \
                 expected Block(LockExpiryStale) but got Continue. \
                 RED GATE v1.4: current impl routes parse Err(_) on proposed → None → Continue. \
                 Implementer must detect Err(MalformedLockBlock) on proposed when holder is present \
                 and Block: LockExpiryStale (AC-016 / ADR-025 §12.2)."
            ),
            other => panic!(
                "test_lock_held_expires_at_absent_blocks: expected Block, got: {:?}",
                other
            ),
        }
    }

    // -----------------------------------------------------------------------
    // (u) AC-016/017 — lock held + expires_at EMPTY → Block LockExpiryStale
    //
    // Traces: AC-016 / AC-017 (v1.4) / ADR-025 §12.2 / BC-5.40.001 PC4
    //
    // Scenario:
    //   - On-disk STATE.md: lock held, valid expires_at (EXPIRES_OLD).
    //   - Proposed STATE.md: lock held (holder present), `expires_at: ""`
    //     (empty string). Timestamp is ADVANCED (TS_NEW).
    //   - extract_lock_subfields(proposed) returns
    //     LockSubfields { holder: Some(..), expires_at: Some("") }.
    //
    // Required result: Block with FULL canonical LockExpiryStale message.
    //
    // GREEN: guard_logic Step 7 detects holder present + expires_at empty string
    //   → Block: LockExpiryStale.
    //
    // AC-017: empty expires_at is treated identically to absent — TTL undefined,
    // fail-closed is the correct behaviour.
    // -----------------------------------------------------------------------

    #[test]
    fn test_lock_held_expires_at_empty_blocks() {
        // On-disk: valid lock with full expires_at (EXPIRES_OLD).
        let on_disk = state_md_with_lock(TS_OLD, EXPIRES_OLD);
        // Proposed: lock held (holder present), timestamp ADVANCED,
        // but expires_at is empty string.
        let proposed = state_md_lock_expires_empty(TS_NEW);

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_write(&proposed);

        let result = guard_logic(payload, callbacks);

        let expected_msg = canonical_lock_expiry_stale_message();
        match result {
            HookResult::Block { reason } => {
                assert_eq!(
                    reason, expected_msg,
                    "test_lock_held_expires_at_empty_blocks: \
                     proposed content has lock holder present but expires_at is empty string. \
                     The guard must Block with FULL canonical LockExpiryStale message \
                     (empty expires_at has undefined TTL — fail-closed). \
                     AC-016/017 / ADR-025 §12.2 / BC-5.40.001 PC4. \
                     Expected: {expected_msg:?}. Got: {reason:?}"
                );
            }
            HookResult::Continue => panic!(
                "test_lock_held_expires_at_empty_blocks: \
                 expected Block(LockExpiryStale) but got Continue. \
                 RED GATE v1.4: current impl routes parse Err(_) on proposed → None → Continue. \
                 Implementer must detect Err(MalformedLockBlock) on proposed when holder is present \
                 (holder extracted before expires_at parse fails) and Block: LockExpiryStale \
                 (AC-016/017 / ADR-025 §12.2)."
            ),
            other => panic!(
                "test_lock_held_expires_at_empty_blocks: expected Block, got: {:?}",
                other
            ),
        }
    }

    // -----------------------------------------------------------------------
    // Continue control — lock held + expires_at DIFFERENT from on-disk → Continue
    //
    // Traces: AC-006 (inverse — not stale), BC-5.40.001 PC6 (no over-blocking)
    //
    // This is the GREEN control that must PASS against both current AND fixed impl.
    // It proves the guard does NOT over-block when both proposed and on-disk have
    // valid, different expires_at values (lock was properly renewed).
    //
    // On-disk: lock held, EXPIRES_OLD. Proposed: lock held, EXPIRES_NEW (advanced).
    // Timestamp also advanced. Expected: Continue.
    //
    // If this test FAILS, the implementer over-blocked valid renewal. Must remain GREEN.
    // -----------------------------------------------------------------------

    #[test]
    fn test_lock_held_expires_at_different_continues() {
        // On-disk: lock held with EXPIRES_OLD.
        let on_disk = state_md_with_lock(TS_OLD, EXPIRES_OLD);
        // Proposed: same lock, but BOTH timestamp AND expires_at advanced.
        let proposed = state_md_with_lock(TS_NEW, EXPIRES_NEW);

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_write(&proposed);

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "test_lock_held_expires_at_different_continues: \
             lock held, timestamp AND expires_at both advanced → must Continue. \
             The guard must NOT over-block valid mid-burst heartbeat renewal. \
             AC-006 (inverse) / BC-5.40.001 PC6 (fail-open success path)."
        );
    }

    // -----------------------------------------------------------------------
    // (v) EC-006 — double-dot relative path resolves → Block
    //
    // Traces: EC-006 / ADR-025 §12.7 R6 (.. segment resolution) / v1.4
    //
    // Scenario:
    //   file_path = "foo/../.factory/STATE.md"
    //
    // Correct normalisation (segment-stack algorithm):
    //   "foo/../.factory/STATE.md"
    //   → process segments: "foo" → push; ".." → pop "foo" → empty; ".factory" → push;
    //     "STATE.md" → push → ".factory/STATE.md"
    //   → Result: ".factory/STATE.md" → guard triggers.
    //
    // GREEN: normalise_path segment-stack algorithm resolves ".." segments.
    //   "foo/../.factory/STATE.md" → ".factory/STATE.md" → guard reads STATE.md
    //   and blocks on stale timestamp (TimestampStale).
    // -----------------------------------------------------------------------

    #[test]
    fn test_double_dot_relative_path_triggers_guard() {
        // On-disk: old timestamp.
        let on_disk = state_md_no_lock(TS_OLD);
        // Proposed: stale timestamp (not advanced) → Block TimestampStale.
        let stale_proposed = state_md_no_lock(TS_OLD);

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());

        // Path variant: double-dot traversal that resolves to .factory/STATE.md.
        // "foo/../.factory/STATE.md" → segment stack: push "foo", pop on "..",
        // push ".factory", push "STATE.md" → ".factory/STATE.md".
        let path = "foo/../.factory/STATE.md";
        let payload = payload_write_path_variant(path, &stale_proposed);

        let result = guard_logic(payload, callbacks);

        let expected_msg = canonical_timestamp_stale_message();
        match result {
            HookResult::Block { reason } => {
                assert_eq!(
                    reason, expected_msg,
                    "test_double_dot_relative_path_triggers_guard: \
                     path '{path}' must normalise to .factory/STATE.md via segment-stack .. resolution \
                     and Block on stale timestamp. \
                     Expected: {expected_msg:?}. Got: {reason:?}"
                );
            }
            HookResult::Continue => panic!(
                "test_double_dot_relative_path_triggers_guard: \
                 path 'foo/../.factory/STATE.md' must normalise to '.factory/STATE.md' via \
                 segment-stack .. resolution and Block. Got Continue instead. \
                 RED GATE v1.4: current normalise_path does not resolve '..' segments. \
                 Implementer must add segment-stack algorithm to normalise_path \
                 (EC-006 / ADR-025 §12.7 R6)."
            ),
            other => panic!(
                "test_double_dot_relative_path_triggers_guard: expected Block, got: {:?}",
                other
            ),
        }
    }

    // -----------------------------------------------------------------------
    // (w) EC-006 — above-root double-dot discarded → Block
    //
    // Traces: EC-006 / ADR-025 §12.7 R6 (above-root .. clamped) / v1.4
    //
    // Scenario:
    //   file_path = "../../.factory/STATE.md"
    //
    // Correct normalisation (segment-stack with above-root clamping):
    //   Segments: "..", "..", ".factory", "STATE.md"
    //   Stack processing:
    //     ".."  → stack is empty → clamp (discard; cannot go above root)
    //     ".."  → stack is empty → clamp
    //     ".factory" → push
    //     "STATE.md" → push
    //   → Result: ".factory/STATE.md" → guard triggers.
    //
    // This matches POSIX path normalisation: you cannot `..` above the
    // relative root. Paths like "../../.factory/STATE.md" collapse to
    // ".factory/STATE.md" after above-root clamping.
    //
    // GREEN: normalise_path segment-stack algorithm applies the same above-root
    // clamping as (v). "../../.factory/STATE.md" → ".factory/STATE.md" → guard
    // reads STATE.md and blocks on stale timestamp (TimestampStale).
    // Above-root ".." is discarded (clamped to empty stack), not surfaced
    // as an error — fail-closed would break legitimate paths where a
    // tool emits absolute paths that start with "../" relative to cwd.
    // -----------------------------------------------------------------------

    // -----------------------------------------------------------------------
    // L4 regression — whitespace-only proposed timestamp → Block TimestampStale
    //
    // Traces: AC-019 (v1.6 L4) / ADR-025 §12.2 / BC-5.40.001 PC4
    //
    // Scenario:
    //   - On-disk STATE.md: valid timestamp (TS_OLD).
    //   - Proposed STATE.md: `timestamp: "   "` (three spaces — whitespace-only).
    //   - extract_top_level_field returns FieldResult::Found("   ").
    //   - proposed_ts.trim().is_empty() → true → Block TimestampStale.
    //
    // GREEN: guard_logic Step 4 checks `proposed_ts.trim().is_empty()` (L4 fix).
    //   A whitespace-only timestamp is not a valid RFC-3339 value; the guard
    //   must fail-closed with Block(TimestampStale).
    //
    // Load-bearing: if impl reverts to `.is_empty()`, a " " value passes the
    // empty check, differs from on-disk TS_OLD, and returns Continue. This test
    // would then FAIL, catching the regression.
    // -----------------------------------------------------------------------

    #[test]
    fn test_timestamp_whitespace_only_in_proposed_blocks() {
        let on_disk = state_md_no_lock(TS_OLD);
        // Proposed: `timestamp: "   "` — three spaces. Not a valid RFC-3339 value.
        // Build by replacing the timestamp field inline.
        let proposed = concat!(
            "---\n",
            "document_type: state\n",
            "version: \"0.0.1-test\"\n",
            "timestamp: \"   \"\n",
            "phase: test\n",
            "---\n\n# STATE\n",
        );

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_write(proposed);

        let result = guard_logic(payload, callbacks);

        let expected_msg = canonical_timestamp_stale_message();
        match result {
            HookResult::Block { reason } => {
                assert_eq!(
                    reason, expected_msg,
                    "test_timestamp_whitespace_only_in_proposed_blocks: \
                     proposed timestamp is whitespace-only (\"   \"); \
                     trim().is_empty() must → Block TimestampStale (L4 / AC-019). \
                     Expected: {expected_msg:?}. Got: {reason:?}"
                );
            }
            HookResult::Continue => panic!(
                "test_timestamp_whitespace_only_in_proposed_blocks: \
                 got Continue — impl regressed to .is_empty() check. \
                 Whitespace-only is not a valid RFC-3339 timestamp; must Block. \
                 L4 regression (AC-019 / ADR-025 §12.2)."
            ),
            other => panic!(
                "test_timestamp_whitespace_only_in_proposed_blocks: expected Block, got: {:?}",
                other
            ),
        }
    }

    // -----------------------------------------------------------------------
    // L4 regression — whitespace-only proposed expires_at → Block LockExpiryStale
    //
    // Traces: AC-016/017 (v1.4/v1.6 L4) / ADR-025 §12.2 / BC-5.40.001 PC4
    //
    // Scenario:
    //   - On-disk STATE.md: lock held, valid expires_at (EXPIRES_OLD).
    //   - Proposed STATE.md: lock held (holder present), timestamp ADVANCED (TS_NEW),
    //     `expires_at: "   "` (three spaces — whitespace-only).
    //   - extract_lock_subfields returns expires_at: Some("   ").
    //   - proposed_expires.trim().is_empty() → true → Block LockExpiryStale.
    //
    // GREEN: guard_logic Step 7 checks `proposed_expires.trim().is_empty()` (L4 fix).
    //   A whitespace-only expires_at is not a valid RFC-3339 value; the TTL
    //   enforcement window is undefined → fail-closed with Block(LockExpiryStale).
    //
    // Load-bearing: if impl reverts to `.is_empty()`, whitespace-only passes the
    // empty check, differs from on-disk EXPIRES_OLD, and returns Continue. This
    // test would then FAIL, catching the regression.
    // -----------------------------------------------------------------------

    #[test]
    fn test_lock_held_expires_whitespace_only_blocks() {
        let on_disk = state_md_with_lock(TS_OLD, EXPIRES_OLD);
        // Proposed: timestamp advanced (TS_NEW), but expires_at is whitespace-only.
        let proposed = state_md_lock_expires_whitespace(TS_NEW);

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_write(&proposed);

        let result = guard_logic(payload, callbacks);

        let expected_msg = canonical_lock_expiry_stale_message();
        match result {
            HookResult::Block { reason } => {
                assert_eq!(
                    reason, expected_msg,
                    "test_lock_held_expires_whitespace_only_blocks: \
                     proposed expires_at is whitespace-only (\"   \"); \
                     trim().is_empty() must → Block LockExpiryStale (L4 / AC-016/017). \
                     Expected: {expected_msg:?}. Got: {reason:?}"
                );
            }
            HookResult::Continue => panic!(
                "test_lock_held_expires_whitespace_only_blocks: \
                 got Continue — impl regressed to .is_empty() check. \
                 Whitespace-only is not a valid RFC-3339 expires_at; must Block. \
                 L4 regression (AC-016/017 / ADR-025 §12.2)."
            ),
            other => panic!(
                "test_lock_held_expires_whitespace_only_blocks: expected Block, got: {:?}",
                other
            ),
        }
    }

    // -----------------------------------------------------------------------
    // L2 regression — log_warn fires on fail-open: read error path
    //
    // Traces: L2 (v1.6) / ADR-025 §12.5 / BC-5.40.001 PC4 (observability)
    //
    // Scenario:
    //   - On-disk read returns Err("simulated read error").
    //   - Guard cannot read on-disk timestamp → fail-open Continue.
    //   - log_warn MUST be called with a token containing "fail-open read-error"
    //     so that fail-open events are observable in production logs.
    //
    // GREEN: guard_logic Step 2 calls log_warn("fail-open read-error ...") before
    //   returning Continue. warn_log Arc captures the call.
    //
    // Load-bearing: if log_warn is removed from the fail-open path, warn_log
    // remains empty and this test FAILS, catching the regression.
    // -----------------------------------------------------------------------

    #[test]
    fn test_log_warn_fires_on_read_error_fail_open() {
        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_read_error("simulated read error", warn_log.clone());

        // Any STATE.md write payload — guard will fail-open before reaching timestamp check.
        let proposed = state_md_no_lock(TS_NEW);
        let payload = payload_write(&proposed);

        let result = guard_logic(payload, callbacks);

        // Guard must Continue (fail-open).
        assert_eq!(
            result,
            HookResult::Continue,
            "test_log_warn_fires_on_read_error_fail_open: \
             read error must produce fail-open Continue (not Block). L2."
        );

        // log_warn must have been called at least once.
        let log = warn_log.lock().unwrap();
        assert!(
            !log.is_empty(),
            "test_log_warn_fires_on_read_error_fail_open: \
             warn_log is empty — log_warn was NOT called on read-error fail-open path. \
             L2 regression: guard_logic Step 2 must call log_warn with \
             'fail-open read-error' token."
        );

        // The warn message must contain the canonical token.
        let has_token = log.iter().any(|m| m.contains("fail-open read-error"));
        assert!(
            has_token,
            "test_log_warn_fires_on_read_error_fail_open: \
             log_warn was called but no entry contains 'fail-open read-error' token. \
             Got: {:?}. L2 regression (ADR-025 §12.5).",
            *log
        );
    }

    // -----------------------------------------------------------------------
    // L2 regression — log_warn fires on fail-open: malformed proposed content
    //
    // Traces: L2 (v1.6) / ADR-025 §12.5 / BC-5.40.001 PC4 (observability)
    //
    // Scenario:
    //   - On-disk STATE.md: valid content (TS_OLD).
    //   - Proposed content: malformed frontmatter (no parseable timestamp field).
    //   - extract_top_level_field returns FieldResult::NotFound → guard cannot
    //     determine proposed timestamp → fail-open Continue.
    //   - log_warn MUST be called with a token containing "fail-open malformed-proposed".
    //
    // GREEN: guard_logic Step 4 calls log_warn("fail-open malformed-proposed ...") when
    //   proposed content has no extractable timestamp field.
    //
    // Load-bearing: if log_warn is removed from this fail-open path, warn_log
    // remains empty and this test FAILS, catching the regression.
    // -----------------------------------------------------------------------

    #[test]
    fn test_log_warn_fires_on_malformed_proposed_fail_open() {
        let on_disk = state_md_no_lock(TS_OLD);
        // state_md_malformed produces content with no recognisable frontmatter fields.
        let proposed = state_md_malformed();

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_write(&proposed);

        let result = guard_logic(payload, callbacks);

        // Guard must Continue (fail-open on malformed proposed content).
        assert_eq!(
            result,
            HookResult::Continue,
            "test_log_warn_fires_on_malformed_proposed_fail_open: \
             malformed proposed content must produce fail-open Continue (not Block). L2."
        );

        // log_warn must have been called at least once.
        let log = warn_log.lock().unwrap();
        assert!(
            !log.is_empty(),
            "test_log_warn_fires_on_malformed_proposed_fail_open: \
             warn_log is empty — log_warn was NOT called on malformed-proposed fail-open path. \
             L2 regression: guard_logic Step 4 must call log_warn with \
             'fail-open malformed-proposed' token."
        );

        // The warn message must contain the canonical token.
        let has_token = log
            .iter()
            .any(|m| m.contains("fail-open malformed-proposed"));
        assert!(
            has_token,
            "test_log_warn_fires_on_malformed_proposed_fail_open: \
             log_warn was called but no entry contains 'fail-open malformed-proposed' token. \
             Got: {:?}. L2 regression (ADR-025 §12.5).",
            *log
        );
    }

    // -----------------------------------------------------------------------
    // M1 regression — canonical_timestamp_stale_message() == AC-005 literal
    //
    // Traces: AC-005 / M1 (v1.6) / ADR-025 §12.2 / BC-5.40.001 PC4
    //
    // Scenario: canonical_timestamp_stale_message() must produce EXACTLY the string
    //   mandated by AC-005, assembled via HookResult::block_with_fix (M1 fix).
    //
    // GREEN: M1 wires block_with_fix as the single format source. If the format
    //   changes (e.g., separator drift, code suffix removed), this test FAILS.
    //
    // Load-bearing: any format drift in GUARD_NAME / TIMESTAMP_STALE_REASON /
    //   TIMESTAMP_STALE_FIX / TIMESTAMP_STALE_CODE OR block_with_fix template
    //   causes this assertion to FAIL, preventing silent AC-005 drift.
    // -----------------------------------------------------------------------

    #[test]
    fn test_canonical_message_equals_ac005_literal() {
        let expected = concat!(
            "BLOCKED by verify-state-timestamp-refresh: ",
            "STATE.md timestamp not advanced in this write. ",
            "Fix: Update 'timestamp:' to the current UTC time before writing STATE.md. ",
            "Code: TimestampStale.",
        );
        let actual = canonical_timestamp_stale_message();
        assert_eq!(
            actual, expected,
            "test_canonical_message_equals_ac005_literal: \
             canonical_timestamp_stale_message() does not match AC-005 exact literal. \
             M1 regression: block_with_fix format or constant values have drifted. \
             Expected: {expected:?}. Got: {actual:?}"
        );
    }

    // -----------------------------------------------------------------------
    // M1 regression — canonical_lock_expiry_stale_message() == AC-006 literal
    //
    // Traces: AC-006 / M1 (v1.6) / ADR-025 §12.2 / BC-5.40.001 PC4
    //
    // Scenario: canonical_lock_expiry_stale_message() must produce EXACTLY the
    //   string mandated by AC-006, assembled via HookResult::block_with_fix (M1 fix).
    //
    // GREEN: M1 wires block_with_fix as the single format source. Any format drift
    //   in GUARD_NAME / LOCK_EXPIRY_STALE_REASON / LOCK_EXPIRY_STALE_FIX /
    //   LOCK_EXPIRY_STALE_CODE OR block_with_fix template causes this to FAIL.
    //
    // Load-bearing: prevents silent AC-006 drift across refactors.
    // -----------------------------------------------------------------------

    #[test]
    fn test_canonical_message_equals_ac006_literal() {
        let expected = concat!(
            "BLOCKED by verify-state-timestamp-refresh: ",
            "factory_lock.expires_at not refreshed in this write while lock is held. ",
            "Fix: Run: factory-lock-write.sh renew .factory/STATE.md before writing STATE.md. ",
            "Code: LockExpiryStale.",
        );
        let actual = canonical_lock_expiry_stale_message();
        assert_eq!(
            actual, expected,
            "test_canonical_message_equals_ac006_literal: \
             canonical_lock_expiry_stale_message() does not match AC-006 exact literal. \
             M1 regression: block_with_fix format or constant values have drifted. \
             Expected: {expected:?}. Got: {actual:?}"
        );
    }

    #[test]
    fn test_double_dot_above_root_path_triggers_guard() {
        let on_disk = state_md_no_lock(TS_OLD);
        let stale_proposed = state_md_no_lock(TS_OLD); // stale → Block TimestampStale.

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());

        // Path with above-root ".." — both leading ".." segments are discarded
        // by the segment-stack clamp, leaving ".factory/STATE.md".
        let path = "../../.factory/STATE.md";
        let payload = payload_write_path_variant(path, &stale_proposed);

        let result = guard_logic(payload, callbacks);

        let expected_msg = canonical_timestamp_stale_message();
        match result {
            HookResult::Block { reason } => {
                assert_eq!(
                    reason, expected_msg,
                    "test_double_dot_above_root_path_triggers_guard: \
                     path '{path}' must normalise to .factory/STATE.md via above-root \
                     .. clamping and Block on stale timestamp. \
                     Expected: {expected_msg:?}. Got: {reason:?}"
                );
            }
            HookResult::Continue => panic!(
                "test_double_dot_above_root_path_triggers_guard: \
                 path '../../.factory/STATE.md' must normalise to '.factory/STATE.md' via \
                 segment-stack with above-root clamping and Block. Got Continue instead. \
                 RED GATE v1.4: current normalise_path does not resolve '..' segments. \
                 Implementer must add above-root-clamped segment-stack algorithm \
                 (EC-006 / ADR-025 §12.7 R6)."
            ),
            other => panic!(
                "test_double_dot_above_root_path_triggers_guard: expected Block, got: {:?}",
                other
            ),
        }
    }

    // -----------------------------------------------------------------------
    // S-19.08 Red Gate tests: T-001, T-002, T-003, T-004, T-005, T-007
    //
    // These tests FAIL against current production code and PASS only after the
    // S-19.08 implementation tasks complete:
    //   Task 9:  Raise STATE_MD_MAX_BYTES to 262144 (closes T-001, T-002, T-003, T-005)
    //   Task 10: Wire factory_lock_parse::extract_frontmatter (closes T-004)
    //   Task 11: Implement state_md_approaching_cap soft-warn emission (closes T-007 A/D)
    //
    // BC Traces: BC-5.40.001 v1.2 Precondition 6 (T-001/T-002/T-003/T-005),
    //            Invariant 7 (T-004/T-005), Invariant 8 (T-007).
    //
    // NOTE on T-005: The behavioural assertion (guard returns Continue on
    // malformed/no-delimiter on-disk content) already holds in current code.
    // The Red Gate for T-005 is the STATE_MD_MAX_BYTES >= 70_000 pre-condition,
    // which fails until Task 9. T-005 is a regression-prevention test that
    // confirms the fail-open behaviour is preserved after the cap-raise + wiring
    // fix. Flagged as expected: not a tautological-test defect — the pre-condition
    // is the intentional gate.
    // -----------------------------------------------------------------------

    // -----------------------------------------------------------------------
    // Fixture helpers for S-19.08 tests
    // -----------------------------------------------------------------------

    /// Build STATE.md bytes padded to `target_size` with `timestamp: "<ts>"` in
    /// frontmatter. The closing `---` delimiter is present; the body is padded
    /// with `# padding\n` comment lines (all valid ASCII/UTF-8).
    ///
    /// Used by T-002/T-003 (70 KiB fixture) and T-007 (soft-warn boundary fixtures).
    fn state_md_padded_with_timestamp_bytes(ts: &str, target_size: usize) -> Vec<u8> {
        let header = format!(
            "---\ndocument_type: state\nversion: \"0.0.1-test\"\ntimestamp: \"{}\"\nphase: test\n---\n\n# STATE\n",
            ts
        );
        let mut bytes = header.into_bytes();
        let pad_line = b"# padding\n";
        while bytes.len() < target_size {
            let remaining = target_size - bytes.len();
            if remaining >= pad_line.len() {
                bytes.extend_from_slice(pad_line);
            } else {
                // Partial pad with '#' bytes to reach exactly target_size.
                bytes.extend(std::iter::repeat_n(b'#', remaining));
            }
        }
        bytes.truncate(target_size);
        bytes
    }

    /// Build callbacks where `read_file` returns the given bytes unconditionally.
    ///
    /// The `max_bytes` argument from the guard is IGNORED — bytes are returned
    /// regardless of cap. This isolates behavioural logic from cap-enforcement.
    /// The Red Gate for T-002/T-003/T-005 comes from the `STATE_MD_MAX_BYTES >= 70_000`
    /// pre-condition assertion, NOT from cap-enforcement at the mock level.
    #[allow(clippy::type_complexity)]
    fn make_callbacks_with_raw_bytes(
        on_disk_bytes: Vec<u8>,
        warn_log: Arc<Mutex<Vec<String>>>,
    ) -> GuardCallbacks<
        impl FnOnce(&str, u32, u32) -> Result<Vec<u8>, String>,
        impl FnMut(&str),
        impl FnMut(&str),
    > {
        let wl = warn_log.clone();
        GuardCallbacks {
            read_file: move |_path, _max, _timeout| Ok(on_disk_bytes),
            log_warn: move |msg: &str| {
                wl.lock().unwrap().push(msg.to_string());
            },
            write_stderr: |_msg| {},
        }
    }

    // -----------------------------------------------------------------------
    // T-001 (AC-001): STATE_MD_MAX_BYTES == 262144
    //
    // BC Trace: BC-5.40.001 v1.2 Precondition 6 (max_bytes = 262144).
    // RED: current value is 65536; assert_eq! fails until Task 9.
    // -----------------------------------------------------------------------

    /// T-001 (AC-001): `STATE_MD_MAX_BYTES` must equal 262144 (256 KiB).
    ///
    /// BC-5.40.001 v1.2 Precondition 6 mandates `STATE_MD_MAX_BYTES = 262144`
    /// for the `verify-state-timestamp-refresh` guard. Mirrors BC-4.13.001
    /// Phase-A Precondition 3 + ADR-025 §Decision 12 §12.5 parity with
    /// `verify-factory-lock` (S-19.02).
    ///
    /// RED: current value is 65536; assert_eq! fails until Task 9 (implementer).
    #[test]
    fn test_BC_5_40_001_T001_state_md_max_bytes_is_262144() {
        assert_eq!(
            STATE_MD_MAX_BYTES, 262144u32,
            "T-001 (AC-001): STATE_MD_MAX_BYTES must equal 262144 (256 KiB) per \
             BC-5.40.001 v1.2 Precondition 6 / ADR-025 §Decision 12 §12.5. \
             Current value: {}",
            STATE_MD_MAX_BYTES
        );
    }

    // -----------------------------------------------------------------------
    // T-002 (AC-002): 70 KiB fixture + stale timestamp → TimestampStale block
    //
    // BC Trace: BC-5.40.001 PC4 (operational at new cap).
    // RED: assert!(STATE_MD_MAX_BYTES >= 70_000) fails with 65536 (Task 9).
    // -----------------------------------------------------------------------

    /// T-002 (AC-002): 70 KiB fixture with stale timestamp → Block(TimestampStale).
    ///
    /// AC-002: plugin reads STATE.md successfully when the file is between 64 KiB
    /// and 256 KiB; correctly detects a stale timestamp and returns block intent.
    ///
    /// - On-disk: 70000-byte STATE.md with `timestamp: TS_OLD` in frontmatter.
    /// - Proposed (Write): stale `timestamp: TS_OLD` (same as on-disk).
    /// - Expected: Block(TimestampStale) — guard ran to completion.
    ///
    /// RED: `assert!(STATE_MD_MAX_BYTES >= 70_000u32)` fails with current cap 65536.
    #[test]
    fn test_BC_5_40_001_T002_70kib_fixture_stale_timestamp_blocks() {
        // Primary Red Gate: cap must be at least 70 KiB for this test to exercise
        // raised-cap behaviour. Fails until Task 9 raises cap to 262144.
        assert!(
            STATE_MD_MAX_BYTES >= 70_000u32,
            "T-002 (AC-002): STATE_MD_MAX_BYTES ({}) must be >= 70000. \
             Raise to 262144 per BC-5.40.001 v1.2 Precondition 6.",
            STATE_MD_MAX_BYTES
        );

        let fixture_bytes = state_md_padded_with_timestamp_bytes(TS_OLD, 70_000);
        assert_eq!(
            fixture_bytes.len(),
            70_000,
            "fixture must be exactly 70000 bytes"
        );

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_raw_bytes(fixture_bytes, warn_log.clone());
        // Proposed: Write payload with STALE timestamp (same as on-disk TS_OLD).
        let proposed_stale = state_md_no_lock(TS_OLD);
        let payload = payload_write(&proposed_stale);

        let result = guard_logic(payload, callbacks);

        let expected_msg = canonical_timestamp_stale_message();
        match result {
            HookResult::Block { reason } => {
                assert_eq!(
                    reason, expected_msg,
                    "T-002: Block reason must be full canonical TimestampStale string. \
                     Expected: {expected_msg:?}. Got: {reason:?}"
                );
            }
            HookResult::Continue => panic!(
                "T-002 (AC-002): 70 KiB fixture with stale timestamp must return \
                 Block(TimestampStale). Got Continue. Guard must be operational at \
                 the new 262144 cap (BC-5.40.001 PC4)."
            ),
            other => panic!("T-002: unexpected result: {:?}", other),
        }
    }

    // -----------------------------------------------------------------------
    // T-003 (AC-002): 70 KiB fixture + advanced timestamp → Continue
    //
    // BC Trace: BC-5.40.001 PC4 success path / PC6 (single-dev zero friction).
    // RED: same STATE_MD_MAX_BYTES >= 70_000 pre-condition fails until Task 9.
    // -----------------------------------------------------------------------

    /// T-003 (AC-002): 70 KiB fixture with advanced timestamp → Continue.
    ///
    /// AC-002 complement: guard returns Continue (allow write) when the timestamp
    /// is advanced on a STATE.md between 64 KiB and 256 KiB.
    ///
    /// - On-disk: 70000-byte STATE.md with `timestamp: TS_OLD`.
    /// - Proposed (Write): advanced `timestamp: TS_NEW`.
    /// - Expected: Continue — guard ran to completion; write is permitted.
    ///
    /// RED: `assert!(STATE_MD_MAX_BYTES >= 70_000u32)` fails with current cap 65536.
    #[test]
    fn test_BC_5_40_001_T003_70kib_fixture_advanced_timestamp_continues() {
        assert!(
            STATE_MD_MAX_BYTES >= 70_000u32,
            "T-003 (AC-002): STATE_MD_MAX_BYTES ({}) must be >= 70000. \
             Raise to 262144 per BC-5.40.001 v1.2 Precondition 6.",
            STATE_MD_MAX_BYTES
        );

        let fixture_bytes = state_md_padded_with_timestamp_bytes(TS_OLD, 70_000);
        assert_eq!(
            fixture_bytes.len(),
            70_000,
            "fixture must be exactly 70000 bytes"
        );

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_raw_bytes(fixture_bytes, warn_log.clone());
        // Proposed: Write payload with ADVANCED timestamp.
        let proposed_advanced = state_md_no_lock(TS_NEW);
        let payload = payload_write(&proposed_advanced);

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "T-003 (AC-002): 70 KiB fixture with advanced timestamp must return Continue. \
             Guard must allow the write when timestamp is advanced (BC-5.40.001 PC6)."
        );
    }

    // -----------------------------------------------------------------------
    // T-004 (AC-003): extract_frontmatter wired — body bytes excluded from parsed slice
    //
    // BC Trace: BC-5.40.001 v1.2 Invariant 7 (extract_frontmatter exclusive use).
    //
    // Observable Red Gate: without extract_frontmatter wiring, String::from_utf8
    // on the FULL on-disk bytes (including non-UTF-8 body) fails → guard returns
    // Continue (fail-open). The test asserts Block(TimestampStale) → FAILS.
    //
    // After fix: extract_frontmatter strips non-UTF-8 body → frontmatter-only
    // bytes are valid UTF-8 → timestamp extracted → stale → Block.
    // -----------------------------------------------------------------------

    /// T-004 (AC-003): `extract_frontmatter` wired — non-UTF-8 body excluded from parse.
    ///
    /// Fixture:
    /// - On-disk bytes: valid UTF-8 frontmatter with `timestamp: TS_OLD`,
    ///   closing `---` delimiter, then non-UTF-8 body bytes (`\xFF\xFE`).
    /// - Proposed (Write): stale `timestamp: TS_OLD`.
    ///
    /// Without `extract_frontmatter` wiring (current):
    ///   `String::from_utf8(full_bytes)` fails on `\xFF\xFE` body bytes →
    ///   guard returns Continue (fail-open). Test asserts Block → FAILS.
    ///
    /// After fix (Task 10 — extract_frontmatter wired):
    ///   `extract_frontmatter` returns frontmatter-only bytes (valid UTF-8);
    ///   re-attach `\n---\n`; `String::from_utf8` succeeds; timestamp extracted;
    ///   stale → Block(TimestampStale).
    ///
    /// RED: guard currently returns Continue; test asserts Block → ASSERTION FAILS.
    #[test]
    fn test_BC_5_40_001_T004_extract_frontmatter_wired_body_bytes_excluded() {
        // Build on-disk fixture: valid frontmatter + closing --- + non-UTF-8 body.
        // state_md_no_lock(TS_OLD) produces:
        //   "---\n...\ntimestamp: \"TS_OLD\"\n...\n---\n\n# STATE\n"
        // We append non-UTF-8 bytes AFTER the valid content to simulate body bytes
        // that String::from_utf8 cannot handle on the full byte slice.
        let mut on_disk_bytes = state_md_no_lock(TS_OLD).into_bytes();
        // \xFF\xFE are invalid UTF-8 start bytes. These appear after the closing
        // `---` delimiter. extract_frontmatter must strip them. Without wiring,
        // String::from_utf8 fails on the full bytes → fail-open Continue (RED GATE).
        on_disk_bytes.extend_from_slice(b"\xFF\xFE non-utf8-body-bytes-must-not-reach-parser\n");

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_raw_bytes(on_disk_bytes, warn_log.clone());
        // Proposed: Write payload with STALE timestamp (same as on-disk TS_OLD).
        let proposed_stale = state_md_no_lock(TS_OLD);
        let payload = payload_write(&proposed_stale);

        let result = guard_logic(payload, callbacks);

        // Expected: Block(TimestampStale).
        // extract_frontmatter strips non-UTF-8 body; frontmatter bytes are valid
        // UTF-8; timestamp found → stale → Block.
        //
        // Current (without extract_frontmatter): String::from_utf8(full_bytes) Err
        // → fail-open Continue. This assertion FAILS → RED GATE.
        let expected_msg = canonical_timestamp_stale_message();
        match result {
            HookResult::Block { reason } => {
                assert_eq!(
                    reason, expected_msg,
                    "T-004: Block reason must equal canonical TimestampStale. \
                     Expected: {expected_msg:?}. Got: {reason:?}"
                );
            }
            HookResult::Continue => panic!(
                "T-004 (AC-003): Guard must Block on stale timestamp when on-disk bytes \
                 include non-UTF-8 body bytes after the closing delimiter. Got Continue. \
                 Root cause: factory_lock_parse::extract_frontmatter NOT wired — \
                 String::from_utf8 fails on full bytes → fail-open Continue. \
                 Fix: wire extract_frontmatter before String::from_utf8 \
                 (BC-5.40.001 Invariant 7, Task 10)."
            ),
            other => panic!("T-004: unexpected result: {:?}", other),
        }
    }

    // -----------------------------------------------------------------------
    // T-005 (AC-003): no delimiter → full content returned (fail-open)
    //
    // BC Trace: BC-5.40.001 v1.2 Invariant 7 (fail-open when delimiter absent).
    //
    // Verifies graceful degradation is PRESERVED after the extract_frontmatter
    // wiring fix: when on-disk content has no closing `---` delimiter,
    // extract_frontmatter returns the full bytes → guard returns Continue
    // (fail-open, no crash or hard error).
    //
    // RED GATE: assert!(STATE_MD_MAX_BYTES >= 70_000) fails until Task 9.
    // See module-level NOTE on T-005 for expected-pass rationale.
    // -----------------------------------------------------------------------

    /// T-005 (AC-003): no closing delimiter → guard returns Continue without error.
    ///
    /// Fixture: on-disk bytes with no closing `---` delimiter (malformed content).
    /// Proposed: Write payload with advanced `timestamp: TS_NEW`.
    ///
    /// BC-5.40.001 Invariant 7: when `extract_frontmatter` returns the full bytes
    /// (delimiter absent → fail-open per function contract), the guard proceeds
    /// gracefully and returns Continue (fail-open on malformed on-disk content).
    ///
    /// RED GATE: `assert!(STATE_MD_MAX_BYTES >= 70_000u32)` fails until Task 9.
    #[test]
    fn test_BC_5_40_001_T005_no_delimiter_full_content_fail_open() {
        // Red Gate pre-condition: same cap assertion as T-002/T-003.
        assert!(
            STATE_MD_MAX_BYTES >= 70_000u32,
            "T-005 (AC-003): STATE_MD_MAX_BYTES ({}) must be >= 70000. \
             Raise to 262144 per BC-5.40.001 v1.2 Precondition 6.",
            STATE_MD_MAX_BYTES
        );

        // On-disk bytes: STATE.md content with NO closing `---` delimiter.
        // extract_frontmatter returns the full bytes (fail-open per function contract).
        // Downstream: extract_top_level_field on the on-disk string → Malformed
        // (no closing delimiter) → guard returns Continue (fail-open).
        let on_disk_no_delimiter: Vec<u8> =
            b"---\ndocument_type: state\ntimestamp: \"2026-06-11T10:00:00Z\"\nphase: test\n"
                .to_vec();
        // Intentionally omitting the closing `---` delimiter.

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_raw_bytes(on_disk_no_delimiter, warn_log.clone());
        // Proposed: advanced timestamp (different from on-disk value).
        let proposed_advanced = state_md_no_lock(TS_NEW);
        let payload = payload_write(&proposed_advanced);

        let result = guard_logic(payload, callbacks);

        // Guard must return Continue without panicking:
        // on-disk has no closing delimiter → extract_frontmatter returns full bytes
        // → extract_top_level_field returns Malformed → fail-open Continue.
        assert_eq!(
            result,
            HookResult::Continue,
            "T-005 (AC-003): On-disk with no closing delimiter must return Continue \
             (fail-open per BC-5.40.001 Invariant 7 / ADR-025 §12.3 row 1). \
             Guard must not panic or hard-error on malformed on-disk content."
        );
    }

    // -----------------------------------------------------------------------
    // T-007 (AC-005): state_md_approaching_cap soft-warn boundary tests A–E
    //
    // BC Trace: BC-5.40.001 v1.2 Invariant 8.
    //   Soft-warn range: bytes_read ∈ (200000, 262144]; inclusive at cap.
    //   Event fields: bytes_read: u64, cap_bytes: u64 (262144).
    //   Threshold is STRICT (> 200000, not >=).
    //
    // Sub-tests:
    //   A: 210000 bytes → state_md_approaching_cap warn emitted.
    //   B: 150000 bytes → zero state_md_approaching_cap warns.
    //   C: 200000 bytes exactly → zero warns (strict > threshold).
    //   D: 262144 bytes exactly → warn emitted AND read succeeds (cap-exact inclusive).
    //   E: 262145 bytes (cap-enforcement) → Continue (fail-open) AND zero warn.
    //
    // Mock strategy per sub-test (TD-VSDD-060 same-form sweep / F-P7-001 fix):
    //   A/B/C: cap-ignoring mock (make_callbacks_with_raw_bytes) — cap is NOT load-bearing.
    //          Fixture sizes 210000 / 150000 / 200000 are all strictly less than 262144.
    //          The Err branch (fixture_len > max_bytes) is unreachable regardless of
    //          comparator, so cap-enforcement adds no falsifiability here. The assertions
    //          test warn-threshold semantics (>200000 strict / <=200000 silent), which are
    //          fully exercised by the cap-ignoring mock. ADJUDICATED: cap-ignoring mock is
    //          correct and sufficient for A/B/C.
    //   D:     cap-enforcement mock (GuardCallbacks inline, production comparator len as u32 >
    //          max_bytes). At 262144-exact with max_bytes=262144: 262144 > 262144 = false →
    //          Ok (read succeeds, warn fires, guard runs to expected verdict). Falsifiable: a
    //          >= comparator regression → 262144 >= 262144 = true → Err → fail-open read-error
    //          warn fires → the !has_read_error assertion fires. Previously, make_callbacks_with_raw_bytes
    //          ignored max_bytes entirely, making that assertion structurally incapable of failing
    //          under any production regression (F-P7-001).
    //   E:     cap-enforcement mock — same pattern as D; fixture_len=262145 > max_bytes=262144 →
    //          Err → Continue (fail-open). E was already correct before this fix.
    //
    // RED GATE: assert_eq!(STATE_MD_MAX_BYTES, 262144u32) fails with current 65536.
    // After Task 9 (cap raise), sub-tests A and D additionally fail until Task 11
    // (soft-warn emission implemented). Sub-tests B/C pass tautologically after
    // Task 9 (no warn is expected and no warn is emitted before Task 11).
    // -----------------------------------------------------------------------

    /// T-007 (AC-005): `state_md_approaching_cap` soft-warn boundary tests A–E.
    ///
    /// BC-5.40.001 v1.2 Invariant 8: guard MUST emit `state_md_approaching_cap`
    /// warn carrying `bytes_read: u64` and `cap_bytes: u64 = 262144` when
    /// `bytes_read > 200000 AND bytes_read <= 262144`. This event is
    /// observability-only — it never alters the Continue/Block verdict.
    ///
    /// RED GATE: `assert_eq!(STATE_MD_MAX_BYTES, 262144u32)` fails until Task 9.
    /// Sub-tests A and D additionally fail until Task 11 (soft-warn emission).
    #[test]
    fn test_BC_5_40_001_T007_state_md_approaching_cap_warn_boundary() {
        // Pre-condition: cap constant must be 262144 for all sub-tests.
        // Primary Red Gate — fails until Task 9 (cap raise to 262144).
        assert_eq!(
            STATE_MD_MAX_BYTES, 262144u32,
            "T-007 (AC-005): STATE_MD_MAX_BYTES must equal 262144. \
             Current value: {}. Raise to 262144 (Task 9) before soft-warn tests run.",
            STATE_MD_MAX_BYTES
        );

        // ---- Sub-test A: 210000 bytes → state_md_approaching_cap warn emitted ----
        // bytes_read = 210000 > 200000 threshold, ≤ 262144 cap → warn MUST fire.
        // RED (after Task 9): warn not yet emitted → assertion fails until Task 11.
        {
            let fixture = state_md_padded_with_timestamp_bytes(TS_OLD, 210_000);
            assert_eq!(
                fixture.len(),
                210_000,
                "T-007 A: fixture must be exactly 210000 bytes"
            );
            let warn_log = Arc::new(Mutex::new(Vec::new()));
            let callbacks = make_callbacks_with_raw_bytes(fixture, warn_log.clone());
            let proposed_advanced = state_md_no_lock(TS_NEW);
            let payload = payload_write(&proposed_advanced);
            let _ = guard_logic(payload, callbacks);
            let warns = warn_log.lock().unwrap();
            assert!(
                warns.iter().any(|w| {
                    w.contains("state_md_approaching_cap")
                        && w.contains("bytes_read=210000")
                        && w.contains("cap_bytes=262144")
                }),
                "T-007 A (AC-005): 210000-byte fixture must emit state_md_approaching_cap warn \
                 with bytes_read=210000 and cap_bytes=262144 \
                 (bytes_read=210000 > 200000 threshold). Got: {:?}",
                warns
            );
        }

        // ---- Sub-test B: 150000 bytes → zero state_md_approaching_cap warns ----
        // bytes_read = 150000 ≤ 200000 threshold → warn must NOT fire.
        {
            let fixture = state_md_padded_with_timestamp_bytes(TS_OLD, 150_000);
            assert_eq!(
                fixture.len(),
                150_000,
                "T-007 B: fixture must be exactly 150000 bytes"
            );
            let warn_log = Arc::new(Mutex::new(Vec::new()));
            let callbacks = make_callbacks_with_raw_bytes(fixture, warn_log.clone());
            let proposed_advanced = state_md_no_lock(TS_NEW);
            let payload = payload_write(&proposed_advanced);
            let _ = guard_logic(payload, callbacks);
            let warns = warn_log.lock().unwrap();
            let approaching: Vec<_> = warns
                .iter()
                .filter(|w| w.contains("state_md_approaching_cap"))
                .collect();
            assert!(
                approaching.is_empty(),
                "T-007 B (AC-005): 150000-byte fixture must NOT emit state_md_approaching_cap \
                 (bytes_read=150000 ≤ 200000 threshold). Got: {:?}",
                approaching
            );
        }

        // ---- Sub-test C: 200000 bytes exactly → zero warns (strict > threshold) ----
        // bytes_read = 200000 is NOT strictly > 200000 → warn must NOT fire.
        {
            let fixture = state_md_padded_with_timestamp_bytes(TS_OLD, 200_000);
            assert_eq!(
                fixture.len(),
                200_000,
                "T-007 C: fixture must be exactly 200000 bytes"
            );
            let warn_log = Arc::new(Mutex::new(Vec::new()));
            let callbacks = make_callbacks_with_raw_bytes(fixture, warn_log.clone());
            let proposed_advanced = state_md_no_lock(TS_NEW);
            let payload = payload_write(&proposed_advanced);
            let _ = guard_logic(payload, callbacks);
            let warns = warn_log.lock().unwrap();
            let approaching: Vec<_> = warns
                .iter()
                .filter(|w| w.contains("state_md_approaching_cap"))
                .collect();
            assert!(
                approaching.is_empty(),
                "T-007 C (AC-005): 200000-byte fixture (exact threshold) must NOT emit \
                 state_md_approaching_cap (threshold is strictly > 200000, not >=). \
                 Got: {:?}",
                approaching
            );
        }

        // ---- Sub-test D: 262144 bytes exactly → warn AND read succeeds ----
        // bytes_read = 262144 = cap → inclusive upper bound: warn MUST fire AND read succeeds.
        // RED (after Task 9): warn not yet emitted → fails until Task 11.
        //
        // Cap-enforcement mock (F-P7-001 fix): read_file compares fixture.len() as u32 > max_bytes
        // using the byte-identical production comparator. At cap-exact (262144 == 262144) the
        // read succeeds → warn fires → guard completes with expected verdict. A >= regression
        // in the mock (mirroring a production regression) yields Err → fail-open warn fires →
        // the !has_read_error assertion fires, making the boundary provably falsifiable.
        {
            let fixture = state_md_padded_with_timestamp_bytes(TS_OLD, 262_144);
            assert_eq!(
                fixture.len(),
                262_144,
                "T-007 D: fixture must be exactly 262144 bytes"
            );
            let warn_log = Arc::new(Mutex::new(Vec::new()));
            let wl = warn_log.clone();
            // Cap-enforcement mock: mirrors real host::read_file OutputTooLarge behaviour.
            // Uses production comparator (>) — 262144 > 262144 = false → Ok at cap-exact.
            let callbacks = GuardCallbacks {
                read_file: move |_path, max_bytes, _timeout| {
                    if fixture.len() as u32 > max_bytes {
                        Err(format!(
                            "OutputTooLarge: fixture_len={} > max_bytes={}",
                            fixture.len(),
                            max_bytes
                        ))
                    } else {
                        Ok(fixture)
                    }
                },
                log_warn: move |msg: &str| {
                    wl.lock().unwrap().push(msg.to_string());
                },
                write_stderr: |_msg| {},
            };
            let proposed_advanced = state_md_no_lock(TS_NEW);
            let payload = payload_write(&proposed_advanced);
            let result = guard_logic(payload, callbacks);
            let warns = warn_log.lock().unwrap();

            // Read must succeed at cap — no fail-open read-error warn.
            let has_read_error = warns.iter().any(|w| w.contains("fail-open read-error"));
            assert!(
                !has_read_error,
                "T-007 D (AC-005): 262144-byte fixture must NOT produce fail-open read-error \
                 warn (read must succeed at cap-exact boundary). Got: {:?}",
                warns
            );
            // Guard must return Continue (advanced timestamp; no lock held).
            assert!(
                matches!(result, HookResult::Continue),
                "T-007 D (AC-005): 262144-byte fixture with advanced timestamp must return \
                 Continue (read succeeded at cap; no lock held). Got: {:?}",
                result
            );
            // Warn must be emitted at the inclusive cap boundary.
            assert!(
                warns.iter().any(|w| {
                    w.contains("state_md_approaching_cap") && w.contains("cap_bytes=262144")
                }),
                "T-007 D (AC-005): 262144-byte fixture must emit state_md_approaching_cap warn \
                 (inclusive upper bound: bytes_read=262144 ≤ cap=262144). Got: {:?}",
                warns
            );
        }

        // ---- Sub-test E: 262145 bytes → fail-open Continue AND zero approaching_cap ----
        // Uses cap-enforcement mock: fixture_size > max_bytes → Err(OutputTooLarge).
        // Guard must return Continue (fail-open per ADR-025 Decision 7) AND must NOT
        // emit state_md_approaching_cap (warn path not reached after Err).
        {
            let fixture_len = 262_145usize;
            let warn_log = Arc::new(Mutex::new(Vec::new()));
            let wl = warn_log.clone();
            // Cap-enforcement mock: mirrors real host::read_file OutputTooLarge behaviour.
            let callbacks = GuardCallbacks {
                read_file: move |_path, max_bytes, _timeout| {
                    if fixture_len as u32 > max_bytes {
                        Err(format!(
                            "OutputTooLarge: fixture_len={} > max_bytes={}",
                            fixture_len, max_bytes
                        ))
                    } else {
                        Ok(state_md_padded_with_timestamp_bytes(TS_OLD, fixture_len))
                    }
                },
                log_warn: move |msg: &str| {
                    wl.lock().unwrap().push(msg.to_string());
                },
                write_stderr: |_msg| {},
            };
            let proposed_advanced = state_md_no_lock(TS_NEW);
            let payload = payload_write(&proposed_advanced);
            let result = guard_logic(payload, callbacks);

            // Must return Continue (fail-open on OutputTooLarge per ADR-025 Decision 7 / EC-010).
            assert_eq!(
                result,
                HookResult::Continue,
                "T-007 E (AC-005): 262145-byte fixture must return Continue \
                 (fail-open on OutputTooLarge per ADR-025 Decision 7 / BC-5.40.001 EC-010)."
            );
            let warns = warn_log.lock().unwrap();
            // Must NOT emit state_md_approaching_cap (OutputTooLarge → read failed before
            // soft-warn check; warn path not reached).
            let approaching: Vec<_> = warns
                .iter()
                .filter(|w| w.contains("state_md_approaching_cap"))
                .collect();
            assert!(
                approaching.is_empty(),
                "T-007 E (AC-005): 262145-byte fixture must NOT emit state_md_approaching_cap \
                 (exceeds cap → read fails → warn path never reached). Got: {:?}",
                approaching
            );
        }
    }

    // -----------------------------------------------------------------------
    // F-P2-002 body-target Edit/MultiEdit reconstruction tests
    //
    // Finding F-P2-001 (HIGH): guard_logic passes frontmatter-only on_disk_content
    // to extract_edit_proposed/extract_multiedit_proposed after extract_frontmatter
    // truncation. An Edit/MultiEdit whose old_string targets BODY content (after the
    // closing ---) is not found in the truncated base → ProposedContent::FailOpen →
    // Continue, even with an unchanged/stale timestamp. Pre-fix behaviour (full-content
    // base) was Block(TimestampStale).
    //
    // Finding F-P2-002: existing Edit/MultiEdit reconstruction tests all use frontmatter
    // fields as old_string, so the suite never exercises the body-target path.
    //
    // ADR-032 Decision 1 supersession (AC-020): body-only Edits whose new_string sets
    // NEITHER timestamp: NOR factory_lock: at column 0 are now payload-neutral and
    // return Continue BEFORE reconstruction. Tests 1, 2, and the large variant have
    // been updated to assert Continue per ADR-032. The F-P2-001 fix (full-content base)
    // remains in place and is exercised by test 3 (boundary-spanning new_string that
    // DOES set timestamp:, routing through the full reconstruction path).
    //
    // GREEN tests (payload-neutral under ADR-032 → Continue):
    //   test_edit_body_target_delimiter_present_payload_neutral_continues     (test 1)
    //   test_multiedit_body_target_payload_neutral_continues                  (test 2)
    //   test_edit_body_target_70kib_delimiter_present_payload_neutral_continues (large variant)
    //
    // GREEN tests (timestamp-advancing → reconstruction path → Continue):
    //   test_edit_boundary_spanning_advanced_timestamp_continues           (test 3)
    //   test_multiedit_mixed_frontmatter_body_edits_advanced_timestamp_continues (test 4)
    // -----------------------------------------------------------------------

    // -----------------------------------------------------------------------
    // F-P2-002 test 1 / ADR-032 Decision 1 — Edit payload, delimiter-present fixture,
    //   old_string = body content, new_string = body-only replacement → payload-neutral
    //   → Continue (AC-020)
    //
    // Traces: F-P2-001/F-P2-002 / AC-012 / BC-5.40.001 PC4 / ADR-032 Decision 1 (AC-020)
    //
    // Scenario:
    //   - On-disk: state_md_no_lock(TS_OLD) — has closing --- delimiter and
    //     "# STATE" body heading after it.
    //   - Edit payload: old_string = "# STATE" (body content after closing ---),
    //     new_string = body-only replacement. Neither timestamp: nor factory_lock:
    //     appears at column 0 in new_string.
    //
    // ADR-032 Decision 1 (AC-020): guard scans new_string for top-level timestamp:
    //   and factory_lock: fields. new_string = "# STATE\n\nBody text added by edit."
    //   sets neither → payload-neutral → guard returns Continue immediately,
    //   before the reconstruction path is reached.
    //
    // GREEN: guard returns Continue — payload-neutral Edit under ADR-032 Decision 1.
    // -----------------------------------------------------------------------

    #[test]
    fn test_edit_body_target_delimiter_present_payload_neutral_continues() {
        let on_disk = state_md_no_lock(TS_OLD);
        let body_target = "# STATE";

        // Confirm body content is in the full on-disk string.
        assert!(
            on_disk.contains(body_target),
            "test fixture (full content) must contain body target {body_target:?}"
        );

        // Edit targets body content only; new_string sets neither timestamp: nor factory_lock:.
        // ADR-032 Decision 1: payload-neutral → Continue.
        let old_str = body_target;
        let new_str = "# STATE\n\nBody text added by edit.";

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_edit(old_str, new_str);

        let result = guard_logic(payload, callbacks);

        // ADR-032 Decision 1: new_string = "# STATE\n\n..." sets neither timestamp: nor
        // factory_lock: at col 0 → payload-neutral → Continue (AC-020).
        assert_eq!(
            result,
            HookResult::Continue,
            "test_edit_body_target_delimiter_present_payload_neutral_continues: \
             body-only Edit with new_string '# STATE\\n\\nBody text added by edit.' \
             (no timestamp: or factory_lock: at col 0) is payload-neutral under \
             ADR-032 Decision 1 (AC-020) → must return Continue."
        );
    }

    // -----------------------------------------------------------------------
    // F-P2-002 test 2 / ADR-032 Decision 1 — MultiEdit payload, first edit targets
    //   body content, new_string = body-only replacement → payload-neutral → Continue
    //
    // Traces: F-P2-001/F-P2-002 / AC-013 / BC-5.40.001 PC4 / ADR-032 Decision 1 (AC-020)
    //
    // ADR-032 Decision 1 (AC-020): guard scans all new_string values in edits[] for
    // top-level timestamp: / factory_lock: fields. The single edit's new_string
    // ("# STATE\n\nBody text added by multiedit.") sets neither → payload-neutral →
    // Continue immediately.
    //
    // GREEN: guard returns Continue — payload-neutral MultiEdit under ADR-032 Decision 1.
    // -----------------------------------------------------------------------

    #[test]
    fn test_multiedit_body_target_payload_neutral_continues() {
        let on_disk = state_md_no_lock(TS_OLD);
        let body_target = "# STATE";

        assert!(
            on_disk.contains(body_target),
            "test fixture (full content) must contain body target {body_target:?}"
        );

        // Single edit targeting body content; new_string sets neither timestamp: nor factory_lock:.
        // ADR-032 Decision 1: payload-neutral → Continue.
        let edit1_old = body_target;
        let edit1_new = "# STATE\n\nBody text added by multiedit.";

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_multiedit(vec![(edit1_old, edit1_new)]);

        let result = guard_logic(payload, callbacks);

        // ADR-032 Decision 1: body-only new_string sets neither timestamp: nor factory_lock:
        // at col 0 → payload-neutral → Continue (AC-020).
        assert_eq!(
            result,
            HookResult::Continue,
            "test_multiedit_body_target_payload_neutral_continues: \
             body-only MultiEdit with new_string '# STATE\\n\\nBody text...' \
             (no timestamp: or factory_lock: at col 0) is payload-neutral under \
             ADR-032 Decision 1 (AC-020) → must return Continue."
        );
    }

    // -----------------------------------------------------------------------
    // F-P2-002 test 3 — Edit payload, old_string spans frontmatter/body boundary,
    //   new_string advances timestamp → MUST Continue
    //
    // Traces: F-P2-001/F-P2-002 / AC-012 / BC-5.40.001 PC4 success path / PC6
    //
    // Note on pure body-only path: under the F-P2-001 fix alone, a body-only Edit
    // with an unchanged timestamp would Block (body old_string found in full content
    // → proposed has TS_OLD → Step 6 → Block(TimestampStale)). ADR-032 Decision 1
    // supersedes this: body-only Edits whose new_string sets neither timestamp: nor
    // factory_lock: at column 0 are payload-neutral and return Continue before
    // reconstruction (AC-020). The full-content reconstruction base (F-P2-001 fix)
    // remains necessary only for timestamp-advancing Edits (sets_timestamp = true)
    // that also span body content — exercised by the boundary-spanning test below.
    //
    // Nearest meaningful positive path (covers body-region reconstruction):
    //   old_string spans the closing --- delimiter into the body heading, and
    //   new_string advances the timestamp while preserving the body heading.
    //   After the fix, full on_disk_content is used → boundary-spanning old_string
    //   found → proposed has TS_NEW → Continue.
    //
    // Under current code (before fix): frontmatter-only on_disk_content used →
    //   boundary-spanning old_string NOT found (body portion absent) → FailOpen →
    //   Continue (same outcome, wrong path). Test is GREEN under both; no regression.
    // -----------------------------------------------------------------------

    #[test]
    fn test_edit_boundary_spanning_advanced_timestamp_continues() {
        let on_disk = state_md_no_lock(TS_OLD);

        // Build old_string spanning the frontmatter/body boundary:
        //   "timestamp: \"TS_OLD\"\nphase: test\n---\n\n# STATE"
        // This substring exists in the FULL on_disk content (verified below) but
        // is absent from the truncated frontmatter-only content (which ends at
        // "phase: test\n---\n" without the "\n# STATE" body portion).
        let old_str_owned = format!("timestamp: \"{}\"\nphase: test\n---\n\n# STATE", TS_OLD);
        let old_str = old_str_owned.as_str();

        // new_string: same region but with TS_NEW — advances timestamp, body intact.
        let new_str_owned = format!("timestamp: \"{}\"\nphase: test\n---\n\n# STATE", TS_NEW);
        let new_str = new_str_owned.as_str();

        // Verify old_string IS present in the full on-disk content (post-fix base).
        assert!(
            on_disk.contains(old_str),
            "test fixture (full content) must contain boundary-spanning old_string: \
             {old_str:?}"
        );

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_edit(old_str, new_str);

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "test_edit_boundary_spanning_advanced_timestamp_continues: \
             Edit with old_string spanning the frontmatter/body boundary and new_string \
             advancing the timestamp must return Continue (F-P2-002 positive path). \
             After fix, full-content reconstruction finds the boundary-spanning \
             old_string and applies the advancement (TS_OLD → TS_NEW) → Continue. \
             Before fix, returns Continue via FailOpen (boundary portion absent from \
             truncated content) — same outcome, wrong path; no regression either way."
        );
    }

    // -----------------------------------------------------------------------
    // F-P2-002 test 4 — MultiEdit: one frontmatter edit advancing timestamp +
    //   one body edit → MUST Continue (regression guard)
    //
    // Traces: F-P2-001/F-P2-002 / AC-013 / BC-5.40.001 PC4 success path / PC6
    //
    // Regression guard: after the fix, agents can combine a timestamp advance with
    // a body update in a single MultiEdit. The fix must NOT over-block this sequence.
    //
    // Under fixed code: full on_disk_content used for sequential reconstruction.
    //   Edit 1: old_string "timestamp: \"TS_OLD\"" in frontmatter → TS_NEW applied.
    //   Edit 2: old_string "# STATE" in body of intermediate content → body updated.
    //   Final proposed content has TS_NEW → different from on-disk TS_OLD → Continue.
    //
    // Under current code (before fix): frontmatter-only on_disk_content used.
    //   Edit 1: found in truncated content → TS_NEW applied to intermediate.
    //   Edit 2: "# STATE" NOT in intermediate (body stripped) → FailOpen → Continue.
    //   (Same Continue outcome, wrong path.) Test GREEN under both; ensures no
    //   regression to Block on valid mixed-edit sequences after the fix.
    // -----------------------------------------------------------------------

    #[test]
    fn test_multiedit_mixed_frontmatter_body_edits_advanced_timestamp_continues() {
        let on_disk = state_md_no_lock(TS_OLD);

        let ts_old_line = format!("timestamp: \"{}\"", TS_OLD);
        let ts_new_line = format!("timestamp: \"{}\"", TS_NEW);
        let body_old = "# STATE";
        let body_new = "# STATE\n\nBody text added by mixed multiedit.";

        assert!(
            on_disk.contains(ts_old_line.as_str()),
            "fixture must contain old timestamp line"
        );
        assert!(
            on_disk.contains(body_old),
            "fixture must contain body target"
        );

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_multiedit(vec![
            (ts_old_line.as_str(), ts_new_line.as_str()),
            (body_old, body_new),
        ]);

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "test_multiedit_mixed_frontmatter_body_edits_advanced_timestamp_continues: \
             MultiEdit with one frontmatter edit advancing timestamp and one body edit \
             must return Continue (F-P2-002 regression guard — fix must NOT over-block \
             valid mixed edit sequences). Fixed code: full reconstruction, TS_NEW in \
             proposed → Continue. Current code: FailOpen on body edit → Continue \
             (same outcome, wrong path)."
        );
    }

    // -----------------------------------------------------------------------
    // F-P2-002 large-fixture variant — Edit, >64 KiB delimiter-present fixture,
    //   old_string = body content, timestamp NOT advanced → MUST Block
    //
    // Traces: F-P2-001/F-P2-002 / AC-012 / BC-5.40.001 v1.2 Precondition 6 / PC4
    //
    // Same scenario as test 1 but with a 70000-byte (>64 KiB) fixture. Locks the
    // F-P2-001/F-P2-002 fix against regression across the raised-cap range used by
    // T-002/T-003 (BC-5.40.001 v1.2 Precondition 6, cap = 262144).
    //
    // RED GATE 1 (fails until Task 9): STATE_MD_MAX_BYTES must be >= 70000.
    // RED GATE 2 (fails after Task 9 until F-P2-001 fix): body-targeting Edit still
    //   returns Continue (truncation bug). Fails until the S-19.08 wiring fix.
    // -----------------------------------------------------------------------

    #[test]
    fn test_edit_body_target_70kib_delimiter_present_payload_neutral_continues() {
        // Red Gate 1: cap constant must be >= 70000.
        // Fails until Task 9 raises STATE_MD_MAX_BYTES to 262144.
        assert!(
            STATE_MD_MAX_BYTES >= 70_000u32,
            "test_edit_body_target_70kib_delimiter_present_payload_neutral_continues: \
             STATE_MD_MAX_BYTES ({}) must be >= 70000. \
             Raise to 262144 per BC-5.40.001 v1.2 Precondition 6 (Task 9).",
            STATE_MD_MAX_BYTES
        );

        // 70 KiB fixture with TS_OLD in frontmatter; body includes "# STATE" heading.
        // state_md_padded_with_timestamp_bytes builds:
        //   "---\n...\ntimestamp: \"TS_OLD\"\n...\n---\n\n# STATE\n" + padding lines.
        let fixture_bytes = state_md_padded_with_timestamp_bytes(TS_OLD, 70_000);
        assert_eq!(
            fixture_bytes.len(),
            70_000,
            "fixture must be exactly 70000 bytes"
        );

        let fixture_str = std::str::from_utf8(&fixture_bytes).expect("fixture is valid UTF-8");
        let body_target = "# STATE";
        assert!(
            fixture_str.contains(body_target),
            "70 KiB fixture must contain body content {body_target:?} (after closing ---)"
        );

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_raw_bytes(fixture_bytes, warn_log.clone());

        // Edit targets body content; timestamp NOT advanced.
        let old_str = body_target; // "# STATE" is after the closing ---
        let new_str = "# STATE\n\nBody text added by edit.";
        let payload = payload_edit(old_str, new_str);

        let result = guard_logic(payload, callbacks);

        // ADR-032 Decision 1: new_string = "# STATE\n\nBody text added by edit." sets neither
        // timestamp: nor factory_lock: at col 0 → payload-neutral → Continue (AC-020).
        // The STATE_MD_MAX_BYTES cap assertion above still validates the cap constant.
        assert_eq!(
            result,
            HookResult::Continue,
            "test_edit_body_target_70kib_delimiter_present_payload_neutral_continues: \
             body-only Edit with new_string '# STATE\\n\\nBody text added by edit.' \
             (no timestamp: or factory_lock: at col 0) is payload-neutral under \
             ADR-032 Decision 1 (AC-020) → must return Continue (large-fixture variant)."
        );
    }

    // -----------------------------------------------------------------------
    // F-P3-001 / ADR-032 Decision 1 — Edit payload, delimiter-present fixture with
    //   non-UTF-8 body bytes, old_string targets frontmatter field (phase:),
    //   new_string = "phase: complete" → payload-neutral → Continue
    //
    // Traces: F-P3-001 / AC-012 / BC-5.40.001 PC4 / ADR-025 D12 §12.2 / ADR-032 Decision 1 (AC-020)
    //
    // Context: the F-P2-001 fix introduced a fallback in on_disk_reconstruction_base.
    // When String::from_utf8(full on_disk_bytes) fails because body bytes are non-UTF-8
    // (\xFF\xFE after closing ---), the reconstruction base falls back to
    // on_disk_field_content.clone() (frontmatter only + synthetic \n---\n per Invariant 7).
    //
    // Pre-ADR-032 behavior: "phase: test" IS present in the frontmatter-only fallback
    // base → extract_edit_proposed reconstructs proposed content → proposed has TS_OLD
    // (unchanged) → Block(TimestampStale).
    //
    // ADR-032 Decision 1 (AC-020) supersedes: guard scans new_string ("phase: complete")
    // for top-level timestamp: / factory_lock: fields. Neither is set → payload-neutral →
    // Continue immediately, before the non-UTF-8 fallback path is reached.
    //
    // Note on regression lock coverage: the non-UTF-8 fallback arm correctness is still
    // verified by test_edit_non_utf8_body_fallback_advanced_timestamp_continues, whose
    // new_string IS "timestamp: TS_NEW" (sets timestamp: → sets_timestamp = true →
    // full reconstruction path exercised, including the fallback arm).
    // -----------------------------------------------------------------------

    #[test]
    fn test_edit_non_utf8_body_fallback_payload_neutral_continues() {
        // Base fixture: valid UTF-8 frontmatter with TS_OLD, closing --- delimiter.
        // state_md_no_lock(TS_OLD) produces:
        //   "---\n...\ntimestamp: \"TS_OLD\"\nphase: test\n---\n\n# STATE\n"
        // The frontmatter region contains "phase: test" (the Edit target).
        let valid_portion = state_md_no_lock(TS_OLD);

        // Confirm the fixture has the closing --- delimiter (ensures delimiter_found=true).
        assert!(
            valid_portion.contains("\n---\n"),
            "F-P3-001 fixture must contain closing '\\n---\\n' delimiter (delimiter_found=true path)"
        );
        let old_str = "phase: test";
        // Confirm old_string is in the frontmatter region (not just the body).
        assert!(
            valid_portion.contains(old_str),
            "F-P3-001 fixture must contain old_string '{old_str}' in the frontmatter"
        );

        // Append non-UTF-8 body bytes after the valid content.
        // \xFF\xFE are invalid UTF-8 start bytes — String::from_utf8(full bytes) fails.
        // This triggers the fallback: on_disk_reconstruction_base = on_disk_field_content.clone()
        // (frontmatter only). The frontmatter is valid UTF-8 so on_disk_field_content succeeds.
        let mut on_disk_bytes = valid_portion.into_bytes();
        on_disk_bytes.extend_from_slice(b"\xFF\xFE non-utf8-body-bytes-after-delimiter\n");

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_raw_bytes(on_disk_bytes, warn_log.clone());

        // Edit payload: old_string targets a frontmatter field; timestamp NOT advanced.
        // After fallback, on_disk_reconstruction_base = on_disk_field_content (frontmatter+---).
        // "phase: test" IS found in the frontmatter-only base → reconstruction succeeds.
        // Proposed content has TS_OLD → same as on_disk_ts → Block(TimestampStale).
        let new_str = "phase: complete";
        let payload = payload_edit(old_str, new_str);

        let result = guard_logic(payload, callbacks);

        // ADR-032 Decision 1: new_string = "phase: complete" sets neither timestamp: nor
        // factory_lock: at col 0 → payload-neutral → Continue (AC-020).
        // Guard returns before the non-UTF-8 fallback reconstruction path is reached.
        assert_eq!(
            result,
            HookResult::Continue,
            "test_edit_non_utf8_body_fallback_payload_neutral_continues: \
             Edit with new_string 'phase: complete' (no timestamp: or factory_lock: at col 0) \
             is payload-neutral under ADR-032 Decision 1 (AC-020) → must return Continue. \
             Pre-ADR-032 Block(TimestampStale) behavior is superseded. \
             Non-UTF-8 fallback arm coverage remains via \
             test_edit_non_utf8_body_fallback_advanced_timestamp_continues."
        );
    }

    // -----------------------------------------------------------------------
    // F-P3-001 companion — same non-UTF-8-body fixture, Edit advances timestamp
    //   → MUST Continue (fallback base still enables valid timestamp advancement)
    //
    // Traces: F-P3-001 / AC-012 / AC-003 / BC-5.40.001 PC4 success path / PC6
    //
    // Proves the fallback reconstruction (on_disk_field_content.clone()) also
    // permits valid advanced-timestamp edits: old_string targets the timestamp
    // line in the frontmatter, which IS present in the frontmatter-only fallback
    // base. After reconstruction the proposed content has TS_NEW → Continue.
    // -----------------------------------------------------------------------

    #[test]
    fn test_edit_non_utf8_body_fallback_advanced_timestamp_continues() {
        // Same fixture construction as the primary F-P3-001 test.
        let mut on_disk_bytes = state_md_no_lock(TS_OLD).into_bytes();
        on_disk_bytes.extend_from_slice(b"\xFF\xFE non-utf8-body-bytes-after-delimiter\n");

        // Edit advances the timestamp: old_string = timestamp line in frontmatter.
        // The frontmatter-only fallback base contains this line → reconstruction succeeds.
        let old_str = format!("timestamp: \"{}\"", TS_OLD);
        let new_str = format!("timestamp: \"{}\"", TS_NEW);

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_raw_bytes(on_disk_bytes, warn_log.clone());
        let payload = payload_edit(&old_str, &new_str);

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "test_edit_non_utf8_body_fallback_advanced_timestamp_continues: \
             Edit that advances timestamp on a delimiter-present non-UTF-8-body fixture \
             must return Continue (F-P3-001 companion). Fallback base (frontmatter only) \
             contains the old timestamp line → reconstruction succeeds → proposed has \
             TS_NEW → different from on-disk TS_OLD → Continue \
             (BC-5.40.001 PC4 success path / PC6)."
        );
    }

    // -----------------------------------------------------------------------
    // ADR-032 Decision 1+3+4: payload-targeted enforcement (AC-020)
    //
    // Red Gate tests (4): must FAIL against unmodified guard_logic, pass only
    // after the ADR-032 payload-scan fix is applied.
    //   - ac020_edit_body_only_no_timestamp_continues
    //   - ac020_multiedit_no_timestamp_in_any_new_string_continues
    //   - ac020_edit_body_lock_held_no_factory_lock_continues
    //   - ac020_edit_factory_lock_only_stale_expires_blocks
    //
    // Regression guards (7): must pass both pre- and post-fix.
    // -----------------------------------------------------------------------

    // -----------------------------------------------------------------------
    // Red Gate 1 of 4: ac020_edit_body_only_no_timestamp_continues
    // Edit where new_string is body text (no timestamp: line);
    // on-disk has OLD timestamp. Reconstructed proposed has same OLD timestamp.
    // Pre-fix: Block(TimestampStale) — Step 6 fires on byte-identical timestamps.
    // Post-fix: payload-neutral → Continue.
    // -----------------------------------------------------------------------
    #[test]
    fn ac020_edit_body_only_no_timestamp_continues() {
        let on_disk = state_md_no_lock(TS_OLD);
        // Body-only Edit: new_string has no timestamp: field at column 0.
        let old_string = "# STATE\n";
        let new_string = "# SESSION CHECKPOINT\nbody text\n";

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_edit(old_string, new_string);

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "ac020_edit_body_only_no_timestamp_continues: Edit with body-only new_string \
             (no timestamp: at col-0) must return Continue (payload-neutral, ADR-032 Decision 1). \
             Pre-fix: Block(TimestampStale) — RED GATE. \
             Post-fix: payload-neutral → Continue."
        );
    }

    // -----------------------------------------------------------------------
    // Regression guard: ac020_edit_explicit_stale_timestamp_blocks
    // Edit where new_string contains timestamp: "OLD" explicitly.
    // Both pre- and post-fix: Block(TimestampStale).
    // -----------------------------------------------------------------------
    #[test]
    fn ac020_edit_explicit_stale_timestamp_blocks() {
        let on_disk = state_md_no_lock(TS_OLD);
        // Identity replacement: old_string == new_string, both contain timestamp: TS_OLD.
        // Proposed content == on-disk content → proposed_ts == on_disk_ts → Block.
        let ts_line = format!("timestamp: \"{TS_OLD}\"");

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_edit(&ts_line, &ts_line);

        let result = guard_logic(payload, callbacks);

        let expected_msg = canonical_timestamp_stale_message();
        assert_eq!(
            result,
            HookResult::Block {
                reason: expected_msg.clone()
            },
            "ac020_edit_explicit_stale_timestamp_blocks: Edit where new_string explicitly sets \
             timestamp: OLD must Block(TimestampStale) both pre- and post-fix (regression guard). \
             Got: {result:?}"
        );
    }

    // -----------------------------------------------------------------------
    // Regression guard: ac020_edit_explicit_advanced_timestamp_continues
    // Edit where new_string contains timestamp: "NEW" (advancing).
    // Both pre- and post-fix: Continue.
    // -----------------------------------------------------------------------
    #[test]
    fn ac020_edit_explicit_advanced_timestamp_continues() {
        let on_disk = state_md_no_lock(TS_OLD);
        let old_ts_line = format!("timestamp: \"{TS_OLD}\"");
        let new_ts_line = format!("timestamp: \"{TS_NEW}\"");

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_edit(&old_ts_line, &new_ts_line);

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "ac020_edit_explicit_advanced_timestamp_continues: Edit that advances timestamp must \
             Continue both pre- and post-fix (regression guard). Got: {result:?}"
        );
    }

    // -----------------------------------------------------------------------
    // Red Gate 2 of 4: ac020_multiedit_no_timestamp_in_any_new_string_continues
    // MultiEdit where no edits[i].new_string contains timestamp:; on-disk OLD.
    // Pre-fix: Block(TimestampStale).
    // Post-fix: payload-neutral → Continue.
    // -----------------------------------------------------------------------
    #[test]
    fn ac020_multiedit_no_timestamp_in_any_new_string_continues() {
        let on_disk = state_md_no_lock(TS_OLD);
        // Single-edit MultiEdit: new_string is body text with no timestamp:.
        let edits = vec![("# STATE\n", "## SESSION HEADER\nbody content\n")];

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_multiedit(edits);

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "ac020_multiedit_no_timestamp_in_any_new_string_continues: MultiEdit where no \
             new_string sets timestamp: must return Continue (payload-neutral, ADR-032 Decision 1). \
             Pre-fix: Block(TimestampStale) — RED GATE. \
             Post-fix: payload-neutral → Continue."
        );
    }

    // -----------------------------------------------------------------------
    // Regression guard: ac020_multiedit_one_new_string_stale_blocks
    // MultiEdit where one edits[i].new_string contains timestamp: "OLD".
    // Both pre- and post-fix: Block(TimestampStale).
    // -----------------------------------------------------------------------
    #[test]
    fn ac020_multiedit_one_new_string_stale_blocks() {
        let on_disk = state_md_no_lock(TS_OLD);
        // Identity replacement: new_string contains timestamp: TS_OLD explicitly.
        let ts_line = format!("timestamp: \"{TS_OLD}\"");
        let edits = vec![(ts_line.as_str(), ts_line.as_str())];

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_multiedit(edits);

        let result = guard_logic(payload, callbacks);

        let expected_msg = canonical_timestamp_stale_message();
        assert_eq!(
            result,
            HookResult::Block {
                reason: expected_msg
            },
            "ac020_multiedit_one_new_string_stale_blocks: MultiEdit where a new_string explicitly \
             sets timestamp: OLD must Block(TimestampStale) both pre- and post-fix (regression guard)."
        );
    }

    // -----------------------------------------------------------------------
    // Regression guard: ac020_write_stale_timestamp_still_blocks
    // Write with stale full content (Write path unchanged by ADR-032).
    // Both pre- and post-fix: Block(TimestampStale).
    // -----------------------------------------------------------------------
    #[test]
    fn ac020_write_stale_timestamp_still_blocks() {
        let on_disk = state_md_no_lock(TS_OLD);
        let proposed = state_md_no_lock(TS_OLD); // stale: same timestamp as on-disk

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_write(&proposed);

        let result = guard_logic(payload, callbacks);

        let expected_msg = canonical_timestamp_stale_message();
        assert_eq!(
            result,
            HookResult::Block {
                reason: expected_msg
            },
            "ac020_write_stale_timestamp_still_blocks: Write with stale timestamp must \
             Block(TimestampStale) — Write path is unconditionally enforced (ADR-032 Decision 2). \
             Regression guard."
        );
    }

    // -----------------------------------------------------------------------
    // Unit test: ac020_new_string_sets_field_helper
    // Tests for new_string_sets_field: found at col-0; not found; indented skipped;
    // multi-line mix.
    // -----------------------------------------------------------------------
    #[test]
    fn ac020_new_string_sets_field_helper() {
        // col-0 match found
        assert!(
            new_string_sets_field("timestamp: \"2026-06-11T10:00:00Z\"", "timestamp"),
            "col-0 timestamp: line must return true"
        );

        // not found (field absent)
        assert!(
            !new_string_sets_field("phase: test\nsome_other: \"val\"", "timestamp"),
            "absent timestamp: must return false"
        );

        // indented sub-field skipped
        assert!(
            !new_string_sets_field(
                "  timestamp: \"2026-06-11T10:00:00Z\"\nphase: test",
                "timestamp"
            ),
            "indented timestamp: must be skipped (not col-0)"
        );

        // tab-indented sub-field skipped
        assert!(
            !new_string_sets_field("\ttimestamp: \"2026-06-11T10:00:00Z\"", "timestamp"),
            "tab-indented timestamp: must be skipped"
        );

        // multi-line mix: col-0 match after some indented lines
        assert!(
            new_string_sets_field(
                "  indented: \"val\"\ntimestamp: \"2026-06-11T10:00:00Z\"\n  more_indent: \"x\"",
                "timestamp"
            ),
            "multi-line: col-0 timestamp: after indented lines must return true"
        );

        // empty new_string
        assert!(
            !new_string_sets_field("", "timestamp"),
            "empty new_string must return false"
        );

        // different field key — present but not the target key
        assert!(
            !new_string_sets_field("version: \"1.0\"", "timestamp"),
            "non-target field present must return false"
        );
    }

    // -----------------------------------------------------------------------
    // Red Gate 3 of 4: ac020_edit_body_lock_held_no_factory_lock_continues
    // Edit: lock held on-disk with stale expires_at; new_string is body text with
    // no factory_lock: line; on-disk timestamp is OLD.
    // Pre-fix: Block(TimestampStale) — Step 6 fires (proposed_ts == on_disk_ts == OLD).
    // Post-fix: payload-neutral → Continue.
    // -----------------------------------------------------------------------
    #[test]
    fn ac020_edit_body_lock_held_no_factory_lock_continues() {
        let on_disk = state_md_with_lock(TS_OLD, EXPIRES_OLD);
        // Body-only Edit: new_string has neither timestamp: nor factory_lock: at col-0.
        let old_string = "# STATE\n";
        let new_string = "# SESSION CHECKPOINT\nbody content here\n";

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_edit(old_string, new_string);

        let result = guard_logic(payload, callbacks);

        assert_eq!(
            result,
            HookResult::Continue,
            "ac020_edit_body_lock_held_no_factory_lock_continues: Body-only Edit with lock held \
             on-disk (no factory_lock: or timestamp: in new_string) must return Continue \
             (payload-neutral, ADR-032 Decision 1+3). \
             Pre-fix: Block(TimestampStale) — RED GATE (Step 6 fires: proposed_ts == on_disk_ts). \
             Post-fix: payload-neutral → Continue."
        );
    }

    // -----------------------------------------------------------------------
    // Regression guard: ac020_edit_factory_lock_in_new_string_stale_expires_blocks
    // Edit: new_string contains BOTH factory_lock: block (stale expires_at) AND
    // timestamp: "NEW" (advancing). Lock held on-disk with stale expires_at.
    // Both pre- and post-fix: Block(LockExpiryStale).
    // (F-ADR032-P2-004: fixture includes timestamp: NEW to ensure same block code
    // pre- and post-fix — true regression guard.)
    // -----------------------------------------------------------------------
    #[test]
    fn ac020_edit_factory_lock_in_new_string_stale_expires_blocks() {
        let on_disk = state_md_with_lock(TS_OLD, EXPIRES_OLD);

        // old_string matches the timestamp+phase+factory_lock section of on-disk content.
        let old_string = format!(
            "timestamp: \"{TS_OLD}\"\nphase: test\nfactory_lock:\n  holder: \"{HOLDER}\"\n  locked_at: \"2026-06-11T10:00:00Z\"\n  expires_at: \"{EXPIRES_OLD}\""
        );
        // new_string advances timestamp: to TS_NEW but keeps factory_lock with stale EXPIRES_OLD.
        let new_string = format!(
            "timestamp: \"{TS_NEW}\"\nphase: test\nfactory_lock:\n  holder: \"{HOLDER}\"\n  locked_at: \"2026-06-11T10:00:00Z\"\n  expires_at: \"{EXPIRES_OLD}\""
        );

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_edit(&old_string, &new_string);

        let result = guard_logic(payload, callbacks);

        let expected_msg = canonical_lock_expiry_stale_message();
        assert_eq!(
            result,
            HookResult::Block {
                reason: expected_msg
            },
            "ac020_edit_factory_lock_in_new_string_stale_expires_blocks: Edit advancing timestamp \
             but keeping stale expires_at must Block(LockExpiryStale) both pre- and post-fix \
             (regression guard, ADR-032 Decision 3 + F-ADR032-P2-004)."
        );
    }

    // -----------------------------------------------------------------------
    // Regression guard: ac020_edit_sets_timestamp_no_factory_lock_stale_expires_blocks
    // Edit: new_string sets timestamp: "NEW" (advancing); no factory_lock: in new_string;
    // lock held on-disk with stale expires_at.
    // Both pre- and post-fix: Block(LockExpiryStale).
    // (Decision 3 option (a): timestamp-advancing Edit must include factory_lock renewal
    // when a lock is held — Step 7 always runs when sets_timestamp=true.)
    // -----------------------------------------------------------------------
    #[test]
    fn ac020_edit_sets_timestamp_no_factory_lock_stale_expires_blocks() {
        let on_disk = state_md_with_lock(TS_OLD, EXPIRES_OLD);

        // Edit advances timestamp only; factory_lock block (with stale expires) is inherited
        // by reconstruction from on-disk content.
        let old_ts_line = format!("timestamp: \"{TS_OLD}\"");
        let new_ts_line = format!("timestamp: \"{TS_NEW}\"");

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_edit(&old_ts_line, &new_ts_line);

        let result = guard_logic(payload, callbacks);

        let expected_msg = canonical_lock_expiry_stale_message();
        assert_eq!(
            result,
            HookResult::Block {
                reason: expected_msg
            },
            "ac020_edit_sets_timestamp_no_factory_lock_stale_expires_blocks: timestamp-advancing \
             Edit that does NOT renew factory_lock.expires_at must Block(LockExpiryStale) \
             both pre- and post-fix (ADR-032 Decision 3 option (a) regression guard)."
        );
    }

    // -----------------------------------------------------------------------
    // Red Gate 4 of 4: ac020_edit_factory_lock_only_stale_expires_blocks
    // Edit where new_string sets factory_lock: block with stale expires_at but no
    // timestamp: line. Lock held on-disk with stale expires_at.
    // Pre-fix: Block(TimestampStale) — Step 6 fires (proposed_ts == on_disk_ts == OLD).
    // Post-fix: sets_factory_lock=true, sets_timestamp=false → skip Steps 4-6; Step 7
    //   runs → Block(LockExpiryStale).
    // Pre-fix result ≠ Post-fix result → RED GATE.
    // -----------------------------------------------------------------------
    #[test]
    fn ac020_edit_factory_lock_only_stale_expires_blocks() {
        let on_disk = state_md_with_lock(TS_OLD, EXPIRES_OLD);

        // Identity replacement of the factory_lock block (stale expires_at unchanged).
        // new_string starts with factory_lock: at col-0 (not indented) → sets_factory_lock=true.
        // new_string has no timestamp: line → sets_timestamp=false.
        let lock_block = format!(
            "factory_lock:\n  holder: \"{HOLDER}\"\n  locked_at: \"2026-06-11T10:00:00Z\"\n  expires_at: \"{EXPIRES_OLD}\""
        );

        let warn_log = Arc::new(Mutex::new(Vec::new()));
        let callbacks = make_callbacks_with_disk(on_disk, warn_log.clone());
        let payload = payload_edit(&lock_block, &lock_block); // identity replacement

        let result = guard_logic(payload, callbacks);

        let expected_msg = canonical_lock_expiry_stale_message();
        assert_eq!(
            result,
            HookResult::Block {
                reason: expected_msg
            },
            "ac020_edit_factory_lock_only_stale_expires_blocks: factory_lock-only Edit with stale \
             expires_at must Block(LockExpiryStale) (ADR-032 Decision 3). \
             Pre-fix: Block(TimestampStale) — RED GATE. \
             Post-fix: sets_factory_lock=true → skip timestamp checks; Step 7 → Block(LockExpiryStale)."
        );
    }
}
