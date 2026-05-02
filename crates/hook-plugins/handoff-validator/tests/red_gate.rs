//! S-8.01 Red Gate — failing integration tests for handoff-validator port.
//!
//! Every test in this file MUST FAIL before implementation begins.
//! Tests exercise AC deliverables that the stub-architect did NOT produce:
//!
//! - AC-001: hooks-registry.toml migrated to native handoff-validator.wasm
//! - AC-002: handoff-validator.sh deleted from repository
//! - AC-003/004: exact stderr message format and exact emit-field set
//! - AC-005: bats parity test file exists at canonical path
//! - AC-006: no exec_subprocess capability block; no shell_bypass_acknowledged
//! - AC-007: no bin/emit-event reference in crate source
//!
//! BCs exercised:
//!   BC-7.03.042 (registry binding, postcondition 1; invariants 1+2)
//!   BC-7.03.043 (empty result warn, postcondition 1)
//!   BC-7.03.044 (short result warn, postcondition 1)
//!   BC-2.02.012 (typed projection, postconditions 1-6; EC-001..EC-005)
//!
//! Run with per-worktree CARGO_TARGET_DIR:
//!   PATH="$HOME/.cargo/bin:$PATH" CARGO_TARGET_DIR=$(pwd)/target \
//!     cargo test -p handoff-validator --tests 2>&1
//!
//! All tests here are EXPECTED TO FAIL until the implementer completes
//! the migration tasks T-2 through T-7.

use handoff_validator::{classify_result, handoff_validator_logic, ResultClassification};
use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// Helper: resolve the worktree root from CARGO_MANIFEST_DIR.
// CARGO_MANIFEST_DIR points to crates/hook-plugins/handoff-validator/
// so we go up three levels to reach the repo root.
// ---------------------------------------------------------------------------

fn worktree_root() -> std::path::PathBuf {
    let manifest = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR must be set during cargo test");
    // crates/hook-plugins/handoff-validator → ../../.. = root
    std::path::Path::new(&manifest)
        .parent() // hook-plugins
        .and_then(|p| p.parent()) // crates
        .and_then(|p| p.parent()) // root
        .expect("CARGO_MANIFEST_DIR must be three levels deep from repo root")
        .to_path_buf()
}

// ---------------------------------------------------------------------------
// Helper: build HookPayload from a JSON string.
// ---------------------------------------------------------------------------

fn make_payload(json: &str) -> HookPayload {
    serde_json::from_str(json).expect("test fixture should parse")
}

fn base_subagentstop(extra: &str) -> String {
    format!(
        r#"{{"event_name":"SubagentStop","session_id":"s","dispatcher_trace_id":"t"{}}}"#,
        if extra.is_empty() {
            String::new()
        } else {
            format!(",{}", extra)
        }
    )
}

// ===========================================================================
// AC-001 / BC-7.03.042 postcondition 1: Registry migration
//
// hooks-registry.toml must reference the NATIVE plugin, not the legacy adapter.
// These tests FAIL until T-6 completes.
// ===========================================================================

/// AC-001 (a): hooks-registry.toml must reference the native handoff-validator
/// WASM plugin, not the legacy-bash-adapter.
///
/// Fails until T-6 updates `plugin = "hook-plugins/handoff-validator.wasm"`.
#[test]
fn test_BC_7_03_042_ac001a_registry_uses_native_handoff_validator_wasm() {
    let root = worktree_root();
    let registry_path = root.join("plugins/vsdd-factory/hooks-registry.toml");
    let content = std::fs::read_to_string(&registry_path)
        .unwrap_or_else(|e| panic!("cannot read {}: {}", registry_path.display(), e));

    // The handoff-validator entry must reference the native .wasm, not the adapter.
    // Strategy: find the [[hooks]] block for handoff-validator and assert native path.
    // We search for the canonical native plugin path between the "handoff-validator"
    // name and the next [[hooks]] boundary.
    let handoff_block_start = content
        .find("name = \"handoff-validator\"")
        .expect("handoff-validator entry must exist in hooks-registry.toml");
    let after_entry = &content[handoff_block_start..];
    // Next [[hooks]] boundary (or end of file)
    let block_end = after_entry
        .find("\n[[hooks]]")
        .unwrap_or(after_entry.len());
    let handoff_block = &after_entry[..block_end];

    assert!(
        handoff_block.contains("hook-plugins/handoff-validator.wasm"),
        "AC-001: hooks-registry.toml handoff-validator entry must use \
         plugin = \"hook-plugins/handoff-validator.wasm\", \
         but found legacy-bash-adapter reference. Complete T-6."
    );
}

