---
document_type: behavioral-contract
level: L3
version: "1.7"
status: active
producer: product-owner
timestamp: 2026-06-10T00:00:00Z
phase: brownfield-backfill
cycle: v1.0-brownfield-backfill
inputs:
  - .factory/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.12.001.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.11.001.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - plugins/vsdd-factory/hooks-registry.toml
input-hash: "68e7eb5"
traces_to: .factory/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md
origin: brownfield
extracted_from: null
subsystem: "SS-04"
capability: "CAP-031"
lifecycle_status: active
introduced: v1.0-brownfield-backfill
modified:
  - "2026-06-11 (v1.1)"
  - "2026-06-11 (v1.2)"
  - "2026-06-11 (v1.3)"
  - "2026-07-06 (v1.4)"
  - "2026-07-06 (v1.5)"
  - "2026-07-07 (v1.6)"
  - "2026-07-07 (v1.7)"
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-4.13.001
section: "4.13"
last_amended: "2026-07-07 (v1.7) — E-19 pass-7 F-P7-006 adjudication (product-owner): Invariant 10 upper-boundary blind spot closed — soft-warn range made explicitly inclusive at cap: bytes_read ∈ (200000, 262144]. Condition restated as bytes_read > 200000 AND bytes_read ≤ cap_bytes (262144). Boundary table added: 200000 no-warn / 200001 warn / 262144 warn+readable / 262145 OUTPUT_TOO_LARGE. Closes F-P7-006 (BC leg). BC-INDEX v3.72→v3.73. [Prior: 2026-07-07 (v1.6) — E-19 pass-4 F-P4-001 fix burst (product-owner): Precondition 3 phased-clause amendment (architect Option B ruling; S-19.07 added to E-19; human approval 2026-07-07). Phase-A (active; S-19.02/W1 shipped): guard reads .factory/STATE.md via host::read_file with max_bytes=262144; [hooks.capabilities.read_file] with .factory in path_allow; compile-time STATE_MD_MAX_BYTES=262144; extract_frontmatter truncates at second --- delimiter (Invariant 9 unchanged). Phase-B (S-19.07; depends_on [S-19.02, S-19.06]; forward-scheduled migration): guard MUST migrate to host::read_prefix(path, max_bytes=8192); [hooks.capabilities.read_file] replaced with [hooks.capabilities.read_prefix] (same path_allow [.factory]); STATE_MD_MAX_BYTES constant and ALL TooLarge/OutputTooLarge handling removed (read_prefix guarantees NEVER OUTPUT_TOO_LARGE per BC-1.17.001); 8192 bound sufficient for any realistic frontmatter (<2KB under compaction discipline); extract_frontmatter retained unchanged; Phase-A unregressed until S-19.07 merges. Story Anchor: dual-story Phase-A S-19.02 / Phase-B S-19.07. Traceability Stories row: TBD → S-19.02 (Phase-A; W1 shipped) + S-19.07 (Phase-B; depends_on [S-19.02, S-19.06]; forward-scheduled). Closes F-P4-001 (BC leg); ADR-025 D18(e); human approval 2026-07-07. BC-INDEX v3.69→v3.70. [Prior: 2026-07-06 (v1.5) — E-19 pass-2 F-P2-001 + F-P2-003 + F-P2-011 fix burst (product-owner): (a) Precondition 1: MultiEdit enumerated in tool-pattern list (anchored form ^(Edit|Write|MultiEdit|Agent)$; ADR-025 2026-06-11 sibling-sweep mandate); Invariant 5 TOML tool field updated to Edit|Write|MultiEdit|Agent. (b) Precondition 3 rationale: stale ~90 KB / ~466 lines replaced with verified 193,220 bytes / 488 lines at 2026-07-06 review checkpoint; cap 262144 RETAINED (architect D-c: cap inflation without compaction discipline defers the crisis; compaction is the correct response); utilization ~74%, headroom ~35%. (c) Invariant 10 added: soft_warn_threshold = 200000 bytes; state_md_approaching_cap observability-only diagnostic event (bytes_read + cap_bytes fields); zero new registry entries. (d) Invariant 9 verification note (F-P2-011): byte-boundary correctness of frontmatter extraction; parity-with-full-file-parse FORBIDDEN. Closes F-P2-001 (BC leg), F-P2-003, F-P2-011. BC-INDEX v3.59→v3.60. [Prior: 2026-07-06 (v1.4) — E-19 pass-1 F-P1-004 fix burst (product-owner): Precondition 3 max_bytes raised 65536→262144 (256 KiB; rationale: STATE.md observed ~90 KB / ~466 lines; 500-line compaction hard cap implies worst-case ≤200 KiB; 256 KiB gives ≥28% headroom). Invariant 9 extended with frontmatter-only-parsing mandate: guard MUST abort after second ---\\n delimiter and MUST NOT parse file body. TD-031 in-scope fix: two volatile executor.rs line cites migrated to stable symbol anchors per TD-VSDD-091 (PC1 block path and Invariant 8 aggregation site). Closes F-P1-004. BC-INDEX v3.57→v3.58. [Prior: 2026-06-11 (v1.3) — POL-14 auto-promotion: lifecycle_status draft→active on PR #182 squash-merge df4f26b8 (S-17.02 MERGED 2026-06-11); BC-INDEX v2.69→v2.70; D-545. [Prior: 2026-06-11 (v1.2) — Boundary-semantics spec error (product-owner; S-17.02 testing finding; issue #170). EC-002 and PC2 prescribed `now > expires_at` as the expiry test, which is self-contradictory: `now == expires_at` would evaluate false under strict `>`, causing the guard to BLOCK at the exact-expiry instant — opposite of the stated EC-002 outcome (boundary → Continue). Corrected to `now >= expires_at` throughout (PC2 condition, EC-002 description, Invariant 3). PC1 blocking condition updated from `now ≤ expires_at` to `now < expires_at` for consistency (boundary is expired, not blocking). BC version v1.1→v1.2. [Prior: 2026-06-11 (v1.1) — Production-correctness spec gap (product-owner; S-17.02 implementation finding; issue #170). Inv 5 registry-shape UPDATED: both exec_subprocess capability blocks now REQUIRE `env_allow = [\"HOME\", \"GIT_CONFIG_GLOBAL\", \"XDG_CONFIG_HOME\"]` alongside `binary_allow = [\"git\"]`; without env_allow the dispatcher calls env_clear() → git config user.email returns empty → IdentityResolutionFailed → HookResult::Continue → lock silently inert. EC-016 added (env_allow omitted footgun). PC7 IdentityResolutionFailed extended to document env_allow dependency. BC version v1.0→v1.1. Prior: 2026-06-10 (v1.0) — Initial authoring (product-owner; brownfield-backfill issue #170; ADR-025 v1.2 D1/D2/D7/D9 deliverables). verify-factory-lock WASM guard behavioral contract. lifecycle_status: draft (POL-14 auto-promotion to active on implementing PR merge).]]]]]"
---

