//! regression-gate — PostToolUse WASM hook plugin.
//!
//! Records test-run pass/fail state to `.factory/regression-state.json`.
//! Fires on every PostToolUse event; body-filters to Bash tool commands only;
//! further filters to 9 test-runner patterns; derives pass/fail from the
//! PostToolUse envelope; writes state file; warns on pass-to-fail transition.
//!
//! # Behavioral Contracts
//!
//! - BC-7.03.071: identity & registry binding (PostToolUse, priority=230,
//!   on_error=continue). Exit 0 always.
//! - BC-7.03.072: matches 9 test runners by substring:
//!   `cargo test`, `cargo nextest`, `pytest`, `pnpm test`, `npm test`,
//!   `go test`, `just test`, `just ci`, `yarn test`.
//!   Non-Bash tools → exit 0. Non-matching Bash commands → exit 0.
//! - BC-7.03.073: pass/fail derivation cascade (priority: exit_code > interrupted):
//!   1. exit_code == 0 → pass
//!   2. exit_code non-zero & non-null → fail
//!   3. interrupted == true → fail
//!   4. interrupted == false → pass
//!   5. neither → STATUS=unknown → skip (exit 0, no state write)
//! - BC-7.03.074: writes `.factory/regression-state.json` with
//!   `{status, timestamp, command}` (ISO-8601 UTC). `.factory/` absent → exit 0.
//! - BC-7.03.075: warns on pass→fail transition: `host::emit_event`
//!   `hook.block severity=warn` + stderr message. Exit 0 always.
//!
//! # Architecture compliance
//!
//! - `bin/emit-event` → replaced by `host::emit_event` (AC-008).
//!   `bin/emit-event` is NOT removed (E-8 D-10; deferred to S-8.29).
//! - No jq subprocess (AC-009 preferred profile confirmed by OQ-6 audit).
//! - `binary_allow = []` (empty) — no subprocesses needed.
//! - Capabilities: `read_file`, `write_file`, `emit_event`.
//! - No dependency on `legacy-bash-adapter` (forbidden per E-8 D-10).
//! - HOST_ABI_VERSION = 1 unchanged.

use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// State file path constant
// ---------------------------------------------------------------------------

/// Path to the regression state file (relative to CLAUDE_PROJECT_DIR).
/// Must be a literal — NOT derived from any env var or input field (SEC-002).
pub const STATE_FILE_PATH: &str = ".factory/regression-state.json";


// ---------------------------------------------------------------------------
// Test runner pattern matching (BC-7.03.072)
// ---------------------------------------------------------------------------

/// The 9 test-runner substrings that trigger this hook body.
///
/// Pattern matching is substring `contains` — same semantics as the bash
/// `case` glob `*"pattern"*`. Order mirrors the bash `case` statement.
/// First-match-wins semantics per EC-007.
pub const TEST_RUNNER_PATTERNS: &[&str] = &[
    "cargo test",
    "cargo nextest",
    "pytest",
    "pnpm test",
    "npm test",
    "go test",
    "just test",
    "just ci",
    "yarn test",
];

/// Returns `true` if the command matches any of the 9 test-runner patterns.
///
/// Substring `contains` semantics match the bash `case *"pattern"*` glob.
/// Per EC-007: first matching pattern wins (patterns are independent — no
/// exclusive-OR relationship between them).
///
/// # BC trace
/// BC-7.03.072 postcondition 1.
pub fn is_test_runner_command(command: &str) -> bool {
    TEST_RUNNER_PATTERNS.iter().any(|p| command.contains(p))
}

// ---------------------------------------------------------------------------
// Status derivation (BC-7.03.073)
// ---------------------------------------------------------------------------

/// Derived test run status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunStatus {
    Pass,
    Fail,
    /// Neither exit_code nor interrupted were determinable — skip state write.
    Unknown,
}