/// AC-001 (b): hooks-registry.toml must NOT have a `script_path` key in the
/// handoff-validator entry. That key is only valid for legacy-bash-adapter.
///
/// Fails until T-6 removes the `[hooks.config]` block.
#[test]
fn test_BC_7_03_042_ac001b_registry_no_script_path_in_handoff_validator_entry() {
    let root = worktree_root();
    let registry_path = root.join("plugins/vsdd-factory/hooks-registry.toml");
    let content = std::fs::read_to_string(&registry_path)
        .unwrap_or_else(|e| panic!("cannot read {}: {}", registry_path.display(), e));

    let handoff_block_start = content
        .find("name = \"handoff-validator\"")
        .expect("handoff-validator entry must exist");
    let after_entry = &content[handoff_block_start..];
    let block_end = after_entry
        .find("\n[[hooks]]")
        .unwrap_or(after_entry.len());
    let handoff_block = &after_entry[..block_end];

    assert!(
        !handoff_block.contains("script_path"),
        "AC-001: hooks-registry.toml handoff-validator entry must NOT contain \
         `script_path` (legacy-bash-adapter only). Complete T-6 to remove [hooks.config]."
    );
}

/// AC-001 (c): After T-6 the [hooks.capabilities.exec_subprocess] block must
/// be ENTIRELY removed from the handoff-validator registry entry.
/// Native WASM crates do not spawn subprocesses; the block is nonsensical and
/// must not be present (story spec AC-001 and T-6 full-block-removal requirement).
///
/// Fails until T-6 removes the exec_subprocess block.
#[test]
fn test_BC_7_03_042_ac001c_registry_no_exec_subprocess_block_in_handoff_validator_entry() {
    let root = worktree_root();
    let registry_path = root.join("plugins/vsdd-factory/hooks-registry.toml");
    let content = std::fs::read_to_string(&registry_path)
        .unwrap_or_else(|e| panic!("cannot read {}: {}", registry_path.display(), e));

    let handoff_block_start = content
        .find("name = \"handoff-validator\"")
        .expect("handoff-validator entry must exist");
    let after_entry = &content[handoff_block_start..];
    let block_end = after_entry
        .find("\n[[hooks]]")
        .unwrap_or(after_entry.len());
    let handoff_block = &after_entry[..block_end];

    assert!(
        !handoff_block.contains("exec_subprocess"),
        "AC-001: [hooks.capabilities.exec_subprocess] block must be ENTIRELY \
         removed from the handoff-validator entry (T-6 full-block-removal). \
         Native WASM crates call no subprocesses."
    );
}

/// AC-001 (d): The `shell_bypass_acknowledged` key must be absent from the
/// handoff-validator registry entry after migration.
/// That key is only valid for legacy-bash-adapter entries.
///
/// Fails until T-6 removes the field.
#[test]
fn test_BC_7_03_042_ac001d_registry_no_shell_bypass_acknowledged_in_handoff_validator_entry() {
    let root = worktree_root();
    let registry_path = root.join("plugins/vsdd-factory/hooks-registry.toml");
    let content = std::fs::read_to_string(&registry_path)
        .unwrap_or_else(|e| panic!("cannot read {}: {}", registry_path.display(), e));

    let handoff_block_start = content
        .find("name = \"handoff-validator\"")
        .expect("handoff-validator entry must exist");
    let after_entry = &content[handoff_block_start..];
    let block_end = after_entry
        .find("\n[[hooks]]")
        .unwrap_or(after_entry.len());
    let handoff_block = &after_entry[..block_end];

    assert!(
        !handoff_block.contains("shell_bypass_acknowledged"),
        "AC-001: `shell_bypass_acknowledged` must be absent from the native \
         handoff-validator entry (legacy-bash-adapter only). Complete T-6."
    );
}

