# Pass 5: Conventions & Patterns

_Phase B convergence round 2 — **CONVERGED**. Round 2 was predominantly corrective: Persuasion matrix fabrication fixed (6 → 7 principles), Plan No-Placeholders token list narrowed from round 1 over-extrapolation, deprecation shim locations identified in commands/ not skills/, CSO rule verbatim citations added._

## SKILL.md Frontmatter Schema

```
---
name: <kebab-case-skill-name>            # matches directory name
description: <trigger criterion string>  # tells the agent when to invoke
---
```

Examples: `skills/brainstorming/SKILL.md:1-4`; `skills/test-driven-development/SKILL.md:1-4`; `skills/verification-before-completion/SKILL.md:1-4`.

The `description` field is the entire trigger criterion — the agent reads descriptions across all available skills and self-selects.

### CSO Rule and Description Budget (round 2 verbatim citations)

- **Hard ceiling**: 1024 characters (Claude Code documented limit); confirmed at `skills/writing-skills/SKILL.md:607` — "max 1024 chars".
- **Semantic rule (round 2 exact citations)**: `skills/writing-skills/SKILL.md:99-100` states verbatim:
  > `description`: Third-person, describes ONLY when to use (NOT what it does)
  >   - Start with "Use when..." to focus on triggering conditions

  And at line 152: "The description should ONLY describe triggering conditions. Do NOT summarize the skill's process or workflow in the description."
- **Empirical rationale (line 154-156, verbatim)**: "Testing revealed that when a description summarizes the skill's workflow, Claude may follow the description instead of reading the full skill content. A description saying 'code review between tasks' caused Claude to do ONE review, even though the skill's flowchart clearly showed TWO reviews [...]. When the description was changed to just 'Use when executing implementation plans with independent tasks' (no workflow summary), Claude correctly read the flowchart and followed the two-stage review process."

  **Canonical case study for WHEN-not-WHAT**: a description-summarizes-workflow bug caused agents to skip the skill body. CSO is not a style preference; it is a **behavioral correctness requirement**.
- **Good vs bad** (`writing-skills/SKILL.md:182-196`):
  - `description: For async testing` (too vague) — BAD
  - `description: I can help you with async tests when they're flaky` (first-person) — BAD
  - `description: Use when tests use setTimeout/sleep and are flaky` — GOOD (symptom-triggered)

## Naming Conventions

- Skills: kebab-case, action-gerund ("brainstorming", "writing-plans", "using-git-worktrees"). Pattern: "verb-ing + object".
- Agents: kebab-case noun ("code-reviewer").
- Directories mirror names exactly.

## Skill Document Structure (empirical pattern)

1. Frontmatter (name + description)
2. Optional `<SUBAGENT-STOP>` / `<EXTREMELY-IMPORTANT>` / `<HARD-GATE>` tags
3. `# Title`
4. `## Overview` — 1-3 sentences
5. `**Core principle:** <one-liner>` (Flexible skills) or `## The Iron Law` (Rigid skills)
6. `**Violating the letter of the rules is violating the spirit of the rules.**` line (Iron Law skills)
7. `**Announce at start:** "I'm using the <skill> skill to <purpose>."` — **not optional** for behavior-shaping skills; creates the evidence trail used by skill-triggering tests.
8. `## When to Use` / `## Checklist` / `## The Process` — process flow as inline Graphviz `dot` code block. Never Mermaid.
9. `## Red Flags` — rationalization table positioned AFTER the main process. Load-bearing ordering.
10. Examples / anti-patterns / references

## Tag Convention

| Tag | Semantics | Enforcement |
|---|---|---|
| `<EXTREMELY-IMPORTANT>` | Agent must treat contents as non-optional, read-first. Used for Iron Laws and safety rails. | Trust-based |
| `<HARD-GATE>` | Agent forbidden from proceeding past the gate until an explicit approval condition is met. | Trust-based |
| `<SUBAGENT-STOP>` | Signals a dispatched subagent to terminate recursive skill loading at this boundary. | Trust-based |