# BC-4.13.001: verify-factory-lock WASM PreToolUse guard MUST block mutating tools when a foreign unexpired factory_lock is held, MUST pass all read-only tools unconditionally, MUST fail-open on crash, MUST be registered async=false with both capability blocks enumerated, and MUST treat expired/absent/malformed locks as unlocked

## Description

The `verify-factory-lock` native-WASM plugin enforces the cross-session single-writer invariant
on the `factory-artifacts` orphan branch. It fires on every `PreToolUse` event for mutating tools
(Edit, Write, MultiEdit, Agent dispatch, and Bash commands pushing to `factory-artifacts`). On each
invocation, it reads the `factory_lock` block from `.factory/STATE.md` via `host::read_file`,
reads the current `git config user.email` via `host::exec_subprocess`, and compares the lock holder
to the caller's identity plus the current time to the `expires_at` field. A foreign unexpired lock
causes `block_intent = true` (exit code 2) with an actionable refusal message. Read-only tools,
absent/expired/malformed locks, and self-held locks always pass. A crashed or timed-out guard
never wedges the factory (`on_error = "continue"`).

This BC covers ADR-025 Decisions 1, 2, 3, 4, 7, 9, and 10, and deliverables D1, D2, and D9
(bats integration tests). It is the primary enforcement mechanism for CAP-031. The lock state
schema (D3), the `/factory-lock` and `/factory-unlock` skills (D4/D5), and the `state-burst`
push fix (D6) are specified in BC-5.40.001 and BC-6.23.001 respectively.

## Preconditions

### PreToolUse activation

1. A `PreToolUse` event has fired for one of the following tool patterns (per the registry
   `tool` regex — see Invariant 5):
   - `Edit` — any file edit
   - `Write` — any file write
   - `MultiEdit` — any multi-file edit (anchored form: `^(Edit|Write|MultiEdit|Agent)$`; same guard semantics as Edit/Write; ADR-025 2026-06-11 sibling-sweep mandate)
   - `Agent` — any agent dispatch
   - `Bash` whose payload matches `git.*push.*factory-artifacts` (factory-artifacts push arm)

   Read-only tool calls (`Read`, non-push `Bash`, `mcp__*`, etc.) are NOT in scope: they never
   trigger this plugin and always proceed unconditionally. The guard's purpose is to block
   in-flight mutations, not reads.

2. The dispatcher has invoked the `verify-factory-lock` WASM plugin with the tool payload.
   The guard is registered in `hooks-registry.toml` as TWO entries (one for Edit|Write|MultiEdit|Agent,
   one for Bash) — see Invariant 5 for the mandatory registration shape.

### File read capability

3. **Phase-A (active; S-19.02/W1 shipped):** The guard reads `.factory/STATE.md` via
   `host::read_file` with `max_bytes = 262144` (256 KiB). The registry-level
   `[hooks.capabilities.read_file]` MUST be present with `.factory` in `path_allow`.
   The plugin-side compile-time cap is `STATE_MD_MAX_BYTES = 262144`;
   `extract_frontmatter(bytes)` truncates at the second `---` delimiter before YAML parse
   (Invariant 9 unchanged). Without this block, the dispatcher returns `CapabilityDenied`
   and the plugin graceful-degrades to `Continue` — the lock never enforces (silent no-op;
   see EC-007 capability-denied footgun and Invariant 6).

   **Phase-B (S-19.07; depends_on [S-19.02, S-19.06]; forward-scheduled migration, not
   in-place replacement):** Once `host::read_prefix` is available and independently proven
   (ADR-025 Decision 15; S-19.06 merged to develop), the guard MUST migrate its read
   primitive to `host::read_prefix(path, max_bytes=8192, timeout_ms=<plugin latency budget>)`.
   The `[hooks.capabilities.read_file]` block is replaced with
   `[hooks.capabilities.read_prefix]` (same `path_allow = [".factory"]`). The
   `STATE_MD_MAX_BYTES` constant and ALL `TooLarge`/`OutputTooLarge` handling are removed
   — `read_prefix` guarantees NEVER `OUTPUT_TOO_LARGE` (BC-1.17.001), making the error
   class structurally impossible. The 8192-byte bound is sufficient for any realistic
   frontmatter (<2 KB under compaction discipline). `extract_frontmatter` is retained
   unchanged. Phase-A behavior is correct and unregressed until S-19.07 merges.