/// AC-001 (e): The preserved fields must remain: event, priority, on_error, timeout_ms.
/// Specifically, priority must be 910 and on_error must be "block".
///
/// This verifies the invariant that the SubagentStop binding is preserved after
/// migration (BC-7.03.042 postcondition 1).
///
/// NOTE: This test passes when the entry still exists (whether migrated or not),
/// so it acts as a forward-guard that also verifies the event/priority/on_error
/// invariant holds in the MIGRATED state. Combined with test_ac001a (which
/// forces native wasm path), this is only fully satisfied post-migration.
#[test]
fn test_BC_7_03_042_ac001e_registry_preserved_binding_fields_event_priority_on_error() {
    let root = worktree_root();
    let registry_path = root.join("plugins/vsdd-factory/hooks-registry.toml");
    let content = std::fs::read_to_string(&registry_path)
        .unwrap_or_else(|e| panic!("cannot read {}: {}", registry_path.display(), e));

    let handoff_block_start = content
        .find("name = \"handoff-validator\"")
        .expect("handoff-validator entry must exist");
    let after_entry = &content[handoff_block_start..];
    let block_end = after_entry
        .find("\n[[hooks]]")
        .unwrap_or(after_entry.len());
    let handoff_block = &after_entry[..block_end];

    assert!(
        handoff_block.contains("event = \"SubagentStop\""),
        "AC-001: SubagentStop event binding must be preserved after migration"
    );
    assert!(
        handoff_block.contains("priority = 910"),
        "AC-001: priority = 910 must be preserved after migration (BC-7.03.042 PC-1)"
    );
    assert!(
        handoff_block.contains("on_error = \"block\""),
        "AC-001: on_error = \"block\" must be preserved after migration (BC-7.03.042 PC-3)"
    );
}

// ===========================================================================
// AC-002 / BC-7.03.042 invariant 1: Shell script lifecycle
//
// handoff-validator.sh must be DELETED after the native port lands.
// This test FAILS until T-7 deletes the file.
// ===========================================================================

/// AC-002 / BC-7.03.042 invariant 1: handoff-validator.sh must not exist
/// in the repository after the native WASM port is complete.
///
/// Fails until T-7 deletes plugins/vsdd-factory/hooks/handoff-validator.sh.
#[test]
fn test_BC_7_03_042_ac002_handoff_validator_sh_deleted() {
    let root = worktree_root();
    let sh_path = root.join("plugins/vsdd-factory/hooks/handoff-validator.sh");
    assert!(
        !sh_path.exists(),
        "AC-002: handoff-validator.sh must be deleted after native WASM port completes \
         (BC-7.03.042 invariant 1). Complete T-7 to remove the file. \
         Path: {}",
        sh_path.display()
    );
}

// ===========================================================================
// AC-005 / BC-7.03.043 + BC-7.03.044: Bats parity test file
//
// The bats integration test file must exist at the canonical path.
// This test FAILS until T-5 writes the file.
// ===========================================================================

/// AC-005: The bats parity test file must exist at
/// `tests/integration/E-8-hook-plugins/handoff-validator.bats`.
///
/// Fails until T-5 writes the bats test file.
#[test]
fn test_BC_7_03_043_ac005_bats_parity_test_file_exists() {
    let root = worktree_root();
    let bats_path = root.join("tests/integration/E-8-hook-plugins/handoff-validator.bats");
    assert!(
        bats_path.exists(),
        "AC-005: bats parity test file must exist at \
         tests/integration/E-8-hook-plugins/handoff-validator.bats. \
         Complete T-5 to write the 7-case parity test suite. \
         Expected path: {}",
        bats_path.display()
    );
}

