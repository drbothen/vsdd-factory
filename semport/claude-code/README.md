# claude-code (Anthropic) — Brownfield Ingest Phase A

**Source**: `/Users/jmagady/Dev/vsdd-factory/.reference/claude-code` @ `22fdf68049e8c24e5a36087bb742857d3d5e407d`
**Scope**: `plugins/` directory (13 plugin folders)
**Ingested**: 2026-04-08

## Executive summary

13 reference plugins, ~26,313 LOC (almost all markdown), exemplifying 6 architectural patterns. The `plugin-dev/` plugin is the canonical reference bundle — ironically lacking a `plugin.json`. Plugins are static file trees loaded by convention via `.claude-plugin/plugin.json` + auto-discovered component directories. `${CLAUDE_PLUGIN_ROOT}` is the sole portability mechanism for command paths. No tests anywhere.

## Passes

- [Pass 0 — Inventory](./pass-0-inventory.md)
- [Pass 1 — Architecture](./pass-1-architecture.md)
- [Pass 2 — Domain Model](./pass-2-domain-model.md)
- [Pass 3 — Behavioral Contracts](./pass-3-behavioral-contracts.md)
- [Pass 4 — NFRs](./pass-4-nfrs.md)
- [Pass 5 — Conventions](./pass-5-conventions.md)
- [Pass 6 — Synthesis & Lessons for vsdd-factory](./pass-6-synthesis.md)

## Top 5 lessons for vsdd-factory

1. Manifest must be `./.claude-plugin/plugin.json` exactly — load-bearing.
2. Skill descriptions must enumerate trigger phrases, not abstract prose.
3. "Output style" is not first-class — reimplement as a SessionStart hook emitting `hookSpecificOutput.additionalContext`.
4. Agent `tools:` is a comma-separated bare string, not a YAML list.
5. Use `${CLAUDE_PLUGIN_ROOT}` for all hook/MCP command paths; use `.factory/` (never `/tmp/` or `~/.claude/`) for plugin state.

See pass-6 for the complete lesson list (L1–L13) and the checklist of claims to verify against vsdd-factory's current design.
