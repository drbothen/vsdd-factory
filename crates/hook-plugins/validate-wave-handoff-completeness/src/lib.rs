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
//!    Returns Continue WITHOUT inspecting `epic_status` — `UnexpectedEpicStatus` does
//!    NOT fire here (F-A003 / BC-4.14.001 INV3 / ADR-026 §Decision 9 v1.16).
//! 4. `wave_id > 1` → full validation of all 9 base required fields;
//!    `UnexpectedEpicStatus` evaluated HERE (not at step 3). AC-004 / PC7.
//! 5. `wave_id` absent → fail-closed (Continue is NOT returned). AC-005 / PC3+PC8.
//!
//! # Architecture compliance
//!
//! - Uses `host::read_file` in `on_post_tool_use` to acquire the FULL written
//!   file content after every Edit or Write. Fail-open on read failure (VP-083
//!   no-false-positive invariant; BC-4.14.001 PC6).
//! - Pure-parse core (`check_handoff_completeness`): no filesystem access, no
//!   process spawning (BC-4.14.001 INV1).
//! - `#[deny(warnings)]` via workspace `[lints]` (`-- -D warnings` in CI).
//! - No `unwrap()` or `expect()` in non-test code paths.
//! - No `regex` crate — stay within WASM fuel budget.
//! - No dependency on `crates/factory-dispatcher` (would create circular dep).
//! - `wave_id` must be a positive integer (>= 1); 0 and negative values are
//!   MALFORMED per BC-4.14.001 PC7 / EC-017.
//! - YAML is parsed ONCE per invocation and threaded through helpers (F-A006).
//!   A YAML parse error blocks with `HandoffIncomplete: YAML parse error at line L, column C`
//!   BEFORE the wave-1 no-op short-circuit (EC-001): malformed YAML never
//!   silently continues.
//! - `UnexpectedEpicStatus` fires ONLY at step 4 (wave_id>1 writes). It does NOT
//!   fire at step 3 (wave_id==1 no-op path) per BC-4.14.001 INV3 / F-A003.

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
/// Wave-1 identity is derived PAYLOAD-ONLY inside `check_handoff_completeness`
/// by parsing `wave_id` from `handoff_content` (F-A005). No external `is_first_wave`
/// flag is needed — the pure core computes it from the payload content, eliminating
/// the mis-wiring vector identified in VP-083 F-P34-002.
///
/// `handoff_content` holds the raw YAML string being validated, or `None`
/// when the tool call did not target a HANDOFF.md path (non-HANDOFF.md no-op).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateContext {
    /// Absolute or relative path of the file being written/edited.
    pub file_path: String,
    /// Raw YAML content being written. `None` signals non-HANDOFF.md target.
    pub handoff_content: Option<String>,
}

/// Result of the pure gate evaluation.
///
/// Mirrors the `HookResult` vocabulary but is decoupled from the SDK type so
/// the pure function can be tested without linking vsdd-hook-sdk.
///
/// `Block.code` carries the machine-readable error code:
/// - `"HandoffIncomplete"` — one or more required base fields are missing or malformed.
/// - `"MissingEpicStatus"` — EPIC-COMPLETE write is missing `epic_status: complete`.
/// - `"UnexpectedEpicStatus"` — non-final wave write (wave_id>1) includes `epic_status`
///   (forbidden). NOT emitted at step 3 (wave_id==1 no-op) per F-A003.
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
// Canonical field order (BC-4.14.001 INV4 / ADR-026 §Decision 2 schema order)
// ---------------------------------------------------------------------------

/// The 9 base required fields in deterministic ADR-026 §Decision 2 schema order.
///
/// Scalar fields: wave_id, last_verified_develop_sha, precompact_flush_sha,
///   factory_lock_holder.
/// List fields: active_bcs, next_wave_stories, open_decisions, pending_fixes,
///   process_gaps.
const BASE_FIELDS_ORDERED: &[&str] = &[
    "wave_id",
    "last_verified_develop_sha",
    "precompact_flush_sha",
    "factory_lock_holder",
    "active_bcs",
    "next_wave_stories",
    "open_decisions",
    "pending_fixes",
    "process_gaps",
];

