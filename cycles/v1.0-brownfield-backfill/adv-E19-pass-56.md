---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-07-10T00:00:00Z
phase: F5
inputs: []
input-hash: "[live-state]"
traces_to: STATE.md
pass: 56
previous_review: adv-E19-pass-55.md
cycle: v1.0-brownfield-backfill
epic: E-19
verdict: NOT-CLEAN
severity_summary: "B0/H0/M1/L0"
streak_before: "0/3"
streak_after: "0/3"
model: "Claude Opus 4.7"
rubric: "policies.yaml v1.4.4"
date: 2026-07-10
perimeter: "D-811 delta: VP-094 v1.2 + VP-INDEX v2.61 + full E-19 carry-forward at D-811 versions; 30 artifacts"
---

# Adversarial Review — E-19 Pass 56 (NOT-CLEAN; B0/H0/M1/L0)

**Verdict:** NOT-CLEAN — B0/H0/M1/L0  
**Streak:** 0/3 → 0/3 (finding resets streak; was already 0/3)  
**Model:** Claude Opus 4.7 (fresh context; Iron Law SATISFIED)  
**Date:** 2026-07-10  
**Rubric:** policies.yaml v1.4.4

---

## Finding ID Convention

This E-19 cycle uses project-local finding IDs in the form `F-P[PASS]-[SEQ]` (e.g., `F-P56-001`), consistent with all prior E-19 passes and the engine-discipline fix-burst commit convention. The `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>` template prefix is not used in this cycle; `F-P56-001` is the canonical ID for the sole finding of pass-56.

---

## Part A — Fix Verification

Pass-55 (adv-E19-pass-55.md) was **NOT-CLEAN — B0/H0/M1/L0**. One finding (F-P55-001) to verify.

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| F-P55-001 | MEDIUM | CLOSED | VP-094 v1.1→v1.2: 6 stale hooks/→bin/ prefix sites corrected + §Traceability label relabeled; architect 364cfd2c; input-hash 9eff742→e2f422f |

### D-811 Delta 8-Site Re-verification

D-811 fixed VP-094 (v1.1→v1.2) across 6 fix sites and VP-INDEX (v2.60→v2.61) across 2 table rows.

| # | Site | Change | Verdict |
|---|------|--------|---------|
| 1 | VP-094.md §Traceability bullet | `hooks/check-stale-verdict.sh` → `bin/check-stale-verdict.sh`; label `Hook scripts:` → `Enforcement scripts:` | PASS |
| 2 | VP-094.md §Proof Harness line 1 | `run bash plugins/vsdd-factory/hooks/check-stale-verdict.sh` → `bin/check-stale-verdict.sh` | PASS |
| 3 | VP-094.md §Proof Harness line 2 | stale invocation corrected | PASS |
| 4 | VP-094.md §Proof Harness line 3 | `run bash plugins/vsdd-factory/hooks/enforce-merge-strategy.sh` → `bin/enforce-merge-strategy.sh` | PASS |
| 5 | VP-094.md §Proof Harness line 4 | stale invocation corrected | PASS |
| 6 | VP-094.md §Proof Harness comment | `# script: plugins/vsdd-factory/hooks/enforce-merge-strategy.sh` → `bin/enforce-merge-strategy.sh` | PASS |
| 7 | VP-INDEX.md Full Index VP-094 row | v1.2 annotation appended; input-hash 9eff742→e2f422f cited | PASS |
| 8 | VP-INDEX.md Story Anchors VP-094 row | v1.2 annotation appended | PASS |

8/8 D-811 fix sites verified PASS.

### 30-Artifact Perimeter Attestation

Perimeter = D-811 delta (VP-094 v1.2 + VP-INDEX v2.61) + full E-19 carry-forward suite.

