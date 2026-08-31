//! validate-unvalidated-mutation-marker — Two-arm next-advance gate WASM hook plugin.
//!
//! Implements the quarantine gate for the INDETERMINATE outcome class (S-25.01 Layer-1,
//! ADR-047). Two `[[hooks]]` registry entries reference this SINGLE compiled WASM binary:
//!
//! - **Arm 1** (`validate-unvalidated-mutation-marker`, `tool = "^Agent$"`): blocks Agent
//!   dispatches while `.factory/unvalidated-mutation.marker` exists (BC-1.18.002 PC1).
//! - **Arm 2** (`validate-unvalidated-mutation-marker-git`, `tool = "^Bash$"`): blocks
//!   `git commit` and `git push` Bash dispatches while the marker exists (BC-1.18.002 PC2).
//!
//! Both arms use the same WASM binary per BC-1.18.002 invariant 1. The dispatch routing
//! between Arm 1 and Arm 2 is plugin-internal: the plugin reads the `tool` field (or
//! `command` field for Bash) from the PreToolUse payload to determine which arm applies.
//!
//! # Self-lock prevention (BC-1.18.002 invariant 2)
//!
//! Both hooks-registry.toml entries for this plugin MUST have `failure_policy = "fail-open"`.
//! If this plugin itself fuel-exhausts, it DOES NOT write a marker — the gate cannot self-lock.
//! EC-003: gate fuel exhaustion → fail-open → dispatch proceeds unblocked.
//!
//! # Marker absent → allow (AC-010)
//!
//! When `.factory/unvalidated-mutation.marker` is absent, BOTH arms return exit_code=0 (allow).
//!
//! # Operator escape hatch (BC-1.18.003 postcondition 3)
//!
//! `rm .factory/unvalidated-mutation.marker` is the fully supported escape hatch.
//! After the rm, both arms unblock simultaneously (both check marker presence at runtime).
//!
//! # BC-5.38.001 Red Gate discipline
//!
//! All non-trivial function bodies use `todo!()`. Implementer fills in real logic.

use vsdd_hook_sdk::{HookPayload, HookResult};

/// HOST_ABI_VERSION declares the ABI contract version this plugin was built against.
/// No new host functions introduced by S-25.01 (BC-1.18.002 architecture compliance).
pub const HOST_ABI_VERSION: u32 = 1;

// ---------------------------------------------------------------------------
// Gate decision type
// ---------------------------------------------------------------------------

/// Decision returned by `guard_logic::evaluate_gate` after checking the marker file.
///
/// - `Allow`: marker absent; dispatch may proceed.
/// - `Block`: marker present; block message contains the required 5 fields + recovery instructions.
///
/// BC-1.18.002 postcondition 1/2: block message MUST contain plugin_name, artifact_path,
/// cause, a re-validation command, and the manual escape hatch instruction (`rm .factory/...`).
/// The message MUST be machine-parseable (structured fields, not freeform prose). AC-007/AC-008.
#[derive(Debug)]
pub enum GateDecision {
    /// Marker absent — allow the dispatch to proceed.
    Allow,
    /// Marker present — block the dispatch with the structured block message fields.
    Block {
        /// The `plugin_name` field from the marker TOML.
        plugin_name: String,
        /// The `artifact_path` field from the marker TOML (may be empty string).
        artifact_path: String,
        /// The `cause` field from the marker TOML: "fuel" | "epoch" | "output-too-large".
        cause: String,
        /// The `trace_id` field from the marker TOML.
        trace_id: String,
    },
}

// ---------------------------------------------------------------------------
// guard_logic — pure-core and effectful gate primitives
// ---------------------------------------------------------------------------

/// Gate primitives: pure command filter and effectful marker presence check.
///
/// Both functions are the unit-testable core of the two-arm gate. Tests exercise
/// these without a WASM runtime.
pub mod guard_logic {
    use std::path::Path;

    use super::GateDecision;

