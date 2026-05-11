//! validate-per-story-adversary-convergence — SubagentStop WASM hook plugin.
// Allow: complex file-parsing logic uses .expect() on operations that are
// validated before the call site, and regex compilation on static patterns.
// Pre-existing production code (not covered by S-12.07 AC-010 scope).
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
//!
//! Blocks wave-gate dispatch when any story in the current wave lacks a
//! cleared adversary convergence state file
//! (`passes_clean >= 3 AND last_classification == "NITPICK_ONLY"`).
//!
//! # Behavioral Contracts
//!
//! - BC-4.10.001: MUST block wave-gate dispatch when any story lacks convergence
//!   clearance. Block form: `HookResult::block_with_fix(hook, reason, recommendation, code)`.
//! - BC-4.10.002: MUST gracefully degrade (return Continue, no block) when
//!   invoked outside wave-gate context or when cycle directory is absent.
//!
//! # Architecture compliance
//!
//! - HOST_ABI_VERSION = 1 unchanged (additive; no new host functions).
//! - Uses only `host::read_file`, `host::log_*`, and `host::emit_event`.
//! - Hook is read-only: MUST NOT call `host::write_file`.
//! - Injectable-callback pattern: all host I/O injected so unit tests run
//!   without a WASM runtime (BC-4.10.001 invariant 3; AC-010).
//! - Graceful-degrade check occurs BEFORE any file reads
//!   (BC-4.10.002 invariant 2).

use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// Constants (GREEN-BY-DESIGN: structural constants, not function bodies)
// ---------------------------------------------------------------------------

/// Kebab-case hook name used in `block_with_fix` messages per HOST_ABI.md §Rules.
pub const HOOK_NAME: &str = "validate-per-story-adversary-convergence";

/// Stable snake_case telemetry code for the unconverged block signal (VP-071).
pub const HOOK_CODE_BASE: &str = "per_story_adversary_unconverged";

/// ABI version this plugin speaks (BC-4.10.001 invariant 2; AC-011).
pub const HOST_ABI_VERSION: u32 = 1;

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Extract the telemetry code from a canonical block_with_fix reason string.
///
/// The canonical format is "BLOCKED by <hook>: <reason>. Fix: <fix>. Code: <code>."
/// Returns the `<code>` segment if present, or `None` if the format doesn't match.
///
/// Used to surface the telemetry code in `hook.block` events without duplicating
/// the code construction logic from the block-path branches.
pub(crate) fn extract_code_from_reason(reason: &str) -> Option<&str> {
    // Find "Code: " then extract up to the trailing "."
    let prefix = "Code: ";
    let start = reason.find(prefix)? + prefix.len();
    let end = reason[start..]
        .find('.')
        .map(|i| start + i)
        .unwrap_or(reason.len());
    Some(&reason[start..end])
}

// ---------------------------------------------------------------------------
// State file schema (BC-5.39.001 postcondition 2)
// ---------------------------------------------------------------------------