/// Nullable scalar fields — `null` is a valid value (key must still be present).
const NULLABLE_SCALAR_FIELDS: &[&str] = &["precompact_flush_sha", "factory_lock_holder"];

/// List-typed fields — key must exist and value must be a valid list.
/// An empty list is valid. (BC-4.14.001 PC7 / F-P32-001)
const LIST_FIELDS: &[&str] = &[
    "active_bcs",
    "next_wave_stories",
    "open_decisions",
    "pending_fixes",
    "process_gaps",
];

// ---------------------------------------------------------------------------
// Pure evaluation function (BC-4.14.001 INV3 5-step order)
// ---------------------------------------------------------------------------

/// Evaluate HANDOFF.md completeness according to the 5-step evaluation order.
///
/// Step 1: Non-HANDOFF.md target → `GateResult::Continue` (no-op).
/// Step 2: EPIC-COMPLETE detection (`next_wave_stories: []`) → validate
///         `epic_status: complete` and base fields; return appropriate `GateResult`.
/// Step 3: `wave_id == 1` AND NOT EPIC-COMPLETE → `GateResult::Continue` (no-op).
///         Returns Continue WITHOUT inspecting `epic_status`. `UnexpectedEpicStatus`
///         does NOT fire here (F-A003 / BC-4.14.001 INV3 v1.16 adjudication).
/// Step 4: `wave_id > 1` → full validation of all 9 base required fields;
///         `UnexpectedEpicStatus` evaluated HERE (not between step 2 and step 3).
/// Step 5: `wave_id` absent → fail-closed (full validation runs; at minimum
///         `HandoffIncomplete: [wave_id]`).
///
/// YAML is parsed ONCE (F-A006 / EC-001). A parse error blocks with
/// `HandoffIncomplete: YAML parse error at line L, column C` (or generic form
/// when location unavailable) BEFORE any no-op short-circuit.
///
/// Wave-1 identity is derived from the parsed `wave_id` field (F-A005 /
/// BC-4.14.001 PC3): the caller does not need to set `is_first_wave` externally.
///
/// All validation is performed on the in-memory YAML parse result only (INV1).
/// No filesystem, git, or sprint-state.yaml access is performed.
pub fn check_handoff_completeness(ctx: &GateContext) -> GateResult {
    // Step 1: Non-HANDOFF.md target → no-op (PC4, INV3 step 1).
    if !path_is_handoff(&ctx.file_path) {
        return GateResult::Continue;
    }

    // None is unreachable from on_post_tool_use (always produces Some or
    // returns early), but IS reachable from test-only GateContext constructors.
    // Treat as empty string → YAML parse of "" → all base fields fail → Block
    // (fail-closed).
    let content = match &ctx.handoff_content {
        Some(c) => c.as_str(),
        None => "",
    };

    // Emit over-200-line advisory if needed (PC5 / INV5). Advisory only —
    // does NOT cause a block; validation continues normally.
    let line_count = content.lines().count();
    if line_count > 200 {
        emit_over_200_line_advisory(line_count);
    }

    // F-A006: Parse YAML ONCE. Surface parse errors BEFORE the wave-1 no-op
    // short-circuit (EC-001). A malformed YAML payload must NEVER silently
    // return Continue via `unwrap_or(false)` fallbacks on a parse error.
    let parsed = match serde_norway::from_str::<serde_norway::Value>(content) {
        Ok(v) => v,
        Err(e) => {
            // SEC-001 (CWE-209): surface line/column only — not raw YAML token
            // content, which could leak sensitive HANDOFF.md field values via
            // `serde_norway::Error` Display output.
            // `serde_norway::Error::location()` returns `Option<Location>` with
            // `.line()` / `.column()` accessors (serde_norway 0.9.42 error.rs).
            let message = match e.location() {
                Some(loc) => format!(
                    "HandoffIncomplete: YAML parse error at line {}, column {}",
                    loc.line(),
                    loc.column()
                ),
                None => "HandoffIncomplete: YAML parse error (malformed YAML)".to_string(),
            };
            return GateResult::Block {
                code: "HandoffIncomplete",
                message,
            };
        }
    };

    // Step 2: EPIC-COMPLETE detection (from pre-parsed value, per INV1 / PC2a).
    // EPIC-COMPLETE = next_wave_stories: [] in the HANDOFF.md payload.
    // Step 2 precedes step 3 — even wave_id==1 goes through EPIC-COMPLETE path.
    let epic_complete = mapping_is_epic_complete(&parsed);

    if epic_complete {
        // Step 2 EPIC-COMPLETE branch: validate epic_status, THEN full base fields.
        return validate_epic_complete_handoff(&parsed);
    }

    // Step 3: wave_id == 1 AND NOT EPIC-COMPLETE → no-op (PC3 / INV3 step 3).
    //
    // F-A003 (BC-4.14.001 v1.16 / ADR-026 §Decision 9): This step returns
    // Continue WITHOUT inspecting epic_status. A wave_id:1 + non-EPIC-COMPLETE +
    // epic_status:present payload returns Continue here. The UnexpectedEpicStatus
    // check does NOT fire at step 3 — it is a step-4 check for wave_id>1 writes.
    //
    // F-A005: Wave-1 identity is derived from the parsed wave_id (not from an
    // external is_first_wave flag). `parsed_wave_id` returns `Some(Ok(1))` when
    // wave_id is the integer 1.
    let wave_id_val = parsed_wave_id(&parsed);
    if let Some(Ok(1)) = wave_id_val {
        return GateResult::Continue;
    }

    // Steps 4 & 5: wave_id > 1, wave_id malformed (0/negative/non-integer),
    // or wave_id absent → full validation.
    //
    // F-A003: UnexpectedEpicStatus is evaluated HERE (step 4 / INV3), not
    // between step 2 and step 3.
    //
    // F-A004: when wave_id is present-but-invalid (0, negative, non-integer)
    // and is the only failing field, the gate emits the exact BC-specified
    // message: "HandoffIncomplete: wave_id must be a positive integer"
    // (not the generic multi-field list format).
    validate_step4_or_5(&parsed)
}

