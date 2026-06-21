---
document_type: architecture-decision-record
level: L3
adr_id: ADR-028
version: "1.3"
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
last_amended: "2026-06-20 (v1.3) — Final consolidated spec-fix pass (architect; round-3 adversarial review remediation): F-R3-001 split-tree non-standard-mount hazard RESOLVED — 'non-standard mounts' portability rationale STRUCK from Decision 1; canonical invariant added: factory-artifacts worktree is ALWAYS mounted at <CLAUDE_PROJECT_DIR>/.factory; startup canonicalize assertion added: if canonicalize(discovered) != canonicalize(<cwd>/.factory) → emit DURABILITY DEGRADED advisory + exit 0 (do NOT proceed split-tree); Red Gate and AC added. F-R3-002 has_factory_lock_key scan region PINNED UNAMBIGUOUSLY — scan ONLY between first '---' and second '---' (awk f==1 semantics); no closing fence → open to EOF (matches bash awk open-region behavior, NOT a divergence); mapping table row added for open-fence + lock key present case; Red Gate added. F-R3-003 git add -u durability hole ADJUDICATED — RECOMMENDED DEFAULT changed to git add -A (or git add .); untracked-new .factory/ files ARE captured; EC-008 reconciled (committed partial > lost new; orthogonality yields to durability); INV3/PC4 updated; Red Gate added for new-untracked-file capture. F-R3-005 renewed-identical vs INV3a FIXED — renew_lock returns Ok(RenewOutcome::NoOp) when recomputed expires_at == existing expires_at (byte-identical; same wall-clock second); INV3a 'non-empty IFF Renewed' corrected to 'non-empty when Renewed AND new expires_at differs'; implementation note added. F-R3-006 wrong-branch Red Gate ADDED — bats test asserts after flush: (a) git -C <wt> rev-parse HEAD advanced; (b) main repo HEAD (develop) UNCHANGED; ensures uniform-git-C guarantee has mechanical test coverage. F-R3-007 committer==holder NOT VALIDATED — documented as acceptable BC precondition note (factory-lock serializes single-session; committer identity is operator-configured prerequisite not holder-validated by plugin). CV-002 Decision 4 forward-reference added — 'Either form is acceptable' text superseded by Decision 9; RenewOutcome form is binding; Result<String> form STRUCK; forward-reference note added to Decision 4. ADR-028 v1.2→v1.3. [Prior: 2026-06-20 (v1.2) — Consolidated design-fix pass (architect): F-NW2-001/002/003 write_file.rs path-domain corrected — production host write_file (invoke.rs lines 746-800) resolves relative paths under ctx.cwd=CLAUDE_PROJECT_DIR (matching read_file.rs lines 83-89); write_file.rs::prepare() resolves under plugin_root BUT is ONLY used by its own unit tests; shipped plugins confirm native WASM is viable WITHOUT a dispatcher release; PREREQUISITE micro-story (S-18.04a-prereq) required to align write_file.rs::resolve_for_write to ctx.cwd + amend BC-2.02.011 invariant 3 + fix precompact-routing.bats equal-roots masking (exact change surface in §Decision 8). F-NW2-004 committer identity: rely on HOME/GIT_CONFIG_GLOBAL for global git identity (GIT_AUTHOR/COMMITTER vars NOT added to env_allow); Red Gate for commit-identity documented. F-NW2-005 renew_lock() signature pinned to pure content-in/content-out `renew_lock(&str) -> Result<RenewOutcome, LockError>` (RenewOutcome::NoOp|Renewed(String)); path-based form struck from AC-018. F-NW2-006 malformed-fence parity: renew_lock() adds factory_lock: presence pre-check so lock-absent-with-malformed-fence -> Ok(NoOp) (bash parity); Err(Malformed) reserved for fence-malformed-AND-lock-present; BC PC3/EC-004 updated; Red Gate added. F-NW2-007 empty-commit vs renew-nonempty: if RenewOutcome::NoOp AND no other changes -> INV5 clean-state exit 0 (never force empty commit); renew-makes-nonempty applies only when RenewOutcome::Renewed; test vector added. F-NW2-008 read_file on absent file -> CAPABILITY_DENIED; AC-007 specifies read error treated as EMPTY prior content for log-append (first-flush Red Gate added). F-NW2-009 worktree-discovery posture: discovery failure is fail-open exit 0 BUT with LOUD advisory so silently-disabled-durability session is visible; BC PC4/AC-017 updated. Consistency fixes: AC-011 bare git steps 5/6/7/10 corrected (B7); phantom F-NW-010 label replaced with stable anchor (B8); BC-7.07.001 Traceability §Decision refs corrected (A7). ADR-028 v1.1→v1.2. [Prior: 2026-06-20 (v1.1) — Adversarial review remediation (architect): F-NW-001 PATH+SSH_AUTH_SOCK added to env_allow; GIT_SSH_COMMAND assessed; Red Gate added for real git push to local bare remote. F-NW-002 [BLOCKER] path_allow domain vs worktree discovery clarified: discovered absolute path used ONLY for git -C argument; host read_file/write_file always use .factory/-relative paths gated by path_allow; canonical invariant documented (discovered-worktree == <cwd>/.factory). F-NW-003 [BLOCKER] uniform git -C mandate: ALL git subprocesses must use git -C <discovered-worktree-abs-path>; bare git commands without -C are forbidden; canonical 11-step execution order updated. F-NW-005 [MAJOR] parse-result→renew-outcome mapping fully specified: Ok(None) → no-op; Ok(Some valid) → update expires_at; Err(malformed) → advisory warn + proceed; no-frontmatter STATE.md → Ok(None) (skip, no advisory); Red Gate added. F-NW-004 [MAJOR] renew fidelity divergence resolved: renew_lock() returns Err on malformed (library faithfulness preserved); precompact-flush CALLER downgrades Err to advisory + proceeds (hook fail-open preserved); contradiction in semantic-faithfulness table removed. F-NW-007 [MAJOR] log-append concurrency assessed: write_file is full-overwrite (read-modify-write, NOT OS-atomic append); single-writer guarantee documented (dispatcher synchronous + factory-lock gate); bounded data-loss window stated; NO dispatcher host-API change required (grounded-documentation path). F-NW-008 [MAJOR] expires_at format pinned: EXACTLY YYYY-MM-DDTHH:MM:SSZ (UTC, second precision, Z suffix — NOT chrono default rfc3339 +00:00 / sub-seconds). F-NW-009 [MINOR] CRLF + file mode: native crates/factory-lock normalizes to LF-only output; file mode not applicable (host write_file creates/overwrites with platform defaults; no chmod). ADR traceability stale cites replaced with stable anchors per POLICY 19 (TD-VSDD-091). ADR-028 v1.0→v1.1. [Prior: 2026-06-20 (v1.0) — Initial authoring (architect; human decisions incorporated: runtime git worktree discovery; native lock renewal binary_allow=[\"git\"] only; shared crate `crates/factory-lock` for reusable renewal logic; ordering confirmation in 11-step canonical order; BC-7.07.001/S-18.04a amendment instructions produced).]"
---

# ADR-028: E-18 precompact-flush native WASM migration — runtime worktree discovery and native lock renewal

## Status

**ACCEPTED — 2026-06-20 (v1.0). Amended 2026-06-20 (v1.1) — adversarial review remediation. Amended 2026-06-20 (v1.2) — consolidated design-fix pass. Amended 2026-06-20 (v1.3) — final spec-fix pass (round-3 adversarial review remediation).**

This ADR codifies two human-directed decisions for the S-18.04a native WASM migration of the
`precompact-flush` hook plugin:

1. **Worktree path discovery** = runtime git discovery via `exec_subprocess("git", &["worktree", "list", "--porcelain"], ...)` — no hard-coded path, no registry config key.
2. **Lock renewal** = native Rust WASM (eliminate bash dependency). Plugin performs lock renewal natively: read STATE.md, parse `factory_lock:` block, update `expires_at`, write STATE.md back. Registry stanza `binary_allow = ["git"]` ONLY — no `bash`, no `shell_bypass_acknowledged`.

This ADR is the authoritative design reference for S-18.04a when the precompact-flush plugin is migrated from `legacy-bash-adapter.wasm` to a native WASM crate.

---

## Changelog

| Version | Date | Change |
|---------|------|--------|
| v1.3 | 2026-06-20 | Final spec-fix pass (round-3): F-R3-001 non-standard-mount rationale STRUCK; startup canonicalize assertion + MISMATCH→DURABILITY DEGRADED; F-R3-002 has_factory_lock_key scan region pinned to awk f==1 semantics (frontmatter only; open to EOF when no closing fence — bash parity, not divergence); F-R3-003 staging adjudicated → git add -A recommended (durability > EC-008 orthogonality); F-R3-005 renew_lock returns NoOp on byte-identical expires_at; F-R3-006 wrong-branch Red Gate added; F-R3-007 committer==holder documented as acceptable; CV-002 Decision 4 forward-reference to Decision 9. |
| v1.2 | 2026-06-20 | Consolidated design-fix pass: F-NW2-001/002/003 write_file.rs path-domain false-blocker corrected + PREREQUISITE micro-story (S-18.04a-prereq) specified; F-NW2-004 committer identity under env_clear resolved (rely on HOME/GIT_CONFIG_GLOBAL); F-NW2-005 renew_lock() pure content signature pinned + path-based form struck; F-NW2-006 malformed-fence parity via factory_lock: presence pre-check in renew_lock(); F-NW2-007 empty-commit vs renew-nonempty precedence; F-NW2-008 read_file absent-file → treat-as-empty-for-log-append; F-NW2-009 worktree-discovery loud advisory posture. Consistency fixes: AC-011 bare git steps B7; phantom F-NW-010 label B8; BC-7.07.001 Traceability §Decision refs A7. |
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

**Canonical invariant (F-R3-001 corrected — non-standard-mount rationale STRUCK):**

The factory-artifacts worktree is ALWAYS mounted at `<CLAUDE_PROJECT_DIR>/.factory` by project
convention (see CLAUDE.md "mounted at `.factory/` via worktree"). This is an invariant, not a
flexibility. Therefore: `canonicalize(discovered-worktree-abs-path) == canonicalize(<cwd>/.factory)`.

**STRUCK:** The prior v1.2 text stated "if a non-standard mount is ever used, the implementation
MUST derive `path_allow` from the discovered path." This is REMOVED. Non-standard mounts are
NOT supported. Supporting them would require: (a) `path_allow` cannot be a static `.factory/`
prefix in the registry stanza (it is static), (b) host `read_file`/`write_file` use the
`path_allow`-gated relative `.factory/` domain (Domain A), while git uses the discovered absolute
path (Domain B). A non-standard mount creates a split-tree: renew writes to `<cwd>/.factory/STATE.md`
(Domain A) while git commits from a DIFFERENT worktree directory (Domain B). This is data loss.

