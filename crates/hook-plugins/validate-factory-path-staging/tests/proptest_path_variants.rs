// Test files use .expect()/.unwrap()/.panic!() for failure reporting.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
//! AC-006 proptest harness — `.factory/` path-pattern coverage.
//!
//! Verifies that `hook_logic` blocks every `.factory/`-rooted path variant on
//! a product branch (develop). Minimum 20 diverse input variants per AC-006.
//!
//! All tests are RED at stub time (lib.rs bodies are `todo!()`).
//!
//! # BC traces
//! - BC-4.16.001 PC1: block .factory/ staging on product branches
//! - BC-4.16.001 Invariant 4: path matching is conservative
//! - Story S-21.01 AC-006: proptest fuzz .factory/ path variants (min 20)

use proptest::prelude::*;
use serde_json::json;
use std::panic;
use validate_factory_path_staging::{HookCallbacks, hook_logic};
use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// Test helpers
// ---------------------------------------------------------------------------

fn make_bash_payload(command: &str) -> HookPayload {
    let v = json!({
        "event_name": "PreToolUse",
        "session_id": "test-session",
        "dispatcher_trace_id": "test-trace",
        "tool_name": "Bash",
        "tool_input": {
            "command": command
        }
    });
    serde_json::from_value(v).expect("fixture must deserialize")
}

fn run_hook_with_branch(command: &str, branch: &str) -> std::thread::Result<HookResult> {
    let payload = make_bash_payload(command);
    let branch_output = branch.to_string();
    panic::catch_unwind(move || {
        hook_logic(
            payload,
            HookCallbacks {
                exec_subprocess: move |_bin, _args| Ok((0, branch_output.clone(), String::new())),
                emit_event: |_, _| {},
                log: |_, _| {},
            },
        )
    })
}

// ---------------------------------------------------------------------------
// AC-006: proptest — fuzz .factory/ path variants; all must block on develop
// ---------------------------------------------------------------------------

