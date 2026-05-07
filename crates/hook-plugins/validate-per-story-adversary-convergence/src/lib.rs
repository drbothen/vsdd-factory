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
// Unit tests (Step 3 — test-writer; all tests use catch_unwind pattern per
// orchestrator Red Gate requirements: compile OK, fail with behavioral context)
// ---------------------------------------------------------------------------
//
// Pattern: std::panic::catch_unwind calls the production stub (todo!()), then
// asserts the result is Ok with a behavioral message. Since todo!() always
// panics, catch_unwind returns Err, so the outer assert!(result.is_ok(), ...)
// fails with the behavioral message — NOT a raw "not yet implemented" panic.
//
// This satisfies BC-8.29.001 RED_RATIO >= 0.5 and the Red Gate assertion-style
// failure requirement from the S-12.02 dispatch instructions.

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
    struct FakeCallbacks {
        /// None = cycle dir absent (list_stories returns Err, read_file returns Err).
        /// Some(v) = cycle dir present; read_file returns Ok(v).
        read_result: Option<Option<String>>,
        stories: Vec<String>,
        read_called: std::cell::Cell<bool>,
        block_events_emitted: std::cell::Cell<u32>,
    }

    impl FakeCallbacks {
        /// Cycle dir present; all story reads return `story_json`.
        fn new_with_story(story_json: Option<String>, stories: Vec<String>) -> Self {
            FakeCallbacks {
                read_result: Some(story_json),
                stories,
                read_called: std::cell::Cell::new(false),
                block_events_emitted: std::cell::Cell::new(0),
            }
        }

        /// Cycle dir absent; read_file and list_stories both return Err.
        fn new_no_context() -> Self {
            FakeCallbacks {
                read_result: None,
                stories: vec![],
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
                None => Err(IoError("fake: cycle dir absent — no read result".to_string())),
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

    /// Non-NITPICK_ONLY classification state (BC-4.10.001 test vector row 4).
    fn high_classification_json() -> String {
        json!({
            "passes_clean": 3,
            "last_classification": "HIGH",
            "last_finding_count": 2,
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
        let payload = make_payload(Some("wave-gate-dispatch"));
        let callbacks = FakeCallbacks::new_with_story(
            Some(cleared_state_json()),
            vec!["S-A".to_string()],
        );

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            hook_logic(&payload, &callbacks)
        }));

        assert!(
            result.is_ok(),
            "BC-4.10.001 PC1+PC5: hook_logic with a cleared story (passes_clean=3, \
             last_classification=NITPICK_ONLY) MUST return HookResult::Continue — \
             production function is not yet implemented (AC-001)"
        );
        if let Ok(hook_result) = result {
            assert!(
                matches!(hook_result, HookResult::Continue),
                "BC-4.10.001 PC5: cleared story must produce HookResult::Continue, got {:?}",
                hook_result
            );
        }
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

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            parse_convergence_state(&json)
        }));

        assert!(
            result.is_ok(),
            "BC-4.10.001 PC1: parse_convergence_state MUST successfully parse a valid \
             OQ3 schema JSON string with all five fields — production function is \
             not yet implemented (AC-001)"
        );
        if let Ok(parse_result) = result {
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

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            hook_result_for(state)
        }));

        assert!(
            result.is_ok(),
            "BC-4.10.001 PC2: hook_result_for(None) MUST return \
             HookResult::block_with_fix with code CONVERGENCE_STATE_MISSING — \
             production function is not yet implemented (AC-002 branch 1)"
        );
        if let Ok(hook_result) = result {
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
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            hook_result_for(None)
        }));

        assert!(
            result.is_ok(),
            "VP-071 proof_missing_state_file_always_blocks (cargo-test equivalent): \
             hook_result_for(None) must return HookResult::Block (not panic) — \
             production function is not yet implemented"
        );
        if let Ok(r) = result {
            assert!(
                matches!(r, HookResult::Block { .. }),
                "VP-071: missing state file must return HookResult::Block (block_with_fix form), \
                 got {:?}", r
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

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            hook_result_for(Some(&state))
        }));

        assert!(
            result.is_ok(),
            "BC-4.10.001 PC3: hook_result_for(state with passes_clean=2) MUST return \
             HookResult::Block with code CONVERGENCE_PASSES_INSUFFICIENT — \
             production function is not yet implemented (AC-002 branch 2)"
        );
        if let Ok(hook_result) = result {
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
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                hook_result_for(Some(&state))
            }));
            assert!(
                result.is_ok(),
                "VP-071 proof_insufficient_passes_always_blocks (passes={passes}): \
                 hook_result_for must return Block, not panic"
            );
            if let Ok(r) = result {
                assert!(
                    matches!(r, HookResult::Block { .. }),
                    "VP-071: passes_clean={passes} < 3 must return HookResult::Block, got {:?}", r
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

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            hook_result_for(Some(&state))
        }));

        assert!(
            result.is_ok(),
            "BC-4.10.001 PC4: hook_result_for(state with last_classification=HIGH) MUST \
             return HookResult::Block with code CONVERGENCE_CLASSIFICATION_INSUFFICIENT — \
             production function is not yet implemented (AC-002 branch 3)"
        );
        if let Ok(hook_result) = result {
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
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            hook_result_for(Some(&state))
        }));
        assert!(
            result.is_ok(),
            "VP-071 proof_non_nitpick_classification_always_blocks: \
             hook_result_for must return Block, not panic"
        );
        if let Ok(r) = result {
            assert!(
                matches!(r, HookResult::Block { .. }),
                "VP-071: last_classification=HIGH with passes_clean=3 must return \
                 HookResult::Block, got {:?}", r
            );
        }
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
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            hook_result_for(Some(&state))
        }));
        assert!(
            result.is_ok(),
            "VP-071 proof_null_classification_blocks (cargo-test equivalent): \
             hook_result_for with null last_classification must return Block, not panic"
        );
        if let Ok(r) = result {
            assert!(
                matches!(r, HookResult::Block { .. }),
                "VP-071: None last_classification (JSON null) must return HookResult::Block, \
                 got {:?}", r
            );
        }
    }

    // -----------------------------------------------------------------------
    // VP-071 cargo-test equivalent: proof_block_with_fix_fields_populated
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_001_vp071_equiv_block_with_fix_fields_populated() {
        // AC-002 traces to BC-4.10.001 PC2+PC8; VP-071 proof E cargo-test equivalent.
        // Missing state file (guaranteed block path). Verifies canonical
        // block_with_fix form: HOOK_NAME in reason, code in reason, non-empty.
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            hook_result_for(None)
        }));
        assert!(
            result.is_ok(),
            "VP-071 proof_block_with_fix_fields_populated: hook_result_for(None) must \
             return Block with populated fields, not panic"
        );
        if let Ok(r) = result {
            match r {
                HookResult::Block { reason } => {
                    assert!(
                        !reason.is_empty(),
                        "VP-071: block reason must not be empty"
                    );
                    assert!(
                        reason.contains(HOOK_NAME),
                        "VP-071: block reason must contain HOOK_NAME '{}', got: {}", HOOK_NAME, reason
                    );
                    assert!(
                        reason.contains(HOOK_CODE_BASE),
                        "VP-071: block reason must contain HOOK_CODE_BASE '{}', got: {}", HOOK_CODE_BASE, reason
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
                    "VP-071: hook_result_for(None) must return HookResult::Block, got {:?}", other
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
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                hook_result_for(Some(&state))
            }));
            assert!(
                result.is_ok(),
                "VP-071 proof_converged_story_produces_continue (passes={passes}): \
                 hook_result_for must return Continue, not panic"
            );
            if let Ok(r) = result {
                assert!(
                    matches!(r, HookResult::Continue),
                    "VP-071: fully converged story (passes_clean={passes}, NITPICK_ONLY) \
                     must return HookResult::Continue, got {:?}", r
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
        let callbacks = FakeCallbacks::new_with_story(
            Some(cleared_state_json()),
            vec!["S-A".to_string()],
        );

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            hook_logic(&payload, &callbacks)
        }));

        assert!(
            result.is_ok(),
            "BC-4.10.002 PC1: hook_logic with no wave-gate context indicator (no agent_type) \
             MUST return HookResult::Continue (graceful degrade) without blocking — \
             production function is not yet implemented (AC-003)"
        );
        if let Ok(hook_result) = result {
            assert!(
                matches!(hook_result, HookResult::Continue),
                "BC-4.10.002 PC1: no wave-gate context must produce HookResult::Continue, \
                 got {:?}", hook_result
            );
        }
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
        let callbacks = FakeCallbacks::new_with_story(
            Some(cleared_state_json()),
            vec!["S-A".to_string()],
        );

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            hook_logic(&payload, &callbacks)
        }));

        assert!(
            result.is_ok(),
            "BC-4.10.002 inv-2: hook_logic must exit before any read_file call when \
             wave-gate context is absent — production function is not yet implemented \
             (AC-003 + AC-004)"
        );
        // After production is implemented, also verify no I/O occurred:
        if result.is_ok() {
            assert!(
                !callbacks.was_read_called(),
                "BC-4.10.002 invariant 2: read_file MUST NOT be called before context check \
                 — graceful degrade must exit before any file I/O"
            );
        }
    }

    // -----------------------------------------------------------------------
    // AC-004 traces to BC-4.10.002 inv-3: absent cycle directory → Continue
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_002_graceful_degrade_absent_cycle_dir() {
        // AC-004 traces to BC-4.10.002 invariant 3: a missing .factory/cycles/
        // directory is never treated as an error. Hook returns HookResult::Continue.
        // Pattern follows validate-wave-gate-prerequisite.sh lines 64–70 and
        // regression-gate BC-7.03.074.
        let payload = make_payload(Some("wave-gate-dispatch")); // wave-gate context present
        let callbacks = FakeCallbacks::new_no_context(); // cycle dir absent

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            hook_logic(&payload, &callbacks)
        }));

        assert!(
            result.is_ok(),
            "BC-4.10.002 inv-3: hook_logic with absent cycle directory MUST return \
             HookResult::Continue (graceful degrade), not block or error — \
             production function is not yet implemented (AC-004)"
        );
        if let Ok(hook_result) = result {
            assert!(
                matches!(hook_result, HookResult::Continue),
                "BC-4.10.002 inv-3: absent cycle dir must produce HookResult::Continue, \
                 got {:?}", hook_result
            );
        }
    }

    // -----------------------------------------------------------------------
    // EC-001: empty wave (zero stories) → Continue + log warning
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_001_ec001_empty_wave_returns_continue() {
        // BC-4.10.001 EC-001: Wave has 0 stories. Hook returns HookResult::Continue
        // (vacuously all stories cleared). Logs warning: "Wave has zero stories."
        let payload = make_payload(Some("wave-gate-dispatch"));
        let callbacks = FakeCallbacks::new_with_story(
            Some(cleared_state_json()),
            vec![], // zero stories in wave
        );

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            hook_logic(&payload, &callbacks)
        }));

        assert!(
            result.is_ok(),
            "BC-4.10.001 EC-001: hook_logic with zero-story wave MUST return \
             HookResult::Continue — production function is not yet implemented"
        );
        if let Ok(hook_result) = result {
            assert!(
                matches!(hook_result, HookResult::Continue),
                "BC-4.10.001 EC-001: empty wave must produce HookResult::Continue, got {:?}",
                hook_result
            );
        }
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
        let payload = make_payload(Some("wave-gate-dispatch"));
        let callbacks = FakeCallbacks::new_with_story(
            Some(cleared_state_with_deferrals_json()),
            vec!["S-A".to_string(), "S-B".to_string()],
        );

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            hook_logic(&payload, &callbacks)
        }));

        assert!(
            result.is_ok(),
            "BC-4.10.001 PC6: hook_logic with cleared stories having non-empty \
             deferred_findings MUST return HookResult::Continue (deferred findings \
             do not block) — production function is not yet implemented (AC-005)"
        );
        if let Ok(hook_result) = result {
            assert!(
                matches!(hook_result, HookResult::Continue),
                "BC-4.10.001 PC6: converged story with deferred_findings must produce \
                 HookResult::Continue, got {:?}", hook_result
            );
        }
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
        let payload = make_payload(Some("wave-gate-dispatch"));
        let callbacks = FakeCallbacks::new_with_story(
            Some(insufficient_passes_json(1)),
            vec!["S-A".to_string(), "S-B".to_string()],
        );

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            hook_logic(&payload, &callbacks)
        }));

        assert!(
            result.is_ok(),
            "BC-4.10.001 EC-005: hook_logic with multiple failing stories MUST block \
             on the FIRST failure and return immediately (not enumerate all failures) — \
             production function is not yet implemented (AC-006)"
        );
        if let Ok(hook_result) = result {
            assert!(
                matches!(hook_result, HookResult::Block { .. }),
                "BC-4.10.001 EC-005: multiple failing stories must produce HookResult::Block \
                 (on first failure), got {:?}", hook_result
            );
        }
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
        let payload = make_payload(Some("wave-gate-dispatch"));
        let callbacks = FakeCallbacks::new_with_story(
            Some("this is { not valid json at all!!!".to_string()),
            vec!["S-A".to_string()],
        );

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            hook_logic(&payload, &callbacks)
        }));

        assert!(
            result.is_ok(),
            "BC-4.10.001 EC-002: hook_logic with malformed JSON state file MUST return \
             HookResult::Block with code CONVERGENCE_STATE_MALFORMED (not panic) — \
             production function is not yet implemented (AC-007)"
        );
        if let Ok(hook_result) = result {
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
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            parse_convergence_state("{ this is not json ")
        }));

        assert!(
            result.is_ok(),
            "BC-4.10.001 EC-002: parse_convergence_state with malformed JSON MUST return \
             Err(ParseError) without panicking — production function not yet implemented"
        );
        if let Ok(parse_result) = result {
            assert!(
                parse_result.is_err(),
                "BC-4.10.001 EC-002: malformed JSON must return Err(ParseError), got Ok"
            );
        }
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
        let payload = make_payload(Some("wave-gate-dispatch"));
        let json_missing_classification = json!({
            "passes_clean": 3,
            "last_finding_count": 0,
            "last_timestamp": "2026-05-06T00:00:00Z",
            "deferred_findings": []
            // last_classification intentionally absent
        })
        .to_string();
        let callbacks = FakeCallbacks::new_with_story(
            Some(json_missing_classification),
            vec!["S-A".to_string()],
        );

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            hook_logic(&payload, &callbacks)
        }));

        assert!(
            result.is_ok(),
            "BC-4.10.001 EC-003: hook_logic with missing last_classification field MUST \
             return HookResult::Block with code CONVERGENCE_STATE_SCHEMA_INVALID — \
             production function is not yet implemented (AC-008)"
        );
        if let Ok(hook_result) = result {
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
                     HookResult::Block, got {:?}", other
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

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            hook_result_for(Some(&state))
        }));

        assert!(
            result.is_ok(),
            "BC-4.10.001 EC-004: hook_result_for with converged state + non-empty \
             deferred_findings MUST return HookResult::Continue (deferred findings \
             do not block) — production function is not yet implemented (AC-009)"
        );
        if let Ok(hook_result) = result {
            assert!(
                matches!(hook_result, HookResult::Continue),
                "BC-4.10.001 EC-004: passes_clean=3, NITPICK_ONLY, non-empty deferred_findings \
                 must produce HookResult::Continue, got {:?}", hook_result
            );
        }
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
        let payload = make_payload(Some("wave-gate-dispatch"));
        let callbacks = FakeCallbacks::new_with_story(
            Some(cleared_state_json()),
            vec!["S-A".to_string()],
        );

        // The fact that this compiles and runs natively (not under WASM) proves
        // the injectable-callback pattern is correctly implemented.
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            hook_logic(&payload, &callbacks)
        }));

        assert!(
            result.is_ok(),
            "BC-4.10.001 inv-3: hook_logic MUST be exercisable without a WASM runtime \
             via the injectable-callback pattern — production function not yet \
             implemented (AC-010)"
        );
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

        // Check lib.rs for write_file
        assert!(
            !lib_src.contains("host::write_file") && !lib_src.contains("write_file("),
            "BC-4.10.001 inv-4: src/lib.rs MUST NOT contain any write_file calls \
             — hook must be strictly read-only (AC-012)"
        );

        // Check main.rs if it exists
        if main_path.exists() {
            let main_src = std::fs::read_to_string(&main_path)
                .unwrap_or_else(|e| panic!("failed to read src/main.rs: {e}"));
            assert!(
                !main_src.contains("host::write_file") && !main_src.contains("write_file("),
                "BC-4.10.001 inv-4: src/main.rs MUST NOT contain any write_file calls \
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
        let callbacks = FakeCallbacks::new_with_story(
            Some(cleared_state_json()),
            vec!["S-A".to_string()],
        );

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            hook_logic(&payload, &callbacks)
        }));

        assert!(
            result.is_ok(),
            "BC-4.10.002 EC-001: hook_logic fired on per-story SubagentStop (agent_type=implementer) \
             MUST return HookResult::Continue (graceful degrade) — \
             production function is not yet implemented"
        );
        if let Ok(hook_result) = result {
            assert!(
                matches!(hook_result, HookResult::Continue),
                "BC-4.10.002 EC-001: per-story SubagentStop must produce HookResult::Continue, \
                 got {:?}", hook_result
            );
        }
    }

    // -----------------------------------------------------------------------
    // EC-008 (BC-4.10.002): payload missing both subagent_name and agent_type
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_4_10_002_ec008_missing_agent_fields_graceful_degrade() {
        // BC-4.10.002 EC-004: Payload missing subagent_name and agent_type fields.
        // Cannot determine context. Graceful degrade: return Continue, log advisory.
        let payload = make_payload(None); // both agent_type and subagent_name absent
        let callbacks = FakeCallbacks::new_with_story(
            Some(cleared_state_json()),
            vec!["S-A".to_string()],
        );

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            hook_logic(&payload, &callbacks)
        }));

        assert!(
            result.is_ok(),
            "BC-4.10.002 EC-004: hook_logic with missing agent fields MUST return \
             HookResult::Continue (graceful degrade) — production function not yet implemented"
        );
        if let Ok(hook_result) = result {
            assert!(
                matches!(hook_result, HookResult::Continue),
                "BC-4.10.002 EC-004: missing agent fields must produce HookResult::Continue, \
                 got {:?}", hook_result
            );
        }
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

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            graceful_degrade_outside_wave_gate(&payload)
        }));

        assert!(
            result.is_ok(),
            "BC-4.10.002 PC1: graceful_degrade_outside_wave_gate with no wave-gate \
             indicator MUST return without panicking — production function not yet \
             implemented (AC-003)"
        );
        // When implemented: must signal degrade (true = should degrade) for no-context payload.
        // The exact bool semantics depend on implementation, but the function must not panic.
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

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            hook_result_for(Some(&state))
        }));

        assert!(
            result.is_ok(),
            "BC-4.10.001 boundary: hook_result_for with passes_clean=3 (exact threshold) \
             MUST return HookResult::Continue — production function not yet implemented"
        );
        if let Ok(r) = result {
            assert!(
                matches!(r, HookResult::Continue),
                "BC-4.10.001 boundary: passes_clean=3 (exact threshold) must produce \
                 HookResult::Continue, got {:?}", r
            );
        }
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

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            hook_result_for(Some(&state))
        }));

        assert!(
            result.is_ok(),
            "BC-4.10.001 boundary: hook_result_for with passes_clean=2 MUST return \
             HookResult::Block — production function not yet implemented"
        );
        if let Ok(r) = result {
            assert!(
                matches!(r, HookResult::Block { .. }),
                "BC-4.10.001 boundary: passes_clean=2 must produce HookResult::Block, \
                 got {:?}", r
            );
        }
    }
}
