---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-07-11T00:00:00Z
phase: F5
inputs: []
input-hash: "[live-state]"
traces_to: STATE.md
pass: 57
previous_review: adv-E19-pass-56.md
cycle: v1.0-brownfield-backfill
epic: E-19
verdict: NOT-CLEAN
severity_summary: "B0/H0/M1/L2"
streak_before: "0/3"
streak_after: "0/3"
model: "Claude Opus 4.7"
rubric: "policies.yaml v1.4.4"
date: 2026-07-11
perimeter: "D-812 delta: VP-094 v1.3 + VP-INDEX v2.62 + full E-19 carry-forward; streak 0/3"
---

# Adversarial Review — E-19 Pass 57 (NOT-CLEAN; B0/H0/M1/L2)

**Verdict:** NOT-CLEAN — B0/H0/M1/L2  
**Streak:** 0/3 → 0/3 (finding resets streak; was already 0/3)  
**Model:** Claude Opus 4.7 (fresh context; Iron Law SATISFIED)  
**Date:** 2026-07-11  
**Rubric:** policies.yaml v1.4.4

---

## Finding ID Convention

This E-19 cycle uses project-local finding IDs in the form `F-P[PASS]-[SEQ]` (e.g., `F-P57-001`), consistent with all prior E-19 passes and the engine-discipline fix-burst commit convention. The `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>` template prefix is not used in this cycle; `F-P57-001` is the canonical ID for the sole finding of pass-57. Observations use the form `O-P[PASS]-[SEQ]`.

---

## Part A — Fix Verification

Pass-56 (adv-E19-pass-56.md) was **NOT-CLEAN — B0/H0/M1/L0**. One finding (F-P56-001) to verify.

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| F-P56-001 | MEDIUM | CLOSED | VP-094 v1.2→v1.3: sentinel strings STALE_READY_VERDICT + RELEASE_PR_SQUASH_FORBIDDEN, exit-1 x4, stderr routing, PS-C message text — all corrected; architect 93d3ca03; input-hash e2f422f UNCHANGED |

### D-812 Delta Fix Verification

D-812 fixed VP-094 (v1.2→v1.3) across 16 sites and VP-INDEX (v2.61→v2.62) across 2 table rows.

| # | Site | Change | Verdict |
|---|------|--------|---------|
| 1 | VP-094.md §Property Statement PS-B prose | VERDICT_STALE → STALE_READY_VERDICT | PASS |
| 2 | VP-094.md §Property Statement PS-B prose | exits 2 → exits 1 | PASS |
| 3 | VP-094.md §Property Statement PS-B prose | stdout → stderr per ADR-030 §Decision 2 | PASS |
| 4 | VP-094.md §Property Statement PS-C prose | MERGE_STRATEGY_REQUIRED → RELEASE_PR_SQUASH_FORBIDDEN | PASS |
| 5 | VP-094.md §Property Statement PS-C prose | exits 2 → exits 1 | PASS |
| 6 | VP-094.md §Property Statement PS-C message text | canonicalized per ADR-030 §Decision 3 | PASS |
| 7-11 | VP-094.md §Proof Harness VP-094-B/B-pass sentinel + exit assertions | VERDICT_STALE→STALE_READY_VERDICT x2; exit-2→exit-1 x2; exit-2→exit-1 in B assertion | PASS |
| 12-15 | VP-094.md §Proof Harness VP-094-C/C-pass/C-nonrelease sentinel + exit | MERGE_STRATEGY_REQUIRED→RELEASE_PR_SQUASH_FORBIDDEN x3; exit-2→exit-1 x3 | PASS |
| 16 | VP-094.md §Proof Harness comment | corrected | PASS |
| 17 | VP-INDEX.md Full Index VP-094 row | v1.3 annotation appended | PASS |
| 18 | VP-INDEX.md Story Anchors VP-094 row | v1.3 annotation appended | PASS |

16/16 D-812 fix sites verified PASS. F-P56-001 **CLOSED**.

### 30-Artifact Perimeter Attestation

Perimeter = D-812 delta (VP-094 v1.3 + VP-INDEX v2.62) + full E-19 carry-forward suite.

