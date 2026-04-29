# AC-5 — Observability migration + Regenerating hooks-registry.toml

**AC statement:** Sections "Observability migration" + "Regenerating
hooks-registry.toml" populated. "Regenerating hooks-registry.toml" is
PRE-FILLED by S-0.05 skeleton — verify completeness only.

**Evidence type:** file snippets (2 sections)

## Observability migration (lines 127-146)

```markdown
## Observability migration

The default observability configuration in v1.0 writes the same JSONL
event log as v0.79.x — to `.factory/logs/dispatcher-internal.jsonl`.
If you have existing dashboards (Grafana, custom scripts, log-tailing
workflows) reading that file, they continue working with no changes.

To add additional sinks or modify behavior, create
`observability-config.toml` in your factory root. Example additions:

- **Datadog:** add a `[sink.datadog]` stanza with your API key and site.
- **Honeycomb:** add a `[sink.honeycomb]` stanza with your API key and
  dataset name.
- **OTel-grpc:** add a `[sink.otel_grpc]` stanza with your collector
  endpoint.

Multiple sinks can run simultaneously — the dispatcher fans events out
to all configured sinks.
```

## Regenerating hooks-registry.toml — PRE-FILLED section (lines 246-283)

```markdown
## Regenerating `hooks-registry.toml`

The v1.0 dispatcher reads `plugins/vsdd-factory/hooks-registry.toml`
to decide which hooks fire on which events. During the v0.79.x → v1.0
migration the file is produced by a generator that reads the historical
bash-hook inventory at `git show 7b4b774^:plugins/vsdd-factory/hooks/hooks.json`
and emits one `[[hooks]]` entry per bash hook, all routed through
`legacy-bash-adapter.wasm`.

Run the generator from the repo root:

```bash
scripts/generate-registry-from-hooks-json.sh
```

The script is idempotent — re-running it on an unchanged input
produces byte-identical output. CI re-runs it on every push and fails
the build if `git diff plugins/vsdd-factory/hooks-registry.toml` is
non-empty.
```

## Commentary

"Observability migration" section was a TODO block — now filled with
Datadog/Honeycomb/OTel-grpc guidance matching v0.79.x default
compatibility. "Regenerating hooks-registry.toml" was PRE-FILLED by
S-0.05; content confirmed complete and unmodified.