/// Derive pass/fail status from the PostToolUse `tool_response` JSON.
///
/// 5-step cascade (BC-7.03.073 postcondition 1):
/// 1. `exit_code == 0` (or `returncode == 0`) → Pass
/// 2. `exit_code` non-zero, non-null → Fail
/// 3. `interrupted == true` → Fail
/// 4. `interrupted == false` → Pass
/// 5. neither determinable → Unknown
///
/// # BC trace
/// BC-7.03.073 postcondition 1.
pub fn derive_status(tool_response: &serde_json::Value) -> RunStatus {
    // Step 1 & 2: prefer exit_code / returncode (back-compat field)
    let exit_code = tool_response
        .get("exit_code")
        .or_else(|| tool_response.get("returncode"));

    if let Some(code) = exit_code {
        if code.is_null() {
            // Null exit_code: not determinable from this field; fall through to interrupted
        } else if let Some(n) = code.as_i64() {
            return if n == 0 { RunStatus::Pass } else { RunStatus::Fail };
        }
    }

    // Step 3 & 4: fall back to interrupted field
    if let Some(interrupted) = tool_response.get("interrupted") {
        if let Some(b) = interrupted.as_bool() {
            return if b { RunStatus::Fail } else { RunStatus::Pass };
        }
    }

    // Step 5: neither determinable
    RunStatus::Unknown
}

// ---------------------------------------------------------------------------
// State file serialization model (BC-7.03.074)
// ---------------------------------------------------------------------------

/// Regression state written to / read from `.factory/regression-state.json`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegressionState {
    pub status: String,
    pub timestamp: String,
    pub command: String,
}

// ---------------------------------------------------------------------------
// Core hook logic (injectable callbacks — testable without WASM runtime)
// ---------------------------------------------------------------------------

/// All side-effecting callbacks injected into `regression_gate_logic` for
/// testability. In production (main.rs), these are wired to host fns.
pub struct HookCallbacks<R, W, E>
where
    R: FnOnce(&str) -> Option<String>,
    W: FnOnce(&str, &str) -> bool,
    E: FnOnce(&str, &[(&str, &str)]),
{
    /// Read a file by path; returns `Some(contents)` or `None` if absent/error.
    pub read_file: R,
    /// Write a file; returns `true` on success, `false` on failure.
    pub write_file: W,
    /// Emit an event (type, fields).
    pub emit_event: E,
}

