---
document_type: story
level: ops
story_id: "S-T.02"
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
depends_on: ["S-T.01"]
blocks: ["S-T.05"]
behavioral_contracts: []
# BC status: pending PO authorship
verification_properties: []
priority: "P1"
tdd_mode: strict
target_module: "crates/factory-dispatcher"
subsystems: ["SS-01", "SS-03"]
estimated_days: 2
assumption_validations: []
risk_mitigations: []
anchored_adr: ADR-015
---

# S-T.02: ADR-015 Wave 1 — FileSink single-stream wiring (D-15.1)

**Epic:** E-10 — Single-stream OTel-aligned event emission (ADR-015)
**Wave:** Wave 1
**Depends on:** S-T.01
**Blocks:** S-T.05
**Estimated effort:** M (5 pts, ~2 days)

## Narrative

- **As a** vsdd-factory operator
- **I want** all plugin-emitted events routed to `events-YYYY-MM-DD.jsonl` via a direct
  `FileSink::write` call in `host::emit_event`
- **So that** Grafana dashboards and `bin/factory-*` tools receive plugin events for
  the first time, immediately making previously-empty panels show data

## Acceptance Criteria

### AC-001: host::emit_event routes to FileSink directly (traces to ADR-015 D-15.1 §Implementation path)

`main.rs` `host::emit_event` calls `FileSink::write` directly on
`events-YYYY-MM-DD.jsonl`. The `Router::submit` / `SinkRegistry` path is NOT called
from the integration code path. The open TODO in `sinks/mod.rs` lines 11–15 is
resolved by this story (the TODO is removed or annotated as intentionally dead code).

**Falsifiable test:** An integration test writes an event via `host::emit_event` and
asserts `events-YYYY-MM-DD.jsonl` contains the event. The test fails if the file is
absent or empty.

### AC-002: dispatcher-internal gated on VSDD_DEBUG_LOG=1 (traces to ADR-015 D-15.1 §ADR-007 amendment)

`dispatcher-internal-YYYY-MM-DD.jsonl` is written ONLY when `VSDD_DEBUG_LOG=1` is set
in the process environment. In release builds without this variable, the debug file is
NOT created and NOT written.

**Falsifiable test:** Two integration test runs — one without `VSDD_DEBUG_LOG` and one
with `VSDD_DEBUG_LOG=1` — assert respectively: (a) `dispatcher-internal-*.jsonl` does
not exist; (b) `dispatcher-internal-*.jsonl` exists and contains at least the startup
lifecycle event.

### AC-003: FileSink write failure falls back to dispatcher-internal + stderr (traces to ADR-015 D-15.1 §FileSink write-failure semantics)

When `FileSink::write` returns an error for `events-*.jsonl` (simulated by pointing
the sink at a read-only or non-existent directory), the dispatcher:

1. Writes the failed event to `dispatcher-internal-YYYY-MM-DD.jsonl` unconditionally
   (regardless of `VSDD_DEBUG_LOG`).
2. Emits a `stderr` warning matching: `[vsdd-dispatcher] WARN: FileSink write failed
   for events-YYYY-MM-DD.jsonl (<error>); event written to dispatcher-internal-YYYY-MM-DD.jsonl as fallback.`
3. Does NOT silently swallow the failure.

**Falsifiable test:** A unit test injects a `FileSink` write error and asserts the
fallback file is written AND the stderr output matches the warning format above.

### AC-004: Router, SinkRegistry, DlqWriter excluded from default-members (traces to ADR-015 D-15.1 §Deprecation and retirement semantics)

The root `Cargo.toml` no longer lists `sink-otel-grpc` in `default-members`. The
`Router`, `SinkRegistry`, and `DlqWriter` types within `sink-core` are marked with
`#[deprecated]` or excluded from the public re-export path so no new production code
path calls them. They remain on disk (Wave 5 physically deletes them per D-15.1).

**Falsifiable test:** `cargo build` with default features succeeds without compiling
`sink-otel-grpc`. A `grep -r "Router::" crates/factory-dispatcher/src/` returns no
active call sites (only potentially dead `#[allow(dead_code)]` or `#[deprecated]`
annotated ones).

### AC-005: HostContext stub queue retired (traces to ADR-015 D-15.1 §Implementation path)

The stub event queue previously used in `HostContext` (holding events before the
Router was wired) is removed from the production code path. All event emission goes
directly via `FileSink::write`.

**Falsifiable test:** `grep -r "stub.*queue\|event.*buffer\|pending.*event" crates/factory-dispatcher/src/` returns no active production code hits (only test code or dead-code annotations).

### AC-006: observability-config.toml retains single-sink stanza (traces to ADR-015 D-15.1 §observability-config.toml)

`observability-config.toml` (if present) no longer contains multi-sink stanza entries.
It retains only: file sink path template, retention policy, and `VSDD_DEBUG_LOG` gate.

**Falsifiable test:** Parsing `observability-config.toml` with the updated schema
succeeds. An old config containing a `[[sinks]]` array with `type = "otel-grpc"`
causes a parse error or warning, not silent ignore.

## Architecture Mapping

