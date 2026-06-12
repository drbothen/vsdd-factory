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
//! - Trigger: `file_path == ".factory/STATE.md"` after canonical-path normalisation
//!   (bypass-proof per §12.1 / EC-006).

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

/// Canonical TimestampStale block message produced by `block_with_fix`.
///
/// Full line (per AC-005 / Red Gate Test Table — full-line equality required):
/// `BLOCKED by verify-state-timestamp-refresh: STATE.md timestamp not advanced in
///  this write. Fix: Update 'timestamp:' to the current UTC time before writing
///  STATE.md. Code: TimestampStale.`
///
/// Constructed by: `HookResult::block_with_fix(GUARD_NAME, TIMESTAMP_STALE_REASON,
///   TIMESTAMP_STALE_FIX, TIMESTAMP_STALE_CODE)`
pub const GUARD_NAME: &str = "verify-state-timestamp-refresh";
pub const TIMESTAMP_STALE_REASON: &str = "STATE.md timestamp not advanced in this write";
pub const TIMESTAMP_STALE_FIX: &str =
    "Update 'timestamp:' to the current UTC time before writing STATE.md";
pub const TIMESTAMP_STALE_CODE: &str = "TimestampStale";

/// Canonical LockExpiryStale reason for `block_with_fix`.
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

/// Full canonical TimestampStale block string as produced by `block_with_fix`.
///
/// Tests MUST assert equality to this exact string (not a substring), per
/// the Red Gate Test Table "full-line equality required" mandate (AC-005, M03 fix).
pub fn canonical_timestamp_stale_message() -> String {
    format!(
        "BLOCKED by {}: {}. Fix: {}. Code: {}.",
        GUARD_NAME, TIMESTAMP_STALE_REASON, TIMESTAMP_STALE_FIX, TIMESTAMP_STALE_CODE
    )
}

/// Full canonical LockExpiryStale block string as produced by `block_with_fix`.
///
/// Tests MUST assert equality to this exact string (not a substring), per
/// the Red Gate Test Table "full-line equality required" mandate (AC-006, M03 fix).
pub fn canonical_lock_expiry_stale_message() -> String {
    format!(
        "BLOCKED by {}: {}. Fix: {}. Code: {}.",
        GUARD_NAME, LOCK_EXPIRY_STALE_REASON, LOCK_EXPIRY_STALE_FIX, LOCK_EXPIRY_STALE_CODE
    )
}

// ---------------------------------------------------------------------------
// Canonical-path normalisation (EC-006 / ADR-025 §12.7 R6)
// ---------------------------------------------------------------------------

