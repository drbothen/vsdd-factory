// Test files use .expect()/.unwrap()/.panic!() for failure reporting.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
//! Unit tests for validate-factory-path-staging.
//!
//! Exercises the production functions declared in `lib.rs` via injectable
//! callbacks and direct pure-function calls. All tests are RED at stub time
//! because lib.rs bodies are `todo!()`.
//!
//! # BC traces
//! - BC-4.16.001 PC1: block .factory/ staging on product branches
//! - BC-4.16.001 PC2: pass non-.factory/ git add on product branches
//! - BC-4.16.001 PC3: pass all commands on factory-artifacts branch
//! - BC-4.16.001 PC4: pass non-git-add commands unconditionally
//! - BC-4.16.001 Invariant 3: fail-open on branch detection failure
//! - BC-5.43.001 PC1: merge pre-check passes on clean diff
//! - BC-5.43.001 PC2: merge pre-check halts on .factory/ path in diff
//! - BC-5.43.001 Invariant 4: fail-open when git diff fails

use crate::{
    HookCallbacks, contains_factory_path_arg, hook_logic, is_git_add_command, is_product_branch,
};
use serde_json::json;
use std::panic;
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

/// Run hook_logic with a mocked branch from exec_subprocess returning exit 0 + branch name.
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

/// Run hook_logic with exec_subprocess returning Err — simulates git unavailable.
fn run_hook_branch_detection_err(command: &str) -> std::thread::Result<HookResult> {
    let payload = make_bash_payload(command);
    panic::catch_unwind(move || {
        hook_logic(
            payload,
            HookCallbacks {
                exec_subprocess: |_bin, _args| Err("git: command not found".to_string()),
                emit_event: |_, _| {},
                log: |_, _| {},
            },
        )
    })
}

/// Run hook_logic with exec_subprocess returning non-zero exit — simulates detached HEAD.
fn run_hook_branch_detection_nonzero(command: &str) -> std::thread::Result<HookResult> {
    let payload = make_bash_payload(command);
    panic::catch_unwind(move || {
        hook_logic(
            payload,
            HookCallbacks {
                exec_subprocess: |_bin, _args| {
                    Ok((
                        128,
                        String::new(),
                        "fatal: not a git repository".to_string(),
                    ))
                },
                emit_event: |_, _| {},
                log: |_, _| {},
            },
        )
    })
}

/// Walk up from CARGO_MANIFEST_DIR until per-story-delivery.md is found.
fn per_story_delivery_md_path() -> std::path::PathBuf {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR must be set during cargo test");
    let mut dir = std::path::PathBuf::from(&manifest_dir);
    loop {
        let candidate = dir.join("plugins/vsdd-factory/agents/orchestrator/per-story-delivery.md");
        if candidate.exists() {
            return candidate;
        }
        if !dir.pop() {
            panic!(
                "could not locate plugins/vsdd-factory/agents/orchestrator/per-story-delivery.md \
                 walking up from CARGO_MANIFEST_DIR ({manifest_dir})"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// is_git_add_command (BC-4.16.001 Precondition 2 / PC4)
// ---------------------------------------------------------------------------

#[test]
fn test_is_git_add_command_detects_git_add() {
    assert!(is_git_add_command("git add .factory/STATE.md"));
}

#[test]
fn test_is_git_add_command_detects_git_add_flags() {
    assert!(is_git_add_command("git add -A"));
    assert!(is_git_add_command("git add -u"));
    assert!(is_git_add_command("git add ."));
}

#[test]
fn test_is_git_add_command_passes_non_git_add() {
    assert!(!is_git_add_command("git commit -m \"test\""));
    assert!(!is_git_add_command("git push origin develop"));
    assert!(!is_git_add_command("git merge feature/S-21.01"));
    assert!(!is_git_add_command("ls -la .factory/"));
}

// ---------------------------------------------------------------------------
// is_product_branch (BC-4.16.001 PC1 / PC3)
// ---------------------------------------------------------------------------

#[test]
fn test_is_product_branch_develop_is_product() {
    assert!(is_product_branch("develop"));
}

#[test]
fn test_is_product_branch_main_is_product() {
    assert!(is_product_branch("main"));
}

#[test]
fn test_is_product_branch_feature_is_product() {
    assert!(is_product_branch("feature/S-21.01"));
}

#[test]
fn test_is_product_branch_release_is_product() {
    assert!(is_product_branch("release/v1.0.0-rc.24"));
}

#[test]
fn test_is_product_branch_maintenance_is_product() {
    assert!(is_product_branch("maintenance/hotfix-001"));
}

#[test]
fn test_is_product_branch_factory_artifacts_is_not_product() {
    assert!(!is_product_branch("factory-artifacts"));
}

// ---------------------------------------------------------------------------
// contains_factory_path_arg (BC-4.16.001 Invariant 4)
// ---------------------------------------------------------------------------

#[test]
fn test_contains_factory_path_arg_detects_factory_path() {
    assert!(contains_factory_path_arg("git add .factory/STATE.md"));
    assert!(contains_factory_path_arg(
        "git add .factory/stories/S-21.01.md"
    ));
}

#[test]
fn test_contains_factory_path_arg_detects_conservative_flags() {
    // Conservative: -A, -u treated as potentially staging .factory/ content
    assert!(contains_factory_path_arg("git add -A"));
    assert!(contains_factory_path_arg("git add -u"));
}

#[test]
fn test_contains_factory_path_arg_passes_non_factory_path() {
    assert!(!contains_factory_path_arg("git add src/main.rs"));
    assert!(!contains_factory_path_arg(
        "git add crates/hook-sdk/src/lib.rs"
    ));
}

// ---------------------------------------------------------------------------
// AC-001 (BC-4.16.001 PC1, Invariant 1):
// hook_logic BLOCKS git add .factory/<path> on product branches
// ---------------------------------------------------------------------------

#[test]
fn test_ac001_bc4_16_001_pc1_blocks_factory_path_on_develop() {
    let result = run_hook_with_branch("git add .factory/STATE.md", "develop");
    assert!(
        result.is_ok(),
        "AC-001: hook_logic panicked (todo!()) for 'git add .factory/STATE.md' on develop. \
         BC-4.16.001 PC1: must return HookResult::Block with FactoryPathOnProductBranch. \
         Production function is unimplemented."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "AC-001 / BC-4.16.001 PC1: 'git add .factory/STATE.md' on develop must exit 2 \
             (block_intent=true). Got exit code {}.",
            hook_result.exit_code()
        );
        match &hook_result {
            HookResult::Block { reason } => {
                assert!(
                    reason.contains("FactoryPathOnProductBranch"),
                    "AC-001 / BC-4.16.001 PC1: block reason must contain 'FactoryPathOnProductBranch' \
                     error variant. Got: '{}'",
                    reason
                );
            }
            other => panic!(
                "AC-001 / BC-4.16.001 PC1: expected HookResult::Block, got {:?}",
                other
            ),
        }
    }
}

#[test]
fn test_ac001_bc4_16_001_pc1_blocks_factory_path_on_main() {
    let result = run_hook_with_branch("git add .factory/STATE.md", "main");
    assert!(
        result.is_ok(),
        "AC-001: hook_logic panicked for 'git add .factory/STATE.md' on main. \
         BC-4.16.001 PC1: main is a product branch — must block. Production unimplemented."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "AC-001 / BC-4.16.001 PC1: 'git add .factory/STATE.md' on main must exit 2."
        );
    }
}

#[test]
fn test_ac001_bc4_16_001_pc1_blocks_factory_path_on_feature_branch() {
    let result = run_hook_with_branch("git add .factory/stories/S-21.01.md", "feature/S-21.01");
    assert!(
        result.is_ok(),
        "AC-001: hook_logic panicked for feature branch. BC-4.16.001 PC1: \
         feature/* is a product branch — must block .factory/ path. Production unimplemented."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "AC-001 / BC-4.16.001 PC1: 'git add .factory/stories/S-21.01.md' on feature/* \
             must exit 2."
        );
    }
}

#[test]
fn test_ac001_bc4_16_001_pc1_blocks_factory_path_on_release_branch() {
    // BC-4.16.001 T-5: release/v1.0.0-rc.24 is a product branch
    let result = run_hook_with_branch(
        "git add .factory/stories/S-21.01.md",
        "release/v1.0.0-rc.24",
    );
    assert!(
        result.is_ok(),
        "AC-001 / BC-4.16.001 T-5: hook_logic panicked for release branch. \
         release/v1.0.0-rc.24 is a product branch — must block. Production unimplemented."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "AC-001 / BC-4.16.001 T-5: 'git add .factory/*' on release/* must exit 2."
        );
    }
}

#[test]
fn test_ac001_bc4_16_001_pc1_block_reason_contains_branch_name() {
    // BC-4.16.001 PC1 canonical block message includes '<branch>' placeholder
    let result = run_hook_with_branch("git add .factory/STATE.md", "develop");
    assert!(
        result.is_ok(),
        "AC-001: hook_logic panicked (todo!()). Production unimplemented."
    );
    if let Ok(HookResult::Block { reason }) = result {
        assert!(
            reason.contains("develop"),
            "AC-001 / BC-4.16.001 PC1: block reason must include the branch name 'develop'. \
             Got: '{}'",
            reason
        );
    }
}

