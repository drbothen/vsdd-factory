---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-07-24T00:00:00Z
phase: 5
inputs: []
input-hash: "[live-state]"
traces_to: ".factory/stories/STORY-S-21.03-pr-manager-orphan-merge.md"
pass: 1
previous_review: null
story: S-21.03
cycle: v1.0-brownfield-backfill
verdict: NOT-CLEAN
reviewed_head: 3e2c88d7
reviewed_branch: feature/S-21.03-pr-manager-orphan-merge
base_commit: a4a79f09
date: 2026-07-24
---

# S-21.03 LOCAL Adversary Pass-1 — NOT-CLEAN

**Date:** 2026-07-24
**Story:** S-21.03 — PR-manager orphan-merge handling
**Pass:** 1 of BC-5.39.001 cascade
**Result:** NOT-CLEAN — streak 0/3
**Severity breakdown:** B0 / H0 / M2 / L1 / NITPICK0 / OBS3
**Total findings:** 3 findings + 3 observations
**Reviewed diff:** HEAD 3e2c88d7 on feature/S-21.03-pr-manager-orphan-merge vs base a4a79f09

---

## Finding ID Convention

Finding IDs for this story's local cascade use the format: `F-S2103-P<PASS>-<SEQ>`

- `F`: Fixed prefix for factory local adversary findings
- `S2103`: Story identifier (S-21.03 compact form)
- `P<PASS>`: Pass number (e.g., `P1`, `P2`)
- `<SEQ>`: Three-digit sequence within the pass (e.g., `001`)

Observations use `O-<SEQ>` (no severity component; informational only).

Examples: `F-S2103-P1-001` (MEDIUM finding, pass 1, first finding), `OBS-1` (first observation).

---

## Part B — New Findings (or all findings for pass 1)

### MEDIUM

#### F-S2103-P1-001 — Load-bearing ancestry assertion placed after destructive branch deletion

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** pr-manager.sh Step 9 (Step 9-pre-A ancestry check) relative to Step 8 branch deletion
- **Description:** The implementation places the Step 9-pre-A ancestry assertion — which verifies that the merged PR commit is an ancestor of the trunk — AFTER Step 8's destructive remote branch deletion. BC-6.10.002 PC3 mandates that the post-merge ancestry check fire "immediately" after state: MERGED is confirmed. Placing this assertion downstream of branch deletion degrades the recovery narrative for the orphan-merge scenario the story targets: if the ancestry check fails at this position, the remote branch has already been deleted and the operator cannot use the standard recovery path that assumes branch existence. For the precise failure mode (orphan merge with no ancestry) that S-21.03 exists to handle, the post-delete position renders the ancestry gate misleading rather than protective.
- **Evidence:** Step ordering in pr-manager.sh: Step 8 (`git push origin --delete <branch>`) precedes Step 9-pre-A (`git merge-base --is-ancestor <sha> origin/develop`). The orphan-merge handling block and recovery instructions in Step 9 assume the remote branch is already gone at the point the error is surfaced.
- **Proposed Fix:** Relocate the Step 9-pre-A ancestry assertion to execute immediately after Step 7's merge confirmation (state: MERGED confirmed) and before Step 8's branch deletion. This preserves recovery options when the assertion fails and matches BC-6.10.002 PC3 "immediately" semantics.
- **Routing:** implementer (relocate assertion before branch deletion; orchestrator adjudicated spec-wins — BC text unambiguous, no PO escalation needed)

---

#### F-S2103-P1-002 — Harness is divergent parallel algorithm: direction, message-body, and consequence parity not tested

