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
                // Parse marker TOML via the `toml` crate (Library table mandate; MEDIUM-4).
                // This correctly handles all escape sequences including \n, \r, control chars.
                // If the marker is corrupt (parse error), fall back to conservative block
                // with sentinel values so the operator knows parsing failed.
                let table: toml::Table = match toml::from_str(&content) {
                    Ok(t) => t,
                    Err(_) => {
                        return GateDecision::Block {
                            plugin_name: "<unparseable-marker>".to_string(),
                            artifact_path: String::new(),
                            cause: "<unknown>".to_string(),
                            trace_id: String::new(),
                        };
                    }
                };

                let plugin_name = table
                    .get("plugin_name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("<unknown>")
                    .to_string();
                let artifact_path = table
                    .get("artifact_path")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let cause = table
                    .get("cause")
                    .and_then(|v| v.as_str())
                    .unwrap_or("<unknown>")
                    .to_string();
                let trace_id = table
                    .get("trace_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                GateDecision::Block {
                    plugin_name,
                    artifact_path,
                    cause,
                    trace_id,
                }
            }
        }
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
        // Implements the spec intent: \bgit\b.*\b(commit|push)\b (BC-1.18.002 PC2).
        //
        // Strategy: tokenize by whitespace, find "git" as an exact standalone word,
        // then scan subsequent tokens. Global options that take a SEPARATE argument
        // (e.g., `-C <path>`, `-c <key>=<val>`, `--namespace <ns>`) cause the next
        // token to be skipped so the argument is not mistaken for the subcommand.
        // When the first positional (non-option, non-argument) token is found, return
        // true iff it equals "commit" or "push" exactly.
        //
        // NOTE: This uses exact subcommand matching rather than the literal spec regex
        // `\b(commit|push)\b` because that regex would also match `commit` within
        // `commit-graph` (since `-` is a word-boundary character). The test
        // `test_BC_1_18_002_is_git_commit_or_push_global_option_forms` requires
        // `git commit-graph write` → false. Observation surfaced: spec regex is
        // imprecise for this edge case; test expectation is the authoritative spec
        // per BC-5.38.001 / VSDD standing rule. Routed to product-owner for
        // BC-1.18.002 AC-009 clarification (not a blocker for this fix).
        //
        // Global options that take a separate following argument (not inline `=`):
        //   -C <path>               change working directory
        //   -c <key>=<value>        set config option
        //   --namespace <prefix>    operate in namespace
        // These are the forms tested and observed in dispatcher commit patterns.
        // Additional git global options (--work-tree, --git-dir, etc.) use inline `=`
        // syntax in practice, so they do not require special argument-skipping here.
        const OPTS_TAKING_ARG: &[&str] = &["-C", "-c", "--namespace"];

        let tokens: Vec<&str> = command.split_whitespace().collect();
        let n = tokens.len();
        let mut i = 0;

        // Find "git" as an exact standalone word token (not "gitk", not in a path).
        while i < n {
            if tokens[i] == "git" {
                i += 1;
                break;
            }
            i += 1;
        }

        // If "git" was not found or no tokens remain, return false.
        if i >= n {
            return false;
        }

        // Scan remaining tokens for the subcommand.
        while i < n {
            let token = tokens[i];

            // If this token is a global option that takes a SEPARATE argument, skip both
            // the option token AND the next token (its argument).
            if OPTS_TAKING_ARG.contains(&token) {
                i += 2; // skip option + its argument
                continue;
            }

            // Any other option flag (starts with `-`): skip the flag itself, no arg to skip.
            if token.starts_with('-') {
                i += 1;
                continue;
            }

            // First positional token reached — this is the git subcommand.
            // Return true iff it is exactly "commit" or "push" (exact word match,
            // not prefix — so "commit-graph" ≠ "commit").
            return token == "commit" || token == "push";
        }

        false
    }
}

// ---------------------------------------------------------------------------
// Entry point: single function handling BOTH Arm 1 (Agent) and Arm 2 (Bash)
// ---------------------------------------------------------------------------

