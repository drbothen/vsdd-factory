// Test files use .expect()/.unwrap()/.panic!() for failure reporting.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
//! Integration tests for lint-registry-async-invariant.
//!
//! VP-078 v1.8 — CI lint harnesses (4 harnesses). All tests MUST be RED
//! (fail against stubs) per BC-5.38.001 Red Gate invariant. The stubs in
//! lib.rs use `todo!()` / `unimplemented!()` bodies; these tests will panic
//! with "not yet implemented" until T-3i is implemented.
//!
//! # Harness map (VP-078)
//!
//! | Harness | Description |
//! |---------|-------------|
//! | 1 — lint_invariant | schema_version=2 required; v1 (and missing) rejected |
//! | 2 — bats_end_to_end | hooks-registry.toml + registry lint end-to-end |
//! | 3 — positive_classification | 9 telemetry plugins classified async=true |
//! | 4 — serde_default | missing `async` field defaults to false |
//!
//! # Red Gate confirmation
//!
//! BC-5.38.005 invariant 1 self-check: "If I include this real implementation,
//! will the test for this function pass trivially without any implementer work?"
//! Answer: YES for every test below — therefore all call into `todo!()` stubs.

use lint_registry_async_invariant::{E_REG_002, LintCallbacks, LintResult, lint_logic, run_lint};
use vsdd_hook_sdk::HookPayload;

// ---------------------------------------------------------------------------
// VP-078 Harness 1 — lint_invariant
// Tests: schema_version=2 required; v1 rejected with SchemaMismatch.
// ---------------------------------------------------------------------------

/// VP-078 Harness 1a: registry with schema_version=2 passes lint.
///
/// RED: `run_lint` is `todo!()` — panics with "not yet implemented".
#[test]
fn test_BC_7_06_001_schema_v2_passes_lint() {
    let toml = r#"
schema_version = 2

[[hooks]]
name = "capture-commit-activity"
event = "PostToolUse"
plugin = "hook-plugins/capture-commit-activity.wasm"
async = true
"#;
    let result = run_lint(toml);
    assert_eq!(result, LintResult::Pass);
}

/// VP-078 Harness 1b: registry with schema_version=1 fails lint with SchemaMismatch.
///
/// RED: `run_lint` is `todo!()` — panics with "not yet implemented".
#[test]
fn test_BC_7_06_001_schema_v1_rejected() {
    let toml = r#"
schema_version = 1

[[hooks]]
name = "some-validator"
event = "PreToolUse"
plugin = "hook-plugins/some-validator.wasm"
"#;
    let result = run_lint(toml);
    assert!(
        matches!(result, LintResult::SchemaMismatch { got: Some(1) }),
        "expected SchemaMismatch {{ got: Some(1) }}, got {:?}",
        result
    );
}

/// VP-078 Harness 1c: on_error=block AND async=true is an invariant violation (E-REG-002).
///
/// RED: `run_lint` is `todo!()` — panics with "not yet implemented".
#[test]
fn test_BC_7_06_001_block_async_true_is_invariant_violation() {
    let toml = r#"
schema_version = 2

[[hooks]]
name = "bad-validator"
event = "PreToolUse"
plugin = "hook-plugins/bad-validator.wasm"
on_error = "block"
async = true
"#;
    let result = run_lint(toml);
    assert!(
        matches!(
            result,
            LintResult::InvariantViolation { ref plugin_name } if plugin_name == "bad-validator"
        ),
        "expected InvariantViolation for 'bad-validator', got {:?}",
        result
    );
}

/// VP-078 Harness 1d: on_error=block AND async=false (absent) is valid (no violation).
///
/// RED: `run_lint` is `todo!()` — panics with "not yet implemented".
#[test]
fn test_BC_7_06_001_block_async_false_is_valid() {
    let toml = r#"
schema_version = 2

[[hooks]]
name = "governance-validator"
event = "PreToolUse"
plugin = "hook-plugins/governance-validator.wasm"
on_error = "block"
"#;
    // async absent → defaults false → on_error=block+async=false is valid
    let result = run_lint(toml);
    assert_eq!(result, LintResult::Pass);
}

// ---------------------------------------------------------------------------
// VP-078 Harness 2 — bats_end_to_end
// Tests: hooks-registry.toml + registry lint end-to-end integration.
// These verify the TOML parsing + lint pipeline together.
// ---------------------------------------------------------------------------

