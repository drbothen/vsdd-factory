---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-05-03T00:00:00Z
last_amended: 2026-05-05
phase: 1.4b
inputs: [gap-analysis-w16-subprocess.md]
input-hash: "[pending-recompute]"
traces_to: gap-analysis-w16-subprocess.md
origin: brownfield
extracted_from: ".factory/architecture/gap-analysis-w16-subprocess.md:Section 5"
subsystem: "SS-01"
capability: "CAP-TBD"
lifecycle_status: active
introduced: v1.0.0
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# Behavioral Contract BC-1.05.036: factory-dispatcher::host::exec_subprocess::emits_completed_event_on_success — host.exec_subprocess.completed event on every successful subprocess completion

## Description

**ADR-015 Awareness (added per E-9 v1.7 Post-Audit Amendment, propagated to BC at v1.21 per TD-VSDD-074):**
The success-path event name `host.exec_subprocess.completed` SHOWN IN THIS BC IS INTERIM. Per ADR-015 D-15.2 reverse-DNS naming requirement, the canonical event name resolves to one of two binary options (per OQ-W16-001 in `.factory/specs/open-questions.md`):
- (a) If `vsdd.host.*` is added to ADR-015 D-15.2 registry → `vsdd.host.exec_subprocess.completed.v1` (category to be assigned per OQ-W16-001 acceptance criterion (a) — ADR-015 D-15.2 registry has 5 categories: lifecycle, domain, audit, error, unknown).
- (b) Otherwise → `vsdd.dispatcher.subprocess_completed.v1` (lifecycle category per existing `vsdd.dispatcher.*` registry entry; OQ-W16-001 acceptance criterion (b)).
S-9.07 implementer MUST close OQ-W16-001 before merging the host-emit-fix story.

On successful subprocess completion (any exit code, including non-zero), the dispatcher MUST emit a `host.exec_subprocess.completed` event through `ctx.emit_internal` to the single-stream `FileSink` per ADR-015 D-15.1 (multi-sink stanza model removed; Router/SinkRegistry retired). This closes an observability gap identified in gap-analysis-w16-subprocess.md Section 5: `exec_subprocess.rs:285-288` currently has no emit call on success. The new event is success-path only; existing denial-path events are preserved unchanged.

## Preconditions

1. Plugin calls `vsdd::exec_subprocess`.
2. Capability check passes (binary is on allow-list, no traversal, no shell bypass violation).
3. Subprocess spawns and completes (any exit code).

## Postconditions

