//! capture-pr-activity — PostToolUse/Bash WASM hook plugin.
//!
//! Watches for `gh pr create`, `gh pr merge`, and `gh pr close` invocations
//! in `tool_input.command`, emits structured `pr.created`, `pr.merged`,
//! `pr.closed`, or `pr.create_failed` events via the host FFI.
//!
//! Non-PR bash commands and non-Bash tool events are no-ops (Continue).

use vsdd_hook_sdk::{HookPayload, HookResult};

// ── Public function surface ───────────────────────────────────────────────────

/// Extract the `tool_input.command` string from a `HookPayload`.
/// Returns `None` if the field is absent or not a string.
pub fn extract_command(payload: &HookPayload) -> Option<String> {
    payload
        .tool_input
        .get("command")
        .and_then(|v| v.as_str())
        .map(|s| s.to_owned())
}

/// Determine which PR subcommand is present in `command`, if any.
///
/// Returns one of: `Some(PrSubcommand::Create)`, `Some(PrSubcommand::Merge)`,
/// `Some(PrSubcommand::Close)` or `None` when the command does not match a
/// real `gh pr <sub>` invocation.
///
/// Anchors to real invocation boundaries: must be preceded by start-of-string,
/// `;`, `&`, `|`, or whitespace. Must NOT match inside quoted strings.
/// Requires exactly single-space separation between tokens.
pub fn detect_pr_subcommand(command: &str) -> Option<PrSubcommand> {
    // Strip shell comments: a `#` outside of quotes at the start of a token
    // boundary means the rest is a comment. For our purposes, if the command
    // starts with `#` (after optional whitespace) it is entirely a comment.
    let trimmed = command.trim_start();
    if trimmed.starts_with('#') {
        return None;
    }

    // Check for gh pr subcommands using boundary-anchored string search.
    // We scan for the pattern `gh pr <sub>` where the `gh` token is preceded
    // by a real shell separator or the start of the string, and NOT inside
    // a double-quoted segment.
    detect_gh_pr_in_command(command)
}

/// Extract a GitHub PR URL (format: `https://github.com/<owner>/<repo>/pull/<N>`)
/// from arbitrary text. Returns `None` when no URL is found or the URL is
/// not parseable according to EC-003.
pub fn extract_pr_url(text: &str) -> Option<String> {
    // Scan for the GitHub PR URL pattern.
    let prefix = "https://github.com/";
    let mut pos = 0;
    while let Some(start) = text[pos..].find(prefix) {
        let abs_start = pos + start;
        let rest = &text[abs_start + prefix.len()..];
        // rest should look like: owner/repo/pull/N
        if let Some(url) = try_parse_pr_url_from_rest(rest, abs_start, prefix) {
            return Some(url);
        }
        pos = abs_start + prefix.len();
    }
    None
}

/// Parse the PR number from a URL. Returns `None` when the URL does not
/// contain a numeric suffix after `/pull/`.
pub fn pr_number_from_url(url: &str) -> Option<String> {
    let pull_marker = "/pull/";
    let idx = url.find(pull_marker)?;
    let after_pull = &url[idx + pull_marker.len()..];
    // Extract the numeric portion (stop at any non-digit).
    let number: String = after_pull
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    if number.is_empty() {
        return None;
    }
    Some(number)
}

/// Parse the repository slug (`owner/repo`) from a GitHub PR URL.
pub fn pr_repo_from_url(url: &str) -> Option<String> {
    let prefix = "https://github.com/";
    let rest = url.strip_prefix(prefix)?;
    // rest = "owner/repo/pull/N..."
    // We want the first two path segments.
    let pull_idx = rest.find("/pull/")?;
    let owner_repo = &rest[..pull_idx];
    // Validate it has exactly one slash (owner/repo).
    let slash_count = owner_repo.chars().filter(|&c| c == '/').count();
    if slash_count != 1 {
        return None;
    }
    Some(owner_repo.to_owned())
}

/// Detect the merge strategy flag in a `gh pr merge` command string.
/// Returns `Some("squash" | "rebase" | "merge")` or `None`.
pub fn detect_merge_strategy(command: &str) -> Option<String> {
    // Check for flags as whole tokens in the command.
    // Order matters: check longest/most specific first.
    let tokens: Vec<&str> = command.split_whitespace().collect();
    for token in &tokens {
        match *token {
            "--squash" => return Some("squash".to_owned()),
            "--rebase" => return Some("rebase".to_owned()),
            "--merge" => return Some("merge".to_owned()),
            _ => {}
        }
    }
    None
}

