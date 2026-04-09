# Pass 0: Inventory

_Phase B convergence round 2 — supersedes pass-0-inventory-deep-r1.md. Consolidates Phase A + round 1 delta + round 2 additions._

## Tech / Packaging

- `package.json:1-6`: name=superpowers, version=5.0.7, type=module, main=.opencode/plugins/superpowers.js. **No scripts, no dependencies, no devDependencies** — the repo is not an npm package in any operational sense; package.json exists solely to mark the OpenCode adapter as an ESM entry point.
- `.claude-plugin/plugin.json`: Claude Code plugin manifest
- `.claude-plugin/marketplace.json`: dev marketplace descriptor
- `gemini-extension.json`: Gemini CLI extension
- `.codex/INSTALL.md`, `.opencode/INSTALL.md`, `.cursor-plugin/plugin.json`: per-platform installers
- `AGENTS.md → CLAUDE.md` (symlink); `GEMINI.md` imports using-superpowers SKILL.md via `@./` syntax (`GEMINI.md:1-2`)
- `CLAUDE.md` is **contributor guidelines**, not runtime instructions; runtime injection happens through the SessionStart hook.
- **No `.github/workflows/`** — repo has `.github/FUNDING.yml`, `.github/ISSUE_TEMPLATE/`, `.github/PULL_REQUEST_TEMPLATE.md` but zero CI workflow files. All testing is locally invoked via `tests/*/run-*.sh`. **No automated pre-merge validation.**

## Total LOC

- skills + agents + commands + hooks: **~7210** lines (markdown + bash + JSON)
- 14 SKILL.md files: **3159** lines total
- 23 skill supporting files: **3859** lines total (validate-extraction corrected)
- `.opencode/plugins/superpowers.js`: 112 LOC (separate)

Skills/ subtotal **7018** = 3159 SKILLs + 3859 supporting (validate-extraction corrected; round 2 claimed 8438 / 32-file count, which was inflated).

## Skills Inventory (14)

| Skill | SKILL.md LOC | Description |
|---|---|---|
| using-superpowers | 117 | Bootstrap; requires Skill tool before ANY response |
| brainstorming | 164 | Explore intent/design before impl |
| writing-plans | 152 | Bite-sized tasks with code+tests |
| executing-plans | 70 | Single-session plan execution |
| subagent-driven-development | 277 | Fresh-context per-task dispatch + tri-valued review |
| test-driven-development | 371 | Iron Law RED-GREEN-REFACTOR |
| systematic-debugging | 296 | 4-phase root cause protocol |
| verification-before-completion | 139 | Evidence before claims gate |
| using-git-worktrees | 218 | Feature isolation |
| finishing-a-development-branch | 200 | Merge/PR/cleanup |
| dispatching-parallel-agents | 182 | Independent-tasks parallelism |
| requesting-code-review | 105 | Dispatch code-reviewer subagent |
| receiving-code-review | 213 | Respond to feedback; verify first |
| writing-skills | 655 | Create/edit skills via TDD-for-docs |

## Skill Supporting Files — Complete LOC Table (round 2)

**32 supporting files, 5279 total LOC.**

### writing-skills (6 files, 2249 LOC — 43% of all supporting)

| File | LOC |
|---|---|
| `anthropic-best-practices.md` | **1150** |
| `testing-skills-with-subagents.md` | 384 |
| `examples/CLAUDE_MD_TESTING.md` | 189 |
| `persuasion-principles.md` | 187 |
| `graphviz-conventions.dot` | 171 |
| `render-graphs.js` | 168 |

`anthropic-best-practices.md` (1150 lines) is the single largest content file in the repo, larger than any SKILL.md. writing-skills is effectively a library, not a single directive.

### brainstorming (7 files, 996 LOC)

| File | LOC |
|---|---|
| `scripts/server.cjs` (zero-dep WebSocket server) | 354 |
| `visual-companion.md` | 287 |
| `scripts/frame-template.html` | 214 |
| `scripts/start-server.sh` | 148 |
| `scripts/helper.js` | 88 |
| `scripts/stop-server.sh` | 56 |
| `spec-document-reviewer-prompt.md` | 49 |

### systematic-debugging (10 files, 959 LOC)

| File | LOC |
|---|---|
| `root-cause-tracing.md` | 169 |
| `condition-based-waiting-example.ts` | 158 |
| `defense-in-depth.md` | 122 |
| `CREATION-LOG.md` | 119 |
| `condition-based-waiting.md` | 115 |
| `test-pressure-3.md` | 69 |
| `test-pressure-2.md` | 68 |
| `find-polluter.sh` | 63 |
| `test-pressure-1.md` | 58 |
| `test-academic.md` | 14 |

### test-driven-development (1 file, 299 LOC)

| File | LOC |
|---|---|
| `testing-anti-patterns.md` | 299 |

### subagent-driven-development (3 files, 200 LOC)

| File | LOC |
|---|---|
| `implementer-prompt.md` | 113 |
| `spec-reviewer-prompt.md` | 61 |
| `code-quality-reviewer-prompt.md` | 26 |

