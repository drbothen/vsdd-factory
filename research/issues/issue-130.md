# Issue #130 — dispatcher creates recursive `.factory/.factory/logs/` shadow when invoked with cwd inside `.factory/`

**Date:** 2026-06-09
**Issue:** #130 (label: `bug`) — *"bug(dispatcher): creates recursive .factory/.factory/logs/ shadow when invoked with cwd inside .factory/"*
**Validator:** research-agent
**Branch/commit at validation:** `develop` @ `82163b7f`

---

## Restated Question

The factory dispatcher resolves its internal log directory relative to `cwd`/`CLAUDE_PROJECT_DIR` instead of locating the true project root. When `cwd == /project/.factory/` (the documented pattern for state-manager and any agent operating on factory artifacts), it creates `/project/.factory/.factory/logs/` — a recursive shadow — instead of routing to `/project/.factory/logs/`. The dispatcher additionally emits noisy `internal.dispatcher_error` events: `missing field 'event_name'`, `missing field 'session_id'`, and `$CLAUDE_PLUGIN_ROOT is not set`. The proposed fixes: (primary) resolve `.factory/logs/` correctly regardless of invocation cwd by walking up to find an enclosing `.factory/` (or via a `FACTORY_ROOT` env var); (secondary) wire `CLAUDE_PLUGIN_ROOT`; (tertiary) tolerate/rate-limit malformed-payload log entries. Six acceptance criteria, including a regression test under `plugins/vsdd-factory/tests/` and teaching the destructive-op guard to distinguish the legit `.factory/` worktree from the `.factory/.factory/` shadow.

---

## Codebase Grounding

### PRIMARY (recursive shadow) — REPRODUCIBLE; root cause located

`crates/factory-dispatcher/src/main.rs`, `resolve_log_dir()` (lines 669-674):

```rust
fn resolve_log_dir() -> PathBuf {
    match std::env::var(ENV_PROJECT_DIR) {
        Ok(root) if !root.is_empty() => PathBuf::from(root).join(".factory").join("logs"),
        _ => PathBuf::from(".factory").join("logs"),
    }
}
```

Both branches unconditionally append `.factory/logs`. If `CLAUDE_PROJECT_DIR` (or cwd) already points at `/project/.factory`, the result is `/project/.factory/.factory/logs` — exactly the recursive shadow the issue reports. There is even an explicit `TODO(S-2.6)` doc-comment immediately above (lines 661-668) acknowledging the deferral:

> *"TODO(S-2.6): v0.79.x has full git-worktree-aware resolution so the log always lands on the main worktree even when the dispatcher is invoked from a subdir. For v1.0-beta.1 we keep it simple … S-2.6 will replace this with the full resolution used by the existing emit-event bash bin."*

`InternalLog::write_inner` (`crates/factory-dispatcher/src/internal_log.rs:243-264`) does `fs::create_dir_all(&self.log_dir)` — so the shadow dir is *created* on first write. Confirmed reproducible at HEAD.

**Precedent for the correct fix already exists in the repo's history.** `CHANGELOG.md:3664` ("0.70.0 — Worktree-aware log dir") documents that the *bash* `bin/emit-event` resolved the main worktree via `git worktree list --porcelain | awk '/^worktree /{print $2; exit}'` (CHANGELOG lines 3679-3684), landing events in `<main-worktree>/.factory/logs/`, with `VSDD_LOG_DIR` override and graceful git-unavailable fallback. The Rust dispatcher dropped this and left the `TODO(S-2.6)`. So #130 is a known-regressed capability with a documented reference implementation.

### SECONDARY (`$CLAUDE_PLUGIN_ROOT is not set`) — REPRODUCIBLE

`crates/factory-dispatcher/src/main.rs`, `resolve_registry_path()` (lines 655-659):

```rust
let plugin_root = std::env::var(ENV_PLUGIN_ROOT)
    .map_err(|_| anyhow::anyhow!("${ENV_PLUGIN_ROOT} is not set"))?;
```

When `CLAUDE_PLUGIN_ROOT` is unset, `run()` returns this error; `main()` catches it and calls `emit_dispatcher_error(...)` (lines 86-89), writing `internal.dispatcher_error` with the message `$CLAUDE_PLUGIN_ROOT is not set` — matching the issue's third repro line. Also at lines 267-269 + 307-310 the dispatcher silently `unwrap_or_default()`s `CLAUDE_PLUGIN_ROOT` into an empty `PathBuf` for `plugin_root`/`resolvers-registry.toml` — the silent-fallback the issue flags as a production-grade violation.

### TERTIARY (`missing field 'event_name' / 'session_id'`) — PARTIALLY FIXED

