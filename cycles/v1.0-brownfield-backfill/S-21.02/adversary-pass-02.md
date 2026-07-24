---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-07-23T00:00:00Z
phase: 5
inputs: []
input-hash: "[live-state]"
traces_to: ".factory/stories/STORY-S-21.02-post-rebase-diff-integrity-gate.md"
pass: 2
previous_review: "adversary-pass-01.md"
story: S-21.02
cycle: v1.0-brownfield-backfill
verdict: NOT-CLEAN
reviewed_head: 715a2f17
reviewed_branch: feature/S-21.02-post-rebase-diff-integrity-gate
base_commit: 7bb0e797
date: 2026-07-23
---

# S-21.02 LOCAL Adversary Pass-2 — NOT-CLEAN

**Date:** 2026-07-23
**Story:** S-21.02 — Post-rebase diff-integrity gate
**Pass:** 2 of BC-5.39.001 cascade
**Result:** NOT-CLEAN — streak 0/3
**Severity breakdown:** B0 / H1 / M2 / L1 / NITPICK0 / OBS2
**Total findings:** 4 findings + 2 observations
**Reviewed diff:** HEAD 715a2f17 on feature/S-21.02-post-rebase-diff-integrity-gate vs base 7bb0e797

---

## Finding ID Convention

Finding IDs for this story's local cascade use the format: `F-S2102-P<PASS>-<SEQ>`

- `F`: Fixed prefix for factory local adversary findings
- `S2102`: Story identifier (S-21.02 compact form)
- `P<PASS>`: Pass number (e.g., `P1`, `P2`)
- `<SEQ>`: Three-digit sequence within the pass (e.g., `001`)

Observations use `O-<SEQ>` (no severity component; informational only).

Examples: `F-S2102-P2-001` (MEDIUM finding, pass 2, first finding), `OBS-1` (first observation).

---

## Part A — Pass-1 Finding Closure Review

All 8 pass-1 findings are CONFIRMED-FIXED. No paper-fixes detected. Every closure is load-bearing.

| Finding | Pass-1 Severity | Status | Closure Evidence |
|---------|----------------|--------|------------------|
| F-S2102-P1-001 | HIGH | CONFIRMED-FIXED | f5ea156a: real-rebase fixtures (git init + cherry-pick + conflict resolution) replace static markdown grep; T-001 now invokes gate binary with fixture repo as working directory; exit code and stdout asserted against BLOCK/ALLOW decision |
| F-S2102-P1-002 | MEDIUM | CONFIRMED-FIXED | f5ea156a: PC2 STOP block expanded to emit full BC-5.44.001 PC2 contract payload: file(s) at risk with blob SHA, failure reason (range-diff deviation vs --stat divergence), four enumerated remediation steps, diagnostic re-run command |
| F-S2102-P1-003 | MEDIUM | CONFIRMED-FIXED | f5ea156a: PRE_REBASE_TIP captured via `git rev-parse HEAD` BEFORE rebase executes; bound as range-diff lower bound; capture ordering verified correct in script |
| F-S2102-P1-004 | MEDIUM | CONFIRMED-FIXED | f5ea156a: tautological `[ -d "$FIXTURE_DIR/.git" ]` setup assertions removed; replaced with behavioral assertions against gate output. `git diff --stat` flag repositioned before `--` path separator |
| F-S2102-P1-005 | LOW | CONFIRMED-FIXED | f5ea156a: header citation updated from stale `# ADR-031 v1.3` to stable `# ADR-031` per TD-VSDD-091 |
| F-S2102-P1-006 | LOW | CONFIRMED-FIXED | f5ea156a: range-diff failure escalation handler added; PC4 emit fires on any detector failure; EC-006/EC-007 rows added to bats test matrix |
| O-1 | OBS | CONFIRMED-FIXED | 6eb4b6a2: Sub-step F.1 ownership attribution line added (`owner: devops-engineer`); cross-reference to `devops-engineer.md` step-f explicit |
| O-2 | OBS | CONFIRMED-FIXED | f5ea156a: merge-base computed via `git merge-base HEAD origin/develop`; used as range lower-bound for both range-diff and --stat comparisons; simulation-proxy comment added |