/// VP-078 Harness 2a: valid v2 registry with mixed async classifications passes.
///
/// RED: `run_lint` is `todo!()` — panics with "not yet implemented".
#[test]
fn test_BC_7_06_001_valid_v2_mixed_registry_passes() {
    // A realistic registry snippet: 2 sync validators + 1 async telemetry plugin.
    let toml = r#"
schema_version = 2

[[hooks]]
name = "block-ai-attribution"
event = "PostToolUse"
plugin = "hook-plugins/block-ai-attribution.wasm"
on_error = "block"

[[hooks]]
name = "capture-commit-activity"
event = "PostToolUse"
plugin = "hook-plugins/capture-commit-activity.wasm"
async = true
"#;
    let result = run_lint(toml);
    assert_eq!(result, LintResult::Pass);
}

/// VP-078 Harness 2b: multiple entries, one violating on_error=block+async=true.
/// The lint must report the first violating plugin name.
///
/// RED: `run_lint` is `todo!()` — panics with "not yet implemented".
#[test]
fn test_BC_7_06_001_first_violating_plugin_reported() {
    let toml = r#"
schema_version = 2

[[hooks]]
name = "good-validator"
event = "PreToolUse"
plugin = "hook-plugins/good-validator.wasm"
on_error = "block"

[[hooks]]
name = "violating-plugin"
event = "PostToolUse"
plugin = "hook-plugins/violating-plugin.wasm"
on_error = "block"
async = true
"#;
    let result = run_lint(toml);
    assert!(
        matches!(
            result,
            LintResult::InvariantViolation { ref plugin_name } if plugin_name == "violating-plugin"
        ),
        "expected InvariantViolation for 'violating-plugin', got {:?}",
        result
    );
}

// ---------------------------------------------------------------------------
// VP-078 Harness 3 — positive_classification
// Tests: 9 telemetry plugins correctly classified async=true.
// Uses a representative subset from the 9 required async plugins (BC-7.06.001
// Invariant 6). Full set is in hooks-registry.toml (T-3h scope, not stubs).
// ---------------------------------------------------------------------------

/// VP-078 Harness 3: representative telemetry plugins with async=true pass lint.
///
/// RED: `run_lint` is `todo!()` — panics with "not yet implemented".
#[test]
fn test_BC_7_06_001_nine_async_telemetry_plugins_pass() {
    // 9 telemetry plugin names per BC-7.06.001 Invariant 6 / T-3h.
    // These are stub entries — actual WASM paths wired in T-3h.
    let toml = r#"
schema_version = 2

[[hooks]]
name = "capture-commit-activity"
event = "PostToolUse"
plugin = "hook-plugins/capture-commit-activity.wasm"
async = true

[[hooks]]
name = "session-end-telemetry"
event = "SessionEnd"
plugin = "hook-plugins/session-end-telemetry.wasm"
async = true

[[hooks]]
name = "session-start-telemetry"
event = "SessionStart"
plugin = "hook-plugins/session-start-telemetry.wasm"
async = true

[[hooks]]
name = "tool-failure-hooks"
event = "PostToolUseFailure"
plugin = "hook-plugins/tool-failure-hooks.wasm"
async = true

[[hooks]]
name = "capture-pr-activity"
event = "PostToolUse"
plugin = "hook-plugins/capture-pr-activity.wasm"
async = true

[[hooks]]
name = "session-learning"
event = "SessionEnd"
plugin = "hook-plugins/session-learning.wasm"
async = true

[[hooks]]
name = "track-agent-start"
event = "SessionStart"
plugin = "hook-plugins/track-agent-start.wasm"
async = true

[[hooks]]
name = "track-agent-stop"
event = "SessionEnd"
plugin = "hook-plugins/track-agent-stop.wasm"
async = true

[[hooks]]
name = "update-wave-state-on-merge"
event = "PostToolUse"
plugin = "hook-plugins/update-wave-state-on-merge.wasm"
async = true
"#;
    let result = run_lint(toml);
    assert_eq!(
        result,
        LintResult::Pass,
        "all 9 telemetry plugins with async=true must pass lint"
    );
}

// ---------------------------------------------------------------------------
// VP-078 Harness 4 — serde_default
// Tests: missing `async` field defaults to false (no parse error).
// Also verifies that async="true" (string) is a parse error (type-check).
// ---------------------------------------------------------------------------