| # | Artifact | Version Attested | Status |
|---|----------|-----------------|--------|
| 1 | BC-INDEX.md | v3.95 | PASS |
| 2 | VP-INDEX.md | v2.61 | PASS |
| 3 | STORY-INDEX.md | v4.176 | PASS |
| 4 | ARCH-INDEX.md | v3.00 | PASS |
| 5 | BC-5.42.001 | v1.6 | PASS |
| 6 | BC-4.13.001 | v1.14 | PASS |
| 7 | BC-2.07.001 | v1.5 | PASS |
| 8 | BC-2.02.011 | v1.7 | PASS |
| 9 | BC-3.08.001 | v1.21 | PASS |
| 10 | BC-1.17.001 | v1.6 | PASS |
| 11 | VP-094.md | v1.2 | **FINDING** (see F-P56-001 below) |
| 12 | VP-095.md | v1.1 | PASS |
| 13 | VP-096.md | v1.1 | PASS |
| 14 | VP-097.md | v1.2 | PASS |
| 15 | VP-098.md | v1.2 | PASS |
| 16 | VP-099.md | v1.0 | PASS |
| 17 | VP-100.md | v1.2 | PASS |
| 18 | VP-101.md | v1.3 | PASS |
| 19 | S-19.01 | v1.17 | PASS |
| 20 | S-19.02 | v1.17 | PASS |
| 21 | S-19.03 | v1.19 | PASS |
| 22 | S-19.04 | v1.11 | PASS |
| 23 | S-19.05 | v1.16 | PASS |
| 24 | S-19.06 | v1.19 | PASS |
| 25 | S-19.07 | v1.16 | PASS |
| 26 | E-19 epic | v1.27 | PASS |
| 27 | ADR-025 | v1.15 | PASS |
| 28 | ADR-030 | v1.3 | PASS |
| 29 | policies.yaml | v1.4.4 | PASS |
| 30 | L2-INDEX | v1.0.14 | PASS |

29/30 attestations PASS; 1 FINDING (VP-094.md v1.2; F-P56-001 below).

---

## Part B — New Findings

### CRITICAL

*(none)*

### HIGH

*(none)*

### MEDIUM

#### F-P56-001: VP-094 §Property Statement and §Proof Harness Assert Wrong Sentinel Strings and Exit Codes vs BC-5.42.001 Canonical Test Vectors

- **Severity:** MEDIUM
- **Category:** spec-fidelity / content-of-anchored-postcondition parity (POLICY 4 v1.4.4 — novel axis: sentinel-string and exit-code values in anchored §Postcondition claims and §Proof Harness assertions must derive from source BC §VP-table canonical test vectors)
- **Location:** VP-094.md v1.2 — §Property Statement PS-B, §Property Statement PS-C, §Proof Harness Skeleton (multiple lines)
- **Description:** VP-094 §Property Statement PS-B asserted `VERDICT_STALE` as the sentinel string for the stale-verdict halt path. The canonical sentinel per BC-5.42.001 §Canonical Test Vectors postcondition-2 and ADR-030 §Decision 2 is `STALE_READY_VERDICT`. Similarly, VP-094 §Property Statement PS-C asserted `MERGE_STRATEGY_REQUIRED` as the sentinel for the release-PR merge-strategy enforcement path. The canonical sentinel per BC-5.42.001 §Description (c) and ADR-030 §Decision 3 is `RELEASE_PR_SQUASH_FORBIDDEN`. VP-094 also asserted exit code 2 (fail-closed block-intent) in PS-B and PS-C descriptions; the canonical exit code per ADR-030 §Decision 2 and §Decision 3 is exit 1 (fail-closed without block-intent). Additionally, PS-B §Proof Harness directed assertions to `stdout` where ADR-030 §Decision 2 specifies `stderr` for the STALE_READY_VERDICT emission. VP-094 §Proof Harness Skeleton propagated all four stale values across multiple harness lines.
- **Evidence — PS-B §Property Statement (verbatim stale, VP-094.md v1.2):**
  ```
  - PS-B (Stale-Verdict Halt): when covered_sha has advanced past the pinned verdict,
    check-stale-verdict.sh detects the delta and emits VERDICT_STALE to stdout before
    exiting with code 2 (fail-closed). No merge action is taken.
  ```
  Canonical (BC-5.42.001 §Postcondition 2 + ADR-030 §Decision 2): sentinel is `STALE_READY_VERDICT`; emission to `stderr`; exit code `1`.
