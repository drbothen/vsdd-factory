//! validate-wave-handoff-completeness — PostToolUse WASM hook plugin.
//!
//! PostToolUse gate that validates HANDOFF.md completeness on every Write or
//! Edit tool call that targets a `HANDOFF.md` path. When the written content
//! is incomplete, the gate blocks with `HandoffIncomplete` listing all failing
//! fields in a single invocation (BC-4.14.001 INV2).
//!
//! # Behavioral Contract
//!
//! BC-4.14.001 — validate-wave-handoff-completeness WASM gate blocks
//! HandoffIncomplete on PostToolUse HANDOFF.md writes.
//!
//! # 5-step Evaluation Order (BC-4.14.001 INV3 / ADR-026 §Decision 9)
//!
//! 1. Non-HANDOFF.md target → no-op (Continue). AC-001 / PC4.
//! 2. `next_wave_stories: []` (EPIC-COMPLETE) → validate `epic_status: complete`. AC-002 / PC2a.
//! 3. `wave_id == 1` AND NOT EPIC-COMPLETE → no-op (Continue). AC-003 / PC3.
//! 4. `wave_id > 1` → full validation of all 9 base required fields. AC-004 / PC7.
//! 5. `wave_id` absent → fail-closed (Continue is NOT returned). AC-005 / PC3+PC8.
//!
//! # Architecture compliance
//!
//! - Pure-parse: no filesystem access, no process spawning (BC-4.14.001 INV1).
//! - `#[deny(warnings)]` via workspace `[lints]` (`-- -D warnings` in CI).
//! - No `unwrap()` or `expect()` in non-test code paths.
//! - No `regex` crate — stay within WASM fuel budget.
//! - No dependency on `crates/factory-dispatcher` (would create circular dep).

use vsdd_hook_sdk::{HookPayload, HookResult};

/// HOST_ABI_VERSION declares the ABI contract version this plugin was built
/// against. Must remain 1.
pub const HOST_ABI_VERSION: u32 = 1;

/// Maximum bytes to read from a HANDOFF.md payload.
///
/// Set to 512 KiB (524_288 bytes) — consistent with the cap used by sibling
/// validate-* hook plugins (validate-state-structure, validate-burst-log).
pub const MAX_BYTES: u32 = 524_288;

// ---------------------------------------------------------------------------
// Pure gate types (testable without wasmtime)
// ---------------------------------------------------------------------------

/// Input context for the pure 5-step evaluation function.
///
/// Decouples the pure gate logic from the WASM dispatcher protocol so the
/// evaluation function can be unit-tested without a WASM runtime.
///
/// `is_first_wave` is computed PAYLOAD-ONLY by the caller before invoking
/// `check_handoff_completeness`: `is_first_wave = (payload.wave_id == 1)`.
/// A missing `wave_id` is represented as `is_first_wave = false` (absent
/// `wave_id` is NOT treated as wave-1 — fail-closed per BC-4.14.001 PC3/PC8).
///
/// `handoff_content` holds the raw YAML string being validated, or `None`
/// when the tool call did not target a HANDOFF.md path (non-HANDOFF.md no-op).
///
/// `close_wave_mode` is reserved for integration callers; at the unit level
/// it is always `false` (the gate is a pure PostToolUse write-time check).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateContext {
    /// Derived from `payload.wave_id == 1` (payload-only; BC-4.14.001 PC3).
    pub is_first_wave: bool,
    /// Absolute or relative path of the file being written/edited.
    pub file_path: String,
    /// Raw YAML content being written. `None` signals non-HANDOFF.md target.
    pub handoff_content: Option<String>,
    /// Reserved; always `false` in unit tests.
    pub close_wave_mode: bool,
}

/// Result of the pure gate evaluation.
///
/// Mirrors the `HookResult` vocabulary but is decoupled from the SDK type so
/// the pure function can be tested without linking vsdd-hook-sdk.
///
/// `Block.code` carries the machine-readable error code:
/// - `"HandoffIncomplete"` — one or more required base fields are missing or malformed.
/// - `"MissingEpicStatus"` — EPIC-COMPLETE write is missing `epic_status: complete`.
/// - `"UnexpectedEpicStatus"` — non-final wave write includes `epic_status` (forbidden).
///
/// `Block.message` carries the human-readable message listing all failing fields
/// in deterministic ADR-026 §D2 schema order (BC-4.14.001 INV2, INV4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GateResult {
    /// Gate allows the write. Continue.
    Continue,
    /// Gate blocks the write. `code` is the machine-readable error class;
    /// `message` lists all failing fields in one invocation (INV2).
    Block {
        /// Machine-readable error code.
        code: &'static str,
        /// Human-readable message naming all failing fields.
        message: String,
    },
}

// ---------------------------------------------------------------------------
// Pure evaluation function (BC-4.14.001 INV3 5-step order)
// ---------------------------------------------------------------------------

