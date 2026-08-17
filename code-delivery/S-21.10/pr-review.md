# PR #780 — Final Fresh-Eyes Review (READY-TO-MERGE)

- **PR:** #780 `feat(S-21.10): failure_policy dispatcher registry schema extension (BC-1.01.016 v1.3, ADR-039 Phase 1)`
- **Branch:** `feature/S-21.10` → `develop`
- **Reviewed HEAD (covered_sha):** `e6e86ba61598a0aebac9504648d03e5af90530a2`
- **Base:** `a6a15e1df4f778df34ebe5fe967b25154e0f9e2f` (develop)
- **Review cycle:** FINAL (cycle 1 APPROVED at `82437a64`; cycle 2 APPROVED at `e6e86ba6`; cycle 3 re-confirmed the tree at the then-tip empty commit `58c0435e`). The empty tip commit `58c0435e` `ci(S-21.10): retrigger CI ...` has since been **dropped by a human-authorized force-push**; the branch tip is now the substantive commit `e6e86ba6` (`fix(S-21.10): re-export FailurePolicy from lib.rs + fix EC-004 comment`). `gh pr view 780 --json headRefOid` confirms HEAD = `e6e86ba6` and `58c0435e` is absent from the 7-commit branch history. All prior verification below holds unchanged at this final HEAD.

## Verdict: APPROVE — covered_sha `e6e86ba61598a0aebac9504648d03e5af90530a2`

0 blocking findings. No BLOCKER or REQUEST_CHANGES findings.

## Final gate re-confirmation (HEAD e6e86ba6)

- PR HEAD = `e6e86ba61598a0aebac9504648d03e5af90530a2` (matches review target).
- Empty tip commit `58c0435e` absent from branch history (force-push dropped it); this was the sole prior blocker.
- CI job `policy-15-attestation-location`: PASSING (run 32033793805, conclusion=success). `attestation-gate-non-vacuity-controls`: success.
- Code merits already approved at this exact SHA by prior reviewer (pr-reviewer-s2110-v4); no code changes since.

## What this PR does

ADR-039 Phase 1 schema extension only (no enforcement change):
- New `FailurePolicy` enum in `crates/factory-dispatcher/src/registry.rs` with two variants `FailClosed` (`"fail-closed"`) and `FailOpen` (`"fail-open"`), `#[serde(rename_all = "kebab-case")]`, `#[default]` on `FailOpen`.
- New `failure_policy: FailurePolicy` field on `RegistryEntry`, `#[serde(default)]` for absent-field backward-compat.
- `plugin_fail_closed` in `executor.rs` is deliberately NOT modified — enforcement flip deferred to S-21.11 (BC-1.03.017 / Phase 4).

## The two new changes in e6e86ba6 — both correct

1. **lib.rs re-export.** `FailurePolicy` added to the `pub use registry::{...}` block in correct alphabetical position (`ExecSubprocessCaps, FailurePolicy, OnError`). Closes cycle-1 non-blocking finding that the new public type was not re-exported at crate root alongside its siblings. Consistent with existing `OnError`/`RegistryEntry` re-export convention.
2. **registry.rs module-comment scope fix (EC-004).** Test-module header now reads "EC-001..EC-003, EC-005..EC-007 … EC-004 (duplicate `failure_policy` key) is a TOML-parser-layer concern and is not covered by registry unit tests." Accurate against BC-1.01.016 EC-004 ("TOML parse error (duplicate key); not a registry-layer concern").

## Spec-fidelity re-confirmation — all hold

1. **AC-001..AC-007 / PC1..PC7** — each postcondition has a dedicated GREEN-by-design test:
   - PC1/PC2: `"fail-closed"`→`FailClosed`, `"fail-open"`→`FailOpen` via `rename_all = "kebab-case"`.
   - PC3 + EC-001/002/003: unknown, empty, wrong-case, and underscore (`"fail_closed"`) values all reject at serde parse time. EC-003 underscore guard is the critical anti-footgun vs. the sibling `OnError`'s `snake_case`.
   - PC4: absent field defaults to `FailOpen` via `#[serde(default)]` + `#[default]`.
   - PC5 / EC-005 / EC-006: `on_error` and `failure_policy` coexist as independent fields; both `block`+`fail-open` and `continue`+`fail-closed` combinations parse and store without conflict.
   - PC6 / EC-007: full production `hooks-registry.toml` (76 entries, none carry `failure_policy`) parses cleanly; zero `FailClosed`; all resolve to `FailOpen`.
2. **AC-006 no-enforcement gate (PC7 RED Gate).** `git diff` shows `plugin_fail_closed` is untouched — the only `executor.rs` change is a test-fixture field addition. The canonical gate test `fail_closed_timeout_with_on_error_continue_is_open` passes unmodified, and the registry-side scope guard `test_..._phase1_failure_policy_does_not_affect_on_error_accessor` confirms `failure_policy` has zero influence on the `on_error()` accessor. Phase 1 boundary respected exactly (ADR-039 Decision 3).
3. **Sibling-sweep (TD-VSDD-060).** The new `RegistryEntry.failure_policy` field is wired into all 7 fixture-construction sites across 6 files (executor.rs, partition.rs kani proof, async_partition_integration.rs, executor_integration.rs, executor_resolver_integration.rs, full_stack_plugin_invocation.rs, resolver_error_isolation_test.rs). Completeness guaranteed by the clean compile.
4. **No AI attribution.** Commit messages checked; zero `Co-Authored-By`/Claude references.

## Local verification (run on HEAD e6e86ba6)

- `cargo fmt --check --all` → clean.
- `cargo clippy --workspace --all-targets -- -D warnings` → clean (0 warnings).
- `cargo test -p factory-dispatcher` → 241 unit tests + all integration suites pass, 0 failures. New `s21_10_bc_1_01_016_failure_policy` module (14 tests) all green; `fail_closed_timeout_with_on_error_continue_is_open` PC7 gate passes unmodified.

## Cycle-3 re-verification (run on final HEAD 58c0435e)

Re-ran the full suite at `58c0435e` (tree-identical to `e6e86ba6`):
- `cargo fmt --check --all` → clean (exit 0).
- `cargo clippy --workspace --all-targets -- -D warnings` → 0 warnings/errors.
- `cargo test -p factory-dispatcher` → **582 pass, 0 fail**. The `s21_10_bc_1_01_016_failure_policy` module (15 tests) all green; canonical AC-006 gate `fail_closed_timeout_with_on_error_continue_is_open` passes unmodified.
- AC-006 confirmed: `plugin_fail_closed(result: &PluginResult, on_error: OnError)` signature unchanged; does not consult `failure_policy`. The `executor.rs` diff is a test-module struct-literal field addition only.
- POLICY 21: zero `.sh` files added.

## Verdict at final HEAD: APPROVE — covered_sha `58c0435e62894523135d1d9d4ac8374259a20182`

## Non-blocking observation (not a finding; no action required for merge)

The test-module header claims tests for "EC-005..EC-007," but only EC-005 carries an explicit ID tag in a test name. EC-006 (`on_error=continue` + `failure_policy=fail-closed`) is behaviorally covered by the PC5 and PC7 tests, and EC-007 (all production entries) by the PC6 production-registry test. Coverage is real; only the by-ID test-name tagging is implicit.
