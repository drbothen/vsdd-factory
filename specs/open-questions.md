---
document_type: open-questions-register
level: ops
version: "1.1"
status: active
producer: state-manager
timestamp: 2026-05-05T00:00:00Z
last_amended: 2026-05-08
d312_note: "BC-3.05.001/002/003 marked superseded_by ADR-015 in D-312 corrigendum (2026-05-06). See also OQ-W16-012."
---

# Open Questions Register

> Tracks unresolved binary-choice and decision-gate items that block downstream
> stories or epics. Each OQ has a named owner, an explicit acceptance criterion,
> and a resolution path. Numbered continuously; no resets across cycles.

---

## OQ-W16-001 — Resolve `vsdd.host.*` registry-prefix decision before E-10 Wave 1 ships

**Source:** gap-analysis-w16-subprocess.md §"How ADR-015 affects the telemetry gap" — see also gap-analysis line 326 ("Resolution tracked in **OQ-W16-001**") for the bidirectional anchor.
**Status:** OPEN
**Owner:** SS-01 implementer or E-10 Wave 1 architect
**Filed:** 2026-05-05

**Question:** ADR-015 D-15.2 registry table (lines 317-332) does not include `vsdd.host.*`. The gap analysis of `host::exec_subprocess` (gap-analysis-w16-subprocess.md §5) proposes `vsdd.host.exec_subprocess.completed.v1` as the event name for the host-emit-fix story, with fallback `vsdd.dispatcher.subprocess_completed.v1`.

