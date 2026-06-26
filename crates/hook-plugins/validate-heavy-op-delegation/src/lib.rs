//! validate-heavy-op-delegation — PreToolUse WASM hook plugin.
//!
//! Advisory gate that emits a `DelegationRecommended` advisory finding to
//! BOTH stderr (human/LLM-visible nudge) and the dispatcher's `plugin.log`
//! channel whenever a PreToolUse Bash command matches one of the configured
//! heavy-operation patterns (first-match semantics). The gate NEVER sets
//! `block_intent = true` — it always returns Continue under all conditions
//! including crash (BC-4.15.001 INV2).
//!
//! # Behavioral Contract
//!
//! BC-4.15.001 — validate-heavy-op-delegation WASM gate emits advisory
//! DelegationRecommended finding on PreToolUse Bash tool calls matching
//! heavy-operation patterns; never blocks; pure-parse pattern matching;
//! no filesystem or context access.
//!
//! # Invariants (BC-4.15.001)
//!
//! - INV1: Pure-parse; reads ONLY the `command` field from the PreToolUse
//!   payload. NO filesystem reads, NO subprocess invocation, NO context access.
//! - INV2: Never blocks; always returns Continue under ALL conditions (match,
//!   no-match, crash). block_intent is ALWAYS false.
//! - INV3: First-match; patterns evaluated in declaration order; stops at the
//!   first matching pattern; exactly ONE advisory emitted per invocation.
//! - INV4: command_preview ≤120-character truncation is invariant; applied
//!   identically to BOTH the stderr emission (PC-B-B1) and the plugin.log
//!   record (PC-B-B2).
//!
//! # Architecture compliance
//!
//! - NO `std::fs::`, `std::process::`, `std::net::` imports (INV1 / ADR-026 §D8).
//! - NO `regex` crate (Architecture Compliance Rule 1; WASM fuel budget).
//! - NO dependency on `crates/factory-lock-parse/` or `crates/context-resolvers/`
//!   (Architecture Compliance Rule 6).
//! - Pattern matching uses `str::contains()` (substring; case-sensitive; INV3).
//! - `#[deny(warnings)]` via workspace `[lints]` (`-- -D warnings` in CI).
//! - No `unwrap()` or `expect()` in non-test code paths.

use vsdd_hook_sdk::{HookPayload, HookResult};

/// HOST_ABI_VERSION declares the ABI contract version this plugin was built
/// against. Must remain 1.
pub const HOST_ABI_VERSION: u32 = 1;

/// Maximum command preview length in Unicode code points (BC-4.15.001 INV4).
pub const COMMAND_PREVIEW_MAX_CHARS: usize = 120;

/// Unicode ellipsis character appended when command exceeds COMMAND_PREVIEW_MAX_CHARS.
pub const ELLIPSIS: char = '\u{2026}';

// ---------------------------------------------------------------------------
// Pure gate types (testable without wasmtime)
// ---------------------------------------------------------------------------

/// Structured advisory emitted on a pattern match (BC-4.15.001 PC-B).
///
/// Carries all fields required by PC-B-B2 (plugin.log structured record) and
/// PC-B-B1 (stderr nudge message). Both channels use the same `command_preview`
/// value — truncated identically per INV4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelegationAdvisory {
    /// The exact first-matching pattern string from the configured list (INV3).
    pub matched_pattern: String,
    /// First ≤120 Unicode code points of the command string, followed by U+2026
    /// if truncated (INV4).
    pub command_preview: String,
    /// Human-readable delegation recommendation message (PC-B-B2 `message` field).
    pub message: String,
}

/// Result of the pure gate evaluation.
///
/// Mirrors the HookResult vocabulary but is decoupled from the SDK type so
/// the pure evaluation function can be tested without linking vsdd-hook-sdk.
///
/// The gate ALWAYS returns Continue or Advisory(…) — never Block. A Block
/// variant is intentionally absent (BC-4.15.001 INV2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GateResult {
    /// No pattern matched; no advisory emitted. Gate returns Continue (PC-A).
    Continue,
    /// First-match found; advisory emitted to both channels. Gate returns
    /// Continue with advisory fields populated (PC-B).
    Advisory(DelegationAdvisory),
}

// ---------------------------------------------------------------------------
// Pure evaluation function (BC-4.15.001 INV1/INV2/INV3/INV4)
// ---------------------------------------------------------------------------

