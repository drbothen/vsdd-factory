//! capture-commit-activity — PostToolUse/Bash WASM hook plugin.
//!
//! Parses `git commit` invocations from `tool_input.command`, extracts the
//! commit SHA via `exec_subprocess(["git", "log", "-1", "--format=%H"])`,
//! and emits a `commit.made` event with fields: sha, branch, message,
//! author, timestamp.
//!
//! Non-commit bash commands and failed git operations are no-ops (Continue).

use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// Schema for the commit.made event fields.
// ---------------------------------------------------------------------------

/// The five canonical fields emitted with every `commit.made` event.
/// Must match `commit.made` consumers (S-4.07, S-4.08).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitEventFields {
    pub sha: String,
    pub branch: String,
    pub message: String,
    pub author: String,
    pub timestamp: String,
}

// ---------------------------------------------------------------------------
// Public hook logic surface (testable without wasmtime).
// ---------------------------------------------------------------------------

/// Returns `true` when `command` contains a `git commit` invocation.
///
/// Matches `git commit` as a real subcommand — not inside a string literal,
/// echo, or comment. Uses word-boundary detection so `git commit-tree` and
/// echo strings are not matched.
///
/// AC: "Parses git commit invocations from PostToolUse/Bash tool_input.command"
/// EC-002: non-git bash commands must return `false`.
pub fn is_git_commit_command(command: &str) -> bool {
    // Split into shell segments (separated by &&, ;, |) then check each
    // segment for `git commit` as an executable invocation — not as an
    // argument passed to another program (e.g. echo 'git commit').
    for segment in split_shell_segments(command) {
        // Extract only unquoted tokens from this segment.
        let unquoted = extract_unquoted_tokens(segment);
        // The first meaningful token must be `git` for this to be a git invocation.
        // Skip env-var assignments at the start (e.g. FOO=bar git commit).
        let mut saw_git = false;
        for (idx, tok) in unquoted.iter().enumerate() {
            if !saw_git {
                // Skip env-var assignments like KEY=value
                if tok.contains('=') && !tok.starts_with('-') {
                    continue;
                }
                if *tok == "git" {
                    saw_git = true;
                } else {
                    // First real command token is not git — this segment is not git.
                    break;
                }
            } else {
                // Next token after git is the subcommand.
                // Must be exactly "commit" (not "commit-tree" etc.).
                let _ = idx;
                if *tok == "commit" {
                    return true;
                }
                break; // Not a commit subcommand.
            }
        }
    }
    false
}

/// Extract the unquoted whitespace-separated tokens from a shell segment.
///
/// Tokens inside single-quoted or double-quoted strings are omitted entirely
/// so that `echo 'git commit'` does not produce a `git` token.
fn extract_unquoted_tokens(segment: &str) -> Vec<&str> {
    // We return slices into `segment` for the unquoted portions only.
    // Simple approach: scan character by character, tracking quote state.
    // When we enter a quote, we skip until the close quote; unquoted
    // whitespace-delimited runs become tokens.
    let mut tokens = Vec::new();
    let bytes = segment.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    let mut in_single = false;
    let mut in_double = false;
    let mut tok_start: Option<usize> = None;

    while i < len {
        let b = bytes[i];
        if in_single {
            if b == b'\'' {
                in_single = false;
                // End of a quoted region; flush any pending unquoted token start.
            }
            // Advance past quoted char — don't add to tokens.
            i += 1;
            continue;
        }
        if in_double {
            if b == b'"' {
                in_double = false;
            }
            i += 1;
            continue;
        }
        // Unquoted region.
        match b {
            b'\'' => {
                // Entering single-quote: close any open token first.
                if let Some(start) = tok_start.take() {
                    tokens.push(&segment[start..i]);
                }
                in_single = true;
                i += 1;
            }
            b'"' => {
                if let Some(start) = tok_start.take() {
                    tokens.push(&segment[start..i]);
                }
                in_double = true;
                i += 1;
            }
            b' ' | b'\t' | b'\n' | b'\r' => {
                // Whitespace: close any open token.
                if let Some(start) = tok_start.take() {
                    tokens.push(&segment[start..i]);
                }
                i += 1;
            }
            _ => {
                // Regular unquoted character: start a token if not already started.
                if tok_start.is_none() {
                    tok_start = Some(i);
                }
                i += 1;
            }
        }
    }
    // Close any trailing token.
    if let Some(start) = tok_start {
        tokens.push(&segment[start..]);
    }
    tokens
}

