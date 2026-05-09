//! Unit tests for validate-stable-anchors.
//!
//! Exercises the production functions in `lib.rs` via injectable callbacks.
//! Test naming follows the BC-based convention: test_BC_S_SS_NNN_xxx() where
//! applicable. Since TD-031 has no BC yet, tests use test_TD031_xxx naming.
//!
//! # Coverage areas
//! - scan_line: detects `*.<ext>:NNN` patterns for SOURCE_FILE_EXTENSIONS only
//! - scan_line: does NOT flag markdown (.md), html, txt (out of scope)
//! - scan_spec: exemption zones (Amendment, Changelog, SITES fence)
//! - is_spec_target: only `.factory/specs/**/*.md` targeted
//! - hook_logic: end-to-end with injectable callbacks
//! - format_violations: sanity check on output
//! - is_source_line_cite: boundary conditions and URL exclusion

#![allow(clippy::type_complexity)]

use super::*;
use serde_json::json;

// -----------------------------------------------------------------------
// Test helpers
// -----------------------------------------------------------------------

fn make_payload(tool_name: &str, file_path: Option<&str>, content: Option<&str>) -> HookPayload {
    let mut v = json!({
        "event_name": "PreToolUse",
        "session_id": "test-session",
        "dispatcher_trace_id": "test-trace",
        "tool_name": tool_name,
        "tool_input": {}
    });
    if let Some(path) = file_path {
        v["tool_input"]["file_path"] = json!(path);
    }
    if let Some(c) = content {
        v["tool_input"]["content"] = json!(c);
    }
    serde_json::from_value(v).expect("fixture must deserialize")
}

fn make_edit_payload(file_path: &str, new_string: &str) -> HookPayload {
    let v = json!({
        "event_name": "PreToolUse",
        "session_id": "test-session",
        "dispatcher_trace_id": "test-trace",
        "tool_name": "Edit",
        "tool_input": {
            "file_path": file_path,
            "new_string": new_string,
            "old_string": "old"
        }
    });
    serde_json::from_value(v).expect("fixture must deserialize")
}

/// Run hook_logic with injectable callbacks.
/// `file_content` is returned by the read_file callback.
fn run_logic(
    payload: HookPayload,
    file_content: Result<String, String>,
) -> (
    HookResult,
    Vec<(u8, String)>,
    Vec<(String, Vec<(String, String)>)>,
) {
    let mut log_calls: Vec<(u8, String)> = Vec::new();
    let mut events: Vec<(String, Vec<(String, String)>)> = Vec::new();

    let result = hook_logic(
        payload,
        HookCallbacks {
            read_file: move |_path| file_content.clone(),
            emit_event: |event_type, fields| {
                events.push((
                    event_type.to_string(),
                    fields
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect(),
                ));
            },
            log: |level, msg| {
                log_calls.push((level, msg.to_string()));
            },
        },
    );

    (result, log_calls, events)
}

// -----------------------------------------------------------------------
// scan_line tests
// -----------------------------------------------------------------------

#[test]
fn test_TD031_scan_line_detects_main_rs_colon_nnn() {
    // Classic TD-031 violation: main.rs:416
    assert!(
        scan_line("The call site is at `main.rs:416` in the dispatcher."),
        "scan_line must detect main.rs:416 (TD-031 canonical violation)"
    );
}

#[test]
fn test_TD031_scan_line_detects_emit_event_rs_colon_nnn() {
    assert!(
        scan_line("See `emit_event.rs:49` string coercion rule."),
        "scan_line must detect emit_event.rs:49"
    );
}

#[test]
fn test_TD031_scan_line_detects_registry_rs_colon_nnn() {
    assert!(
        scan_line("registry.rs:83 declares cwd_allow."),
        "scan_line must detect registry.rs:83"
    );
}

#[test]
fn test_TD031_scan_line_detects_partition_rs_colon_nnn() {
    assert!(
        scan_line("partition.rs:90 implements partition_plugins."),
        "scan_line must detect partition.rs:90"
    );
}

#[test]
fn test_TD031_scan_line_detects_aggregator_rs_colon_nnn() {
    assert!(
        scan_line("aggregator.rs:55 is the entry point."),
        "scan_line must detect aggregator.rs:55"
    );
}

