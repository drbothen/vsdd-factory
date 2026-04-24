# Local observability stack

Opt-in Docker stack that renders the vsdd-factory event log in a local
Grafana dashboard.

**This stack is optional.** vsdd-factory emits structured events to
`.factory/logs/events-*.jsonl` with or without this stack running. Use
`factory-query` / `factory-report` / `factory-dashboard` (all shipped
under `bin/`) for CLI-based views that don't need Docker.

## Quickstart

From your project root (the directory containing `.factory/`):

```bash
# Start the stack (background)
"${CLAUDE_PLUGIN_ROOT}/bin/factory-obs" up

# Wait ~10 seconds, then open the dashboard
"${CLAUDE_PLUGIN_ROOT}/bin/factory-obs" dashboard

# Stop when done (preserves volumes)
"${CLAUDE_PLUGIN_ROOT}/bin/factory-obs" down
```

Default URLs:

| Service | URL | Notes |
|---------|-----|-------|
| Grafana UI | http://localhost:3000 | Anonymous admin, or admin/admin |
| Loki API | http://localhost:3100 | Log query endpoint |
| Prometheus | http://localhost:9090 | Metrics (Claude token usage, costs) |
| OTLP HTTP | http://localhost:4318 | For Claude Code native OTel, if enabled |

Override any of these via env vars:
- `VSDD_OBS_GRAFANA_PORT=8080 factory-obs up`
- `VSDD_OBS_PROMETHEUS_PORT=19090 factory-obs up`

## Watching multiple factories (v0.78.0+)

The stack can aggregate events from any number of factory projects,
wherever they live on the filesystem. Each project is registered
explicitly via a user-level registry at
`~/.config/vsdd-factory/watched-factories`:

```bash
# Register the factory whose root is the current directory
cd ~/Dev/prism
factory-obs register

# Or register an absolute path from anywhere
factory-obs register /opt/work/other-project

# See what's registered
factory-obs list

# Stop watching one
factory-obs unregister /opt/work/other-project

# Apply changes (generates docker-compose.override.yml from the
# registry and (re)starts the collector)
factory-obs up
```

Each registered factory lands at `/var/log/factory/<sanitized-name>-<hash>/`
inside the collector container, and the filelog receiver globs all
subdirectories. The basename is derived from the project root's final
path component; the 8-char hash suffix disambiguates projects that
share a basename but live in different parents.

If the registry is empty when you run `factory-obs up`, the stack
falls back to watching the current directory (if it's a factory root)
or the path given by `VSDD_FACTORY_LOGS` — same behavior as earlier
releases, single-factory.

## Architecture

```
.factory/logs/events-*.jsonl   ← hooks emit here (phases 1–2)
           │
           │  (mounted read-only)
           ▼
   ┌──────────────────┐
   │ otel-collector   │  tails files, parses JSON, ships to Loki
   │ :4318 OTLP HTTP  │
   └────────┬─────────┘
            │
            ▼
   ┌──────────────────┐
   │ Loki             │  stores structured logs
   │ :3100            │
   └────────┬─────────┘
            │
            ▼
   ┌──────────────────┐
   │ Grafana          │  queries Loki, renders dashboards
   │ :3000            │
   └──────────────────┘
```

No metrics backend in this minimal stack — Loki's LogQL `count_over_time`
is sufficient for all current charts. A Prometheus/Mimir container can
be added in a future release if we need true time-series metrics.

## What ships

- **`docker-compose.yml`** — 3 services (otel-collector + loki + grafana).
  Minimal, no cloud dependencies.
- **`otel-collector-config.yaml`** — tails `.factory/logs/events-*.jsonl`,
  parses JSON, promotes `type`/`hook`/`reason`/`severity` to resource
  attributes (which become Loki stream labels), exports to Loki.
- **`grafana-provisioning/`** — preconfigured Loki datasource and
  dashboard provider.
- **`grafana-dashboards/factory-overview.json`** — starter dashboard with:
  - Total events / Blocks (hard) / Blocks (warn) / Actions stat panels
  - Events over time stacked by type
  - Top block reasons table
  - Blocks by hook bar gauge
  - Recent events log stream

## Persistent state

- `grafana-data` volume — dashboard annotations, personal preferences.
- `loki-data` volume — ingested log chunks.
- `collector-state` volume — the file_storage extension's offset tracking
  so events aren't re-ingested on container restart.

All three survive `factory-obs down`. Use `factory-obs reset` to wipe.

## Troubleshooting

**Dashboard is empty.** Three common causes:
1. No events yet. Trigger a hook (run a dangerous command so the guards
   block it) or wait for normal activity.
2. `.factory/logs/` not mounted. Check `factory-obs status` — the
   `otel-collector` service should show `running`. Check logs with
   `factory-obs logs`.
3. Time window wrong. Grafana defaults to last 24h; if your events are
   older, widen the time picker.

**`factory-obs up` fails with "port already in use".** Another service
on your host is bound to the port. Override:
```bash
VSDD_OBS_GRAFANA_PORT=3001 VSDD_OBS_LOKI_PORT=3101 factory-obs up
```

**`.factory/logs/` is owned by root after running the stack.** The
collector mounts the dir read-only so this shouldn't happen. If you see
it, `factory-obs reset` + fix ownership with `sudo chown -R $USER
.factory/logs/`.

**Anonymous admin access in Grafana.** Deliberate — this stack is
local-only dev. If you plan to expose it beyond localhost, edit the
`GF_AUTH_*` env vars in `docker-compose.yml`.

## Testing without docker

`factory-obs` itself is a shell wrapper; testing it in CI requires
docker, which many environments don't provide. The shipped BATS suite
(`tests/factory-obs.bats`) tests only the parts that don't need docker
(argument parsing, help output, compose file location). Dashboard JSON
and collector config are validated separately via YAML/JSON parsers.

For full end-to-end validation, run the stack locally and hit it with
the emit-event helper:

```bash
factory-obs up
sleep 10
# Fire some events
for i in 1 2 3; do
  "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" type=hook.block \
    hook=smoke-test reason=local_test command="fake-command-$i"
done
# Refresh Grafana dashboard
```

## Uninstalling

```bash
factory-obs reset           # stop containers + wipe volumes
docker image rm \
  otel/opentelemetry-collector-contrib:0.149.0 \
  grafana/loki:3.6.10 \
  grafana/grafana:13.0.1 \
  prom/prometheus:v3.5.2    # optional: remove images too
```