#[test]
fn test_ac001_bc4_16_001_pc1_block_reason_contains_factory_artifacts_guidance() {
    // BC-4.16.001 PC1 canonical error message directs user to factory-artifacts branch
    let result = run_hook_with_branch("git add .factory/STATE.md", "develop");
    assert!(
        result.is_ok(),
        "AC-001: hook_logic panicked. Production unimplemented."
    );
    if let Ok(HookResult::Block { reason }) = result {
        assert!(
            reason.contains("factory-artifacts"),
            "AC-001 / BC-4.16.001 PC1: block reason must mention 'factory-artifacts' branch \
             (canonical fix guidance per PC1 message). Got: '{}'",
            reason
        );
    }
}

// ---------------------------------------------------------------------------
// AC-002 (BC-4.16.001 PC3):
// hook_logic PASSES on factory-artifacts branch regardless of path
// ---------------------------------------------------------------------------

#[test]
fn test_ac002_bc4_16_001_pc3_passes_factory_state_md_on_factory_artifacts() {
    let result = run_hook_with_branch("git add .factory/STATE.md", "factory-artifacts");
    assert!(
        result.is_ok(),
        "AC-002: hook_logic panicked for 'git add .factory/STATE.md' on factory-artifacts. \
         BC-4.16.001 PC3: factory-artifacts branch must pass unconditionally. \
         Production unimplemented."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result,
            HookResult::Continue,
            "AC-002 / BC-4.16.001 PC3: 'git add .factory/STATE.md' on factory-artifacts must \
             return Continue. Factory artifact commits require staging .factory/ paths on this \
             branch. Got: {:?}.",
            hook_result
        );
    }
}

#[test]
fn test_ac002_bc4_16_001_pc3_factory_artifacts_passes_deep_factory_path() {
    // BC-4.16.001 PC3: unconditional pass on factory-artifacts for any .factory/ path
    let result = run_hook_with_branch(
        "git add .factory/cycles/v1.0-brownfield-backfill/burst-log.md",
        "factory-artifacts",
    );
    assert!(
        result.is_ok(),
        "AC-002: hook_logic panicked for deep .factory/ path on factory-artifacts. \
         BC-4.16.001 PC3: must pass unconditionally. Production unimplemented."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result,
            HookResult::Continue,
            "AC-002 / BC-4.16.001 PC3: deep .factory/ cycle-doc path on factory-artifacts \
             must Continue."
        );
    }
}

// ---------------------------------------------------------------------------
// AC-003 (BC-4.16.001 PC4):
// Non-git-add commands pass unconditionally (no path inspection)
// ---------------------------------------------------------------------------

#[test]
fn test_ac003_bc4_16_001_pc4_passes_git_commit_on_develop() {
    let result = run_hook_with_branch("git commit -m \"state: advance pass-N\"", "develop");
    assert!(
        result.is_ok(),
        "AC-003: hook_logic panicked for 'git commit'. BC-4.16.001 PC4: \
         non-git-add commands pass unconditionally. Production unimplemented."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result,
            HookResult::Continue,
            "AC-003 / BC-4.16.001 PC4: 'git commit' must return Continue. \
             Only 'git add' commands are in scope for this guard."
        );
    }
}

#[test]
fn test_ac003_bc4_16_001_pc4_passes_git_push_on_develop() {
    let result = run_hook_with_branch("git push origin develop", "develop");
    assert!(
        result.is_ok(),
        "AC-003: hook_logic panicked for 'git push'. BC-4.16.001 PC4. Production unimplemented."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result,
            HookResult::Continue,
            "AC-003 / BC-4.16.001 PC4: 'git push' must return Continue."
        );
    }
}

#[test]
fn test_ac003_bc4_16_001_pc4_passes_git_merge_on_develop() {
    // BC-4.16.001 PC4: git merge is NOT in scope — Layer-2 (BC-5.43.001) guards merges
    let result = run_hook_with_branch("git merge feature/S-21.01", "develop");
    assert!(
        result.is_ok(),
        "AC-003: hook_logic panicked for 'git merge'. BC-4.16.001 PC4. Production unimplemented."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result,
            HookResult::Continue,
            "AC-003 / BC-4.16.001 PC4: 'git merge' must return Continue. \
             Merge is Layer-2 domain (BC-5.43.001); Layer-1 WASM scope is git-add only."
        );
    }
}

#[test]
fn test_ac003_bc4_16_001_ec009_passes_git_commit_amend() {
    // BC-4.16.001 EC-009: git commit --amend (no git add) → PC4
    let result = run_hook_with_branch("git commit --amend --no-edit", "develop");
    assert!(
        result.is_ok(),
        "AC-003 / EC-009: hook_logic panicked for 'git commit --amend'. \
         BC-4.16.001 EC-009: non-git-add passes unconditionally. Production unimplemented."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result,
            HookResult::Continue,
            "AC-003 / BC-4.16.001 EC-009: 'git commit --amend' must return Continue."
        );
    }
}

// ---------------------------------------------------------------------------
// AC-004 (BC-4.16.001 PC2):
// git add of non-.factory/ paths passes unconditionally on product branches
// ---------------------------------------------------------------------------

#[test]
fn test_ac004_bc4_16_001_pc2_passes_src_path_on_develop() {
    let result = run_hook_with_branch("git add src/main.rs", "develop");
    assert!(
        result.is_ok(),
        "AC-004: hook_logic panicked for 'git add src/main.rs' on develop. \
         BC-4.16.001 PC2: non-.factory/ paths pass unconditionally. Production unimplemented."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result,
            HookResult::Continue,
            "AC-004 / BC-4.16.001 PC2: 'git add src/main.rs' on develop must Continue."
        );
    }
}

#[test]
fn test_ac004_bc4_16_001_t3_passes_src_lib_rs_on_feature_branch() {
    // BC-4.16.001 T-3: branch=feature/S-21.01, path=src/lib.rs → PASSED PC2
    let result = run_hook_with_branch(
        "git add crates/hook-plugins/validate-factory-path-staging/src/lib.rs",
        "feature/S-21.01",
    );
    assert!(
        result.is_ok(),
        "AC-004 / BC-4.16.001 T-3: hook_logic panicked for crates/ path on feature branch. \
         PC2: non-.factory/ path must pass. Production unimplemented."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result,
            HookResult::Continue,
            "AC-004 / BC-4.16.001 T-3: crates/ path on feature/* must Continue."
        );
    }
}

#[test]
fn test_ac004_bc4_16_001_pc2_passes_plugins_path_on_main() {
    let result = run_hook_with_branch("git add plugins/vsdd-factory/hooks-registry.toml", "main");
    assert!(
        result.is_ok(),
        "AC-004: hook_logic panicked for plugins path on main. \
         BC-4.16.001 PC2: non-.factory/ path passes. Production unimplemented."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result,
            HookResult::Continue,
            "AC-004 / BC-4.16.001 PC2: plugins/ path on main must Continue."
        );
    }
}

// ---------------------------------------------------------------------------
// AC-005 (BC-4.16.001 Invariant 3):
// Fail-open when branch detection fails (git unavailable, detached HEAD)
// ---------------------------------------------------------------------------

#[test]
fn test_ac005_bc4_16_001_inv3_fail_open_on_exec_subprocess_error() {
    // Simulates git unavailable (exec_subprocess returns Err)
    let result = run_hook_branch_detection_err("git add .factory/STATE.md");
    assert!(
        result.is_ok(),
        "AC-005: hook_logic panicked when branch detection fails (Err from exec_subprocess). \
         BC-4.16.001 Invariant 3: uncertain branch state is NOT a blocking condition — \
         must fail-open (Continue). Production unimplemented."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result,
            HookResult::Continue,
            "AC-005 / BC-4.16.001 Invariant 3: exec_subprocess Err → fail-open Continue. \
             Got: {:?}.",
            hook_result
        );
    }
}

#[test]
fn test_ac005_bc4_16_001_inv3_fail_open_on_nonzero_branch_exit() {
    // Simulates detached HEAD (exit code 128, empty stdout)
    let result = run_hook_branch_detection_nonzero("git add .factory/STATE.md");
    assert!(
        result.is_ok(),
        "AC-005: hook_logic panicked when git branch --show-current returns exit 128. \
         BC-4.16.001 Invariant 3: non-zero exit → fail-open (Continue). \
         Production unimplemented."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result,
            HookResult::Continue,
            "AC-005 / BC-4.16.001 Invariant 3: non-zero branch-detection exit must return \
             Continue (fail-open). Got: {:?}.",
            hook_result
        );
    }
}

// ---------------------------------------------------------------------------
// BC-4.16.001 Edge Cases (EC-NNN)
// ---------------------------------------------------------------------------

#[test]
fn test_bc4_16_001_ec001_blocks_factory_state_md_on_develop() {
    // EC-001: git add .factory/STATE.md on develop → BLOCKED
    let result = run_hook_with_branch("git add .factory/STATE.md", "develop");
    assert!(
        result.is_ok(),
        "EC-001 / BC-4.16.001: hook_logic panicked. Production unimplemented."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "EC-001 / BC-4.16.001: 'git add .factory/STATE.md' on develop must exit 2."
        );
    }
}

#[test]
fn test_bc4_16_001_ec002_passes_factory_state_md_on_factory_artifacts() {
    // EC-002: git add .factory/STATE.md on factory-artifacts → PASSED
    let result = run_hook_with_branch("git add .factory/STATE.md", "factory-artifacts");
    assert!(
        result.is_ok(),
        "EC-002 / BC-4.16.001: hook_logic panicked. Production unimplemented."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result,
            HookResult::Continue,
            "EC-002 / BC-4.16.001: factory-artifacts branch must Continue."
        );
    }
}