#[test]
fn test_TD031_scan_line_detects_any_rs_colon_nnn() {
    // The lint is general — any *.rs:NNN is volatile.
    assert!(
        scan_line("See exec_subprocess.rs:248 for the implementation."),
        "scan_line must detect exec_subprocess.rs:248 (generalised lint)"
    );
}

#[test]
fn test_TD031_scan_line_no_match_for_stable_symbol_anchor() {
    // Stable anchor: function name, no line number.
    assert!(
        !scan_line("The call at `factory_dispatcher::main::run` is the entry point."),
        "scan_line must NOT flag stable symbol anchor factory_dispatcher::main::run"
    );
}

#[test]
fn test_TD031_scan_line_no_match_for_plain_rs_extension() {
    // ".rs" without ":NNN" is fine (e.g., a file path in prose).
    assert!(
        !scan_line("The file `main.rs` contains the dispatcher logic."),
        "scan_line must NOT flag `main.rs` without a line number"
    );
}

#[test]
fn test_TD031_scan_line_no_match_for_rs_colon_non_digit() {
    // ".rs:word" is not a line cite.
    assert!(
        !scan_line("See main.rs:emit_event for the function."),
        "scan_line must NOT flag main.rs:emit_event (no digit after colon)"
    );
}

#[test]
fn test_TD031_scan_line_no_match_for_empty_line() {
    assert!(!scan_line(""), "scan_line must return false for empty line");
}

#[test]
fn test_TD031_scan_line_no_match_for_generic_prose() {
    assert!(
        !scan_line("The stable anchor convention is defined in TD-VSDD-091."),
        "scan_line must return false for ordinary prose"
    );
}

#[test]
fn test_TD031_scan_line_detects_multi_digit_line_number() {
    assert!(
        scan_line("line range main.rs:133-142 covers the emit calls."),
        "scan_line must detect main.rs:133 (multi-digit, range notation)"
    );
}

// -----------------------------------------------------------------------
// is_spec_target tests
// -----------------------------------------------------------------------

#[test]
fn test_TD031_is_spec_target_accepts_behavioral_contract() {
    assert!(
        is_spec_target(".factory/specs/behavioral-contracts/ss-07/BC-7.06.001.md"),
        "is_spec_target must return true for BC spec under .factory/specs/"
    );
}

#[test]
fn test_TD031_is_spec_target_accepts_vp_spec() {
    assert!(
        is_spec_target(".factory/specs/verification-properties/VP-079.md"),
        "is_spec_target must return true for VP spec"
    );
}

#[test]
fn test_TD031_is_spec_target_accepts_prd() {
    assert!(
        is_spec_target(".factory/specs/prd.md"),
        "is_spec_target must return true for prd.md"
    );
}

#[test]
fn test_TD031_is_spec_target_rejects_non_spec_factory_file() {
    assert!(
        !is_spec_target(".factory/stories/S-15.01-convergence.md"),
        "is_spec_target must return false for stories (not specs/)"
    );
}

#[test]
fn test_TD031_is_spec_target_rejects_non_factory_file() {
    assert!(
        !is_spec_target("src/lib.rs"),
        "is_spec_target must return false for source files"
    );
}

#[test]
fn test_TD031_is_spec_target_rejects_non_md_spec_file() {
    assert!(
        !is_spec_target(".factory/specs/behavioral-contracts/ss-07/BC-7.06.001.toml"),
        "is_spec_target must return false for non-.md files"
    );
}

#[test]
fn test_TD031_is_spec_target_rejects_registry_toml() {
    // hooks-registry.toml is NOT a spec file.
    assert!(
        !is_spec_target("plugins/vsdd-factory/hooks-registry.toml"),
        "is_spec_target must return false for hooks-registry.toml"
    );
}

// -----------------------------------------------------------------------
// scan_spec tests — exemption zones
// -----------------------------------------------------------------------

#[test]
fn test_TD031_scan_spec_body_violation_detected() {
    let content = "# Title\n\nThe call is at `main.rs:416` in the dispatcher.\n";
    let violations = scan_spec(content);
    assert_eq!(
        violations.len(),
        1,
        "scan_spec must detect main.rs:416 in spec body"
    );
    assert_eq!(violations[0].line_number, 3);
}

