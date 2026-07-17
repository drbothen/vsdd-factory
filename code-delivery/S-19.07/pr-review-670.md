# PR #670 — Fresh-Eyes Final Diff Review (pr-reviewer)

- **PR:** #670 — `feat(S-19.07): verify-factory-lock read_prefix migration (BC-4.13.001 v1.17 Phase-B)`
- **Branch:** `feature/S-19.07` → `develop`
- **HEAD SHA reviewed:** `e7b518e7` (`e7b518e7e6d7a4cdf0b081ae7be775bb8c55670e`)
- **Review date:** 2026-07-17
- **Reviewer:** `vsdd-factory:pr-reviewer` (fresh-context re-run for evidence persistence; no prior-pass findings assumed)
- **Scope:** PR diff + PR description + test evidence + CI status ONLY (charter: not full-codebase internals)
- **Verdict:** **APPROVE** (merge-ready; 0 BLOCKER / 0 HIGH. 1 MEDIUM + 7 LOW are non-gating follow-ups. Human merges personally per D-665.)

---

## CI Evidence (all green)

`gh pr checks 670`: 14 checks PASS, 1 skipping (release-branch guardrail, N/A for feature→develop).

- `cargo-host (ubuntu-latest)` PASS, `cargo-host (macos-latest)` PASS
- `bats-full-suite (linux)` PASS, `bats-darwin-leg (macos)` PASS, `bats-wave-handoff (macos)` PASS
- `build-dispatcher` PASS on all 5 platforms (darwin-arm64/x64, linux-arm64/x64, windows-x64)
- `SAST (Semgrep)` PASS, `validate` PASS, `platforms-drift` PASS

`mergeStateStatus: CLEAN`, `mergeable: MERGEABLE`, no prior review decision.

## Test Evidence (from diff)

- `cargo test -p verify-factory-lock --lib`: 31 passed / 0 failed (baseline 28 → +3)
- `cargo test --workspace --all-targets`: 2057 passed / 0 failed (baseline 2055 → +2 = +3 new lib − 1 deleted integration)
- `bats verify-factory-lock-read-prefix.bats`: 5 passed / 0 failed
- Machine-captured baseline transcript present (`transcript-baseline-workspace-tests.txt`, D-449(a) literal-shell discipline).

## Core change assessment

The functional change is sound and well-covered: `GuardCallbacks.read_file` → `read_prefix`; production call site `host::read_prefix(".factory/STATE.md", 262144, READ_TIMEOUT_MS)`; removal of `STATE_MD_MAX_BYTES`, `OutputTooLarge`/`TooLarge` handling, and the Invariant-10 soft-warn block; `extract_frontmatter` retained unchanged; error mapping preserves CapabilityDenied → graceful `Continue` (fail-open, PC6). The deleted `integration_ac004_no_output_too_large.rs` is legitimately void under Phase-B (read_prefix cannot return OutputTooLarge). New unit tests call the real `guard_logic` (no POLICY 11 tautology) and include an executed mutation check demonstrating the old 8192 bound would miss a lock at ~35 KB frontmatter depth.

---

## Findings

