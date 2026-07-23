//! validate-factory-path-staging — PreToolUse WASM hook plugin.
//!
//! Implements the invariant layer (Layer-1) for INV-E21-001 (Nested Worktree
//! Path Exclusivity). Fires on every `PreToolUse` event where the Bash tool
//! payload contains a `git add` command. Blocks staging of `.factory/`-rooted
//! paths on product branches; passes all other commands unconditionally.
//!
//! # Behavioral Contracts
//!
//! - BC-4.16.001: blocks `git add` of `.factory/` paths on product branches
//!   (PC1); passes non-`.factory/` staging (PC2); passes on `factory-artifacts`
//!   branch (PC3); passes non-`git add` commands (PC4); fail-open on crash/
//!   branch-detection failure (Invariants 2/3).
//!
//! # Architecture compliance
//!
//! - HOST_ABI_VERSION = 1 (no new host functions introduced).
//! - ADR-031 §Decision 3: this crate is DISTINCT from
//!   `crates/hook-plugins/validate-artifact-path/` (serves BC-4.11.001).
//! - POLICY 21: native WASM only — no `.sh` files introduced.
//! - Pure `fn hook_logic(...)` takes all host I/O as injectable closures.
//!   Unit tests exercise every branch without a WASM runtime.

use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// ABI version constant (BC-4.16.001 architecture compliance)
// ---------------------------------------------------------------------------

/// HOST_ABI_VERSION declares the ABI contract version this plugin was built
/// against. The dispatcher reads this before any host call. Must remain 1.
pub const HOST_ABI_VERSION: u32 = 1;

// ---------------------------------------------------------------------------
// Branch and command classification (pure functions — injectable-testable)
// ---------------------------------------------------------------------------

/// Returns true if the Bash payload string contains a `git add` or `git stage` command.
///
/// Matches `git\s+(add|stage)` (case-insensitive, any whitespace between tokens) per
/// BC-4.16.001 Precondition 2 v1.3. `git stage` is a true git synonym for `git add`
/// (verified: `git help stage` confirms it is an alias). Whitespace-tolerant: handles
/// double-space and tab-separated forms (e.g., `git  add`, `git\tadd`).
///
/// Implementation: tokenizes by whitespace and checks whether the first `git` token
/// is followed immediately by `add` or `stage`. No regex dependency — hand-rolled
/// tokenization consistent with sibling validator crates which avoid the regex crate
/// due to WASM fuel budget constraints.
///
/// All other commands — `git commit`, `git push`, etc. — are not in scope and this
/// function returns false for them.
///
/// # BC trace
/// BC-4.16.001 Precondition 2 v1.3: detect `git\s+(add|stage)` by whitespace tokenization.
/// BC-4.16.001 PC4: non-`git add`/`git stage` commands pass unconditionally.
pub fn is_git_add_command(payload: &str) -> bool {
    let mut tokens = payload.split_whitespace();
    while let Some(token) = tokens.next() {
        if token.eq_ignore_ascii_case("git") {
            return tokens
                .next()
                .map(|t| t.eq_ignore_ascii_case("add") || t.eq_ignore_ascii_case("stage"))
                .unwrap_or(false);
        }
    }
    false
}

