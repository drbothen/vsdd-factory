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

/// Returns true if the Bash payload string contains a `git add` command.
///
/// Matches the literal substring `git add` (case-insensitive per BC-4.16.001
/// Precondition 2). All other commands — `git commit`, `git push`, etc. — are
/// not in scope and this function returns false for them.
///
/// # BC trace
/// BC-4.16.001 Precondition 2: detect `git add` by substring match.
/// BC-4.16.001 PC4: non-`git add` commands pass unconditionally.
pub fn is_git_add_command(payload: &str) -> bool {
    todo!()
}

/// Returns true if the `git add` argument text contains or implies a
/// `.factory/`-rooted path that should be blocked on a product branch.
///
/// Conservative matching per BC-4.16.001 Invariant 4:
/// - Literal `.factory/` prefix match.
/// - `/.factory/` as a path component (absolute paths).
/// - `-A`, `-u`, and `.` flags from a `.factory/`-adjacent CWD are treated
///   as potentially staging `.factory/` content and blocked conservatively.
///
/// # BC trace
/// BC-4.16.001 Invariant 4: path matching is conservative.
/// BC-4.16.001 EC-004: `git add -A` from CWD under `.factory/` is blocked.
/// BC-4.16.001 EC-008: `git add *.md` glob from project root is blocked.
pub fn contains_factory_path_arg(git_add_args: &str) -> bool {
    todo!()
}

/// Returns true if `branch` is a product branch (not `factory-artifacts`).
///
/// Product branches: `develop`, `main`, `feature/*`, `release/*`,
/// `maintenance/*`, and any unrecognized branch name (conservative default).
/// The only non-blocking branch is `factory-artifacts`.
///
/// # BC trace
/// BC-4.16.001 PC3: `factory-artifacts` branch passes unconditionally.
/// BC-4.16.001 PC1: all other branches are product branches.
pub fn is_product_branch(branch: &str) -> bool {
    todo!()
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
    callbacks: HookCallbacks<B, E, L>,
) -> HookResult
where
    B: FnOnce(&str, &[&str]) -> Result<(i32, String, String), String>,
    E: FnMut(&str, &[(&str, &str)]),
    L: FnMut(u8, &str),
{
    todo!()
}

// ---------------------------------------------------------------------------
// Top-level entry point (wired to real host fns in main.rs)
// ---------------------------------------------------------------------------

/// Called from the WASI entry point in `main.rs`.
///
/// Wires the real vsdd_hook_sdk host functions to the injectable-callback
/// surface of `hook_logic`.
pub fn on_pre_tool_use(payload: HookPayload) -> HookResult {
    todo!()
}

// ---------------------------------------------------------------------------
// Unit tests (Red Gate — all must fail until implementation is complete)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests;