**Startup assertion (F-R3-001 addition):**

At plugin startup, after discovering the factory-artifacts worktree path, the plugin MUST verify:

```
canonicalize(discovered_worktree_abs_path) == canonicalize(<CLAUDE_PROJECT_DIR> + "/.factory")
```

Where `CLAUDE_PROJECT_DIR` is `ctx.cwd` (the main repo root, set by `ENV_PROJECT_DIR`).

- **If MATCH**: proceed normally. Domain A (`.factory/`-relative host I/O) and Domain B (git
  `-C <discovered>`) are consistent — they address the same filesystem location.
- **If MISMATCH**: emit `DURABILITY DEGRADED` advisory to stderr (UPPERCASE prefix, same visibility
  requirement as Decision 13) and exit 0. Do NOT proceed with a split-tree flush. The message:
  `precompact-flush: DURABILITY DEGRADED — factory-artifacts worktree path mismatch: discovered
  <discovered_path> but expected <cwd>/.factory; flush SKIPPED to prevent split-tree data loss.
  Ensure factory-artifacts is mounted at .factory/ (run: git worktree add .factory factory-artifacts).`

**Reframed purpose of runtime discovery:**

`git worktree list --porcelain` discovery is NOT a portability mechanism — it is a CONSISTENCY
CHECK that obtains the absolute path for `git -C` while verifying the invariant holds. The
benefits are:
- No hard-coded path string in the plugin source code (git's own registry is the source of truth).
- Startup assertion catches mis-configured worktree mounts before producing corrupted state.
- Consistent with ADR-024's approach: runtime resolution for log-dir and CLAUDE_PLUGIN_ROOT.

**Rationale:**

- No registry config key: reduces configuration surface; the git worktree list is the authoritative
  source of truth for worktree mounts and is always available when git is in scope.
- Consistent with ADR-024's approach: the dispatcher itself uses runtime resolution for log-dir
  and CLAUDE_PLUGIN_ROOT rather than hard-coded conventions.
- Non-standard mount support struck: the split-tree hazard (Domain A writes to one path, Domain B
  git operates on another) creates data loss risk that cannot be safely mitigated within current
  plugin capabilities. The canonical convention (mount at `.factory/`) is the only supported mode.

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
| Stage changes | `git -C <wt> add -A` (see F-R3-003 below) |
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

**Note (CV-002 — Decision 9 supersedes this paragraph):** Decision 9 (added in v1.2) pins the
`RenewOutcome` form as the BINDING signature. The alternative `Result<String, LockError>` form
mentioned in v1.0/v1.1 is STRUCK. See §Decision 9 for the authoritative enum definition and
signature. The "Either form is acceptable" text below applied only before Decision 9 was added
and is superseded; it is retained here only as historical context.

~~Alternatively, the signature may distinguish no-op from write:~~ *(Historical — STRUCK by
Decision 9)*

The only acceptable form is:
```rust
pub enum RenewOutcome { NoOp, Renewed(String) }
pub fn renew_lock(state_md_content: &str) -> Result<RenewOutcome, LockError>
```

The key constraints are: (a) `Err` is returned for malformed blocks so the caller can emit an
advisory; (b) `NoOp` is returned when the lock is absent or the key is absent (see Decision 9
presence pre-check); (c) the crate has no `std::fs` dependency (pure content-in/content-out).

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
(5)  git -C <wt> add -A
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

### Decision 8 (new) — PREREQUISITE correctness fix: align write_file.rs to cwd (F-NW2-001/002/003)

**Background — false blocker resolved:**

A spike confirmed that the "write_file blocker" previously discussed is FALSE for production: the
production plugin invocation path in `crates/factory-dispatcher/src/invoke.rs` (lines 746-800)
implements `write_file` with relative paths resolved under `ctx.cwd = CLAUDE_PROJECT_DIR` (the
main repo root). This matches `read_file.rs` (lines 83-89: `resolve_for_read` uses `ctx.cwd`).
Shipped plugins (`regression-gate`, `update-wave-state-on-merge`) already write `.factory/` files
via this cwd-rooted production path in production. Native WASM migration is viable without any
dispatcher host-ABI change or release.

**The actual defect:**

`crates/factory-dispatcher/src/host/write_file.rs::resolve_for_write()` (line 111-117) resolves
relative paths under `plugin_root` rather than `ctx.cwd`. This function is called by `prepare()`
which is used ONLY in unit tests within `write_file.rs` itself — the production dispatcher uses
the inline implementation in `invoke.rs`. The defect is therefore a unit-test facade inconsistency,
not a production blocker. However, it creates two risks:

1. **BC-2.02.011 invariant 3 is stale**: it claims write_file joins with `ctx.plugin_root` (from
   the unit-test facade), but production uses `ctx.cwd`. The invariant is incorrect.
2. **precompact-routing.bats equal-roots masking**: at line 216, the bats test sets
   `CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK'` — both roots equal the same
   directory. This means the precompact flush positive-flush Red Gate
   (`test_flush_completes_via_git_only_positive`) does NOT exercise the distinction between the
   `write_file.rs` plugin_root path and the `invoke.rs` cwd path — both resolve to the same
   files. The test is tautological with respect to path resolution.

**PREREQUISITE deliverable (S-18.04a-prereq):**

This prerequisite MUST ship as its own PR and be merged before S-18.04a begins implementation.
It contains no native WASM plugin code — only correctness fixes to existing code and specs.

Suggested story ID: `S-18.04a-prereq` (blocking S-18.04a). Scope:

**(a) align `write_file.rs::resolve_for_write` to `ctx.cwd`:**

In `crates/factory-dispatcher/src/host/write_file.rs`, replace the `prepare()` function's path
resolution to match `invoke.rs` production semantics:

```rust
// Change `prepare()` signature to accept cwd:
fn prepare(ctx: &HostContext, path: &str, contents: &[u8], max_bytes: u32) -> Result<(), i32> {
    // ...
    // Replace:
    let resolved = resolve_for_write(Path::new(path), &ctx.plugin_root);
    // With:
    let resolved = resolve_for_write(Path::new(path), &ctx.cwd);
    // ...
    // Replace allowlist check:
    if !path_allowed(&resolved, &caps.path_allow, &ctx.plugin_root) {
    // With:
    if !path_allowed(&resolved, &caps.path_allow, &ctx.cwd) {
}

// Change `resolve_for_write` to accept the cwd base:
fn resolve_for_write(path: &Path, base: &Path) -> PathBuf {
    if path.is_absolute() { path.to_path_buf() } else { base.join(path) }
}
```

Update the doc comment on `resolve_for_write` to reflect `base` = `ctx.cwd` (matching
`read_file.rs::resolve_for_read`). Update unit tests to set `ctx.cwd` instead of
`ctx.plugin_root` for the path prefix.

**(b) amend BC-2.02.011 invariant 3:**

Current text (stale): "Relative `path` values are joined with `ctx.plugin_root`."
Correct text: "Relative `path` values are joined with `ctx.cwd` (`CLAUDE_PROJECT_DIR`),
mirroring `resolve_for_read` as of S-8.07. The `plugin_root`-rooted resolution in
`write_file.rs::prepare()` was a unit-test facade bug; production `invoke.rs` has always
used `ctx.cwd`. S-18.04a-prereq aligns the unit-test facade to production semantics."

**(c) fix precompact-routing.bats line 216 (equal-roots masking):**

Change the `_run_dispatcher()` helper to use DISTINCT directories for plugin root and project dir:

```bash
# Before (line 216 — tautological, both roots equal):
run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1"

# After (non-tautological — project dir is a subdirectory):
PROJECT_DIR="$WORK/project"
mkdir -p "$PROJECT_DIR"
run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$PROJECT_DIR' '$DISPATCHER' 2>&1"
```

This makes the precompact flush positive-flush Red Gate (`test_flush_completes_via_git_only_positive`)
genuinely exercise the cwd write path rather than coincidentally passing because both roots are equal.
All path_allow entries in the test registry must be updated to absolute paths or paths relative to
`$PROJECT_DIR` accordingly.

**Why this is a BLOCKING prerequisite for S-18.04a:**

S-18.04a's `test_flush_completes_via_git_only_positive` Red Gate (renamed from tautological
`test_bats_tautology_positive_flush_completion` per B8) asserts that the native plugin writes
`.factory/hooks/precompact-flush-log` via host `write_file`. This assertion is non-tautological
only when `CLAUDE_PLUGIN_ROOT != CLAUDE_PROJECT_DIR`. If S-18.04a ships without the bats fix,
the test passes vacuously under equal-roots and will not catch a regression where write_file
accidentally uses plugin_root.

**No dispatcher release required:** S-18.04a-prereq is a Rust source change + bats test fix.
It does not change the dispatcher's ABI, wire protocol, or published hook-sdk interface. It does
not require a new release tag before native WASM plugin development begins — it only needs to
land on `develop` and be tested before S-18.04a branches.

---

### Decision 9 (new) — `renew_lock()` pure content-in/content-out signature with factory_lock: presence pre-check (F-NW2-005/006)

**Binding signature:**

```rust
pub enum RenewOutcome {
    /// Lock was absent (or no frontmatter present with no lock) — STATE.md unchanged.
    NoOp,
    /// Lock was held and expires_at was updated. Contains the new full STATE.md content.
    Renewed(String),
}

pub fn renew_lock(state_md_content: &str) -> Result<RenewOutcome, LockError>
```

The prior v1.1 text allowed an alternative `renew_lock(&Path)` / `renew_lock(&factory_artifacts_path)`
path-based form. This is **STRUCK**. The only acceptable signature is the pure content-in/content-out
form above. Callers (the `precompact-flush` plugin) own the read_file and write_file operations;
`crates/factory-lock` stays pure (no `std::fs` calls, no host I/O — compatible with WASM hermetic
model). This also means `crates/factory-lock` cannot depend on `std::fs` and remains `no_std`-friendly.

**factory_lock: presence pre-check (F-NW2-006 — bash parity):**

`crates/factory-lock-parse::parse_factory_lock()` returns `Err(MalformedLockBlock)` when the
STATE.md content starts with `---\n` (opening fence present) but has no closing `---` delimiter —
EVEN IF there is no `factory_lock:` block in the frontmatter. This diverges from bash
`factory-lock-write.sh renew`, which checks `factory_lock:` presence first (awk search for
`factory_lock:` between frontmatter fences) and silently exits 0 if absent — the fence shape
is never checked for the "absent lock" early-exit path.

