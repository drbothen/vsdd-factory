# Review Findings — S-18.04b

## Convergence Tracking

| Cycle | Total Findings | Blocking/High | Fixed | Remaining |
|-------|---------------|---------------|-------|-----------|
| Security Review | 5 (1 MEDIUM, 4 LOW) | 0 HIGH/CRITICAL | SEC-002 + SEC-004 dispatched to implementer | SEC-001, SEC-003, SEC-012 (LOW, accepted) |
| Code Review Cycle 1 | 5 (3 MEDIUM, 2 MINOR) | 0 BLOCKING | CR-001/002/003/005 dispatched to implementer | CR-004 (MINOR, no-op — test correct) |
| Review Cycle 2 | TBD | TBD | TBD | TBD |

## Security Review Findings (SEC)

### SEC-002 (MEDIUM — IN FIX): CWE-22 Path Traversal — precompact-flush-prune.sh
- File: `plugins/vsdd-factory/hooks/precompact-flush-prune.sh:62`
- LOG_FILE path argument not validated to .factory/ scope
- Fix: path-prefix check added after `LOG_FILE="$1"`
- Status: Dispatched to implementer (commit pending)

### SEC-004 (LOW — IN FIX): CWE-754 Missing Telemetry — fail-open path
- Files: `validate-burst-log/src/lib.rs:833`, `validate-dispatch-advance/src/lib.rs:~1239`
- Fail-open on all-empty git_context returns Continue silently (no log_warn)
- Fix: add `host::log_warn(...)` on all-empty fail-open path in both crates
- Status: Dispatched to implementer (commit pending)

### SEC-001 (LOW — ACCEPTED): wc output in arithmetic context — CWE-20
- Practical risk nil under normal deployment (POSIX wc, set -e abort on non-numeric)
- No fix required for merge

### SEC-003 (LOW — ACCEPTED): TOCTOU temp-file race — CWE-362
- Accepted design pattern for atomic log-file replacement on POSIX; single-user dev tool
- No fix required for merge

### SEC-012 (LOW — ACCEPTED): `contains_sentinel` case-sensitivity asymmetry — CWE-178
- Intentional design: "backfill" case-insensitive, "Stage N" case-sensitive (documented convention)
- No fix required for merge

## Code Review Cycle 1 Findings (CR)

### CR-001 (MEDIUM — IN FIX): Missing test for FIELD-4=commit, FIELD-2=None edge case
- Files: Both exemption.rs test suites
- AC-003 sub-path (FIELD-4=commit but FIELD-2 absent → treat as corrupted → exempt) not explicitly tested
- Fix: Add test in both exemption.rs files
- Status: Dispatched to implementer

### CR-002 (MEDIUM — IN FIX): Chain detection fires on ALL Bash events (no command filter)
- Files: Both lib.rs (check_chain_from_git_context entry point)
- Every non-git-commit Bash event enters WASM unnecessarily; fast fail-open but wasted overhead
- ADR-029 §Decision 1 semantic intent: fires on PostToolUse Bash git-commit events
- Fix: Add command-string guard at top of check_chain_from_git_context
- Status: Dispatched to implementer

### CR-003 (MEDIUM — IN FIX): on_error="continue" on chain-detection entries not documented
- File: `plugins/vsdd-factory/hooks-registry.toml`
- Crash-fail-open on chain-detection gate is a security tradeoff not documented in registry comment
- Fix: either flip to `on_error = "block"` (preferred — WASM logic handles legitimate fail-open via Continue) or add explicit comment
- Status: Dispatched to implementer

### CR-004 (MINOR — ACCEPTED): vp084-proof.bats Test 3 diagnostic clarity
- No code bug; test correctly fails on regression; documentation note only
- No fix required

### CR-005 (MINOR — IN FIX): od output format assumption in prune.sh
- File: `plugins/vsdd-factory/hooks/precompact-flush-prune.sh:80`
- Add `xxd -p` fallback + `\t` in tr -d to match bats helper pattern for portability
- Status: Dispatched to implementer

## Review Verdicts

| Review | Verdict | Date |
|--------|---------|------|
| Security Review | APPROVE (0 CRITICAL/HIGH) | 2026-06-25 |
| Code Review Cycle 1 | APPROVE (0 BLOCKING) | 2026-06-25 |
| PR Review Cycle 1 (pr-reviewer) | PENDING | — |
