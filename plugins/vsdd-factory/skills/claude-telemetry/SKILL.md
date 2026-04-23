---
name: claude-telemetry
description: Wire Claude Code's native OpenTelemetry export into the local vsdd-factory observability stack. Writes the five OTEL_* env vars to .claude/settings.local.json so Claude ships its tool calls, token counts, and API events to the same Loki that our hook events use. Reversible; local-only.
disable-model-invocation: true
allowed-tools: Read, Write, Bash
---

# Claude Telemetry → Factory Observability

Claude Code emits detailed OTel telemetry (every tool call, token count,
session event, API request/response summary) when
`CLAUDE_CODE_ENABLE_TELEMETRY=1` is set. Our otel-collector (started via
`factory-obs up`) accepts this export on port 4318 and forwards the logs
to the same Loki that hook events use, tagged `service_name=claude-code`
so they're queryable separately.

This skill manages the five environment variables Claude needs to export
to our collector. It writes them to `.claude/settings.local.json` under
the `env` block so the current project's Claude sessions pick them up
on next start.

## Prerequisites

- The observability stack must be running (`factory-obs up`). If it
  isn't, Claude will silently fail to export and the user will see
  nothing in Grafana. Check before enabling — suggest `factory-obs up`
  first if needed.
- Claude Code reads `.claude/settings.local.json` on session start.
  **The user must restart their Claude Code session** for the env vars
  to take effect. Surface this prominently after `on`.

## Announce at Start

Before any other action, say verbatim:

> I'm using the claude-telemetry skill to configure Claude Code's native OTel export.

## Arguments

| Arg | Effect |
|---|---|
| `on` (default) | Write the five OTEL env vars into `.claude/settings.local.json`. |
| `off` | Remove those five keys (preserves any other `env` entries). |
| `status` | Print which of the five keys are currently set and their values. |

If the user passes something else, show `status` and a usage hint.

## The env vars

Five keys, all strings, all go under the `env` object:

| Key | Value | Why |
|---|---|---|
| `CLAUDE_CODE_ENABLE_TELEMETRY` | `"1"` | Master enable flag. |
| `OTEL_METRICS_EXPORTER` | `"otlp"` | Metrics go to the collector (absorbed by debug pipeline for now). |
| `OTEL_LOGS_EXPORTER` | `"otlp"` | Logs go to Loki via the collector. |
| `OTEL_EXPORTER_OTLP_PROTOCOL` | `"http/protobuf"` | Must match the receiver — compose exposes 4318 HTTP, not 4317 gRPC. |
| `OTEL_EXPORTER_OTLP_ENDPOINT` | `"http://localhost:4318"` | Host the collector is bound to. Override via `VSDD_OBS_OTLP_HTTP_PORT` if 4318 is remapped. |

## Execute — `on`

1. Determine the target file: `<project_root>/.claude/settings.local.json`.
   If `.claude/` doesn't exist, create it.
2. Read existing contents (or start with `{}` if the file is missing).
3. Merge the five keys into the `env` object using `jq`:

   ```bash
   jq '.env = ((.env // {}) + {
       "CLAUDE_CODE_ENABLE_TELEMETRY": "1",
       "OTEL_METRICS_EXPORTER": "otlp",
       "OTEL_LOGS_EXPORTER": "otlp",
       "OTEL_EXPORTER_OTLP_PROTOCOL": "http/protobuf",
       "OTEL_EXPORTER_OTLP_ENDPOINT": "http://localhost:4318"
   })' .claude/settings.local.json > .claude/settings.local.json.tmp \
     && mv .claude/settings.local.json.tmp .claude/settings.local.json
   ```

4. Confirm by printing the five keys from the updated file.
5. **Print a prominent restart reminder**: "Restart your Claude Code
   session so the env vars take effect." Also confirm the collector is
   up (`docker ps --filter name=vsdd-obs-collector`) and tell the user
   to run `factory-obs up` first if not.
6. Mention the LogQL query for filtering Claude's telemetry in Grafana:
   `{service_name="claude-code"}`.

## Execute — `off`

1. Read `.claude/settings.local.json`. If missing or the keys aren't
   present, print "claude-telemetry already off" and exit.
2. Remove the five keys using `jq`:

   ```bash
   jq 'del(
       .env.CLAUDE_CODE_ENABLE_TELEMETRY,
       .env.OTEL_METRICS_EXPORTER,
       .env.OTEL_LOGS_EXPORTER,
       .env.OTEL_EXPORTER_OTLP_PROTOCOL,
       .env.OTEL_EXPORTER_OTLP_ENDPOINT
   ) | if (.env | length) == 0 then del(.env) else . end' \
     .claude/settings.local.json > .claude/settings.local.json.tmp \
     && mv .claude/settings.local.json.tmp .claude/settings.local.json
   ```

3. Confirm removal and remind the user to restart Claude.

## Execute — `status`

Read `.claude/settings.local.json` and print which of the five keys are
set and their values. If any are missing or mismatched, point that out.
If the file doesn't exist, print "no .claude/settings.local.json — telemetry off".

## Do not touch

- **Other `env` keys** — only add/remove the five listed above. Preserve
  all others exactly.
- **Other top-level keys** — `agent`, `permissions`, etc. must not change.
- **`settings.json` (shared)** — always write to `.local.json`. The shared
  file is a team artifact; the local file is per-user and gitignored.

## When to use

- First-time setup on a new project: `on` to enable, then `factory-obs up`.
- Debugging "why isn't Claude's telemetry in Grafana?": `status` to
  verify the env is set + collector is up.
- Disabling before a commit if the env vars shouldn't leak into shared
  settings (they shouldn't — `.local.json` is typically gitignored, but
  double-check project conventions).

## Non-goals

This skill **does not**:
- Start or stop the observability stack — that's `/vsdd-factory:factory-obs`.
- Configure the collector itself — the collector accepts OTLP out of the
  box once `up`.
- Enable `OTEL_LOG_USER_PROMPTS` or `OTEL_LOG_TOOL_DETAILS` or
  `OTEL_LOG_TOOL_CONTENT` — those gate sensitive content (prompts, tool
  payloads) and require explicit user consent. If the user asks for
  prompt content in logs, instruct them to add those keys manually with
  a note about privacy implications.
- Modify the plugin's shipped `docker-compose.yml` or collector config.