/// Evaluate a Bash command string against the configured pattern list.
///
/// This is the pure-parse core of the gate. It reads ONLY the `command`
/// string and the `patterns` list — no filesystem access, no subprocess
/// invocation, no context-window reads (INV1).
///
/// Pattern matching is substring containment (case-sensitive; INV3):
/// a pattern P matches command C if `C.contains(P)`. The loop stops at the
/// first matching pattern (first-match; INV3). Exactly one advisory is emitted
/// per invocation regardless of how many patterns would match.
///
/// The `command_preview` is computed once and used in both the returned
/// `DelegationAdvisory` fields and the WASM-facing entry point — ensuring
/// channel-identical truncation (INV4).
///
/// Returns `GateResult::Continue` when no pattern matches (PC-A).
/// Returns `GateResult::Advisory(advisory)` when the first match is found (PC-B).
pub fn evaluate_patterns(command: &str, patterns: &[&str]) -> GateResult {
    // INV3: iterate patterns in declaration order; stop at first match.
    for pattern in patterns {
        if command.contains(*pattern) {
            // First match found. Apply INV5 4-pass redaction then INV4 truncation.
            // redact-then-truncate ordering: BC-4.15.001 INV5 / SEC-002.
            let redacted = redact_command_preview(command);
            let command_preview = truncate_command_preview(&redacted);
            let message = build_recommendation_message(pattern, &command_preview);
            return GateResult::Advisory(DelegationAdvisory {
                matched_pattern: pattern.to_string(),
                command_preview,
                message,
            });
        }
    }
    // No match: PC-A — Continue with no emission.
    GateResult::Continue
}

/// Apply 4-pass secret redaction to a Bash command string (BC-4.15.001 INV5 / SEC-002).
///
/// MUST be called BEFORE `truncate_command_preview` so that the 120-char
/// truncation window shows redacted content, not raw secrets (redact-then-truncate
/// ordering; BC-4.15.001 EC-021).
///
/// All four passes are applied in sequence. Each pass may produce a longer or
/// shorter string than its input; the output of each pass is the input of the
/// next. Pure std string operations only — no regex, no new dependencies (INV1).
///
/// ## Pass 1 — Flag-argument secrets
///
/// Tokenizes on ASCII whitespace. For each `--flag` token whose flag name
/// (lowercased, stripped of leading dashes, pre-`=` if `=`-separated) appears
/// in the sensitive-flag keyword list, the following value token is replaced
/// with `***REDACTED***` (two-token `--flag value` form) or the post-`=`
/// portion is replaced with `***REDACTED***` (single-token `--flag=value` form).
/// Short single-dash options (e.g., `-p`) are not matched. Bare flags with no
/// following value token are not redacted.
///
/// ## Pass 2 — Environment-variable assignment prefixes
///
/// Scans whitespace-separated tokens that appear before the first non-`KEY=value`
/// token. For each `IDENT=value` token where `IDENT` (uppercased) is NOT on the
/// allowlist and contains one of the sensitive env-var keywords, the value portion
/// is replaced with `***REDACTED***`.
///
/// Allowlist (never redacted): `SSH_AUTH_SOCK`, `SSH_ASKPASS`, tokens whose
/// uppercase IDENT ends with `_SERVICE_HOST`.
///
/// ## Pass 3 — Authorization / Cookie header values
///
/// Scans the raw string (character-level) for case-insensitive occurrences of
/// `authorization:`, `cookie:`, or `set-cookie:`. Replaces the portion from
/// the `:` (exclusive of the header name itself) onward to the next whitespace,
/// `"`, `'`, or end-of-string with `***REDACTED***`. Any leading `"` or `'`
/// before the header keyword is stripped from the output token.
///
/// ## Pass 4 — URL inline credentials
///
/// Scans whitespace-separated tokens for tokens containing `://`. For each such
/// token, if a `@` character appears after the `://` prefix and before the next
/// `/` (or end-of-token), the user-info substring (`://…@`) is replaced with
/// `://***REDACTED***@`.
pub fn redact_command_preview(command: &str) -> String {
    let s = redact_pass1_flag_args(command);
    let s = redact_pass2_env_assignments(&s);
    let s = redact_pass3_auth_headers(&s);
    redact_pass4_url_credentials(&s)
}