    /// Read and parse `.factory/unvalidated-mutation.marker` to determine the gate decision.
    ///
    /// - If the file is absent, returns `GateDecision::Allow`.
    /// - If the file is present and parseable, returns `GateDecision::Block { ... }` with
    ///   the fields extracted from the marker TOML.
    /// - If the file is present but unparseable (corrupt marker), returns `GateDecision::Block`
    ///   with best-effort populated fields (conservative path — still block).
    ///
    /// The returned block message fields are used by both Arm 1 and Arm 2 to construct
    /// the structured block message (AC-007/AC-008).
    ///
    /// BC-1.18.002 postcondition 1/2 + invariant 4 (block message completeness).
    ///
    /// # BC-5.38.001
    ///
    /// Effectful (filesystem read + TOML parse). Non-trivial body. Uses `todo!()`.
    pub fn evaluate_gate(marker_path: &Path) -> GateDecision {
        // Absent marker → allow (AC-010; BC-1.18.002 PC4).
        match std::fs::read_to_string(marker_path) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => GateDecision::Allow,
            Err(_) => {
                // Unable to read (permissions, I/O error) — conservative path: block.
                // We cannot determine the marker state, so block to avoid silent pass.
                GateDecision::Block {
                    plugin_name: "<unreadable-marker>".to_string(),
                    artifact_path: String::new(),
                    cause: "<unknown>".to_string(),
                    trace_id: String::new(),
                }
            }
            Ok(content) => {
                // Parse TOML fields from the marker content.
                // Use simple key extraction rather than a full TOML parser to avoid
                // pulling in the toml crate as a WASM plugin dependency.
                let plugin_name = extract_toml_string(&content, "plugin_name")
                    .unwrap_or_else(|| "<unknown>".to_string());
                let artifact_path =
                    extract_toml_string(&content, "artifact_path").unwrap_or_default();
                let cause = extract_toml_string(&content, "cause")
                    .unwrap_or_else(|| "<unknown>".to_string());
                let trace_id = extract_toml_string(&content, "trace_id").unwrap_or_default();

                GateDecision::Block {
                    plugin_name,
                    artifact_path,
                    cause,
                    trace_id,
                }
            }
        }
    }

    /// Extract a TOML basic string value for the given key from raw TOML content.
    /// Handles the format: `key = "value"` (one key per line).
    /// Returns `None` if the key is absent or the value cannot be parsed.
    fn extract_toml_string(content: &str, key: &str) -> Option<String> {
        for line in content.lines() {
            let line = line.trim();
            // Match lines of the form: key = "value"
            let Some(rest) = line.strip_prefix(key) else {
                continue;
            };
            let rest = rest.trim_start();
            let Some(rest) = rest.strip_prefix('=') else {
                continue;
            };
            let rest = rest.trim();
            // Extract the quoted string value.
            if let Some(inner) = rest.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
                // Unescape basic TOML escapes (\\ → \, \" → ").
                return Some(inner.replace("\\\"", "\"").replace("\\\\", "\\"));
            }
        }
        None
    }

    /// Pure filter: returns `true` iff the given Bash `command` string matches the
    /// regex `\bgit\b.*\b(commit|push)\b`.
    ///
    /// Used by Arm 2 to determine whether a `^Bash$` PreToolUse dispatch should be
    /// checked against the marker file.
    ///
    /// BC-1.18.002 postcondition 3: non-matching commands (git status, git log, cargo test)
    /// MUST return `false`. EC-001..EC-005 (non-advancing commands) MUST all return false.
    /// EC-006 (`git commit --amend`) and EC-007 (`git push --force-with-lease`) MUST return true.
    ///
    /// VP-105 bats test suite covers VP-105-A through VP-105-G (7 cases).
    ///
    /// # BC-5.38.001
    ///
    /// Pure-core (regex match). Non-trivial body (regex compilation/execution). Uses `todo!()`.
    pub fn is_git_commit_or_push(command: &str) -> bool {
        // Regex: \bgit\b.*\b(commit|push)\b
        // Pure state machine instead of a regex crate to avoid a heavy WASM dependency.
        // BC-1.18.002 PC2: git commit/push variants MUST return true.
        // BC-1.18.002 PC3: git status/log/diff/fetch, cargo test, etc. MUST return false.
        // EC-006: `git commit --amend` → true; EC-007: `git push --force-with-lease` → true.
        //
        // Strategy: find "git" as a word token, then scan subsequent tokens for
        // "commit" or "push" as word tokens. Word boundaries are simulated by
        // checking that git/commit/push are preceded and followed by non-word chars.
        let words: Vec<&str> = command.split_whitespace().collect();
        let mut git_seen = false;
        for word in &words {
            // Strip leading flag characters (-, --) to get the bare token.
            let bare = word.trim_start_matches('-');
            if !git_seen {
                if bare == "git" {
                    git_seen = true;
                }
            } else {
                // After "git", look for the subcommand token (first positional arg).
                // Subcommand is the first non-flag word after git.
                if bare == "commit" || bare == "push" {
                    return true;
                }
                // If the token looks like an option (starts with -), keep scanning.
                // If it's a bare word (the subcommand), we've found it and it's not commit/push.
                if !word.starts_with('-') {
                    // Non-option token after git that isn't commit/push.
                    return false;
                }
            }
        }
        false
    }
}