1. On successful subprocess completion (i.e., subprocess process actually exits before timeout AND within output cap; see Postcondition 5 for error-path reality), exactly one `host.exec_subprocess.completed` event is emitted via `ctx.emit_internal`. **Footnote on signal-death:** when `child.wait()` reports a status without an exit code (Unix signal termination), the dispatcher substitutes `exit_code = -1` per `status.code().unwrap_or(-1)` at exec_subprocess.rs:286. See EC-009 for v1 known limitation; OQ-W16-002 tracks v2 disambiguation. **Footnote on best-effort emit:** if `ctx.emit_internal` itself encounters an IO failure (FileSink write error, ENOSPC), the event is silently dropped per v1 best-effort semantics; see EC-010 and Postcondition 6.
2. Event payload includes all 8 fields: `{plugin_id: String, binary: String /* canonicalized full path */, args_count: u32, exit_code: i32, duration_ms: u64, stdout_bytes: u64, stderr_bytes: u64, truncated: bool /* reserved for future ABI break: always false in v1; truncation currently returns Err(OUTPUT_TOO_LARGE -3); see gap-analysis Section 5 'fundamentally insufficient' Gap 1 */}`. Per ADR-015 D-15.2:270, the host-enrichment layer additionally stamps an `outcome` enum field on this event using the canonical taxonomy (`success | failure | error | timeout | skipped | blocked`). For `host.exec_subprocess.completed` events the mapping is: `exit_code == 0 → 'success'`; `exit_code != 0 → 'failure'`. (TIMEOUT path emits no event per Postcondition 5; OUTPUT_TOO_LARGE and INTERNAL_ERROR likewise emit no event.) The `outcome` field is host-stamped per ADR-015 D-15.3, not part of the 8-field domain payload listed above. **`stdout_bytes`/`stderr_bytes` semantic:** defined as bytes returned in envelope AFTER any potential truncation — equal to the bytes encoded into the envelope. In v1, this equals bytes-actually-read-from-the-pipe (since truncation aborts the success path per Postcondition 5 and returns Err, not a partial envelope). At any future ABI break that introduces a real `truncated: true` semantic, `stdout_bytes` MAY exceed `max_output_bytes` if pre-truncate counting is adopted; until then the field's invariant is `stdout_bytes ≤ max_output_bytes`. Same definition applies to `stderr_bytes`. See EC-006 cross-ref.
3. `duration_ms` is measured from `Instant::now()` at `Command::spawn()` to process exit; implementer adds a `let started = Instant::now();` capture immediately before `command.spawn()` at exec_subprocess.rs:252 (which is the actual spawn point); the existing deadline `Instant` at exec_subprocess.rs:270 is post-spawn and is NOT the duration reference.
4. Event is emitted via `ctx.emit_internal` (same code path as the existing four `emit_denial` calls). The current `emit_internal` implementation at host/mod.rs:109-116 routes to `internal_log` (corresponding to `dispatcher-internal-*.jsonl`) — this is INTERIM. ADR-015 D-15.1 mandates the implementation be migrated to write through the single-stream `FileSink` to `events-*.jsonl` (multi-sink stanza model removed per ADR-015 D-15.1 §"Decision"; Router/SinkRegistry retired per ADR-015 D-15.1 §"Decision" "the `sink-otel-grpc` crate AND the `Router`, `SinkRegistry` types within `sink-core` are retired"; external export to remote observability backends is handled by OTel Collector outside the dispatcher). BC-1.05.036's implementation MUST include rewiring `emit_internal` to the single-stream FileSink BEFORE the success-path event is consumer-visible per ADR-015 D-15.1; otherwise the new event will be invisible to consumers reading `events-*.jsonl`. Track ADR-015 routing migration as part of E-9 Wave 1 implementation scope (see E-9 epic frontmatter).
5. **Error-path event reality (per gap-analysis §1):** Today only `internal.capability_denied` is emitted on the 4 denial paths (`no_exec_subprocess_capability` per exec_subprocess.rs:148, `binary_not_on_allow_list` per exec_subprocess.rs:155, `shell_bypass_not_acknowledged` per exec_subprocess.rs:162, `setuid_or_setgid_binary` per exec_subprocess.rs:169). Note: env_allow violations are silently filtered (no event emitted) and cwd_allow is currently unenforced — see gap-analysis Section 1 row "Cwd allow-list" PARTIAL. TIMEOUT (-2) and OUTPUT_TOO_LARGE (-3) paths return error codes WITHOUT emitting any event. The success-path event introduced by this BC closes the success-path observability gap. Adding TIMEOUT/OUTPUT_TOO_LARGE error-path events is OUT OF SCOPE for this BC and may be tracked in a future OQ if needed. Additionally, INTERNAL_ERROR (-99) paths return error codes WITHOUT emitting any event: spawn failure (exec_subprocess.rs:252), stdin take/write failure (:258, :259-262), stdout/stderr take failure (:267-268), try_wait error (:299). `INTERNAL_ERROR` constant defined at `crates/factory-dispatcher/src/host/mod.rs:184` as `i32 = -99`. All three no-event error paths (TIMEOUT, OUTPUT_TOO_LARGE, INTERNAL_ERROR) are out-of-scope for this BC's success-path event contract.
6. **Best-effort emit semantics:** On `ctx.emit_internal` IO failure (FileSink write error, broken pipe to OTel collector, ENOSPC) the dispatcher SHALL silently drop the event per v1 best-effort semantics — `host/mod.rs:109-116` ignores the `log.write(&event)` return value. The host call's return value (`Ok(envelope)` or `Err(code)`) is NOT affected by emit-side IO failure. **v1 known limitation:** IO-failed emissions are not retried, not logged, not exposed via fallback channel. Track in **OQ-W16-003** if observability of emit-side failures becomes a security requirement. See EC-010.

**Input bounds (implicit):** `args_len`, `stdin_len`, and total argv+envp size are NOT explicitly validated by this BC. Bounds enforcement is implicit: (a) `read_wasm_bytes` at host/memory.rs:35 refuses `len` exceeding guest memory size — returns INVALID_ARGUMENT (-4); (b) `command.spawn()` at exec_subprocess.rs:252 returns Err on kernel-level limits (e.g., ARG_MAX exceedance) → INTERNAL_ERROR (-99). No pre-spawn argv-length check is performed.

## Invariants

1. Success-path telemetry is emitted for every subprocess completion regardless of the subprocess's own exit code (exit code 0 and non-zero both trigger the event).

## Related BCs