---

## Part B — New Findings

### HIGH

#### F-S2102-P2-003 — BC-5.44.001 H1/Description named wrong actor "pr-manager"; partial v1.3 sweep residue

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** BC-5.44.001 H1 title + Description section; story S-21.02 v1.4 cite sweep
- **Description:** The BC-5.44.001 H1 title and Description as of the pass-1 fix burst cited "pr-manager and orchestrator" as the executing agent for the post-rebase diff-integrity gate. This is factually incorrect: the gate is a per-story delivery harness obligation belonging to devops-engineer (for inter-wave rebases) and implementer (for per-story delivery rebases). "pr-manager" is the PR lifecycle coordinator and has no rebase responsibility. The v1.3 text was a residue from the initial BC draft before the story-writer's actor-resolution sweep.
- **Evidence:** BC-5.44.001 v1.3 H1: "pr-manager and orchestrator MUST run a post-rebase diff-integrity gate...". The corrected actor is devops-engineer (inter-wave) and implementer (per-story), consistent with BC-5.44.001's own §Preconditions (PC1 names devops-engineer) and the story's §Inter-Wave Rebase ownership attribution added at O-1 closure (6eb4b6a2).
- **Routing:** product-owner (BC actor correction) — FIXED at 7a6111f1: BC-5.44.001 v1.3→v1.4 H1+Description actor corrected; c73152fb: story S-21.02 v1.4→v1.5 propagation cite sweep.
- **Status:** FIXED

---

### MEDIUM

#### F-S2102-P2-001 — range-diff primary never functionally exercised; fixtures never rebased; rd_out unparsed

- **Severity:** MEDIUM
- **Category:** coverage-gap
- **Location:** bats test file (T-001 fixture setup); gate implementation (range-diff invocation + output parsing)
- **Description:** The pass-1 fix routed F-S2102-P1-001 to test-writer with the intent to replace static markdown grep with real gate invocation. The resulting test suite constructed git fixtures but did not perform an actual rebase: the fixture's "post-rebase" state was fabricated by direct commits rather than by running `git rebase`, meaning the range-diff detector's pre-rebase-tip vs post-rebase-tip range was always empty (same commit). Additionally, the range-diff output (`rd_out`) was captured but never parsed for `!`-prefixed commit lines (the BC-5.44.001 signal for reordered/dropped commits). The range-diff detector's primary signal path was functionally untested.
- **Evidence:** T-001 fixture: `git cherry-pick` used to approximate post-rebase state; no `git rebase` executed; PRE_REBASE_TIP == POST_REBASE_TIP in the fixture sequence. Gate script: `rd_out=$(git range-diff ...)` captured; no subsequent `grep '^!'` or equivalent parse of rd_out present.
- **Routing:** test-writer — FIXED at f5ea156a: real-rebase fixtures added (conflict resolved via `-X theirs`); two-range range-diff parsing added with `!`-commit pre-assertion; gate invoked against rebased fixture repo.
- **Status:** FIXED
- **Honest caveat:** The T-001 fixture triggers range-diff `!` via a real conflict resolved with `-X theirs` (feature deletion wins over sibling rename). This exercises the full detection pipeline but is NOT a classical adjacent-hunk ORT silent-drop, which is hard to construct deterministically. This limitation is documented in fixture comments.

---