`renew_lock()` MUST add a `factory_lock:` presence pre-check BEFORE calling `parse_factory_lock()`:

```rust
pub fn renew_lock(state_md_content: &str) -> Result<RenewOutcome, LockError> {
    // Step 1: Pre-check — is factory_lock: present at all?
    // Scan the frontmatter region for the `factory_lock:` key.
    // If absent → Ok(NoOp) regardless of fence shape (bash parity).
    if !has_factory_lock_key(state_md_content) {
        return Ok(RenewOutcome::NoOp);
    }
    // Step 2: Parse the full frontmatter (fence-aware).
    // If fence is malformed AND lock key IS present → Err(Malformed).
    let lock_state = parse_factory_lock(state_md_content)?;
    // Step 3: If lock is null/absent despite key being present → NoOp.
    let held = match lock_state {
        None => return Ok(RenewOutcome::NoOp),
        Some(ls) => ls,
    };
    // Step 4: Update expires_at, return Renewed(new_content).
    // ...
    Ok(RenewOutcome::Renewed(new_content))
}
```

The `has_factory_lock_key()` helper scans for the literal string `factory_lock:` within the
frontmatter region (lines between first `---\n` and next `\n---\n` or EOF). If the frontmatter
is malformed (no closing fence), the scan looks between `---\n` and EOF. If `factory_lock:`
is not found, return `Ok(NoOp)` immediately — no call to `parse_factory_lock()`.

**Updated Err(Malformed) semantics:**

`Err(LockError::Malformed)` is returned ONLY when `factory_lock:` IS present in the frontmatter
AND the block is malformed (missing field, empty field, missing closing fence after the lock key
is present). This is the exact bash parity: bash only errors out (exit 1 `RenewalMissed`) when
the lock BLOCK is present but missing `expires_at`.

**Updated parse-result → renew-outcome mapping (supersedes v1.1 F-NW-005 mapping):**

| State | `has_factory_lock_key()` | `parse_factory_lock()` | `renew_lock()` returns |
|-------|--------------------------|------------------------|------------------------|
| No frontmatter (no `---\n` prefix) | false | n/a | `Ok(NoOp)` |
| Frontmatter present, no closing fence, NO lock key | false | n/a | `Ok(NoOp)` |
| Frontmatter present, no closing fence, lock key present | true | `Err(Malformed)` | `Err(Malformed)` |
| Frontmatter present, closing fence, no `factory_lock:` key | false | n/a | `Ok(NoOp)` |
| Frontmatter present, `factory_lock:` null/absent | true | `Ok(None)` | `Ok(NoOp)` |
| Frontmatter present, valid lock with all three fields | true | `Ok(Some(lock))` | `Ok(Renewed(content))` |
| Frontmatter present, lock key present but malformed | true | `Err(Malformed)` | `Err(Malformed)` |

**Red Gate requirement (F-NW2-006):**

Add to S-18.04a Red Gate Test Table:

```
test_renew_lock_malformed_fence_no_lock_key_returns_noop
File: crates/factory-lock/tests/renew.rs
Input: STATE.md with opening `---\n` fence but no closing `---` and no `factory_lock:` key
Expected: Ok(RenewOutcome::NoOp) — NO advisory emitted
Red Gate: Compile error (crate not implemented); would fail if Err(Malformed) returned
```

---

### Decision 10 (new) — Committer identity under env_clear (F-NW2-004)

**Analysis:**

`exec_subprocess` runs with `env_clear` followed by re-injection of only the names in `env_allow`
(see `crates/factory-dispatcher/src/host/exec_subprocess.rs` lines 242-250). The current
`env_allow` for git is:

```toml
env_allow = ["HOME", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME", "PATH", "SSH_AUTH_SOCK"]
```

For `git commit` to succeed, git must resolve a committer identity (`user.name` + `user.email`).
Git resolves identity in the following precedence order:

1. `GIT_AUTHOR_NAME` / `GIT_COMMITTER_NAME` / `GIT_AUTHOR_EMAIL` / `GIT_COMMITTER_EMAIL`
   environment variables (highest priority — per-commit override).
2. `user.name` / `user.email` in git config files (`~/.gitconfig`, `$GIT_CONFIG_GLOBAL`,
   `$XDG_CONFIG_HOME/git/config`).
3. OS-level `gecos` / hostname fallback (if git is compiled with this fallback enabled).

**Decision:**

`GIT_AUTHOR_NAME`, `GIT_AUTHOR_EMAIL`, `GIT_COMMITTER_NAME`, `GIT_COMMITTER_EMAIL` are NOT
added to `env_allow`. The production-grade default relies on the operator's global git identity
configured in `~/.gitconfig` or `$GIT_CONFIG_GLOBAL`. This is the correct posture because:

- `HOME` is already in `env_allow`. When git resolves `~/.gitconfig`, it expands `~` using
  `HOME`. With `HOME` re-injected, git can find the global config.
- `GIT_CONFIG_GLOBAL` and `XDG_CONFIG_HOME` are also in `env_allow`, covering non-standard
  config locations.
- Factory operations already require a properly configured git environment (git push to remote
  requires SSH credentials); committer identity is a baseline requirement that operators must
  have configured.
- Adding `GIT_AUTHOR_*` / `GIT_COMMITTER_*` to `env_allow` would expose any session-level
  identity override to the plugin, which is unnecessary.

**Documented precondition:**

The operator MUST have `user.name` and `user.email` configured in `~/.gitconfig` or a file
reachable via `GIT_CONFIG_GLOBAL`. If identity is not configured, `git commit` exits non-zero
with `Author identity unknown` — the plugin treats this as AC-005b commit-failure → exit 2
(block compaction). This is the correct production fail-safe: a mis-configured environment should
block the flush rather than emit an empty or mis-attributed commit.

**Red Gate requirement (F-NW2-004):**

Add to S-18.04a Red Gate Test Table:

```
test_git_commit_succeeds_with_global_identity_via_home
File: plugins/vsdd-factory/tests/precompact-flush-native.bats
ACs: AC-005, AC-017
Description: Test fixture creates a temp $HOME with a .gitconfig containing user.name and user.email.
  Dispatcher is invoked with HOME pointing to this temp dir (HOME in env_allow).
  Assert: git commit exits 0; commit SHA is non-empty; author identity matches .gitconfig.
Red Gate: Plugin not yet compiled; test setup not yet implemented.
```

---

### Decision 11 (new) — empty-commit vs renew-nonempty precedence (F-NW2-007)

**Precedence rule:**

INV5 ("empty commit is forbidden") is evaluated AFTER the `renew_lock()` call, using the
post-renewal STATE.md content as the baseline for `git diff --cached` semantics. The precedence
is:

1. Renew step runs → produces `RenewOutcome::NoOp` (lock absent) or `RenewOutcome::Renewed(new_content)`.
2. If `RenewOutcome::Renewed`: write updated STATE.md via `host::write_file` → the git staging
   area will include the renewal change → flush commit is non-empty (INV3a: "renew makes nonempty").
3. If `RenewOutcome::NoOp`: STATE.md not written → check `git -C <wt> diff --cached` for
   any other staged changes.
   - If no changes: INV5 clean-state → exit 0 silently (do NOT force a commit).
   - If changes exist: proceed to commit.

**Byte-identical renewal guard:**

If two PreCompact flushes fire within the same wall-clock second AND the lock was held both
times, the first flush renews `expires_at = T` and commits. The second flush also computes
`expires_at = T` (same second). After `renew_lock()`, the STATE.md content is byte-identical
to the committed content (both flushes compute the same `T`). `git -C <wt> diff --cached`
after `git add -u` shows no diff → INV5 clean-state → exit 0 (no empty commit forced).

This is the correct behavior: forcing a commit with no content change would produce an empty
commit that clutters the factory-artifacts history. The previous v1.1 text implied that renew
ALWAYS makes the commit non-empty — this was an overstatement. INV3a is now refined:
"Renewal makes the commit non-empty IF AND ONLY IF `renew_lock()` returns
`RenewOutcome::Renewed` and the new expires_at differs from the committed expires_at."

**Test vector (add to BC-7.07.001 §Canonical Test Vectors):**

| Input | Expected Output | Category |
|-------|----------------|----------|
| PreCompact event; STATE.md readable; factory_lock: absent; no pending factory-artifacts changes | exit 0; no commit; no precompact-flush-log line appended (`RenewOutcome::NoOp` + clean state) | no-lock-clean-state |

---

### Decision 12 (new) — read_file on absent/non-existent file for log-append (F-NW2-008)

**Problem:**

`host::read_file` (both `read_file.rs::prepare` and `invoke.rs` production path) calls
`canonicalize()` on the target path to validate the allowlist. `canonicalize()` fails on
non-existent files (returns `Err(NotFound)`), which causes `path_allowed()` to return `false`,
and `read_file` returns `CAPABILITY_DENIED (-1)`. This means the FIRST flush (when
`.factory/hooks/precompact-flush-log` does not exist yet) will receive a `CAPABILITY_DENIED`
from `read_file` on the log path.

**Resolution for log-append (AC-007):**

A `read_file` error on the precompact-flush-log path — including `CAPABILITY_DENIED` on a
non-existent file — MUST be treated as EMPTY prior content (empty byte string `""`), NOT as
an append failure. The plugin concatenates `"" + new_entry\n` and calls `write_file` (which
supports creating new files, per BC-2.02.011 EC-005 and `write_file.rs::resolve_path_for_allowlist`
which handles non-existent file creation).

This mirrors the behavior of `regression-gate`'s "any read error → no prior state" pattern.
It is NOT a silent failure: the plugin is intentionally handling the "file does not exist" case
as an empty baseline, which is the correct semantic for an append-create operation.

**What is NOT treated as empty:**

Only the log-append read is treated as empty-on-error. Errors from `read_file` on STATE.md
(the primary state file) are treated as AC-002 (STATE.md unreadable → exit 0 + warn to stderr).
Do not generalize the empty-on-error treatment to STATE.md reads.

**SHA-pinned reset guard interaction:**