- **Evidence — PS-C §Property Statement (verbatim stale, VP-094.md v1.2):**
  ```
  - PS-C (Release-PR Merge-Strategy Enforcement): when the PR targets a release/* branch
    via --squash, enforce-merge-strategy.sh exits 2 (fail-closed) with sentinel
    MERGE_STRATEGY_REQUIRED.
  ```
  Canonical (BC-5.42.001 §Description (c) + ADR-030 §Decision 3): sentinel is `RELEASE_PR_SQUASH_FORBIDDEN`; canonical message form `"Release PR MUST use --merge not --squash"`; exit code `1`.
- **Evidence — §Proof Harness Skeleton stale assertions (VP-094.md v1.2, representative):**
  ```
  @test "VP-094-B: stale verdict emits VERDICT_STALE and exits 2 (fail-closed)" {
      ...
      [ "$exit_code" -eq 2 ] || { ... }
      echo "$result" | grep -q "VERDICT_STALE" || { ... }
      ...
  }
  @test "VP-094-C: release squash rejected with MERGE_STRATEGY_REQUIRED and exits 2" {
      ...
      [ "$status" -eq 2 ] || { ... }
      echo "$output" | grep -q "MERGE_STRATEGY_REQUIRED" || { ... }
  }
  ```
- **BC Self-Contradiction:** BC-5.42.001 v1.6 §VP Properties table rows directly specify the sentinel strings and exit codes. VP-094 is the verification property for BC-5.42.001; its §Proof Harness must assert the canonical values from the BC's own test vectors. The mismatches mean VP-094 v1.2 tested for values that could never pass against the actual implementation — the VP harness was asserting wrong sentinels that would zero-match in any real execution.
- **Survival History:** F-P56-001 survived passes 22–55 (VP-094 v1.0 through v1.2). Passes 22–42 predated the stable-anchor migration; pass-42 (D-797) migrated volatile BC-version pins to stable anchors but did not extract and validate sentinel-string values. Pass-55 (D-811) corrected namespace paths (hooks/→bin/) but likewise did not verify postcondition sentinel-string content. This class of drift — stale content inside an anchored §Postcondition claim — is not caught by the prior 9 standing gates which focused on path/namespace anchors, not on the literal test-vector values cited inside those anchors.

**VP-095..VP-101 Sentinel-String / Exit-Code Class Sweep:**

| VP | Source BC | Sentinel/Exit Assertions Present | Verdict |
|----|-----------|----------------------------------|---------|
| VP-095 | BC-4.13.001 | Size constant STATE_MD_MAX_BYTES=262144 (numeric; derives from BC-4.13.001 §Precondition 3 §Decision 14 — no string sentinels) | CLEAN |
| VP-096 | BC-4.13.001 | Pure function purity/determinism; no string sentinels or exit codes | CLEAN |
| VP-097 | BC-2.07.001 + BC-2.02.011 | Kani proof traversal safety; no named string sentinels | CLEAN |
| VP-098 | BC-2.07.001 | Return codes NOT_FOUND (-5) and CAPABILITY_DENIED (-1) per BC-2.07.001 §Postconditions 2/3; derive from BC — confirmed correct | CLEAN |
| VP-099 | (none — structural lint gate) | No sentinel strings; registry lint only | CLEAN |
| VP-100 | BC-3.08.001 | Event type plugin.abandoned derives from BC-3.08.001 §Invariant 6 — confirmed correct | CLEAN |
| VP-101 | BC-1.17.001 | Return code NOT_FOUND (-5) per BC-1.17.001 §Postcondition 3 — confirmed correct | CLEAN |

7/7 sibling VPs CLEAN. VP-094 was the sole defective site in this sentinel-string / exit-code class.

**CLOSED:** architect 93d3ca03 — 16 sites corrected: VERDICT_STALE→STALE_READY_VERDICT ×5 (PS-B prose + harness lines 124/125/137-138); MERGE_STRATEGY_REQUIRED→RELEASE_PR_SQUASH_FORBIDDEN ×3 (PS-C prose + harness lines 166/167); exit-2→exit-1 ×4+prose (PS-B/PS-C §Property Statement descriptions + harness assertions); PS-C message text canonicalized; PS-B stdout→stderr per ADR-030 §Decision 2; input-hash e2f422f UNCHANGED.

