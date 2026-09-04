//! validate-factory-path-staged — PostToolUse WASM hook plugin.
//!
//! The **post-hoc detective mirror** of `validate-factory-path-staging`
//! (BC-4.16.001). Fires on every completed `PostToolUse` event for the
//! `Bash` tool and, **unconditionally — with no command-text pre-filter of
//! any kind** (BROAD trigger scope, ratified 2026-09-04; final, not
//! provisional), inspects the actual git index and current-branch state via
//! two `host::exec_subprocess` calls: `git diff --cached --name-only` and
//! `git branch --show-current`.
//!
//! This plugin does **not** validate the *content* of a `.factory/`
//! artifact write; like its sibling, it is a narrow git-staging exclusivity
//! guard (INV-E21-001, CAP-034), applied post-hoc. Its second, equally
//! load-bearing purpose (S-25.04 AC-001) is to give S-25.01's durable-marker
//! + next-advance-gate mechanism (BC-1.18.001) a **structurally-reachable
//! PostToolUse trigger path** — a capability `validate-factory-path-staging`'s
//! own PreToolUse-only registration can never provide (BC-1.18.001
//! Invariant 4; BC-1.18.004 Postcondition 4).
//!
//! # Behavioral Contracts
//!
//! - BC-4.16.002 v1.0: PC1 detect `.factory/` path staged on a product
//!   branch (block); PC2 pass (no `.factory/` path staged, or branch is
//!   `factory-artifacts`); PC3 INDETERMINATE trigger on this plugin's own
//!   cannot-complete (fuel/epoch/OutputTooLarge) — reaches
//!   `write_indeterminate_marker` verbatim (S-25.01 machinery, REUSE-
//!   UNCHANGED, Invariant 5); PC4 fail-open on branch-detection failure
//!   (mirrors BC-4.16.001 Invariant 3 exactly); PC5 advisory-only on a
//!   non-resource-exhaustion crash (`on_error = "continue"`).
//!
//! # Architecture compliance
//!
//! - HOST_ABI_VERSION = 1 (no new host functions introduced).
//! - No dependency on factory-dispatcher or other workspace crates
//!   (forbidden) — the `.factory/` path-matching predicate is reused per
//!   BC-4.16.001 Invariant 4's ALGORITHM (case-insensitive `.factory/`
//!   prefix / path-component match), not via a cross-hook-plugin-crate
//!   dependency; the implementer keeps the two crates' predicates in sync
//!   by construction (same algorithm, independently compiled WASM binaries).
//! - POLICY 21: native WASM only — no `.sh` files introduced.
//! - Pure `fn hook_logic(...)` takes all host I/O as injectable closures.
//!   Unit tests exercise every branch without a WASM runtime.
//!
//! # BC-5.38.001 Red Gate discipline
//!
//! All non-trivial function bodies use `todo!()`. Implementer fills in real
//! logic. See the stub commit report for the GREEN-BY-DESIGN exception
//! (`is_product_branch`) and confirmation that no WIRING-EXEMPT bodies were
//! introduced.

use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// ABI version constant (BC-4.16.002 architecture compliance)
// ---------------------------------------------------------------------------

/// HOST_ABI_VERSION declares the ABI contract version this plugin was built
/// against. The dispatcher reads this before any host call. Must remain 1.
pub const HOST_ABI_VERSION: u32 = 1;

// ---------------------------------------------------------------------------
// Log level constants (injectable `log` callback level parameter)
// ---------------------------------------------------------------------------

/// Named constants for the `level: u8` parameter of the injectable `log`
/// callback in `HookCallbacks`. Matches the semantic mapping documented on
/// the `log` field: 0=trace, 1=debug, 2=info, 3=warn, 4=error.
pub mod log_level {
    pub const TRACE: u8 = 0;
    pub const DEBUG: u8 = 1;
    pub const INFO: u8 = 2;
    pub const WARN: u8 = 3;
    pub const ERROR: u8 = 4;
}

// ---------------------------------------------------------------------------
// Canonical error / block code (BC-4.16.002 PC1)
// ---------------------------------------------------------------------------

/// Canonical block/error code for a detected `.factory/` path staged on a
/// product branch after a completed Bash command (BC-4.16.002 PC1 "Error
/// variant: `FactoryPathStagedOnProductBranch`"). Passed as the `code`
/// argument to `HookResult::block_with_fix` — mirrors the sibling's own
/// `"FactoryPathOnProductBranch"` code convention (BC-4.16.001), with the
/// `Staged` distinction preserving the two plugins' independent identity in
/// telemetry and block messages (past participle "staged" vs. gerund
/// "staging" — architect F2 §2.1 naming rationale).
pub const FACTORY_PATH_STAGED_ON_PRODUCT_BRANCH: &str = "FactoryPathStagedOnProductBranch";

