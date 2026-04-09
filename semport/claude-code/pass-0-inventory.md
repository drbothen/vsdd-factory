# Pass 0 — Inventory

**Repo root**: `plugins/`, `examples/`, `scripts/`, `Script/`, `CHANGELOG.md`, `README.md`, `LICENSE.md`, `SECURITY.md`, `demo.gif`. No root build system (no `package.json`, `Cargo.toml`, `pyproject.toml`) — this is a docs + reference-plugin monorepo.

**Plugins totals**: **13 plugin folders** (+ `plugins/README.md`); **138 files** total under `plugins/`; **~26,313 LOC** in markdown (dominant format). Runtime code is minimal: `hookify/` (Python package: `core/`, `matchers/`, `utils/`, 4 hook entry scripts) and `security-guidance/hooks/security_reminder_hook.py` (~280 LOC). Shell: `ralph-wiggum/hooks/stop-hook.sh`, `ralph-wiggum/scripts/setup-ralph-loop.sh`, `explanatory-output-style/hooks-handlers/session-start.sh`, `learning-output-style/hooks-handlers/session-start.sh`, plus `plugin-dev` skill example scripts.

## Per-plugin manifest

| Plugin | plugin.json | Commands | Agents | Skills | Hooks | Notable |
|---|---|---|---|---|---|---|
| agent-sdk-dev | yes | 1 (`new-sdk-app`) | 2 (`agent-sdk-verifier-py`/`-ts`) | — | — | SDK bootstrap + verifier |
| claude-opus-4-5-migration | yes | — | — | 1 (+ `references/`) | — | Skill-only |
| code-review | yes | 1 (`code-review.md`) | — (inline 5-agent orchestration in command) | — | — | Parallel agent command |
| commit-commands | yes | 3 (`commit`, `commit-push-pr`, `clean_gone`) | — | — | — | Uses `allowed-tools` subcommand globs |
| explanatory-output-style | yes | — | — | — | SessionStart → sh handler | "Output style" via hook+additionalContext |
| feature-dev | yes | 1 (`feature-dev.md`) | 3 (`code-architect`, `code-explorer`, `code-reviewer`) | — | — | 7-phase workflow, agents declare `tools:`/`model:`/`color:` |
| frontend-design | yes | — | — | 1 | — | Skill-only, large design doc |
| hookify | yes | 4 | 1 (`conversation-analyzer`) | 1 (`writing-rules`) | 4 events (Pre/Post/Stop/UserPromptSubmit, `hooks.json` + .py handlers) | **Only plugin with runtime Python package**; reads `.local.md` user rules |
| learning-output-style | yes | — | — | — | SessionStart → sh | Twin of explanatory |
| plugin-dev | **NO plugin.json** | 1 (`create-plugin`) | 3 (`agent-creator`, `plugin-validator`, `skill-reviewer`) | **7** (agent/command/hook/mcp-integration/plugin-settings/plugin-structure/skill development) | — | Canonical reference bundle (biggest by LOC) |
| pr-review-toolkit | yes | 1 (`review-pr`) | 6 (`code-reviewer`, `code-simplifier`, `comment-analyzer`, `pr-test-analyzer`, `silent-failure-hunter`, `type-design-analyzer`) | — | — | Most agents; all use `model: inherit` |
| ralph-wiggum | yes | 3 (`ralph-loop`, `cancel-ralph`, `help`) | — | — | Stop hook (sh) | Loop interception |
| security-guidance | yes | — | — | — | PreToolUse `Edit\|Write\|MultiEdit` (Python ~280 LOC) | 9 hardcoded security patterns |

## Size and tech stack

**Biggest plugin by LOC**: `plugin-dev` (7 skills × {SKILL.md + references/ + examples/ + scripts/}). Second tier: `frontend-design`, `pr-review-toolkit`.

**Tech stack**: external Claude Code CLI (not in repo). Plugins are pure config+prompts+scripts. Python3 for `hookify`/`security-guidance` hooks, bash for `ralph` + output-style plugins. **No shared libs across plugins, no dependency manifests inside any plugin, no tests anywhere.**

**Dependency graph**: flat. All plugins self-contained. Only `plugin-dev` conceptually informs the others (no code dependency).
