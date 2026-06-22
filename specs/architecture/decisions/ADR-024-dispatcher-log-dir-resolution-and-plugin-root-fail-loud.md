---
document_type: architecture-decision-record
level: L3
adr_id: ADR-024
version: "1.4"
status: accepted
producer: architect
timestamp: 2026-06-09T00:00:00Z
amended: 2026-06-22T00:00:00Z
amendment_reason: "v1.4 (S-18.14 pass-1 adversary fix burst): (F-1 MAJOR POLICY 5) §Decision 5 §Purity Boundary corrected — phantom `InternalLog::write_started` method reference replaced with correct anchors: emission is `InternalEvent::now(DISPATCHER_STARTED)` builder chain emitted via `internal_log.write(...)` in `main.rs`; const name `DISPATCHER_STARTED` defined in `internal_log.rs`; no method named `write_started` exists in the dispatcher source. (F-5 ADVISORY POLICY 6) SS-04 subsystem-set advisory RESOLVED NO: ADR-024 subsystem set remains SS-01/SS-03/SS-07 — production fix scope is dispatcher core (SS-01), event emission (SS-03), and hook bash layer (SS-07); SS-04 (Plugin Ecosystem) appears in story/VP scope as the integration test vehicle (WASM test harness), not as a production component modified by this ADR; rationale inline in §Decision 5 SS-04 note. [Prior: v1.3 2026-06-22 S-18.14 spec-evolution (D-676): Decision 1 Addendum — resolver WASM plugin path resolution; Decision 5 — dispatcher.started log_dir observability. v1.2 2026-06-10: (C2-CRIT-2/C2-HIGH-1) Decision 3 dedup hash input tightened; (C2-CRIT-1/C2-HIGH-2) Decision 4 guard corrected; process-gap note added. v1.1 2026-06-09: pass-1 adversary amendments (level-count prose, Level E, LOW-1 control flow). v1.0 2026-06-09 initial acceptance.]"
title: "ADR-024: Dispatcher log-dir worktree-aware resolution, CLAUDE_PLUGIN_ROOT fail-loud contract, resolver WASM plugin path resolution, internal-error dedup, and destructive-guard shadow exception"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
anchors:
  - SS-01
  - SS-03
  - SS-07
  - ADR-001
  - ADR-007
  - ADR-018
  - ADR-020
  - issue-130
  - S-18.14
subsystem: "SS-01"
supersedes: null
superseded_by: null
decision_status: accepted
human_gate_required: false
human_gate_reason: "No sealed BCs modified. All decisions are within architect scope: dispatcher path-resolution algorithm, env-var contract, internal log behavior, and guard predicate narrowing."
---

# ADR-024: Dispatcher log-dir worktree-aware resolution, CLAUDE_PLUGIN_ROOT fail-loud contract, resolver WASM plugin path resolution, internal-error dedup, and destructive-guard shadow exception

## Status

**ACCEPTED.** v1.2 resolved all four open architecture questions from issue #130
("dispatcher creates recursive `.factory/.factory/logs/` shadow"). v1.3 extends the ADR
with Decision 1 subsection on resolver WASM plugin path resolution (S-18.14 spec-evolution,
HIGH defect: 8,560 `resolver.load_error` events / 0 successful loads) and Decision 5 on
`log_dir` observability. No human authorization gate — all decisions are within architect scope.

## Context

### Bug report summary (issue #130)

`resolve_log_dir()` in `crates/factory-dispatcher/src/main.rs` unconditionally appends
`.factory/logs` to `CLAUDE_PROJECT_DIR` (or to `./` when the env var is absent). When
the dispatcher is invoked with `cwd` or `CLAUDE_PROJECT_DIR` pointing to
`/project/.factory/` — the documented pattern for state-manager and agents operating on
factory artifacts — the result is `/project/.factory/.factory/logs/`, a recursive shadow
directory. `InternalLog::write_inner` calls `fs::create_dir_all` on first write, so the
shadow is silently created.

A `TODO(S-2.6)` comment at lines 661–668 of `main.rs` explicitly acknowledges the
deferral. A correct reference implementation exists in the repo's history at
`CHANGELOG.md:3664` (v0.70.0 "Worktree-aware log dir") where `bin/emit-event` used
`git worktree list --porcelain | awk '/^worktree /{print $2; exit}'` to land events in
the main worktree regardless of invocation directory.

Three secondary issues accompany the primary:

- `CLAUDE_PLUGIN_ROOT` is silently defaulted to an empty `PathBuf` at lines 267–269 and
  307–310 of `main.rs`, making every plugin-root-relative path silently wrong when the
  env var is absent, rather than failing with an actionable error.
- `InternalLog` has no per-session deduplication: identical `internal.dispatcher_error`
  events are appended on every hook invocation, creating high-volume noise.
- `destructive-command-guard.sh` guards `.factory/` recursively; if the shadow
  `.factory/.factory/` directory is ever created, deleting it requires bypassing the
  guard, creating a friction trap.

All six issue ACs are unmet at HEAD (per research cache `issue-130.md`).

### Why this is architecturally significant

This ADR spans three components:

1. **SS-01 (dispatcher core):** `resolve_log_dir()` and `resolve_registry_path()` —
   path-resolution algorithm change.
2. **SS-03 (event emission):** `InternalLog` deduplication — behavior change in the
   internal diagnostic stream.
3. **SS-07 (hook bash layer):** `destructive-command-guard.sh` — security-sensitive
   predicate narrowing.

The destructive-guard change is the most security-sensitive: it adds a new exception
path to a blocking guard. The exception must be scoped precisely so it cannot be
exploited to delete the legitimate `.factory/` worktree. This requires an explicit
architectural decision — it cannot be made ad-hoc by the implementer.

The `CLAUDE_PLUGIN_ROOT` fail-loud contract also cross-references issue #129's
canonicalization principle and sets a precedent for how the dispatcher handles absent
required env vars.