### Subprocess capability

4. The guard MUST invoke `git config user.email` via `host::exec_subprocess` to obtain the
   caller's identity. The registry-level `[hooks.capabilities.exec_subprocess]` MUST enumerate
   BOTH `binary_allow = ["git"]` AND `env_allow = ["HOME", "GIT_CONFIG_GLOBAL",
   "XDG_CONFIG_HOME"]` explicitly. The dispatcher calls `env_clear()` before subprocess
   execution; only env vars listed in `env_allow` are passed through. Without `HOME` (and
   optionally `GIT_CONFIG_GLOBAL` / `XDG_CONFIG_HOME`), `git config user.email` returns
   empty output regardless of the developer's git configuration → `IdentityResolutionFailed`
   → `HookResult::Continue` (PC7 fail-open) → the lock SILENTLY NEVER ENFORCES. This is the
   same class of footgun as omitting the capability block entirely (EC-007/EC-008), but subtler
   because the capability block IS present — only the env pass-through is missing (EC-016).
   Both capability fields are REQUIRED together — omitting either is the single most likely
   implementation-time footgun (ADR-025 D2, Rationale §Capability blocks must be enumerated).

### Registry correctness

5. The `verify-factory-lock` plugin MUST be registered with `async = false` in
   `hooks-registry.toml`. An `async = true` plugin's `block_intent` is discarded by the
   dispatcher (advisory-only per ADR-019): registering as async silently reduces the guard to
   a no-op blocker. `async = false` is a correctness requirement, not a performance preference
   (ADR-025 Rationale §Why `async = false` is mandatory; ADR-019 CI lint invariant
   `on_error=block ⇒ async=false`).

## Postconditions

### PC1 — Foreign unexpired lock: Block with actionable message

When ALL of the following are true:
- `factory_lock.holder` is present and non-null in STATE.md frontmatter (Invariants 1-3)
- `now < factory_lock.expires_at` (TTL not yet elapsed; the boundary instant `now == expires_at` is treated as expired — see PC2)
- `factory_lock.holder != current_git_email` (caller is not the lock holder)

The guard MUST return `block_intent = true` (exit code 2). The block message MUST include ALL
five required fields:
- `holder` — the exact git email of the current lock holder
- `locked_at` — the ISO-8601 timestamp when the lock was acquired
- `expires_at` — the ISO-8601 timestamp when the lock auto-expires
- `time_remaining` — human-readable duration (e.g., "37 min remaining"); computed as
  `expires_at - now`, rounded down to the nearest minute
- `/factory-unlock --force` — the exact command string to break-glass force-release the lock

The block path runs through the `plugin_requests_block` function in
`crates/factory-dispatcher/src/executor.rs` for sync-group plugins (the `async = false`
requirement is necessary for this path — see Precondition 5).

**Error variant:** `ForeignLockHeld`

### PC2 — Expired lock: Pass (treat as unlocked)

When `factory_lock.holder` is present but `now >= factory_lock.expires_at`, the guard MUST return
`HookResult::Continue` immediately. The boundary instant `now == expires_at` is treated as
expired — the lock is considered to have just lapsed. An expired lock is treated as absent; the
caller proceeds without any block. The guard MUST NOT emit a warning for expired-lock
pass-through (it is the normal TTL expiry path, not an error condition).

**Error variant:** `LockExpired` (for internal instrumentation only; not surfaced as a block)

### PC3 — Self-held lock: Pass unconditionally

When `factory_lock.holder == current_git_email` (regardless of expiry), the guard MUST return
`HookResult::Continue`. The developer who acquired the lock is never blocked by their own lock.
This is the single-developer happy path and MUST add zero visible friction beyond the
`host::read_file` + `host::exec_subprocess` latency (sub-5ms under normal conditions per
ADR-025 Decision 10 latency budget).

### PC4 — Absent, null, or malformed lock: Pass (fail-open)

When the `factory_lock` block is absent from STATE.md frontmatter, present but null, or
malformed (missing required fields `holder`/`locked_at`/`expires_at`, or fields not parseable
as the required types), the guard MUST return `HookResult::Continue`. The factory operates
unlocked when no lock is in force. A malformed block is treated the same as an absent block —
fail-open (ADR-025 Decision 2). The guard MUST emit `host::log_warn` for malformed blocks
(advisory only; non-blocking) to assist debugging.

**Error variant:** `MalformedLockBlock` (log_warn only; no block)

### PC5 — Read-only tools: Pass unconditionally (not triggered)

Tools not matching the `tool` regex in the registry entry (Read, non-push Bash, mcp__* tools,
etc.) are NOT intercepted by this plugin. They proceed unconditionally. This is a registry-level
property: the guard is never invoked, so no STATE.md read occurs. A blocked developer CAN
always read `.factory/STATE.md` to inspect the lock state, regardless of who holds the lock
(ADR-025 Decision 4).

### PC6 — STATE.md read failure: Pass (fail-open)

When `host::read_file(".factory/STATE.md")` returns any `HostError` variant
(`OutputTooLarge`, `Timeout`, `CapabilityDenied`, `InvalidArgument`, `Other(i32)`), the guard
MUST return `HookResult::Continue` after emitting `host::log_warn` with the error variant name.
A guard that cannot read STATE.md MUST NOT block — fail-open is correct for this efficiency-class
lock (ADR-025 Decision 7, Kleppmann §8 distinction).