/// Evaluate HANDOFF.md completeness according to the 5-step evaluation order.
///
/// Step 1: Non-HANDOFF.md target → `GateResult::Continue` (no-op).
/// Step 2: EPIC-COMPLETE detection (`next_wave_stories: []`) → validate
///         `epic_status: complete` and base scalar fields; return appropriate
///         `GateResult`.
/// Step 3: `wave_id == 1` AND NOT EPIC-COMPLETE → `GateResult::Continue` (no-op).
/// Step 4: `wave_id > 1` → full validation of all 9 base required fields.
/// Step 5: `wave_id` absent → fail-closed (full validation runs; at minimum
///         `HandoffIncomplete: [wave_id]`).
///
/// EPIC-COMPLETE is derived from `handoff_content.next_wave_stories == []`
/// (payload-parse), NOT from `ctx.is_first_wave`. `is_first_wave` MUST NOT
/// short-circuit the EPIC-COMPLETE branch (step 2 precedes step 3 per INV3;
/// F-P34-002 clarifying note in VP-083).
///
/// All validation is performed on the in-memory YAML parse result only (INV1).
/// No filesystem, git, or sprint-state.yaml access is performed.
///
/// # BC-5.38.001 compliance
///
/// This function is NON-TRIVIAL (5-step branching logic, YAML parse, field
/// validation, deterministic error aggregation). Body is `todo!()` per
/// BC-5.38.001 obligation. The implementer fills in the real logic.
///
/// # Self-Check (BC-5.38.005 invariant 1)
///
/// "If I include this real implementation, will the test for this function
/// pass trivially without any implementer work?" — Yes. Therefore: `todo!()`.
pub fn check_handoff_completeness(ctx: &GateContext) -> GateResult {
    todo!(
        "S-18.02: implement 5-step HANDOFF.md completeness evaluation per \
        BC-4.14.001 INV3. Inputs: GateContext {{ file_path: {:?}, \
        is_first_wave: {}, handoff_content: {:?} }}",
        ctx.file_path,
        ctx.is_first_wave,
        ctx.handoff_content
    )
}

// ---------------------------------------------------------------------------
// WASM-facing gate function (PostToolUse dispatcher integration)
// ---------------------------------------------------------------------------

/// PostToolUse hook entry point: parse the dispatcher payload and invoke
/// the pure 5-step gate.
///
/// Extracts `file_path` from `payload.tool_input["file_path"]` (Write) or
/// `payload.tool_input["path"]` (Edit), and `content` from
/// `payload.tool_input["content"]` (Write) or the diff+file read (Edit).
///
/// Maps `GateResult::Continue` → `HookResult::Continue`.
/// Maps `GateResult::Block { .. }` → `HookResult::block_with_fix(...)`.
///
/// # Fail-open on crash (BC-4.14.001 PC6)
///
/// The registry sets `on_error = "continue"` so any WASM panic causes the
/// dispatcher to fail open (Continue). This function itself must never panic
/// in non-test code — all error paths return `HookResult::Continue` or a
/// `HookResult::block_with_fix(...)`.
///
/// # BC-5.38.001 compliance
///
/// NON-TRIVIAL (payload extraction, path pattern matching, content routing).
/// Body is `todo!()` per BC-5.38.001 obligation.
///
/// # Self-Check (BC-5.38.005 invariant 1)
///
/// "If I include this real implementation, will the test for this function
/// pass trivially without any implementer work?" — Yes. Therefore: `todo!()`.
pub fn on_post_tool_use(payload: HookPayload) -> HookResult {
    todo!(
        "S-18.02: implement PostToolUse payload extraction and gate dispatch \
        per BC-4.14.001 dispatcher protocol. \
        event={:?} tool={:?}",
        payload.event_name,
        payload.tool_name
    )
}

// ---------------------------------------------------------------------------
// YAML parse helpers (pure; WASM fuel-budget conscious)
// ---------------------------------------------------------------------------

/// Parse the HANDOFF.md YAML string and extract the `wave_id` field as an
/// optional `i64`.
///
/// Returns `None` when the field is absent (triggers fail-closed path).
/// Returns `Some(n)` when `wave_id` is a valid integer.
/// Returns an error string when the YAML is malformed.
///
/// # BC-5.38.001 compliance
///
/// NON-TRIVIAL. Body is `todo!()`.
///
/// # Self-Check (BC-5.38.005 invariant 1)
///
/// "If I include this real implementation, will the test for this function
/// pass trivially without any implementer work?" — Yes. Therefore: `todo!()`.
pub fn extract_wave_id(yaml_str: &str) -> Result<Option<i64>, String> {
    todo!(
        "S-18.02: parse HANDOFF.md YAML and extract wave_id field. \
        Input length: {} bytes",
        yaml_str.len()
    )
}