/// Pass 1: flag-argument secret redaction (BC-4.15.001 INV5 Pass 1).
///
/// For `--flag value` form: the ENTIRE value token is replaced with
/// `***REDACTED***` (full-value masking; BC-4.15.001 INV5).
///
/// For `--flag=value` form: the entire value portion (after `=`) is replaced
/// with `***REDACTED***`.
fn redact_pass1_flag_args(command: &str) -> String {
    /// Sensitive flag keywords (lowercase, dashes normalised to hyphens).
    const SENSITIVE_FLAGS: &[&str] = &[
        "password",
        "passwd",
        "token",
        "secret",
        "api-key",
        "apikey",
        "api_key",
        "client-secret",
        "auth-token",
        "access-token",
        "access-key",
        "secret-key",
        "credential",
        "passphrase",
        "private-key",
    ];

    // Collect whitespace-separated tokens.
    let tokens: Vec<&str> = command.split_whitespace().collect();
    if tokens.is_empty() {
        return command.to_string();
    }

    let mut result_tokens: Vec<String> = Vec::with_capacity(tokens.len());
    let mut redact_next = false;

    for (i, &tok) in tokens.iter().enumerate() {
        if redact_next {
            // This token is the entire value after a `--sensitive-flag`. Replace the
            // whole token with `***REDACTED***` (full-value masking; BC-4.15.001 INV5).
            result_tokens.push("***REDACTED***".to_string());
            redact_next = false;
            continue;
        }

        // Must start with `--` (long option). Single-dash short options are excluded.
        if !tok.starts_with("--") {
            result_tokens.push(tok.to_string());
            continue;
        }

        // Strip the leading `--`.
        let flag_body = &tok[2..];

        if let Some(eq_pos) = flag_body.find('=') {
            // `--flag=value` form: extract flag name (pre-`=`) and value (post-`=`).
            let flag_name = &flag_body[..eq_pos];
            let normalised = flag_name.to_lowercase().replace('_', "-");
            if SENSITIVE_FLAGS.contains(&normalised.as_str()) {
                // Replace the entire post-`=` value with `***REDACTED***`.
                result_tokens.push(format!("--{}=***REDACTED***", flag_name));
            } else {
                result_tokens.push(tok.to_string());
            }
        } else {
            // `--flag` form (no `=`). Check if flag name is sensitive.
            let normalised = flag_body.to_lowercase().replace('_', "-");
            if SENSITIVE_FLAGS.contains(&normalised.as_str()) && i + 1 < tokens.len() {
                // Keep `--flag` as-is; mark the next token for full replacement.
                result_tokens.push(tok.to_string());
                redact_next = true;
            } else {
                result_tokens.push(tok.to_string());
            }
        }
    }

    result_tokens.join(" ")
}

/// Pass 2: environment-variable assignment prefix redaction (BC-4.15.001 INV5 Pass 2).
fn redact_pass2_env_assignments(command: &str) -> String {
    /// Sensitive env-var IDENT substrings (uppercase).
    /// Note: bare `KEY` is intentionally excluded (EC-020 guard: `--key` bare flag).
    const SENSITIVE_ENV: &[&str] = &[
        "PASSWORD",
        "PASSWD",
        "SECRET",
        "TOKEN",
        "APIKEY",
        "API_KEY",
        "ACCESS_KEY",
        "PRIVATE_KEY",
        "AUTH_TOKEN",
        "CLIENT_SECRET",
        "CREDENTIAL",
        "PASSPHRASE",
    ];

    /// Allowlisted IDENT prefixes/exact matches that are NEVER redacted.
    const ENV_ALLOWLIST: &[&str] = &["SSH_AUTH_SOCK", "SSH_ASKPASS"];

    let tokens: Vec<&str> = command.split_whitespace().collect();
    if tokens.is_empty() {
        return command.to_string();
    }

    let mut result_tokens: Vec<String> = Vec::with_capacity(tokens.len());
    let mut in_env_prefix = true; // ENV=value tokens only at the start of the command.

    for &tok in &tokens {
        if !in_env_prefix {
            result_tokens.push(tok.to_string());
            continue;
        }

        // `IDENT=value` form: IDENT must be all-uppercase or mixed, contains `=`.
        if let Some(eq_pos) = tok.find('=') {
            let ident = &tok[..eq_pos];
            let value = &tok[eq_pos + 1..]; // Everything after the `=`.

            // IDENT must look like an env var: only uppercase ASCII + digits + underscore.
            let looks_like_env = ident
                .chars()
                .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_');

            if !looks_like_env || ident.is_empty() {
                // Not an env-var prefix token; stop scanning for env-var prefixes.
                in_env_prefix = false;
                result_tokens.push(tok.to_string());
                continue;
            }

            let upper_ident = ident.to_uppercase();

            // Allowlist check.
            let is_allowlisted = ENV_ALLOWLIST.contains(&upper_ident.as_str())
                || upper_ident.ends_with("_SERVICE_HOST");

            if is_allowlisted {
                result_tokens.push(tok.to_string());
                continue;
            }

            // Check if IDENT contains a sensitive keyword.
            let is_sensitive = SENSITIVE_ENV.iter().any(|kw| upper_ident.contains(kw));

            if is_sensitive && !value.is_empty() {
                result_tokens.push(format!("{}=***REDACTED***", ident));
            } else {
                result_tokens.push(tok.to_string());
            }
        } else {
            // Not a `KEY=value` token; stop the env-prefix scan.
            in_env_prefix = false;
            result_tokens.push(tok.to_string());
        }
    }

    result_tokens.join(" ")
}

