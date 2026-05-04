---
document_type: story
level: ops
story_id: "S-T.08"
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
depends_on: ["S-T.07"]
blocks: ["S-T.09"]
behavioral_contracts: []
# BC status: pending PO authorship
verification_properties: []
priority: "P1"
tdd_mode: strict
target_module: "bin/"
subsystems: ["SS-01", "SS-07"]
estimated_days: 2
assumption_validations: []
risk_mitigations: []
anchored_adr: ADR-015
---

# S-T.08: ADR-015 Wave 4 — Bash hook parity (bin/emit-event schema alignment)

**Epic:** E-10 — Single-stream OTel-aligned event emission (ADR-015)
**Wave:** Wave 4
**Depends on:** S-T.07
**Blocks:** S-T.09
**Estimated effort:** M (5 pts, ~2 days)

## Narrative

- **As a** vsdd-factory operator using bash hooks that call `bin/emit-event`
- **I want** bash-sourced events to carry the same Resource attributes and
  per-event identity fields as WASM plugin events
- **So that** dashboard queries can join bash events with WASM events without
  losing the project/worktree/trace context that ADR-015 requires

## Acceptance Criteria

### AC-001: bin/emit-event adds all Resource attributes before writing (traces to ADR-015 D-15.3 §Bash hook parity)

`bin/emit-event` (or an equivalent enhanced binary) reads the Resource attributes
that the dispatcher would stamp at startup and includes them in every event it writes
to `events-*.jsonl`. The minimum required Resource attributes are:

`service.name` | `service.namespace` | `service.instance.id` | `service.version` |
`deployment.environment.name` | `host.name` | `host.id` (cascade) | `os.type` |
`process.pid` | `vcs.repository.url.full` | `vcs.repository.name` | `vcs.owner.name` |
`vcs.provider.name` | `worktree.id` | `project.id` | `schema_url`

All fallback cascade logic from S-T.03 applies equally to bash events.

**Falsifiable test:** A test bash hook invocation via `bin/emit-event` produces a
record in `events-*.jsonl` that contains all 16 Resource fields listed above with
non-empty values.

### AC-002: bin/emit-event adds per-event identity fields (traces to ADR-015 D-15.3 §Bash hook parity + D-15.2 Per-event attributes)

`bin/emit-event` stamps the same per-event identity fields that the WASM dispatcher
stamps at `host::emit_event` time:

`timestamp` | `observed_timestamp` | `event.id` (UUIDv4) | `event.category` |
`event.schema_url` | `event.source = "bash-adapter"` | `severity_number` |
`severity_text` | `trace_id` (from `VSDD_TRACE_ID` env if set) |
`span_id` (new UUIDv4 per invocation) | `parent_span_id` (from `VSDD_PARENT_SPAN_ID` env)

**Falsifiable test:** A test bash event record contains all fields listed above.
`event.source` is exactly `"bash-adapter"` (this marker identifies thin events
during the migration window per ADR-015 D-15.3).

### AC-003: Bash hooks route through host::emit_event path (preferred) OR bin/emit-event is enhanced (traces to ADR-015 D-15.3 §Bash hook parity alternative)

Two implementation paths are acceptable per ADR-015:
1. **Preferred:** Bash hooks that emit events are routed through the dispatcher's
   `host::emit_event` path (native WASM port track).
2. **Acceptable:** `bin/emit-event` is enhanced to add all Resource + per-event fields.

The implementer chooses the path. If path 1 is chosen, the bash hook must call the
dispatcher rather than writing directly to the JSONL file. If path 2 is chosen, the
bash binary becomes feature-equivalent to the WASM host enrichment.

**Falsifiable test:** After this story merges, bash hook events in `events-*.jsonl`
contain a non-empty `service.namespace` field (impossible with the old thin format).

### AC-004: event.source = "bash-adapter" marker identifies thin events during migration (traces to ADR-015 D-15.3 §Bash hook parity)

Until all bash hooks are ported to native WASM, `bin/emit-event`-sourced events
carry `event.source = "bash-adapter"`. This allows dashboards to identify and
optionally exclude thin events during the migration window.

**Falsifiable test:** All events written by `bin/emit-event` (before this story's
enhancement lands) had `event.source = "bash-adapter"` OR were missing the field
entirely. After this story, enhanced bash events carry `event.source = "bash-adapter"`
AND the full Resource attribute set.

### AC-005: bin/emit-event VSDD_TRACE_ID and VSDD_PARENT_SPAN_ID propagation (traces to ADR-015 D-15.4 §Trace propagation)

`bin/emit-event` reads `VSDD_TRACE_ID` from the environment (if set) and uses it
as the `trace_id` for the emitted event. `VSDD_PARENT_SPAN_ID` is read and used as
`parent_span_id`. This allows bash events to participate in the dispatcher's
cross-boundary trace chain.

**Falsifiable test:** Setting `VSDD_TRACE_ID=test-trace-001` before invoking
`bin/emit-event` results in an event record with `trace_id = "test-trace-001"`.

### AC-006: Legacy bash hooks do not regress (traces to ADR-015 Wave 4 scope)

All existing bash hooks that previously called `bin/emit-event` continue to work
after Wave 4. The enhanced `bin/emit-event` is backward-compatible with existing
call sites — no existing bash hook requires modification.

