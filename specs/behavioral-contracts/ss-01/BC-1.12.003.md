---
document_type: behavioral-contract
level: L3
version: "1.1"
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
capability: "CAP-030"
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

# Behavioral Contract BC-1.12.003: factory-dispatcher::resource_attributes::startup_stamping — all 15 OTel Resource attributes stamped at dispatcher startup with deterministic fallback cascade; no Resource field absent or null

## Description

Per ADR-015 D-15.2, every event emitted to `events-*.jsonl` carries a set of
**Resource attributes** — fields that describe the dispatcher process and its
environment, stable across all events in one invocation. These Resource attributes
are stamped ONCE at dispatcher startup, before the first event is emitted.

This BC governs the startup computation of all 15 Resource fields defined in
ADR-015 D-15.2, including the D-15.2.c fallback cascade policy for derived fields
(`vcs.repository.url.full`, `project.id`, `worktree.id`, `host.id`, and all
fields derived from the VCS URL). No Resource field may be absent or `null` in any
emitted event.

This is a future-implementation contract for S-10.03 (Wave 1 Resource-attribute
enrichment). Canonical Test Vectors describe post-Wave-1 behavior and are designed
as future-implementation witnesses — distinguishing a correct startup-stamping
implementation from a reasonable-but-wrong "partially stamped" or "null-on-fallback"
misimplementation.

## Preconditions

1. The dispatcher process is starting up (before the first `host::emit_event` call).
2. The process environment is accessible: `CLAUDE_PROJECT_DIR`, `CI`, `VSDD_TRACE_ID`,
   and platform-specific env vars are readable.
3. The current working directory is the worktree root (set via `CLAUDE_PROJECT_DIR`
   per BC-1.08.004, falling back to `current_dir()`).

## Postconditions

