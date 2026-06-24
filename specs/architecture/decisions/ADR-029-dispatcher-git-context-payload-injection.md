---
adr_id: ADR-029
title: "ADR-029: Dispatcher git-context payload injection for WASM chain-detection gates"
status: proposed
date: 2026-06-24
deciders:
  - architect
  - human (Option A approval)
supersedes: null
superseded_by: null
traces_to: .factory/specs/architecture/ARCH-INDEX.md
related_adrs:
  - ADR-028 (E-18 precompact-flush native WASM migration — established WASM-native hook pattern)
  - ADR-026 (wave-boundary checkpoint — defines PreCompact flush lifecycle and precompact-flush-log append-log)
  - ADR-002 (WASM plugin ABI — defines host function contract)
  - ADR-006 (host ABI version — governs HOST_ABI_VERSION bump policy)
subsystem: SS-04
---

# ADR-029: Dispatcher git-context payload injection for WASM chain-detection gates

## Status

Proposed — pending S-18.04b implementation.

## Context

`validate-burst-log` and `validate-dispatch-advance` (WASM hook plugins under
`crates/hook-plugins/`) implement the `MULTI_COMMIT_CHAIN_NOT_ALLOWED` detector
(TD-VSDD-053) and the PreCompact flush exemption (BC-5.41.003). Both plugins
currently call `host::exec_subprocess("git", ...)` inside `check_factory_artifacts_chain()`
to obtain HEAD and HEAD^ commit subjects and SHAs from the factory-artifacts worktree.

This is a correctness defect with two dimensions:

**Decoupling defect.** The chain check fires on PostToolUse Edit/Write events (file
writes) rather than on Bash git-commit events. A file write that is unrelated to a
commit will cause the WASM to exec git, read the CURRENT factory-artifacts HEAD (which
was committed earlier in the session, not "being committed now"), and evaluate that
stale HEAD/HEAD^ pair. The gate conflates "file was written" with "a commit just
occurred". This is structurally incorrect: the gate must evaluate the commit being made,
not an arbitrary prior HEAD.

**PC1 violation.** BC-5.41.003 PC1 specifies that "the WASM gate does NOT exec
`git cat-file -t`". By extension, BC-5.41.003 Architecture Anchors state "WASM does
NOT exec git; no bash equivalent exists". The current `check_factory_artifacts_chain()`
implementation uses `exec_subprocess("git", ...)` to obtain commit subjects and SHAs,
violating PC1. ADR-028 establishes WASM-native migration as the architectural direction
(E-18 alignment); exec-git-in-WASM contradicts this.

**Human decision.** The human approved Option A: keep chain-detection WASM-native
(consistent with ADR-028). The host dispatcher (allowed to exec git) injects commit
context into the hook payload as a `git_context` field. The WASM plugin reads
`git_context` from the payload instead of exeving git.

## Decision

### Decision 1: Trigger on PostToolUse Bash (git commit), not PostToolUse Edit/Write

The chain-detection gate MUST run on the PostToolUse event for a Bash `git commit` tool
call on the factory-artifacts worktree, NOT unconditionally on every Edit/Write
PostToolUse event.

Rationale: the chain detector evaluates the commit being made. A file write unrelated to
a commit must not trigger git inspection. The existing Edit/Write hooks
(`validate-burst-log`, `validate-dispatch-advance`) validate file content structure;
that duty remains. The chain check is separate: it belongs on the commit event.

Implementation: the chain-detection WASM gate runs as a new PostToolUse hook on
`tool = "Bash"`, filtered by command content. The dispatcher detects a `git commit`
command in the Bash `tool_input.command` field during the PostToolUse envelope
enrichment phase (after the command has executed successfully), then injects
`git_context` into the payload before routing to chain-detection plugins. The existing
`validate-burst-log` and `validate-dispatch-advance` hooks on `Edit|Write` MUST have
their `check_factory_artifacts_chain()` call removed entirely — that call is the
exec-git violation. The chain check moves to a new trigger.