#[test]
fn test_bc4_16_001_ec004_git_add_dash_a_conservative_block_on_develop() {
    // EC-004: git add -A on develop → BLOCKED (conservative per Invariant 4)
    let result = run_hook_with_branch("git add -A", "develop");
    assert!(
        result.is_ok(),
        "EC-004 / BC-4.16.001: hook_logic panicked for 'git add -A' on develop. \
         Invariant 4: -A is conservatively blocked. Production unimplemented."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "EC-004 / BC-4.16.001 Invariant 4: 'git add -A' on develop must exit 2 \
             (conservative block — may stage .factory/ content)."
        );
    }
}

#[test]
fn test_bc4_16_001_ec005_mixed_path_blocks_entire_command_on_develop() {
    // EC-005: git add src/main.rs .factory/STATE.md — ANY .factory/ match blocks whole command
    let result = run_hook_with_branch("git add src/main.rs .factory/STATE.md", "develop");
    assert!(
        result.is_ok(),
        "EC-005 / BC-4.16.001: hook_logic panicked for mixed-path git add. \
         Production unimplemented."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "EC-005 / BC-4.16.001: mixed git add with .factory/ arg must exit 2. \
             ANY .factory/ match blocks the entire command."
        );
    }
}

#[test]
fn test_bc4_16_001_ec006_branch_detection_failure_is_fail_open() {
    // EC-006: branch detection fails (detached HEAD) → PASSED (fail-open Invariant 3)
    let result = run_hook_branch_detection_nonzero("git add .factory/STATE.md");
    assert!(
        result.is_ok(),
        "EC-006 / BC-4.16.001: hook_logic panicked on branch-detection failure. \
         Production unimplemented."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result,
            HookResult::Continue,
            "EC-006 / BC-4.16.001 Invariant 3: detached HEAD → fail-open Continue."
        );
    }
}

#[test]
fn test_bc4_16_001_ec008_glob_conservative_block_on_develop() {
    // EC-008: git add *.md from project root on develop → BLOCKED (conservative Invariant 4)
    let result = run_hook_with_branch("git add *.md", "develop");
    assert!(
        result.is_ok(),
        "EC-008 / BC-4.16.001: hook_logic panicked for glob 'git add *.md'. \
         Production unimplemented."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "EC-008 / BC-4.16.001 Invariant 4: 'git add *.md' on develop must exit 2 \
             (conservative block — glob may expand to .factory/**/*.md)."
        );
    }
}

#[test]
fn test_bc4_16_001_ec010_git_add_dash_u_conservative_block_on_feature_branch() {
    // EC-010: git add -u on feature/* → BLOCKED (conservative Invariant 4)
    let result = run_hook_with_branch(
        "git add -u",
        "feature/S-21.01-validate-factory-path-staging",
    );
    assert!(
        result.is_ok(),
        "EC-010 / BC-4.16.001: hook_logic panicked for 'git add -u' on feature branch. \
         Production unimplemented."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "EC-010 / BC-4.16.001 EC-010: 'git add -u' on feature/* must exit 2 \
             (conservative block — -u stages all tracked modifications)."
        );
    }
}

// ---------------------------------------------------------------------------
// BC-4.16.001 T-4 / T-5 canonical test vectors
// ---------------------------------------------------------------------------

#[test]
fn test_bc4_16_001_t4_git_add_dash_a_blocks_on_develop() {
    // BC-4.16.001 T-4: branch=develop, payload=git add -A → BLOCKED (conservative PC1)
    let result = run_hook_with_branch("git add -A", "develop");
    assert!(
        result.is_ok(),
        "BC-4.16.001 T-4: hook_logic panicked. Production unimplemented."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "BC-4.16.001 T-4: 'git add -A' on develop must exit 2."
        );
    }
}

#[test]
fn test_bc4_16_001_t5_release_branch_factory_path_blocks() {
    // BC-4.16.001 T-5: branch=release/v1.0.0-rc.24, path=.factory/stories → BLOCKED PC1
    let result = run_hook_with_branch(
        "git add .factory/stories/S-21.01.md",
        "release/v1.0.0-rc.24",
    );
    assert!(
        result.is_ok(),
        "BC-4.16.001 T-5: hook_logic panicked. Production unimplemented."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "BC-4.16.001 T-5: .factory/ path on release branch must exit 2."
        );
    }
}

// ---------------------------------------------------------------------------
// AC-007 (BC-5.43.001 PC2, Invariant 1):
// per-story-delivery.md §Main-Checkout Sync Protocol section — content checks
//
// These tests verify the S-21.01 Layer-2 deliverable (ADR-031 §Decision 2):
// per-story-delivery.md must contain the mandatory pre-check constraint.
// They fail until the implementer adds the §Main-Checkout Sync Protocol section.
// ---------------------------------------------------------------------------

#[test]
fn test_ac007_bc5_43_001_per_story_delivery_md_has_main_checkout_sync_protocol_section() {
    // AC-007 gate: section must be present
    let path = per_story_delivery_md_path();
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", path.display(), e));
    assert!(
        content.contains("Main-Checkout Sync Protocol"),
        "AC-007 / BC-5.43.001 / ADR-031 §Decision 2: \
         plugins/vsdd-factory/agents/orchestrator/per-story-delivery.md must contain a \
         '§Main-Checkout Sync Protocol' section as the S-21.01 Layer-2 deliverable. \
         The section is absent — story S-21.01 implementer must add it."
    );
}

#[test]
fn test_ac007_bc5_43_001_per_story_delivery_md_mandates_git_diff_name_only_gate() {
    // AC-007 gate (a): section must run git diff --name-only HEAD..<target-ref>
    let path = per_story_delivery_md_path();
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", path.display(), e));
    assert!(
        content.contains("git diff --name-only"),
        "AC-007 / BC-5.43.001 PC2: §Main-Checkout Sync Protocol must mandate \
         'git diff --name-only HEAD..<target-ref>' as the required pre-check command \
         (BC-5.43.001 §Description). The command is absent from per-story-delivery.md."
    );
}

#[test]
fn test_ac007_bc5_43_001_per_story_delivery_md_halts_with_factory_path_deletion_error() {
    // AC-007 gate (b): must halt with FactoryPathDeletionInMergeDiff when .factory/ path detected
    let path = per_story_delivery_md_path();
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", path.display(), e));
    assert!(
        content.contains("FactoryPathDeletionInMergeDiff"),
        "AC-007 / BC-5.43.001 PC2: §Main-Checkout Sync Protocol must specify halt with \
         'FactoryPathDeletionInMergeDiff' error variant when .factory/ path is detected \
         in the merge diff. The error variant is absent from per-story-delivery.md."
    );
}

#[test]
fn test_ac007_bc5_43_001_per_story_delivery_md_covers_git_pull_or_merge() {
    // AC-007 gate (d): covers both documented steps and ad-hoc operator Bash
    let path = per_story_delivery_md_path();
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", path.display(), e));
    let covers_pull_or_merge = content.contains("git pull") || content.contains("git merge");
    assert!(
        covers_pull_or_merge,
        "AC-007 / BC-5.43.001: §Main-Checkout Sync Protocol must cover 'git pull' and/or \
         'git merge' on the main product checkout. Neither appears in per-story-delivery.md \
         in context of the pre-check protocol."
    );
}

// ---------------------------------------------------------------------------
// Section-extraction helpers (F-P1-005: scope assertions to section body)
// ---------------------------------------------------------------------------

/// Extract the §Main-Checkout Sync Protocol section body from per-story-delivery.md.
/// Returns the text from "## Main-Checkout Sync Protocol" up to (but not including)
/// the next "## " top-level heading. Returns an empty string if the section is absent.
fn extract_main_checkout_sync_protocol_section(content: &str) -> String {
    let section_marker = "## Main-Checkout Sync Protocol";
    let start = match content.find(section_marker) {
        Some(i) => i,
        None => return String::new(),
    };
    let rest = &content[start..];
    // Find the next top-level section heading (## followed by space, not a subsection ###)
    // The section_marker itself starts with ##, so skip past it before searching.
    let after_heading = section_marker.len();
    let end = rest[after_heading..]
        .find("\n## ")
        .map(|i| after_heading + i)
        .unwrap_or(rest.len());
    rest[..end].to_string()
}

/// Extract the "### Fail-Open When git diff Fails" subsection body from
/// the §Main-Checkout Sync Protocol section. Returns empty string if absent.
fn extract_fail_open_subsection(section: &str) -> String {
    let subsection_marker = "### Fail-Open When git diff Fails";
    let start = match section.find(subsection_marker) {
        Some(i) => i,
        None => return String::new(),
    };
    let rest = &section[start..];
    let after_heading = subsection_marker.len();
    // Find the next subsection heading (### ) or end of section
    let end = rest[after_heading..]
        .find("\n### ")
        .map(|i| after_heading + i)
        .unwrap_or(rest.len());
    rest[..end].to_string()
}

// ---------------------------------------------------------------------------
// AC-008 (BC-5.43.001 PC1):
// Merge proceeds when git diff --name-only returns no .factory/ paths
// (F-P1-005: scoped to §Main-Checkout Sync Protocol section body)
// ---------------------------------------------------------------------------

#[test]
fn test_ac008_bc5_43_001_per_story_delivery_md_documents_pass_on_clean_diff() {
    // F-P1-005 repair: scope assertion to §Main-Checkout Sync Protocol section body only.
    // Whole-file word greps are tautological — "proceed" appears in unrelated sections.
    let path = per_story_delivery_md_path();
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", path.display(), e));
    let section = extract_main_checkout_sync_protocol_section(&content);
    assert!(
        !section.is_empty(),
        "AC-008 prerequisite / BC-5.43.001: §Main-Checkout Sync Protocol section must exist \
         before AC-008 can pass. Fix AC-007 tests first."
    );
    // AC-008 behavioral gate: within the section, the pass-through case must use the canonical
    // phrase "MUST proceed normally" (BC-5.43.001 PC1) — not generic prose that coincidentally
    // contains "proceed" elsewhere in the file.
    assert!(
        section.contains("MUST proceed normally"),
        "AC-008 / BC-5.43.001 PC1 (F-P1-005 section-scoped): §Main-Checkout Sync Protocol \
         must contain 'MUST proceed normally' WITHIN the section body to document that the \
         merge/pull proceeds when git diff --name-only returns no .factory/ paths. \
         The canonical phrase is absent from the section body (present in different section \
         would not satisfy this gate — whole-file grep is tautological per F-P1-005)."
    );
}

