---
name: factory-obs
description: Manage the local observability stack (OTel Collector + Loki + Prometheus + Grafana + image-renderer) that visualizes vsdd-factory hook events and Claude Code OTel telemetry. Starts/stops/resets the Docker stack, manages the multi-factory watched-list registry (register/unregister/list), and opens the Grafana dashboard. Use when the user asks to start/stop the observability stack, register a project to be watched, list registered factories, or open the Grafana dashboards. Opt-in, local-only, no cloud services.
argument-hint: "[up|down|reset|status|logs|dashboard|register|unregister|list|regenerate|help]"
allowed-tools: Bash
---

# Factory Observability

Launch, stop, reset, or inspect the local Docker observability stack —
a 5-service stack (OTel Collector + Loki + Prometheus + Grafana +
Grafana Image Renderer) that ingests `.factory/logs/events-*.jsonl`
into Loki and Claude Code's native OTel metrics into Prometheus, and
surfaces the data in 7 preconfigured Grafana dashboards.

Since v0.78.0 the stack can watch **multiple factory projects** at
once. Each project is registered explicitly via a user-level registry
at `~/.config/vsdd-factory/watched-factories`. `factory-obs up`
generates a `docker-compose.override.yml` from that registry with one
bind mount per factory (each at `/var/log/factory/<safe-name>/`), and
the collector globs `/var/log/factory/*/events-*.jsonl` so every
registered factory feeds the same Loki.

The stack is entirely local and opt-in — nothing is emitted off-box. If
Docker isn't running or isn't installed, the skill surfaces the error
and stops; it never attempts to fall back to cloud services.

## Announce at Start

Before any other action, say verbatim:

> I'm using the factory-obs skill to manage the local observability stack.

## Arguments

The skill accepts one positional argument matching the `factory-obs`
subcommand. If no argument is given, default to `up` and then print the
Grafana URL.

| Arg | Effect |
|---|---|
| `up` (default) | Regenerate `docker-compose.override.yml` from the registry, then start the stack. Prints Grafana URL when healthy. |
| `down` | Stop the stack, keep volumes (preserves event offsets + dashboards). |
| `reset` | Stop and wipe volumes. Use when the collector is misbehaving. |
| `status` | `docker compose ps` — container health snapshot. |
| `logs` | Tail collector + Grafana logs (useful for debugging ingestion). |
| `dashboard` | Print the Grafana URL; open a browser if running interactively. |
| `register [PATH]` | Add a factory project to the watched-list registry. Default: walk up from `cwd` to find the nearest `.factory/` ancestor. |
| `unregister [PATH]` | Remove a factory project from the registry. Same `cwd` autoresolve as `register`. |
| `list` (or `registered`) | Print all registered factories with their safe-names and filesystem status. |
| `regenerate` | Rewrite `docker-compose.override.yml` from the registry without (re)starting the stack. Mostly for scripting / CI / debugging. |
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

Users may already have `3000`/`3100`/`4318`/`9090`/`8081` bound by
another service. The binary honors these env vars (names documented in
`factory-obs help`):

- `VSDD_OBS_GRAFANA_PORT` — Grafana UI port (default 3000)
- `VSDD_OBS_LOKI_PORT` — Loki HTTP port (default 3100)
- `VSDD_OBS_OTLP_HTTP_PORT` — OTLP HTTP receiver port (default 4318)
- `VSDD_OBS_PROMETHEUS_PORT` — Prometheus UI/API port (default 9090)
- `VSDD_OBS_RENDERER_PORT` — Grafana Image Renderer port (default 8081)
- `VSDD_FACTORY_LOGS` — single-path fallback for the logs dir when the
  registry is empty (legacy; prefer `register`)
- `VSDD_OBS_REGISTRY` — override the registry file location (primarily
  for bats tests; real users should leave this alone)
- `VSDD_OBS_OPEN_BROWSER` — `1` to force, `0` to suppress, unset for
  TTY auto-detect

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
- `register` — user starts working on a NEW factory project and wants
  the existing stack to include its events. Run from inside the new
  project with no args, then `factory-obs up` to apply.
- `unregister` — user no longer wants to capture events from a
  factory they previously registered (archived project, etc.).
- `list` — audit time — "what factories is this stack watching?"
- `regenerate` — someone edited the registry file by hand and wants
  to preview the resulting `docker-compose.override.yml` before
  starting the stack.

## Non-goals

This skill **does not**:
- Modify the base compose file or collector config (edits go through
  normal PR flow). The *override* file is regenerated each run — that
  one IS modified, but it's gitignored and machine-local.
- Start Claude Code's native OTel telemetry — that's orthogonal; use
  the `/vsdd-factory:claude-telemetry` skill for that.
- Run without Docker — no cloud fallback by design.
- Emit events itself — use `emit-event` or let hooks fire naturally.
- Scan the filesystem for factories automatically — registration is
  always explicit. This is deliberate: the plugin can be installed in
  projects anywhere on the filesystem, and we don't assume any parent
  directory (`~/Dev`, `~/work`, etc.).

For querying the ingested data without Grafana, use `factory-query`,
`factory-report`, or `factory-replay` from the shell.
