# Step 3: Guided Elicitation

> **Parent skill:** `guided-brief-creation` — see `../SKILL.md` for the full workflow.
> **Agent:** product-owner
> **This step:** walk your human partner through each section of the brief, drawing out the content through structured conversation.

## Inputs

- Intent statement and elicitation notes from steps 1–2
- Discovery summary, `market-intel.md`, and gap list from step 2
- The brief template at `templates/product-brief-template.md`

## Outputs

- Section-by-section content in working memory, ready for step 4 to draft
- Updated `elicitation-notes.md` with overflow details

## Procedure

For each section of the brief, run the elicitation script. Stay in the section until your partner has given a substantive answer. Mirror back, then move on.

### Section: What Is This?

Goal: one paragraph that captures the essence.

> "If you had 30 seconds to explain this to someone — say, in an elevator — what would you say?"

Listen. Mirror back. Then push for sharpness:
> "Can we cut it shorter? What's the one sentence that survives?"

Capture the one-sentence version AND the longer paragraph.

### Section: Who Is It For?

Goal: a specific, named audience.

> "Describe your ideal user. What's their job? What's a typical day for them? What frustrates them today?"

Push for specificity:
> "Is that really the primary user, or is there someone upstream — someone who hands them work or signs off on what they do?"

Reference `market-intel.md` if it exists:
> "Research surfaced [pain X] in this audience. Does that match what you've seen?"

Distinguish primary from secondary audiences. The brief carries one primary audience.

### Section: Why Does It Matter?

Goal: connect to outcomes.

> "If this doesn't get built, what happens? Who loses? What gets worse?"

Challenge nice-to-haves:
> "Is that a nice-to-have or a need-to-have? What would convince a skeptic that this matters?"

Capture the answer in cost/benefit terms when possible.

### Section: What Makes It Different?

Goal: competitive positioning.

> "What do people use today instead of this? Why is that not good enough?"

If they say "nothing exists":
> "Then why hasn't anyone built it? Either there's a hidden reason it can't exist, or there's an opportunity. Which do you think?"

Reference `market-intel.md`:
> "Research found [competitor]. How does this compare?"

### Section: Scope

Goal: the smallest valuable v1.

> "What's the ONE thing this must do in version 1? If it does only that, is it still worth building?"

Push for cuts:
> "If you had to cut your list in half, what stays?"
> "What goes in v2 instead of v1?"

Capture both in-scope and explicitly out-of-scope. Out-of-scope is as important as in-scope.

### Section: Success

Goal: measurable outcomes.

> "How would you know if this succeeded? What number would change?"

Challenge unmeasurables:
> "Is 'users love it' measurable? How would you actually track it? NPS? Retention? Daily active use?"

Capture the metric AND how it would be tracked.

### Section: Constraints (carry over from step 1)

Goal: surface anything that limits the solution space.

> "Anything you forgot to mention earlier? Budget, deadline, tech you must use or must avoid, integrations, compliance?"

Reference any constraints captured in step 1 — confirm they still apply.

## Throughout elicitation

- **Capture-don't-interrupt:** if details exceed brief scope (implementation specifics, edge cases, future ideas), capture in `elicitation-notes.md` and move on.
- **Mirror back every section:** "So for 'who is it for', I have [X]. Does that capture it?"
- **Do not move on until each section has substance.** A blank section is a failure of this step.

## Decision points

- **Contradictory answers between sections** (e.g., audience says "enterprise" but scope says "weekend project") → flag the contradiction immediately. Do not proceed: "These two answers point in different directions. Which one is right?"
- **Human stalls on a section** → offer a forced-choice prompt: "Pick one of these three options, even if none is perfect."
- **Multiple audiences emerge** → ask "which one is primary?" and capture the others as secondary. The brief leads with primary.

## Failure modes

- **Filling in the section yourself when the human gives a thin answer.** Push for more. "That's a start — give me more."
- **Skipping mirror-backs to save time.** Mirror-backs catch misunderstandings. Skipping them causes step 4 rework.
- **Letting elicitation become a brain dump.** Stay in section structure. Capture overflow to notes, return to the section.

## Quality gate

- [ ] Every brief section has substantive content captured
- [ ] Mirror-backs done for each section
- [ ] Contradictions surfaced and resolved (or explicitly flagged)
- [ ] Out-of-scope items captured alongside in-scope
- [ ] Success metrics are measurable, not aspirational
- [ ] `elicitation-notes.md` updated with overflow

## Hand-off to next step

Pass section-by-section content and updated notes to `step-04-draft-review.md`.