// ---------------------------------------------------------------------------
// Step 4/5 validation (wave_id>1 or absent/malformed)
// ---------------------------------------------------------------------------

/// Run step 4/5 validation from a pre-parsed YAML value.
///
/// Called after step 3 short-circuit check determines wave_id != 1.
/// Covers:
///   - wave_id > 1 (step 4): full base-field validation + UnexpectedEpicStatus check.
///   - wave_id malformed (0/-1/non-int, step 5a): wave_id fails as malformed scalar.
///   - wave_id absent (step 5): full validation includes wave_id as missing.
fn validate_step4_or_5(parsed: &serde_norway::Value) -> GateResult {
    // F-A003: UnexpectedEpicStatus at step 4. epic_status must be absent on
    // non-final waves reaching this step (wave_id>1 or malformed/absent).
    if mapping_has_epic_status(parsed) {
        return GateResult::Block {
            code: "UnexpectedEpicStatus",
            message: "HandoffIncomplete: unexpected field epic_status on non-final wave"
                .to_string(),
        };
    }

    let failing = collect_failing_base_fields(parsed);

    if failing.is_empty() {
        return GateResult::Continue;
    }

    // F-A004 (BC-4.14.001 PC7 / EC-017): when wave_id is the sole failing field
    // AND it is present-but-invalid (not absent), emit the exact canonical message.
    if failing == ["wave_id"]
        && parsed
            .as_mapping()
            .map(|m| m.contains_key("wave_id"))
            .unwrap_or(false)
    {
        // wave_id key present but value not a positive integer (0, -1, non-int).
        return GateResult::Block {
            code: "HandoffIncomplete",
            message: "HandoffIncomplete: wave_id must be a positive integer".to_string(),
        };
    }

    GateResult::Block {
        code: "HandoffIncomplete",
        message: format!(
            "HandoffIncomplete: required fields missing or malformed: [{}]",
            failing.join(", ")
        ),
    }
}

