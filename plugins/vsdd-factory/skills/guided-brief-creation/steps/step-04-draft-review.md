# Step 4: Draft and Review

> **Parent skill:** `guided-brief-creation` — see `../SKILL.md` for the full workflow.
> **Agent:** product-owner
> **This step:** turn elicited content into a draft brief and walk your human partner through it section by section.

## Inputs

- Section content from `step-03-guided-elicitation.md`
- The brief template at `templates/product-brief-template.md`
- `elicitation-notes.md` for overflow context

## Outputs

- A draft brief in working memory (not yet written to `.factory/planning/product-brief.md` — that happens in step 6)
- Section-by-section human approval

## Procedure

1. **Draft the brief using the template.** Use this structure (mirrors `templates/product-brief-template.md`):

```markdown
# Product Brief: <name>

**Status:** draft
**Date:** <YYYY-MM-DD>

## What Is This

<Two parts: one-sentence elevator version, then expanded paragraph from step 3.>

## Who Is It For

**Primary audience:** <specific named audience>

<Description of their job, day, frustrations.>

**Secondary audiences:** <if any>

## Why Does It Matter

<Outcome-driven justification. Cost of not building it.>

## What Makes It Different

<Competitive positioning. What people use today and why it's not enough.>

## Scope

**In v1:**
- <item>
- <item>

**Explicitly out of v1:**
- <item>
- <item>

## Success Criteria

- <Measurable metric 1, with tracking method>
- <Measurable metric 2, with tracking method>

## Constraints

- <Constraint>
- <Constraint>

## Open Questions

- <Things still unresolved>
```

2. **Present section by section, not all at once.** For each section:
   1. Show the section to your human partner.
   2. Ask: "Does this capture your intent? What's wrong or missing?"
   3. Iterate on the section until they approve before moving to the next.

3. **Do not batch all corrections to the end.** Iterating section-by-section catches drift early and prevents whole-brief rewrites.

4. **Use exact quotes** from elicitation where they were strong. If your human partner said "I want it to feel like a kitchen timer for AI agents," that line lives in the brief verbatim. Do not corporate-speak it into "intuitive task management."

5. **Track approval state** in your working notes:
   - Section 1: approved
   - Section 2: revising — change requested
   - ...

6. After every section is approved, ask the meta question:
   > "Reading the whole thing top to bottom — does it feel like one product, or two products in a trench coat?"

   If they hesitate, identify the seam and either tighten scope or split the brief.

## Decision points

- **Section needs more elicitation** → return to step 3 for that section only. Do not draft from imagination.
- **Two products emerge in one brief** → stop. Tell your partner: "I'm seeing two products. Let's pick one and brief the other separately." Return to selection.
- **Human approves everything in 30 seconds** → suspect rubber-stamping. Push back: "Read it once more carefully. Anything you'd change?"

## Failure modes

- **Drafting the whole brief and presenting it as a wall of text.** Section-by-section is non-negotiable. Walls of text get rubber-stamped.
- **Smoothing out the human's voice.** Their phrasing IS the brief in many places. Preserve it.
- **Treating "looks fine" as approval.** Push: "What would make it better?"

## Quality gate

- [ ] Every section has been presented to and approved by the human
- [ ] No section is empty or placeholder
- [ ] Out-of-scope is explicit, not implied
- [ ] Success criteria are measurable
- [ ] Open questions section is populated (or explicitly "None")
- [ ] Whole-brief read-through completed and approved

## Hand-off to next step

Pass the approved draft to `step-05-adversarial-review.md` (optional) or directly to `step-06-finalize.md` if your human partner skips the adversarial pass.