| # | Artifact | Version Attested | Status |
|---|----------|-----------------|--------|
| 1 | BC-INDEX.md | v3.95 | PASS |
| 2 | VP-INDEX.md | v2.62 | PASS |
| 3 | STORY-INDEX.md | v4.176 | PASS |
| 4 | ARCH-INDEX.md | v3.00 | PASS |
| 5 | BC-5.42.001 | v1.6 | PASS |
| 6 | BC-4.13.001 | v1.14 | PASS |
| 7 | BC-2.07.001 | v1.5 | PASS |
| 8 | BC-2.02.011 | v1.7 | PASS |
| 9 | BC-3.08.001 | v1.21 | PASS |
| 10 | BC-1.17.001 | v1.6 | PASS |
| 11 | VP-094.md | v1.3 | **FINDING** (see F-P57-001 below — invocation signatures) |
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

29/30 attestations PASS; 1 FINDING (VP-094.md v1.3; F-P57-001 below).

---

## Part B — New Findings

### CRITICAL

*(none)*

### HIGH

*(none)*

### MEDIUM

#### F-P57-001: VP-094 §Proof Harness Skeleton Invokes Scripts With Superseded Named-Flag Signatures Contradicting ADR-030 §Decision 2/3 + BC-5.42.001

- **Severity:** MEDIUM
- **Category:** spec-fidelity (POLICY 4 semantic_anchoring_integrity + POLICY 5 creators_justify_anchors)
- **Location:** VP-094.md §Proof Harness Skeleton — test bodies `VP-094-B`, `VP-094-B-pass`, `VP-094-C`, `VP-094-C-pass`, `VP-094-C-nonrelease`

**Verbatim evidence (VP-094.md v1.3, §Proof Harness):**
```
    result=$(bash plugins/vsdd-factory/bin/check-stale-verdict.sh \
        --covered-sha "$covered_sha" \
        --live-sha "$live_sha" 2>&1) || true
```
```
    run bash plugins/vsdd-factory/bin/check-stale-verdict.sh \
        --covered-sha "$sha" \
        --live-sha "$sha"
```
```
    run bash plugins/vsdd-factory/bin/enforce-merge-strategy.sh \
        --branch "release/v1.0.0-rc.23" \
        --strategy "squash"
```
(same `--branch`/`--strategy` form in `VP-094-C-pass` and `VP-094-C-nonrelease`).

**Canonical SoT (contradicted):**
- ADR-030 §Decision 2: `**Invocation:** check-stale-verdict.sh <pr_number> <covered_sha>` — positional 2-arg; the live HEAD SHA is obtained *internally* via `gh pr view <pr_number> --json headRefOid`.
- ADR-030 §Decision 3: `**Invocation:** enforce-merge-strategy.sh <pr_number> [--merge|--squash|--rebase]` — the branch name is obtained *internally* via `gh pr view <pr_number> --json headRefName`.
- BC-5.42.001 §Description (b): "invoke `check-stale-verdict.sh <pr_number> <covered_sha>`. The script calls `gh pr view <pr_number> --json headRefOid`."
- BC-5.42.001 §Canonical Test Vectors keys on `(pr_number, covered_sha, mocked headRefOid)` — there is no `--live-sha` and no `--branch` parameter in the contract.

**Why it violates the policy:** VP-094's own §Property Statement PS-B/PS-C explicitly cite `ADR-030 §Decision 2` and `ADR-030 §Decision 3` as authority, but the §Proof Harness below invokes both scripts with named-flag signatures that contradict those very decisions and BC-5.42.001 §Description. Two distinct defects:
1. **Signature form:** `--covered-sha`/`--live-sha` and `--branch`/`--strategy` are not the SoT signatures (positional `<pr_number> <covered_sha>` and `<pr_number> [--merge|...]`). No `<pr_number>` is passed at all, despite both scripts requiring it as the `gh pr view` argument.
2. **Design-defeating seam:** `--live-sha` and `--branch` bypass the `gh pr view` fetch that is the *load-bearing behavior* of both scripts. `check-stale-verdict.sh` exists precisely to fetch the live HEAD (its whole purpose per D-749); a `--live-sha` parameter would reduce it to a pure two-value comparison that can never detect a real advanced HEAD in production (production callers do not know the live HEAD — that is what the script is for). An implementer following this harness literally would build a script that structurally cannot enforce the invariant.

