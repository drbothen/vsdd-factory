---
name: multi-variant-design
description: >
  Generates 2-3 design variants for complex screens. Each variant scored
  on 6 dimensions by different agents. Top variant + runner-up presented
  to human for selection or synthesis.
agents:
  primary: ux-designer
  supporting: [accessibility-auditor, business-analyst, architect, consistency-validator]
inputs:
  - UX spec screen definition
  - .factory/design-system/ (tokens, components, patterns)
outputs:
  - .factory/ui-quality/variants/SCR-NNN-variants.md
gate: Human selects variant or requests synthesis
condition: "feature_type in ['ui', 'full-stack']"
---

# Multi-Variant Design Generation

## When Multi-Variant Runs

- Screens with >5 interactive elements
- Screens with complex data display (tables, charts, dashboards)
- Screens identified as "high-traffic" in the UX spec
- Human explicitly requests "explore alternatives for this screen"

## Runs During

Given the UX spec and PRD (during spec crystallization / spec evolution).
This is a design exploration activity, not an implementation activity.

## Procedure

1. **ux-designer identifies complex screens** from UX spec
2. **ux-designer generates 3 variants** for each complex screen
   - Each variant uses design system tokens and components
   - Each variant takes a different layout/interaction approach
   - Variants documented in `.factory/ui-quality/variants/SCR-NNN-variants.md`
3. **4 agents score each variant in parallel:**

| Dimension | Agent | What It Checks |
|-----------|-------|---------------|
| Usability | business-analyst | Task completion efficiency, cognitive load |
| Accessibility | accessibility-auditor | WCAG compliance, keyboard nav, screen reader |
| Performance | architect | Estimated render complexity, CWV impact |
| Design Compliance | consistency-validator | Token usage, component contract adherence |
| Visual Hierarchy | ux-designer | Information architecture, scannability |
| Responsive Fitness | ux-designer | How well it adapts across breakpoints |

4. **Composite scores computed** (weighted average)
5. **Top variant + runner-up presented to human**
6. **Human selects** or requests synthesis of best aspects
7. **Selected variant becomes the UX spec** for that screen

## Variant Document Format

```markdown
# Screen Variants: [Screen Name] (SCR-NNN)

## Variant A: [Approach Name]
### Layout Description
### Component Usage
### Interaction Flow
### Scores
| Dimension | Score | Notes |
|-----------|-------|-------|

## Variant B: [Approach Name]
...

## Variant C: [Approach Name]
...

## Recommendation
Top: Variant [X] (score: N.NN)
Runner-up: Variant [Y] (score: N.NN)

## Human Decision
[Pending / Selected: Variant X / Synthesis requested]
```

## Failure Modes

- If fewer than 2 distinct variants can be generated (simple screen, limited layout options): report constraint, proceed with single design, and note in variants document
- If scoring deadlocks (two variants within 0.05 composite score): present both to human with dimension-level breakdown for tiebreaking
- If an agent scorer is unavailable: skip that dimension, note the missing score, and reweight remaining dimensions