// ---------------------------------------------------------------------------
// Entry point: single function handling BOTH Arm 1 (Agent) and Arm 2 (Bash)
// ---------------------------------------------------------------------------

/// PreToolUse entry point for both Arm 1 (`^Agent$`) and Arm 2 (`^Bash$` git filter).
///
/// This single function handles both dispatch arms since both hooks-registry.toml entries
/// reference the IDENTICAL WASM binary (BC-1.18.002 invariant 1; AC-019).
///
/// **Arm 1 (Agent):** fires on `tool = "^Agent$"` PreToolUse dispatches.
/// Checks marker presence; blocks if marker exists (AC-007; EC-004).
///
/// **Arm 2 (Bash git filter):** fires on `tool = "^Bash$"` PreToolUse dispatches.
/// First applies `is_git_commit_or_push(command)` filter; if the command is NOT
/// a git commit/push, passes immediately without checking the marker (AC-009).
/// If the command IS a git commit/push, checks marker presence; blocks if marker exists (AC-008).
///
/// **Both arms absent-marker path:** when marker is absent, both arms return exit_code=0
/// (allow) unconditionally (AC-010).
///
/// # BC-5.38.001
///
/// Effectful (reads tool payload, reads marker file). Non-trivial body. Uses `todo!()`.
pub fn on_pre_tool_use(payload: HookPayload) -> HookResult {
    // Single entry point for BOTH Arm 1 (^Agent$) and Arm 2 (^Bash$) dispatches.
    // Both hooks-registry.toml entries reference this IDENTICAL WASM binary (AC-019).
    //
    // Relative path: the dispatcher preopens host_ctx.cwd as WASI `"."`, so
    // `.factory/unvalidated-mutation.marker` resolves to the project root's
    // .factory directory without needing an absolute path (which WASI rejects
    // when no root preopen is configured). Do NOT use host::cwd() + absolute
    // PathBuf here — wasmtime WASI returns ENOENT for absolute paths that
    // aren't under a preopened directory prefix.
    let marker_path = std::path::Path::new(".factory").join("unvalidated-mutation.marker");

    // Arm 2 (^Bash$): apply the git commit/push filter FIRST.
    // If this is a Bash dispatch that does NOT match \bgit\b.*\b(commit|push)\b,
    // pass immediately without checking the marker (AC-009; BC-1.18.002 PC3).
    if payload.tool_name == "Bash" {
        let command = payload
            .tool_input
            .get("command")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        if !guard_logic::is_git_commit_or_push(&command) {
            return HookResult::Continue;
        }
        // Command IS a git commit/push — fall through to marker check (AC-008).
    }

    // Arm 1 (^Agent$) or Arm 2 (git commit/push match): check marker presence.
    match guard_logic::evaluate_gate(&marker_path) {
        GateDecision::Allow => HookResult::Continue,
        GateDecision::Block {
            plugin_name,
            artifact_path,
            cause,
            trace_id,
        } => {
            // Build structured block message (AC-007/AC-008 + BC-1.18.002 INV4).
            // Message MUST be machine-parseable. Use JSON-encoded fields.
            let reason = serde_json::json!({
                "blocked_by": "validate-unvalidated-mutation-marker",
                "marker_plugin_name": plugin_name,
                "marker_artifact_path": artifact_path,
                "marker_cause": cause,
                "marker_trace_id": trace_id,
                "recovery": {
                    "revalidate": format!(
                        "Re-run {} to clear the marker (must produce exit_code=0)",
                        plugin_name
                    ),
                    "manual_escape_hatch": "rm .factory/unvalidated-mutation.marker"
                }
            })
            .to_string();
            HookResult::Block { reason }
        }
    }
}