| Component | Module | Pure/Effectful | ADR-015 Reference |
|-----------|--------|---------------|-------------------|
| `host::emit_event` | `crates/factory-dispatcher/src/main.rs` | Effectful (file write) | D-15.1 Implementation path |
| `FileSink` | `crates/sink-file/src/lib.rs` | Effectful (file I/O) | D-15.1 kept as single-stream writer |
| `Router` / `SinkRegistry` | `crates/sink-core/src/` | Effectful — to be excluded | D-15.1 retired (Wave 1 = deprecated) |
| `DlqWriter` | `crates/sink-core/src/` | Effectful — to be excluded | D-15.1 retired |
| `sink-otel-grpc` | `crates/sink-otel-grpc/` | Effectful — to be excluded from default-members | D-15.1 retired |
| Debug file writer | `crates/factory-dispatcher/src/` | Effectful (conditional) | D-15.1 gated on VSDD_DEBUG_LOG |

**Forbidden dependencies (build-time enforcement):** After this story merges, no
new `use` of `Router`, `SinkRegistry`, or `DlqWriter` from `factory-dispatcher`
`main.rs` or `lib.rs`. Any PR adding such a dependency MUST be blocked at code review.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `events-*.jsonl` directory doesn't exist at startup | `FileSink` creates the directory (or returns error caught by AC-003 fallback) |
| EC-002 | Disk full mid-write | Triggers AC-003 fallback path: fallback file written + stderr warning |
| EC-003 | Two dispatcher processes running simultaneously, both writing events-*.jsonl | File-level append is atomic on POSIX for small writes; no additional locking needed at this scale per ADR-015 rationale |
| EC-004 | `VSDD_DEBUG_LOG` set to non-"1" value (e.g. "true" or "yes") | Only the literal value `"1"` enables the debug file; other truthy strings do NOT (aligns with conventional `RUST_LOG` gating patterns) |

## Tasks

- [ ] T-0: STOP CHECK — confirm S-T.01 outputs exist in `.factory/measurements/` before proceeding
- [ ] T-1: Remove or annotate the TODO in `sinks/mod.rs` lines 11–15
- [ ] T-2: Wire `FileSink::write` call in `host::emit_event` in `main.rs`
- [ ] T-3: Gate `dispatcher-internal-*.jsonl` writes behind `VSDD_DEBUG_LOG=1` env check
- [ ] T-4: Implement FileSink write-failure fallback (unconditional debug file write + stderr warning)
- [ ] T-5: Remove `sink-otel-grpc` from `default-members` in root `Cargo.toml`
- [ ] T-6: Mark `Router`, `SinkRegistry`, `DlqWriter` as deprecated in `sink-core`; confirm no active call sites in `factory-dispatcher`
- [ ] T-7: Remove stub event queue from `HostContext` production code path
- [ ] T-8: Update `observability-config.toml` schema to remove multi-sink stanza
- [ ] T-9: Write integration test: event via `host::emit_event` → `events-*.jsonl` contains event
- [ ] T-10: Write integration test: no `VSDD_DEBUG_LOG` → `dispatcher-internal-*.jsonl` absent
- [ ] T-11: Write unit test: injected `FileSink` write error → fallback file + stderr warning

## Previous Story Intelligence

S-T.01 provides: confirmed list of all broken panel queries, field inventory, and
the three content-defect bug citations. Use the S-T.01 measurement files to verify
that events now appearing in `events-*.jsonl` are visible to the broken Grafana
panels (even if with old field names — that is Wave 3's concern).

## Architecture Compliance Rules

Per ADR-015 D-15.1:
- `sink-core` crate is KEPT. Only `Router`, `SinkRegistry`, `DlqWriter` types are retired.
- `sink-file` (FileSink) is KEPT and becomes the single-stream writer.
- `sink-otel-grpc` goes to DEPRECATED state in Wave 1 (not yet deleted; deletion is Wave 5).
- "Deprecated" = excluded from `default-members` + `publish = false` + not called from production code.
- TD-015-a (cargo-metadata CI check against retired-crate re-coupling) is deferred per ADR-015 D-15.1.
  This story does NOT implement the CI check; it is future work.

## Library and Framework Requirements

| Library | Version | Notes |
|---------|---------|-------|
| `sink-file` (internal) | workspace | Single-stream writer; keep as-is |
| `sink-core` (internal) | workspace | Keep crate; deprecate Router/SinkRegistry/DlqWriter types |
| `sink-otel-grpc` (internal) | workspace | Exclude from default-members; publish = false |

## File Structure Requirements

| File | Action | Notes |
|------|--------|-------|
| `crates/factory-dispatcher/src/main.rs` | MODIFY | Wire FileSink; gate debug file; add fallback |
| `crates/factory-dispatcher/src/sinks/mod.rs` | MODIFY | Remove/annotate TODO lines 11–15 |
| `Cargo.toml` (root) | MODIFY | Remove sink-otel-grpc from default-members; add publish=false |
| `crates/sink-core/src/` | MODIFY | Deprecate Router/SinkRegistry/DlqWriter types |
| `observability-config.toml` (if present) | MODIFY | Remove multi-sink stanza schema |
| `crates/factory-dispatcher/tests/` | CREATE | Integration tests for single-stream routing + debug gate + fallback |

## Token Budget Estimate

| Context Source | Estimated Tokens |
|----------------|-----------------|
| This story spec | ~5,000 |
| ADR-015 (D-15.1 section) | ~5,000 |
| main.rs (full read) | ~4,000 |
| sinks/mod.rs | ~1,000 |
| sink-core types | ~3,000 |
| Cargo.toml (root) | ~2,000 |
| Test output context | ~3,000 |
| **Total** | **~23,000** |

Well within 20% of a 200k-token context window.

## CHANGELOG

| Version | Date | Change |
|---------|------|--------|
| v1.0 | 2026-05-04 | Initial authoring from ADR-015 D-15.1 decomposition. |
