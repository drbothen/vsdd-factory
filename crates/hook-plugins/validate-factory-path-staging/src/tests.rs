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
    contains_factory_path_arg, hook_logic, is_git_add_command, is_product_branch, HookCallbacks,
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
                exec_subprocess: move |_bin, _args| {
                    Ok((0, branch_output.clone(), String::new()))
                },
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
                    Ok((128, String::new(), "fatal: not a git repository".to_string()))
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
        let candidate =
            dir.join("plugins/vsdd-factory/agents/orchestrator/per-story-delivery.md");
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
    assert!(contains_factory_path_arg("git add .factory/stories/S-21.01.md"));
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
    assert!(!contains_factory_path_arg("git add crates/hook-sdk/src/lib.rs"));
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
    let result = run_hook_with_branch(
        "git add .factory/stories/S-21.01.md",
        "feature/S-21.01",
    );
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
    let result = run_hook_with_branch(
        "git add plugins/vsdd-factory/hooks-registry.toml",
        "main",
    );
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
// AC-008 (BC-5.43.001 PC1):
// Merge proceeds when git diff --name-only returns no .factory/ paths
// ---------------------------------------------------------------------------

#[test]
fn test_ac008_bc5_43_001_per_story_delivery_md_documents_pass_on_clean_diff() {
    // AC-008: section must document that merge proceeds when diff is clean
    let path = per_story_delivery_md_path();
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", path.display(), e));
    assert!(
        content.contains("Main-Checkout Sync Protocol"),
        "AC-008 prerequisite / BC-5.43.001: §Main-Checkout Sync Protocol section must exist \
         before AC-008 can pass. Fix AC-007 tests first."
    );
    let documents_pass_case = content.contains("proceed")
        || content.contains("passes")
        || content.contains("safe to")
        || content.contains("no .factory");
    assert!(
        documents_pass_case,
        "AC-008 / BC-5.43.001 PC1: §Main-Checkout Sync Protocol must document that the \
         operation PROCEEDS (passes transparently) when git diff --name-only returns no \
         .factory/ paths. The pass-through condition is absent from per-story-delivery.md."
    );
}

// ---------------------------------------------------------------------------
// AC-009 (BC-5.43.001 Invariant 4):
// Fail-open (proceed with warning) when git diff --name-only itself fails
// ---------------------------------------------------------------------------

#[test]
fn test_ac009_bc5_43_001_per_story_delivery_md_documents_fail_open_on_diff_failure() {
    // AC-009: section must document fail-open when git diff fails
    let path = per_story_delivery_md_path();
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", path.display(), e));
    assert!(
        content.contains("Main-Checkout Sync Protocol"),
        "AC-009 prerequisite / BC-5.43.001: §Main-Checkout Sync Protocol section must exist \
         before AC-009 can pass. Fix AC-007 tests first."
    );
    // BC-5.43.001 Invariant 4: log warning AND proceed when git diff fails
    let documents_fail_open = content.contains("warning")
        || content.contains("fail-open")
        || content.contains("log")
        || content.contains("proceed");
    assert!(
        documents_fail_open,
        "AC-009 / BC-5.43.001 Invariant 4: §Main-Checkout Sync Protocol must document \
         fail-open behavior: log a warning AND proceed when git diff --name-only fails \
         (network error, unresolvable ref). The fail-open clause is absent from \
         per-story-delivery.md."
    );
}
