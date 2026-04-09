# Pass 1 — Architecture

## Canonical layout

From `plugin-dev/skills/plugin-structure/SKILL.md:22-37` and `manifest-reference.md:5-9`:

```
plugin-name/
├── .claude-plugin/plugin.json   # REQUIRED, exact path
├── commands/*.md                # auto-discovered
├── agents/*.md                  # auto-discovered
├── skills/<name>/SKILL.md       # REQUIRED per skill, auto-discovered
│   └── references/, examples/, scripts/
├── hooks/hooks.json             # + handler scripts
├── .mcp.json
└── README.md (prose only)
```

Custom `commands`/`agents` fields in `plugin.json` **supplement** defaults, not replace (`manifest-reference.md:365-371`).

## Six observed structural patterns

- **A. Command-and-agents orchestration** — `feature-dev`, `code-review`, `pr-review-toolkit`, `agent-sdk-dev`. Single command in `commands/` spawns agents via the `Task` tool. Agents carry `model:`, `tools:`, `color:` frontmatter (`feature-dev/agents/code-architect.md:1-7`).
- **B. Skill-only** — `claude-opus-4-5-migration`, `frontend-design`. Just `skills/<name>/SKILL.md`; auto-activated by description match.
- **C. Hook-only output-style emulation** — `explanatory-output-style`, `learning-output-style`. SessionStart hook emits JSON `{hookSpecificOutput:{additionalContext:"..."}}` on stdout (`explanatory-output-style/hooks-handlers/session-start.sh:6-13`). **The deprecated "output style" concept is reimplemented as a hook.**
- **D. Security/policy hook** — `security-guidance`. PreToolUse `matcher: "Edit|Write|MultiEdit"` (`hooks.json:11`); on match, exit 2 + stderr to block (`security_reminder_hook.py:271-273`). Session-scoped dedup state in `~/.claude/security_warnings_state_<session_id>.json`.
- **E. Runtime Python package** — `hookify` only. `hooks/{pretooluse,posttooluse,stop,userpromptsubmit}.py` bootstrap `sys.path` from `${CLAUDE_PLUGIN_ROOT}` then import `hookify.core.*` (`hookify/hooks/pretooluse.py:14-23`). Fragile but functional.
- **F. Loop interception** — `ralph-wiggum`. Stop hook intercepts exit to continue iterating until a cancel file exists.

## Composition inside a plugin

- Commands invoke same-plugin skills via the `Skill` tool — `hookify/commands/hookify.md:9` "FIRST: Load the hookify:writing-rules skill".
- Backtick-bang context injection: `commit-commands/commands/commit.md:5-9` embeds `` !`git status` `` directly into the prompt. First-class feature, not a convention.
- `allowed-tools` with subcommand globs: `commit.md:2` scopes to `Bash(git add:*), Bash(git status:*), Bash(git commit:*)` — the only compile-time blast-radius control in the ecosystem.

## Hook invocation data flow

Event → runtime reads `hooks.json` → matches event+matcher → spawns configured command with env `CLAUDE_PLUGIN_ROOT=<plugin dir>` + stdin JSON `{session_id, tool_name, tool_input,...}`. Exit 0 = allow, exit 2 = block (stderr shown to model), stdout JSON = structured return (e.g. `additionalContext`).

## Deployment topology

None. Plugins are static file trees loaded into a local Claude Code process. No daemon, no server, no package manager.