#### F-S2102-P2-002 — Undocumented harness condition (c) diverges from documented algorithm (a)+(b)

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** gate implementation (integrity check algorithm); BC-5.44.001 §Algorithm
- **Description:** The gate implementation contained a third integrity-check branch (condition c) that was not present in BC-5.44.001's two-condition algorithm ((a) range-diff deviation, (b) --stat divergence). Condition (c) fired on merge-base computation failure and returned PASS-with-warning, diverging from the documented fail-closed contract under precondition failures. This undocumented branch created an exploitable gap: a merge-base failure would silently permit a force-push that condition (a)+(b) would have blocked.
- **Evidence:** Gate script contained `elif [ -z "$MERGE_BASE" ]; then echo "WARNING: merge-base unavailable, skipping range-diff"; exit 0` — absent from BC-5.44.001 §Algorithm and contradicting PC4 (escalate-on-detector-failure).
- **Routing:** test-writer — FIXED at f5ea156a: condition (c) removed; merge-base failure now routes through PC4 escalation handler (exit 2 with full diagnostic); T-005 sub-case C added for merge-base failure path.
- **Status:** FIXED

---

### LOW

#### F-S2102-P2-004 — step-f AC-002 deliverable zero automated coverage

- **Severity:** LOW
- **Category:** coverage-gap
- **Location:** bats test file (test matrix); story S-21.02 AC-002 gate column
- **Description:** AC-002 specifies a deliverable: the gate script must emit a structured JSON report to a log path before exiting. The AC-002 gate column entry specifies "bats: assert JSON report written to log path". No test in the bats matrix asserted the JSON report's existence, content structure, or file path targeting.
- **Evidence:** No `assert_output --partial '"gate_result"'` or `[ -f "$LOG_PATH" ]` assertion in any bats test. AC-002 gate column entry present but unenforced.
- **Routing:** test-writer — FIXED at f5ea156a: step-f doc-parity markers added; AC-002 JSON report assertion added to test matrix.
- **Status:** FIXED

---

### Observations (non-finding, informational)

#### OBS-1 — merge-base failure fail-open; both legs now covered

The original gate's fail-open behavior on merge-base failure (condition c, noted in F-S2102-P2-002) was previously documented in a comment as "acceptable for rare network partition scenarios." The remediation correctly removes this carve-out. Both documentary leg (6eb4b6a2: escalate-branch path added to story spec) and harness leg (f5ea156a: T-005 sub-case C) are covered. No residue.

#### OBS-2 — Intentionality keyword-grep proxy

The range-diff `!`-commit detection uses a keyword-grep proxy (`grep -E '^!'`) rather than parsing the full range-diff structured output. This is a simulation proxy per BC-5.44.001's intent (the `!` prefix is the canonical range-diff signal for reordered/dropped commits). A simulation-proxy comment has been added at f5ea156a. This is an accepted design limitation: full range-diff structured parsing is out of scope for v1.0; the `!`-grep is deterministic and correct for the documented threat class.

---

### Clean Axes Verified

- **All 8 pass-1 findings closed load-bearing:** confirmed (see Part A table above).
- **Scope discipline:** Fix burst touched only gate script, bats test file, story spec, and BC file. No lateral expansion.
- **POLICY 21 compliance:** No new `.sh` files introduced.
- **TD-VSDD-091:** Spec narrative cites behavioral anchors (BC-5.44.001 clause identifiers) not line numbers.
- **BC-5.44.001 v1.4 actor correction propagated:** story S-21.02 v1.5 cites corrected actor per c73152fb.

---

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER  | 0 |
| HIGH     | 1 |
| MEDIUM   | 2 |
| LOW      | 1 |
| NITPICK  | 0 |
| OBS      | 2 |

**Overall Assessment:** block
**Convergence:** findings remain (all fixed in-burst) — streak 0/3, iterate to pass-3
**Readiness:** all pass-2 findings FIXED in-burst (7a6111f1, c73152fb, 6eb4b6a2, f5ea156a); pass-3 adversary cascade required

**All pass-2 findings were fixed during the fix burst. Pass-3 adversary review required for streak progression.**

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 2 |
| **New findings** | 4 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (4 / (4 + 0)) |
| **Median severity** | 2.5 (between HIGH and MEDIUM) |
| **Trajectory** | 6 → 4 |
| **Verdict** | FINDINGS_REMAIN |