/// Returns true if the `git add` argument text contains or implies a
/// `.factory/`-rooted path that should be blocked on a product branch.
///
/// Conservative matching per BC-4.16.001 Invariant 4 v1.3:
/// - Literal `.factory/` prefix match (captures both relative and absolute
///   path forms where `.factory/` appears as a component).
/// - Bare `.factory` token (no trailing slash): git treats `.factory` as
///   `.factory/**` for staging — identical dual-tracking scope to `.factory/`
///   (v1.3 addition).
/// - `-A`, `--all`, `-u`, `--update`, `.`, `./` flags: treated conservatively
///   as potentially staging `.factory/` content. `./` is CWD-relative with
///   explicit slash, semantically identical to `.` for staging (v1.3 addition).
/// - `:/`-family pathspec magic: `:/` anchors from repo root and can include
///   `.factory/` paths regardless of CWD. Quoted forms (e.g., `':/.factory'`)
///   are detected after stripping surrounding `'` or `"` characters (v1.3
///   addition).
/// - Glob wildcards (`*`, `?`, `[`): conservatively blocked because the guard
///   inspects only literal argument text; git has not yet expanded the glob
///   and may produce `.factory/**` matches (EC-008).
/// - Combined short flags containing `A` or `u` (e.g., `-Au`).
///
/// # BC trace
/// BC-4.16.001 Invariant 4 v1.3: path matching is conservative.
/// BC-4.16.001 EC-004: `git add -A` from CWD under `.factory/` is blocked.
/// BC-4.16.001 EC-008: `git add *.md` glob from project root is blocked.
/// BC-4.16.001 EC-010: `git add -u` is blocked (tracks all modifications).
pub fn contains_factory_path_arg(git_add_args: &str) -> bool {
    // Explicit .factory/ path prefix or component anywhere in payload.
    if git_add_args.contains(".factory/") {
        return true;
    }

    // Scan tokens for conservative path forms, wildcards, and flags.
    // Skip the "git", "add", and "stage" command words; inspect only argument tokens.
    for token in git_add_args.split_whitespace() {
        if matches!(token, "git" | "add" | "stage") {
            continue;
        }

        // Strip surrounding single or double quotes for pathspec-magic analysis.
        // Handles `':/.factory'` and `":/..."` quoted forms.
        let unquoted = token.trim_matches(|c| c == '\'' || c == '"');

        // Bare .factory token without trailing slash (BC-4.16.001 Invariant 4 v1.3):
        // git expands `.factory` to `.factory/**` for staging — same dual-tracking
        // scope as `.factory/`.
        if unquoted == ".factory" {
            return true;
        }

        // :/-family pathspec magic (BC-4.16.001 Invariant 4 v1.3): anchors from
        // repo root; can reach .factory/ paths regardless of CWD.
        if unquoted.starts_with(":/") {
            return true;
        }

        match token {
            // Conservative bulk-stage flags: may include .factory/ content
            "-A" | "--all" | "-u" | "--update" | "." => return true,
            // "./" is CWD-relative with explicit slash — semantically identical to
            // "." for staging; may stage .factory/** when CWD is the project root
            // (BC-4.16.001 Invariant 4 v1.3)
            "./" => return true,
            // Glob wildcards: guard cannot evaluate expansions at PreToolUse time
            t if t.contains('*') || t.contains('?') || t.starts_with('[') => return true,
            // Combined short flags (e.g. "-Au", "-uA"): A=all, u=update
            t if t.starts_with('-') && !t.starts_with("--") && t.len() > 2 => {
                let flags = &t[1..];
                if flags.contains('A') || flags.contains('u') {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

/// Returns true if `branch` is a product branch (not `factory-artifacts`).
///
/// Product branches: `develop`, `main`, `feature/*`, `release/*`,
/// `maintenance/*`, and any unrecognized branch name (conservative default).
/// The only non-blocking branch is `factory-artifacts`.
///
/// Unrecognized branch names are treated as product branches — the conservative
/// default prevents a mistakenly named branch from silently bypassing the guard.
///
/// # BC trace
/// BC-4.16.001 PC3: `factory-artifacts` branch passes unconditionally.
/// BC-4.16.001 PC1: all other branches are product branches.
pub fn is_product_branch(branch: &str) -> bool {
    branch != "factory-artifacts"
}

// ---------------------------------------------------------------------------
// Injectable callback surface (testable without WASM runtime)
// ---------------------------------------------------------------------------

/// All side-effecting callbacks injected into `hook_logic` for testability.
/// In production (main.rs), these are wired to host functions.
pub struct HookCallbacks<B, E, L>
where
    B: FnOnce(&str, &[&str]) -> Result<(i32, String, String), String>,
    E: FnMut(&str, &[(&str, &str)]),
    L: FnMut(u8, &str),
{
    /// Execute a subprocess (binary, args); returns (exit_code, stdout, stderr).
    /// Used to invoke `git branch --show-current` for branch detection.
    pub exec_subprocess: B,
    /// Emit a structured event (type, fields).
    pub emit_event: E,
    /// Log a message at the given level (0=trace, 1=debug, 2=info, 3=warn, 4=error).
    pub log: L,
}

// ---------------------------------------------------------------------------
// Core hook logic (injectable callbacks — testable without WASM runtime)
// ---------------------------------------------------------------------------

/// Core validate-factory-path-staging hook logic.
///
/// All host I/O is injected via `callbacks` so unit tests can exercise every
/// branch without a WASM runtime.
///
/// Algorithm:
/// 1. Extract the Bash command string from `payload.tool_input["command"]`.
/// 2. If the command is NOT a `git add` command, return `Continue` (PC4).
/// 3. Detect the current branch via `exec_subprocess("git", ["branch", "--show-current"])`.
///    If branch detection fails, return `Continue` (fail-open, Invariant 3).
/// 4. If branch == `factory-artifacts`, return `Continue` (PC3).
/// 5. If the command arguments contain a `.factory/`-rooted path (or conservative
///    wildcards), return `block_intent = true` (exit 2) with `FactoryPathOnProductBranch`
///    error (PC1).
/// 6. Otherwise return `Continue` (PC2).
///
/// # BC traces
/// - BC-4.16.001 PC1: block .factory/ staging on product branches
/// - BC-4.16.001 PC2: pass non-.factory/ git add commands
/// - BC-4.16.001 PC3: pass all commands on factory-artifacts branch
/// - BC-4.16.001 PC4: pass non-git-add commands immediately
/// - BC-4.16.001 Invariants 2/3: fail-open on crash or branch detection failure
pub fn hook_logic<B, E, L>(
    payload: HookPayload,
    mut callbacks: HookCallbacks<B, E, L>,
) -> HookResult
where
    B: FnOnce(&str, &[&str]) -> Result<(i32, String, String), String>,
    E: FnMut(&str, &[(&str, &str)]),
    L: FnMut(u8, &str),
{
    // Step 1: Extract the Bash command string from tool_input["command"].
    let command = match payload.tool_input.get("command").and_then(|v| v.as_str()) {
        Some(cmd) => cmd.to_string(),
        None => {
            (callbacks.log)(
                2,
                "validate-factory-path-staging: no 'command' field in tool_input",
            );
            return HookResult::Continue;
        }
    };

    // Step 2: PC4 — non-git-add commands pass unconditionally (no path inspection).
    if !is_git_add_command(&command) {
        return HookResult::Continue;
    }

    // Step 3: Branch detection via exec_subprocess.
    // Fail-open on any failure per BC-4.16.001 Invariant 3.
    let branch = match (callbacks.exec_subprocess)("git", &["branch", "--show-current"]) {
        Ok((exit_code, stdout, stderr)) => {
            if exit_code != 0 {
                (callbacks.log)(
                    3,
                    &format!(
                        "validate-factory-path-staging: branch detection returned exit \
                         {exit_code} (stderr: {stderr}), failing open per Invariant 3"
                    ),
                );
                return HookResult::Continue;
            }
            let b = stdout.trim().to_string();
            if b.is_empty() {
                // Empty stdout = detached HEAD state — fail-open per Invariant 3.
                (callbacks.log)(
                    3,
                    "validate-factory-path-staging: empty branch output (detached HEAD?), \
                     failing open per Invariant 3",
                );
                return HookResult::Continue;
            }
            b
        }
        Err(e) => {
            // git unavailable or exec failure — fail-open per Invariant 3.
            (callbacks.log)(
                3,
                &format!(
                    "validate-factory-path-staging: branch detection failed ({e}), \
                     failing open per Invariant 3"
                ),
            );
            return HookResult::Continue;
        }
    };

    // Step 4: PC3 — factory-artifacts branch passes unconditionally.
    // Factory artifact commits require staging .factory/ paths on this branch.
    if !is_product_branch(&branch) {
        return HookResult::Continue;
    }

    // Step 5: PC1 — block if payload contains a .factory/-rooted path or
    // conservative wildcard/flag (per BC-4.16.001 Invariant 4).
    if contains_factory_path_arg(&command) {
        return HookResult::block_with_fix(
            "validate-factory-path-staging",
            format!(
                "FactoryPathOnProductBranch — git add of .factory/ path on product \
                 branch '{branch}'. .factory/ paths are exclusively owned by the \
                 factory-artifacts worktree. Staging .factory/ content on a product \
                 branch creates the dual-tracking condition that allows product-branch \
                 merges to silently delete factory artifact files"
            ),
            "Switch to the .factory/ worktree and commit from there on the \
             factory-artifacts branch",
            "FactoryPathOnProductBranch",
        );
    }

    // Step 6: PC2 — non-.factory/ git add commands pass.
    HookResult::Continue
}

// ---------------------------------------------------------------------------
// Top-level entry point (wired to real host fns in main.rs)
// ---------------------------------------------------------------------------

/// Called from the WASI entry point in `main.rs`.
///
/// Wires the real vsdd_hook_sdk host functions to the injectable-callback
/// surface of `hook_logic`.
pub fn on_pre_tool_use(payload: HookPayload) -> HookResult {
    hook_logic(
        payload,
        HookCallbacks {
            exec_subprocess: |cmd, args| match vsdd_hook_sdk::host::exec_subprocess(
                cmd,
                args,
                &[],
                5000,
                512,
            ) {
                Ok(result) => {
                    let stdout = String::from_utf8_lossy(&result.stdout).into_owned();
                    let stderr = String::from_utf8_lossy(&result.stderr).into_owned();
                    Ok((result.exit_code, stdout, stderr))
                }
                Err(e) => Err(format!("{e:?}")),
            },
            emit_event: |event_type, fields| {
                vsdd_hook_sdk::host::emit_event(event_type, fields);
            },
            log: |level, msg| match level {
                0..=2 => vsdd_hook_sdk::host::log_info(msg),
                3 => vsdd_hook_sdk::host::log_warn(msg),
                _ => vsdd_hook_sdk::host::log_error(msg),
            },
        },
    )
}

// ---------------------------------------------------------------------------
// Unit tests (Red Gate — all must fail until implementation is complete)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests;