// ---------------------------------------------------------------------------
// AC-009 (BC-5.43.001 Invariant 4):
// Fail-open (proceed with warning) when git diff --name-only itself fails
// (F-P1-005: scoped to §Fail-Open subsection; checks for non-zero exit documentation)
// ---------------------------------------------------------------------------

#[test]
fn test_ac009_bc5_43_001_per_story_delivery_md_documents_fail_open_on_diff_failure() {
    // F-P1-005 repair: scope assertion to §Main-Checkout Sync Protocol section body and
    // specifically the §Fail-Open When git diff Fails subsection. Whole-file word greps
    // (e.g., "proceed" on line 62) are tautological — they pass even with wrong section content.
    //
    // AC-009 behavioral gate: the fail-open subsection MUST document:
    //   1. The non-zero exit / failure scenario (checked via "non-zero" keyword)
    //   2. That a warning is logged
    //   3. That the merge/pull proceeds (fail-open)
    // This test is RED until the §Fail-Open subsection explicitly cites "non-zero" as the
    // failure trigger (BC-5.43.001 Invariant 4 — mock git diff returning non-zero exit).
    let path = per_story_delivery_md_path();
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", path.display(), e));
    let section = extract_main_checkout_sync_protocol_section(&content);
    assert!(
        !section.is_empty(),
        "AC-009 prerequisite / BC-5.43.001: §Main-Checkout Sync Protocol section must exist \
         before AC-009 can pass."
    );
    let fail_open = extract_fail_open_subsection(&section);
    assert!(
        !fail_open.is_empty(),
        "AC-009 / BC-5.43.001 Invariant 4: §Fail-Open When git diff Fails subsection must \
         exist within §Main-Checkout Sync Protocol. The subsection heading is absent."
    );
    // Gate (a): the subsection must explicitly call out the non-zero exit trigger.
    // AC-009 scenario: mock git diff returning non-zero exit → fail-open behavior.
    // The section currently says "fails (network error, unresolvable ref)" but must also
    // explicitly document the non-zero exit code trigger per AC-009 Test Plan T-009.
    assert!(
        fail_open.contains("non-zero"),
        "AC-009 / BC-5.43.001 Invariant 4 (F-P1-005 RED gate): §Fail-Open When git diff Fails \
         subsection must explicitly document the 'non-zero' exit trigger — the AC-009 \
         test scenario is 'mock git diff returning non-zero exit'. The phrase 'non-zero' is \
         absent from the subsection. Implementer must add explicit non-zero exit language to \
         the §Fail-Open subsection per AC-009 Test Plan T-009."
    );
    // Gate (b): warning must be logged
    assert!(
        fail_open.contains("warning"),
        "AC-009 / BC-5.43.001 Invariant 4: §Fail-Open subsection must document that a \
         warning is logged when git diff fails."
    );
    // Gate (c): merge must still proceed (fail-open)
    assert!(
        fail_open.contains("Proceed") || fail_open.contains("proceed"),
        "AC-009 / BC-5.43.001 Invariant 4: §Fail-Open subsection must document that the \
         merge/pull proceeds (fail-open) when git diff fails."
    );
}

// ---------------------------------------------------------------------------
// F-P1-001: BC-4.16.001 Invariant 4 v1.3 — new conservative forms:
// bare .factory (no slash), ./ (CWD-relative), :/ (pathspec-magic), ':/.factory'
// All MUST be RED against current implementation (misses these v1.3 additions).
// ---------------------------------------------------------------------------

#[test]
fn test_contains_factory_path_arg_detects_bare_factory_no_slash() {
    // BC-4.16.001 Invariant 4 v1.3: bare `.factory` without trailing slash must block.
    // git treats `.factory` as `.factory/**` for staging — identical scope to `.factory/`.
    assert!(
        contains_factory_path_arg("git add .factory"),
        "F-P1-001 / BC-4.16.001 Invariant 4 v1.3: bare '.factory' (no trailing slash) MUST \
         be conservatively blocked. git expands '.factory' to '.factory/**' for staging \
         purposes — identical dual-tracking vector as '.factory/'."
    );
}

#[test]
fn test_contains_factory_path_arg_detects_dot_slash() {
    // BC-4.16.001 Invariant 4 v1.3: './' is CWD-relative with explicit slash — semantically
    // identical to '.' for staging. Must be conservatively blocked.
    assert!(
        contains_factory_path_arg("git add ./"),
        "F-P1-001 / BC-4.16.001 Invariant 4 v1.3: './' MUST be conservatively blocked — \
         CWD-relative with explicit slash; semantically identical to '.' for staging; \
         may stage .factory/** when CWD is the project root or .factory/."
    );
}

#[test]
fn test_contains_factory_path_arg_detects_pathspec_magic_root() {
    // BC-4.16.001 Invariant 4 v1.3: ':/' pathspec-magic anchors from repo root —
    // can include .factory/ paths regardless of CWD. Conservatively blocked.
    assert!(
        contains_factory_path_arg("git add :/"),
        "F-P1-001 / BC-4.16.001 Invariant 4 v1.3: ':/' pathspec-magic MUST be conservatively \
         blocked — anchors from repository root; can include .factory/ paths regardless of CWD."
    );
}

#[test]
fn test_contains_factory_path_arg_detects_pathspec_magic_factory_path() {
    // BC-4.16.001 Invariant 4 v1.3: ':/.factory'-family forms (quoted and unquoted).
    assert!(
        contains_factory_path_arg("git add ':/.factory'"),
        "F-P1-001 / BC-4.16.001 Invariant 4 v1.3: quoted ':/.factory' MUST be blocked."
    );
    assert!(
        contains_factory_path_arg("git add :/.factory"),
        "F-P1-001 / BC-4.16.001 Invariant 4 v1.3: unquoted ':/.factory' MUST be blocked."
    );
    assert!(
        contains_factory_path_arg("git add :/path"),
        "F-P1-001 / BC-4.16.001 Invariant 4 v1.3: ':/path' pathspec-magic anchors from \
         repo root — conservatively blocked (may reach .factory/ paths)."
    );
}

// hook_logic integration tests: bare .factory on each product branch class

#[test]
fn test_fp1_001_bc4_16_001_blocks_bare_factory_on_develop() {
    let result = run_hook_with_branch("git add .factory", "develop");
    assert!(
        result.is_ok(),
        "F-P1-001 / BC-4.16.001 Invariant 4 v1.3: hook_logic panicked for \
         'git add .factory' on develop. Must return HookResult::Block, not panic."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "F-P1-001 / BC-4.16.001 Invariant 4 v1.3: 'git add .factory' (bare, no slash) \
             on develop MUST exit 2 (block_intent=true). Current impl only checks '.factory/' \
             with trailing slash — misses the bare form added in v1.3."
        );
        match &hook_result {
            HookResult::Block { reason } => {
                assert!(
                    reason.contains("FactoryPathOnProductBranch"),
                    "F-P1-001 / BC-4.16.001 PC1: block reason must contain \
                     'FactoryPathOnProductBranch'. Got: '{}'",
                    reason
                );
            }
            other => panic!(
                "F-P1-001: expected HookResult::Block for 'git add .factory' on develop, \
                 got {:?}",
                other
            ),
        }
    }
}

#[test]
fn test_fp1_001_bc4_16_001_blocks_bare_factory_on_main() {
    let result = run_hook_with_branch("git add .factory", "main");
    assert!(
        result.is_ok(),
        "F-P1-001: hook_logic panicked for 'git add .factory' on main."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "F-P1-001 / BC-4.16.001 Invariant 4 v1.3: 'git add .factory' on main MUST \
             exit 2 (main is a product branch)."
        );
    }
}

#[test]
fn test_fp1_001_bc4_16_001_blocks_bare_factory_on_feature_branch() {
    let result = run_hook_with_branch("git add .factory", "feature/S-21.01");
    assert!(
        result.is_ok(),
        "F-P1-001: hook_logic panicked for 'git add .factory' on feature/S-21.01."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "F-P1-001 / BC-4.16.001 Invariant 4 v1.3: 'git add .factory' on feature/* MUST \
             exit 2 (feature/* is a product branch)."
        );
    }
}

#[test]
fn test_fp1_001_bc4_16_001_blocks_bare_factory_on_release_branch() {
    let result = run_hook_with_branch("git add .factory", "release/v1.0.0-rc.24");
    assert!(
        result.is_ok(),
        "F-P1-001: hook_logic panicked for 'git add .factory' on release/v1.0.0-rc.24."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "F-P1-001 / BC-4.16.001 Invariant 4 v1.3: 'git add .factory' on release/* MUST \
             exit 2 (release/* is a product branch)."
        );
    }
}

#[test]
fn test_fp1_001_bc4_16_001_blocks_bare_factory_on_maintenance_branch() {
    let result = run_hook_with_branch("git add .factory", "maintenance/hotfix-001");
    assert!(
        result.is_ok(),
        "F-P1-001: hook_logic panicked for 'git add .factory' on maintenance/hotfix-001."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "F-P1-001 / BC-4.16.001 Invariant 4 v1.3: 'git add .factory' on maintenance/* \
             MUST exit 2 (maintenance/* is a product branch)."
        );
    }
}