/// AC-005 (content): The bats file must contain all 7 test case markers
/// required by T-5 (cases a through g).
///
/// Fails until T-5 writes all 7 test cases.
#[test]
fn test_BC_7_03_043_ac005_bats_file_contains_all_7_test_cases() {
    let root = worktree_root();
    let bats_path = root.join("tests/integration/E-8-hook-plugins/handoff-validator.bats");
    let content = std::fs::read_to_string(&bats_path)
        .unwrap_or_else(|e| panic!("bats file must exist before content check: {}", e));

    // Case (a): empty result → exit 0 + stderr "empty result"
    assert!(
        content.contains("empty result") || content.contains("empty_result"),
        "AC-005(a): bats file must contain empty-result test case"
    );
    // Case (d): exactly 39 non-whitespace chars → warning (below threshold)
    assert!(
        content.contains("39"),
        "AC-005(d): bats file must contain LEN=39 boundary test (below threshold)"
    );
    // Case (e): exactly 40 non-whitespace chars → NO warning (at-or-above threshold)
    assert!(
        content.contains("40"),
        "AC-005(e): bats file must contain LEN=40 boundary test (at-or-above threshold)"
    );
    // Case (g): malformed JSON → exit 0, no panic
    assert!(
        content.contains("malformed") || content.contains("invalid"),
        "AC-005(g): bats file must contain malformed JSON graceful-exit test"
    );
}

// ===========================================================================
// AC-007 / E-8 D-10: No bin/emit-event reference in crate source
//
// The native WASM crate must use host::emit_event, not bin/emit-event subprocess.
// These tests verify the crate source has been migrated.
// ===========================================================================

/// AC-007: The handoff-validator crate source must NOT contain any live
/// (non-comment) subprocess call to `bin/emit-event`.
/// Native WASM plugins call host::emit_event; subprocess-based emit is forbidden.
///
/// Comments that REFERENCE bin/emit-event (e.g. "bin/emit-event calls are replaced
/// by host::emit_event") are allowed — they document the migration. What is forbidden
/// is a live Rust subprocess call such as `Command::new("bin/emit-event")` or
/// `exec_subprocess("bin/emit-event", ...)`.
///
/// This test passes immediately (stub architect already uses host::emit_event).
/// Included as a regression gate against future regressions.
#[test]
fn test_BC_7_03_042_ac007_no_subprocess_call_to_bin_emit_event_in_crate_source() {
    let manifest = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR must be set");
    let src_dir = std::path::Path::new(&manifest).join("src");

    // Forbidden patterns: live subprocess call patterns (not comment-only references).
    // We look for exec_subprocess or Command::new with bin/emit-event as argument.
    // Comment-only references (lines starting with // or //!) are excluded.
    let forbidden_patterns = [
        "exec_subprocess(\"bin/emit-event\"",
        "exec_subprocess('bin/emit-event'",
        "Command::new(\"bin/emit-event\"",
        "Command::new('bin/emit-event'",
    ];

    for file in ["lib.rs", "main.rs"] {
        let path = src_dir.join(file);
        if !path.exists() {
            continue;
        }
        let content = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("cannot read {}: {}", path.display(), e));

        for pattern in &forbidden_patterns {
            // Filter out comment lines before checking
            let has_live_call = content
                .lines()
                .filter(|line| {
                    let trimmed = line.trim();
                    !trimmed.starts_with("//")
                })
                .any(|line| line.contains(pattern));

            assert!(
                !has_live_call,
                "AC-007: {file} must not contain live subprocess call '{pattern}'. \
                 Use host::emit_event instead (E-8 D-10)."
            );
        }
    }
}

// ===========================================================================
// AC-003 / BC-7.03.043: Exact emit-field set for empty result
//
// The emitted event must have EXACTLY these 5 key-value pairs (no more, no less):
//   hook=handoff-validator, matcher=SubagentStop, reason=subagent_empty_result,
//   severity=warn, subagent=<agent>
// ===========================================================================