The sha-pinned reset guard (INV3 step 9) fires on APPEND FAILURE — i.e., when `write_file`
returns an error AFTER a successful `read_file` (or empty-baseline). If `read_file` returns
CAPABILITY_DENIED (non-existent file), the plugin proceeds with the empty baseline and calls
`write_file`. If `write_file` then fails (e.g., parent directory doesn't exist), THAT is the
append failure that triggers the reset guard. The `read_file` CAPABILITY_DENIED on the
non-existent log file is NOT itself an append failure.

**Red Gate requirement (F-NW2-008):**

```
test_first_flush_with_absent_log_appends_successfully
File: crates/hook-plugins/precompact-flush/tests/integration.rs
AC: AC-007
Description: Fixture has no precompact-flush-log file. PreCompact event fires.
  Assert: plugin exits 0; precompact-flush-log is created with exactly one LF-terminated
  4-field entry. No advisory emitted for the absent log.
Red Gate: Compile error; would fail if read_file CAPABILITY_DENIED propagated as append failure.
```

---

### Decision 13 (new) — worktree-discovery failure posture: fail-open with LOUD advisory (F-NW2-009)

**Context:**

Two distinct failure cases arise during worktree discovery:

1. **`git worktree list --porcelain` command itself fails** — e.g., git binary not found,
   env_allow insufficient, or git process crashes. This is an ENVIRONMENT problem.
2. **Command succeeds but factory-artifacts branch is not listed** — the git command ran, but
   the factory-artifacts worktree is not mounted. This is a CONFIGURATION problem.

Both cases currently map to `exit 0 + advisory warning` (BC-7.07.001 PC7 / PC4 / BC-7.07.001
INV3 step 1 fail-open). This is correct for the `on_error = "continue"` contract — we must
not block compaction when we can't determine whether there is anything to flush.

**However, silent skip is dangerous.** If a session silently skips every PreCompact flush
because the worktree discovery fails (e.g., due to a PATH issue introduced in a config change),
the durability guarantee is silently disabled. The next compaction proceeds with no prior flush,
and STATE.md changes are lost.

**Revised posture — LOUD advisory:**

The advisory message MUST be structured and unambiguous enough that a human reading the session
log immediately recognizes that durability is degraded. The messages MUST be:

- For case 1 (git command failed): `precompact-flush: DURABILITY DEGRADED — git worktree list
  command failed (exit <N>); factory-artifacts worktree cannot be discovered; flush SKIPPED
  this compaction event. Check PATH/git configuration.`
- For case 2 (command succeeded, no factory-artifacts stanza found): `precompact-flush:
  DURABILITY DEGRADED — factory-artifacts branch not found in git worktree list output;
  flush SKIPPED this compaction event. Ensure the factory-artifacts worktree is mounted at
  .factory/ (run: git worktree add .factory factory-artifacts).`

Both messages are written to stderr with the prefix `precompact-flush: DURABILITY DEGRADED`
(uppercase) so they are findable in logs and session history.

**Impact on BC-7.07.001 PC4 / AC-017:**

BC-7.07.001 PC4 text should add: "If discovery fails (command error OR no stanza found), the
plugin exits 0 with a DURABILITY DEGRADED advisory to stderr (see §Decision 13 message format).
The exit-0 fail-open is maintained per on_error=continue; the LOUD advisory ensures the
degraded state is detectable."

---

### Decision 14 (new v1.3) — `has_factory_lock_key()` scan region: unambiguous frontmatter-only scoping (F-R3-002)

**Problem (v1.2 ambiguity):**

Decision 9 described `has_factory_lock_key()` as scanning "within the frontmatter region (lines
between first `---\n` and next `\n---\n` or EOF)." The phrase "or EOF" was ambiguous — it did
not explicitly state what happens when there is an OPENING `---` fence but no CLOSING `---` fence,
and whether the scan in that case is "awk parity" or "a new divergence."

**Binding resolution (F-R3-002):**

`has_factory_lock_key()` scans EXACTLY the frontmatter region as defined by:

- **Opening boundary**: the first occurrence of `---\n` at the start of the content.
- **Closing boundary**: if a second `---` delimiter exists (as `\n---\n` or `\n---` at EOF),
  the scan region ends there (exclusive). This is equivalent to bash awk `f==1` — the region
  BETWEEN the first and second fence delimiters.
- **No closing fence**: if there is NO second `---` delimiter, the scan continues to EOF. This
  matches bash `awk '/^---$/{f++} f==1 && /^factory_lock:/{found=1} f>=2{exit}'` behavior:
  when the closing `---` never sets `f=2`, awk reads the entire rest of the file with `f==1`
  active — so any `factory_lock:` key in the body is detected as "present in frontmatter."

**This is bash parity, NOT a divergence.** Both bash awk and `has_factory_lock_key()` see a
`factory_lock:` key anywhere between the opening `---` and EOF (when no closing fence exists).
The v1.2 text implied this might be a divergence; it is not — it is faithful reproduction of
the bash awk open-region behavior.

**Updated parse-result → renew-outcome mapping (v1.3 addition — open-fence case):**

| State | `has_factory_lock_key()` | `parse_factory_lock()` | `renew_lock()` returns |
|-------|--------------------------|------------------------|------------------------|
| Opening `---`, NO closing `---`, `factory_lock:` ABSENT in body | false | n/a | `Ok(NoOp)` |
| Opening `---`, NO closing `---`, `factory_lock:` PRESENT in body | **true** (awk parity) | `Err(Malformed)` | `Err(Malformed)` — caller advisory warn; proceed |

The second row is the key addition: when the fence is unclosed AND the lock key IS present in
the body region, `parse_factory_lock()` returns `Err(Malformed)` (missing closing fence). The
caller treats this as advisory (fail-open per §EC-004). This is the correct production-grade
behavior: an unclosed-fence STATE.md with a factory_lock: key present is a malformed document;
the advisory is appropriate.

**Implementation note:**

`has_factory_lock_key()` searches for the literal string `factory_lock:` (at line start, with
optional leading spaces) in the region from after the opening `---\n` to before the closing
`---` (or EOF if absent). Do NOT use `parse_factory_lock()` for this pre-check — it must be
a direct string scan to avoid the recursive call problem.

**Red Gate requirement (F-R3-002):**

```
test_has_factory_lock_key_open_fence_with_lock_key_returns_true
File: crates/factory-lock/tests/renew.rs
Input: STATE.md content starting with "---\n" (opening fence), no closing "---",
  "factory_lock:" key present in the body region.
Expected: has_factory_lock_key() returns true; renew_lock() returns Err(Malformed)
  (because parse_factory_lock() finds no closing fence); caller emits advisory.
Red Gate: Compile error — crate not implemented; would fail if Ok(NoOp) returned.
```

---

### Decision 15 (new v1.3) — Staging strategy: `git add -A` over `git add -u` (F-R3-003 adjudication)

**The durability hole in `git add -u`:**

`git add -u` stages ONLY tracked changes (modifications and deletions to files git already knows
about). If a flush's only pending content is a NEW untracked `.factory/` file (e.g., a new BC
file written by an agent that hasn't been staged before), `git add -u` produces no staged content.
The subsequent `git -C <wt> diff --cached` shows empty → INV5 clean-state → exit 0. The new
file is silently LOST on compaction.

**EC-008 reconciliation:**

The prior choice of `add -u` was motivated by EC-008 (avoid sweeping a concurrent burst's partial
scratch file). Analysis:

- The factory-artifacts worktree is DEDICATED to `.factory/` pipeline state. It has no
  "scratch files" from other processes — its entire purpose is durable factory state.
- The factory-lock mechanism (ADR-025) serializes writers at the session level. Under normal
  operation, only ONE session is writing to `.factory/` at a time.
- A concurrent burst's partial file in `.factory/` is itself uncommitted state worth preserving
  on compaction — committing it is recoverable (can be amended or reverted); LOSING it is not.
- **Durability yields over orthogonality when the failure mode is unrecoverable data loss.**

**Adjudicated decision (production-grade default):**

Stage with `git -C <wt> add -A` (or equivalently `git -C <wt> add .`). This ensures:
1. All tracked modifications are staged (same as `add -u`).
2. All untracked NEW `.factory/` files are staged (the `add -u` gap is closed).
3. All tracked deletions are staged (same as `add -u`).

**INV5 and empty-commit interaction:**

`git -C <wt> add -A` followed by `git -C <wt> diff --cached` correctly detects:
- A flush with ONLY a new untracked file: `add -A` stages it → diff shows change → commit proceeds.
- A flush with NO changes at all: `add -A` produces nothing staged → diff is empty → INV5
  clean-state → exit 0. The INV5 gate behavior is unchanged.

**EC-008 documented outcome:**

EC-008 is reclassified: "Concurrent burst partial-file risk: assessed LOW. The factory-artifacts
worktree is a dedicated pipeline state directory. Partial files from a concurrent burst are
themselves pipeline state — staging them on PreCompact flush is preferable to silently losing
them. The factory-lock mechanism prevents typical concurrent-write scenarios. `git add -A` is
the production-grade default."

**Updated canonical execution order (step 5):**

Replace `git -C <wt> add -u` with `git -C <wt> add -A` in §Decision 5 (canonical execution
order) and all downstream AC references.

**Red Gate requirement (F-R3-003):**

```
test_flush_stages_new_untracked_factory_file
File: crates/hook-plugins/precompact-flush/tests/integration.rs
AC: AC-004 (staging), AC-011 step 5
Description: Fixture creates a new untracked file in the factory-artifacts worktree
  (e.g., .factory/new-spec-file.md). PreCompact event fires. Assert: the new file
  IS included in the flush commit (git log --name-only shows the new file).
Red Gate: Compile error; would fail if git add -u used (new file would be omitted).
```

---

### Decision 16 (new v1.3) — `renew_lock()` NoOp on byte-identical `expires_at` (F-R3-005)

**Problem:**

`renew_lock()` as specified in v1.2 computes `expires_at = now + 2700s` and writes the updated
content unconditionally when the lock is held, returning `RenewOutcome::Renewed(content)`. If
two PreCompact flushes fire within the same wall-clock second, both compute the same `expires_at`
value (same `T` = `SystemTime::now()` truncated to second precision). The second flush writes
back a `STATE.md` with a byte-identical `expires_at` field, returning `Renewed(content)` —
which causes the flush to proceed with a commit even though the content is unchanged. This
violates INV3a's intent ("renewal makes the commit non-empty").

**Decision (F-R3-005 — NoOp-on-identical):**

`renew_lock()` MUST compare the recomputed `expires_at` string with the EXISTING `expires_at`
string from the parsed `LockState`. If they are byte-identical (same string, same second),
return `Ok(RenewOutcome::NoOp)` — do NOT write `write_file` and do NOT return `Renewed`.

```rust
let new_expires_at = format_expires_at(SystemTime::now());  // YYYY-MM-DDTHH:MM:SSZ

// Only renew if expires_at actually changes.
if new_expires_at == held.expires_at {
    return Ok(RenewOutcome::NoOp);
}

// Proceed with update and return Renewed.
let new_content = update_expires_at(state_md_content, &new_expires_at);
Ok(RenewOutcome::Renewed(new_content))
```

**Updated INV3a:**

