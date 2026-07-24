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
pass: 1
previous_review: null
story: S-21.02
cycle: v1.0-brownfield-backfill
verdict: NOT-CLEAN
reviewed_head: 631edb28
reviewed_branch: feature/S-21.02-post-rebase-diff-integrity-gate
base_commit: 7bb0e797
date: 2026-07-23
---

# S-21.02 LOCAL Adversary Pass-1 — NOT-CLEAN

**Date:** 2026-07-23
**Story:** S-21.02 — Post-rebase diff-integrity gate
**Pass:** 1 of BC-5.39.001 cascade
**Result:** NOT-CLEAN — streak 0/3
**Severity breakdown:** B0 / H1 / M3 / L2 / NITPICK0 / OBS2
**Total findings:** 6 findings + 2 observations
**Reviewed diff:** HEAD 631edb28 on feature/S-21.02-post-rebase-diff-integrity-gate vs base 7bb0e797

---

## Finding ID Convention

Finding IDs for this story's local cascade use the format: `F-S2102-P<PASS>-<SEQ>`

- `F`: Fixed prefix for factory local adversary findings
- `S2102`: Story identifier (S-21.02 compact form)
- `P<PASS>`: Pass number (e.g., `P1`, `P2`)
- `<SEQ>`: Three-digit sequence within the pass (e.g., `001`)

Observations use `O-<SEQ>` (no severity component; informational only).

Examples: `F-S2102-P1-001` (HIGH finding, pass 1, first finding), `O-1` (first observation).

---

## Part B — New Findings (or all findings for pass 1)

### HIGH

#### F-S2102-P1-001 — T-001..T-003 git fixtures unconnected to gate logic

- **Severity:** HIGH
- **Category:** coverage-gap
- **Location:** bats test file (T-001, T-002, T-003 fixture setup blocks) / AC-003(a)(b)(c), AC-004, AC-005
- **Description:** The bats test file constructs three git fixture repositories covering the pre-rebase, post-rebase, and conflict scenarios. However, none of these fixtures are ever fed as input to the actual gate under test. The pass/fail decisions for AC-003(a), AC-003(b), AC-003(c), AC-004, and AC-005 are resolved entirely by grepping the `§Inter-Wave Rebase` markdown section of story files rather than by invoking the gate implementation against the constructed git state.
- **Evidence:** In T-001, T-002, and T-003: fixture construction (`git init`, `git commit`) appears in setup but no subsequent test assertion calls the gate binary or invokes the gate script with the fixture repository as `$GIT_DIR` or as a working-directory argument. All gate-verdict assertions (`assert_output --partial "BLOCKED"`, `assert_success`) reference static markdown grep output.
- **Proposed Fix:** Rewrite T-001..T-003 as an executable harness per the AC gate column pattern ("bats: mock git diff...; assert force-push blocked"): invoke the gate binary/script with the fixture repository as the working directory; assert block/allow decision from the gate's exit code and stdout. PO escalation reserved if test-writer demonstrates the pattern is unsatisfiable given current AC specification.
- **Routing:** test-writer

---

### MEDIUM

#### F-S2102-P1-002 — PC2 STOP block truncates BC-5.44.001 PC2 MUST-emit contract

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** gate implementation (PC2 STOP block)
- **Description:** The implementation's PC2 STOP block emits only a one-line error message before returning exit code 2. BC-5.44.001 PC2 specifies a MUST-emit contract that includes: (1) the identity of the file(s) at risk (file path + blob SHA), (2) the reason the diff-integrity check failed (range-diff deviation vs. --stat divergence), (3) four explicit remediation steps the user must follow, and (4) the diagnostic command to re-run for confirmation.
- **Evidence:** The current implementation drops items (1)–(4), emitting only a generic "force-push blocked: diff-integrity check failed" line.
- **Proposed Fix:** Expand PC2 STOP block to emit the full BC-5.44.001 PC2 contract payload: file(s) at risk, failure reason, four remediation steps, diagnostic re-run command.
- **Routing:** implementer

---

#### F-S2102-P1-003 — Pre-rebase tip not captured; range-diff detector is an unbound placeholder

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** gate implementation (range-diff detector invocation)
- **Description:** The gate's primary integrity detector is documented as a `git range-diff` comparison between the pre-rebase and post-rebase commit ranges. However, the implementation captures the post-rebase tip via `git rev-parse HEAD` but never captures the pre-rebase tip. The `PRE_REBASE_TIP` variable used in the range-diff invocation is referenced but never assigned. The capture instruction appears in step comments ordered AFTER the rebase has already executed.
- **Evidence:** `PRE_REBASE_TIP` is undefined at range-diff invocation; the command runs with an unset variable, silently producing empty diff (always passes) or failing with a swallowed git parse error.
- **Proposed Fix:** Capture pre-rebase tip via `git rev-parse HEAD` BEFORE the rebase executes; bind `PRE_REBASE_TIP`; ensure range-diff invocation receives both range bounds.
- **Routing:** implementer

---

#### F-S2102-P1-004 — Tautological fixture-sanity checks + malformed git diff invocation

- **Severity:** MEDIUM
- **Category:** coverage-gap
- **Location:** bats test file (fixture-sanity assertions, T-004)

**(a) Tautological fixture-sanity (POLICY 11 violation):** Several bats tests assert that the fixture repository was constructed successfully by checking `[ -d "$FIXTURE_DIR/.git" ]`. This checks a property of the test setup itself, not of the system under test. Per POLICY 11 (no tautological assertions), fixture-sanity checks must not substitute for behavioral assertions.

