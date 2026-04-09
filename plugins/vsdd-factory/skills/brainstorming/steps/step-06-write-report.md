# Step 6: Write Brainstorming Report

> **Parent skill:** `brainstorming` — see `../SKILL.md` for the full workflow.
> **Agent:** product-owner
> **This step:** persist the entire session as `.factory/planning/brainstorming-report.md`.

## Inputs

- Session frame from step 1
- Raw idea log from step 3
- Themes and strongest concepts from step 4
- Selected direction, rationale, parked concepts, rejected ideas from step 5

## Outputs

- `.factory/planning/brainstorming-report.md`
- A one-line recommendation for the next step (research vs direct to brief creation)

## Procedure

1. Create `.factory/planning/` if it does not exist.

2. Write the report using exactly this structure:

```markdown
# Brainstorming Report

## Session Summary

- **Date:** <YYYY-MM-DD>
- **Domain:** <one or two sentences from step 1>
- **Mode:** greenfield | improvement
- **Audience:** <captured in step 1, or "undefined — needs validation">
- **Constraints:** <captured in step 1, or "none stated">
- **Techniques used:** <e.g., SCAMPER, Reverse brainstorming>

## Selected Direction

**Concept name:** <short name>

- **Problem:** <what's broken in the world>
- **Solution:** <what this concept does about it>
- **Audience:** <who specifically>
- **Differentiator:** <why this beats what exists>

**Rationale for selection:** <verbatim from human in step 5>

## Themes

### Theme 1: <name>
- <idea>
- <idea>

### Theme 2: <name>
- <idea>

<...one section per theme from step 4...>

## All Ideas Generated

<Raw idea log from step 3, in original order. Do not filter. Mark dismissed ideas with [dismissed: <reason>] inline.>

## Not Chosen — Reference for Later

<Parked concepts from step 5. For each, give the four-part articulation so a future session can pick them up cold.>

## Rejected Ideas

<Ideas the human explicitly rejected, with the reason. These reveal the real selection criteria.>

## Open Questions

- <Things the human did not know during the session>
- <Assumptions that need validation>
- <Audience gaps if audience was undefined>

## Recommended Next Step

<One of:>
- Proceed directly to `guided-brief-creation` — direction is clear and audience is well-defined.
- Run `research-agent` first on <topic> — open questions about <competitive landscape | technical feasibility | audience validation> should be answered before briefing.
```

3. Fill every section. Do not leave placeholders. If a section has no content, write "None" — absence is itself information.

4. After writing, present the file path to your human partner:
   > "Report is at `.factory/planning/brainstorming-report.md`. Want me to read it back, or are you ready to move on?"

5. State the recommended next step out loud and confirm:
   > "I'd suggest [research-agent on X | guided-brief-creation] next. Sound right?"

## Failure modes

- **Summarizing instead of preserving.** "All Ideas Generated" is verbatim, not editorialized.
- **Skipping rejected-ideas because it feels negative.** Most useful section for understanding real criteria.
- **Recommending the next step without justifying it.** Tie to a specific gap or readiness signal.

## Quality gate

- [ ] File exists at `.factory/planning/brainstorming-report.md`
- [ ] All sections filled (or explicitly "None")
- [ ] Selected direction has all four parts
- [ ] Recommended next step is named and justified
- [ ] Human acknowledged the report

## Hand-off to next step

End of `brainstorming` workflow. Orchestrator routes next based on the recommended next step in the report.