/// BC-7.03.043 postcondition 1 (exact field set): When result is empty,
/// the emitted event must have exactly the 5 canonical fields and no others.
///
/// The stub-architect tests already check individual field values.
/// This test verifies the EXACT FIELD COUNT (5 for empty, 6 for short)
/// to catch any stray fields.
#[test]
fn test_BC_7_03_043_empty_result_emits_exactly_5_fields() {
    let payload = make_payload(&base_subagentstop(
        r#""agent_type":"test-agent","last_assistant_message":"""#,
    ));

    let mut field_count: usize = 0;
    handoff_validator_logic(
        payload,
        |event_type, fields| {
            assert_eq!(event_type, "hook.block", "event type must be hook.block");
            field_count = fields.len();
        },
        |_| {},
    );

    assert_eq!(
        field_count, 5,
        "BC-7.03.043 PC-1: empty-result event must have exactly 5 fields \
         (hook, matcher, reason, severity, subagent) — \
         got {field_count} fields"
    );
}

/// BC-7.03.043: The exact field NAMES for the empty-result path must be
/// hook, matcher, reason, severity, subagent (in any order).
#[test]
fn test_BC_7_03_043_empty_result_emitted_field_names_are_canonical() {
    let payload = make_payload(&base_subagentstop(
        r#""agent_type":"test-agent","last_assistant_message":"""#,
    ));

    let mut keys: Vec<String> = Vec::new();
    handoff_validator_logic(
        payload,
        |_, fields| {
            keys = fields.iter().map(|(k, _)| k.to_string()).collect();
        },
        |_| {},
    );

    let mut sorted_keys = keys.clone();
    sorted_keys.sort();
    let expected_sorted = {
        let mut e = vec!["hook", "matcher", "reason", "severity", "subagent"];
        e.sort();
        e
    };
    assert_eq!(
        sorted_keys.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
        expected_sorted,
        "BC-7.03.043: emitted fields for empty result must be exactly \
         [hook, matcher, reason, severity, subagent] — got: {:?}",
        keys
    );
}

// ===========================================================================
// AC-004 / BC-7.03.044: Exact emit-field set for short result
//
// The emitted event must have EXACTLY 6 fields (same 5 as empty + result_len).
// ===========================================================================

/// BC-7.03.044 postcondition 1 (exact field set): When result is short (1..39),
/// the emitted event must have exactly 6 canonical fields including result_len.
#[test]
fn test_BC_7_03_044_short_result_emits_exactly_6_fields() {
    let payload = make_payload(&base_subagentstop(
        r#""agent_type":"test-agent","last_assistant_message":"hello""#,
    ));

    let mut field_count: usize = 0;
    handoff_validator_logic(
        payload,
        |event_type, fields| {
            assert_eq!(event_type, "hook.block");
            field_count = fields.len();
        },
        |_| {},
    );

    assert_eq!(
        field_count, 6,
        "BC-7.03.044 PC-1: short-result event must have exactly 6 fields \
         (hook, matcher, reason, severity, subagent, result_len) — \
         got {field_count} fields"
    );
}

/// BC-7.03.044: The exact field NAMES for the short-result path must be
/// hook, matcher, reason, severity, subagent, result_len (in any order).
#[test]
fn test_BC_7_03_044_short_result_emitted_field_names_are_canonical() {
    let payload = make_payload(&base_subagentstop(
        r#""agent_type":"test-agent","last_assistant_message":"hello""#,
    ));

    let mut keys: Vec<String> = Vec::new();
    handoff_validator_logic(
        payload,
        |_, fields| {
            keys = fields.iter().map(|(k, _)| k.to_string()).collect();
        },
        |_| {},
    );

    let mut sorted_keys = keys.clone();
    sorted_keys.sort();
    let expected_sorted = {
        let mut e = vec!["hook", "matcher", "reason", "result_len", "severity", "subagent"];
        e.sort();
        e
    };
    assert_eq!(
        sorted_keys.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
        expected_sorted,
        "BC-7.03.044: emitted fields for short result must be exactly \
         [hook, matcher, reason, result_len, severity, subagent] — got: {:?}",
        keys
    );
}

// ===========================================================================
// AC-003 / BC-7.03.043: Exact stderr message format
// ===========================================================================

