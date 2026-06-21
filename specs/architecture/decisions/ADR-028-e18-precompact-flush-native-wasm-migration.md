---
document_type: architecture-decision-record
level: L3
adr_id: ADR-028
version: "1.1"
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
last_amended: "2026-06-20 (v1.1) — Adversarial review remediation (architect): F-NW-001 [BLOCKER] PATH+SSH_AUTH_SOCK added to env_allow; GIT_SSH_COMMAND assessed; Red Gate added for real git push to local bare remote. F-NW-002 [BLOCKER] path_allow domain vs worktree discovery clarified: discovered absolute path used ONLY for git -C argument; host read_file/write_file always use .factory/-relative paths gated by path_allow; canonical invariant documented (discovered-worktree == <cwd>/.factory). F-NW-003 [BLOCKER] uniform git -C mandate: ALL git subprocesses must use git -C <discovered-worktree-abs-path>; bare git commands without -C are forbidden; canonical 11-step execution order updated. F-NW-005 [MAJOR] parse-result→renew-outcome mapping fully specified: Ok(None) → no-op; Ok(Some valid) → update expires_at; Err(malformed) → advisory warn + proceed; no-frontmatter STATE.md → Ok(None) (skip, no advisory); Red Gate added. F-NW-004 [MAJOR] renew fidelity divergence resolved: renew_lock() returns Err on malformed (library faithfulness preserved); precompact-flush CALLER downgrades Err to advisory + proceeds (hook fail-open preserved); contradiction in semantic-faithfulness table removed. F-NW-007 [MAJOR] log-append concurrency assessed: write_file is full-overwrite (read-modify-write, NOT OS-atomic append); single-writer guarantee documented (dispatcher synchronous + factory-lock gate); bounded data-loss window stated; NO dispatcher host-API change required (grounded-documentation path). F-NW-008 [MAJOR] expires_at format pinned: EXACTLY YYYY-MM-DDTHH:MM:SSZ (UTC, second precision, Z suffix — NOT chrono default rfc3339 +00:00 / sub-seconds). F-NW-009 [MINOR] CRLF + file mode: native crates/factory-lock normalizes to LF-only output; file mode not applicable (host write_file creates/overwrites with platform defaults; no chmod). ADR traceability stale cites replaced with stable anchors per POLICY 19 (TD-VSDD-091). ADR-028 v1.0→v1.1. [Prior: 2026-06-20 (v1.0) — Initial authoring (architect; human decisions incorporated: runtime git worktree discovery; native lock renewal binary_allow=[\"git\"] only; shared crate `crates/factory-lock` for reusable renewal logic; ordering confirmation in 11-step canonical order; BC-7.07.001/S-18.04a amendment instructions produced).]"
---

# ADR-028: E-18 precompact-flush native WASM migration — runtime worktree discovery and native lock renewal

## Status

**ACCEPTED — 2026-06-20 (v1.0). Amended 2026-06-20 (v1.1) — adversarial review remediation.**

This ADR codifies two human-directed decisions for the S-18.04a native WASM migration of the
`precompact-flush` hook plugin:

1. **Worktree path discovery** = runtime git discovery via `exec_subprocess("git", &["worktree", "list", "--porcelain"], ...)` — no hard-coded path, no registry config key.
2. **Lock renewal** = native Rust WASM (eliminate bash dependency). Plugin performs lock renewal natively: read STATE.md, parse `factory_lock:` block, update `expires_at`, write STATE.md back. Registry stanza `binary_allow = ["git"]` ONLY — no `bash`, no `shell_bypass_acknowledged`.

This ADR is the authoritative design reference for S-18.04a when the precompact-flush plugin is migrated from `legacy-bash-adapter.wasm` to a native WASM crate.

---

## Changelog

| Version | Date | Change |
|---------|------|--------|
| v1.1 | 2026-06-20 | Adversarial review remediation: F-NW-001 env_allow PATH+SSH_AUTH_SOCK; F-NW-002 path_allow domain clarification + canonical .factory/ mount invariant; F-NW-003 uniform git -C mandate; F-NW-004 renew_lock() Err semantics + caller-downgrade; F-NW-005 parse-result→renew-outcome mapping; F-NW-007 log-append concurrency grounded-documentation; F-NW-008 expires_at format exact contract; F-NW-009 CRLF + file mode native contract. Stale traceability version tokens replaced with stable POLICY 19 anchors. Amendment instructions for PO/story-writer/state-manager produced. |
| v1.0 | 2026-06-20 | Initial authoring (architect; two human decisions: runtime worktree discovery + native lock renewal). |

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
for lock renewal (BC-7.07.001 §Inv3 step 3; ADR-025 §Decision 11 Mechanism 1). This creates a
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
worktree path.

**Critical domain distinction (F-NW-002):**

The discovered absolute worktree path serves TWO distinct purposes with TWO distinct domains:

- **Domain A — Host I/O operations (`read_file` / `write_file`)**: These are always performed via
  `.factory/`-relative paths gated by `path_allow = [".factory/"]`. The plugin uses paths like
  `.factory/STATE.md` and `.factory/hooks/precompact-flush-log` — relative to the dispatcher's
  `cwd` (which is `CLAUDE_PROJECT_DIR`, i.e., the main repo root).

- **Domain B — Git subprocess argument only**: The discovered absolute worktree path is used
  EXCLUSIVELY as the `git -C <discovered-worktree-abs-path>` argument for every git subprocess
  (see Decision 3). It is NEVER passed to `read_file` or `write_file` as a path prefix.

**Canonical invariant:** The factory-artifacts worktree is mounted at `<CLAUDE_PROJECT_DIR>/.factory`
by project convention (see CLAUDE.md "mounted at `.factory/` via worktree"). Therefore:
`discovered-worktree-abs-path == resolve(<CLAUDE_PROJECT_DIR>/.factory)`.