> `renew_lock()` returns `RenewOutcome::Renewed(content)` IFF the lock was held AND the newly
> computed `expires_at` DIFFERS from the existing `expires_at` (byte-comparison). If the lock
> was held but the recomputed value is byte-identical (same wall-clock second), returns
> `RenewOutcome::NoOp` — treating the renewal as a no-op. This ensures `RenewOutcome::Renewed`
> is a RELIABLE signal that STATE.md content actually changed.

**Why NoOp-on-identical (not soften-INV3a):**

The cleaner contract: `RenewOutcome::Renewed` always means "STATE.md was actually modified;
the flush commit will be non-empty due to the renewal." This allows callers to use `Renewed`
as a reliable "STATE.md changed" indicator without checking diff --cached again. The alternative
(softening INV3a to "Renewal MAY make commit non-empty") would require callers to re-check
the diff even after `Renewed` — which is a more complex caller contract.

**Red Gate requirement (F-R3-005):**

```
test_renew_lock_identical_expires_at_returns_noop
File: crates/factory-lock/tests/renew.rs
Input: STATE.md with factory_lock block where expires_at = now + 2700s (already renewed in
  the same second — simulated by setting expires_at to the exact value renew_lock would compute).
Expected: Ok(RenewOutcome::NoOp) — no write_file call; no content change.
Red Gate: Compile error; would fail if Renewed(content) returned for byte-identical expires_at.
```

---

### Decision 17 (new v1.3) — Wrong-branch Red Gate (F-R3-006)

**Gap:** Decision 3 (uniform `git -C` mandate) is the highest-blast-radius invariant in this
ADR — accidentally running git commands without `-C` on the main repo (develop) would commit
to the wrong branch, corrupt the SHA guard, or corrupt develop's history. Decision 3 has prose
enforcement and unit-test-level coverage in `crates/hook-plugins/precompact-flush/tests/integration.rs`,
but no integration-level bats test that asserts "develop HEAD did NOT move."

**Required Red Gate (F-R3-006):**

Add to S-18.04a Red Gate Test Table (bats integration level):

```
test_flush_does_not_advance_main_repo_head
File: plugins/vsdd-factory/tests/precompact-flush-native.bats
AC: AC-004, AC-005, AC-009 (uniform git -C; staging, commit, push all target factory-artifacts)
Description:
  (a) Capture DEVELOP_HEAD_BEFORE = git rev-parse HEAD (in the main repo / project dir,
      i.e., without -C) BEFORE dispatching the PreCompact event.
  (b) Dispatch PreCompact event through the dispatcher with the native precompact-flush.wasm.
  (c) Assert: git -C <wt> rev-parse HEAD (factory-artifacts) ADVANCED (new commit present).
  (d) Assert: git rev-parse HEAD (main repo, develop) == DEVELOP_HEAD_BEFORE (UNCHANGED).
Red Gate: Plugin not yet compiled. Would fail if any git subprocess in the plugin omits -C
  and accidentally commits to develop.
```

This test is a structural guard against regression of the uniform-git-C invariant: it is
falsifiable by the most dangerous class of bug (wrong-repo commit) and cannot be trivially
circumvented by a no-op implementation.

---

### Decision 18 (new v1.3) — Committer identity not validated against lock holder (F-R3-007)

**Assessment:**

The plugin does NOT validate that the git committer identity (`user.name` / `user.email`,
resolved from `HOME/.gitconfig` per Decision 10) matches the factory-lock `holder` field in
STATE.md. This is an INTENTIONAL design choice:

- **The factory-lock serializes single-session access.** Under normal operation, only the
  session that acquired the lock performs PreCompact flushes. Committer identity and lock holder
  are the same operator by construction.
- **The plugin's role is durability, not identity audit.** The flush's purpose is to commit
  STATE.md changes before compaction. Validating committer==holder would require parsing the
  holder identity (a human-readable string, not a git identity) and cross-referencing it with
  git's resolved committer — a fragile pattern (operator may use different name formats in
  different contexts).
- **`git commit` itself provides identity validation.** If no committer identity is configured
  (Decision 10 precondition not met), `git commit` exits non-zero and the flush fails safely
  (exit 2 per AC-005b).

**Documented BC precondition note:**

Add to BC-7.07.001 §Precondition section (or §Invariants):
> The plugin does NOT validate that the git committer identity matches the factory-lock `holder`
> field. This is acceptable because: (a) the factory-lock mechanism serializes single-session
> access — the committer IS the lock holder by construction; (b) committer identity is validated
> at the OS/git level (global gitconfig precondition per Decision 10); (c) committer-vs-holder
> cross-referencing is fragile and out of scope for a durability hook.

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

### (a-v1.2) Product-owner: BC-7.07.001 amendments (v1.2 additions)

These amendments supplement the v1.1 amendments (A1–A6) already applied in BC-7.07.001 v1.16.

**Amendment A7 — Traceability: §Decision references corrected (consistency ISSUE-5/6)**

In the BC-7.07.001 Traceability table, the ADR rows for §Decision 3/4/5 cite the ADR-028
decision by their NOW-SHIFTED numbers. In ADR-028 v1.2, the numbering is:

- §Decision 3 = Uniform `git -C <worktree>` (formerly "Decision 3 — Uniform git -C"; unchanged)
- §Decision 4 = Shared crate `crates/factory-lock` (formerly "Decision 3 — Code structure"; renumbered in v1.0)
- §Decision 5 = Native renew precedes git add (formerly "Decision 4 — Canonical execution order"; renumbered in v1.0)

The BC-7.07.001 Traceability §ADR row currently reads:
```
ADR-028 §Decision 3 (uniform git -C) | AC-004, AC-005, AC-006, AC-008, AC-009
ADR-028 §Decision 4 (shared crate crates/factory-lock) | AC-018, Architecture Mapping
ADR-028 §Decision 5 (native renew precedes git add) | AC-011 step 4
```

Verify these row §Decision numbers match the ADR-028 v1.2 numbering above (§Decision 3/4/5).
If any row cites a stale number (e.g., "§Decision 2" for shared-crate or "§Decision 3" for
renew-precedes-add from the original v1.0 before the renumbering), update to the numbers
above. Use stable anchor form `ADR-028 §Decision N (description)` per TD-VSDD-091
(POLICY 19 anti-volatile-pin — no file:line cites).

Also update the Traceability "Architecture Module" row:
- Current: "SS-07 (Hook Bash Layer) — ... ADR-028 native stanza"
- No change needed: the module text is already behavior-anchored, not version-pinned.

**Amendment A8 — PC3 + EC-004: factory_lock: presence pre-check (F-NW2-006)**

Update BC-7.07.001 PC3 to reflect the `factory_lock:` presence pre-check in `renew_lock()`:

> `crates/factory-lock::renew_lock()` performs a `factory_lock:` key presence pre-check before
> invoking the full fence-aware parser. If `factory_lock:` is NOT present in the frontmatter
> (regardless of fence shape), `renew_lock()` returns `Ok(RenewOutcome::NoOp)` immediately —
> this matches bash `factory-lock-write.sh renew`'s behavior of silently exiting 0 when the
> lock key is absent. `Err(LockError::Malformed)` is returned ONLY when `factory_lock:` IS
> present but the block is malformed. The hook caller treats `Err` as advisory (fail-open per
> §EC-004). `Ok(RenewOutcome::NoOp)` is returned both for lock-absent and for
> lock-key-absent-with-malformed-fence, preserving bash parity.

Update EC-004 to add:
> Note: `renew_lock()` returns `Err(Malformed)` ONLY when the `factory_lock:` key IS present
> but the block is malformed. If the key is absent (even in malformed-fence frontmatter),
> `renew_lock()` returns `Ok(RenewOutcome::NoOp)` — distinguishable from `Err(Malformed)`.

**Amendment A9 — PC4 / Discovery failure: LOUD advisory message format (F-NW2-009)**

In BC-7.07.001 PC4, add:

> If worktree discovery fails — either because the `git worktree list --porcelain` subprocess
> exits non-zero (environment/git error) OR because the output contains no stanza with
> `branch refs/heads/factory-artifacts` (configuration error) — the plugin exits 0 (fail-open
> per `on_error = continue`) but MUST write a DURABILITY DEGRADED advisory to stderr:
>
> - Git command failed: `precompact-flush: DURABILITY DEGRADED — git worktree list command
>   failed (exit <N>); factory-artifacts worktree cannot be discovered; flush SKIPPED this
>   compaction event. Check PATH/git configuration.`
> - Factory-artifacts not found: `precompact-flush: DURABILITY DEGRADED — factory-artifacts
>   branch not found in git worktree list output; flush SKIPPED this compaction event. Ensure
>   the factory-artifacts worktree is mounted at .factory/ (run: git worktree add .factory
>   factory-artifacts).`
>
> The LOUD advisory (uppercase DURABILITY DEGRADED prefix) distinguishes this from a normal
> no-op (clean state) and ensures a human reading session logs can identify a silently-disabled
> durability session.

**Amendment A10 — INV3 step 3: RenewOutcome::NoOp vs Renewed precedence (F-NW2-007)**

Update BC-7.07.001 INV3 step 3 / INV3a to reflect Decision 11:

> (3) check `factory_lock:` block — if `renew_lock()` returns `Ok(RenewOutcome::NoOp)` (key
> absent OR lock null/absent): skip step 4 (no write_file call; no-op); proceed to step 5.
> If `Ok(RenewOutcome::Renewed(content))`: proceed to step 4 (write renewed STATE.md).
> If `Err(Malformed)`: advisory warn; skip step 4; proceed to step 5.
>
> (5) `git -C <wt> add -u` — stages only tracked files. IMPORTANT: if step 3 returned
> `RenewOutcome::NoOp` AND step 5 shows no staged changes (`git -C <wt> diff --cached`
> is empty), the plugin MUST apply INV5 (exit 0; no empty commit) rather than forcing a
> commit. The empty-commit guard checks post-add staging, not pre-renewal staging.

Update INV3a:
> `crates/factory-lock::renew_lock()` returns `RenewOutcome::Renewed(content)` ONLY when the
> lock was held AND `expires_at` was actually updated. If the lock was absent (returns
> `RenewOutcome::NoOp`), no write_file call is made and the staging area is unchanged by the
> renewal step. Therefore "renew makes the flush commit non-empty" is conditional: it applies
> ONLY when `RenewOutcome::Renewed` — not when `NoOp`. INV5 must be evaluated after `git add`
> using `git diff --cached`, not before.

Add canonical test vector to §Canonical Test Vectors table:
```
| Input: STATE.md readable; factory_lock: absent; no pending factory-artifacts changes
| Expected: exit 0; no commit; no precompact-flush-log line appended (NoOp + clean state)
| Category: no-lock-clean-state
```

**Amendment A11 — AC-018 path-based form struck (F-NW2-005)**

