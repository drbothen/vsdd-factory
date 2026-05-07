//! validate-per-story-adversary-convergence — SubagentStop WASM hook plugin.
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
    todo!("S-12.02 Step 4 — implement pure convergence criterion check per BC-4.10.001 PC2-5 and VP-071")
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
    todo!("S-12.02 Step 4 — parse JSON and validate required fields are present")
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
    todo!("S-12.02 Step 4 — determine if payload indicates wave-gate dispatch context")
}

// ---------------------------------------------------------------------------
// Story listing
// ---------------------------------------------------------------------------

/// List story IDs in the current wave by reading the cycle directory.
///
/// Calls `callbacks.read_dir` (or equivalent) to enumerate story directories
/// under `.factory/cycles/<cycle-id>/`. Returns `Err(IoError)` if the cycle
/// directory cannot be read (graceful degrade caller converts this to Continue).
///
/// The injectable-callback pattern (`callbacks: &impl HookCallbacks`) allows
/// unit tests to inject fake directory listings without a WASM runtime.
///
/// # BC traces
/// - BC-4.10.001 postcondition 1 (reads state file for each story)
/// - BC-4.10.002 invariant 3 (absent cycle directory → graceful degrade)
/// - AC-004 (absent cycle dir → Continue)
pub fn list_wave_stories(
    cycle_id: &str,
    callbacks: &impl HookCallbacks,
) -> Result<Vec<String>, IoError> {
    todo!("S-12.02 Step 4 — read story dirs from .factory/cycles/<cycle-id>/")
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

    /// List story subdirectory names under `.factory/cycles/<cycle-id>/`.
    /// Returns an empty `Vec` when the directory exists but has no story dirs.
    /// Returns `Err(IoError)` when the cycle directory itself is absent.
    fn list_stories(&self, cycle_id: &str) -> Result<Vec<String>, IoError>;

    /// Log a debug/info-level advisory message (maps to `host::log_info` in
    /// the SDK — the SDK does not expose a separate `log_debug` function;
    /// BC-4.10.002 postcondition 3 references `host::log_debug` but the
    /// implemented hook uses `host::log_info` for advisory messages).
    fn log_debug(&self, msg: &str);

    /// Log an error-level message via `host::log_error`.
    fn log_error(&self, msg: &str);
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
/// 2. List wave stories via `callbacks.list_stories`. If cycle dir absent →
///    log advisory, return `HookResult::Continue`.
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
    todo!("S-12.02 Step 4 — implement full hook logic: graceful-degrade check, story enumeration, per-story convergence evaluation, deferred-findings aggregation")
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
// Unit tests (Step 3 test-writer will add tests here; stubs are red targets)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // Minimal HookCallbacks implementation for tests.
    struct FakeCallbacks {
        read_result: Option<Option<String>>,
        stories: Vec<String>,
        read_called: std::cell::Cell<bool>,
    }

    impl FakeCallbacks {
        fn new_with_story(story_json: Option<String>, stories: Vec<String>) -> Self {
            FakeCallbacks {
                read_result: Some(story_json),
                stories,
                read_called: std::cell::Cell::new(false),
            }
        }

        fn new_no_context() -> Self {
            FakeCallbacks {
                read_result: None,
                stories: vec![],
                read_called: std::cell::Cell::new(false),
            }
        }
    }

    impl HookCallbacks for FakeCallbacks {
        fn read_file(&self, _path: &str) -> Result<Option<String>, IoError> {
            self.read_called.set(true);
            match &self.read_result {
                Some(v) => Ok(v.clone()),
                None => Err(IoError("fake: no read result configured".to_string())),
            }
        }

        fn list_stories(&self, _cycle_id: &str) -> Result<Vec<String>, IoError> {
            if self.read_result.is_none() {
                return Err(IoError("fake: cycle dir absent".to_string()));
            }
            Ok(self.stories.clone())
        }

        fn log_debug(&self, _msg: &str) {}
        fn log_error(&self, _msg: &str) {}
    }

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

    // -----------------------------------------------------------------------
    // AC-001: cleared state returns Continue
    // -----------------------------------------------------------------------

    #[test]
    fn test_cleared_state_returns_continue() {
        todo!("S-12.02 Step 3 — test-writer implements; stub is red target per BC-8.29.001")
    }

    // -----------------------------------------------------------------------
    // AC-002 branch 1: missing state file → block CONVERGENCE_STATE_MISSING
    // -----------------------------------------------------------------------

    #[test]
    fn test_block_code_convergence_state_missing() {
        todo!("S-12.02 Step 3 — test-writer implements; stub is red target per BC-8.29.001")
    }

    // -----------------------------------------------------------------------
    // AC-002 branch 2: passes_clean < 3 → block CONVERGENCE_PASSES_INSUFFICIENT
    // -----------------------------------------------------------------------

    #[test]
    fn test_block_code_passes_insufficient() {
        todo!("S-12.02 Step 3 — test-writer implements; stub is red target per BC-8.29.001")
    }

    // -----------------------------------------------------------------------
    // AC-002 branch 3: non-NITPICK_ONLY → block CONVERGENCE_CLASSIFICATION_INSUFFICIENT
    // -----------------------------------------------------------------------

    #[test]
    fn test_block_code_classification_insufficient() {
        todo!("S-12.02 Step 3 — test-writer implements; stub is red target per BC-8.29.001")
    }

    // -----------------------------------------------------------------------
    // AC-003: graceful degrade — no wave-gate context
    // -----------------------------------------------------------------------

    #[test]
    fn test_graceful_degrade_no_wave_gate_context() {
        todo!("S-12.02 Step 3 — test-writer implements; stub is red target per BC-8.29.001")
    }

    // -----------------------------------------------------------------------
    // AC-003 + AC-004: read_file NOT called before context check
    // -----------------------------------------------------------------------

    #[test]
    fn test_graceful_degrade_no_read_file_before_context_check() {
        todo!("S-12.02 Step 3 — test-writer implements; stub is red target per BC-8.29.001")
    }

    // -----------------------------------------------------------------------
    // AC-004: absent cycle directory → Continue
    // -----------------------------------------------------------------------

    #[test]
    fn test_graceful_degrade_absent_cycle_dir() {
        todo!("S-12.02 Step 3 — test-writer implements; stub is red target per BC-8.29.001")
    }

    // -----------------------------------------------------------------------
    // AC-005: deferred_findings aggregated on Continue
    // -----------------------------------------------------------------------

    #[test]
    fn test_deferred_findings_aggregated_on_continue() {
        todo!("S-12.02 Step 3 — test-writer implements; stub is red target per BC-8.29.001")
    }

    // -----------------------------------------------------------------------
    // AC-006: first failure only → single block message
    // -----------------------------------------------------------------------

    #[test]
    fn test_first_failure_only_block() {
        todo!("S-12.02 Step 3 — test-writer implements; stub is red target per BC-8.29.001")
    }

    // -----------------------------------------------------------------------
    // AC-007: malformed JSON → block CONVERGENCE_STATE_MALFORMED
    // -----------------------------------------------------------------------

    #[test]
    fn test_malformed_json_block_with_malformed_code() {
        todo!("S-12.02 Step 3 — test-writer implements; stub is red target per BC-8.29.001")
    }

    // -----------------------------------------------------------------------
    // AC-008: missing last_classification field → block CONVERGENCE_STATE_SCHEMA_INVALID
    // -----------------------------------------------------------------------

    #[test]
    fn test_missing_classification_block_with_schema_invalid_code() {
        todo!("S-12.02 Step 3 — test-writer implements; stub is red target per BC-8.29.001")
    }

    // -----------------------------------------------------------------------
    // AC-009: deferred_findings do not block
    // -----------------------------------------------------------------------

    #[test]
    fn test_deferred_findings_do_not_block() {
        todo!("S-12.02 Step 3 — test-writer implements; stub is red target per BC-8.29.001")
    }

    // -----------------------------------------------------------------------
    // AC-012: no write_file calls — structural grep test
    // -----------------------------------------------------------------------

    #[test]
    fn test_no_write_file_calls() {
        todo!("S-12.02 Step 3 — test-writer implements structural grep check; stub is red target per BC-8.29.001")
    }
}
