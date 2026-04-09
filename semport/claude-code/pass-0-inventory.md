# Pass 0 — Inventory

_Phase B deepening round 1._

**Repo root**: `plugins/`, `examples/`, `scripts/`, `Script/`, `CHANGELOG.md`, `README.md`, `LICENSE.md`, `SECURITY.md`, `demo.gif`. No root build system (no `package.json`, `Cargo.toml`, `pyproject.toml`) — this is a docs + reference-plugin monorepo.

**Plugins totals**: **13 plugin folders** (+ `plugins/README.md`); **138 files** total under `plugins/` (verified round 1); LOC by ext: markdown **26,313**, python **1,147**, shell **1,552**, json **285**.

**Runtime code inventory (round 1 corrected)**:

- `hookify/core/` — **real** Python package: `config_loader.py` (297), `rule_engine.py` (313), `__init__.py` (0). Total 610 LOC.
- `hookify/hooks/` — 4 entry scripts: `pretooluse.py` (74), `posttooluse.py` (66), `userpromptsubmit.py` (58), `stop.py` (59). Total 257 LOC.
- `hookify/matchers/` — **empty shell**: only `__init__.py` (0 lines). No modules.
- `hookify/utils/` — **empty shell**: only `__init__.py` (0 lines). No modules. Phase A implied these were populated; corrected.
- `security-guidance/hooks/security_reminder_hook.py` — ~280 LOC, 9 hardcoded patterns.
- Shell handlers: `ralph-wiggum/hooks/stop-hook.sh`, `ralph-wiggum/scripts/setup-ralph-loop.sh`, `explanatory-output-style/hooks-handlers/session-start.sh`, `learning-output-style/hooks-handlers/session-start.sh`.

## Per-plugin manifest

| Plugin | plugin.json | Commands | Agents | Skills | Hooks | Notable |
|---|---|---|---|---|---|---|
| agent-sdk-dev | yes | 1 (`new-sdk-app`) | 2 (`agent-sdk-verifier-py`/`-ts`) | — | — | SDK bootstrap + verifier |
| claude-opus-4-5-migration | yes | — | — | 1 (+ `references/`) | — | Skill-only |
| code-review | yes | 1 (`code-review.md`) | — (inline 5-agent orchestration in command) | — | — | Parallel agent command |
| commit-commands | yes | 3 (`commit`, `commit-push-pr`, `clean_gone`) | — | — | — | Uses `allowed-tools` subcommand globs |
| explanatory-output-style | yes | — | — | — | SessionStart → sh handler | Hook-as-output-style |
| feature-dev | yes | 1 (`feature-dev.md`) | 3 (`code-architect`, `code-explorer`, `code-reviewer`) | — | — | 7-phase workflow |
| frontend-design | yes | — | — | 1 | — | Skill-only |
| hookify | yes | 4 | 1 (`conversation-analyzer`) | 1 (`writing-rules`) | 4 events (Pre/Post/Stop/UserPromptSubmit) | Only plugin with runtime Python package (`hookify/core` 610 LOC); reads `.claude/hookify.local.md` via sed/grep (see Plugin Settings) |
| learning-output-style | yes | — | — | — | SessionStart → sh | Twin of explanatory |
| plugin-dev | **NO plugin.json** (verified round 1: zero `plugin.json` anywhere in tree) | 1 (`create-plugin`) | 3 (`agent-creator` 176, `plugin-validator` 183, `skill-reviewer` 184 LOC) | **7** (agent-development, command-development, hook-development, mcp-integration, plugin-settings, plugin-structure, skill-development) | — | Reference bundle — NOT loadable as plugin. Source of truth for every spec claim in passes 2+3. |
| pr-review-toolkit | yes | 1 (`review-pr`) | 6 agents (all `model: inherit`) | — | — | Most agents |
| ralph-wiggum | yes | 3 (`ralph-loop`, `cancel-ralph`, `help`) | — | — | Stop hook (sh) | Loop interception |
| security-guidance | yes | — | — | — | PreToolUse `Edit\|Write\|MultiEdit` (Python ~280 LOC) | 9 hardcoded patterns |

## plugin-dev skills anatomy (new round 1)

Phase A listed 7 skills but did not enumerate their ancillary files. Round 1 inventory:

| Skill | SKILL.md | references/ | examples/ | scripts/ | Notes |
|---|---|---|---|---|---|
| agent-development | yes | 3 (system-prompt-design, triggering-examples, agent-creation-system-prompt) | 2 | 1 (`validate-agent.sh`) | Source for BC-A05/A06 |
| command-development | yes | 7 (plugin-features, frontmatter, marketplace, documentation-patterns, advanced-workflows, testing-strategies, interactive-commands) | 2 | — | Largest references bundle |
| hook-development | yes | 3 (migration, patterns, advanced) | 3 (validate-write.sh, load-context.sh, validate-bash.sh) | 4 (hook-linter.sh, validate-hook-schema.sh, test-hook.sh, README.md) | `validate-hook-schema.sh` is load-bearing for TENSION-03 |
| mcp-integration | yes | 3 (authentication, server-types, tool-usage) | 3 (stdio-server.json, http-server.json, sse-server.json) | — | `authentication.md:233-258` is source for `headersHelper` |
| plugin-settings | yes | 2 (real-world-examples, parsing-techniques) | 3 (example-settings.md, read-settings-hook.sh, create-settings-command.md) | 2 (validate-settings.sh, parse-frontmatter.sh) | Sole source for pass-2 Plugin Settings subsystem |
| plugin-structure | yes | 2 (component-patterns, manifest-reference) | 3 (advanced, minimal, standard) | — | `component-patterns.md:7-26` = Discovery/Activation |
| skill-development | yes | 1 (skill-creator-original) | — | — | Smallest |

## Root ancillary dirs (new round 1)

- `examples/hooks/bash_command_validator_example.py` — 1 file. Reference PreToolUse example for bash validation.
- `examples/settings/` — 4 files: `settings-lax.json`, `settings-strict.json`, `settings-bash-sandbox.json`, `README.md`. Concrete reference user-settings files demonstrating BC-DRAFT-H01b direct (wrapper-less) hook format. First concrete evidence of the user-settings hook schema in-repo.
- `scripts/` — **NOT plugin runtime**. 5 TypeScript files (auto-close-duplicates, sweep, lifecycle-comment, issue-lifecycle, backfill-duplicate-comments) + 3 shell (gh.sh, edit-issue-labels.sh, comment-on-duplicates.sh). GitHub issue automation for the repo itself.
- `Script/` — 1 file: `run_devcontainer_claude_code.ps1`. PowerShell devcontainer launcher. Unrelated to plugin runtime.

## Size and tech stack

**Biggest plugin by LOC**: `plugin-dev` (7 skills, dominant markdown). Second tier: `frontend-design`, `pr-review-toolkit`.

**Tech stack**: external Claude Code CLI (not in repo). Plugins are pure config+prompts+scripts. Python3 for `hookify/core` + `security-guidance`, bash for `ralph` + output-style + `plugin-dev` example/validator scripts. **No shared libs across plugins, no dependency manifests inside any plugin, no tests anywhere.**

**Dependency graph**: flat. All plugins self-contained. `plugin-dev` is conceptual-only — it contains zero `plugin.json`, therefore cannot be loaded; it exists purely as the reference corpus for authoring other plugins. All other 12 plugins are independently loadable.