---

## Decision 1 — Log-dir resolution order (replaces `resolve_log_dir()`)

### Decision

The seven-level precedence order below replaces the current two-branch
`resolve_log_dir()`. Each level is tried in order; the first match wins.

| Level | Source | Condition | Result |
|-------|--------|-----------|--------|
| A | `VSDD_LOG_DIR` env var | set and non-empty | Use value directly; append `logs/` if value does not already end in `logs` or `logs/`. No additional `.factory` appended. |
| B | `FACTORY_ROOT` env var | set and non-empty | Use `$FACTORY_ROOT/logs`. No additional `.factory` appended. |
| C | `CLAUDE_PROJECT_DIR` / cwd — basename-is-`.factory` guard | The resolved directory's final component is `.factory` (case-insensitive on macOS/Windows; case-sensitive on Linux) | Use the path directly as the factory root; append `logs/`. Do NOT re-append `.factory`. |
| D | Walk-up to enclosing `.factory` | Neither C-condition nor level-A/B override matches; walk the parent chain from cwd up to filesystem root | First ancestor whose `file_name()` == `.factory` (case-insensitive on macOS/Windows) is used as the factory root; append `logs/`. Guard against symlink loops by tracking visited inodes (device+inode pair). Stop at filesystem root (`path.parent() == None` or `path == path.parent()`). |
| E | Cwd child `.factory` directory | Levels A–D all fail AND `<cwd>/.factory` exists as a directory | Use `<cwd>/.factory/logs`. This is a pure, subprocess-free `std::path::Path::exists()` check. Handles the dominant repo-root invocation pattern (cwd == repo root, `.factory/` is a child directory) without spawning git. |
| F | Git worktree main-root | Levels A–E all fail | Spawn `git worktree list --porcelain` with a 200ms hard timeout (see Latency section). Parse the first `worktree <path>` line. If the path exists and is a directory, use `<path>/.factory/logs`. If git exits non-zero, is unavailable, or times out: fall through to G. |
| G | Cwd fallback | Level F fails | `./.factory/logs` (current behavior). |

**Rationale for ordering:**

- A and B are explicit operator overrides; they win unconditionally. This preserves
  backward compatibility with any environment that already sets `VSDD_LOG_DIR`.
- C is the single-step fix for the primary bug: if `CLAUDE_PROJECT_DIR=/project/.factory`,
  the basename is `.factory`, so we use it directly without re-appending. This closes
  the primary bug with one predicate.
- D handles the case where neither env var is set and cwd is a subdirectory inside
  `.factory/` (e.g. cwd = `/project/.factory/cycles/`). Walking up finds `.factory`.
- **E (new)** handles the dominant real-world invocation pattern: cwd is the repo root,
  `.factory/` is a child directory. Levels C and D do not match this case (cwd basename
  is NOT `.factory`; there is no `.factory` ancestor above cwd). Without Level E, every
  repo-root hook invocation falls through to F (git subprocess, up to 200ms per call).
  Level E eliminates that subprocess for the common case with a single `Path::exists()`
  call — sub-millisecond, no I/O beyond a single `stat(2)`.
