---
document_type: behavioral-contract
level: L3
version: "1.5"
status: draft
producer: phase-1-4b-bc-extractor
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs:
  - .factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md
  - .factory/specs/behavioral-contracts/bc-id-mapping.md
  - .factory/cycles/v1.0-brownfield-backfill/e-21-arch-delta-analysis.md
input-hash: "e350ae2"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L1186"
subsystem: SS-06
capability: "CAP-038"
lifecycle_status: active
introduced: v1.0.0-beta.4
modified:
  - "2026-07-19 (v1.2) — E-21 factory-state data-loss hardening (product-owner; issue #358): PC2 + PC3 added — pr-manager step-7 trunk-assertion postconditions (INV-E21-006; at authoring time cited as INV-E21-005 before ADR-031 v1.1 renumber; corrected at v1.3). PC2: post-create baseRefName assertion. PC3: post-merge --is-ancestor assertion. Changelog section added."
  - "2026-07-19 (v1.3) — adv pass-1 fix burst (F-P1-001/010) per ADR-031 v1.1 rulings (product-owner): (1) capability frontmatter TBD→CAP-038 (ADR-031 §Decision 7+8); (2) INV-E21-005→INV-E21-006 sweep (TD-VSDD-060 — PR Trunk Ancestry was renumbered append-only per ADR-031 v1.1 §Decision 1; INV-E21-005 reassigned to Post-Rebase Diff Integrity/BC-5.44.001); (3) §Traceability Architecture Module deliver-story/SKILL.md→pr-manager.md (SoT = §Architecture Anchors; F-P1-010); (4) ADR-031 §Decision 8 cite added to §Traceability; (5) L2 Capability TBD→CAP-038 + Capability Anchor Justification added."
  - "2026-07-24 (v1.5) — F-S2103-P4-001 residual-sibling closure (product-owner; S-21.03 LOCAL cascade pass 4): §Architecture Anchors line 209: step 9 (post-merge assertion) corrected to Step 8-post-A (post-merge ancestry assertion, before branch deletion); tense updated to 'amended by S-21.03'. POLICY 5 v1.3.4 full-document sweep: 1 class-(c) hit fixed; 0 class-(a) hits; 0 class-(b) hits."
  - "2026-07-24 (v1.4) — F-S2103-P3-002/003 fix burst (product-owner; S-21.03 LOCAL cascade pass 3): (1) PC3 recovery block made deletion-agnostic — delete_branch_on_merge=true grounded fact; recovery anchors on PR-retained headRefOid reference surviving branch auto-deletion; removed implicit intact-head-branch assumption; (2) PC3 &&-joined fetch+ancestry command split into two distinct steps with separate failure semantics — Step A git fetch failure → TrunkFetchFailed (transient-escalate; ancestry UNANSWERED; no orphan-merge recovery); Step B merge-base non-zero after successful fetch → MergeNotAncestorOfTrunk P0 as before; (3) EC-007 added (fetch failure); (4) TrunkFetchFailed test vector added; (5) Traceability Architecture Module step placement updated to Step 8-post-A per story v1.4 + ADR-031 v1.7."
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
last_amended: "(v1.5) — F-S2103-P4-001 residual-sibling: §Architecture Anchors step 9 corrected to Step 8-post-A (post-merge ancestry assertion, before branch deletion); tense updated (to be amended → amended by S-21.03). [Prior: (v1.4) — PC3 deletion-agnostic recovery; fetch+ancestry split Step A/B; EC-007; Traceability step placement → Step 8-post-A. (v1.3) — CAP-038; INV-E21-006 sweep; Architecture Module → pr-manager.md. (v1.2) — PC2+PC3 added.]"
---

# Behavioral Contract BC-6.10.002: deliver-story: 9-step dispatch sequence with exit conditions

> Source: `pass-3-deep-skills-batch-1.md` line 1186 (was `BC-AUDIT-313`)
> Subsystem: SS-06 — Skill Catalog
> Section: Story delivery skills