/// Pass 3: authorization/cookie header value redaction (BC-4.15.001 INV5 Pass 3).
///
/// Operates on the raw string (character-level) to handle cases where the
/// header and value may appear inside quoted strings in the Bash command.
///
/// Header prefixes matched (case-insensitive): `authorization:`, `cookie:`,
/// `set-cookie:`.
///
/// The redacted output replaces everything from the `:` after the header name
/// onward to the next unescaped `"`, `'`, or end-of-string with `***REDACTED***`.
/// Any leading `"` or `'` quote character immediately before the header keyword
/// is stripped.
fn redact_pass3_auth_headers(command: &str) -> String {
    const HEADER_PREFIXES: &[&str] = &["authorization:", "cookie:", "set-cookie:"];

    // Work token-by-token on whitespace, preserving join.
    let tokens: Vec<&str> = command.split_whitespace().collect();
    if tokens.is_empty() {
        return command.to_string();
    }

    let mut result_tokens: Vec<String> = Vec::with_capacity(tokens.len());
    let mut skip_until_end_of_value = false;

    for &tok in &tokens {
        if skip_until_end_of_value {
            // This token is a continuation of the header value (e.g., `Bearer token`).
            // If it ends with `"` or `'`, stop consuming after this one.
            if tok.ends_with('"') || tok.ends_with('\'') {
                skip_until_end_of_value = false;
            }
            // Do not push this token — it is part of the redacted header value.
            continue;
        }

        // Strip leading quote character for comparison.
        let stripped = tok.trim_start_matches(['"', '\'']);
        let lower = stripped.to_lowercase();

        let matched_prefix = HEADER_PREFIXES.iter().find(|&&pfx| lower.starts_with(pfx));

        if let Some(&pfx) = matched_prefix {
            // Determine the header name portion (before the `:`, without leading quote).
            let colon_pos = pfx.len() - 1; // position of `:` in `pfx`
            // The header name in original case from `stripped`.
            let header_name = &stripped[..colon_pos];

            // The value portion within this same token (after the `:`).
            let after_colon = &stripped[pfx.len()..];

            if after_colon.trim_end_matches(['"', '\'']).is_empty() || after_colon.is_empty() {
                // Value is in the following token(s).
                result_tokens.push(format!("{}:***REDACTED***", header_name));
                skip_until_end_of_value = true;
            } else {
                // Inline value: `Authorization: Bearer token` all in one token would be
                // unusual but possible. Replace entire value.
                result_tokens.push(format!("{}:***REDACTED***", header_name));
                // If a closing quote is embedded in the inline value, no need to skip next.
            }
        } else {
            result_tokens.push(tok.to_string());
        }
    }

    result_tokens.join(" ")
}

