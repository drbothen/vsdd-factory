//! capture-pr-activity hook — pre-implementation stub.
//!
//! Real implementation arrives in S-3.02. This stub provides the
//! function surface that the RED-gate test suite calls; every public
//! function panics with `unimplemented!()` so that all tests fail
//! before the implementer begins.

use vsdd_hook_sdk::{HookPayload, HookResult};

// ── Public function surface (all unimplemented) ──────────────────────────────

/// Extract the `tool_input.command` string from a `HookPayload`.
/// Returns `None` if the field is absent or not a string.
pub fn extract_command(_payload: &HookPayload) -> Option<String> {
    unimplemented!("S-3.02: extract_command not yet implemented")
}

/// Determine which PR subcommand is present in `command`, if any.
///
/// Returns one of: `Some(PrSubcommand::Create)`, `Some(PrSubcommand::Merge)`,
/// `Some(PrSubcommand::Close)` or `None` when the command does not match a
/// real `gh pr <sub>` invocation.
pub fn detect_pr_subcommand(_command: &str) -> Option<PrSubcommand> {
    unimplemented!("S-3.02: detect_pr_subcommand not yet implemented")
}

/// Extract a GitHub PR URL (format: `https://github.com/<owner>/<repo>/pull/<N>`)
/// from arbitrary text. Returns `None` when no URL is found or the URL is
/// not parseable according to EC-003.
pub fn extract_pr_url(_text: &str) -> Option<String> {
    unimplemented!("S-3.02: extract_pr_url not yet implemented")
}

/// Parse the PR number from a URL. Returns `None` when the URL does not
/// contain a numeric suffix after `/pull/`.
pub fn pr_number_from_url(_url: &str) -> Option<String> {
    unimplemented!("S-3.02: pr_number_from_url not yet implemented")
}

/// Parse the repository slug (`owner/repo`) from a GitHub PR URL.
pub fn pr_repo_from_url(_url: &str) -> Option<String> {
    unimplemented!("S-3.02: pr_repo_from_url not yet implemented")
}

/// Detect the merge strategy flag in a `gh pr merge` command string.
/// Returns `Some("squash" | "rebase" | "merge")` or `None`.
pub fn detect_merge_strategy(_command: &str) -> Option<String> {
    unimplemented!("S-3.02: detect_merge_strategy not yet implemented")
}

/// Build the field list for a `pr.created` event from the parsed payload.
/// Returns the event-type string and a list of `(key, value)` string pairs
/// ready to be passed to `vsdd_hook_sdk::host::emit_event`.
pub fn build_pr_created_fields(
    _pr_url: &str,
    _pr_number: &str,
    _pr_repo: &str,
    _title: Option<&str>,
) -> (&'static str, Vec<(String, String)>) {
    unimplemented!("S-3.02: build_pr_created_fields not yet implemented")
}

/// Build the field list for a `pr.merged` event.
pub fn build_pr_merged_fields(
    _pr_url: Option<&str>,
    _pr_number: &str,
    _pr_repo: Option<&str>,
    _merge_strategy: Option<&str>,
) -> (&'static str, Vec<(String, String)>) {
    unimplemented!("S-3.02: build_pr_merged_fields not yet implemented")
}

/// Build the field list for a `pr.closed` event.
pub fn build_pr_closed_fields(
    _pr_url: Option<&str>,
    _pr_number: &str,
    _pr_repo: Option<&str>,
) -> (&'static str, Vec<(String, String)>) {
    unimplemented!("S-3.02: build_pr_closed_fields not yet implemented")
}

/// Build the field list for a `pr.create_failed` event (EC-001).
pub fn build_pr_create_failed_fields(_command: &str) -> (&'static str, Vec<(String, String)>) {
    unimplemented!("S-3.02: build_pr_create_failed_fields not yet implemented")
}

/// Top-level hook dispatch: drives the full PostToolUse/Bash pipeline.
///
/// This is what the `#[hook]` entry-point will delegate to once the
/// host FFI is available. For native (non-WASM) test targets we call it
/// directly without going through the macro.
pub fn dispatch(_payload: &HookPayload) -> HookResult {
    unimplemented!("S-3.02: dispatch not yet implemented")
}

// ── Discriminant enum ────────────────────────────────────────────────────────

/// The three PR subcommands this plugin handles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrSubcommand {
    Create,
    Merge,
    Close,
}
