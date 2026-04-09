# Pass 1 — Architecture

_Phase B deepening round 1._

## Canonical layout

From `plugin-dev/skills/plugin-structure/SKILL.md:22-37` and `manifest-reference.md:5-9`:

```
plugin-name/
├── .claude-plugin/plugin.json   # REQUIRED, exact path (absent => not loadable)
├── commands/*.md                # default ./commands (override in plugin.json)
├── agents/*.md                  # default ./agents
├── skills/<name>/SKILL.md       # REQUIRED per skill
│   └── references/, examples/, scripts/
├── hooks/hooks.json             # default ./hooks/hooks.json, inline object OR file path
├── .mcp.json                    # default ./.mcp.json, inline object OR file path
└── README.md (prose only)
```

Custom `commands`/`agents` fields in `plugin.json` **supplement** defaults, not replace (`manifest-reference.md:365-371`). Nested command subdirectories are NOT auto-discovered (`component-patterns.md:111-121`).

## Component lifecycle (round 1, new)

From `plugin-dev/skills/plugin-structure/references/component-patterns.md:7-26`. This is the **canonical runtime loading model**:

1. **Discovery phase** — executes ONCE at Claude Code session init. For each enabled plugin: read `plugin.json`, enumerate component directories per defaults-plus-supplements rule, parse frontmatter, register commands/agents/skills/hooks, start MCP stdio servers eagerly, merge plugin hooks with user-settings hooks into a single event→handler-list map.
2. **Activation phase** — continuous during session. Commands fire on `/slash`; Agents fire on `Task` dispatch; Skills fire on description match; Hooks fire on event; MCP tools fire on tool invocation. No re-scan.

**Architectural consequence: no hot-reload.** Editing any plugin file requires restarting Claude Code. This pins the entire runtime model — everything else flows from Discovery being one-shot.

## Architectural patterns (round 1 reconciled)

Phase A's six patterns (A–F) remain valid. Round 1 adds one pattern and refines two.

- **A. Command-and-agents orchestration** — `feature-dev`, `code-review`, `pr-review-toolkit`, `agent-sdk-dev`. Single command spawns agents via `Task`. Agents carry `model:`/`tools:`/`color:` frontmatter (validated per BC-A05/A06).
- **B. Skill-only** — `claude-opus-4-5-migration`, `frontend-design`. Just `skills/<name>/SKILL.md`; auto-activated by description match.
- **C. Hook-only output-style emulation** — `explanatory-output-style`, `learning-output-style`. SessionStart emits `{hookSpecificOutput:{additionalContext:"..."}}`. The deprecated "output style" concept is reimplemented as a hook.
- **D. Security/policy hook** — `security-guidance`. PreToolUse `Edit|Write|MultiEdit`; exit 2 blocks. **Refined round 1**: now that PreToolUse supports `permissionDecision` + `updatedInput` (pass 2 round 2), such plugins can *rewrite* the invocation, not just block it — no in-repo plugin exercises this yet.
- **E. Runtime Python package** — `hookify` only. `hooks/{pretooluse,posttooluse,stop,userpromptsubmit}.py` bootstrap `sys.path` from `${CLAUDE_PLUGIN_ROOT}` then import `hookify.core.*`. **Refined round 1**: only `hookify/core/` is real (610 LOC); `hookify/matchers/` and `hookify/utils/` are empty `__init__.py` placeholders. The actual dispatch is `hooks entry → core.config_loader → core.rule_engine`.
- **F. Loop interception** — `ralph-wiggum`. Stop hook continues iterating until cancel file exists.
- **G. (new round 1) Plugin Settings overlay** — `hookify` exemplifies. `.claude/<plugin-name>.local.md` (YAML frontmatter + markdown body) is read at runtime by hooks (bash sed/grep), commands (Read tool), and agents (instructions) to toggle behavior per-project. **ONLY runtime-adjustable config path** in the ecosystem (the rest loads at Discovery only). Gitignored, `chmod 600`. Source: `plugin-dev/skills/plugin-settings/SKILL.md:1-60` and `plugins/hookify/examples/*.local.md`.
- **H. (refinement) MCP transport taxonomy** — plugin.json `mcpServers` or `.mcp.json` declare stdio / sse / http / ws transports. Stdio eager (session-lifetime process); remote transports may be lazy. Auth: env vars (stdio), OAuth (sse/http), static `headers`, or dynamic `headersHelper` script.

