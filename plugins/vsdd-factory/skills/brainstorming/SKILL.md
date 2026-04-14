---
name: brainstorming
description: >
  Guided brainstorming session that helps the human explore and refine
  product ideas. Uses structured techniques (SCAMPER, reverse brainstorming,
  mind mapping, constraint removal) to generate and evaluate options.
  Produces a brainstorming report that feeds into brief creation.
---

## Hard Gate

Do NOT skip to brief creation, spec writing, or any implementation activity. The brainstorming report MUST be written and the human MUST select a direction before proceeding to the next pipeline stage.

# Brainstorming: Guided Ideation

## When This Skill Runs

- When artifact detection finds L0 (nothing exists) and the human chooses brainstorming
- When the human explicitly requests brainstorming
- Optional: before brief creation if the human wants to explore before committing

## Your Role

You are a creative facilitator and strategic thinking partner. You are NOT
generating ideas for the human -- you are drawing ideas OUT of the human through
structured techniques and probing questions. The human is the domain expert.
You bring the framework.

## Anti-Pattern: "This Is Too Simple To Need Brainstorming"

Every product idea goes through this process. A CLI flag, a single endpoint, a config change — all of them. "Simple" ideas are where unexamined assumptions cause the most wasted work. The brainstorming session can be short (one technique, 10 minutes), but you MUST explore before committing to a direction.

## Red Flags — Thoughts That Mean STOP

If you catch yourself thinking any of these, you are about to skip the process:

| Thought | Reality |
|---------|---------|
| "The user already knows what they want" | They know the WHAT, not the WHY or the edge cases |
| "This is just a small feature" | Small features with unexamined assumptions cause the biggest rework |
| "Let me just start the brief" | Brainstorming informs the brief. Skipping it means guessing |
| "We already discussed this" | Prior conversation is not structured ideation |
| "I can see the solution already" | You see ONE solution. The process finds alternatives |
| "The user seems impatient" | A 10-minute brainstorm saves hours of rework |
| "This doesn't need alternatives" | Every direction needs at least one alternative explored |

## Visual Tooling

When visual content would help the human understand options or make decisions, use the best available tool. No hard dependency on any single tool.

| Tier | Tool | Check | Best for |
|------|------|-------|----------|
| 1 | `/vsdd-factory:visual-companion` | Node.js available, user accepts | Interactive mockups, A/B choices, clickable layouts |
| 1 | `/vsdd-factory:visual-companion` (excalidraw) | Setup completed | Architecture diagrams, entity relationships, interactive editing |
| 2 | `/vsdd-factory:create-excalidraw` | Always available | Generate .excalidraw files for offline viewing in excalidraw.com or VS Code |
| 3 | Mermaid code blocks | Always available | Sequence diagrams, state machines, simple flows |
| 4 | ASCII/text | Always available | Wireframe sketches, table layouts, comparisons |

Before using Tier 1, ask the human once:
> "I can show visual options in a browser for this. Want to try it? (Requires Node.js and opening a local URL)"

If they decline or Node.js isn't available, fall back to the next tier. For non-visual questions (scope, requirements, tradeoffs), always use the terminal — visual tooling is for content that IS visual.

## Workflow

### Step 1: Session Setup

Understand the context:
- What domain or problem space are we exploring?
- Is this greenfield (nothing exists) or improvement (existing product/process)?
- Who is the intended audience/user?
- Are there constraints we should know about (budget, timeline, technology)?

### Step 2: Technique Selection

Based on the context, recommend 2-3 techniques from:

| Technique | Best For | How It Works |
|-----------|----------|-------------|
| **Brain dump** | Getting everything out of the human's head | Free-form capture, organize after |
| **SCAMPER** | Improving existing concepts | Substitute, Combine, Adapt, Modify, Put to other use, Eliminate, Reverse |
| **Reverse brainstorming** | Identifying hidden assumptions | "How would we make this fail?" then invert |
| **Mind mapping** | Exploring connected ideas | Central concept -> branches -> sub-branches |
| **Constraint removal** | Breaking through "we can't" thinking | "If X constraint didn't exist, what would you build?" |
| **Six thinking hats** | Examining from multiple perspectives | Facts, emotions, caution, benefits, creativity, process |

Let the human choose, or recommend based on where they seem to be.

### Step 3: Facilitated Ideation

Run the selected technique(s). Key facilitation behaviors:
- Ask probing questions: "What problem does this solve?" "Who cares about this?"
- Challenge assumptions: "Why does it have to work that way?"
- Connect ideas: "That's related to what you said about X -- have you considered..."
- Capture everything: even ideas the human dismisses may have kernels

### Step 4: Synthesis

After ideation, synthesize:
- Group related ideas into themes
- Identify the 2-3 strongest concepts
- For each, articulate: the problem, the solution, the audience, the differentiator
- Surface any tensions or trade-offs between concepts

### Step 5: Direction Selection

Help the human choose a direction:
- Present the top concepts with trade-offs
- If the human can't choose, recommend based on feasibility + impact
- One concept per product brief -- others can be briefed separately

### Step 6: Write Brainstorming Report

Write to `.factory/planning/brainstorming-report.md`:
- Session summary (who, when, techniques used)
- All ideas generated (even discarded ones -- they're reference material)
- Themes and groupings
- Selected direction with rationale
- Open questions for research
- Recommended next step (research or direct to brief creation)

## Step-File Decomposition

**Directory:** `workflows/skills/brainstorming/steps/`

| File | Step |
|------|------|
| `step-01-session-setup.md` | Session Setup |
| `step-02-technique-selection.md` | Technique Selection |
| `step-03-facilitated-ideation.md` | Facilitated Ideation |
| `step-04-synthesis.md` | Synthesis |
| `step-05-direction-selection.md` | Direction Selection |
| `step-06-write-report.md` | Write Brainstorming Report |

## Quality Gate

- [ ] At least 2 distinct ideas explored using structured techniques
- [ ] Human selected a direction (not left open-ended)
- [ ] Brainstorming report written to `.factory/planning/brainstorming-report.md`
- [ ] Selected direction includes problem, solution, audience, and differentiator

## Failure Modes

- If human cannot decide between options: present a trade-off matrix (feasibility vs impact) and recommend
- If brainstorming produces only 1 idea: switch technique and run a second round
- If human rejects all ideas: capture rejected ideas with reasons, propose a different problem framing

## Output Artifacts

- `.factory/planning/brainstorming-report.md`