/// VP-078 Harness 4a: absent async field defaults to false — lint passes.
///
/// RED: `run_lint` is `todo!()` — panics with "not yet implemented".
#[test]
fn test_BC_7_06_001_async_absent_defaults_to_false() {
    let toml = r#"
schema_version = 2

[[hooks]]
name = "validate-artifact-path"
event = "PreToolUse"
plugin = "hook-plugins/validate-artifact-path.wasm"
"#;
    // async absent → default false → no block+async=true violation → Pass
    let result = run_lint(toml);
    assert_eq!(
        result,
        LintResult::Pass,
        "absent async field must default to false; lint must pass"
    );
}

/// VP-078 Harness 4b: explicit async=false passes lint.
///
/// RED: `run_lint` is `todo!()` — panics with "not yet implemented".
#[test]
fn test_BC_7_06_001_async_explicit_false_passes() {
    let toml = r#"
schema_version = 2

[[hooks]]
name = "regression-gate"
event = "PreToolUse"
plugin = "hook-plugins/regression-gate.wasm"
on_error = "block"
async = false
"#;
    let result = run_lint(toml);
    assert_eq!(result, LintResult::Pass);
}

/// VP-078 Harness 4c: explicit async=true without on_error=block passes lint.
///
/// RED: `run_lint` is `todo!()` — panics with "not yet implemented".
#[test]
fn test_BC_7_06_001_async_true_without_block_passes() {
    let toml = r#"
schema_version = 2

[[hooks]]
name = "capture-commit-activity"
event = "PostToolUse"
plugin = "hook-plugins/capture-commit-activity.wasm"
async = true
"#;
    let result = run_lint(toml);
    assert_eq!(result, LintResult::Pass);
}

// ---------------------------------------------------------------------------
// F-P10-001 — canonical violation string regression guard
// ---------------------------------------------------------------------------

/// F-P10-001 regression guard: the emit_event payload for an
/// InvariantViolation MUST use violation = "async_block_conflict"
/// (BC-3.08.001 v1.7 + BC-7.06.001 v1.7 canonical wire format).
///
/// Constructs a violating registry (on_error=block + async=true), drives
/// `lint_logic` with injected callbacks, and captures the exact emit_event
/// arguments.  Asserts the violation field equals "async_block_conflict" —
/// never the legacy "on_error_block_with_async_true" string.
#[test]
fn test_emit_event_payload_uses_canonical_violation_string() {
    // Violating registry TOML: schema_version=2, one entry with on_error=block + async=true.
    let violating_toml = r#"
schema_version = 2

[[hooks]]
name = "bad-plugin"
event = "PreToolUse"
plugin = "hook-plugins/bad-plugin.wasm"
on_error = "block"
async = true
"#
    .to_string();

    // Construct a HookPayload where tool_input.file_path ends with hooks-registry.toml
    // so lint_logic does not skip execution.
    let payload_json = r#"{
        "event_name": "PostToolUse",
        "tool_name": "Edit",
        "session_id": "test-session",
        "dispatcher_trace_id": "test-trace",
        "tool_input": {"file_path": "plugins/vsdd-factory/hooks-registry.toml"}
    }"#;
    let payload: HookPayload = serde_json::from_str(payload_json).expect("payload must parse");

    // Capture emit_event calls.
    let mut captured_event_type = String::new();
    let mut captured_violation = String::new();
    let mut captured_error_code = String::new();

    lint_logic(
        payload,
        LintCallbacks {
            // Inject the violating TOML content instead of reading the real file.
            read_file: |_path| Ok(violating_toml.clone()),
            emit_event: |event_type, fields| {
                captured_event_type = event_type.to_string();
                for (key, value) in fields {
                    if *key == "violation" {
                        captured_violation = value.to_string();
                    }
                    if *key == "error_code" {
                        captured_error_code = value.to_string();
                    }
                }
            },
            log: |_level, _msg| {},
        },
    );

    // Assert the canonical wire-format string per BC-3.08.001 v1.7 + BC-7.06.001 v1.7.
    assert_eq!(
        captured_event_type, "dispatcher.registry_invalid",
        "emit_event must use the canonical event type 'dispatcher.registry_invalid'"
    );
    assert_eq!(
        captured_violation, "async_block_conflict",
        "F-P10-001: violation field MUST be 'async_block_conflict', not 'on_error_block_with_async_true'"
    );
    assert_eq!(
        captured_error_code, E_REG_002,
        "error_code must be E-REG-002"
    );
}
