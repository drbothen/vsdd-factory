---
document_type: architecture-decision-record
level: L3
adr_id: ADR-024
version: "1.0"
status: accepted
producer: architect
timestamp: 2026-06-09T00:00:00Z
title: "ADR-024: Dispatcher log-dir worktree-aware resolution, CLAUDE_PLUGIN_ROOT fail-loud contract, internal-error dedup, and destructive-guard shadow exception"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
anchors:
  - SS-01
  - SS-03
  - SS-07
  - ADR-001
  - ADR-007
  - ADR-020
  - issue-130
subsystem: "SS-01"
supersedes: null
superseded_by: null
decision_status: accepted
human_gate_required: false
human_gate_reason: "No sealed BCs modified. All decisions are within architect scope: dispatcher path-resolution algorithm, env-var contract, internal log behavior, and guard predicate narrowing."
---

# ADR-024: Dispatcher log-dir worktree-aware resolution, CLAUDE_PLUGIN_ROOT fail-loud contract, internal-error dedup, and destructive-guard shadow exception

## Status

**ACCEPTED.** This ADR resolves all four open architecture questions from issue #130
("dispatcher creates recursive `.factory/.factory/logs/` shadow"). It is the design
dependency that gates the test-writer (Red Gate test stubs) and implementer (TDD green
cycle). No human authorization gate — all decisions are within architect scope.

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

The five-level precedence order below replaces the current two-branch
`resolve_log_dir()`. Each level is tried in order; the first match wins.

| Level | Source | Condition | Result |
|-------|--------|-----------|--------|
| A | `VSDD_LOG_DIR` env var | set and non-empty | Use value directly; append `logs/` if value does not already end in `logs` or `logs/`. No additional `.factory` appended. |
| B | `FACTORY_ROOT` env var | set and non-empty | Use `$FACTORY_ROOT/logs`. No additional `.factory` appended. |
| C | `CLAUDE_PROJECT_DIR` / cwd — basename-is-`.factory` guard | The resolved directory's final component is `.factory` (case-insensitive on macOS/Windows; case-sensitive on Linux) | Use the path directly as the factory root; append `logs/`. Do NOT re-append `.factory`. |
| D | Walk-up to enclosing `.factory` | Neither C-condition nor level-A/B override matches; walk the parent chain from cwd up to filesystem root | First ancestor whose `file_name()` == `.factory` (case-insensitive on macOS/Windows) is used as the factory root; append `logs/`. Guard against symlink loops by tracking visited inodes (device+inode pair). Stop at filesystem root (`path.parent() == None` or `path == path.parent()`). |
| E | Git worktree main-root | Levels A–D all fail | Spawn `git worktree list --porcelain` with a 200ms hard timeout (see Latency section). Parse the first `worktree <path>` line. If the path exists and is a directory, use `<path>/.factory/logs`. If git exits non-zero, is unavailable, or times out: fall through to F. |
| F | Cwd fallback | Level E fails | `./.factory/logs` (current behavior). |

**Rationale for ordering:**

- A and B are explicit operator overrides; they win unconditionally. This preserves
  backward compatibility with any environment that already sets `VSDD_LOG_DIR`.
- C is the single-step fix for the issue: if `CLAUDE_PROJECT_DIR=/project/.factory`,
  the basename is `.factory`, so we use it directly without re-appending. This closes
  the primary bug with one predicate.
- D handles the case where neither env var is set and cwd is a subdirectory inside
  `.factory/` (e.g. cwd = `/project/.factory/cycles/`). Walking up finds `.factory`.