- F provides worktree consolidation (all linked worktrees land events in the main
  worktree's `.factory/logs/`) matching the v0.70.0 reference implementation. It is
  now a genuine last-resort: Level E already handles the common non-worktree case,
  so Level F fires only for linked-worktree invocations (cwd is a linked worktree root
  with no `.factory/` child, and no ancestor named `.factory`).
- G is the safe-fallback: same behavior as today when no git repo is present and
  `.factory/` does not exist as a child of cwd.

**Latency constraint (per ADR-020 Class A, p95 ≤ 1500ms):**

- Levels A–E are pure Rust `std::path` / `std::env` operations. No subprocess. Level E
  adds one `stat(2)` call (`Path::exists()`). All of A–E are sub-millisecond.
- Level F spawns `git`. The `Command::new("git")` call MUST use a hard timeout of
  200ms. If `git` is unavailable (`which git` fails at binary lookup), the
  `std::process::Command::spawn()` call returns `Err`; treat as fallthrough to G
  without logging an error (git absent is not an error condition).
- `resolve_log_dir()` MUST NEVER panic. Every branch that can produce an `Err` or
  `None` must fall through to the next level or to G.

**Case-sensitivity rule:**

- Linux: basename comparison is case-sensitive (`.factory` only).
- macOS/Windows: basename comparison is case-insensitive. Use
  `eq_ignore_ascii_case(".factory")` on `OsStr::to_str()` with a `Some` guard. If
  `to_str()` returns `None` (non-UTF-8 path), treat as not-matching and fall through.

**Symlink loop guard for level D:**

Track visited `(st_dev, st_ino)` pairs via `std::fs::symlink_metadata`. If a path has
already been seen, break the walk and fall through to level E.

---

## Decision 1 Addendum — Resolver WASM plugin path resolution (v1.3)

### Decision

Relative `plugin` paths in `resolvers-registry.toml` (e.g.,
`plugin = "hook-plugins/vsdd-context-resolvers.wasm"`) MUST be resolved against the
TOML file's parent directory — which equals `CLAUDE_PLUGIN_ROOT` at runtime — NOT
against the dispatcher's process working directory (CWD).

**Behavioral contract (function-name anchors per TD-VSDD-091):**

In `resolver_loader::load_registry`:

1. After parsing `resolvers-registry.toml`, the function obtains `toml_path.parent()` —
   the directory containing the TOML file.
2. For each resolver entry, if `entry.plugin` is a relative path (i.e.,
   `Path::new(&entry.plugin).is_absolute()` is `false`), the absolute path MUST be
   constructed as `toml_parent.join(&entry.plugin)`.
3. The joined absolute path — NOT the bare relative string — is what is passed to
   `get_or_compile` for WASM compilation and to `path.canonicalize()` for filesystem
   existence validation.
4. If `entry.plugin` is already absolute, pass it through unchanged.
5. This resolution MUST be applied at EVERY call site to `get_or_compile` within
   `load_registry`, for both `fail_closed: true` and `fail_closed: false` code paths.

**Why CWD-relative was wrong:**

The dispatcher is invoked by the Claude Code hook infrastructure. At hook invocation time,
the process CWD is the host project directory (e.g., `/Users/<user>/project/`). The WASM
plugin files live under `CLAUDE_PLUGIN_ROOT` (e.g.,
`~/.claude/plugins/cache/claude-mp/vsdd-factory/<version>/`). A relative path
`hook-plugins/vsdd-context-resolvers.wasm` from the TOML resolves correctly only when
the base is the TOML's own parent directory. Using CWD as the base yields a path
that does not exist, causing `path.canonicalize()` to return `Err(ENOENT)`, which the
dispatcher surfaces as `resolver.load_error`. This is the root cause of the 8,560
`resolver.load_error` / 0 successful loads observed empirically since rc.21.

**Purity boundary note:**

`load_registry` is an effectful-shell function (reads filesystem). The path-joining step
`toml_parent.join(&entry.plugin)` is a pure computation (no I/O) and is freely
unit-testable with an absolute synthetic TOML path.

**Relationship to Decision 2 (`CLAUDE_PLUGIN_ROOT` fail-loud contract):**

Decision 2 governs the hooks-registry path when `CLAUDE_PLUGIN_ROOT` is absent. This
addendum governs how the resolver WASM paths within an already-loaded
`resolvers-registry.toml` are interpreted. The two decisions are complementary: Decision 2
handles registry-not-found; this addendum handles wasm-within-registry-not-found.

**Release dependency (operator-cache):**

Per CLAUDE.md "Dispatcher binary discipline", the fix to `resolver_loader.rs` must be
RELEASED (rc tag + cross-platform binary build via `.github/workflows/release.yml`) to
reach the operator-level cache at
`~/.claude/plugins/cache/claude-mp/vsdd-factory/<version>/`. Develop-branch edits do not
affect the cached dispatcher binary consumed by the running Claude Code harness. A release
MUST be cut after S-18.14 merges.

**Test obligation (S-18.14):**

Unit test in `crates/factory-dispatcher/` (or `crates/context-resolvers/`):

1. Construct a synthetic `resolvers-registry.toml` with a relative `plugin` path (e.g.,
   `plugin = "hook-plugins/vsdd-context-resolvers.wasm"`).
2. Call `load_registry` with an ABSOLUTE path to the synthetic TOML (e.g.,
   `/tmp/test-root/resolvers-registry.toml`), simulating `CLAUDE_PLUGIN_ROOT/resolvers-registry.toml`.
3. Assert the resolved WASM path is `/tmp/test-root/hook-plugins/vsdd-context-resolvers.wasm`
   (TOML-parent-relative), NOT `<process CWD>/hook-plugins/vsdd-context-resolvers.wasm`.
4. Assert that a CWD-relative resolution would produce a DIFFERENT path, proving the test
   distinguishes the old (buggy) behavior from the new (correct) behavior.

---

## Decision 5 — `log_dir` field in `dispatcher.started` event (v1.3)

### Decision

The `dispatcher.started` event payload MUST include a `log_dir` field whose value is the
absolute path to the directory where the dispatcher writes its internal log for this
invocation.

**Behavioral contract:**

- The `log_dir` value is populated from `InternalLog::log_dir()` — the accessor that
  already exists on the `InternalLog` struct (no new computation required).
- The field is emitted unconditionally on every `dispatcher.started` event. It is NOT
  optional, NOT behind a feature flag.
- The value is the resolved absolute path (the same path produced by Decision 1's
  seven-level precedence order after it has resolved to its final value).

**Rationale:**

Without `log_dir` in `dispatcher.started`, operators cannot determine where to find the
dispatcher's internal event log without tracing through the seven-level resolution algorithm
manually. This is a low-effort, high-value observability improvement: the accessor already
exists, the only change is wiring it into the started payload.

**Purity boundary:**

`dispatcher.started` is emitted once per dispatcher invocation via the
`InternalEvent::now(DISPATCHER_STARTED)` builder chain, emitted via
`internal_log.write(...)` in `main.rs`. (`DISPATCHER_STARTED` is the
`"dispatcher.started"` string constant defined in `internal_log.rs`. No method named
`write_started` exists in the dispatcher source — TD-VSDD-091 requires function-name
anchors, not phantom method references.) Wiring `log_dir()` into that call is an
effectful-shell operation (reads from the already-resolved `PathBuf` stored on
`InternalLog`). No new filesystem I/O is required.

**SS-04 subsystem scope note (F-5 advisory resolution):**

This ADR's subsystem set is SS-01 (dispatcher core), SS-03 (event emission), and SS-07
(hook bash layer). SS-04 (Plugin Ecosystem) is NOT added. The S-18.14 story and
VP-073/074/075 include SS-04 because the integration test vehicle for this story is a
WASM hook plugin — that test vehicle lives in the plugin ecosystem (SS-04). However,
the production change governed by this ADR touches only `main.rs`, `internal_log.rs`,
`resolver_loader.rs`, and `destructive-command-guard.sh` — all SS-01/SS-03/SS-07
production components. The ADR documents architectural decisions for the production
components; the test-vehicle scope is owned by the story and VPs, not the ADR.

**Test obligation:**

The existing `dispatcher.started` event test MUST assert that the emitted JSON contains a
`log_dir` string field whose value is the directory path (not empty, not null).

---

## Decision 2 — `CLAUDE_PLUGIN_ROOT` fail-loud contract

### Decision

Replace the current `unwrap_or_default()` silent fallback with a two-tier approach:

**Tier 1 — fail-loud diagnostic (replaces `unwrap_or_default()` at lines 267–269 and 307–310):**

When `CLAUDE_PLUGIN_ROOT` is absent or empty, the dispatcher MUST:

1. Emit an `internal.dispatcher_error` event with `message` =
   `"$CLAUDE_PLUGIN_ROOT is not set or empty — hook registry and resolver registry paths unresolvable; all plugins will be skipped. Set CLAUDE_PLUGIN_ROOT to the vsdd-factory plugin directory."`.
2. Set `plugin_root = PathBuf::new()` (empty) and `resolvers_registry_path` to an
   empty path so that registry loading returns the documented
   "absent registry → empty registry, not an error" path per BC-1.13.001 INV2.
3. Continue dispatch — do not hard-abort. Dispatching with an empty plugin set is
   degraded but non-crashing behavior (plugins are simply skipped). This preserves the
   dispatcher's non-panicking contract.

**Tier 2 — registry-path resolution for `resolve_registry_path()` (lines 655–659):**

`resolve_registry_path()` currently propagates an error that causes `run()` to return
early and `main()` to call `emit_dispatcher_error`. This flow is already correct for the
case where `CLAUDE_PLUGIN_ROOT` is absent. The change required is to make the error
message actionable:

```
$CLAUDE_PLUGIN_ROOT is not set — cannot resolve hooks-registry.toml.
Ensure the vsdd-factory plugin is installed and CLAUDE_PLUGIN_ROOT points to its
directory (e.g. ~/.claude/plugins/cache/claude-mp/vsdd-factory/<version>/).
```

**No plugin-cache install-path fallback.** A hardcoded fallback path
(e.g. `~/.claude/plugins/cache/claude-mp/vsdd-factory/...`) would silently consume a
stale or wrong-version registry. The correct fix is to ensure `CLAUDE_PLUGIN_ROOT` is
always set by the install/activation flow (DX concern, not dispatcher concern). The
dispatcher's job is to fail loud and tell the operator what to fix — not to guess.

**Intended control-flow when `CLAUDE_PLUGIN_ROOT` is absent (LOW-1 clarification):**

The intended behavior is **degraded-continue** — emit the Tier-1 diagnostic error and
process the dispatch with an empty plugin set (no plugins run), exiting 0. This is NOT
a hard abort.

To make this intent reachable, the implementer MUST ensure the Tier-1 check at startup
(the `unwrap_or_default()` replacement at lines 267–269 and 307–310) executes BEFORE
`resolve_registry_path()` is called. The call order in `run()` MUST be:

1. Read `CLAUDE_PLUGIN_ROOT` env var.
2. If absent/empty: emit `internal.dispatcher_error` (Tier 1), set `plugin_root =
   PathBuf::new()` (empty), and proceed to dispatch with empty plugin set — DO NOT call
   `resolve_registry_path()` (there is nothing to resolve).
3. If present and non-empty: call `resolve_registry_path()` (Tier 2). If that returns
   an error, emit the actionable Tier-2 error message and return early (current behavior,
   which is correct for the "env var set but path invalid" case).

This means Tier 1 and Tier 2 are mutually exclusive paths, not sequential checks.
The adversary concern that `resolve_registry_path()` may be called before Tier-1 runs
is valid under the current implementation; the implementer must restructure the call
order so that an absent `CLAUDE_PLUGIN_ROOT` short-circuits before any
`resolve_registry_path()` call.

**Summary: `CLAUDE_PLUGIN_ROOT` absent → degraded-continue (exit 0, empty plugin set,
`internal.dispatcher_error` emitted). `CLAUDE_PLUGIN_ROOT` set but path invalid →
hard-error (Tier-2 actionable message, return early from `run()`).**

**Relationship to issue #129:** This decision is an instance of the general
canonicalization-and-fail-loud principle in #129. ADR-024 codifies the specific
behavior for this env var; the broader principle will be codified separately in #129's
resolution.

---

## Decision 3 — Internal-error dedup / rate-limiting

### Decision

Add per-session deduplication to `InternalLog` for `internal.dispatcher_error` events
only, using a fixed-capacity seen-set stored as a `HashSet<u64>` (hash of a
bounded-prefix of the raw message string value) behind a `Mutex` on `InternalLog`.

**Hash input specification (amended v1.2):**

The hash input is constructed as:

```
hash_input = event.type_ + ":" + bounded_prefix(message_string_value, N=4096)
```

Where:

- `message_string_value` = `event.message.as_str().unwrap_or("")` — the raw string
  value of the `message` JSON field, NOT the JSON representation (`to_string()` /
  `as_json`). If the `message` field is absent, not a JSON string type, or
  `Value::Null`, use the empty string `""`.
- `bounded_prefix(s, N)` = the longest prefix of `s` that is both (a) at most N bytes
  long and (b) ends on a valid UTF-8 char boundary. In Rust:
  ```rust
  let n = N.min(s.len());
  let safe_n = s.floor_char_boundary(n); // std::str floor_char_boundary (stable Rust 1.80+)
  &s[..safe_n]
  ```
  For targets below Rust 1.80, use the manual fallback: scan backwards from `n` while
  `!s.is_char_boundary(n) { n -= 1; }`. This is O(1) in practice (≤3 iterations for
  any valid UTF-8 sequence).
- **N = 4096 bytes.** This bound is chosen to:
  - Eliminate the char-boundary panic seen in the pass-1 implementation (the old 256-byte
    `as_bytes()[..256]` slice could split a multibyte char).
  - Avoid JSON-quote false-collision: using `Value::as_str()` returns the raw string
    content without JSON escape sequences, so two messages with different literal
    content always produce different prefixes (up to byte N).
  - Bound per-event hashing cost: a registry/toml parse error can embed a multi-MB
    offending TOML fragment in the message. Hashing the full string would be O(MB)
    per invocation. With N=4096 the cost is O(4 KiB) unconditionally.
  - Preserve correct dedup for all realistic dispatcher errors: every known error
    message class (env-var absent, registry parse failure, plugin crash, payload
    malformed) differs well within the first 4096 bytes.

**Full specification:**

- `InternalLog` gains a new field: `seen_errors: Mutex<HashSet<u64>>`.
- `write_inner` checks: before writing any event whose `type_` == `"internal.dispatcher_error"`,
  compute `hash = DefaultHasher(event.type_ + ":" + bounded_prefix(message_string_value, 4096))`.
  If `seen_errors` already contains the hash, skip the write (return `Ok(())`). Otherwise
  insert the hash and proceed with the write.
- The dedup applies ONLY to `internal.dispatcher_error`. All other event types
  (including other `internal.*` types) are written unconditionally.
- **Cap the seen-set at 1024 entries.** If the set reaches 1024, stop inserting (do not
  evict). This prevents unbounded memory growth in a runaway session without adding
  complexity.
- **Non-panicking contract preserved.** `Mutex::lock()` failure → log the event anyway
  (treat as if dedup check passed). `HashSet` operations cannot panic.
- **No persistence.** The seen-set is in-process memory. Process restart (new dispatcher
  invocation) starts fresh. This is correct: each dispatcher invocation is one hook
  event, so "per-session" here means "per-process lifetime" which is milliseconds.

**Residual tradeoff (accepted):** Two distinct error messages that differ ONLY in bytes
after position 4096 will dedup to the same hash and one will be suppressed. This is
pathological — it requires two messages with identical type and identical first 4096 bytes
but different content thereafter. No known dispatcher error class has this property.
The tradeoff is explicitly accepted in exchange for bounded hashing cost.

**Why only `internal.dispatcher_error`?**

This is the noisy event class: it fires on every hook invocation when `CLAUDE_PLUGIN_ROOT`
is absent or when a malformed payload arrives. Other `internal.*` events (capability
denial, plugin invocations, lifecycle events) are per-invocation and must not be
deduplicated — they contain unique trace IDs and are used for debugging.

**Relationship to payload.rs:** The `#[serde(alias = "hook_event_name")]` fix already
eliminates the most common cause of `missing field 'event_name'` noise (per
`payload.rs:16-23`). Dedup provides defense-in-depth for residual noise from legacy or
malformed payloads.

**Testing obligation (C2-MED-1/MED-2):** The test-writer MUST update `internal_log.rs`
dedup test doc-blocks to reflect the bounded-full-value contract: hash input is
`bounded_prefix(Value::as_str(), 4096)`, not "256 bytes of JSON repr". Tests that assert
dedup identity must use raw string values (not JSON-escaped) as their message inputs.

---

## Decision 4 — Destructive-op guard shadow exception (SECURITY-SENSITIVE)

### Decision

Add a single targeted exception to `destructive-command-guard.sh`'s
`.factory/`-recursive-delete guard (lines 73–90) that permits deletion of paths strictly
inside `.factory/.factory/` (the recursive shadow), while keeping all real `.factory/`
deletion blocked.

**v1.1 predicate retracted.** The v1.1 substring predicate
(`[[ "$COMMAND" == *".factory/.factory"* ]]`) is INCORRECT and is replaced in full by
the lexical path-normalization predicate specified below. The v1.1 "structurally
impossible" security-analysis claim is also retracted — two exploits existed:

1. **Traversal under-protect:** `rm -rf .factory/.factory/../specs` contains the
   substring `.factory/.factory`, so the v1.1 predicate ALLOWED it. But lexical
   normalization resolves the path to `.factory/specs` — outside the shadow root —
   so the deletion MUST be blocked.
2. **Nested-shadow over-block:** `rm -rf .factory/.factory/.factory` (a path that is
   entirely INSIDE the shadow root) does NOT match the v1.1 regex
   `\.factory/\.factory(/|$)` because it has three `.factory` components, so it was
   wrongly BLOCKED. It must be allowed.

**Corrected predicate algorithm (Bash):**

For each target argument in the command that contains `.factory`:

1. Tokenize the `rm`/`find` target arguments from `$COMMAND`.
2. For each token that contains `.factory`, lexically normalize the path by
   resolving all `.` and `..` components without filesystem access:
   - Split on `/`.
   - Maintain a stack: push non-empty non-`.` components; on `..` pop the last
     stack entry (if any); ignore empty or `.` entries.
   - Reconstruct the normalized path from the stack.
3. Check: does the normalized path start with `.factory/.factory/` (or equal
   `.factory/.factory` exactly)?
   - YES → this token is inside the shadow root; continue checking other tokens.
   - NO → this token resolves outside the shadow root; the entire command MUST be
     BLOCKED (exit 2). Do NOT take the exception.
4. If ALL `.factory`-bearing tokens pass step 3 (every one normalizes to inside the
   shadow root), take the exception (allow the command).

**Conservative simplification (acceptable):** Any `.factory`-bearing token that
contains a `..` component adjacent to `.factory` (i.e., `..` appears directly before
or after a `.factory` path component) MUST be treated as outside the shadow root and
the command blocked. This is conservative — some such paths could theoretically
normalize to inside the shadow — but `..` adjacent to `.factory` is a strong signal
of a traversal attempt and the conservative rejection is explicitly accepted over a
more complex normalization that could be mis-implemented.

**Bash implementation sketch (for implementer guidance):**

```bash
# Returns 0 if ALL .factory-bearing tokens in "$1" normalize to inside the shadow root.
# Returns 1 if any token resolves outside the shadow root (or contains suspicious ..).
_all_targets_inside_shadow() {
  local cmd="$1"
  local found_factory_token=0
  # Tokenize on whitespace; skip flags (starting with -)
  for token in $cmd; do
    [[ "$token" == -* ]] && continue
    if [[ "$token" != *".factory"* ]]; then continue; fi
    found_factory_token=1
    # Conservative: reject any .. adjacent to a .factory component.
    # Matches: .factory/.. (factory then dotdot) OR ../.factory (dotdot then factory).
    if [[ "$token" =~ \.factory/\.\. ]] || [[ "$token" =~ \.\./\.factory ]]; then
      return 1
    fi
    # Lexical normalization
    local stack=()
    IFS='/' read -ra parts <<< "$token"
    for part in "${parts[@]}"; do
      if [[ "$part" == ".." ]]; then
        (( ${#stack[@]} > 0 )) && unset 'stack[-1]'
      elif [[ -n "$part" && "$part" != "." ]]; then
        stack+=("$part")
      fi
    done
    local normalized
    normalized=$(IFS='/'; echo "${stack[*]}")
    # Must start with .factory/.factory
    if [[ "$normalized" != ".factory/.factory" && \
          "$normalized" != ".factory/.factory/"* ]]; then
      return 1
    fi
  done
  # If no .factory token was found at all, this function should not have been called;
  # return 1 (do not grant exception without evidence of shadow target).
  (( found_factory_token == 0 )) && return 1
  return 0
}
```

Insert the exception INSIDE the existing `for protected_re in ...` loop, BEFORE the
existing `.worktrees/` exception (line 77):

```bash
if _all_targets_inside_shadow "$COMMAND"; then
  continue  # All .factory-bearing targets normalize to inside .factory/.factory/
fi
```

**Allow/block matrix (authoritative; implementer and test-writer MUST satisfy):**

| Command | Outcome | Reason |
|---------|---------|--------|
| `rm -rf .factory/` | BLOCK | Normalizes to `.factory` — outside shadow |
| `rm -rf .factory/specs` | BLOCK | Normalizes to `.factory/specs` — outside shadow |
| `rm -rf .factory/ .factory/.factory` | BLOCK | Multi-target: `.factory/` token outside shadow |
| `rm -rf .factory/.factory/../specs` | BLOCK | Normalizes to `.factory/specs` — outside shadow (traversal) |
| `rm -rf .factory/.factory/..` | BLOCK | `..` adjacent to `.factory` — conservative reject |
| `find .factory -delete` | BLOCK | `.factory` normalizes outside shadow |
| `find .factory/.factory/.. -delete` | BLOCK | `..` adjacent — conservative reject |
| `find .factory -delete ; find .factory/.factory -delete` | BLOCK | First find target outside shadow |
| `rm -rf .factory/.factory/` | ALLOW | Normalizes to `.factory/.factory` — inside shadow |
| `rm -rf .factory/.factory/logs` | ALLOW | Normalizes to `.factory/.factory/logs` — inside shadow |
| `rm -rf .factory/.factory/.factory` | ALLOW | Normalizes to `.factory/.factory/.factory` — inside shadow (nested) |
| `find .factory/.factory -delete` | ALLOW | Normalizes to `.factory/.factory` — inside shadow |

**Security analysis (corrected):**

The exception is TARGET-RESOLUTION-SCOPED, not substring-scoped. An attacker must
provide a target argument that lexically normalizes to inside `.factory/.factory/` —
the shadow root. The real factory worktree is mounted at `.factory/` and its
subdirectories are `.factory/specs`, `.factory/stories`, `.factory/STATE.md`, etc.
None of these normalize to `.factory/.factory/<anything>` unless a `..` escape is used,
and the conservative `..`-adjacent-to-`.factory` rule blocks all such attempts. The
only way to satisfy the predicate is to target paths that genuinely begin with
`.factory/.factory/` after normalization.

**Prior claim retracted:** The v1.1 claim that exploiting the substring exception is
"structurally impossible" is FALSE. The substring predicate admitted traversal attacks
(`rm -rf .factory/.factory/../specs`) and incorrectly blocked legitimate nested-shadow
operations (`rm -rf .factory/.factory/.factory`). Both are fixed by the normalization
predicate above.

**Find-delete guard update:** The `find ... -delete` block (lines 148–158) uses
`\.factory\b` as its pattern. Add the same `_all_targets_inside_shadow` call inside
that block (same placement, same logic).

**Git rm guard:** The `git rm` block (lines 277–286) guards `.factory/specs/`,
`.factory/stories/`, `.factory/STATE.md`. These path strings normalize to outside the
shadow root and are unaffected. No change needed.

**Testing obligation (C2-CRIT-1/C2-HIGH-2):** The test-writer MUST produce bats tests
asserting the full allow/block matrix above. At minimum:
- `rm -rf .factory/` → exits 2 (BLOCK).
- `rm -rf .factory/specs` → exits 2 (BLOCK).
- `rm -rf .factory/.factory/../specs` → exits 2 (BLOCK — traversal attempt).
- `rm -rf .factory/.factory/..` → exits 2 (BLOCK — conservative `..` reject).
- `rm -rf .factory/ .factory/.factory` → exits 2 (BLOCK — multi-target with real path).
- `rm -rf .factory/.factory/` → exits 0 (ALLOW — exact shadow root).
- `rm -rf .factory/.factory/logs` → exits 0 (ALLOW — shadow subdirectory).
- `rm -rf .factory/.factory/.factory` → exits 0 (ALLOW — nested shadow, was wrongly blocked by v1.1).
- `find .factory/.factory -delete` → exits 0 (ALLOW — shadow root, find form).
- `find .factory/.factory/.. -delete` → exits 2 (BLOCK — conservative `..` reject).

**Note on `main.rs` doc-comment (C2-HIGH-3):** The `resolve_log_dir` doc-comment in
`main.rs` MUST describe the algorithm as "seven-level A–G". This is not an ADR change
(the seven levels are already correct in Decision 1); it is a code-comment correctness
obligation for the implementer. Confirmed here as a testing and review checkpoint.

---

## Alternatives Considered

### Log-dir resolution: "Only fix level C, skip git subprocess"

Level C alone closes the primary bug. Levels D, E, and F are necessary: D handles
cwd-inside-`.factory/`; E handles the dominant repo-root pattern without spawning git;
F handles linked-worktree consolidation (v0.70.0 capability). Without D–F, those cases
either produce fragmented logs or impose a git subprocess on every common invocation.
Rejected: production-grade default requires full restoration of v0.70.0 behavior and
elimination of the unnecessary git subprocess for repo-root invocations.

### `CLAUDE_PLUGIN_ROOT`: Hard-abort on missing env var

Hard-aborting (exit 2) when `CLAUDE_PLUGIN_ROOT` is absent would block ALL hook
processing when the env var is unset — including hooks invoked from test harnesses that
intentionally run without a real plugin root. The fail-loud-but-continue tier preserves
the dispatcher's degraded-but-functional contract. Rejected.

### Internal-error dedup: Ring buffer with eviction

A ring buffer would evict old entries and re-log them after rotation. For the specific
`internal.dispatcher_error` use case, re-logging adds no value: the message content is
identical. Fixed-cap no-eviction is simpler and correct for this use case. Rejected.

### Guard exception: Allow `.factory/` deletion when target is "clearly a shadow"

A heuristic (e.g., "allow if `.factory/.factory/` exists on disk") would require a
filesystem read inside the guard. The guard must be deterministic and sub-50ms per its
contract. A pure lexical approach is the only acceptable mechanism.

The v1.1 substring predicate (`*".factory/.factory"*`) was incorrect: it admitted
traversal attacks (`rm -rf .factory/.factory/../specs` normalizes to outside the shadow)
and over-blocked nested-shadow operations (`rm -rf .factory/.factory/.factory` is fully
inside the shadow). The correct mechanism is lexical path normalization per Decision 4
above — it is deterministic, sub-millisecond, requires no filesystem I/O, and correctly
handles both traversal attempts and nested shadow paths.

### Decision 3 dedup: Full `Value::as_str()` with no bound

Hashing the full raw string value with no byte ceiling bounds the correctness risk
(no false-collisions from truncation) but unbounds the cost: a single TOML parse error
embedding a multi-MB fragment would hash O(MB) on every hook invocation. The 4096-byte
ceiling is the correct tradeoff: it eliminates the cost risk, still eliminates JSON-quote
false-collisions (raw string value, not JSON repr), and is char-boundary-safe. Rejected:
unbounded cost in adversarial input scenarios violates the production-grade default.

---

## Consequences

### Behavioral changes

1. `resolve_log_dir()` no longer creates `.factory/.factory/logs/` when invoked with
   `CLAUDE_PROJECT_DIR` pointing at `.factory/`. AC-1 and AC-2 of issue #130 are closed.
2. `CLAUDE_PLUGIN_ROOT` absence produces an actionable stderr + `internal.dispatcher_error`
   diagnostic. AC-3 partially closed (the "not set" message becomes actionable; the silent
   default fallback is replaced with explicit degraded-mode behavior).
3. Identical `internal.dispatcher_error` messages are logged at most once per process
   lifetime. Dedup hash is keyed on `event.type_ + ":" + bounded_prefix(Value::as_str(), 4096)` —
   raw string value (not JSON repr), char-boundary-safe ceiling at 4096 bytes. AC-4 closed.
4. `destructive-command-guard.sh` permits deletion of paths inside `.factory/.factory/`
   (the recursive shadow root) via lexical path-normalization predicate, while keeping
   all real `.factory/` deletion blocked. Traversal attacks (`..` adjacent to `.factory`)
   are rejected conservatively. AC-6 closed.
5. AC-5 (regression test) is a test-writer deliverable, not an architectural decision.

### Files to change

| File | Change |
|------|--------|
| `crates/factory-dispatcher/src/main.rs` | Replace `resolve_log_dir()` (seven-level A–G per Decision 1); update `plugin_root` defaulting at `CLAUDE_PLUGIN_ROOT`-absent sites; update `resolve_registry_path()` error message; wire `InternalLog::log_dir()` into `dispatcher.started` payload (Decision 5) |
| `crates/factory-dispatcher/src/resolver_loader.rs` | In `load_registry` and at all `get_or_compile` call sites: resolve relative `entry.plugin` paths against `toml_path.parent()` before passing to `get_or_compile` / `path.canonicalize()` (Decision 1 Addendum) |
| `crates/factory-dispatcher/src/internal_log.rs` | Add `seen_errors: Mutex<HashSet<u64>>` field to `InternalLog`; update `write_inner` with dedup check; expose `log_dir()` accessor if not already present |
| `plugins/vsdd-factory/hooks/destructive-command-guard.sh` | Add shadow exception inside the `.factory/`-recursive-delete loop (lines 73–90) and inside the find-delete guard (lines 148–158) |

### Process note — spec-drift routing obligation

**When an implementer's TDD fix changes behavior that an `accepted` ADR specifies
verbatim, the fix-burst MUST route an architect ADR amendment in the SAME burst.**

This obligation derives from CLAUDE.md Architectural Authority Rule 12: "the SPEC wins
— code is brought into alignment via fix-burst or follow-up story." When the implementer
chose to change `dedup_hash_for` from the v1.1-specified "256-byte JSON repr slice" to
the full `Value::as_str()` (to fix the char-boundary panic), that was a correct
implementation judgment — but it created spec-vs-code drift that persisted through
commit and required a second adversary pass (C2-CRIT-2) to surface. The routing
obligation that should have fired at Commit B of the implementer's fix-burst:

> Implementer changes behavior specified in ADR-024 Decision 3 → routes finding to
> orchestrator → orchestrator dispatches architect for ADR amendment → ADR amendment
> lands in the SAME fix-burst commit sequence before merge.

This ADR amendment (v1.2) is the retroactive closure. The process-gap lesson is
codified here so future implementers encountering similar ADR-specified behaviors treat
the spec-vs-code drift as a routing trigger, not a silent override.

### Release requirement

This is a `crates/` dispatcher change. Per CLAUDE.md "Dispatcher binary discipline",
the fix must be RELEASED (rc tag + cross-platform binary build via
`.github/workflows/release.yml`) to reach the operator-level cache at
`~/.claude/plugins/cache/claude-mp/vsdd-factory/<version>/`. Develop-branch edits do
not affect the cached plugin binary consumed by the running Claude Code harness.

The bash hook change (`destructive-command-guard.sh`) is a plugin-source change and
also requires a release to take effect in the operator-level cache.

### Latency budget compliance (ADR-020)

- Levels A–E: < 1ms (pure env/path operations; Level E adds one `stat(2)` call).
- Level F (git): bounded by 200ms timeout on `git worktree list --porcelain`. In the
  nominal case (git present, in-repo): ~5–20ms. In the timeout case: 200ms +
  fallthrough. Level F is now a genuine last-resort for linked-worktree invocations
  only. The dominant repo-root pattern is handled by Level E (sub-millisecond, no git).
- Total log-dir resolution overhead at dispatch time: nominally < 1ms for all
  repo-root invocations (levels A–E); worst-case (level F timeout) ≤ 200ms, still
  within ADR-020 Class A p95 = 1500ms budget.

### Purity boundary (SS-01 purity notes)

`resolve_log_dir()` is currently a pure function returning a `PathBuf`. After this
change it remains pure for levels A–E (reads env vars + walks path metadata; Level E
adds one `stat(2)` call which is a read-only filesystem probe). Level F spawns a
subprocess — this moves the function from "pure" to "effectful shell" in the purity
boundary sense. This is acceptable: `resolve_log_dir()` is explicitly a side-effect
initialization step at dispatcher startup, not a per-hook pure computation. No formal
verification applies to this function; integration test coverage (AC-5) is the required
verification mechanism.

---

## Decision Log

| Version | Date | Author | Change |
|---------|------|--------|--------|
| 1.0 | 2026-06-09 | architect | Initial acceptance. Closes all four architecture questions from issue #130. |
| 1.1 | 2026-06-09 | architect | Pass-1 adversary review amendments: (M-1) corrected level-count prose from "five-level" to "seven-level" throughout; (L-2) added Level E (cwd child `.factory` directory check) between old Level D and old Level E (git), renumbered old E→F and old F→G, eliminates git subprocess for dominant repo-root invocation pattern; (LOW-1) added explicit control-flow intent for `CLAUDE_PLUGIN_ROOT`-absent: degraded-continue (Tier 1, empty plugin set, exit 0) with Tier 1 check occurring BEFORE `resolve_registry_path()` call, making the two tiers mutually exclusive. |
| 1.2 | 2026-06-09 | architect | Pass-2 adversary review amendments: (C2-CRIT-2/C2-HIGH-1) Decision 3 hash input changed from "first 256 bytes of message_json_value" (JSON repr, fixed byte slice) to "bounded_prefix(Value::as_str(), N=4096)" (raw string value, char-boundary-safe, 4096-byte ceiling) — simultaneously fixes char-boundary panic, JSON-quote false-collision, and unbounded hashing cost; accepted residual tradeoff: messages differing only after byte 4096 dedup to same hash (pathological, accepted); (C2-CRIT-1/C2-HIGH-2) Decision 4 guard predicate replaced: v1.1 substring predicate removed and replaced with lexical path-normalization predicate — tokenize targets, normalize `.`/`..` components, allow only when ALL `.factory`-bearing tokens normalize to strictly inside `.factory/.factory/`, conservative `..`-adjacent reject rule added; full allow/block matrix added; "structurally impossible" security-analysis claim retracted; nested-shadow over-block and traversal under-protect both fixed; (process-gap) Process note added recording spec-drift routing obligation; (C2-HIGH-3) Decision 4 testing obligation note added for `main.rs` doc-comment seven-level assertion; (C2-MED-1/MED-2) Decision 3 testing obligation note added for `internal_log.rs` dedup test doc-block regrounding. |
| 1.3 | 2026-06-22 | architect | S-18.14 spec-evolution (D-676): (1) Decision 1 Addendum — Resolver WASM plugin path resolution: relative `plugin` paths in `resolvers-registry.toml` MUST resolve against `toml_path.parent()` (= `CLAUDE_PLUGIN_ROOT`) NOT process CWD; applies to `load_registry` and all `get_or_compile` call sites in `resolver_loader`; root cause of 8,560 `resolver.load_error` / 0 successful loads since rc.21; unit test obligation specified (distinguish TOML-parent-relative vs CWD-relative); release dependency documented; (2) Decision 5 — `log_dir` observability: `dispatcher.started` event payload MUST include `log_dir` field from `InternalLog::log_dir()`; no new computation required; test obligation added; (3) `resolver_loader.rs` added to Files to change table; ADR-018 and S-18.14 added to anchors. |
| 1.4 | 2026-06-22 | architect | S-18.14 pass-1 adversary fix burst: (F-1 MAJOR POLICY 5) §Decision 5 §Purity Boundary corrected — phantom `InternalLog::write_started` method reference removed and replaced with correct source anchors: `DISPATCHER_STARTED` const (defined in `internal_log.rs`) emitted via `InternalEvent::now(DISPATCHER_STARTED)` builder chain called via `internal_log.write(...)` in `main.rs`; no method named `write_started` exists — verified by `grep write_started crates/factory-dispatcher/src` → zero matches; TD-VSDD-091-compliant behavioral anchors used throughout. (F-5 ADVISORY POLICY 6) SS-04 subsystem-set advisory RESOLVED NO: ADR-024 subsystem set retained as SS-01/SS-03/SS-07; SS-04 scope belongs to story/VP test-vehicle (integration test WASM harness), not to the production components governed by this ADR; SS-04 rationale note added to §Decision 5. |
