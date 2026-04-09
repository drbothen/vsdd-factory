# Step 1: Session Setup

> **Parent skill:** `brainstorming` — see `../SKILL.md` for the full workflow.
> **Agent:** orchestrator
> **This step:** establish context and frame for the session before any techniques are selected.

## Inputs

- Initial human message that triggered the session (vague idea, problem statement, or "let's brainstorm")
- Optional: artifacts your human partner pasted in or referenced
- Optional: routing context from `artifact-detection` (typically L0 — nothing exists)

## Outputs

- A session frame in working memory: domain, mode (greenfield vs improvement), audience, constraints, energy
- A short context block you will reuse in step 6 (the report)

## Procedure

1. Open with a short framing line. Use this kind of opener verbatim:
   > "Before we start generating ideas, I want to understand the shape of what we're exploring. I'll ask four quick questions, then we'll pick a technique."

2. Ask the four framing questions one at a time. Wait for an answer before asking the next. Do not batch.

   1. **Domain.** "What domain or problem space are we exploring? Give me a sentence or two — not a pitch."
   2. **Mode.** "Is this greenfield — nothing exists yet — or are we improving something that already exists? If it's improvement, what's there today?"
   3. **Audience.** "Who is this for? Be specific — 'developers' is too broad, 'Rust developers debugging async deadlocks' is the kind of thing I want."
   4. **Constraints.** "Any constraints I should know about up front? Budget, timeline, technology you're stuck with, technology you refuse to use, organizational politics — anything."

3. After each answer, mirror it back in one sentence and ask "did I get that right?" before moving on. This catches drift early and signals you are listening.

4. Read your human partner's energy. If answers are terse, acknowledge it: "You sound tired of the framing — want to skip ahead and just dump ideas?" If they are excited, ride it.

5. Capture answers verbatim into your working notes. You will need them for the report.

## Decision points

- **Greenfield vs improvement** drives technique selection in step 2. Greenfield favors brain dump, mind mapping, constraint removal. Improvement favors SCAMPER and reverse brainstorming.
- **Energy is low** → propose a single technique and a 15-minute timebox instead of a full session.
- **Human resists framing** → skip to step 2 with whatever you have. Note the gap; revisit during synthesis.

## Failure modes

- **Human dumps a fully-formed pitch instead of answering the framing.** Capture the pitch, then say: "Helpful context — let me still get answers to the four framing questions so I pick the right technique." Do not skip framing.
- **Audience cannot be articulated.** Do not push. Note "audience: undefined — needs validation" and revisit in step 4.
- **Problem and solution are conflated.** Gently separate: "That sounds like a solution. What's the underlying problem it addresses?"

## Quality gate

- [ ] Domain captured in one or two sentences
- [ ] Mode classified (greenfield or improvement)
- [ ] Audience captured (or explicitly flagged as undefined)
- [ ] Constraints captured (or explicitly "none mentioned")
- [ ] Human confirmed the framing back to you

## Hand-off to next step

Pass the session frame (domain, mode, audience, constraints, energy) to `step-02-technique-selection.md`.
