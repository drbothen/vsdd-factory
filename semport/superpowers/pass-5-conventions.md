# Pass 5: Conventions & Patterns

## SKILL.md Frontmatter Schema

```
---
name: <kebab-case-skill-name>            # matches directory name
description: <trigger criterion string>  # tells the agent when to invoke
---
```

Examples:

- `skills/brainstorming/SKILL.md:1-4`
- `skills/test-driven-development/SKILL.md:1-4`
- `skills/verification-before-completion/SKILL.md:1-4` — description is unusually long; uses it as a full trigger prompt

The `description` field is **the entire trigger criterion** — the agent reads descriptions across all available skills and self-selects. Writing a good description is writing a good dispatch rule.

## Naming Conventions

- Skills: kebab-case, action-gerund ("brainstorming", "writing-plans", "executing-plans", "using-git-worktrees", "finishing-a-development-branch"). The naming pattern is "verb-ing + object" so the description reads naturally.
- Agents: kebab-case noun ("code-reviewer")
- Directories mirror names exactly

## Skill Document Structure (empirical pattern across all skills)

1. Frontmatter (name + description)
2. Optional `<SUBAGENT-STOP>` / `<EXTREMELY-IMPORTANT>` / `<HARD-GATE>` tags
3. `# Title`
4. `## Overview` — 1-3 sentences
5. `**Core principle:** <one-liner>`
6. `**Violating the letter of the rules is violating the spirit of the rules.**` (TDD, debugging, verification — Iron Law skills)
7. `## The Iron Law` (Rigid skills only) — single blockquote with the law
8. `## When to Use` / `## Checklist` / `## The Process` / process flow in graphviz `dot` code block
9. `## Red Flags` — rationalization table
10. Examples / anti-patterns / references

## Announce-at-Start Convention

Most skills open execution with `**Announce at start:** "I'm using the <skill> skill to <purpose>."` — e.g. `skills/writing-plans/SKILL.md:14`, `skills/finishing-a-development-branch/SKILL.md:14`. This creates a user-visible audit trail of which skills fired.

## Graphviz DOT for Flow Diagrams

Every process flow is rendered as inline `dot` code (not mermaid), e.g. `skills/test-driven-development/SKILL.md:49-68`, `skills/using-superpowers/SKILL.md:48-76`, `skills/brainstorming/SKILL.md:36-74`. Choice likely driven by DOT's more precise shape vocabulary (doublecircle for start/end, diamond for decisions, box for actions).

## Red Flags Table Pattern

| Thought | Reality |
|---|---|
| rationalization | rebuttal |

12+ entries typical. Positioned after the main process so the agent has already been told the rule and is now pre-inoculated against excuses. This is **behavioral TDD** per `writing-skills`: enumerate the failure modes observed during pressure testing, plug each one.

## Iron Law Pattern

A single all-caps blockquote, always the same form:

- "NO PRODUCTION CODE WITHOUT A FAILING TEST FIRST" (TDD)
- "NO FIXES WITHOUT ROOT CAUSE INVESTIGATION FIRST" (debugging)
- "NO COMPLETION CLAIMS WITHOUT FRESH VERIFICATION EVIDENCE" (verification)

Reserved for the Rigid skills. Flexible skills use "Core principle" instead.

## Skill-Priority Rules

`skills/using-superpowers/SKILL.md:98-105`:

1. Process skills first (brainstorming, debugging) — determine HOW
2. Implementation skills second — guide execution

> "Let's build X" → brainstorming first, then impl skills
> "Fix this bug" → debugging first, then domain skills

## Trigger Phrase Testing Convention

`tests/skill-triggering/prompts/<skill-name>.txt` holds the implicit phrase that should trigger the skill (e.g. `test-driven-development.txt`, `writing-plans.txt`). `tests/explicit-skill-requests/prompts/` holds explicit phrasings. Running the test = asking an agent the prompt and checking whether it invoked the right skill.

## Output Conventions

- Design docs: `docs/superpowers/specs/YYYY-MM-DD-<topic>-design.md`
- Plans: `docs/superpowers/plans/YYYY-MM-DD-<feature>.md`
- Date-prefixed filename is the only organizational structure

## Voice Conventions

- "Your human partner" — not "the user" (project-wide, reinforced in `CLAUDE.md:77`)
- Imperative mood, short paragraphs, frequent ALL-CAPS for absolute rules
- Direct confrontational tone toward the agent's own tendencies ("Thinking 'skip TDD just this once'? Stop. That's rationalization." `skills/test-driven-development/SKILL.md:29`)

## Anti-Patterns (explicitly called out)

- `skills/test-driven-development/testing-anti-patterns.md` — reference doc
- "This is just a simple question" (using-superpowers red flag)
- "This Is Too Simple To Need A Design" (`skills/brainstorming/SKILL.md:16`)
