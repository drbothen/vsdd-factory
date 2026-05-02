# Demo Evidence Report — S-8.05: Native port: validate-pr-review-posted

**Story:** S-8.05 — Native port: validate-pr-review-posted (SubagentStop)
**Branch:** feature/S-8.05-native-port-validate-pr-review-posted
**Latest commit:** c5b6e81
**Date recorded:** 2026-05-02
**BC anchors:** BC-7.04.040, BC-7.04.041, BC-7.04.042, BC-7.04.043, BC-7.04.044, BC-2.02.012
**Toolchain:** rustc 1.95.0 (59807616e 2026-04-14)

---

## Coverage Summary

| AC | Description | Evidence File | Recording | Status |
|----|-------------|---------------|-----------|--------|
| AC-001 | WASM crate built; registry migrated to native .wasm; priority=950, on_error=continue, timeout_ms=5000; no script_path, no exec_subprocess | [AC-1.md](AC-1.md) | [gif](AC-001-registry-migration.gif) | PASS |
| AC-002 | hooks.json entry positively absent (grep verified); validate-pr-review-posted.sh deleted | [AC-2.md](AC-2.md) | [gif](AC-002-bash-deleted-hooksjson-absent.gif) | PASS |
| AC-003 | Non-pr-reviewer agent exits 0 immediately; BC-2.02.012 Postcondition 5 canonical fallback chain; g.1 primary arm; g.2 subagent_name fallback | [AC-3.md](AC-3.md) | [gif](AC-003-agent-scoping-bc2-02-012.gif) | PASS |
| AC-004 | Check 1: pr-review.md not written → accumulate error; raw string regex; case-sensitive | [AC-4.md](AC-4.md) | [gif](AC-004-check1-pr-review-md.gif) | PASS |
| AC-005 | Check 2: gh pr comment detected → accumulate error; substring containment | [AC-5.md](AC-5.md) | [gif](AC-005-check2-gh-pr-comment.gif) | PASS |
| AC-006 | Check 3a/3b: no formal review / no verdict → accumulate error; advisory block-mode: hook.block emit + stderr; 3-line remediation block; HookResult::Continue always | [AC-6.md](AC-6.md) | [gif](AC-006-check3-advisory-block.gif) | PASS |
| AC-007 | 14 bats parity tests (3 AC-001, 2 AC-002, 7 AC-007 cases + g.1/g.2); BC-2.02.012 Postcondition 6 result fallback chain; 14 unit tests | [AC-7.md](AC-7.md) | [gif](AC-007-bats-parity-tests.gif) | PASS |
| AC-008 | host::emit_event replaces bin/emit-event; bare statement form (no let _ =); bin/emit-event not removed (E-8 D-10) | [AC-8.md](AC-8.md) | [gif](AC-008-emit-event-host-fn.gif) | PASS |

All 8 acceptance criteria: **PASS**. Zero regressions.

---

## Test Counts

| Suite | Count | Result |
|-------|-------|--------|
| Rust unit tests (`cargo test -p validate-pr-review-posted`) | 14 | 14 PASS |
| Bats integration tests (`tests/integration/E-8-hook-plugins/validate-pr-review-posted.bats`) | 14 | 14 PASS |
| **Total** | **28** | **28 PASS / 0 FAIL** |

---

## AC-001: WASM crate built; registry migration complete

Crate at `crates/hook-plugins/validate-pr-review-posted/` with `Cargo.toml` targeting
`wasm32-wasip1`. Entry-point pattern: `[lib]` (hook logic) + `[[bin]]` (trampoline)
mirroring `capture-commit-activity` sibling. Dependencies: `vsdd-hook-sdk = { path = "../../hook-sdk" }`,
`serde_json`, `regex`.

Registry entry (`hooks-registry.toml`):
- `event = "SubagentStop"`, `priority = 950`, `on_error = "continue"`, `timeout_ms = 5000`
- `plugin = "hook-plugins/validate-pr-review-posted.wasm"` (no `script_path`)
- No `binary_allow`, no `exec_subprocess`, no `env_allow` — native WASM makes no subprocess calls

See [AC-1.md](AC-1.md).

---

## AC-002: .sh deleted; hooks.json entries removed

`plugins/vsdd-factory/hooks/validate-pr-review-posted.sh` deleted.
Positive verification: `grep -r validate-pr-review-posted plugins/vsdd-factory/hooks/hooks.json*`
returns zero results across all 6 files (template + 5 platform variants).
Native WASM plugins route via `hooks-registry.toml` only (E-8 D-7 / DRIFT-004).

See [AC-2.md](AC-2.md).

---

## AC-003: Agent scoping + BC-2.02.012 typed projection

`validate_pr_review_logic` exits `HookResult::Continue` immediately when the resolved agent
does not contain `pr-reviewer`, `pr_reviewer`, or `pr-review-triage` (substring containment,
not glob or regex — parity with bash `case "*pr-reviewer*|*pr_reviewer*|*pr-review-triage*"`).

Agent identity resolved via BC-2.02.012 Postcondition 5:
```rust
let agent: &str = payload.agent_type.as_deref()
    .or(payload.subagent_name.as_deref())
    .unwrap_or("unknown");
```

Result content resolved via BC-2.02.012 Postcondition 6:
```rust
let result: &str = payload.last_assistant_message.as_deref()
    .or(payload.result.as_deref())
    .unwrap_or("");
```

Both chains use `Option<String>` + `#[serde(default)]` — JSON null deserializes to `None`,
providing jq `//` null-as-advance semantics for free (BC-2.02.012 Invariant 3).
No `envelope.get(...)` anywhere in the crate.

Bats g.1 and g.2 sub-cases verify primary and fallback arms respectively (tests 13 and 14).