**Error variant:** `StateReadError(HostError variant name)`

### PC7 — git config subprocess failure: Pass (fail-open)

When `host::exec_subprocess(["git", "config", "user.email"])` fails (non-zero exit, empty
output, or HostError), the guard MUST return `HookResult::Continue` after emitting
`host::log_warn`. Identity resolution failure MUST NOT block — same fail-open rationale as PC6.

**env_allow dependency:** The dispatcher calls `env_clear()` before invoking the subprocess;
only env vars listed in `[hooks.capabilities.exec_subprocess] env_allow` are passed through.
If `env_allow` is absent or omits `HOME`, git cannot locate `~/.gitconfig` or the system
config, and `git config user.email` returns empty output — triggering this path even on a
correctly configured machine. The result is indistinguishable from a genuinely unconfigured
git identity: the guard returns `HookResult::Continue` and the lock silently never enforces.
This is EC-016; see also Precondition 4 and Invariant 5 for the mandatory `env_allow` values.

**Error variant:** `IdentityResolutionFailed`

### PC8 — On-error behavior: Always Continue (fail-open)

The plugin's `on_error` field in `hooks-registry.toml` MUST be `"continue"`. A crashed or
timed-out plugin returns `HookResult::Continue` (the dispatcher's fail-open default for
`on_error = "continue"` entries). The factory is NEVER wedged by a broken guard. Guard crashes
are surfaced as advisory `internal.dispatcher_error` events via SS-03.

## Invariants

1. **`factory_lock.holder` is the sole identity source**: The guard reads only
   `factory_lock.holder` from STATE.md and `git config user.email` for the caller. No other
   identity source (hostname, PID, Claude session ID, environment variables) is consulted.
   Developer-level granularity is correct and accepted (ADR-025 Decision 3).

2. **TTL comparison is wall-clock monotonic**: `expires_at` is compared against the system
   clock at the moment of guard invocation. The guard uses `std::time::SystemTime::now()` (or
   equivalent WASM-compatible clock). Clock skew between machines is an accepted residual risk
   for the cooperative threat model.

3. **Absent `factory_lock` block = unlocked**: `None` for the entire block equals unlocked.
   `Some(block)` with `expires_at` in the past OR equal to `now` (i.e., `now >= expires_at`)
   also equals unlocked — the boundary instant is treated as expired. Only
   `Some(block)` with `now < expires_at` (strictly in the future) AND `holder != current_email`
   equals locked.

4. **No STATE.md write**: The guard is read-only at runtime. It NEVER writes STATE.md. Writing
   the `factory_lock` block is exclusively `state-manager`'s responsibility (TD-VSDD-053).

5. **Registry shape is mandatory (two entries)**: The canonical registration in
   `hooks-registry.toml` requires BOTH entries and BOTH capability blocks:
   ```toml
   [[hooks]]
   name = "verify-factory-lock"
   plugin = "hook-plugins/verify-factory-lock.wasm"
   event = "PreToolUse"
   tool = "Edit|Write|MultiEdit|Agent"
   async = false
   on_error = "continue"
   timeout_ms = 5000

   [hooks.capabilities.read_file]
   path_allow = [".factory/STATE.md"]

   [hooks.capabilities.exec_subprocess]
   binary_allow = ["git"]
   env_allow = ["HOME", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME"]

   [[hooks]]
   name = "verify-factory-lock-bash"
   plugin = "hook-plugins/verify-factory-lock.wasm"
   event = "PreToolUse"
   tool = "Bash"
   async = false
   on_error = "continue"
   timeout_ms = 5000

   [hooks.capabilities.read_file]
   path_allow = [".factory/STATE.md"]

   [hooks.capabilities.exec_subprocess]
   binary_allow = ["git"]
   env_allow = ["HOME", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME"]
   ```
   The Bash entry filters on commands matching `git.*push.*factory-artifacts` within the plugin
   logic (the registry `tool = "Bash"` is the trigger; the push-regex check is internal to the
   WASM plugin for Bash payloads, allowing non-push Bash to proceed without STATE.md reads).

6. **Capability-denied graceful degrade is advisory, not a crash**: If either capability block
   is omitted from the registry, `host::read_file` or `host::exec_subprocess` returns
   `CapabilityDenied`. The plugin MUST catch this as a `HostError` variant and graceful-degrade
   to `Continue` (not panic). This degrade MUST emit `host::log_warn("capability_denied: ...")`.
   However, the guard is now silently inert — this is the footgun documented in ADR-025 D2.
   The bats integration test MUST cover this case (D9 §"capability-omitted registry entry
   graceful-degrades").

7. **Latency budget**: The guard's hot path (one `read_file` call + one `exec_subprocess` call
   + timestamp comparison) MUST complete within the ADR-020 Class A p95 budget of 1500ms.
   Under normal conditions (local filesystem, git installed), the budget is well within 100ms.
   The `timeout_ms = 5000` is a backstop for pathological conditions.

8. **Block path is synchronous**: The `block_intent = true` result is only effective when the
   plugin is in the sync-group (`async = false`). The block decision aggregates at the
   `plugin_requests_block` call site in `crates/factory-dispatcher/src/executor.rs`. An async
   plugin's `block_intent` is silently discarded (advisory telemetry only per ADR-019).
   `async = false` is therefore a correctness constraint on the registration, not an
   optimization hint.

