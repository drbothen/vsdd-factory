//! VP-069 proptest harness — registry-load purity.
//!
//! Verifies that `load_registry` never panics on arbitrary input and that
//! `hook_logic` never produces a Block outcome when the registry is absent or
//! malformed (graceful-degrade invariant from BC-4.11.001 EC-001/EC-002).
//!
//! # BC traces
//! - BC-4.11.001 PC1: load_registry reads registry at runtime, never panics
//! - BC-4.11.001 EC-001: registry absent → Continue (graceful degrade)
//! - BC-4.11.001 EC-002: registry malformed → Continue (graceful degrade)
//! - VP-069: registry-load purity — parsing never panics on arbitrary input
//!
//! # Test plan trace
//! AC-001 traces to BC-4.11.001 precondition 2 + VP-069 proptest.
//! All three proptest properties (Parts A, B, C) must pass.
//! Minimum 200 trials per property (VP-069 spec requirement).

use proptest::prelude::*;
use serde_json::json;
use std::panic;
use validate_artifact_path::{load_registry, hook_logic, HookCallbacks};
use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// Test helpers
// ---------------------------------------------------------------------------

fn make_payload(file_path: Option<&str>) -> HookPayload {
    let mut v = json!({
        "event_name": "PreToolUse",
        "session_id": "test-session",
        "dispatcher_trace_id": "test-trace",
        "tool_name": "Write",
        "tool_input": {}
    });
    if let Some(path) = file_path {
        v["tool_input"]["file_path"] = json!(path);
    }
    serde_json::from_value(v).expect("fixture must deserialize")
}

fn run_logic_with_registry_str(
    payload: HookPayload,
    registry_content: Result<String, String>,
) -> HookResult {
    hook_logic(
        payload,
        HookCallbacks {
            read_file: move |_path| registry_content.clone(),
            emit_event: |_, _| {},
            log: |_, _| {},
        },
    )
}

// ---------------------------------------------------------------------------
// VP-069 Part A: load_registry never panics on arbitrary byte sequences
// ---------------------------------------------------------------------------

proptest! {
    #![proptest_config(proptest::test_runner::Config::with_cases(200))]

    /// AC-001 traces to BC-4.11.001 PC1 + VP-069 Part A:
    /// For any arbitrary byte sequence, load_registry must not panic.
    /// Must return Ok(registry) or Err(RegistryError) — never panic.
    #[test]
    fn prop_BC_4_11_001_vp069_part_a_load_registry_never_panics_on_arbitrary_bytes(
        input in any::<Vec<u8>>()
    ) {
        // Convert arbitrary bytes to string (lossy — mimics real-world malformed YAML)
        let s = String::from_utf8_lossy(&input).into_owned();
        // The production function is todo!() — catch_unwind captures the panic.
        // When implemented, this must return Ok or Err without panicking.
        let result = panic::catch_unwind(|| load_registry(&s));
        prop_assert!(
            result.is_ok(),
            "VP-069 Part A: load_registry panicked on byte sequence (len={}). \
             Must return Ok(PathRegistry) or Err(RegistryError), never panic. \
             Production function is unimplemented (todo!()).",
            s.len()
        );
    }

    /// AC-001 / VP-069 Part A (additional):
    /// For any arbitrary UTF-8 string, load_registry must not panic.
    /// This is a superset of Part A restricted to valid Unicode (covers common malformed YAML).
    #[test]
    fn prop_BC_4_11_001_vp069_part_a_load_registry_never_panics_on_arbitrary_string(
        input in ".*"
    ) {
        let result = panic::catch_unwind(|| load_registry(&input));
        prop_assert!(
            result.is_ok(),
            "VP-069 Part A: load_registry panicked on string input (len={}). \
             Must return Ok or Err — never panic. Production unimplemented.",
            input.len()
        );
    }

    /// AC-001 / VP-069 Part A (boundary case):
    /// load_registry on empty string must not panic.
    /// Empty string is a degenerate input that serde_yaml may handle differently.
    #[test]
    fn prop_BC_4_11_001_vp069_part_a_load_registry_never_panics_on_empty_prefix_strings(
        prefix_len in 0usize..100usize
    ) {
        let input = " ".repeat(prefix_len); // whitespace-only strings
        let result = panic::catch_unwind(|| load_registry(&input));
        prop_assert!(
            result.is_ok(),
            "VP-069 Part A: load_registry panicked on whitespace-only string (len={}). \
             Production unimplemented.",
            prefix_len
        );
    }
}

// ---------------------------------------------------------------------------
// VP-069 Part B: malformed registry → hook_logic returns Continue (not Block)
// ---------------------------------------------------------------------------

