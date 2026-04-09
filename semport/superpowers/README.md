# Superpowers Brownfield Ingest — Phase A (Broad Sweep)

Source: `/Users/jmagady/Dev/vsdd-factory/.reference/superpowers` @ `917e5f53`. Multi-platform skill ecosystem plugin (Claude Code, Cursor, Codex, OpenCode, Gemini, Copilot CLI) at v5.0.7. 14 skills, 1 agent, 3 commands (all deprecated shims), 1 active hook (SessionStart), ~8.6k LOC of behavior-shaping markdown + shell. Core thesis: a SessionStart hook injects the `using-superpowers` skill directly into context with an EXTREMELY_IMPORTANT wrapper instructing the agent that "if there's even a 1% chance a skill might apply, you MUST invoke it." Skills are model-invoked (not user-invoked) via the Skill tool, each a self-contained SKILL.md with YAML frontmatter (`name`, `description`) and aggressive anti-rationalization content (Red Flags tables, Iron Laws, HARD-GATE blocks). The workflow is brainstorm → writing-plans → subagent-driven-development (fresh subagent per task with two-stage review: spec compliance then code quality) → verification-before-completion → finishing-a-development-branch. Philosophy is "skills-as-code-shaping-agent-behavior," tested adversarially, explicitly rejecting Anthropic's own skill-authoring guidance in favor of empirically tuned content.

## Files
- pass-0-inventory.md — file tree, LOC, per-artifact inventory
- pass-1-architecture.md — skill composition, discovery, workflow pipeline
- pass-2-domain-model.md — core concepts and vocabulary
- pass-3-behavioral-contracts.md — contracts encoded in skills + tests
- pass-4-nfrs.md — quality guarantees and guardrails
- pass-5-conventions.md — SKILL.md schema, patterns, red-flag tables
- pass-6-synthesis.md — cross-reference + Superpowers vs vsdd-factory comparison