9. **`factory_lock` block parsing is fail-open**: Frontmatter parsing uses a YAML subset
   scanner (not a full YAML parser). The guard scans for the `factory_lock:` key and its
   three sub-fields (`holder:`, `locked_at:`, `expires_at:`) using line-by-line scan within
   the frontmatter region (between first and second `---\n` delimiters). Any parse ambiguity
   (nested structures, quoted colons, missing delimiters) routes to `MalformedLockBlock`
   (PC4 fail-open path), never to a Block. **Frontmatter-only mandate:** The guard MUST abort
   scanning immediately after encountering the second `---\n` delimiter and MUST NOT attempt
   to parse the file body. The lock verdict is derived exclusively from the frontmatter region;
   no body content is read, parsed, or required. This property is preserved regardless of
   STATE.md body size: `max_bytes = 262144` (Precondition 3) ensures the host read does not
   return `OutputTooLarge` on valid STATE.md files, but the parser stops at the frontmatter
   boundary and never processes the remaining bytes.

   **Verification note (F-P2-011):** The correct test for this invariant is byte-boundary
   correctness of the frontmatter extraction: the extracted bytes MUST byte-equal the file
   prefix through (and including) the second `---\n` delimiter line. Tests MUST NOT verify
   this property by comparison against a full-file YAML parse — that path is explicitly
   forbidden by this invariant.

10. **Soft warning threshold for STATE.md utilization**: `soft_warn_threshold = 200000` bytes.
    When a hook that already reads STATE.md in full (i.e., calls `host::read_file` on
    `.factory/STATE.md`) completes a successful read and observes
    `bytes_read > soft_warn_threshold AND bytes_read ≤ cap_bytes (262144)`, it MUST emit a
    diagnostic event of type `state_md_approaching_cap` carrying two fields:
    `bytes_read: u64` (the observed byte count) and `cap_bytes: u64` (the current `max_bytes`
    value, 262144). The soft-warn range is `bytes_read ∈ (200000, 262144]` — inclusive at the
    cap boundary:

    | `bytes_read` | Outcome |
    |---|---|
    | ≤ 200000 | No warn emitted; normal read |
    | 200001 | `state_md_approaching_cap` emitted; read succeeds |
    | 262144 | `state_md_approaching_cap` emitted AND read succeeds — this is the most alarming observable state (1 byte from failure); the warn MUST fire at this boundary |
    | 262145 | `OutputTooLarge` returned by host; soft-warn path not reached |

    This event is observability-only — it NEVER triggers a block or alters the `Continue`/`Block`
    verdict. It is implemented as an extension to hooks that already read STATE.md in full; zero
    new registry entries are required. The `verify-factory-lock` guard, which reads STATE.md on
    every mutating-tool PreToolUse invocation, MUST emit this event when
    `bytes_read > soft_warn_threshold AND bytes_read ≤ cap_bytes`. The threshold is not a hard
    cap; it is a leading indicator for compaction scheduling.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `factory_lock` block absent from STATE.md | `HookResult::Continue` (PC4 fail-open: no lock = unlocked) |
| EC-002 | `factory_lock.expires_at` is exactly `now` (boundary) | `HookResult::Continue` — expired lock. The expiry test is `now >= expires_at`; at the exact boundary `now == expires_at` this evaluates `true` → lock is treated as just-expired → guard returns Continue (LockExpired path). The boundary instant is NOT blocking. |
| EC-003 | `factory_lock.holder` == `current_git_email` and lock is unexpired | `HookResult::Continue` (PC3 self-held; developer is not blocked by their own lock) |
| EC-004 | `factory_lock.holder` is set but `holder` field is empty string | `MalformedLockBlock` → `HookResult::Continue` (PC4; empty holder is malformed) |
| EC-005 | `factory_lock.expires_at` is not a valid ISO-8601 datetime | `MalformedLockBlock` → `HookResult::Continue` (PC4; unparseable expiry = treat as absent) |
| EC-006 | STATE.md is missing (file not found) | `HostError` → `StateReadError` → `HookResult::Continue` (PC6 fail-open) |
| EC-007 | `[hooks.capabilities.read_file]` block omitted from registry | `host::read_file` returns `CapabilityDenied`; plugin graceful-degrades to `HookResult::Continue` with `log_warn`; lock NEVER enforces (silent no-op — the capability footgun) |
| EC-008 | `[hooks.capabilities.exec_subprocess]` block omitted from registry | `host::exec_subprocess` returns `CapabilityDenied`; identity resolution fails; plugin graceful-degrades to `HookResult::Continue` with `log_warn`; lock NEVER enforces |
| EC-009 | `git config user.email` returns empty output (git email not configured) | `IdentityResolutionFailed` → `HookResult::Continue` (PC7 fail-open; treat as unlocked) |
| EC-010 | `async = true` in registry (misconfigured) | Plugin fires but `block_intent` is discarded by dispatcher (advisory-only per ADR-019); guard silently becomes no-op blocker — indistinguishable from a working guard until a concurrent-session incident |
| EC-011 | Non-push Bash command (e.g., `cat .factory/STATE.md`) arrives at Bash entry | Plugin checks internal push-regex; no match; `HookResult::Continue` immediately (no STATE.md read, sub-millisecond) |
| EC-012 | `factory_lock` frontmatter has `holder` and `expires_at` but missing `locked_at` | `MalformedLockBlock` → `HookResult::Continue` (PC4; `locked_at` is required for the refusal message per ADR-025 Decision 4) |
| EC-013 | STATE.md frontmatter region missing closing `---\n` delimiter | `MalformedLockBlock` (parse fail-open) → `HookResult::Continue` |
| EC-014 | Two concurrent Edit calls from different sessions; both arrive before either is blocked | First Edit proceeds (no lock set yet); CAS push at `state-burst` is the safety net for the push layer (BC-5.40.001). The guard provides per-call blocking, not atomicity across concurrent arrivals. |
| EC-015 | `host::exec_subprocess` times out (git subprocess stalls) | `HostError::Timeout` → `IdentityResolutionFailed` → `HookResult::Continue` (PC7 fail-open) |
| EC-016 | `env_allow` absent (or omits `HOME`) from `[hooks.capabilities.exec_subprocess]` in registry — dispatcher calls `env_clear()` → `git config user.email` runs without `HOME` → empty output → `IdentityResolutionFailed` → `HookResult::Continue` → guard silently inert (lock never enforces). Same footgun class as EC-007/EC-008 (capability block omitted), but subtler: the capability block IS present, only the env pass-through is missing. | `IdentityResolutionFailed` → `HookResult::Continue` (PC7 fail-open; lock silently inert); bats integration test MUST cover this case (D9 §"env_allow-omitted registry entry silently fails identity resolution") |