// ---------------------------------------------------------------------------
// Pure-core predicates and helpers (injectable-testable, no host I/O)
// ---------------------------------------------------------------------------

/// Returns `true` if `path` is a `.factory/`-rooted path.
///
/// Matches a literal `.factory/` path prefix or `.factory/` as an interior
/// path component, **case-insensitively** (e.g. `.Factory/STATE.md`
/// matches) — reusing BC-4.16.001 Invariant 4's `.factory/` path-matching
/// ALGORITHM verbatim (architect F2 §2.1: "pure and directly reusable, not
/// merely pattern-mirrored"). This crate does not depend on the sibling
/// crate directly (forbidden: no cross-hook-plugin-crate dependency — see
/// module-level compliance notes); the implementer keeps this predicate's
/// observable behavior byte-identical to
/// `validate_factory_path_staging::contains_factory_path_arg`'s fast-path
/// semantics.
///
/// Called once per staged path returned by `git diff --cached --name-only`
/// (BC-4.16.002 Precondition 2 — BROAD, unconditional scan; no
/// command-text pre-filter).
///
/// # BC trace
/// BC-4.16.002 PC1 / Invariant 4: case-insensitive `.factory/` path-matching
/// predicate, reused verbatim from BC-4.16.001.
///
/// # BC-5.38.001
/// Non-trivial (case-folding + path-component matching). Uses `todo!()`.
pub fn is_factory_path(path: &str) -> bool {
    // Reused verbatim from `validate_factory_path_staging::contains_factory_path_arg`'s
    // fast-path semantics: case-insensitive `.factory/` literal anywhere in the path.
    // Matches both a leading `.factory/` prefix and an interior `/.factory/` path
    // component in a single check (BC-4.16.001 Invariant 4).
    path.to_ascii_lowercase().contains(".factory/")
}

/// Returns `true` if `branch` is a product branch (not `factory-artifacts`).
///
/// Mirrors `validate_factory_path_staging::is_product_branch`
/// (BC-4.16.001) verbatim: the only non-blocking branch is
/// `factory-artifacts`; every other branch name (including unrecognized
/// ones) is conservatively treated as a product branch.
///
/// # BC trace
/// BC-4.16.002 PC2 / EC-006: `factory-artifacts` branch passes
/// unconditionally.
/// BC-4.16.002 PC1: all other branches are product branches.
///
/// # BC-5.38.001 GREEN-BY-DESIGN
/// Zero branching, no I/O, no non-trivial helpers, 1 line — correct by
/// construction. See stub commit report GREEN-BY-DESIGN table.
pub fn is_product_branch(branch: &str) -> bool {
    branch != "factory-artifacts"
}

