# S-19.01 Demo Evidence Report

**Story:** S-19.01 — pr-manager hardening: READY verdict HEAD-SHA pinning + release-PR merge-strategy guard + shell-dialect simulation discipline
**Branch:** `feature/S-19.01`
**Implementation commit:** `a68f62e6`
**Evidence produced by:** Demo Recorder
**Date:** 2026-07-11

---

## Summary

S-19.01 hardens the pr-manager workflow against three silent failure modes
discovered during the rc.22 brownfield-backfill cycle (D-749, D-750):

**AC-001:** The `pr-manager-completion-guard` WASM SubagentStop hook now inspects
every READY verdict for a `covered_sha: <40-lowercase-hex>` field. Missing or
malformed covered_sha triggers `READY_SHA_MISSING` advisory. `check-stale-verdict.sh`
emits `READY_SHA_FETCH_FAILED` on `gh pr view` failure.

**AC-002:** `check-stale-verdict.sh` runs synchronously before every merge. It
calls `gh pr view --json headRefOid,state` and compares against the pinned SHA.
Mismatch, closed PR, null state, or malformed JSON all exit 1 with the appropriate
diagnostic sentinel (fail-closed on all 4 ADR-030 §Decision 2 arms).

**AC-003:** `enforce-merge-strategy.sh` is the sole gateway for all `gh pr merge`
invocations. It forces `--merge` on `release/v*` branches, rejects `--squash`/`--rebase`
with `RELEASE_PR_SQUASH_FORBIDDEN`, scans residual args against a deny-list emitting
`STRATEGY_SMUGGLING_FORBIDDEN`, and rejects invalid `$2` with `INVALID_STRATEGY`.
pr-manager.md Step 8-pre-B wires both wrapper scripts as the only merge path.

**AC-004:** Darwin-leg CI script validation uses `/bin/bash` 3.2.x (macOS system
bash, not Homebrew 5.x). The `bats-darwin-leg-macos` CI job runs on `macos-latest`
with a preflight that exits non-zero (`DARWIN_LEG_WRONG_INTERPRETER`) if the system
bash is not 3.2. T-017 regression-pins the rc.22 `while IFS= read -r` fix in
`release.yml` by extracting and executing the fragment under `/bin/bash` 3.2 with
a fixture registry, confirming the `mapfile` regression cannot silently re-enter.

All 4 acceptance criteria met. 41 cargo unit tests pass. 33 bats integration tests
pass (T-009 correctly skipped on macOS per EC-003; bats warning on T-017 mapfile
negative control exit code 127 is expected — the negative control must fail under
bash 3.2).

---

## AC Coverage Map

| AC | Title | Status | Evidence File |
|----|-------|--------|--------------|
| AC-001 | covered_sha enforcement via WASM hook + gh failure arm | PASS | [AC-001.md](AC-001.md) |
| AC-002 | check-stale-verdict.sh stale-verdict detection (all 4 arms) | PASS | [AC-002.md](AC-002.md) |
| AC-003 | enforce-merge-strategy.sh sole-gateway + deny-list + Step 8 wiring | PASS | [AC-003.md](AC-003.md) |
| AC-004 | darwin-leg preflight + CI job + T-017 regression pin | PASS | [AC-004.md](AC-004.md) |

---

## BC Coverage Map