/// BC-7.03.043 postcondition 1 (exact stderr format): The stderr message for
/// empty result must match the canonical format from the story spec AC-003:
///   "handoff-validator: subagent '<agent>' returned an empty result.\n
///    This is a silent-failure risk — verify before continuing.\n"
#[test]
fn test_BC_7_03_043_empty_result_stderr_exact_format() {
    let payload = make_payload(&base_subagentstop(
        r#""agent_type":"my-agent","last_assistant_message":"""#,
    ));

    let mut stderr_msg: Option<String> = None;
    handoff_validator_logic(
        payload,
        |_, _| {},
        |msg| {
            stderr_msg = Some(msg.to_string());
        },
    );

    let msg = stderr_msg.expect("stderr must be written for empty result");
    let expected = "handoff-validator: subagent 'my-agent' returned an empty result.\n  This is a silent-failure risk \u{2014} verify before continuing.\n";
    assert_eq!(
        msg, expected,
        "BC-7.03.043: stderr message for empty result must match exact canonical format \
         from AC-003. Got:\n{:?}\nExpected:\n{:?}",
        msg, expected
    );
}

/// BC-7.03.044 postcondition 1 (exact stderr format): The stderr message for
/// short result must match the canonical format from the story spec AC-004:
///   "handoff-validator: subagent '<agent>' returned only <N> non-whitespace characters.\n
///    Suspiciously short — verify the subagent completed its task.\n"
#[test]
fn test_BC_7_03_044_short_result_stderr_exact_format() {
    let payload = make_payload(&base_subagentstop(
        r#""agent_type":"my-agent","last_assistant_message":"hello""#,
    ));
    // "hello" has 5 non-whitespace chars

    let mut stderr_msg: Option<String> = None;
    handoff_validator_logic(
        payload,
        |_, _| {},
        |msg| {
            stderr_msg = Some(msg.to_string());
        },
    );

    let msg = stderr_msg.expect("stderr must be written for short result");
    let expected = "handoff-validator: subagent 'my-agent' returned only 5 non-whitespace characters.\n  Suspiciously short \u{2014} verify the subagent completed its task.\n";
    assert_eq!(
        msg, expected,
        "BC-7.03.044: stderr message for short result must match exact canonical format \
         from AC-004. Got:\n{:?}\nExpected:\n{:?}",
        msg, expected
    );
}

// ===========================================================================
// EC-001 / BC-2.02.012: Both message fields absent → warn as empty, agent=unknown
// ===========================================================================

/// BC-2.02.012 EC-001 / BC-7.03.043: When both last_assistant_message and result
/// fields are absent, the 2-stage fallback chain resolves to "" → empty warn,
/// agent identity resolves to "unknown" via BC-2.02.012 Postcondition 5.
///
/// Tests AC-003 typed-projection + EC-001 combined.
#[test]
fn test_BC_7_03_043_ec001_both_message_fields_absent_warns_empty_with_unknown_agent() {
    // Payload with NO last_assistant_message, NO result, NO agent_type, NO subagent_name
    let payload = make_payload(&base_subagentstop(""));

    let mut emitted_reason: Option<String> = None;
    let mut emitted_agent: Option<String> = None;
    let mut stderr_written = false;

    handoff_validator_logic(
        payload,
        |_, fields| {
            emitted_reason = fields
                .iter()
                .find(|(k, _)| *k == "reason")
                .map(|(_, v)| v.to_string());
            emitted_agent = fields
                .iter()
                .find(|(k, _)| *k == "subagent")
                .map(|(_, v)| v.to_string());
        },
        |_| {
            stderr_written = true;
        },
    );

    assert_eq!(
        emitted_reason.as_deref(),
        Some("subagent_empty_result"),
        "EC-001: both message fields absent must emit subagent_empty_result"
    );
    assert_eq!(
        emitted_agent.as_deref(),
        Some("unknown"),
        "EC-001: both identity fields absent → agent must be 'unknown' \
         (BC-2.02.012 Postcondition 5 final unwrap_or)"
    );
    assert!(stderr_written, "EC-001: stderr warning must be written");
}

// ===========================================================================
// EC-003 / BC-7.03.042 postcondition 2: Exactly 40 chars at boundary
// ===========================================================================