/// Core regression-gate hook logic.
///
/// All host I/O is injected via `callbacks` so unit tests can exercise every
/// branch without a WASM runtime.
///
/// Reads `tool_name` and `tool_input.command` from `payload`; performs Bash
/// guard and 9-pattern match; derives STATUS; reads prior state; writes new
/// state; warns on pass-to-fail transition. Returns `HookResult::Continue`
/// in all cases (advisory, on_error=continue).
///
/// # BC traces
/// - BC-7.03.071 postcondition 1: exit 0 always (Continue on every path)
/// - BC-7.03.072 postcondition 1: Bash guard + 9-pattern match
/// - BC-7.03.073 postcondition 1: status derivation cascade
/// - BC-7.03.074 postcondition 1: state file write
/// - BC-7.03.075 postcondition 1: pass-to-fail warning
pub fn regression_gate_logic<R, W, E>(
    payload: HookPayload,
    factory_dir_exists: bool,
    callbacks: HookCallbacks<R, W, E>,
    stderr_warn: &mut dyn FnMut(&str),
) -> HookResult
where
    R: FnOnce(&str) -> Option<String>,
    W: FnOnce(&str, &str) -> bool,
    E: FnOnce(&str, &[(&str, &str)]),
{
    // BC-7.03.072: Bash tool guard — body-filter (no registry tool filter per
    // Architecture Compliance Rules: "regression-gate has no tool filter in registry").
    if payload.tool_name != "Bash" {
        return HookResult::Continue;
    }

    // Extract command from tool_input.
    let command = match payload
        .tool_input
        .get("command")
        .and_then(|v| v.as_str())
    {
        Some(c) if !c.is_empty() => c.to_string(),
        _ => return HookResult::Continue,
    };

    // BC-7.03.072: 9-pattern substring match.
    if !is_test_runner_command(&command) {
        return HookResult::Continue;
    }

    // BC-7.03.073: derive pass/fail status from tool_response.
    let status = match &payload.tool_response {
        Some(resp) => derive_status(resp),
        None => RunStatus::Unknown,
    };

    // BC-7.03.073 step 5: unknown → skip entirely (exit 0, no state write).
    if status == RunStatus::Unknown {
        return HookResult::Continue;
    }

    // BC-7.03.074: .factory/ directory guard.
    if !factory_dir_exists {
        return HookResult::Continue;
    }

    let status_str = if status == RunStatus::Pass { "pass" } else { "fail" };

    // BC-7.03.075: read prior status for regression detection.
    // EC-003: silently ignore malformed JSON (2>/dev/null semantics).
    let prior_status: String = match (callbacks.read_file)(STATE_FILE_PATH) {
        Some(contents) => {
            // Silently suppress parse errors (EC-003: 2>/dev/null parity)
            serde_json::from_str::<RegressionState>(&contents)
                .map(|s| s.status)
                .unwrap_or_else(|_| "unknown".to_string())
        }
        None => "unknown".to_string(),
    };

    // BC-7.03.074: write new state file {status, timestamp, command}.
    // Timestamp: ISO-8601 UTC — replaces `date -u` subprocess (AC-009).
    let timestamp = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let new_state = RegressionState {
        status: status_str.to_string(),
        timestamp: timestamp.clone(),
        command: command.clone(),
    };
    let state_json = match serde_json::to_string(&new_state) {
        Ok(s) => s,
        Err(_) => {
            // Should not happen with this struct, but be defensive.
            vsdd_hook_sdk::host::log_error(
                "regression-gate: failed to serialize state JSON",
            );
            return HookResult::Continue;
        }
    };

    let write_ok = (callbacks.write_file)(STATE_FILE_PATH, &state_json);

    // EC-006 (intentional deviation from bash): log write failure but still
    // continue. The bash source silently exits 0; we add observability.
    if !write_ok {
        vsdd_hook_sdk::host::log_error(&format!(
            "regression-gate: failed to write state file: {}",
            STATE_FILE_PATH
        ));
        // Cannot compare states without a successful write — no regression warning.
        return HookResult::Continue;
    }

    // BC-7.03.075: warn on pass-to-fail transition.
    if prior_status == "pass" && status_str == "fail" {
        (callbacks.emit_event)(
            "hook.block",
            &[
                ("hook", "regression-gate"),
                ("matcher", "Bash"),
                ("reason", "regression_gate_pass_to_fail"),
                ("severity", "warn"),
                ("command", command.as_str()),
            ],
        );

        // Stderr message with actual newline characters (0x0A) per AC-006.
        let msg = format!(
            "regression-gate: suite transitioned pass \u{2192} fail.\n  command: {}\n  recorded: {}\n",
            command, STATE_FILE_PATH
        );
        stderr_warn(&msg);
    }

    HookResult::Continue
}

// ---------------------------------------------------------------------------
// Top-level entry point (wired to real host fns in main.rs)
// ---------------------------------------------------------------------------

/// Called from the WASI entry point in `main.rs`.
///
/// Wires the real vsdd_hook_sdk host functions to the injectable-callback
/// surface of `regression_gate_logic`.
pub fn on_post_tool_use(payload: HookPayload) -> HookResult {
    use vsdd_hook_sdk::host;

    // Check .factory/ directory existence (host FS check via read attempt).
    // The WASM guest checks via the host's read_file capability: attempt to
    // read a sentinel path and treat CapabilityDenied/Other as "absent".
    // A simpler approach: try to read the state file; if the error is NOT
    // "file not found" or "capability denied", the directory likely exists.
    // Actually: the cleanest approach is to try to read the state file and
    // treat any error as "directory absent" for the guard. If the file is
    // absent but the directory exists, read_file returns an error; the bash
    // source checks `[[ ! -d "$STATE_DIR" ]]`. We replicate by using the
    // host read attempt — if it succeeds or fails with "file not found"
    // (vs capability denied), the directory exists.
    //
    // In practice for the WASM sandbox: the capability declaration allows
    // reading `.factory/regression-state.json`, so if the path is
    // capability-denied it means the directory can't be accessed. We treat
    // CapabilityDenied as absent; Other errors (file not found) as directory
    // present (file just not created yet).
    let factory_dir_exists = check_factory_dir_exists();

    let mut stderr_fn = |msg: &str| {
        eprint!("{}", msg);
    };

    regression_gate_logic(
        payload,
        factory_dir_exists,
        HookCallbacks {
            read_file: |path| {
                match host::read_file(path, 4096, 5000) {
                    Ok(bytes) => String::from_utf8(bytes).ok(),
                    Err(_) => None,
                }
            },
            write_file: |path, contents| {
                match host::write_file(path, contents.as_bytes(), 512, 5000) {
                    Ok(()) => true,
                    Err(e) => {
                        host::log_error(&format!(
                            "regression-gate: write_file error: {:?}", e
                        ));
                        false
                    }
                }
            },
            emit_event: |event_type, fields| {
                // Silently swallow emit errors (best-effort, advisory hook).
                let _ = host::emit_event(event_type, fields);
            },
        },
        &mut stderr_fn,
    )
}

