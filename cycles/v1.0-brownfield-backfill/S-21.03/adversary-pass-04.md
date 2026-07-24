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
pass: 4
previous_review: "adversary-pass-03.md"
story: S-21.03
cycle: v1.0-brownfield-backfill
verdict: NOT-CLEAN
reviewed_head: 2a6fa036
reviewed_branch: feature/S-21.03-pr-manager-orphan-merge
base_commit: a4a79f09
date: 2026-07-24
---

# S-21.03 LOCAL Adversary Pass-4 — NOT-CLEAN

**Date:** 2026-07-24
**Story:** S-21.03 — PR-manager orphan-merge handling
**Pass:** 4 of BC-5.39.001 cascade
**Result:** NOT-CLEAN — streak 0/3
**Severity breakdown:** B0 / H2 / M2 / L1 / NITPICK0 / OBS2 (+1 process-gap)
**Total findings:** 5 findings + 2 observations + 1 process-gap
**Reviewed diff:** HEAD 2a6fa036 on feature/S-21.03-pr-manager-orphan-merge vs base a4a79f09

---

## Finding ID Convention

Finding IDs for this story's local cascade use the format: `F-S2103-P<PASS>-<SEQ>`

- `F`: Fixed prefix for factory local adversary findings
- `S2103`: Story identifier (S-21.03 compact form)
- `P<PASS>`: Pass number (e.g., `P4`)
- `<SEQ>`: Three-digit sequence within the pass (e.g., `001`)

Examples: `F-S2103-P4-001` (HIGH finding, pass 4, first finding).

---

## Part A — Prior Pass Finding Resolution

### F-S2103-P3-001 — Previous Story Intelligence stale "step 9" reference

- **Pass-3 verdict:** MEDIUM
- **Pass-4 resolution:** CONFIRMED-FIXED
- **Evidence:** §Previous Story Intelligence section confirmed updated to Step 8-post-A consistently with §Tasks and §Architecture Compliance Rules sections at 9b020a98 (story v1.5). No residual "step 9" cites in non-historical sections.

---

### F-S2103-P3-002 — Recovery affordance contingent on `delete_branch_on_merge`; not deletion-agnostic

- **Pass-3 verdict:** MEDIUM
- **Pass-4 resolution:** CONFIRMED in BC+pr-manager; ADR sibling missed (→ F-S2103-P4-003)
- **Evidence:** BC-6.10.002 v1.4 (36d69804) correctly specifies `headRefOid`-based deletion-agnostic recovery. pr-manager.sh mirror at 96627798 confirmed aligned. However, ADR-031 §Decision 8 still contained the original `--delete-branch` removal rationale framing it as the recovery affordance mechanism ("Per F-S2103-P2-001, `--delete-branch` is no longer passed at merge invocation, making the pre-deletion orphan-merge recovery affordance real") — a claim that is factually wrong given `delete_branch_on_merge=true`. This ADR sibling lag is classified F-S2103-P4-003 MEDIUM.

---

### F-S2103-P3-003 — git fetch && merge-base conflation; false orphan-merge P0

- **Pass-3 verdict:** MEDIUM
- **Pass-4 resolution:** CONFIRMED-FIXED
- **Evidence:** T-006 confirmed present at 12282097, title corrected to reference EC-007 at 2a6fa036 (pre-pass-4). Step A/B split and TrunkFetchFailed EC-007 variant confirmed in BC-6.10.002 v1.4 and pr-manager.sh mirror.

---

### T-006 title — mis-cited AC-006 (nonexistent); should reference EC-007

- **Pass-3 addendum:** FIXED
- **Pass-4 resolution:** CONFIRMED-FIXED
- **Evidence:** Corrected at 2a6fa036 prior to pass-4 dispatch. T-006 title correctly references EC-007 (TrunkFetchFailed).

---

### OBS-P3-1 / OBS-P3-2 — Extractor anchor / ordering assertion hardening

- **Pass-3 verdict:** FIXED at 12282097
- **Pass-4 resolution:** PARTIALLY-FIXED — PAPER-FIX detected (→ F-S2103-P4-002)
- **Evidence:** Bold-heading anchors and belt-and-braces guards were added. However adversary independent mutation analysis revealed that 5 of the bats assertion guards use the `&&{false;}||true` idiom which is structurally unable to fail regardless of input: the `&&{false;}` block executes `false` (exit 1) which short-circuits `&&`, then `||true` catches the non-zero and returns success unconditionally. This includes T-006's core negative assertion. The guards compile and appear to pass but provide zero detection power. This is a TD-VSDD-059 paper-fix of the OBS-P3-1/2 hardening intent. Classified F-S2103-P4-002 HIGH.

