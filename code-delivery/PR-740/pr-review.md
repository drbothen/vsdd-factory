# PR #740 Review — docs(agents): state-manager idempotency contract + orchestrator transient-dispatch recovery

**Verdict: REQUEST_CHANGES**

**Reviewer standard applied:** docs-only PR → documented contracts must match *actual* agent behavior (technical-writer standard: describe current behavior, never aspirational). Findings below were verified against the actual agent files that ship in this repo (`plugins/vsdd-factory/agents/state-manager.md` and `plugins/vsdd-factory/agents/orchestrator/*`), which are outside the `.factory/` information wall.

## Summary

The intent is sound and the scope is honest (it explicitly declines the harness-level asks in issue #212 and lands only what this repo owns). The prose is clear and well-organized. However, three of the concrete factual claims do not match the documented behavior of the agents they describe, and two of those undercut the very safety property the sections exist to establish. Because a docs-only PR's entire value is accuracy, and because all fixes are small in-scope edits, this is REQUEST_CHANGES rather than APPROVE.

## Findings

### [HIGH] Non-idempotent enumeration omits `lessons.md` (and other append-style artifacts) — the safety warning is incomplete

`state-manager.md` — new "Idempotency" section, closing paragraph.

The section states: "The one non-idempotent class is **append-style records** — burst-log.md entries, convergence-trajectory.md per-pass rows, and the STATE.md Current Phase Steps row..."

The state-manager agent file's own update-event list (lines 134–136) enumerates **four** append events on the same trigger surface:

1. Burst complete → append `burst-log.md`  ✅ listed
2. Current Phase Steps row  ✅ listed
3. Adversary pass → append `convergence-trajectory.md`  ✅ listed
4. **Lesson learned → "Append to `cycles/<cycle>/lessons.md`"**  ❌ **omitted**

`lessons.md` is a high-frequency append (`L-EDP1-NNN` entries) and is explicitly documented as append in the same file. The 4 INDEX files and `decision-log.md` also receive appended/added rows. A re-dispatched lesson-capture task would double the lesson entry — the exact corruption this section warns against — with no warning here. Because the paragraph is phrased as a closed set ("The **one** non-idempotent class is ... A, B, C"), a reader treats it as exhaustive.

**Suggestion:** Add `lessons.md` to the enumeration, and either enumerate the INDEX/decision-log appends or replace the closed list with "including but not limited to" plus a pointer to the agent's update-event list as the authoritative source.

### [MEDIUM] STATE.md "set-to-value, not append" lead clause overclaims and is contradicted by the same section

`state-manager.md` — new "Idempotency" section, first bullet.

Bullet 1 opens: "**STATE.md updates** are set-to-value, not append." The section's own final paragraph then carves out "the STATE.md **Current Phase Steps** row" as append-style. A reader who acts on the general lead clause alone (STATE.md is set-to-value → safe to re-run) would wrongly re-dispatch a burst-complete task and double the Current Phase Steps row. The specific fields the bullet then lists (phase transitions, file sizes, finding counts, gate verdicts) are indeed set-to-value, so the fix is to scope the lead clause.

**Suggestion:** Change the lead to "**Most STATE.md fields** are set-to-value, not append" (or "The STATE.md fields below are set-to-value"), so the general claim no longer collides with the documented Current Phase Steps exception.

### [MEDIUM] `git add -A` idempotency claim ignores broad-staging of unintended / concurrent changes

`state-manager.md` — new "Idempotency" section, git-commits bullet.

The bullet claims commits "are safe to re-attempt: if the content is already committed, `git add -A && git commit` reports 'nothing to commit' rather than producing a duplicate." The `git add -A` the agent actually uses stages **every** change in the worktree, not just the task's intended files. The "nothing to commit" guarantee only holds when the tree is clean relative to HEAD. In the failure scenario this section is written for — a dropped connection where a dead agent may have written partial/unintended files, or where concurrent bursts have touched the tree — a verbatim re-run can sweep unrelated changes into the commit. That is not idempotent; it is "idempotent only if the working tree is otherwise clean."

**Suggestion:** Qualify the claim: idempotent *when the worktree contains only this task's changes*; note that `git add -A` stages all pending changes, so a re-dispatch should verify the tree scope (or the burst protocol/`state-burst` skill should own the staging boundary).

### [MEDIUM] Fragile, imprecise cross-reference to the "Human Notification table"

`orchestrator.md` — new "Transient Dispatch Failure Recovery" subsection, step 3.

The text says "escalate to the human per the Human Notification table (Agent timeout — 3rd retry)." Verification: the target exists but **in a sibling file** — `orchestrator/steady-state.md`, under the heading `## Human Notification System`, row `| Agent timeout (3rd retry) | ESCALATION | Any phase |`. Three drift points:

- **No file pointer.** A reader inside `orchestrator.md` finds no "Human Notification table" in that file; the reference is unqualified.
- **Wording drift.** Cited as "Human Notification **table** (Agent timeout **—** 3rd retry)"; actual is heading "Human Notification **System**" and row "Agent timeout **(**3rd retry**)**". This is the exact drift risk called out in the review brief.
- **Scope mismatch.** The table lives in the steady-state sequence; the new recovery protocol is written as general dispatch guidance for any mode. Citing a steady-state-scoped table from a general protocol may mislead in non-steady-state runs.

**Suggestion:** Point explicitly to `steady-state.md` "Human Notification System" and match the row label verbatim ("Agent timeout (3rd retry)"), or, if the escalation ceiling is meant to be mode-general, reference the orchestrator's own `Failure & Escalation` Level 2 ("after 3 retries") which lives in the same file.

### [LOW] "Inspect the expected outputs" assumes an enumerable output set

`orchestrator.md` — recovery subsection, step 1.

Phase transitions can touch many files and agent outputs are often open-ended, so "for every file the dispatched task was to produce or modify" is not always knowable to the orchestrator. This is acceptable as prose because step 2/step 3 degrade gracefully ("escalate if you cannot determine the boundary"), but the instruction slightly overstates the orchestrator's ability to know the full output set a priori.

**Suggestion:** Add a half-sentence acknowledging outputs may be open-ended and that partial-knowledge cases fall through to escalation.

### [LOW] "You have read access — use it" is unqualified

`orchestrator.md` — recovery subsection, step 1.

For the orchestrator specifically this is accurate (it has read access), so the risk is low. Noting only that the instruction is stated as a blanket property; info-asymmetry agents (e.g. holdout-evaluator) are separate agents and unaffected, so no change is strictly required.

### [NIT] Placement of the Idempotency section

Placing "Idempotency" immediately after "Failure & Escalation" is reasonable — idempotency is recovery-adjacent and is consumed by the orchestrator's recovery protocol. No change needed; flagged only because the review brief asked.

## What I verified (no rubber-stamp)

- Confirmed the orchestrator `Failure & Escalation` ceiling ("after 3 retries") exists (orchestrator.md L412) and that the named notification row exists in steady-state.md (L136) — so the cross-reference is fragile, not broken.
- Confirmed `lessons.md` is documented as an append operation in state-manager.md (L136), establishing the HIGH enumeration gap.
- Confirmed state-manager uses `git add -A` in its commit blocks (L413/423/455), establishing the broad-staging concern.
- Confirmed the STATE.md Current Phase Steps row is keep-last-5 append (L134, L111), establishing the internal-contradiction concern in bullet 1.
- Confirmed structural sections (Tool Access / Failure & Escalation / Remember) are preserved, consistent with the reported 68/68 permissions.bats pass.

## Path to APPROVE

All findings are small in-scope prose edits (add `lessons.md` to the list; scope the two overclaim clauses; fix the cross-reference file pointer + wording). No code, no test, no scope expansion required. Re-request review after the edits.
