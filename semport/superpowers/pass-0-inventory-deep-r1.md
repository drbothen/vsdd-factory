# Pass 0 Deepening Round 1: Inventory Completion

Builds on `pass-0-inventory.md` (Phase A). Phase A content preserved; this round adds enumerations Phase A skipped.

## LOC Re-verification (confirms Phase A)

- `skills/`: 8438 lines (all files, incl. SKILL.md + supporting)
- `agents/`: 48 lines (1 file)
- `commands/`: 15 lines (3 deprecation shims)
- `hooks/`: 129 lines (4 files)
- Total: 8630 — matches Phase A exactly.
- `.opencode/plugins/superpowers.js`: 112 LOC (not previously measured; not counted in the 8630 total).

## Skill Supporting Files (31 files, enumerated)

Phase A said "Skills also ship supporting reference files, e.g. ...". Complete list:

### brainstorming (6 supporting files)
- `skills/brainstorming/scripts/frame-template.html`
- `skills/brainstorming/scripts/helper.js`
- `skills/brainstorming/scripts/server.cjs` — zero-dep brainstorm WebSocket server
- `skills/brainstorming/scripts/start-server.sh`
- `skills/brainstorming/scripts/stop-server.sh`
- `skills/brainstorming/spec-document-reviewer-prompt.md` — subagent prompt template for spec doc review
- `skills/brainstorming/visual-companion.md`

### writing-plans (1)
- `skills/writing-plans/plan-document-reviewer-prompt.md` — subagent prompt template for plan review (confirms pass 3 round 3 finding of inline Self-Review)

### subagent-driven-development (3 — the three prompt templates)
- `skills/subagent-driven-development/implementer-prompt.md`
- `skills/subagent-driven-development/spec-reviewer-prompt.md`
- `skills/subagent-driven-development/code-quality-reviewer-prompt.md`

### test-driven-development (1)
- `skills/test-driven-development/testing-anti-patterns.md`

### systematic-debugging (8)
- `skills/systematic-debugging/condition-based-waiting.md`
- `skills/systematic-debugging/condition-based-waiting-example.ts`
- `skills/systematic-debugging/CREATION-LOG.md`
- `skills/systematic-debugging/defense-in-depth.md`
- `skills/systematic-debugging/find-polluter.sh` — operational script (bash)
- `skills/systematic-debugging/root-cause-tracing.md`
- `skills/systematic-debugging/test-academic.md`
- `skills/systematic-debugging/test-pressure-1.md`, `test-pressure-2.md`, `test-pressure-3.md` (3 files) — Pressure Testing protocol fixtures (confirms pass 2/3 discovery of 3+ pressure types)

### requesting-code-review (1)
- `skills/requesting-code-review/code-reviewer.md` — local copy of the code-reviewer prompt (distinct from top-level `agents/code-reviewer.md`, see Agent note)

### using-superpowers (3 — platform tool references)
- `skills/using-superpowers/references/codex-tools.md`
- `skills/using-superpowers/references/copilot-tools.md`
- `skills/using-superpowers/references/gemini-tools.md`

### writing-skills (6)
- `skills/writing-skills/anthropic-best-practices.md`
- `skills/writing-skills/persuasion-principles.md` — Persuasion Principles matrix (confirms pass 2/3 discovery)
- `skills/writing-skills/testing-skills-with-subagents.md` — empirical testing-with-subagents methodology
- `skills/writing-skills/examples/CLAUDE_MD_TESTING.md`
- `skills/writing-skills/graphviz-conventions.dot`
- `skills/writing-skills/render-graphs.js`

### Skills with NO supporting files
- executing-plans, verification-before-completion, using-git-worktrees, finishing-a-development-branch, dispatching-parallel-agents, receiving-code-review — SKILL.md only

## Agent Correction

Phase A listed 1 agent correctly but conflated mental model. Clarification:

- `agents/code-reviewer.md` (48 LOC) — the top-level **Agent** (Senior Code Reviewer) dispatched by `requesting-code-review`.
- `skills/requesting-code-review/code-reviewer.md` — a **local copy** of the reviewer prompt inside the skill directory. Not a separate agent; it is the prompt text the skill sends. The top-level file is the canonical agent registration.
- Subagent prompt templates in `skills/subagent-driven-development/*.md` are **NOT agents** — they are prompt bodies the SDD skill injects into ad-hoc Task tool calls with fresh context.

Total true Agents: 1 (`agents/code-reviewer.md`).

## SDD Fixtures (triple format confirmed)

`tests/subagent-driven-dev/`:

- `run-test.sh` — runner script
- `go-fractals/`
  - `design.md` (81 lines) — Go ASCII fractal generator design
  - `plan.md` (172 lines) — bite-sized task plan
  - `scaffold.sh` (45 lines) — fixture bootstrap
- `svelte-todo/`
  - `design.md` (70 lines)
  - `plan.md` (222 lines)
  - `scaffold.sh` (46 lines)

Both fixtures conform to the triple format (design + plan + scaffold) pass 3 round 3 described.

## tests/brainstorm-server/ (5 files)

- `package.json`, `package-lock.json` — npm test harness
- `server.test.js` — JS unit tests for brainstorm server
- `ws-protocol.test.js` — WebSocket protocol tests
- `windows-lifecycle.test.sh` — bash script for Windows start/stop lifecycle

## .opencode/plugins/superpowers.js (112 LOC)

JS adapter for OpenCode's plugin API. Responsibilities (confirmed via pass 2 round 2): first-user-message context injection, hook path registration. Measured but NOT included in the 8630-line total (JS, not markdown/bash).

## Multi-platform Files (complete enumeration)

- `.claude-plugin/plugin.json`, `.claude-plugin/marketplace.json`
- `.codex/INSTALL.md`
- `.opencode/INSTALL.md`, `.opencode/plugins/superpowers.js`
- `.cursor-plugin/plugin.json`
- Root: `gemini-extension.json`, `GEMINI.md`, `AGENTS.md` (symlink → `CLAUDE.md`)

## docs/ Enumeration (16 files)

- `docs/README.codex.md`, `docs/README.opencode.md` — per-platform READMEs
- `docs/testing.md` — test methodology doc
- `docs/windows/polyglot-hooks.md` — polyglot hook mechanism
- `docs/plans/` (4 historical plans): opencode-support-design, opencode-support-implementation, skills-improvements-from-user-feedback, visual-brainstorming
- `docs/superpowers/plans/` (4): document-review-system, visual-brainstorming-refactor, zero-dep-brainstorm-server, codex-app-compatibility
- `docs/superpowers/specs/` (4): matching specs for each of the above plans

**Observation**: `docs/superpowers/{plans,specs}/` is the **canonical output location** for the brainstorm → writing-plans pipeline. Superpowers dogfoods its own pipeline.

## scripts/ (1 file)

- `scripts/bump-version.sh` — version bump operational script

## Tests Layout (complete, 2-level)

- `tests/brainstorm-server/` (5 files, above)
- `tests/claude-code/` (7): analyze-token-usage.py, README.md, run-skill-tests.sh, test-document-review-system.sh, test-helpers.sh, test-subagent-driven-development.sh, test-subagent-driven-development-integration.sh
- `tests/explicit-skill-requests/` (6): run-all.sh, run-test.sh, run-claude-describes-sdd.sh, run-extended-multiturn-test.sh, run-haiku-test.sh, run-multiturn-test.sh
- `tests/opencode/` (5): run-tests.sh, setup.sh, test-plugin-loading.sh, test-priority.sh, test-tools.sh
- `tests/skill-triggering/` (2): run-all.sh, run-test.sh
- `tests/subagent-driven-dev/` (7, above)

## Delta Summary

- New items: 31 skill supporting files, 16 docs files, 5 brainstorm-server test files, 1 scripts file, opencode plugin measured (112 LOC), full multi-platform file list, full tests layout, agent vs prompt-template disambiguation.
- Refinements: LOC totals re-verified identical to Phase A. Agent count confirmed at 1. Skill count confirmed at 14.
- Remaining gaps: Supporting-file LOC totals not individually measured (round 2 candidate if needed).
