---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-05-06T00:00:00Z
phase: 1a
inputs:
  - .factory/specs/architecture/decisions/ADR-015-single-stream-otel-schema.md
  - .factory/stories/epics/E-10-single-stream-otel-event-emission.md
input-hash: "[pending-recompute]"
traces_to: ADR-015-single-stream-otel-schema.md
origin: greenfield
subsystem: "SS-01"
capability: "CAP-010"
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

# Behavioral Contract BC-1.12.002: factory-dispatcher::debug_stream::vsdd_debug_log_gate — dispatcher-internal-YYYY-MM-DD.jsonl writes gated by VSDD_DEBUG_LOG=1; off by default in release builds; ADR-007 always-on guarantee amended

## Description

ADR-015 D-15.1 amends ADR-007's "always-on" guarantee for the debug file
`dispatcher-internal-YYYY-MM-DD.jsonl`. After the Wave 1 FileSink rewire, this
file is an **opt-in debug stream** gated by the `VSDD_DEBUG_LOG=1` environment
variable. In production (release) builds where `VSDD_DEBUG_LOG` is unset (or
set to any value other than `"1"`), no writes occur to `dispatcher-internal-*.jsonl`
on the normal execution path.

The debug stream remains the unconditional **write-failure fallback** for
`FileSink` (per BC-1.11.002) regardless of `VSDD_DEBUG_LOG` — this is a
distinct code path from the gate described in this BC.

This BC is a future-implementation contract for S-10.02 (Wave 1). All
Canonical Test Vectors describe post-Wave-1 behavior. A misimplementation
that always writes to `dispatcher-internal-*.jsonl` regardless of the gate
is distinguishable by the test vectors below.

## Preconditions

1. The dispatcher process has started.
2. A write to `dispatcher-internal-YYYY-MM-DD.jsonl` is triggered by a lifecycle
   event path (e.g., dispatcher startup event, pre-Wave-1 `InternalLog` path) OR
   by the debug-supplementary write when `VSDD_DEBUG_LOG=1`.
3. This BC does NOT govern the write-failure fallback path — that path is
   governed by BC-1.11.002 and writes unconditionally regardless of
   `VSDD_DEBUG_LOG`.

## Postconditions

1. When `VSDD_DEBUG_LOG` is NOT set (or is set to a value other than `"1"`),
   no write to `dispatcher-internal-YYYY-MM-DD.jsonl` occurs on the normal
   execution path. The file MAY NOT exist at all in this case.
   **Future-implementation witness:** A misimplementation that still writes to
   `dispatcher-internal-*.jsonl` unconditionally (per the pre-Wave-1 always-on
   behavior) will produce this file even when `VSDD_DEBUG_LOG` is unset. The
   distinguishing test: run the dispatcher with events emitted; assert
   `dispatcher-internal-*.jsonl` is absent or has zero new bytes (beyond any
   pre-existing content) when `VSDD_DEBUG_LOG` is unset.
2. When `VSDD_DEBUG_LOG=1` is set in the dispatcher process environment, writes
   to `dispatcher-internal-YYYY-MM-DD.jsonl` occur for each event emission (in
   addition to the primary `events-*.jsonl` write per BC-1.12.001). The debug
   file receives a supplementary copy of every event written to `events-*.jsonl`.
3. The `VSDD_DEBUG_LOG` gate is evaluated at the write site, not at startup.
   A dispatcher started without `VSDD_DEBUG_LOG=1` that later has the variable
   injected into its environment via a side-channel does not automatically acquire
   debug writes (OS-level env vars are stable for a process lifetime after exec;
   this is a non-issue in normal operation).
4. Operators who relied on `dispatcher-internal-*.jsonl` being present without
   configuration (pre-Wave-1 always-on behavior) MUST set `VSDD_DEBUG_LOG=1`
   to restore that file. This behavioral change is a KNOWN TRADE-OFF per ADR-015
   Negative consequences (ADR-015 §"ADR-007 always-on guarantee is weakened").

## Invariants

1. `VSDD_DEBUG_LOG=1` is the only supported gate value. Any other non-empty string
   (`"true"`, `"yes"`, `"on"`) does NOT enable the debug stream. The implementation
   MUST check for the exact string `"1"`.
2. The write-failure fallback path to `dispatcher-internal-*.jsonl` (BC-1.11.002)
   is NEVER gated by `VSDD_DEBUG_LOG`. Fallback writes are unconditional.
