# Observability sinks (v1.0)

> **Status:** skeleton — filled in by stories S-1.8, S-1.9, S-4.1
> through S-4.7. Section bodies are deliberately empty until the
> story that ships each sink driver lands. Greppable `TODO(S-X.Y)`
> markers gate the 1.0.0 release per S-5.7 acceptance criteria.

The v1.0 dispatcher emits hook events through a configurable sink
pipeline. A single `observability-config.toml` declares any number of
sinks of any combination of backend types, each with independent
credentials, routing filters, retry behavior, and circuit-breaker
state. This guide is the operator reference for tuning that pipeline.

## Supported sink types

<!-- TODO(S-1.8, S-1.9, S-4.1, S-4.2, S-4.3): the headline table —
     name | transport | concurrent allowed | typical use. Cover file,
     http, otel-grpc, datadog, honeycomb. Note that operators can
     declare multiple instances of the same type with different tags. -->

## File sink

<!-- TODO(S-1.8): the default-on sink. Writes JSONL to
     `.factory/logs/events-YYYY-MM-DD.jsonl`. Same on-disk format as
     0.79.x for backward compat with existing Grafana / Loki configs.
     Configurable: directory, rotation policy, max-bytes-per-file. -->

## HTTP sink

<!-- TODO(S-4.1): generic POST-to-URL sink. Configurable: endpoint,
     headers, batch size, JSON-vs-NDJSON encoding, retry policy. -->

## OTel-gRPC sink

<!-- TODO(S-1.9): OpenTelemetry traces over gRPC. Configurable:
     endpoint, headers (auth tokens), service name, attribute mapping
     from event fields. Beta.1 scope. -->

## Datadog sink

<!-- TODO(S-4.2): native Datadog logs intake. Configurable: site
     (US1/US3/US5/EU/...), API key (env-var-resolved), service tag,
     source tag, custom tags. Multiple Datadog sinks supported (e.g.
     dev account + prod account). -->

## Honeycomb sink

<!-- TODO(S-4.3): Honeycomb events API. Configurable: dataset,
     API key, team. Multiple instances supported (e.g. write to two
     environments concurrently). -->

## Multi-sink recipes

<!-- TODO(S-4.6): worked examples for common operator scenarios —
     "local Grafana + remote Datadog", "two Datadog accounts", "all
     events to file + filter critical events to Honeycomb". Show the
     `observability-config.toml` for each. -->

## Resilience tuning

<!-- TODO(S-4.4, S-4.5): per-sink retry config (max attempts, base
     delay, jitter), circuit breaker (open/half-open thresholds,
     cool-down), dead-letter queue location and replay procedure.
     Defaults documented + when to override them. -->

## Routing filters and tag enrichment

<!-- TODO(S-4.6): how to send subset of events to a particular sink
     (e.g. only `commit.made` to Honeycomb), and how to add static
     or computed tags per sink (e.g. `env=prod`, `factory_name=...`). -->