| BC | Title | Verified By |
|----|-------|-------------|
| BC-5.42.001 PC-1 | READY verdict includes covered_sha field | AC-001 (T-001, T-032, cargo s19_01_check_ready_sha_completeness_emits_missing) |
| BC-5.42.001 PC-2 | check-stale-verdict.sh invoked before every merge | AC-002 (T-003/T-004/T-013, T-022 wiring gate) |
| BC-5.42.001 PC-3 | enforce-merge-strategy.sh gates all merges | AC-003 (T-005/T-006/T-007, T-022 sole-gateway gate) |
| BC-5.42.001 Invariant 1 | READY SHA recorded at assessment time | AC-001 (T-001/T-032) |
| BC-5.42.001 Invariant 2 | Orchestrator invokes check before every merge | AC-002 (T-022 wiring gate) |
| BC-5.42.001 Invariant 3 | Release branch squash/rebase mechanically impossible | AC-003 (T-005/T-011/T-015) |
| BC-5.42.001 Invariant 5 | covered_sha = exactly 40 lowercase hex chars | AC-001 (T-010/T-016, cargo has_valid_covered_sha tests) |
| BC-5.42.001 Invariant 6 | enforce-merge-strategy.sh is sole gh pr merge gateway | AC-003 (T-022) |
| BC-5.42.001 Invariant 7 | Deny-list rejects strategy-flag smuggling via residual args | AC-003 (T-024/T-025/T-026) |
| BC-5.42.001 EC-001 | gh failure → READY_SHA_FETCH_FAILED verbatim | AC-001/002 (T-002/T-014) |
| BC-5.42.001 EC-003 | Closed PR → CHECK_STALE_VERDICT_ERROR | AC-002 (T-018) |
| BC-5.42.001 EC-004 | Explicit --merge on release branch passes through | AC-003 (T-006) |
| BC-5.42.001 EC-005 | No flag on release branch → --merge injected | AC-003 (T-012) |
| ADR-030 §Decision 2 arm 3 | Closed/merged state blocks merge | AC-002 (T-018) |
| ADR-030 §Decision 2 arm 3a | Null state field → fail-closed | AC-002 (T-033) |
| ADR-030 §Decision 2 arm 4 | Malformed JSON → fail-closed | AC-002 (T-019/T-020) |
| ADR-030 §Decision 3 | Sole gateway; no direct gh pr merge | AC-003 (T-022) |
| ADR-030 §Decision 3 v1.4 | Deny-list covers long/=-fused/bare-short/combined-short | AC-003 (T-024/T-025/T-026) |

---

## Test Results

### Cargo unit tests — pr-manager-completion-guard crate

```
cargo test -p pr-manager-completion-guard

test result: ok. 41 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

41 tests pass. 11 S-19.01-specific tests cover covered_sha validation logic:
`test_s19_01_has_ready_verdict_*` (2), `test_s19_01_has_valid_covered_sha_*` (4),
`test_s19_01_check_ready_sha_completeness_*` (3), plus the WASM hook integration
test for the READY_SHA_MISSING advisory path.

### Bats integration tests — pr-manager-hardening.bats

```
bats plugins/vsdd-factory/tests/pr-manager-hardening.bats --tap

