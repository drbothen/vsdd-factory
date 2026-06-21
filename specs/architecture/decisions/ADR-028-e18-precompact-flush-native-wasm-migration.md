---
document_type: architecture-decision-record
level: L3
adr_id: ADR-028
version: "1.0"
status: accepted
producer: architect
timestamp: 2026-06-20T00:00:00Z
title: "ADR-028: E-18 precompact-flush native WASM migration — runtime worktree discovery and native lock renewal"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
anchors:
  - SS-04
  - SS-07
subsystems_affected:
  - SS-04
  - SS-07
supersedes: null
superseded_by: null
decision_status: accepted
human_gate_required: false
human_gate_reason: "Two explicit human decisions provided: (1) runtime git worktree discovery strategy; (2) native WASM lock renewal eliminating bash dependency. Both decisions are self-contained and implementable without further human input. All design work required for native renewal is resolved in this ADR."
last_amended: "2026-06-20 (v1.0) — Initial authoring (architect; human decisions incorporated: runtime git worktree discovery; native lock renewal binary_allow=[\"git\"] only; shared crate `crates/factory-lock` for reusable renewal logic; ordering confirmation in 11-step canonical order; BC-7.07.001/S-18.04a amendment instructions produced)."
---

# ADR-028: E-18 precompact-flush native WASM migration — runtime worktree discovery and native lock renewal

## Status

**ACCEPTED — 2026-06-20.**

This ADR codifies two human-directed decisions for the S-18.04a native WASM migration of the
`precompact-flush` hook plugin:

1. **Worktree path discovery** = runtime git discovery via `exec_subprocess("git", &["worktree", "list", "--porcelain"], ...)` — no hard-coded path, no registry config key.
2. **Lock renewal** = native Rust WASM (eliminate bash dependency). Plugin performs lock renewal natively: read STATE.md, parse `factory_lock:` block, update `expires_at`, write STATE.md back. Registry stanza `binary_allow = ["git"]` ONLY — no `bash`, no `shell_bypass_acknowledged`.

This ADR is the authoritative design reference for S-18.04a when the precompact-flush plugin is migrated from `legacy-bash-adapter.wasm` to a native WASM crate.

---

## Context

### Current state (bash-adapter era)

`precompact-flush.sh` is currently registered as a shell hook via `legacy-bash-adapter.wasm`:

```toml
[[hooks]]
name = "precompact-flush"
event = "PreCompact"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
priority = 100
timeout_ms = 30000
on_error = "continue"
async = false

[hooks.config]
script_path = "hooks/precompact-flush.sh"
```

The shell hook calls `bash plugins/vsdd-factory/bin/factory-lock-write.sh renew .factory/STATE.md`
for lock renewal (BC-7.07.001 Inv3 step 3; ADR-025 §Decision 11 Mechanism 1). This creates a
bash dependency in the hook's capability block — currently the hook declares `binary_allow =
["bash", "git"]` to support both the lock renewal call and git operations.

### Why native WASM migration

ADR-014 establishes the standing policy that all new hook plugins MUST be written as native WASM
crates rather than bash scripts routed through `legacy-bash-adapter.wasm`. The precompact-flush
plugin is in scope for migration to a native WASM crate in S-18.04a (or a follow-on story if the
scope is split).

### Two open design questions at migration time

The native WASM migration surfaces two design questions that the bash-adapter era deferred to the
shell's ambient capabilities:

**Q1 — Worktree path discovery**: How does the native WASM plugin discover the factory-artifacts
worktree path? The bash hook ran with the shell's ambient CWD (`.`); it could use relative paths
like `.factory/STATE.md`. In native WASM the plugin must explicitly discover this path.

**Q2 — Lock renewal without bash**: How does the plugin renew the factory lock without calling
`bash factory-lock-write.sh renew`? The bash hook delegated to the shell script. In native WASM
there is no bash, so the renewal must be implemented natively in Rust within the WASM sandbox.

Both questions have been resolved by explicit human direction (recorded as decisions below).

---

## Decision

### Decision 1 — Runtime git worktree discovery (worktree path)

The `precompact-flush` native WASM plugin discovers the factory-artifacts worktree mount path
**at runtime** by executing:

```
exec_subprocess("git", &["worktree", "list", "--porcelain"], ...)
```

and parsing the output for the factory-artifacts branch mount.

**Parsing algorithm:**

The `git worktree list --porcelain` output has the following format per worktree:

```
worktree /absolute/path/to/worktree
HEAD <sha>
branch refs/heads/<branch-name>