#[test]
fn test_TD031_scan_spec_amendment_section_exempt() {
    let content = r#"# Title

The call is here (body — enforced).

## Amendment 2026-05-08 F5 fix-burst-14

The old cite was `main.rs:416` but it drifted.
This is `main.rs:427` in the amended section.
"#;
    let violations = scan_spec(content);
    assert!(
        violations.is_empty(),
        "scan_spec must not flag rs:NNN in Amendment sections. Got: {:?}",
        violations
    );
}

#[test]
fn test_TD031_scan_spec_changelog_section_exempt() {
    let content = r#"# Title

Normal body — enforced.

## Changelog

| v1.10 | main.rs:394 updated to main.rs:416 |
| v1.9 | main.rs:162 added |
"#;
    let violations = scan_spec(content);
    assert!(
        violations.is_empty(),
        "scan_spec must not flag rs:NNN in Changelog sections. Got: {:?}",
        violations
    );
}

#[test]
fn test_TD031_scan_spec_non_exempt_heading_reenables_lint() {
    let content = r#"# Title

## Amendment historical

main.rs:416 here (exempt).

## Implementation Notes

main.rs:500 here (should be flagged).
"#;
    let violations = scan_spec(content);
    assert_eq!(
        violations.len(),
        1,
        "scan_spec must enforce lint after non-Amendment heading. Got: {:?}",
        violations
    );
    assert_eq!(violations[0].line_number, 9);
}

#[test]
fn test_TD031_scan_spec_sites_fence_exempt() {
    // VP-079 Scenario 6 SITES array inside a bash code fence.
    let content = r#"# Title

Spec body — enforced.

## Amendment 2026-05-08

```bash
    SITES=(
        "133 emit_dispatcher_schema_mismatch SITE_1"
        "143,150 emit_dispatcher_registry_invalid SITE_2"
        "423 emit_plugin_async_block_discarded SITE_3"
    )
```
"#;
    let violations = scan_spec(content);
    assert!(
        violations.is_empty(),
        "scan_spec must not flag rs:NNN in SITES fence within Amendment section. Got: {:?}",
        violations
    );
}

#[test]
fn test_TD031_scan_spec_code_fence_in_body_also_exempt() {
    // Code fences in spec body (illustrative code blocks) are also exempt.
    let content = r#"# Title

Normal prose here — enforced.

```rust
// This is illustrative code
let x = main_rs_line_416(); // main.rs:416 in code comment
```

More prose here — enforced.
"#;
    let violations = scan_spec(content);
    assert!(
        violations.is_empty(),
        "scan_spec must not flag rs:NNN inside code fences in body. Got: {:?}",
        violations
    );
}

#[test]
fn test_TD031_scan_spec_violation_before_amendment_detected() {
    let content = r#"# Title

The violation is at `main.rs:133` here in the body.

## Amendment 2026-05-08

main.rs:416 here (exempt).
"#;
    let violations = scan_spec(content);
    assert_eq!(
        violations.len(),
        1,
        "scan_spec must detect violation in body before Amendment section"
    );
    assert_eq!(violations[0].line_number, 3);
}

#[test]
fn test_TD031_scan_spec_clean_content_returns_empty() {
    let content = r#"# Title

The stable anchor is `factory_dispatcher::main::run` — no line cite.
The function `emit_plugin_async_block_discarded` is called here.
"#;
    let violations = scan_spec(content);
    assert!(
        violations.is_empty(),
        "scan_spec must return empty violations for clean content"
    );
}

#[test]
fn test_TD031_scan_spec_multiple_violations_detected() {
    let content = r#"# Title

First cite at `main.rs:133`.
Second cite at `emit_event.rs:49`.
Third cite at `partition.rs:90`.
"#;
    let violations = scan_spec(content);
    assert_eq!(
        violations.len(),
        3,
        "scan_spec must detect all three violations. Got: {:?}",
        violations
    );
}

// -----------------------------------------------------------------------
// hook_logic end-to-end tests
// -----------------------------------------------------------------------