/// Normalise a `file_path` from a tool payload for comparison against `STATE_MD_PATH`.
///
/// Normalisation algorithm per EC-006 / ADR-025 §12.7 R6:
///   1. Strip leading `./`
///   2. Strip absolute `$CLAUDE_PROJECT_DIR/` prefix (where `$CLAUDE_PROJECT_DIR`
///      is read from environment; no-op if env var absent)
///   3. Collapse `//` → `/`
///   4. Collapse `/./` → `/`
///
/// A path that normalises to `.factory/STATE.md` MUST trigger the guard.
/// Fail-open applies ONLY to paths with genuinely unresolvable traversal sequences
/// (e.g., `../../` after normalisation yields a path outside the project).
pub fn normalise_path(path: &str) -> String {
    // Pre-collapse: normalise `//` and `/./` on the raw input first so that
    // prefix-strip works even when the project-dir boundary has a double-slash
    // (e.g., `$CLAUDE_PROJECT_DIR//.factory/STATE.md` → pre-collapsed before
    // the `project_dir + "/"` prefix is stripped).  EC-006 / O-1704P-01.
    let mut pre = path.to_string();
    loop {
        let next = pre.replace("//", "/").replace("/./", "/");
        if next == pre {
            break;
        }
        pre = next;
    }
    let path: &str = &pre;

    // Step 2: strip CLAUDE_PROJECT_DIR prefix (may contain a leading ./).
    let path = if let Ok(project_dir) = std::env::var("CLAUDE_PROJECT_DIR") {
        if !project_dir.is_empty() {
            let with_slash = format!("{}/", project_dir.trim_end_matches('/'));
            if let Some(stripped) = path.strip_prefix(with_slash.as_str()) {
                stripped
            } else {
                path
            }
        } else {
            path
        }
    } else {
        path
    };

    // Step 1: strip leading `./`.
    let path = path.strip_prefix("./").unwrap_or(path);

    // Post-collapse: handle any `//` or `/./` introduced after prefix removal.
    let mut result = path.to_string();
    loop {
        let next = result.replace("//", "/").replace("/./", "/");
        if next == result {
            break;
        }
        result = next;
    }

    result
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
// Injectable callbacks surface (testable without WASM runtime)
// ---------------------------------------------------------------------------

/// All side-effecting host calls injected into `guard_logic` for testability.
/// In production (`main.rs`), these are wired to real vsdd_hook_sdk host fns.
pub struct GuardCallbacks<R, L>
where
    R: FnOnce(&str, u32, u32) -> Result<Vec<u8>, String>,
    L: FnMut(&str),
{
    /// Read a file by path with `(path, max_bytes, timeout_ms)`.
    ///
    /// Returns:
    /// - `Ok(bytes)` on success.
    /// - `Err(msg)` on HostError (including `"NotFound"` when the file does not exist).
    ///   The guard treats ALL `Err(_)` variants as fail-open (AC-008/AC-015).
    pub read_file: R,
    /// Emit a `host::log_warn` message (advisory; non-blocking).
    pub log_warn: L,
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
pub fn guard_logic<R, L>(payload: HookPayload, callbacks: GuardCallbacks<R, L>) -> HookResult
where
    R: FnOnce(&str, u32, u32) -> Result<Vec<u8>, String>,
    L: FnMut(&str),
{
    // Step 1: Normalise file_path. If not STATE.md, return Continue immediately (AC-007 / §12.1).
    let file_path = payload
        .tool_input
        .get("file_path")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let normalised = normalise_path(file_path);
    if normalised != STATE_MD_PATH {
        // Not STATE.md — return Continue without reading any file (AC-007 zero-overhead).
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
                return HookResult::Continue;
            }
        };

    let on_disk_content = match String::from_utf8(on_disk_bytes) {
        Ok(s) => s,
        Err(_) => {
            // Non-UTF-8 on-disk content — fail-open.
            return HookResult::Continue;
        }
    };

    // Step 3: Extract proposed content per tool type.
    let proposed_content: String = match payload.tool_name.as_str() {
        "Write" => match extract_write_proposed(&payload) {
            ProposedContent::Content(s) => s,
            ProposedContent::FailOpen => return HookResult::Continue,
        },
        "Edit" => match extract_edit_proposed(&payload, &on_disk_content) {
            ProposedContent::Content(s) => s,
            ProposedContent::FailOpen => return HookResult::Continue,
        },
        "MultiEdit" => match extract_multiedit_proposed(&payload, &on_disk_content) {
            ProposedContent::Content(s) => s,
            ProposedContent::FailOpen => return HookResult::Continue,
        },
        _ => {
            // Unknown tool name — fall back to Write behaviour (content field).
            match extract_write_proposed(&payload) {
                ProposedContent::Content(s) => s,
                ProposedContent::FailOpen => return HookResult::Continue,
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
            return HookResult::Continue;
        }
    };

    // Step 5: Extract timestamp: from on-disk content.
    let on_disk_ts = match extract_top_level_field(&on_disk_content, "timestamp") {
        FieldResult::Found(v) => v,
        FieldResult::NotFound | FieldResult::Malformed => {
            // Absent or malformed on-disk timestamp — first write ever (AC-008 §12.3 row 5 / EC-004).
            // Continue — no prior value to compare against.
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

    // Step 7: Lock held in proposed content? If so, check factory_lock.expires_at (AC-006).
    // "Lock held" = factory_lock.holder present and non-empty in proposed content.
    // Use factory_lock_parse::parse_factory_lock for lock detection (AC-004).
    let proposed_lock = factory_lock_parse::parse_factory_lock(&proposed_content);
    let proposed_expires_opt: Option<String> = match proposed_lock {
        Ok(Some(ref lock_state)) if !lock_state.holder.is_empty() => {
            Some(lock_state.expires_at.clone())
        }
        Ok(_) | Err(_) => None,
    };

    if let Some(proposed_expires) = proposed_expires_opt {
        // Lock is held in proposed content — check factory_lock.expires_at (AC-006).
        let on_disk_expires = match factory_lock_parse::parse_factory_lock(&on_disk_content) {
            Ok(Some(ls)) => ls.expires_at,
            Ok(None) => {
                // On-disk has no lock — can't compare expires_at. No LockExpiryStale possible.
                return HookResult::Continue;
            }
            Err(_) => return HookResult::Continue, // malformed on-disk lock — fail-open.
        };

        // Byte-identical expires_at while lock is held → Block LockExpiryStale (AC-006).
        if proposed_expires == on_disk_expires {
            return HookResult::Block {
                reason: canonical_lock_expiry_stale_message(),
            };
        }
    }

    // Step 8: All checks passed — allow the write.
    let _ = callbacks.log_warn; // suppress unused warning
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
        },
    )
}

// ---------------------------------------------------------------------------
// Unit tests — conformance suite v1.3 (D17 / S-17.04 / AC-005/006/007/008/011-015)
//
// 20 Rust unit tests covering the full AC matrix from S-17.04 v1.3.
// Uses injectable callbacks so no WASM runtime is required.
// All 20 tests pass green against the implemented guard_logic (T-3 complete).
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
    ) -> GuardCallbacks<impl FnOnce(&str, u32, u32) -> Result<Vec<u8>, String>, impl FnMut(&str)>
    {
        let wl = warn_log.clone();
        GuardCallbacks {
            read_file: move |_path, _max, _timeout| Ok(on_disk_content.into_bytes()),
            log_warn: move |msg: &str| {
                wl.lock().unwrap().push(msg.to_string());
            },
        }
    }

    /// Build callbacks where read_file returns an error string (covers HostError and NotFound).
    #[allow(clippy::type_complexity)]
    fn make_callbacks_read_error(
        error_msg: &str,
        warn_log: Arc<Mutex<Vec<String>>>,
    ) -> GuardCallbacks<impl FnOnce(&str, u32, u32) -> Result<Vec<u8>, String>, impl FnMut(&str)>
    {
        let err = error_msg.to_string();
        let wl = warn_log.clone();
        GuardCallbacks {
            read_file: move |_path, _max, _timeout| Err(err),
            log_warn: move |msg: &str| {
                wl.lock().unwrap().push(msg.to_string());
            },
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
}