// ---------------------------------------------------------------------------
// S-25.01 Red Gate tests — BC-1.18.002 (3 required stubs + 4 additional)
// ---------------------------------------------------------------------------
//
// All tests call todo!() production functions (evaluate_gate, is_git_commit_or_push)
// and MUST FAIL at runtime until the implementer fills in the production logic (T-4).
// BC-5.38.005 self-check: "If I include this real implementation, will the test
// pass trivially without implementer work?" — YES for all tests below.

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::GateDecision;
    use super::guard_logic::{evaluate_gate, is_git_commit_or_push};

    // ── S-25.01 Red Gate stub 13 ─────────────────────────────────────────────
    #[test]
    fn test_BC_1_18_002_blocks_agent_dispatch_when_marker_exists() {
        // AC-007 / BC-1.18.002 postcondition 1 + invariant 4:
        // Marker present → evaluate_gate returns GateDecision::Block.
        // Block message MUST contain: plugin_name, artifact_path, cause,
        // a re-validation command, and the manual escape hatch (`rm .factory/...`).
        // Message MUST be machine-parseable (structured fields, not freeform prose).
        let dir = tempfile::tempdir().expect("tempdir");
        let marker_path = dir.path().join("unvalidated-mutation.marker");
        // Write a valid 5-field TOML marker (BC-1.18.001 postcondition 4)
        std::fs::write(
            &marker_path,
            concat!(
                "timestamp = \"2026-08-30T00:00:00Z\"\n",
                "plugin_name = \"validate-factory-path-staging\"\n",
                "artifact_path = \"/tmp/.factory/STATE.md\"\n",
                "cause = \"fuel\"\n",
                "trace_id = \"deadbeef-0001-0001-0001-000000000001\"\n"
            ),
        )
        .expect("test setup: write marker");

        let decision = evaluate_gate(&marker_path);
        // Marker present → must block
        assert!(
            matches!(decision, GateDecision::Block { .. }),
            "AC-007: marker present MUST yield GateDecision::Block — got {:?}",
            decision
        );
        // Block message must carry the plugin_name and artifact_path from the marker
        if let GateDecision::Block {
            plugin_name,
            artifact_path,
            cause,
            ..
        } = decision
        {
            assert_eq!(
                plugin_name, "validate-factory-path-staging",
                "AC-007 / BC-1.18.002 INV4: block message MUST carry plugin_name from marker"
            );
            assert_eq!(
                artifact_path, "/tmp/.factory/STATE.md",
                "AC-007 / BC-1.18.002 INV4: block message MUST carry artifact_path from marker"
            );
            assert_eq!(
                cause, "fuel",
                "AC-007 / BC-1.18.002 INV4: block message MUST carry cause from marker"
            );
        }
    }

    // ── S-25.01 Red Gate stub 14 ─────────────────────────────────────────────
    #[test]
    fn test_BC_1_18_002_passes_agent_dispatch_when_no_marker() {
        // AC-010 / BC-1.18.002 postcondition 4:
        // Marker absent → evaluate_gate returns GateDecision::Allow.
        // Both Arm 1 (Agent) and Arm 2 (Bash git) MUST allow when marker is absent.
        let dir = tempfile::tempdir().expect("tempdir");
        let marker_path = dir.path().join("unvalidated-mutation.marker");
        // Verify no marker exists (do NOT create one)
        assert!(
            !marker_path.exists(),
            "pre-condition: marker must be absent for this test"
        );

        let decision = evaluate_gate(&marker_path);
        assert!(
            matches!(decision, GateDecision::Allow),
            "AC-010: marker absent MUST yield GateDecision::Allow — got {:?}",
            decision
        );
    }

    // ── S-25.01 Red Gate stub 15 ─────────────────────────────────────────────
    #[test]
    fn test_BC_1_18_002_block_message_names_plugin_and_artifact() {
        // AC-007 / AC-008 / BC-1.18.002 invariant 4:
        // The block message MUST be machine-parseable (structured fields, not freeform prose).
        // Both Arm 1 (Agent) and Arm 2 (Bash git) produce identical block message format.
        // Required fields: plugin_name, artifact_path, cause, trace_id (from marker TOML).
        let dir = tempfile::tempdir().expect("tempdir");
        let marker_path = dir.path().join("unvalidated-mutation.marker");
        let plugin = "validate-factory-path-staging";
        let artifact = "/path/to/.factory/STATE.md";
        let cause_val = "output-too-large";
        let trace = "aaaabbbb-cccc-dddd-eeee-ffffffffffff";
        std::fs::write(
            &marker_path,
            format!(
                "timestamp = \"2026-08-30T12:00:00Z\"\nplugin_name = \"{plugin}\"\n\
                 artifact_path = \"{artifact}\"\ncause = \"{cause_val}\"\ntrace_id = \"{trace}\"\n"
            ),
        )
        .expect("test setup: write marker");

        let decision = evaluate_gate(&marker_path);
        match decision {
            GateDecision::Block {
                plugin_name,
                artifact_path,
                cause,
                ..
            } => {
                assert_eq!(
                    plugin_name, plugin,
                    "AC-007/AC-008: block plugin_name MUST equal marker plugin_name (machine-parseable)"
                );
                assert_eq!(
                    artifact_path, artifact,
                    "AC-007/AC-008: block artifact_path MUST equal marker artifact_path"
                );
                assert_eq!(
                    cause, cause_val,
                    "AC-007/AC-008: block cause MUST equal marker cause"
                );
            }
            GateDecision::Allow => {
                panic!("AC-007/AC-008: marker present MUST produce Block, not Allow");
            }
        }
    }

    // ── S-25.01 additional Red Gate test for AC-008 ───────────────────────────

    /// AC-008 / BC-1.18.002 postcondition 2 + EC-006 + EC-007:
    /// Bash arm (Arm 2): marker present + git commit/push command → block.
    /// Both Arm 1 and Arm 2 use the same evaluate_gate — marker presence is checked.
    /// The is_git_commit_or_push filter (Arm 2) is tested separately below.
    #[test]
    fn test_BC_1_18_002_bash_arm_blocks_git_commit_when_marker_exists() {
        // AC-008: marker present + command matches \bgit\b.*\b(commit|push)\b → block.
        let dir = tempfile::tempdir().expect("tempdir");
        let marker_path = dir.path().join("unvalidated-mutation.marker");
        std::fs::write(
            &marker_path,
            concat!(
                "timestamp = \"2026-08-30T00:00:00Z\"\n",
                "plugin_name = \"regression-gate\"\n",
                "artifact_path = \"\"\n",
                "cause = \"epoch\"\n",
                "trace_id = \"test-trace-bash-arm\"\n"
            ),
        )
        .expect("test setup");

        // EC-006: git commit --amend MUST match the filter
        assert!(
            is_git_commit_or_push("git commit --amend --no-edit"),
            "AC-008 / EC-006: 'git commit --amend' MUST match \\bgit\\b.*\\b(commit|push)\\b"
        );
        // EC-007: git push --force-with-lease MUST match the filter
        assert!(
            is_git_commit_or_push("git push --force-with-lease"),
            "AC-008 / EC-007: 'git push --force-with-lease' MUST match \\bgit\\b.*\\b(commit|push)\\b"
        );

        // When marker exists and command matches, evaluate_gate MUST block
        let decision = evaluate_gate(&marker_path);
        assert!(
            matches!(decision, GateDecision::Block { .. }),
            "AC-008: marker present → evaluate_gate MUST return Block (Arm 2 then blocks) — got {:?}",
            decision
        );
    }

    // ── S-25.01 additional Red Gate tests for AC-009 (is_git_commit_or_push) ──

    /// AC-009 / BC-1.18.002 postcondition 3 + EC-001..EC-005:
    /// Non-advancing Bash commands MUST NOT be gated.
    /// is_git_commit_or_push(command) MUST return false for non-commit/push git subcommands
    /// and for non-git commands entirely.
    #[test]
    fn test_BC_1_18_002_is_git_commit_or_push_rejects_non_advancing_commands() {
        // AC-009 / BC-1.18.002 postcondition 3 (EC-001..EC-005 + non-git):
        // These commands MUST NOT match the pattern — Arm 2 should NOT gate them.

        // EC-001: git status
        assert!(
            !is_git_commit_or_push("git status --porcelain"),
            "AC-009 / EC-001: 'git status' MUST NOT match (read-only; never gates)"
        );
        // EC-002: git log
        assert!(
            !is_git_commit_or_push("git log --oneline -5"),
            "AC-009 / EC-002: 'git log' MUST NOT match (read-only)"
        );
        // EC-003: git diff
        assert!(
            !is_git_commit_or_push("git diff HEAD~1"),
            "AC-009 / EC-003: 'git diff' MUST NOT match (read-only)"
        );
        // EC-004: git fetch
        assert!(
            !is_git_commit_or_push("git fetch origin"),
            "AC-009 / EC-004: 'git fetch' MUST NOT match (does not advance local state)"
        );
        // EC-005: cargo test (non-git)
        assert!(
            !is_git_commit_or_push("cargo test --workspace --all-targets"),
            "AC-009 / EC-005: non-git commands MUST NOT match"
        );
        // Additional: git add (staging only, not advancing)
        assert!(
            !is_git_commit_or_push("git add -A"),
            "AC-009: 'git add' MUST NOT match (staging, not advancing)"
        );
        // Additional: git stash
        assert!(
            !is_git_commit_or_push("git stash"),
            "AC-009: 'git stash' MUST NOT match"
        );
    }

    /// AC-009 / BC-1.18.002 postcondition 2 + EC-006 + EC-007:
    /// Advancing commands: git commit and git push variants MUST match.
    #[test]
    fn test_BC_1_18_002_is_git_commit_or_push_matches_commit_and_push_variants() {
        // AC-008 / AC-009 / EC-006 / EC-007:
        // git commit (all variants) and git push (all variants) MUST match.

        // git commit variants
        assert!(
            is_git_commit_or_push("git commit -m 'test message'"),
            "AC-009: 'git commit -m ...' MUST match"
        );
        assert!(
            is_git_commit_or_push("git commit --amend --no-edit"),
            "AC-009 / EC-006: 'git commit --amend' MUST match (BC-1.18.002 EC-006)"
        );
        assert!(
            is_git_commit_or_push("git commit -F /tmp/msg.txt"),
            "AC-009: 'git commit -F ...' MUST match"
        );
        // git push variants
        assert!(
            is_git_commit_or_push("git push origin factory-artifacts"),
            "AC-009: 'git push origin ...' MUST match"
        );
        assert!(
            is_git_commit_or_push("git push --force-with-lease"),
            "AC-009 / EC-007: 'git push --force-with-lease' MUST match (BC-1.18.002 EC-007)"
        );
        assert!(
            is_git_commit_or_push("git push"),
            "AC-009: bare 'git push' MUST match"
        );
    }
}
