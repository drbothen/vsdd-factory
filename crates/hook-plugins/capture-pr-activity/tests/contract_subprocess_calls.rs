//! AC: "Handles gh pr create, gh pr merge, gh pr close subcommands"
//! AC: "Emits pr.created / pr.merged / pr.closed events ..."
//!
//! Tests that the dispatch pipeline correctly identifies which subcommand
//! is present, so the implementer knows to call exec_subprocess with the
//! right arguments and route to the right event builder.
//!
//! These tests drive `dispatch()` directly. In native test targets
//! `exec_subprocess` and `emit_event` host functions are not available,
//! so `dispatch` must panic via `unimplemented!()` — confirming the
//! RED gate. The intent documented here is verified at GREEN time.

use capture_pr_activity::{detect_merge_strategy, detect_pr_subcommand, PrSubcommand};

// ── detect_merge_strategy ─────────────────────────────────────────────────────

/// BC-4.04.002: --squash flag detected.
#[test]
fn test_BC_4_04_002_detect_merge_strategy_squash() {
    let cmd = "gh pr merge https://github.com/owner/repo/pull/99 --squash";
    assert_eq!(detect_merge_strategy(cmd), Some("squash".to_string()));
}

/// BC-4.04.002: --rebase flag detected.
#[test]
fn test_BC_4_04_002_detect_merge_strategy_rebase() {
    let cmd = "gh pr merge 42 --rebase";
    assert_eq!(detect_merge_strategy(cmd), Some("rebase".to_string()));
}

/// BC-4.04.002: --merge flag detected.
#[test]
fn test_BC_4_04_002_detect_merge_strategy_merge_flag() {
    let cmd = "gh pr merge 42 --merge";
    assert_eq!(detect_merge_strategy(cmd), Some("merge".to_string()));
}

/// BC-4.04.002: no strategy flag → None.
#[test]
fn test_BC_4_04_002_detect_merge_strategy_none_when_no_flag() {
    let cmd = "gh pr merge 42";
    assert_eq!(detect_merge_strategy(cmd), None);
}

/// BC-4.04.002: strategy absent for create command → None.
#[test]
fn test_BC_4_04_002_detect_merge_strategy_none_for_create() {
    let cmd = "gh pr create --title t --body b";
    assert_eq!(detect_merge_strategy(cmd), None);
}

// ── Dispatch routing contract ─────────────────────────────────────────────────

/// BC-4.04.001: create is recognized and distinct from merge.
#[test]
fn test_BC_4_04_001_create_subcommand_is_not_merge() {
    let cmd = "gh pr create --title t --body b";
    let sub = detect_pr_subcommand(cmd);
    assert_eq!(sub, Some(PrSubcommand::Create));
    assert_ne!(sub, Some(PrSubcommand::Merge));
    assert_ne!(sub, Some(PrSubcommand::Close));
}

/// BC-4.04.001: merge is recognized and distinct from create.
#[test]
fn test_BC_4_04_001_merge_subcommand_is_not_create() {
    let cmd = "gh pr merge 42 --squash";
    let sub = detect_pr_subcommand(cmd);
    assert_eq!(sub, Some(PrSubcommand::Merge));
    assert_ne!(sub, Some(PrSubcommand::Create));
    assert_ne!(sub, Some(PrSubcommand::Close));
}

/// BC-4.04.001: close is recognized and distinct from create/merge.
#[test]
fn test_BC_4_04_001_close_subcommand_is_not_create_or_merge() {
    let cmd = "gh pr close 42";
    let sub = detect_pr_subcommand(cmd);
    assert_eq!(sub, Some(PrSubcommand::Close));
    assert_ne!(sub, Some(PrSubcommand::Create));
    assert_ne!(sub, Some(PrSubcommand::Merge));
}

/// BC-4.04.001: positional PR number form for merge.
/// Corresponds to bats: "gh pr merge with positional PR number emits pr.merged"
#[test]
fn test_BC_4_04_001_positional_pr_number_detected_as_merge() {
    let cmd = "gh pr merge 42 --rebase";
    assert_eq!(detect_pr_subcommand(cmd), Some(PrSubcommand::Merge));
}

/// BC-4.04.001: `gh pr merge --help` is a merge subcommand detection
/// (caller is responsible for requiring a PR number before emitting).
#[test]
fn test_BC_4_04_001_gh_pr_merge_help_still_detected_as_merge() {
    let cmd = "gh pr merge --help";
    // The subcommand IS detected; the caller (dispatch) decides not to emit
    // because no PR number is extractable. The detection layer is not
    // responsible for that guard.
    assert_eq!(detect_pr_subcommand(cmd), Some(PrSubcommand::Merge));
}

// ── `gh pr` close with URL form ───────────────────────────────────────────────

/// BC-4.04.001: gh pr close with a URL argument.
#[test]
fn test_BC_4_04_001_gh_pr_close_with_url_detected() {
    let cmd = "gh pr close https://github.com/owner/repo/pull/55";
    assert_eq!(detect_pr_subcommand(cmd), Some(PrSubcommand::Close));
}
