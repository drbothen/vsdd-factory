# Step 4: Synthesis

> **Parent skill:** `brainstorming` — see `../SKILL.md` for the full workflow.
> **Agent:** product-owner
> **This step:** turn the raw idea log into themes and identify the two or three strongest concepts.

## Inputs

- Raw idea log from `step-03-facilitated-ideation.md`
- Session frame from step 1 (audience and constraints matter here)

## Outputs

- A themed grouping of all ideas (every idea belongs to one theme)
- 2–3 "strongest concepts" each with: problem, solution, audience, differentiator
- A list of tensions or trade-offs between concepts

## Procedure

1. Take a beat. Tell your human partner what is happening:
   > "Now I'm going to organize what we generated. Give me a minute to group these into themes, then I'll show you what I see and you can correct me."

2. **Group into themes.** Read the raw log and cluster ideas by what they have in common. Aim for 3–6 themes. Avoid single-idea themes — force outliers into the closest cluster and note them.

3. Present themes back, one at a time:
   > "Theme 1: [name]. This includes [ideas]. The thread I see is [pattern]. Does that grouping make sense, or am I forcing it?"

   Adjust based on feedback. Themes are a tool, not a deliverable.

4. **Identify the strongest concepts.** Pick 2–3 that satisfy:
   - Strong signal in the log (multiple related ideas, returned to repeatedly)
   - Human visibly leaned in when discussing it
   - Plausibly fits the constraints from step 1

5. For each strongest concept, articulate the four parts using exactly this structure:

   > **Concept: [short name]**
   > - **Problem:** [what's broken in the world]
   > - **Solution:** [what this concept does about it]
   > - **Audience:** [who specifically this is for]
   > - **Differentiator:** [why this beats what exists today]

   If you cannot fill in all four, that is data — say so: "I can't fill in the differentiator. What makes it different from [alternative]?"

6. **Surface tensions.** Look for trade-offs between concepts:
   - Do they target different audiences?
   - Do they require incompatible technical foundations?
   - Does picking one foreclose another?
   - Different time-to-value?

   Present as a short list:
   > "Two tensions I see: (1) Concept A and Concept B target different users. (2) Concept C requires infrastructure A doesn't need."

7. Capture rejected ideas with reasons. They go in the report — they are reference material, not waste.

## Decision points

- **Only one concept emerges** → fine. Move to step 5 with one option, note in the report that the session converged.
- **Five or more concepts** → force-rank, present the top three. Five overwhelms a human.
- **No clear strongest concept** → return to step 3 with a different technique. Synthesis cannot manufacture signal that is not there.

## Failure modes

- **Theming by surface keyword instead of underlying pattern.** "Both ideas mention APIs" is not a theme. "Both ideas reduce integration friction" is.
- **Filling in the four parts from your assumptions instead of asking.** When unsure, ask. Do not invent the audience.
- **Hiding rejected ideas.** They go in the report.

## Quality gate

- [ ] Every idea from the raw log is in a theme or marked outlier
- [ ] 2–3 strongest concepts identified, each with all four parts (or explicit gaps named)
- [ ] Tensions surfaced and named
- [ ] Human confirmed the synthesis matches their thinking

## Hand-off to next step

Pass themed groupings, strongest concepts, and tensions to `step-05-direction-selection.md`.
