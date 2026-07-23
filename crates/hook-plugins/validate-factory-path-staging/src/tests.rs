// Test files use .expect()/.unwrap()/.panic!() for failure reporting.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
//! Unit tests for validate-factory-path-staging.
//!
//! These tests exercise the pure functions declared in `lib.rs` without
//! a WASM runtime. All tests are RED at stub time (lib.rs bodies are `todo!()`).
//!
//! # BC traces
//! - BC-4.16.001 PC1: block .factory/ staging on product branches
//! - BC-4.16.001 PC2: pass non-.factory/ git add on product branches
//! - BC-4.16.001 PC3: pass all commands on factory-artifacts branch
//! - BC-4.16.001 PC4: pass non-git-add commands unconditionally
//! - BC-4.16.001 Invariant 3: fail-open on branch detection failure

use crate::{contains_factory_path_arg, is_git_add_command, is_product_branch};

// ---------------------------------------------------------------------------
// is_git_add_command
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
// is_product_branch
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
// contains_factory_path_arg
// ---------------------------------------------------------------------------

#[test]
fn test_contains_factory_path_arg_detects_factory_path() {
    assert!(contains_factory_path_arg("git add .factory/STATE.md"));
    assert!(contains_factory_path_arg("git add .factory/stories/S-21.01.md"));
}

#[test]
fn test_contains_factory_path_arg_detects_conservative_flags() {
    // Conservative: -A, -u, . treated as potentially staging .factory/ content
    assert!(contains_factory_path_arg("git add -A"));
    assert!(contains_factory_path_arg("git add -u"));
}

#[test]
fn test_contains_factory_path_arg_passes_non_factory_path() {
    assert!(!contains_factory_path_arg("git add src/main.rs"));
    assert!(!contains_factory_path_arg("git add crates/hook-sdk/src/lib.rs"));
}