This is the same anchor-fidelity class the D-812 gate (10th standing gate) targets, and it survived the pass-56 fix: the architect corrected sentinel strings and exit codes *inside these same harness lines* (`-eq 2 → -eq 1` on the line immediately after the `--covered-sha`/`--live-sha` invocation) but did not sweep the invocation signature. Note ADR-030's own v1.1 changelog records that this exact named→positional signature change was adjudicated (F-W1V-002) with "propagation directives issued for BC-5.42.001 §Architecture Anchors and S-19.01 §Architecture Mapping" — VP-094 was not named in that propagation, and is now the **sole** surviving artifact carrying the superseded named-flag form (grep for `--live-sha|--covered-sha|--strategy|--branch` across `.factory/` returns VP-094.md as the only spec that invokes these scripts with named flags; S-19.01 §File Structure lines describe both scripts as `gh pr view <pr_number> --json headRefOid`/`headRefName`, aligned to ADR-030). Blast radius = 1 file → MEDIUM per partial-fix regression discipline.

**Proposed routing:** architect (VP-094 is architect-owned; the corrected §Proof Harness must invoke `check-stale-verdict.sh <pr_number> <covered_sha>` with `gh` mocked to supply the live `headRefOid`, and `enforce-merge-strategy.sh <pr_number> [--squash|--merge]` with `gh` mocked to supply `headRefName`, per ADR-030 §Decision 2/§Decision 3 and BC-5.42.001 §Canonical Test Vectors).

### LOW

#### O-P57-001: VP-094-B False-RED Exit-Code Capture (FIXED same-burst; NOT accepted-with-record)

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** VP-094.md §Proof Harness `VP-094-B`

VP-094-B exit-code capture is broken: `result=$(bash ... 2>&1) || true` followed by `local exit_code=$?` captures the exit status of `true` (always 0), not the script's exit. The subsequent `[ "$exit_code" -eq 1 ]` therefore evaluates `[ 0 -eq 1 ]` and reports `FAIL: expected exit 1 (fail-closed) on stale verdict` even when the script correctly exits 1 — a false-RED. This line was touched by the D-812 fix (`-eq 2 → -eq 1`) without correcting the capture. Skeleton pending test-writer instantiation (`feasibility: feasible-pending-harness`), hence LOW. Recommend replacing with bats `run` + `$status` (as the sibling `VP-094-B-pass` test already does) so the exit code is captured correctly.

**Status:** FIXED same-burst (architect 6716b14b) — replaced with bats `run` + `$status`. NOT accepted-with-record. CLOSED.

#### O-P57-002: VP-094 PS-C "Exits 0 Unconditionally" Imprecise vs ADR-030 §Decision 3 (FIXED same-burst; NOT accepted-with-record)

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** VP-094.md §Property Statement PS-C

VP-094.md §Property Statement PS-C states: "For non-release PRs the hook exits 0 unconditionally (BC-5.42.001 PC3/INV-3)." ADR-030 §Decision 3 step 4 specifies the wrapper "Invokes `gh pr merge` with final flags; **propagates gh's exit code**" for non-release PRs — i.e., the exit is gh's code, not "0 unconditionally." Imprecise but low-signal; pre-existing (not in the D-812 delta). Recommend "delegates to `gh pr merge` and propagates its exit code" on the next VP-094 touch.

**Status:** FIXED same-burst (architect 6716b14b) — PS-C updated to "delegates to `gh pr merge` and propagates its exit code." NOT accepted-with-record. CLOSED.

---

## Axis Sweep