#[test]
fn test_fp1_001_bc4_16_001_passes_bare_factory_on_factory_artifacts() {
    // factory-artifacts is NOT a product branch — all staging is legitimate (PC3).
    // This complementary test verifies the guard does not over-block.
    let result = run_hook_with_branch("git add .factory", "factory-artifacts");
    assert!(
        result.is_ok(),
        "F-P1-001: hook_logic panicked for 'git add .factory' on factory-artifacts."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result,
            HookResult::Continue,
            "F-P1-001 / BC-4.16.001 PC3: 'git add .factory' on factory-artifacts MUST return \
             Continue (PC3 unconditional pass — factory artifact commits require staging here). \
             Got: {:?}",
            hook_result
        );
    }
}

#[test]
fn test_fp1_001_bc4_16_001_blocks_dot_slash_on_develop() {
    let result = run_hook_with_branch("git add ./", "develop");
    assert!(
        result.is_ok(),
        "F-P1-001: hook_logic panicked for 'git add ./' on develop."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "F-P1-001 / BC-4.16.001 Invariant 4 v1.3: 'git add ./' on develop MUST exit 2. \
             './' is CWD-relative with explicit slash; semantically identical to '.' for \
             staging. Current impl matches '.' exactly but not './'."
        );
    }
}

#[test]
fn test_fp1_001_bc4_16_001_blocks_pathspec_root_on_develop() {
    let result = run_hook_with_branch("git add :/", "develop");
    assert!(
        result.is_ok(),
        "F-P1-001: hook_logic panicked for 'git add :/' on develop."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "F-P1-001 / BC-4.16.001 Invariant 4 v1.3: 'git add :/' on develop MUST exit 2. \
             ':/' pathspec-magic anchors from repo root — can include .factory/ paths."
        );
    }
}

#[test]
fn test_fp1_001_bc4_16_001_blocks_pathspec_factory_quoted_on_develop() {
    // ':/.factory' — pathspec-magic with explicit .factory path
    let result = run_hook_with_branch("git add ':/.factory'", "develop");
    assert!(
        result.is_ok(),
        "F-P1-001: hook_logic panicked for \"git add ':/.factory'\" on develop."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "F-P1-001 / BC-4.16.001 Invariant 4 v1.3: \"git add ':/.factory'\" on develop \
             MUST exit 2. ':/.factory' pathspec-magic names a .factory path directly."
        );
    }
}

// ---------------------------------------------------------------------------
// F-P1-002: BC-4.16.001 Precondition 2 v1.3 — git stage synonym + whitespace variants.
// All MUST be RED against current implementation.
// ---------------------------------------------------------------------------

#[test]
fn test_is_git_add_command_detects_git_stage() {
    // BC-4.16.001 Precondition 2 v1.3: `git stage` is a true git synonym for `git add`.
    // Pattern changed from `git\s+add` to `git\s+(add|stage)`. Current impl uses
    // `contains("git add")` which misses `git stage`.
    assert!(
        is_git_add_command("git stage .factory/STATE.md"),
        "F-P1-002 / BC-4.16.001 Precondition 2 v1.3: 'git stage' is a true git synonym \
         for 'git add' (ADR-031 §Decision 2 Layer-1). is_git_add_command MUST return true \
         for 'git stage' payloads. Current impl only checks for 'git add' literal substring."
    );
}

#[test]
fn test_is_git_add_command_detects_double_space() {
    // BC-4.16.001 Precondition 2: `git\s+(add|stage)` regex allows any whitespace.
    // Double space `git  add` should match. Current impl uses `contains("git add")` with
    // single space — fails to match double-space form.
    assert!(
        is_git_add_command("git  add .factory/STATE.md"),
        "F-P1-002 / BC-4.16.001 Precondition 2 v1.3: 'git  add' (double space) MUST be \
         detected as a git add command. Pattern is `git\\s+(add|stage)` (regex whitespace), \
         not literal 'git add' (single space)."
    );
}

#[test]
fn test_is_git_add_command_detects_tab_separated() {
    // BC-4.16.001 Precondition 2: tab-separated `git\tadd` must match `git\s+(add|stage)`.
    assert!(
        is_git_add_command("git\tadd .factory/STATE.md"),
        "F-P1-002 / BC-4.16.001 Precondition 2 v1.3: tab-separated 'git\\tadd' MUST be \
         detected as a git add command. Pattern is `git\\s+(add|stage)` (any whitespace)."
    );
}

#[test]
fn test_fp1_002_bc4_16_001_blocks_git_stage_factory_path_on_develop() {
    // BC-4.16.001 Precondition 2 v1.3: git stage .factory/ path on product branch → Block.
    let result = run_hook_with_branch("git stage .factory/STATE.md", "develop");
    assert!(
        result.is_ok(),
        "F-P1-002: hook_logic panicked for 'git stage .factory/STATE.md' on develop."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "F-P1-002 / BC-4.16.001 PC1 + Precondition 2 v1.3: 'git stage .factory/STATE.md' \
             on develop MUST exit 2. 'git stage' is a true git synonym — same dual-tracking \
             vector as 'git add'. Current impl passes 'git stage' via PC4 (non-git-add branch)."
        );
        match &hook_result {
            HookResult::Block { reason } => {
                assert!(
                    reason.contains("FactoryPathOnProductBranch"),
                    "F-P1-002: block reason must contain 'FactoryPathOnProductBranch'. \
                     Got: '{}'",
                    reason
                );
            }
            other => panic!(
                "F-P1-002: expected HookResult::Block for 'git stage .factory/STATE.md' \
                 on develop, got {:?}",
                other
            ),
        }
    }
}

#[test]
fn test_fp1_002_bc4_16_001_blocks_git_stage_factory_path_on_main() {
    let result = run_hook_with_branch("git stage .factory/STATE.md", "main");
    assert!(result.is_ok(), "F-P1-002: hook_logic panicked on main.");
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "F-P1-002 / BC-4.16.001 Precondition 2 v1.3: 'git stage' on main MUST exit 2."
        );
    }
}

#[test]
fn test_fp1_002_bc4_16_001_blocks_git_stage_factory_path_on_feature() {
    let result = run_hook_with_branch("git stage .factory/STATE.md", "feature/S-21.01");
    assert!(result.is_ok(), "F-P1-002: hook_logic panicked on feature/.");
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "F-P1-002 / BC-4.16.001 Precondition 2 v1.3: 'git stage' on feature/* MUST exit 2."
        );
    }
}

#[test]
fn test_fp1_002_bc4_16_001_blocks_git_stage_factory_path_on_release() {
    let result = run_hook_with_branch("git stage .factory/STATE.md", "release/v1.0.0-rc.24");
    assert!(result.is_ok(), "F-P1-002: hook_logic panicked on release/.");
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "F-P1-002 / BC-4.16.001 Precondition 2 v1.3: 'git stage' on release/* MUST exit 2."
        );
    }
}

#[test]
fn test_fp1_002_bc4_16_001_blocks_git_stage_factory_path_on_maintenance() {
    let result = run_hook_with_branch("git stage .factory/STATE.md", "maintenance/hotfix-001");
    assert!(
        result.is_ok(),
        "F-P1-002: hook_logic panicked on maintenance/."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "F-P1-002 / BC-4.16.001 Precondition 2 v1.3: 'git stage' on maintenance/* MUST \
             exit 2."
        );
    }
}

#[test]
fn test_fp1_002_bc4_16_001_blocks_double_space_git_add_factory_on_develop() {
    // BC-4.16.001 Precondition 2 v1.3: `git  add` (double space) must be detected.
    let result = run_hook_with_branch("git  add .factory/STATE.md", "develop");
    assert!(
        result.is_ok(),
        "F-P1-002: hook_logic panicked for 'git  add' (double space)."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "F-P1-002 / BC-4.16.001 Precondition 2 v1.3: 'git  add .factory/STATE.md' \
             (double space) on develop MUST exit 2. Pattern is `git\\s+(add|stage)` — \
             any whitespace between git and add."
        );
    }
}

#[test]
fn test_fp1_002_bc4_16_001_blocks_tab_git_add_factory_on_develop() {
    // BC-4.16.001 Precondition 2 v1.3: `git\tadd` (tab-separated) must be detected.
    let result = run_hook_with_branch("git\tadd .factory/STATE.md", "develop");
    assert!(
        result.is_ok(),
        "F-P1-002: hook_logic panicked for 'git\\tadd' (tab)."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "F-P1-002 / BC-4.16.001 Precondition 2 v1.3: 'git\\tadd .factory/STATE.md' \
             (tab separator) on develop MUST exit 2."
        );
    }
}

#[test]
fn test_fp1_002_bc4_16_001_blocks_double_space_git_stage_factory_on_develop() {
    // BC-4.16.001 Precondition 2 v1.3: `git  stage` (double space + git stage synonym).
    let result = run_hook_with_branch("git  stage .factory/STATE.md", "develop");
    assert!(
        result.is_ok(),
        "F-P1-002: hook_logic panicked for 'git  stage' (double space)."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "F-P1-002 / BC-4.16.001 Precondition 2 v1.3: 'git  stage .factory/STATE.md' \
             (double space + git stage) on develop MUST exit 2."
        );
    }
}