## Description

deliver-story: 9-step dispatch sequence with exit conditions. 9 sequential steps each with named subagent + exit condition: 1) devops-engineer create worktree → `git worktree list` shows it; 2) test-writer stubs → `cargo check` passes; 3) test-writer failing tests → Red Gate verified; 4) implementer TDD → tests green + clippy + fmt + zero todo!()/unimplemented!(); 5) demo-recorder → evidence per AC; 6) implementer push → remote SHA visible; 7) pr-manager 9-step PR lifecycle (see PC2 and PC3 for mandatory trunk-assertion postconditions within step 7); 8) devops-engineer cleanup; 9) state update on factory-artifacts.

## Preconditions

1. Skill invocation after preconditions pass

## Postconditions

### PC1 — Step sequence integrity

Each step's exit condition is independently verified before advancing; skill never skips a step.

### PC2 — Step 7: pr-manager MUST assert baseRefName equals configured trunk post-create

Within step 7 (pr-manager 9-step PR lifecycle), after `gh pr create --base <trunk>` completes
successfully, pr-manager MUST immediately delegate to `github-ops` to run:

```
gh pr view <pr_number> --json baseRefName
```

The returned `baseRefName` value MUST equal the configured trunk (`develop` for feature pipelines).
If the returned value does not equal the configured trunk, pr-manager MUST hard-fail the burst
with the following error:

```
HARD FAIL: PR #<pr_number> baseRefName '<actual_base>' does not match configured trunk '<trunk>'.
The PR was not created against the correct target branch. This is likely a gh CLI base-inference
bug (issue #358 class). Do NOT proceed to merge. Investigate and close/recreate the PR with an
explicit --base <trunk> flag.
```

The PR MUST NOT be merged until this assertion passes. The story MUST NOT be marked delivered
until the PR has been correctly targeted.

**INV-E21-006 instantiation:** This postcondition enforces PR Trunk Ancestry at the post-create
checkpoint. It catches the case where `gh pr create` overrides `--base` via tracking-upstream
inference, producing a PR that is silently targeted against a non-trunk branch.

**Error variant:** `BaseRefNameMismatch`

### PC3 — Step 7: pr-manager MUST assert merge commit is an ancestor of origin/trunk post-merge

Within step 7 (pr-manager 9-step PR lifecycle), after `gh pr merge` completes with `state: MERGED`,
pr-manager MUST immediately delegate to `github-ops` to retrieve the merge SHA and execute the
following **two distinct sequential steps** with separate failure semantics:

Retrieve first: `<merge_sha>` is the `mergeCommit.oid` returned by
`gh pr view <pr_number> --json mergeCommit`.

**Step A — Fetch trunk:**

```
git fetch origin <trunk>
```

If `git fetch origin <trunk>` fails for any reason (network, auth, or other transient error):
- This is **NOT** a `MergeNotAncestorOfTrunk` finding. The ancestry question is **UNANSWERED**.
- pr-manager MUST retry once.
- If the retry also fails, pr-manager MUST raise a transient-escalation error and halt:

```
TRANSIENT ESCALATION [TrunkFetchFailed]: git fetch origin <trunk> failed after 1 retry.
Cannot determine ancestry of merge commit <merge_sha>. This is a network/auth/transient failure —
NOT an orphan-merge condition. Do NOT proceed to delivery. Do NOT enter orphan-merge recovery.
Escalate to human for resolution.
```

**Error variant (fetch failure):** `TrunkFetchFailed` — transient; halt + escalate to human; do not assert orphan merge; do not mark story delivered; ancestry question is UNANSWERED.

**Step B — Assert ancestry (executes only after Step A succeeds):**

```
git merge-base --is-ancestor <merge_sha> origin/<trunk>
```

If `git merge-base --is-ancestor` returns a non-zero exit code after a successful Step A fetch,
pr-manager MUST raise a P0 data error immediately:

