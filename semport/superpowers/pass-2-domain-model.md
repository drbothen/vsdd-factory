# Pass 2: Domain Model

## Core Concepts

### Skill (model-invoked)

A self-contained markdown file instructing the agent how to perform a specific kind of work. Defined by YAML frontmatter (`name`, `description`) and loaded via Skill tool. Skills are **code that shapes agent behavior** (`CLAUDE.md:69`), not prose. Categorized as **Rigid** (TDD, debugging — follow exactly) or **Flexible** (patterns — adapt principles to context) (`skills/using-superpowers/SKILL.md:107-113`). Types: **Technique**, **Pattern**, **Reference** (`skills/writing-skills/SKILL.md:62-70`).

### Command (user-invoked)

Slash-command entry point. In superpowers, all three commands are deprecation shims; the project has moved all functionality to skills. Intent: commands are for users who know exactly what they want; skills are for the agent to discover itself.

### Hook

Platform lifecycle callback. Superpowers uses only SessionStart (`hooks/hooks.json:4`), and only to inject the bootstrap skill. Hooks are the sole "active code" in the plugin.

### Subagent

A fresh, context-isolated Claude invocation dispatched by the primary agent with a precisely crafted prompt. Never inherits parent context. Used for: implementation tasks, spec compliance review, code quality review, parallel investigation, code review (`skills/dispatching-parallel-agents/SKILL.md:10-11`; `skills/subagent-driven-development/SKILL.md:10-11`).

### Subagent-Driven Development

Workflow pattern where every task in a plan is delegated to a fresh subagent and verified by two more subagents (spec reviewer, then code quality reviewer) before marking complete (`skills/subagent-driven-development/SKILL.md:6-13`). The primary agent orchestrates but does not implement — preserves context for coordination.

### TDD Enforcement ("Iron Law")

`NO PRODUCTION CODE WITHOUT A FAILING TEST FIRST` (`skills/test-driven-development/SKILL.md:33-34`). Violation remedy: delete the code, start over. Explicitly forbids keeping pre-test code "as reference" or "adapting" it (lines 39-45). Exceptions (throwaway prototypes, generated code, config) require asking the human partner first (lines 22-27).

### Red-Flag Detection

A pattern where every critical skill includes a table of **rationalizations** the agent might use to skip the skill, paired with rebuttals. Example from using-superpowers (`skills/using-superpowers/SKILL.md:80-96`):

| Thought | Reality |
|---------|---------|
| "This is just a simple question" | Questions are tasks. Check for skills. |
| "The skill is overkill" | Simple things become complex. Use it. |
| "I know what that means" | Knowing the concept != using the skill. Invoke it. |

This is behavioral inoculation against the agent's own convergent failure modes.

### Verification-Before-Completion

A **gate function** mandating that no completion claim is made without running a fresh verification command in the current message. "Skip any step = lying, not verifying" (`skills/verification-before-completion/SKILL.md:26-38`). Encodes the same principle as Corverax's "silent failures are the enemy" (SOUL.md #4) but enforces it at the completion-claim boundary.

### Systematic Debugging

Four-phase process with an Iron Law: `NO FIXES WITHOUT ROOT CAUSE INVESTIGATION FIRST` (`skills/systematic-debugging/SKILL.md:18-19`). Phase 1 must complete before proposing any fix (line 21). Explicitly calls out time-pressure rationalizations (lines 34-38).

### HARD-GATE

A markdown tag used in brainstorming (`<HARD-GATE>`, `skills/brainstorming/SKILL.md:12-14`) to mark non-negotiable blocking conditions. Not machine-enforced — relies on the agent parsing the tag and self-gating.

### EXTREMELY-IMPORTANT / SUBAGENT-STOP

Tag-based directives: `<EXTREMELY-IMPORTANT>` for absolute rules (`skills/using-superpowers/SKILL.md:10-16`); `<SUBAGENT-STOP>` to exempt dispatched subagents from the bootstrap (`skills/using-superpowers/SKILL.md:6-8`) — otherwise subagents would recursively try to load using-superpowers.

## Ubiquitous Language Glossary

- **Your human partner** — deliberate term for the user; carries collaborative/protective framing (`CLAUDE.md:9`: "protect your human partner from that outcome"). NOT interchangeable with "the user" per project voice.
- **Skill** — behavior-shaping reference document.
- **Iron Law** — non-negotiable rule within a skill.
- **Red Flag** — a rationalization the agent must catch and reject.
- **Pressure testing** — running adversarial scenarios against a skill to verify it holds (`skills/writing-skills/SKILL.md:14-17`).
- **RED/GREEN/REFACTOR** — TDD cycle states.
- **Fresh subagent** — context-isolated delegate.
- **Spec reviewer / code quality reviewer** — the two stages of post-implementation review.

## State Machines

- **Brainstorming workflow**: explore → clarify → propose → design-sections → user-approves → write-spec → self-review → user-reviews → transition-to-writing-plans (`skills/brainstorming/SKILL.md:22-32`)
- **TDD cycle**: red → verify_red → green → verify_green → refactor → verify_green → next → red (`skills/test-driven-development/SKILL.md:49-68`)
- **SDD per-task loop**: dispatch-implementer → implementer-done → dispatch-spec-reviewer → {fix-spec or} dispatch-quality-reviewer → {fix-quality or} mark-complete (`skills/subagent-driven-development/SKILL.md:42-59`)
- **Debugging**: 4 phases, must complete sequentially (`skills/systematic-debugging/SKILL.md:46-50`)

## No Persistent Domain Entities

Superpowers has no runtime data model, no state files, no database. The "domain" is conversational behavior. Artifacts it produces:

- `docs/superpowers/specs/YYYY-MM-DD-<topic>-design.md` (brainstorming output)
- `docs/superpowers/plans/YYYY-MM-DD-<feature>.md` (writing-plans output)

These are outputs of the workflow, not managed domain entities.