// ---------------------------------------------------------------------------
// F-P2-001 [BLOCKER]: BC-4.16.001 v1.4 — chained/sequential command forms.
//
// BC-4.16.001 v1.4 Precondition 2 explicitly requires the git add/stage
// matcher to scan anywhere in the payload, including &&, ; and | chained
// forms. The current is_git_add_command implementation exits early at the
// first "git" token and therefore misses git add commands that appear after
// a different git command (e.g. `git status && git add`).
//
// Unit tests on is_git_add_command: assert true for chained forms (RED).
// Integration tests via hook_logic: assert Block on product branches (RED).
// Regression-pin tests: assert already-correct forms remain correct (GREEN).
// ---------------------------------------------------------------------------

// -- is_git_add_command unit tests for chained forms (RED) --

#[test]
fn test_fp2_001_is_git_add_command_detects_chained_and_git_add_after_status() {
    // BC-4.16.001 v1.4 Precondition 2: git add appearing after && must be detected.
    // Current impl: "git" → next "status" → returns false immediately (early exit).
    assert!(
        is_git_add_command("git status && git add .factory/STATE.md"),
        "F-P2-001 / BC-4.16.001 v1.4 Precondition 2: \
         'git status && git add .factory/STATE.md' MUST be detected as containing a git \
         add command. The '&&' chained form places git add after a different git command — \
         current impl exits early at the first 'git' token (matched to 'status')."
    );
}

#[test]
fn test_fp2_001_is_git_add_command_detects_chained_and_git_stage_after_pull() {
    // BC-4.16.001 v1.4 Precondition 2: git stage appearing after && must be detected.
    assert!(
        is_git_add_command("git pull && git stage .factory/x"),
        "F-P2-001 / BC-4.16.001 v1.4 Precondition 2: \
         'git pull && git stage .factory/x' MUST detect git stage in the chained form. \
         Current impl exits early at the first 'git' token (matched to 'pull')."
    );
}

#[test]
fn test_fp2_001_is_git_add_command_detects_semicolon_chained_git_add() {
    // BC-4.16.001 v1.4 Precondition 2: ';' sequential form must be detected.
    // 'diff;' is a single token (no space before ';') — current impl sees 'diff;'
    // as the subcommand token, which is neither 'add' nor 'stage', and returns false.
    assert!(
        is_git_add_command("git diff; git add .factory/STATE.md"),
        "F-P2-001 / BC-4.16.001 v1.4 Precondition 2: \
         'git diff; git add .factory/STATE.md' MUST detect git add after the ';' separator. \
         'diff;' is a single whitespace-separated token — current impl treats it as the \
         subcommand and exits before scanning the second 'git add'."
    );
}

// -- F-P2-001 integration tests via hook_logic (RED) --

#[test]
fn test_fp2_001_bc4_16_001_blocks_chained_and_git_add_factory_on_develop() {
    // F-P2-001 [BLOCKER] / BC-4.16.001 v1.4 Precondition 2:
    // Chained form `git status && git add .factory/STATE.md` on develop MUST block.
    // Currently BYPASSES: is_git_add_command returns false (first 'git' → 'status').
    let result = run_hook_with_branch("git status && git add .factory/STATE.md", "develop");
    assert!(
        result.is_ok(),
        "F-P2-001: hook_logic panicked for \
         'git status && git add .factory/STATE.md' on develop."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "F-P2-001 [BLOCKER] / BC-4.16.001 v1.4 Precondition 2: \
             'git status && git add .factory/STATE.md' on develop MUST exit 2. The guard \
             must scan all tokens in the payload for git add/stage, not stop at the first \
             git command. Currently BYPASSES (first 'git status' fools the scanner)."
        );
        match &hook_result {
            HookResult::Block { reason } => {
                assert!(
                    reason.contains("FactoryPathOnProductBranch"),
                    "F-P2-001: block reason must contain 'FactoryPathOnProductBranch'. \
                     Got: '{}'",
                    reason
                );
            }
            other => panic!(
                "F-P2-001: expected HookResult::Block for chained && form on develop, \
                 got {:?}",
                other
            ),
        }
    }
}

#[test]
fn test_fp2_001_bc4_16_001_blocks_chained_and_git_stage_factory_after_pull() {
    // F-P2-001 [BLOCKER]: `git pull && git stage .factory/x` on develop MUST block.
    let result = run_hook_with_branch("git pull && git stage .factory/x", "develop");
    assert!(
        result.is_ok(),
        "F-P2-001: hook_logic panicked for 'git pull && git stage .factory/x'."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "F-P2-001 [BLOCKER] / BC-4.16.001 v1.4 Precondition 2: \
             'git pull && git stage .factory/x' on develop MUST exit 2. \
             'git stage' after '&&' must be detected (same dual-tracking vector). \
             Currently BYPASSES (first 'git pull' fools the scanner)."
        );
    }
}

#[test]
fn test_fp2_001_bc4_16_001_blocks_semicolon_chained_git_add_factory_on_develop() {
    // F-P2-001 [BLOCKER]: `git diff; git add .factory/STATE.md` on develop MUST block.
    let result = run_hook_with_branch("git diff; git add .factory/STATE.md", "develop");
    assert!(
        result.is_ok(),
        "F-P2-001: hook_logic panicked for 'git diff; git add .factory/STATE.md'."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "F-P2-001 [BLOCKER] / BC-4.16.001 v1.4 Precondition 2: \
             'git diff; git add .factory/STATE.md' on develop MUST exit 2. \
             The ';' sequential form with the 'diff;' token (no space before ';') \
             must not prevent detection of the subsequent 'git add'. \
             Currently BYPASSES ('diff;' token treated as non-matching subcommand)."
        );
    }
}

// -- F-P2-001 regression-pin tests (GREEN — already correct) --

#[test]
fn test_fp2_001_regression_pipe_git_add_factory_already_blocks_on_develop() {
    // Regression-pin / BC-4.16.001 v1.4 Precondition 2: pipe form `echo hi | git add`.
    // is_git_add_command iterates past 'echo', 'hi', '|' (not 'git') until it finds
    // 'git' followed by 'add' — the while loop does not exit early here because 'echo'
    // is not 'git'. This form already blocks correctly; pin ensures it stays that way.
    let result = run_hook_with_branch("echo hi | git add .factory/f", "develop");
    assert!(
        result.is_ok(),
        "F-P2-001 regression-pin: hook_logic panicked for pipe form \
         'echo hi | git add .factory/f'."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "F-P2-001 regression-pin / BC-4.16.001 v1.4 Precondition 2: \
             'echo hi | git add .factory/f' on develop MUST exit 2. \
             The while loop finds 'git add' past the pipe separator — already correct."
        );
    }
}

#[test]
fn test_fp2_001_bc4_16_001_negative_chained_non_factory_add_continues() {
    // BC-4.16.001 PC2: chained form with non-.factory/ path MUST pass.
    // Before fix: is_git_add_command false (PC4) → Continue.
    // After fix: is_git_add_command true, contains_factory_path_arg false (PC2) → Continue.
    // Either way Continue — regression guard against over-blocking.
    let result = run_hook_with_branch("git status && git add src/main.rs", "develop");
    assert!(
        result.is_ok(),
        "F-P2-001 negative: hook_logic panicked for \
         'git status && git add src/main.rs' on develop."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result,
            HookResult::Continue,
            "F-P2-001 negative / BC-4.16.001 PC2: \
             'git status && git add src/main.rs' on develop MUST return Continue \
             (no .factory/ path in the git add argument)."
        );
    }
}

#[test]
fn test_fp2_001_regression_first_cmd_git_add_factory_already_blocks() {
    // Regression-pin: first command in chain is git add — already detected correctly.
    // 'git add .factory/x && git status': first 'git' → 'add' → true; .factory/ found.
    let result = run_hook_with_branch("git add .factory/x && git status", "develop");
    assert!(
        result.is_ok(),
        "F-P2-001 regression-pin: hook_logic panicked for \
         'git add .factory/x && git status'."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "F-P2-001 regression-pin / BC-4.16.001 PC1: \
             'git add .factory/x && git status' on develop MUST exit 2. \
             First command is 'git add' — already detected by current impl."
        );
    }
}

// ---------------------------------------------------------------------------
// F-P2-002 [MEDIUM]: BC-4.16.001 v1.4 — global-option forms.
//
// BC-4.16.001 v1.4 Precondition 2 detection contract: "any token sequence
// beginning git whose first non-option subcommand token is add or stage,
// tolerating any number of intervening global options or flags". Forms like
// `git -C <path> add`, `git --no-pager add`, `git -c key=val add` are all
// in scope. Current is_git_add_command checks only the token immediately
// after 'git', so global options cause it to return false (BYPASS).
// ---------------------------------------------------------------------------

// -- is_git_add_command unit tests for global-option forms (RED) --

#[test]
fn test_fp2_002_is_git_add_command_detects_global_option_dash_c_path() {
    // BC-4.16.001 v1.4 Precondition 2: 'git -C <path> add' must be detected.
    // Current impl: "git" → next "-C" → neither "add" nor "stage" → returns false.
    assert!(
        is_git_add_command("git -C . add .factory/STATE.md"),
        "F-P2-002 / BC-4.16.001 v1.4 Precondition 2: \
         'git -C . add' MUST be detected as a git add command. The '-C <path>' global \
         option precedes the 'add' subcommand — current impl checks the token immediately \
         after 'git' ('-C'), not the first non-option subcommand token ('add')."
    );
}

#[test]
fn test_fp2_002_is_git_add_command_detects_global_option_no_pager() {
    // BC-4.16.001 v1.4 Precondition 2: 'git --no-pager add' must be detected.
    assert!(
        is_git_add_command("git --no-pager add .factory/x"),
        "F-P2-002 / BC-4.16.001 v1.4 Precondition 2: \
         'git --no-pager add' MUST be detected as a git add command. '--no-pager' is a \
         global option before the subcommand — current impl treats it as the subcommand."
    );
}