---

## Part B — New Findings

### HIGH

#### F-S2103-P4-001 — BC §Architecture Anchors "step 9" residual contradicts §Traceability in same document

- **Severity:** HIGH
- **Category:** spec-fidelity (S-7.01(c) partial-fix; intra-document contradiction)
- **Location:** BC-6.10.002 §Architecture Anchors section
- **Description:** Following the Step 9 → Step 8-post-A relocation and successive pass-1/2/3 sweeps, the §Architecture Anchors section of BC-6.10.002 retained a "step 9" reference describing placement of the post-merge ancestry assertion. This created a direct intra-document contradiction: §Traceability (the normative step-placement section) correctly specified Step 8-post-A while §Architecture Anchors contradicted with "step 9". This is a higher-severity partial-fix than P3-001 (which was a different-section same-story miss) because it is a within-BC contradiction between two normative sections.
- **Evidence:** §Architecture Anchors verbatim: "...post-merge ancestry assertion at step 9..." while §Traceability specified "Step 8-post-A (immediately after state: MERGED)". Adversary performed full-document classified sweep of all "step 9" / "step-9" occurrences across all sections; only §Architecture Anchors remained stale.
- **Fix:** Perform full-document sweep of BC-6.10.002 classifying all "step 9" occurrences as historical (changelog entries, prior-version cross-references) vs. normative (requirement text). Update all normative sites to Step 8-post-A. Historical entries in §Changelog preserved per TD-VSDD-091 append-only stability.
- **Routing:** product-owner
- **Status:** FIXED at d7b7f1c2 — BC-6.10.002 v1.5; full-document classified sweep; all normative "step 9" sites corrected; historical entries preserved.

---

#### F-S2103-P4-002 — Five bats guards use `&&{false;}||true` idiom: structurally unable to fail (TD-VSDD-059 paper-fix)

- **Severity:** HIGH
- **Category:** test-correctness (TD-VSDD-059 paper-fix detection)
- **Location:** bats test file — T-001 through T-005 negative-assertion guards; T-006 core assertion
- **Description:** Five guards in the bats test suite use the shell pattern `cmd && { false; } || true`. This idiom is structurally unable to return failure: if `cmd` succeeds (exit 0), `&&{false;}` executes `false` which exits 1, then `||true` catches exit 1 and returns 0 (success). If `cmd` fails (exit !=0), `&&{false;}` is short-circuited, but `||true` again catches any non-zero and returns 0. The result is that these guards always return 0 regardless of whether the underlying command produces the expected failure. This renders T-006's core negative assertion — which was the primary hardening deliverable for OBS-P3-1/2 — completely inert. The bats test suite passes green but detects nothing.
- **Evidence:** Per-guard mutation verification:
  - **Overrun mutant:** extractor called with artificially overrun input (step block boundary overflowed by injected content); `&&{false;}||true` guard returned 0 regardless — failing-line: `assert_output --partial "STEP_8_POST_A_FOUND"` would produce no output, guard exits 0, test passes incorrectly.
  - **Misclassification mutant:** extractor called with input where Step 8-post-A section was renamed to Step 8-post-B; guard returned 0 — failing-line: core negative assertion `[[ -z "$extracted" ]]` exits 0 via `||true`.
  - **Mandate-deletion mutant:** ancestry-check mandate text deleted from Step 8-post-A block; guard returned 0 — failing-line: `assert_output --partial "ANCESTRY_CHECK"` silenced by `||true`.
  All three mutant classes confirm zero detection power.
- **Fix:** Replace all five `&&{false;}||true` guards with structurally correct negative assertions using `run` + `assert_failure` or direct `[[ ... ]] || fail "message"` form. Per-guard mutation re-verification required post-fix.
- **Routing:** test-writer + implementer (bats harness)
- **Status:** FIXED at fc08a70a — all 5 inert guards replaced with correct assertion forms; per-guard mutation verification (overrun mutant, misclassification mutant, mandate-deletion mutant) confirmed with verbatim failing-line evidence for each.