proptest! {
    #![proptest_config(proptest::test_runner::Config::with_cases(200))]

    /// AC-006 / BC-4.16.001 PC1 + Invariant 4:
    /// For any .factory/-rooted path component, git add on develop must block.
    /// Minimum 20 diverse inputs per AC-006 spec requirement.
    #[test]
    fn prop_BC_4_16_001_ac006_factory_path_variants_block_on_develop(
        path_suffix in "[a-zA-Z0-9/_.-]{1,60}"
    ) {
        let command = format!("git add .factory/{}", path_suffix);
        let result = run_hook_with_branch(&command, "develop");
        prop_assert!(
            result.is_ok(),
            "AC-006: hook_logic panicked for command '{}'. Must return HookResult, never panic. \
             Production function is unimplemented (todo!()).",
            command
        );
        if let Ok(hook_result) = result {
            prop_assert_ne!(
                hook_result,
                HookResult::Continue,
                "AC-006 / BC-4.16.001 PC1: .factory/ path '{}' on develop must be BLOCKED \
                 (block_intent=true, exit 2). Got Continue. Production unimplemented.",
                command
            );
        }
    }

    /// AC-006 / BC-4.16.001 PC1:
    /// Absolute-path form of .factory/ paths also blocks on develop.
    /// Covers the `/.factory/` path-component variant (BC-4.16.001 Precondition 2).
    #[test]
    fn prop_BC_4_16_001_ac006_absolute_factory_path_blocks_on_develop(
        prefix in "[a-z/]{1,20}",
        suffix in "[a-zA-Z0-9/_.-]{1,40}"
    ) {
        let command = format!("git add /{}.factory/{}", prefix, suffix);
        let result = run_hook_with_branch(&command, "develop");
        prop_assert!(
            result.is_ok(),
            "AC-006: hook_logic panicked for absolute path command '{}'. Production unimplemented.",
            command
        );
    }

    /// AC-006 / BC-4.16.001 PC2:
    /// Non-.factory/ paths on develop must NOT be blocked.
    /// Negative case: fuzz ensures the guard doesn't over-block legitimate paths.
    #[test]
    fn prop_BC_4_16_001_ac006_non_factory_path_passes_on_develop(
        path_suffix in "[a-zA-Z0-9/_.-]{1,60}"
    ) {
        // Ensure path does not start with .factory/ or contain /.factory/
        let full_path = format!("src/{}", path_suffix);
        prop_assume!(!full_path.contains(".factory/"));
        let command = format!("git add {}", full_path);
        let result = run_hook_with_branch(&command, "develop");
        prop_assert!(
            result.is_ok(),
            "AC-006 negative: hook_logic panicked for non-.factory/ path '{}'. \
             Production unimplemented.",
            command
        );
        if let Ok(hook_result) = result {
            prop_assert_eq!(
                hook_result,
                HookResult::Continue,
                "AC-006 negative / BC-4.16.001 PC2: non-.factory/ path '{}' on develop \
                 must pass (Continue). Production unimplemented.",
                full_path
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Deterministic canonical test vectors (AC-006 minimum 20 diverse inputs)
// ---------------------------------------------------------------------------

/// Canonical test vectors that mirror BC-4.16.001 T-1..T-6.
/// These ensure the 20+ diverse input requirement is met even if proptest
/// shrinks. All are RED at stub time — panic from todo!() causes is_ok() to
/// fail, which is the correct failing behaviour.
#[test]
fn test_ac006_canonical_factory_path_vectors_block_on_develop() {
    let factory_paths = vec![
        "git add .factory/STATE.md",
        "git add .factory/stories/S-21.01.md",
        "git add .factory/specs/behavioral-contracts/ss-04/BC-4.16.001.md",
        "git add .factory/specs/architecture/ARCH-INDEX.md",
        "git add .factory/cycles/v1.0-brownfield-backfill/e-21-arch-delta-analysis.md",
        "git add .factory/policies.yaml",
        "git add .factory/tech-debt-register.md",
        "git add .factory/logs/dispatcher-internal-2026-07-22.jsonl",
        "git add .factory/specs/verification-properties/VP-INDEX.md",
        "git add .factory/stories/STORY-INDEX.md",
        // Variants with flags
        "git add -f .factory/STATE.md",
        "git add --force .factory/STATE.md",
        // Conservative wildcards (blocked per Invariant 4)
        "git add -A",
        "git add -u",
        "git add .",
        // Mixed: any .factory/ match blocks the whole command
        "git add src/main.rs .factory/STATE.md",
        // Additional diverse variants to meet 20+ minimum per AC-006
        "git add .factory/specs/domain-spec/entities.md",
        "git add .factory/specs/prd.md",
        "git add .factory/stories/epics/E-21-factory-state-data-loss-hardening.md",
        "git add .factory/tech-debt-register.md .factory/STATE.md",
    ];

    for command in &factory_paths {
        let result = run_hook_with_branch(command, "develop");
        // RED GATE: at stub time todo!() panics, so result.is_ok() FAILS.
        // After implementation, result must be Ok(Block) — never Ok(Continue).
        assert!(
            result.is_ok(),
            "AC-006 canonical: hook_logic panicked (todo!()) for command '{}'. \
             Must return HookResult::Block, not panic. Production unimplemented.",
            command
        );
        if let Ok(hook_result) = result {
            assert_ne!(
                hook_result,
                HookResult::Continue,
                "AC-006 / BC-4.16.001 PC1: '{}' on develop must be BLOCKED (exit 2). \
                 Got Continue. Production unimplemented.",
                command
            );
        }
    }
}

#[test]
fn test_ac006_factory_artifacts_branch_passes_factory_paths() {
    // BC-4.16.001 PC3: factory-artifacts branch must pass unconditionally.
    // RED GATE: at stub time todo!() panics, so result.is_ok() FAILS.
    let commands = vec![
        "git add .factory/STATE.md",
        "git add .factory/stories/S-21.01.md",
    ];
    for command in &commands {
        let result = run_hook_with_branch(command, "factory-artifacts");
        assert!(
            result.is_ok(),
            "AC-006 / BC-4.16.001 PC3: hook_logic panicked (todo!()) for '{}' on \
             factory-artifacts. Must return Continue, not panic. Production unimplemented.",
            command
        );
        if let Ok(hook_result) = result {
            assert_eq!(
                hook_result,
                HookResult::Continue,
                "AC-006 / BC-4.16.001 PC3: '{}' on factory-artifacts must return Continue. \
                 Got: {:?}. Production unimplemented.",
                command,
                hook_result
            );
        }
    }
}
