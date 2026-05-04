---
name: onboard-observability
description: One-command setup to wire the current factory project into the local vsdd-factory observability stack. Runs `factory-obs register` (adds this project's .factory/logs to the collector's watch list) AND writes Claude Code's OTel env vars to .claude/settings.local.json (so Claude ships its tool calls, token counts, and costs to Loki + Prometheus). Use when the user asks to "register this project with observability", "set up observability here", "onboard the observability stack", or any similar phrasing that implies first-time observability setup on a project. Idempotent — safe to run repeatedly.
allowed-tools: Bash, Read, Write
---

# Onboard Observability

Two-part first-time setup for a factory project's observability:

1. **Register the project** with `factory-obs` so its `.factory/logs/events-*.jsonl` stream is tailed by the local OTel collector.
2. **Enable Claude Code OTel telemetry** by writing the 5 OTEL_* env vars to `.claude/settings.local.json`, so Claude's own tool-call/cost/token events flow to the same stack.

After this skill runs successfully, everything is ready — the user just needs to **restart their Claude Code session** for the OTel env vars to take effect, and run `factory-obs up` if the stack isn't already running.

## Announce at Start

Before any other action, say verbatim:

> I'm using the onboard-observability skill to wire this project into the local observability stack.

## Prerequisites (check + abort if missing)

1. Current working directory (or the nearest ancestor) must contain a `.factory/` subdirectory. Walk up from `$PWD` to find it. If none, abort with a clear error explaining the user needs to run this from inside a factory project.
2. The `factory-obs` binary must be present at `${CLAUDE_PLUGIN_ROOT}/bin/factory-obs`. If not, abort with "vsdd-factory plugin not found — is it installed in this Claude Code environment?".

Don't fail silently — always print what's missing so the user can fix it.

## Execute

### Step 1 — Register the project with factory-obs

Invoke the register subcommand. It autoresolves the project root from cwd and dedupes if already registered.

```bash
"${CLAUDE_PLUGIN_ROOT}/bin/factory-obs" register
```

Surface the binary's output verbatim. Expect one of:
- `factory-obs: registered <abs-path>` — first-time success
- `factory-obs: already registered: <abs-path>` — idempotent no-op

Either is success.

### Step 2 — Write Claude OTel env vars to `.claude/settings.local.json`

This is the same operation the `/vsdd-factory:claude-telemetry on` skill performs. Reproduced inline here so this skill is self-contained:

```bash
# Ensure .claude/ exists.
mkdir -p .claude

# If settings.local.json doesn't exist, seed with {}.
if [ ! -f .claude/settings.local.json ]; then
  echo '{}' > .claude/settings.local.json
fi

# Merge the 5 OTEL_* keys into .env. Prune the legacy
# OTEL_EXPORTER_OTLP_METRICS_TEMPORALITY_PREFERENCE key if present
# (left over from legacy claude-telemetry runs predating the
# collector's deltatocumulative processor).
jq '.env = ((.env // {}) + {
    "CLAUDE_CODE_ENABLE_TELEMETRY": "1",
    "OTEL_METRICS_EXPORTER": "otlp",
    "OTEL_LOGS_EXPORTER": "otlp",
    "OTEL_EXPORTER_OTLP_PROTOCOL": "http/protobuf",
    "OTEL_EXPORTER_OTLP_ENDPOINT": "http://localhost:4318"
}) | del(.env.OTEL_EXPORTER_OTLP_METRICS_TEMPORALITY_PREFERENCE)' \
  .claude/settings.local.json > .claude/settings.local.json.tmp \
  && mv .claude/settings.local.json.tmp .claude/settings.local.json
```

Confirm by printing the 5 resulting env keys:

```bash
jq '.env | { CLAUDE_CODE_ENABLE_TELEMETRY, OTEL_METRICS_EXPORTER, OTEL_LOGS_EXPORTER, OTEL_EXPORTER_OTLP_PROTOCOL, OTEL_EXPORTER_OTLP_ENDPOINT }' .claude/settings.local.json
```

### Step 3 — Status summary

Print a crisp summary to the user:

```
✓ Registered <abs-path> with factory-obs
✓ Wrote 5 OTEL_* env vars to .claude/settings.local.json
✓ Observability onboarding complete.

Next steps:
  1. Restart this Claude Code session so the OTEL env vars take effect.
  2. If the stack isn't running, run:  factory-obs up
  3. Open the Grafana dashboards:      factory-obs dashboard
     (or visit http://localhost:3000 directly)
```

If `factory-obs status` can be run without side effects and the stack is clearly already up (healthy containers), mention that instead of telling the user to run `up`.

## Idempotency

Safe to run multiple times in the same project:
- `factory-obs register` deduplicates on absolute path.
- The jq merge preserves existing env keys and only overwrites the 5 OTEL_* keys. Other unrelated keys (permissions, agent hints, etc.) are untouched.

Safe to run across multiple projects:
- Each project gets its own `.claude/settings.local.json` with the same telemetry env vars. They all ship to the same `localhost:4318` endpoint, and the Loki/Prom datasources distinguish sessions by `session_id` label.

## Do not touch

- **Other keys in `.env`** — only the 5 listed above. Preserve all others exactly as-is.
- **Other top-level keys** in `settings.local.json` — `permissions`, `agent`, etc. must not change.
- **`settings.json`** (shared / team-level) — this skill always writes to `.local.json` only.
- **The Docker stack's base compose file or collector config** — those edits go through normal PR flow, not this skill.

## Non-goals

This skill **does not**:
- Start or stop the Docker stack — invoke `/vsdd-factory:factory-obs up` for that. This skill only registers + enables telemetry config.
- Register multiple projects at once — run separately in each project's root.
- Unregister or disable telemetry — use `/vsdd-factory:factory-obs unregister` and `/vsdd-factory:claude-telemetry off` for those.
- Run without Docker — no cloud fallback.

## When to use

- **Brand new project** that just had the vsdd-factory plugin installed.
- **Existing project** you've been working in but haven't connected to your running stack yet (e.g., you set up the stack from a different project and want this one's events in the same Grafana).
- **Troubleshooting**: "my events aren't showing up in Grafana" — re-running this skill is a quick idempotent fix that verifies both register + telemetry config are in place.

## When NOT to use

- If the user wants to **manage the stack lifecycle** (up/down/reset) — use `/vsdd-factory:factory-obs` directly.
- If the user wants to **only** enable/disable telemetry (not registration) — use `/vsdd-factory:claude-telemetry`.
- If the user wants to see **which projects are registered** — use `/vsdd-factory:factory-obs list`.