In BC-7.07.001 PC4 (the precondition text and Architecture Anchors), replace any reference to
`crates/factory-lock::renew_lock(&factory_artifacts_path)` or `renew_lock(&Path)` with the
pure content form:

> The renewal function signature is `pub fn renew_lock(state_md_content: &str) -> Result<RenewOutcome, LockError>`.
> The PLUGIN owns the `host::read_file` and `host::write_file` calls; `crates/factory-lock` is
> a pure library with no `std::fs` or I/O dependencies. This makes `crates/factory-lock`
> WASM-hermetic (no filesystem syscalls inside the crate itself).

---

### (b-v1.2) Story-writer: S-18.04a amendments (v1.2 additions)

These amendments supplement the v1.1 amendments (B1–B6) already applied in S-18.04a v1.9.

**Amendment B7 — AC-011: fix bare git commands in canonical order summary (consistency ISSUE-2)**

In AC-011 §canonical execution order steps 5, 6, 7, 10, the summary list uses bare `git` without
`-C <wt>`. The correct forms are:

```
Step 5:  exec_subprocess("git", &["-C", &wt, "add", "-u"])                              — was: exec_subprocess("git", &["add", "-u"])
Step 6:  exec_subprocess("git", &["-C", &wt, "commit", "-m", msg])                      — was: exec_subprocess("git", &["commit", "-m", msg])
Step 7:  SHA_B = exec_subprocess("git", &["-C", &wt, "rev-parse", "HEAD"])               — was: exec_subprocess("git", &["rev-parse", "HEAD"])
Step 10: exec_subprocess("git", &["-C", &wt, "push", "origin", "factory-artifacts"])    — was: exec_subprocess("git", &["push", "origin", "factory-artifacts"])
```

