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

1. On successful subprocess termination (i.e., `child.wait()` returns `Ok(Some(status))` within timeout AND output cap; see Postcondition 5 for error-path reality and EC-009 for signal-death substitution), exactly one `host.exec_subprocess.completed` event is emitted via `ctx.emit_internal`. **Footnote on signal-death:** when `child.wait()` reports a status without an exit code (Unix signal termination), the dispatcher substitutes `exit_code = -1` per `status.code().unwrap_or(-1)` at exec_subprocess.rs:286. See EC-009 for v1 known limitation; OQ-W16-002 tracks v2 disambiguation. **Footnote on best-effort emit:** if `internal_log.write_inner` encounters an IO failure (ENOSPC, missing directory, permissions), `write` emits an eprintln to stderr and returns `()`; the event is NOT silently dropped — it is logged to stderr AND still pushed to the in-memory `events` queue; see EC-010 and Postcondition 6.
2. Event payload includes all 8 fields: `{plugin_id: String, binary: String /* canonicalized full path */, args_count: u32, exit_code: i32, duration_ms: u64, stdout_bytes: u64, stderr_bytes: u64, truncated: bool /* reserved for future ABI break: always false in v1; truncation currently returns Err(OUTPUT_TOO_LARGE -3); see gap-analysis Section 5 'fundamentally insufficient' Gap 1 */}`. Per ADR-015 D-15.2:270, the host-enrichment layer additionally stamps an `outcome` enum field on this event using the canonical taxonomy (`success | failure | error | timeout | skipped | blocked`). For `host.exec_subprocess.completed` events the mapping is: `exit_code == 0 → 'success'`; `exit_code != 0 → 'failure'`. (TIMEOUT path emits no event per Postcondition 5; OUTPUT_TOO_LARGE and INTERNAL_ERROR likewise emit no event.) The `outcome` field is host-stamped per ADR-015 D-15.3, not part of the 8-field domain payload listed above. **`stdout_bytes`/`stderr_bytes` semantic:** defined as bytes returned in envelope AFTER any potential truncation — equal to the bytes encoded into the envelope. In v1, this equals bytes-actually-read-from-the-pipe (since truncation aborts the success path per Postcondition 5 and returns Err, not a partial envelope). At any future ABI break that introduces a real `truncated: true` semantic, `stdout_bytes` MAY exceed `max_output_bytes` if pre-truncate counting is adopted; until then the field's invariant is `stdout_bytes ≤ max_output_bytes`. Same definition applies to `stderr_bytes`. See EC-006 cross-ref.
3. `duration_ms` is measured from `Instant::now()` at `Command::spawn()` to process exit; implementer adds a `let started = Instant::now();` capture immediately before `command.spawn()` at exec_subprocess.rs:252 (which is the actual spawn point); the existing deadline `Instant` at exec_subprocess.rs:270 is post-spawn and is NOT the duration reference.
4. Event is emitted via `ctx.emit_internal` (same code path as the existing four `emit_denial` calls). The `emit_internal` implementation at host/mod.rs:109-116 has a bifurcation: **If `ctx.internal_log.is_some()`** (production path — `InternalLog` wired by dispatcher main), `log.write(&event)` is called first, writing to `dispatcher-internal-*.jsonl`; this is INTERIM per ADR-015 D-15.1 (migration target: single-stream FileSink to `events-*.jsonl`). **If `ctx.internal_log` is `None`** (test-context path — `HostContext::new` at host/mod.rs:96 sets `internal_log: None`), the `log.write` call is SKIPPED entirely. In both cases, `events.lock().push(event)` at host/mod.rs:113-115 runs independently and the event is pushed to the in-memory `events` queue regardless of whether `internal_log` is wired. Tests using `HostContext::new` therefore only observe the event through the `events` queue (via `drain_events`), not through any file on disk. ADR-015 D-15.1 mandates the implementation be migrated to write through the single-stream `FileSink` to `events-*.jsonl`; BC-1.05.036's implementation MUST include rewiring `emit_internal` to the single-stream FileSink BEFORE the success-path event is consumer-visible per ADR-015 D-15.1. Track ADR-015 routing migration as part of E-9 Wave 1 implementation scope (see E-9 epic frontmatter).
5. **Error-path event reality (per gap-analysis §1):** Today only `internal.capability_denied` is emitted on the 4 denial paths (`no_exec_subprocess_capability` per exec_subprocess.rs:148, `binary_not_on_allow_list` per exec_subprocess.rs:155, `shell_bypass_not_acknowledged` per exec_subprocess.rs:162, `setuid_or_setgid_binary` per exec_subprocess.rs:169). Note: `env_allow` names absent from `ctx.env_view` are silently omitted (no event; see EC-014) and `cwd_allow` is currently unenforced (no denial event for cwd outside the allow-list; see EC-012 and OQ-W16-007). TIMEOUT (-2) and OUTPUT_TOO_LARGE (-3) paths return error codes WITHOUT emitting any event (two distinct OUTPUT_TOO_LARGE paths exist: EC-005A subprocess-output-overflow at exec_subprocess.rs:278-283 and EC-005B result_buf_cap-overflow at exec_subprocess.rs:86-88 — both return -3 and emit no event). The success-path event introduced by this BC closes the success-path observability gap. Adding TIMEOUT/OUTPUT_TOO_LARGE error-path events is OUT OF SCOPE for this BC and may be tracked in a future OQ if needed. Additionally, INTERNAL_ERROR (-99) paths return error codes WITHOUT emitting any event: spawn failure (exec_subprocess.rs:252), stdin take/write failure (:258, :259-262), stdout/stderr take failure (:267-268), try_wait error (:299). `INTERNAL_ERROR` constant defined at `crates/factory-dispatcher/src/host/mod.rs:184` as `i32 = -99`. All three no-event error paths (TIMEOUT, OUTPUT_TOO_LARGE, INTERNAL_ERROR) are out-of-scope for this BC's success-path event contract.
6. **Best-effort emit semantics (internal_log path):** `internal_log.write(&event)` has return type `()` — it NEVER returns an error to its caller (`emit_internal` at host/mod.rs:110-111). On internal IO failure, `write_inner` returns `std::io::Result<()>` (internal_log.rs:240); `write` (internal_log.rs:228-238) swallows the `Err` via `eprintln!("factory-dispatcher: internal_log write failed ({}): {}", event.type_, e)` to stderr and returns `()`. The `emit_internal` call site at host/mod.rs:110-111 therefore sees no error regardless of IO outcome. Separately, `events.lock().push(event)` at host/mod.rs:113-115 is an independent step — it runs whether or not the internal_log write succeeded. The host call's return value (`Ok(envelope)` or `Err(code)`) is NOT affected by emit-side IO failure. **v1 known limitation:** Internal_log IO failures are reported to stderr (not silent) but are not propagated to callers, not retried, and not exposed via fallback counter. Track in **OQ-W16-003** if observability of emit-side failures becomes a security requirement. See EC-010. (Witnessed by Test Vector row 11 — best-effort-emit-witness — and EC-010.)

