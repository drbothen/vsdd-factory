# Early-Phase Gaps — Design Spec

## Summary

Adopt 4 insights from superpowers:brainstorming into vsdd-factory's early-phase pipeline, plus port the visual companion as a standalone skill.

## Problem

vsdd-factory's early-phase skills (brainstorming, brief creation, spec crystallization) lack several behavior-shaping and quality mechanisms that superpowers has proven effective:

1. No pre-adversarial self-review — specs go straight to expensive adversarial review without catching obvious gaps first
2. No explicit anti-pattern/Red Flags guidance in brainstorming — agents can rationalize skipping the process
3. No visual tooling for interactive mockups and diagrams during ideation
4. No explicit hard gates preventing premature advancement through the pipeline

## Deliverables

### 1. Visual Companion — Standalone Skill

**Location:** `skills/visual-companion/`

**Structure:**
```
skills/visual-companion/
├── SKILL.md
├── visual-guide.md
└── scripts/
    ├── server.cjs
    ├── helper.js
    ├── frame-template.html
    ├── start-server.sh
    └── stop-server.sh
```

**Ported from:** superpowers:brainstorming/scripts/ (zero-dependency Node.js HTTP + WebSocket server)

**Changes from superpowers:**
- Rename "Superpowers Brainstorming" → "VSDD Visual Companion" in frame template header
- Remove superpowers GitHub link from header
- Change session directory from `.superpowers/brainstorm/` → `.factory/visual-companion/`
- SKILL.md is the entry point — start/stop server, usage overview
- visual-guide.md has detailed CSS classes, event format, design tips

**The visual companion is optional.** Skills reference it through a tiered visual tooling strategy:

| Tier | Tool | When available | Best for |
|------|------|----------------|----------|
| 1 | `/vsdd-factory:visual-companion` | Node.js installed, user accepts | Interactive mockups, A/B choices |
| 2 | `/vsdd-factory:excalidraw-export` | Already in plugin | Architecture diagrams, flow charts |
| 3 | Mermaid code blocks | Always | Sequence diagrams, state machines |
| 4 | ASCII/text | Always | Wireframe sketches, comparisons |

### 2. Pre-adversarial Self-Review Checklist

Added to **4 skills**: create-brief, create-prd, create-architecture, create-domain-spec.

Placed after artifact writing, before adversarial review suggestion:

```markdown
## Self-Review (before adversarial review)

Before routing to adversarial review, check your own work:

1. **Placeholder scan:** Any "TBD", "TODO", incomplete sections, or vague requirements? Fix them now.
2. **Internal consistency:** Do any sections contradict each other? Do IDs match across files?
3. **Scope check:** Is this focused enough for the next pipeline stage, or does it need decomposition?
4. **Ambiguity check:** Could any requirement be interpreted two different ways? Pick one and make it explicit.

Fix issues inline. This is a cheap filter — catch obvious gaps before spending tokens on the adversary.
```

Adapted per skill with context-specific hints (e.g., create-prd: "Do BC IDs match the PRD index?").

### 3. "Too Simple" Anti-Pattern + Red Flags Table

Added to **brainstorming skill** near the top, after description, before technique selection.

**Anti-pattern:**
```markdown
## Anti-Pattern: "This Is Too Simple To Need Brainstorming"

Every product idea goes through this process. A CLI flag, a single endpoint, a config change — all of them. "Simple" ideas are where unexamined assumptions cause the most wasted work. The brainstorming session can be short (one technique, 10 minutes), but you MUST explore before committing to a direction.
```

**Red Flags:**

| Thought | Reality |
|---------|---------|
| "The user already knows what they want" | They know the WHAT, not the WHY or the edge cases |
| "This is just a small feature" | Small features with unexamined assumptions cause the biggest rework |
| "Let me just start the brief" | Brainstorming informs the brief. Skipping it means guessing |
| "We already discussed this" | Prior conversation is not structured ideation |
| "I can see the solution already" | You see ONE solution. The process finds alternatives |
| "The user seems impatient" | A 10-minute brainstorm saves hours of rework |
| "This doesn't need alternatives" | Every direction needs at least one alternative explored |

### 4. Explicit Hard Gate Language

Added to **5 skills**, tailored per position in pipeline:

- **brainstorming:** Do NOT skip to brief creation. Brainstorming report MUST be written, human MUST select direction.
- **guided-brief-creation:** Do NOT skip to PRD. Brief MUST be completed and validated.
- **create-brief:** Do NOT skip to PRD. Every discovery section MUST be explored with the human.
- **create-prd:** Do NOT skip to architecture. Every BC MUST have testable preconditions/postconditions.
- **create-architecture:** Do NOT skip to stories. Purity boundaries MUST be drawn, VPs MUST be defined.

### 5. Visual Tooling References

Added to **3 skills**: brainstorming, guided-brief-creation, create-architecture.

Shared tiered tooling table with consent prompt for Tier 1. Non-visual questions always use terminal.

### 6. Doc Updates

| File | Change |
|------|--------|
| `docs/FACTORY.md` | Mention visual companion in project tooling section |
| `docs/VSDD.md` | Add visual-companion to Tooling section |

## Files Created

- `skills/visual-companion/SKILL.md`
- `skills/visual-companion/visual-guide.md`
- `skills/visual-companion/scripts/server.cjs`
- `skills/visual-companion/scripts/helper.js`
- `skills/visual-companion/scripts/frame-template.html`
- `skills/visual-companion/scripts/start-server.sh`
- `skills/visual-companion/scripts/stop-server.sh`

## Files Modified

- `skills/brainstorming/SKILL.md` — anti-pattern, Red Flags, hard gate, visual tooling
- `skills/guided-brief-creation/SKILL.md` — hard gate, visual tooling
- `skills/create-brief/SKILL.md` — hard gate, self-review
- `skills/create-prd/SKILL.md` — hard gate, self-review
- `skills/create-architecture/SKILL.md` — hard gate, self-review, visual tooling
- `skills/create-domain-spec/SKILL.md` — self-review
- `docs/FACTORY.md` — visual companion mention
- `docs/VSDD.md` — visual companion in Tooling section

## Non-Goals

- This does NOT change the pipeline structure or workflow files
- This does NOT add new quality gates — the self-review is advisory, not blocking
- This does NOT make the visual companion mandatory — it's always Tier 1 in an optional tiered strategy