// ---------------------------------------------------------------------------
// EPIC-COMPLETE branch (step 2)
// ---------------------------------------------------------------------------

/// Validate an EPIC-COMPLETE HANDOFF.md write (step 2).
///
/// Per BC-4.14.001 PC2a + ADR-026 §Decision 9:
/// 1. Validate `epic_status: complete`. If absent → MissingEpicStatus.
///    If present but value is non-string OR string != "complete" →
///    HandoffIncomplete: epic_status malformed.
/// 2. If epic_status valid → continue to full 9-base-field validation.
///    This augments (does NOT replace) base-field validation.
fn validate_epic_complete_handoff(parsed: &serde_norway::Value) -> GateResult {
    // Check epic_status key presence first (F-NEW-01 / BC-4.14.001 PC2a):
    // - Key ABSENT → MissingEpicStatus.
    // - Key PRESENT, value is non-string → HandoffIncomplete (epic_status malformed).
    //   mapping_epic_status_value returns None for both absent and non-string values,
    //   so we must gate on key presence to distinguish the two cases.
    // - Key PRESENT, value is string != "complete" → HandoffIncomplete (epic_status malformed).
    // - Key PRESENT, value is "complete" → valid; continue to base-field validation.
    let key_present = parsed
        .as_mapping()
        .map(|m| m.contains_key("epic_status"))
        .unwrap_or(false);

    if !key_present {
        return GateResult::Block {
            code: "MissingEpicStatus",
            message: "HandoffIncomplete: epic_status required on EPIC-COMPLETE wave (next_wave_stories: [])".to_string(),
        };
    }

    // Key is present; now check the value.
    match mapping_epic_status_value(parsed) {
        None => {
            // Key present but value is not a string (e.g., integer, list, bool).
            // Per BC-4.14.001 PC2a, this is "present but not complete" → malformed.
            return GateResult::Block {
                code: "HandoffIncomplete",
                message: "HandoffIncomplete: epic_status malformed — must be 'complete'"
                    .to_string(),
            };
        }
        Some(status) if status.trim() != "complete" => {
            return GateResult::Block {
                code: "HandoffIncomplete",
                message: "HandoffIncomplete: epic_status malformed — must be 'complete'"
                    .to_string(),
            };
        }
        Some(_) => {
            // epic_status is valid ("complete") — continue to full base-field validation
            // per ADR-026 §Decision 9 step 2.
        }
    }

    // Full 9-base-field validation (EPIC-COMPLETE augments, does not replace).
    let failing = collect_failing_base_fields(parsed);

    if failing.is_empty() {
        GateResult::Continue
    } else {
        GateResult::Block {
            code: "HandoffIncomplete",
            message: format!(
                "HandoffIncomplete: required fields missing or malformed: [{}]",
                failing.join(", ")
            ),
        }
    }
}

// ---------------------------------------------------------------------------
// WASM-facing gate function (PostToolUse dispatcher integration)
// ---------------------------------------------------------------------------