This invariant means Domain A paths (relative `.factory/...`) and Domain B paths (absolute
`<cwd>/.factory/...`) resolve to the same files. If a non-standard mount is ever used, the
implementation MUST derive `path_allow` from the discovered path rather than relying on the
`.factory/` convention. The production recommendation is to always mount at `.factory/` and
assert the invariant at plugin startup.

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
- This is consistent with BC-7.07.001 §PC7 (exit 0 fail-open when STATE.md unreadable — a missing
  worktree is an equally unrecoverable startup condition).

**Capability block (v1.1 — F-NW-001 corrected):**

```toml
[hooks.capabilities.exec_subprocess]
binary_allow = ["git"]
env_allow = ["HOME", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME", "PATH", "SSH_AUTH_SOCK"]
```

**F-NW-001 rationale:** The host `exec_subprocess` runs with `env_clear` followed by re-injection
of only the names listed in `env_allow` (see `crates/factory-dispatcher/src/host/exec_subprocess.rs`
`execute_bounded()` lines 242–247). The original `env_allow` omitted `PATH` and `SSH_AUTH_SOCK`:

- **`PATH`**: Required by `git` to locate `ssh`, credential helpers, and other git-invoked
  subprocesses. Without `PATH`, git cannot find the SSH binary needed for `git push` over SSH.
  Git operations that do not involve external programs (e.g., `git add`, `git commit`,
  `git rev-parse`, `git reset`) may succeed without `PATH` depending on the git binary's
  compile-time defaults, but `git push` reliably fails without SSH reachability.
- **`SSH_AUTH_SOCK`**: Required to reach the SSH agent (socket path). Without this variable,
  `ssh` cannot find the agent to retrieve loaded private keys, causing authentication failure
  on `git push` over SSH.
- **`GIT_SSH_COMMAND`**: Optional. If the operator has configured a custom SSH invocation
  (e.g., `GIT_SSH_COMMAND="ssh -i /path/to/key"`), omitting it causes git to fall back to
  the default `ssh` binary. This is acceptable for the common case. The risk is low; do not
  add `GIT_SSH_COMMAND` to `env_allow` by default. Operators needing a custom SSH command
  can extend `env_allow` in their registry override.

**Red Gate requirement (F-NW-001):** A bats integration test MUST exercise a real `git push`
to a LOCAL BARE REMOTE (created by `git init --bare` in the test fixture) using the native
precompact-flush.wasm plugin running through the factory-dispatcher with the above `env_allow`.
The test asserts: (a) `git push` exits 0, (b) the bare remote has received the commit. This
proves `PATH` + `SSH_AUTH_SOCK` in `env_allow` is sufficient for git push without requiring
external network or credentials. See §Downstream Amendment Instructions (b) AC addition.

---

### Decision 2 — Native WASM lock renewal (eliminate bash dependency)

The `precompact-flush` native WASM plugin performs lock renewal **natively in Rust**, without
calling `bash factory-lock-write.sh renew`. The plugin:

1. Reads STATE.md via `host::read_file` (already needed for flush logic).
2. Parses the YAML frontmatter `factory_lock:` block via `crates/factory-lock-parse`.
3. If `factory_lock:` is **absent or `factory_lock.holder` is absent/null** → skip renewal
   entirely (no-op per ADR-025 §Decision 11 opt-in model; consistent with BC-7.07.001 §Inv3 step 2).
4. If lock IS held → update `expires_at` to `now + 2700s` (UTC, EXACTLY `YYYY-MM-DDTHH:MM:SSZ`
   format — see F-NW-008 below), preserving `holder` and `locked_at` unchanged.
5. Writes the updated STATE.md back via `host::write_file`.
6. The renewed STATE.md is then committed by the flush (step 4 git add, step 5 git commit in
   BC-7.07.001 §Inv3) — so the renewed `expires_at` is included in the flush commit automatically.

**`expires_at` format contract (F-NW-008):**

The native implementation MUST output `expires_at` in EXACTLY this format:
`YYYY-MM-DDTHH:MM:SSZ` — UTC, second precision, uppercase `Z` suffix.

This matches `factory-lock-write.sh` (which uses `date -u +%Y-%m-%dT%H:%M:%SZ`) and
BC-5.40.001 §Invariant 2. It does NOT match `chrono`'s default `to_rfc3339()` output,
which emits `+00:00` offset notation and may include sub-second precision. The implementation
MUST use `chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()` (or equivalent),
NOT `to_rfc3339()`. Any deviation from the `Z`-suffix form or inclusion of sub-seconds is
a spec violation and a failing Red Gate test.

**CRLF and file mode contract (F-NW-009):**