- **Severity:** MEDIUM
- **Category:** coverage-gap
- **Location:** bats test file (harness structure) / S-21.02 F-P2-002 class recurrence
- **Description:** The bats harness for S-21.03 implements a parallel reimplementation of the pr-manager Step 9 decision logic rather than invoking the implementation script against controlled stubs and asserting on its output. As a result, the harness tests capture gate deletion (whether a guard was removed) but do NOT detect: (1) inverted decision logic in the implementation (e.g., ancestor check sense reversed), (2) message truncation or body drift in the emitted WARN/BLOCK text, or (3) consequence misalignment (correct gate fires but wrong exit code or subsequent step is skipped). This is a recurrence of the S-21.02 F-P2-002 class finding: doc-parity token-greps are not a substitute for behavioral harness assertions.
- **Evidence:** Harness assertion pattern: `grep "ORPHAN_MERGE_DETECTED"` against static script text, not against live stdout produced by running the implementation with a constructed fixture. No direction-assertion (i.e., no test that inverted logic exits with a different code). No message-body assertion against emitted text.
- **Proposed Fix:** Extend bats tests to include direction assertions (inject a fixture where the merge commit IS a trunk ancestor and verify ALLOW; inject one where it IS NOT and verify BLOCK with the prescribed exit code and message body) and consequence assertions (verify that Step 8 branch deletion does NOT execute when Step 9-pre-A fires BLOCK).
- **Routing:** test-writer (direction + message-body + consequence parity assertions)

---

### LOW

#### F-S2103-P1-003 — Harness JSON regex requires no-space-after-colon; real gh output pretty-prints with spaces

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** bats test file (JSON fixture stubs) / Step 5 gh-pr-view parsing
- **Description:** The bats JSON fixture stubs used to simulate `gh pr view --json` output are constructed in compact form without spaces after colons (e.g., `{"state":"MERGED","mergeCommit":{"oid":"abc"}}`). The implementation's JSON parser (awk/grep/jq path) is tested only against this compact form. In production, `gh pr view --json` pretty-prints output with spaces after colons and newlines between fields. A regex anchored to no-space-after-colon will misparse real `gh` output, silently producing an empty merge-commit OID and causing a false ORPHAN_MERGE_DETECTED classification.
- **Evidence:** All `echo '{"state":...}'` fixture stubs in bats test file use compact JSON. No tolerant-parser test (e.g., `echo '{ "state": "MERGED", ... }'`) exists in the matrix.
- **Proposed Fix:** Update fixture stubs to use realistic pretty-printed JSON matching actual `gh` output format; update the parser to be tolerant of whitespace (prefer `jq -r` invocation or whitespace-tolerant awk over rigid regex).
- **Routing:** test-writer (realistic stub JSON + tolerant parser)

---

### Observations (non-finding, informational)

#### OBS-1 — Trunk hardcoded to `develop`; BC says "configured trunk"

- **Severity:** LOW (informational)
- **Category:** spec-fidelity
- **Location:** pr-manager.sh Step 9-pre-A; Merge-Strategy Gate
- **Description:** The implementation hardcodes `origin/develop` as the trunk reference for the ancestry check. BC-6.10.002 PC3 refers to "the configured trunk branch" without fixing a name. Projects where trunk is `main` or a custom branch name would produce a false-fail from the ancestry gate. This also creates incoherence with the Merge-Strategy Gate, which resolves the configured trunk dynamically.
- **Suggested resolution:** Read trunk from git config (`git config vsdd.trunk-branch` or fall back to `develop`) so the same pr-manager.sh works on non-develop-trunk repos. Align with BC phrasing and Merge-Strategy Gate coherence.
- **Routing:** implementer (BC phrasing coherence + Merge-Strategy Gate alignment)

#### OBS-2 — POLICY 21 detection is .sh-extension-based; new extensionless shims satisfy letter but evade detector [process-gap]