#[test]
fn test_fp2_002_is_git_add_command_detects_global_option_dash_c_kv_stage() {
    // BC-4.16.001 v1.4 Precondition 2: 'git -c key=val stage' must be detected.
    assert!(
        is_git_add_command("git -c user.name=x stage .factory/y"),
        "F-P2-002 / BC-4.16.001 v1.4 Precondition 2: \
         'git -c user.name=x stage' MUST be detected as a git stage command. \
         '-c key=val' is a global config option before the subcommand."
    );
}

// -- F-P2-002 integration tests via hook_logic (RED) --

#[test]
fn test_fp2_002_bc4_16_001_blocks_global_dash_c_add_factory_on_develop() {
    // F-P2-002 [MEDIUM]: `git -C . add .factory/STATE.md` on develop MUST block.
    let result = run_hook_with_branch("git -C . add .factory/STATE.md", "develop");
    assert!(
        result.is_ok(),
        "F-P2-002: hook_logic panicked for 'git -C . add .factory/STATE.md'."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "F-P2-002 [MEDIUM] / BC-4.16.001 v1.4 Precondition 2: \
             'git -C . add .factory/STATE.md' on develop MUST exit 2. '-C <path>' is a \
             global option before the 'add' subcommand — current impl checks the token \
             immediately after 'git' ('-C'), bypassing the guard."
        );
    }
}

#[test]
fn test_fp2_002_bc4_16_001_blocks_global_no_pager_add_factory_on_develop() {
    // F-P2-002 [MEDIUM]: `git --no-pager add .factory/x` on develop MUST block.
    let result = run_hook_with_branch("git --no-pager add .factory/x", "develop");
    assert!(
        result.is_ok(),
        "F-P2-002: hook_logic panicked for 'git --no-pager add .factory/x'."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "F-P2-002 [MEDIUM] / BC-4.16.001 v1.4 Precondition 2: \
             'git --no-pager add .factory/x' on develop MUST exit 2. \
             '--no-pager' is a global option — current impl bypasses guard."
        );
    }
}

#[test]
fn test_fp2_002_bc4_16_001_blocks_global_dash_c_kv_stage_factory_on_develop() {
    // F-P2-002 [MEDIUM]: `git -c user.name=x stage .factory/y` on develop MUST block.
    let result = run_hook_with_branch("git -c user.name=x stage .factory/y", "develop");
    assert!(
        result.is_ok(),
        "F-P2-002: hook_logic panicked for 'git -c user.name=x stage .factory/y'."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "F-P2-002 [MEDIUM] / BC-4.16.001 v1.4 Precondition 2: \
             'git -c user.name=x stage .factory/y' on develop MUST exit 2. \
             '-c key=val' is a global config option — current impl bypasses guard."
        );
    }
}

// -- F-P2-002 negative regression-pin (GREEN) --

#[test]
fn test_fp2_002_bc4_16_001_negative_global_dash_c_add_non_factory_continues() {
    // BC-4.16.001 PC2: global-option form with non-.factory/ path MUST pass.
    // Before fix: is_git_add_command false (PC4 bypass) → Continue.
    // After fix: is_git_add_command true, contains_factory_path_arg false (PC2) → Continue.
    // Either way Continue — regression guard against over-blocking global-option forms.
    let result = run_hook_with_branch("git -C . add src/lib.rs", "develop");
    assert!(
        result.is_ok(),
        "F-P2-002 negative: hook_logic panicked for 'git -C . add src/lib.rs'."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result,
            HookResult::Continue,
            "F-P2-002 negative / BC-4.16.001 PC2: \
             'git -C . add src/lib.rs' on develop MUST return Continue \
             (non-.factory/ path; global-option form must not over-block)."
        );
    }
}

// ---------------------------------------------------------------------------
// F-P2-003 [LOW]: BC-4.16.001 v1.4 Invariant 4 — case-insensitive .factory/ matching.
//
// macOS HFS+ and Windows NTFS are case-folding filesystems where
// `git add .Factory/STATE.md` targets the same file as
// `git add .factory/STATE.md`. BC-4.16.001 v1.4 Invariant 4 requires
// conservative case-insensitive blocking. Current contains_factory_path_arg
// uses a case-sensitive `.contains(".factory/")` check — BYPASS on variants.
// ---------------------------------------------------------------------------

// -- contains_factory_path_arg unit tests for case variants (RED) --

#[test]
fn test_fp2_003_contains_factory_path_arg_detects_capitalized_factory_dir() {
    // BC-4.16.001 v1.4 Invariant 4: '.Factory/' must match (case-insensitive).
    // Current impl: `.contains(".factory/")` is case-sensitive → '.Factory/' not found.
    assert!(
        contains_factory_path_arg("git add .Factory/STATE.md"),
        "F-P2-003 / BC-4.16.001 v1.4 Invariant 4: '.Factory/STATE.md' MUST be detected \
         as a .factory/ path (case-insensitive). macOS HFS+ and Windows NTFS treat \
         '.Factory/' as the same directory as '.factory/'. Current impl uses a \
         case-sensitive '.contains(\".factory/\")' check — '.Factory/' is not found."
    );
}

#[test]
fn test_fp2_003_contains_factory_path_arg_detects_allcaps_factory_dir() {
    // BC-4.16.001 v1.4 Invariant 4: '.FACTORY/' must match (case-insensitive).
    assert!(
        contains_factory_path_arg("git add .FACTORY/x"),
        "F-P2-003 / BC-4.16.001 v1.4 Invariant 4: '.FACTORY/x' MUST be detected as a \
         .factory/ path match. All-caps variant targets same directory on case-folding \
         filesystems. Conservative blocking required."
    );
}

// -- F-P2-003 integration tests via hook_logic (RED) --

#[test]
fn test_fp2_003_bc4_16_001_blocks_capitalized_factory_path_on_develop() {
    // F-P2-003 [LOW]: `git add .Factory/STATE.md` on develop MUST block.
    // is_git_add_command detects 'git add' (plain form) → true.
    // contains_factory_path_arg: '.factory/' case-sensitive check misses '.Factory/' → false.
    // Result: Continue (BYPASS). After fix: case-insensitive check → true → Block.
    let result = run_hook_with_branch("git add .Factory/STATE.md", "develop");
    assert!(
        result.is_ok(),
        "F-P2-003: hook_logic panicked for 'git add .Factory/STATE.md' on develop."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "F-P2-003 [LOW] / BC-4.16.001 v1.4 Invariant 4: \
             'git add .Factory/STATE.md' on develop MUST exit 2. '.Factory/' targets \
             the same directory as '.factory/' on macOS HFS+ and Windows NTFS \
             (case-folding filesystems). Case-insensitive blocking required. \
             Currently BYPASSES (case-sensitive '.factory/' check misses '.Factory/')."
        );
        match &hook_result {
            HookResult::Block { reason } => {
                assert!(
                    reason.contains("FactoryPathOnProductBranch"),
                    "F-P2-003: block reason must contain 'FactoryPathOnProductBranch'. \
                     Got: '{}'",
                    reason
                );
            }
            other => panic!(
                "F-P2-003: expected HookResult::Block for '.Factory/' on develop, \
                 got {:?}",
                other
            ),
        }
    }
}

#[test]
fn test_fp2_003_bc4_16_001_blocks_allcaps_factory_path_on_develop() {
    // F-P2-003 [LOW]: `git add .FACTORY/x` on develop MUST block.
    let result = run_hook_with_branch("git add .FACTORY/x", "develop");
    assert!(
        result.is_ok(),
        "F-P2-003: hook_logic panicked for 'git add .FACTORY/x' on develop."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "F-P2-003 [LOW] / BC-4.16.001 v1.4 Invariant 4: \
             'git add .FACTORY/x' on develop MUST exit 2. All-caps '.FACTORY/' variant \
             must be conservatively blocked on case-folding filesystems. \
             Currently BYPASSES (case-sensitive check)."
        );
    }
}

// -- F-P2-003 negative regression-pin (GREEN) --

#[test]
fn test_fp2_003_bc4_16_001_negative_capitalized_factory_passes_on_factory_artifacts() {
    // BC-4.16.001 PC3: factory-artifacts branch passes unconditionally regardless of case.
    // Branch detection → 'factory-artifacts' → not product branch → Continue (PC3).
    // Before fix and after fix: same Continue result. Pin against over-blocking.
    let result = run_hook_with_branch("git add .Factory/STATE.md", "factory-artifacts");
    assert!(
        result.is_ok(),
        "F-P2-003 negative: hook_logic panicked for 'git add .Factory/STATE.md' \
         on factory-artifacts."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result,
            HookResult::Continue,
            "F-P2-003 negative / BC-4.16.001 PC3: 'git add .Factory/STATE.md' on \
             factory-artifacts MUST return Continue (PC3 unconditional pass — \
             factory-artifacts branch staging is always legitimate)."
        );
    }
}

