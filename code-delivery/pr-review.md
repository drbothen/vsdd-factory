# PR Review — #814 (feature/S-25.04)

**Reviewer:** pr-reviewer (fresh-eyes, pre-merge gate)
**PR:** https://github.com/drbothen/vsdd-factory/pull/814
**Title:** feat(S-25.04): validate-factory-path-staged PostToolUse companion validator (closes Layer-1 zero-enforcement gap)
**Target:** develop
**Verdict:** REQUEST_CHANGES

## Verdict summary

One MEDIUM blocker. Core behavior is otherwise strong and thoroughly tested. Clearing M1 makes this an easy APPROVE.

## What the PR does (verified against the diff)

Adds crate `crates/hook-plugins/validate-factory-path-staged/` (lib.rs + main.rs + tests.rs + proptest) — a PostToolUse `^Bash$` detective mirror of the PreToolUse `validate-factory-path-staging`. On each completed Bash command it runs `git diff --cached --name-only` then `git branch --show-current` and blocks a `.factory/` path staged on a product branch. Registry entry: `priority=161`, `failure_policy="fail-closed"`, `on_error="continue"`, `timeout_ms=5000`, `exec_subprocess binary_allow=["git"]`. `registry.rs` sentinel renamed `cohort_a` → `sanctioned_fail_closed`, 4th entry added, compared via `.len()` not a hardcoded literal.

## Claim verification — all check out

- **Reuse-not-reimplementation:** no `write_indeterminate_marker(` call site in the new crate; `Cargo.toml` depends only on `vsdd-hook-sdk`/`serde`/`serde_json` — zero `factory-dispatcher` dependency, so reuse is structurally enforced. TRUE.
- **Registry mirror/extend without touching sibling:** the sibling's functional keys (name/event/tool/priority/failure_policy) are byte-unchanged. `priority = 161` collision test asserts exactly 1 occurrence. TRUE.
- **45/45 tests:** ~40 in `tests.rs` + 5 in the proptest file. Block path (T-1/T-9), pass path (T-2/T-3/T-8/EC-006), branch-detection fail-open (T-5, exec Err, empty stdout), BROAD scope (T-4 non-git-add text still detected, EC-007 both exec calls issued for `npm test`), exec call shape/order, and the registry structural-reachability gates are all covered.
- **Security:** control chars filtered from block message/event (`is_ascii_graphic() || ' '`); git args are fixed vectors (no injection); output size-capped. Clean.

## Findings

| # | Severity | Finding |
|---|----------|---------|
| M1 | MEDIUM | The `git diff --cached --name-only` (staged-path listing) **failure** fail-open path is untested, and its spec entry is deferred inline. Every `run_hook`/`run_hook_recording_calls` call passes `exec_ok(0, …)` as the FIRST result; the only `exec_err`/non-zero (128) results are in the SECOND (branch) position. So both failure arms of the diff match in `hook_logic` (lib.rs ~600-624: `exit_code != 0` and `Err(e)` → log warn → `None` → PC2 Continue) have ZERO coverage — a silent fail-open (a staged `.factory/` path goes undetected if the diff call fails), whose symmetric twin (branch-detection fail-open) IS tested. The code comment on that path admits: "the formal BC invariant/edge-case entry for this specific path is being added by product-owner at the finalization sweep" — a deferred-spec / "pending owner" pattern the project's production-grade default (CLAUDE.md Rule 6 + self-audit) forbids when answerable in scope. **Fix:** add two tests (diff call non-zero exit → Continue; diff call Err → Continue) and land the BC-4.16.002 invariant/EC now rather than deferring. Small, in-scope. |
| L1 | LOW | AC-003 "sibling byte-unchanged" holds for keys/values, but the COMMENT block above the sibling's `failure_policy` line was rewritten (EFFECTIVE-NOW → ASSIGNED-NOW). Disclosed in the evidence report; the AC-003 demo greps `validate-factory-path-staging.$`, which can't catch comment edits, so "0 diff lines touch the sibling" slightly understates it. Doc-only. |
| L2 | LOW/info | The plugin fires two `git` subprocess spawns on EVERY completed PostToolUse Bash dispatch, unconditionally (BROAD scope). Architect-ratified; noting per-command overhead for awareness, not a defect. |
| L3 | info | `is_factory_path` matches `.factory/` as a lowercased substring, so `app.factory/x` would match and block on a product branch. Inherited verbatim from BC-4.16.001, conservative-by-design — not introduced here, no action. |

## Note on workspace 592/593

The 1 failure (`test_S_19_04_ac006_T009_hermetic_tracked_bundle_zero_orphans`) is about two OTHER orphaned wasm binaries; this PR's new wasm IS registry-referenced so it adds no orphan. Author documented it as pre-existing (verified at merge-base) and flagged for devops/state-manager routing. Reasonable; not a blocker for this PR.