/// Pass 4: URL inline credential redaction (BC-4.15.001 INV5 Pass 4).
///
/// Scans whitespace-separated tokens for tokens containing `://`. For each
/// such token, replaces the user-info portion (`user:pass@`) with
/// `***REDACTED***@`, yielding `scheme://***REDACTED***@host/path`.
fn redact_pass4_url_credentials(command: &str) -> String {
    let tokens: Vec<&str> = command.split_whitespace().collect();
    if tokens.is_empty() {
        return command.to_string();
    }

    let result_tokens: Vec<String> = tokens
        .iter()
        .map(|&tok| {
            if let Some(scheme_end) = tok.find("://") {
                let after_scheme = &tok[scheme_end + 3..]; // skip `://`
                // Find `@` in the remaining part (before the next `/`).
                let path_start = after_scheme.find('/').unwrap_or(after_scheme.len());
                let host_part = &after_scheme[..path_start];

                if let Some(at_pos) = host_part.rfind('@') {
                    // userinfo is host_part[..at_pos], e.g., `user:pass`.
                    let scheme_and_sep = &tok[..scheme_end + 3]; // e.g., `https://`
                    let after_at = &after_scheme[at_pos + 1..]; // e.g., `example.com/db`
                    format!("{}***REDACTED***@{}", scheme_and_sep, after_at)
                } else {
                    tok.to_string()
                }
            } else {
                tok.to_string()
            }
        })
        .collect();

    result_tokens.join(" ")
}

/// Compute the command_preview field (BC-4.15.001 INV4).
///
/// If `command` is ≤ `COMMAND_PREVIEW_MAX_CHARS` Unicode code points, returns
/// the full command string unchanged.
///
/// If `command` exceeds `COMMAND_PREVIEW_MAX_CHARS` code points, returns the
/// first `COMMAND_PREVIEW_MAX_CHARS` code points followed by `ELLIPSIS`
/// (U+2026), yielding a (COMMAND_PREVIEW_MAX_CHARS + 1)-code-point string.
///
/// This function is the SINGLE truncation implementation used by BOTH
/// the stderr emission (PC-B-B1) and the plugin.log record (PC-B-B2).
/// Having a single implementation prevents channel-divergence drift (INV4 /
/// Architecture Compliance Rule 4).
pub fn truncate_command_preview(command: &str) -> String {
    let char_count = command.chars().count();
    if char_count <= COMMAND_PREVIEW_MAX_CHARS {
        command.to_string()
    } else {
        let truncated: String = command.chars().take(COMMAND_PREVIEW_MAX_CHARS).collect();
        format!("{}{}", truncated, ELLIPSIS)
    }
}

/// Build the human-readable delegation recommendation message.
///
/// Used in both the stderr nudge (PC-B-B1) and the plugin.log `message`
/// field (PC-B-B2). The message conveys:
/// - That the command is a heavy operation.
/// - The matched pattern string.
/// - The command_preview (already truncated by `truncate_command_preview`).
/// - The recommendation to delegate to a sub-agent or worktree.
///
/// ## Channel-identity invariant (AC-006)
///
/// `command_preview` is rendered with `{}` (Display; raw bytes) so that both
/// the stderr nudge (PC-B-B1) and the plugin.log `message` field (PC-B-B2)
/// carry the byte-identical preview string. Using `{:?}` (Debug) would
/// add surrounding quotes and escape backslashes/special chars, producing a
/// stderr preview that diverges from the plugin.log `command_preview` field,
/// which is a spec violation (AC-006 requires identical previews across channels).
/// `matched_pattern` continues to use `{:?}` so it is clearly delimited from
/// surrounding prose.
pub fn build_recommendation_message(matched_pattern: &str, command_preview: &str) -> String {
    // command_preview: `{}` (Display) — byte-identical in both emission channels (AC-006).
    // matched_pattern: `{:?}` (Debug) — quoted for readability in the nudge message.
    format!(
        "[DelegationRecommended] Heavy operation detected (matched: {:?}): {}\n\
         Consider delegating this operation to a sub-agent or background worktree to reduce \
         context-window pressure and prevent uncoordinated auto-compaction events (ADR-026 §Decision 12).",
        matched_pattern, command_preview
    )
}

// ---------------------------------------------------------------------------
// WASM-facing gate function (PreToolUse dispatcher integration)
// ---------------------------------------------------------------------------