3. Setting `VSDD_DEBUG_LOG=1` does NOT change the primary write destination —
   `events-*.jsonl` remains the primary stream. The debug file is additive.

## Related BCs

- BC-1.12.001 — Single-stream FileSink routing (composes with: this BC defines
  the secondary/debug stream behavior; BC-1.12.001 defines the primary stream)
- BC-1.11.002 — FileSink write-failure cascade (sibling: the fallback path
  writes unconditionally to the debug file regardless of this gate; BC-1.11.002
  governs that path; this BC governs the voluntary debug-supplementary-write path)

## Architecture Anchors

- `crates/factory-dispatcher/src/main.rs` — `VSDD_DEBUG_LOG` environment variable
  check; the gate is evaluated before any write to `dispatcher-internal-*.jsonl`
- `crates/factory-dispatcher/src/internal_log.rs` — `InternalLog` struct; post-Wave-1,
  its write path is conditional on `VSDD_DEBUG_LOG=1`
- ADR-015 D-15.1 — "gated by the `VSDD_DEBUG_LOG=1` environment variable and is off
  by default in release builds. ADR-007's 'always-on' guarantee is amended"
- `observability-config.toml` — `debug_log_enabled` config key (default `false`);
  `VSDD_DEBUG_LOG=1` env var overrides this config key at runtime

## Story Anchor

S-10.02 (Wave 1: FileSink single-stream wiring; includes debug-stream gate)

## VP Anchors