`host::write_file` calls `std::fs::write` (full overwrite). The native `renew_lock()` function:
- MUST output LF-only line endings (`\n`). If the input STATE.md contains CRLF, the CRLF
  normalization performed by `crates/factory-lock-parse` (which calls `content.replace("\r\n", "\n")`)
  means the parsed+rewritten content will be LF-only. This is intentional: LF normalization is
  a documented benefit (consistent with `factory-lock-write.sh`'s `_normalize_crlf` helper).
- File mode: `std::fs::write` creates/overwrites with the process umask as the effective mode.
  There is NO `chmod` step (unlike the bash script's `chmod "$orig_mode" "$tmpfile"`). For
  STATE.md in the factory-artifacts worktree, the file mode is irrelevant (git tracks content,
  not mode for non-executable files). This is an accepted behavioral delta from the bash script.

**Parse-result → renew-outcome mapping (F-NW-005):**

`crates/factory-lock-parse::parse_factory_lock()` returns:
- `Ok(None)` — factory_lock absent, null, or empty block → **skip renewal (no-op, proceed
  to git add)**. No advisory warning. This covers both "no frontmatter" (no opening `---`) and
  "frontmatter present but no factory_lock key".
- `Ok(Some(LockState))` — valid lock block, all three sub-fields present → **update `expires_at`
  to `now + 2700s`, write back, proceed to git add**.
- `Err(MalformedLockBlock)` — block present but malformed (missing field, empty field, missing
  closing fence, unexpected inline value) → **emit advisory warning to stderr + proceed to git
  add (fail-open per BC-7.07.001 §PC3 / §EC-004)**. This includes the case where `expires_at`
  is missing from a held block — the bash script exits 1 on this case (`RenewalMissed`), but
  the native implementation treats it as advisory-fail-open at the hook layer.

The `Ok(None)` mapping for "no frontmatter" STATE.md is production-grade default: a STATE.md
without frontmatter is not a locked file; skip is correct. No advisory warning is warranted
(this would spam on every no-lock flush). The Err→advisory mapping is the authoritative
divergence from the bash `RenewalMissed` exit-1 behavior — see F-NW-004 below.

**Semantic faithfulness to `factory-lock-write.sh renew` (F-NW-004 corrected):**

The bash `renew` mode exits 1 on `RenewalMissed` (block present but missing `expires_at`).
The native `crates/factory-lock::renew_lock()` returns `Err(LockError::Malformed)` on the
same input (preserving bash semantic faithfulness at the library layer). The divergence occurs
at the CALLER layer — the `precompact-flush` hook CALLER downgrades `Err` to advisory warning
and proceeds, whereas the bash script exits 1. This is an intentional divergence at the hook
layer per BC-7.07.001 §EC-004 fail-open semantics. The library itself does NOT swallow the
error; the hook caller is the authoritative fail-open policy point.

| Behavior | `factory-lock-write.sh renew` | `renew_lock()` (library) | `precompact-flush` caller |
|----------|-------------------------------|--------------------------|--------------------------|
| Lock absent → behavior | Exit 0 no-op | `Ok(None)` | skip renewal; proceed |
| Lock held → `holder` | Preserved | Preserved | — |
| Lock held → `locked_at` | Preserved | Preserved | — |
| Lock held → `expires_at` | Updated `now + 2700s` Z-suffix | Updated `now + 2700s` Z-suffix | — |
| Clock source | Single `date -u +%s` call | Single `SystemTime::now()` | — |
| `expires_at` absent in held block | Exit 1 `RenewalMissed` | `Err(Malformed)` | advisory warn; proceed (fail-open) |
| Malformed block (other errors) | Exit 1 `RenewalMissed` | `Err(Malformed)` | advisory warn; proceed (fail-open) |
| No frontmatter at all | Exit 0 no-op (awk finds no `factory_lock:`) | `Ok(None)` | skip renewal; proceed |
| Post-renew assertion | Read back + assert match | Not performed (write is synchronous; trust the write) | — |
| Frontmatter boundary | awk-based fence tracking | Rust fence-aware parser | — |
| Line endings output | LF-only (normalized by `_normalize_crlf`) | LF-only (chrono/string output; no CRLF) | — |
| File mode | Preserved via `chmod "$orig_mode"` | Not applicable (host write_file umask-default) | — |

**registry stanza (v1.1 — F-NW-001 corrected):**

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
env_allow = ["HOME", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME", "PATH", "SSH_AUTH_SOCK"]
```

**`binary_allow = ["git"]` ONLY** — no `"bash"`, no `shell_bypass_acknowledged`. This is the
binding constraint from the human decision.

---

### Decision 3 — Uniform `git -C <worktree>` on every git subprocess (F-NW-003)

The dispatcher sets `child_process.cwd = CLAUDE_PROJECT_DIR` (the MAIN REPO root, not the
factory-artifacts worktree root) — see `crates/factory-dispatcher/src/main.rs` line ~303
(`base_host_ctx.cwd = std::env::var(ENV_PROJECT_DIR)...`). The factory-artifacts worktree is
a separate directory (`<CLAUDE_PROJECT_DIR>/.factory`).

Therefore: **ALL git subprocesses in the plugin MUST use `git -C <discovered-worktree-abs-path>`**
to target the factory-artifacts worktree. Bare `git add`, `git commit`, `git push`, `git rev-parse`,
`git reset`, `git diff --cached`, `git status` commands WITHOUT `-C` would operate on the MAIN
REPO (develop branch), not the factory-artifacts branch. This would:
- Commit to the wrong branch (`develop` instead of `factory-artifacts`)
- Corrupt the SHA guard (SHA_B from `rev-parse HEAD` on the main repo, not factory-artifacts)
- Break the atomic log guard (reset on wrong branch)

**Canonical git subprocess calls (uniform -C form):**

| Operation | Correct form |
|-----------|-------------|
| Discover worktree | `git worktree list --porcelain` (no `-C` needed — runs from main repo) |
| Stage changes | `git -C <wt> add -u` |
| Commit | `git -C <wt> commit -m <msg>` |
| Capture SHA_B | `git -C <wt> rev-parse HEAD` |
| Check current HEAD | `git -C <wt> rev-parse HEAD` |
| SHA-pinned reset | `git -C <wt> reset --soft <sha_b>^` |
| Push | `git -C <wt> push origin factory-artifacts` |
| Check for changes | `git -C <wt> diff --cached` (or `git -C <wt> status --porcelain`) |

Where `<wt>` = the absolute path discovered in Decision 1.

**The ONLY git command that does NOT use `-C`** is the initial `git worktree list --porcelain`,
because that command is intentionally run against the main repo to discover the worktree path.
All subsequent git commands MUST use `-C`.

---

### Decision 4 (formerly Decision 3) — Code structure: shared crate `crates/factory-lock` (library)

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

**`renew_lock()` signature and error semantics (F-NW-004):**

```rust
pub fn renew_lock(state_md_content: &str) -> Result<String, LockError>
```

- Returns `Ok(new_content)` when renewal succeeded or was a no-op (lock absent → content unchanged).
- Returns `Err(LockError::Malformed(...))` when the block is present but malformed (missing
  field, missing closing fence, unexpected inline value). Callers decide whether to treat `Err`
  as fatal or advisory. The `precompact-flush` hook treats it as advisory (proceed with flush).
- Does NOT return `Ok(content)` for malformed blocks — the library preserves bash semantic
  faithfulness by signaling the error to the caller.

Alternatively, the signature may distinguish no-op from write:
```rust
pub enum RenewOutcome { NoOp, Renewed(String) }
pub fn renew_lock(state_md_content: &str) -> Result<RenewOutcome, LockError>
```

Either form is acceptable; the key constraint is that `Err` is returned for malformed blocks
so the caller can emit an advisory.

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

**`crates/factory-lock` is NOT a WASM output crate** — it has no `[[bin]]` and does not compile
to WASM directly. It is a workspace library crate linked into WASM plugin crates as a dependency.

---

### Decision 5 (formerly Decision 4) — Canonical execution order: native renew precedes git add

In BC-7.07.001 §Inv3, the canonical execution order for the flush is:

```
(1)  discover factory-artifacts worktree abs-path via git worktree list --porcelain
(2)  read STATE.md via host::read_file (path_allow domain: .factory/STATE.md relative)
(3)  check factory_lock: block via crates/factory-lock-parse::parse_factory_lock()
     — Ok(None) → skip step 4 (no-op)
     — Ok(Some) → proceed to step 4
     — Err(malformed) → advisory warn to stderr; skip step 4; proceed to step 5
(4)  if lock held: crates/factory-lock::renew_lock() → write updated STATE.md via
     host::write_file (path_allow domain: .factory/STATE.md relative);
     format EXACTLY YYYY-MM-DDTHH:MM:SSZ; if renew_lock() returns Err: advisory warn, proceed
(5)  git -C <wt> add -u
(6)  git -C <wt> commit -m "PreCompact flush <cycle>/<step> <ISO-timestamp>" (LOCAL)
(7)  SHA_B = git -C <wt> rev-parse HEAD (IMMEDIATELY after commit; MUST precede append)
(8)  append to .factory/hooks/precompact-flush-log via host::read_file + concat + host::write_file
     (read-modify-write; LF-terminated; see F-NW-007 below)
(9)  IF append fails: CURRENT_HEAD = git -C <wt> rev-parse HEAD
     — if CURRENT_HEAD == SHA_B: git -C <wt> reset --soft SHA_B^; exit 2
     — if CURRENT_HEAD != SHA_B: exit 2 with human-intervention message (do NOT reset)
     — if reset fails: exit 2 human-intervention
(10) IF append succeeds: git -C <wt> push origin factory-artifacts
(11) IF push fails: exit 2 with retry-is-push-only diagnostic
     IF push succeeds: exit 0
```

**Step 4 (native renew) MUST precede step 5 (git add)** so that the renewed `expires_at` is
included in the staged working tree and therefore in the flush commit.

---

### Decision 6 (formerly Decision 5) — Standing policy: all new hooks are native WASM

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

### Decision 7 (new) — Log-append concurrency: grounded-documentation path (F-NW-007)

**The problem:** `host::write_file` calls `std::fs::write` (full overwrite). The log-append
operation is therefore `read_file` + string-concat + `write_file` — a read-modify-write, NOT the
OS-atomic `>>` append the bash hook used. A concurrent writer between the read and write would
lose the concurrent writer's data.

**Assessment of actual writer set:**

The `.factory/hooks/precompact-flush-log` file has exactly TWO potential writers:
1. `precompact-flush` (PreCompact event — this plugin)
2. `precompact-flush-prune.sh` (S-18.04b, invoked by `check-state-health`)

**Is concurrent write possible?**

- The dispatcher processes hooks synchronously within a plugin tier (one plugin at a time per
  event). Two concurrent `precompact-flush` invocations on the SAME dispatcher process cannot
  interleave.
- `check-state-health` (which invokes `precompact-flush-prune.sh`) fires on a SEPARATE agent
  dispatch — it is not a hook plugin and does not run within the dispatcher synchronous event
  pipeline. True concurrency between a PreCompact flush and a `check-state-health` prune is
  theoretically possible (two simultaneous Claude Code sessions), but:
  - The factory-lock mechanism (ADR-025) serializes STATE.md writers at the session level.
  - Pruning by `precompact-flush-prune.sh` reads the log and writes back a truncated form;
    a race would at worst lose the last flush's log entry (the factory-lock gate prevents
    concurrent sessions from BOTH reaching the flush + prune critical section simultaneously
    in typical usage).
  - The log is an AUDIT RECORD, not a durability mechanism. The flush COMMIT to factory-artifacts
    is the durability guarantee. A missing log entry causes a BC-5.41.003 tolerated-stale window
    (documented in BC-7.07.001 §EC-003b), not data loss of STATE.md content.

**Resolution:**

The production-grade resolution is **grounded-documentation** — document the single-writer
guarantee and the bounded data-loss window, rather than adding a host O_APPEND primitive
(which would require a dispatcher source change, a new release, and a new story):

- Under normal factory operation (factory-lock held), only one session performs PreCompact flushes.
  The single-writer guarantee holds for the common case.
- The bounded data-loss window (a log entry could be lost in a race between `precompact-flush`
  and `precompact-flush-prune.sh` in concurrent sessions) is acceptable: the commit is the
  durability guarantee; the log is secondary. The next successful flush will append its own entry.
- **NO dispatcher host-API change (O_APPEND) is required** for S-18.04a. This is NOT a scope
  expansion. If a future adversarial review identifies a concrete exploit path for the race,
  an O_APPEND host primitive may be added in a separate dispatcher story. Flagging this potential
  scope item for orchestrator awareness: no immediate action required.

**Implementation note for story-writer:** AC-007 must document this guarantee explicitly so
the implementer understands the semantics of `read_file` + `write_file` for log append.

---

## BC-6.23.001 and S-17.04 Amendment Assessment

**BC-6.23.001 (factory-lock skill acquire/release behaviors):** No amendment required. BC-6.23.001
governs the explicit `/factory-lock` and `/factory-unlock` skills and the `state-manager`
delegation pattern. The native WASM plugin's lock renewal is mechanism-internal to the
`precompact-flush` hook — it is not a new user-facing acquire or release. The skill-delegation
invariant (BC-6.23.001 §Invariant 5: "Neither `/factory-lock` nor `/factory-unlock` writes
STATE.md directly — they delegate to `state-manager`") is about the explicit skill path, not
the hook path. The PreCompact hook is not a skill; it fires automatically. No contradiction.

**S-17.04 (verify-state-timestamp-refresh; BC-5.40.001 §PC4 enforcement):** No amendment required.
S-17.04 delivers the `verify-state-timestamp-refresh` WASM guard and the `state-burst` SKILL
renew step. The native renew in `precompact-flush` is an ADDITIONAL renew path (hook fires on
PreCompact event), not a replacement for the `state-burst` SKILL step. S-17.04's mandate that
`factory-lock-write.sh renew` is called in the state-burst SKILL before `git add` remains
unchanged. ADR-028 adds a second caller (the native WASM `precompact-flush` plugin) for the
specific PreCompact firing scenario.

**BC-7.07.001 (precompact-flush behavioral contract):** Amendment IS required. See §Downstream
Amendment Instructions below for exact changes needed (v1.1 additions to the v1.0 instructions).

---

## Downstream Amendment Instructions

### (a) Product-owner: BC-7.07.001 amendments (v1.1 additions)

These amendments supplement the six v1.0 mechanism amendments already applied in BC-7.07.001 v1.15.
They address the adversarial review findings not covered by the prior amendments.

**Amendment A1 — Precondition 1: env_allow correction (F-NW-001)**

In the registry stanza shown in BC-7.07.001 §Precondition 1, replace:
```toml
env_allow = ["HOME", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME"]
```
With:
```toml
env_allow = ["HOME", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME", "PATH", "SSH_AUTH_SOCK"]
```
Add a note: "`PATH` is required for git to locate `ssh` and credential helpers (needed by
`git push`). `SSH_AUTH_SOCK` is required for SSH agent communication. Without these, `git push`
over SSH will fail with authentication errors."

**Amendment A2 — Precondition 4: -C scoping, format, parse-mapping (F-NW-002, F-NW-003, F-NW-005, F-NW-008)**

Replace the current Precondition 4 text with:
> The native WASM plugin performs lock renewal natively in Rust via `crates/factory-lock`
> (a shared library crate). Lock renewal does NOT invoke `bash factory-lock-write.sh renew`.
> The bash helper `plugins/vsdd-factory/bin/factory-lock-write.sh` remains available for agent/skill
> callers (e.g., the `state-burst` skill per ADR-025 §Decision 11 Mechanism 1) but is NOT a
> dependency of the native WASM plugin. Factory-artifacts worktree path is discovered via runtime
> `git worktree list --porcelain` (ADR-028 §Decision 1); the discovered absolute path is used
> ONLY as the `git -C <abs>` argument — host read_file/write_file always use `.factory/`-relative
> paths. `renew_lock()` outputs `expires_at` in EXACTLY `YYYY-MM-DDTHH:MM:SSZ` format (UTC,
> second precision, Z suffix — NOT chrono rfc3339 +00:00 form). `renew_lock()` returns Err on
> malformed blocks; the hook caller treats Err as advisory (fail-open per §EC-004). `Ok(None)`
> (lock absent) → skip renewal, proceed.

**Amendment A3 — Postcondition 3 (PC3): add format pin and parse-mapping (F-NW-005, F-NW-008)**

Add to PC3:
> `factory_lock.expires_at` is output in EXACTLY `YYYY-MM-DDTHH:MM:SSZ` format (UTC, second
> precision, Z suffix — matches BC-5.40.001 §Invariant 2 and factory-lock-write.sh output format).
> Parse outcomes: `Ok(None)` (lock absent or no frontmatter) → skip renewal (no-op, no advisory);
> `Ok(Some)` (valid lock) → update `expires_at`; `Err` (malformed block) → advisory warn + proceed.

**Amendment A4 — Invariant 3 (INV3) step 3: -C scoping and format pin (F-NW-003, F-NW-008)**

Replace step 3 with:
> (3) if lock held: invoke `crates/factory-lock::renew_lock(state_md_content)` (native Rust;
> no bash exec; outputs `expires_at = now + 2700s` in EXACTLY `YYYY-MM-DDTHH:MM:SSZ` format;
> preserves `holder` and `locked_at`; if `Err`: advisory warn + proceed; renew MUST precede git add)

Replace ALL git subprocesses in §INV3 steps 4–10 with `-C <discovered-worktree-abs-path>` form:
> (4) `git -C <wt> add -u`; (5) `git -C <wt> commit -m ...`; (6) `SHA_B = git -C <wt> rev-parse HEAD`;
> (7) append to precompact-flush-log via host write_file (read-modify-write; see §Decision 7);
> (8) if append fails: `CURRENT_HEAD = git -C <wt> rev-parse HEAD`; reset via `git -C <wt> reset --soft SHA_B^`;
> (9) if append succeeds: `git -C <wt> push origin factory-artifacts`; ...

**Amendment A5 — Invariant 5 / EC-004: INV5 renew-nonempty note (F-NW-004)**

Add a note under INV3 (or EC-004):
> `crates/factory-lock::renew_lock()` returns `Err(Malformed)` on malformed blocks (not `Ok(None)`).
> The `precompact-flush` hook CALLER is the fail-open policy point: it downgrades `Err` to advisory
> warning and proceeds. The library itself does not swallow the error. Future callers that need
> fatal semantics on malformed blocks may treat `Err` as fatal.

**Amendment A6 — Architecture Anchors: env_allow note (F-NW-001)**

Add to the Architecture Anchors section:
> `exec_subprocess` capability: `env_allow = ["HOME", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME", "PATH", "SSH_AUTH_SOCK"]` — `PATH` needed for `git push` SSH binary discovery; `SSH_AUTH_SOCK` for SSH agent auth.

---

### (b) Story-writer: S-18.04a amendments (v1.1 additions)

These amendments supplement the v1.0 amendments already applied in S-18.04a v1.8.
They address the adversarial review findings not covered by the prior amendments.

**Amendment B1 — AC-001: env_allow correction (F-NW-001)**

In AC-001's registry TOML block, replace:
```toml
env_allow = ["HOME", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME"]
```
With:
```toml
env_allow = ["HOME", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME", "PATH", "SSH_AUTH_SOCK"]
```

**Amendment B2 — AC-004 + AC-005 + AC-006 + AC-008 + AC-009: uniform -C ACs (F-NW-003)**

In every AC that mentions a `git` subprocess, change bare `git` invocations to
`exec_subprocess("git", &["-C", &wt, ...])` form. Specifically:

- AC-004: `exec_subprocess("git", &["-C", &wt, "add", "-u"])`
- AC-005: `exec_subprocess("git", &["-C", &wt, "commit", "-m", msg])`
- AC-006: `exec_subprocess("git", &["-C", &wt, "rev-parse", "HEAD"])`
- AC-008: `exec_subprocess("git", &["-C", &wt, "rev-parse", "HEAD"])` (for CURRENT_HEAD check)
          + `exec_subprocess("git", &["-C", &wt, "reset", "--soft", &format!("{}^", sha_b)])`
- AC-009: `exec_subprocess("git", &["-C", &wt, "push", "origin", "factory-artifacts"])`

Add a note: `wt` is the absolute worktree path discovered in AC-017. ALL git subprocesses
except the initial `git worktree list --porcelain` MUST use `-C <wt>`.

**Amendment B3 — AC-007: log-append semantics (F-NW-007)**

Replace AC-007's implementation note with:
> The plugin appends a 4-field log entry via host `read_file` + string concatenation + host
> `write_file` (read-modify-write; NOT OS-atomic append). Under normal factory operation
> (factory-lock held), only one session performs PreCompact flushes; the single-writer guarantee
> holds for the common case. The append MUST produce a `\n`-terminated line as the final byte
> of the file. Field-4 is the literal string `commit`.

**Amendment B4 — AC-018: format-exact contract (F-NW-008)**

Add to AC-018:
> `expires_at` MUST be output in EXACTLY `YYYY-MM-DDTHH:MM:SSZ` format: UTC, second precision,
> uppercase `Z` suffix. Use `chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()` — NOT
> `to_rfc3339()` (which emits `+00:00` and may include sub-seconds). Any deviation is a Red Gate
> failure.

Add to AC-018 (CRLF/file mode, F-NW-009):
> `renew_lock()` outputs LF-only line endings (CRLF normalization is performed internally by
> `parse_factory_lock()` before rewriting). File mode is not preserved (host `write_file` uses
> platform umask defaults); this is an accepted delta from the bash script for STATE.md in git.

**Amendment B5 — Red Gate additions (F-NW-001, F-NW-005, F-NW-008)**

Add to the Red Gate Test Table:

| Test name | File | AC | BC clause | Red Gate condition |
|-----------|------|----|-----------|-------------------|
| `test_git_push_succeeds_with_local_bare_remote` | `plugins/vsdd-factory/tests/precompact-flush-native.bats` | AC-001, AC-009 | ADR-028 §Decision 1 F-NW-001 (PATH+SSH_AUTH_SOCK in env_allow) | Plugin cannot push to local bare remote without PATH; test fails |
| `test_renew_lock_malformed_returns_err_not_ok` | `crates/factory-lock/tests/renew.rs` | AC-018 | ADR-028 §Decision 2 F-NW-004 (library returns Err on malformed; not Ok(None)) | Compile error — crate not yet implemented |
| `test_renew_no_frontmatter_skips_noop` | `crates/factory-lock/tests/renew.rs` | AC-018 | ADR-028 §Decision 2 F-NW-005 (no-frontmatter STATE.md → Ok(None) skip, no advisory) | Compile error |
| `test_expires_at_format_is_z_suffix_second_precision` | `crates/factory-lock/tests/renew.rs` | AC-018 | ADR-028 §Decision 2 F-NW-008 (YYYY-MM-DDTHH:MM:SSZ exact format) | Compile error; would fail if chrono to_rfc3339() used |
| `test_caller_downgrades_renew_err_to_advisory` | `crates/hook-plugins/precompact-flush/tests/integration.rs` | AC-013 | ADR-028 §Decision 2 F-NW-004 (caller treats renew Err as advisory; proceeds to commit) | Compile error |
| `test_bats_tautology_positive_flush_completion` | `plugins/vsdd-factory/tests/precompact-flush-native.bats` | AC-001, AC-005, AC-007 | BC-7.07.001 §INV3 (assert flush commit created AND log entry appended; NOT just no-bash-subprocess) | Plugin not yet compiled; currently asserts no bash only |

The `test_bats_tautology_positive_flush_completion` test replaces or supplements the existing
bats test that only asserts "no bash subprocess in dispatcher log" — a no-bash assertion is
necessary but not sufficient. The test must assert POSITIVE flush completion: commit SHA exists
on factory-artifacts AND precompact-flush-log entry with that SHA is appended.

**Amendment B6 — Traceability table: add Precondition-4 row (F-NW-002, F-NW-003)**

Add to S-18.04a §Traceability table:
| Behavioral Contract | BC-7.07.001 Precondition 4 | AC-017, AC-018 (runtime worktree discovery; .factory/-relative host IO; git -C <wt> for all git subprocesses) |
| ADR | ADR-028 §Decision 3 (uniform git -C) | AC-004, AC-005, AC-006, AC-008, AC-009 |

---

### (c) State-manager: index parity drifts

These are state-manager-domain fixes (index title/version parity). Architect specifies; state-manager
executes in the same burst as BC/story amendments.

**Index fix SM-1 — STORY-INDEX line 661: title stale (S-18.04a title drift)**

STORY-INDEX line 661 shows:
```
| S-18.04a | precompact-flush.sh Core | ...
```
The story title per STORY-INDEX:661 says `precompact-flush.sh Core` but the H1 of the story file
(S-18.04a v1.8) says `precompact-flush Native WASM Plugin Core`. Per POLICY 7, the H1 heading
is the authoritative title. Update the catalog row title cell to verbatim H1:
`precompact-flush Native WASM Plugin Core`.

**Index fix SM-2 — STORY-INDEX line 704: title stale (E-18 summary)**

The E-18 summary block at STORY-INDEX line 704 contains:
`S-18.04a (precompact-flush.sh core; 13 pts; ...)`
Update to:
`S-18.04a (precompact-flush Native WASM Plugin Core; 13 pts; ...)`

**Index fix SM-3 — BC-INDEX line ~2200: BC-7.07.001 title (verify verbatim H1 parity)**

Confirm that the BC-INDEX catalog row for BC-7.07.001 uses the EXACT verbatim H1 from the BC
file. The current H1 is:
```
BC-7.07.001: precompact-flush native WASM plugin fires synchronously on PreCompact; hermetic (STATE.md + git only); renews factory lock natively when held via crates/factory-lock::renew_lock() (no-op when absent); commits (local) with `PreCompact flush <cycle>/<step>` message; appends commit SHA to precompact-flush-log; pushes to remote; on append failure: SHA-pinned reset (`git reset --soft SHA_B^` when HEAD==SHA_B; no-reset + human-intervention exit 2 when HEAD!=SHA_B); exit 2 on commit failure (local; no network) or push failure (network); fail-open on crash
```
If the BC-INDEX row does not match verbatim, update it. If it already matches, no action needed.

**Index fix SM-4 — ARCH-INDEX ADR-028 row version cell**

The ARCH-INDEX Architecture Decisions table currently shows ADR-028 at version `v1.0` (implicit
in the row text "ACCEPTED 2026-06-20; ADR-028 v1.0"). Update the row's version reference to
`v1.1` in the ARCH-INDEX table to reflect this amendment. Also update ARCH-INDEX `last_amended`
and `changelog` with the v2.62 entry per POLICY 14 5-leg parity (version bump is state-manager
domain, not architect domain — architect only edits the ADR file itself; the ARCH-INDEX provenance
leg is state-manager's Commit D responsibility).

---

## Consequences

### Positive

- **Bash dependency eliminated from PreCompact hook.** The `precompact-flush` plugin's `binary_allow`
  reduces from `["bash", "git"]` to `["git"]` only. This removes one surface of the shell-exec
  attack footprint in the WASM sandbox.
- **Runtime worktree discovery is always correct.** No hard-coded `.factory` assumption; the
  actual mount path is read from git's authoritative worktree registry at runtime.
- **Native renewal fidelity.** The Rust implementation of `renew_lock()` mirrors `factory-lock-write.sh
  renew` semantics precisely: preserve `holder` + `locked_at`; update `expires_at = now + 2700s`
  in EXACTLY `YYYY-MM-DDTHH:MM:SSZ` format; no-op when lock absent; `Err` on malformed (library
  layer); fail-open advisory at hook caller layer.
- **Shared crate reusable.** `crates/factory-lock` can be used by future native WASM plugins
  needing lock operations.
- **Standing ADR-014 policy satisfied.** Native WASM migration closes the legacy-bash-adapter
  technical debt for the PreCompact hook.
- **Uniform git -C discipline.** All git subprocesses explicitly scoped to the factory-artifacts
  worktree; no risk of accidentally operating on the main repo.

### Negative / Trade-offs

- **Log-append is read-modify-write, not atomic.** `host::write_file` is a full overwrite. A
  theoretical race between `precompact-flush` and `precompact-flush-prune.sh` in concurrent
  sessions could lose a log entry. Mitigated by: factory-lock serializes typical usage; log is
  secondary to the commit; bounded data-loss window is acceptable. No dispatcher change required.
- **File mode not preserved.** `host::write_file` uses platform umask defaults, unlike the bash
  script's explicit `chmod`. Acceptable for STATE.md in a git worktree (git does not track mode
  for non-executable files).
- **New crate `crates/factory-lock` must be authored.** Frontmatter-boundary-aware YAML surgery
  in Rust requires careful parsing. Mitigation: unit tests over the full matrix of frontmatter
  shapes.
- **`bash factory-lock-write.sh renew` remains in production for skill callers.** Two code paths
  now implement renewal (bash script + Rust crate). They must remain semantically synchronized.
  Mitigation: the Rust `renew_lock()` unit tests use the same semantic table as the bash script.

---

## Risks Addressed

| Risk | Mitigation |
|------|-----------|
| Hard-coded `.factory` path breaks in non-standard worktree mounts | Decision 1: runtime `git worktree list --porcelain` discovery |
| bash exec in WASM sandbox expands attack surface | Decision 2: `binary_allow = ["git"]` only; renewal is native Rust |
| git push fails silently due to missing PATH or SSH_AUTH_SOCK | Decision 1 (v1.1): `PATH` + `SSH_AUTH_SOCK` added to `env_allow`; Red Gate bats test with local bare remote |
| git subprocesses operating on main repo instead of factory-artifacts | Decision 3: uniform `git -C <wt>` on ALL git subprocesses |
| Native renew diverges semantically from bash script renew | Decision 4 (F-NW-004): library returns Err on malformed (faithful); caller downgrades Err to advisory (fail-open); semantic faithfulness table in ADR; unit tests validate contract |
| `expires_at` format diverges from bash output (e.g., chrono rfc3339 vs Z-suffix) | Decision 2 (F-NW-008): format pinned to `YYYY-MM-DDTHH:MM:SSZ`; Red Gate test for exact format |
| Log append race between flush and prune (concurrent sessions) | Decision 7: grounded-documentation; factory-lock serializes typical usage; no dispatcher change needed |
| Rust frontmatter surgery corrupts body content of STATE.md | Shared crate must be frontmatter-boundary-aware; unit tests include body-content edge cases |
| `expires_at` not included in flush commit (ordering defect) | Decision 5: native renew (step 4) MUST precede git add (step 5) |
| No-frontmatter STATE.md causes panic or unexpected advisory during renew | Decision 2 (F-NW-005): Ok(None) for no-frontmatter; skip renewal silently |

---

## Feasibility Risks

| Risk | Severity | Assessment |
|------|----------|-----------|
| YAML frontmatter surgery in Rust — incorrect fence detection corrupts STATE.md | HIGH | Mitigation: unit test over ≥20 frontmatter shapes; property-test with arbitrary body content |
| `git worktree list --porcelain` format changes across git versions | LOW | Format stable since git 2.5; macOS ships git ≥ 2.30; Linux CI git ≥ 2.34 |
| Native `renew_lock()` fidelity drift from bash script | MEDIUM | Mitigation: semantic faithfulness table in this ADR + unit tests; any divergence found during TDD is a RED gate failure |
| `expires_at` format bug (chrono default vs pinned format) | MEDIUM | Mitigation: Red Gate `test_expires_at_format_is_z_suffix_second_precision` |
| Git push fails due to missing env vars in sandbox | HIGH (pre-v1.1) → LOW (post-v1.1) | Mitigated by adding PATH + SSH_AUTH_SOCK to env_allow + local-bare-remote Red Gate bats test |
| Log-append data loss race | LOW | Mitigated by factory-lock serialization; log is secondary to commit; documented |
| `crates/factory-lock` dependency cycle (if `crates/factory-lock-parse` imports it back) | LOW | Keep dependency unidirectional: `factory-lock` → `factory-lock-parse`; never reverse |

---

## Traceability

| Source | ID | Coverage |
|--------|----|---------|
| ADR | ADR-014 | Standing policy: all new hooks native WASM |
| ADR | ADR-025 §Decision 11 | Lock renewal mechanism; `factory-lock-write.sh renew` as Mechanism 1 for skill callers |
| ADR | ADR-026 §Decision 6 | PreCompact flush canonical execution order (commit→append→push) |
| ADR | ADR-027 §Decision 4 | S-18.04a path convention (`FACTORY_ARTIFACTS_PATH`) |
| Behavioral Contract | BC-7.07.001 | precompact-flush full behavior spec; §Inv3 canonical order; §PC3 lock renewal; §EC-004 fail-open |
| Behavioral Contract | BC-5.40.001 | factory_lock schema; TTL invariant 2 (2700s non-configurable); expires_at invariant 3; Z-suffix format |
| Behavioral Contract | BC-6.23.001 | Explicit acquire/release skills; mechanism-internal renewal does not conflict |
| Story | S-18.04a | Implementing story; amendments required per §Downstream Amendment Instructions |
| Story | S-17.04 | Delivers `factory-lock-write.sh renew` and `verify-state-timestamp-refresh`; ADR-028 is additive, not conflicting |
| Capability | CAP-032 | Context-durability feature; E-18 |
| Shell script | `plugins/vsdd-factory/bin/factory-lock-write.sh` | Canonical renew semantics source; native implementation must be faithful |
| Host source | `crates/factory-dispatcher/src/host/exec_subprocess.rs` | `env_clear()` + `env_allow` re-injection (lines 242–247); `cwd` = `CLAUDE_PROJECT_DIR` |
| Host source | `crates/factory-dispatcher/src/host/write_file.rs` | `std::fs::write` full-overwrite semantics; path_allow domain |
| Host source | `crates/factory-dispatcher/src/main.rs` | `base_host_ctx.cwd = ENV_PROJECT_DIR` (~line 303) |
| Parse crate | `crates/factory-lock-parse/src/lib.rs` | `parse_factory_lock()` return taxonomy: `Ok(None)` / `Ok(Some)` / `Err(MalformedLockBlock)` |
| Subsystem | SS-04 | Plugin Ecosystem (WASM plugin crate; shared library crate) |
| Subsystem | SS-07 | Hook Bash Layer (registry stanza; migration from bash hook) |
| Feature | E-18 / issue #173 | Context-durability epic |
