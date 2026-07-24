# PR #761 — Fresh-Eyes Final Review (S-21.03) — Round 1

**PR:** feat(S-21.03): pr-manager trunk-assertion hardening — post-create baseRefName check and post-merge ancestry assertion (E-21 W1, issue #358)
**Branch:** origin/feature/S-21.03-pr-manager-trunk-assertion (HEAD 856dffe5)
**Base:** origin/develop
**Reviewer:** pr-reviewer (fresh-eyes, different-model cognitive diversity)
**Method:** diff + PR description only; independent review

## Verdict: APPROVE

I independently reviewed all 8 changed files, verified every doc-parity token
is present in `pr-manager.md` at the claimed line anchors (134 / 145 / 327 /
404 / 497), confirmed diff coherence (every path maps to S-21.03 or
pr-manager), and re-ran the bats suite from a clean checkout of HEAD
`856dffe5` — 7/7 pass. No BLOCKER or MINOR findings survived verification.

### Findings

| # | Severity | Category | Finding | Note |
|---|----------|----------|---------|------|
| F1 | INFORMATIONAL | coverage | The EXECUTABLE-HARNESS layer (`_run_base_ref_assertion`, `_run_ancestry_assertion`, `_run_fetch_then_ancestry_assertion`, `_run_null_mergecommit_assertion`) is a bash *reimplementation* of the BC procedure living inside the `.bats` file — it does not execute the `pr-manager.md` prose (which is an LLM prompt, not runnable code). The binding to the actual artifact is the DOC-PARITY grep layer. | Inherent to testing an agent-prompt spec. Mitigated: DOC-PARITY asserts the direction phrases (`does not equal`, `non-zero exit`) and consequence phrases (`MUST NOT be merged`, `MUST NOT be marked delivered`), so a logic inversion that stripped those tokens would fail the grep layer. Acceptable. |
| F2 | INFORMATIONAL | demo | Demo evidence is `.txt` (grep captures + bats-run), not `.gif`/`.webm`. | The artifact under test is markdown prose in an agent prompt + a bats file — there is no UI/CLI/API to screen-record. `.txt` grep/bats captures are the correct, information-complete evidence form; a screencast would add nothing. `evidence-report.md` documents this explicitly per demo-recorder instruction. Not a blocking gap. |
| F3 | INFORMATIONAL | size | 1006 insertions, but 620 lines are the bats suite and ~380 are demo evidence + doc prose; the behavioral change to `pr-manager.md` is 148 lines. | Well within reason for a spec-hardening story; no product-code churn. |

## AC compliance check

| AC / PC | BC-6.10.002 v1.5 requirement | Implementation site | Satisfied |
|---------|------------------------------|---------------------|-----------|
| AC-001 / PC2 (error) | After `gh pr create`, assert `baseRefName == configured trunk`; hard-fail `BaseRefNameMismatch`; PR MUST NOT be merged | Step 3-post-A: `gh pr view --json baseRefName`, `BaseRefNameMismatch` hard-fail body with actual+expected, "PR MUST NOT be merged … until this assertion passes" | YES |
| AC-002 / PC2 (happy) | Correct baseRefName → assertion passes; proceed to merge | Step 3-post-A "Continue immediately to step 4"; `STEP_COMPLETE: step=3 … baseRefName assertion passed` | YES |
| AC-003 / PC3 (error) | After MERGED, `git fetch origin <trunk>` then `git merge-base --is-ancestor`; non-zero exit → P0 `MergeNotAncestorOfTrunk`; story MUST NOT be delivered | Step 8-post-A Step A (fetch) + Step B (`merge-base --is-ancestor`); non-zero exit → P0 DATA ERROR MergeNotAncestorOfTrunk; "MUST NOT be marked delivered" | YES |
| AC-004 / PC3 (happy) | `merge-base --is-ancestor` exit 0 → assertion passes; story may be delivered | Step 8-post-A: "On Step B assertion pass: proceed to Step 8b"; Step 9 confirms gate | YES |
| AC-005 / PC3 (EC-006) | null `mergeCommit.oid` → `MergeNotAncestorOfTrunk` | Step 8-post-A: null/absent `mergeCommit.oid` → P0 MergeNotAncestorOfTrunk before fetch | YES |
| EC-007 (additive) | fetch fails → retry once → `TrunkFetchFailed` HALT (UNANSWERED, NOT orphan-merge; do NOT enter recovery) | Step 8-post-A Step A: retry-once, TrunkFetchFailed escalation, "Do NOT … enter orphan-merge recovery"; "Do NOT proceed to Step B or Step 8b/8c/8d/Step 9" | YES |
| PC3 ordering / Invariant 2 | HALT before branch deletion (Steps 8b/8c/8d) | Step 8-post-A (line 327) precedes Step 8b (line 404); ordering enforced by test `_assert_post_a_precedes_deletion_steps` | YES |

Additional verification:
- **Merge-strategy regression check:** the diff removes `--delete-branch` forwarding from `enforce-merge-strategy.sh` and re-anchors deletion on the verified Steps 8b/8c/8d gated on the ancestry assertion. Rationale is documented and grounded (`delete_branch_on_merge=true`); recovery guidance correctly anchors on the PR-retained `headRefOid` (survives auto-deletion). Coherent with PC3 HALT-before-deletion.
- **Diff coherence:** all 8 files map to S-21.03 / pr-manager; no unrelated changes.
- **Commit quality:** 16 conventional commits, all `S-21.03`-scoped, clear messages; no AI attribution.
- **Independent bats run at HEAD 856dffe5:** 7/7 ok.

## Summary

The two post-action assertions (Step 3-post-A baseRefName check, Step 8-post-A
fetch+ancestry assertion with EC-006 null-SHA and EC-007 fetch-failure paths)
faithfully implement BC-6.10.002 v1.5 PC2/PC3 with correct HALT-before-deletion
ordering, and the bats suite passes 7/7 on an independent clean checkout — APPROVE
with only informational notes (all inherent to testing an agent-prompt spec).