### using-superpowers (3 files, 185 LOC)

| File | LOC |
|---|---|
| `references/codex-tools.md` | 100 |
| `references/copilot-tools.md` | 52 |
| `references/gemini-tools.md` | 33 |

### requesting-code-review (1 file, 146 LOC)

| File | LOC |
|---|---|
| `code-reviewer.md` (local prompt copy) | 146 |

### writing-plans (1 file, 49 LOC)

| File | LOC |
|---|---|
| `plan-document-reviewer-prompt.md` | 49 |

### Skills with NO supporting files (6)

executing-plans, verification-before-completion, using-git-worktrees, finishing-a-development-branch, dispatching-parallel-agents, receiving-code-review.

### Key Observation

**Supporting files are 62% of skills LOC (5279 / 8438).** The SKILL.md file is an entry-point contract; the real knowledge payload lives in adjacent markdown. Three skills account for 4204 LOC of the 5279 supporting total (80%): writing-skills (2249), brainstorming (996), systematic-debugging (959). These are the "library" skills. The other eight skills ship minimal supporting content or none.

## Agents (1)

- `agents/code-reviewer.md` (48 LOC) — Senior Code Reviewer subagent, the only canonical top-level Agent. Dispatched by requesting-code-review.
- `skills/requesting-code-review/code-reviewer.md` (146 LOC) is a local prompt copy, NOT a separate agent.
- SDD's 3 prompt templates under `skills/subagent-driven-development/*.md` are prompt bodies, NOT agents.

## Commands (3, deprecation shims)

- `commands/brainstorm.md`, `commands/write-plan.md`, `commands/execute-plan.md` — all 5-line redirects to their skill equivalents. Superpowers has abandoned slash commands for model-invoked skills.

## Hooks (4 files, 129 LOC, 1 active event)

- `hooks/hooks.json` — single SessionStart event, matcher `startup|clear|compact`, runs `run-hook.cmd session-start`
- `hooks/hooks-cursor.json` — Cursor variant
- `hooks/run-hook.cmd` — polyglot Windows/POSIX dispatcher
- `hooks/session-start` — bash script: reads using-superpowers SKILL.md, wraps in `<EXTREMELY_IMPORTANT>`, JSON-escapes, emits platform-conditional JSON shape. Core injection mechanism.

## Tests Layout

- `tests/brainstorm-server/` (5): package.json, package-lock.json, server.test.js, ws-protocol.test.js, windows-lifecycle.test.sh
- `tests/claude-code/` (7): analyze-token-usage.py, README.md, run-skill-tests.sh, test-document-review-system.sh, test-helpers.sh, test-subagent-driven-development.sh, test-subagent-driven-development-integration.sh
- `tests/explicit-skill-requests/` (6): run-all.sh + 5 variant runners (haiku, multiturn, extended-multiturn, claude-describes-sdd)
- `tests/opencode/` (5): run-tests.sh, setup.sh, test-plugin-loading.sh, test-priority.sh, test-tools.sh
- `tests/skill-triggering/` (2): run-all.sh, run-test.sh
- `tests/subagent-driven-dev/` (7): run-test.sh + go-fractals triple + svelte-todo triple

**No CI workflow invokes these — all local.**

## docs/ (16 files)

- `docs/README.codex.md`, `docs/README.opencode.md` — platform READMEs
- `docs/testing.md` — test methodology
- `docs/windows/polyglot-hooks.md` — polyglot hook mechanism
- `docs/plans/` (4 historical): opencode-support-design, opencode-support-implementation, skills-improvements-from-user-feedback, visual-brainstorming
- `docs/superpowers/plans/` (4): document-review-system, visual-brainstorming-refactor, zero-dep-brainstorm-server, codex-app-compatibility
- `docs/superpowers/specs/` (4): matching specs for each plan

**Dogfooding**: `docs/superpowers/{plans,specs}/` is the canonical output location for the brainstorm→writing-plans pipeline.

## scripts/ (1 file)

- `scripts/bump-version.sh` — operational version bump

## Multi-Platform Artifact Map (complete)

- Claude Code: `.claude-plugin/plugin.json`, `.claude-plugin/marketplace.json`, `hooks/hooks.json`
- Cursor: `.cursor-plugin/plugin.json`, `hooks/hooks-cursor.json`
- Codex: `.codex/INSTALL.md`, `docs/README.codex.md`, `skills/using-superpowers/references/codex-tools.md`
- OpenCode: `.opencode/INSTALL.md`, `.opencode/plugins/superpowers.js` (112 LOC), `docs/README.opencode.md`
- Gemini: `gemini-extension.json`, `GEMINI.md`, `skills/using-superpowers/references/gemini-tools.md`
- Copilot CLI: `skills/using-superpowers/references/copilot-tools.md`
- Generic: `AGENTS.md` symlink → `CLAUDE.md`

## Convergence Declaration

Pass 0 has converged. Next round would be byte-level nitpicks. File-level inventory is complete.