## Canonical Test Vectors

These are the D9 bats integration test vectors (from ADR-025 §D9):

| Test # | Precondition | Tool | Expected Result |
|--------|-------------|------|----------------|
| T-1 | `factory_lock` absent | Edit any file | `HookResult::Continue` (unlocked path) |
| T-2 | `factory_lock.holder` = other developer; `expires_at` = now + 30min | Edit `.factory/STATE.md` | `HookResult::Block` with message containing holder email, locked_at, expires_at, time_remaining, `/factory-unlock --force` |
| T-3 | `factory_lock.holder` = other developer; `expires_at` = now - 1min | Edit `.factory/STATE.md` | `HookResult::Continue` (TTL expired — LockExpired path) |
| T-4 | `factory_lock.holder` = current developer email; `expires_at` = now + 30min | Edit `.factory/STATE.md` | `HookResult::Continue` (self-held) |
| T-5 | `factory_lock.holder` = other developer; `expires_at` = now + 30min | `Read .factory/STATE.md` | Not triggered (Read not in scope); `HookResult::Continue` |
| T-6 | `factory_lock.holder` = other developer; `expires_at` = now + 30min | `Bash git push origin factory-artifacts` | `HookResult::Block` (factory-artifacts push arm) |
| T-7 | `factory_lock.holder` = other developer; `expires_at` = now + 30min | `Bash cat .factory/STATE.md` | `HookResult::Continue` (non-push Bash; internal regex no-match) |
| T-8 | `[hooks.capabilities.read_file]` omitted in registry | Edit any file | `HookResult::Continue` (graceful-degrade; `log_warn` emitted; EC-007) |
| T-9 | `factory_lock.expires_at` malformed (not ISO-8601) | Edit any file | `HookResult::Continue` (EC-005 malformed) |
| T-10 | Acquire CAS rejection: two concurrent `/factory-lock` invocations see the same unlocked STATE.md | Second `/factory-lock` push returns non-fast-forward | Second acquire fails with `AcquireRaceRejected`; first succeeds (covered by BC-6.23.001 T-4) |

## SDK Grounding Evidence

The following literal-shell greps confirm stable anchors in the production codebase at
brownfield-backfill HEAD. These are HEAD-reproducible structural predicates (POLICY 5 Part B).

**Grep 1 — `plugin_requests_block` function name in executor.rs (block path anchor):**
```
grep -rn "plugin_requests_block" crates/factory-dispatcher/src/
```
Expected: at least one hit in `executor.rs` — this is the production block path cited in
ADR-025 Decision 1 and is the stable behavioral anchor for PC1's block path.

**Grep 2 — `on_error = "continue"` pattern in hooks-registry.toml (existing precedent):**
```
grep -c 'on_error.*continue' plugins/vsdd-factory/hooks-registry.toml
```
Expected: non-zero count — confirms `on_error = "continue"` is an established pattern in the
registry (same as `validate-artifact-path.wasm` per ADR-025 Decision 7 + ADR-016 precedent).

**Grep 3 — `HOST_ABI_VERSION` in hook-sdk (ABI version anchor):**
```
grep -n "HOST_ABI_VERSION" crates/hook-sdk/src/lib.rs
```
Expected: `HOST_ABI_VERSION: u32 = 1` — confirms ABI version 1 is unchanged; no dispatcher
changes required for the new guard (ADR-025 Decision 1).

**Grep 4 — `binary_allow.*git` pattern in hooks-registry.toml (exec_subprocess precedent):**
```
grep -n 'binary_allow.*git' plugins/vsdd-factory/hooks-registry.toml
```
Expected: at least one hit from `capture-commit-activity` registry entry — confirms the
`exec_subprocess binary_allow = ["git"]` pattern is already established in production.