### LOW

*(none)*

---

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 1 |
| LOW | 0 |

**Overall Assessment:** pass-with-findings (B0/H0/M1/L0)  
**Convergence:** findings remain — streak remains 0/3 (was 0/3 entering pass-56; F-P56-001 MEDIUM prevents advancement)  
**Readiness:** requires fix burst (F-P56-001 CLOSED architect 93d3ca03); pass-57 required

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 56 |
| **New findings** | 1 (F-P56-001 — VP-094 sentinel-string and exit-code drift; novel axis: content-of-anchored-postcondition parity) |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (1 new / (1+0)) |
| **Median severity** | MEDIUM |
| **Trajectory** | ...→2→0→1→1 (passes 53/54/55/56 = 2,0,1,1) |
| **Verdict** | FINDINGS_REMAIN — streak 0/3 (was 0/3 entering; no advancement); three consecutive CLEANs required for 3/3 CONVERGED |

**Novelty note:** F-P56-001 exposes a novel defect class — sentinel-string and exit-code values inside anchored §Postcondition claims (PS-B/PS-C §Property Statement text and §Proof Harness assertions) can drift from the source BC §Canonical Test Vectors even when the §Postcondition structural anchors themselves are correctly cited. The prior 9 standing gates (D-811 roster) verified path/namespace anchor parity, volatile-pin migration, and description-bearing prose accuracy for module: fields and §Traceability bullets — but did not extract and compare literal sentinel string values or exit-code integers cited inside §Property Statement descriptions and §Proof Harness test assertions against the source BC §VP-table postcondition canonical form. This is a new 10th gate axis: **POLICY 4 v1.4.4 anchor-prose-parity extends to sentinel-string values and exit-code values in §Property Statement PS-* descriptions and §Proof Harness assertions** — these are load-bearing test-fidelity anchors because they determine whether the proof harness tests for the correct observable behavior. Codification recommendation: extend standing gate roster to 10 via D-812 (POLICY 4 v1.4.4 sentinel-string/exit-code axis). **CODIFIED D-812.**

**Per-Policy Attestations (policies.yaml v1.4.4):**

| Policy | Gate Description | Result |
|--------|-----------------|--------|
| POLICY 1 | VP-INDEX append-only (no rows removed) | PASS |
| POLICY 2 | VP-INDEX Full Index ordering (VP-094..101 contiguous; no reordering) | PASS |
| POLICY 3 | state-manager runs LAST in burst | PASS (SM leg is this report's persister) |
| POLICY 4 v1.4.4 | Description-bearing anchor-prose parity for all VP anchors (module:, §Feasibility Artifact, §Traceability Function-anchor bullets, §Property Statement, §Proof Harness sentinel values) | **FINDING F-P56-001** — VP-094 §Property Statement PS-B/PS-C + §Proof Harness sentinel strings and exit codes do not derive from source BC §VP-table canonical test vectors |
| POLICY 5 v1.3.8 | Category-(j) class sweep on stale-VP-prose findings | PASS (VP-095..101 swept; CLEAN) |
| POLICY 6 | ARCH-INDEX subsystem names canonical | PASS (no subsystem changes) |
| POLICY 7 | BC-INDEX title-cell verbatim parity | PASS (no BC title changes) |
| POLICY 8 v1.3 | BC frontmatter array atomic propagation | PASS (no BC frontmatter changes) |
| POLICY 9 | VP title-change propagation to verification-architecture.md + verification-coverage-matrix.md | PASS (VP-094 description-only annotation; no title change; no propagation required) |
| POLICY 14 | 5-leg quintuple parity on all version bumps | PASS (VP-094.md v1.3: version:, body Changelog, modified[], last_amended, VP-INDEX Full Index + Story Anchors — architect 93d3ca03) |
| POLICY 16 | D-NNN global-max gate | PASS (D-812 allocated after verifying D-811 is current max) |
| POLICY 19 | No volatile pins introduced | PASS (no file:line citations added) |

**Iron Law compliance:** Confirmed. Fresh context for pass-56. Prior pass reports NOT loaded. Rubric policies.yaml v1.4.4 applied.