---

### MEDIUM

#### F-S2103-P4-003 — ADR-031 §Decision 8 stale rationale: "−−delete-branch removal makes affordance real"

- **Severity:** MEDIUM
- **Category:** spec-fidelity (ADR sibling lag from F-S2103-P3-002 partial fix)
- **Location:** ADR-031 §Decision 8 changelog / rationale text
- **Description:** ADR-031 §Decision 8 (v1.7, committed c97faaae) retained the claim: "Per F-S2103-P2-001, `--delete-branch` is no longer passed at merge invocation, making the pre-deletion orphan-merge recovery affordance real." This rationale is factually wrong: with `delete_branch_on_merge=true` on this repository (confirmed via `gh api`), GitHub auto-deletes the remote head at merge time regardless of whether `--delete-branch` was passed. The `--delete-branch` omission alone does not preserve any recovery affordance. BC-6.10.002 v1.4 (36d69804) already grounded this correctly via `headRefOid`-based deletion-agnostic recovery, but the ADR sibling was not updated in the same burst.
- **Evidence:** ADR-031 §Decision 8 v1.7 verbatim: "making the pre-deletion orphan-merge recovery affordance real" — predicate depends on `--delete-branch` absence preserving remote head, which is false under auto-delete. BC-6.10.002 v1.4 §PC3 correctly specifies `headRefOid` anchor as the deletion-agnostic recovery vehicle.
- **Fix:** Re-ground ADR-031 §Decision 8 rationale to reflect: (a) Step 8-post-A ordering guarantee (assertion runs before pr-manager deletion sequence 8b/8c/8d), and (b) deletion-agnostic `headRefOid` anchor (PR-retained field, survives auto-delete). Preserve v1.7 changelog as historical; append v1.8 amendment note.
- **Routing:** architect
- **Status:** FIXED at 8f4d725c — ADR-031 v1.8; §Decision 8 rationale re-grounded to ordering guarantee + headRefOid anchor; v1.7 changelog preserved historical; OBS-P4-2 adjudication note added.

---

#### F-S2103-P4-004 — Bats test file header section stale: references §Step 9 + missing EC-007/T-006

- **Severity:** MEDIUM
- **Category:** spec-fidelity (documentation parity)
- **Location:** bats test file §Header / §Purpose section
- **Description:** The bats test file header section retained two staleness issues: (1) referenced "Step 9" for the ancestry assertion placement, inconsistent with the Step 8-post-A correction pervasive across all other artifacts; (2) omitted EC-007 (TrunkFetchFailed) and T-006 from the test coverage summary, despite both being added at 12282097. The header is the first section reviewers read; header/body inconsistency creates confusion about what the harness actually tests.
- **Evidence:** Bats header verbatim: "...tests the Step 9 ancestry assertion..." and coverage list omitted EC-007/T-006 entries.
- **Fix:** Update header to reference Step 8-post-A; add EC-007 and T-006 to coverage summary.
- **Routing:** test-writer
- **Status:** FIXED at fc08a70a — header updated to Step 8-post-A; EC-007 and T-006 added to coverage summary.

---

### LOW

#### F-S2103-P4-005 — Story Token Budget cites ADR-031 v1.3 while normative content reflects v1.8

- **Severity:** LOW
- **Category:** spec-fidelity (version cite drift)
- **Location:** S-21.03 story spec Token Budget / Architecture References section
- **Description:** The story spec Token Budget section cited "ADR-031 v1.3 §Decision 8" as the architecture anchor for the post-merge ancestry assertion. By pass-4, ADR-031 had advanced to v1.8 (via v1.6 §Decision 6, v1.7 §Decision 8 placement, and v1.8 recovery-affordance re-grounding). The v1.3 cite is internally inconsistent: the story spec body references Step 8-post-A (v1.7+ language) while the Token Budget cites the pre-amendment v1.3. This is distinct from the S-21.02 accepted uniform-provenance case (where v1.3 was the architectural baseline; v1.4+ amendments post-dated story authorship). Here the cite drift is within the same cascade.
- **Evidence:** Story spec Token Budget: "ADR-031 v1.3 §Decision 8" while story body tasks reference "Step 8-post-A" language introduced at ADR-031 v1.7.
- **Fix:** Update Token Budget ADR-031 cite to v1.8 (current as of this burst); confirm §Decision 8 anchor unchanged.
- **Routing:** story-writer
- **Status:** FIXED at fd861d94 — story v1.6; Token Budget ADR-031 cite updated v1.3→v1.8; input-hash 56e51ee→9db7499.