/// Build the field list for a `pr.created` event from the parsed payload.
/// Returns the event-type string and a list of `(key, value)` string pairs
/// ready to be passed to `vsdd_hook_sdk::host::emit_event`.
pub fn build_pr_created_fields(
    pr_url: &str,
    pr_number: &str,
    pr_repo: &str,
    title: Option<&str>,
) -> (&'static str, Vec<(String, String)>) {
    let mut fields = Vec::new();
    fields.push(("pr_url".to_owned(), pr_url.to_owned()));
    fields.push(("pr_number".to_owned(), pr_number.to_owned()));
    fields.push(("pr_repo".to_owned(), pr_repo.to_owned()));
    if let Some(t) = title {
        fields.push(("title".to_owned(), t.to_owned()));
    }
    ("pr.created", fields)
}

/// Build the field list for a `pr.merged` event.
pub fn build_pr_merged_fields(
    pr_url: Option<&str>,
    pr_number: &str,
    pr_repo: Option<&str>,
    merge_strategy: Option<&str>,
) -> (&'static str, Vec<(String, String)>) {
    let mut fields = Vec::new();
    if let Some(url) = pr_url {
        fields.push(("pr_url".to_owned(), url.to_owned()));
    }
    fields.push(("pr_number".to_owned(), pr_number.to_owned()));
    if let Some(repo) = pr_repo {
        fields.push(("pr_repo".to_owned(), repo.to_owned()));
    }
    if let Some(strategy) = merge_strategy {
        fields.push(("merge_strategy".to_owned(), strategy.to_owned()));
    }
    ("pr.merged", fields)
}

/// Build the field list for a `pr.closed` event.
pub fn build_pr_closed_fields(
    pr_url: Option<&str>,
    pr_number: &str,
    pr_repo: Option<&str>,
) -> (&'static str, Vec<(String, String)>) {
    let mut fields = Vec::new();
    if let Some(url) = pr_url {
        fields.push(("pr_url".to_owned(), url.to_owned()));
    }
    fields.push(("pr_number".to_owned(), pr_number.to_owned()));
    if let Some(repo) = pr_repo {
        fields.push(("pr_repo".to_owned(), repo.to_owned()));
    }
    ("pr.closed", fields)
}

/// Build the field list for a `pr.create_failed` event (EC-001).
pub fn build_pr_create_failed_fields(command: &str) -> (&'static str, Vec<(String, String)>) {
    let fields = vec![("command".to_owned(), command.to_owned())];
    ("pr.create_failed", fields)
}