**(b) Malformed git diff invocation:** T-004 uses `git diff origin/develop -- autoload.gd --stat`. The `--stat` flag placed after the `--` path-separator is parsed by git as a literal pathspec named `--stat`, not as the summary flag. The command passes for the wrong reason: git finds no file named `--stat` and produces empty output.

- **Evidence:** (a) `[ -d "$FIXTURE_DIR/.git" ]` assertions in T-001..T-003 preambles. (b) `git diff origin/develop -- autoload.gd --stat` at T-004.
- **Proposed Fix:** (a) Remove tautological setup assertions; replace with behavioral assertions against gate output. (b) Correct `--stat` position to precede `--`: `git diff --stat origin/develop -- autoload.gd`.
- **Routing:** test-writer

---

### LOW

#### F-S2102-P1-005 — Bats header cites stale "ADR-031 v1.3" version pin

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** bats test file header block
- **Description:** The bats test file header block cites `# ADR-031 v1.3` as the authoritative reference for the tool-filter anchoring convention. ADR-031 is currently at v1.5. The v1.3 pin is a stale volatile citation contra TD-VSDD-091.
- **Evidence:** Header comment `# ADR-031 v1.3` in bats test file preamble; current ADR-031 is v1.5.
- **Proposed Fix:** Update header citation to stable form per TD-VSDD-091: `# ADR-031` (no version suffix) or `# ADR-031 (current)`.
- **Routing:** test-writer

---

#### F-S2102-P1-006 — Escalate-on-failure covers only --stat failure; PC4/EC-006/EC-007 untested

- **Severity:** LOW
- **Category:** coverage-gap
- **Location:** gate implementation (escalation handler) + bats test file (test matrix)
- **Description:** The gate's escalation-on-failure path is implemented and tested only for the `git diff --stat` failure branch. The range-diff failure branch has no escalation handler: if `git range-diff` exits non-zero the gate either silently continues (treating missing output as a PASS) or aborts without the PC4 diagnostic emit specified by BC-5.44.001. PC4 (escalate-on-detector-failure), EC-006 (range-diff process error), and EC-007 (range-diff output parse error) are absent from the bats test matrix.
- **Evidence:** No escalation handler for range-diff non-zero exit; no PC4/EC-006/EC-007 rows in bats test file.
- **Proposed Fix:** Implementer: add escalation handler for range-diff failure path; ensure PC4 emit fires on any detector failure. Test-writer: add PC4/EC-006/EC-007 coverage rows to bats matrix.
- **Routing:** implementer (escalation handler doc) + test-writer (PC4/EC-006/EC-007 coverage)

---

### Observations (non-finding, informational)

#### O-1 — Role-ownership ambiguity in step-f Sub-step F.1

The story's `§Inter-Wave Rebase` Sub-step F.1 names the implementer as the agent responsible for the final push to origin. However `devops-engineer.md` owns the push step in the per-story delivery playbook (step-f is devops-engineer territory). This creates an ambiguous ownership handoff. No behavioral contract is violated but the ownership gap should be made explicit.

**Suggested resolution:** Add an explicit ownership attribution line to Sub-step F.1 (`owner: devops-engineer`) or cross-reference `devops-engineer.md` step-f explicitly.

**Routing:** implementer (add explicit ownership line to Sub-step F.1)

#### O-2 — BC-5.44.001 Invariant 3 sibling-set computation not operationalized

BC-5.44.001 Invariant 3 specifies that the gate must compute the sibling-set for the rebased branch (the set of commits sharing the same merge-base as the rebased branch against `origin/develop`). The current implementation does not perform merge-base sequence computation, comparing HEAD directly against a fixed reference. For branches branched off a non-current develop tip, this produces false positives.

**Suggested resolution:** Compute merge-base via `git merge-base HEAD origin/develop` and use that as the range lower-bound for both range-diff and --stat comparisons.

**Routing:** implementer (operationalize Invariant 3 sibling-set computation)

---

### Clean Axes Verified

- **Scope discipline:** Only 3 File-Structure files touched in the diff (gate script, bats test file, story spec update). No lateral scope expansion.
- **pr-manager.md:** Untouched in this diff. Story delivery ownership chain intact.
- **POLICY 21 compliance:** No new `.sh` files introduced.
- **TD-VSDD-091:** Spec narrative cites behavioral anchors (BC-5.44.001 clause identifiers) rather than line numbers. Excepted ADR version citation covered under F-S2102-P1-005.

---

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER  | 0 |
| HIGH     | 1 |
| MEDIUM   | 3 |
| LOW      | 2 |
| NITPICK  | 0 |
| OBS      | 2 |

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Readiness:** requires revision (fix burst: implementer then test-writer per routing table above)

**Orchestrator disposition — F-S2102-P1-001 routing rationale:** Routed to test-writer, NOT product-owner. The story AC gate column itself specifies the executable pattern ("bats: mock git diff...; assert force-push blocked"), satisfiable via a bats harness executing the documented gate procedure. PO escalation reserved if test-writer demonstrates unsatisfiability.

**Recommended fix burst dispatch:**
1. **implementer** — F-S2102-P1-002, F-S2102-P1-003, F-S2102-P1-006 (doc half), O-1, O-2
2. **test-writer** — F-S2102-P1-001, F-S2102-P1-004, F-S2102-P1-005, F-S2102-P1-006 (test half)

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 1 |
| **New findings** | 6 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (6 / (6 + 0)) |
| **Median severity** | 3.0 (MEDIUM) |
| **Trajectory** | 6 |
| **Verdict** | FINDINGS_REMAIN |