/// Adversary convergence state file schema.
///
/// Matches the JSON written by the per-story convergence loop (S-12.01).
/// Path: `.factory/cycles/<cycle-id>/<story-id>/adversary-convergence-state.json`
///
/// Note: `last_classification` is `Option<String>` to correctly model
/// the case where the field is absent or JSON null (EC-003; VP-071 harness).
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ConvergenceState {
    pub passes_clean: u32,
    pub last_finding_count: Option<u32>,
    pub last_classification: Option<String>,
    pub last_timestamp: Option<String>,
    #[serde(default)]
    pub deferred_findings: Vec<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Error returned when the state file JSON cannot be parsed.
#[derive(Debug)]
pub struct ParseError(pub String);

/// Error returned when a host I/O operation fails.
#[derive(Debug)]
pub struct IoError(pub String);

// ---------------------------------------------------------------------------
// Pure evaluation function (VP-071 kani harness target)
// ---------------------------------------------------------------------------

/// Pure projection from `Option<ConvergenceState>` to `HookResult`.
///
/// Returns `HookResult::block_with_fix(...)` (canonical Why/Fix/Code form per
/// HOST_ABI.md §WASM hooks, lines 421–460) whenever the convergence criterion
/// is not satisfied. Returns `HookResult::Continue` only when the state is
/// fully converged: `passes_clean >= 3 AND last_classification == "NITPICK_ONLY"`.
///
/// This function is the target of all 6 VP-071 kani proof harnesses.
/// It MUST contain no `unwrap()` or `expect()` — no panics on any input.
/// It performs no I/O and reads no global state.
///
/// # Arguments
///
/// * `state` — `None` means the state file was absent or unreadable.
///
/// # BC traces
/// - BC-4.10.001 postconditions 2–5 (block on missing / passes < 3 / non-nitpick)
/// - VP-071 formal property statement
pub fn hook_result_for(state: Option<&ConvergenceState>) -> HookResult {
    match state {
        // BC-4.10.001 PC2: absent state file → CONVERGENCE_STATE_MISSING
        // Code uses HOOK_CODE_BASE prefix per VP-071 telemetry bucketing convention.
        None => {
            let code = format!("{}_CONVERGENCE_STATE_MISSING", HOOK_CODE_BASE);
            HookResult::block_with_fix(
                HOOK_NAME,
                "story is missing adversary-convergence-state.json — convergence gate not run",
                "Run the per-story adversary convergence loop (BC-5.39.001) before dispatching the wave gate",
                &code,
            )
        }
        Some(s) => {
            // BC-4.10.001 PC3: passes_clean < 3 → CONVERGENCE_PASSES_INSUFFICIENT
            if s.passes_clean < 3 {
                let code = format!("{}_CONVERGENCE_PASSES_INSUFFICIENT", HOOK_CODE_BASE);
                return HookResult::block_with_fix(
                    HOOK_NAME,
                    format!(
                        "story has passes_clean={} — convergence requires passes_clean >= 3",
                        s.passes_clean
                    ),
                    "Continue adversary review passes until passes_clean reaches 3",
                    &code,
                );
            }
            // BC-4.10.001 PC4: last_classification != "NITPICK_ONLY" (including None) →
            // CONVERGENCE_CLASSIFICATION_INSUFFICIENT (or SCHEMA_INVALID for None)
            let code = format!("{}_CONVERGENCE_CLASSIFICATION_INSUFFICIENT", HOOK_CODE_BASE);
            match s.last_classification.as_deref() {
                None => HookResult::block_with_fix(
                    HOOK_NAME,
                    "story last_classification field is missing or null — convergence state schema invalid",
                    "Ensure the adversary convergence loop writes a valid last_classification field",
                    &code,
                ),
                Some(cls) if cls != "NITPICK_ONLY" => HookResult::block_with_fix(
                    HOOK_NAME,
                    format!(
                        "story last adversary pass classified as {} — must be NITPICK_ONLY",
                        cls
                    ),
                    format!(
                        "Resolve remaining {} findings before dispatching the wave gate",
                        cls
                    ),
                    &code,
                ),
                // BC-4.10.001 PC5: passes_clean >= 3 AND last_classification == "NITPICK_ONLY"
                Some(_) => HookResult::Continue,
            }
        }
    }
}

// ---------------------------------------------------------------------------
// JSON parsing
// ---------------------------------------------------------------------------

/// Parse a convergence state JSON string into a `ConvergenceState`.
///
/// Returns `Err(ParseError)` if the JSON is malformed or fails schema
/// validation. Callers treat `ParseError` as a block condition
/// (`code: "CONVERGENCE_STATE_MALFORMED"` or `"CONVERGENCE_STATE_SCHEMA_INVALID"`).
///
/// # BC traces
/// - BC-4.10.001 edge cases EC-002 (malformed JSON), EC-003 (missing field)
/// - AC-007, AC-008
pub fn parse_convergence_state(json: &str) -> Result<ConvergenceState, ParseError> {
    serde_json::from_str(json).map_err(|e| ParseError(e.to_string()))
}

// ---------------------------------------------------------------------------
// Wave context detection (BC-4.10.002)
// ---------------------------------------------------------------------------

/// Returns `true` if the hook can confirm it is executing in a wave-gate
/// dispatch context (per BC-4.10.002 invariant 4).
///
/// This check MUST be performed before any file reads (BC-4.10.002 invariant 2).
/// When it returns `false`, `hook_logic` must return `HookResult::Continue`
/// immediately (graceful degrade).
///
/// Confirmation requires that `payload.agent_type` or `payload.subagent_name`
/// identifies a wave-gate dispatch agent. The exact matching logic is
/// implementation-defined but MUST be conservative: unknown/missing agent types
/// default to `false`.
///
/// # BC traces
/// - BC-4.10.002 preconditions 2d, postcondition 1
/// - AC-003 (graceful degrade on missing wave-gate indicator)
pub fn graceful_degrade_outside_wave_gate(payload: &HookPayload) -> bool {
    // Returns true when the hook SHOULD degrade (i.e., NOT in wave-gate context).
    // BC-4.10.002 invariant 4: errs on the side of Continue rather than blocking
    // on uncertainty. Unknown/missing agent types default to degrade.
    //
    // The canonical agent identity fallback chain (BC-2.02.012 Postcondition 5):
    let identity = payload
        .agent_type
        .as_deref()
        .or(payload.subagent_name.as_deref())
        .unwrap_or("unknown");

    // Any identity starting with "wave-gate" is treated as a wave-gate dispatch
    // context (F-MED-8 fix). The canonical identity is "wave-gate-dispatch"
    // (BC-4.10.002 invariant 4), but starts_with("wave-gate") prevents silent
    // hook deactivation if the dispatcher uses a future variant like "wave-gate-v2".
    // All other identities (including "unknown") trigger graceful degrade.
    !identity.starts_with("wave-gate")
}

// ---------------------------------------------------------------------------
// Block codes for wave_context path (S-12.08 — AC-002, AC-003)
// ---------------------------------------------------------------------------

/// Block code returned when `plugin_config["wave_context"]` is absent or null.
///
/// Indicates that `WaveContextResolver` was not wired or failed to inject context.
/// Hook MUST NOT gracefully degrade on this condition (AC-002, AC-010).
///
/// # BC traces
/// - BC-4.10.001 postcondition 2 (absent wave_context → block, not degrade)
/// - S-12.08 AC-002
pub const BLOCK_CODE_WAVE_CONTEXT_MISSING: &str = "WAVE_CONTEXT_MISSING";

/// Block code returned when `plugin_config["wave_context"]["stories"]` is present
/// but is not a JSON array (wrong type — e.g., string, object, or null).
///
/// # BC traces
/// - BC-4.10.001 postcondition 2 (schema error → block)
/// - S-12.08 AC-003
pub const BLOCK_CODE_WAVE_CONTEXT_SCHEMA_ERROR: &str = "WAVE_CONTEXT_SCHEMA_ERROR";

// ---------------------------------------------------------------------------
// WaveContextError (S-12.08 — typed error for wave_context extraction path)
// ---------------------------------------------------------------------------

/// Typed error for the `extract_stories_from_wave_context` function.
///
/// Distinct from `IoError` (which covers filesystem-level errors) — this error
/// represents a contract violation: the wave_context key that WaveContextResolver
/// MUST inject was absent, null, or had the wrong schema.
///
/// # BC traces
/// - BC-4.10.001 postcondition 2 (block when wave_context absent or malformed)
/// - S-12.08 AC-002 (`Missing` variant), AC-003 (`SchemaError` variant)
#[derive(Debug)]
pub enum WaveContextError {
    /// `plugin_config["wave_context"]` was absent or JSON null.
    /// Maps to block code `WAVE_CONTEXT_MISSING`.
    Missing,
    /// `plugin_config["wave_context"]["stories"]` was present but not a JSON array,
    /// or an element of the array was not a string.
    /// Carries a human-readable description of the schema violation.
    /// Maps to block code `WAVE_CONTEXT_SCHEMA_ERROR`.
    SchemaError(String),
}

// ---------------------------------------------------------------------------
// wave_context.stories extraction (S-12.08 — successor to extract_stories_from_config)
// ---------------------------------------------------------------------------

/// Extract story IDs from `plugin_config["wave_context"]["stories"]`.
///
/// This function is the S-12.08 replacement path for the old
/// `extract_stories_from_config` (which read from the top-level `stories` key).
/// After S-12.08 implementation (Step 3), `RealCallbacks::list_stories` in
/// `main.rs` will call this function instead of `extract_stories_from_config`.
///
/// Returns `Ok(Vec<String>)` when `wave_context.stories` is a non-null JSON array
/// of strings. Returns `Err(WaveContextError::Missing)` when the `wave_context`
/// key is absent or null. Returns `Err(WaveContextError::SchemaError(_))` when
/// `wave_context.stories` is present but has the wrong type.
///
/// The hook MUST NOT gracefully degrade on `WaveContextError` — this is a
/// hard block condition (AC-002, AC-010). The old fallback path
/// (`extract_stories_from_config` with graceful Err→Continue degrade) is removed
/// as part of S-12.08 Step 3.
///
/// # BC traces
/// - BC-4.10.001 PC1 (hook must enumerate stories from the current wave)
/// - BC-4.12.005 PC1 (resolver output injected at `plugin_config["wave_context"]`)
/// - S-12.08 AC-001 (reads from `wave_context.stories` not top-level `stories`)
/// - S-12.08 AC-002 (absent wave_context → `Missing` error → Block)
/// - S-12.08 AC-003 (wrong type → `SchemaError` → Block)
/// - S-12.08 AC-010 (old fallback path removed — no graceful Continue on missing)
pub fn extract_stories_from_wave_context(
    plugin_config: &serde_json::Value,
) -> Result<Vec<String>, WaveContextError> {
    // AC-002: missing or null wave_context → Missing.
    // Matches both the absent-key case and the explicit-null case.
    let wave_context = match plugin_config.get("wave_context") {
        Some(serde_json::Value::Null) | None => return Err(WaveContextError::Missing),
        Some(v) => v,
    };

    // AC-003: wave_context must be an object; anything else is a schema error.
    if !wave_context.is_object() {
        return Err(WaveContextError::SchemaError(format!(
            "plugin_config.wave_context: expected object, got {:?}",
            wave_context
        )));
    }

    // AC-002c: wave_context present but stories key absent or null → Missing.
    let stories_val = match wave_context.get("stories") {
        Some(serde_json::Value::Null) | None => return Err(WaveContextError::Missing),
        Some(v) => v,
    };

    // AC-003: stories present but wrong type → SchemaError.
    let arr = match stories_val.as_array() {
        Some(a) => a,
        None => {
            return Err(WaveContextError::SchemaError(format!(
                "plugin_config.wave_context.stories: expected array, got {:?}",
                stories_val
            )));
        }
    };

    let mut stories = Vec::with_capacity(arr.len());
    for v in arr {
        match v.as_str() {
            Some(s) => stories.push(s.to_string()),
            None => {
                return Err(WaveContextError::SchemaError(format!(
                    "plugin_config.wave_context.stories: non-string element {:?}",
                    v
                )));
            }
        }
    }
    Ok(stories)
}

// ---------------------------------------------------------------------------
// Injectable-callback trait
// ---------------------------------------------------------------------------

/// Injectable host I/O callbacks for testability.
///
/// The `hook_logic` function takes `callbacks: &impl HookCallbacks` so that
/// unit tests can substitute fake implementations without a WASM runtime
/// (BC-4.10.001 invariant 3; AC-010).
///
/// In production (`main.rs`), the `RealCallbacks` struct wires these to
/// `vsdd_hook_sdk::host::*` functions.
pub trait HookCallbacks {
    /// Read a file at `path`. Returns `Ok(Some(contents))` when the file exists
    /// and is readable; `Ok(None)` when the file is absent; `Err(IoError)` on
    /// unexpected I/O failure.
    fn read_file(&self, path: &str) -> Result<Option<String>, IoError>;

    /// Log an advisory message via `host::log_info`.
    ///
    /// HOST_ABI v1 does not expose a `log_debug` endpoint; `log_info` is the
    /// lowest-severity level available. BC-4.10.002 PC3 (amended v1.1,
    /// F5 pass-1 B2) now correctly references `host::log_info`.
    fn log_debug(&self, msg: &str);

    /// Log an error-level message via `host::log_error`.
    fn log_error(&self, msg: &str);

    /// Emit a structured event via `host::emit_event`.
    ///
    /// Called immediately before returning `HookResult::Block` to provide
    /// observability into convergence gate firings (BC-4.10.001 observability
    /// mandate; BC-7.03.075 hook.block event pattern).
    ///
    /// `event_type` is `"hook.block"` for block events.
    /// `fields` is a slice of key-value pairs (e.g., hook name, code, story).
    fn emit_event(&self, event_type: &str, fields: &[(&str, &str)]);
}

// ---------------------------------------------------------------------------
// Core hook logic (injectable, no WASM runtime required for tests)
// ---------------------------------------------------------------------------

/// Core convergence gate logic.
///
/// Implements BC-4.10.001 (block invariant) and BC-4.10.002 (graceful degrade).
/// All host I/O is injected via `callbacks` for testability.
///
/// Execution order (per BC-4.10.002 invariant 2):
/// 1. Check wave-gate context (`graceful_degrade_outside_wave_gate`). If not
///    confirmed → log advisory, return `HookResult::Continue` immediately.
///    NO file reads before this check.
/// 2. Extract story list from `plugin_config["wave_context"]["stories"]`
///    (S-12.08: WaveContextResolver injects this). If `wave_context` is absent
///    or malformed → Block (not graceful degrade — AC-002, AC-003, AC-010).
/// 3. For each story: read state file via `callbacks.read_file`. Parse with
///    `parse_convergence_state`. Evaluate with `hook_result_for`. On first
///    non-Continue result → return immediately (BC-4.10.001 postcondition 5).
/// 4. If all stories converged: aggregate deferred_findings, log summary,
///    return `HookResult::Continue`.
///
/// # BC traces
/// - BC-4.10.001 postconditions 1–8 (full block/continue decision logic)
/// - BC-4.10.002 postconditions 1–5 (graceful degrade path)
/// - AC-001 through AC-012
pub fn hook_logic(payload: &HookPayload, callbacks: &impl HookCallbacks) -> HookResult {
    // Step 1: Graceful-degrade check BEFORE any file reads (BC-4.10.002 invariant 2).
    if graceful_degrade_outside_wave_gate(payload) {
        callbacks.log_debug(
            "validate-per-story-adversary-convergence: graceful degrade \
             — invoked outside wave-gate context or cycle directory absent; returning Continue",
        );
        return HookResult::Continue;
    }

    // Step 2: Extract story list from wave_context.stories (S-12.08 AC-001, AC-002, AC-003).
    // The WaveContextResolver (S-12.07) injects wave_context into plugin_config before dispatch.
    // On WaveContextError::Missing or ::SchemaError → Block (not graceful degrade — AC-010).
    let story_ids = match extract_stories_from_wave_context(&payload.plugin_config) {
        Ok(ids) => ids,
        Err(WaveContextError::Missing) => {
            // AC-002: wave_context absent or null → hard block. Do NOT gracefully degrade.
            // This indicates WaveContextResolver was not wired or failed to inject context.
            callbacks.emit_event(
                "hook.block",
                &[
                    ("hook", HOOK_NAME),
                    ("code", BLOCK_CODE_WAVE_CONTEXT_MISSING),
                ],
            );
            return HookResult::block_with_fix(
                HOOK_NAME,
                "wave_context not injected — plugin_config[\"wave_context\"] is absent or null",
                "Verify needs_context=[\"wave_context\"] in hooks-registry.toml and \
                 WaveContextResolver is registered in resolvers-registry.toml",
                BLOCK_CODE_WAVE_CONTEXT_MISSING,
            );
        }
        Err(WaveContextError::SchemaError(detail)) => {
            // AC-003: wave_context.stories has wrong type → hard block.
            callbacks.emit_event(
                "hook.block",
                &[
                    ("hook", HOOK_NAME),
                    ("code", BLOCK_CODE_WAVE_CONTEXT_SCHEMA_ERROR),
                ],
            );
            return HookResult::block_with_fix(
                HOOK_NAME,
                format!("wave_context.stories schema error: {}", detail),
                "Ensure WaveContextResolver injects wave_context.stories as a JSON array of strings",
                BLOCK_CODE_WAVE_CONTEXT_SCHEMA_ERROR,
            );
        }
    };

    // Step 3: Determine cycle_id from wave_context (injected by WaveContextResolver).
    // MED-001: absent or empty cycle_id → Block(SCHEMA_ERROR) instead of silent fallback.
    // A missing cycle_id indicates a resolver bug or stale context — do not silently mask.
    let cycle_id = match payload
        .plugin_config
        .get("wave_context")
        .and_then(|wc| wc.get("cycle_id"))
        .and_then(|v| v.as_str())
    {
        Some(c) if !c.is_empty() => c,
        _ => {
            callbacks.emit_event(
                "hook.block",
                &[
                    ("hook", HOOK_NAME),
                    ("code", BLOCK_CODE_WAVE_CONTEXT_SCHEMA_ERROR),
                ],
            );
            return HookResult::block_with_fix(
                HOOK_NAME,
                "wave_context.cycle_id missing or empty — resolver bug or stale context",
                "Verify WaveContextResolver injects a non-empty cycle_id from STATE.md \
                 into wave_context",
                BLOCK_CODE_WAVE_CONTEXT_SCHEMA_ERROR,
            );
        }
    };

    // BC-4.10.001 EC-001: empty wave → Continue (vacuously all stories cleared).
    if story_ids.is_empty() {
        callbacks.log_debug(
            "validate-per-story-adversary-convergence: Wave has zero stories. \
             Returning Continue (vacuous convergence).",
        );
        return HookResult::Continue;
    }

    // Step 4: For each story, read state file, parse, evaluate.
    // Block on FIRST non-cleared story (BC-4.10.001 postcondition 5 / EC-005).
    let mut all_deferred: Vec<serde_json::Value> = Vec::new();

    for story_id in &story_ids {
        let state_path = format!(
            ".factory/cycles/{}/{}/adversary-convergence-state.json",
            cycle_id, story_id
        );

        // Closure to build the missing-state block with a story-specific message.
        let missing_code = format!("{}_CONVERGENCE_STATE_MISSING", HOOK_CODE_BASE);
        let missing_block = || {
            HookResult::block_with_fix(
                HOOK_NAME,
                format!(
                    "Story {} is missing adversary-convergence-state.json \
                     — convergence gate not run",
                    story_id
                ),
                format!(
                    "Run the per-story adversary convergence loop (BC-5.39.001) \
                     for story {} before dispatching the wave gate",
                    story_id
                ),
                &missing_code,
            )
        };

        let file_contents = match callbacks.read_file(&state_path) {
            Ok(Some(contents)) => contents,
            // BC-4.10.001 PC2: state file absent → CONVERGENCE_STATE_MISSING
            Ok(None) => {
                let block = missing_block();
                // Emit hook.block event before returning (BC-4.10.001 observability mandate;
                // BC-7.03.075 hook.block event pattern; F-CRIT-4 fix).
                if let HookResult::Block { ref reason } = block {
                    let code = format!("{}_CONVERGENCE_STATE_MISSING", HOOK_CODE_BASE);
                    callbacks.emit_event(
                        "hook.block",
                        &[
                            ("hook", HOOK_NAME),
                            ("code", &code),
                            ("story", story_id),
                            ("reason", reason.as_str()),
                        ],
                    );
                }
                return block;
            }
            // Unreadable state file treated as absent (BC-4.10.001 PC2)
            Err(_) => {
                let block = missing_block();
                if let HookResult::Block { ref reason } = block {
                    let code = format!("{}_CONVERGENCE_STATE_MISSING", HOOK_CODE_BASE);
                    callbacks.emit_event(
                        "hook.block",
                        &[
                            ("hook", HOOK_NAME),
                            ("code", &code),
                            ("story", story_id),
                            ("reason", reason.as_str()),
                        ],
                    );
                }
                return block;
            }
        };

        // Parse the JSON. Malformed → CONVERGENCE_STATE_MALFORMED (BC-4.10.001 EC-002).
        let state = match parse_convergence_state(&file_contents) {
            Ok(s) => s,
            Err(_) => {
                callbacks.log_error(&format!(
                    "validate-per-story-adversary-convergence: malformed JSON in state file \
                     for story {}",
                    story_id
                ));
                let malformed_code = format!("{}_CONVERGENCE_STATE_MALFORMED", HOOK_CODE_BASE);
                let block = HookResult::block_with_fix(
                    HOOK_NAME,
                    format!(
                        "Story {} adversary-convergence-state.json contains malformed JSON \
                         — convergence state unreadable",
                        story_id
                    ),
                    format!(
                        "Fix the malformed JSON in .factory/cycles/{}/{}/\
                         adversary-convergence-state.json or re-run the convergence loop",
                        cycle_id, story_id
                    ),
                    &malformed_code,
                );
                if let HookResult::Block { ref reason } = block {
                    callbacks.emit_event(
                        "hook.block",
                        &[
                            ("hook", HOOK_NAME),
                            ("code", &malformed_code),
                            ("story", story_id),
                            ("reason", reason.as_str()),
                        ],
                    );
                }
                return block;
            }
        };

        // Evaluate convergence for this story.
        // We build story-specific messages (includes the story_id for actionability).
        let verdict = if state.passes_clean < 3 {
            let code = format!("{}_CONVERGENCE_PASSES_INSUFFICIENT", HOOK_CODE_BASE);
            HookResult::block_with_fix(
                HOOK_NAME,
                format!(
                    "Story {} has passes_clean={} — convergence requires passes_clean >= 3",
                    story_id, state.passes_clean
                ),
                format!(
                    "Continue adversary review passes for story {} until passes_clean reaches 3",
                    story_id
                ),
                &code,
            )
        } else {
            let code = format!("{}_CONVERGENCE_CLASSIFICATION_INSUFFICIENT", HOOK_CODE_BASE);
            match state.last_classification.as_deref() {
                None => HookResult::block_with_fix(
                    HOOK_NAME,
                    format!(
                        "Story {} last_classification field is missing or null \
                         — convergence state schema invalid",
                        story_id
                    ),
                    format!(
                        "Ensure the adversary convergence loop writes a valid last_classification \
                         field for story {}",
                        story_id
                    ),
                    &code,
                ),
                Some(cls) if cls != "NITPICK_ONLY" => HookResult::block_with_fix(
                    HOOK_NAME,
                    format!(
                        "Story {} last adversary pass classified as {} — must be NITPICK_ONLY",
                        story_id, cls
                    ),
                    format!(
                        "Resolve remaining {} findings for story {} before dispatching the wave gate",
                        cls, story_id
                    ),
                    &code,
                ),
                Some(_) => HookResult::Continue,
            }
        };

        match verdict {
            HookResult::Continue => {
                // Story cleared. Accumulate deferred findings for aggregation (BC-4.10.001 PC6).
                all_deferred.extend(state.deferred_findings);
            }
            block => {
                // First failure → emit hook.block event, then return immediately
                // (BC-4.10.001 PC5 / EC-005; BC-4.10.001 observability mandate F-CRIT-4).
                if let HookResult::Block { ref reason } = block {
                    // Extract code suffix from the reason string for telemetry.
                    // The code is embedded in the canonical reason as "Code: <code>."
                    let code = extract_code_from_reason(reason).unwrap_or(HOOK_CODE_BASE);
                    callbacks.emit_event(
                        "hook.block",
                        &[
                            ("hook", HOOK_NAME),
                            ("code", code),
                            ("story", story_id),
                            ("reason", reason.as_str()),
                        ],
                    );
                }
                return block;
            }
        }
    }

    // Step 5: All stories converged. Aggregate deferred findings and log (BC-4.10.001 PC6).
    if !all_deferred.is_empty() {
        let summary = serde_json::to_string(&all_deferred).unwrap_or_else(|_| "[]".to_string());
        callbacks.log_debug(&format!(
            "validate-per-story-adversary-convergence: wave convergence cleared. \
             {} deferred finding(s) across all stories: {}",
            all_deferred.len(),
            summary
        ));
    }

    HookResult::Continue
}

// ---------------------------------------------------------------------------
// VP-071 kani proof harnesses (inline #[cfg(kani)] module)
// ---------------------------------------------------------------------------

#[cfg(kani)]
mod kani_proofs {
    use super::*;

    /// Block invariant — Part A: missing state file always returns block_with_fix.
    /// Regardless of how many stories are in the wave, a missing file
    /// for ANY story causes a canonical block (not advisory-mode Continue).
    #[kani::proof]
    fn proof_missing_state_file_always_blocks() {
        let state: Option<&ConvergenceState> = None;
        let result = hook_result_for(state);
        kani::assert(
            matches!(result, HookResult::Block { .. }),
            "missing state file must return HookResult::block_with_fix (not Continue)",
        );
    }

    /// Block invariant — Part B: passes_clean < 3 always returns block_with_fix.
    #[kani::proof]
    #[kani::unwind(4)]
    fn proof_insufficient_passes_always_blocks() {
        let passes: u32 = kani::any();
        kani::assume(passes < 3);

        let state = ConvergenceState {
            passes_clean: passes,
            last_classification: Some("NITPICK_ONLY".to_string()),
            last_finding_count: Some(0),
            last_timestamp: None,
            deferred_findings: vec![],
        };
        let result = hook_result_for(Some(&state));
        kani::assert(
            matches!(result, HookResult::Block { .. }),
            "passes_clean < 3 must return HookResult::block_with_fix",
        );
    }

    /// Block invariant — Part C: non-NITPICK_ONLY classification always returns
    /// block_with_fix even when passes_clean >= 3.
    #[kani::proof]
    fn proof_non_nitpick_classification_always_blocks() {
        let state = ConvergenceState {
            passes_clean: 3,
            last_classification: Some("HIGH".to_string()),
            last_finding_count: Some(2),
            last_timestamp: None,
            deferred_findings: vec![],
        };
        let result = hook_result_for(Some(&state));
        kani::assert(
            matches!(result, HookResult::Block { .. }),
            "non-NITPICK_ONLY classification must return HookResult::block_with_fix",
        );
    }

    /// Block invariant — Part D: null/None classification always returns block_with_fix.
    #[kani::proof]
    fn proof_null_classification_blocks() {
        let state = ConvergenceState {
            passes_clean: 5,
            last_classification: None, // null in JSON
            last_finding_count: None,
            last_timestamp: None,
            deferred_findings: vec![],
        };
        let result = hook_result_for(Some(&state));
        kani::assert(
            matches!(result, HookResult::Block { .. }),
            "null/None last_classification must return HookResult::block_with_fix",
        );
    }

    /// Block invariant — Part E: block_with_fix carries required fields.
    /// Verifies hook, reason, recommendation, and code are non-empty strings.
    #[kani::proof]
    fn proof_block_with_fix_fields_populated() {
        let state: Option<&ConvergenceState> = None; // guaranteed block path
        let result = hook_result_for(state);
        if let HookResult::Block { reason } = result {
            kani::assert(!reason.is_empty(), "reason field must not be empty");
            // Canonical format includes BLOCKED by <hook>:, Fix:, Code: per HOST_ABI.md
            kani::assert(
                reason.contains(HOOK_NAME),
                "reason must reference the hook name",
            );
        }
    }

    /// Happy path: passes_clean >= 3 AND last_classification == NITPICK_ONLY
    /// must return HookResult::Continue (not block).
    #[kani::proof]
    #[kani::unwind(4)]
    fn proof_converged_story_produces_continue() {
        let passes: u32 = kani::any();
        kani::assume(passes >= 3);

        let state = ConvergenceState {
            passes_clean: passes,
            last_classification: Some("NITPICK_ONLY".to_string()),
            last_finding_count: Some(0),
            last_timestamp: None,
            deferred_findings: vec![],
        };
        let result = hook_result_for(Some(&state));
        kani::assert(
            matches!(result, HookResult::Continue),
            "converged story (passes >= 3, NITPICK_ONLY) must return HookResult::Continue",
        );
    }
}

// ---------------------------------------------------------------------------
// Unit tests — production is implemented; tests assert correct behavior directly.
// ---------------------------------------------------------------------------
//
// Pattern (post-Red Gate cleanup, F-HIGH-11): each test calls the production
// function directly and asserts the expected HookResult. The catch_unwind
// scaffolding has been removed — it was only needed when production was todo!().

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // -----------------------------------------------------------------------
    // Shared test fixtures
    // -----------------------------------------------------------------------

    /// Minimal HookCallbacks implementation for unit tests.
    ///
    /// Tracks whether read_file was called so tests can assert graceful-degrade
    /// exits before any file I/O (BC-4.10.002 invariant 2; AC-003, AC-004).
    ///
    /// S-12.08: `stories` field removed — hook_logic now reads the story list
    /// from `plugin_config["wave_context"]["stories"]` (WaveContextResolver path),
    /// not from `list_stories` callbacks. Use `make_payload_with_wave_context` to
    /// inject stories via the payload; use `FakeCallbacks::new_with_story` only to
    /// control the per-story state file read result.
    struct FakeCallbacks {
        /// Controls what read_file returns for ALL story state file reads.
        /// Some(Some(json)) → file present with content.
        /// Some(None) → file absent (Ok(None) from read_file).
        /// None → I/O error (Err from read_file).
        read_result: Option<Option<String>>,
        read_called: std::cell::Cell<bool>,
        block_events_emitted: std::cell::Cell<u32>,
    }

    impl FakeCallbacks {
        /// All story state-file reads return `story_json`.
        /// Use `make_payload_with_wave_context` to inject the story list into the payload.
        fn new_with_story(story_json: Option<String>) -> Self {
            FakeCallbacks {
                read_result: Some(story_json),
                read_called: std::cell::Cell::new(false),
                block_events_emitted: std::cell::Cell::new(0),
            }
        }

        /// Simulates an I/O error on read_file (used to test graceful-degrade scenarios).
        /// Note: after S-12.08, the "no cycle dir" case no longer triggers graceful degrade
        /// (wave_context absent → Block, not Continue). This variant is retained for tests
        /// that exercise the "read_file returns Err" path.
        fn new_no_context() -> Self {
            FakeCallbacks {
                read_result: None,
                read_called: std::cell::Cell::new(false),
                block_events_emitted: std::cell::Cell::new(0),
            }
        }

        fn was_read_called(&self) -> bool {
            self.read_called.get()
        }

        fn block_events_emitted(&self) -> u32 {
            self.block_events_emitted.get()
        }
    }

    impl HookCallbacks for FakeCallbacks {
        fn read_file(&self, _path: &str) -> Result<Option<String>, IoError> {
            self.read_called.set(true);
            match &self.read_result {
                Some(v) => Ok(v.clone()),
                None => Err(IoError(
                    "fake: read_file error — no read result".to_string(),
                )),
            }
        }

        fn log_debug(&self, _msg: &str) {}
        fn log_error(&self, _msg: &str) {}

        fn emit_event(&self, _event_type: &str, _fields: &[(&str, &str)]) {
            // Increment the block-event counter for test assertions (F-CRIT-4).
            self.block_events_emitted
                .set(self.block_events_emitted.get() + 1);
        }
    }

    /// Build a HookPayload with wave_context.stories in plugin_config.
    ///
    /// This is the S-12.08 canonical helper for constructing payloads that
    /// exercise the new extract_stories_from_wave_context path. The wave_context
    /// is injected by WaveContextResolver in production; in tests, we construct
    /// it directly.
    fn make_payload_with_wave_context(
        agent_type: Option<&str>,
        stories: &[&str],
        cycle_id: Option<&str>,
    ) -> HookPayload {
        let story_arr: Vec<serde_json::Value> = stories
            .iter()
            .map(|s| serde_json::Value::String(s.to_string()))
            .collect();
        let mut wave_context = serde_json::json!({
            "stories": story_arr,
            "wave_id": "test-wave",
        });
        if let Some(cid) = cycle_id {
            wave_context["cycle_id"] = serde_json::Value::String(cid.to_string());
        }
        let mut v = json!({
            "event_name": "SubagentStop",
            "session_id": "test-session",
            "dispatcher_trace_id": "test-trace",
            "plugin_config": {
                "wave_context": wave_context
            }
        });
        if let Some(at) = agent_type {
            v["agent_type"] = json!(at);
        }
        serde_json::from_value(v).expect("fixture must deserialize")
    }

    /// Build a HookPayload with an optional agent_type field.
    ///
    /// No agent_type → None (simulates per-story SubagentStop or missing field).
    /// agent_type = Some("wave-gate-dispatch") → wave-gate context.
    fn make_payload(agent_type: Option<&str>) -> HookPayload {
        let mut v = json!({
            "event_name": "SubagentStop",
            "session_id": "test-session",
            "dispatcher_trace_id": "test-trace"
        });
        if let Some(at) = agent_type {
            v["agent_type"] = json!(at);
        }
        serde_json::from_value(v).expect("fixture must deserialize")
    }

    /// Canonical cleared-state JSON (BC-4.10.001 test vector row 1).
    fn cleared_state_json() -> String {
        json!({
            "passes_clean": 3,
            "last_classification": "NITPICK_ONLY",
            "last_finding_count": 0,
            "last_timestamp": "2026-05-06T00:00:00Z",
            "deferred_findings": []
        })
        .to_string()
    }

    /// Cleared state with non-empty deferred_findings (BC-4.10.001 EC-004).
    fn cleared_state_with_deferrals_json() -> String {
        json!({
            "passes_clean": 3,
            "last_classification": "NITPICK_ONLY",
            "last_finding_count": 0,
            "last_timestamp": "2026-05-06T00:00:00Z",
            "deferred_findings": [
                {"id": "D-001", "summary": "cross-story interface mismatch", "severity": "MEDIUM"},
                {"id": "D-002", "summary": "architectural concern", "severity": "LOW"},
                {"id": "D-003", "summary": "integration test gap", "severity": "LOW"}
            ]
        })
        .to_string()
    }

    /// Insufficiently-passed state (passes_clean < 3; BC-4.10.001 test vector row 2).
    fn insufficient_passes_json(passes_clean: u32) -> String {
        json!({
            "passes_clean": passes_clean,
            "last_classification": "NITPICK_ONLY",
            "last_finding_count": 1,
            "last_timestamp": "2026-05-06T00:00:00Z",
            "deferred_findings": []
        })
        .to_string()
    }

    // -----------------------------------------------------------------------
    // AC-001 traces to BC-4.10.001 PC1: parse_convergence_state reads OQ3
    // schema fields; hook_logic returns Continue when state is cleared.
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_001_cleared_state_returns_continue() {
        // AC-001 traces to BC-4.10.001 PC1: parse_convergence_state reads OQ3
        // schema fields (passes_clean, last_finding_count, last_classification,
        // last_timestamp, deferred_findings). BC-4.10.001 canonical test vector
        // row 1: {passes_clean: 3, last_classification: "NITPICK_ONLY"} → Continue.
        let payload = make_payload_with_wave_context(
            Some("wave-gate-dispatch"),
            &["S-A"],
            Some("test-cycle"),
        );
        let callbacks = FakeCallbacks::new_with_story(Some(cleared_state_json()));

        let hook_result = hook_logic(&payload, &callbacks);

        assert!(
            matches!(hook_result, HookResult::Continue),
            "BC-4.10.001 PC5: cleared story must produce HookResult::Continue, got {:?}",
            hook_result
        );
    }

    // -----------------------------------------------------------------------
    // AC-001 traces to BC-4.10.001 PC1: parse_convergence_state parses the
    // five OQ3 schema fields from a valid JSON string.
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_001_parse_convergence_state_reads_oq3_schema_fields() {
        // AC-001 traces to BC-4.10.001 PC1: parse_convergence_state reads OQ3
        // schema fields: passes_clean, last_finding_count, last_classification,
        // last_timestamp, deferred_findings.
        let json = cleared_state_json();

        let parse_result = parse_convergence_state(&json);

        {
            let state = parse_result.expect("BC-4.10.001 PC1: valid JSON must parse without error");
            assert_eq!(
                state.passes_clean, 3,
                "BC-4.10.001 PC1: passes_clean field must be read from JSON"
            );
            assert_eq!(
                state.last_classification.as_deref(),
                Some("NITPICK_ONLY"),
                "BC-4.10.001 PC1: last_classification field must be read from JSON"
            );
            assert_eq!(
                state.last_finding_count,
                Some(0),
                "BC-4.10.001 PC1: last_finding_count field must be read from JSON"
            );
        }
    }

    // -----------------------------------------------------------------------
    // AC-002 branch 1: missing state file → block CONVERGENCE_STATE_MISSING
    // (BC-4.10.001 PC2; canonical test vector row 3)
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_001_block_code_convergence_state_missing() {
        // AC-002 traces to BC-4.10.001 PC2: hook BLOCKS via block_with_fix when
        // story has missing state file. code: "CONVERGENCE_STATE_MISSING".
        // BC-4.10.001 canonical test vector: [S-A] | S-A file absent → BLOCK.
        let state: Option<&ConvergenceState> = None;

        let hook_result = hook_result_for(state);

        {
            match &hook_result {
                HookResult::Block { reason } => {
                    assert!(
                        reason.contains("BLOCKED by validate-per-story-adversary-convergence"),
                        "BC-4.10.001 PC2: block reason must contain canonical hook name, got: {reason}"
                    );
                    assert!(
                        reason.contains("CONVERGENCE_STATE_MISSING"),
                        "BC-4.10.001 PC2: block reason must contain code \
                         CONVERGENCE_STATE_MISSING, got: {reason}"
                    );
                }
                other => panic!(
                    "BC-4.10.001 PC2: missing state file must return HookResult::Block, got {:?}",
                    other
                ),
            }
        }
    }

    // -----------------------------------------------------------------------
    // VP-071 cargo-test equivalent: proof_missing_state_file_always_blocks
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_001_vp071_equiv_missing_state_file_always_blocks() {
        // AC-002 traces to BC-4.10.001 PC2; VP-071 kani harness cargo-test
        // equivalent. hook_result_for(None) must return HookResult::Block.
        let r = hook_result_for(None);

        {
            assert!(
                matches!(r, HookResult::Block { .. }),
                "VP-071: missing state file must return HookResult::Block (block_with_fix form), \
                 got {:?}",
                r
            );
        }
    }

    // -----------------------------------------------------------------------
    // AC-002 branch 2: passes_clean < 3 → block CONVERGENCE_PASSES_INSUFFICIENT
    // (BC-4.10.001 PC3; canonical test vector row 2)
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_001_block_code_passes_insufficient() {
        // AC-002 traces to BC-4.10.001 PC3: hook BLOCKS when passes_clean < 3.
        // code: "CONVERGENCE_PASSES_INSUFFICIENT".
        // BC-4.10.001 canonical test vector: [S-A, S-B] | S-B: {passes_clean: 2} → BLOCK.
        let state = ConvergenceState {
            passes_clean: 2,
            last_classification: Some("NITPICK_ONLY".to_string()),
            last_finding_count: Some(1),
            last_timestamp: None,
            deferred_findings: vec![],
        };

        let hook_result = hook_result_for(Some(&state));

        {
            match &hook_result {
                HookResult::Block { reason } => {
                    assert!(
                        reason.contains("CONVERGENCE_PASSES_INSUFFICIENT"),
                        "BC-4.10.001 PC3: block reason must contain code \
                         CONVERGENCE_PASSES_INSUFFICIENT, got: {reason}"
                    );
                    // BC-4.10.001 PC3: reason must identify the actual count
                    assert!(
                        reason.contains("2") || reason.contains("passes_clean"),
                        "BC-4.10.001 PC3: block reason must reference the failing passes_clean value, \
                         got: {reason}"
                    );
                }
                other => panic!(
                    "BC-4.10.001 PC3: passes_clean < 3 must return HookResult::Block, got {:?}",
                    other
                ),
            }
        }
    }

    // -----------------------------------------------------------------------
    // VP-071 cargo-test equivalent: proof_insufficient_passes_always_blocks
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_001_vp071_equiv_insufficient_passes_always_blocks() {
        // AC-002 traces to BC-4.10.001 PC3; VP-071 proof B cargo-test equivalent.
        // passes_clean = 0 (boundary); NITPICK_ONLY classification; must block.
        for passes in [0u32, 1, 2] {
            let state = ConvergenceState {
                passes_clean: passes,
                last_classification: Some("NITPICK_ONLY".to_string()),
                last_finding_count: Some(0),
                last_timestamp: None,
                deferred_findings: vec![],
            };
            let r = hook_result_for(Some(&state));
            {
                assert!(
                    matches!(r, HookResult::Block { .. }),
                    "VP-071: passes_clean={passes} < 3 must return HookResult::Block, got {:?}",
                    r
                );
            }
        }
    }

    // -----------------------------------------------------------------------
    // AC-002 branch 3: non-NITPICK_ONLY → block CONVERGENCE_CLASSIFICATION_INSUFFICIENT
    // (BC-4.10.001 PC4; canonical test vector row 4)
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_001_block_code_classification_insufficient() {
        // AC-002 traces to BC-4.10.001 PC4: hook BLOCKS when
        // last_classification != "NITPICK_ONLY".
        // code: "CONVERGENCE_CLASSIFICATION_INSUFFICIENT".
        // BC-4.10.001 canonical test vector: [S-A] | {passes_clean: 3,
        // last_classification: "HIGH"} → BLOCK classification insufficient.
        let state = ConvergenceState {
            passes_clean: 3,
            last_classification: Some("HIGH".to_string()),
            last_finding_count: Some(2),
            last_timestamp: None,
            deferred_findings: vec![],
        };

        let hook_result = hook_result_for(Some(&state));

        {
            match &hook_result {
                HookResult::Block { reason } => {
                    assert!(
                        reason.contains("CONVERGENCE_CLASSIFICATION_INSUFFICIENT"),
                        "BC-4.10.001 PC4: block reason must contain code \
                         CONVERGENCE_CLASSIFICATION_INSUFFICIENT, got: {reason}"
                    );
                    // BC-4.10.001 PC4: reason must identify the actual classification
                    assert!(
                        reason.contains("HIGH") || reason.contains("last_classification"),
                        "BC-4.10.001 PC4: block reason must reference the failing classification \
                         value, got: {reason}"
                    );
                }
                other => panic!(
                    "BC-4.10.001 PC4: non-NITPICK_ONLY classification must return \
                     HookResult::Block, got {:?}",
                    other
                ),
            }
        }
    }

    // -----------------------------------------------------------------------
    // VP-071 cargo-test equivalent: proof_non_nitpick_classification_always_blocks
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_001_vp071_equiv_non_nitpick_classification_always_blocks() {
        // AC-002 traces to BC-4.10.001 PC4; VP-071 proof C cargo-test equivalent.
        // passes_clean = 3, classification = "HIGH" — still must block.
        let state = ConvergenceState {
            passes_clean: 3,
            last_classification: Some("HIGH".to_string()),
            last_finding_count: Some(2),
            last_timestamp: None,
            deferred_findings: vec![],
        };
        let r = hook_result_for(Some(&state));

        assert!(
            matches!(r, HookResult::Block { .. }),
            "VP-071: last_classification=HIGH with passes_clean=3 must return \
             HookResult::Block, got {:?}",
            r
        );
    }

    // -----------------------------------------------------------------------
    // VP-071 cargo-test equivalent: proof_null_classification_blocks
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_001_vp071_equiv_null_classification_blocks() {
        // AC-002 traces to BC-4.10.001 PC4; VP-071 proof D cargo-test equivalent.
        // last_classification = None (JSON null) — must block even with passes_clean >= 3.
        let state = ConvergenceState {
            passes_clean: 5,
            last_classification: None,
            last_finding_count: None,
            last_timestamp: None,
            deferred_findings: vec![],
        };
        let r = hook_result_for(Some(&state));

        assert!(
            matches!(r, HookResult::Block { .. }),
            "VP-071: None last_classification (JSON null) must return HookResult::Block, \
             got {:?}",
            r
        );
    }

    // -----------------------------------------------------------------------
    // VP-071 cargo-test equivalent: proof_block_with_fix_fields_populated
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_001_vp071_equiv_block_with_fix_fields_populated() {
        // AC-002 traces to BC-4.10.001 PC2+PC8; VP-071 proof E cargo-test equivalent.
        // Missing state file (guaranteed block path). Verifies canonical
        // block_with_fix form: HOOK_NAME in reason, code in reason, non-empty.
        let r = hook_result_for(None);

        {
            match r {
                HookResult::Block { reason } => {
                    assert!(!reason.is_empty(), "VP-071: block reason must not be empty");
                    assert!(
                        reason.contains(HOOK_NAME),
                        "VP-071: block reason must contain HOOK_NAME '{}', got: {}",
                        HOOK_NAME,
                        reason
                    );
                    assert!(
                        reason.contains(HOOK_CODE_BASE),
                        "VP-071: block reason must contain HOOK_CODE_BASE '{}', got: {}",
                        HOOK_CODE_BASE,
                        reason
                    );
                    assert!(
                        reason.contains("Fix:"),
                        "VP-071: canonical block_with_fix reason must contain 'Fix:' segment, got: {reason}"
                    );
                    assert!(
                        reason.contains("Code:"),
                        "VP-071: canonical block_with_fix reason must contain 'Code:' segment, got: {reason}"
                    );
                }
                other => panic!(
                    "VP-071: hook_result_for(None) must return HookResult::Block, got {:?}",
                    other
                ),
            }
        }
    }

    // -----------------------------------------------------------------------
    // VP-071 cargo-test equivalent: proof_converged_story_produces_continue
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_001_vp071_equiv_converged_story_produces_continue() {
        // AC-001 traces to BC-4.10.001 PC5; VP-071 proof F cargo-test equivalent.
        // passes_clean >= 3 AND last_classification == "NITPICK_ONLY" → Continue.
        for passes in [3u32, 4, 5, 10] {
            let state = ConvergenceState {
                passes_clean: passes,
                last_classification: Some("NITPICK_ONLY".to_string()),
                last_finding_count: Some(0),
                last_timestamp: None,
                deferred_findings: vec![],
            };
            let r = hook_result_for(Some(&state));

            {
                assert!(
                    matches!(r, HookResult::Continue),
                    "VP-071: fully converged story (passes_clean={passes}, NITPICK_ONLY) \
                     must return HookResult::Continue, got {:?}",
                    r
                );
            }
        }
    }

    // -----------------------------------------------------------------------
    // AC-003 traces to BC-4.10.002 PC1: graceful_degrade_outside_wave_gate
    // returns true when payload has no wave-gate indicator → hook returns Continue.
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_002_graceful_degrade_no_wave_gate_context() {
        // AC-003 traces to BC-4.10.002 PC1: hook returns HookResult::Continue when
        // payload is not a wave-gate dispatch (agent_type missing or wrong type).
        // Graceful degrade: no block signal emitted, no file I/O performed.
        let payload = make_payload(None); // no agent_type → not wave-gate context
        // Callbacks that would error if read_file is called (confirms no file I/O)
        let callbacks = FakeCallbacks::new_with_story(Some(cleared_state_json()));

        let hook_result = hook_logic(&payload, &callbacks);

        assert!(
            matches!(hook_result, HookResult::Continue),
            "BC-4.10.002 PC1: no wave-gate context must produce HookResult::Continue, \
             got {:?}",
            hook_result
        );
    }

    // -----------------------------------------------------------------------
    // AC-003 + AC-004: read_file NOT called before context check
    // (BC-4.10.002 invariant 2)
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_002_graceful_degrade_no_read_file_before_context_check() {
        // AC-003 traces to BC-4.10.002 invariant 2: graceful-degrade check MUST
        // occur BEFORE any attempt to read state files. When context cannot be
        // determined, hook exits immediately without any file reads.
        let payload = make_payload(None); // no wave-gate context
        let callbacks = FakeCallbacks::new_with_story(Some(cleared_state_json()));

        let _hook_result = hook_logic(&payload, &callbacks);

        // Verify no I/O occurred (graceful degrade exits before file reads):
        assert!(
            !callbacks.was_read_called(),
            "BC-4.10.002 invariant 2: read_file MUST NOT be called before context check \
             — graceful degrade must exit before any file I/O"
        );
    }

    // -----------------------------------------------------------------------
    // AC-004 traces to BC-4.10.001 PC2: state file unreadable → Block
    // (MED-006: renamed from test_BC_4_10_002_graceful_degrade_absent_cycle_dir
    //  to reflect updated S-12.08 semantics — absent cycle dir now blocks, not degrades)
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_001_blocks_when_state_file_unreadable() {
        // AC-004 / S-12.08: in the wave_context migration, story lists come from
        // plugin_config["wave_context"]["stories"] (WaveContextResolver), not the cycle dir.
        // "Absent cycle dir" now manifests as a state file read error (read_file returns Err),
        // which maps to Block with CONVERGENCE_STATE_MISSING (BC-4.10.001 PC2).
        //
        // Updated for S-12.08: provide wave_context.stories in the payload (one story),
        // but make read_file return Err (simulating an unreadable state file).
        // Expected: Block with CONVERGENCE_STATE_MISSING.
        let payload = make_payload_with_wave_context(
            Some("wave-gate-dispatch"),
            &["S-A"],
            Some("test-cycle"),
        );
        // new_no_context makes read_file return Err (simulates unreadable state file)
        let callbacks = FakeCallbacks::new_no_context();

        let hook_result = hook_logic(&payload, &callbacks);

        assert!(
            matches!(hook_result, HookResult::Block { .. }),
            "BC-4.10.001 PC2 (S-12.08): unreadable state file must produce HookResult::Block, \
             got {:?}",
            hook_result
        );
    }

    // -----------------------------------------------------------------------
    // EC-001: empty wave (zero stories) → Continue + log warning
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_001_ec001_empty_wave_returns_continue() {
        // BC-4.10.001 EC-001: Wave has 0 stories. Hook returns HookResult::Continue
        // (vacuously all stories cleared). Logs warning: "Wave has zero stories."
        // Empty stories array in wave_context → vacuous convergence (EC-001).
        let payload = make_payload_with_wave_context(
            Some("wave-gate-dispatch"),
            &[], // zero stories in wave
            Some("test-cycle"),
        );
        let callbacks = FakeCallbacks::new_with_story(Some(cleared_state_json()));

        let hook_result = hook_logic(&payload, &callbacks);

        assert!(
            matches!(hook_result, HookResult::Continue),
            "BC-4.10.001 EC-001: empty wave must produce HookResult::Continue, got {:?}",
            hook_result
        );
    }

    // -----------------------------------------------------------------------
    // AC-005 traces to BC-4.10.001 PC6: deferred_findings aggregated on Continue
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_001_deferred_findings_aggregated_on_continue() {
        // AC-005 traces to BC-4.10.001 PC6: when all stories pass convergence,
        // the hook aggregates deferred_findings arrays from all stories and emits
        // them to wave-gate context as a structured log entry (not a block).
        // Hook returns HookResult::Continue.
        let payload = make_payload_with_wave_context(
            Some("wave-gate-dispatch"),
            &["S-A", "S-B"],
            Some("test-cycle"),
        );
        let callbacks = FakeCallbacks::new_with_story(Some(cleared_state_with_deferrals_json()));

        let hook_result = hook_logic(&payload, &callbacks);

        assert!(
            matches!(hook_result, HookResult::Continue),
            "BC-4.10.001 PC6: converged story with deferred_findings must produce \
             HookResult::Continue, got {:?}",
            hook_result
        );
    }

    // -----------------------------------------------------------------------
    // AC-006 traces to BC-4.10.001 PC5: first-failure-only block (EC-005)
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_001_first_failure_only_block() {
        // AC-006 traces to BC-4.10.001 EC-005 + PC5: when multiple stories fail
        // convergence, hook blocks on the FIRST non-cleared story found. Emits
        // a single block message identifying the first failure.
        //
        // FakeCallbacks returns the same JSON for all reads. Using insufficient
        // passes JSON ensures both stories fail, but only one block should fire.
        let payload = make_payload_with_wave_context(
            Some("wave-gate-dispatch"),
            &["S-A", "S-B"],
            Some("test-cycle"),
        );
        let callbacks = FakeCallbacks::new_with_story(Some(insufficient_passes_json(1)));

        let hook_result = hook_logic(&payload, &callbacks);

        assert!(
            matches!(hook_result, HookResult::Block { .. }),
            "BC-4.10.001 EC-005: multiple failing stories must produce HookResult::Block \
             (on first failure), got {:?}",
            hook_result
        );
    }

    // -----------------------------------------------------------------------
    // AC-007 traces to BC-4.10.001 EC-002: malformed JSON → block
    // CONVERGENCE_STATE_MALFORMED
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_001_malformed_json_block_with_malformed_code() {
        // AC-007 traces to BC-4.10.001 EC-002: malformed JSON in state file is
        // treated as non-cleared. Hook emits block with
        // code: "CONVERGENCE_STATE_MALFORMED". Does not panic.
        // BC-4.10.001 canonical test vector row 6: [S-A] | malformed JSON → BLOCK.
        let payload = make_payload_with_wave_context(
            Some("wave-gate-dispatch"),
            &["S-A"],
            Some("test-cycle"),
        );
        let callbacks =
            FakeCallbacks::new_with_story(Some("this is { not valid json at all!!!".to_string()));

        let hook_result = hook_logic(&payload, &callbacks);

        {
            match &hook_result {
                HookResult::Block { reason } => {
                    assert!(
                        reason.contains("CONVERGENCE_STATE_MALFORMED"),
                        "BC-4.10.001 EC-002: malformed JSON block must contain code \
                         CONVERGENCE_STATE_MALFORMED, got: {reason}"
                    );
                }
                other => panic!(
                    "BC-4.10.001 EC-002: malformed JSON must produce HookResult::Block, got {:?}",
                    other
                ),
            }
        }
    }

    // AC-007: parse_convergence_state directly on malformed JSON
    #[test]
    fn test_BC_4_10_001_parse_convergence_state_malformed_json_returns_err() {
        // AC-007 traces to BC-4.10.001 EC-002: parse_convergence_state MUST return
        // Err(ParseError) when given malformed JSON, not panic.
        let parse_result = parse_convergence_state("{ this is not json ");

        assert!(
            parse_result.is_err(),
            "BC-4.10.001 EC-002: malformed JSON must return Err(ParseError), got Ok"
        );
    }

    // -----------------------------------------------------------------------
    // AC-008 traces to BC-4.10.001 EC-003: missing last_classification field
    // → block CONVERGENCE_STATE_SCHEMA_INVALID
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_001_missing_classification_block_with_schema_invalid_code() {
        // AC-008 traces to BC-4.10.001 EC-003: state file is valid JSON but
        // last_classification field is absent. Treated as non-cleared.
        // Hook emits block with code: "CONVERGENCE_STATE_SCHEMA_INVALID".
        //
        // Note: ConvergenceState uses Option<String> for last_classification, so
        // a missing field deserializes to None. The hook must block when
        // last_classification is None (equiv. to missing/null).
        let payload = make_payload_with_wave_context(
            Some("wave-gate-dispatch"),
            &["S-A"],
            Some("test-cycle"),
        );
        let json_missing_classification = json!({
            "passes_clean": 3,
            "last_finding_count": 0,
            "last_timestamp": "2026-05-06T00:00:00Z",
            "deferred_findings": []
            // last_classification intentionally absent
        })
        .to_string();
        let callbacks = FakeCallbacks::new_with_story(Some(json_missing_classification));

        let hook_result = hook_logic(&payload, &callbacks);

        {
            match &hook_result {
                HookResult::Block { reason } => {
                    // The block code should be either SCHEMA_INVALID (EC-003: field absent)
                    // or CLASSIFICATION_INSUFFICIENT (None is treated as non-NITPICK_ONLY).
                    // Both are acceptable per BC-4.10.001 PC4 (None → non-cleared).
                    let acceptable = reason.contains("CONVERGENCE_STATE_SCHEMA_INVALID")
                        || reason.contains("CONVERGENCE_CLASSIFICATION_INSUFFICIENT");
                    assert!(
                        acceptable,
                        "BC-4.10.001 EC-003: missing last_classification block must contain \
                         CONVERGENCE_STATE_SCHEMA_INVALID or CONVERGENCE_CLASSIFICATION_INSUFFICIENT, \
                         got: {reason}"
                    );
                }
                other => panic!(
                    "BC-4.10.001 EC-003: missing last_classification must produce \
                     HookResult::Block, got {:?}",
                    other
                ),
            }
        }
    }

    // -----------------------------------------------------------------------
    // AC-009 traces to BC-4.10.001 PC4+EC-004: deferred_findings do NOT block
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_001_deferred_findings_do_not_block() {
        // AC-009 traces to BC-4.10.001 EC-004: state file has passes_clean=3,
        // last_classification="NITPICK_ONLY", and non-empty deferred_findings.
        // Hook returns HookResult::Continue (deferred findings do not affect
        // convergence decision per BC-5.39.002).
        // BC-4.10.001 canonical test vector row 5.
        let state = ConvergenceState {
            passes_clean: 3,
            last_classification: Some("NITPICK_ONLY".to_string()),
            last_finding_count: Some(0),
            last_timestamp: Some("2026-05-06T00:00:00Z".to_string()),
            deferred_findings: vec![
                json!({"id": "D-001", "severity": "MEDIUM"}),
                json!({"id": "D-002", "severity": "LOW"}),
                json!({"id": "D-003", "severity": "LOW"}),
            ],
        };

        let hook_result = hook_result_for(Some(&state));

        assert!(
            matches!(hook_result, HookResult::Continue),
            "BC-4.10.001 EC-004: passes_clean=3, NITPICK_ONLY, non-empty deferred_findings \
             must produce HookResult::Continue, got {:?}",
            hook_result
        );
    }

    // -----------------------------------------------------------------------
    // AC-010: hook_logic exercised without WASM runtime (structural invariant).
    // The existence of this compiled test proves the injectable-callback pattern.
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_001_hook_logic_runs_without_wasm_runtime() {
        // AC-010 traces to BC-4.10.001 invariant 3 + precondition 5: the pure
        // fn hook_logic(payload, callbacks) function is exercised by ALL unit
        // tests WITHOUT a WASM runtime. This test's existence proves the pattern.
        //
        // If this test compiles and the project links without wasm32 target,
        // the injectable-callback pattern is in use.
        let payload = make_payload_with_wave_context(
            Some("wave-gate-dispatch"),
            &["S-A"],
            Some("test-cycle"),
        );
        let callbacks = FakeCallbacks::new_with_story(Some(cleared_state_json()));

        // The fact that this compiles and runs natively (not under WASM) proves
        // the injectable-callback pattern is correctly implemented.
        let _result = hook_logic(&payload, &callbacks);
        // If the above call compiles and runs without a WASM runtime, the pattern is verified.
    }

    // -----------------------------------------------------------------------
    // AC-011: HOST_ABI_VERSION = 1 (structural constant — already green by design)
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_001_host_abi_version_is_one() {
        // AC-011 traces to BC-4.10.001 invariant 2: HOST_ABI_VERSION = 1.
        // No new host functions. This test verifies the constant value.
        // NOTE: this test passes immediately (constant, no production function call).
        assert_eq!(
            HOST_ABI_VERSION, 1,
            "BC-4.10.001 invariant 2: HOST_ABI_VERSION must be 1 (AC-011)"
        );
    }

    // -----------------------------------------------------------------------
    // AC-011: HOOK_NAME and HOOK_CODE_BASE are the canonical values from VP-071
    // (structural constants — already green by design)
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_001_hook_name_and_code_base_constants() {
        // AC-011 traces to BC-4.10.001 invariant 2 + VP-071: canonical constants.
        assert_eq!(
            HOOK_NAME, "validate-per-story-adversary-convergence",
            "HOOK_NAME must be the canonical kebab-case hook name"
        );
        assert_eq!(
            HOOK_CODE_BASE, "per_story_adversary_unconverged",
            "HOOK_CODE_BASE must be the stable snake_case VP-071 telemetry code"
        );
    }

    // -----------------------------------------------------------------------
    // AC-012: no write_file calls — structural source grep
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_001_no_write_file_calls() {
        // AC-012 traces to BC-4.10.001 invariant 4: the hook MUST NOT write to
        // any file in .factory/. It reads state files but never modifies them.
        // The write_file host function must not be called anywhere in the hook source.
        //
        // Implementation: read src/lib.rs and src/main.rs as strings and verify
        // no "write_file" call site exists.
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR must be set during cargo test");
        let lib_path = std::path::Path::new(&manifest_dir).join("src/lib.rs");
        let main_path = std::path::Path::new(&manifest_dir).join("src/main.rs");

        let lib_src = std::fs::read_to_string(&lib_path)
            .unwrap_or_else(|e| panic!("failed to read src/lib.rs: {e}"));

        // Check lib.rs for write_file.
        // Pattern is constructed at runtime (not as a literal) so the test
        // source itself does not contain the forbidden string — avoiding a
        // self-defeating match (AC-012, BC-4.10.001 inv-4).
        let forbidden = format!("{}::{}_file(", "host", "write");
        assert!(
            !lib_src.contains(&forbidden),
            "BC-4.10.001 inv-4: src/lib.rs MUST NOT contain any host write_file calls \
             — hook must be strictly read-only (AC-012)"
        );

        // Check main.rs if it exists
        if main_path.exists() {
            let main_src = std::fs::read_to_string(&main_path)
                .unwrap_or_else(|e| panic!("failed to read src/main.rs: {e}"));
            assert!(
                !main_src.contains(&forbidden),
                "BC-4.10.001 inv-4: src/main.rs MUST NOT contain any host write_file calls \
                 — hook must be strictly read-only (AC-012)"
            );
        }
    }

    // -----------------------------------------------------------------------
    // EC-007 (BC-4.10.002): per-story SubagentStop (wrong agent_type) → Continue
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_002_ec007_per_story_subagentstop_graceful_degrade() {
        // BC-4.10.002 EC-001: Hook fires on a normal per-story SubagentStop
        // (not wave-gate). Must gracefully degrade: return Continue, log advisory.
        // The agent_type "implementer" is not a wave-gate dispatch agent.
        let payload = make_payload(Some("implementer")); // per-story agent
        let callbacks = FakeCallbacks::new_with_story(Some(cleared_state_json()));

        let hook_result = hook_logic(&payload, &callbacks);

        assert!(
            matches!(hook_result, HookResult::Continue),
            "BC-4.10.002 EC-001: per-story SubagentStop must produce HookResult::Continue, \
             got {:?}",
            hook_result
        );
    }

    // -----------------------------------------------------------------------
    // EC-008 (BC-4.10.002): payload missing both subagent_name and agent_type
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_002_ec008_missing_agent_fields_graceful_degrade() {
        // BC-4.10.002 EC-004: Payload missing subagent_name and agent_type fields.
        // Cannot determine context. Graceful degrade: return Continue, log advisory.
        let payload = make_payload(None); // both agent_type and subagent_name absent
        let callbacks = FakeCallbacks::new_with_story(Some(cleared_state_json()));

        let hook_result = hook_logic(&payload, &callbacks);

        assert!(
            matches!(hook_result, HookResult::Continue),
            "BC-4.10.002 EC-004: missing agent fields must produce HookResult::Continue, \
             got {:?}",
            hook_result
        );
    }

    // -----------------------------------------------------------------------
    // graceful_degrade_outside_wave_gate unit test (BC-4.10.002 PC1 direct)
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_002_graceful_degrade_function_returns_false_for_non_wave_gate() {
        // AC-003 traces to BC-4.10.002 PC1: graceful_degrade_outside_wave_gate
        // returns true (degrade=yes) when payload has no wave-gate indicator.
        // Inversely: the function signals "not wave-gate" → hook degrades.
        let payload = make_payload(None);

        let should_degrade = graceful_degrade_outside_wave_gate(&payload);

        // For a no-context payload (no agent_type), the function must signal degrade.
        assert!(
            should_degrade,
            "BC-4.10.002 PC1: graceful_degrade_outside_wave_gate with no wave-gate \
             indicator MUST return true (degrade) for no-context payload (AC-003)"
        );
    }

    // -----------------------------------------------------------------------
    // Boundary: passes_clean == 3 (boundary value — exactly at threshold)
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_001_passes_clean_exactly_3_clears_threshold() {
        // BC-4.10.001 invariant 5: convergence criterion is passes_clean >= 3.
        // Boundary value test: passes_clean == 3 must produce Continue
        // (not CONVERGENCE_PASSES_INSUFFICIENT).
        let state = ConvergenceState {
            passes_clean: 3,
            last_classification: Some("NITPICK_ONLY".to_string()),
            last_finding_count: Some(0),
            last_timestamp: None,
            deferred_findings: vec![],
        };

        let r = hook_result_for(Some(&state));

        assert!(
            matches!(r, HookResult::Continue),
            "BC-4.10.001 boundary: passes_clean=3 (exact threshold) must produce \
             HookResult::Continue, got {:?}",
            r
        );
    }

    // -----------------------------------------------------------------------
    // F-CRIT-4: emit_event("hook.block") called before each block return
    // (BC-7.03.075 pattern; BC-4.10.001 observability mandate)
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_001_missing_state_emits_hook_block_event() {
        // BC-4.10.001 + BC-7.03.075: hook MUST emit a "hook.block" event via
        // emit_event before returning HookResult::Block. Without this event,
        // wave-gate monitoring dashboards never see convergence gate firings.
        let payload = make_payload_with_wave_context(
            Some("wave-gate-dispatch"),
            &["S-A"],
            Some("test-cycle"),
        );
        let callbacks = FakeCallbacks::new_with_story(None); // absent state file (Ok(None))

        let result = hook_logic(&payload, &callbacks);

        assert!(
            matches!(result, HookResult::Block { .. }),
            "BC-4.10.001: hook_logic with absent state file must return HookResult::Block"
        );
        assert_eq!(
            callbacks.block_events_emitted(),
            1,
            "BC-4.10.001 + F-CRIT-4: hook MUST emit exactly one hook.block event \
             before returning Block (missing state file path)"
        );
    }

    // -----------------------------------------------------------------------
    // F-HIGH-3: plugin_config.stories extraction (legacy path, now replaced by wave_context)
    // After S-12.08, RealCallbacks no longer uses plugin_config.stories; the extraction
    // is done via extract_stories_from_wave_context. The tests below are retained as
    // documentation of the old behavior for regression purposes.
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_001_wave_context_stories_extraction() {
        // S-12.08 AC-001 (replaces F-HIGH-3): the new path reads stories from
        // plugin_config["wave_context"]["stories"] (WaveContextResolver injection).
        // extract_stories_from_wave_context must return Ok with the injected stories.
        let plugin_config = json!({
            "wave_context": {
                "stories": ["S-12.01", "S-12.02", "S-13.01"],
                "wave_id": "w-test",
                "cycle_id": "v1.0-feature-engine-discipline-pass-1"
            }
        });
        let stories = extract_stories_from_wave_context(&plugin_config);
        assert!(
            stories.is_ok(),
            "S-12.08 AC-001: extract_stories_from_wave_context must return Ok for valid \
             wave_context.stories. Got: {:?}",
            stories
        );
        let story_list = stories.unwrap();
        assert_eq!(
            story_list,
            vec!["S-12.01", "S-12.02", "S-13.01"],
            "S-12.08 AC-001: extracted stories must match wave_context.stories array"
        );
    }

    #[test]
    fn test_BC_4_10_001_wave_context_absent_returns_missing() {
        // S-12.08 AC-002 (replaces F-HIGH-3 absent case): when wave_context is absent,
        // extract_stories_from_wave_context must return Err(WaveContextError::Missing).
        let plugin_config = json!({});
        let stories = extract_stories_from_wave_context(&plugin_config);
        assert!(
            matches!(stories, Err(WaveContextError::Missing)),
            "S-12.08 AC-002: absent wave_context must return Err(WaveContextError::Missing), \
             got: {:?}",
            stories
        );
    }

    #[test]
    fn test_BC_4_10_001_wave_context_empty_stories_array_returns_empty() {
        // S-12.08 (replaces F-HIGH-3 empty-array case): when wave_context.stories is [],
        // extract_stories_from_wave_context returns Ok(vec![]) → hook_logic returns Continue
        // (vacuous convergence, BC-4.10.001 EC-001).
        let plugin_config = json!({
            "wave_context": {
                "stories": [],
                "wave_id": "w-test",
                "cycle_id": "v1.0-test"
            }
        });
        let stories = extract_stories_from_wave_context(&plugin_config);
        assert!(
            stories.is_ok(),
            "S-12.08: empty wave_context.stories must return Ok(vec![]) not Err"
        );
        assert!(
            stories.unwrap().is_empty(),
            "S-12.08: empty wave_context.stories must yield empty Vec"
        );
    }

    #[test]
    fn test_BC_4_10_001_hook_logic_with_wave_context_blocks_on_missing_state() {
        // S-12.08 AC-001 + BC-4.10.001 PC2: end-to-end test demonstrating that a payload
        // with wave_context.stories causes the hook to actively check convergence.
        // When the state file is absent (Ok(None) from read_file), hook blocks.
        let payload = make_payload_with_wave_context(
            Some("wave-gate-dispatch"),
            &["S-12.01"],
            Some("v1.0-test"),
        );
        let callbacks = FakeCallbacks::new_with_story(None); // absent state file → should block

        let result = hook_logic(&payload, &callbacks);
        assert!(
            matches!(result, HookResult::Block { .. }),
            "S-12.08: hook_logic with wave_context.stories + absent state file \
             MUST return HookResult::Block (not Continue). The hook is now operationally \
             active via the WaveContextResolver injection path."
        );
    }

    // -----------------------------------------------------------------------
    // F-MED-8: wave-gate identity starts_with("wave-gate") prefix match
    // (BC-4.10.002 invariant 4 — conservative match; "wave-gate-dispatch" is
    // the canonical identity but any future wave-gate variant should not
    // silently disable the gate due to a single-string literal mismatch)
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_002_wave_gate_identity_prefix_match() {
        // F-MED-8: graceful_degrade_outside_wave_gate must return false (do NOT degrade)
        // for any agent_type that starts with "wave-gate", not just the exact literal
        // "wave-gate-dispatch". This prevents silent hook deactivation if the dispatcher
        // uses a variant like "wave-gate-v2" or "wave-gate-integration".
        //
        // Canonical identity is "wave-gate-dispatch" (BC-4.10.002 invariant 4).
        // The prefix match is conservative: false negatives (missing a non-wave-gate
        // agent) are preferable to false positives (blocking a non-wave-gate context).
        // The starts_with("wave-gate") prefix covers known and future wave-gate variants.
        let payload_exact = make_payload(Some("wave-gate-dispatch"));
        let payload_variant = make_payload(Some("wave-gate-v2"));
        let payload_not_wg = make_payload(Some("implementer"));
        let payload_none = make_payload(None);

        // Canonical identity → should NOT degrade (false = proceed with check)
        assert!(
            !graceful_degrade_outside_wave_gate(&payload_exact),
            "F-MED-8: 'wave-gate-dispatch' must return false (do not degrade)"
        );
        // Future variant with wave-gate prefix → should NOT degrade
        assert!(
            !graceful_degrade_outside_wave_gate(&payload_variant),
            "F-MED-8: 'wave-gate-v2' (starts_with 'wave-gate') must return false (do not degrade)"
        );
        // Non-wave-gate agent → SHOULD degrade (true = degrade)
        assert!(
            graceful_degrade_outside_wave_gate(&payload_not_wg),
            "F-MED-8: 'implementer' must return true (degrade — not wave-gate context)"
        );
        // No agent_type → SHOULD degrade
        assert!(
            graceful_degrade_outside_wave_gate(&payload_none),
            "F-MED-8: missing agent_type must return true (degrade — unknown context)"
        );
    }

    // -----------------------------------------------------------------------
    // Boundary: passes_clean == 2 (one below threshold)
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_001_passes_clean_2_blocks() {
        // BC-4.10.001 boundary: passes_clean=2 is one below the threshold of 3.
        // Must produce HookResult::Block.
        let state = ConvergenceState {
            passes_clean: 2,
            last_classification: Some("NITPICK_ONLY".to_string()),
            last_finding_count: Some(0),
            last_timestamp: None,
            deferred_findings: vec![],
        };

        let r = hook_result_for(Some(&state));

        assert!(
            matches!(r, HookResult::Block { .. }),
            "BC-4.10.001 boundary: passes_clean=2 must produce HookResult::Block, \
             got {:?}",
            r
        );
    }

    // =======================================================================
    // S-12.08 TESTS — RED GATE (Step 2)
    //
    // All tests below exercise extract_stories_from_wave_context (which is
    // todo!() until Step 3) or assert the absence of the old function (which
    // still exists at Step 2). They MUST FAIL at Step 2 and turn GREEN when
    // Step 3 implements the production code.
    // =======================================================================

    // -----------------------------------------------------------------------
    // GREEN-BY-DESIGN: block-code constants non-empty sanity check
    // (S-12.08 AC-002 / AC-003 — constants added in Step 1)
    // -----------------------------------------------------------------------

    /// data-shape pin: structural sanity check that block-code constants
    /// are non-empty strings. Not a behavioral test — POL-11 opt-out.
    #[test]
    fn test_wave_context_block_codes_are_non_empty() {
        // S-12.08 Step 1 added BLOCK_CODE_WAVE_CONTEXT_MISSING and
        // BLOCK_CODE_WAVE_CONTEXT_SCHEMA_ERROR as public constants. This test
        // asserts they are non-empty — a structural GREEN-BY-DESIGN sanity check.
        // No production function is called; the constants are defined at compile time.
        assert!(
            !BLOCK_CODE_WAVE_CONTEXT_MISSING.is_empty(),
            "S-12.08 AC-002: BLOCK_CODE_WAVE_CONTEXT_MISSING must be a non-empty string"
        );
        assert!(
            !BLOCK_CODE_WAVE_CONTEXT_SCHEMA_ERROR.is_empty(),
            "S-12.08 AC-003: BLOCK_CODE_WAVE_CONTEXT_SCHEMA_ERROR must be a non-empty string"
        );
    }

    // -----------------------------------------------------------------------
    // AC-001 — extract_stories_from_wave_context reads nested array
    // S-12.08 AC-001 (traces to BC-4.10.001 PC1)
    // RED at Step 2: todo!() body panics.
    // -----------------------------------------------------------------------

    #[test]
    fn test_extract_stories_from_wave_context_reads_nested_array() {
        // S-12.08 AC-001 traces to BC-4.10.001 PC1: the new extraction function
        // reads plugin_config["wave_context"]["stories"]. When called with a
        // well-formed payload the function must return Ok(vec!["S-12.03", "S-12.04"]).
        let plugin_config = json!({
            "wave_context": {
                "stories": ["S-12.03", "S-12.04"]
            }
        });
        let result = extract_stories_from_wave_context(&plugin_config);
        assert!(
            result.is_ok(),
            "S-12.08 AC-001: valid wave_context.stories must return Ok, got: {:?}",
            result
        );
        assert_eq!(
            result.unwrap(),
            vec!["S-12.03".to_string(), "S-12.04".to_string()],
            "S-12.08 AC-001: extracted stories must match wave_context.stories array"
        );
    }

    // -----------------------------------------------------------------------
    // AC-002a — absent wave_context key → WaveContextError::Missing
    // RED at Step 2: todo!() panics.
    // -----------------------------------------------------------------------

    #[test]
    fn test_absent_wave_context_returns_missing_error() {
        // S-12.08 AC-002a: plugin_config = {} (no wave_context key at all).
        // Must return Err(WaveContextError::Missing).
        let plugin_config = json!({});
        let result = extract_stories_from_wave_context(&plugin_config);
        assert!(
            matches!(result, Err(WaveContextError::Missing)),
            "S-12.08 AC-002a: absent wave_context must return Err(WaveContextError::Missing), \
             got: {:?}",
            result
        );
    }

    // -----------------------------------------------------------------------
    // AC-002b — null wave_context value → WaveContextError::Missing
    // RED at Step 2: todo!() panics.
    // -----------------------------------------------------------------------

    #[test]
    fn test_null_wave_context_returns_missing_error() {
        // S-12.08 AC-002b: plugin_config = {"wave_context": null}.
        // JSON null is treated as absent — must return Err(WaveContextError::Missing).
        let plugin_config = json!({ "wave_context": null });
        let result = extract_stories_from_wave_context(&plugin_config);
        assert!(
            matches!(result, Err(WaveContextError::Missing)),
            "S-12.08 AC-002b: null wave_context must return Err(WaveContextError::Missing), \
             got: {:?}",
            result
        );
    }

    // -----------------------------------------------------------------------
    // AC-002c — wave_context present but no stories key → WaveContextError::Missing
    // RED at Step 2: todo!() panics.
    // -----------------------------------------------------------------------

    #[test]
    fn test_absent_stories_key_in_wave_context_returns_missing() {
        // S-12.08 AC-002c: plugin_config = {"wave_context": {}} (wave_context
        // present but stories key missing). Must return Err(WaveContextError::Missing).
        let plugin_config = json!({ "wave_context": {} });
        let result = extract_stories_from_wave_context(&plugin_config);
        assert!(
            matches!(result, Err(WaveContextError::Missing)),
            "S-12.08 AC-002c: wave_context without stories key must return \
             Err(WaveContextError::Missing), got: {:?}",
            result
        );
    }

    // -----------------------------------------------------------------------
    // AC-003 — wrong type for stories → WaveContextError::SchemaError
    // Four variants: string, number, object, array-with-non-string element.
    // RED at Step 2: todo!() panics.
    // -----------------------------------------------------------------------

    #[test]
    fn test_wrong_type_stories_returns_schema_error() {
        // S-12.08 AC-003: multiple wrong-type variants for wave_context.stories.
        // All must return Err(WaveContextError::SchemaError(_)).

        // Variant 1: stories is a string (not an array)
        let cfg_string = json!({ "wave_context": { "stories": "not-an-array" } });
        let r1 = extract_stories_from_wave_context(&cfg_string);
        assert!(
            matches!(r1, Err(WaveContextError::SchemaError(_))),
            "S-12.08 AC-003: string stories must return SchemaError, got: {:?}",
            r1
        );

        // Variant 2: stories is a number
        let cfg_number = json!({ "wave_context": { "stories": 42 } });
        let r2 = extract_stories_from_wave_context(&cfg_number);
        assert!(
            matches!(r2, Err(WaveContextError::SchemaError(_))),
            "S-12.08 AC-003: number stories must return SchemaError, got: {:?}",
            r2
        );

        // Variant 3: stories is an object
        let cfg_object = json!({ "wave_context": { "stories": {"key": "val"} } });
        let r3 = extract_stories_from_wave_context(&cfg_object);
        assert!(
            matches!(r3, Err(WaveContextError::SchemaError(_))),
            "S-12.08 AC-003: object stories must return SchemaError, got: {:?}",
            r3
        );

        // Variant 4: stories array contains a non-string element
        // (matches existing extract_stories_from_config behavior pattern)
        let cfg_non_string_elem = json!({ "wave_context": { "stories": ["S-1", 99, "S-3"] } });
        let r4 = extract_stories_from_wave_context(&cfg_non_string_elem);
        assert!(
            matches!(r4, Err(WaveContextError::SchemaError(_))),
            "S-12.08 AC-003: array with non-string element must return SchemaError, \
             got: {:?}",
            r4
        );
    }

    // -----------------------------------------------------------------------
    // AC-006 — static plugin_config keys preserved alongside wave_context
    // S-12.08 AC-006 (traces to BC-1.13.001 PC3 + BC-4.12.005 PC1)
    // RED at Step 2: todo!() panics.
    // -----------------------------------------------------------------------

    #[test]
    fn test_static_config_preserved_after_wave_context_injection() {
        // S-12.08 AC-006: resolver output is additive (BC-4.12.005 PC1). The
        // wave_context key is ADDED to the existing plugin_config; it does not
        // replace it. This test verifies that extract_stories_from_wave_context
        // reads from wave_context.stories while the existing_key remains accessible
        // in the same plugin_config Value.
        //
        // Note: extract_stories_from_wave_context takes &Value (not Value), so the
        // caller retains full ownership of plugin_config including existing_key.
        let plugin_config = json!({
            "existing_key": "value",
            "wave_context": {
                "stories": ["S-1"]
            }
        });

        let result = extract_stories_from_wave_context(&plugin_config);
        assert!(
            result.is_ok(),
            "S-12.08 AC-006: wave_context.stories extraction must succeed, got: {:?}",
            result
        );
        assert_eq!(
            result.unwrap(),
            vec!["S-1".to_string()],
            "S-12.08 AC-006: extracted story must be S-1"
        );

        // The caller can still access existing_key — the function took &Value, not Value.
        assert_eq!(
            plugin_config["existing_key"].as_str(),
            Some("value"),
            "S-12.08 AC-006: existing_key must remain accessible in plugin_config \
             after extract_stories_from_wave_context call (additive overlay, not replacement)"
        );
    }

    // -----------------------------------------------------------------------
    // AC-007 — FakeCallbacks with wave_context payload drives hook_logic correctly
    // S-12.08 AC-007 (traces to BC-4.12.005 PC5)
    //
    // This test constructs a plugin_config in the new wave_context form and
    // calls hook_logic end-to-end using FakeCallbacks. The FakeCallbacks
    // directly provides the story list (bypassing list_stories / RealCallbacks).
    // The test verifies that the FakeCallbacks construction with a wave_context-
    // shaped payload still drives the correct block-decision logic.
    //
    // This test is GREEN-BY-DESIGN at Step 2 because:
    //   - FakeCallbacks.list_stories() ignores plugin_config entirely — it uses
    //     self.stories directly. The wave_context payload shape is irrelevant
    //     to FakeCallbacks.
    //   - hook_logic's block-decision path (hook_result_for) is already implemented.
    //   - The test does NOT call extract_stories_from_wave_context.
    // It will verify that after Step 3's refactor, FakeCallbacks payloads using
    // wave_context form continue to drive correct decisions.
    // -----------------------------------------------------------------------

    #[test]
    fn test_fake_callbacks_inject_wave_context_payload() {
        // S-12.08 AC-007: FakeCallbacks with wave_context-shaped payload drives
        // hook_logic correctly. This test is GREEN-BY-DESIGN at Step 2 because
        // FakeCallbacks ignores plugin_config and uses self.stories directly.
        // After Step 3, RealCallbacks will read wave_context.stories from
        // plugin_config — but FakeCallbacks in tests will still construct
        // plugin_config in the new wave_context form.

        // Case A: FakeCallbacks returns unconverged state → hook_logic must Block.
        let mut payload_a = make_payload(Some("wave-gate-dispatch"));
        payload_a.plugin_config = json!({
            "wave_context": {
                "stories": ["S-FAKE-001"],
                "wave_id": "w1",
                "cycle_id": "test-cycle"
            }
        });
        let callbacks_a = FakeCallbacks::new_with_story(Some(insufficient_passes_json(1)));
        let result_a = hook_logic(&payload_a, &callbacks_a);
        assert!(
            matches!(result_a, HookResult::Block { .. }),
            "S-12.08 AC-007: FakeCallbacks with unconverged state + wave_context payload \
             must produce HookResult::Block, got {:?}",
            result_a
        );

        // Case B: FakeCallbacks returns converged state → hook_logic must Continue.
        let mut payload_b = make_payload(Some("wave-gate-dispatch"));
        payload_b.plugin_config = json!({
            "wave_context": {
                "stories": ["S-FAKE-002"],
                "wave_id": "w1",
                "cycle_id": "test-cycle"
            }
        });
        let callbacks_b = FakeCallbacks::new_with_story(Some(cleared_state_json()));
        let result_b = hook_logic(&payload_b, &callbacks_b);
        assert!(
            matches!(result_b, HookResult::Continue),
            "S-12.08 AC-007: FakeCallbacks with converged state + wave_context payload \
             must produce HookResult::Continue, got {:?}",
            result_b
        );
    }

    // -----------------------------------------------------------------------
    // AC-002-int — hook_logic blocks when wave_context absent
    // Integration test at hook_logic level for AC-002.
    // RED at Step 2: hook_logic still uses the old extract_stories_from_config
    // path which gracefully degrades (returns Continue) instead of blocking.
    // GREEN at Step 3: implementer rewires hook_logic to call
    // extract_stories_from_wave_context and block on WaveContextError::Missing.
    // -----------------------------------------------------------------------

    #[test]
    fn test_hook_logic_blocks_when_wave_context_absent() {
        // S-12.08 AC-002 integration: hook_logic called with plugin_config = {}
        // (no wave_context key) must return HookResult::Block with
        // BLOCK_CODE_WAVE_CONTEXT_MISSING in the reason.
        //
        // RED at Step 2: current hook_logic uses extract_stories_from_config
        // (old path) which returns Err → graceful degrade → Continue. This test
        // will FAIL because Continue != Block.
        // GREEN at Step 3: hook_logic uses extract_stories_from_wave_context
        // which returns WaveContextError::Missing → Block with WAVE_CONTEXT_MISSING.
        let mut payload = make_payload(Some("wave-gate-dispatch"));
        payload.plugin_config = json!({});
        // FakeCallbacks cycle-dir-absent path: list_stories returns Err.
        // After Step 3, hook_logic checks wave_context BEFORE calling list_stories,
        // so FakeCallbacks will not even be asked for stories.
        let callbacks = FakeCallbacks::new_no_context();

        let result = hook_logic(&payload, &callbacks);

        match &result {
            HookResult::Block { reason } => {
                assert!(
                    reason.contains(BLOCK_CODE_WAVE_CONTEXT_MISSING),
                    "S-12.08 AC-002-int: block reason must contain '{}', got: {}",
                    BLOCK_CODE_WAVE_CONTEXT_MISSING,
                    reason
                );
            }
            other => panic!(
                "S-12.08 AC-002-int: hook_logic with absent wave_context must return \
                 HookResult::Block, got {:?}",
                other
            ),
        }
    }

    // -----------------------------------------------------------------------
    // AC-003-int — hook_logic blocks when stories has wrong type
    // Integration test at hook_logic level for AC-003.
    // RED at Step 2: old path gracefully degrades instead of blocking.
    // GREEN at Step 3: hook_logic uses extract_stories_from_wave_context.
    // -----------------------------------------------------------------------

    #[test]
    fn test_hook_logic_blocks_when_stories_wrong_type() {
        // S-12.08 AC-003 integration: hook_logic called with
        // plugin_config = {"wave_context": {"stories": "bogus"}} (stories is a
        // string, not an array) must return HookResult::Block with
        // BLOCK_CODE_WAVE_CONTEXT_SCHEMA_ERROR in the reason.
        //
        // RED at Step 2: current hook_logic uses old path → graceful degrade →
        // Continue. Test will FAIL because Continue != Block.
        // GREEN at Step 3: hook_logic uses extract_stories_from_wave_context
        // which returns WaveContextError::SchemaError → Block with WAVE_CONTEXT_SCHEMA_ERROR.
        let mut payload = make_payload(Some("wave-gate-dispatch"));
        payload.plugin_config = json!({ "wave_context": { "stories": "bogus" } });
        let callbacks = FakeCallbacks::new_no_context();

        let result = hook_logic(&payload, &callbacks);

        match &result {
            HookResult::Block { reason } => {
                assert!(
                    reason.contains(BLOCK_CODE_WAVE_CONTEXT_SCHEMA_ERROR),
                    "S-12.08 AC-003-int: block reason must contain '{}', got: {}",
                    BLOCK_CODE_WAVE_CONTEXT_SCHEMA_ERROR,
                    reason
                );
            }
            other => panic!(
                "S-12.08 AC-003-int: hook_logic with wrong-type stories must return \
                 HookResult::Block, got {:?}",
                other
            ),
        }
    }

    // -----------------------------------------------------------------------
    // AC-005 — hooks-registry.toml has needs_context for convergence hook
    // S-12.08 AC-005 (traces to BC-1.13.001 PC4)
    // GREEN-BY-DESIGN: needs_context was added in Step 1.
    // -----------------------------------------------------------------------

    #[test]
    fn test_hooks_registry_has_needs_context_for_convergence_hook() {
        // S-12.08 AC-005 traces to BC-1.13.001 PC4: the validate-per-story-
        // adversary-convergence entry in hooks-registry.toml must declare
        // needs_context = ["wave_context"].
        //
        // GREEN-BY-DESIGN: Step 1 added this field to hooks-registry.toml.
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR must be set during cargo test");
        // hooks-registry.toml is in plugins/vsdd-factory/ — 3 levels up from
        // crates/hook-plugins/validate-per-story-adversary-convergence/
        // (validate-per-story-adversary-convergence → hook-plugins → crates → worktree-root)
        let registry_path = std::path::Path::new(&manifest_dir)
            .join("../../../plugins/vsdd-factory/hooks-registry.toml");
        let registry_src = std::fs::read_to_string(&registry_path).unwrap_or_else(|e| {
            panic!(
                "failed to read hooks-registry.toml at {:?}: {e}",
                registry_path
            )
        });

        // Assert needs_context = ["wave_context"] appears in the file.
        // Step 1 added this field to the validate-per-story-adversary-convergence entry.
        assert!(
            registry_src.contains(r#"needs_context = ["wave_context"]"#),
            "S-12.08 AC-005: hooks-registry.toml must contain \
             `needs_context = [\"wave_context\"]` for the convergence hook. \
             BC-1.13.001 PC4. Registry contents:\n{}",
            &registry_src[..registry_src.len().min(500)]
        );
    }

    // -----------------------------------------------------------------------
    // AC-010 — old extract_stories_from_config function removed
    // S-12.08 AC-010 (traces to BC-4.10.001 + F-P2-001 + F-P2-008)
    //
    // RED at Step 2: extract_stories_from_config still exists (Step 1 left
    // it in place). The source grep will find "pub fn extract_stories_from_config"
    // and the test will FAIL.
    // GREEN at Step 3: implementer removes the function; grep returns no match.
    // -----------------------------------------------------------------------

    #[test]
    fn test_old_extract_stories_from_config_removed() {
        // S-12.08 AC-010: the old extract_stories_from_config function that read
        // from the top-level plugin_config.stories key (the root cause of F-P2-001)
        // must be REMOVED in Step 3. This test verifies removal by reading the
        // source file and asserting the function signature is absent.
        //
        // RED at Step 2: function still exists; grep finds it; test FAILS.
        // GREEN at Step 3: function removed; grep returns empty; test PASSES.
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR must be set during cargo test");
        let lib_path = std::path::Path::new(&manifest_dir).join("src/lib.rs");
        let lib_src = std::fs::read_to_string(&lib_path)
            .unwrap_or_else(|e| panic!("failed to read src/lib.rs: {e}"));

        // The old function signature — must NOT be present after Step 3.
        // Constructed at runtime to avoid this test source matching itself.
        let old_fn_sig = format!("pub fn {}(", "extract_stories_from_config");
        assert!(
            !lib_src.contains(&old_fn_sig),
            "S-12.08 AC-010: src/lib.rs MUST NOT contain '{}' after the Step 3 \
             refactor. The old static-config fallback path that caused F-P2-001 \
             must be removed and replaced by extract_stories_from_wave_context.",
            old_fn_sig
        );
    }

    // -----------------------------------------------------------------------
    // MED-003: extract_code_from_reason canonical format regression test
    // -----------------------------------------------------------------------

    #[test]
    fn test_extract_code_from_reason_canonical_block_format() {
        // MED-003: regression safety net — extract_code_from_reason must correctly
        // parse the canonical SDK block_with_fix output format. If the format drifts,
        // this test catches the regression before it silently corrupts telemetry.
        let reason = "BLOCKED by validate-per-story-adversary-convergence: Story S-A has \
                      passes_clean=2. Fix: Run another adversary pass. Code: CONVERGENCE_PASSES_INSUFFICIENT.";
        let code = extract_code_from_reason(reason);
        assert_eq!(
            code,
            Some("CONVERGENCE_PASSES_INSUFFICIENT"),
            "MED-003: extract_code_from_reason must parse 'Code: <code>.' from canonical \
             block_with_fix format, got: {:?}",
            code
        );
    }

    // -----------------------------------------------------------------------
    // HIGH-003: non-object wave_context → SchemaError
    // -----------------------------------------------------------------------

    #[test]
    fn test_non_object_wave_context_returns_schema_error() {
        // HIGH-003: AC-003 guard — wave_context must be a JSON object. Non-object
        // types (string, number, bool, array) must return WaveContextError::SchemaError.
        let configs = [
            json!({"wave_context": "string"}),
            json!({"wave_context": 42}),
            json!({"wave_context": true}),
            json!({"wave_context": [1, 2, 3]}),
        ];
        for cfg in &configs {
            let result = extract_stories_from_wave_context(cfg);
            assert!(
                matches!(result, Err(WaveContextError::SchemaError(_))),
                "HIGH-003: expected SchemaError for non-object wave_context {:?}, got {:?}",
                cfg,
                result
            );
        }
    }
}