/// Top-level hook dispatch: drives the full PostToolUse/Bash pipeline.
///
/// Returns `Continue` for:
/// - Non-Bash tool events (no-op by design)
/// - Bash commands that do not contain a `gh pr` subcommand
/// - Any PR subcommand after emitting the appropriate event
/// - EC-001: `gh pr create` with no URL in stdout (emits `pr.create_failed`)
pub fn dispatch(payload: &HookPayload) -> HookResult {
    use vsdd_hook_sdk::host;

    // Non-Bash tools are a no-op.
    if payload.tool_name != "Bash" {
        return HookResult::Continue;
    }

    let command = match extract_command(payload) {
        Some(c) => c,
        None => return HookResult::Continue,
    };

    let subcommand = match detect_pr_subcommand(&command) {
        Some(s) => s,
        None => return HookResult::Continue,
    };

    // Extract stdout from tool_response for URL parsing.
    let stdout = payload
        .tool_response
        .as_ref()
        .and_then(|r| r.get("stdout"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_owned();

    // Check for interrupted / failed execution — treat as failure for creates.
    let interrupted = payload
        .tool_response
        .as_ref()
        .and_then(|r| r.get("interrupted"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    match subcommand {
        PrSubcommand::Create => {
            if interrupted {
                // EC-001: command was interrupted, emit create_failed.
                let (event_type, fields) = build_pr_create_failed_fields(&command);
                let field_refs: Vec<(&str, &str)> = fields
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .collect();
                host::emit_event(event_type, &field_refs);
                return HookResult::Continue;
            }

            // Extract PR URL from stdout (gh pr create prints the URL on success).
            let pr_url = extract_pr_url(&stdout);

            match pr_url {
                Some(ref url) => {
                    let pr_number = pr_number_from_url(url).unwrap_or_default();
                    let pr_repo = pr_repo_from_url(url).unwrap_or_default();
                    // Best-effort title extraction from --title "..." in command.
                    let title = extract_title_from_command(&command);
                    let (event_type, fields) =
                        build_pr_created_fields(url, &pr_number, &pr_repo, title.as_deref());
                    let field_refs: Vec<(&str, &str)> = fields
                        .iter()
                        .map(|(k, v)| (k.as_str(), v.as_str()))
                        .collect();
                    host::emit_event(event_type, &field_refs);
                }
                None => {
                    // EC-001: no URL in stdout means create failed or unusual output.
                    let (event_type, fields) = build_pr_create_failed_fields(&command);
                    let field_refs: Vec<(&str, &str)> = fields
                        .iter()
                        .map(|(k, v)| (k.as_str(), v.as_str()))
                        .collect();
                    host::emit_event(event_type, &field_refs);
                }
            }
        }

        PrSubcommand::Merge => {
            // Try stdout first, then command args for the URL.
            let pr_url = extract_pr_url(&stdout).or_else(|| extract_pr_url(&command));
            let merge_strategy = detect_merge_strategy(&command);

            let (pr_number, pr_repo) = match pr_url.as_deref() {
                Some(url) => (pr_number_from_url(url), pr_repo_from_url(url)),
                None => {
                    // Positional PR number form: `gh pr merge 42`.
                    let positional = extract_positional_pr_number(&command);
                    (positional, None)
                }
            };

            // If we can't determine a PR number, skip emitting.
            if let Some(num) = pr_number {
                let (event_type, fields) = build_pr_merged_fields(
                    pr_url.as_deref(),
                    &num,
                    pr_repo.as_deref(),
                    merge_strategy.as_deref(),
                );
                let field_refs: Vec<(&str, &str)> = fields
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .collect();
                host::emit_event(event_type, &field_refs);
            }
        }

        PrSubcommand::Close => {
            // Try stdout first, then command args for the URL.
            let pr_url = extract_pr_url(&stdout).or_else(|| extract_pr_url(&command));

            let (pr_number, pr_repo) = match pr_url.as_deref() {
                Some(url) => (pr_number_from_url(url), pr_repo_from_url(url)),
                None => {
                    // Positional PR number form: `gh pr close 42`.
                    let positional = extract_positional_pr_number_for_close(&command);
                    (positional, None)
                }
            };

            if let Some(num) = pr_number {
                let (event_type, fields) =
                    build_pr_closed_fields(pr_url.as_deref(), &num, pr_repo.as_deref());
                let field_refs: Vec<(&str, &str)> = fields
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .collect();
                host::emit_event(event_type, &field_refs);
            }
        }
    }

    HookResult::Continue
}

// ── Discriminant enum ────────────────────────────────────────────────────────

/// The three PR subcommands this plugin handles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrSubcommand {
    Create,
    Merge,
    Close,
}

// ── Private helpers ───────────────────────────────────────────────────────────

/// Try to parse a PR URL starting from `rest` (the portion of the string after
/// `https://github.com/`). Returns the full URL if valid.
fn try_parse_pr_url_from_rest(rest: &str, abs_start: usize, prefix: &str) -> Option<String> {
    // Must contain /pull/ followed by digits.
    let pull_marker = "/pull/";
    let pull_idx = rest.find(pull_marker)?;
    let after_pull = &rest[pull_idx + pull_marker.len()..];

    // There must be at least one digit.
    if !after_pull.starts_with(|c: char| c.is_ascii_digit()) {
        return None;
    }

    // Collect the numeric PR number.
    let pr_num_end = after_pull
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(after_pull.len());

    // Validate owner/repo portion (must have exactly one slash, no spaces).
    let owner_repo = &rest[..pull_idx];
    if owner_repo.contains(' ')
        || owner_repo.is_empty()
        || owner_repo.chars().filter(|&c| c == '/').count() != 1
    {
        return None;
    }
    // owner and repo must both be non-empty.
    let slash_pos = owner_repo.find('/')?;
    if slash_pos == 0 || slash_pos == owner_repo.len() - 1 {
        return None;
    }

    let url_len = prefix.len() + pull_idx + pull_marker.len() + pr_num_end;
    let full_url = &(prefix.to_owned() + &rest[..pull_idx + pull_marker.len() + pr_num_end]);
    let _ = abs_start; // used implicitly
    let _ = url_len;
    Some(full_url.clone())
}

/// Detect `gh pr <subcommand>` as a real shell invocation in `command`.
///
/// Rules:
/// - `gh` must be preceded by: start-of-string, `;`, `&`, `|`, or whitespace.
/// - Must NOT be inside a double-quoted string.
/// - Must use single-space separation between `gh`, `pr`, and the subcommand.
fn detect_gh_pr_in_command(command: &str) -> Option<PrSubcommand> {
    let chars: Vec<char> = command.chars().collect();
    let len = chars.len();
    let mut i = 0;
    let mut in_double_quote = false;

    while i < len {
        let ch = chars[i];

        // Track double-quote state (simple, no escape handling for our purposes).
        if ch == '"' {
            in_double_quote = !in_double_quote;
            i += 1;
            continue;
        }

        // Skip characters inside double quotes.
        if in_double_quote {
            i += 1;
            continue;
        }

        // Look for `gh` at a valid boundary.
        let at_boundary = i == 0 || matches!(chars[i - 1], ';' | '&' | '|' | ' ' | '\t' | '\n');

        if !at_boundary {
            i += 1;
            continue;
        }

        // Try to match `gh pr <subcommand>` starting at position i.
        let remaining = &command[command
            .char_indices()
            .nth(i)
            .map(|(pos, _)| pos)
            .unwrap_or(command.len())..];

        if let Some(sub) = try_match_gh_pr(remaining) {
            return Some(sub);
        }

        i += 1;
    }

    None
}

/// Try to match `gh pr create|merge|close` at the start of `s` using
/// exactly single-space separators.
fn try_match_gh_pr(s: &str) -> Option<PrSubcommand> {
    // Must start with `gh ` (exactly one space).
    let after_gh = s.strip_prefix("gh ")?;

    // Must continue with `pr ` (exactly one space) or `pr` at end-of-relevant-segment.
    let after_pr = after_gh.strip_prefix("pr ")?;

    // Now match the subcommand word (must be followed by end-of-string, space, or separator).
    for (word, sub) in &[
        ("create", PrSubcommand::Create),
        ("merge", PrSubcommand::Merge),
        ("close", PrSubcommand::Close),
    ] {
        if let Some(rest) = after_pr.strip_prefix(word) {
            // Subcommand must be followed by whitespace, end-of-string, or separator.
            if rest.is_empty()
                || rest.starts_with(|c: char| c.is_whitespace() || matches!(c, ';' | '&' | '|'))
            {
                return Some(*sub);
            }
        }
    }

    None
}

/// Extract `--title "..."` value from a command string.
fn extract_title_from_command(command: &str) -> Option<String> {
    let title_flag = "--title";
    let idx = command.find(title_flag)?;
    let after_flag = command[idx + title_flag.len()..].trim_start();

    if let Some(inner) = after_flag.strip_prefix('"') {
        // Quoted title.
        let end = inner.find('"')?;
        Some(inner[..end].to_owned())
    } else {
        // Unquoted: take until next whitespace.
        let word: String = after_flag
            .chars()
            .take_while(|c| !c.is_whitespace())
            .collect();
        if word.is_empty() { None } else { Some(word) }
    }
}

/// Extract a positional PR number from `gh pr merge <number>` form.
fn extract_positional_pr_number(command: &str) -> Option<String> {
    // Pattern: `gh pr merge <number>` where <number> is all digits.
    // Find `gh pr merge` and look at the next token.
    let pattern = "gh pr merge";
    let idx = command.find(pattern)?;
    let after = command[idx + pattern.len()..].trim_start();
    let number: String = after.chars().take_while(|c| c.is_ascii_digit()).collect();
    if number.is_empty() {
        None
    } else {
        Some(number)
    }
}

/// Extract a positional PR number from `gh pr close <number>` form.
fn extract_positional_pr_number_for_close(command: &str) -> Option<String> {
    let pattern = "gh pr close";
    let idx = command.find(pattern)?;
    let after = command[idx + pattern.len()..].trim_start();
    let number: String = after.chars().take_while(|c| c.is_ascii_digit()).collect();
    if number.is_empty() {
        None
    } else {
        Some(number)
    }
}