/// PostToolUse hook entry point: parse the dispatcher payload and invoke
/// the pure 5-step gate.
///
/// Extracts `file_path` from `payload.tool_input["file_path"]` (Write) or
/// `payload.tool_input["path"]` (Edit), then reads the **full** on-disk file
/// content via `host::read_file` to validate the complete HANDOFF.md (not just
/// the fragment carried in `new_string` for Edit calls).
///
/// ## Read-failure handling (VP-083 / BC-4.14.001 PC6)
///
/// If `host::read_file` returns an error (e.g., `CapabilityDenied` in non-WASM
/// test harnesses, permission failures, timeout), the gate FAILS OPEN and
/// returns `HookResult::Continue`. This is consistent with the sibling plugin
/// `validate-burst-log` and the VP-083 no-false-positive invariant.
///
/// # Fail-open on crash (BC-4.14.001 PC6)
///
/// The registry sets `on_error = "continue"` so any WASM panic causes the
/// dispatcher to fail open (Continue). This function itself must never panic
/// in non-test code — all error paths return `HookResult::Continue` or a
/// `HookResult::block_with_fix(...)`.
pub fn on_post_tool_use(payload: HookPayload) -> HookResult {
    use vsdd_hook_sdk::host;

    // Extract file_path: Write uses "file_path", Edit uses "path".
    let file_path = payload
        .tool_input
        .get("file_path")
        .and_then(|v| v.as_str())
        .or_else(|| payload.tool_input.get("path").and_then(|v| v.as_str()))
        .unwrap_or("")
        .to_string();

    // Short-circuit on non-HANDOFF.md path (path-component-strict guard).
    if !path_is_handoff(&file_path) {
        return HookResult::Continue;
    }

    // Read the FULL on-disk content via host::read_file.
    // This correctly handles Edit calls where tool_input["new_string"] is only
    // a fragment of the file. After the write completes, the on-disk file
    // contains the complete HANDOFF.md that we must validate.
    //
    // Fail-open on any read error (CapabilityDenied in unit harness,
    // permission failure, timeout) per VP-083 no-false-positive invariant and
    // BC-4.14.001 PC6.
    // Timeout budget: read_file gets 4000ms, leaving ~1000ms margin for YAML
    // parse within the 5000ms outer registry timeout (hooks-registry.toml
    // `timeout_ms = 5000`). Sibling validate-burst-log uses 2000ms read /
    // 3000ms parse split; this plugin uses 4000ms read to handle larger
    // HANDOFF.md payloads while still preserving a parse margin.
    let handoff_content = match host::read_file(&file_path, MAX_BYTES, 4000) {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(s) => Some(s),
            Err(e) => {
                host::log_warn(&format!(
                    "[validate-wave-handoff-completeness] UTF-8 decode failure reading \
                    {file_path}: {e} — failing open (Continue)"
                ));
                return HookResult::Continue;
            }
        },
        Err(e) => {
            host::log_warn(&format!(
                "[validate-wave-handoff-completeness] host::read_file failed for \
                {file_path}: {e:?} — failing open (Continue)"
            ));
            return HookResult::Continue;
        }
    };

    // F-A005: GateContext no longer carries is_first_wave — the pure core
    // derives wave identity from the parsed wave_id in handoff_content.
    let ctx = GateContext {
        file_path,
        handoff_content,
    };

    match check_handoff_completeness(&ctx) {
        GateResult::Continue => HookResult::Continue,
        GateResult::Block { code, message } => HookResult::block_with_fix(
            "validate-wave-handoff-completeness",
            &message,
            "Ensure all required HANDOFF.md fields are present and well-formed per ADR-026 §Decision 2",
            code,
        ),
    }
}

// ---------------------------------------------------------------------------
// YAML parse helpers (public; used by tests and by the pure core)
// ---------------------------------------------------------------------------

/// Parse the HANDOFF.md YAML string and extract the `wave_id` field as an
/// optional `i64`.
///
/// Returns `None` when the field is absent (triggers fail-closed path).
/// Returns `Some(n)` when `wave_id` is a valid integer.
/// Returns an error string when the YAML is malformed.
pub fn extract_wave_id(yaml_str: &str) -> Result<Option<i64>, String> {
    let value: serde_norway::Value = serde_norway::from_str(yaml_str).map_err(|e| e.to_string())?;

    let mapping = match value.as_mapping() {
        Some(m) => m,
        None => return Ok(None),
    };

    match mapping.get("wave_id") {
        None => Ok(None),
        Some(v) => match v.as_i64() {
            Some(n) => Ok(Some(n)),
            None => Err(format!("wave_id field is not an integer: {v:?}")),
        },
    }
}

