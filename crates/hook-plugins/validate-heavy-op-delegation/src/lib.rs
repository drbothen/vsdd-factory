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
            // First match found. Compute preview ONCE (INV4: shared utility).
            let command_preview = truncate_command_preview(command);
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
pub fn build_recommendation_message(matched_pattern: &str, command_preview: &str) -> String {
    format!(
        "[DelegationRecommended] Heavy operation detected (matched: {:?}): {:?}\n\
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