See [AC-3.md](AC-3.md).

---

## AC-004: Check 1 — pr-review.md written

Pattern: `r"pr-review\.md|wrote.*review|review.*written|Write.*pr-review"` (raw string, case-sensitive).
Accumulates: `"pr-review.md may not have been written to .factory/code-delivery/"`.
Four pattern paths cover: literal filename, `wrote.*review`, `review.*written`, Write tool call.

See [AC-4.md](AC-4.md).

---

## AC-005: Check 2 — gh pr comment fallback

Substring check: `result.contains("gh pr comment")`.
Accumulates: `"Used 'gh pr comment' instead of 'gh pr review' — findings won't show as a formal review verdict"`.
Fires independently of Check 1 — all errors accumulated before the single block emit.

See [AC-5.md](AC-5.md).

---

## AC-006: Check 3a/3b; advisory block-mode; error accumulation; remediation block

**Check 3a:** Pattern `r"gh pr review|pr review.*posted|review.*posted.*GitHub|APPROVE|REQUEST_CHANGES"`.
Accumulates: `"No evidence that a formal GitHub review was posted via 'gh pr review'"`.

**Check 3b:** Guard on literal `gh pr review` token (NOT a re-test of Check 3a's full disjunction)
+ absence of `r"approve|request-changes|APPROVE|REQUEST_CHANGES"`.
Accumulates: `"Review posted but no verdict (--approve or --request-changes) detected"`.

**Advisory block-mode:** When errors non-empty:
1. `host::emit_event("hook.block", [...])` — bare statement form
2. Formatted error list + 3-line remediation block written to stderr
3. Return `HookResult::Continue` (exit 0 to Claude Code)

This preserves the `on_error = "continue"` registry semantics while communicating the
block via the event log and stderr (advisory block-mode per story spec).

See [AC-6.md](AC-6.md).

---

## AC-007: Bats parity tests (14/14)

14 bats tests in `tests/integration/E-8-hook-plugins/validate-pr-review-posted.bats`.
Route through `target/release/factory-dispatcher` — the dispatcher provides all WASM host functions.
Dispatcher-internal log grepped for `hook.block reason=pr_review_not_posted` assertions.

Case coverage: (a) all-pass, (b) check1 error, (c) check2 error, (d) check3a error,
(e) check3b error, (f) multi-error accumulation, (g) non-pr-reviewer exit, (g.1) primary
BC-2.02.012 chain arm, (g.2) subagent_name fallback chain arm.

Plus 3 AC-001 registry checks and 2 AC-002 deletion checks run as bats tests.

See [AC-7.md](AC-7.md).

---

## AC-008: host::emit_event; bin/emit-event preserved

`vsdd_hook_sdk::host::emit_event` used as bare statement — no `let _ =`, no `?`, no `unwrap()`.
Returns `()` per `host.rs:53` — there is no Result.
`bin/emit-event` binary preserved at repo root (E-8 D-10; removal deferred to S-8.29).
No `bin/emit-event` reference anywhere in `crates/hook-plugins/validate-pr-review-posted/`.

See [AC-8.md](AC-8.md).

---

## Recordings Index

| File | AC | Format |
|------|----|--------|
| `AC-001-registry-migration.gif` | AC-001 | GIF |
| `AC-001-registry-migration.webm` | AC-001 | WebM |
| `AC-001-registry-migration.tape` | AC-001 | VHS source |
| `AC-002-bash-deleted-hooksjson-absent.gif` | AC-002 | GIF |
| `AC-002-bash-deleted-hooksjson-absent.webm` | AC-002 | WebM |
| `AC-002-bash-deleted-hooksjson-absent.tape` | AC-002 | VHS source |
| `AC-003-agent-scoping-bc2-02-012.gif` | AC-003 | GIF |
| `AC-003-agent-scoping-bc2-02-012.webm` | AC-003 | WebM |
| `AC-003-agent-scoping-bc2-02-012.tape` | AC-003 | VHS source |
| `AC-004-check1-pr-review-md.gif` | AC-004 | GIF |
| `AC-004-check1-pr-review-md.webm` | AC-004 | WebM |
| `AC-004-check1-pr-review-md.tape` | AC-004 | VHS source |
| `AC-005-check2-gh-pr-comment.gif` | AC-005 | GIF |
| `AC-005-check2-gh-pr-comment.webm` | AC-005 | WebM |
| `AC-005-check2-gh-pr-comment.tape` | AC-005 | VHS source |
| `AC-006-check3-advisory-block.gif` | AC-006 | GIF |
| `AC-006-check3-advisory-block.webm` | AC-006 | WebM |
| `AC-006-check3-advisory-block.tape` | AC-006 | VHS source |
| `AC-007-bats-parity-tests.gif` | AC-007 | GIF |
| `AC-007-bats-parity-tests.webm` | AC-007 | WebM |
| `AC-007-bats-parity-tests.tape` | AC-007 | VHS source |
| `AC-008-emit-event-host-fn.gif` | AC-008 | GIF |
| `AC-008-emit-event-host-fn.webm` | AC-008 | WebM |
| `AC-008-emit-event-host-fn.tape` | AC-008 | VHS source |

---

## Commits on Branch

| Hash | Description |
|------|-------------|
| `c5b6e81` | feat(s-8.05): green — validate-pr-review-posted native WASM port + advisory block-mode + bats dispatcher |
| `d6de21b` | chore(s-8.05): stub-architect — validate-pr-review-posted crate scaffold |

Rustc 1.95.0 used throughout. All `cargo build/test` clean.
Bats tests route through `factory-dispatcher` release build.
Advisory block-mode: hook always exits 0 (HookResult::Continue); block communicated via hook.block event + stderr.
