---
name: create-brief
description: Create a product brief through guided discovery. Asks questions to understand the product vision, users, constraints, and success criteria. Writes to .factory/specs/product-brief.md.
disable-model-invocation: true
allowed-tools: Read, Write, Edit, Bash, AskUserQuestion
---

# Create Product Brief

Guide the user through creating a product brief — the L1 foundation that everything else builds on.

## Templates

Read and follow the output format in:
- `.claude/templates/product-brief-template.md` — L1 product brief structure

## Before Starting

1. Run `/factory-health` to ensure `.factory/` is healthy.
2. Check if `.factory/specs/product-brief.md` already exists. If so, ask: update existing or start fresh?
3. Check `.factory/specs/research/RESEARCH-INDEX.md` for prior domain research. If research exists, read the relevant reports to inform the brief creation — don't ask questions the research already answers.

## Discovery Process

Ask questions **one at a time**. Use multiple choice when possible.

### 1. Vision & Problem

- What problem does this product solve?
- Who has this problem? (target users)
- What happens if this problem isn't solved? (pain/stakes)

### 2. Users & Personas

- Who are the primary users?
- Who are secondary users?
- What's their technical sophistication?

### 3. Core Value Proposition

- What makes this different from alternatives?
- What's the one thing this must do well?
- What is explicitly out of scope?

### 4. Success Criteria

- How will you know this works?
- What metrics matter?
- What's the MVP vs full vision?

### 5. Constraints

- Technical constraints (language, platform, integrations)
- Timeline constraints
- Team/resource constraints
- Regulatory or compliance requirements

### 6. Prior Art

- Existing codebases to reference (for gene transfusion/semport)
- Competitor products
- Internal tools this replaces

## Output

Write to `.factory/specs/product-brief.md`:

```markdown
# Product Brief: <Product Name>

**Author:** <user name>
**Date:** <current date>
**Status:** draft

## Problem Statement
<What problem, who has it, what are the stakes>

## Target Users
<Primary and secondary personas>

## Value Proposition
<What makes this unique, core differentiator>

## Success Criteria
<Measurable outcomes>

## Scope
### In Scope
<What we're building>

### Out of Scope
<What we're explicitly not building>

## Constraints
<Technical, timeline, regulatory>

## Prior Art & References
<Existing codebases, competitors, reference implementations>

## Open Questions
<Things we need to resolve before moving to PRD>
```

## After Writing

1. Commit to factory-artifacts: `cd .factory && git add specs/product-brief.md && git commit -m "factory(phase-1): create product brief"`
2. Invoke `state-update` to set phase to `phase-1`, status `in-progress`.
3. Tell the user: "Brief created. Next step: `/create-prd` to elaborate into a full PRD, or `/create-domain-spec` if the domain needs deeper modeling first."
