# Pass 5: Conventions & Patterns

_Phase B deepening round 1._

## SKILL.md Frontmatter Schema

```
---
name: <kebab-case-skill-name>            # matches directory name
description: <trigger criterion string>  # tells the agent when to invoke
---
```

Examples: `skills/brainstorming/SKILL.md:1-4`; `skills/test-driven-development/SKILL.md:1-4`; `skills/verification-before-completion/SKILL.md:1-4` (description unusually long; uses it as a full trigger prompt).

The `description` field is the entire trigger criterion — the agent reads descriptions across all available skills and self-selects.

### Round 1: CSO Rule and Description Budget

Pass 2 round 2 surfaced the **Claude Search Optimization (CSO)** rule for description fields:

- **Hard ceiling**: 1024 characters (Claude Code's documented limit).
- **Semantic rule**: description must answer **WHEN to use** the skill, not WHAT it does. "Use when receiving code review feedback, before implementing suggestions, especially if feedback seems unclear..." (`skills/receiving-code-review/SKILL.md:3`) is correct. "A skill for handling code reviews" would be wrong — it describes the skill instead of the trigger.
- Good descriptions front-load the trigger context, then enumerate edge-case triggers, then state the value.
- The description is compiled into the agent's dispatch table at session start; a vague description = an unreachable skill.

## Naming Conventions

- Skills: kebab-case, action-gerund ("brainstorming", "writing-plans", "using-git-worktrees"). Pattern: "verb-ing + object" so the description reads naturally.
- Agents: kebab-case noun ("code-reviewer").
- Directories mirror names exactly.

## Skill Document Structure (empirical pattern across all skills)

1. Frontmatter (name + description)
2. Optional `<SUBAGENT-STOP>` / `<EXTREMELY-IMPORTANT>` / `<HARD-GATE>` tags
3. `# Title`
4. `## Overview` — 1-3 sentences
5. `**Core principle:** <one-liner>` (Flexible skills) or `## The Iron Law` (Rigid skills)
6. `**Violating the letter of the rules is violating the spirit of the rules.**` line (Iron Law skills)
7. `**Announce at start:** "I'm using the <skill> skill to <purpose>."` — user-visible audit trail convention. Round 1: **not optional** for behavior-shaping skills; the announcement creates the evidence trail used by skill-triggering tests.
8. `## When to Use` / `## Checklist` / `## The Process` — process flow rendered as inline Graphviz `dot` code block. Never Mermaid.
9. `## Red Flags` — rationalization table, positioned AFTER the main process so the agent has been told the rule before being pre-inoculated against excuses. Load-bearing ordering.
10. Examples / anti-patterns / references

## Tag Convention (round 1)

The repo uses a small set of XML-style tags embedded in skill prose as behavioral markers. Parsed by the agent (not tooling), effect depends on the agent honoring the contract:

| Tag | Semantics | Enforcement |
|---|---|---|
| `<EXTREMELY-IMPORTANT>` | Agent must treat contents as non-optional, read-first. Used for Iron Laws and safety rails. | Trust-based |
| `<HARD-GATE>` | Agent forbidden from proceeding past the gate until an explicit approval condition is met. | Trust-based |
| `<SUBAGENT-STOP>` | Signals a dispatched subagent to terminate recursive skill loading at this boundary. | Trust-based |

All three are prose conventions, not machine-enforced. They encode a "no linter, strong prompt" philosophy. Authors of new skills should only introduce these tags when the behavior being gated is Iron Law class.

## Iron Law Pattern

A single all-caps blockquote, always the same form. Reserved for Rigid skills. Canonical form (round 1):

```
> NO <verb> <scope> WITHOUT <prerequisite> FIRST
```

Instances:

- "NO PRODUCTION CODE WITHOUT A FAILING TEST FIRST" (TDD)
- "NO FIXES WITHOUT ROOT CAUSE INVESTIGATION FIRST" (debugging)
- "NO COMPLETION CLAIMS WITHOUT FRESH VERIFICATION EVIDENCE" (verification)

Rigid skills use Iron Laws; Flexible skills use `**Core principle:** <one-liner>`. Mixing the two in a single skill is an authoring smell.

## Skill-Type Categorization (round 1)

Skills partition into two dimensions:

**Axis 1: Rigidity**

- **Rigid**: Follow exactly. Do not adapt. Has an Iron Law. Examples: test-driven-development, systematic-debugging, verification-before-completion, brainstorming (HARD-GATE form), writing-skills.
- **Flexible**: Adapt to context. Has a Core principle instead of an Iron Law.

**Axis 2: Function**

- **Discipline** — shapes behavior (TDD, debugging, verification, receiving-code-review). Red Flags tables mandatory.
- **Technique** — how to do a thing (using-git-worktrees, writing-plans). Process-oriented.
- **Pattern** — recurring structure (dispatching-parallel-agents, SDD). Example-heavy.
- **Reference** — lookup material (tool-name maps, anti-pattern catalogs). Minimal prose.

The axes compose: TDD = Rigid-Discipline, writing-plans = Flexible-Technique, dispatching-parallel-agents = Flexible-Pattern, `copilot-tools.md` = Flexible-Reference. The category determines voice, structure, and whether an Iron Law is appropriate.

## Persuasion Principles Matrix (round 1)

Pass 2 round 2 extracted a Persuasion Principles matrix that governs which social-influence levers a skill author may pull when writing behavior-shaping prose. Six Cialdini-style principles evaluated; five permitted, one forbidden.

| Principle | Permitted | Rationale |
|---|---|---|
| Authority | Yes | Cite empirical evidence, pressure-test results, maintainer decisions |
| Commitment | Yes | Announce-at-start, checklists, explicit gates create commitment |
| Scarcity | Yes | "This is your only chance to run RED correctly" framing |
| Social Proof | Yes | "Every successful TDD session does X" |
| Unity | Yes | "Your human partner" framing creates in-group identity |
| Liking | **No** | Forbidden. Performative agreement ("You're absolutely right!") is explicitly banned. The agent must not use liking-as-persuasion to influence the human; doing so correlates with skipped verification. |

Skill authors are required to audit new prose against this matrix. A skill that relies on Liking is treated as a correctness bug.

## Voice Conventions

- **"Your human partner"** — not "the user". Project-wide, reinforced in `CLAUDE.md:77`. Round 1: Unity principle in the Persuasion matrix. Replacing it in a PR is automatic rejection.
- Imperative mood, short paragraphs, frequent ALL-CAPS for absolute rules.
- Direct confrontational tone toward the agent's own tendencies ("Thinking 'skip TDD just this once'? Stop. That's rationalization." `skills/test-driven-development/SKILL.md:29`).
- **Forbidden Gratitude class** (round 1): phrases "You're absolutely right!", "Great point!", "Excellent feedback!", "Let me implement that now" (pre-verification) explicitly banned (`skills/receiving-code-review/SKILL.md:29-32`). Authors of new skills must not add or encourage these phrases. Linguistic constraint on the repo's voice, not just agent runtime output.
- **Circle K codephrase** (round 1): reserved token the human partner can use to signal the agent is sliding into performative compliance. Skill authors do not invent new codephrases; this is the canonical out-of-band reset.

## Plan Document Conventions (round 1)

Plan files live at `docs/superpowers/plans/YYYY-MM-DD-<feature>.md` and have a fixed structure:

Header schema:

```
# <Feature Title>

## Problem
<what is broken / what needs to exist>

## Approach
<high-level path>

## Files
<path:line-range> — <change description>
...
```

Rules:

- **No-Placeholders** forbidden-token set: `TODO`, `TBD`, `XXX`, `<fill in>`, `???`, bare ellipsis-as-content. A plan containing any of them is incomplete and must not be executed. Plan-layer analog of the PR template no-placeholders rule.
- Files block uses `path:line` or `path:line-range` format, never bare paths. Line ranges force the author to have actually read the file.
- One plan per feature; plans are dated and immutable once execution starts (any change requires a new plan).

## Graphviz DOT for Flow Diagrams

Every process flow rendered as inline `dot` code (not Mermaid). Choice driven by DOT's precise shape vocabulary (doublecircle for start/end, diamond for decisions, box for actions). Round 1: a new skill that uses Mermaid instead of DOT is a convention violation and should be called out in review.

## Red Flags Table Pattern

| Thought | Reality |
|---|---|
| rationalization | rebuttal |

12+ entries typical. Positioned after the main process. This is **behavioral TDD** per `writing-skills`: enumerate the failure modes observed during pressure testing, plug each one. Round 1: Red Flags tables are the tuned output of the Pressure Testing protocol and are explicitly protected by the CLAUDE.md eval-evidence rule — modifying them without before/after evidence is grounds for PR rejection.

## Skill-Priority Rules

`skills/using-superpowers/SKILL.md:98-105`:

1. Process skills first (brainstorming, debugging) — determine HOW
2. Implementation skills second — guide execution

> "Let's build X" → brainstorming first, then impl skills
> "Fix this bug" → debugging first, then domain skills

## Trigger Phrase Testing Convention

`tests/skill-triggering/prompts/<skill-name>.txt` holds the implicit phrase that should trigger the skill. `tests/explicit-skill-requests/prompts/` holds explicit phrasings. Running the test = asking an agent the prompt and checking whether it invoked the right skill (round 1: via grep over Claude Code stream-json output in `tests/skill-triggering/run-test.sh`).

## Cross-Platform Tool-Mapping Convention (round 1)

`skills/using-superpowers/references/copilot-tools.md` and `codex-tools.md` encode a portability convention: for every non-Claude platform, the references directory must contain a tool-name translation table (Claude tool name → platform-equivalent tool or workaround). New skills that reference tools by Claude-specific names must either be platform-agnostic in the main body or provide an override table in `references/`. A missing mapping is a portability bug.

## Deprecation-Shim Response Template (round 1, NEW)

The repo has three deprecation shims (legacy skill entry points preserved for backward compatibility) that share an identical response structure. Formalized as a template rule:

```
---
name: <legacy-name>
description: <redirects to replacement>
---

# <Legacy Name> (deprecated)

This skill has been replaced by `<replacement-skill>`. Use that instead.

<1-2 sentence migration note>
```

Convention: a shim must (a) preserve the legacy `name` exactly so existing dispatch tables still resolve, (b) point to exactly one replacement skill, (c) contain no process content of its own — any process content would bifurcate the source of truth. Authors adding a new deprecation shim MUST follow this template verbatim.

## Output Conventions

- Design docs: `docs/superpowers/specs/YYYY-MM-DD-<topic>-design.md`
- Plans: `docs/superpowers/plans/YYYY-MM-DD-<feature>.md`
- Date-prefixed filename is the only organizational structure.

## Anti-Patterns (explicitly called out)

- `skills/test-driven-development/testing-anti-patterns.md` — reference doc.
- "This is just a simple question" (using-superpowers Red Flag).
- "This Is Too Simple To Need A Design" (`skills/brainstorming/SKILL.md:16`).
- (round 1) **"Compliance-with-Anthropic-skills-docs" restructuring** — explicit authoring anti-pattern per `CLAUDE.md:37`. Reformatting skills to match Anthropic's published skill-writing guidance is a rejection-class change unless eval evidence is provided.
- (round 1) **Liking-as-persuasion** (Persuasion matrix) — using flattery or warmth to shape agent behavior.
- (round 1) **Mermaid flow diagrams** where DOT is expected.
- (round 1) **"the user"** instead of "your human partner".

## Delta Summary

- New items: CSO description rule, Iron Law canonical form, Tag Convention table, Skill-Type Categorization (2-axis), Persuasion Principles Matrix (6 entries with Liking forbidden), Forbidden Gratitude voice class, Circle K codephrase, Plan Document schema + No-Placeholders forbidden tokens, Cross-Platform Tool-Mapping convention, Deprecation-Shim template, 4 new anti-patterns.
- Refined: SKILL.md structure (Announce-at-start as mandatory, Iron Law slot), Red Flags Table (tied to eval-evidence protection), Graphviz DOT (violation framing), Trigger Phrase Testing (stream-json harness citation).