### Decision 2: `git_context` payload schema

The dispatcher injects a top-level `git_context` JSON object into the hook payload for
qualifying PostToolUse Bash git-commit events. The schema is:

```json
{
  "git_context": {
    "head_subject":        "<string — git log --format=%s -1 HEAD of factory-artifacts worktree>",
    "head_sha":            "<string — git rev-parse HEAD of factory-artifacts worktree, full 40-char hex>",
    "head_parent_subject": "<string — git log --format=%s -1 HEAD^ of factory-artifacts worktree>",
    "head_parent_sha":     "<string — git rev-parse HEAD^ of factory-artifacts worktree, full 40-char hex>"
  }
}
```

Field types: all four are `String`. Empty string indicates the field could not be
populated (e.g., HEAD^ does not exist on initial commit). The WASM plugin treats an
empty `head_parent_subject` as "no chain possible — skip check" (fail-open, consistent
with BC-5.41.003 invariant 5).

Payload delivery: `git_context` is inserted into the `extra` map of `HookPayload`
(the `#[serde(flatten)]` field in `crates/factory-dispatcher/src/payload.rs`). No
new named field is added to `HookPayload` itself — `git_context` rides in `extra`,
which WASM plugins access via `payload.extra.get("git_context")`. This avoids a
`payload.rs` schema change and keeps HOST_ABI_VERSION at 1 (see Decision 4).

### Decision 3: Dispatcher responsibility — host-side git execution

When the dispatcher processes a PostToolUse event for `tool = "Bash"` and the
`tool_input.command` string contains a `git commit` invocation targeting the
factory-artifacts worktree (detected by the presence of `-C <factory_dir>` or
by command containing `.factory` directory reference), the dispatcher MUST:

1. Determine the factory-artifacts worktree path: `<CLAUDE_PROJECT_DIR>/.factory`
   (same convention as ADR-028 §Decision canonical invariant).
2. Execute `git -C <factory_dir> log --format=%s -1 HEAD` to obtain `head_subject`.
3. Execute `git -C <factory_dir> rev-parse HEAD` to obtain `head_sha`.
4. Execute `git -C <factory_dir> log --format=%s -1 HEAD^` to obtain
   `head_parent_subject`. If HEAD^ does not exist (exit non-zero), set
   `head_parent_subject = ""` and `head_parent_sha = ""`.
5. Execute `git -C <factory_dir> rev-parse HEAD^` to obtain `head_parent_sha` (same
   conditional as step 4).
6. Inject the populated `git_context` object into the hook payload's `extra` map.
7. Route the enriched payload to registered PostToolUse Bash plugins.

The dispatcher already has `exec_subprocess` capability at the host layer
(`crates/factory-dispatcher/src/host/exec_subprocess.rs`). No new host capability
is required. The WASM plugins are exec-free (PC1 preserved).

Fail-open: if any git command fails (git not found, not a git repo, network error),
the dispatcher logs a warning and injects `git_context` with all fields set to `""`.
The WASM plugin treats all-empty as "no chain context — skip check" (same fail-open
as the current implementation, but without the exec-git-in-WASM violation).

### Decision 4: HOST_ABI_VERSION remains 1

`git_context` is injected into the `extra` field of `HookPayload` (the flatten map),
not as a new named host function. No new host function is added to the SDK's `host`
module. Therefore HOST_ABI_VERSION does not change. Existing plugins that do not read
`git_context` from `extra` are unaffected.

### Decision 5: WASM gate reads `git_context` from payload.extra, not from exec_subprocess

The WASM plugin's `check_factory_artifacts_chain()` function is redesigned to:

1. Read `git_context` from `payload.extra.get("git_context")`.
2. Extract `head_subject`, `head_sha`, `head_parent_subject`, `head_parent_sha` as strings.
3. If `git_context` is absent or all fields are empty: return `None` (fail-open).
4. Read precompact-flush-log via `host::read_file` (unchanged — this is correct).
5. Call `check_multi_commit_chain(head_subject, head_sha, head_parent_subject, head_parent_sha, flush_log_last_line)`.