`crates/factory-dispatcher/src/payload.rs`:
- `event_name` now carries `#[serde(alias = "hook_event_name")]` (lines 16-24) with a test `accepts_hook_event_name_alias_from_real_harness` (lines 171-185). The doc-comment (lines 16-23) explicitly says this was added because the real harness sends `hook_event_name` and *"surfaced 'missing field event_name' errors on every real harness invocation."* So the **most common** cause of the issue's `missing field 'event_name'` noise is already remediated.
- However, `validate()` (lines 84-88) still returns `PayloadError::MissingField("event_name" | "session_id")` for genuinely empty/missing fields, and `HookPayload::from_reader` surfaces `payload json parse failed: {0}` (line 60). So malformed/legacy payloads still produce these entries; there is **no rate-limiting / once-per-session de-duplication** of identical internal errors (the issue's tertiary ask). The `InternalLog` is append-only best-effort with no dedup (`internal_log.rs:231-264`).

### AC coverage check (issue's 6 ACs)

| AC | Status at HEAD |
|----|----------------|
| 1. logs resolve to real `.factory/logs/` regardless of cwd | **NOT MET** — `resolve_log_dir` appends `.factory/logs` unconditionally |
| 2. `.factory/.factory/` shadow never created | **NOT MET** — created by `create_dir_all` |
| 3. `$CLAUDE_PLUGIN_ROOT not set` eliminated | **NOT MET** — still errored + silent-default fallback |
| 4. malformed payloads logged once/session | **NOT MET** — no dedup/rate-limit |
| 5. test under `tests/` asserting no shadow with `cwd=<project>/.factory/` | **NOT MET** — no such test found |
| 6. destructive-op guard distinguishes `.factory/` worktree vs `.factory/.factory/` shadow | **NOT MET** — guard scope unchanged (`CHANGELOG.md:4731` guards `.factory` recursively) |

Net: the recursive-shadow primary + secondary + tertiary-dedup remain open; only the *common* `hook_event_name` alias cause of the parse-error noise is fixed.

---

## External Research (technical soundness)

Primary-source confirmations (Perplexity deep-research, 2026-06-09):

- **`git worktree list --porcelain` lists the main worktree first**, and the porcelain format is documented stable across git versions ("The main worktree is listed first, followed by each of the linked worktrees"). Combine with `-z` for NUL-terminated safe parsing. This validates the v0.70.0 awk approach and is the canonical way to find the project root for log resolution. — https://git-scm.com/docs/git-worktree , https://man.archlinux.org/man/git-worktree.1.en
- **`git rev-parse --git-common-dir`** distinguishes a linked worktree (points at the main repo's shared git dir) from the main worktree — a robust alternative to path-walking for locating the canonical root. — https://git-scm.com/docs/git-rev-parse
- **Upward `.factory`-basename walk gotchas:** on macOS (APFS) / Windows (NTFS) case-insensitive filesystems, `Path::file_name()` comparisons are case-sensitive in Rust on every platform, and symlinked parents can create loops. A robust walk should compare case-insensitively where appropriate and guard against symlink cycles (inode/device tracking). — https://doc.rust-lang.org/std/path/index.html , https://doc.rust-lang.org/std/fs/fn.symlink_metadata.html , https://blobfolio.com/2021/faster-path-canonicalization-rust/
- **Idempotent / silent-failure principle:** the issue correctly ties the silent `CLAUDE_PLUGIN_ROOT` fallback to the broader "fail loud" canonicalization principle (issue cross-refs #129); confirmed best practice is fail-loud-with-diagnostic, not silent default.

The proposed walk-up-to-`.factory` (issue pseudocode) is sound BUT the **git-worktree-aware** resolution (v0.70.0 reference impl) is strictly more correct for this repo's two-worktree model, because it lands logs on the *main* worktree even when invoked from a linked worktree — which the naive walk-up does not guarantee. Recommend a hybrid: prefer `CLAUDE_PROJECT_DIR`/`FACTORY_ROOT` if it already ends in `.factory` (don't re-append), else walk up to an enclosing `.factory`, else fall back to git-worktree main-root resolution, else cwd.

---

## Verdict

> **VALID-PARTIAL** — Confidence: **High**
>
> The PRIMARY recursive-shadow bug is reproducible and root-caused to `resolve_log_dir()` (`main.rs:669-674`), with an explicit `TODO(S-2.6)` confirming the deferral and a documented v0.70.0 reference implementation. The SECONDARY `$CLAUDE_PLUGIN_ROOT is not set` error + silent-default fallback are present (`main.rs:655-659, 267-269`). The TERTIARY noise is **partially addressed** — the `#[serde(alias = "hook_event_name")]` fix (`payload.rs:16-24`) already eliminates the most common `missing field 'event_name'` cause, but per-session de-duplication / rate-limiting of internal errors is NOT implemented. None of the 6 ACs is fully met; AC-1/2/3/5/6 are open. Residual: implement worktree-aware (non-re-appending) log-dir resolution, fail-loud `CLAUDE_PLUGIN_ROOT` handling, internal-error dedup, the regression test, and the destructive-guard shadow-vs-worktree distinction.

---

## Recommended Approach (zero re-research)

**Route to:** `architect` for the resolution-strategy decision (cross-component: dispatcher + destructive-guard hook + env-wiring), then `implementer` for the dispatcher change (TDD), `test-writer` for the regression test, `devops-engineer`/`dx-engineer` for the `CLAUDE_PLUGIN_ROOT` install-time wiring. This is a `crates/` dispatcher change → requires a RELEASE (rc tag) to reach the operator-level cache (CLAUDE.md "Dispatcher binary discipline").

**Key files to touch:**
1. `crates/factory-dispatcher/src/main.rs`:
   - `resolve_log_dir()` (lines 669-674): replace with non-re-appending, worktree-aware resolution. Order: (a) explicit `VSDD_LOG_DIR`/`FACTORY_ROOT` override; (b) if `CLAUDE_PROJECT_DIR`/cwd basename is already `.factory`, use it directly (do NOT append `.factory` again); (c) walk up to an enclosing `.factory` dir (case-insensitive basename compare, symlink-loop guarded); (d) git-worktree-main-root resolution (`git worktree list --porcelain`, first entry) → `<main>/.factory/logs`; (e) fall back to `./.factory/logs`. Retire the `TODO(S-2.6)`.
   - `resolve_registry_path()` (lines 655-659) + `plugin_root` defaulting (lines 267-269, 307-310): fail loud with an actionable diagnostic when `CLAUDE_PLUGIN_ROOT` is unset; consider resolving from the known MP plugin-cache install path as a fallback (issue secondary option).
2. `crates/factory-dispatcher/src/internal_log.rs`: add per-session de-duplication / rate-limiting for identical `internal.dispatcher_error` messages (issue tertiary AC-4). Keep best-effort/non-panicking contract.
3. Destructive-op guard (bash hook, see `CHANGELOG.md:4731` + `plugins/vsdd-factory/hooks/*.sh`): teach it to permit deletion of the `.factory/.factory/` shadow while continuing to guard the real `.factory/` worktree (issue AC-6).
4. New test: `crates/factory-dispatcher/tests/` (or `plugins/vsdd-factory/tests/` bats) — invoke dispatcher with `cwd`/`CLAUDE_PROJECT_DIR = <project>/.factory/` and assert NO `.factory/.factory/` is created and the log lands in `<project>/.factory/logs/` (issue AC-5).

**Approach:**
- Reuse the v0.70.0 algorithm (CHANGELOG lines 3679-3684) as the worktree-main-root resolver; the awk first-`worktree`-entry rule is documented-stable (git-scm). Implement in Rust with a `git worktree list --porcelain` subprocess OR `git rev-parse --git-common-dir` parent. Guard the no-git / not-a-repo case with the cwd fallback.
- The non-re-append guard (basename already `.factory`) is the single change that fixes the recursive shadow; the worktree-aware part additionally fixes log fragmentation across worktrees.

**Risks:**
- Spawning `git` from the dispatcher adds latency + a `git`-availability dependency; gate behind the override-then-walk-up path so the git call is last-resort. Keep best-effort: log-dir resolution must never panic or block dispatch.
- Changing destructive-guard semantics is security-sensitive — must NOT weaken protection of the real `.factory/`. Scope the shadow exception to the exact `.factory/.factory/` path.
- Dispatcher change requires an rc release to land at operator level (develop edits don't affect the cached plugin).

**Test strategy:**
- Unit: `resolve_log_dir` table tests — `CLAUDE_PROJECT_DIR` = `/p`, `/p/.factory`, unset+cwd inside `.factory`, override set; assert no double-`.factory`.
- Integration: AC-5 cwd-inside-`.factory` no-shadow assertion.
- Guard: bats test that `.factory/.factory/` is deletable while `.factory/` deletion is still blocked.
- Regression: assert `internal.dispatcher_error` dedup (N identical → 1 logged).

**Dependencies:**
- Related to **#129** (production-grade canonicalization / fail-loud) per the issue's "Related" note — the silent `CLAUDE_PLUGIN_ROOT` fallback is an instance of that principle.
- Independent of #128/#169/#176 (different subsystem: dispatcher path-resolution vs PR/worktree-review process).

---

## Research Methods

| Tool | Queries | Purpose |
|------|---------|---------|
| **Perplexity perplexity_research (PRIMARY)** | 1 (shared across #128/#130/#169/#176) | git worktree main-root detection, path canonicalization / case-insensitive basename gotchas, symlink-loop guarding |
| Read | 3 | main.rs, internal_log.rs, CHANGELOG.md (0.70.0 section) |
| Grep | 3 | resolve_log_dir / CLAUDE_PLUGIN_ROOT / recursive-shadow / payload-parse across crates + CHANGELOG |
| Glob | 1 | crates enumeration |
| Training data | 0 areas | All path/git claims externally sourced; code claims by direct read |

**Total MCP tool calls:** 1 (deep research, shared)
**Training data reliance:** Low — resolution-strategy claims verified against git-scm.com + rust-lang.org; bug reproducibility verified by direct reading of `main.rs` / `internal_log.rs` / `payload.rs` with line cites and the v0.70.0 CHANGELOG reference implementation.
