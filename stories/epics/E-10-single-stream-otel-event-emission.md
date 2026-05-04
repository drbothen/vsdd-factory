---
document_type: epic
epic_id: "E-10"
version: "1.0"
title: "Single-stream OTel-aligned event emission (ADR-015)"
status: draft
anchored_adr: ADR-015
prd_capabilities: [CAP-011, CAP-015]
prd_frs: []
anchor_strategy: adr-driven-migration
priority: P1
target_release: "v1.1"
story_count: 9
subsystems_affected: [SS-01, SS-03]
producer: story-writer
timestamp: 2026-05-04T00:00:00Z
phase: 2
traces_to: .factory/specs/architecture/decisions/ADR-015-single-stream-otel-schema.md
depends_on: []
last_amended: 2026-05-04
inputs:
  - .factory/specs/architecture/decisions/ADR-015-single-stream-otel-schema.md
input-hash: ""
---

# Epic E-10: Single-stream OTel-aligned event emission (ADR-015)

## Description

Implements the four normative decisions in ADR-015 (ACCEPTED 2026-05-04) across a
six-wave migration plan. ADR-015 supersedes ADR-005 (multi-sink fan-out) and amends
ADR-007 (always-on telemetry). The core problem: plugin-emitted events currently land
in `dispatcher-internal-YYYY-MM-DD.jsonl` alongside lifecycle noise; every downstream
consumer reads `events-YYYY-MM-DD.jsonl`, which is empty. Three known content-defect
bugs (pr.opened vs pr.created mismatch, plugin_version always emitting dispatcher
version, open_to_merge_seconds never emitted) are bundled into the migration.

The epic delivers:

1. **D-15.1** — Single physical stream: all events to `events-*.jsonl`; FileSink is
   the direct writer; Router/SinkRegistry/DlqWriter retired; `dispatcher-internal-*.jsonl`
   gated to `VSDD_DEBUG_LOG=1`.
2. **D-15.2** — OTel-aligned schema: Resource attributes stamped at startup;
   per-event identity fields stamped at emit time; `event.category` derived from a
   compile-time registry; `event.name` in reverse-DNS + `.vN` format.
3. **D-15.3** — Producer-side enrichment contract: host fields win; plugin overrides
   are surfaced via `vsdd.internal.host_field_override.v1` + stderr warning + inline
   `event.host_overrides`; block path gains `vsdd.block.plugin_blocked.v1` audit event.
4. **D-15.4** — Trace propagation: `VSDD_TRACE_ID` + `VSDD_PARENT_SPAN_ID` injected
   into every `exec_subprocess` call unconditionally (dispatcher-side invariants, not
   per-plugin env_allowlist).

## Scope

### In Scope

- Wave 0: read-only audit (Grafana panel inventory, field snapshot)
- Wave 1: host-side FileSink wiring + Resource-attribute enrichment + lifecycle event types
- Wave 2: plugin schema migration (all 11 affected WASM plugins) + dual-emit shims + bug-fix bundle
- Wave 3: Grafana consumer migration + dual-emit shim removal + deprecation announcement
- Wave 4: bash hook parity via `bin/emit-event` enhancement
- Wave 5: crate retirement (`sink-otel-grpc` deleted; `Router`/`SinkRegistry`/`DlqWriter` removed from `sink-core`)
- Three content-defect bug fixes: `pr.opened`→`pr.created` reconciliation,
  `plugin_version` fix, `open_to_merge_seconds` emission

### Out of Scope (Deferred to OQs)

- `observability-config.toml` schema post-migration (OQ-1, SS-03)
- Persistent deprecation registry artifact (OQ-9, SS-03)
- Atomic dual-emit host helper (OQ-8, SS-01)
- FileSink partial-write / atomic-rename recovery (OQ-7, SS-01)
- Windows fallback implementation beyond cascade terminal default (OQ-2)
- Operator-extensible `event-category-registry.toml` (D-15.2.a deferred)
- Tamper-evident audit log / WORM layer
- `factory-obs` / OTel Collector configuration changes

## PRD Capabilities Covered

| Capability ID | Name | Priority |
|--------------|------|----------|
| CAP-011 | Emit structured events to the observability stream | P0 |
| CAP-015 | Enrich events with OTel-aligned resource attributes | P1 |

> Note: CAP-011 and CAP-015 are the closest matches in the domain spec to ADR-015's
> subject matter. If the PRD capabilities do not yet enumerate these by these exact IDs,
> treat this section as aspirational; the story-writer anchored to the closest semantically
> matching capabilities. The ADR-015 reference is the authoritative anchor for this epic.

## Subsystem Anchors

- **SS-01 (Dispatcher Core):** owns `main.rs`, `host::emit_event`, FileSink wiring,
  exec_subprocess trace injection, Resource attribute stamping, per-event field stamping,
  event.category registry, lifecycle event emission.
- **SS-03 (Observability Sinks):** owns `sink-core`, `sink-file`, `sink-otel-grpc`,
  SS-03-observability-sinks.md spec, BC-3.* contracts; Wave 5 spec rewrite is SS-03 work.

## Migration Wave → Story Mapping

| Wave | Story | Description | Size |
|------|-------|-------------|------|
| Wave 0 | S-T.01 | Read-only audit: Grafana panel inventory + event field snapshot | S |
| Wave 1 | S-T.02 | FileSink single-stream wiring (D-15.1) | M |
| Wave 1 | S-T.03 | Resource-attribute enrichment (D-15.2 Resource fields) | M |
| Wave 1 | S-T.04 | Trace propagation + lifecycle event types (D-15.4 + D-15.3 internals) | M |
| Wave 2 | S-T.05 | Plugin schema migration + dual-emit shims + bug-fix bundle | L |
| Wave 3 | S-T.06 | Consumer migration: Grafana + bin/factory-* tools + OTel collector | M |
| Wave 3 | S-T.07 | Wave 3 sub-tasks: deprecation announcement + operator audit gate | S |
| Wave 4 | S-T.08 | Bash hook parity: bin/emit-event schema alignment | M |
| Wave 5 | S-T.09 | Crate retirement + SS-03 spec rewrite | M |

## Dependency Topology (Intra-epic)

```
S-T.01 (Wave 0) ──→ S-T.02 (Wave 1)
                  ──→ S-T.03 (Wave 1)
                  ──→ S-T.04 (Wave 1)

S-T.02 ──→ S-T.05 (Wave 2)
S-T.03 ──→ S-T.05
S-T.04 ──→ S-T.05

S-T.05 ──→ S-T.06 (Wave 3)
S-T.06 ──→ S-T.07 (Wave 3 sub-tasks)

S-T.07 ──→ S-T.08 (Wave 4)
S-T.08 ──→ S-T.09 (Wave 5)
```

## Open Questions (Escalation References)

| OQ ID | Scope | Description |
|-------|-------|-------------|
| OQ-1 | SS-03 | `observability-config.toml` minimal schema post-migration |
| OQ-2 | SS-01 | Platform support matrix for Wave 1 (Windows registry fallback vs stub) |
| OQ-5 | SS-03 | Grafana dashboard migration scope and ownership |
| OQ-7 | SS-01 | FileSink partial-write recovery semantics |
| OQ-8 | SS-01 | Atomic dual-emit host helper (deferred) |
| OQ-9 | SS-03 | Persistent deprecation registry artifact (post-Wave-3) |

## CHANGELOG

| Version | Date | Change |
|---------|------|--------|
| v1.0 | 2026-05-04 | Initial authoring from ADR-015 migration plan decomposition. |