The function signature of `check_multi_commit_chain` and `is_precompact_flush_exempt`
is unchanged. The pure logic is correct. Only the I/O wiring changes: exec_subprocess
calls are removed; payload.extra read replaces them.

### Decision 6: vp084-proof.bats schema alignment

The negative-control test in `vp084-proof.bats` (line 298) already passes
`git_context` as a top-level envelope field with `head_subject` and `head_parent_subject`:

```bash
envelope=$(printf '{"event":"PostToolUse","tool":"Edit",...,"git_context":{"head_subject":"stage 1 backfill","head_parent_subject":"stage 2 backfill"}}' ...)
```

This schema is PARTIALLY aligned with ADR-029. The gap: the bats test uses
`tool = "Edit"` (not `tool = "Bash"`), and the schema omits `head_sha` and
`head_parent_sha`. The corrected production schema for bats is:

```bash
envelope=$(printf '{"event":"PostToolUse","tool":"Bash","tool_input":{"command":"git -C .factory commit ..."},"git_context":{"head_subject":"...","head_sha":"<40-char-hex>","head_parent_subject":"...","head_parent_sha":"<40-char-hex>"}}' ...)
```

The bats test must be updated to match production trigger semantics (`tool = "Bash"`)
and include `head_sha` / `head_parent_sha`. The WASM plugin must accept the schema
with all four fields present; it MUST NOT require `head_sha`/`head_parent_sha` to be
non-empty for basic chain detection (they are only needed for the SHA-corroboration
path of `is_precompact_flush_exempt`).

### Decision 7: Prerequisite story for dispatcher git-context injection

The dispatcher-side `git_context` injection (decisions 1, 2, 3) requires changes to:
- `crates/factory-dispatcher/src/main.rs` (payload enrichment before routing)
- `crates/factory-dispatcher/src/invoke.rs` (git-context injection at PostToolUse Bash)
- `plugins/vsdd-factory/hooks-registry.toml` (new chain-detection hook entry or
  updated trigger for existing plugins)

This is a dispatcher release-level change (new binary behavior; requires release for
operator-level cache to pick up). It is analogous in scope to the `write_file.rs`
path-resolution fix that warranted S-18.04a-prereq. A SEPARATE prerequisite story
(tentatively S-18.04b-prereq) is warranted for the dispatcher injection side, so that:

- S-18.04b-prereq: dispatcher `git_context` injection on Bash PostToolUse git-commit
  events; no WASM changes in this story.
- S-18.04b: WASM plugins updated to read `git_context` from payload.extra; exec_subprocess
  calls removed; bats proof updated.

This preserves Red Gate discipline: S-18.04b's WASM changes fail (no `git_context` in
payload) until S-18.04b-prereq delivers the dispatcher injection.

## Consequences

**Positive:**
- WASM plugins are exec-free (PC1 fully satisfied).
- Chain detection evaluates the actual commit being made, not a stale HEAD.
- `git_context` schema is explicit and testable in isolation.
- HOST_ABI_VERSION stays at 1 — no ABI bump, no plugin recompile cascade.
- Pattern is extensible: future plugins can read `git_context` from payload.extra
  without additional dispatcher changes.

**Negative:**
- Dispatcher must detect `git commit` commands in Bash tool_input (command-content
  inspection). This is a heuristic; a Bash command that contains "git commit" as a
  substring but is not a real commit (e.g., `echo "git commit"`) would trigger
  injection. This is acceptable: fail-open (the WASM reads empty/irrelevant context
  and skips the check) is the correct posture; there is no security consequence to
  injecting git_context on a non-commit bash call.
- Requires a prerequisite story before S-18.04b can be fully implemented.

## ARCH-INDEX subsystem

SS-04 (WASM Hook Plugin Layer) — git_context injection straddles SS-04 (WASM
plugin consumer) and the dispatcher host. The dispatcher is SS-03 (Dispatcher Core).
This ADR governs the contract between them; both subsystems are affected.