- **Severity:** LOW (informational; process-gap)
- **Category:** governance
- **Location:** fixtures/pr-manager-trunk/gh, fixtures/pr-manager-trunk/git (extensionless shim files)
- **Description:** The new test-double shims introduced in this story (`fixtures/pr-manager-trunk/gh` and `fixtures/pr-manager-trunk/git`) have no `.sh` extension. POLICY 21 detection is currently implemented as an `.sh`-extension scanner. These shims are functionally identical to the D-846-grandfathered `stub-gh.sh` class — `#!/bin/sh` executables standing in for real binaries — but evade the POLICY 21 detector because they lack the `.sh` suffix. This is not a blocker for this story; both classes of shim are semantically equivalent and the exemption rationale is clear. However, the governance gap (shebang-based detection vs. extension-based detection) should be resolved at the merge gate.
- **Suggested resolution:** Orchestrator disposition: governance question queued for human at the S-21.03 merge gate. Options: (a) explicitly exempt test-double shims (by directory convention `fixtures/**`) in the POLICY 21 detector, or (b) extend detection to shebang-based scanning. Follow-up handling per Cycle-Closing Checklist at story close.
- **Routing:** Not a story-scope blocker. Human decision at merge gate.

#### OBS-3 — Step 9 stale claim about remote branch deletion verification (pre-existing, in-scope)

- **Severity:** LOW (informational; pre-existing)
- **Category:** spec-fidelity
- **Location:** pr-manager.sh Step 9 inline comment / step narrative
- **Description:** Step 9 contains the claim "remote branch verified deleted (ls-remote returning empty)" which is stated as a general postcondition. This claim is false for two in-scope failure paths: (1) protection-rejected deletion (the branch still exists on origin) and (2) fork PRs (the head ref is on a fork and the deletion command targets the wrong remote). Step 8c uses `--exit-code` semantics which are not equivalent to the ls-remote empty-return claim in Step 9's narrative. The mismatch misleads recovery operators who follow Step 9's narrative to diagnose a failed deletion.
- **Suggested resolution:** Update Step 9 narrative to accurately describe Step 8c's `--exit-code` semantics and add explicit clauses covering the protection-rejected and fork-PR cases.
- **Routing:** implementer (correct Step 9 stale claim)

---

### Clean Axes Verified

- **Scope discipline:** Files touched confined to pr-manager.sh, bats test file, and story spec update. No lateral scope expansion.
- **POLICY 21 compliance (letter):** No new `.sh` files introduced (extensionless shims satisfy the letter of POLICY 21; governance gap recorded as OBS-2).
- **TD-VSDD-091:** Spec narrative cites behavioral anchors (BC-6.10.002 clause identifiers) rather than line numbers.
- **POLICY 19:** No hardcoded credentials or secrets in fixture stubs.
- **Step-numbering disambiguation:** Step 9-pre-A label unambiguous within the step sequence; no numbering collision with existing steps.
- **Novelty:** HIGH (pass 1 baseline; no prior pass to carry forward).

---

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER  | 0 |
| HIGH     | 0 |
| MEDIUM   | 2 |
| LOW      | 1 |
| NITPICK  | 0 |
| OBS      | 3 |

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Readiness:** requires revision (fix burst: implementer + test-writer concurrent per routing table above)

**Orchestrator disposition — F-S2103-P1-001 routing rationale:** Routed to implementer (NOT product-owner). BC-6.10.002 PC3 text is unambiguous on "immediately after state: MERGED" ordering; the fix is a mechanical relocation of the assertion block in the implementation, not a spec change. No PO escalation needed.

**Orchestrator disposition — OBS-2 process-gap:** Not a story-scope blocker. Governance question (exempt test-double shims explicitly vs. extend detection to shebang scanning) queued for human at the S-21.03 merge gate. Follow-up handling per Cycle-Closing Checklist at story close.

**Recommended fix burst dispatch (concurrent):**
1. **implementer** — F-S2103-P1-001 (relocate assertion), OBS-1 (trunk config), OBS-3 (Step 9 stale claim)
2. **test-writer** — F-S2103-P1-002 (direction + message-body + consequence parity), F-S2103-P1-003 (realistic JSON stubs + tolerant parser)

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 1 |
| **New findings** | 3 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (3 / (3 + 0)) |
| **Median severity** | 2.5 (MEDIUM–LOW boundary) |
| **Trajectory** | 3 |
| **Verdict** | FINDINGS_REMAIN |
