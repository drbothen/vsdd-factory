# S-18.00 Review Findings — Convergence Tracking

Story: S-18.00 — Dispatcher PreCompact/PostCompact Routing + check-harness-version.sh
PR: #191 (https://github.com/drbothen/vsdd-factory/pull/191)
Merge commit: b025d31d557fffaba72a03ee4b344eb9cfbd2275
Final HEAD at merge: e46b2773

## Convergence Summary

| Cycle | Source | Findings | Blocking | Fixed | Remaining |
|-------|--------|----------|----------|-------|-----------|
| CI run 1 (ae2a19fd) | CI validate + security-reviewer | 4 | 3 | 4 | 0 |
| CI run 2 (9e3e76a7/9bf80449) | CI bats-full-suite (generate-registry) | 2 | 1 | 2 | 0 |
| PR review cycle 1 (e46b2773) | pr-reviewer | 6 | 0 | 2 | 0 (4 NITS/accepted) |
| **Total** | | **12** | **4** | **8 fixed + 4 accepted** | **0 blocking** |

## Finding Register

### CI Run 1 (commit ae2a19fd) — CI validate + security-reviewer

| ID | Finding | Severity | Status |
|----|---------|----------|--------|
| CI-001 | Orphan hook references: stub-exit0.sh + stub-exit2.sh missing from plugins/vsdd-factory/hooks/ | BLOCKER | FIXED ae2a19fd |
| SEC-001 | check-harness-version.sh: no max-length guard on VERSION_STRING (CWE-400) | MEDIUM | FIXED ae2a19fd |
| SEC-002 | check-harness-version.sh: pre-release version suffix not rejected before numeric comparison (CWE-390) | MEDIUM | FIXED ae2a19fd |
| SEC-003 | env_allow scope for CLAUDE_CODE_VERSION/CLAUDE_VERSION | INFO | VERIFIED CLEAN (no finding) |

### CI Run 2 (commits 9e3e76a7 + 9bf80449) — CI bats-full-suite cascade

| ID | Finding | Severity | Status |
|----|---------|----------|--------|
| CI-002 | generate-registry.bats: stub-exit0/exit2 in hooks/ but not in skip_names — broke registry generator snapshot restore, cascaded to resolver test failures | BLOCKER | FIXED 9e3e76a7 + 9bf80449 |

Root cause analysis: `generate-registry.bats` teardown restores registry snapshot. When the generator failed (due to CI-002), the snapshot restore was skipped, leaving hooks-registry.toml in corrupted state that caused resolver bats to fail via `validate-per-story-adversary-convergence` blocking with WAVE_CONTEXT_MISSING. Not a logic regression in S-18.00.

### PR Review Cycle 1 (commit e46b2773) — pr-reviewer

| ID | Finding | Severity | Status |
|----|---------|----------|--------|
| PR-001 | dispatch_precompact/dispatch_postcompact are empty no-op anchor functions exposed as public API | MINOR | ACCEPTED (traceability design; BC Architecture Anchor pattern) |
| PR-002 | stub-exit0.sh + stub-exit2.sh in production hooks/ dir rather than tests/fixtures/ | MINOR | FIXED e46b2773 |
| PR-003 | Duplicate comment block in hooks-registry.toml | NITS | ACCEPTED (cosmetic) |
| PR-004 | TD-VSDD-091: volatile line-number citation in check-harness-version.bats error message | MINOR | FIXED e46b2773 |
| PR-005 | SEC-004 carry-over: EventType::Other no tracing::warn! on unknown event | LOW | ACCEPTED (inert — no plugins registered for unknown events) |
| PR-006 | Duplicate comment lines in hooks-registry.toml | NITS | ACCEPTED (cosmetic) |

## Convergence Declaration

pr-reviewer verdict: **APPROVE** (0 blocking findings after cycle 1 fixes)
security-reviewer verdict: **0 CRITICAL/HIGH** (2 MEDIUM fixed; 2 LOW/INFO accepted)
CI verdict: **ALL 10 JOBS PASS** on final commit e46b2773 (run 27740856142)
Merge status: **MERGED** — squash commit b025d31d557fffaba72a03ee4b344eb9cfbd2275 on develop
Remote branch: **DELETED** (confirmed via git ls-remote --exit-code, exit code 2)