/// Testable inner implementation — takes an explicit `marker_path` so unit tests
/// can inject a tempdir marker without coupling to the WASI preopened CWD.
///
/// Called by `on_pre_tool_use` with the hardcoded production path; called directly
/// by tests with a tempdir-based path. This extraction ensures tests exercise the
/// real dispatch-routing logic and real block-message assembly, not a reconstruction.
///
/// # BC-5.38.001
///
/// Effectful (reads tool payload, reads marker file). Non-trivial body.
pub(crate) fn on_pre_tool_use_impl(
    payload: HookPayload,
    marker_path: &std::path::Path,
) -> HookResult {
    // Single entry point for BOTH Arm 1 (^Agent$) and Arm 2 (^Bash$) dispatches.
    // Both hooks-registry.toml entries reference this IDENTICAL WASM binary (AC-019).

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
    match guard_logic::evaluate_gate(marker_path) {
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
/// Delegates to `on_pre_tool_use_impl` with the production marker path. The production
/// path is relative to the WASI preopened CWD (`"."`), which the dispatcher sets to
/// the project root — so `.factory/unvalidated-mutation.marker` resolves correctly
/// without requiring an absolute path (which WASI rejects when no root preopen is
/// configured).
pub fn on_pre_tool_use(payload: HookPayload) -> HookResult {
    // Relative path: the dispatcher preopens host_ctx.cwd as WASI `"."`, so
    // `.factory/unvalidated-mutation.marker` resolves to the project root's
    // .factory directory without needing an absolute path (which WASI rejects
    // when no root preopen is configured). Do NOT use host::cwd() + absolute
    // PathBuf here — wasmtime WASI returns ENOENT for absolute paths that
    // aren't under a preopened directory prefix.
    let marker_path = std::path::Path::new(".factory").join("unvalidated-mutation.marker");
    on_pre_tool_use_impl(payload, &marker_path)
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

    // ── BLOCKER-3 (RED): global-option forms ─────────────────────────────────

    /// BLOCKER-3 (RED): `is_git_commit_or_push` MUST return `true` when `git` is
    /// followed by global options that take arguments (`-C <path>`, `-c <key>=<val>`,
    /// `--namespace <ns>`) before the `commit` or `push` subcommand. AC-008 / AC-009 /
    /// BC-1.18.002 PC2.
    ///
    /// Currently FAILS (RED) for the `-C`, `-c`, and `--namespace` forms because the
    /// current scanner stops at the first non-flag positional token after `git`,
    /// treating it as the subcommand. With `git -C /path commit`, the scanner sees
    /// `/path` (the argument to `-C`) as the positional token and returns `false`.
    ///
    /// The regex `\bgit\b.*\b(commit|push)\b` (BC-1.18.002 PC2) correctly matches all
    /// these forms because `.*` spans the global options and their arguments.
    #[test]
    fn test_BC_1_18_002_is_git_commit_or_push_global_option_forms() {
        // AC-008 / AC-009 / BC-1.18.002 PC2
        // git global options with arguments:
        //   -C <path>         : change working directory
        //   -c <key>=<value>  : set a config option
        //   --namespace <ns>  : operate in namespace
        // All are valid before the subcommand and occur in real dispatcher commits.

        // RED cases (currently fail with current scanner):

        // git -C /path commit -m x → TRUE
        assert!(
            is_git_commit_or_push("git -C /path commit -m x"),
            "BLOCKER-3 / AC-008 / BC-1.18.002 PC2: \
             'git -C /path commit' MUST match \\bgit\\b.*\\b(commit|push)\\b. \
             FAILS: current scanner treats '/path' (arg to -C) as subcommand, returns false."
        );

        // git -C .factory push → TRUE
        assert!(
            is_git_commit_or_push("git -C .factory push"),
            "BLOCKER-3 / AC-008 / BC-1.18.002 PC2: \
             'git -C .factory push' MUST match. \
             FAILS: current scanner treats '.factory' (arg to -C) as subcommand, returns false."
        );

        // git -c user.email=x commit → TRUE
        assert!(
            is_git_commit_or_push("git -c user.email=x commit"),
            "BLOCKER-3 / AC-008 / BC-1.18.002 PC2: \
             'git -c user.email=x commit' MUST match. \
             FAILS: current scanner treats 'user.email=x' (arg to -c) as subcommand, returns false."
        );

        // git --namespace foo push → TRUE
        assert!(
            is_git_commit_or_push("git --namespace foo push"),
            "BLOCKER-3 / AC-008 / BC-1.18.002 PC2: \
             'git --namespace foo push' MUST match. \
             FAILS: current scanner treats 'foo' (arg to --namespace) as subcommand, returns false."
        );

        // git   commit (extra whitespace) → TRUE (split_whitespace normalizes)
        // NOTE: This case likely already passes; included for completeness.
        assert!(
            is_git_commit_or_push("git   commit"),
            "AC-008: 'git   commit' with extra spaces MUST match (split_whitespace normalizes)"
        );

        // Non-matching forms MUST remain FALSE after the fix:

        // gitk is a different program entirely (no word boundary match)
        assert!(
            !is_git_commit_or_push("gitk"),
            "AC-009: 'gitk' is NOT 'git'; MUST NOT match"
        );

        // git commit-graph write: 'commit-graph' contains 'commit' as a prefix but the
        // regex \bcommit\b does NOT match it because '-' is a word boundary.
        // However, the *current* split_whitespace scanner also correctly returns false here
        // (first token after git is 'commit-graph' ≠ 'commit'). Both current and correct
        // implementations agree on this case. Verify the correct semantic is preserved.
        // BC-1.18.002 v1.1 EC-011: commit-graph → false.
        assert!(
            !is_git_commit_or_push("git commit-graph write"),
            "AC-009 / EC-011 / BC-1.18.002 v1.1: 'git commit-graph write' MUST NOT match — \
             'commit-graph' is NOT the 'commit' subcommand (exact subcommand matching; EC-011)"
        );

        // Read-only commands MUST NOT match
        assert!(
            !is_git_commit_or_push("git status"),
            "AC-009 / EC-001: 'git status' MUST NOT match"
        );
        assert!(
            !is_git_commit_or_push("git log --oneline"),
            "AC-009 / EC-002: 'git log' MUST NOT match"
        );

        // ── BC-1.18.002 v1.1 PO canonical vector set — all 12 exact forms ──────
        //
        // PO ruling (BC-1.18.002 v1.1): is_git_commit_or_push uses EXACT-SUBCOMMAND
        // matching. The following 12 vectors are the authoritative golden set.
        //
        // TRUE (advancing — MUST be gated):
        //   git commit -m "fix"
        //   git push origin main
        //   git -C .factory commit -m "state"    [EC-012: git -C commit → true]
        //   git -c user.email=x push origin main
        //   git commit --amend --no-edit
        //   git push --force-with-lease
        //
        // FALSE (non-advancing — MUST NOT be gated):
        //   git commit-graph write               [EC-011: commit-graph → false]
        //   git status --porcelain
        //   git log --oneline
        //   git diff HEAD~1
        //   git fetch origin
        //   cargo test --workspace

        // TRUE canonical vectors ------------------------------------------------

        assert!(
            is_git_commit_or_push("git commit -m \"fix\""),
            "BC-1.18.002 v1.1 canonical TRUE: 'git commit -m \"fix\"' MUST match"
        );
        assert!(
            is_git_commit_or_push("git push origin main"),
            "BC-1.18.002 v1.1 canonical TRUE: 'git push origin main' MUST match"
        );
        // EC-012: git -C <path> commit → true.
        // The -C global option takes a separate argument; the subcommand is 'commit'.
        assert!(
            is_git_commit_or_push("git -C .factory commit -m \"state\""),
            "BC-1.18.002 v1.1 EC-012: 'git -C .factory commit -m \"state\"' MUST match — \
             -C takes the path argument; 'commit' is the subcommand (EC-012: git -C commit → true)"
        );
        assert!(
            is_git_commit_or_push("git -c user.email=x push origin main"),
            "BC-1.18.002 v1.1 canonical TRUE: 'git -c user.email=x push origin main' MUST match \
             (-c takes the key=value argument; 'push' is the subcommand)"
        );
        assert!(
            is_git_commit_or_push("git commit --amend --no-edit"),
            "BC-1.18.002 v1.1 canonical TRUE: 'git commit --amend --no-edit' MUST match"
        );
        assert!(
            is_git_commit_or_push("git push --force-with-lease"),
            "BC-1.18.002 v1.1 canonical TRUE: 'git push --force-with-lease' MUST match"
        );

        // FALSE canonical vectors -----------------------------------------------

        // EC-011: commit-graph → false (also asserted above; canonical coverage).
        assert!(
            !is_git_commit_or_push("git commit-graph write"),
            "BC-1.18.002 v1.1 EC-011 canonical FALSE: 'git commit-graph write' MUST NOT match \
             ('commit-graph' is a plumbing subcommand, not 'commit'; EC-011)"
        );
        assert!(
            !is_git_commit_or_push("git status --porcelain"),
            "BC-1.18.002 v1.1 canonical FALSE: 'git status --porcelain' MUST NOT match"
        );
        assert!(
            !is_git_commit_or_push("git log --oneline"),
            "BC-1.18.002 v1.1 canonical FALSE: 'git log --oneline' MUST NOT match"
        );
        assert!(
            !is_git_commit_or_push("git diff HEAD~1"),
            "BC-1.18.002 v1.1 canonical FALSE: 'git diff HEAD~1' MUST NOT match"
        );
        assert!(
            !is_git_commit_or_push("git fetch origin"),
            "BC-1.18.002 v1.1 canonical FALSE: 'git fetch origin' MUST NOT match"
        );
        assert!(
            !is_git_commit_or_push("cargo test --workspace"),
            "BC-1.18.002 v1.1 canonical FALSE: 'cargo test --workspace' MUST NOT match \
             (non-git command)"
        );
    }

    // ── LOW-6: block message content (AC-007 / AC-008) ───────────────────────

    /// LOW-6: `on_pre_tool_use_impl` produces a JSON block message whose ACTUAL
    /// emitted structure (not a reconstruction) MUST contain:
    ///   - `marker_plugin_name` — the plugin_name from the marker TOML
    ///   - `recovery.revalidate` — references the blocking plugin name
    ///   - `recovery.manual_escape_hatch` — exactly "rm .factory/unvalidated-mutation.marker"
    ///   - be machine-parseable (valid JSON)
    ///
    /// AC-007 / AC-008 / BC-1.18.002 INV4.
    ///
    /// **Adversary OBS fix (S-25.01):** the previous version of this test
    /// *reconstructed* the block-message JSON using a copied `serde_json::json!`
    /// shape, then asserted on that copy — validating a duplicate, not the real
    /// production output. A future change to `on_pre_tool_use_impl`'s actual JSON
    /// shape would NOT have been caught. This rewrite calls `on_pre_tool_use_impl`
    /// directly with a real `HookPayload` (Agent arm AND Bash git commit arm) and
    /// asserts on the actual returned `HookResult::Block { reason }`.
    ///
    /// The `marker_path` parameter on `on_pre_tool_use_impl` is the injection point
    /// that makes native unit tests viable without writing to the real `.factory/`
    /// directory or depending on WASI preopened-directory semantics.
    #[test]
    fn test_BC_1_18_002_block_message_contains_required_fields_and_escape_hatch() {
        use super::on_pre_tool_use_impl;
        use vsdd_hook_sdk::HookResult;

        let dir = tempfile::tempdir().expect("tempdir");
        let marker_path = dir.path().join("unvalidated-mutation.marker");

        let expected_plugin = "validate-factory-path-staging";
        let expected_artifact = "/path/to/.factory/STATE.md";
        let expected_cause = "fuel";
        let expected_trace = "trace-low6-test";

        std::fs::write(
            &marker_path,
            format!(
                "timestamp = \"2026-08-31T00:00:00Z\"\n\
                 plugin_name = \"{expected_plugin}\"\n\
                 artifact_path = \"{expected_artifact}\"\n\
                 cause = \"{expected_cause}\"\n\
                 trace_id = \"{expected_trace}\"\n"
            ),
        )
        .expect("test setup: write marker");

        // ── Arm 1: Agent dispatch ──────────────────────────────────────────────
        // AC-007: when marker is present, an Agent PreToolUse dispatch MUST produce
        // HookResult::Block with a machine-parseable JSON reason.
        let agent_payload: vsdd_hook_sdk::HookPayload = serde_json::from_str(
            r#"{
                "event_name": "PreToolUse",
                "tool_name": "Agent",
                "session_id": "test-sess-low6",
                "dispatcher_trace_id": "test-trace-low6",
                "tool_input": {
                    "subagent_type": "vsdd-factory:state-manager",
                    "prompt": "advance state"
                }
            }"#,
        )
        .expect("agent payload should deserialize");

        let agent_result = on_pre_tool_use_impl(agent_payload, &marker_path);
        let agent_reason = match agent_result {
            HookResult::Block { ref reason } => reason.clone(),
            other => panic!(
                "AC-007: Agent arm MUST produce HookResult::Block when marker present — got {other:?}"
            ),
        };

        // Assert on the ACTUAL emitted reason (not a reconstruction).
        // AC-007: block reason MUST be valid JSON
        let agent_parsed: serde_json::Value = serde_json::from_str(&agent_reason)
            .expect("AC-007 / BC-1.18.002 INV4: Agent arm block reason MUST be valid JSON");

        // AC-007: marker plugin_name MUST appear in the block message
        assert_eq!(
            agent_parsed
                .get("marker_plugin_name")
                .and_then(|v| v.as_str()),
            Some(expected_plugin),
            "AC-007 / BC-1.18.002 INV4: 'marker_plugin_name' MUST equal marker plugin_name '{expected_plugin}'"
        );

        // AC-007: recovery object MUST be present with both required subfields
        let agent_recovery = agent_parsed
            .get("recovery")
            .expect("AC-007 / BC-1.18.002 INV4: block reason MUST have a structured 'recovery' field");

        let revalidate = agent_recovery
            .get("revalidate")
            .and_then(|v| v.as_str())
            .expect("AC-007: recovery MUST have 'revalidate' subfield");
        // The revalidate command MUST reference the blocking plugin by name so the
        // operator knows which plugin to re-run.
        assert!(
            revalidate.contains(expected_plugin),
            "AC-007: revalidate command MUST reference the marker plugin_name '{expected_plugin}' \
             so the operator knows what to re-run — got: {revalidate}"
        );

        let escape_hatch = agent_recovery
            .get("manual_escape_hatch")
            .and_then(|v| v.as_str())
            .expect("AC-007: recovery MUST have 'manual_escape_hatch' subfield");
        assert_eq!(
            escape_hatch,
            "rm .factory/unvalidated-mutation.marker",
            "AC-007 / BC-1.18.003 PC3: manual_escape_hatch MUST be \
             'rm .factory/unvalidated-mutation.marker' — the fully supported operator escape hatch"
        );

        // ── Arm 2: Bash git commit dispatch ───────────────────────────────────
        // AC-008: when marker is present, a Bash git commit PreToolUse dispatch
        // MUST also produce HookResult::Block with the same structured JSON reason.
        let bash_payload: vsdd_hook_sdk::HookPayload = serde_json::from_str(
            r#"{
                "event_name": "PreToolUse",
                "tool_name": "Bash",
                "session_id": "test-sess-low6",
                "dispatcher_trace_id": "test-trace-low6",
                "tool_input": {
                    "command": "git commit -m 'fix: S-25.01 block-message test invokes production path'"
                }
            }"#,
        )
        .expect("bash payload should deserialize");

        let bash_result = on_pre_tool_use_impl(bash_payload, &marker_path);
        let bash_reason = match bash_result {
            HookResult::Block { ref reason } => reason.clone(),
            other => panic!(
                "AC-008: Bash git commit arm MUST produce HookResult::Block when marker present \
                 — got {other:?}"
            ),
        };

        // Assert on the ACTUAL emitted reason from the Bash arm.
        let bash_parsed: serde_json::Value = serde_json::from_str(&bash_reason)
            .expect("AC-008 / BC-1.18.002 INV4: Bash arm block reason MUST be valid JSON");

        assert_eq!(
            bash_parsed
                .get("marker_plugin_name")
                .and_then(|v| v.as_str()),
            Some(expected_plugin),
            "AC-008 / BC-1.18.002 INV4: Bash arm 'marker_plugin_name' MUST equal \
             marker plugin_name '{expected_plugin}'"
        );
        assert_eq!(
            bash_parsed
                .get("recovery")
                .and_then(|r| r.get("manual_escape_hatch"))
                .and_then(|v| v.as_str()),
            Some("rm .factory/unvalidated-mutation.marker"),
            "AC-008 / BC-1.18.003 PC3: Bash arm escape hatch MUST be \
             'rm .factory/unvalidated-mutation.marker'"
        );
    }
}