Replace all four bare-git forms with the `-C <wt>` forms above. The existing note at the bottom
of AC-011 ("ALL git subprocesses except the initial `git worktree list --porcelain` MUST use
`-C <wt>`") remains correct as-is; the summary steps must now also be consistent with it.

**Amendment B8 — Red Gate row: replace phantom F-NW-010 label with stable anchor (consistency ISSUE-4)**

In the Red Gate Test Table, the final row currently has:

```
| `test_flush_completes_via_git_only_positive` | ... | BC-7.07.001 §INV3 F-NW-010 (POSITIVE flush completion ...) |
```

The label `F-NW-010` is a phantom (no corresponding finding was formally numbered F-NW-010 in
ADR-028 v1.0 or v1.1). Replace the BC clause text with a stable behavioral anchor:

```
BC-7.07.001 §INV3 (positive-flush-completion: assert BOTH (a) commit SHA exists on
factory-artifacts AND (b) precompact-flush-log entry with that SHA is appended; NOT merely
absence-of-bash-subprocess; ADR-028 §Decision 3 F-NW-001 env_allow + §Decision 8 bats non-tautological)
```

Do NOT use `F-NW-010` anywhere in the spec. Use the stable `§INV3 positive-flush-completion`
anchor instead.

**Amendment B9 — AC-017: LOUD advisory message format (F-NW2-009)**

In AC-017, update the failure-mode advisory text:

```
If the git worktree list command exits non-zero:
  Write to stderr: "precompact-flush: DURABILITY DEGRADED — git worktree list command failed
  (exit <N>); factory-artifacts worktree cannot be discovered; flush SKIPPED this compaction
  event. Check PATH/git configuration." Exit 0 (fail-open).

If no stanza with branch refs/heads/factory-artifacts is found:
  Write to stderr: "precompact-flush: DURABILITY DEGRADED — factory-artifacts branch not
  found in git worktree list output; flush SKIPPED this compaction event. Ensure the
  factory-artifacts worktree is mounted at .factory/ (run: git worktree add .factory
  factory-artifacts)." Exit 0 (fail-open).
```

Replace the prior single-message advisory format in AC-017 with these two distinct messages.
The DURABILITY DEGRADED prefix (uppercase) is mandatory.

**Amendment B10 — AC-018: path-based form struck; RenewOutcome enum added (F-NW2-005)**

Replace the AC-018 renew_lock() call form `crates/factory-lock::renew_lock(&factory_artifacts_path)`
with:

> `crates/factory-lock::renew_lock(state_md_content: &str)` — pure content-in/content-out.
> The plugin reads STATE.md via `host::read_file` (→ `state_md_content`), calls `renew_lock()`,
> and based on the result:
> - `Ok(RenewOutcome::NoOp)` → no write_file call; STATE.md unchanged; proceed to git add.
> - `Ok(RenewOutcome::Renewed(new_content))` → call `host::write_file(".factory/STATE.md", new_content)`;
>   proceed to git add.
> - `Err(LockError::Malformed)` → advisory warn to stderr; no write_file call; proceed to git add
>   (fail-open per EC-004).

Remove any text suggesting a path-based `renew_lock(&Path)` form.

**Amendment B11 — AC-007: absent log read → empty baseline (F-NW2-008)**

In AC-007, add:

> If `host::read_file` on `.factory/hooks/precompact-flush-log` returns an error (including
> `CAPABILITY_DENIED` when the file does not yet exist — `read_file` requires the file to exist
> for `canonicalize()` to succeed), the plugin MUST treat the error as EMPTY prior content
> (`""`). The new entry is concatenated to `""` and written via `host::write_file` (which
> supports creating new files per BC-2.02.011 EC-005). This is NOT a failure; it is the
> intended first-flush behavior. Do NOT treat a `read_file` error on the log path as an append
> failure (which would trigger the SHA-pinned reset guard).

**Amendment B12 — AC-011: RenewOutcome::NoOp + INV5 interaction (F-NW2-007)**

In AC-011 step 3, update:

> Check `factory_lock:` block via `renew_lock(state_md_content)`:
> - `Ok(RenewOutcome::NoOp)` → skip step 4 (no write_file); proceed to step 5 (git add)
> - `Ok(RenewOutcome::Renewed(content))` → proceed to step 4 (write_file with content)
> - `Err(Malformed)` → advisory warn; skip step 4; proceed to step 5

In AC-011 step 5 note, add:

> After `git -C <wt> add -A`, check for staged changes via `git -C <wt> diff --cached`. If
> there are no staged changes AND step 3 returned `RenewOutcome::NoOp`, apply INV5 (exit 0
> clean-state; do NOT commit). If step 3 returned `RenewOutcome::Renewed`, the renewal write
> guarantees at least the STATE.md change is staged; INV5 does not apply.

**Amendment B13 — New Red Gate rows (F-NW2-004/006/007/008/009)**

Add to the Red Gate Test Table:

| Test name | File | AC | BC clause | Red Gate condition |
|-----------|------|----|-----------|-------------------|
| `test_git_commit_succeeds_with_global_identity_via_home` | `plugins/vsdd-factory/tests/precompact-flush-native.bats` | AC-005, AC-017 | ADR-028 §Decision 10 F-NW2-004 (committer identity via HOME/.gitconfig; GIT_AUTHOR/COMMITTER vars absent from env_allow) | Plugin not yet compiled; test fixture not yet implemented |
| `test_renew_lock_malformed_fence_no_lock_key_returns_noop` | `crates/factory-lock/tests/renew.rs` | AC-018 | ADR-028 §Decision 9 F-NW2-006 (malformed fence + no factory_lock key → Ok(NoOp); bash parity) | Compile error; would fail if Err(Malformed) returned |
| `test_no_lock_clean_state_exits_0_no_commit` | `crates/hook-plugins/precompact-flush/tests/integration.rs` | AC-005, AC-011, AC-018 | ADR-028 §Decision 11 F-NW2-007 (RenewOutcome::NoOp + clean state → exit 0, no commit forced) | Compile error; would fail if empty commit created |
| `test_first_flush_with_absent_log_appends_successfully` | `crates/hook-plugins/precompact-flush/tests/integration.rs` | AC-007 | ADR-028 §Decision 12 F-NW2-008 (read_file CAPABILITY_DENIED on absent log → empty baseline; log created with one entry) | Compile error; would fail if read_file error treated as append failure |
| `test_worktree_discovery_failure_emits_durability_degraded` | `crates/hook-plugins/precompact-flush/tests/integration.rs` | AC-017 | ADR-028 §Decision 13 F-NW2-009 (discovery failure → exit 0 + stderr DURABILITY DEGRADED prefix; NOT silent) | Compile error; would fail if advisory omitted or quiet |

---

### (a-v1.3) Product-owner: BC-7.07.001 amendments (v1.3 additions)

These amendments supplement the v1.2 amendments (A1–A11) already applied in BC-7.07.001 v1.17.
They address the round-3 adversarial review findings.

**Amendment A12 — PC4 + Discovery startup assertion: canonicalize check (F-R3-001)**

Add to BC-7.07.001 §Precondition 4 (after the LOUD advisory text added in A9):

> After discovering the factory-artifacts worktree absolute path via `git worktree list
> --porcelain`, the plugin MUST perform a startup canonicalize assertion before proceeding:
>
> ```
> canonicalize(discovered_worktree_abs_path) == canonicalize(ctx.cwd + "/.factory")
> ```
>
> If they MATCH: proceed normally (Domain A `.factory/`-relative host I/O and Domain B git
> `-C <discovered>` are consistent — same filesystem location).
>
> If they MISMATCH: emit to stderr:
> `precompact-flush: DURABILITY DEGRADED — factory-artifacts worktree path mismatch: discovered
> <discovered_path> but expected <cwd>/.factory; flush SKIPPED to prevent split-tree data loss.
> Ensure factory-artifacts is mounted at .factory/ (run: git worktree add .factory factory-artifacts).`
> Exit 0 (fail-open — do NOT proceed with a split-tree flush).
>
> This assertion makes the canonical invariant (factory-artifacts ALWAYS at `<cwd>/.factory`)
> mechanically enforced at runtime, not just documented in the ADR.

**Amendment A13 — PC4 + INV3 step 5: staging strategy (F-R3-003 adjudication)**

Replace all references to `git -C <wt> add -u` in BC-7.07.001 with `git -C <wt> add -A`:

In §INV3 step 4 (formerly step 5 in AC-011 numbering), update:
> (4) `git -C <wt> add -A` — stages ALL changes in the factory-artifacts worktree: tracked
> modifications, tracked deletions, AND new untracked files. This ensures that new `.factory/`
> files created during the session (but not yet tracked by git) are captured in the flush commit
> and not silently lost on compaction. The factory-artifacts worktree is a dedicated pipeline
> state directory; staging all its changes is the production-grade default.
>
> Note: `git add -A` differs from `git add -u` in that it includes untracked files. EC-008
> (concurrent burst partial-file risk) is reclassified as LOW: the factory-lock mechanism
> serializes typical usage; a partial file from a concurrent burst is itself pipeline state
> worth preserving (committable partial > unrecoverably lost).

Add a canonical test vector to §Canonical Test Vectors:
```
| Input: factory-artifacts worktree has one new untracked .factory/ file; no STATE.md lock
| Expected: flush commit includes the new file; exit 0; git log --name-only shows the file
| Category: new-untracked-capture
```

**Amendment A14 — INV3a + PC3: renew_lock NoOp-on-identical (F-R3-005)**

Update BC-7.07.001 INV3a:

> `crates/factory-lock::renew_lock()` returns `RenewOutcome::Renewed(content)` IFF the lock
> was held AND the recomputed `expires_at` string DIFFERS from the existing `expires_at` value
> (byte-comparison). If the recomputed value is byte-identical (same wall-clock second), the
> function returns `Ok(RenewOutcome::NoOp)` — no STATE.md write is performed. This ensures
> `RenewOutcome::Renewed` is a reliable signal that STATE.md content actually changed.

Update PC3 (add a note after the NoOp mapping):
> Note: `renew_lock()` returns `Ok(RenewOutcome::NoOp)` in two cases: (a) lock is absent or
> lock key is absent (pre-check returns false), and (b) lock is held but recomputed
> `expires_at` is byte-identical to the existing value (same wall-clock second).

**Amendment A15 — Preconditions: committer-identity note (F-R3-007)**

Add a note to BC-7.07.001 §Precondition or §Notes (not a blocking precondition — an informational
note):

> **Committer identity:** The plugin does NOT validate that the git committer identity matches
> the factory-lock `holder` field. This is acceptable because the factory-lock mechanism
> serializes single-session access — the session that holds the lock is the session performing
> the flush, so committer and lock-holder are the same operator by construction. If no committer
> identity is configured (see §Decision 10 in ADR-028), `git commit` exits non-zero and the
> flush fails safely (exit 2 per §AC-005b).

---

### (b-v1.3) Story-writer: S-18.04a amendments (v1.3 additions)

These amendments supplement the v1.2 amendments (B1–B13) already applied in S-18.04a v1.10.

**Amendment B14 — depends_on: add S-18.04a-prereq (CV-006)**

In S-18.04a's `depends_on` frontmatter array, add `S-18.04a-prereq`:

```yaml
depends_on:
  - S-18.00
  - S-17.04
  - S-18.04a-prereq   # ADD: write_file.rs facade fix + BC-2.02.011 §Inv3 + bats equal-roots
```

Rationale: `S-18.04a-prereq` blocks S-18.04a per ADR-028 §Decision 8 (the bats equal-roots fix
is a prerequisite for S-18.04a's non-tautological Red Gate). The STORY-INDEX catalog row
(line 662) already shows `depends_on: [S-18.00, S-17.04, S-18.04a-prereq]` — but the story
file frontmatter was not updated in the D-671 burst. This is the authoritative fix.

**Amendment B15 — AC-004: staging command (F-R3-003)**

Replace `git -C <wt> add -u` with `git -C <wt> add -A` in AC-004:

> `exec_subprocess("git", &["-C", &wt, "add", "-A"])` — stage ALL changes (tracked
> modifications + deletions + new untracked `.factory/` files). NOT `add -u` (which omits
> new untracked files and risks silent data loss on compaction when new files are the only
> pending change).

**Amendment B16 — AC-011 step 5: staging command (F-R3-003)**

In AC-011's canonical execution order summary, step 5 already reads:
`exec_subprocess("git", &["-C", &wt, "add", "-u"])` (corrected from bare git in B7).
Replace with:
`exec_subprocess("git", &["-C", &wt, "add", "-A"])`.

**Amendment B17 — AC-017 (v1.3): startup canonicalize assertion (F-R3-001)**

After the existing AC-017 worktree discovery and LOUD advisory text, add:

> After confirming the factory-artifacts stanza is found, perform the startup canonicalize check:
> ```
> let expected = Path::new(&ctx.cwd).join(".factory").canonicalize()?;
> let discovered = Path::new(&discovered_abs_path).canonicalize()?;
> if discovered != expected {
>     emit_durability_degraded_mismatch(&discovered_abs_path, &ctx.cwd);
>     return exit_0();
> }
> // paths match — proceed
> ```
> If canonicalize fails on either path (path does not exist), treat as a MISMATCH and emit the
> DURABILITY DEGRADED advisory. Exit 0 (fail-open).

**Amendment B18 — Red Gate rows (F-R3-001/002/003/005/006)**

Add to the Red Gate Test Table:

| Test name | File | AC | BC clause | Red Gate condition |
|-----------|------|----|-----------|-------------------|
| `test_path_mismatch_emits_durability_degraded_and_exits_0` | `crates/hook-plugins/precompact-flush/tests/integration.rs` | AC-017 | ADR-028 §Decision 14 F-R3-001 (canonicalize mismatch → DURABILITY DEGRADED + exit 0; NOT split-tree flush) | Compile error; would fail if flush proceeds on path mismatch |
| `test_has_factory_lock_key_open_fence_with_lock_key_returns_true` | `crates/factory-lock/tests/renew.rs` | AC-018 | ADR-028 §Decision 14 F-R3-002 (open fence + factory_lock: present → Err(Malformed); bash parity) | Compile error; would fail if Ok(NoOp) returned |
| `test_flush_stages_new_untracked_factory_file` | `crates/hook-plugins/precompact-flush/tests/integration.rs` | AC-004, AC-011 | ADR-028 §Decision 15 F-R3-003 (git add -A stages new untracked .factory/ files; add -u would miss them) | Compile error; would fail if git add -u used |
| `test_renew_lock_identical_expires_at_returns_noop` | `crates/factory-lock/tests/renew.rs` | AC-018 | ADR-028 §Decision 16 F-R3-005 (byte-identical expires_at → Ok(NoOp); no write_file) | Compile error; would fail if Renewed returned for identical expires_at |
| `test_flush_does_not_advance_main_repo_head` | `plugins/vsdd-factory/tests/precompact-flush-native.bats` | AC-004, AC-005, AC-009 | ADR-028 §Decision 17 F-R3-006 (uniform git -C; develop HEAD unchanged after flush; factory-artifacts HEAD advanced) | Plugin not yet compiled; would fail if any git subprocess omits -C |

---

### (b-v1.3 prereq) Story-writer: S-18.04a-prereq amendments (v1.3 additions)

**Amendment P1 — STORY-INDEX title alignment (CV-001)**

The STORY-INDEX catalog row for S-18.04a-prereq (line 661) uses the title:
`"write_file cwd alignment prereq — write_file.rs facade fix to cwd-rooted invocation + BC-2.02.011 staleness fix + bats equal-roots de-masking"`

The story file H1 (POLICY 7, H1 is authoritative) is:
`"S-18.04a-prereq — write_file.rs cwd alignment + BC-2.02.011 §Inv3 + bats equal-roots fix"`

The catalog title column must match the H1 (after the story ID prefix). The STORY-INDEX title
cell should read verbatim:
`write_file.rs cwd alignment + BC-2.02.011 §Inv3 + bats equal-roots fix`

This is state-manager domain (STORY-INDEX edit), not story-writer domain — routed to state-manager
as CV-001 below. Story-writer confirms the H1 in the prereq story file is authoritative and
does NOT need to change.

---

### (c-v1.3) State-manager: index parity drifts (v1.3 additions)

**Index fix SM-6 — STORY-INDEX S-18.04a-prereq depends_on column: remove spurious S-17.04 (CV-004)**

STORY-INDEX line 661 currently shows `depends_on: [S-18.00, S-17.04]` for S-18.04a-prereq.
The story file frontmatter `depends_on` has only `[S-18.00]`. S-17.04 is spurious — S-18.04a-prereq
does NOT depend on S-17.04 (it only needs S-18.00 for PreCompact event type routing). Remove
`S-17.04` from the STORY-INDEX catalog row's depends_on column for S-18.04a-prereq:

Current: `| [S-18.00, S-17.04] |`
Correct:  `| [S-18.00] |`

**Index fix SM-7 — STORY-INDEX S-18.04a-prereq title column: verbatim H1 (CV-001)**

STORY-INDEX line 661 title column currently reads:
`write_file cwd alignment prereq — write_file.rs facade fix to cwd-rooted invocation + BC-2.02.011 staleness fix + bats equal-roots de-masking`

The story file H1 (authoritative per POLICY 7) is:
`S-18.04a-prereq — write_file.rs cwd alignment + BC-2.02.011 §Inv3 + bats equal-roots fix`

The title column should use the H1 text AFTER the story ID prefix. Update to:
`write_file.rs cwd alignment + BC-2.02.011 §Inv3 + bats equal-roots fix`

**Index fix SM-8 — ARCH-INDEX ADR-028 row version cell (v1.3 update)**

Update the ARCH-INDEX Architecture Decisions table ADR-028 row version reference from `v1.2`
to `v1.3`. Also update ARCH-INDEX `version`, `last_amended`, and `changelog` with a v2.64
entry per POLICY 14 5-leg parity. The changelog entry should read:
"v2.64 — ADR-028 row updated v1.2→v1.3 (architect: F-R3-001 non-standard-mount hazard —
startup canonicalize assertion + DURABILITY DEGRADED mismatch advisory; F-R3-002
has_factory_lock_key open-fence+lock-key=Err(Malformed) bash parity; F-R3-003 staging
adjudicated → git add -A (durability > EC-008); F-R3-005 renew_lock NoOp on byte-identical
expires_at; F-R3-006 wrong-branch Red Gate; F-R3-007 committer==holder documented; CV-002
Decision 4 forward-ref to Decision 9). No new capability/ABI/release-blocker."
The version bump is state-manager domain.

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

**Index fix SM-4 — ARCH-INDEX ADR-028 row version cell (v1.2 update)**

The ARCH-INDEX Architecture Decisions table currently shows ADR-028 at version `v1.1` (per the
v2.62 update from D-670). Update the row's version reference to `v1.2` in the ARCH-INDEX table
to reflect this v1.2 amendment. Also update ARCH-INDEX `version`, `last_amended`, and
`changelog` with a v2.63 entry per POLICY 14 5-leg parity. The changelog entry should read:
"v2.63 — ADR-028 row updated v1.1→v1.2 (architect: F-NW2-001..009 consolidated design-fix pass;
write_file.rs path-domain false-blocker corrected; PREREQUISITE micro-story S-18.04a-prereq
specified; renew_lock() pure content signature; malformed-fence parity; empty-commit precedence;
read_file absent-as-empty; LOUD advisory; consistency fixes A7/B7/B8)."
The version bump is state-manager domain, not architect domain.