```
P0 DATA ERROR: PR #<pr_number> merge commit <merge_sha> is NOT an ancestor of origin/<trunk>.
The PR merged into a non-trunk branch (orphan merge — issue #358 class). Story content did NOT
land on trunk. Required recovery:
  1. Identify where the merge commit landed: `git branch -r --contains <merge_sha>`.
  2. Retrieve the story's head commit SHA via PR-retained reference (survives branch auto-deletion):
     `gh pr view <pr_number> --json headRefOid`
     Then fetch it: `git fetch origin <headRefOid>` or re-checkout via `gh pr checkout <pr_number>`.
     NOTE: This deployment has delete_branch_on_merge=true (grounded fact: `gh api
     repos/drbothen/vsdd-factory --jq .delete_branch_on_merge` → true). The head branch is
     auto-deleted at merge time and is NOT directly available as a standalone ref. The mandated
     recovery path anchors on the PR-retained headRefOid reference, which survives branch deletion.
     If the repo's auto-delete setting is ever disabled, the head branch would also remain directly
     available as a convenience, but the mandated recovery path must not depend on this.
  3. Open a new PR from that commit SHA targeting <trunk> and merge it.
  4. Do NOT mark this story as delivered until the merge commit is confirmed on trunk.
```

The story MUST NOT be marked delivered until Step B assertion passes with exit code 0.

