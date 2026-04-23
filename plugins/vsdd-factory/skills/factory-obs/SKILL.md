---
name: factory-obs
description: Manage the local observability stack (OTel Collector + Loki + Grafana) that visualizes vsdd-factory hook events. Starts/stops/resets the Docker stack and opens the Grafana dashboard. Opt-in, local-only, no cloud services.
disable-model-invocation: true
allowed-tools: Bash
---

# Factory Observability

Launch, stop, reset, or inspect the local Docker observability stack that
ingests `.factory/logs/events-*.jsonl` into Loki and surfaces them in a
preconfigured Grafana dashboard.

The stack is entirely local and opt-in — nothing is emitted off-box. If
Docker isn't running or isn't installed, the skill surfaces the error and
stops; it never attempts to fall back to cloud services.

## Announce at Start

Before any other action, say verbatim:

> I'm using the factory-obs skill to manage the local observability stack.

## Arguments

The skill accepts one positional argument matching the `factory-obs`
subcommand. If no argument is given, default to `up` and then print the
Grafana URL.

| Arg | Effect |
|---|---|
| `up` (default) | Start the stack. Waits for Loki health, prints Grafana URL. |
| `down` | Stop the stack, keep volumes (preserves event offsets + dashboards). |
| `reset` | Stop and wipe volumes. Use when the collector is misbehaving. |
| `status` | `docker compose ps` — container health snapshot. |
| `logs` | Tail collector + Grafana logs (useful for debugging ingestion). |
| `dashboard` | Print the Grafana URL; open a browser if running interactively. |
| `help` | Show the binary's usage text. |

## Execute

Run the `factory-obs` binary with the requested subcommand:

```bash
"${CLAUDE_PLUGIN_ROOT}/bin/factory-obs" "${ARG:-up}"
```

If the user asked for `up` (or defaulted), also surface the Grafana URL
explicitly after the binary returns, so they don't have to scroll back:

```bash
"${CLAUDE_PLUGIN_ROOT}/bin/factory-obs" dashboard
```

## Environment overrides

Users may already have `3000`/`3100`/`4318` bound by another service. The
binary honors these env vars (names documented in `factory-obs help`):

- `VSDD_OBS_GRAFANA_PORT` — Grafana UI port (default 3000)
- `VSDD_OBS_LOKI_PORT` — Loki HTTP port (default 3100)
- `VSDD_OBS_OTLP_HTTP_PORT` — OTLP HTTP receiver port (default 4318)
- `VSDD_FACTORY_LOGS` — path to `.factory/logs/` (default: auto-detected from project root)
- `VSDD_OBS_OPEN_BROWSER` — `1` to force, `0` to suppress, unset for TTY auto-detect

Don't set these unless the user asked — defaults are correct for the
typical case.

## Output

Surface the binary's stdout to the user as-is. Do not summarize the
container list or port mapping — the user will want to see the raw
confirmation.

After `up`, it's helpful to remind the user that events only appear once a
factory hook fires. A quick smoke test:

```bash
"${CLAUDE_PLUGIN_ROOT}/bin/emit-event" '{"type":"hook.action","hook":"manual-test","action":"smoke"}'
```

Events should be visible in Grafana within ~10 seconds.

## When to use

- `up` — starting a session where the user wants to watch events live.
- `down` — freeing resources when done, without losing history.
- `reset` — collector is crash-looping or volumes are in a bad state.
- `status` — "is the stack running?" sanity check.
- `logs` — ingestion isn't flowing and we need to see collector output.

## Non-goals

This skill **does not**:
- Modify the compose file or collector config (edits go through normal PR flow).
- Start Claude Code's native OTel telemetry — that's an orthogonal Claude-side config.
- Run without Docker — no cloud fallback by design.
- Emit events itself — use `emit-event` or let hooks fire naturally.

For querying the ingested data without Grafana, use `factory-query`,
`factory-report`, or `factory-replay` from the shell.
