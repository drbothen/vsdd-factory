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

/// Returns true if the Bash payload string contains a `git add` or `git stage` command
/// in any form, including chained and global-option forms.
///
/// Detection contract per BC-4.16.001 Precondition 2 v1.4:
/// - Scans ALL tokens in the payload (not just the first `git` occurrence) so that
///   chained forms (`&&`, `;`, `|`) are fully covered — a `git add` or `git stage`
///   appearing after a different git command (e.g. `git status && git add`) is detected.
/// - For each `git` token found, skips any number of intervening global options or flags
///   before checking whether the first non-option subcommand token is `add` or `stage`
///   (case-insensitive). Handles `git -C <path> add`, `git --no-pager add`,
///   `git -c key=val add`, etc.
/// - `-C` and `-c` consume one following value token; all other `-*` flags do not.
/// - Subcommand tokens may have trailing shell metacharacters glued to them (e.g.
///   `"diff;"` in `git diff; git add`); the core word is extracted by stripping
///   trailing `;`, `&`, `|` before the case-insensitive comparison.
/// - No regex dependency — hand-rolled tokenizer consistent with sibling validator crates
///   (WASM fuel budget constraint).
///
/// # BC trace
/// BC-4.16.001 Precondition 2 v1.4: detect `git\s+(add|stage)` including global-option
///   forms and chained-command forms anywhere in the payload.
/// BC-4.16.001 PC4: non-`git add`/`git stage` commands pass unconditionally.
pub fn is_git_add_command(payload: &str) -> bool {
    let tokens: Vec<&str> = payload.split_whitespace().collect();
    let mut i = 0;
    'outer: while i < tokens.len() {
        if tokens[i].eq_ignore_ascii_case("git") {
            let mut j = i + 1;
            // Skip global options. -C and -c each consume one following value token;
            // all other -* flags are self-contained.
            while j < tokens.len() {
                let t = tokens[j];
                if t.starts_with('-') {
                    j += 1;
                    // -C <path> and -c <key=val> consume the next token as their value.
                    if (t == "-C" || t == "-c") && j < tokens.len() {
                        j += 1;
                    }
                } else {
                    // First non-option token is the subcommand. Strip trailing shell
                    // metacharacters that may be glued to the subcommand word (e.g. "diff;").
                    let core = t.trim_end_matches([';', '&', '|']);
                    if core.eq_ignore_ascii_case("add") || core.eq_ignore_ascii_case("stage") {
                        return true;
                    }
                    // Subcommand is not add/stage; resume outer scan past this token.
                    i = j + 1;
                    continue 'outer;
                }
            }
            // Inner loop exhausted without finding a subcommand for this git token.
            break 'outer;
        } else {
            i += 1;
        }
    }
    false
}