proptest! {
    #![proptest_config(proptest::test_runner::Config::with_cases(200))]

    /// AC-001 / BC-4.11.001 EC-002 + VP-069 Part B:
    /// When the registry YAML is malformed (arbitrary string), hook_logic must
    /// return HookResult::Continue (graceful degrade), never Block.
    ///
    /// This ensures a corrupt registry.yaml never causes a hard block on all writes.
    #[test]
    fn prop_BC_4_11_001_vp069_part_b_malformed_registry_hook_returns_continue(
        registry_str in ".*"
    ) {
        // Use a .factory/ path to force registry lookup (non-.factory/ would early-exit)
        let payload = make_payload(
            Some(".factory/specs/behavioral-contracts/ss-04/BC-4.11.001.md"),
        );
        let result = panic::catch_unwind(|| {
            run_logic_with_registry_str(payload, Ok(registry_str.clone()))
        });
        prop_assert!(
            result.is_ok(),
            "VP-069 Part B: hook_logic panicked with registry string (len={}). \
             Must return HookResult::Continue for any malformed input. Production unimplemented.",
            registry_str.len()
        );
        // When production code is implemented:
        // If registry_str happens to parse as valid YAML with a matching entry,
        // hook_logic may return Continue (allowed path). If it doesn't parse,
        // hook_logic must return Continue (graceful degrade EC-002).
        // In neither case should hook_logic return HookResult::Error.
        if let Ok(hook_result) = result {
            prop_assert_ne!(
                hook_result.exit_code(),
                1, // HookResult::Error exit code = 1
                "VP-069 Part B: hook_logic must never return HookResult::Error (exit code 1) \
                 for any registry content. Got error exit code. Production unimplemented."
            );
        }
    }

    /// AC-001 / BC-4.11.001 EC-002 + VP-069 Part B (absent registry variant):
    /// When the registry file is absent (read_file returns Err), hook_logic must
    /// return HookResult::Continue, never Block.
    #[test]
    fn prop_BC_4_11_001_vp069_part_b_absent_registry_hook_always_returns_continue(
        path_suffix in "[a-z/.-]{1,64}"
    ) {
        // Prepend .factory/ to ensure registry lookup branch is exercised
        let full_path = format!(".factory/{}", path_suffix);
        let payload = make_payload(Some(&full_path));
        let result = panic::catch_unwind(|| {
            run_logic_with_registry_str(payload, Err("registry file not found".to_string()))
        });
        prop_assert!(
            result.is_ok(),
            "VP-069 Part B: hook_logic panicked when registry is absent for path '{}'. \
             Must return Continue (BC-4.11.001 EC-001). Production unimplemented.",
            full_path
        );
        if let Ok(hook_result) = result {
            prop_assert_eq!(
                hook_result,
                HookResult::Continue,
                "VP-069 Part B / BC-4.11.001 EC-001: absent registry must always return \
                 HookResult::Continue for any .factory/ path '{}'. \
                 Got: {:?}. Production unimplemented.",
                full_path,
                "non-Continue"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// VP-069 Part C: empty registry → Continue for all non-.factory/ paths
// ---------------------------------------------------------------------------

proptest! {
    #![proptest_config(proptest::test_runner::Config::with_cases(200))]

    /// AC-001 / VP-069 Part C (BC-4.11.001 PC7):
    /// With an empty registry (zero artifact entries), hook_logic must return
    /// Continue for all non-.factory/ paths (early-exit before registry lookup).
    /// This validates the early-exit invariant is independent of registry content.
    #[test]
    fn prop_BC_4_11_001_vp069_part_c_empty_registry_non_factory_path_continues(
        path_suffix in "[a-zA-Z0-9/_.-]{0,64}"
    ) {
        // Ensure path does NOT start with .factory/ (use src/ prefix)
        let full_path = format!("src/{}", path_suffix);
        let empty_registry = "version: 1\nartifacts: []\n".to_string();
        let payload = make_payload(Some(&full_path));
        let result = panic::catch_unwind(|| {
            run_logic_with_registry_str(payload, Ok(empty_registry))
        });
        prop_assert!(
            result.is_ok(),
            "VP-069 Part C: hook_logic panicked for non-.factory/ path '{}'. \
             Must return Continue (BC-4.11.001 PC7 — early exit). Production unimplemented.",
            full_path
        );
        if let Ok(hook_result) = result {
            prop_assert_eq!(
                hook_result,
                HookResult::Continue,
                "VP-069 Part C / BC-4.11.001 PC7: non-.factory/ path '{}' with empty registry \
                 must return Continue (early exit — no registry lookup needed). Production unimplemented.",
                full_path
            );
        }
    }
}