1. All 15 Resource attribute fields listed in ADR-015 D-15.2 Resource attributes table
   are computed and stored in a startup-time `ResourceContext` struct (or equivalent):

   | Field | Value |
   |-------|-------|
   | `service.name` | `"vsdd-factory"` (literal constant; no env substitution) |
   | `service.namespace` | basename of `CLAUDE_PROJECT_DIR` (e.g., `"my-project"` for `/home/user/my-project`) |
   | `service.instance.id` | UUIDv4 generated at startup via `uuid::Uuid::new_v4()` (fresh per invocation; not persisted) |
   | `service.version` | `env!("CARGO_PKG_VERSION")` (compile-time constant; the dispatcher's own version) |
   | `deployment.environment.name` | `"ci"` if `CI=true` in env; else `"local-dev"` |
   | `host.name` | result of `gethostname()` syscall |
   | `host.id` | machine-stable ID per D-15.2.c cascade (see Postcondition 3) |
   | `os.type` | `"macos"` \| `"linux"` \| `"windows"` (compile-time `cfg` target OS) |
   | `process.pid` | `std::process::id()` |
   | `vcs.repository.url.full` | result of `git remote get-url origin` cascade (see Postcondition 2) |
   | `vcs.repository.name` | repo basename parsed from `vcs.repository.url.full` |
   | `vcs.provider.name` | `"github"` \| `"gitlab"` \| `"other"` parsed from URL |
   | `vcs.owner.name` | org or user parsed from URL |
   | `worktree.id` | first 12 hex chars of SHA-256 of absolute worktree path |
   | `schema_url` | `"https://vsdd-factory.dev/schemas/events/v2"` (literal; process-level baseline) |

   **Source-of-truth verification (TD-VSDD-093):** `service.name = "vsdd-factory"` is a
   literal string per ADR-015 D-15.2 Resource attributes table, row 1. `service.version =
   env!("CARGO_PKG_VERSION")` is per ADR-015 D-15.2 Resource attributes table, row 4.
   `schema_url = "https://vsdd-factory.dev/schemas/events/v2"` is per ADR-015 D-15.2
   Resource attributes table, row 15 (the process-level baseline `schema_url`; distinct
   from the per-event `event.schema_url` field added in D-15.2.d).

2. **`vcs.repository.url.full` fallback cascade (D-15.2.c):**
   - Step 1: Run `git remote get-url origin` in the worktree directory.
   - Step 2: If the command succeeds and returns a non-empty URL, use it as
     `vcs.repository.url.full`.
   - Step 3: If the command fails (no remote, no git repo, bare clone, detached worktree,
     any error): fall back to `file://<absolute worktree path>` where `<absolute worktree
     path>` is the resolved `CLAUDE_PROJECT_DIR` (or `current_dir()` fallback).
   - Result: `vcs.repository.url.full` is ALWAYS a non-empty string (either a real remote
     URL or a `file://` URI). NULL or empty string is a postcondition violation.
   - `project.id`: SHA-256 of `vcs.repository.url.full` (computed AFTER cascade; always
     computable because `vcs.repository.url.full` is always set).
   - `worktree.id`: SHA-256 of absolute worktree path resolved at startup; first 12 hex
     characters used (not full SHA-256). Always computable from `cwd`.

3. **`host.id` fallback cascade (D-15.2.c):**
   - Step 1 (Linux): Read `/etc/machine-id` as a UTF-8 string, trim whitespace.
   - Step 2 (macOS): Run `ioreg -rd1 -c IOPlatformExpertDevice` and extract
     `IOPlatformUUID` value.
   - Step 3 (Windows): Read
     `HKLM\SOFTWARE\Microsoft\Cryptography\MachineGuid` registry key via the
     `winreg` crate (target-OS-gated; per OQ-2 resolution in ADR-015 v1.8:
     full Windows registry cascade ships in Wave 1, NOT stubbed to default).
   - Step 4 (terminal fallback): If all platform-specific steps fail (distroless
     containers, minimal environments, or `gethostname()` returns empty or errors):
     use the literal string `"unknown-host"`.
   - **Terminal fallback observable event:** When the literal `"unknown-host"` terminal
     default is reached, the dispatcher MUST emit a `vsdd.internal.host_id_fallback.v1`
     lifecycle event at startup carrying the dispatcher's PID and absolute cwd. This makes
     the collision-prone fallback observable (multiple containers resolving to `"unknown-host"`
     produce a visible signal). Per ADR-015 D-15.2.c: "when the terminal default is reached,
     the dispatcher MUST emit a `vsdd.internal.host_id_fallback.v1` lifecycle event."
   - No step in the cascade may produce NULL or empty string. Each step either produces a
     non-empty value or falls to the next step.

4. All 15 Resource attributes are computed BEFORE the first event is emitted.
   No event is emitted with missing or null Resource fields.

5. Resource attributes are IMMUTABLE for the dispatcher's process lifetime.
   They are stamped once at startup and attached to every subsequent event.
   A plugin cannot override Resource fields (ADR-015 D-15.3 — host-stamped
   values win unconditionally; see BC-1.12.004 for the override-visibility contract).

## Invariants

1. No Resource field is absent or `null` in any emitted event. Every field has a
   deterministic value (possibly a fallback) before the first event is emitted.
   (Per ADR-015 D-15.2: "No Resource field is allowed to be absent or `null`.")
2. `service.instance.id` is a fresh UUIDv4 per dispatcher invocation. Two dispatcher
   invocations on the same machine MUST NOT share a `service.instance.id`.
3. `host.id` terminal fallback is `"unknown-host"` (the literal string). It is NEVER
   SHA-256 of an empty string, NEVER null, NEVER an empty string.
4. `worktree.id` is always exactly 12 lowercase hex characters (first 12 of SHA-256
   hex encoding of the absolute worktree path string).
5. `project.id` is always the full SHA-256 hex string (64 characters) of
   `vcs.repository.url.full` after cascade.
6. **`deployment.environment.name` strict-string CI detection:** `deployment.environment.name = "ci"`
   requires the EXACT string `"true"` in the `CI` env var. `CI=1`, `CI=yes`, `CI=on`,
   `CI=TRUE` (uppercase), `CI=True` all map to `"local-dev"`. This strict-string semantics
   is a known sharp edge — environments using non-`"true"` CI flags (rare but possible)
   will be classified as `"local-dev"` rather than `"ci"`. Operators relying on `CI=1`
   (common in some CI systems) must set `CI=true` instead for correct classification.

## Related BCs

- BC-1.12.001 — Single-stream FileSink routing (depends on: Resource attributes from
  this BC are merged into every event before FileSink write)
- BC-1.12.004 — `emit_internal` bifurcation and per-event host stamping (composes with:
  this BC governs startup-time Resource attribute computation; BC-1.12.004 governs
  per-event-emit-time enrichment and the `emit_internal` Some/None bifurcation)
- BC-1.11.001 — VSDD_TRACE_ID injection into exec_subprocess (sibling ADR-015 BC: the
  `trace_id` in per-event fields derives from the startup-stamped context)

## Architecture Anchors

- `crates/factory-dispatcher/src/main.rs` — dispatcher startup: Resource attribute
  computation before the main event loop
- `crates/factory-dispatcher/src/resource_context.rs` (NEW module to be created in S-10.03)
  — `ResourceContext` struct holding all 15 attributes
- ADR-015 D-15.2 Resource attributes table — the authoritative list of all 15 fields
  and their value sources
- ADR-015 D-15.2.c — fallback cascade policy for `vcs.repository.url.full`, `project.id`,
  `worktree.id`, `host.id`, and derived VCS fields

## Story Anchor

S-10.03 (Wave 1: Resource-attribute enrichment; Windows registry cascade per OQ-2 resolution)

## VP Anchors

(TBD — to be assigned after S-10.03 story authoring)

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Normal git repo with `origin` remote | `vcs.repository.url.full` = git remote URL; `vcs.repository.name`, `vcs.owner.name`, `vcs.provider.name` parsed from URL; `project.id` = SHA-256 of URL |
| EC-002 | No git remote (`git remote get-url origin` returns non-zero) | `vcs.repository.url.full` = `file://<absolute worktree path>`; `vcs.repository.name` = directory basename; `vcs.owner.name` = `"unknown"`; `vcs.provider.name` = `"unknown"` |
| EC-003 | Not a git repo at all (no `.git` directory) | Same as EC-002: falls back to `file://` URI; no error or panic |
| EC-004 | Linux machine with valid `/etc/machine-id` | `host.id` = content of `/etc/machine-id` (trimmed); no fallback needed |
| EC-005 | macOS machine | `host.id` = IOPlatformUUID from `ioreg` output |
| EC-006 | Windows machine | `host.id` = `MachineGuid` from registry via `winreg` crate (full registry cascade per OQ-2 resolution; NOT stubbed) |
| EC-007 | Distroless container (no `/etc/machine-id`, no `ioreg`, not Windows) | `host.id` = `"unknown-host"` (terminal literal); `vsdd.internal.host_id_fallback.v1` lifecycle event emitted with `process.pid` and absolute cwd |
| EC-008 | `gethostname()` returns empty string or error | `host.id` falls through all platform steps to terminal default `"unknown-host"`; `vsdd.internal.host_id_fallback.v1` emitted |
| EC-009 | `CLAUDE_PROJECT_DIR` is set to an absolute path | `service.namespace` = basename of that path; `worktree.id` = SHA-256 prefix of that absolute path |
| EC-010 | `CLAUDE_PROJECT_DIR` is NOT set | Falls back to `current_dir()` per BC-1.08.004; `service.namespace` = basename of cwd; no error or null |
| EC-011 | `CI=true` in environment (exact string) | `deployment.environment.name` = `"ci"` (the ONLY value that triggers `"ci"` classification) |
| EC-011b | `CI=1` in environment (non-`"true"` truthy string) | `deployment.environment.name` = `"local-dev"` (strict-string gate; `"1"` is NOT `"true"`; this is a known sharp edge per Invariant 6) |
| EC-012 | `CI` not set or `CI=false` | `deployment.environment.name` = `"local-dev"` |
| EC-013 | Two simultaneous dispatcher processes on the same machine | Each generates a fresh `service.instance.id` UUIDv4; they MUST be distinct (UUIDv4 collision probability negligible at this scale) |
| EC-014 | **Terminal `host.id` fallback: `gethostname()` returns `""` (empty string)** | Empty string from `gethostname()` is treated as a failure condition (not a valid host.id). Falls to terminal default `"unknown-host"`. `vsdd.internal.host_id_fallback.v1` emitted. **v1 known limitation:** multiple distroless containers will share `host.id = "unknown-host"`; the observable fallback event makes this collision visible (per ADR-015 D-15.2.c). **SOUL #4 acknowledgment:** if the cascade implementation uses `let host_id = cascade().unwrap_or("unknown-host")` pattern, the `unwrap_or` must NOT silently suppress the `host_id_fallback` event emission. The implementation MUST emit the event when terminal default is reached. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Start dispatcher in a git repo with `origin` remote; `CLAUDE_PROJECT_DIR` set; Linux with `/etc/machine-id` present; `CI` unset | All 15 Resource fields present and non-null; `service.name = "vsdd-factory"`; `service.namespace = <project dir basename>`; `service.instance.id` is a valid UUIDv4; `deployment.environment.name = "local-dev"`; `host.id` read from `/etc/machine-id`; `vcs.repository.url.full` = git remote URL | happy-path-linux |
| **Misimplementation distinguisher:** implementation leaves `host.id = null` when `/etc/machine-id` fails | Test MUST assert `host.id` is NEVER null. Any null in any Resource field is a postcondition violation. | misimplementation-witness-null-resource |
| **Misimplementation distinguisher:** `vcs.repository.url.full` left empty on no-remote | Test MUST assert `vcs.repository.url.full` starts with `file://` (not empty/null) when no git remote exists. | misimplementation-witness-empty-vcs-url |
| No git remote; `CLAUDE_PROJECT_DIR = /home/user/my-project` | `vcs.repository.url.full = "file:///home/user/my-project"`; `vcs.repository.name = "my-project"`; `vcs.owner.name = "unknown"`; `vcs.provider.name = "unknown"`; `project.id = <SHA-256 of "file:///home/user/my-project">` (64 hex chars) | no-remote-fallback |
| Distroless container (Linux, no `/etc/machine-id`); `gethostname()` returns `""` | `host.id = "unknown-host"`; `vsdd.internal.host_id_fallback.v1` event emitted in `events-*.jsonl` with `process.pid` and absolute cwd; event category `"lifecycle"` per `vsdd.internal.*` registry | host-id-terminal-fallback |
| `CI=true` in environment (exact string) | `deployment.environment.name = "ci"` | ci-detection-exact-string |
| `CI=1` in environment (non-`"true"` value) | `deployment.environment.name = "local-dev"` (strict-string gate; `"1"` ≠ `"true"`; known sharp edge per Invariant 6) | ci-detection-non-true-string |
| Worktree at absolute path `/Users/dev/repo` | `worktree.id` = first 12 hex chars of `sha256("/Users/dev/repo")`; exact value is implementation-computed; test MUST assert length = 12 and matches `[0-9a-f]{12}` pattern | worktree-id-format |
| Two consecutive dispatcher invocations on the same machine | `service.instance.id` values are distinct (different UUIDv4s); `host.id` values are identical (machine-stable) | instance-id-uniqueness |
| `gethostname()` returns non-empty hostname; no `/etc/machine-id` (Linux) | Step 1 fails (no machine-id); Step 4 (terminal) reached; `host.id = "unknown-host"`; **NOTE:** SHA-256 of `gethostname()` is NOT a valid fallback step — per ADR-015 D-15.2.c, the terminal default is the LITERAL `"unknown-host"`, NOT a hash. A misimplementation using `sha256(hostname)` as terminal default violates this postcondition. | terminal-fallback-not-sha256 |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD) | No Resource field is null in any emitted event | property-based test: assert `None` / `null` absent from all 15 Resource fields across arbitrary invocations |
| (TBD) | `host.id` terminal fallback emits `vsdd.internal.host_id_fallback.v1` | integration test: inject platform fallback (mock all cascade steps to fail); assert event appears in `events-*.jsonl` |
| (TBD) | `vcs.repository.url.full` starts with `file://` when no git remote | integration test in non-git directory; assert field value |
| (TBD) | `service.instance.id` is distinct across two invocations | integration test: run dispatcher twice; compare `service.instance.id` from both invocations |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-030 ("Enrich emitted events with OTel-aligned resource attributes") per capabilities.md §CAP-030 |
| Capability Anchor Justification | CAP-030 ("Enrich emitted events with OTel-aligned resource attributes") per capabilities.md §CAP-030. This BC specifies the computation and stamping of all 15 Resource attribute fields (service.name, service.version, host.id, host.name, os.type, process.pid, process.runtime.name, process.runtime.version, vsdd.dispatcher.version, vsdd.session_id, vsdd.worktree_id, vsdd.project_root, vsdd.config_hash, telemetry.sdk.name, telemetry.sdk.version) at dispatcher startup — exactly the Resource attribute block that CAP-030 defines in its P1 capability description. The fallback cascades (Postconditions 2 and 3) ensure no field is absent or null, fulfilling CAP-030's outcome: "every event in events-*.jsonl carries a complete OTel-aligned resource block enabling correlation across Grafana/Loki/Honeycomb without post-processing enrichment." |
| L2 Domain Invariants | DI-017 (renamed by ADR-015 v1.7 from dispatcher_trace_id → trace_id; this BC stamps trace_id as part of per-event identity fields computed at startup from VSDD_TRACE_ID env or per-invocation UUID; the renamed field is used in all Resource events emitted by this BC) |
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/main.rs` (startup stamping), `crates/factory-dispatcher/src/resource_context.rs` (new module, S-10.03) |
| Stories | S-10.03 (Wave 1 Resource-attribute enrichment; Windows registry cascade) |
| Epic | E-10 (Single-stream OTel-aligned event emission) |
| ADR | ADR-015 D-15.2 (OTel-aligned schema; Resource attributes table); ADR-015 D-15.2.c (fallback cascade policy) |
| OQ Resolved | OQ-2 (full Windows `host.id` registry-lookup cascade per ADR-015 v1.8 adjudication: `winreg` crate, target-OS-gated; NOT stubbed) |

### Purity Classification

| Property | Assessment |
|----------|-----------|
| I/O operations | YES — `git remote get-url origin` subprocess; `/etc/machine-id` file read; `ioreg` subprocess (macOS); Windows registry read; `gethostname()` syscall |
| Global state access | YES — reads `CLAUDE_PROJECT_DIR`, `CI` env vars; reads process environment |
| Deterministic | YES for a given environment; `service.instance.id` is a fresh UUID per invocation (intentionally non-deterministic) |
| Thread safety | YES — startup-only computation; results are immutable thereafter |
| Overall classification | Effectful shell (filesystem + subprocess + env + platform syscalls; all errors handled by cascade) |

### TD-VSDD-092 (BC-SOUL4-coverage) Verification

Source-walk for silent-discard patterns in the Resource attribute computation:

- `git remote get-url origin` failure: the shell-command failure must be caught and
  handled by the `file://` fallback, NOT silently swallowed. Implementation MUST NOT
  use `let _ = std::process::Command::new("git")...` pattern without checking status.
- `host.id` cascade: each cascade step must check the result before falling to the next.
  A `let _ = read_machine_id()` pattern would silently skip valid machine-id reads —
  PROHIBITED. The cascade must use explicit `if let Ok(id) = ... { return id }` pattern.
- `vsdd.internal.host_id_fallback.v1` emission: when terminal default reached, the event
  emit MUST NOT be behind a `let _ = emit_event(...)` pattern. The event is itself a
  critical observability signal. BC-1.12.001 EC-006 (lifecycle events to single-stream)
  covers the routing; this BC requires the EMISSION to occur.
- EC-014 covers the case where `gethostname()` returns empty string — the empty-string
  return must trigger fallback, not be used as a `host.id` value.