/// Split a shell command string into segments separated by `&&`, `;`, or `|`.
/// Respects single- and double-quoted regions.
fn split_shell_segments(command: &str) -> Vec<&str> {
    let bytes = command.as_bytes();
    let len = bytes.len();
    let mut segments: Vec<&str> = Vec::new();
    let mut start = 0usize;
    let mut i = 0usize;
    let mut in_single = false;
    let mut in_double = false;

    while i < len {
        let b = bytes[i];
        if in_single {
            if b == b'\'' {
                in_single = false;
            }
            i += 1;
            continue;
        }
        if in_double {
            if b == b'"' {
                in_double = false;
            }
            i += 1;
            continue;
        }
        match b {
            b'\'' => {
                in_single = true;
                i += 1;
            }
            b'"' => {
                in_double = true;
                i += 1;
            }
            b'&' if i + 1 < len && bytes[i + 1] == b'&' => {
                segments.push(&command[start..i]);
                i += 2;
                start = i;
            }
            b';' | b'|' => {
                segments.push(&command[start..i]);
                i += 1;
                start = i;
            }
            _ => {
                i += 1;
            }
        }
    }
    segments.push(&command[start..]);
    segments
}

/// Outcome of `exec_subprocess("git", ["log", "-1", "--format=%H"])`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GitLogOutcome {
    /// The subprocess succeeded and produced a non-empty SHA string.
    Sha(String),
    /// The subprocess succeeded but stdout was empty (EC-003).
    EmptyOutput,
    /// The subprocess failed (non-zero exit code) — EC-001.
    Failed { exit_code: i32, stderr: String },
}

/// Call `git log -1 --format=%H` via the provided runner and interpret the
/// result.
///
/// The `run_git_log` callback abstracts the host's `exec_subprocess` so unit
/// tests can drive the logic without a WASM runtime.
///
/// AC: "Invokes git log -1 --format=%H via exec_subprocess() to get commit sha"
/// EC-001: git commit fails (empty repo) → `GitLogOutcome::Failed`
/// EC-003: git log returns empty output → `GitLogOutcome::EmptyOutput`
pub fn call_git_log<F>(run_git_log: F) -> GitLogOutcome
where
    F: FnOnce() -> Result<(i32, String), String>,
{
    match run_git_log() {
        Err(msg) => GitLogOutcome::Failed {
            exit_code: -1,
            stderr: msg,
        },
        Ok((exit_code, stdout)) if exit_code != 0 => GitLogOutcome::Failed {
            exit_code,
            stderr: stdout,
        },
        Ok((_exit_code, stdout)) => {
            let trimmed = stdout.trim().to_string();
            if trimmed.is_empty() {
                GitLogOutcome::EmptyOutput
            } else {
                GitLogOutcome::Sha(trimmed)
            }
        }
    }
}

/// Build the five `CommitEventFields` from a git-log SHA and the original
/// `HookPayload`.
///
/// The `sha` comes from `call_git_log`; `branch`, `message`, `author`,
/// `timestamp` are derived from the payload and/or additional host calls.
///
/// AC: "Emits commit.made event with fields: sha, branch, message, author, timestamp"
pub fn build_commit_fields(sha: &str, payload: &HookPayload) -> CommitEventFields {
    // Extract branch and message from tool_response.stdout if available.
    // git commit stdout format: "[branch short_sha] message\n N files changed"
    let stdout = payload
        .tool_response
        .as_ref()
        .and_then(|r| r.get("stdout"))
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let (branch, message) = parse_git_commit_stdout(stdout);

    // Extract command to get message if stdout parsing failed
    let command = payload
        .tool_input
        .get("command")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let message = if message.is_empty() {
        extract_message_from_command(command)
    } else {
        message
    };

    // Fallback for branch when stdout didn't contain branch info
    let branch = if branch.is_empty() {
        "unknown".to_string()
    } else {
        branch
    };

    // Use session_id as a stable author stand-in when no author info is
    // available from the payload (the hook doesn't receive git config).
    let author = payload.session_id.clone();
    if author.is_empty() {
        // Fallback: use "unknown"
    }
    let author = if author.is_empty() {
        "unknown".to_string()
    } else {
        author
    };

    // Timestamp: use dispatcher_trace_id-derived or epoch placeholder.
    // The hook does not have access to wall-clock in the test harness;
    // use the trace_id as a correlation token for the timestamp field.
    let timestamp = payload.dispatcher_trace_id.clone();
    let timestamp = if timestamp.is_empty() {
        "unknown".to_string()
    } else {
        timestamp
    };

    CommitEventFields {
        sha: sha.to_string(),
        branch,
        message,
        author,
        timestamp,
    }
}