---

### Observations (non-blocking)

#### OBS-P4-1 — Retry-succeed path for TrunkFetchFailed is not tested

- **Verdict:** FIXED at fc08a70a
- **Evidence:** T-007 added with `GIT_FETCH_FAIL_ONCE` stub environment variable; test verifies that a single fetch failure followed by a successful retry does not trigger TrunkFetchFailed advisory and proceeds to ancestry check. Stub implementation confirmed in bats harness.

---

#### OBS-P4-2 — SS-05/SS-06 cross-plane tension: BC-6.10.002 anchored SS-06 while S-21.03 enforcement is pr-manager (SS-05)

- **Verdict:** ADJUDICATED at 8f4d725c (architect ruling — no re-anchoring)
- **Evidence:** Architect adjudication: BC-6.10.002's `subsystem: SS-06` is correct and unchanged. The deliver-story skill is an SS-06 artifact; its governing BC anchors to SS-06 per the VSDD BC-S.SS.NNN naming convention. CAP-038's SS-05 assignment in ADR-031 §Decision 7 registers the capability's enforcement-agent location (pr-manager), not the contract host. Story S-21.03's `subsystems: [SS-06]` is correct; the primary deliverable is the BC-6.10.002 amendment. The split — contract in SS-06, enforcement agent in SS-05 — is the correct architectural expression; clarifying note added to ADR-031 §Decision 8 at 8f4d725c.

---

### Process-Gap (non-finding, queued for story-close codification)

#### [process-gap] — Inert-guard idiom not caught by any lint; prior mutation attestation exercised only one mutant class

- **Class:** test-harness discipline gap
- **Description:** The `&&{false;}||true` shell idiom appears syntactically valid and passes shellcheck; no existing hook or CI lint catches structurally-inert assertion guards. Additionally, prior mutation verification in the pass-3 burst-log Dim-2 exercised only the structural-delete mutant class (one class of three). Per-guard multi-class mutation verification (overrun, misclassification, mandate-deletion) was not codified as a required discipline.
- **Action queued:** (1) Consider adding a shellcheck or custom lint rule for the `&&{false;}||true` anti-pattern; (2) per-guard mutant-verification discipline candidate for story-close codification — requires verifying at minimum: structural-delete mutant, misclassification mutant, overrun mutant. Both items join the Cycle-Closing Checklist alongside the POLICY 21 extensionless-shim governance question.

---

### Clean Axes Verified

- **Scope discipline:** Files touched confined to BC-6.10.002, ADR-031, bats test file, and story spec. No lateral scope expansion.
- **TD-VSDD-091:** All spec narrative uses behavioral anchors (BC clause identifiers, step labels, EC-NNN error codes), not line numbers.
- **POLICY 19:** No hardcoded credentials or secrets in fixture stubs.
- **F-S2103-P3-001 CONFIRMED-FIXED:** §Previous Story Intelligence Step 8-post-A confirmed; no residual "step 9" in normative sections.
- **F-S2103-P3-003 + T-006-title CONFIRMED-FIXED:** Step A/B split + EC-007 + T-006 (title EC-007) confirmed.
- **Novelty:** HIGH — F-P4-002 (structurally inert guards — TD-VSDD-059 paper-fix class) represents a new defect pattern not present in prior passes; F-P4-001 is a deeper intra-document contradiction within the BC itself.

---

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER  | 0 |
| HIGH     | 2 |
| MEDIUM   | 2 |
| LOW      | 1 |
| NITPICK  | 0 |
| OBS      | 2 |

**Overall Assessment:** block
**Convergence:** findings remain — iterate (streak 0/3)
**Readiness:** requires revision (fix burst dispatched; all findings FIXED in-burst prior to this persist)
**Next reviewed HEAD:** fc08a70a

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 4 |
| **New findings** | 5 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.00 (5 / (5 + 0)) |
| **Median severity** | 3.0 (MEDIUM) |
| **Trajectory** | 3→3→3→5 |
| **Verdict** | FINDINGS_REMAIN |
