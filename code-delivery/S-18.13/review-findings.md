# Review Findings — S-18.13

**Story:** S-18.13 v1.8 — wave-handoff Write-tool gate-trigger fix  
**PR:** TBD (step 3 in progress)  
**Start:** 2026-06-20  

## Convergence Tracking

| Cycle | Reviewer | Findings | Blocking | Fixed | Remaining |
|-------|----------|----------|----------|-------|-----------|
| 1 | pr-reviewer | 2 | 0 | 0 advisory | 0 → APPROVE |
| 1 | security-reviewer | 7 | 0 CRIT/HIGH; 3 MEDIUM; 2 LOW; 2 INFO | in progress | pending |

**PR reviewer cycle-1 verdict: APPROVE — 0 blocking findings**
- ADVISORY: SC2295 expansion inside `${..}` as pattern — lib/write-handoff.sh:175 (pre-existing)
- ADVISORY: SC2034 BROKEN_STORY_IDS set-but-unused — wave-handoff.sh:295 (harmless)
- 63/63 bats PASS, 0 skips; AC-002 runs live with real dispatcher+WASM

**Security cycle-1 verdict: REQUEST_CHANGES (3 MEDIUM require production-grade explicit validation)**
Security fixes routed to implementer (SEC-001/002/003/005). After fixes, security re-review + final pr-reviewer approval needed.

## Security Review — Cycle 1

| Severity | ID | Finding | Disposition |
|----------|-----|---------|-------------|
| MEDIUM | SEC-001 | YAML injection: factory_lock_holder no explicit allowlist validation in write-handoff.sh | Fix → implementer |
| MEDIUM | SEC-002 | YAML injection: current_status/current_id no explicit allowlist in parse-sprint-state.sh | Fix → implementer |
| MEDIUM | SEC-003 | ARTIFACTS_WT: no realpath canonicalization or git worktree validation in wave-handoff.sh | Fix → implementer |
| LOW | SEC-004 | HANDOFF_WRITE_TOOL_UNAVAILABLE env kill-switch (by design EC-016) | ADVISORY — doc comment only |
| LOW | SEC-005 | wave_id no upper bound check before YAML emission | Fix → implementer |
| INFO | — | factory_lock_holder in HANDOFF.md persisted to git history | Accepted (existing design) |
| INFO | — | GIT_DIR unset is correct defensive practice | No action |

## Pre-existing CI Failures (NOT S-18.13's responsibility)

Per orchestrator advisory:
- `resolver-integration` — timing flake on develop bd6e50ce; pre-existing drift item
- `validate-state-structure/pass-real-state-md-snapshot` — worktree-path issue (TD-VSDD-101); pre-existing drift item

These must be confirmed to fail at the merge-base too. Any CI red on anything ELSE (wave-handoff suite, cargo, bats-wave-handoff-macos) IS S-18.13's responsibility.
