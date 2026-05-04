---
document_type: story
level: ops
story_id: "S-T.06"
epic_id: "E-10"
version: "1.0"
status: draft
producer: story-writer
timestamp: 2026-05-04T00:00:00Z
phase: 2
inputs:
  - .factory/specs/architecture/decisions/ADR-015-single-stream-otel-schema.md
  - .factory/stories/epics/E-10-single-stream-otel-event-emission.md
input-hash: ""
traces_to: .factory/stories/epics/E-10-single-stream-otel-event-emission.md
functional_requirements: []
cycle: v1.0-brownfield-backfill
points: "5"
wave: 17
depends_on: ["S-T.05"]
blocks: ["S-T.07"]
behavioral_contracts: []
# BC status: pending PO authorship
verification_properties: []
priority: "P1"
tdd_mode: strict
target_module: ".factory/obs/"
subsystems: ["SS-03"]
estimated_days: 2
assumption_validations: ["OQ-5"]
risk_mitigations: []
anchored_adr: ADR-015
---

# S-T.06: ADR-015 Wave 3 — Consumer migration (Grafana + tools + OTel collector)

**Epic:** E-10 — Single-stream OTel-aligned event emission (ADR-015)
**Wave:** Wave 3
**Depends on:** S-T.05
**Blocks:** S-T.07
**Estimated effort:** M (5 pts, ~2 days)

## Narrative

- **As a** vsdd-factory operator monitoring Claude Code sessions
- **I want** all Grafana panels and CLI tools updated to use the new reverse-DNS
  event names and `event.category` filters
- **So that** the `pr_throughput` panel and other domain panels show real data and
  the `unknown_category_events` panel alerts on uncategorized events

## Acceptance Criteria

### AC-001: pr_throughput Grafana panel returns at least one row within 24h (traces to ADR-015 Wave 3 §Acceptance criterion 1)

The Grafana panel named `pr_throughput` is rewritten to query `vsdd.pr.created.v1`
(or the appropriate new reverse-DNS name) with `event.category = "domain"` filter.
Within 24 hours of this story merging to `main`, the panel returns at least one row.

Zero rows = migration is broken. Wave 4 (S-T.08) is BLOCKED until this passes.

**Falsifiable test:** The Grafana panel JSON is updated. Manual/automated check
confirms non-zero data within 24h of merge. This is the Wave 3 hard gate.

### AC-002: unknown_category_events Grafana panel exists with WARN alert (traces to ADR-015 Wave 3 §Acceptance criterion 2)

A Grafana panel named `unknown_category_events` MUST exist with:
- A query for events where `event.category = "unknown"` in a 1-hour window.
- A non-zero WARN-severity alert configured: fires when `event.category = "unknown"`
  count > 0 in any 1-hour window.

Wave 3 CANNOT close without this panel and alert deployed.

**Falsifiable test:** The Grafana dashboard JSON file contains a panel with name
`unknown_category_events` and an alert rule targeting `event.category = "unknown"`.

### AC-003: All Grafana panel queries updated to reverse-DNS event names (traces to ADR-015 Wave 3 §Grafana dashboard rewrite)