**Note on ARCH-INDEX v2.62 historical changelog body (consistency ISSUE-1):**

The D-670 burst (v2.62) historical body in ARCH-INDEX already cites `BC-INDEX v3.26 / STORY-INDEX
v4.43 / ARCH-INDEX v2.62` — this was the correction applied in D-670 and is ALREADY correct.
No further correction to the v2.62 historical entry is needed.

**Index fix SM-5 — PREREQUISITE micro-story S-18.04a-prereq registration**

State-manager adds a new story catalog row to STORY-INDEX for `S-18.04a-prereq`:

```
| S-18.04a-prereq | write_file.rs cwd alignment + BC-2.02.011 §Inv3 + bats equal-roots fix |
  PENDING | 3 pts | SS-01,SS-02 | blocks: S-18.04a | ADR-028 §Decision 8 |
```

Story ID is `S-18.04a-prereq`. It blocks S-18.04a. Story-writer creates the full story file;
this catalog row is state-manager's domain (index update). The story is 3 points, track:
maintenance, phase: F3 (or feature depending on current cycle).

---

## Consequences

### Positive

- **Bash dependency eliminated from PreCompact hook.** The `precompact-flush` plugin's `binary_allow`
  reduces from `["bash", "git"]` to `["git"]` only. This removes one surface of the shell-exec
  attack footprint in the WASM sandbox.
- **Runtime worktree discovery with startup assertion.** No hard-coded `.factory` assumption;
  the actual mount path is read from git's authoritative worktree registry at runtime and
  verified via canonicalize assertion against `<cwd>/.factory`. Non-standard mounts are detected
  at startup and blocked (DURABILITY DEGRADED advisory) before any split-tree I/O occurs.
- **Native renewal fidelity.** The Rust implementation of `renew_lock()` mirrors `factory-lock-write.sh
  renew` semantics precisely: preserve `holder` + `locked_at`; update `expires_at = now + 2700s`
  in EXACTLY `YYYY-MM-DDTHH:MM:SSZ` format; no-op when lock absent, lock key absent (Decision 9
  bash-parity pre-check), OR when recomputed expires_at is byte-identical (Decision 16 NoOp-on-
  identical); `Err` on malformed-AND-lock-present (library layer); fail-open advisory at hook
  caller layer.
- **Shared crate reusable.** `crates/factory-lock` can be used by future native WASM plugins
  needing lock operations. Pure content-in/content-out signature is WASM-hermetic (no std::fs).
- **Standing ADR-014 policy satisfied.** Native WASM migration closes the legacy-bash-adapter
  technical debt for the PreCompact hook.
- **Uniform git -C discipline.** All git subprocesses explicitly scoped to the factory-artifacts
  worktree; no risk of accidentally operating on the main repo.
- **No dispatcher release required.** Production write_file in invoke.rs is already cwd-rooted.
  Native WASM migration can proceed without a new dispatcher ABI version or release tag.
- **Durability degradation is visible.** LOUD advisory (DURABILITY DEGRADED prefix) on worktree
  discovery failure ensures silently-disabled flush sessions are detectable in logs.

### Negative / Trade-offs

- **PREREQUISITE micro-story (S-18.04a-prereq) required before S-18.04a.** Three mechanical
  fixes must land first: write_file.rs::resolve_for_write alignment, BC-2.02.011 invariant 3
  correction, and precompact-routing.bats equal-roots fix. Estimated 3 points; small but blocking.
- **Log-append is read-modify-write, not atomic.** `host::write_file` is a full overwrite. A
  theoretical race between `precompact-flush` and `precompact-flush-prune.sh` in concurrent
  sessions could lose a log entry. Mitigated by: factory-lock serializes typical usage; log is
  secondary to the commit; bounded data-loss window is acceptable. No dispatcher change required.
- **File mode not preserved.** `host::write_file` uses platform umask defaults, unlike the bash
  script's explicit `chmod`. Acceptable for STATE.md in a git worktree (git does not track mode
  for non-executable files).
- **New crate `crates/factory-lock` must be authored.** Frontmatter-boundary-aware YAML surgery
  in Rust requires careful parsing. Mitigation: unit tests over the full matrix of frontmatter
  shapes, including the factory_lock: key presence pre-check edge cases.
- **`bash factory-lock-write.sh renew` remains in production for skill callers.** Two code paths
  now implement renewal (bash script + Rust crate). They must remain semantically synchronized.
  Mitigation: the Rust `renew_lock()` unit tests use the same semantic table as the bash script.
- **Committer identity is a precondition, not a capability.** git commit fails with `Author
  identity unknown` if the operator's ~/.gitconfig is not configured. This is a deployment
  precondition, not a plugin bug; the fail-safe is correct (exit 2 blocks compaction).

---

## Risks Addressed

| Risk | Mitigation |
|------|-----------|
| Hard-coded `.factory` path breaks in non-standard worktree mounts | Decision 1 (v1.3): non-standard-mount rationale STRUCK; startup canonicalize assertion at runtime; MISMATCH → DURABILITY DEGRADED + exit 0 (no split-tree flush) |
| bash exec in WASM sandbox expands attack surface | Decision 2: `binary_allow = ["git"]` only; renewal is native Rust |
| git push fails silently due to missing PATH or SSH_AUTH_SOCK | Decision 1 (v1.1): `PATH` + `SSH_AUTH_SOCK` added to `env_allow`; Red Gate bats test with local bare remote |
| git subprocesses operating on main repo instead of factory-artifacts | Decision 3: uniform `git -C <wt>` on ALL git subprocesses; Decision 17 (v1.3): wrong-branch Red Gate (bats asserts develop HEAD unchanged) |
| Native renew diverges semantically from bash script renew | Decision 4 (v1.1): library returns Err on malformed (faithful); caller downgrades Err to advisory; Decision 9 (v1.2): bash parity for malformed-fence + no-lock-key case via presence pre-check; Decision 14 (v1.3): open-fence + lock-key present → Err(Malformed) (bash parity confirmed) |
| `expires_at` format diverges from bash output (e.g., chrono rfc3339 vs Z-suffix) | Decision 2 (v1.1): format pinned to `YYYY-MM-DDTHH:MM:SSZ`; Red Gate test for exact format |
| Log append race between flush and prune (concurrent sessions) | Decision 7: grounded-documentation; factory-lock serializes typical usage; no dispatcher change needed |
| Rust frontmatter surgery corrupts body content of STATE.md | Shared crate must be frontmatter-boundary-aware; unit tests include body-content edge cases and presence pre-check |
| `expires_at` not included in flush commit (ordering defect) | Decision 5: native renew MUST precede git add |
| No-frontmatter STATE.md causes unexpected advisory during renew | Decision 9 (v1.2): `has_factory_lock_key()` pre-check returns `Ok(NoOp)` for no-frontmatter; no parse attempted |
| Malformed-fence + no lock key emits spurious Err (diverges from bash parity) | Decision 9 (v1.2): presence pre-check catches factory_lock: absent before fence parse; Ok(NoOp) returned |
| Empty commit forced when RenewOutcome::NoOp + clean state | Decision 11 (v1.2): INV5 evaluated post-git-add; NoOp + no-staged-changes → exit 0 clean-state |
| First flush fails because read_file returns CAPABILITY_DENIED on absent log | Decision 12 (v1.2): read_file error on log path treated as empty baseline; write_file creates file |
| Silently-disabled durability goes undetected in session logs | Decision 13 (v1.2): LOUD advisory (DURABILITY DEGRADED prefix) on worktree discovery failure |
| git commit fails due to missing committer identity in env_clear sandbox | Decision 10 (v1.2): HOME (→ ~/.gitconfig) in env_allow provides committer identity; documented precondition |
| write_file.rs unit-test facade inconsistency hides production behavior | Decision 8 (v1.2): PREREQUISITE S-18.04a-prereq aligns write_file.rs::resolve_for_write to ctx.cwd; fixes BC-2.02.011 Inv3; fixes bats equal-roots masking |
| New untracked .factory/ file silently omitted from flush commit | Decision 15 (v1.3): `git add -A` staged all files including untracked; EC-008 reclassified LOW (committed partial > lost new) |
| renew_lock returns Renewed for byte-identical expires_at (spurious write) | Decision 16 (v1.3): NoOp-on-identical — renew_lock returns Ok(NoOp) when recomputed expires_at == existing value |
| Committer identity mismatch with lock holder causes compliance confusion | Decision 18 (v1.3): documented as acceptable; factory-lock serializes single-session; committer==holder by construction |

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
| canonicalize() failure on valid path (e.g., `.factory/` exists but is a symlink target) | LOW | Treat canonicalize Err as MISMATCH → DURABILITY DEGRADED; conservative fail-open; symlink factory-artifacts mounts are unsupported |
| `git add -A` stages unintended files from a concurrent burst's scratch writes | LOW | EC-008 reclassified; factory-artifacts is a dedicated pipeline-state directory; factory-lock serializes; staged partial > lost |

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
| Host source | `crates/factory-dispatcher/src/host/write_file.rs` | Unit-test facade: `prepare()` uses `plugin_root` (stale — to be aligned to `ctx.cwd` in S-18.04a-prereq); production path is `invoke.rs` (cwd-rooted) |
| Host source | `crates/factory-dispatcher/src/invoke.rs` | Production `write_file` impl (lines 746-800): resolves relative paths under `ctx.cwd = CLAUDE_PROJECT_DIR`; path_allow also rooted at cwd |
| Host source | `crates/factory-dispatcher/src/main.rs` | `base_host_ctx.cwd = ENV_PROJECT_DIR` (~line 303); `base_host_ctx.plugin_root` from `ENV_PLUGIN_ROOT` (~line 313) |
| Bash script | `plugins/vsdd-factory/bin/factory-lock-write.sh` lines 337-347 | Bash renew pre-checks `factory_lock:` key presence (awk) before fence-aware parse; silently exits 0 when key absent; authoritative semantic reference for `renew_lock()` parity |
| Bats test | `plugins/vsdd-factory/tests/precompact-routing.bats` line 216 | Equal-roots masking defect: `CLAUDE_PLUGIN_ROOT=$WORK CLAUDE_PROJECT_DIR=$WORK`; to be fixed in S-18.04a-prereq |
| Parse crate | `crates/factory-lock-parse/src/lib.rs` | `parse_factory_lock()` return taxonomy: `Ok(None)` / `Ok(Some)` / `Err(MalformedLockBlock)` |
| Subsystem | SS-04 | Plugin Ecosystem (WASM plugin crate; shared library crate) |
| Subsystem | SS-07 | Hook Bash Layer (registry stanza; migration from bash hook) |
| Feature | E-18 / issue #173 | Context-durability epic |