## Composition inside a plugin

- Commands invoke same-plugin skills via the `Skill` tool — `hookify/commands/hookify.md:9`.
- Backtick-bang context injection: `commit-commands/commands/commit.md:5-9` embeds `` !`git status` `` — first-class feature.
- `allowed-tools` with subcommand globs: `commit.md:2` scopes to `Bash(git add:*), ...` — the only compile-time blast-radius control. Field type is **String OR Array**.

## Hook invocation data flow (round 1 rewritten)

Updated for parallel execution, dual handler types, and permission-decision envelopes.

```
Event fires (e.g. PreToolUse)
        │
        ▼
Runtime consults merged event→handler map built at Discovery
        │    (plugin hooks ∪ user-settings hooks)
        ▼
For each entry whose matcher regex matches tool_name (case-sensitive,
  pipe-alt, ".*", or literal; omitting matcher = match-all per docs —
  see TENSION-03):
        │
        ├─ type:"command"  → spawn subprocess, stdin = JSON envelope
        │                    {session_id, transcript_path, cwd,
        │                     permission_mode, hook_event_name,
        │                     ...event-specifics}
        │                    env += CLAUDE_PROJECT_DIR, CLAUDE_PLUGIN_ROOT,
        │                    (SessionStart only) CLAUDE_ENV_FILE,
        │                    CLAUDE_CODE_REMOTE
        │                    timeout default 60s
        │
        └─ type:"prompt"   → LLM-evaluated prompt string
                             (only on Stop/SubagentStop/UserPromptSubmit/PreToolUse)
                             timeout default 30s
        │
        ▼
ALL matching handlers run IN PARALLEL.
No cross-handler state. Non-deterministic order.
        │
        ▼
Per handler, stdout parsed as JSON envelope:
  • common: {continue?, suppressOutput?, systemMessage?}
  • PreToolUse: {hookSpecificOutput:{permissionDecision:"allow"|"deny"|"ask",
                                     updatedInput:{...}}}  ← can REWRITE input
  • Stop/SubagentStop: {decision:"approve"|"block", reason, systemMessage}
  • SessionStart: {hookSpecificOutput:{hookEventName:"SessionStart",
                                       additionalContext:"..."}}
Exit codes: 0=success, 2=blocking (stderr → Claude), other=non-blocking error
```

## Plugin-hook / user-hook merge (round 1, new)

Plugin `hooks/hooks.json` uses a wrapper format `{description?, hooks: {<Event>: [...]}}`. User `~/.claude/settings.json` uses the direct format `{<Event>: [...]}` with NO wrapper. Concrete examples of the user format live in `examples/settings/*.json` at repo root. At Discovery, the runtime strips the plugin wrapper and **merges all event-keyed handler lists into a single map** keyed by event name. At invocation time there is no distinction between a plugin-sourced hook and a user-sourced hook: they race in parallel against the same stdin envelope. This is the primary reason hot-reload is impossible (Discovery owns the merge).

## Plugin Settings runtime overlay (round 1, new)

`.claude/<plugin-name>.local.md` is the single architectural escape hatch from the immutable Discovery model. At runtime:

- **Hooks** read it via `sed`/`grep` on the frontmatter before deciding whether to act.
- **Commands** read it via the `Read` tool when invoked.
- **Agents** are instructed to consult it during their task.

Because this file lives outside the plugin directory and is user-owned, changes take effect on the NEXT event/invocation — no restart. Every other configuration path requires restart. This makes Plugin Settings the **only dynamic layer** in an otherwise fully-static runtime.

## Composition of configuration (round 1)

Layer ordering from lowest (immutable, session-lifetime) to highest (dynamic, per-invocation):

1. `plugin.json` defaults — immutable, Discovery-time
2. `plugin.json` overrides — immutable, Discovery-time (supplement semantics)
3. User `~/.claude/settings.json` hooks — immutable, Discovery-time, merged with plugin hooks
4. Per-command frontmatter — fixed at parse time
5. Per-agent frontmatter — fixed at parse time, regex/enum validated
6. **Plugin Settings `.claude/<name>.local.md`** — runtime, per-invocation, user-editable without restart

## Deployment topology

Still none. No daemon, no server, no package manager. Plugins are static file trees. Discovery is one-shot at Claude Code process init; all runtime dynamism comes from (a) parallel hook races, (b) MCP remote transports, and (c) Plugin Settings overlay reads. Restart Claude Code = full reset.