| Axis | Result |
|------|--------|
| D-812 sentinel-string values match BC/ADR SoT | PASS — `STALE_READY_VERDICT` (PS-B + harness line 138), `RELEASE_PR_SQUASH_FORBIDDEN` (PS-C + harness line 167) match BC §Description (b)/(c) + §Canonical Test Vectors + ADR-030 §Decision 2/3 |
| D-812 exit-code integers match SoT | PASS — exit 1 (PS-B "code 1", PS-C "exits 1", harness `-eq 1`) matches ADR-030 §Decision 2/3 ("exits 1 (fail-closed)"; "all exit 1, fail-closed"); BC "non-zero" is satisfied by exit 1 |
| D-812 stderr routing | PASS — PS-B "emit ... to stderr" matches ADR-030 §Decision 2 + BC §Description (b) "on stderr" |
| PS-C message text canonicalization | PASS — `branch <branch_name> requires --merge per RELEASING.md` verbatim vs BC §Description (c) + ADR-030 §Decision 3 |
| §Proof Harness invocation signature fidelity | **FAIL — F-P57-001** (named-flag `--covered-sha`/`--live-sha`/`--branch`/`--strategy` contradict ADR-030 §Decision 2/3 + BC §Description positional signatures) |
| POLICY 14/17 5-leg parity (VP-094 v1.3) | PASS — `version: "1.3"`, `last_amended:` v1.3 prefix, `modified[]` v1.3 entry, VP-INDEX Full Index v1.3 annotation, VP-INDEX Story Anchors v1.3 annotation all present same-burst |
| POLICY 9 (VP-INDEX → arch docs) | PASS — VP-094 present in both arch docs with verbatim H1 title; description-only annotation; integration=34 reconciles |
| POLICY 4 anchor-prose parity (module:, §Traceability, §Feasibility) | PASS — `module: plugins/vsdd-factory/agents/pr-manager.md`; §Traceability "Enforcement scripts: bin/check-stale-verdict.sh / bin/enforce-merge-strategy.sh" match BC §Architecture Anchors; SS-05 anchor correct |
| POLICY 1 append-only | PASS — no VP renumbered/removed; total_vps 101 unchanged |
| Sentinel/exit class sweep VP-095..VP-101 | PASS — no other VP invokes the pr-manager scripts; VP-098 NOT_FOUND(-5)/CAPABILITY_DENIED(-1), VP-101 NOT_FOUND(-5), VP-100 plugin.abandoned match their BCs |

### Standing Gate Roster (1–10)

| Gate | Result |
|------|--------|
| 1. D-794 BC-INDEX title parity | PASS (no BC title change in delta) |
| 2. D-795 ADR no version-token BC cites | PASS (ADR-030 body cites BC-5.42.001 with no `v[0-9]` load-bearing token) |
| 3. D-797 VP source_bc volatile-pin sweep | PASS (VP-094 source_bc uses stable §Postcondition 1+2+3 anchor form) |
| 4. D-798 pre-pass class-sweep completeness | **FAIL** — sentinel/exit class swept in pass-56 but sibling **signature** drift in the same §Proof Harness not swept (F-P57-001) |
| 5. D-800 index cells derive from own changelog | PASS (VP-INDEX VP-094 Full Index/Story Anchors rows derive from VP-094 modified[]/last_amended) |
| 6. D-801 remediation predicate enumeration | PASS (VP-INDEX v2.62 changelog enumerates VP-094 + 4-index) |
| 7. D-802 modified[] version-monotonicity | PASS (VP-094 modified[]: v1.1 → v1.2 → v1.3 ordered) |
| 8. D-803/D-808 STORY-INDEX/epic row parity | PASS (no epic/story-index change in delta) |
| 9. D-811 namespace/path sweep in §Traceability + §Proof Harness | PASS (bin/ prefix consistent in both §Traceability and all §Proof Harness invocations) |
| 10. D-812 PS-* + §Proof Harness sentinel/exit match SoT | PASS on sentinel/exit VALUES; the harness **signature** fidelity is a distinct sub-axis → F-P57-001 |

**Do-not-re-report list honored:** O-P41-001, O-P41-002, O-P44-001, O-P49-001 — none re-raised. F-P56-001 (sentinel/exit) confirmed CLOSED (VP-094 v1.3 sentinel strings + exit-1 + stderr all correct); F-P57-001 is a novel, distinct defect (invocation signature, not sentinel/exit).

---

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 1 |
| LOW | 2 |

**Overall Assessment:** pass-with-findings (B0/H0/M1/L2)  
**Convergence:** findings remain — streak remains 0/3 (was 0/3 entering pass-57; F-P57-001 MEDIUM prevents advancement)  
**Readiness:** requires fix burst (F-P57-001; O-P57-001/O-P57-002 fixed same-burst by architect 6716b14b); pass-58 required

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 57 |
| **New findings** | 1 (F-P57-001 — VP-094 §Proof Harness invocation-signature drift; novel axis: script invocation-form parity with ADR §Decision canonical positional/gh-fetch contract) |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (1 new / (1+0)) |
| **Median severity** | MEDIUM |
| **Trajectory** | ...→0→1→1→3 (passes 54/55/56/57 = 0,1,1,3) |
| **Verdict** | FINDINGS_REMAIN — streak 0/3 (was 0/3 entering; no advancement); three consecutive CLEANs required for 3/3 CONVERGED |