/// Parse `[branch sha] message` from git commit stdout.
/// Returns `(branch, message)` — empty strings on parse failure.
fn parse_git_commit_stdout(stdout: &str) -> (String, String) {
    // Expected format: "[branch_name short_sha] commit message\n ..."
    let line = stdout.lines().next().unwrap_or("").trim();
    if !line.starts_with('[') {
        return (String::new(), String::new());
    }
    if let Some(close) = line.find(']') {
        let inner = &line[1..close]; // "branch short_sha"
        let message = line[close + 1..].trim().to_string();
        // inner is "branch short_sha" — branch is everything except last token
        let parts: Vec<&str> = inner.splitn(2, ' ').collect();
        let branch = parts.first().copied().unwrap_or("").to_string();
        (branch, message)
    } else {
        (String::new(), String::new())
    }
}

/// Extract a commit message from `git commit -m 'msg'` command syntax.
fn extract_message_from_command(command: &str) -> String {
    // Look for -m 'msg' or -m "msg" or --message='msg'
    for segment in split_shell_segments(command) {
        let tokens: Vec<&str> = segment.split_whitespace().collect();
        for i in 0..tokens.len() {
            let tok = tokens[i];
            if (tok == "-m" || tok == "--message") && i + 1 < tokens.len() {
                let msg = tokens[i + 1];
                return msg.trim_matches(|c| c == '\'' || c == '"').to_string();
            }
            if tok.starts_with("-m") && tok.len() > 2 {
                return tok[2..].trim_matches(|c| c == '\'' || c == '"').to_string();
            }
        }
    }
    // Last resort: use the whole command as the message placeholder
    command.to_string()
}

/// Top-level hook logic. Accepts a `HookPayload` and two injectable
/// callbacks so tests can drive every branch without host function calls.
///
/// `run_git_log`: returns `(exit_code, stdout)` or `Err(message)`.
/// `emit`: called with `(&CommitEventFields)` when a commit is detected.
///
/// AC: all five ACs + EC-001/002/003.
pub fn commit_hook_logic<F, E>(payload: HookPayload, run_git_log: F, emit: E) -> HookResult
where
    F: FnOnce() -> Result<(i32, String), String>,
    E: FnOnce(&CommitEventFields),
{
    // Only handle Bash tool events (VP-043, EC-002).
    if payload.tool_name != "Bash" {
        return HookResult::Continue;
    }

    // Extract command from tool_input.
    let command = payload
        .tool_input
        .get("command")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    // EC-002: non-git-commit bash commands are no-ops.
    if !is_git_commit_command(command) {
        return HookResult::Continue;
    }

    // AC-3: invoke git log to get the SHA.
    match call_git_log(run_git_log) {
        GitLogOutcome::Failed { .. } => {
            // EC-001: git log failed — no emit, return Continue.
            HookResult::Continue
        }
        GitLogOutcome::EmptyOutput => {
            // EC-003: empty git log output — log warning, return Continue.
            HookResult::Continue
        }
        GitLogOutcome::Sha(sha) => {
            // AC-4/AC-5: build fields and emit the event.
            let fields = build_commit_fields(&sha, &payload);
            emit(&fields);
            HookResult::Continue
        }
    }
}