Every Grafana panel that previously queried old event names (catalogued in
S-T.01's `adr015-wave0-grafana-query-inventory.md`) is updated to the corresponding
`vsdd.*.*\.v1` name. `event.category = "domain"` filter is added to panels that
previously assumed the file contained only domain events.

**Falsifiable test:** `grep -r '"pr.opened"\|"open_to_merge_seconds"' .factory/obs/`
(or equivalent dashboard path) returns zero hits after this story merges.

### AC-004: bin/factory-* tools filter on event.category = "domain" by default (traces to ADR-015 D-15.1 §Consumer fan-out)

All `bin/factory-*` CLI tools (`factory-query`, `factory-replay`, `factory-sla`,
`factory-report`, `factory-dashboard`) default to filtering events by
`event.category = "domain"` when reading `events-*.jsonl`. Lifecycle and audit
events are excluded from default output unless `--include-lifecycle` or similar
flag is provided.

**Falsifiable test:** Running `bin/factory-query` against a test `events-*.jsonl`
containing both lifecycle and domain events outputs ONLY domain events without flags.

### AC-005: OTel collector config unchanged for filelog receiver (traces to ADR-015 D-15.1 §Consumer fan-out + Wave 3)

The OTel collector's `filelog` receiver config pointing at `events-*.jsonl` requires
NO path or glob changes — it already reads all events from the single stream. Any
OTTL transform for semconv field mapping is added if needed.

**Falsifiable test:** The existing OTel collector configuration continues to load
without errors after Wave 3 changes. If OTTL transforms are added, they are present
in the collector config file.

### AC-006: Dual-emit shims removed (traces to ADR-015 Wave 3 §Remove dual-emit shims)

After Grafana queries are updated to new event names, the dual-emit shims installed
in S-T.05 are removed from all 11 plugins. Each plugin now emits ONLY the new
reverse-DNS name. Old names are no longer emitted.

**Falsifiable test:** `grep -r "pr\.created\b\|commit\.made\b" crates/hook-plugins/`
(old bare names without reverse-DNS prefix) returns zero hits after shim removal.

> **Pre-shim-removal gate (per ADR-015 Wave 3):** Operator MUST audit all dashboard
> queries against the deprecation registry before shim removal. Any query still keyed
> on an old name becomes permanently silent after this step. This gate is documented
> in S-T.07.

## Architecture Mapping

| Component | Module | Pure/Effectful | ADR-015 Reference |
|-----------|--------|---------------|-------------------|
| Grafana dashboard JSON files | `.factory/obs/` or equivalent | Pure (configuration) | Wave 3 |
| `bin/factory-*` CLI tools | `bin/` | Effectful (file read + filter) | D-15.1 Consumer fan-out |
| OTel collector config | `.factory/obs/otel-collector*.yaml` | Pure (configuration) | Wave 3 |
| Dual-emit shim removal | `crates/hook-plugins/*/src/lib.rs` | Code change | Wave 3 shim removal |

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Grafana dashboard files stored outside `.factory/obs/` (OQ-5 scope) | Implementer locates actual path from S-T.01 inventory; OQ-5 must be resolved before Wave 3 can proceed |
| EC-002 | `pr_throughput` panel returns zero after 24h despite merge | Wave 3 is BROKEN; S-T.07 and S-T.08 are BLOCKED; requires rollback investigation |
| EC-003 | `bin/factory-*` tools don't support `event.category` filtering | Add filtering capability; this is a required behavioral change per D-15.1 |
| EC-004 | Post-shim-removal: a plugin still emitting old name | `event.category = "unknown"` fires in the `unknown_category_events` alert (AC-002 safety net) |
| EC-005 | Dashboard JSON is generated by a CI tool and overwritten | Coordinate with the dashboard generation workflow; changes must survive CI regeneration |

## Tasks

- [ ] T-0: STOP CHECK — confirm S-T.05 merged; confirm Wave 2 dual-emit is active and panels show data with old names
- [ ] T-1: Operator pre-shim-removal audit: review S-T.01 inventory against all current dashboard queries
- [ ] T-2: Update all Grafana panel queries to reverse-DNS event names (vsdd.pr.created.v1, etc.)
- [ ] T-3: Add `event.category = "domain"` filters to all domain-event panels
- [ ] T-4: Create `unknown_category_events` panel with WARN alert (AC-002 hard gate)
- [ ] T-5: Update `bin/factory-*` tools to default-filter on `event.category = "domain"`
- [ ] T-6: Review OTel collector config; add OTTL transforms if needed (AC-005)
- [ ] T-7: Remove dual-emit shims from all 11 plugins (after S-T.07 deprecation announcement is ready to go)
- [ ] T-8: Run integration test: `pr_throughput` panel query returns data (AC-001 gate)
- [ ] T-9: Verify `unknown_category_events` panel and alert exist in dashboard JSON (AC-002 gate)
- [ ] T-10: Verify `grep` for old bare event names returns zero hits (AC-006 falsifiable test)

## Previous Story Intelligence

S-T.05 installed dual-emit shims and fixed the three content-defect bugs. Use the
S-T.01 `adr015-wave0-grafana-query-inventory.md` as the definitive list of panels to
update. The `pr.opened` panel is the primary broken panel documented in S-T.01 AC-001;
this story fixes it on the consumer side.

OQ-5 (Grafana migration scope and ownership) must be answered before this story is
dispatched to an implementer. The story spec acknowledges the open question but
proceeds under the assumption that dashboard JSON files exist in `.factory/obs/` or
a documented path.

## Architecture Compliance Rules

Per ADR-015 Wave 3:
- AC-001 (pr_throughput panel ≥1 row in 24h) is a HARD GATE. S-T.08 is blocked
  until this passes.
- AC-002 (`unknown_category_events` panel + WARN alert) is a HARD GATE. Wave 3
  cannot close without it.
- Shim removal MUST happen after the operator pre-audit (T-1) and after S-T.07's
  deprecation announcement is prepared.

## Library and Framework Requirements

| Library | Version | Notes |
|---------|---------|-------|
| Grafana (dashboard JSON format) | version per factory-obs stack | No code dependency; JSON editing |
| OTel Collector OTTL | version per factory-obs stack | If OTTL transforms are added |

## File Structure Requirements

| File | Action | Notes |
|------|--------|-------|
| `.factory/obs/*.json` (Grafana dashboard files) | MODIFY | Updated panel queries |
| `.factory/obs/otel-collector*.yaml` | MODIFY | OTTL transforms if needed |
| `bin/factory-query`, `factory-replay`, etc. | MODIFY | Add event.category default filter |
| `crates/hook-plugins/*/src/lib.rs` (11 plugins) | MODIFY | Remove dual-emit shims |

## Token Budget Estimate

| Context Source | Estimated Tokens |
|----------------|-----------------|
| This story spec | ~6,000 |
| ADR-015 Wave 3 section | ~3,000 |
| Grafana dashboard JSON files | ~5,000 |
| S-T.01 inventory file | ~3,000 |
| bin/factory-* tool sources | ~4,000 |
| 11 plugin sources (shim removal only) | ~5,000 |
| **Total** | **~26,000** |

~13% of a 200k context window. Within budget.

## CHANGELOG

| Version | Date | Change |
|---------|------|--------|
| v1.0 | 2026-05-04 | Initial authoring from ADR-015 Wave 3 decomposition. |