/// BC-7.03.042 / AC-005 case (e): Exactly 40 non-whitespace characters →
/// Sufficient → no event emitted, no stderr.
/// Tests the off-by-one parity with bash `(( LEN < 40 ))`.
/// classify_result("a"*40) already tests this in lib.rs, but this test
/// verifies it through the full handoff_validator_logic path.
#[test]
fn test_BC_7_03_042_ec003_exactly_40_chars_is_sufficient_no_event_no_stderr() {
    let s = "a".repeat(40);
    let payload = make_payload(&base_subagentstop(&format!(
        r#""last_assistant_message":"{}""#, s
    )));

    let mut emitted = false;
    let mut stderr_written = false;
    let result = handoff_validator_logic(
        payload,
        |_, _| { emitted = true; },
        |_| { stderr_written = true; },
    );

    assert!(!emitted, "EC-003: 40-char result must NOT emit any event (threshold is < 40)");
    assert!(!stderr_written, "EC-003: 40-char result must NOT write to stderr");
    assert_eq!(result, HookResult::Continue, "EC-003: must return Continue");
}

// ===========================================================================
// EC-005 / BC-2.02.012 EC-005: Non-SubagentStop event passthrough
// ===========================================================================

/// BC-2.02.012 EC-005: A PreToolUse envelope has all four SubagentStop fields
/// absent (None). handoff_validator_logic must handle this gracefully — no event
/// emitted (because the empty-string message path still emits a warning, but
/// this test documents that the HOOK ITSELF is registered only for SubagentStop;
/// the pure logic function should be resilient to non-SubagentStop payloads).
///
/// NOTE: In production the dispatcher routing will NEVER send a PreToolUse event
/// to this hook (registry binding is SubagentStop only). This test verifies the
/// pure logic behaves predictably when called with a non-SubagentStop envelope.
///
/// Since last_assistant_message will be None and result will be None, the 2-stage
/// chain resolves to "" — the empty path fires. This is acceptable behavior for
/// the pure function tested in isolation. The dispatcher routing is the real guard.
#[test]
fn test_BC_2_02_012_ec005_non_subagentstop_payload_pure_logic_returns_continue() {
    let payload = make_payload(
        r#"{"event_name":"PreToolUse","tool_name":"Bash","session_id":"s",
           "dispatcher_trace_id":"t","tool_input":{"command":"git status"}}"#,
    );

    // Pure logic with a PreToolUse payload: all SubagentStop fields are None.
    // The message resolves to "" → empty warn fires. Return is still Continue.
    let result = handoff_validator_logic(
        payload,
        |_, _| {}, // ignore event emission
        |_| {},    // ignore stderr
    );

    assert_eq!(
        result,
        HookResult::Continue,
        "EC-005: handoff_validator_logic must always return Continue (advisory hook)"
    );
}

// ===========================================================================
// BC-2.02.012 Postcondition 6 / AC-003: result field fallback for message
// Short result via `result` field (not last_assistant_message)
// ===========================================================================

/// BC-2.02.012 Postcondition 6 / AC-003: When last_assistant_message is absent
/// but result is present and short (5 chars), the 2-stage fallback chain must
/// pick up `result` and emit truncated_result warning with correct result_len.
#[test]
fn test_BC_7_03_044_result_field_fallback_short_value_emits_truncated_warning() {
    // last_assistant_message absent, result = "hello" (5 chars)
    let payload = make_payload(&base_subagentstop(
        r#""agent_type":"my-agent","result":"hello""#,
    ));

    let mut emitted_reason: Option<String> = None;
    let mut emitted_len: Option<String> = None;

    handoff_validator_logic(
        payload,
        |_, fields| {
            emitted_reason = fields
                .iter()
                .find(|(k, _)| *k == "reason")
                .map(|(_, v)| v.to_string());
            emitted_len = fields
                .iter()
                .find(|(k, _)| *k == "result_len")
                .map(|(_, v)| v.to_string());
        },
        |_| {},
    );

    assert_eq!(
        emitted_reason.as_deref(),
        Some("subagent_truncated_result"),
        "BC-2.02.012 PC-6: short `result` field must trigger truncated_result \
         via 2-stage fallback chain"
    );
    assert_eq!(
        emitted_len.as_deref(),
        Some("5"),
        "BC-2.02.012 PC-6: result_len must reflect the short `result` field length"
    );
}

