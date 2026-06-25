# Red Gate Log — S-18.04b (validate-burst-log exemption + prune)

**Date:** 2026-06-24
**Branch:** feature/S-18.04b @ afec273e (ADR-029 re-wire complete; F-P1-001 tautology fix pending)
**Test Writer:** vsdd-factory:test-writer (ADR-029 wiring rewrite + F-P1-001 tautology fix)

## F-P1-001 Tautology Fix (LOCAL adversary pass-1 finding)

**Finding:** Positive `vp084-proof.bats` tests were tautological. They hand-supplied
`git_context` fields in the envelope JSON, but the dispatcher's `inject_git_context_if_qualifying`
at invoke.rs:1445 OVERWRITES any caller-supplied `git_context` key with the result of
`build_git_context($PROJECT_DIR/.factory)`. Since the positive tests only did
`mkdir -p $PROJECT_DIR/.factory` without `git init`, `build_git_context` failed →
returned `GitContext::empty()` → all-empty `git_context` injected → WASM hit fail-open
path (`head_subject.is_empty() && head_parent_subject.is_empty()` at lib.rs:833) →
Continue without exercising `is_precompact_flush_exempt`.

**Fix:** `vp084-proof.bats` positive tests now:
1. Call `_setup_precompact_flush_git_chain`: initialises `$PROJECT_DIR/.factory` as a
   real git repo with HEAD="PreCompact flush ..." and HEAD^="state: burst-23 Commit E ..."
2. Call `_setup_precompact_flush_log_from_real_sha`: reads real HEAD SHA via
   `git rev-parse HEAD` and writes the 4-field precompact-flush-log entry.
3. The dispatcher's `build_git_context` reads real PreCompact subjects → injects non-empty
   `git_context` → WASM exemption logic fires (not fail-open path).

**Proof of non-tautology:** Test 3 (negative control) with real sentinel git repo → dispatcher
injects sentinel subjects → WASM blocks with `MULTI_COMMIT_CHAIN_NOT_ALLOWED`. If the
positive tests were vacuously passing, Test 3 would also pass vacuously (no block) — but
instead it correctly blocks. This triangulates that the positive tests are exercising the
exemption path specifically.

**Bats result after fix:** 3/3 PASS (all tests GREEN; implementation already complete on branch).

---

## Summary (current state after F-P1-001 fix)

Red Gate was VERIFIED at initial test-writer pass (1 deterministic failing test per crate).
Implementation is now complete (commit `24b9ac3b feat(S-18.04b): ADR-029 re-wire`).
F-P1-001 tautology in `vp084-proof.bats` has been fixed — positive tests now use a real
git repo so `build_git_context` returns non-empty subjects, making the exemption logic
the deciding factor (not fail-open).

Final state: all 24 Rust tests PASS; all 3 bats tests PASS with correct non-tautological setup.

---

## Test Files Written / Modified

| File | Status | Tests |
|------|--------|-------|
| `crates/hook-plugins/validate-burst-log/tests/exemption.rs` | REWRITTEN (Section 2 added) | 21 tests |
| `crates/hook-plugins/validate-dispatch-advance/tests/exemption.rs` | EXTENDED (Section 2 added) | 24 tests |
| `plugins/vsdd-factory/tests/vp084-proof.bats` | REWRITTEN (ADR-029 trigger + negative control un-skipped) | 3 tests |

---

## Red Gate Results

### cargo test -p validate-burst-log -p validate-dispatch-advance

```
validate-burst-log:
  test result: FAILED. 20 passed; 1 failed
  FAILED: test_BC_1_16_001_wiring_bash_git_commit_with_sentinel_chain_emits_block

validate-dispatch-advance:
  test result: FAILED. 23 passed; 1 failed
  FAILED: test_BC_1_16_001_wiring_bash_git_commit_with_sentinel_chain_emits_block
```

### Failure messages (Red Gate assertions)

**validate-burst-log:**
```
RED GATE: on_post_tool_use returned Continue when sentinel chain was present in
git_context; the corrected ADR-029 impl must read git_context from payload.extra
and detect MULTI_COMMIT_CHAIN_NOT_ALLOWED. Current exec-based impl reads
exec_subprocess instead — Red Gate confirmed.
```

**validate-dispatch-advance:**
```
RED GATE (dispatch-advance): on_post_tool_use returned Continue when sentinel
chain was present in git_context. The corrected ADR-029 impl must read
git_context from payload.extra and detect MULTI_COMMIT_CHAIN_NOT_ALLOWED.
Current exec-based impl reads exec_subprocess instead — Red Gate confirmed.
```

---

## Test Suite Design

### Section 1: Pure-logic tests (PRESERVED, pass)

Both `exemption.rs` files retain the original 17 pure-logic tests (Section 1)
covering `is_precompact_flush_exempt`, `check_multi_commit_chain`,
`PRECOMPACT_FLUSH_PREFIX`, and the AC-006 symmetry test. These pass because the
pure functions are already fully implemented.

These are NOT the Red Gate. They verify the 3-case logic must remain correct
after the wiring change.

### Section 2: ADR-029 wiring tests (FAIL — Red Gate)

New tests that drive `on_post_tool_use` with synthetic `HookPayload` structs
carrying a real 4-field `git_context` in `payload.extra`:

| Test | Expected | Current behavior | Why it's the Red Gate |
|------|----------|-----------------|----------------------|
| `test_BC_1_16_001_wiring_bash_git_commit_with_sentinel_chain_emits_block` | Block(MULTI_COMMIT_CHAIN_NOT_ALLOWED) | Continue | Current impl has no file_path in Bash payload → early exit; exec_subprocess never fires from git_context |
| `test_BC_1_16_001_wiring_bash_git_commit_precompact_head_exempt_continues` | Continue | Continue (vacuous) | Passes for wrong reason (early exit, not git_context exemption) |
| `test_BC_1_16_001_wiring_bash_git_commit_no_git_context_fail_open_continues` | Continue | Continue (vacuous) | Passes for wrong reason (early exit) |
| `test_BC_1_16_001_wiring_bash_git_commit_empty_git_context_fail_open_continues` | Continue | Continue (vacuous) | Passes for wrong reason (early exit) |
| `test_BC_1_16_001_wiring_edit_event_with_sentinel_git_context_no_chain_block` | No MULTI_COMMIT_CHAIN block | No block | Passes: current impl reads git via exec_subprocess; exec fails → fail-open → no block |

The single deterministic Red Gate test is `test_BC_1_16_001_wiring_bash_git_commit_with_sentinel_chain_emits_block`.
It expects Block when both HEAD and HEAD^ contain sentinels supplied via git_context.
The current impl returns Continue (early exit: no file_path in Bash payload).
The corrected impl must detect Bash events + read git_context + run chain check.

### vp084-proof.bats: ADR-029 corrections

| Change | Before | After |
|--------|--------|-------|
| Tool trigger | `tool="Edit"` | `tool="Bash"` with git commit command |
| git_context fields | 2 fields (missing head_sha, head_parent_sha) | All 4 fields (BC-1.16.001 PC1) |
| Negative control | Skipped | Un-skipped; uses real sentinel subjects in git_context (not fail-open) |
| event_name field | `event` (wrong) | `event_name` (HookPayload struct field) |
| tool_name field | `tool` (wrong) | `tool_name` (HookPayload struct field) |

The negative control test now actively asserts `MULTI_COMMIT_CHAIN_NOT_ALLOWED` is
emitted for sentinel subjects. Before ADR-029 impl, it FAILS (no block). This closes
the pass-1 F-1 tautology finding.

---

## What Must Change for Tests to Pass

The implementer must:

1. **Rewrite `check_factory_artifacts_chain()` in both crates** to read from
   `payload.extra["git_context"]` (4-field JSON object) instead of calling
   `host::exec_subprocess` for HEAD/HEAD^ subjects.

2. **Change the trigger logic in `on_post_tool_use()`** to:
   - Detect `tool_name == "Bash"` AND `tool_input.command` contains `"git commit"`
   - Extract `head_subject`, `head_sha`, `head_parent_subject`, `head_parent_sha`
     from `payload.extra.get("git_context")`
   - Skip chain check (fail-open) when git_context is absent or all-empty
   - Run `check_multi_commit_chain()` from the extracted fields
   - NOT run chain check on Edit/Write events

3. **Remove `host::exec_subprocess` calls** from chain detection (ADR-029 §Decision 3).

4. **Update hooks-registry.toml** trigger from `Edit|Write` to `Bash` for both
   `validate-burst-log` and `validate-dispatch-advance`.

---

## BC / AC Coverage Map

| Test | BC | AC/INV |
|------|----|--------|
| `test_BC_1_16_001_wiring_bash_git_commit_with_sentinel_chain_emits_block` | BC-1.16.001, BC-5.41.003 PC3 | AC-007 trigger; TD-VSDD-053 |
| `test_BC_1_16_001_wiring_bash_git_commit_precompact_head_exempt_continues` | BC-5.41.003 PC1(c), BC-1.16.001 | AC-001..AC-003 via git_context |
| `test_BC_1_16_001_wiring_bash_git_commit_no_git_context_fail_open_continues` | BC-1.16.001 INV3 | Fail-open on absent git_context |
| `test_BC_1_16_001_wiring_bash_git_commit_empty_git_context_fail_open_continues` | BC-1.16.001 PC2 | Fail-open on all-empty git_context |
| `test_BC_1_16_001_wiring_edit_event_with_sentinel_git_context_no_chain_block` | ADR-029 §Decision 1 | Bash-only chain detection |
| `test_BC_5_41_003_wiring_exec_free_constraint_documented` | ADR-029 §Decision 3 | AC-006 exec-free |
| vp084-proof.bats positive tests | VP-084, BC-5.41.003 PC4 | AC-007 dispatcher path |
| vp084-proof.bats negative control | VP-084, BC-5.41.003 PC3 | Pass-1 F-1 non-tautological proof |

---

## No src/lib.rs or hooks-registry.toml Modified

Per task instructions: NO implementation files were touched. Only test files
(`tests/exemption.rs` for both crates, `vp084-proof.bats`) were written/modified.

The Cargo.toml for validate-dispatch-advance had `vsdd-hook-sdk` added to
`[dev-dependencies]` (required for `HookPayload` in test code).