**Falsifiable test:** Full test suite (including bats tests for bash hooks) passes
after Wave 4 changes.

## Architecture Mapping

| Component | Module | Pure/Effectful | ADR-015 Reference |
|-----------|--------|---------------|-------------------|
| `bin/emit-event` | `bin/emit-event` (bash or Rust binary) | Effectful (file write) | Wave 4; D-15.3 Bash hook parity |
| Bash hooks (existing) | `plugins/vsdd-factory/hooks/*.sh` | Effectful (call emit-event) | Wave 4 — backward compat required |
| `host::emit_event` routing (preferred path) | `crates/factory-dispatcher/src/main.rs` | Effectful | D-15.3 preferred path |

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `bin/emit-event` called outside a dispatcher context (no VSDD_TRACE_ID) | Generates a new per-invocation trace_id (same as WASM dispatcher behavior when no VSDD_TRACE_ID is set) |
| EC-002 | Bash hook runs in environment without CLAUDE_PROJECT_DIR set | `service.namespace` falls back to `basename(cwd)` (same cascade as S-T.03 EC-001) |
| EC-003 | `bin/emit-event` was deleted in commit 818fb95 (per ADR-015 Context) | Restore and enhance; or implement the preferred routing-through-dispatcher path |
| EC-004 | Bash hook already sets `event.source` to a custom value | `event.source = "bash-adapter"` overrides the plugin-supplied value (host wins per D-15.3); `event.host_overrides` is stamped |

## Tasks

- [ ] T-0: STOP CHECK — confirm S-T.07 Wave 3 announcement delivered; confirm Wave 3 hard gates passed
- [ ] T-1: Determine if `bin/emit-event` was deleted (commit 818fb95); if so, restore or implement preferred routing path
- [ ] T-2: Choose implementation path (preferred: route bash hooks through dispatcher; acceptable: enhance bin/emit-event)
- [ ] T-3: Implement Resource attribute collection in chosen path (reuse S-T.03 cascade logic)
- [ ] T-4: Implement per-event identity field stamping (event.id UUIDv4, timestamps, category lookup)
- [ ] T-5: Implement `event.source = "bash-adapter"` marker
- [ ] T-6: Implement VSDD_TRACE_ID / VSDD_PARENT_SPAN_ID propagation from env
- [ ] T-7: Write test: bash event contains all 16 Resource fields (AC-001)
- [ ] T-8: Write test: bash event contains all per-event identity fields (AC-002)
- [ ] T-9: Write test: VSDD_TRACE_ID propagation (AC-005)
- [ ] T-10: Run full bats test suite; confirm backward compat (AC-006)

## Previous Story Intelligence

S-T.03 implemented the Resource attribute cascade for the WASM dispatcher. The same
cascade logic should be reused (extracted to a shared function or reimplemented) in
`bin/emit-event`. Confirm with the S-T.03 implementer whether a shared library is
available or if bash replication is the simpler path.

S-T.07 confirmed Wave 3 hard gates passed. The fact that bash events are now the
only remaining source of thin events makes this story the final parity gap to close.

## Architecture Compliance Rules

Per ADR-015 Wave 4 and D-15.3:
- `event.source = "bash-adapter"` is the official marker for bash-sourced events
  until they are ported to native WASM.
- Resource attribute cascades from S-T.03 apply equally to bash events.
- The preferred path (routing through `host::emit_event`) is architecturally
  superior to a standalone binary because it eliminates the duplicate enrichment
  code path.

**Forbidden:** Bash events that carry `event.source = "bash-adapter"` but are missing
Resource attributes. Either all Resource attributes are present OR the `bash-adapter`
marker clearly identifies them as thin (not both conditions true simultaneously post-Wave-4).

## Library and Framework Requirements

| Library | Version | Notes |
|---------|---------|-------|
| `uuid` (if Rust binary) | workspace | UUIDv4 for event.id and span_id |
| bash standard library | system | If bin/emit-event remains a bash script |
| `hostname`, `date`, platform commands | system | Resource cascade implementation in bash |

## File Structure Requirements

| File | Action | Notes |
|------|--------|-------|
| `bin/emit-event` | MODIFY or CREATE | Enhanced with Resource + per-event fields; backward-compatible |
| `plugins/vsdd-factory/hooks/*.sh` | NO CHANGE (AC-006 backward compat) | Existing bash hooks unchanged |
| `tests/bash/emit-event-*.bats` | CREATE | bats tests for enhanced emit-event |

## Token Budget Estimate

| Context Source | Estimated Tokens |
|----------------|-----------------|
| This story spec | ~5,500 |
| ADR-015 Wave 4 + D-15.3 bash parity | ~3,000 |
| S-T.03 enrichment implementation (reference) | ~4,000 |
| `bin/emit-event` source | ~2,000 |
| bash hook sources (scan) | ~3,000 |
| Test file context | ~3,000 |
| **Total** | **~20,500** |

Well within 20% of a 200k context window.

## CHANGELOG

| Version | Date | Change |
|---------|------|--------|
| v1.0 | 2026-05-04 | Initial authoring from ADR-015 Wave 4 decomposition. |
