# PR #759 Fresh-Eyes Review — S-21.01 validate-factory-path-staging

**Reviewer:** pr-reviewer (fresh context, different model family)
**PR:** #759 — S-21.01 — validate-factory-path-staging WASM guard + orchestrator merge pre-check
**Branch:** feature/S-21.01-validate-factory-path-staging (re-review head 99533daf)
**Base:** develop
**Verdict:** APPROVE — all pass-1 findings resolved by fix commit `99533daf`.

---

## Pass 2 re-review (fix commit `99533daf` on top of `17b86e05`)

Re-verified the four pass-1 fix findings against the actual committed code in the
`.worktrees/S-21.01` worktree at head `99533daf`.

Build gate (re-run at fix head):
- `cargo clippy -p validate-factory-path-staging --all-targets -- -D warnings` → **clean**
- `cargo test -p validate-factory-path-staging --lib` → **133 passed**
- `cargo test -p validate-factory-path-staging` (integration/proptest) → **5 passed**

### Fix 1 (was MEDIUM) — `emit_event` on both block paths — RESOLVED
- Invariant 6 block: `emit_event("hook.block", ...)` at `lib.rs:702-710` fires before
  `block_with_fix` (`:711`), with `trigger: "invariant6_target_aware"`.
- Normal-path block: `emit_event` at `:734-742` fires before `block_with_fix` (`:743`),
  with `trigger: "factory_path_arg"`.
- Both emit `("branch", &branch)` — the raw branch value, not `safe_branch`, as required.
- Existing tests unbroken (callbacks wire `emit_event: |_, _| {}` no-ops); 133 lib tests pass.

### Fix 2 (was SEC-003 LOW) — `MAX_COMMAND_LEN` guard — RESOLVED
- `const MAX_COMMAND_LEN: usize = 65_536` at `:607` inside `hook_logic`.
- Applied at `:608`, before `is_git_add_command` (`:617`) — precedes all `split_whitespace`
  tokenization.
- Oversized input returns `HookResult::Continue` (fail-open) with a level-3 (warn) log.

### Fix 3 (was SEC-004 LOW) — block-reason sanitization — RESOLVED
- `safe_branch` + `safe_target` computed before the Invariant 6 block (`:694-701`);
  `safe_branch` computed before the normal-path block (`:730-733`).
- Filter is `c.is_ascii_graphic() || *c == ' '`.
- Format strings use `{safe_target}`/`{safe_branch}` (`:715`, `:747`), not raw values.

### Fix 4 (was NITPICK) — BC version v1.7 in module doc comments — RESOLVED
- Module `//!` doc comments now cite `BC-4.16.001 v1.7` (`:11`, `:15`); function `///`
  BC-trace comments bumped v1.5/v1.6 → v1.7.

### Sibling-sweep — CLEAN
- `main.rs` is a pure delegation to `on_pre_tool_use` (no independent block logic), so no
  parallel `emit_event` omission is possible.

### No-new-issues checks — CLEAN
- No harmful shadowing: `safe_branch`/`safe_target` are new bindings; `branch` remains in
  scope and is the value passed to `emit_event`.
- No clippy warnings under `-D warnings`. `target` remains live (used in `exec_subprocess`
  and to compute `safe_target`), so no unused-variable warning.

### Non-blocking observation (out of scope of the four fixes)
Three residual pre-v1.7 cites remain — `lib.rs:227` (`v1.5`), `:333` and `:342` (`v1.4`) —
all in `is_factory_arg_token`/`contains_factory_path_arg`. These read as historical
"introduced in Invariant 4 v1.4/v1.5" citations, not current-BC-version claims, and were
neither touched by nor introduced by `99533daf`. Optional cleanup only.

---

## Pass 1 record (head `17b86e05`) — findings now resolved

Original verdict: REQUEST_CHANGES (one MEDIUM + LOW/NITPICK). Retained for traceability.

### [MEDIUM] `emit_event` wired but never invoked → **FIXED in `99533daf`**
The `emit_event` callback was declared and wired in `on_pre_tool_use` but `hook_logic`
never called it, so `FactoryPathOnProductBranch` blocks were invisible to the
BC-3.08.001 domain-event / observability stack. Now emitted on both block branches.

### [LOW] CI `run-all.sh` step does not set `CI_REQUIRE_ARTIFACTS=1`
`.github/workflows/ci.yml` `bats-full-suite` (not in this PR's diff). Suite executes for
real today; belt-and-suspenders observation on the surrounding harness. Not a merge blocker.

### [NITPICK] Doc-comment BC version lag → **FIXED in `99533daf`** (module + function cites)

### [NITPICK] Demo evidence is `.txt`, not `.gif`/`.webm`
Appropriate medium for a PreToolUse WASM guard with no UI; `evidence-report.md` documents
the no-VHS rationale and maps every AC. Acceptable for hook artifacts.

---

## Checklist result (pass 1, unchanged)

| # | Item | Result |
|---|------|--------|
| 1 | Diff coherence | PASS — all S-21.01-scoped |
| 2 | Description accuracy | PASS |
| 3 | Test coverage | PASS — unit + proptest + real-dispatcher bats for every AC |
| 4 | Demo evidence | PASS (with nit) — `.txt` scripted-terminal captures; method note present |
| 5 | Commit quality | PASS — conventional, story-ID tagged, red-then-fix TDD |
| 6 | Diff size | PASS |
| 7 | Missing changes | PASS — AC-001..AC-009 all delivered |
| 8 | Dependency status | PASS — base is current develop head |

## Summary

Functionally correct, well-tested, idiomatic Rust; the two-layer INV-E21-001 defense is
faithfully implemented and all nine ACs pass. All four pass-1 fix findings are correctly
resolved by `99533daf` with no regressions (clippy clean, 133 + 5 tests green). **Approve.**