| ID | Severity | File / Anchor | Finding | Proposed routing |
|----|----------|---------------|---------|------------------|
| F-670-01 | MEDIUM | `crates/hook-plugins/verify-factory-lock/src/lib.rs` (test closures, ~6 sites) + `evidence-report.md` | Inconsistent / volatile version pins. Code comments and test error strings cite `ADR-025 §D15 v1.17`, but the PR description and evidence-report cite `ADR-025 v1.18 §Decision 15` as the adjudicating version. One test doc-comment cites `VP-095 v1.3` while everything else cites `VP-095 v1.5`. The bound value (262144) is correct everywhere; only the cited version tokens contradict. Prefer the stable version-free form `ADR-025 §Decision 15` (POLICY 19 / TD-VSDD-091 volatile-pin spirit) and make VP-095 cites uniformly v1.5. | implementer (mechanical cite fix, in-scope) |
| F-670-02 | LOW | `plugins/vsdd-factory/tests/verify-factory-lock-read-prefix.bats` (new file) | POLICY 21 (`no_new_shell_scripts`) scope clarification. This adds a new bash-based `.bats` test file. POLICY 21's literal scope is `.sh` files and its verification steps enumerate `.sh`; bats is the repo's canonical integration framework (CLAUDE.md) so new `.bats` files appear to be the established, exempt pattern. Surface for confirmation only — not treated as a blocker. | orchestrator (confirm bats exemption) |
| F-670-03 | LOW | `plugins/vsdd-factory/hooks/dispatcher/bin/{darwin-arm64,darwin-x64}/factory-dispatcher` | Platform binary-update asymmetry. Only the two darwin dispatcher binaries were rebuilt (CI `bats-darwin-leg` consumes the committed darwin binary and required ReadPrefixCaps). linux-arm64/linux-x64/windows-x64 committed binaries are not touched. windows-x64 has no develop-branch bats leg, so a stale committed windows binary would not be caught by develop CI. Mitigated: `release.yml` cross-compiles all 5 platforms fresh at release time, so operator-level binaries are regenerated regardless of develop state. Confirm this holds before release. | devops-engineer (confirm release rebuilds all platforms) |
| F-670-04 | LOW | `lib.rs` `state_md_malformed_expires_at()` (`#[allow(dead_code)]`) | Dead-code fixture retained with a comment admitting "no bats test in the S-19.07 suite currently exercises this scenario." Production-grade default is remove-or-wire. The malformed-timestamp path is already covered at function level by `test_..._parse_iso8601_errors_on_invalid_timestamp`, so the fixture is redundant. | implementer (remove or wire a test) |
| F-670-05 | LOW | `lib.rs` test names `test_S1907_T003`/`T004` vs `test_S1907_FP1002_*` | Test-function-name vs story-task-role mismatch. Functions named `T003`/`T004` map to story tasks T-006/T-007 (regression), while the FP1002 functions map to tasks T-003/T-004. Internally documented in the transcript, but the naming invites confusion for future readers. | implementer (rename or add mapping comment) |
| F-670-06 | LOW | `lib.rs` `test_S1907_FP1002_real_shape_35kb_frontmatter_foreign_lock_blocks` mutation branch | The "old 8192-byte cap" mutation mock ignores `max_bytes` and hardcodes `fixture[..8192]`. It simulates the bug's observable effect (short prefix → lock missed) rather than mutating the production bound. The real proof that production passes 262144 lives in the sibling closures that assert `max_bytes != 262144 → Err`. The "mutation-check (TD-VSDD-059) / executed evidence per D-449(a)" framing slightly overstates what this branch proves. | implementer (soften comment framing; optional) |
| F-670-07 | LOW | `lib.rs` production call site (bare literal `262144`) | The named constant `STATE_MD_MAX_BYTES` was removed and `262144` is now a bare literal at the production call site (commented with ADR anchor). A named constant would be more self-documenting. Note: the ~6 hardcoded 262144 literals in tests are defensible as an independent oracle (they would catch an accidental production-bound change), so duplication there is intentional; the production-side literal is the only readability nit. | implementer (reintroduce a named `STATE_MD_PREFIX_BYTES`; optional) |
| F-670-08 | LOW | `lib.rs` (removed Invariant-10 soft-warn) + `evidence-report.md` EC-005 note | Observability change. The "state_md_approaching_cap" soft-warn was removed (Invariant 10 retired by BC-4.13.001 v1.17 Phase-B). Under read_prefix, if STATE.md frontmatter ever exceeds 262144 bytes the closing `---` falls outside the prefix → `extract_frontmatter` fails → MalformedLockBlock → fail-open `Continue`, silently, with only a generic MalformedLockBlock warn (no cap-specific diagnostic). The evidence conflates the retired approaching-cap signal with `internal.capability_denied`, which is a different event. Risk is remote (observed frontmatter ~35 KB vs 262144 envelope) and the removal is spec-authorized. | product-owner/architect (confirm spec intent; advisory) |

**Counts:** 0 BLOCKER, 0 HIGH, 1 MEDIUM, 7 LOW.

## Production-grade lens applied

Each LOW was re-checked for hidden BLOCKER status. None qualify: the functional migration is complete and fully tested, CI is green on all exercised platforms, the fail-open behavior is spec-mandated, and the mutation check (though its old-cap branch is illustrative) is backed by real `max_bytes != 262144 → Err` assertions. F-670-01 (version cites) is the only finding with a concrete internal contradiction and is the recommended pre-merge fix, but it is documentation-consistency, not correctness, so it does not gate merge.

## Verdict

**APPROVE.** PR #670 is merge-ready. Recommend F-670-01 (version-cite consistency) be fixed in-scope before merge per the production-grade default; the remaining LOW items are optional cleanups. Merge to be performed by the human operator per D-665 (no automated merge).