**INV-E21-006 instantiation:** This postcondition is the **load-bearing check** for PR Trunk
Ancestry. It catches the orphan-merge scenario (issue #358 concrete instance) where the PR reports
`state: MERGED` but the merge commit is not reachable from trunk.

**Error variant (ancestry check):** `MergeNotAncestorOfTrunk`

## Invariants

1. TBD — derive from skill SKILL.md frontmatter and acceptance criteria.

2. **PC2 and PC3 are non-optional exits for step 7.** The pr-manager 9-step lifecycle is not
   complete until both assertions pass. Neither a successful `gh pr create` response alone nor a
   `state: MERGED` response alone satisfies step 7's exit condition.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | TBD | TBD |
| EC-002 | `gh pr view --json baseRefName` returns trunk correctly | PC2 passes; step 7 continues to merge phase |
| EC-003 | `gh pr view --json baseRefName` returns a non-trunk branch | `BaseRefNameMismatch` error; hard-fail; PR not merged |
| EC-004 | `git merge-base --is-ancestor` returns exit 0 | PC3 passes; story may be marked delivered |
| EC-005 | `git merge-base --is-ancestor` returns exit 1 (merge not on trunk) | `MergeNotAncestorOfTrunk` P0 error; story not delivered |
| EC-006 | `gh pr view --json mergeCommit` returns null (merge failed silently) | PC3 pre-check: if mergeCommit.oid is null, treat as `MergeNotAncestorOfTrunk` — unknown merge SHA cannot be verified |
| EC-007 | `git fetch origin <trunk>` fails (network/auth/transient error) before ancestry check can execute (Step A failure) | `TrunkFetchFailed` — retry once; if retry fails, halt + escalate to human; ancestry is UNANSWERED; story NOT marked delivered; do NOT enter orphan-merge recovery; this is not a `MergeNotAncestorOfTrunk` condition |

## Canonical Test Vectors

> Golden-file test inputs and expected outputs.

| Input | Expected Output | Category |
|-------|----------------|----------|
| TBD — happy path from skill acceptance | TBD | happy-path |
| `gh pr view --json baseRefName` returns `{"baseRefName": "develop"}` (trunk) | PC2 passes; continues to merge | PC2 happy-path |
| `gh pr view --json baseRefName` returns `{"baseRefName": "feature/S-007-impl"}` | `BaseRefNameMismatch` error; hard-fail | PC2 error |
| `git merge-base --is-ancestor <sha> origin/develop` exits 0 | PC3 passes; story delivered | PC3 happy-path |
| `git merge-base --is-ancestor <sha> origin/develop` exits 1 | `MergeNotAncestorOfTrunk` P0 error | PC3 error |
| `gh pr view --json mergeCommit` returns `{"mergeCommit": null}` | `MergeNotAncestorOfTrunk` P0 error (null SHA treated as unverified) | PC3 null-merge |
| `git fetch origin <trunk>` exits non-zero (network/auth error); retry also exits non-zero | `TrunkFetchFailed` transient escalation; halt + escalate to human; do NOT enter orphan-merge recovery; ancestry UNANSWERED | PC3 transient-fetch-error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | TBD — assertion derived from acceptance: "Each step's exit condition independently verified before advancing; skill never skips a step." | manual |
| (TBD) | PC2: `BaseRefNameMismatch` fires on trunk mismatch | bats: stub `gh pr view --json baseRefName` returning wrong branch; assert hard-fail |
| (TBD) | PC3: `MergeNotAncestorOfTrunk` fires on non-ancestor merge SHA | bats: stub `git merge-base --is-ancestor` returning exit 1; assert P0 error |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-038 |
| Capability Anchor Justification | CAP-038 registered in ARCH-INDEX v3.07 (ADR-031 §Decision 7+8, commit 14a78515): "PR trunk ancestry integrity — post-create baseRefName assertion + post-merge ancestry guard (INV-E21-006)." BC-6.10.002 is the sole implementing BC for CAP-038. ADR-031 §Decision 8 mandates amendment to the pr-manager 9-step lifecycle. |
| L2 Domain Invariants | TBD |
| Architecture Module | `plugins/vsdd-factory/agents/pr-manager.md` (PC2 baseRefName assertion at step 3 post-create; PC3 ancestry assertion at Step 8-post-A post-merge; as amended by S-21.03 per ADR-031 §Decision 8 + story v1.4 + ADR-031 v1.7) |
| ADR Reference | ADR-031 §Decision 8 (INV-E21-006 enforcement; post-create baseRefName assertion + post-merge `git merge-base --is-ancestor` guard; CAP-038 allocated; pr-manager 9-step lifecycle amendment) |
| Stories | TBD; S-21.03 (E-21 Wave 1 — pc2/pc3 postconditions) |
| Source Issues | #358 (PR base not locked to trunk; orphan merge possible) |

## Related BCs (Recommended)

- TBD — link to sibling BCs in same skill (cross-reference once full SS-06 catalog written)
- BC-5.42.001 — pr-manager READY-verdict + merge-strategy enforcement (companion pr-manager hardening BC)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#deliver-story-9-step-dispatch-sequence-with-exit-conditions` — TBD anchor
- `plugins/vsdd-factory/agents/pr-manager.md` — step 3 (post-create assertion) and Step 8-post-A (post-merge ancestry assertion, before branch deletion); amended by S-21.03

## Story Anchor (Recommended)

S-21.03 (E-21 Wave 1 — pr-manager trunk-assertion hardening: post-create baseRefName check and post-merge ancestry assertion)

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/deliver-story/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source line(s) within skill** | 52-121 |

#### Evidence Types Used

- documentation: stated in SKILL.md frontmatter and prose
- inferred: behavior derived from skill acceptance criteria

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | TBD |
| **Global state access** | TBD |
| **Deterministic** | TBD |
| **Thread safety** | TBD |
| **Overall classification** | TBD |

#### Refactoring Notes

TBD — assess once architecture mapping is complete.

#### Source Excerpt (verbatim)

```text
#### BC-AUDIT-313 — deliver-story: 9-step dispatch sequence with exit conditions

**Skill:** `plugins/vsdd-factory/skills/deliver-story/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 52-121
**Trigger:** Skill invocation after preconditions pass
**Behavior:** 9 sequential steps each with named subagent + exit condition: 1) devops-engineer create worktree → `git worktree list` shows it; 2) test-writer stubs → `cargo check` passes; 3) test-writer failing tests → Red Gate verified; 4) implementer TDD → tests green + clippy + fmt + zero todo!()/unimplemented!(); 5) demo-recorder → evidence per AC; 6) implementer push → remote SHA visible; 7) pr-manager 9-step PR lifecycle; 8) devops-engineer cleanup; 9) state update on factory-artifacts.
**Acceptance:** Each step's exit condition independently verified before advancing; skill never skips a step.
```

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.5 | 2026-07-24 | F-S2103-P4-001 residual-sibling closure (S-21.03 LOCAL cascade pass 4). §Architecture Anchors line 209: "step 9 (post-merge assertion) to be amended by S-21.03" corrected to "Step 8-post-A (post-merge ancestry assertion, before branch deletion); amended by S-21.03". POLICY 5 v1.3.4 full-document sweep: 1 hit (line 209), classified (c) stale assertion-placement cite → fixed; 0 class (a) hits (no pr-manager Step 9 checklist cites in document); 0 class (b) hits (no historical/changelog "step 9" entries). |
| 1.4 | 2026-07-24 | F-S2103-P3-002/003 fix burst (S-21.03 LOCAL cascade pass 3). (1) PC3 recovery block made deletion-agnostic: delete_branch_on_merge=true grounded fact cited (gh api repos/drbothen/vsdd-factory --jq .delete_branch_on_merge → true); recovery anchors on PR-retained `gh pr view --json headRefOid` which survives branch auto-deletion; removed implicit intact-head-branch assumption (F-S2103-P3-002). (2) PC3 `&&`-joined command split into two distinct sequential steps with separate failure semantics: Step A `git fetch origin <trunk>` failure → `TrunkFetchFailed` transient-escalate (retry once, then halt+escalate; ancestry UNANSWERED; do not enter orphan-merge recovery); Step B `git merge-base --is-ancestor` non-zero after successful fetch → `MergeNotAncestorOfTrunk` P0 as before (F-S2103-P3-003). (3) EC-007 added (fetch failure → TrunkFetchFailed). (4) TrunkFetchFailed test vector added. (5) Traceability Architecture Module step placement updated to Step 8-post-A per story v1.4 + ADR-031 v1.7 (closes deferred BC-leg of F-S2103-P2-003). |
| 1.3 | 2026-07-19 | adv pass-1 fix burst (F-P1-001/010) per ADR-031 v1.1 rulings (product-owner). capability TBD→CAP-038 (ADR-031 §Decision 7+8). INV-E21-005→INV-E21-006 sweep (TD-VSDD-060; PR Trunk Ancestry renumbered append-only per ADR-031 v1.1 §Decision 1; INV-E21-005 reassigned to Post-Rebase Diff Integrity/BC-5.44.001). §Traceability Architecture Module deliver-story/SKILL.md→pr-manager.md (SoT = §Architecture Anchors; F-P1-010). ADR Reference row added (ADR-031 §Decision 8). L2 Capability TBD→CAP-038; Capability Anchor Justification added. |
| 1.2 | 2026-07-19 | E-21 factory-state data-loss hardening (product-owner; issue #358; S-21.03). PC2 added: pr-manager step 7 MUST assert `gh pr view --json baseRefName` equals configured trunk post-create (`BaseRefNameMismatch` hard-fail on mismatch). PC3 added: pr-manager step 7 MUST assert `git merge-base --is-ancestor <merge_sha> origin/<trunk>` exits 0 post-merge (`MergeNotAncestorOfTrunk` P0 error on non-ancestor). INV-E21-006 (PR Trunk Ancestry) instantiation [cited as INV-E21-005 at authoring time; corrected at v1.3]. Invariant 2 added (PC2+PC3 non-optional). EC-002..EC-006 added. 5 new test vector rows. Changelog section added. |
| 1.1 | 2026-04-25 | Brownfield extraction metadata correction; content as extracted from pass-3-deep-skills-batch-1.md#L1186. |
| 1.0 | 2026-04-25 | Initial brownfield extraction (phase-1-4b-bc-extractor; BC-AUDIT-313 → BC-6.10.002). |