**Input bounds (implicit):** `args_len`, `stdin_len`, and total argv+envp size are NOT explicitly validated by this BC. Bounds enforcement is implicit: (a) `read_wasm_bytes` at host/memory.rs:35 returns `HostCallError::OutOfBounds` for `len` exceeding guest memory size; the caller at exec_subprocess.rs:54-67 maps the error to `codes::INVALID_ARGUMENT` (-4); (b) `command.spawn()` at exec_subprocess.rs:252 returns Err on kernel-level limits (e.g., ARG_MAX exceedance) → INTERNAL_ERROR (-99). No pre-spawn argv-length check is performed.

## Invariants

1. Success-path telemetry is emitted for every subprocess completion regardless of the subprocess's own exit code (exit code 0 and non-zero both trigger the event).

## Related BCs

- BC-1.05.001 — exec_subprocess capability check (depends on: this event fires only after the capability check passes)
- BC-1.05.032 — timeout enforcement (sibling: timeout path returns `Err(TIMEOUT -2)` and emits NO event in v1; future error-path emit is out-of-scope per BC-1.05.036 Postcondition 5)
- BC-1.05.005 — OUTPUT_TOO_LARGE path (sibling: output-too-large path returns `Err(OUTPUT_TOO_LARGE -3)` and emits NO event in v1; aligns with BC-1.05.036 Postcondition 5; NOTE: two distinct OUTPUT_TOO_LARGE paths exist — EC-005A subprocess-output-overflow at exec_subprocess.rs:278-283 and EC-005B result_buf_cap-overflow at exec_subprocess.rs:86-88)
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
| EC-005A | Subprocess stdout or stderr output exceeds `max_output_bytes` (exec_subprocess.rs:278-283): after subprocess exits, `stdout_buf.len() > max_output_bytes as usize OR stderr_buf.len() > max_output_bytes as usize` — `truncated` flag fires | Returns `Err(OUTPUT_TOO_LARGE -3)`; **NO event emitted in v1** (per Postcondition 5; future error-path emit is out-of-scope); `host.exec_subprocess.completed` NOT emitted. This is the subprocess-output-overflow path. [error] |
| EC-005B | Encoded result envelope size exceeds `result_buf_cap` (exec_subprocess.rs:86-88): after subprocess completes and output is within cap, `encode_envelope(exit_code, stdout, stderr).len() > result_buf_cap` — envelope too large for the guest-provided result buffer | Returns `Err(OUTPUT_TOO_LARGE -3)` (same code as EC-005A); **NO event emitted in v1** (per Postcondition 5); `host.exec_subprocess.completed` NOT emitted. Distinct from EC-005A: subprocess output WAS within `max_output_bytes`, but the encoded envelope (12-byte header overhead + stdout + stderr) does not fit in the guest's `result_buf_cap`. **Operator note:** plugins MUST size result buffers to accommodate at least `stdout_len + stderr_len + 12` bytes — the `encode_envelope` overhead at exec_subprocess.rs:101-109 is exactly 12 bytes (4 bytes exit_code + 4 bytes stdout_len + 4 bytes stderr_len). Sized correctly: `result_buf_cap >= max_output_bytes * 2 + 12`. [error] |
| EC-006 | Payload field type check | All 8 fields present with declared types (`plugin_id: String`, `binary: String`, `args_count: u32`, `exit_code: i32`, `duration_ms: u64`, `stdout_bytes: u64`, `stderr_bytes: u64`, `truncated: bool /* reserved for future ABI break: always false in v1; truncation currently returns Err(OUTPUT_TOO_LARGE -3); see gap-analysis Section 5 'fundamentally insufficient' Gap 1 */`). `stdout_bytes`/`stderr_bytes` semantic in v1: post-truncate (envelope bytes); see Postcondition 2 field description. |
| EC-007 | Subprocess spawn fails / pipe take/write fails / try_wait error | Returns `Err(INTERNAL_ERROR -99)`; **NO event emitted in v1** (per Postcondition 5; spawn at exec_subprocess.rs:252, stdin take/write at :258/:259-262, stdout/stderr take at :267-268, try_wait at :299); `host.exec_subprocess.completed` NOT emitted |
| EC-008 | Outcome enum stamping | exit_code=0 → host stamps `outcome='success'` per ADR-015 D-15.3 (mapping per Postcondition 2); exit_code≠0 → `outcome='failure'`. TIMEOUT/OUTPUT_TOO_LARGE/INTERNAL_ERROR paths emit no event so no outcome stamping applies. |
| EC-009 | Subprocess terminated by signal (kernel SIGKILL/SIGSEGV/SIGINT/external `kill -9`) — `status.code()` returns `None`; no exit code | `host.exec_subprocess.completed` IS emitted with `exit_code = -1` (from `status.code().unwrap_or(-1)` at exec_subprocess.rs:286), `outcome = 'failure'`. **v1 known limitation:** `exit_code = -1` is indistinguishable from a literal `_exit(-1)` from C — signal-vs-exit collision is NOT disambiguated in v1. **Future:** track in **OQ-W16-002** if security observability needs to distinguish SIGSEGV (potential exploit) from SIGKILL (OOM-killer) from real exit `-1`. POSIX 128+signum convention (e.g., 139 for SIGSEGV) considered for v2 ABI break. [known-limitation] |
| EC-010 | `internal_log.write_inner` encounters IO error (e.g., log directory missing, ENOSPC, permissions denied): `write_inner` returns `Err(io::Error)`; `write` (internal_log.rs:228-238) catches it via `if let Err(e) = self.write_inner(event)` and emits `eprintln!("factory-dispatcher: internal_log write failed ({}): {}", event.type_, e)` to stderr, then returns `()`. The `events.lock().push(event)` at host/mod.rs:113-115 runs independently and SUCCEEDS (in-memory queue is unaffected by internal_log IO failure). Host call return value unaffected (still `Ok(envelope)` for success path). **Test-harness note:** to exercise this path, tests MUST set `ctx.internal_log = Some(Arc::new(InternalLog::new(path_that_will_fail_io)))` — e.g., a directory that does not exist and cannot be created, or a read-only filesystem path. Tests using `HostContext::new` (which sets `internal_log: None`) do NOT exercise this path; they skip `log.write` entirely (see Postcondition 4 bifurcation). Postcondition 1 "exactly one event emitted" is best-effort: emitted-to-`emit_internal`-API, NOT guaranteed-delivered-to-`dispatcher-internal-*.jsonl`. See Postcondition 6 and **OQ-W16-003**. [best-effort-emit-witness] |
| EC-011 | `events: Mutex<Vec<InternalEvent>>` is poisoned by panic in another concurrent host call | `emit_internal` silently drops (host/mod.rs:113 `if let Ok(mut events) = self.events.lock()`); subsequent `drain_events` (host/mod.rs:102 `.expect("events mutex poisoned")`) panics dispatcher. Asymmetric write-vs-read poison handling. Track in **OQ-W16-004**. [known-limitation] |
| EC-012 | `caps.cwd_allow` is populated (e.g., `cwd_allow = ["/tmp/safe"]`) but subprocess `cwd` is set to a path NOT in `cwd_allow` (e.g., `ctx.cwd = "/etc"`) | **v1 known limitation:** `cwd_allow` is declared in `ExecSubprocessCaps` (registry.rs:83) but NOT enforced in `execute_bounded` (exec_subprocess.rs:248-250 uses `ctx.cwd` directly — `if !cwd.as_os_str().is_empty() { command.current_dir(cwd); }` — without consulting `caps.cwd_allow` at all). Subprocess `cwd` is `ctx.cwd` regardless of `cwd_allow` content. No denial event is emitted for a cwd outside the allow-list. Track enforcement decision in **OQ-W16-007** (v1: no-op as documented; v2 candidate: enforce with `cwd_not_on_allow_list` denial reason). [known-limitation] |
| EC-013A | `timeout_ms = 0` | Deadline is set to `Instant::now() + Duration::from_millis(0)` at exec_subprocess.rs:270. On the first poll iteration at the `if Instant::now() >= deadline` check (exec_subprocess.rs:292), the deadline has already expired (or expires within the same microsecond); subprocess is killed via `child.kill()` and `child.wait()` at exec_subprocess.rs:293-294; returns `Err(TIMEOUT -2)`. No subprocess output is collected. **v1 has no 'unlimited timeout' mode** — `timeout_ms = 0` means "kill immediately", not "no timeout". [edge-case] |
| EC-013B | `max_output_bytes = 0` | After subprocess exits, `stdout_buf.len() > 0 as usize` is true if subprocess produces ANY output (even 1 byte); `truncated = true` fires immediately; returns `Err(OUTPUT_TOO_LARGE -3)`. If subprocess produces exactly zero bytes of stdout AND zero bytes of stderr, `truncated = false` and the success path proceeds with empty buffers. **v1 effectively means 'any output → OUTPUT_TOO_LARGE'** when `max_output_bytes = 0`. [edge-case] |
| EC-014 | `caps.env_allow` lists a variable name (e.g., `"MY_VAR"`) that is absent from `ctx.env_view` | At exec_subprocess.rs:243-247: `for name in &caps.env_allow { if let Some(val) = env_view.get(name) { command.env(name, val); } }` — the name is silently omitted from the subprocess environment; no event emitted; no error returned. Subprocess does NOT receive `MY_VAR`. **v1 best-effort env-forwarding:** a plugin cannot distinguish "name was set to empty value in dispatcher env" from "name was absent from dispatcher env". Track alongside `cwd_allow` enforcement decision in **OQ-W16-007**. [known-limitation] |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Capability passes; subprocess exits 0 | Exactly one `host.exec_subprocess.completed` event; `exit_code=0` | happy-path |
| Capability passes; subprocess exits 1 | Exactly one `host.exec_subprocess.completed` event; `exit_code=1` | happy-path |
| Outcome enum (success): exit_code=0 invocation | Exactly one event; payload exit_code=0; host-stamped outcome='success' (per Postcondition 2 mapping; ADR-015 D-15.3 enrichment) | host-stamping |
| Outcome enum (failure): exit_code=1 invocation | Exactly one event; payload exit_code=1; host-stamped outcome='failure' | host-stamping |
| Capability check fails | `internal.capability_denied` emitted; `host.exec_subprocess.completed` NOT emitted | error |
| Subprocess timeout | Returns `Err(TIMEOUT -2)`; NO event emitted in v1 per Postcondition 5; `host.exec_subprocess.completed` NOT emitted | error |
| Subprocess output exceeds `max_output_bytes` (EC-005A path: stdout or stderr > max_output_bytes after subprocess exits; exec_subprocess.rs:278-283) | Returns `Err(OUTPUT_TOO_LARGE -3)`; **NO event emitted in v1** per Postcondition 5; `host.exec_subprocess.completed` NOT emitted | error |
| Subprocess spawn fails (or pipe take/write fails or try_wait error) | Returns `Err(INTERNAL_ERROR -99)`; NO event emitted in v1 per Postcondition 5 (spawn at :252, stdin take/write at :258/:259-262, stdout/stderr take at :267-268, try_wait at :299); `host.exec_subprocess.completed` NOT emitted | error |
| Successful completion | Event payload contains all 8 fields with correct types | edge-case |
| Subprocess terminated by SIGKILL: `command = "bash"`, `args = ["-c", "kill -9 $$"]`; capability-passes; subprocess receives SIGKILL from itself → `child.wait()` returns `Ok(Some(status))` with `status.code() == None` (signal-death) | `host.exec_subprocess.completed` event emitted with `exit_code = -1`, `outcome = 'failure'`, `terminated_by_signal: None` (v1 schema does not expose signal field per OQ-W16-002 deferral); host call returns `Ok(envelope)`. Witnesses EC-009. | signal-death-witness |
| `internal_log.write_inner` IO error: `ctx.internal_log = Some(InternalLog::new(<directory that does not exist and cannot be created>))`; capability passes; subprocess exits 0 | `write_inner` returns `Err(io::Error)`; `write` (internal_log.rs:228-238) emits `eprintln!("factory-dispatcher: internal_log write failed (host.exec_subprocess.completed): ...")` to stderr; `events.lock().push(event)` at host/mod.rs:113-115 SUCCEEDS; host call returns `Ok(envelope)`. The event IS in the in-memory queue; it is absent from `dispatcher-internal-*.jsonl`. Witnesses EC-010 + Postcondition 6. **Test setup:** requires `internal_log: Some(InternalLog::new(missing_dir))` — NOT `HostContext::new` which sets `None`. | best-effort-emit-witness |
| `ctx.internal_log = None` (test-context via `HostContext::new`); capability passes; subprocess exits 0 | `emit_internal` at host/mod.rs:109-112 skips `log.write` entirely (None branch); `events.lock().push(event)` at host/mod.rs:113-115 runs; exactly one event in in-memory queue; no file written. Witnesses Postcondition 4 bifurcation (internal_log: None path). | test-context-no-log-witness |
| Subprocess output within `max_output_bytes`; encoded envelope size (`stdout_len + stderr_len + 12`) exceeds guest `result_buf_cap` | Returns `Err(OUTPUT_TOO_LARGE -3)` at exec_subprocess.rs:86-88 (envelope.len() > result_buf_cap check); NO event emitted (Postcondition 5 / EC-005B). Host call returns -3. `host.exec_subprocess.completed` NOT emitted. Witnesses EC-005B result_buf_cap-overflow path. | error |
| `events` Mutex poisoned by concurrent panic in another host call | `emit_internal` silently drops the event (host/mod.rs:113 `if let Ok` short-circuits to `else => no-op`); subsequent `drain_events` (host/mod.rs:102) panics; host call return value `Ok(envelope)` UNCHANGED for the emit-side. Witnesses EC-011. | mutex-poison-witness |

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
