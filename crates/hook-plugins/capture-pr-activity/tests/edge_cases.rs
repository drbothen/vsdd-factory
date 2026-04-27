//! Edge case tests per the S-3.02 EC table and bats corpus.
//!
//! EC-001: `gh pr create` fails → emit `pr.create_failed`; return Continue.
//! EC-002: Unknown `gh pr` subcommand → no-op; return Continue.
//! EC-003: PR URL not parseable → log warning; omit URL field from event.
//!
//! Additional boundary cases from the bats test corpus:
//! - No PR number in merge command → no event emitted.
//! - Echoed `gh pr create` must NOT match.
//! - Multi-command strings (semicolons, pipes, ampersands).
//! - Whitespace variants in `gh pr create`.

use capture_pr_activity::{
    PrSubcommand, build_pr_created_fields, detect_pr_subcommand, extract_pr_url, pr_number_from_url,
};

// ── EC-001: gh pr create failure ─────────────────────────────────────────────

/// EC-001: build_pr_create_failed_fields returns ("pr.create_failed", fields).
#[test]
fn test_EC_001_build_pr_create_failed_fields_called_panics_in_stub() {
    use capture_pr_activity::build_pr_create_failed_fields;
    let (event_type, fields) = build_pr_create_failed_fields("gh pr create --title t --body b");
    assert_eq!(event_type, "pr.create_failed");
    assert!(
        !fields.is_empty(),
        "pr.create_failed event must have at least one field"
    );
}

// ── EC-002: Unknown gh pr subcommand → no-op ─────────────────────────────────

/// EC-002: `gh pr list` is unknown → None.
#[test]
fn test_EC_002_gh_pr_list_is_unknown_subcommand() {
    assert_eq!(detect_pr_subcommand("gh pr list"), None);
}

/// EC-002: `gh pr diff` is unknown → None.
#[test]
fn test_EC_002_gh_pr_diff_is_unknown_subcommand() {
    assert_eq!(detect_pr_subcommand("gh pr diff 42"), None);
}

/// EC-002: `gh pr checkout` is unknown → None.
#[test]
fn test_EC_002_gh_pr_checkout_is_unknown_subcommand() {
    assert_eq!(detect_pr_subcommand("gh pr checkout 42"), None);
}

/// EC-002: `gh pr edit` is unknown → None.
#[test]
fn test_EC_002_gh_pr_edit_is_unknown_subcommand() {
    assert_eq!(detect_pr_subcommand("gh pr edit 42 --title new"), None);
}

// ── EC-003: PR URL not parseable ─────────────────────────────────────────────

/// EC-003: non-GitHub URL is not extracted.
#[test]
fn test_EC_003_non_github_url_not_extracted() {
    let text = "https://gitlab.com/owner/repo/merge_requests/1";
    assert_eq!(extract_pr_url(text), None);
}

/// EC-003: partial URL (missing /pull/<N>) is not extracted.
#[test]
fn test_EC_003_partial_github_url_not_extracted() {
    let text = "https://github.com/owner/repo";
    assert_eq!(extract_pr_url(text), None);
}

/// EC-003: pr_number_from_url returns None for non-pull path.
#[test]
fn test_EC_003_pr_number_none_for_non_pull_url() {
    assert_eq!(
        pr_number_from_url("https://github.com/owner/repo/issues/5"),
        None
    );
}

/// EC-003: build_pr_created_fields with empty URL still works (url field present but empty).
/// This tests that the caller can construct a valid create event even
/// when the URL was not parseable — the url parameter would be empty.
#[test]
fn test_EC_003_pr_created_fields_panics_in_stub() {
    let (event_type, fields) = build_pr_created_fields("", "42", "owner/repo", None);
    assert_eq!(event_type, "pr.created");
    let pr_number = fields.iter().find(|(k, _)| k == "pr_number");
    assert!(pr_number.is_some(), "pr_number field must be present");
    assert_eq!(pr_number.unwrap().1, "42");
}

// ── Boundary: echo/shell expansion must not match ────────────────────────────

/// bats: "echoed mention of gh pr create is NOT matched"
#[test]
fn test_TV_001_echo_gh_pr_create_not_matched() {
    let cmd = "echo \"Use gh pr create to open a PR\"";
    assert_eq!(detect_pr_subcommand(cmd), None);
}

/// bats: comment containing gh pr create not matched.
#[test]
fn test_TV_002_comment_containing_gh_pr_create_not_matched() {
    let cmd = "# gh pr create --title t --body b";
    // A comment-only line must not match — the spec says "not a real invocation".
    assert_eq!(detect_pr_subcommand(cmd), None);
}

// ── Boundary: multi-token command strings ────────────────────────────────────

/// Multi-command: create after `&&`.
#[test]
fn test_TV_003_gh_pr_create_after_ampersand_chain_detected() {
    let cmd = "git push && gh pr create --title t --body b";
    assert_eq!(detect_pr_subcommand(cmd), Some(PrSubcommand::Create));
}

/// Multi-command: merge after `||`.
#[test]
fn test_TV_004_gh_pr_merge_after_or_chain_detected() {
    let cmd = "git push || gh pr merge 42 --squash";
    assert_eq!(detect_pr_subcommand(cmd), Some(PrSubcommand::Merge));
}

// ── Boundary: extra whitespace in gh pr create ────────────────────────────────

/// Extra whitespace between tokens.
#[test]
fn test_TV_005_extra_whitespace_in_gh_pr_create() {
    // "gh  pr  create" with double spaces — the real gh CLI requires single-space
    // separation, so this must NOT match.
    assert_eq!(
        detect_pr_subcommand("gh  pr  create --title t --body b"),
        None
    );
}

// ── Boundary: pr number extraction edge cases ─────────────────────────────────

/// PR number must be numeric.
#[test]
fn test_TV_006_pr_number_must_be_numeric() {
    // A URL with a non-numeric PR identifier is invalid.
    assert_eq!(
        pr_number_from_url("https://github.com/owner/repo/pull/abc"),
        None
    );
}

/// PR number 1 (minimal).
#[test]
fn test_TV_007_pr_number_one_extracted_correctly() {
    assert_eq!(
        pr_number_from_url("https://github.com/owner/repo/pull/1"),
        Some("1".to_string())
    );
}

/// Large PR number (5 digits).
#[test]
fn test_TV_008_large_pr_number_extracted_correctly() {
    assert_eq!(
        pr_number_from_url("https://github.com/owner/repo/pull/10000"),
        Some("10000".to_string())
    );
}
