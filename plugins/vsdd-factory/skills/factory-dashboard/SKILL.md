---
name: factory-dashboard
description: Render a live markdown dashboard of the VSDD factory pipeline — current phase, wave progress, recent hook activity, and pending gates. Read-only diagnostic; no writes.

allowed-tools: Bash
---

# Factory Dashboard

Emit a single-page markdown dashboard summarizing the current state of the
factory. Combines STATE.md frontmatter, wave-state.yaml, and the observability
event log into one view.

This skill is distinct from `factory-health` — that one validates the
`.factory/` worktree structure, while this one summarizes pipeline state.

## Announce at Start

Before any other action, say verbatim:

> I'm using the factory-dashboard skill to render the pipeline dashboard.

## Execute

Run the `factory-dashboard` binary from the plugin's `bin/` directory:

```bash
"${CLAUDE_PLUGIN_ROOT}/bin/factory-dashboard"
```

The script reads `.factory/STATE.md`, `.factory/wave-state.yaml`, and
`.factory/logs/events-*.jsonl` relative to the current working directory.
All three sources are optional — missing files produce clean "not
initialized" notices rather than errors.

## Options

- `--factory PATH` — use a different `.factory/` location.
- `--days N` — change the event-log lookback window (default: 7 days).

Examples:

```bash
"${CLAUDE_PLUGIN_ROOT}/bin/factory-dashboard" --days 30
"${CLAUDE_PLUGIN_ROOT}/bin/factory-dashboard" --factory /path/to/other/project/.factory
```

## Output

The dashboard is emitted on stdout as markdown. Surface it to the user as-is
— no summarization. The user will read the rendered markdown directly.

## When to use

- Start of a new session, to orient.
- After a long break, to re-check pipeline state.
- Before spawning a worker, to confirm prerequisites (waves, gates).
- When diagnosing "why did X fire?" — scan recent activity for patterns.

## Non-goals

This skill **does not**:
- Modify any files (read-only).
- Run hooks or trigger pipeline actions.
- Query external services.
- Produce actionable recommendations beyond the dashboard itself.

For deeper queries, use `factory-query` or `factory-report` directly from
the shell. For worktree validation, use `/vsdd-factory:factory-health`.