/// Returns true if the payload contains a `git add`/`git stage` command whose
/// arguments include or imply a `.factory/`-rooted path that should be blocked
/// on a product branch.
///
/// Conservative matching per BC-4.16.001 Invariant 4 v1.4:
/// - Literal `.factory/` prefix match anywhere in the payload (fast path).
/// - Parses each `git add`/`git stage` invocation in the payload (including
///   chained forms). Global option values (e.g. the directory argument to
///   `-C <path>`) are skipped and NOT treated as staging targets — only tokens
///   that are actual arguments to the `add`/`stage` subcommand are checked.
/// - In the argument region of each `add`/`stage` invocation, the following
///   patterns trigger a conservative block:
///   - Bare `.factory` token (no trailing slash): git expands to `.factory/**`.
///   - `:/`-family pathspec magic: anchors from repo root, can reach `.factory/`.
///   - `-A`, `--all`, `-u`, `--update`, `.`, `./`: bulk-stage flags / CWD forms.
///   - Glob wildcards (`*`, `?`, `[`): guard cannot evaluate expansions pre-run.
///   - Combined short flags containing `A` or `u` (e.g. `-Au`).
///
/// # BC trace
/// BC-4.16.001 Invariant 4 v1.4: path matching is conservative.
/// BC-4.16.001 EC-004: `git add -A` from CWD under `.factory/` is blocked.
/// BC-4.16.001 EC-008: `git add *.md` glob from project root is blocked.
/// BC-4.16.001 EC-010: `git add -u` is blocked (tracks all modifications).
pub fn contains_factory_path_arg(payload: &str) -> bool {
    // Fast path: `.factory/` literal prefix or component anywhere in payload.
    // Case-insensitive upgrade handled separately (F-P2-003).
    if payload.contains(".factory/") {
        return true;
    }

    // Parse git add/stage invocations in the payload and inspect their argument regions.
    // Global option values (e.g. the path argument to `-C <path>`) are skipped so that
    // e.g. `git -C . add src/lib.rs` correctly returns false (`.` is a -C value, not an
    // add argument).
    let tokens: Vec<&str> = payload.split_whitespace().collect();
    let mut i = 0;
    'outer: while i < tokens.len() {
        if tokens[i].eq_ignore_ascii_case("git") {
            let mut j = i + 1;
            // Skip global options (-C and -c consume one following value token).
            while j < tokens.len() {
                let t = tokens[j];
                if t.starts_with('-') {
                    j += 1;
                    if (t == "-C" || t == "-c") && j < tokens.len() {
                        j += 1;
                    }
                } else {
                    // First non-option token is the subcommand.
                    let subcore = t.trim_end_matches([';', '&', '|']);
                    if !subcore.eq_ignore_ascii_case("add")
                        && !subcore.eq_ignore_ascii_case("stage")
                    {
                        // Not a git add/stage; resume outer scan past this subcommand.
                        i = j + 1;
                        continue 'outer;
                    }
                    // Found git add/stage. Check argument tokens (after the subcommand).
                    j += 1;
                    while j < tokens.len() {
                        let arg = tokens[j];
                        // Shell chain terminators end this command's argument list.
                        if arg == "&&" || arg == "||" || arg == ";" {
                            i = j + 1;
                            continue 'outer;
                        }
                        // Token with trailing chain operator: content before it is an arg.
                        let arg_core = arg.trim_end_matches([';', '&', '|']);
                        if arg_core.len() < arg.len() {
                            if is_factory_arg_token(arg_core) {
                                return true;
                            }
                            i = j + 1;
                            continue 'outer;
                        }
                        if is_factory_arg_token(arg) {
                            return true;
                        }
                        j += 1;
                    }
                    // Exhausted tokens while scanning add arguments.
                    break 'outer;
                }
            }
            // Inner loop exhausted without finding a subcommand.
            break 'outer;
        } else {
            i += 1;
        }
    }
    false
}

/// Returns true if `token` is a single git add/stage argument that conservatively
/// implies or targets a `.factory/`-rooted path. Called only on tokens confirmed to
/// be in the argument region of a `git add`/`git stage` invocation (not global option
/// values).
///
/// Does NOT handle the `.factory/` literal prefix — that is caught by the fast path
/// in `contains_factory_path_arg`.
///
/// # BC trace
/// BC-4.16.001 Invariant 4 v1.4: conservative argument-level blocking forms.
fn is_factory_arg_token(token: &str) -> bool {
    // Strip surrounding single or double quotes for pathspec-magic analysis.
    // Handles `':/.factory'` and `":/..."` quoted forms.
    let unquoted = token.trim_matches(|c| c == '\'' || c == '"');

    // Bare .factory token without trailing slash: git expands `.factory` to
    // `.factory/**` for staging — same dual-tracking scope as `.factory/`.
    if unquoted == ".factory" {
        return true;
    }

    // :/-family pathspec magic: anchors from repo root; can include .factory/.
    if unquoted.starts_with(":/") {
        return true;
    }

    match token {
        // Conservative bulk-stage flags: may include .factory/ content.
        "-A" | "--all" | "-u" | "--update" | "." => true,
        // "./" is CWD-relative with explicit slash — semantically identical to "."
        // for staging; may stage .factory/** when CWD is the project root.
        "./" => true,
        // Glob wildcards: guard cannot evaluate expansions at PreToolUse time.
        t if t.contains('*') || t.contains('?') || t.starts_with('[') => true,
        // Combined short flags (e.g. "-Au", "-uA"): A=all, u=update.
        t if t.starts_with('-') && !t.starts_with("--") && t.len() > 2 => {
            let flags = &t[1..];
            flags.contains('A') || flags.contains('u')
        }
        _ => false,
    }
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