worktree /absolute/path/to/another
HEAD <sha>
detached
```

The plugin iterates stanzas, looking for the stanza whose `branch` line matches
`refs/heads/factory-artifacts`. The `worktree` line of that stanza is the factory-artifacts
worktree path. The plugin reads `<worktree-path>/STATE.md` for lock renewal and all factory-artifacts
filesystem operations.

**Rationale:**

- No hard-coded path: production mount (`.factory`) could theoretically differ between developer
  environments if the worktree was added with a custom path. Runtime discovery is always correct.
- No registry config key: reduces configuration surface; the git worktree list is the authoritative
  source of truth for worktree mounts and is always available when git is in scope.
- Consistent with ADR-024's approach: the dispatcher itself uses runtime resolution for log-dir
  and CLAUDE_PLUGIN_ROOT rather than hard-coded conventions.

**Failure mode:**

If `git worktree list --porcelain` produces no stanza with `branch refs/heads/factory-artifacts`:
- The plugin MUST exit 0 with advisory warning to stderr: `precompact-flush: factory-artifacts
  worktree not found via git worktree list; flush skipped.`
- This is consistent with BC-7.07.001 PC7 (exit 0 fail-open when STATE.md unreadable — a missing
  worktree is an equally unrecoverable startup condition).

**capability block:**

```toml
[hooks.capabilities.exec_subprocess]
binary_allow = ["git"]
env_allow = ["HOME", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME"]
```

(git is the only binary needed; see Decision 2 for lock renewal — bash is NOT needed.)

---

### Decision 2 — Native WASM lock renewal (eliminate bash dependency)

The `precompact-flush` native WASM plugin performs lock renewal **natively in Rust**, without
calling `bash factory-lock-write.sh renew`. The plugin:

1. Reads STATE.md via `host::read_file` (already needed for flush logic).
2. Parses the YAML frontmatter `factory_lock:` block.
3. If `factory_lock:` is **absent or `factory_lock.holder` is absent/null** → skip renewal
   entirely (no-op per ADR-025 opt-in model; consistent with BC-7.07.001 Inv3 step 2).
4. If lock IS held → update `expires_at` to `now + 2700s` (UTC ISO-8601), preserving `holder`
   and `locked_at` unchanged.
5. Writes the updated STATE.md back via `host::write_file` (or equivalent host function for
   in-place frontmatter update).
6. The renewed STATE.md is then committed by the flush (step 4 git add, step 5 git commit in
   BC-7.07.001 Inv3) — so the renewed `expires_at` is included in the flush commit automatically.

**Semantic faithfulness to `factory-lock-write.sh renew`:**

The native implementation MUST be semantically faithful to `factory-lock-write.sh renew`:

| Behavior | factory-lock-write.sh renew | Native WASM implementation |
|----------|----------------------------|---------------------------|
| Lock absent → behavior | Exit 0 no-op (`no factory_lock block present`) | Skip renewal (no-op), proceed to commit |
| Lock held → `holder` | Preserved unchanged | Preserved unchanged |
| Lock held → `locked_at` | Preserved unchanged | Preserved unchanged |
| Lock held → `expires_at` | Updated to `now + 2700s` (UTC ISO-8601) | Updated to `now + 2700s` (UTC ISO-8601) |
| Clock source | Single `date -u +%s` call → `now + 2700` | Single `SystemTime::now()` call → `now + 2700s` |
| `expires_at` sub-field absent in held block | Exit 1 `RenewalMissed` | Exit with advisory warn; proceed with commit (fail-open per BC-7.07.001 EC-004) |
| Post-renew assertion | Reads back file, asserts `expires_at` matches | Optional (native write is synchronous; trust the write; no re-read needed) |
| Frontmatter boundary awareness | awk-based: modifies ONLY inside the first `---` / `---` fence | Rust parser: modifies ONLY inside the YAML frontmatter block; body lines are preserved |

**TTL constant:** `TTL_SECONDS = 2700` (non-configurable per BC-5.40.001 Invariant 2).

**Error handling:**

Renewal failure is **non-fatal** (advisory) per BC-7.07.001 PC3 + EC-004: "Lock renewal failure
is treated as a non-fatal warning; the flush proceeds and attempts the commit regardless." The
native implementation mirrors this: if the frontmatter parse fails, the `factory_lock:` block is
malformed, or the write fails, the plugin emits an advisory warning to stderr and proceeds to the
commit step.

**registry stanza (corrected from bash-adapter era):**

```toml
[[hooks]]
name = "precompact-flush"
event = "PreCompact"
plugin = "hook-plugins/precompact-flush.wasm"
priority = 100
timeout_ms = 30000
on_error = "continue"
async = false

[hooks.capabilities.read_file]
path_allow = [".factory/"]

[hooks.capabilities.write_file]
path_allow = [".factory/"]

[hooks.capabilities.exec_subprocess]
binary_allow = ["git"]
env_allow = ["HOME", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME"]
```

**`binary_allow = ["git"]` ONLY** — no `"bash"`, no `shell_bypass_acknowledged`. This is the
binding constraint from the human decision.

---

### Decision 3 — Code structure: shared crate `crates/factory-lock` (library)

The native lock renewal logic MUST reside in a **new shared crate** `crates/factory-lock` (a
pure library crate — no `[[bin]]`, no WASM output):

```
crates/
  factory-lock/          ← NEW shared library crate
    Cargo.toml
    src/
      lib.rs             ← pub fn renew_lock(state_md: &str) -> Result<String, LockError>
                         ← pub fn acquire_lock(...) -> Result<String, LockError>
                         ← pub fn clear_lock(state_md: &str) -> Result<String, LockError>
                         ← pub struct FactoryLock { holder, locked_at, expires_at }
  hook-plugins/
    precompact-flush/    ← NEW native WASM plugin crate (has [[bin]])
      Cargo.toml         ← depends on crates/factory-lock
      src/
        main.rs
```

**Justification for shared crate (not inline in precompact-flush):**

1. **Non-trivial logic**: The lock renewal logic requires frontmatter-boundary-aware YAML parsing
   (awk's fence-awareness in factory-lock-write.sh is non-trivial: it tracks `fence` count to
   scope edits to the frontmatter only, preserving body lines). This logic is 50-100 lines of
   Rust and warrants isolation.
2. **Reusability**: `verify-factory-lock` (ADR-025 §Decision 12) already uses `crates/factory-lock-parse`
   for lock-parse operations. The `factory-lock` shared crate extends this to full CRUD (acquire,
   renew, clear), enabling future native WASM plugins (e.g., a native `verify-lock-renewal` guard)
   to share the same implementation without duplicating frontmatter surgery.
3. **Testability**: A standalone library crate is testable with standard `cargo test` without
   invoking the WASM runtime. The renewal logic can be property-tested (proptest / Hypothesis-style)
   against arbitrary frontmatter inputs.
4. **Separation from `factory-lock-parse`**: `crates/factory-lock-parse` (ADR-025 §D15) is a
   read-only parse crate. `crates/factory-lock` is the write-capable sibling. The naming
   distinction is preserved: `-parse` for read-only; no suffix for full CRUD.

**Relationship to existing `crates/factory-lock-parse`:**

`crates/factory-lock` SHOULD depend on `crates/factory-lock-parse` for the parse half of
`renew_lock` (read the existing `expires_at` and validate the block structure), then perform
the write half natively. This avoids duplicating the parse logic.

**`crates/factory-lock` is NOT a WASM output crate** — it has no `[[bin]]` and does not compile
to WASM directly. It is a workspace library crate linked into WASM plugin crates as a dependency.
The CI WASM floor-count gate counts `[[bin]]`-bearing crates only; adding `crates/factory-lock`
does NOT raise the floor count (consistent with ADR-025 §D15 `factory-lock-parse` precedent).

---

### Decision 4 — Canonical execution order: native renew precedes git add

In BC-7.07.001 Inv3, the canonical execution order for the flush is:

```
(1) read STATE.md
(2) check factory_lock: block — skip step 3 if absent/null
(3) if lock held: renew expires_at (native Rust in WASM — no bash)
(4) git add
(5) git commit (LOCAL)
(6) capture SHA_B immediately after commit
(7) attempt append to precompact-flush-log
(8) IF append fails: SHA-pinned concurrent-commit guard (reset or human-intervention)
(9) IF append succeeds: git push to remote factory-artifacts
(10) IF push fails: exit 2 with retry-is-push-only diagnostic
(11) IF push succeeds: exit 0
```

**Step 3 (native renew) MUST precede step 4 (git add)** so that the renewed `expires_at` is
included in the staged working tree and therefore in the flush commit. If renew happens AFTER
git add, the pre-renewal `expires_at` value is staged and committed, leaving the remote lock
with a stale expiry after the flush. This ordering is unchanged from the bash-era BC-7.07.001
Inv3; ADR-028 merely confirms it remains correct for the native implementation.

The flush commit (step 5) therefore always carries the current `expires_at` on the factory-artifacts
branch — consistent with BC-7.07.001 INV3 + BC-5.40.001 INV3 (expires_at = locked_at + TTL for
the initial acquisition; renewed expires_at = renew_instant + TTL for mid-burst renewal).

---

### Decision 5 — Standing policy: all new hooks are native WASM

Per ADR-014 (Tier-2 native WASM migration), all new hook plugins registered AFTER the ADR-014
acceptance date MUST be implemented as native WASM crates. `precompact-flush` (legacy-bash-adapter
era, registered pre-migration) is a migration candidate. This ADR formalizes that the migration
target is a native WASM crate `crates/hook-plugins/precompact-flush/` with `binary_allow = ["git"]`
only.

The bash helper `plugins/vsdd-factory/bin/factory-lock-write.sh` is **NOT deleted or deprecated**
by this ADR. It remains the canonical tool for agent/skill callers that invoke lock operations via
the Claude Code Bash tool (e.g., the `state-burst` skill renew step per ADR-025 §Decision 11
Mechanism 1). The native renewal in the WASM plugin is a parallel path — not a replacement — for
the specific case of the PreCompact hook firing inside the WASM sandbox.

---

## BC-6.23.001 and S-17.04 Amendment Assessment

**BC-6.23.001 (factory-lock skill acquire/release behaviors):** No amendment required. BC-6.23.001
governs the explicit `/factory-lock` and `/factory-unlock` skills and the `state-manager`
delegation pattern. The native WASM plugin's lock renewal is mechanism-internal to the
`precompact-flush` hook — it is not a new user-facing acquire or release. The skill-delegation
invariant (BC-6.23.001 Invariant 5: "Neither `/factory-lock` nor `/factory-unlock` writes
STATE.md directly — they delegate to `state-manager`") is about the explicit skill path, not
the hook path. The PreCompact hook is not a skill; it fires automatically. No contradiction.

**S-17.04 (verify-state-timestamp-refresh; BC-5.40.001 PC4 enforcement):** No amendment required.
S-17.04 delivers the `verify-state-timestamp-refresh` WASM guard and the `state-burst` SKILL
renew step. The native renew in `precompact-flush` is an ADDITIONAL renew path (hook fires on
PreCompact event), not a replacement for the `state-burst` SKILL step. S-17.04's mandate that
`factory-lock-write.sh renew` is called in the state-burst SKILL before `git add` remains
unchanged. ADR-028 adds a second caller (the native WASM `precompact-flush` plugin) for the
specific PreCompact firing scenario.

**BC-7.07.001 (precompact-flush.sh behavioral contract):** Amendment IS required. See §Downstream
Amendment Instructions below for exact changes needed.

---

## Downstream Amendment Instructions

### (a) Product-owner: BC-7.07.001 amendments

The following targeted amendments to BC-7.07.001 are required to reflect the native WASM
migration. These are PO-domain changes (spec content); architect produces the change spec here;
PO authors the actual BC amendment.

**Amendment 1 — Precondition 1: registry stanza update**

Replace the current Precondition 1 `[[hooks]]` block:
```toml
[[hooks]]
name = "precompact-flush"
event = "PreCompact"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
priority = 100
timeout_ms = 30000
on_error = "continue"
async = false

[hooks.config]
script_path = "hooks/precompact-flush.sh"
```

With the native WASM registry stanza:
```toml
[[hooks]]
name = "precompact-flush"
event = "PreCompact"
plugin = "hook-plugins/precompact-flush.wasm"
priority = 100
timeout_ms = 30000
on_error = "continue"
async = false

[hooks.capabilities.read_file]
path_allow = [".factory/"]

[hooks.capabilities.write_file]
path_allow = [".factory/"]

[hooks.capabilities.exec_subprocess]
binary_allow = ["git"]
env_allow = ["HOME", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME"]
```

Note: `binary_allow = ["git"]` ONLY. `"bash"` is explicitly absent.

**Amendment 2 — Precondition 4: lock renewal mechanism**

Replace:
> `factory-lock-write.sh` exists at `plugins/vsdd-factory/bin/factory-lock-write.sh` and
> supports `renew` subcommand (ADR-025 D11 Mechanism 1 deliverable; S-17.04 dependency).
> Canonical invocation form: `bash plugins/vsdd-factory/bin/factory-lock-write.sh renew <STATE.md-path>`.

With:
> The native WASM plugin performs lock renewal natively in Rust via `crates/factory-lock`
> (a shared library crate). Lock renewal does NOT invoke `bash factory-lock-write.sh renew`.
> The bash helper `plugins/vsdd-factory/bin/factory-lock-write.sh` remains available for agent/skill
> callers (e.g., the `state-burst` skill per ADR-025 §Decision 11 Mechanism 1) but is NOT a
> dependency of the native WASM plugin. Factory-artifacts worktree path is discovered via runtime
> `git worktree list --porcelain` (ADR-028 §Decision 1).

**Amendment 3 — Postcondition 3 (PC3): native renewal mechanism**

Replace:
> If the lock IS held, the hook invokes `bash plugins/vsdd-factory/bin/factory-lock-write.sh renew .factory/STATE.md` (ADR-025 D11 Mechanism 1) before any git commit.

With:
> If the lock IS held, the hook natively updates `factory_lock.expires_at = now + 2700s` (UTC ISO-8601)
> in STATE.md via `crates/factory-lock::renew_lock()` — a pure Rust in-process update with no
> exec_subprocess call for renewal. The `holder` and `locked_at` fields are preserved unchanged.
> Lock renewal failure is non-fatal (advisory warning; flush proceeds per EC-004).

**Amendment 4 — Invariant 3 (INV3), step 3: native renewal mechanism**

Replace step 3:
> (3) if lock held: invoke `bash plugins/vsdd-factory/bin/factory-lock-write.sh renew .factory/STATE.md`

With:
> (3) if lock held: invoke `crates/factory-lock::renew_lock(&state_md_path)` (native Rust;
> no bash exec; updates `expires_at = now + 2700s` in STATE.md; preserves `holder` and `locked_at`)

**Amendment 5 — Invariant 6 (INV6): script hardening clause**

Replace (if present):
> The script uses `set -euo pipefail` throughout.

With:
> The plugin uses idiomatic Rust error propagation (`Result<T, E>`) throughout. Unhandled errors
> propagate via `?` operator; panics are treated as crashes triggering `on_error = "continue"`
> fail-open per Postcondition 9.

**Amendment 6 — Architecture Anchors: crate reference**

Replace the reference to `plugins/vsdd-factory/hooks/precompact-flush.sh` in §Architecture Anchors:

```
- `crates/hook-plugins/precompact-flush/` — NEW native WASM plugin crate (S-18.04a deliverable); replaces `hooks/precompact-flush.sh` + legacy-bash-adapter registration
- `crates/factory-lock/` — shared library crate providing `renew_lock()`, `acquire_lock()`, `clear_lock()` used by the WASM plugin for native lock renewal without bash
- `plugins/vsdd-factory/bin/factory-lock-write.sh` — retained for agent/skill callers (NOT a plugin dependency post-migration)
- `plugins/vsdd-factory/hooks-registry.toml` — `[[hooks]] event = "PreCompact"` entry updated to native WASM stanza (ADR-028 §Decision 2 corrected stanza)
```

---

### (b) Story-writer: S-18.04a amendments

The following targeted amendments to S-18.04a are required to reflect the native WASM migration.
These are story-writer-domain changes; architect produces the change spec here; story-writer
authors the actual story amendment.

**Amendment 1 — `target_module`: bash hook → native WASM crate**

Replace:
```yaml
target_module: plugins/vsdd-factory/hooks/precompact-flush.sh
```
With:
```yaml
target_module: crates/hook-plugins/precompact-flush/src/main.rs
```
And add:
```yaml
target_module_secondary:
  - crates/factory-lock/src/lib.rs
```

**Amendment 2 — File list: reflect Rust crate deliverables**

Replace the story's file list (if enumerated) from:
- `plugins/vsdd-factory/hooks/precompact-flush.sh`
- `plugins/vsdd-factory/hooks-registry.toml` (stanza update)

With:
- `crates/hook-plugins/precompact-flush/Cargo.toml` — new WASM plugin crate
- `crates/hook-plugins/precompact-flush/src/main.rs` — native WASM hook entry point
- `crates/factory-lock/Cargo.toml` — new shared library crate
- `crates/factory-lock/src/lib.rs` — `renew_lock()`, `acquire_lock()`, `clear_lock()`
- `plugins/vsdd-factory/hooks-registry.toml` — updated `[[hooks]]` stanza (native WASM; `binary_allow = ["git"]`)
- `Cargo.toml` (workspace) — new crates added to `[workspace.members]`

**Amendment 3 — Runtime worktree discovery: AC for git worktree list parsing**

Add an AC covering the runtime `git worktree list --porcelain` discovery:

> AC-NNN (traces to ADR-028 §Decision 1): The native WASM plugin discovers the factory-artifacts
> worktree mount path at runtime by executing `git worktree list --porcelain` and parsing the
> stanza whose `branch` line is `refs/heads/factory-artifacts`. If no such stanza is found, the
> plugin exits 0 with an advisory warning (fail-open) and skips the flush.

**Amendment 4 — Native renewal: AC for Rust in-process renewal**

Add an AC covering the native lock renewal:

> AC-NNN (traces to ADR-028 §Decision 2 + BC-7.07.001 PC3): The native WASM plugin renews
> `factory_lock.expires_at = now + 2700s` natively in Rust via `crates/factory-lock::renew_lock()`
> — no `exec_subprocess("bash", ...)` call for renewal. The `binary_allow` registry stanza contains
> only `["git"]`. The renewal is non-fatal: if the lock block is absent or malformed, the plugin
> emits an advisory warning and proceeds to the commit step.

**Amendment 5 — Red Gate: add cargo unit test for native renewal**

Add to the Red Gate test table:

| Red Gate test | Expected failure mode (pre-implementation) |
|---------------|-------------------------------------------|
| `test_renew_lock_updates_expires_at_only()` — call `crates/factory-lock::renew_lock()` with a held lock STATE.md; assert `expires_at` updated to `now + 2700s`, `holder` and `locked_at` unchanged | Compile error (crate not yet implemented) |
| `test_renew_lock_noop_when_absent()` — call `renew_lock()` with STATE.md having no `factory_lock:` block; assert no-op (content unchanged, exit Ok) | Compile error |
| `test_worktree_discovery_parses_factory_artifacts_branch()` — call worktree discovery with synthetic `git worktree list --porcelain` output; assert returned path matches the `factory-artifacts` stanza's `worktree` line | Compile error |
| bats: `precompact-flush.wasm` invoked via factory-dispatcher with `factory_lock:` held in STATE.md; assert `expires_at` in committed STATE.md is `>= test_start + 2700s`; assert no bash subprocess in dispatcher log | Hook not yet compiled |

---

## Consequences

### Positive

- **Bash dependency eliminated from PreCompact hook.** The `precompact-flush` plugin's `binary_allow`
  reduces from `["bash", "git"]` to `["git"]` only. This removes one surface of the shell-exec
  attack footprint in the WASM sandbox.
- **Runtime worktree discovery is always correct.** No hard-coded `.factory` assumption; the
  actual mount path is read from git's authoritative worktree registry at runtime.
- **Native renewal fidelity.** The Rust implementation of `renew_lock()` mirrors `factory-lock-write.sh
  renew` semantics precisely: preserve `holder` + `locked_at`; update `expires_at = now + 2700s`;
  no-op when lock absent; fail-open (advisory) on malformed block.
- **Shared crate reusable.** `crates/factory-lock` can be used by future native WASM plugins
  needing lock operations (e.g., a native `verify-lock-renewal` guard as an upgrade to ADR-025
  §Decision 11 Mechanism 2).
- **Standing ADR-014 policy satisfied.** Native WASM migration closes the legacy-bash-adapter
  technical debt for the PreCompact hook.

### Negative / Trade-offs

- **New crate `crates/factory-lock` must be authored.** This is non-trivial: frontmatter-boundary-aware
  YAML surgery in Rust requires careful parsing of the `---` fence delimiters, analogous to
  factory-lock-write.sh's awk logic. Incorrect parsing could corrupt STATE.md frontmatter.
  Mitigation: unit tests over the full matrix of frontmatter shapes (held lock, absent lock,
  malformed block, body-content `factory_lock:` lines that must not be modified).
- **`git worktree list --porcelain` adds a startup subprocess.** This is one additional `git`
  exec at plugin initialization. At 30s timeout budget this is negligible. Mitigation: cache
  the result for the lifetime of a single plugin invocation (no re-discovery mid-flush needed).
- **`bash factory-lock-write.sh renew` remains in production for skill callers.** Two code paths
  now implement renewal (bash script + Rust crate). They must remain semantically synchronized.
  Mitigation: the Rust `renew_lock()` implementation's unit tests use the same semantic table as
  factory-lock-write.sh's renew logic (verified above); any future change to the TTL constant
  (BC-5.40.001 Invariant 2) must update both.

---

## Risks Addressed

| Risk | Mitigation |
|------|-----------|
| Hard-coded `.factory` path breaks in non-standard worktree mounts | Decision 1: runtime `git worktree list --porcelain` discovery; no path assumption |
| bash exec in WASM sandbox expands attack surface | Decision 2: `binary_allow = ["git"]` only; renewal is native Rust |
| Native renew diverges semantically from bash script renew | Decision 3 table: semantic faithfulness requirements; unit tests validate same input→output contract |
| Rust frontmatter surgery corrupts body content of STATE.md | Shared crate must be frontmatter-boundary-aware; body lines after second `---` fence must pass through unchanged; unit tests include body-content edge cases |
| `expires_at` not included in flush commit (ordering defect) | Decision 4: native renew (step 3) MUST precede git add (step 4) per BC-7.07.001 Inv3 |
| New crate under `crates/hook-plugins/` inflates CI WASM floor count unexpectedly | `crates/factory-lock` has no `[[bin]]` → not counted; `crates/hook-plugins/precompact-flush/` has `[[bin]]` → floor count +1 (intentional: replaces the bash hook with a native WASM plugin) |

---

## Feasibility Risks

| Risk | Severity | Assessment |
|------|----------|-----------|
| YAML frontmatter surgery in Rust — incorrect fence detection corrupts STATE.md | HIGH | Mitigation: unit test over ≥20 frontmatter shapes; property-test with arbitrary body content; the fence pattern (`^---$` only) is well-defined |
| `git worktree list --porcelain` format changes across git versions | LOW | Format is stable since git 2.5 (2015); macOS ships git ≥ 2.30; Linux CI ships git ≥ 2.34; no versioning risk in practice |
| Native `renew_lock()` fidelity drift from bash script | MEDIUM | Mitigation: semantic faithfulness table in this ADR + unit tests; ADR-028 is the canonical spec; any divergence found during TDD is a RED gate failure, not a spec amendment |
| Startup cost of `git worktree list` per PreCompact event | NEGLIGIBLE | One subprocess; sub-100ms; 30s timeout budget absorbs it |
| `crates/factory-lock` dependency cycle (if `crates/factory-lock-parse` imports it back) | LOW | Keep dependency unidirectional: `factory-lock` depends on `factory-lock-parse`; `factory-lock-parse` has no dependency on `factory-lock` |

---

## Traceability

| Source | ID | Coverage |
|--------|----|---------|
| ADR | ADR-014 | Standing policy: all new hooks native WASM |
| ADR | ADR-025 §Decision 11 | Lock renewal mechanism; `factory-lock-write.sh renew` as Mechanism 1 for skill callers |
| ADR | ADR-026 §Decision 6 | PreCompact flush canonical execution order (commit→append→push) |
| ADR | ADR-027 §Decision 4 | S-18.04a path convention (`FACTORY_ARTIFACTS_PATH`) |
| Behavioral Contract | BC-7.07.001 v1.14 | precompact-flush.sh full behavior spec; Inv3 canonical order; PC3 lock renewal; EC-004 fail-open |
| Behavioral Contract | BC-5.40.001 | factory_lock schema; TTL invariant 2 (2700s non-configurable); expires_at invariant 3 |
| Behavioral Contract | BC-6.23.001 | Explicit acquire/release skills; mechanism-internal renewal does not conflict |
| Story | S-18.04a v1.7 | Implementing story; amendments required per §Downstream Amendment Instructions |
| Story | S-17.04 v1.6 | Delivers `factory-lock-write.sh renew` and `verify-state-timestamp-refresh`; ADR-028 is additive, not conflicting |
| Capability | CAP-032 | Context-durability feature; E-18 |
| Shell script | `plugins/vsdd-factory/bin/factory-lock-write.sh` | Canonical renew semantics source; native implementation must be faithful |
| Subsystem | SS-04 | Plugin Ecosystem (WASM plugin crate; shared library crate) |
| Subsystem | SS-07 | Hook Bash Layer (registry stanza; migration from bash hook) |
| Feature | E-18 / issue #173 | Context-durability epic |