(TBD — to be assigned after S-10.02 story authoring)

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `VSDD_DEBUG_LOG` not set in environment | No writes to `dispatcher-internal-*.jsonl` on normal path; file is absent (or unchanged from a prior run) |
| EC-002 | `VSDD_DEBUG_LOG=1` set in environment | Every event write to `events-*.jsonl` is also mirrored to `dispatcher-internal-*.jsonl`; both files receive the event |
| EC-003 | `VSDD_DEBUG_LOG=true` (non-"1" truthy string) | Debug stream is NOT enabled; gate checks for exact string `"1"` only; behavior same as EC-001 |
| EC-004 | `VSDD_DEBUG_LOG=0` (explicit disable) | Debug stream is NOT enabled; same as unset; behavior same as EC-001 |
| EC-005 | `FileSink::write` fails (disk full); `VSDD_DEBUG_LOG` unset | Write-failure fallback (BC-1.11.002) writes to `dispatcher-internal-*.jsonl` unconditionally, bypassing the gate. This EC demonstrates that the fallback is NOT gated by `VSDD_DEBUG_LOG`. |
| EC-006 | `FileSink::write` fails (disk full); `VSDD_DEBUG_LOG=1` | Fallback write to `dispatcher-internal-*.jsonl` per BC-1.11.002; AND debug supplementary write also triggered. Result: one write to the debug file (fallback + debug supplementary may merge or deduplicate — implementation detail; at minimum the event is written once to the debug file). |
| EC-007 | `observability-config.toml` has `debug_log_enabled = true`; `VSDD_DEBUG_LOG` unset | The env var `VSDD_DEBUG_LOG=1` takes precedence if set; when unset, the `debug_log_enabled = true` config key MAY also enable the debug stream (implementation-level decision: the config key is a static default; the env var is a runtime override). This is a Phase 1b clarification point — the SS-03 spec owns the config schema. |
| EC-008 | Operator upgrades from pre-Wave-1 dispatcher to post-Wave-1; no `VSDD_DEBUG_LOG` set | `dispatcher-internal-*.jsonl` stops being written. Operator's monitoring systems relying on that file receive no new events. Operator MUST set `VSDD_DEBUG_LOG=1` to restore the file. This is a KNOWN BREAKING CHANGE per ADR-015 §Negative consequences. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Run dispatcher; emit 3 events; `VSDD_DEBUG_LOG` unset | `events-*.jsonl` contains 3 lines; `dispatcher-internal-*.jsonl` absent or has 0 new bytes | happy-path-gate-off |
| **Misimplementation distinguisher:** post-Wave-1 code still writes unconditionally to `dispatcher-internal-*.jsonl` (pre-D-15.1 behavior) | Test MUST assert `dispatcher-internal-*.jsonl` has 0 new bytes when gate is off. A misimplementation that always writes produces non-zero bytes — distinguishable from correct behavior. | misimplementation-witness |
| Run dispatcher; emit 3 events; `VSDD_DEBUG_LOG=1` | `events-*.jsonl` contains 3 lines; `dispatcher-internal-*.jsonl` ALSO contains 3 lines (or more if debug path duplicates) | happy-path-gate-on |
| `VSDD_DEBUG_LOG=true` (not "1"); emit 1 event | `events-*.jsonl` has 1 line; `dispatcher-internal-*.jsonl` absent or unchanged | non-canonical-truthy-string-no-op |
| `VSDD_DEBUG_LOG=0`; emit 1 event | `events-*.jsonl` has 1 line; `dispatcher-internal-*.jsonl` absent or unchanged | explicit-zero-gate-off |
| `FileSink::write` fails; `VSDD_DEBUG_LOG` unset | `dispatcher-internal-*.jsonl` receives fallback write (1 line) regardless of gate; stderr warning emitted | fallback-bypasses-gate |
| `FileSink::write` succeeds; `VSDD_DEBUG_LOG` unset | `dispatcher-internal-*.jsonl` receives 0 new bytes; confirms gate is checked BEFORE write, not after | gate-check-order |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD) | Gate-off: no bytes written to `dispatcher-internal-*.jsonl` on normal path | integration test: spawn dispatcher without `VSDD_DEBUG_LOG`; assert file absent or byte count unchanged |
| (TBD) | Gate-on: every event written to `events-*.jsonl` also appears in `dispatcher-internal-*.jsonl` | integration test: spawn with `VSDD_DEBUG_LOG=1`; assert line counts match |
| (TBD) | Fallback path bypasses gate | integration test: inject `FileSink` write failure; assert `dispatcher-internal-*.jsonl` grows regardless of `VSDD_DEBUG_LOG` |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-010 ("Always-on dispatcher self-telemetry independent of sink config") per capabilities.md §CAP-010 |
| Capability Anchor Justification | CAP-010 ("Always-on dispatcher self-telemetry independent of sink config") per capabilities.md §CAP-010. This BC governs the `dispatcher-internal-*.jsonl` self-telemetry file, which is exactly the always-on debug/audit mechanism that CAP-010 defines. ADR-015 D-15.1 amends the always-on guarantee to be opt-in via `VSDD_DEBUG_LOG=1`; CAP-010 is the correct anchor because the capability's purpose (providing an independent telemetry channel) is unchanged — only its activation model is amended. |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/internal_log.rs`, `crates/factory-dispatcher/src/main.rs` (gate check) |
| Stories | S-10.02 (Wave 1 FileSink single-stream wiring + debug-stream gate) |
| Epic | E-10 (Single-stream OTel-aligned event emission) |
| ADR | ADR-015 D-15.1 ("gated by the `VSDD_DEBUG_LOG=1` environment variable"); ADR-007 (amended: always-on guarantee weakened) |

### Purity Classification

| Property | Assessment |
|----------|-----------|
| I/O operations | YES — conditional write to `dispatcher-internal-*.jsonl`; env var read |
| Global state access | YES — reads `VSDD_DEBUG_LOG` from process environment at write site |
| Deterministic | YES given fixed `VSDD_DEBUG_LOG` value |
| Thread safety | YES — env var read is read-only; `InternalLog` write is bounded to single-threaded dispatcher |
| Overall classification | Effectful shell (conditional I/O with defined gate semantics) |

### TD-VSDD-092 (BC-SOUL4-coverage) Verification

Source-walk for silent-discard patterns in the `VSDD_DEBUG_LOG` gate implementation:

- The pre-Wave-1 `InternalLog::write` at `internal_log.rs:228-238` swallows IO errors via
  `eprintln!` fallback (not silent — it logs to stderr). After Wave 1, when the gate is `true`,
  the same `InternalLog::write` path is used. The stderr-fallback behavior is inherited and
  explicitly NOT a silent discard (per BC-1.05.036 Postcondition 6 / EC-010).
- The gate check itself (`std::env::var("VSDD_DEBUG_LOG")`) returns `Err` if the var is unset —
  the `Err` path correctly maps to "gate off" (no write). No silent-discard of the gate result.
- No `let _ =` patterns expected at the gate site. Implementation MUST NOT use `let _ = write_debug(...)` pattern.
