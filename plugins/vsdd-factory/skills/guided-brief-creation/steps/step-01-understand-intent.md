# Step 1: Understand Intent

> **Parent skill:** `guided-brief-creation` — see `../SKILL.md` for the full workflow.
> **Agent:** product-owner
> **This step:** before drafting anything, understand WHAT the brief is about and what context the human brings.

## Inputs

- Initial human message (idea, description, brain dump, or "help me write a brief")
- Optional: pasted documents, links, prior brainstorming output
- Optional: routing context from `artifact-detection`

## Outputs

- A captured statement of intent in working memory
- A list of context sources to feed to step 2 (documents, brainstorming reports, research)
- An "elicitation notes" working file capturing details that don't belong in the brief itself

## Procedure

1. **If your human partner provided context** (description, docs, brain dump):

   1. Read what they provided.
   2. Summarize back in your own words: "Here's what I'm hearing. You're trying to build [X] for [Y] because [Z]. Is that right?"
   3. Wait for confirmation or correction.
   4. Ask: "Do you have any existing documents, research, or brainstorming I should review before we start?"

2. **If your human partner provided nothing** (just "help me write a brief"):

   1. Ask: "What's your product or project idea about? Don't worry about structure — just tell me."
   2. Let them brain dump. Do not interrupt for structure. Do not redirect to "the right section."
   3. When they slow down, ask: "Anything else before I start organizing?"

3. **Capture-don't-interrupt rule (from SKILL.md § Stage 1):** if your partner shares details that exceed brief scope (specific requirements, technical preferences, timeline, team structure), capture them — but do not redirect. Let their creative flow continue. These details go into `elicitation-notes.md`, not the brief.

4. **Detect the source mode** and record it:
   - **Fresh start:** no prior context, no docs, no brainstorming.
   - **Post-brainstorming:** routed from `brainstorming` with a report at `.factory/planning/brainstorming-report.md`.
   - **Document-driven:** human pasted or referenced documents.
   - **Mixed:** any combination.

5. **Probe for the missing pieces.** Even at this stage, you need to know enough to route step 2 properly:
   - "Is there an existing thing this replaces, or is this new?"
   - "Have you talked to anyone who would use this? What did they say?"
   - "Do you have skin in the game on a particular technology, or is the implementation open?"

   Capture answers but do not push hard. The goal is to know what to research in step 2, not to fill the brief.

## Decision points

- **Brainstorming report exists** → automatic context source for step 2. Read it before continuing.
- **Documents provided** → step 2 will spawn `research-agent` to analyze them. Note the paths.
- **Domain is unfamiliar** (specialized industry, niche tech) → step 2 will spawn `research-agent` for domain context before elicitation.
- **Human provides nothing and cannot articulate** → suggest brainstorming first: "I think we should start with brainstorming — want me to switch to that workflow?"

## Failure modes

- **Interrupting the brain dump to impose structure.** Capture everything first; structure later. The structure step is step 3.
- **Skipping the mirror-back.** Without it, you build on your own assumptions. Always summarize and confirm.
- **Treating the elicitation notes as the brief.** They are working material — the brief is a separate, structured deliverable in step 6.

## Quality gate

- [ ] Statement of intent captured in your own words and confirmed by the human
- [ ] Source mode detected (fresh / post-brainstorming / document-driven / mixed)
- [ ] All context sources noted (paths, links)
- [ ] `elicitation-notes.md` started (or planned) for overflow details
- [ ] Human confirmed they have shared everything they want to share

## Hand-off to next step

Pass intent statement, source mode, and context source list to `step-02-contextual-discovery.md`.