**Grep 5 — `validate-artifact-path` as PreToolUse `read_file` guard pattern (structural anchor):**
```
grep -n "validate-artifact-path" plugins/vsdd-factory/hooks-registry.toml
```
Expected: at least one hit — confirms the existing PreToolUse native-WASM `read_file` guard
pattern that `verify-factory-lock` follows (ADR-025 Rationale §Why native WASM).

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (unit-test) | Foreign unexpired lock returns Block with all 5 required fields | Rust unit test: construct STATE.md with foreign lock, `now + 30min`; assert Block + message fields |
| (unit-test) | Expired lock returns Continue unconditionally | Rust unit test: `expires_at = now - 1s`; assert Continue |
| (unit-test) | Self-held lock returns Continue unconditionally | Rust unit test: `holder == current_email`; assert Continue |
| (unit-test) | Absent `factory_lock` block returns Continue | Rust unit test: STATE.md with no `factory_lock` key; assert Continue |
| (unit-test) | Malformed `factory_lock` block returns Continue with log_warn | Rust unit test: invalid ISO-8601 `expires_at`; assert Continue + warn |
| (unit-test) | `CapabilityDenied` on `read_file` graceful-degrades to Continue | Rust unit test: mock `host::read_file` returning `CapabilityDenied`; assert Continue |
| (unit-test) | `CapabilityDenied` on `exec_subprocess` graceful-degrades to Continue | Rust unit test: mock returning `CapabilityDenied`; assert Continue |
| (bats) | Bats D9 test coverage per ADR-025 §D9 (9 scenarios) | See Canonical Test Vectors T-1..T-10 |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-031 |
| Capability Anchor Justification | CAP-031 ("Enforce single-writer cross-session exclusivity on factory-artifacts state") per capabilities.md §CAP-031 — this BC defines the `verify-factory-lock` WASM PreToolUse guard that IS the primary enforcement mechanism for CAP-031. The guard blocks mutating tools when a foreign unexpired lock is held, which is exactly the blocking behavior CAP-031 specifies. |
| L2 Domain Invariants | none (cross-session operational invariant, not L2 domain spec) |
| Architecture Module | `crates/hook-plugins/verify-factory-lock/` (new crate; compiles to `hook-plugins/verify-factory-lock.wasm`); `crates/hook-sdk/src/lib.rs` (HOST_ABI_VERSION=1 anchor); `crates/factory-dispatcher/src/executor.rs` (plugin_requests_block block path; sync-group partition); `plugins/vsdd-factory/hooks-registry.toml` (registry entries D2) |
| Stories | S-19.02 (Phase-A; `verify-factory-lock` guard implementation; W1 shipped); S-19.07 (Phase-B; `host::read_prefix` migration; forward-scheduled; depends_on [S-19.02, S-19.06]) |
| ADR Reference | ADR-025 v1.2 (primary — all 10 decisions); ADR-016 (artifact path guard pattern + `on_error = "continue"` precedent); ADR-019 (sync/async partition; `async = false` CI lint invariant); ADR-020 (Class A latency budget ≤1500ms p95) |

## Related BCs

- BC-5.40.001 — depends on (defines the `factory_lock` STATE.md frontmatter schema that this guard reads; PC4 malformed-block behavior and TTL semantics)
- BC-6.23.001 — composes with (defines the `/factory-lock` acquire and `/factory-unlock` release skills; the guard enforces what those skills set)
- BC-4.11.001 — sibling (validate-artifact-path WASM guard; same PreToolUse pattern, same `on_error = "continue"` and `host::read_file` discipline)

## Architecture Anchors

- `crates/hook-plugins/verify-factory-lock/` — new Rust crate (to be created); compiled to `hook-plugins/verify-factory-lock.wasm`
- `crates/hook-sdk/src/lib.rs` — `HOST_ABI_VERSION` (stable ABI anchor; remains 1 — no dispatcher changes required)
- `crates/factory-dispatcher/src/executor.rs` — `plugin_requests_block` function (sync-group block aggregation path)
- `plugins/vsdd-factory/hooks-registry.toml` — registry entries (two PreToolUse entries; both capability blocks REQUIRED)
- `.factory/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md` — authoritative design

## Story Anchor

Dual-story anchor: Phase-A — S-19.02 (`verify-factory-lock` guard implementation; W1
shipped); Phase-B — S-19.07 (`host::read_prefix` migration; forward-scheduled;
depends_on [S-19.02, S-19.06]).

## VP Anchors

