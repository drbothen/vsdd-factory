---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: architect
timestamp: 2026-05-04T00:00:00Z
phase: 1.2-rev
inputs:
  - .factory/specs/architecture/decisions/ADR-015-single-stream-otel-schema.md
  - .factory/specs/architecture/SS-03-event-emission.md
input-hash: "[pending-recompute]"
traces_to: ADR-015-single-stream-otel-schema.md
origin: spec-revision
subsystem: "SS-01"
capability: "CAP-TBD"
lifecycle_status: active
introduced: v1.1.0
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# Behavioral Contract BC-1.11.002: factory-dispatcher::file_sink::partial_write_recovery — boundary-marker strategy for JSONL partial-write detection and write-failure cascade

## Description

`FileSink` is the sole writer of `events-YYYY-MM-DD.jsonl` after ADR-015
D-15.1. This BC governs three related invariants:

1. **Partial-write detection strategy:** `FileSink` uses boundary-marker
   semantics — a truncated final line (resulting from a crash mid-write) is
   detectable by consumers as a JSON parse error on the last line, and MUST
   be tolerated as a non-fatal artifact. All prior complete lines in the file
   are valid JSONL.

2. **Write-failure cascade:** When `FileSink::write` returns an error, the
   dispatcher MUST attempt an unconditional fallback write to the debug file
   (`dispatcher-internal-YYYY-MM-DD.jsonl`) and emit a `stderr` warning.
   Silent swallowing of write failures is prohibited.

3. **fsync policy:** `FileSink` does NOT call `fsync` by default. An opt-in
   `sync_on_write` config flag enables per-write `fsync` for operators with
   durability requirements. This is the implementation contract for the
   fsync policy defined in SS-03-event-emission.md.

This BC resolves OQ-7 from ADR-015.

## Preconditions

### Partial-write recovery
1. `events-YYYY-MM-DD.jsonl` exists and contains at least one previously
   written complete JSONL line.
2. A write attempt was interrupted (process killed, OS crash, I/O error)
   after some bytes of the new event were written but before the terminating
   `\n` was written or flushed.

### Write-failure cascade
1. `FileSink::write` returns an `Err(...)` for any reason: disk full,
   permission denied, read-only filesystem, I/O error.
2. The dispatcher is still running (the error is returned to the caller,
   not a crash condition).

## Postconditions

### Partial-write recovery (consumer side)
1. A consumer (e.g., `factory-query`, Loki Promtail filelog receiver) that
   encounters a JSON parse error on the FINAL line of a `events-*.jsonl`
   file MUST skip that line and continue processing. This is not an error
   condition; it is the expected truncation artifact of a dispatcher crash
   mid-write.
2. All lines except the final line are valid JSONL. A parse error on any
   non-final line IS an error condition and MUST be reported.

### FileSink write sequence (producer side)
1. `FileSink::write` serializes the event to a JSON byte buffer in memory.
2. A single `write_all(&buffer)` call writes the full JSON payload.
3. A second `write_all(b"\n")` call writes the newline terminator.
4. `flush()` is called to flush the BufWriter; this does NOT call `fsync`
   unless `sync_on_write = true`.
5. If `write_all` returns an error at step 2 or 3, the write-failure cascade
   is triggered (see below).

### Write-failure cascade
1. The failed event is written to `dispatcher-internal-YYYY-MM-DD.jsonl`
   unconditionally — regardless of the `VSDD_DEBUG_LOG` setting.
2. The dispatcher emits the following to `stderr`:
   `[vsdd-dispatcher] WARN: FileSink write failed for events-YYYY-MM-DD.jsonl
   (<error>); event written to dispatcher-internal-YYYY-MM-DD.jsonl as fallback.`
3. The fallback write is also best-effort: a failure to write the fallback is
   logged to `stderr` but does NOT abort the invocation or change the exit code.
4. The primary `FileSink` write failure does NOT change the dispatcher exit
   code (non-blocking; consistent with ADR-001 non-blocking error contract).

## Invariants

1. A parse error on the final line of `events-*.jsonl` is NEVER treated as
   file corruption by any consumer. It is a truncation artifact.
2. A parse error on any non-final line ALWAYS indicates actual file corruption
   (distinct from truncation).
3. `FileSink::write` NEVER silently discards a failed write. Either the event
   is written to the primary file, or it is written to the fallback file with
   a `stderr` warning. At least one of these two outcomes MUST occur.
4. `fsync` is called after each write ONLY when `sync_on_write = true` in
   `observability-config.toml`. The default is `false`.

## Related BCs

- BC-1.06.001 — Internal log writes are best-effort; never panic; never
  propagate (establishes the precedent for best-effort fallback semantics)