/// Determine whether the HANDOFF.md payload is in EPIC-COMPLETE context.
///
/// EPIC-COMPLETE context is defined PAYLOAD-ONLY: `next_wave_stories: []`
/// (empty list) in the parsed YAML. Non-empty `next_wave_stories` or absent
/// `next_wave_stories` → NOT EPIC-COMPLETE. (BC-4.14.001 PC2a)
pub fn is_epic_complete(yaml_str: &str) -> Result<bool, String> {
    let value: serde_norway::Value = serde_norway::from_str(yaml_str).map_err(|e| e.to_string())?;

    let mapping = match value.as_mapping() {
        Some(m) => m,
        None => return Ok(false),
    };

    match mapping.get("next_wave_stories") {
        None => Ok(false),
        Some(v) => {
            // Empty sequence → EPIC-COMPLETE; non-empty → NOT EPIC-COMPLETE.
            Ok(v.as_sequence().map(|s| s.is_empty()).unwrap_or(false))
        }
    }
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
///   (`precompact_flush_sha`, `factory_lock_holder`). `wave_id` must be a
///   positive integer (>= 1) per BC-4.14.001 PC7 / EC-017.
/// - LIST fields (`active_bcs`, `next_wave_stories`, `open_decisions`,
///   `pending_fixes`, `process_gaps`): key must exist; value must be a
///   syntactically-valid list. Empty list (`[]`) is VALID (NOT malformed).
///
/// Note (F-A005): the legacy `_epic_complete` parameter has been removed.
/// `epic_status` validation is handled upstream in `validate_epic_complete_handoff`
/// before this function is called, so no conditional behaviour is needed here.
pub fn validate_base_fields(yaml_str: &str) -> Result<Vec<String>, String> {
    let value: serde_norway::Value = serde_norway::from_str(yaml_str).map_err(|e| e.to_string())?;

    let mapping = match value.as_mapping() {
        Some(m) => m,
        None => {
            return Ok(BASE_FIELDS_ORDERED.iter().map(|f| f.to_string()).collect());
        }
    };

    let mut failing = Vec::new();

    for &field in BASE_FIELDS_ORDERED {
        if !validate_field(field, mapping) {
            failing.push(field.to_string());
        }
    }

    Ok(failing)
}

// ---------------------------------------------------------------------------
// Internal helpers operating on pre-parsed serde_norway::Value
// (avoids repeated parses; WASM fuel-budget conscious)
// ---------------------------------------------------------------------------

/// Determine EPIC-COMPLETE from a pre-parsed value.
///
/// `next_wave_stories: []` (empty sequence) → EPIC-COMPLETE.
fn mapping_is_epic_complete(value: &serde_norway::Value) -> bool {
    let Some(mapping) = value.as_mapping() else {
        return false;
    };
    match mapping.get("next_wave_stories") {
        None => false,
        Some(v) => v.as_sequence().map(|s| s.is_empty()).unwrap_or(false),
    }
}

/// Extract `wave_id` from a pre-parsed value.
///
/// Returns `Some(Ok(n))` for a valid integer, `Some(Err(_))` for a present
/// but non-integer value, and `None` when the key is absent.
fn parsed_wave_id(value: &serde_norway::Value) -> Option<Result<i64, String>> {
    let mapping = value.as_mapping()?;
    let v = mapping.get("wave_id")?;
    match v.as_i64() {
        Some(n) => Some(Ok(n)),
        None => Some(Err(format!("wave_id field is not an integer: {v:?}"))),
    }
}

/// Check whether `epic_status` key is present in the pre-parsed YAML mapping.
///
/// Used at step 4 (wave_id>1 path) to detect `UnexpectedEpicStatus` per
/// BC-4.14.001 INV3 / F-A003 adjudication (v1.16). NOT called at step 3.
fn mapping_has_epic_status(value: &serde_norway::Value) -> bool {
    value
        .as_mapping()
        .map(|m| m.contains_key("epic_status"))
        .unwrap_or(false)
}

/// Get the string value of `epic_status` from the pre-parsed YAML mapping.
///
/// Returns `None` if the key is absent or the value is not a string.
fn mapping_epic_status_value(value: &serde_norway::Value) -> Option<String> {
    let mapping = value.as_mapping()?;
    let v = mapping.get("epic_status")?;
    v.as_str().map(|s| s.to_string())
}

/// Collect all failing base field names from a pre-parsed value.
///
/// Returns field names in deterministic ADR-026 §D2 schema order.
fn collect_failing_base_fields(value: &serde_norway::Value) -> Vec<String> {
    let mapping = match value.as_mapping() {
        Some(m) => m,
        None => {
            return BASE_FIELDS_ORDERED.iter().map(|f| f.to_string()).collect();
        }
    };

    let mut failing = Vec::new();
    for &field in BASE_FIELDS_ORDERED {
        if !validate_field(field, mapping) {
            failing.push(field.to_string());
        }
    }
    failing
}

/// Validate a single field in a YAML mapping.
///
/// Returns `true` if the field is present and syntactically valid.
/// Returns `false` if the field is missing or malformed.
fn validate_field(field: &str, mapping: &serde_norway::Mapping) -> bool {
    let value = match mapping.get(field) {
        Some(v) => v,
        None => return false, // field absent
    };

    if LIST_FIELDS.contains(&field) {
        // List fields: value must be a sequence. Empty sequence is valid.
        value.as_sequence().is_some()
    } else if NULLABLE_SCALAR_FIELDS.contains(&field) {
        // Nullable scalar: null is valid; non-empty string is valid;
        // empty string is malformed.
        if value.is_null() {
            true
        } else if let Some(s) = value.as_str() {
            !s.is_empty()
        } else {
            false
        }
    } else {
        // Non-nullable scalar (wave_id, last_verified_develop_sha):
        // - wave_id: positive integer (>= 1); 0 and negatives are malformed per
        //   BC-4.14.001 PC7 / EC-017.
        // - last_verified_develop_sha: non-empty string.
        if field == "wave_id" {
            value.as_i64().map(|n| n >= 1).unwrap_or(false)
        } else {
            value.as_str().map(|s| !s.is_empty()).unwrap_or(false)
        }
    }
}

/// Check whether the target file path's file-name component is exactly `HANDOFF.md`.
///
/// Uses path-component-strict matching (`std::path::Path::file_name()`) rather
/// than `ends_with`, preventing false-positive fires on paths like
/// `foo/WAVE-HANDOFF.md`, `xHANDOFF.md`, or `foo/MY-HANDOFF.md` where
/// `ends_with("HANDOFF.md")` would also return `true`.
///
/// Returns `true` iff the final path component is exactly `HANDOFF.md`
/// (case-sensitive per BC-4.14.001 PC4). No filesystem access is performed.
///
/// Returns `false` if the path has no file-name component (e.g., `/`).
///
/// # BC trace
/// BC-4.14.001 PC4 — hook only activates on HANDOFF.md writes.
/// Mirrors the pattern used by sibling `validate-burst-log::is_burst_log_target`.
pub fn path_is_handoff(file_path: &str) -> bool {
    std::path::Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        == Some("HANDOFF.md")
}

/// Emit a 200-line advisory warning via the host log.
///
/// Called when `handoff_content` line count exceeds 200 (BC-4.14.001 PC5 /
/// INV5). The warning is advisory only — the gate continues parsing and
/// validating all fields normally. This function does NOT cause a block.
pub fn emit_over_200_line_advisory(line_count: usize) {
    vsdd_hook_sdk::host::log_warn(&format!(
        "validate-wave-handoff-completeness: HANDOFF.md body exceeds 200-line advisory cap \
        (line_count={line_count}). The gate continues validating all fields normally. \
        Consider compacting HANDOFF.md to stay within the 200-line budget (ADR-026 Decision 8)."
    ));
}
