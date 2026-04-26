---
document_type: rule
rule_id: lessons-codification
version: "1.0"
subsystems: [SS-05, SS-07, SS-08]
created: 2026-04-26
traces_to: .factory/STATE.md#Lessons-Learned
stories: [S-7.02]
---

# Rule: Novel Adversary Process Catch → Codification Follow-Up

## Summary

Every novel adversary process catch (process gap, not content defect) MUST trigger
a codification follow-up before sub-cycle closure. Codification = update agent
prompts, templates, and/or hooks. Track in STATE.md Lessons Learned section.

## Definition of "Novel Process Catch"

A finding qualifies as a **novel process catch** when it identifies a gap in:
- An agent prompt or workflow step (not a gap in a specific spec artifact)
- A hook or validation script (missing enforcement)
- A rule file or governance document (missing policy)
- A pipeline template (structural gap in output format)

The adversary tags such findings with `[process-gap]` in the Observations section.

Contrast with a **content defect**: a specific BC, VP, story, or doc that has
wrong information. Content defects are fixed in place — no codification follow-up
required unless the same defect pattern appears systematically (3+ instances).

## Required Follow-Up Protocol

When the orchestrator closes a sub-cycle, it MUST:

1. **Scan** the final convergence report for any `[process-gap]` tagged findings.

2. **For each process-gap finding**, open a follow-up E-N story (in the
   appropriate self-improvement epic) OR record a deferred item in the Drift
   Items table of STATE.md with an explicit justification for deferral.

3. **Do not declare CLOSED** until either:
   - A follow-up story exists (at minimum in STORY-INDEX.md with `status: draft`), OR
   - A justified deferral is logged in STATE.md Drift Items with a target release.

## Enforcement

- The **orchestrator** references this rule during the cycle-closing checklist.
  See `agents/orchestrator/orchestrator.md` — Cycle-Closing Checklist section.

- The **adversary** tags process-gap findings with `[process-gap]` at finding
  creation time, enabling automated scan in step 1 above.

- The **state-manager** logs confirmed codifications in `cycles/<cycle>/lessons.md`
  using the tag `[codified]` once the follow-up story is created or merged.

## Canonical Example

E-7 (self-improvement epic) was created specifically to codify lessons from the
v1.0-brownfield-backfill cycle. Stories S-7.01 and S-7.02 are direct codification
follow-ups from process-gap findings in s6.01-pass-1.md:

| Finding | Tag | Codification Story |
|---------|-----|--------------------|
| F-001: empty behavioral_contracts in story | [process-gap] | S-7.01 (story-writer spec-first gate) |
| F-011: missing capability anchor justification | [process-gap] | S-7.01 (product-owner anchor requirement) |
| F-023: adversary didn't check sibling-file propagation | [process-gap] | S-7.01 (adversary partial-fix-regression axis) |
| F-027: state-manager declared count change complete prematurely | [process-gap] | S-7.02 (defensive sweep + hook) |

## Cross-References

- `agents/orchestrator/orchestrator.md` — Cycle-Closing Checklist (references this rule)
- `agents/adversary.md` — Partial-Fix-Regression Discipline (tags [process-gap])
- `agents/state-manager.md` — Defensive Sweep Discipline (companion process rule)
- E-7 (self-improvement epic) — canonical example of this rule applied in production