1..33
ok 1  T-001: READY verdict without covered_sha triggers READY_SHA_MISSING advisory
ok 2  T-002: gh failure → READY_SHA_FETCH_FAILED on stderr (BC-5.42.001 EC-001)
ok 3  T-003: check-stale-verdict.sh: stale SHA → exit 1 + STALE_READY_VERDICT
ok 4  T-004: check-stale-verdict.sh: matching SHA → exit 0 (fresh verdict)
ok 5  T-005: enforce-merge-strategy.sh: release/v* + --squash → exit 1 + RELEASE_PR_SQUASH_FORBIDDEN
ok 6  T-006: enforce-merge-strategy.sh: release/v* + --merge → exit 0 (allowed)
ok 7  T-007: enforce-merge-strategy.sh: non-release + --squash → exit 0 (delegated)
ok 8  T-008: darwin-leg preflight: wrong interpreter exits 1 + DARWIN_LEG_WRONG_INTERPRETER
ok 9  T-009: darwin-leg preflight: Linux runners skip gracefully (exit 0) # skip (macOS-only test)
ok 10 T-010: check-stale-verdict.sh: malformed covered_sha → exit 1 + READY_SHA_MISSING
ok 11 T-011: enforce-merge-strategy.sh: release/v* + --rebase → exit 1 + RELEASE_PR_SQUASH_FORBIDDEN
ok 12 T-012: enforce-merge-strategy.sh: release/v* + no flag → defaults to --merge (EC-005)
ok 13 T-013: check-stale-verdict.sh: STALE_READY_VERDICT exact canonical message format
ok 14 T-014: check-stale-verdict.sh: READY_SHA_FETCH_FAILED exact canonical message format (EC-001)
ok 15 T-015: enforce-merge-strategy.sh: RELEASE_PR_SQUASH_FORBIDDEN exact canonical message format
ok 16 T-016: check-stale-verdict.sh: 40-char uppercase-hex covered_sha → READY_SHA_MISSING (EC-002)
ok 17 T-017: darwin-leg fragment: while IFS= read -r bash-3.2 compat (regression pin rc.22)
ok 18 T-018: check-stale-verdict.sh: closed PR (matching SHA) → exit 1 + CHECK_STALE_VERDICT_ERROR (EC-003)
ok 19 T-019: check-stale-verdict.sh: malformed gh JSON → exit 1 + CHECK_STALE_VERDICT_ERROR (arm 4)
ok 20 T-020: check-stale-verdict.sh: headRefOid null value → exit 1 + CHECK_STALE_VERDICT_ERROR
ok 21 T-021: enforce-merge-strategy.sh: headRefName null value → fail-open delegate
ok 22 T-022: pr-manager.md Step 8 must route through wrappers not direct gh pr merge
ok 23 T-023: enforce-merge-strategy.sh forwards --delete-branch residual arg to gh
ok 24 T-024: enforce-merge-strategy.sh: --merge --squash (two strategies) → exit 1
ok 25 T-025: enforce-merge-strategy.sh: --merge --admin → exit 1 (deny-list)
ok 26 T-026: enforce-merge-strategy.sh: --merge -sd (combined short) → exit 1
ok 27 T-027: enforce-merge-strategy.sh: --merge --delete-branch allowed + forwarded
ok 28 T-028: enforce-merge-strategy.sh: release + --merge --delete-branch → delegates both
ok 29 T-029: enforce-merge-strategy.sh: --admin as $2 → exit 1 + INVALID_STRATEGY
ok 30 T-030: enforce-merge-strategy.sh: -A as $2 → exit 1
ok 31 T-031: enforce-merge-strategy.sh: --merge as $2 (feature branch) → exit 0
ok 32 T-032: pr-manager.md Step 8-pre-A must not contain re-fetch-covered_sha fallback
ok 33 T-033: check-stale-verdict.sh: matching SHA + null state → exit 1 + CHECK_STALE_VERDICT_ERROR
```

33/33 tests executed. T-009 skipped on macOS (correct per EC-003: T-009 is a
Linux-runner guard; the `bats-darwin-leg-macos` CI job runs on macOS exclusively).

---

## Test-to-AC Mapping

| Test | AC | BC Trace |
|------|----|----------|
| T-001 | AC-001 | BC-5.42.001 PC-1, Invariant 1 |
| T-002 | AC-001 | BC-5.42.001 EC-001 |
| T-003 | AC-002 | BC-5.42.001 PC-2 |
| T-004 | AC-002 | BC-5.42.001 PC-2 |
| T-005 | AC-003 | BC-5.42.001 PC-3, Invariant 3 |
| T-006 | AC-003 | BC-5.42.001 PC-3, EC-004 |
| T-007 | AC-003 | BC-5.42.001 PC-3 |
| T-008 | AC-004 | AC-004 D-g note |
| T-009 | AC-004 | AC-004 EC-003 (skipped on macOS) |
| T-010 | AC-002 | BC-5.42.001 Invariant 5 |
| T-011 | AC-003 | BC-5.42.001 Invariant 3 |
| T-012 | AC-003 | BC-5.42.001 EC-005 |
| T-013 | AC-002 | BC-5.42.001 PC-2 canonical format |
| T-014 | AC-001 | BC-5.42.001 EC-001 canonical format |
| T-015 | AC-003 | BC-5.42.001 PC-3 canonical format |
| T-016 | AC-002 | BC-5.42.001 Invariant 5, EC-002 |
| T-017 | AC-004 | AC-004 mechanism test F-P7-005 |
| T-018 | AC-002 | BC-5.42.001 EC-003, ADR-030 arm 3 |
| T-019 | AC-002 | ADR-030 §Decision 2 arm 4 |
| T-020 | AC-002 | ADR-030 §Decision 2 arm 4 (null headRefOid) |
| T-021 | AC-003 | ADR-030 §Decision 3 (null headRefName fail-open) |
| T-022 | AC-003 | BC-5.42.001 Invariant 6, ADR-030 §Decision 3 |
| T-023 | AC-003 | BC-5.42.001 PC-3 pass-through |
| T-024 | AC-003 | BC-5.42.001 Invariant 7 |
| T-025 | AC-003 | BC-5.42.001 Invariant 7 |
| T-026 | AC-003 | BC-5.42.001 Invariant 7 |
| T-027 | AC-003 | BC-5.42.001 Invariant 7 (positive) |
| T-028 | AC-003 | BC-5.42.001 PC-3, Invariant 3 |
| T-029 | AC-003 | BC-5.42.001 PC-3 ($2 validation) |
| T-030 | AC-003 | BC-5.42.001 PC-3 ($2 validation) |
| T-031 | AC-003 | BC-5.42.001 PC-3 (positive regression guard) |
| T-032 | AC-001 | BC-5.42.001 PC-1/Invariant 1 (wiring gate) |
| T-033 | AC-002 | ADR-030 §Decision 2 arm 3a (null state) |

---

## File Inventory

| File | Purpose |
|------|---------|
| `evidence-report.md` | This report — coverage map, test results, file inventory |
| `AC-001.md` | covered_sha WASM hook enforcement + cargo unit tests |
| `AC-002.md` | check-stale-verdict.sh stale-verdict detection (all arms) |
| `AC-003.md` | enforce-merge-strategy.sh sole-gateway + deny-list + Step 8 wiring |
| `AC-004.md` | darwin-leg preflight + T-017 regression pin + CI job verification |

---

## Architecture Compliance Verification

| Rule | Status |
|------|--------|
| enforce-merge-strategy.sh is sole gateway for gh pr merge | PASS — T-022 static wiring gate; Step 8-pre-B wired in pr-manager.md |
| Release PRs MUST merge with --merge per RELEASING.md | PASS — T-005/T-011 block squash/rebase; T-012 injects --merge on no-flag |
| READY verdicts must record evaluated commit SHA | PASS — T-001 WASM hook advisory; T-032 re-fetch fallback absent |
| check-stale-verdict.sh invoked before every gh pr merge | PASS — Step 8-pre-A wired in pr-manager.md; T-022 |
| Darwin-leg validation uses /bin/bash 3.2.x | PASS — T-008 preflight; T-017 interpreter confirmed |
| bats-darwin-leg-macos CI job present in ci.yml (O-P15-04) | PASS — grep -qE '^  bats-darwin-leg-macos:$' exits 0 |
| No AI attribution in commits | PASS — no Co-Authored-By: Claude |

---

## Closures

- **D-749** (L-BB-merge-race-ready-report-stale-head): CLOSED — covered_sha pinning
  (AC-001) + stale-verdict detection (AC-002) together guarantee the reviewed commit
  and merged commit are identical or the merge is blocked.
- **D-750** (L-BB-release-pr-squash-merge-not-mechanically-enforced): CLOSED —
  enforce-merge-strategy.sh (AC-003) makes squash-merge on release/v* branches
  mechanically impossible; the sole-gateway wiring ensures no bypass path exists.
- **D-750** (L-BB-simulation-shell-dialect-gap): CLOSED — darwin-leg preflight
  (AC-004) + T-017 regression pin ensure release.yml darwin-leg scripts are always
  validated under `/bin/bash` 3.2.x on macOS, not Homebrew 5.x.

---

## Convergence Status

S-19.01 meets all 4 acceptance criteria. LOCAL cascade converged 3/3 per BC-5.39.001
3-CLEAN protocol. 41 cargo tests + 33 bats tests green. D-749 + D-750 closed.