#[test]
fn test_TD031_hook_logic_non_spec_path_returns_continue() {
    let payload = make_payload("Write", Some("src/lib.rs"), Some("content"));
    let (result, _, _) = run_logic(payload, Ok("irrelevant".to_string()));
    assert_eq!(
        result,
        HookResult::Continue,
        "hook_logic must return Continue for non-spec path"
    );
}

#[test]
fn test_TD031_hook_logic_no_file_path_returns_continue() {
    let payload = make_payload("Write", None, None);
    let (result, _, _) = run_logic(payload, Ok("irrelevant".to_string()));
    assert_eq!(
        result,
        HookResult::Continue,
        "hook_logic must return Continue when file_path absent"
    );
}

#[test]
fn test_TD031_hook_logic_clean_write_returns_continue() {
    let payload = make_payload(
        "Write",
        Some(".factory/specs/behavioral-contracts/ss-07/BC-7.06.001.md"),
        Some("# BC\n\nThe anchor is `factory_dispatcher::main::run`.\n"),
    );
    let (result, _, _) = run_logic(payload, Ok("existing content".to_string()));
    assert_eq!(
        result,
        HookResult::Continue,
        "hook_logic must return Continue for clean spec write"
    );
}

#[test]
fn test_TD031_hook_logic_violation_in_write_content_blocks() {
    let payload = make_payload(
        "Write",
        Some(".factory/specs/verification-properties/VP-079.md"),
        Some("# VP-079\n\nThe call at `main.rs:416` is flagged.\n"),
    );
    let (result, _, events) = run_logic(payload, Ok("existing".to_string()));
    assert!(
        matches!(result, HookResult::Block { .. }),
        "hook_logic must Block when write content contains main.rs:416. Got: {:?}",
        result
    );
    if let HookResult::Block { reason } = &result {
        assert!(
            reason.contains("TD_031_STABLE_ANCHOR_VIOLATION"),
            "block reason must contain error code. Got: {}",
            reason
        );
        assert!(
            reason.contains("validate-stable-anchors"),
            "block reason must contain hook name. Got: {}",
            reason
        );
    }
    assert!(
        events.iter().any(|(t, _)| t == "td031.violation"),
        "hook_logic must emit td031.violation event. Got: {:?}",
        events
    );
}

#[test]
fn test_TD031_hook_logic_violation_in_edit_new_string_blocks() {
    let payload = make_edit_payload(
        ".factory/specs/behavioral-contracts/ss-07/BC-7.06.001.md",
        "The fixed anchor at `main.rs:289`.",
    );
    let (result, _, _) = run_logic(payload, Err("file not found".to_string()));
    assert!(
        matches!(result, HookResult::Block { .. }),
        "hook_logic must Block when new_string contains main.rs:289"
    );
}

#[test]
fn test_TD031_hook_logic_block_reason_starts_with_blocked_by() {
    let payload = make_payload(
        "Write",
        Some(".factory/specs/prd.md"),
        Some("# PRD\n\nSee `main.rs:100` for details.\n"),
    );
    let (result, _, _) = run_logic(payload, Ok("".to_string()));
    if let HookResult::Block { reason } = result {
        assert!(
            reason.starts_with("BLOCKED by"),
            "block reason must start with 'BLOCKED by' (canonical format). Got: {}",
            reason
        );
    }
}

#[test]
fn test_TD031_hook_logic_file_unreadable_returns_continue() {
    // If the file is unreadable and there's no content/new_string in payload,
    // the hook should gracefully degrade to Continue.
    let payload = make_payload(
        "Write",
        Some(".factory/specs/prd.md"),
        None, // no content field
    );
    let (result, _, _) = run_logic(payload, Err("file not found".to_string()));
    assert_eq!(
        result,
        HookResult::Continue,
        "hook_logic must return Continue when file is unreadable and no content in payload"
    );
}

#[test]
fn test_TD031_hook_logic_amendment_in_write_content_exempt() {
    // Write content with an Amendment section — exempt from lint.
    let content =
        "# BC-7.06.001\n\n## Amendment 2026-05-08\n\nOld cite: main.rs:289 → stable anchor.\n";
    let payload = make_payload(
        "Write",
        Some(".factory/specs/behavioral-contracts/ss-07/BC-7.06.001.md"),
        Some(content),
    );
    let (result, _, _) = run_logic(payload, Ok("".to_string()));
    assert_eq!(
        result,
        HookResult::Continue,
        "hook_logic must Continue when rs:NNN appears only in Amendment section"
    );
}

