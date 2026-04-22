---
name: guided-brief-creation
description: >
  Interactive, facilitated workflow that guides the human from raw ideas
  to a structured product brief. Uses staged elicitation — understand intent
  first, then fill sections through conversation, then draft and review.
  Inspired by the BMAD Method's collaborative discovery pattern.
---

> **Delegation Reference:** This skill describes work the orchestrator delegates
> to specialist agents via the Agent tool. Each step names the target agent.
> The orchestrator does NOT execute these steps directly.

## Hard Gate

Do NOT skip to PRD creation, architecture design, or any implementation activity. The product brief MUST be completed and validated before proceeding.

# Guided Brief Creation

## Prerequisites

- Human has provided an initial product idea or description (via orchestrator conversation)
- `.factory/planning/` directory exists (or will be created during workflow)
- `templates/product-brief-template.md` exists for brief structure

## When This Skill Runs

- When artifact detection finds L0 (no artifacts) and human chooses brief creation
- After brainstorming, to capture the selected direction as a brief
- When the human says "help me write a product brief"

## Your Role

You are a product-focused collaborator guiding the human from raw ideas to a
structured product brief. You are NOT writing the brief for them -- you are
drawing out their vision through structured conversation.

The human is the domain expert. You bring structured thinking, facilitation,
and the ability to synthesize scattered input into clear narrative.

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

### Stage 1: Understand Intent

Before anything else, understand WHAT the brief is about.

**If the human provided context** (description, docs, brain dump):
- Summarize your understanding of the product/idea
- Ask: "Is that right? Anything to add or correct?"
- Ask: "Do you have any existing documents, research, or brainstorming I should review?"

**If the human provided nothing:**
- Ask: "What's your product or project idea about?"
- Let them brain dump -- capture everything, don't interrupt for structure
- Ask: "Anything else before I start organizing?"

**Capture-don't-interrupt:** If the human shares details beyond brief scope
(requirements, technical preferences, timeline), capture them for later.
Don't redirect -- let their creative flow continue.

### Stage 2: Contextual Discovery

If the human provided existing documents:
- Spawn research-agent to analyze provided documents
- Extract relevant insights for the brief
- If web research would help (competitive landscape, technical feasibility),
  spawn research-agent with Perplexity

If the human provided brainstorming output:
- Read `.factory/planning/brainstorming-report.md`
- Use the selected direction as the brief's foundation

### Stage 3: Guided Elicitation

For each section of the product brief template, engage the human:

**"What Is This?"** -- Help them articulate the essence in one paragraph.
Ask: "If you had 30 seconds to explain this to someone, what would you say?"

**"Who Is It For?"** -- Push for specificity.
Ask: "Describe your ideal user. What's their job? What frustrates them today?"
Challenge: "Is that really the primary user, or is there someone upstream?"

**"Why Does It Matter?"** -- Connect to business outcomes.
Ask: "If this doesn't get built, what happens? Who loses?"
Challenge: "Is that a nice-to-have or a need-to-have?"

**"What Makes It Different?"** -- Force competitive positioning.
Ask: "What do people use today instead? Why is that not good enough?"
If they say "nothing exists": "Then why hasn't anyone built it?"

**"Scope"** -- Draw boundaries.
Ask: "What's the ONE thing this must do in version 1?"
Challenge: "If you had to cut that list in half, what stays?"

**"Success"** -- Quantify outcomes.
Ask: "How would you know if this succeeded? What number would change?"
Challenge: "Is that measurable? How would you actually track it?"

### Stage 4: Draft & Review

- Draft the brief using `templates/product-brief-template.md`
- Present to the human section by section
- For each section: "Does this capture your intent? What's wrong or missing?"
- Iterate until the human approves each section

### Stage 5: Adversarial Review (Optional)

If the human wants confidence in the brief:
- Spawn adversary agent (fresh context) to review for:
  - Vague or unmeasurable success criteria
  - Scope too broad for a single product
  - Missing audience (who else might care?)
  - Competitive positioning gaps
- Present findings to human, iterate if needed

### Stage 6: Finalize

- Write final brief to `.factory/planning/product-brief.md`
- If detailed notes were captured during elicitation (requirements, technical
  constraints, timeline), write them to `.factory/planning/elicitation-notes.md`
  for Phase 1 to reference
- Route to Phase 1 (Spec Crystallization)

## Step-File Decomposition

**Directory:** `workflows/skills/guided-brief-creation/steps/`

| File | Step |
|------|------|
| `step-01-understand-intent.md` | Understand Intent |
| `step-02-contextual-discovery.md` | Contextual Discovery |
| `step-03-guided-elicitation.md` | Guided Elicitation |
| `step-04-draft-review.md` | Draft & Review |
| `step-05-adversarial-review.md` | Adversarial Review (Optional) |
| `step-06-finalize.md` | Finalize Brief |

## Market Intelligence Integration

When creating the Audience & Pain section, reference `.factory/planning/market-intel.md`:
- Confirmed pains → include in brief
- Unconfirmed pains → flag as assumptions requiring validation
- Differentiation opportunities → inform Identity & Scope
- Risk signals → inform Integration & Constraints

## Output Artifacts

- `.factory/planning/product-brief.md`
- `.factory/planning/elicitation-notes.md` (if additional context was captured)

## Failure Modes

- If human provides contradictory requirements: flag the specific contradictions and ask the human to resolve before proceeding
- If domain is unfamiliar (specialized industry, niche technology): spawn research-agent for domain context before Stage 3 elicitation
- If human disengages mid-elicitation: save progress to elicitation-notes.md and present a resumption summary on return
