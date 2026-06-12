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
/// 64 KiB is sufficient; mirrors `verify-factory-lock` (ADR-025 Decision 12 §12.5).
pub const STATE_MD_MAX_BYTES: u32 = 65536;

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

    let on_disk_content = match String::from_utf8(on_disk_bytes) {
        Ok(s) => s,
        Err(_) => {
            // Non-UTF-8 on-disk content — fail-open.
            (callbacks.log_warn)(
                "verify-state-timestamp-refresh: fail-open utf8 (STATE.md is not valid UTF-8)",
            );
            (callbacks.write_stderr)(
                "verify-state-timestamp-refresh: guard_ran (continue: fail-open utf8)\n",
            );
            return HookResult::Continue;
        }
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
        "Edit" => match extract_edit_proposed(&payload, &on_disk_content) {
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
        "MultiEdit" => match extract_multiedit_proposed(&payload, &on_disk_content) {
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

    // Step 5: Extract timestamp: from on-disk content.
    let on_disk_ts = match extract_top_level_field(&on_disk_content, "timestamp") {
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

    // Step 7: Lock held in proposed content? If so, enforce factory_lock.expires_at freshness
    // (AC-006 / AC-016 / AC-017 / ADR-025 §12.2 revised).
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
    if let Some(proposed_subfields) = extract_lock_subfields(&proposed_content) {
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
            let on_disk_subfields = extract_lock_subfields(&on_disk_content);
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
    #![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

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
    // AC-012 — Edit payload: reconstruct stale timestamp → Block: TimestampStale
    // Traces: AC-012 / ADR-025 D12 §12.2 / BC-5.40.001 PC4
    //
    // Edit payload: old_string = old_ts_line, new_string = same old_ts_line
    // (timestamp not updated in the edit). Guard reconstructs proposed from on-disk
    // + edit fragment and finds timestamp still byte-identical.
    //
    // GREEN: guard blocks — reconstructed Edit proposed has stale timestamp.
    // -----------------------------------------------------------------------

    #[test]
    fn test_edit_payload_reconstruct_stale_timestamp_blocks() {
        let on_disk = state_md_no_lock(TS_OLD);

        // The edit changes something else (e.g., the phase field), NOT the timestamp.
        // old_string: the phase line in on-disk content.
        // new_string: different phase value but timestamp unchanged.
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

        let expected_msg = canonical_timestamp_stale_message();
        match result {
            HookResult::Block { reason } => {
                assert_eq!(
                    reason, expected_msg,
                    "test_edit_payload_reconstruct_stale_timestamp_blocks: After Edit reconstruction, \
                     unchanged timestamp must Block with FULL canonical message. \
                     Expected: {expected_msg:?}. Got: {reason:?}"
                );
            }
            HookResult::Continue => panic!(
                "test_edit_payload_reconstruct_stale_timestamp_blocks: expected Block(TimestampStale) \
                 but got Continue. Edit reconstruction yielded unchanged timestamp → must Block. RED GATE."
            ),
            other => panic!("Expected Block, got: {:?}", other),
        }
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
    // AC-013 — MultiEdit payload: reconstruct stale timestamp → Block: TimestampStale
    // Traces: AC-013 / ADR-025 D12 §12.2 / BC-5.40.001 PC4
    //
    // MultiEdit: two edits applied sequentially. Neither edit touches the timestamp.
    // Guard reconstructs full content from edits[] and finds timestamp unchanged.
    //
    // GREEN: guard blocks — reconstructed MultiEdit proposed has stale timestamp.
    // -----------------------------------------------------------------------

    #[test]
    fn test_multiedit_payload_reconstruct_stale_timestamp_blocks() {
        let on_disk = state_md_no_lock(TS_OLD);

        // Two edits that don't touch the timestamp line.
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

        let expected_msg = canonical_timestamp_stale_message();
        match result {
            HookResult::Block { reason } => {
                assert_eq!(
                    reason, expected_msg,
                    "test_multiedit_payload_reconstruct_stale_timestamp_blocks: After MultiEdit \
                     reconstruction, unchanged timestamp must Block with FULL canonical message. \
                     Expected: {expected_msg:?}. Got: {reason:?}"
                );
            }
            HookResult::Continue => panic!(
                "test_multiedit_payload_reconstruct_stale_timestamp_blocks: expected Block(TimestampStale) \
                 but got Continue. MultiEdit reconstruction has unchanged timestamp → must Block. RED GATE."
            ),
            other => panic!("Expected Block, got: {:?}", other),
        }
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
}