TBD — VP IDs to be assigned after VP authoring pass.

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.7 | 2026-07-07 | E-19 pass-7 F-P7-006 adjudication (product-owner): Invariant 10 upper-boundary blind spot closed — soft-warn range made explicitly inclusive at cap: `bytes_read ∈ (200000, 262144]`. Previously the condition `bytes_read > soft_warn_threshold` lacked an explicit upper-bound qualifier; an implementation reading it as `> 200000 AND < 262144` (exclusive upper) would silently skip the most alarming observable state (bytes_read = 262144: 1 byte from failure, yet readable). Invariant 10 amended: (a) condition restated as `bytes_read > 200000 AND bytes_read ≤ cap_bytes (262144)`; (b) boundary table added (200000 no-warn / 200001 warn / 262144 warn+readable / 262145 OUTPUT_TOO_LARGE); (c) soft-warn range explicitly `(200000, 262144]` inclusive at cap. BC-INDEX v3.72→v3.73. |
| 1.6 | 2026-07-07 | E-19 pass-4 F-P4-001 fix burst (product-owner): Precondition 3 phased-clause amendment (architect Option B ruling; S-19.07 added to E-19; human approval 2026-07-07). Phase-A (active; S-19.02/W1 shipped): guard reads `.factory/STATE.md` via `host::read_file` with `max_bytes=262144`; `[hooks.capabilities.read_file]` with `.factory` in `path_allow`; compile-time `STATE_MD_MAX_BYTES=262144`; `extract_frontmatter` truncates at second `---` delimiter (Invariant 9 unchanged). Phase-B (S-19.07; depends_on [S-19.02, S-19.06]; forward-scheduled migration): guard MUST migrate to `host::read_prefix(path, max_bytes=8192)`; `[hooks.capabilities.read_file]` replaced with `[hooks.capabilities.read_prefix]` (same `path_allow = [".factory"]`); `STATE_MD_MAX_BYTES` constant and ALL `TooLarge`/`OutputTooLarge` handling removed — `read_prefix` guarantees NEVER `OUTPUT_TOO_LARGE` (BC-1.17.001); 8192 bound sufficient for any realistic frontmatter (<2 KB under compaction discipline); `extract_frontmatter` retained unchanged; Phase-A unregressed until S-19.07 merges. Story Anchor: dual-story Phase-A S-19.02 / Phase-B S-19.07. Traceability Stories row: TBD → S-19.02 + S-19.07. Closes F-P4-001 (BC leg); ADR-025 D18(e); human approval 2026-07-07. BC-INDEX v3.69→v3.70. |
| 1.5 | 2026-07-06 | E-19 pass-2 F-P2-001 + F-P2-003 + F-P2-011 fix burst (product-owner): (a) Precondition 1: MultiEdit enumerated in tool-pattern list (anchored form `^(Edit\|Write\|MultiEdit\|Agent)$`; ADR-025 2026-06-11 sibling-sweep mandate); Invariant 5 TOML entry updated to `tool = "Edit\|Write\|MultiEdit\|Agent"`. (b) Precondition 3 rationale: stale ~90 KB / ~466 lines replaced with verified 193,220 bytes / 488 lines at 2026-07-06 review checkpoint; cap 262144 RETAINED (architect D-c: cap inflation without compaction discipline defers the crisis; compaction is the correct response); utilization ~74%, headroom ~35%. (c) Invariant 10 added: soft_warn_threshold = 200000 bytes; state_md_approaching_cap observability-only diagnostic event (bytes_read + cap_bytes fields); zero new registry entries; verify-factory-lock guard MUST emit when bytes_read > threshold. (d) Invariant 9 verification note (F-P2-011): required property is byte-boundary correctness of frontmatter extraction (output MUST byte-equal file prefix through second `---\n` delimiter line); parity-with-full-file-parse verification FORBIDDEN. Closes F-P2-001 (BC leg), F-P2-003, F-P2-011. BC-INDEX v3.59→v3.60. |
| 1.4 | 2026-07-06 | E-19 pass-1 F-P1-004 fix burst (product-owner): Precondition 3 `max_bytes` raised 65536→262144 (256 KiB); rationale: STATE.md observed ~90 KB / ~466 lines; 500-line compaction hard cap implies worst-case ≤200 KiB; 256 KiB gives ≥28% headroom above worst-case. Invariant 9 extended with frontmatter-only-parsing mandate: guard MUST abort scanning after second `---\n` delimiter and MUST NOT parse file body; lock verdict derived exclusively from frontmatter. TD-031 in-scope fix (TD-VSDD-091): two volatile executor.rs line cites replaced with stable symbol anchors (`plugin_requests_block` function) in PC1 block-path paragraph and Invariant 8 aggregation-site paragraph. Closes F-P1-004. BC-INDEX v3.57→v3.58. |
| 1.3 | 2026-06-11 | POL-14 auto-promotion: lifecycle_status draft→active on PR #182 squash-merge df4f26b8 (S-17.02 MERGED 2026-06-11; D-545). BC-4.13.001 is now the enforcement BC for the deployed verify-factory-lock WASM guard. BC-INDEX v2.69→v2.70. No spec content changes. |
| 1.2 | 2026-06-11 | Boundary-semantics spec error corrected (product-owner; S-17.02 testing; issue #170). EC-002 and PC2 prescribed `now > expires_at` as the expiry test, which is self-contradictory: under strict `>`, `now == expires_at` evaluates false → guard would BLOCK at the exact-expiry instant, contradicting EC-002's stated outcome (boundary → Continue). Corrected to `now >= expires_at` in PC2, EC-002, and Invariant 3. PC1 blocking condition corrected from `now ≤ expires_at` to `now < expires_at` (boundary is expired, not blocking). All four locations now consistently state: the lock is expired (Continue) when `now >= expires_at`; the guard blocks only when `now < expires_at`. EC-002 outcome (boundary → Continue) unchanged. BC version v1.1→v1.2. |
| 1.1 | 2026-06-11 | Production-correctness spec gap found during S-17.02 implementation (product-owner; issue #170; S-17.02). Inv 5 registry-shape: both `[hooks.capabilities.exec_subprocess]` blocks now REQUIRE `env_allow = ["HOME", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME"]` alongside `binary_allow = ["git"]` — without env_allow the dispatcher's env_clear() causes `git config user.email` to return empty → IdentityResolutionFailed → HookResult::Continue → lock silently inert. Precondition 4 updated to document env_allow requirement. PC7 extended with env_allow dependency note (env_clear() path to IdentityResolutionFailed). EC-016 added (env_allow omitted footgun — same class as EC-007/EC-008 but subtler). 16 edge cases total EC-001..EC-016. BC version v1.0→v1.1. |
| 1.0 | 2026-06-10 | Initial authoring (product-owner; brownfield-backfill issue #170; ADR-025 v1.2 D1/D2/D9 deliverables). verify-factory-lock WASM guard: PC1 (ForeignLockHeld block), PC2 (LockExpired pass), PC3 (self-held pass), PC4 (absent/malformed fail-open), PC5 (read-only pass), PC6 (read failure fail-open), PC7 (identity resolution fail-open), PC8 (on_error=continue). 9 error variants. 15 edge cases EC-001..EC-015. 10 canonical test vectors T-1..T-10. CAP-031 registered same burst. lifecycle_status: draft (POL-14 auto-promotion on implementing PR merge). |
