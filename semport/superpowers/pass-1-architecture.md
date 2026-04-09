# Pass 1: Architecture

## The Injection Mechanism

Superpowers has no traditional "runtime" — it is a bundle of markdown + one bash hook. Architecture is:

1. Platform loads plugin → fires SessionStart event
2. `hooks/session-start` reads `skills/using-superpowers/SKILL.md` verbatim (`hooks/session-start:18-19`)
3. Script wraps content in `<EXTREMELY_IMPORTANT>You have superpowers.\n...</EXTREMELY_IMPORTANT>` and JSON-escapes (`hooks/session-start:38`)
4. Emits JSON with the correct field for the detected platform (`CURSOR_PLUGIN_ROOT` → `additional_context`, `CLAUDE_PLUGIN_ROOT` → `hookSpecificOutput.additionalContext`, else top-level `additionalContext` for Copilot CLI/SDK-standard, `hooks/session-start:49-67`)
5. Platform concatenates the injected context into the session's system prompt
6. Agent is now primed with the "1% rule" and the full catalog of how to access skills

## Skill Composition Model

Skills are **model-invoked units**, not code:
- Each skill is `skills/<name>/SKILL.md` + optional supporting files
- Frontmatter: `name`, `description` (acts as trigger criterion for model self-selection)
- Loaded via the Skill tool in Claude Code / Copilot CLI, `activate_skill` in Gemini (`skills/using-superpowers/SKILL.md:30-36`)
- Skills explicitly forbid using Read on skill files — must go through Skill tool so the platform tracks invocation (`skills/using-superpowers/SKILL.md:30`)

## Discovery Mechanism

1. `using-superpowers` (the bootstrap) lists/describes the skill system and the 1% rule
2. Platforms auto-discover `skills/*/SKILL.md` files and surface descriptions to the agent
3. Agent reads descriptions, decides whether a skill could apply, invokes the Skill tool by name
4. Skill content is loaded into context on demand (not all at once)

This is **lazy loading by semantic matching** — the agent itself is the dispatcher.

## The Brainstorm → Plan → Execute Pipeline

Documented in `README.md:108-124` and reinforced skill-to-skill via mandatory sub-skill invocations:

```
brainstorming  (skills/brainstorming/SKILL.md)
  HARD-GATE: no impl until design approved (line 12-14)
  Output: docs/superpowers/specs/YYYY-MM-DD-<topic>-design.md
  -> invokes writing-plans
using-git-worktrees
  Create isolated worktree, verify clean test baseline
writing-plans  (skills/writing-plans/SKILL.md)
  Bite-sized 2-5 min tasks, each with file paths, code, tests
  Output: docs/superpowers/plans/YYYY-MM-DD-<feature>.md
subagent-driven-development  (skills/subagent-driven-development/SKILL.md)
  Per task: dispatch implementer subagent -> spec reviewer subagent -> code quality reviewer subagent
  Two-stage review (line 8, 37-38)
  Alternative: executing-plans (single session, no subagents)
test-driven-development
  Injected throughout; RED-GREEN-REFACTOR; "Iron Law: no prod code without failing test first" (line 33-34)
requesting-code-review -> dispatches agents/code-reviewer.md
verification-before-completion
  Gate function before any completion claim (line 24-38)
finishing-a-development-branch
  Verify tests -> present merge/PR/keep/discard options -> clean worktree
```

## Skill Chaining

Skills chain via prose directives ("REQUIRED SUB-SKILL: Use superpowers:finishing-a-development-branch", `skills/executing-plans/SKILL.md:36`; writing-skills requires background in test-driven-development, `skills/writing-skills/SKILL.md:18`). Chaining is declarative-in-markdown, not code; enforcement depends on the agent actually following instructions it reads. The 1% rule + Red Flags tables + Iron Laws are the compliance mechanism.

## Subagent Architecture

`subagent-driven-development` uses **three distinct subagent prompts** per task (referenced as `./implementer-prompt.md`, `./spec-reviewer-prompt.md`, `./code-quality-reviewer-prompt.md`, `skills/subagent-driven-development/SKILL.md:47-55`). Fresh context per subagent is the key invariant: "They should never inherit your session's context or history — you construct exactly what they need" (`skills/subagent-driven-development/SKILL.md:10-11`). This mirrors Corverax adversarial review's fresh-context principle but applies it to *every task*, not just review.

## Deployment Topology

Zero-dependency, zero-runtime markdown+bash package. Runs wherever the host CLI supports plugin hooks. No services, no state, no build step. The `.opencode/plugins/superpowers.js` is a thin adapter for OpenCode's JS plugin API.