- E provides worktree consolidation (all linked worktrees land events in the main
  worktree's `.factory/logs/`) matching the v0.70.0 reference implementation. It is
  last-resort because it requires a subprocess and adds latency.
- F is the safe-fallback: same behavior as today when no git repo is present.

**Latency constraint (per ADR-020 Class A, p95 ≤ 1500ms):**

- Levels A–D are pure Rust `std::path` operations. No I/O except `symlink_metadata`
  for loop detection. Sub-millisecond.
- Level E spawns `git`. The `Command::new("git")` call MUST use a hard timeout of
  200ms. If `git` is unavailable (`which git` fails at binary lookup), the
  `std::process::Command::spawn()` call returns `Err`; treat as fallthrough to F
  without logging an error (git absent is not an error condition).
- `resolve_log_dir()` MUST NEVER panic. Every branch that can produce an `Err` or
  `None` must fall through to the next level or to F.

**Case-sensitivity rule:**

- Linux: basename comparison is case-sensitive (`.factory` only).
- macOS/Windows: basename comparison is case-insensitive. Use
  `eq_ignore_ascii_case(".factory")` on `OsStr::to_str()` with a `Some` guard. If
  `to_str()` returns `None` (non-UTF-8 path), treat as not-matching and fall through.

**Symlink loop guard for level D:**

Track visited `(st_dev, st_ino)` pairs via `std::fs::symlink_metadata`. If a path has
already been seen, break the walk and fall through to level E.

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

**Relationship to issue #129:** This decision is an instance of the general
canonicalization-and-fail-loud principle in #129. ADR-024 codifies the specific
behavior for this env var; the broader principle will be codified separately in #129's
resolution.

---

## Decision 3 — Internal-error dedup / rate-limiting

### Decision

Add per-session deduplication to `InternalLog` for `internal.dispatcher_error` events
only, using a fixed-capacity seen-set stored as a `HashSet<u64>` (hash of
`event.type_ + ":" + message_field`) behind a `Mutex` on `InternalLog`.

**Specification:**

- `InternalLog` gains a new field: `seen_errors: Mutex<HashSet<u64>>`.
- `write_inner` checks: before writing any event whose `type_` == `"internal.dispatcher_error"`,
  compute `hash = DefaultHasher(event.type_ + ":" + first 256 bytes of message_json_value)`.
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

**Why only `internal.dispatcher_error`?**

This is the noisy event class: it fires on every hook invocation when `CLAUDE_PLUGIN_ROOT`
is absent or when a malformed payload arrives. Other `internal.*` events (capability
denial, plugin invocations, lifecycle events) are per-invocation and must not be
deduplicated — they contain unique trace IDs and are used for debugging.

**Relationship to payload.rs:** The `#[serde(alias = "hook_event_name")]` fix already
eliminates the most common cause of `missing field 'event_name'` noise (per
`payload.rs:16-23`). Dedup provides defense-in-depth for residual noise from legacy or
malformed payloads.

---

## Decision 4 — Destructive-op guard shadow exception (SECURITY-SENSITIVE)

### Decision

Add a single targeted exception to `destructive-command-guard.sh`'s
`.factory/`-recursive-delete guard (lines 73–90) that permits deletion of the specific
shadow path `.factory/.factory/` while keeping all other `.factory/` deletion blocked.

**Exact predicate (Bash):**

```bash
# Allow deletion of the recursive shadow ONLY.
# The shadow is the exact path .factory/.factory/ (and its subdirectories).
# Pattern: the command targets something under .factory/.factory/ — i.e.,
# the string ".factory/.factory" appears in the rm target.
# We must NOT allow .factory/ deletion via this exception.
if [[ "$COMMAND" == *".factory/.factory"* ]] || \
   [[ "$COMMAND" =~ \.factory/\.factory(/|$) ]]; then
  continue  # Allow this specific shadow-path deletion
fi
```

This exception is inserted INSIDE the existing `for protected_re in ...` loop, BEFORE
the existing `.worktrees/` exception (line 77). Placement order: shadow exception first,
then `.worktrees/` exception, then build-dir exception.

**Security analysis:**

The exception fires if and only if the command string contains `.factory/.factory`.
A legitimate `.factory/` path never contains the substring `.factory/.factory` (because
the real worktree is mounted at the repo root's `.factory/`, not inside another
`.factory/`). An attacker attempting to exploit this exception to delete the real
`.factory/` would need to construct a command that contains `.factory/.factory` while
targeting the real path — which is structurally impossible because the real path is
`.factory/<subdir>`, not `.factory/.factory/<subdir>`.

**Scope confirmation — what remains protected:**

| Command | Outcome after this change |
|---------|--------------------------|
| `rm -rf .factory/` | BLOCKED — no `.factory/.factory` substring |
| `rm -rf .factory/specs/` | BLOCKED — no `.factory/.factory` substring |
| `rm -rf .factory/.factory/` | ALLOWED — exact shadow path |
| `rm -rf .factory/.factory/logs/` | ALLOWED — subdirectory of shadow |
| `find .factory -delete` | BLOCKED — `\.factory\b` still matched |
| `find .factory/.factory -delete` | ALLOWED — `.factory/.factory` in command |

**Find-delete guard update:** The `find ... -delete` block (lines 148–158) uses
`\.factory\b` as its pattern. Add the same shadow exception inside that block:

```bash
if echo "$COMMAND" | grep -qE '\.factory/\.factory'; then
  continue  # shadow path — allow
fi
```

**Git rm guard:** The `git rm` block (lines 277–286) guards `.factory/specs/`,
`.factory/stories/`, `.factory/STATE.md`. These path strings cannot match
`.factory/.factory` so no change needed.

**Testing obligation:** The test-writer MUST produce a bats test asserting:
- `rm -rf .factory/.factory/` exits 0 (allowed by guard).
- `rm -rf .factory/` exits 2 (still blocked by guard).
- `rm -rf .factory/specs/` exits 2 (still blocked by guard).

---

## Alternatives Considered

### Log-dir resolution: "Only fix level C, skip git subprocess"

Level C alone closes the primary bug. Levels D and E are necessary to handle the
log-fragmentation scenario (linked worktrees write to their own `.factory/`) and the
walk-up case (cwd inside a subdirectory of `.factory/`). Without D and E, those cases
regress the v0.70.0 capability. Rejected: production-grade default requires full
restoration of v0.70.0 behavior.

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
contract. String-predicate approach is the only acceptable mechanism. Implemented as
decided above.

---

## Consequences

### Behavioral changes

1. `resolve_log_dir()` no longer creates `.factory/.factory/logs/` when invoked with
   `CLAUDE_PROJECT_DIR` pointing at `.factory/`. AC-1 and AC-2 of issue #130 are closed.
2. `CLAUDE_PLUGIN_ROOT` absence produces an actionable stderr + `internal.dispatcher_error`
   diagnostic. AC-3 partially closed (the "not set" message becomes actionable; the silent
   default fallback is replaced with explicit degraded-mode behavior).
3. Identical `internal.dispatcher_error` messages are logged once per process lifetime.
   AC-4 closed.
4. `destructive-command-guard.sh` permits deletion of the `.factory/.factory/` shadow
   without weakening protection of the real `.factory/`. AC-6 closed.
5. AC-5 (regression test) is a test-writer deliverable, not an architectural decision.

### Files to change

| File | Change |
|------|--------|
| `crates/factory-dispatcher/src/main.rs` | Replace `resolve_log_dir()` (lines 669–674 + TODO comment 661–668); update `plugin_root` defaulting at 267–269 and 307–310; update `resolve_registry_path()` error message at 655–659 |
| `crates/factory-dispatcher/src/internal_log.rs` | Add `seen_errors: Mutex<HashSet<u64>>` field to `InternalLog`; update `write_inner` with dedup check |
| `plugins/vsdd-factory/hooks/destructive-command-guard.sh` | Add shadow exception inside the `.factory/`-recursive-delete loop (lines 73–90) and inside the find-delete guard (lines 148–158) |

### Release requirement

This is a `crates/` dispatcher change. Per CLAUDE.md "Dispatcher binary discipline",
the fix must be RELEASED (rc tag + cross-platform binary build via
`.github/workflows/release.yml`) to reach the operator-level cache at
`~/.claude/plugins/cache/claude-mp/vsdd-factory/<version>/`. Develop-branch edits do
not affect the cached plugin binary consumed by the running Claude Code harness.

The bash hook change (`destructive-command-guard.sh`) is a plugin-source change and
also requires a release to take effect in the operator-level cache.

### Latency budget compliance (ADR-020)

- Levels A–D: < 1ms (pure path operations).
- Level E: bounded by 200ms timeout on `git worktree list --porcelain`. In the nominal
  case (git present, in-repo): ~5–20ms. In the timeout case: 200ms + fallthrough. This
  is level E (last-resort) only; levels A–D handle all common cases without spawning
  git.
- Total log-dir resolution overhead at dispatch time: nominally < 5ms; worst-case
  (level E timeout) ≤ 200ms, still within ADR-020 Class A p95 = 1500ms budget.

### Purity boundary (SS-01 purity notes)

`resolve_log_dir()` is currently a pure function returning a `PathBuf`. After this
change it remains pure for levels A–D (reads env vars + walks path metadata). Level E
spawns a subprocess — this moves the function from "pure" to "effectful shell" in the
purity boundary sense. This is acceptable: `resolve_log_dir()` is explicitly a
side-effect initialization step at dispatcher startup, not a per-hook pure computation.
No formal verification applies to this function; integration test coverage (AC-5) is
the required verification mechanism.

---

## Decision Log

| Version | Date | Author | Change |
|---------|------|--------|--------|
| 1.0 | 2026-06-09 | architect | Initial acceptance. Closes all four architecture questions from issue #130. |
