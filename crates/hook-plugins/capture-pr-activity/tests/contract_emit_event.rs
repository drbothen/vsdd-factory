//! AC: "Emits pr.created, pr.merged, pr.closed events with PR number,
//!       title, URL, branch fields"
//!
//! BC-4.04.002 — emit `pr.created` / `pr.merged` / `pr.closed` events
//! with schema {pr_number, title, url, branch}; URL field omitted when
//! not parseable (EC-003).
//!
//! BC-4.04.003 — `gh pr create` failure emits `pr.create_failed` event;
//! returns Continue (not Error).

use capture_pr_activity::{
    build_pr_closed_fields, build_pr_create_failed_fields, build_pr_created_fields,
    build_pr_merged_fields, extract_pr_url, pr_number_from_url, pr_repo_from_url,
};

// ── TV (test vectors from bats corpus) ───────────────────────────────────────

/// TV-001: GitHub PR URL parses correctly.
const TV_PR_URL: &str = "https://github.com/owner/repo/pull/42";
const TV_PR_NUMBER: &str = "42";
const TV_PR_REPO: &str = "owner/repo";

// ── extract_pr_url ────────────────────────────────────────────────────────────

/// BC-4.04.002 postcondition: well-formed URL is extracted from text.
#[test]
fn test_BC_4_04_002_extracts_pr_url_from_stdout() {
    let stdout = "https://github.com/owner/repo/pull/42\nsome other output";
    assert_eq!(
        extract_pr_url(stdout),
        Some(TV_PR_URL.to_string())
    );
}

/// BC-4.04.002 postcondition: URL embedded mid-text is still found.
#[test]
fn test_BC_4_04_002_extracts_pr_url_embedded_in_text() {
    let text = "PR created: https://github.com/foo/bar/pull/7 — done";
    assert_eq!(
        extract_pr_url(text),
        Some("https://github.com/foo/bar/pull/7".to_string())
    );
}

/// EC-003: URL not parseable → None (field omitted from event).
#[test]
fn test_BC_4_04_002_ec003_unparseable_url_returns_none() {
    let text = "something-unexpected without any github URL";
    assert_eq!(extract_pr_url(text), None);
}

/// EC-003: empty string → None.
#[test]
fn test_BC_4_04_002_ec003_empty_text_returns_none() {
    assert_eq!(extract_pr_url(""), None);
}

// ── pr_number_from_url ────────────────────────────────────────────────────────

/// BC-4.04.002 postcondition: PR number extracted from well-formed URL.
#[test]
fn test_BC_4_04_002_pr_number_from_url_happy_path() {
    assert_eq!(
        pr_number_from_url(TV_PR_URL),
        Some(TV_PR_NUMBER.to_string())
    );
}

/// BC-4.04.002 postcondition: PR number 99.
#[test]
fn test_BC_4_04_002_pr_number_from_url_returns_99() {
    assert_eq!(
        pr_number_from_url("https://github.com/owner/repo/pull/99"),
        Some("99".to_string())
    );
}

/// BC-4.04.002 precondition violation: malformed URL → None.
#[test]
fn test_BC_4_04_002_pr_number_from_url_malformed_returns_none() {
    assert_eq!(pr_number_from_url("not-a-url"), None);
}

// ── pr_repo_from_url ──────────────────────────────────────────────────────────

/// BC-4.04.002 postcondition: repo slug extracted correctly.
#[test]
fn test_BC_4_04_002_pr_repo_from_url_happy_path() {
    assert_eq!(
        pr_repo_from_url(TV_PR_URL),
        Some(TV_PR_REPO.to_string())
    );
}

/// BC-4.04.002 postcondition: org/repo with hyphens.
#[test]
fn test_BC_4_04_002_pr_repo_from_url_with_hyphens() {
    assert_eq!(
        pr_repo_from_url("https://github.com/my-org/my-repo/pull/5"),
        Some("my-org/my-repo".to_string())
    );
}

// ── build_pr_created_fields ───────────────────────────────────────────────────

/// BC-4.04.002 postcondition: pr.created event type is correct.
#[test]
fn test_BC_4_04_002_pr_created_event_type_is_pr_created() {
    let (event_type, _fields) =
        build_pr_created_fields(TV_PR_URL, TV_PR_NUMBER, TV_PR_REPO, Some("feat: thing"));
    assert_eq!(event_type, "pr.created");
}

/// BC-4.04.002 postcondition: pr.created fields contain pr_number.
#[test]
fn test_BC_4_04_002_pr_created_fields_contain_pr_number() {
    let (_event_type, fields) =
        build_pr_created_fields(TV_PR_URL, TV_PR_NUMBER, TV_PR_REPO, Some("feat: thing"));
    let pr_number = fields.iter().find(|(k, _)| k == "pr_number");
    assert!(pr_number.is_some(), "pr_number field must be present");
    assert_eq!(pr_number.unwrap().1, TV_PR_NUMBER);
}

/// BC-4.04.002 postcondition: pr.created fields contain pr_url.
#[test]
fn test_BC_4_04_002_pr_created_fields_contain_pr_url() {
    let (_event_type, fields) =
        build_pr_created_fields(TV_PR_URL, TV_PR_NUMBER, TV_PR_REPO, Some("feat: thing"));
    let pr_url = fields.iter().find(|(k, _)| k == "pr_url");
    assert!(pr_url.is_some(), "pr_url field must be present");
    assert_eq!(pr_url.unwrap().1, TV_PR_URL);
}