// ---------------------------------------------------------------------------
// F-P3-001 [LOW]: BC-4.16.001 v1.4 Precondition 2 + Invariant 4 —
// quoted-subcommand bypass.
//
// `is_git_add_command` strips trailing `;`, `&`, `|` metacharacters from the
// subcommand token via `trim_end_matches([';', '&', '|'])` but does NOT strip
// surrounding single or double quotes. This is inconsistent with
// `is_factory_arg_token`, which calls
// `token.trim_matches(|c| c == '\'' || c == '"')` on path-arg tokens before
// any comparison.
//
// Affected bypass forms:
//   git "add" .factory/STATE.md  — `"add"` fails eq_ignore_ascii_case("add")
//   git 'stage' .factory/y       — `'stage'` fails eq_ignore_ascii_case("stage")
//   git status && git "add" .factory/f — chained + quoted: both conditions apply
//
// BC traces:
//   BC-4.16.001 v1.4 Precondition 2: detect `git (add|stage)` in any textual
//     form appearing anywhere in the payload, including quoted subcommands.
//   BC-4.16.001 Invariant 4: conservative-on-ambiguity — any form that plausibly
//     stages .factory/ content must be blocked.
// ---------------------------------------------------------------------------

// -- is_git_add_command unit tests for quoted-subcommand forms (RED) --

#[test]
fn test_fp3_001_is_git_add_command_detects_double_quoted_add() {
    // BC-4.16.001 v1.4 Precondition 2: `git "add" x` MUST be detected as a git add command.
    // Failure mode: `is_git_add_command` strips trailing `;`,`&`,`|` from the subcommand token
    // but NOT surrounding quotes. The token `"add"` (5 chars, with literal double-quote
    // characters) fails the eq_ignore_ascii_case("add") comparison — returns false (BYPASS).
    // Fix: strip surrounding single/double quotes from the subcommand token before comparison,
    // consistent with is_factory_arg_token which calls trim_matches(|c| c == '\'' || c == '"').
    assert!(
        is_git_add_command("git \"add\" x"),
        "F-P3-001 / BC-4.16.001 v1.4 Precondition 2: 'git \"add\" x' MUST be detected as \
         a git add command. is_git_add_command strips trailing ';','&','|' metacharacters \
         from the subcommand token but NOT surrounding quotes — '\"add\"' (5-char token \
         with literal double-quote characters) fails eq_ignore_ascii_case(\"add\"). \
         Fix: add trim_matches quote-stripping to the subcommand token, consistent with \
         is_factory_arg_token."
    );
}

#[test]
fn test_fp3_001_is_git_add_command_detects_single_quoted_stage() {
    // BC-4.16.001 v1.4 Precondition 2: `git 'stage' y` MUST be detected as a git stage command.
    // The single-quoted token `'stage'` (7 chars, with literal single-quote characters)
    // fails eq_ignore_ascii_case("stage") — is_git_add_command returns false (BYPASS).
    assert!(
        is_git_add_command("git 'stage' y"),
        "F-P3-001 / BC-4.16.001 v1.4 Precondition 2: \"git 'stage' y\" MUST be detected as \
         a git stage command. The single-quoted subcommand token \"'stage'\" (7-char token \
         with literal single-quote characters) fails eq_ignore_ascii_case(\"stage\") because \
         surrounding quotes are not stripped before comparison. Currently returns false \
         (BYPASS). Fix: strip surrounding quotes from the subcommand token."
    );
}

#[test]
fn test_fp3_001_is_git_add_command_detects_chained_and_double_quoted_add() {
    // BC-4.16.001 v1.4 Precondition 2: chained form with quoted subcommand MUST be detected.
    // The outer loop (F-P2-001 fix, already present) correctly advances past `git status` and
    // finds the second `git`. The inner loop then hits `"add"` (double-quoted), which fails
    // eq_ignore_ascii_case("add") because surrounding quotes are not stripped — returns false.
    // This test requires both the F-P2-001 outer-loop fix (already present in this worktree)
    // AND the F-P3-001 quote-strip fix to pass.
    assert!(
        is_git_add_command("git status && git \"add\" .factory/f"),
        "F-P3-001 / BC-4.16.001 v1.4 Precondition 2: \
         'git status && git \"add\" .factory/f' MUST be detected as containing a git add \
         command. The outer loop advances past 'git status' (F-P2-001 fix); the inner loop \
         then encounters '\"add\"' (double-quoted subcommand token) which fails \
         eq_ignore_ascii_case(\"add\") because surrounding quotes are not stripped. \
         Currently returns false (BYPASS). Fix: strip surrounding quotes from the \
         subcommand token before comparison."
    );
}

// -- Integration RED tests via hook_logic --

#[test]
fn test_fp3_001_bc4_16_001_blocks_double_quoted_add_factory_on_develop() {
    // F-P3-001 [LOW]: `git "add" .factory/STATE.md` on develop MUST block.
    // Current path: is_git_add_command returns false (quoted subcommand) → PC4 → Continue
    // (BYPASS). After fix: is_git_add_command returns true; contains_factory_path_arg fast-path
    // finds ".factory/" → true → PC1 → Block.
    let result = run_hook_with_branch("git \"add\" .factory/STATE.md", "develop");
    assert!(
        result.is_ok(),
        "F-P3-001: hook_logic panicked for 'git \"add\" .factory/STATE.md' on develop. \
         Must return HookResult, not panic."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "F-P3-001 [LOW] / BC-4.16.001 v1.4 Precondition 2 + Invariant 4: \
             'git \"add\" .factory/STATE.md' on develop MUST exit 2 (block_intent=true). \
             The double-quoted subcommand '\"add\"' bypasses is_git_add_command — the token \
             '\"add\"' fails eq_ignore_ascii_case(\"add\") because surrounding quotes are \
             not stripped before comparison. Currently BYPASSES via PC4 (Continue instead \
             of Block)."
        );
        match &hook_result {
            HookResult::Block { reason } => {
                assert!(
                    reason.contains("FactoryPathOnProductBranch"),
                    "F-P3-001: block reason must contain 'FactoryPathOnProductBranch'. \
                     Got: '{}'",
                    reason
                );
            }
            other => panic!(
                "F-P3-001: expected HookResult::Block for 'git \"add\" .factory/STATE.md' \
                 on develop, got {:?}",
                other
            ),
        }
    }
}

#[test]
fn test_fp3_001_bc4_16_001_blocks_single_quoted_stage_factory_on_main() {
    // F-P3-001 [LOW]: `git 'stage' .factory/x` on main MUST block.
    // is_git_add_command returns false for "'stage'" subcommand → PC4 → Continue (BYPASS).
    // After fix: is_git_add_command returns true; contains_factory_path_arg finds ".factory/"
    // → true → PC1 → Block.
    let result = run_hook_with_branch("git 'stage' .factory/x", "main");
    assert!(
        result.is_ok(),
        "F-P3-001: hook_logic panicked for \"git 'stage' .factory/x\" on main. \
         Must return HookResult, not panic."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result.exit_code(),
            2,
            "F-P3-001 [LOW] / BC-4.16.001 v1.4 Precondition 2 + Invariant 4: \
             \"git 'stage' .factory/x\" on main MUST exit 2 (block_intent=true). \
             The single-quoted subcommand \"'stage'\" bypasses is_git_add_command because \
             surrounding quotes are not stripped before eq_ignore_ascii_case comparison. \
             Currently BYPASSES via PC4 (Continue instead of Block)."
        );
        match &hook_result {
            HookResult::Block { reason } => {
                assert!(
                    reason.contains("FactoryPathOnProductBranch"),
                    "F-P3-001: block reason must contain 'FactoryPathOnProductBranch'. \
                     Got: '{}'",
                    reason
                );
            }
            other => panic!(
                "F-P3-001: expected HookResult::Block for \"git 'stage' .factory/x\" \
                 on main, got {:?}",
                other
            ),
        }
    }
}

// -- Negative GREEN pins --

#[test]
fn test_fp3_001_bc4_16_001_negative_double_quoted_add_non_factory_continues() {
    // BC-4.16.001 PC2: `git "add" src/lib.rs` on develop MUST return Continue.
    // Before fix: is_git_add_command false (quoted subcommand) → PC4 → Continue.
    // After fix: is_git_add_command true; contains_factory_path_arg false (src/lib.rs is not
    //   a .factory/ path) → PC2 → Continue.
    // Either way the result is Continue — regression guard against over-blocking.
    let result = run_hook_with_branch("git \"add\" src/lib.rs", "develop");
    assert!(
        result.is_ok(),
        "F-P3-001 negative: hook_logic panicked for 'git \"add\" src/lib.rs' on develop."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result,
            HookResult::Continue,
            "F-P3-001 negative / BC-4.16.001 PC2: 'git \"add\" src/lib.rs' on develop MUST \
             return Continue. A double-quoted subcommand with a non-.factory/ path argument \
             has no dual-tracking risk and MUST NOT be blocked. Got: {:?}.",
            hook_result
        );
    }
}

#[test]
fn test_fp3_001_bc4_16_001_negative_double_quoted_add_factory_on_factory_artifacts_continues() {
    // BC-4.16.001 PC3: `git "add" .factory/STATE.md` on factory-artifacts MUST return Continue.
    // Before fix: is_git_add_command false → PC4 → Continue.
    // After fix: is_git_add_command true; branch=factory-artifacts → is_product_branch false
    //   → PC3 → Continue.
    // Either way the result is Continue — pin against over-blocking the factory-artifacts branch.
    let result = run_hook_with_branch("git \"add\" .factory/STATE.md", "factory-artifacts");
    assert!(
        result.is_ok(),
        "F-P3-001 negative: hook_logic panicked for 'git \"add\" .factory/STATE.md' on \
         factory-artifacts."
    );
    if let Ok(hook_result) = result {
        assert_eq!(
            hook_result,
            HookResult::Continue,
            "F-P3-001 negative / BC-4.16.001 PC3: 'git \"add\" .factory/STATE.md' on \
             factory-artifacts MUST return Continue. PC3 is unconditional — factory artifact \
             commits legitimately stage .factory/ paths on the factory-artifacts branch. \
             Got: {:?}.",
            hook_result
        );
    }
}