**Acceptance criterion (binary):**
- (a) ADR-015 D-15.2 registry amended to include `vsdd.host.* | <category>` AND the canonical event.name for the host-emit-fix is `vsdd.host.exec_subprocess.completed.v1` (mapped to that prefix's category) BEFORE E-10 Wave 1 host-emit-fix story merges, OR
- (b) the event.name uses `vsdd.dispatcher.subprocess_completed.v1` exactly (NOT a different unregistered prefix; NOT `vsdd.unknown.foo.bar.v1` or other variants).

**Why this matters:** unregistered event prefixes resolve to `event.category = "unknown"` per D-15.2.b; ADR-015 Wave 3 acceptance criterion 2 (line 634) installs an `unknown_category_events` Grafana alert that would actively fire.

**Resolution path:** PR amending ADR-015 to add `vsdd.host.* | lifecycle` (or another category as appropriate) OR explicit choice (b) noted in E-10 Wave 1 story acceptance criteria with rationale.

---

## OQ-W16-002 — Signal-death disambiguation in `host.exec_subprocess.completed` (exit_code=-1 collision)

**Source:** BC-1.05.036 EC-009 added in D-281 pass-38 HIGH-P38-003 closure.
**Status:** OPEN
**Owner:** SS-01 implementer or E-9 Wave 1 architect
**Filed:** 2026-05-05

**Question:** Should `host.exec_subprocess.completed` events distinguish signal-death (SIGSEGV/SIGKILL/SIGINT) from a literal exit code `-1`? Current v1 implementation in `execute_bounded` (`status.code().unwrap_or(-1)`) substitutes -1 for both signal-death (Unix) and literal `_exit(-1)` from C — the two cases are indistinguishable in the emitted event.

**Acceptance criterion (binary):**
- (a) v1 retains `-1` substitution as documented in BC-1.05.036 EC-009 + Postcondition 1 footnote — no change; OR
- (b) future ABI break adopts POSIX 128+signum convention (e.g., 139 for SIGSEGV, 137 for SIGKILL) with separate `terminated_by_signal: Option<i32>` payload field; BC-1.05.036 updated accordingly.

**Why this matters:** Security observability — `exit_code=-1` from SIGSEGV (potential exploit) vs SIGKILL (OOM-killer) vs benign `_exit(-1)` are collapsed into one signal. Grafana alert rules cannot distinguish these cases.

**Resolution path:** Default v1 = (a). Revisit at next major dispatcher ABI break (v2) when `terminated_by_signal: Option<i32>` or POSIX 128+signum convention can be evaluated without breaking existing consumers.

**Decision needed by:** NEXT major dispatcher ABI break (v2)

---

## OQ-W16-003 — Observability of `emit_internal` IO failures (silent-drop vs fallback)

**Source:** BC-1.05.036 Postcondition 6 + EC-010 added in D-281 pass-38 MED-P38-002 closure.
**Status:** OPEN
**Owner:** SS-01 implementer or E-10 Wave 1 architect
**Filed:** 2026-05-05

**Question:** Should v2 expose emit-side IO failures (FileSink write error, broken pipe, ENOSPC) via a fallback channel (stderr, health metric, counter) rather than silently dropping? Current v1 in `HostContext::emit_internal` ignores `log.write(&event)` return value — failures are unobservable.

**Acceptance criterion (binary):**
- (a) v1 retains silent-drop per Postcondition 6 — no change; OR
- (b) future version adds a fallback channel (e.g., write drop-count to `dispatcher-internal-*.jsonl` stderr, or increment a `emit_drop_count` metric counter accessible via health endpoint) — BC-1.05.036 Postcondition 6 updated to specify the fallback.

**Why this matters:** If the FileSink fills or breaks, all events are silently dropped with no operator signal. In a security context (audit trail for `vsdd.capability.denied.*`), silent emit failures are a security observability gap.

**Resolution path:** Default v1 = (a) per Postcondition 6 best-effort semantics. Revisit if OTel Collector integration (E-10 Wave 1) introduces a queue-depth metric that makes drops detectable.

**Decision needed by:** E-10 Wave 1 FileSink wiring story (S-10.02)

---

## OQ-W16-004 — Mutex poison asymmetry: emit_internal silent vs drain_events panic

**Source:** BC-1.05.036 EC-011 added in D-281 pass-38 MED-P38-003 closure.
**Status:** OPEN
**Owner:** SS-01 implementer
**Filed:** 2026-05-05

**Question:** Should v2 harmonize the Mutex poison handling in `emit_internal` (currently silently drops on poison per `if let Ok`) and `drain_events` (currently panics on poison per `.expect("events mutex poisoned")`)? Current asymmetry means: poisoned write → silent event loss; subsequent read → dispatcher panic.

**Acceptance criterion (binary):**
- (a) v1 retains asymmetric behavior as documented in BC-1.05.036 EC-011 — no change; OR
- (b) future version harmonizes: either both panic (fail-fast), both recover (replace poisoned mutex), or both log-and-drop. BC-1.05.036 EC-011 + Purity Classification updated accordingly.

**Why this matters:** A panic in any concurrent host call poisons the shared `events` Mutex. All subsequent `emit_internal` calls silently drop until the panic propagates to `drain_events` which then panics the entire dispatcher. The window between write-silent-drop and read-panic creates a diagnostic blind spot.

**Resolution path:** Default v1 = (a) per EC-011 known-limitation. Harmonization decision may be folded into E-10 Wave 1 emit infrastructure work.

**Decision needed by:** E-10 Wave 1 emit infrastructure story

---

## OQ-W16-005 — Distinguish directory-cmd (canonicalize succeeds) from missing-cmd (canonicalize fails) in exec_subprocess

**Source:** BC-1.05.035 EC-009 added in D-281 pass-38 LOW-P38-001 closure; HIGH-P39-001 (pass-39) identified this OQ as filed in three artifacts but absent from register.
**Status:** OPEN
**Owner:** SS-01 implementer or E-9 Wave 1 architect
**Filed:** 2026-05-05

**Question:** Should the dispatcher distinguish a `cmd` that resolves to a directory (canonicalize-succeeds) from a `cmd` that resolves to a missing or unspawnable file? Currently both produce CAPABILITY_DENIED via different ladder steps and observability semantics differ (binary_canonicalize_failed event for missing; INTERNAL_ERROR with no event for directory-spawn-fail).

**Acceptance criterion (binary):**
- (a) v1 retains current behavior — directory cmd reaches `Command::new` in `execute_bounded` and spawn fails returning INTERNAL_ERROR (-99) with no emit_denial; documented as known-limitation in BC-1.05.035 EC-009. OR
- (b) v2 adds a pre-spawn `Path::is_file()` check at line 152.5 (after canonicalize success, before allow-check) emitting `emit_denial(ctx, cmd, "binary_not_executable", details)` for directory or non-file paths.

**Why this matters:** v1 option (a) masks broken-capability-config — if the allow-list contains a directory name (e.g., `bin`), the dispatcher silently returns INTERNAL_ERROR (-99) with no observability event, rather than CAPABILITY_DENIED with a `binary_canonicalize_failed` or `binary_not_executable` emit. Security observability gap: no Grafana alert can detect this misconfiguration.

**Resolution path:** Default v1 = (a) per EC-009 known-limitation. Revisit at next major dispatcher ABI break, or earlier if directory-cmd masking surfaces as a security observability gap in production.

**Decision needed by:** NEXT major dispatcher ABI break; or earlier if directory-cmd masking surfaces as a security observability gap.

---

## OQ-W16-006 — NFD/NFC Unicode normalization on macOS HFS+ for non-ASCII binary allow-list entries

**Source:** BC-1.05.035 Architecture Anchors cross-platform note added in D-281 pass-38 LOW-P38-003 closure.
**Status:** OPEN
**Owner:** SS-01 implementer or security reviewer
**Filed:** 2026-05-05

**Question:** If non-ASCII binary allow-list entries are introduced (e.g., paths containing Japanese, Arabic, or other Unicode characters), `Path::canonicalize` on macOS HFS+ may return NFD-normalized paths that are not byte-equal to NFC-normalized allow-list entries. The `binary_allowed` byte-exact comparison in `host/exec_subprocess.rs::binary_allowed` would then silently deny valid allow-listed binaries.

**Acceptance criterion (binary):**
- (a) W-16 allow-list entries remain ASCII-only (typical `bash`, `/usr/bin/bash` etc.) — non-issue; OQ remains OPEN but dormant; OR
- (b) non-ASCII allow-list entries are introduced → BC-1.05.035 `binary_allowed` implementation MUST perform Unicode normalization (NFC) on both the canonical path and the allow-list entry before comparison; BC updated accordingly.

**Why this matters:** Silent CAPABILITY_DENIED for allow-listed binaries due to normalization mismatch would be a security-masquerading-as-correctness defect — the allow-list appears to match but doesn't.

**Resolution path:** Default = (a) for W-16 scope (all S-9.0N stories use ASCII-only `bash` binary). Monitor if allow-list entries ever gain non-ASCII content.

**Decision needed by:** Any story introducing non-ASCII binary allow-list entries

---

## OQ-W16-007 — `cwd_allow` enforcement and `env_allow` absent-name behavior in exec_subprocess

**Source:** BC-1.05.036 EC-012 (cwd_allow unenforcement) and EC-014 (env_allow silent-skip) added in D-283 pass-40 HIGH-P40-004 and MED-P40-003 closure.
**Status:** OPEN
**Owner:** SS-01 implementer or E-9 Wave 1 architect
**Filed:** 2026-05-05

**Question (two related issues):**

**(a) cwd_allow:** `ExecSubprocessCaps.cwd_allow: Vec<String>` is declared in `registry.rs::ExecSubprocessCaps` but the `execute_bounded` function in `exec_subprocess.rs` uses `ctx.cwd` directly — `if !cwd.as_os_str().is_empty() { command.current_dir(cwd); }` — without consulting `caps.cwd_allow`. The field is stored in the registry but has no enforcement effect in v1. Operators populating `cwd_allow` to restrict subprocess working directory receive no actual enforcement.

**(b) env_allow absent-name:** In `execute_bounded`, the `env_allow` loop (`for name in &caps.env_allow { if let Some(val) = env_view.get(name) { command.env(name, val); } }`) — names in `env_allow` that are absent from `ctx.env_view` are silently omitted. No event is emitted. A plugin cannot distinguish "variable was set to empty string in dispatcher env" from "variable was absent from dispatcher env".

**Acceptance criterion (binary for each):**

For `cwd_allow` enforcement:
- (a) v1 retains no-op as documented in BC-1.05.036 EC-012 — `cwd_allow` field is config-only with no enforcement effect; OR
- (b) v2 adds enforcement: after canonicalize succeeds and binary_allow passes, check `ctx.cwd` against `caps.cwd_allow`; if `cwd_allow` is non-empty and `ctx.cwd` is not on the list → `emit_denial(ctx, cmd, "cwd_not_on_allow_list", details)` → `CAPABILITY_DENIED` (-1); BC-1.05.036 EC-012 updated accordingly.

For `env_allow` absent-name:
- (a) v1 retains silent-omit as documented in BC-1.05.036 EC-014 — best-effort env-forwarding; OR
- (b) v2 adds a debug-level `internal.env_var_not_forwarded` event (not an error; informational) when a listed name is absent from env_view; BC-1.05.036 EC-014 updated accordingly.

**Why this matters:** `cwd_allow` enforcement: operators may believe they are restricting subprocess working directories when no enforcement is occurring — a silent security policy gap. `env_allow` absent-name: plugins cannot audit which env vars were actually forwarded, creating observability gaps for debugging permission issues.

**Resolution path:** Default v1 = (a) for both, as documented. `cwd_allow` enforcement is the higher-priority item; it should be addressed in any security hardening pass on exec_subprocess. `env_allow` absent-name can be addressed opportunistically.

**Decision needed by:** Next exec_subprocess security hardening story (S-9.07 or follow-on)

---

## OQ-W16-008 — Host-call panic handling and `args` non-UTF-8 lossy conversion in exec_subprocess

**Source:** BC-1.05.035 EC-014 (panic propagation — no catch_unwind) and EC-012 (args lossy UTF-8) added in D-283 pass-40 HIGH-P40-005 and MED-P40-001 closure.
**Status:** OPEN
**Owner:** SS-01 implementer or E-9 Wave 1 architect
**Filed:** 2026-05-05

**Question (two related issues):**

**(a) Host-call panic handling:** No `catch_unwind` wraps the host-call surface in `exec_subprocess.rs`. If any call in the host-call body panics (e.g., due to a future `unwrap()` introduced in a helper, or an unexpected stdlib programming error), the panic propagates uncaught to the wasmtime host-call boundary. Wasmtime converts it to a wasmtime `Trap`. The `internal.host_function_panic` event class (`internal_log::INTERNAL_HOST_FUNCTION_PANIC`) is declared as a `pub const` but has zero emit call sites in `exec_subprocess` or any other host function (grep of `crates/factory-dispatcher/src/` finds only the const declaration in `internal_log.rs::INTERNAL_HOST_FUNCTION_PANIC` and the re-export in `lib.rs::pub use internal_log`). The `HostContext.internal_log` field doc-comment (`host/mod.rs::HostContext::internal_log` field) documents that the field exists to support both `internal.capability_denied` and `internal.host_function_panic` emission — this is field-purpose documentation, not a TODO; the emit call for panic events has simply not been written. Panics in host calls are therefore unobservable in v1.

**(b) `args` non-UTF-8 lossy conversion:** In `exec_subprocess.rs::register`, argument bytes are decoded using `String::from_utf8_lossy(&bytes[i..i+len]).into_owned()`. Non-UTF-8 bytes are silently replaced with U+FFFD replacement characters. This is asymmetric with `cmd` strict UTF-8 enforcement (`read_wasm_string` returns `INVALID_ARGUMENT` for non-UTF-8 `cmd`). A plugin can inadvertently pass mangled arguments to a subprocess with no error signal.

**Acceptance criterion (binary for each):**

For panic handling:
- (a) v1 retains panic-propagates-as-Trap behavior as documented in BC-1.05.035 EC-014 and Panic semantics note — no change; OR
- (b) future version wraps the host-call body in `std::panic::catch_unwind`; on panic: emits `INTERNAL_HOST_FUNCTION_PANIC` event via `ctx.emit_internal` (best-effort); returns `codes::INTERNAL_ERROR` (-99) to the plugin rather than propagating a Trap. BC-1.05.035 Postcondition/panic note and EC-014 updated accordingly.

For `args` non-UTF-8:
- (a) v1 retains lossy substitution as documented in BC-1.05.035 EC-012 — no change; OR
- (b) future version symmetrizes: validate each arg with `String::from_utf8` (strict); return `INVALID_ARGUMENT` (-4) on non-UTF-8 arg bytes; BC-1.05.035 EC-012 updated accordingly.

**Why this matters:** Panic handling: unhandled panics in host calls produce wasmtime Traps which may crash or destabilize the plugin invocation without any audit trail or operator signal. The `INTERNAL_HOST_FUNCTION_PANIC` event class exists in the codebase specifically to address this — its absence from exec_subprocess is a known gap. `args` non-UTF-8: silent mangling may cause cryptic subprocess failures when non-UTF-8 plugin-controlled data is passed as arguments; the asymmetry with `cmd` encoding creates a footgun for plugin authors.

**Resolution path:** Default v1 = (a) for both. Panic handling (b) is the higher-priority item for security observability; consider addressing in the story that first implements `INTERNAL_HOST_FUNCTION_PANIC` emission (the const is `internal_log::INTERNAL_HOST_FUNCTION_PANIC`; the `HostContext::internal_log` field in `host/mod.rs` exists to carry the log handle to host functions for exactly this purpose). `args` symmetry (b) is a usability improvement; can be addressed in a future ABI-compatible pass (no ABI break required — returning INVALID_ARGUMENT for invalid args was already the behavior if the length-prefix encoding was malformed).

**Decision needed by:** Any story implementing `INTERNAL_HOST_FUNCTION_PANIC` event emission (const: `internal_log::INTERNAL_HOST_FUNCTION_PANIC`; `HostContext::internal_log` field in `host/mod.rs` carries the handle); or next security hardening pass on exec_subprocess

---

## OQ-W16-009 — Silent stdout/stderr under-count from `read_to_end` IO error in exec_subprocess

**Source:** BC-1.05.036 §Edge Cases EC-015 added in D-293 pass-50 HIGH-P50-001 closure.
**Status:** OPEN
**Owner:** SS-01 implementer or E-9 Wave 1 architect
**Filed:** 2026-05-06

**Question:** In `execute_bounded`, `let _ = stdout.read_to_end(&mut stdout_buf)` and `let _ = stderr.read_to_end(&mut stderr_buf)` discard their `io::Result` via `let _ =`. If a kernel pipe IO error occurs mid-read (e.g., EIO on a tmpfs-backed pipe, disk-backed swap failure, or kernel pipe buffer corruption), `read_to_end` halts with a partial buffer. Execution continues: the `truncated` check evaluates against the partial buffer; if the partial buffer is under `max_output_bytes`, `truncated = false`; the success path completes; `host.exec_subprocess.completed` is emitted with `stdout_bytes`/`stderr_bytes` counts reflecting only the partially-read bytes, and `outcome = 'success'`. The plugin caller receives a success envelope with under-counted byte fields and has no mechanism to detect that a pipe IO error occurred.

**Acceptance criterion (binary):**
- (a) v1 retains silent-discard in `execute_bounded` (stdout/stderr `read_to_end` results discarded via `let _ =`) — `stdout_bytes`/`stderr_bytes` remain best-effort counts as documented in BC-1.05.036 §Postconditions Postcondition 2 and §Edge Cases EC-015; OR
- (b) future version propagates `read_to_end` errors: on `Err`, returns `Err(codes::INTERNAL_ERROR)` (or a new `IO_ERROR` code) so the caller can distinguish a complete-read success from an IO-truncated-read. BC-1.05.036 Postcondition 2 and EC-015 updated accordingly.

**Why this matters:** Silent pipe IO errors produce success envelopes with under-counted `stdout_bytes`/`stderr_bytes`. In a security audit context, a tool relying on `stdout_bytes` for completeness verification would be silently misled. The `outcome='success'` stamp is affirmatively misleading when the underlying read failed partway through.

**Resolution path:** Default v1 = (a) per EC-015 known-limitation. Option (b) is the preferred long-term behavior; it can be addressed in any future hardening pass that revisits `execute_bounded` IO error handling without requiring an ABI break (the error is internal to the dispatcher; plugin-visible behavior would change only for the failure case, which currently returns INTERNAL_ERROR anyway in other error paths).

**Decision needed by:** Next exec_subprocess hardening story or E-9 Wave 1 INTERNAL_ERROR-path audit

---

## OQ-W16-010 — `child.kill()` / `child.wait()` cleanup-phase hang with no secondary deadline in exec_subprocess

**Source:** BC-1.05.036 §Edge Cases EC-016 added in D-293 pass-50 HIGH-P50-002 closure.
**Status:** OPEN
**Owner:** SS-01 implementer or E-9 Wave 1 architect
**Filed:** 2026-05-06

**Question:** In `execute_bounded`, both the TIMEOUT cleanup branch and the stdin-fail cleanup branch execute `let _ = child.kill(); let _ = child.wait()`, discarding `io::Result` from both calls via `let _ =`. If `child.kill()` returns `Err` (e.g., child already exited, kernel error, permission denied) AND `child.wait()` subsequently blocks indefinitely (e.g., child in NFS D-state, kernel-level uninterruptible wait, or zombie process that cannot be reaped), `execute_bounded` hangs inside `child.wait()` with no secondary deadline. The host call never returns to the wasm caller; no TIMEOUT (-2) is reported; no event is emitted. The TIMEOUT enforcement (`if Instant::now() >= deadline`) covers only the deadline check — post-TIMEOUT cleanup has no second timeout. The same hazard applies to the stdin-fail cleanup branch.

**Acceptance criterion (binary):**
- (a) v1 retains no-secondary-deadline behavior as documented in BC-1.05.036 §Postconditions Postcondition 5 footnote and §Edge Cases EC-016 — both `child.kill()` and `child.wait()` errors are silently discarded; OR
- (b) future version wraps the cleanup sequence with a secondary deadline (e.g., `child.wait_timeout(Duration::from_millis(500))` or a separate thread with a timeout); on secondary deadline expiry, logs a `internal.host_function_cleanup_timeout` event (best-effort) and returns `Err(codes::TIMEOUT)` to the wasm caller anyway. BC-1.05.036 Postcondition 5 footnote and EC-016 updated accordingly.

**Why this matters:** A subprocess in D-state (uninterruptible wait, common on NFS mounts or under swap pressure) cannot be killed via SIGKILL and will cause `child.wait()` to block indefinitely. In a production dispatcher handling plugin invocations, this hangs a worker thread indefinitely — the wasm caller receives no response, and the hang is invisible from the caller's perspective. The TIMEOUT mechanism, intended to bound host-call duration, is defeated for any subprocess that enters D-state during the cleanup phase.

**Resolution path:** Default v1 = (a) per EC-016 known-limitation. Option (b) requires introducing `wait_timeout` or a reaper thread pattern. The simplest approach is to use `std::process::Child::try_wait` in a short loop after `kill()`, falling back to returning `TIMEOUT` after a secondary deadline (e.g., 500ms). This prevents indefinite hangs at the cost of potential zombie processes on NFS-backed filesystems.

**Decision needed by:** Next exec_subprocess hardening story or E-9 Wave 1 timeout-enforcement audit

---

## OQ-W16-011 — `VSDD_DEBUG_LOG` env var vs `debug_log_enabled` config key precedence for debug stream gate

**Source:** BC-1.12.002 EC-007 (surfaced during E-10 Phase 1a BC authorship, 2026-05-06).
**Status:** RESOLVED 2026-05-06
**Owner:** Architect (D-311 routing burst)
**Filed:** 2026-05-06
**Resolved:** 2026-05-06 (D-311)

**Question:** ADR-015 D-15.1 states the debug file is "gated by the `VSDD_DEBUG_LOG=1` environment variable." The `observability-config.toml` schema (OQ-1, resolved in SS-03-event-emission.md) includes a `debug_log_enabled` config key (default `false`). When BOTH are present, which takes precedence?

### Resolution

**Option chosen:** (c) — 12-factor override semantics: env var dominates when present; config key governs when env var absent.

**Canonical precedence rule:**

When `VSDD_DEBUG_LOG=1` is set in the dispatcher process environment, the debug stream
is enabled regardless of the `debug_log_enabled` config key value. When `VSDD_DEBUG_LOG`
is absent from the environment, the `debug_log_enabled` config key governs:
`debug_log_enabled = true` enables the debug stream; `debug_log_enabled = false`
(the default) disables it. Any non-`"1"` value for `VSDD_DEBUG_LOG` (including
`"0"`, `"true"`, `"false"`) does NOT enable the stream via the env var path — the
config key governs in that case as if the env var were unset.

**Source-of-truth grounding:**

This is a forward-implementation contract. There is NO current implementation of
`VSDD_DEBUG_LOG` reading or `debug_log_enabled` config parsing in the dispatcher
codebase. Verification:
- `crates/factory-dispatcher/src/sinks/mod.rs` `ObservabilityConfig` struct has only
  `schema_version` and `sinks` fields (the v1 schema). No `debug_log_enabled` field.
- `grep` of all dispatcher `.rs` files for `VSDD_DEBUG_LOG` and `debug_log_enabled`
  returns zero matches.

The authoritative forward-spec source is `SS-03-event-emission.md` § `observability-config.toml`
Schema (OQ-1 resolution):
- Schema comment: `"# Overridden by VSDD_DEBUG_LOG=1 env var (env var takes precedence)."`
- Normative prose: `"The VSDD_DEBUG_LOG=1 environment variable ALWAYS overrides
  debug_log_enabled; the env var takes precedence."`
- Cross-Cutting section: `"The debug file is active only when VSDD_DEBUG_LOG=1 is set
  (or debug_log_enabled = true in config)."`

These three passages together establish the 12-factor pattern: env var dominates when
present; config key governs when env var absent. This is consistent with ADR-015 D-15.1
("gated by the VSDD_DEBUG_LOG=1 environment variable") because D-15.1 describes the
env-var activation path — it does not prohibit the config key from also being a gate.

**Downstream amendment required:** BC-1.12.002 EC-007 used hedging language ("MAY also
enable") pending this resolution. BC-1.12.002 has been amended in D-311 to replace
"MAY" with "MUST" and to add a new Invariant documenting the two-key gate semantics.

**Decision needed by:** S-10.02 (Wave 1: FileSink single-stream wiring + debug-stream gate implementation)

---

## OQ-W16-012 — BC-3.05.001/002/003 supersession timing: close in D-312 or defer to Wave 5 cleanup

**Source:** D-312 corrigendum (2026-05-06); process-gap discovery during D-311 ID-collision correction.
**Status:** RESOLVED 2026-05-06 (in D-312)
**Owner:** Architect (D-312 corrigendum burst)
**Filed:** 2026-05-06
**Resolved:** 2026-05-06 (D-312)

**Question:** BC-3.05.001/002/003 document pre-ADR-015 multi-sink behavior
(`load_builds_file_sink_from_parsed_config`, `registry_fans_events_to_file_sinks`,
`ten_events_arrive_with_correct_attribute_mapping`). ADR-015 (accepted 2026-05-04)
explicitly states in its Consequences section: "BC-3.* contracts covering multi-sink
fan-out, DLQ, and sink health events are withdrawn or revised." Should supersession
be marked now (D-312) or deferred to Wave 5 (S-10.09 SS-03 spec rewrite)?

### Resolution

**Option chosen:** (b) — mark superseded NOW in D-312 corrigendum.

**Rationale:** ADR-015 is ACCEPTED and binding. The three BCs describe behavior
that ADR-015 explicitly retires: SinkRegistry-based FileSink loading (BC-3.05.001),
Router/SinkRegistry multi-sink fan-out (BC-3.05.002), and the sink-otel-grpc gRPC
integration (BC-3.05.003). Wave 5 (S-10.09) delivers the PHYSICAL removal of the
crates and types; the BEHAVIORAL supersession (the decision that these contracts
no longer govern production behavior) is already established by ADR-015 acceptance.
Deferring the supersession marker to Wave 5 would leave three `lifecycle_status:
active` BCs in the BC-INDEX for contracts that describe retired architecture —
an active drift source under POLICY 1 spirit.

**Action taken in D-312:**
- BC-3.05.001 frontmatter: `lifecycle_status: retired`, `superseded_by: "ADR-015"`,
  `deprecated: 2026-05-06`, `retired: 2026-05-06`.
- BC-3.05.002 frontmatter: same pattern.
- BC-3.05.003 frontmatter: same pattern.
- BC bodies preserved verbatim per POLICY 1 (append-only numbering).

**Downstream:** State-manager Phase 2 must update BC-INDEX.md status column for
BC-3.05.001, BC-3.05.002, BC-3.05.003 from `draft` to `retired`.

**Decision needed by:** N/A — resolved in D-312.

## Changelog

- v1.1 (2026-05-08): TD-VSDD-091 stable-anchor migration sweep (Chunk 2) — 13 body cites migrated from file.ext:NNN line anchors to stable symbol anchors across OQ-W16-002 through OQ-W16-010.
