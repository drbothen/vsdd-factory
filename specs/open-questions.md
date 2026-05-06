---
document_type: open-questions-register
level: ops
version: "1.0"
status: active
producer: state-manager
timestamp: 2026-05-05T00:00:00Z
last_amended: 2026-05-05
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

**Question:** Should `host.exec_subprocess.completed` events distinguish signal-death (SIGSEGV/SIGKILL/SIGINT) from a literal exit code `-1`? Current v1 implementation at exec_subprocess.rs:286 (`status.code().unwrap_or(-1)`) substitutes -1 for both signal-death (Unix) and literal `_exit(-1)` from C — the two cases are indistinguishable in the emitted event.

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

**Question:** Should v2 expose emit-side IO failures (FileSink write error, broken pipe, ENOSPC) via a fallback channel (stderr, health metric, counter) rather than silently dropping? Current v1 at host/mod.rs:109-116 ignores `log.write(&event)` return value — failures are unobservable.

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

## OQ-W16-006 — NFD/NFC Unicode normalization on macOS HFS+ for non-ASCII binary allow-list entries

**Source:** BC-1.05.035 Architecture Anchors cross-platform note added in D-281 pass-38 LOW-P38-003 closure.
**Status:** OPEN
**Owner:** SS-01 implementer or security reviewer
**Filed:** 2026-05-05

**Question:** If non-ASCII binary allow-list entries are introduced (e.g., paths containing Japanese, Arabic, or other Unicode characters), `Path::canonicalize` on macOS HFS+ may return NFD-normalized paths that are not byte-equal to NFC-normalized allow-list entries. The `binary_allowed` byte-exact comparison at host/exec_subprocess.rs:191 would then silently deny valid allow-listed binaries.

**Acceptance criterion (binary):**
- (a) W-16 allow-list entries remain ASCII-only (typical `bash`, `/usr/bin/bash` etc.) — non-issue; OQ remains OPEN but dormant; OR
- (b) non-ASCII allow-list entries are introduced → BC-1.05.035 `binary_allowed` implementation MUST perform Unicode normalization (NFC) on both the canonical path and the allow-list entry before comparison; BC updated accordingly.

**Why this matters:** Silent CAPABILITY_DENIED for allow-listed binaries due to normalization mismatch would be a security-masquerading-as-correctness defect — the allow-list appears to match but doesn't.

**Resolution path:** Default = (a) for W-16 scope (all S-9.0N stories use ASCII-only `bash` binary). Monitor if allow-list entries ever gain non-ASCII content.

**Decision needed by:** Any story introducing non-ASCII binary allow-list entries