// ===========================================================================
// BC-2.02.012 Postcondition 6 / AC-003: result field fallback for empty message
// Empty result via `result` field (not last_assistant_message)
// ===========================================================================

/// BC-2.02.012 Postcondition 6 / AC-003: When last_assistant_message is absent
/// but result is present and empty, the 2-stage fallback resolves to "" → empty warn.
#[test]
fn test_BC_7_03_043_result_field_fallback_empty_value_emits_empty_warning() {
    let payload = make_payload(&base_subagentstop(
        r#""agent_type":"my-agent","result":"""#,
    ));

    let mut emitted_reason: Option<String> = None;
    handoff_validator_logic(
        payload,
        |_, fields| {
            emitted_reason = fields
                .iter()
                .find(|(k, _)| *k == "reason")
                .map(|(_, v)| v.to_string());
        },
        |_| {},
    );

    assert_eq!(
        emitted_reason.as_deref(),
        Some("subagent_empty_result"),
        "BC-2.02.012 PC-6: empty `result` field must trigger subagent_empty_result \
         via 2-stage fallback chain"
    );
}

// ===========================================================================
// AC-006 / BC-7.03.042 invariant 2: Graceful JSON parse failure
// This test passes immediately (serde handles it) — included as a regression gate.
// ===========================================================================

/// AC-006 / BC-7.03.042 invariant 2: Malformed JSON input must not panic.
/// The hook deserializes via serde_json; a parse error gracefully exits 0 (main.rs).
/// This test checks the pure deserialization path returns Err (handled in main.rs).
#[test]
fn test_BC_7_03_042_ac006_malformed_json_is_graceful_error_not_panic() {
    let result: Result<HookPayload, _> = serde_json::from_str("{{not valid json");
    assert!(
        result.is_err(),
        "AC-006: malformed JSON must return a serde error (main.rs handles it as exit 0)"
    );
}

/// AC-006: Completely empty input "" → serde error (graceful, exit 0 in main.rs).
#[test]
fn test_BC_7_03_042_ac006_empty_input_is_graceful_error() {
    let result: Result<HookPayload, _> = serde_json::from_str("");
    assert!(
        result.is_err(),
        "AC-006: empty input must return a serde error (graceful degradation, exit 0)"
    );
}

// ===========================================================================
// BC-7.03.043 + BC-7.03.044 postcondition 2: hook always returns Continue
// This is a behavioral parity contract covering all three result paths.
// ===========================================================================

/// BC-7.03.043 + BC-7.03.044 + BC-7.03.042: handoff_validator_logic ALWAYS
/// returns HookResult::Continue regardless of result length.
/// The hook is advisory-only; it never hard-blocks. Parity with bash exit 0.
#[test]
fn test_BC_7_03_042_invariant_hook_always_returns_continue_for_all_lengths() {
    let s39 = "a".repeat(39);
    let s40 = "a".repeat(40);
    let s50 = "a".repeat(50);
    let s100 = "a".repeat(100);

    let cases: Vec<(&str, &str)> = vec![
        // (description, last_assistant_message)
        ("empty", ""),
        // whitespace-only uses spaces only (JSON-safe; tab is a control char in JSON)
        ("whitespace_only", "     "),
        ("short_1_char", "x"),
        ("short_39_chars", &s39),
        ("at_boundary_40", &s40),
        ("sufficient_50", &s50),
        ("sufficient_100", &s100),
    ];

    for (desc, msg) in cases {
        let json = base_subagentstop(&format!(
            r#""last_assistant_message":"{}""#,
            msg.replace('\\', "\\\\").replace('"', "\\\"")
        ));
        let payload = make_payload(&json);
        let result = handoff_validator_logic(payload, |_, _| {}, |_| {});
        assert_eq!(
            result,
            HookResult::Continue,
            "BC-7.03.042: hook must return Continue for case '{desc}' (advisory-only)"
        );
    }
}
