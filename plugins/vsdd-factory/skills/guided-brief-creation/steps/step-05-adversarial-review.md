# Step 5: Adversarial Review (Optional)

> **Parent skill:** `guided-brief-creation` — see `../SKILL.md` for the full workflow.
> **Agent:** adversary
> **This step:** spawn a fresh-context adversary to attack the draft brief for vagueness, scope creep, missing audiences, and competitive blind spots.

## Inputs

- Approved draft from `step-04-draft-review.md`
- `market-intel.md` if it exists
- `elicitation-notes.md`

## Outputs

- Adversarial findings (in `.factory/planning/adversarial-review-brief.md` or held in memory)
- Updated draft if findings are accepted

## Procedure

1. **Ask first.** This step is optional. Use this script:

   > "Want me to send this through a fresh-eyes adversary review before we finalize? It runs a different model with no memory of our conversation, looking specifically for vague claims, scope creep, missing audiences, and weak competitive positioning. Takes about a minute. Worth it for confidence."

   If the human says no, skip to step 6.

2. **Spawn `adversary` agent** with:
   - The draft brief (full text)
   - `market-intel.md` if it exists
   - NO conversation history, NO elicitation notes (information asymmetry — adversary must read the brief cold)

3. **Adversary prompt** should ask for findings in these categories:

   - **Vague or unmeasurable success criteria.** Quote the line, explain why it can't be tracked, propose a sharper version.
   - **Scope too broad for one product.** Identify the seam where two products are hiding.
   - **Missing audiences.** Who else might care? Who is upstream of the named user? Who is the buyer vs the user?
   - **Competitive positioning gaps.** What competitor did the brief skip? Where does the differentiation argument fail?
   - **Hidden assumptions.** What does the brief assume to be true that hasn't been validated?
   - **Internal contradictions.** Does the scope match the audience? Does the success metric match the problem?

4. **Receive findings** as a structured list. Each finding should have:
   - Category
   - Severity (blocker / important / nitpick)
   - Quoted excerpt from the brief
   - Specific suggested change

5. **Triage with your human partner.** For each finding:
   - **Blocker:** must address before finalizing. Return to step 3 or 4 for the affected section.
   - **Important:** address if the human agrees. Capture the decision either way.
   - **Nitpick:** note in `elicitation-notes.md`, do not block.

6. **Iterate if needed.** If blockers were addressed, optionally re-run the adversary on the revised draft. Stop after at most two adversary passes — diminishing returns.

7. **Record outcome** in `.factory/planning/adversarial-review-brief.md`:

```markdown
# Adversarial Review: Product Brief

**Date:** <YYYY-MM-DD>
**Passes:** <N>

## Findings

### Blockers (addressed)
- <quote>: <change made>

### Important (addressed)
- <quote>: <change made or "rejected: <reason>">

### Nitpicks (noted)
- <quote>: <decision>

## Outcome

<One of: APPROVED FOR FINALIZE | RETURNED TO STEP 3 | RETURNED TO STEP 4>
```

## Decision points

- **Adversary returns no findings** → suspicious. Either the brief is genuinely tight, or the adversary did not read carefully. Spot-check one section by asking the adversary "what's the weakest claim in section [X]?" If still nothing, accept and move on.
- **Adversary returns 20+ findings** → the brief needs more elicitation, not more review. Return to step 3 for the weakest sections.
- **Human disagrees with a blocker** → capture the disagreement and the rationale. The human owns the brief; the adversary is advisory.

## Failure modes

- **Sharing conversation history with the adversary.** The information asymmetry is the value. Spawning the adversary with full context defeats the purpose.
- **Treating adversary findings as commands.** They are inputs to a human decision. The human still owns the call.
- **Running more than two passes.** Diminishing returns. If the brief still has issues after two passes, the issue is in elicitation, not review.

## Quality gate

- [ ] Adversary spawned with brief only (no conversation history)
- [ ] Findings categorized and triaged with the human
- [ ] All blockers addressed (or explicitly overridden with rationale)
- [ ] Outcome recorded in `adversarial-review-brief.md`

## Hand-off to next step

Pass the (possibly revised) draft to `step-06-finalize.md`.
