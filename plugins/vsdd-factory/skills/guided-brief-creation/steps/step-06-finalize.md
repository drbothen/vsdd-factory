# Step 6: Finalize Brief

> **Parent skill:** `guided-brief-creation` — see `../SKILL.md` for the full workflow.
> **Agent:** product-owner
> **This step:** persist the approved brief and any elicitation notes, then route to Phase 1.

## Inputs

- Approved draft from step 4 (possibly revised by step 5)
- `elicitation-notes.md` content from steps 1–3
- Adversarial review outcome from step 5 if it ran

## Outputs

- `.factory/planning/product-brief.md` — the final brief
- `.factory/planning/elicitation-notes.md` — overflow context for Phase 1 (if non-trivial content was captured)
- Routing handoff to Phase 1 (Spec Crystallization)

## Procedure

1. **Create `.factory/planning/`** if it does not exist.

2. **Write the final brief** to `.factory/planning/product-brief.md`. Use the structure from step 4. Update the status frontmatter to `final` (not `draft`).

3. **Write elicitation notes** to `.factory/planning/elicitation-notes.md` if any non-trivial overflow was captured. Use this structure:

```markdown
# Elicitation Notes

Captured during guided brief creation. These details exceeded the brief's scope but may inform Phase 1 (PRD, architecture, NFRs).

## Implementation hints
<Things the human said about how it should be built. Not commitments — hints.>

## Edge cases mentioned
<Specific scenarios surfaced during elicitation that didn't fit the brief.>

## Future-version ideas
<Out-of-scope items that should be revisited later.>

## Assumptions to validate
<Things the human asserted that have not been confirmed.>

## Open questions
<Things the human did not know the answer to.>

## Document analysis findings
<If documents were analyzed in step 2, key findings here.>
```

4. **If `market-intel.md` was created in step 2**, leave it in place. Phase 1 will reference it.

5. **If adversarial review ran in step 5**, leave `adversarial-review-brief.md` in place. It is part of the audit trail.

6. **Confirm with the human:**
   > "Final brief is at `.factory/planning/product-brief.md`. Elicitation notes are at `.factory/planning/elicitation-notes.md` for Phase 1 to reference. Ready to route to spec crystallization?"

7. **Route to Phase 1.** The next skill is `create-prd` (or whichever spec-crystallization entry the orchestrator uses). Pass:
   - Path to the brief
   - Path to elicitation notes (if exists)
   - Path to market-intel.md (if exists)
   - Path to adversarial review (if exists)

8. **Update `.factory/STATE.md`** to reflect that Phase 0 (planning) is complete and Phase 1 (spec crystallization) is starting. Use the `state-update` skill if available.

## Failure modes

- **Forgetting to persist elicitation notes.** They contain hours of human context that Phase 1 will otherwise rediscover painfully. Always write them.
- **Marking the brief `final` while open questions remain.** Open questions are fine in a final brief — they become Phase 1 inputs. But they must be listed, not hidden.
- **Routing to Phase 1 without confirming.** Always confirm out loud. The human may want to pause.

## Quality gate

- [ ] `product-brief.md` exists at `.factory/planning/product-brief.md` with status `final`
- [ ] Elicitation notes written if any overflow exists
- [ ] All supporting artifacts (`market-intel.md`, `adversarial-review-brief.md`) preserved
- [ ] Human confirmed routing to Phase 1
- [ ] `.factory/STATE.md` updated to reflect phase transition

## Hand-off to next step

End of `guided-brief-creation` workflow. Orchestrator invokes the spec-crystallization entry skill (typically `create-prd`).