/// PreToolUse hook entry point: parse the dispatcher payload and invoke
/// the pure pattern-matching evaluation.
///
/// Extracts the `command` field from `payload.tool_input["command"]` (Bash
/// tool call). If the field is absent or not a string, the gate returns
/// Continue immediately (fail-open; no emission).
///
/// Reads the pattern list from `payload.plugin_config["patterns"]` (injected
/// by the dispatcher from the `[hooks.config]` registry table per AC-008):
/// - If `plugin_config["patterns"]` is present (including an explicit empty
///   array `[]`), it is used AS-IS. An empty array means "match nothing"
///   (BC-4.15.001 EC-012 / AC-011).
/// - Falls back to `DEFAULT_PATTERNS` ONLY when the `patterns` key is absent
///   entirely from `plugin_config` (BC-4.15.001 PC1 default-patterns fallback).
///
/// On a match, emits:
/// - PC-B-B1: stderr nudge via `eprintln!` (writes directly to dispatcher stderr).
/// - PC-B-B2: plugin.log structured record via `vsdd_hook_sdk::host::emit_event`.
///
/// ALWAYS returns `HookResult::Continue` (INV2 — never blocks).
///
/// ## Fail-open guarantee (BC-4.15.001 PC-C)
///
/// The registry sets `on_error = "continue"` so any WASM panic causes the
/// dispatcher to fail open (Continue). This function itself must never panic
/// in non-test code — all code paths return `HookResult::Continue`.
pub fn on_pre_tool_use(payload: HookPayload) -> HookResult {
    use vsdd_hook_sdk::host;

    // Extract the command field from the Bash PreToolUse payload (INV1).
    // If absent or not a string: fail-open Continue (no emission).
    let command = match payload.tool_input.get("command").and_then(|v| v.as_str()) {
        Some(cmd) => cmd,
        None => return HookResult::Continue,
    };

    // Read the pattern list from plugin_config["patterns"] (BC-4.15.001 PC1).
    //
    // The dispatcher injects [hooks.config] patterns into plugin_config before
    // dispatching (crates/factory-dispatcher/src/executor.rs::build_plugin_config).
    // Semantics (BC-4.15.001 EC-012 + EC-013):
    //   - Key present (including explicit []): use it as-is ([] means "match nothing").
    //   - Key absent: fall back to DEFAULT_PATTERNS (v1 default set).
    //
    // When the key is present, an owned Vec<String> is built from the JSON array
    // and a Vec<&str> view is derived for the &[&str] slice required by
    // evaluate_patterns. Non-string elements are silently skipped (fail-open on
    // malformed config — INV2 never panics, never blocks).
    let gate_result = if let Some(arr) = payload
        .plugin_config
        .get("patterns")
        .and_then(|v| v.as_array())
    {
        let owned: Vec<String> = arr
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_owned()))
            .collect();
        let refs: Vec<&str> = owned.iter().map(|s| s.as_str()).collect();
        evaluate_patterns(command, &refs)
    } else {
        // Key absent: use the v1 default set (BC-4.15.001 PC1 default fallback).
        evaluate_patterns(command, DEFAULT_PATTERNS)
    };

    if let GateResult::Advisory(ref advisory) = gate_result {
        // PC-B-B1: emit stderr nudge (human/LLM-visible; not stdout).
        // eprintln! writes to the WASM module's stderr, which flows through
        // to the dispatcher process stderr.
        eprintln!("{}", advisory.message);

        // PC-B-B2: emit structured plugin.log record to dispatcher internal log.
        // The dispatcher captures emit_event("plugin.log", ...) calls and writes
        // them to the JSONL internal log with the provided fields.
        host::emit_event(
            "plugin.log",
            &[
                ("level", "warn"),
                ("code", "DelegationRecommended"),
                ("matched_pattern", &advisory.matched_pattern),
                ("command_preview", &advisory.command_preview),
                ("message", &advisory.message),
            ],
        );
    }

    // INV2: ALWAYS return Continue. Never block.
    HookResult::Continue
}

// ---------------------------------------------------------------------------
// V1 default pattern list (BC-4.15.001 PC1 / AC-008)
// ---------------------------------------------------------------------------

/// V1 default heavy-operation patterns (BC-4.15.001 PC1 / hooks-registry.toml
/// [hooks.config] patterns = [...]).
///
/// These are the seven patterns in the canonical `[hooks.config]` block.
/// The implementer wires the registry-provided config into `on_pre_tool_use`
/// at runtime; this constant serves as the fallback / test default.
pub const DEFAULT_PATTERNS: &[&str] = &[
    "cargo test --release",
    "grep -r",
    "grep -R",
    "find . -name",
    "find . -type",
    "./run-all.sh",
    "./run-bats.sh",
];