/// BC-4.04.002 postcondition: pr.created fields contain pr_repo.
#[test]
fn test_BC_4_04_002_pr_created_fields_contain_pr_repo() {
    let (_event_type, fields) =
        build_pr_created_fields(TV_PR_URL, TV_PR_NUMBER, TV_PR_REPO, None);
    let pr_repo = fields.iter().find(|(k, _)| k == "pr_repo");
    assert!(pr_repo.is_some(), "pr_repo field must be present");
    assert_eq!(pr_repo.unwrap().1, TV_PR_REPO);
}

/// BC-4.04.002 postcondition: title included when present.
#[test]
fn test_BC_4_04_002_pr_created_fields_include_title_when_provided() {
    let (_event_type, fields) =
        build_pr_created_fields(TV_PR_URL, TV_PR_NUMBER, TV_PR_REPO, Some("feat(S-3.02): thing"));
    let title = fields.iter().find(|(k, _)| k == "title");
    assert!(title.is_some(), "title field must be present when provided");
    assert_eq!(title.unwrap().1, "feat(S-3.02): thing");
}

/// BC-4.04.002 postcondition: title omitted when None.
#[test]
fn test_BC_4_04_002_pr_created_fields_omit_title_when_none() {
    let (_event_type, fields) =
        build_pr_created_fields(TV_PR_URL, TV_PR_NUMBER, TV_PR_REPO, None);
    let title = fields.iter().find(|(k, _)| k == "title");
    assert!(title.is_none(), "title field must be absent when not provided");
}

// ── build_pr_merged_fields ────────────────────────────────────────────────────

/// BC-4.04.002 postcondition: pr.merged event type is correct.
#[test]
fn test_BC_4_04_002_pr_merged_event_type_is_pr_merged() {
    let (event_type, _fields) =
        build_pr_merged_fields(Some(TV_PR_URL), TV_PR_NUMBER, Some(TV_PR_REPO), Some("squash"));
    assert_eq!(event_type, "pr.merged");
}

/// BC-4.04.002 postcondition: pr.merged fields contain pr_number.
#[test]
fn test_BC_4_04_002_pr_merged_fields_contain_pr_number() {
    let (_event_type, fields) =
        build_pr_merged_fields(Some(TV_PR_URL), TV_PR_NUMBER, Some(TV_PR_REPO), None);
    let pr_number = fields.iter().find(|(k, _)| k == "pr_number");
    assert!(pr_number.is_some(), "pr_number field must be present");
    assert_eq!(pr_number.unwrap().1, TV_PR_NUMBER);
}

/// BC-4.04.002 postcondition: merge_strategy included when present.
#[test]
fn test_BC_4_04_002_pr_merged_fields_include_merge_strategy_when_provided() {
    let (_event_type, fields) =
        build_pr_merged_fields(Some(TV_PR_URL), TV_PR_NUMBER, Some(TV_PR_REPO), Some("rebase"));
    let strategy = fields.iter().find(|(k, _)| k == "merge_strategy");
    assert!(strategy.is_some(), "merge_strategy must be present when provided");
    assert_eq!(strategy.unwrap().1, "rebase");
}

/// BC-4.04.002 postcondition: merge_strategy omitted when None.
#[test]
fn test_BC_4_04_002_pr_merged_fields_omit_merge_strategy_when_none() {
    let (_event_type, fields) =
        build_pr_merged_fields(Some(TV_PR_URL), TV_PR_NUMBER, Some(TV_PR_REPO), None);
    let strategy = fields.iter().find(|(k, _)| k == "merge_strategy");
    assert!(strategy.is_none(), "merge_strategy must be absent when not provided");
}

/// EC-003 in merge path: pr_url omitted when not parseable.
#[test]
fn test_BC_4_04_002_ec003_pr_merged_omits_url_when_none() {
    let (_event_type, fields) =
        build_pr_merged_fields(None, TV_PR_NUMBER, None, None);
    let pr_url = fields.iter().find(|(k, _)| k == "pr_url");
    assert!(pr_url.is_none(), "pr_url must be absent when URL was not parseable");
}

// ── build_pr_closed_fields ────────────────────────────────────────────────────

/// BC-4.04.002 postcondition: pr.closed event type is correct.
#[test]
fn test_BC_4_04_002_pr_closed_event_type_is_pr_closed() {
    let (event_type, _fields) =
        build_pr_closed_fields(Some(TV_PR_URL), TV_PR_NUMBER, Some(TV_PR_REPO));
    assert_eq!(event_type, "pr.closed");
}

/// BC-4.04.002 postcondition: pr.closed fields contain pr_number.
#[test]
fn test_BC_4_04_002_pr_closed_fields_contain_pr_number() {
    let (_event_type, fields) =
        build_pr_closed_fields(Some(TV_PR_URL), TV_PR_NUMBER, Some(TV_PR_REPO));
    let pr_number = fields.iter().find(|(k, _)| k == "pr_number");
    assert!(pr_number.is_some(), "pr_number field must be present");
    assert_eq!(pr_number.unwrap().1, TV_PR_NUMBER);
}

// ── build_pr_create_failed_fields (BC-4.04.003) ───────────────────────────────

/// BC-4.04.003 postcondition: pr.create_failed event type is correct.
#[test]
fn test_BC_4_04_003_pr_create_failed_event_type() {
    let cmd = "gh pr create --title t --body b";
    let (event_type, _fields) = build_pr_create_failed_fields(cmd);
    assert_eq!(event_type, "pr.create_failed");
}

/// BC-4.04.003 postcondition: pr.create_failed fields are non-empty.
#[test]
fn test_BC_4_04_003_pr_create_failed_fields_are_non_empty() {
    let cmd = "gh pr create --title t --body b";
    let (_event_type, fields) = build_pr_create_failed_fields(cmd);
    assert!(!fields.is_empty(), "pr.create_failed event must have at least one field");
}