// -----------------------------------------------------------------------
// is_source_line_cite boundary tests
// -----------------------------------------------------------------------

#[test]
fn test_TD031_is_source_line_cite_basic_rs_match() {
    let line = "main.rs:416";
    let bytes = line.as_bytes();
    let pos = line.find('.').unwrap();
    assert!(is_source_line_cite(bytes, pos));
}

#[test]
fn test_TD031_is_source_line_cite_no_digit_after_colon() {
    let line = "main.rs:emit";
    let bytes = line.as_bytes();
    let pos = line.find('.').unwrap();
    assert!(!is_source_line_cite(bytes, pos));
}

#[test]
fn test_TD031_is_source_line_cite_no_word_char_before() {
    // `.rs:416` with no stem before — should not match.
    let line = ".rs:416";
    let bytes = line.as_bytes();
    let pos = 0usize; // dot is at position 0
    assert!(!is_source_line_cite(bytes, pos));
}

#[test]
fn test_TD031_is_source_line_cite_underscore_stem_matches() {
    let line = "my_file.rs:100 reference";
    let bytes = line.as_bytes();
    let pos = line.find('.').unwrap();
    assert!(is_source_line_cite(bytes, pos));
}

// -----------------------------------------------------------------------
// scan_line — additional extension tests (Task C)
// -----------------------------------------------------------------------

#[test]
fn test_TD031_scan_line_detects_bats_colon_nnn() {
    assert!(
        scan_line("See `setup.bats:42` for the failing hook test."),
        "scan_line must detect setup.bats:42 (.bats extension)"
    );
}

#[test]
fn test_TD031_scan_line_detects_toml_colon_nnn() {
    assert!(
        scan_line("The workspace setting at `Cargo.toml:17` is wrong."),
        "scan_line must detect Cargo.toml:17 (.toml extension)"
    );
}

#[test]
fn test_TD031_scan_line_detects_sh_colon_nnn() {
    assert!(
        scan_line("install.sh:99 is the deployment step."),
        "scan_line must detect install.sh:99 (.sh extension)"
    );
}

#[test]
fn test_TD031_scan_line_no_match_for_md_extension() {
    // Markdown cross-document references (`.md:NNN`) are out of scope for
    // TD-VSDD-091. They are handled by heading-anchor conventions, not this hook.
    assert!(
        !scan_line("Cross-spec citation: `BC-3.08.001.md:123` is a heading anchor ref."),
        "scan_line must NOT flag .md:NNN (markdown cross-doc refs are out of scope)"
    );
}

#[test]
fn test_TD031_scan_line_no_match_for_html_extension() {
    // HTML files are doc/markup, not source-code — excluded from allowlist.
    assert!(
        !scan_line("See `index.html:42` for the rendered output."),
        "scan_line must NOT flag .html:NNN (html is not in SOURCE_FILE_EXTENSIONS)"
    );
}

#[test]
fn test_TD031_scan_line_no_match_for_txt_extension() {
    // Plain text files are doc, not source-code — excluded from allowlist.
    assert!(
        !scan_line("Reference notes.txt:5 for the meeting summary."),
        "scan_line must NOT flag .txt:NNN (txt is not in SOURCE_FILE_EXTENSIONS)"
    );
}

#[test]
fn test_TD031_scan_line_detects_lobster_colon_nnn() {
    // .lobster is a VSDD workflow file extension — included in the allowlist.
    assert!(
        scan_line("Workflow step at `pipeline.lobster:18` needs updating."),
        "scan_line must detect .lobster:NNN (.lobster is in SOURCE_FILE_EXTENSIONS)"
    );
}

#[test]
fn test_TD031_scan_line_detects_py_colon_nnn() {
    assert!(
        scan_line("Python entry point at `manage.py:12`."),
        "scan_line must detect manage.py:12 (.py extension)"
    );
}

#[test]
fn test_TD031_scan_line_detects_yaml_colon_nnn() {
    assert!(
        scan_line("Pipeline step at `deploy.yaml:88`."),
        "scan_line must detect deploy.yaml:88 (.yaml extension)"
    );
}

