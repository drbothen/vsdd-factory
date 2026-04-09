# Pass 0: Inventory

## Tech / Packaging
- `package.json:1-6`: name=superpowers, version=5.0.7, type=module, main=.opencode/plugins/superpowers.js
- `.claude-plugin/plugin.json`: Claude Code plugin manifest
- `.claude-plugin/marketplace.json`: dev marketplace descriptor
- `gemini-extension.json`: Gemini CLI extension
- `.codex/INSTALL.md`, `.opencode/INSTALL.md`, `.cursor-plugin/plugin.json`: per-platform installers
- `AGENTS.md -> CLAUDE.md` (symlink); `GEMINI.md` imports using-superpowers SKILL.md via `@./` syntax (`GEMINI.md:1-2`)
- `CLAUDE.md` is **contributor guidelines**, not runtime instructions (`CLAUDE.md:1-86`); runtime injection happens through the SessionStart hook instead

## Total LOC
- skills + agents + commands + hooks: 8630 lines total
- 14 SKILL.md files: 3159 lines total

## Skills Inventory (14)
| Skill | LOC | Description (from frontmatter) |
|---|---|---|
| using-superpowers | 117 | Starting any conversation; requires Skill tool before ANY response (`skills/using-superpowers/SKILL.md:2-4`) |
| brainstorming | 164 | Before creative work; explore intent/design before impl (`skills/brainstorming/SKILL.md:2-4`) |
| writing-plans | 152 | With spec, before code; bite-sized tasks (`skills/writing-plans/SKILL.md:2-4`) |
| executing-plans | 70 | Execute written plan in separate session (`skills/executing-plans/SKILL.md:2-4`) |
| subagent-driven-development | 277 | Dispatch fresh subagent per task + two-stage review (`skills/subagent-driven-development/SKILL.md:2-4`) |
| test-driven-development | 371 | Every feature/bugfix; RED-GREEN-REFACTOR "Iron Law" (`skills/test-driven-development/SKILL.md:2-4,33-34`) |
| systematic-debugging | 296 | 4-phase root cause; no fix w/o root cause (`skills/systematic-debugging/SKILL.md:2-4,18-19`) |
| verification-before-completion | 139 | Before completion claims; evidence before claims (`skills/verification-before-completion/SKILL.md:2-4,18-19`) |
| using-git-worktrees | 218 | Feature isolation (`skills/using-git-worktrees/SKILL.md:2-4`) |
| finishing-a-development-branch | 200 | Merge/PR/cleanup (`skills/finishing-a-development-branch/SKILL.md:2-4`) |
| dispatching-parallel-agents | 182 | 2+ independent tasks in parallel (`skills/dispatching-parallel-agents/SKILL.md:2-4`) |
| requesting-code-review | 105 | Dispatch code-reviewer subagent (`skills/requesting-code-review/SKILL.md:2-4`) |
| receiving-code-review | 213 | Respond to review feedback; verify before implementing (`skills/receiving-code-review/SKILL.md:2-4`) |
| writing-skills | 655 | Create/edit skills via TDD-applied-to-documentation (`skills/writing-skills/SKILL.md:2-4,10-18`) |

Skills also ship supporting reference files, e.g. `skills/test-driven-development/testing-anti-patterns.md`, `skills/systematic-debugging/condition-based-waiting.md`, etc.

## Agents (1)
- `agents/code-reviewer.md` — Senior Code Reviewer subagent dispatched by requesting-code-review; reviews against plan + coding standards, categorizes Critical/Important/Suggestion (`agents/code-reviewer.md:1-40`)

## Commands (3)
All three are **deprecation shims** pointing at the equivalent skill:
- `commands/brainstorm.md` — "Deprecated — use superpowers:brainstorming" (`commands/brainstorm.md:1-6`)
- `commands/write-plan.md` — "Deprecated — use superpowers:writing-plans" (`commands/write-plan.md:1-6`)
- `commands/execute-plan.md` — "Deprecated — use superpowers:executing-plans" (`commands/execute-plan.md:1-6`)

Superpowers has effectively abandoned user-invoked slash commands in favor of model-invoked skills.

## Hooks (4 files, 1 active event)
- `hooks/hooks.json` — single SessionStart event, matcher `startup|clear|compact`, runs `run-hook.cmd session-start` (`hooks/hooks.json:1-14`)
- `hooks/hooks-cursor.json` — Cursor variant
- `hooks/run-hook.cmd` — Windows/POSIX dispatcher (polyglot, see `docs/windows/polyglot-hooks.md`)
- `hooks/session-start` — bash script that reads `skills/using-superpowers/SKILL.md`, JSON-escapes it, wraps it in `<EXTREMELY_IMPORTANT>You have superpowers...</EXTREMELY_IMPORTANT>`, and injects as `additionalContext` / `additional_context` / `hookSpecificOutput.additionalContext` depending on platform env vars (`hooks/session-start:1-70`). This is **the core mechanism** by which superpowers installs its skill-invocation discipline into every new session.

## Tests
- `tests/claude-code/` — bash tests for skill behavior, including subagent-driven-development integration
- `tests/explicit-skill-requests/` — prompts verifying skills trigger on explicit user phrases
- `tests/skill-triggering/` — prompts verifying skills auto-trigger on implicit phrases
- `tests/subagent-driven-dev/` — end-to-end scaffolded scenarios (go-fractals, svelte-todo)
- `tests/opencode/`, `tests/brainstorm-server/` — platform and component tests

## Multi-platform Artifacts
- Claude Code: `.claude-plugin/`, `hooks/hooks.json`
- Cursor: `.cursor-plugin/plugin.json`, `hooks/hooks-cursor.json`
- Codex: `.codex/INSTALL.md`, `docs/README.codex.md`
- OpenCode: `.opencode/plugins/superpowers.js` (JS plugin), `docs/README.opencode.md`
- Gemini: `gemini-extension.json`, `GEMINI.md`
- Copilot CLI: referenced in using-superpowers (tool mapping doc)