/// Determine whether the HANDOFF.md payload is in EPIC-COMPLETE context.
///
/// EPIC-COMPLETE context is defined PAYLOAD-ONLY: `next_wave_stories: []`
/// (empty list) in the parsed YAML. Non-empty `next_wave_stories` or absent
/// `next_wave_stories` → NOT EPIC-COMPLETE. (BC-4.14.001 PC2a)
///
/// # BC-5.38.001 compliance
///
/// NON-TRIVIAL. Body is `todo!()`.
///
/// # Self-Check (BC-5.38.005 invariant 1)
///
/// "If I include this real implementation, will the test for this function
/// pass trivially without any implementer work?" — Yes. Therefore: `todo!()`.
pub fn is_epic_complete(yaml_str: &str) -> Result<bool, String> {
    todo!(
        "S-18.02: parse HANDOFF.md YAML and detect EPIC-COMPLETE context \
        (next_wave_stories == []). Input length: {} bytes",
        yaml_str.len()
    )
}

/// Validate all 9 base required fields per ADR-026 §Decision 2 schema.
///
/// Returns a `Vec<String>` of failing field names in deterministic schema order
/// (BC-4.14.001 INV4). An empty `Vec` means all fields are present and valid.
///
/// Field-type rules (BC-4.14.001 PC7 / F-P32-001):
/// - SCALAR fields (`wave_id`, `last_verified_develop_sha`,
///   `precompact_flush_sha`, `factory_lock_holder`): key must exist; value
///   must be non-empty string OR null only where schema permits null
///   (`precompact_flush_sha`, `factory_lock_holder`).
/// - LIST fields (`active_bcs`, `next_wave_stories`, `open_decisions`,
///   `pending_fixes`, `process_gaps`): key must exist; value must be a
///   syntactically-valid list. Empty list (`[]`) is VALID (NOT malformed).
///
/// Also validates `epic_status` conditional field per PC2a when `epic_complete`
/// is true.
///
/// # BC-5.38.001 compliance
///
/// NON-TRIVIAL (9-field validation loop, scalar vs list type dispatch,
/// deterministic ordering). Body is `todo!()`.
///
/// # Self-Check (BC-5.38.005 invariant 1)
///
/// "If I include this real implementation, will the test for this function
/// pass trivially without any implementer work?" — Yes. Therefore: `todo!()`.
pub fn validate_base_fields(
    yaml_str: &str,
    epic_complete: bool,
) -> Result<Vec<String>, String> {
    todo!(
        "S-18.02: validate all 9 base required fields per ADR-026 §Decision 2 schema. \
        epic_complete={epic_complete}. Input length: {} bytes",
        yaml_str.len()
    )
}

/// Check whether the target file path matches the HANDOFF.md pattern.
///
/// Returns `true` iff the path ends with `HANDOFF.md` (case-sensitive match
/// per BC-4.14.001 PC4). No filesystem access is performed.
///
/// # BC-5.38.001 / GREEN-BY-DESIGN check
///
/// This function has zero branching beyond a single `str::ends_with` call,
/// no I/O, no non-trivial helpers, and a body ≤ 3 lines. It satisfies all
/// four GREEN-BY-DESIGN criteria (BC-5.38.002):
///   1. Zero branching (single `ends_with` — no `if`/`match`/`?`/`unwrap`)
///   2. No I/O
///   3. No calls to non-trivial helpers
///   4. Body ≤ 3 lines
///
/// Per BC-5.38.002 GREEN-BY-DESIGN protocol and BC-5.38.005 self-check:
/// "If I include this real implementation, will the test for this function
/// pass trivially without any implementer work?" — YES for any test that
/// exercises `path_is_handoff`. This function is included as a real body
/// because it is correct-by-construction; it is listed in the stub commit
/// report under GREEN-BY-DESIGN.
pub fn path_is_handoff(file_path: &str) -> bool {
    file_path.ends_with("HANDOFF.md")
}

/// Emit a 200-line advisory warning via the host log.
///
/// Called when `handoff_content` line count exceeds 200 (BC-4.14.001 PC5 /
/// INV5). The warning is advisory only — the gate continues parsing and
/// validating all fields normally. This function does NOT cause a block.
///
/// # BC-5.38.001 compliance
///
/// NON-TRIVIAL (host log call + format string). Body is `todo!()`.
///
/// # Self-Check (BC-5.38.005 invariant 1)
///
/// "If I include this real implementation, will the test for this function
/// pass trivially without any implementer work?" — Yes. Therefore: `todo!()`.
pub fn emit_over_200_line_advisory(line_count: usize) {
    todo!(
        "S-18.02: emit plugin.log advisory via host::log_warn for \
        HANDOFF.md body exceeding 200-line cap. \
        line_count={line_count}"
    )
}