- BC-1.11.001 — VSDD_TRACE_ID injection (sibling ADR-015 BC in this cluster)
- BC-1.11.003 — Atomic dual-emit host helper (sibling; addresses orphan-half
  risk that this BC does NOT fully prevent)

## Architecture Anchors

- `crates/sink-file/src/lib.rs` — `FileSink::write` implementation; write
  sequence and failure cascade
- `crates/factory-dispatcher/src/internal_log.rs` — unconditional fallback
  write target on FileSink failure
- `SS-03-event-emission.md` § FileSink Write Semantics — design rationale for
  boundary-marker over atomic-rename
- ADR-015 D-15.1 — policy decision for write-failure cascade

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Disk full during `write_all(&buffer)` — partial JSON written | Truncated final line in events-*.jsonl; cascade triggers; fallback write attempted; stderr warning emitted |
| EC-002 | Disk full during `write_all(b"\n")` — full JSON written but no newline | Final line parses as valid JSON but has no `\n`; the NEXT write attempt opens a new BufWriter and continues; the unterminated line is treated as truncated by consumers |
| EC-003 | Filesystem becomes read-only mid-session | `write_all` returns a permissions error; cascade triggers; fallback write to internal log (same filesystem — may also fail); stderr warning always emitted |
| EC-004 | Fallback write to internal log also fails (same full disk) | Both write attempts fail; dispatcher emits two stderr warnings (one for primary failure, one for fallback failure); invocation continues; exit code unchanged |
| EC-005 | `sync_on_write = true`; `fsync` fails (e.g., NFS transport error) | `fsync` error triggers the write-failure cascade as if `write_all` had failed; same fallback + stderr behavior |
| EC-006 | Consumer reads the file while dispatcher is mid-write | Consumer sees a partial final line; treats it as truncation artifact (non-fatal); all prior lines are complete and valid |
| EC-007 | Consumer reads a file after a normal session end (no crash) | Final line is a complete JSONL record with `\n`; no truncation artifact |

## Canonical Test Vectors

| Scenario | Expected Producer Behavior | Expected Consumer Behavior |
|----------|---------------------------|---------------------------|
| Normal write (no failure) | `write_all(json)`, `write_all("\n")`, `flush()` all succeed | All lines parse as valid JSONL |
| `write_all(json)` fails (disk full) | Cascade: fallback write to internal log; stderr warning | Consumer skips final partial line if present |
| `write_all("\n")` fails after full JSON written | Cascade triggered; newline-missing final line treated as truncated | Consumer skips final line on parse error |
| `sync_on_write = true`, `fsync` fails | Cascade triggered; stderr warning | Consumer skips final line on parse error |
| Fallback write also fails | Two stderr warnings; no panic; invocation continues | N/A (event is lost) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — Phase 1.6b) | `write_all` failure always triggers cascade (never silent drop) | property-based test or Kani harness |
| (TBD — Phase 1.6b) | Cascade always produces a `stderr` warning | integration test (stderr capture) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-01/SS-03 — `crates/sink-file/src/lib.rs`, `crates/factory-dispatcher/src/internal_log.rs` |
| Stories | S-10.03 (Wave 1 enrichment + FileSink integration) |
| ADR | ADR-015 D-15.1 (write-failure semantics); SS-03-event-emission.md § FileSink Write Semantics (OQ-7 resolution) |
| OQ Resolved | OQ-7 (FileSink partial-write recovery — boundary-marker strategy chosen) |

### Purity Classification

| Property | Assessment |
|----------|-----------|
| I/O operations | YES — file write, optional fsync, fallback file write |
| Global state access | NO — BufWriter is per-invocation state |
| Deterministic | YES given fixed filesystem state |
| Thread safety | YES — `FileSink` is not shared across threads in single-stream model |
| Overall classification | Effectful shell (file I/O with defined failure contract) |

### Design Decision: Boundary-Marker vs. Atomic-Rename

Atomic-rename (write-to-temp, `rename(2)`) was rejected because:
1. It breaks the OTel Collector filelog receiver's inode-offset checkpoint
   mechanism — rename creates a new inode, invalidating the checkpoint and
   causing event re-delivery or loss on collector restart.
2. Daily rotation would require a full-file rewrite with atomic-rename, not
   just opening the new dated file.
3. It adds temp-file management complexity (naming, cleanup on crash) that
   conflicts with the simplicity goal of ADR-015.

The boundary-marker (final-line-skip-on-error) strategy is the established
JSONL append convention used by logrotate, OTel Collector filelog receiver,
and Loki Promtail. It is compatible with all external consumer tooling and
bounds the data loss to at most one event per crash — the event in flight at
the moment of the crash.