- BC-1.05.001 — exec_subprocess capability check (depends on: this event fires only after the capability check passes)
- BC-1.05.032 — timeout enforcement (sibling: timeout path returns `Err(TIMEOUT -2)` and emits NO event in v1; future error-path emit is out-of-scope per BC-1.05.036 Postcondition 5)
- BC-1.05.005 — OUTPUT_TOO_LARGE path (sibling: output-too-large path returns `Err(OUTPUT_TOO_LARGE -3)` and emits NO event in v1; aligns with BC-1.05.036 Postcondition 5)
- BC-1.05.035 — path canonicalization guard (sibling extension from same gap analysis). NOTE: BC-1.05.035 adds canonicalization for TOCTOU prevention. Symlink-resolved targets that fall outside `binary_allow` reach the existing CAPABILITY_DENIED path via emit_denial('binary_not_on_allow_list'). No novel error-code pairing introduced. All denial paths (including symlink-miss) return CAPABILITY_DENIED (-1), consistent with the 4 existing denial paths. Cross-dependency: BC-1.05.036's success-path event payload `binary` field (per EC-006, declared as 'canonicalized full path') depends on BC-1.05.035 Postcondition 1 propagating the canonical path through to `Command::new(...)` at line 230. If BC-1.05.035 Postcondition 1 is misimplemented (canonical at allow-check only, raw at spawn), BC-1.05.036's `binary` field will be the raw cmd — a contract violation in BC-1.05.036.

## Architecture Anchors

- `crates/factory-dispatcher/src/host/exec_subprocess.rs:285-288` — current success path with no emit call; `emit_event!("host.exec_subprocess.completed", ...)` added here
- `crates/factory-dispatcher/src/host/exec_subprocess.rs:155` — representative existing `emit_denial(ctx, cmd, "binary_not_on_allow_list", ...)` call site (the four CALL sites are at 148/155/162/169; new emit-on-success call follows the same `ctx.emit_internal(...)` pattern). The fn definition is at exec_subprocess.rs:304-309 for reference.
- `.factory/architecture/gap-analysis-w16-subprocess.md` Section 5 — authority for this extension

## Story Anchor

S-9.07 (validate-wave-gate-prerequisite WASM port) — implementation task

## VP Anchors