/// Check whether `.factory/` directory exists by attempting a bounded read
/// of the state file path. Treats CapabilityDenied as "absent" (guard fails);
/// any other result (including file-not-found with Other error) means the
/// directory is accessible.
fn check_factory_dir_exists() -> bool {
    use vsdd_hook_sdk::host;
    match host::read_file(STATE_FILE_PATH, 4096, 5000) {
        Ok(_) => true,
        Err(host::HostError::CapabilityDenied) => false,
        // File not found (Other) still means .factory/ exists (just no state yet)
        Err(host::HostError::Other(_)) => true,
        // Timeout or output-too-large: treat conservatively as exists
        Err(_) => true,
    }
}

// ---------------------------------------------------------------------------
// Unit tests (BC-7.03.071 through BC-7.03.075)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    fn make_payload(
        tool_name: &str,
        command: &str,
        tool_response: Option<serde_json::Value>,
    ) -> HookPayload {
        let mut v = json!({
            "event_name": "PostToolUse",
            "session_id": "test-session",
            "dispatcher_trace_id": "test-trace",
            "tool_name": tool_name,
            "tool_input": {
                "command": command
            }
        });
        if let Some(resp) = tool_response {
            v["tool_response"] = resp;
        }
        serde_json::from_value(v).expect("fixture must deserialize")
    }

    fn run_logic(
        payload: HookPayload,
        factory_dir_exists: bool,
        prior_state: Option<&str>,
    ) -> (HookResult, Option<String>, Option<String>) {
        // Returns: (result, written_state_json, emitted_event_type)
        let mut written: Option<String> = None;
        let mut emitted_event: Option<String> = None;
        let mut stderr_output: Option<String> = None;

        let result = regression_gate_logic(
            payload,
            factory_dir_exists,
            HookCallbacks {
                read_file: |_path| prior_state.map(|s| s.to_string()),
                write_file: |_path, contents| {
                    written = Some(contents.to_string());
                    true
                },
                emit_event: |event_type, _fields| {
                    emitted_event = Some(event_type.to_string());
                },
            },
            &mut |msg: &str| {
                stderr_output = Some(msg.to_string());
            },
        );

        // Return written or emitted depending on test needs
        (result, written, emitted_event)
    }

    // Helper that also captures emit fields
    fn run_logic_full(
        payload: HookPayload,
        factory_dir_exists: bool,
        prior_state: Option<&str>,
    ) -> (
        HookResult,
        Option<String>,
        Option<(String, Vec<(String, String)>)>,
        Option<String>,
    ) {
        let mut written: Option<String> = None;
        let mut emitted: Option<(String, Vec<(String, String)>)> = None;
        let mut stderr_out: Option<String> = None;

        let result = regression_gate_logic(
            payload,
            factory_dir_exists,
            HookCallbacks {
                read_file: |_path| prior_state.map(|s| s.to_string()),
                write_file: |_path, contents| {
                    written = Some(contents.to_string());
                    true
                },
                emit_event: |event_type, fields| {
                    emitted = Some((
                        event_type.to_string(),
                        fields.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
                    ));
                },
            },
            &mut |msg: &str| {
                stderr_out = Some(msg.to_string());
            },
        );

        (result, written, emitted, stderr_out)
    }

    // -----------------------------------------------------------------------
    // BC-7.03.072: 9 test-runner pattern coverage unit tests
    // Additional pattern-coverage unit tests for all 9 substrings (AC-007)
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_7_03_072_pattern_cargo_test() {
        assert!(is_test_runner_command("cargo test --workspace"));
        assert!(is_test_runner_command("cargo test -p my-crate"));
    }

    #[test]
    fn test_BC_7_03_072_pattern_cargo_nextest() {
        assert!(is_test_runner_command("cargo nextest run"));
        assert!(is_test_runner_command("cargo nextest run --workspace"));
    }

    #[test]
    fn test_BC_7_03_072_pattern_pytest() {
        assert!(is_test_runner_command("pytest tests/"));
        assert!(is_test_runner_command("python -m pytest -v"));
    }

    #[test]
    fn test_BC_7_03_072_pattern_pnpm_test() {
        assert!(is_test_runner_command("pnpm test"));
        assert!(is_test_runner_command("pnpm test -- --watch"));
    }

    #[test]
    fn test_BC_7_03_072_pattern_npm_test() {
        assert!(is_test_runner_command("npm test"));
        assert!(is_test_runner_command("npm test -- --ci"));
    }

    #[test]
    fn test_BC_7_03_072_pattern_go_test() {
        assert!(is_test_runner_command("go test ./..."));
        assert!(is_test_runner_command("go test -v ./pkg/..."));
    }

    #[test]
    fn test_BC_7_03_072_pattern_just_test() {
        assert!(is_test_runner_command("just test"));
        assert!(is_test_runner_command("just test integration"));
    }

    #[test]
    fn test_BC_7_03_072_pattern_just_ci() {
        assert!(is_test_runner_command("just ci"));
        assert!(is_test_runner_command("just ci --verbose"));
    }

    #[test]
    fn test_BC_7_03_072_pattern_yarn_test() {
        assert!(is_test_runner_command("yarn test"));
        assert!(is_test_runner_command("yarn test --watchAll=false"));
    }

    #[test]
    fn test_BC_7_03_072_non_test_command_does_not_match() {
        assert!(!is_test_runner_command("git commit -m msg"));
        assert!(!is_test_runner_command("cargo build --release"));
        assert!(!is_test_runner_command("echo hello"));
        assert!(!is_test_runner_command("ls -la"));
    }

    // -----------------------------------------------------------------------
    // BC-7.03.073: pass/fail derivation cascade
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_7_03_073_exit_code_0_is_pass() {
        let resp = json!({"exit_code": 0});
        assert_eq!(derive_status(&resp), RunStatus::Pass);
    }

    #[test]
    fn test_BC_7_03_073_exit_code_nonzero_is_fail() {
        let resp = json!({"exit_code": 1});
        assert_eq!(derive_status(&resp), RunStatus::Fail);

        let resp = json!({"exit_code": 127});
        assert_eq!(derive_status(&resp), RunStatus::Fail);
    }

    #[test]
    fn test_BC_7_03_073_returncode_0_is_pass() {
        // Back-compat: returncode field (no exit_code)
        let resp = json!({"returncode": 0});
        assert_eq!(derive_status(&resp), RunStatus::Pass);
    }

    #[test]
    fn test_BC_7_03_073_interrupted_true_is_fail() {
        // No exit_code; interrupted=true
        let resp = json!({"interrupted": true});
        assert_eq!(derive_status(&resp), RunStatus::Fail);
    }

    #[test]
    fn test_BC_7_03_073_interrupted_false_is_pass() {
        // No exit_code; interrupted=false
        let resp = json!({"interrupted": false});
        assert_eq!(derive_status(&resp), RunStatus::Pass);
    }

    #[test]
    fn test_BC_7_03_073_neither_field_is_unknown() {
        // AC-004 step 5: neither exit_code nor interrupted → unknown
        let resp = json!({"stdout": "some output", "stderr": ""});
        assert_eq!(derive_status(&resp), RunStatus::Unknown);
    }

    #[test]
    fn test_BC_7_03_073_null_exit_code_falls_back_to_interrupted() {
        // exit_code: null + interrupted: false → pass (fall-through per cascade)
        let resp = json!({"exit_code": null, "interrupted": false});
        assert_eq!(derive_status(&resp), RunStatus::Pass);
    }

    #[test]
    fn test_BC_7_03_073_exit_code_takes_priority_over_interrupted() {
        // exit_code=0 + interrupted=true → Pass (exit_code wins)
        let resp = json!({"exit_code": 0, "interrupted": true});
        assert_eq!(derive_status(&resp), RunStatus::Pass);
    }

    // -----------------------------------------------------------------------
    // BC-7.03.071 / BC-7.03.072: Bash guard + early exits
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_7_03_071_non_bash_tool_exits_0_immediately() {
        // EC-001: PostToolUse for non-Bash tool → exit 0, no state write
        let payload = make_payload("Read", "cargo test", Some(json!({"exit_code": 0})));
        let (result, written, _) = run_logic(payload, true, None);
        assert_eq!(result, HookResult::Continue);
        assert!(written.is_none(), "no state file write for non-Bash tool");
    }

    #[test]
    fn test_BC_7_03_072_non_test_command_exits_0_no_state_write() {
        // f: git commit -m msg → exit 0, no state write
        let payload = make_payload("Bash", "git commit -m 'chore: update'", Some(json!({"exit_code": 0})));
        let (result, written, _) = run_logic(payload, true, None);
        assert_eq!(result, HookResult::Continue);
        assert!(written.is_none(), "no state write for non-test command");
    }

    #[test]
    fn test_BC_7_03_072_empty_command_exits_0_no_state_write() {
        let payload = make_payload("Bash", "", Some(json!({"exit_code": 0})));
        let (result, written, _) = run_logic(payload, true, None);
        assert_eq!(result, HookResult::Continue);
        assert!(written.is_none());
    }

    // -----------------------------------------------------------------------
    // BC-7.03.074: state file write
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_7_03_074_writes_state_file_on_pass() {
        // Scenario (a): cargo test pass, no prior state file
        let payload = make_payload("Bash", "cargo test", Some(json!({"exit_code": 0})));
        let (result, written, _) = run_logic(payload, true, None);
        assert_eq!(result, HookResult::Continue);
        let json_str = written.expect("state file must be written");
        let state: RegressionState = serde_json::from_str(&json_str).expect("valid JSON");
        assert_eq!(state.status, "pass");
        assert_eq!(state.command, "cargo test");
        assert!(!state.timestamp.is_empty());
    }

    #[test]
    fn test_BC_7_03_074_writes_state_file_on_fail() {
        let payload = make_payload("Bash", "cargo test", Some(json!({"exit_code": 1})));
        let (result, written, _) = run_logic(payload, true, None);
        assert_eq!(result, HookResult::Continue);
        let json_str = written.expect("state file must be written");
        let state: RegressionState = serde_json::from_str(&json_str).expect("valid JSON");
        assert_eq!(state.status, "fail");
    }

    #[test]
    fn test_BC_7_03_074_factory_dir_absent_exits_0_no_state_write() {
        // Scenario (g): .factory/ absent → exit 0, no state write
        let payload = make_payload("Bash", "cargo test", Some(json!({"exit_code": 0})));
        let (result, written, _) = run_logic(payload, false, None);
        assert_eq!(result, HookResult::Continue);
        assert!(written.is_none(), ".factory/ absent → no state write");
    }

    #[test]
    fn test_BC_7_03_074_state_file_contains_iso8601_timestamp() {
        let payload = make_payload("Bash", "cargo test", Some(json!({"exit_code": 0})));
        let (_, written, _) = run_logic(payload, true, None);
        let json_str = written.expect("state file must be written");
        let state: RegressionState = serde_json::from_str(&json_str).expect("valid JSON");
        // ISO-8601 UTC: YYYY-MM-DDTHH:MM:SSZ
        assert!(
            state.timestamp.contains('T') && state.timestamp.ends_with('Z'),
            "timestamp must be ISO-8601 UTC: got '{}'", state.timestamp
        );
    }

    // -----------------------------------------------------------------------
    // BC-7.03.073 step 5: unknown status → skip
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_7_03_073_unknown_status_no_state_write() {
        // Scenario (e): no exit_code, no interrupted field → unknown → skip
        let payload = make_payload("Bash", "cargo test", Some(json!({"stdout": "output"})));
        let (result, written, emitted) = run_logic(payload, true, None);
        assert_eq!(result, HookResult::Continue);
        assert!(written.is_none(), "unknown status → no state write");
        assert!(emitted.is_none(), "unknown status → no event");
    }

    // -----------------------------------------------------------------------
    // BC-7.03.075: pass-to-fail regression warning
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_7_03_075_pass_to_fail_emits_hook_block_warn() {
        // Scenario (c): exit_code=1, prior state=pass → regression warning
        let prior = r#"{"status":"pass","timestamp":"2026-05-01T00:00:00Z","command":"cargo test"}"#;
        let payload = make_payload("Bash", "cargo test", Some(json!({"exit_code": 1})));
        let (result, written, emitted, stderr) =
            run_logic_full(payload, true, Some(prior));

        assert_eq!(result, HookResult::Continue);
        assert!(written.is_some(), "state must be written even on regression");

        let (event_type, fields) = emitted.expect("hook.block must be emitted");
        assert_eq!(event_type, "hook.block");
        let fmap: std::collections::HashMap<&str, &str> = fields
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        assert_eq!(fmap.get("hook"), Some(&"regression-gate"));
        assert_eq!(fmap.get("matcher"), Some(&"Bash"));
        assert_eq!(fmap.get("reason"), Some(&"regression_gate_pass_to_fail"));
        assert_eq!(fmap.get("severity"), Some(&"warn"));
        assert_eq!(fmap.get("command"), Some(&"cargo test"));

        let msg = stderr.expect("stderr warning must be written");
        assert!(msg.contains("pass"), "stderr must contain 'pass'");
        assert!(msg.contains("fail"), "stderr must contain 'fail'");
        assert!(msg.contains("cargo test"), "stderr must contain the command");
        assert!(msg.contains(".factory/regression-state.json"), "stderr must contain state file path");
    }

    #[test]
    fn test_BC_7_03_075_interrupted_true_prior_pass_emits_warning() {
        // Scenario (d): interrupted=true, prior state=pass → regression warning
        let prior = r#"{"status":"pass","timestamp":"2026-05-01T00:00:00Z","command":"cargo test"}"#;
        let payload = make_payload("Bash", "cargo test", Some(json!({"interrupted": true})));
        let (result, _written, emitted, _stderr) =
            run_logic_full(payload, true, Some(prior));

        assert_eq!(result, HookResult::Continue);
        let (event_type, fields) = emitted.expect("hook.block must be emitted");
        assert_eq!(event_type, "hook.block");
        let fmap: std::collections::HashMap<&str, &str> =
            fields.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
        assert_eq!(fmap.get("severity"), Some(&"warn"));
        assert_eq!(fmap.get("reason"), Some(&"regression_gate_pass_to_fail"));
    }

    #[test]
    fn test_BC_7_03_075_fail_to_fail_no_warning() {
        // Scenario (h): exit_code=1, prior state=fail → NO regression warning
        let prior = r#"{"status":"fail","timestamp":"2026-05-01T00:00:00Z","command":"cargo test"}"#;
        let payload = make_payload("Bash", "cargo test", Some(json!({"exit_code": 1})));
        let (result, written, emitted, stderr) =
            run_logic_full(payload, true, Some(prior));

        assert_eq!(result, HookResult::Continue);
        assert!(written.is_some(), "state is still written on fail-to-fail");
        assert!(emitted.is_none(), "no event emitted on fail-to-fail");
        assert!(stderr.is_none(), "no stderr on fail-to-fail");
    }

    #[test]
    fn test_BC_7_03_075_no_prior_state_pass_no_warning() {
        // No prior state (first run) + pass → no warning
        let payload = make_payload("Bash", "cargo test", Some(json!({"exit_code": 0})));
        let (result, written, emitted, _stderr) =
            run_logic_full(payload, true, None);

        assert_eq!(result, HookResult::Continue);
        assert!(written.is_some());
        assert!(emitted.is_none(), "no event when no prior state");
    }

    #[test]
    fn test_BC_7_03_075_no_prior_state_fail_no_warning() {
        // No prior state (first run) + fail → no warning (unknown-to-fail is EC-004)
        let payload = make_payload("Bash", "cargo test", Some(json!({"exit_code": 1})));
        let (result, _written, emitted, _stderr) =
            run_logic_full(payload, true, None);

        assert_eq!(result, HookResult::Continue);
        assert!(emitted.is_none(), "EC-004: unknown-to-fail must not warn");
    }

    // -----------------------------------------------------------------------
    // EC-003: malformed state file → silent suppression
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_7_03_075_ec003_malformed_state_file_silent_suppress() {
        // EC-003: malformed JSON in state file → treat prior status as "unknown"
        // → no regression warning
        let prior = "this is not valid json {{";
        let payload = make_payload("Bash", "cargo test", Some(json!({"exit_code": 1})));
        let (result, written, emitted, stderr) =
            run_logic_full(payload, true, Some(prior));

        assert_eq!(result, HookResult::Continue);
        // State is still written (we overwrote the malformed file)
        assert!(written.is_some(), "malformed prior state → still write new state");
        // No regression warning (prior is "unknown", not "pass")
        assert!(emitted.is_none(), "EC-003: malformed prior → no regression warning");
        assert!(stderr.is_none(), "EC-003: no stderr on malformed prior state");
    }

    // -----------------------------------------------------------------------
    // BC-7.03.071 invariant: exit 0 always (HookResult::Continue)
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_7_03_071_invariant_continue_on_all_paths() {
        // Sample a variety of inputs — all must return Continue
        let cases: Vec<(HookPayload, bool, Option<&str>)> = vec![
            // Non-Bash tool
            (make_payload("Edit", "cargo test", None), true, None),
            // Non-test command
            (make_payload("Bash", "git status", Some(json!({"exit_code": 0}))), true, None),
            // Unknown status
            (make_payload("Bash", "cargo test", Some(json!({}))), true, None),
            // .factory/ absent
            (make_payload("Bash", "cargo test", Some(json!({"exit_code": 0}))), false, None),
            // Normal pass
            (make_payload("Bash", "cargo test", Some(json!({"exit_code": 0}))), true, None),
            // Normal fail
            (make_payload("Bash", "cargo test", Some(json!({"exit_code": 1}))), true, None),
            // Pass-to-fail regression
            (
                make_payload("Bash", "cargo test", Some(json!({"exit_code": 1}))),
                true,
                Some(r#"{"status":"pass","timestamp":"2026-05-01T00:00:00Z","command":"cargo test"}"#),
            ),
        ];

        for (payload, factory_exists, prior) in cases {
            let (result, _, _) = run_logic(payload, factory_exists, prior);
            assert_eq!(
                result,
                HookResult::Continue,
                "expected Continue (exit 0) on all paths"
            );
        }
    }

    // -----------------------------------------------------------------------
    // AC-007 scenario (i): pytest pattern coverage
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_7_03_072_pytest_pattern_triggers_hook_body() {
        // pytest tests/ → matches, state file written
        let payload = make_payload("Bash", "pytest tests/", Some(json!({"exit_code": 0})));
        let (result, written, _) = run_logic(payload, true, None);
        assert_eq!(result, HookResult::Continue);
        let json_str = written.expect("pytest command must trigger state write");
        let state: RegressionState = serde_json::from_str(&json_str).expect("valid JSON");
        assert_eq!(state.status, "pass");
        assert_eq!(state.command, "pytest tests/");
    }

    // -----------------------------------------------------------------------
    // AC-007 scenario (b): interrupted=false → pass (no prior state)
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_7_03_073_interrupted_false_prior_none_pass_no_warning() {
        let payload = make_payload("Bash", "cargo test", Some(json!({"interrupted": false})));
        let (result, written, emitted, _) = run_logic_full(payload, true, None);

        assert_eq!(result, HookResult::Continue);
        let json_str = written.expect("state file must be written");
        let state: RegressionState = serde_json::from_str(&json_str).unwrap();
        assert_eq!(state.status, "pass");
        assert!(emitted.is_none(), "no regression warning when no prior state");
    }

    // -----------------------------------------------------------------------
    // Stderr message format validation (AC-006)
    // -----------------------------------------------------------------------

    #[test]
    fn test_BC_7_03_075_stderr_message_contains_rendered_newlines() {
        // AC-006: newlines in stderr message are actual 0x0A characters
        let prior = r#"{"status":"pass","timestamp":"2026-05-01T00:00:00Z","command":"cargo test"}"#;
        let payload = make_payload("Bash", "cargo test", Some(json!({"exit_code": 1})));
        let (_, _, _, stderr) = run_logic_full(payload, true, Some(prior));
        let msg = stderr.expect("stderr must be written");
        // Should contain at least 2 newline characters (0x0A)
        let newline_count = msg.bytes().filter(|&b| b == b'\n').count();
        assert!(newline_count >= 2, "stderr must contain at least 2 newlines (0x0A); got {}", newline_count);
    }
}