All three are prose conventions, not machine-enforced. Encodes a "no linter, strong prompt" philosophy.

## Iron Law Pattern

A single all-caps blockquote, always the same form. Reserved for Rigid skills. Canonical form:

```
> NO <verb> <scope> WITHOUT <prerequisite> FIRST
```

Instances (round 2 — writing-skills added verbatim):

- "NO PRODUCTION CODE WITHOUT A FAILING TEST FIRST" (TDD)
- "NO FIXES WITHOUT ROOT CAUSE INVESTIGATION FIRST" (debugging)
- "NO COMPLETION CLAIMS WITHOUT FRESH VERIFICATION EVIDENCE" (verification)
- **"NO SKILL WITHOUT A FAILING TEST FIRST"** (writing-skills, `skills/writing-skills/SKILL.md:377`, header is `## The Iron Law (Same as TDD)` at line 374 — the authoring Iron Law explicitly inherits TDD's verb structure by substitution)

Rigid skills use Iron Laws; Flexible skills use `**Core principle:** <one-liner>`.

## Skill-Type Categorization

**Axis 1: Rigidity** — Rigid (Iron Law) vs Flexible (Core principle).
**Axis 2: Function** — Discipline / Technique / Pattern / Reference.

Composition: TDD = Rigid-Discipline, writing-plans = Flexible-Technique, dispatching-parallel-agents = Flexible-Pattern, `copilot-tools.md` = Flexible-Reference.

## Persuasion Principles Matrix (round 2 HALLUCINATION CORRECTED)

**Round 1 documented 6 principles. Actual is SEVEN** (adds Reciprocity). `skills/writing-skills/persuasion-principles.md` grounded in Cialdini 2021 and empirically tested in Meincke et al. 2025 (N=28,000 AI conversations, compliance 33%→72%).

| # | Principle | Permitted? | Source evidence |
|---|---|---|---|
| 1 | Authority | Yes — primary for discipline skills | `persuasion-principles.md:11-28`. "YOU MUST", "Never", "Always", "No exceptions". |
| 2 | Commitment | Yes — announce-at-start, TodoWrite, forced explicit choices | `:30-47`. "When you find a skill, you MUST announce: 'I'm using [Skill Name]'" |
| 3 | **Scarcity** | **Yes — first-class permitted** (round 2 correction: round 1 treated as framing-only) | `:49-66`. "Before proceeding", "Immediately after X", "After completing a task, IMMEDIATELY request code review" |
| 4 | Social Proof | Yes — "Every time", "Always", "X without Y = failure" | `:68-85`. "Checklists without TodoWrite tracking = steps get skipped. Every time." |
| 5 | Unity | Yes — "we're colleagues", "our codebase", "your human partner" | `:87-103`. |
| 6 | **Reciprocity** | **Avoid almost always** (round 2: round 1 missed this entry entirely) — "rarely needed in skills", "can feel manipulative" | `:105-113`. |
| 7 | Liking | **Forbidden for compliance enforcement** — "DON'T USE for compliance", "Conflicts with honest feedback culture", "Creates sycophancy" | `:115-124`. |

**Combination matrix** (`persuasion-principles.md:128-133`, verbatim):

| Skill Type | Use | Avoid |
|---|---|---|
| Discipline-enforcing | Authority + Commitment + Social Proof | Liking, Reciprocity |
| Guidance/technique | Moderate Authority + Unity | Heavy authority |
| Collaborative | Unity + Commitment | Authority, Liking |
| Reference | Clarity only | All persuasion |

**Ethical test** (`:165`, verbatim): "Would this technique serve the user's genuine interests if they fully understood it?"

**Parahuman-model claim** (`:147-151`): "LLMs are parahuman. Trained on human text containing these patterns. Authority language precedes compliance in training data. Commitment sequences (statement → action) frequently modeled. Social proof patterns (everyone does X) establish norms." Theoretical basis for the entire repo's behavior-shaping philosophy.

Skill authors must audit new prose against this matrix. A skill relying on Liking is a **correctness bug**; a skill relying on Reciprocity is an **authoring smell**.

## Voice Conventions

- **"Your human partner"** — not "the user". Project-wide, reinforced in `CLAUDE.md:78`. Unity principle.
- Imperative mood, short paragraphs, frequent ALL-CAPS for absolute rules.
- Direct confrontational tone toward the agent's own tendencies.
- **Forbidden Gratitude class**: "You're absolutely right!", "Great point!", "Excellent feedback!", "Let me implement that now" (pre-verification) explicitly banned (`receiving-code-review/SKILL.md:29-32`).
- **Circle K codephrase**: canonical out-of-band reset token.

## Plan Document Conventions (round 2 CORRECTED)

Plan files live at `docs/superpowers/plans/YYYY-MM-DD-<feature>.md`.

**No-Placeholders forbidden-token set (round 2 verbatim)**: round 1 claimed the set was `TODO`, `TBD`, `XXX`, `<fill in>`, `???`, ellipsis-as-content. **This was partly fabricated.** The actual source at `skills/writing-plans/SKILL.md:106-114` (section header `## No Placeholders`) reads verbatim:

> Every step must contain the actual content an engineer needs. These are **plan failures** - never write them:
> - "TBD", "TODO", "implement later", "fill in details"
> - "Add appropriate error handling" / "add validation" / "handle edge cases"
> - "Write tests for the above" (without actual test code)
> - "Similar to Task N" (repeat the code - the engineer may be reading tasks out of order)
> - Steps that describe what to do without showing how (code blocks required for code steps)
> - References to types, functions, or methods not defined in any task

**Canonical taxonomic term**: such violations are called **"plan failures"** (verbatim). This is a named class in the repo's vocabulary.

Additional rules:
- Files block uses `path:line` or `path:line-range` format, never bare paths.
- One plan per feature; plans are dated.

## Graphviz DOT for Flow Diagrams

Every process flow rendered as inline `dot` code (not Mermaid). A new skill that uses Mermaid instead of DOT is a convention violation.

## Red Flags Table Pattern

| Thought | Reality |
|---|---|
| rationalization | rebuttal |

12+ entries typical. Positioned after the main process. **Behavioral TDD** per writing-skills: enumerate failure modes observed during pressure testing, plug each one. Red Flags tables are protected by the CLAUDE.md eval-evidence rule (`CLAUDE.md:74`) — modifying them without before/after evidence is grounds for PR rejection.

## Skill-Priority Rules

`skills/using-superpowers/SKILL.md:98-105`:

1. Process skills first (brainstorming, debugging) — determine HOW
2. Implementation skills second — guide execution

## Trigger Phrase Testing Convention

`tests/skill-triggering/prompts/<skill-name>.txt` holds the implicit phrase. `tests/explicit-skill-requests/prompts/` holds explicit phrasings. Running the test = grep over Claude Code stream-json output (`tests/skill-triggering/run-test.sh`).

## Cross-Platform Tool-Mapping Convention

`skills/using-superpowers/references/copilot-tools.md` and `codex-tools.md` encode a portability convention: every non-Claude platform gets a tool-name translation table. A missing mapping is a portability bug.

## Deprecation-Shim Response Template (round 2 LOCATED & ENUMERATED)

Round 1 described the Deprecation-Shim template but couldn't find instances in `skills/`. **Round 2 located all three: they live under `commands/` (legacy slash-command entry points), not `skills/`.** That's why round 1 failed — it searched the wrong directory.

| File | Legacy entry point | Replacement skill | Verbatim redirect text |
|---|---|---|---|
| `commands/brainstorm.md:5` | `/brainstorm` | `superpowers brainstorming` | "Tell your human partner that this command is deprecated and will be removed in the next major release. They should ask you to use the 'superpowers brainstorming' skill instead." |
| `commands/write-plan.md:5` | `/write-plan` | `superpowers writing-plans` | "Tell your human partner that this command is deprecated and will be removed in the next major release. They should ask you to use the 'superpowers writing-plans' skill instead." |
| `commands/execute-plan.md:5` | `/execute-plan` | `superpowers executing-plans` | "Tell your human partner that this command is deprecated and will be removed in the next major release. They should ask you to use the 'superpowers executing-plans' skill instead." |

**Template rule (round 2 refined)**: a deprecation shim is a `commands/*.md` file whose body instructs the agent to (a) tell the human partner the command is deprecated, (b) name a specific replacement skill (exactly one), (c) announce removal in "the next major release", and (d) contain no process content of its own. The identical template verbatim across all three files confirms this is a deliberate convention: the shim body is effectively a constant string with the replacement-name as the only variable.

The shim convention is a **command-layer** deprecation mechanism, not a skill-layer one. Adding a new deprecation shim requires creating a `commands/<old-name>.md` file following this template verbatim.

## Output Conventions

- Design docs: `docs/superpowers/specs/YYYY-MM-DD-<topic>-design.md`
- Plans: `docs/superpowers/plans/YYYY-MM-DD-<feature>.md`
- Date-prefixed filename is the only organizational structure.

## Anti-Patterns (round 2 expanded)

- `skills/test-driven-development/testing-anti-patterns.md` — reference doc.
- "This is just a simple question" (using-superpowers Red Flag).
- "This Is Too Simple To Need A Design" (`skills/brainstorming/SKILL.md:16`).
- **"Compliance-with-Anthropic-skills-docs" restructuring** — per `CLAUDE.md:35-37`.
- **Liking-as-persuasion** — see Persuasion matrix.
- **Reciprocity-as-persuasion (round 2 NEW)** — "rarely needed in skills", "can feel manipulative" (`persuasion-principles.md:105-113`). Promoted to explicit anti-pattern for symmetry with Liking.
- **Mermaid flow diagrams** where DOT is expected.
- **"the user"** instead of "your human partner".
- **Description summarizes workflow (round 2 NEW)** — from the CSO case study at `writing-skills/SKILL.md:154-156`: "A description saying 'code review between tasks' caused Claude to do ONE review, even though the skill's flowchart clearly showed TWO reviews". Description-as-workflow-summary is now a named anti-pattern with empirical evidence.
- **First-person descriptions (round 2 NEW)** — `writing-skills/SKILL.md:99, 187` forbids `description: I can help you...` patterns. Descriptions must be third-person.

## Changes from round 1 (convergence corrections)

1. **Persuasion matrix corrected**: 6 principles → 7 (Reciprocity was missing). Scarcity confirmed as first-class permitted (not just framing). Liking forbidden specifically for compliance enforcement, Reciprocity "avoid almost always". Full combination matrix for four skill types added. Meincke 2025 empirical anchor added.
2. **CSO rule verbatim citations added**: `writing-skills/SKILL.md:99-100, 152, 154-156` — the description-summarizes-workflow case study is now quoted as the canonical rationale.
3. **Plan No-Placeholders token list corrected**: actual verbatim list from `writing-plans/SKILL.md:109` is "TBD", "TODO", "implement later", "fill in details" only. "Plan failures" captured as new taxonomic term.
4. **Writing-skills Iron Law exact form**: `NO SKILL WITHOUT A FAILING TEST FIRST` at `writing-skills/SKILL.md:377`, header "The Iron Law (Same as TDD)".
5. **Deprecation shims enumerated**: three instances located in `commands/` (not `skills/`) — brainstorm, write-plan, execute-plan. Verbatim template text captured.
6. **Anti-pattern additions**: Reciprocity-as-persuasion, description-summarizes-workflow, first-person descriptions.