#[test]
fn test_TD031_scan_line_detects_yml_colon_nnn() {
    assert!(
        scan_line("CI config at `ci.yml:33` needs updating."),
        "scan_line must detect ci.yml:33 (.yml extension)"
    );
}

#[test]
fn test_TD031_scan_line_detects_ts_colon_nnn() {
    assert!(
        scan_line("TypeScript type at `types.ts:200`."),
        "scan_line must detect types.ts:200 (.ts extension)"
    );
}

#[test]
fn test_TD031_scan_line_detects_go_colon_nnn() {
    assert!(
        scan_line("Go handler at `server.go:77`."),
        "scan_line must detect server.go:77 (.go extension)"
    );
}

// -----------------------------------------------------------------------
// URL exclusion tests (Task C)
// -----------------------------------------------------------------------

#[test]
fn test_TD031_scan_line_no_match_for_url_with_port() {
    // https://example.com:8080/foo — the ":8080" follows ".com" which looks
    // like "com:8" but the URL guard (lookbehind for "://") must prevent match.
    assert!(
        !scan_line("See https://example.com:8080/foo for details."),
        "scan_line must NOT flag https://example.com:8080 (URL port, not source cite)"
    );
}

#[test]
fn test_TD031_scan_line_no_match_for_http_url() {
    assert!(
        !scan_line("Visit http://localhost.dev:3000/api to test."),
        "scan_line must NOT flag http://localhost.dev:3000 (URL port)"
    );
}

#[test]
fn test_TD031_scan_line_git_at_url_no_match() {
    // git@github.com:user/repo.git — ".git" would match if not excluded, but
    // "github.com:user" has "://" nowhere near; however ".git:u" — 'u' is not
    // a digit, so the digit check prevents this regardless.
    assert!(
        !scan_line("Clone: git@github.com:user/repo.git"),
        "scan_line must NOT flag git@github.com:user/repo.git (no digit after colon)"
    );
}

#[test]
fn test_TD031_scan_line_extension_too_long_no_match() {
    // Extension longer than 8 characters should not match.
    assert!(
        !scan_line("See file.toolongext:42 for details."),
        "scan_line must NOT flag extensions longer than 8 characters"
    );
}

#[test]
fn test_TD031_scan_line_uppercase_ext_no_match() {
    // Uppercase extension bytes are not ASCII lowercase — should not match.
    assert!(
        !scan_line("See file.RS:42 for the Rust source."),
        "scan_line must NOT flag uppercase extensions (only lowercase ext bytes match)"
    );
}

#[test]
fn test_TD031_scan_line_no_match_for_bats_range_notation_no_digit_gap() {
    // ".bats:NNN-NNN" — scan_line checks if the first char after colon is a
    // digit. "133" starts with '1' so this SHOULD match (same as main.rs:133-142).
    assert!(
        scan_line("See `validate.bats:133-142` for the range."),
        "scan_line must detect .bats:133-142 (first char after colon is a digit)"
    );
}

#[test]
fn test_is_spec_target_absolute_path_matches() {
    assert!(is_spec_target("/abs/project/.factory/specs/foo.md"));
    assert!(is_spec_target(
        "/home/user/project/.factory/specs/bc/BC-1.01.001.md"
    ));
}

#[test]
fn test_is_spec_target_relative_path_matches() {
    assert!(is_spec_target(".factory/specs/foo.md"));
    assert!(is_spec_target(".factory/specs/architecture/SS-01.md"));
}

#[test]
fn test_is_spec_target_rejects_non_spec_paths() {
    assert!(!is_spec_target("crates/foo/src/lib.rs"));
    assert!(!is_spec_target("/path/to/.factory/cycles/foo.md")); // cycles != specs
    assert!(!is_spec_target("/path/to/.factory/specs/foo.txt")); // wrong extension
    assert!(!is_spec_target(".factory/storiesfoo.md")); // missing /
}

#[test]
fn test_is_spec_target_rejects_partial_matches() {
    // Should NOT match a file named ".factory/specs.md" (no trailing /)
    assert!(!is_spec_target(".factory/specs.md"));
    // Should NOT match prefix.factory/specs/foo.md (no leading slash before .factory)
    assert!(!is_spec_target("prefix.factory/specs/foo.md"));
}