(TBD — to be assigned in Phase 1.6b)

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Subprocess exits 0 | `host.exec_subprocess.completed` emitted with `exit_code=0` |
| EC-002 | Subprocess exits non-zero (e.g., 1) | `host.exec_subprocess.completed` emitted with `exit_code=1` |
| EC-003 | Capability check fails (one of: `no_exec_subprocess_capability`, `binary_not_on_allow_list`, `shell_bypass_not_acknowledged`, `setuid_or_setgid_binary` per exec_subprocess.rs:148/155/162/169; OR `binary_canonicalize_failed` per BC-1.05.035 Postcondition 3 NEW path inserted before line 152; env_allow + cwd_allow violations do NOT trigger this EC) | `internal.capability_denied` emitted; `host.exec_subprocess.completed` NOT emitted |
| EC-004 | Subprocess times out | Returns `Err(TIMEOUT -2)`; **NO event emitted in v1** (per Postcondition 5; future error-path emit is out-of-scope); `host.exec_subprocess.completed` NOT emitted |
| EC-005 | Subprocess output exceeds cap | Returns `Err(OUTPUT_TOO_LARGE -3)`; **NO event emitted in v1** (per Postcondition 5; future error-path emit is out-of-scope); `host.exec_subprocess.completed` NOT emitted |
| EC-006 | Payload field type check | All 8 fields present with declared types (`plugin_id: String`, `binary: String`, `args_count: u32`, `exit_code: i32`, `duration_ms: u64`, `stdout_bytes: u64`, `stderr_bytes: u64`, `truncated: bool /* reserved for future ABI break: always false in v1; truncation currently returns Err(OUTPUT_TOO_LARGE -3); see gap-analysis Section 5 'fundamentally insufficient' Gap 1 */`). `stdout_bytes`/`stderr_bytes` semantic in v1: post-truncate (envelope bytes); see Postcondition 2 field description. |
| EC-007 | Subprocess spawn fails / pipe take/write fails / try_wait error | Returns `Err(INTERNAL_ERROR -99)`; **NO event emitted in v1** (per Postcondition 5; spawn at exec_subprocess.rs:252, stdin take/write at :258/:259-262, stdout/stderr take at :267-268, try_wait at :299); `host.exec_subprocess.completed` NOT emitted |
| EC-008 | Outcome enum stamping | exit_code=0 → host stamps `outcome='success'` per ADR-015 D-15.3 (mapping per Postcondition 2); exit_code≠0 → `outcome='failure'`. TIMEOUT/OUTPUT_TOO_LARGE/INTERNAL_ERROR paths emit no event so no outcome stamping applies. |
| EC-009 | Subprocess terminated by signal (kernel SIGKILL/SIGSEGV/SIGINT/external `kill -9`) — `status.code()` returns `None`; no exit code | `host.exec_subprocess.completed` IS emitted with `exit_code = -1` (from `status.code().unwrap_or(-1)` at exec_subprocess.rs:286), `outcome = 'failure'`. **v1 known limitation:** `exit_code = -1` is indistinguishable from a literal `_exit(-1)` from C — signal-vs-exit collision is NOT disambiguated in v1. **Future:** track in **OQ-W16-002** if security observability needs to distinguish SIGSEGV (potential exploit) from SIGKILL (OOM-killer) from real exit `-1`. POSIX 128+signum convention (e.g., 139 for SIGSEGV) considered for v2 ABI break. | known-limitation |
| EC-010 | `ctx.emit_internal` IO failure (FileSink full, broken pipe, ENOSPC): `log.write(&event)` returns Err but return value is discarded at host/mod.rs:111 | Host call return value unaffected (still `Ok(envelope)` for success path); event silently dropped. Postcondition 1 "exactly one event emitted" is best-effort: emitted-to-emit_internal-API, NOT guaranteed-delivered-to-consumer. See Postcondition 6 and OQ-W16-003. | best-effort-silent-drop |
| EC-011 | `events: Mutex<Vec<InternalEvent>>` is poisoned by a panic in another concurrent host call | `emit_internal` silently drops the event (host/mod.rs:113 `if let Ok(mut events) = self.events.lock()`); subsequent `drain_events` (host/mod.rs:102 `.expect("events mutex poisoned")`) panics the dispatcher. Asymmetric write-vs-read poison handling. **v1 known limitation:** post-poison emissions are silently dropped until the dispatcher process is restarted or the Mutex is refreshed. Track in **OQ-W16-004**. | known-limitation |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Capability passes; subprocess exits 0 | Exactly one `host.exec_subprocess.completed` event; `exit_code=0` | happy-path |
| Capability passes; subprocess exits 1 | Exactly one `host.exec_subprocess.completed` event; `exit_code=1` | happy-path |
| Outcome enum (success): exit_code=0 invocation | Exactly one event; payload exit_code=0; host-stamped outcome='success' (per Postcondition 2 mapping; ADR-015 D-15.3 enrichment) | host-stamping |
| Outcome enum (failure): exit_code=1 invocation | Exactly one event; payload exit_code=1; host-stamped outcome='failure' | host-stamping |
| Capability check fails | `internal.capability_denied` emitted; `host.exec_subprocess.completed` NOT emitted | error |
| Subprocess timeout | Returns `Err(TIMEOUT -2)`; NO event emitted in v1 per Postcondition 5; `host.exec_subprocess.completed` NOT emitted | error |
| Subprocess output exceeds cap | Returns `Err(OUTPUT_TOO_LARGE -3)`; **NO event emitted in v1** per Postcondition 5; `host.exec_subprocess.completed` NOT emitted | error |
| Subprocess spawn fails (or pipe take/write fails or try_wait error) | Returns `Err(INTERNAL_ERROR -99)`; NO event emitted in v1 per Postcondition 5 (spawn at :252, stdin take/write at :258/:259-262, stdout/stderr take at :267-268, try_wait at :299); `host.exec_subprocess.completed` NOT emitted | error |
| Successful completion | Event payload contains all 8 fields with correct types | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD (anchor in Phase 1.5) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/host/exec_subprocess.rs` |
| Stories | S-9.07 |
| Capability Anchor Justification | CAP-TBD — capability anchor to be confirmed in Phase 1.5; this BC governs exec_subprocess success-path observability in factory-dispatcher |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `.factory/architecture/gap-analysis-w16-subprocess.md` Section 5 item 2; `crates/factory-dispatcher/src/host/exec_subprocess.rs:285-288` |
| **Confidence** | HIGH (gap explicitly identified by architect gap analysis; analogous pattern exists in emit_denial at lines 304–309) |
| **Extraction Date** | 2026-05-03 |
| **Extracted from** | `.factory/architecture/gap-analysis-w16-subprocess.md` Section 5 |

#### Evidence Types Used

- gap-analysis (architect-identified missing emit call at exec_subprocess.rs:285–288)
- assertion (analogous emit_denial pattern at exec_subprocess.rs:304–309 confirms implementation pattern)

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | YES — emits event via `ctx.emit_internal`: synchronous `Mutex::lock` + `Vec::push` to events queue per `host/mod.rs:105-116`, then host writes to single-stream `FileSink` per ADR-015 D-15.1 (no channel send; not async) |
| **Global state access** | No |
| **Deterministic** | YES — event emission is deterministic given subprocess completion |
| **Thread safety** | YES on the success-path (same write pattern as existing emit_denial calls). KNOWN LIMITATION: Mutex poison asymmetry — `emit_internal` silently drops on poison (host/mod.rs:113), while `drain_events` panics on poison (host/mod.rs:102). See EC-011. |
| **Overall classification** | Deterministic with best-effort sink I/O |

#### Refactoring Notes

(TBD — to be assessed in Phase 1.6b verification properties pass)