**Novelty note:** F-P57-001 exposes a novel defect sub-class within the anchor-fidelity family: **invocation-signature form** of §Proof Harness script invocations can contradict the ADR §Decision canonical form (positional vs named-flag) even when sentinel strings, exit codes, and namespace paths are all correct. The pass-56 fix (D-812) corrected sentinel/exit VALUES inside the same `--covered-sha`/`--live-sha` invocation lines without sweeping the invocation signature itself. ADR-030's own v1.1 changelog recorded a named→positional migration (F-W1V-002) with propagation directives for BC and story; VP-094 §Proof Harness was not enumerated in those directives, creating a dormant survivor. The 10 prior standing gates verified path/namespace, sentinel/exit, volatile-pin, and description parity — but not **invocation-form** parity with ADR §Decision canonical signatures. This is a new 11th gate axis: **POLICY 4 v1.4.5 anchor-prose-parity extends to §Proof Harness script invocation-form** — the signature (positional vs named-flag, including required arguments) must match the ADR §Decision canonical invocation form, because a mismatched invocation form defeats the test's ability to validate the load-bearing behavior. Codification recommendation: extend standing gate roster to 11 via D-815 (POLICY 4 v1.4.5 invocation-signature-form parity axis). **CODIFIED D-815.**

**Adversary self-correction note:** The adversary emitted a mid-line self-correction in the Verdict: "B0/H1... correction: B0/H0/M1/L2" — the HIGH count was initially misstated before inline correction. The actual verdict is B0/H0/M1/L2. This report is persisted verbatim per source-attestation discipline.

**Per-Policy Attestations (policies.yaml v1.4.4):**

| Policy | Gate Description | Result |
|--------|-----------------|--------|
| POLICY 1 | VP-INDEX append-only (no rows removed) | PASS |
| POLICY 2 | VP-INDEX Full Index ordering (VP-094..101 contiguous; no reordering) | PASS |
| POLICY 3 | state-manager runs LAST in burst | PASS (SM leg is this report's persister) |
| POLICY 4 v1.4.4 | Description-bearing anchor-prose parity for all VP anchors (module:, §Feasibility Artifact, §Traceability Function-anchor bullets, §Property Statement, §Proof Harness sentinel values) | **FINDING F-P57-001** — VP-094 §Proof Harness all five script-invoking @test blocks use named-flag invocation form contradicting ADR-030 §Decision 2/3 positional/gh-fetch canonical forms |
| POLICY 5 v1.3.8 | Category-(j) class sweep on stale-VP-prose findings | PASS (VP-095..101 swept for invocation-signature form; CLEAN) |
| POLICY 6 | ARCH-INDEX subsystem names canonical | PASS (no subsystem changes) |
| POLICY 7 | BC-INDEX title-cell verbatim parity | PASS (no BC title changes) |
| POLICY 8 v1.3 | BC frontmatter array atomic propagation | PASS (no BC frontmatter changes) |
| POLICY 9 | VP title-change propagation to verification-architecture.md + verification-coverage-matrix.md | PASS (VP-094 description-only annotation; no title change; no propagation required) |
| POLICY 14 | 5-leg quintuple parity on all version bumps | PASS (VP-094.md v1.3: version:, body Changelog, modified[], last_amended, VP-INDEX Full Index + Story Anchors — architect 93d3ca03) |
| POLICY 16 | D-NNN global-max gate | PASS (D-815 allocated after verifying D-813 is current max in decision-log; D-814 consumed by dispatch-side STATE.md-only advance) |
| POLICY 19 | No volatile pins introduced | PASS (no file:line citations added) |

**Iron Law compliance:** Confirmed. Fresh context for pass-57. Prior pass reports NOT loaded (only adv-E19-pass-56.md Part A read per Iron Law). Rubric policies.yaml v1.4.4 applied.