/// Scans the newline-delimited stdout of `git diff --cached --name-only`
/// and returns the first staged path that matches `is_factory_path`, if
/// any.
///
/// This is the BROAD-scope unconditional check (BC-4.16.002 Precondition 2
/// / Invariant 7): called on **every** completed `PostToolUse ^Bash$`
/// dispatch regardless of the triggering command's own text, since the
/// check runs against actual index state, not payload text.
///
/// # BC trace
/// BC-4.16.002 PC1: detect at least one `.factory/`-matching staged path.
/// BC-4.16.002 PC2 / EC-002 / EC-007: `None` when no staged path matches.
///
/// # BC-5.38.001
/// Non-trivial (line-splitting + delegates to the non-trivial
/// `is_factory_path` helper — BC-5.38.005 self-check: a test would pass
/// trivially without implementer work if this had a real body). Uses
/// `todo!()`.
pub fn find_staged_factory_path(git_diff_cached_name_only_stdout: &str) -> Option<String> {
    for line in git_diff_cached_name_only_stdout.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if is_factory_path(trimmed) {
            return Some(trimmed.to_string());
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Injectable callback surface (testable without WASM runtime)
// ---------------------------------------------------------------------------

/// All side-effecting callbacks injected into `hook_logic` for testability.
/// In production (main.rs / `on_post_tool_use`), these are wired to host
/// functions.
///
/// `exec_subprocess` is `FnMut` (not `FnOnce`, unlike the sibling's single
/// branch-detection call) because `hook_logic` issues it TWICE per
/// invocation — once for `git diff --cached --name-only` and once for
/// `git branch --show-current` (BC-4.16.002 Precondition 3).
pub struct HookCallbacks<B, E, L>
where
    B: FnMut(&str, &[&str]) -> Result<(i32, String, String), String>,
    E: FnMut(&str, &[(&str, &str)]),
    L: FnMut(u8, &str),
{
    /// Execute a subprocess (binary, args); returns (exit_code, stdout, stderr).
    /// Called twice per invocation: staged-path listing, then current-branch
    /// detection (BC-4.16.002 Precondition 3).
    pub exec_subprocess: B,
    /// Emit a structured event (type, fields).
    pub emit_event: E,
    /// Log a message at the given level. Use the `log_level` module constants
    /// (`log_level::TRACE`=0, `log_level::DEBUG`=1, `log_level::INFO`=2,
    /// `log_level::WARN`=3, `log_level::ERROR`=4).
    pub log: L,
}

// ---------------------------------------------------------------------------
// Core hook logic (injectable callbacks — testable without WASM runtime)
// ---------------------------------------------------------------------------

/// Core validate-factory-path-staged hook logic.
///
/// All host I/O is injected via `callbacks` so unit tests can exercise every
/// branch without a WASM runtime.
///
/// Algorithm (BC-4.16.002 Preconditions 1-4, Postconditions PC1-PC5):
/// 1. Unconditionally issue `git diff --cached --name-only` via
///    `exec_subprocess` — no command-text pre-filter of `payload` is
///    applied (BROAD scope, Precondition 2, Invariant 7). If this call
///    cannot complete (WASM resource exhaustion), the DISPATCHER — not
///    this function — classifies the outcome INDETERMINATE and reaches
///    `write_indeterminate_marker` (PC3); that classification happens
///    outside this pure-core function's control flow, at the WASM
///    host boundary.
/// 2. Scan the stdout via `find_staged_factory_path` for a `.factory/`-
///    matching staged path.
/// 3. Unconditionally issue `git branch --show-current` via
///    `exec_subprocess`. On failure (non-zero exit, empty stdout, detached
///    HEAD) — fail open (PC4), mirroring BC-4.16.001 Invariant 3 exactly.
/// 4. If no `.factory/` path was staged, OR the detected branch is
///    `factory-artifacts` (`is_product_branch` returns `false`) — pass
///    (PC2).
/// 5. Otherwise — a `.factory/` path is staged AND the branch is a product
///    branch — block with `FACTORY_PATH_STAGED_ON_PRODUCT_BRANCH` (PC1).
///    The already-staged path is NOT automatically unstaged (Postcondition
///    PC1 item 3 — detective, not preventive; Invariant 6).
///
/// # BC traces
/// - BC-4.16.002 PC1: block `.factory/` staging on product branches
/// - BC-4.16.002 PC2: pass when nothing relevant staged, or on
///   `factory-artifacts`
/// - BC-4.16.002 PC3: INDETERMINATE trigger — dispatcher-side, not this
///   function's own control flow (Invariant 5: reuse-not-reimplementation)
/// - BC-4.16.002 PC4: fail-open on branch-detection failure (Invariant 3)
/// - BC-4.16.002 PC5: advisory-only on non-resource-exhaustion crash
///   (`on_error = "continue"`, handled dispatcher-side, not here)
/// - BC-4.16.002 Invariant 7: BROAD unconditional trigger scope, no
///   command-text pre-filter
///
/// # BC-5.38.001
/// Non-trivial (branching + calls to non-trivial helpers + I/O via
/// injected callbacks). Uses `todo!()`.
pub fn hook_logic<B, E, L>(
    _payload: HookPayload,
    mut callbacks: HookCallbacks<B, E, L>,
) -> HookResult
where
    B: FnMut(&str, &[&str]) -> Result<(i32, String, String), String>,
    E: FnMut(&str, &[(&str, &str)]),
    L: FnMut(u8, &str),
{
    // Step 1 (BROAD, unconditional — Precondition 2 / Invariant 7): list staged
    // paths. No inspection of `_payload`'s command text — the check runs against
    // actual git index state, not payload text, on every completed dispatch.
    let diff_result = (callbacks.exec_subprocess)("git", &["diff", "--cached", "--name-only"]);

    let staged_factory_path: Option<String> = match diff_result {
        Ok((exit_code, stdout, stderr)) => {
            if exit_code != 0 {
                (callbacks.log)(
                    log_level::WARN,
                    &format!(
                        "validate-factory-path-staged: git diff --cached --name-only \
                         returned exit {exit_code} (stderr: {stderr})"
                    ),
                );
                None
            } else {
                find_staged_factory_path(&stdout)
            }
        }
        Err(e) => {
            (callbacks.log)(
                log_level::WARN,
                &format!(
                    "validate-factory-path-staged: git diff --cached --name-only failed \
                     ({e})"
                ),
            );
            None
        }
    };

    // Step 2 (BROAD, unconditional — Precondition 3): detect the current branch.
    // Always issued, regardless of the Step 1 outcome (Invariant 7).
    let branch_result = (callbacks.exec_subprocess)("git", &["branch", "--show-current"]);

    // Step 3: fail-open on branch-detection failure (PC4 / Invariant 3).
    let branch = match branch_result {
        Ok((exit_code, stdout, stderr)) => {
            if exit_code != 0 {
                (callbacks.log)(
                    log_level::WARN,
                    &format!(
                        "validate-factory-path-staged: branch detection returned exit \
                         {exit_code} (stderr: {stderr}), failing open per PC4/Invariant 3"
                    ),
                );
                return HookResult::Continue;
            }
            let b = stdout.trim().to_string();
            if b.is_empty() {
                // Empty stdout = detached HEAD state — fail-open per Invariant 3.
                (callbacks.log)(
                    log_level::WARN,
                    "validate-factory-path-staged: empty branch output (detached HEAD?), \
                     failing open per PC4/Invariant 3",
                );
                return HookResult::Continue;
            }
            b
        }
        Err(e) => {
            // git unavailable or exec failure — fail-open per Invariant 3.
            (callbacks.log)(
                log_level::WARN,
                &format!(
                    "validate-factory-path-staged: branch detection failed ({e}), failing \
                     open per PC4/Invariant 3"
                ),
            );
            return HookResult::Continue;
        }
    };

    // Step 4: PC2 — nothing relevant staged.
    let staged_path = match staged_factory_path {
        Some(p) => p,
        None => return HookResult::Continue,
    };

    // Step 4b: PC2 — factory-artifacts branch passes unconditionally (EC-006).
    if !is_product_branch(&branch) {
        return HookResult::Continue;
    }

    // Step 5: PC1 — a `.factory/` path is staged AND the branch is a product
    // branch. Block. The already-staged path is NOT automatically unstaged
    // (PC1 item 3 — detective, not preventive; Invariant 6).
    let safe_branch: String = branch
        .chars()
        .filter(|c| c.is_ascii_graphic() || *c == ' ')
        .collect();
    let safe_path: String = staged_path
        .chars()
        .filter(|c| c.is_ascii_graphic() || *c == ' ')
        .collect();

    (callbacks.emit_event)(
        "hook.block",
        &[
            ("hook", "validate-factory-path-staged"),
            ("code", FACTORY_PATH_STAGED_ON_PRODUCT_BRANCH),
            ("branch", &branch),
            ("path", &staged_path),
        ],
    );

    HookResult::block_with_fix(
        "validate-factory-path-staged",
        format!(
            "DETECTED: .factory/ path staged on product branch '{safe_branch}' (post-hoc \
             check). .factory/ paths are exclusively owned by the factory-artifacts \
             worktree. A staging operation reached the git index without being \
             intercepted by validate-factory-path-staging's PreToolUse guard (git \
             plumbing, alias, wrapper script, or under-matched invocation text). Staged \
             path: '{safe_path}'"
        ),
        "Unstage immediately: git restore --staged <path> (or equivalent), or switch to \
         the .factory/ worktree and commit from there on the factory-artifacts branch",
        FACTORY_PATH_STAGED_ON_PRODUCT_BRANCH,
    )
}

// ---------------------------------------------------------------------------
// Top-level entry point (wired to real host fns in main.rs)
// ---------------------------------------------------------------------------

/// Called from the WASI entry point in `main.rs`.
///
/// Wires the real vsdd_hook_sdk host functions to the injectable-callback
/// surface of `hook_logic`. Left as `todo!()` per BC-5.38.001 Red Gate
/// discipline — implementer wires `vsdd_hook_sdk::host::exec_subprocess`,
/// `host::emit_event`, and `host::log_*` exactly as the sibling crate's
/// `on_pre_tool_use` does, adjusted for the two-call (staged-paths +
/// branch) `exec_subprocess` usage this plugin requires.
pub fn on_post_tool_use(payload: HookPayload) -> HookResult {
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
                0..=log_level::INFO => vsdd_hook_sdk::host::log_info(msg),
                log_level::WARN => vsdd_hook_sdk::host::log_warn(msg),
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
