---
name: rehydrate-wave
description: Rehydrate session context at the start of a new session after a wave-boundary reset. Reads wave-state.yaml EXCLUSIVELY from the factory-artifacts branch via git show, injects exactly the listed spec files (Set(stories[*].spec_files) ∪ Set(arch_files) ∪ {state_pointer}), and pauses for operator confirmation before any pipeline work begins. Satisfies BC-6.24.001 postconditions 1–8 and VP-088 §1–§4.
argument-hint: ""

allowed-tools: Read, Bash
---

# Rehydrate Wave

Restore session context at the start of a new session after a wave-boundary reset.
Reads `wave-state.yaml` from the `factory-artifacts` branch (never the working tree),
injects exactly the listed spec files into the agent context, and pauses for operator
confirmation — satisfying BC-6.24.001 v1.10 and VP-088 v1.1.

## When to Invoke

Invoke `/rehydrate-wave` as the **first skill** in any new session after a
wave-boundary reset. The human clears the session, starts a new Claude Code session,
and the first action is `/rehydrate-wave`.

Do NOT invoke if you have not cleared the session — mid-session rehydration
may contaminate the context with a stale prior session.

## Invocation Contract

```
/rehydrate-wave \
  --repo <main-repo-dir> \
  --artifacts-worktree <factory-artifacts-worktree-path>
```

### Arguments

| Argument | Purpose |
|----------|---------|
| `--repo <path>` | Root of the main code repository. Used for `git show factory-artifacts:wave-state.yaml`. |
| `--artifacts-worktree <path>` | Path to the checked-out `factory-artifacts` worktree (e.g., `.factory`). |

Production invocation example:

```
/rehydrate-wave \
  --repo . \
  --artifacts-worktree .factory
```

## Agent Steps

### S1 — Invoke `rehydrate-wave.sh`

```bash
bash plugins/vsdd-factory/skills/rehydrate-wave/rehydrate-wave.sh \
  --repo <REPO_DIR> \
  --artifacts-worktree <ARTIFACTS_WT>
```

The script:
1. Reads `wave-state.yaml` from `factory-artifacts` via `git show factory-artifacts:wave-state.yaml`
   (BC-6.24.001 postcondition 1 — git-sourced read; no working-tree fallback).
2. Computes the injected set:
   `Set(stories[*].spec_files) ∪ Set(arch_files) ∪ {state_pointer}` with deduplication
   (BC-6.24.001 postcondition 2 / invariant 2).
3. Checks each listed file's existence on the filesystem. Missing files: emit a WARNING
   and continue (BC-6.24.001 postcondition 6). Do NOT hard-block on a missing file.
4. Emits the human-readable injected file list before the confirmation prompt
   (BC-6.24.001 invariant 4 — transparency).
5. Emits the machine-stable sentinel `INJECTED_FILE_COUNT=<n>` on stdout
   (VP-088 §2 PC2-SIGNAL).
6. Emits an operator confirmation prompt (BC-6.24.001 postcondition 5).

### S2 — Read each injected file

Read every file listed in the `rehydrate-wave.sh` output (the lines starting with `  - `).
Use the `Read` tool to load each file into the session context.

This is the step that actually injects the spec content into the agent context
(BC-6.24.001 postcondition 2 — exactly listed specs injected).

### S3 — Confirm rehydration scope to operator

After reading all injected files, present the injected file list to the operator and
confirm the rehydration scope. Pause and request explicit operator confirmation before
proceeding to any pipeline work (BC-6.24.001 postcondition 5).

**The session MUST NOT start any pipeline step until the operator confirms.**

Example confirmation prompt:

> Rehydration complete. I have loaded the following files into context:
> [list each file]
>
> This is wave N context. Please confirm to proceed with pipeline work.

## Error Paths

### wave-state.yaml absent and no EPIC-COMPLETE HANDOFF.md

If `git show factory-artifacts:wave-state.yaml` fails AND there is no `HANDOFF.md` with
`epic_status: complete` on `factory-artifacts`, the script hard-blocks with:

```
RehydrationError: wave-state.yaml not found on factory-artifacts; cannot rehydrate. Run /wave-handoff on wave N to produce the manifest.
```

**Action:** Run `/wave-handoff` on the prior wave to produce `wave-state.yaml` before
invoking `/rehydrate-wave` (BC-6.24.001 postcondition 7).

No files are injected. Exit is non-zero.

### EPIC-COMPLETE path

If `wave-state.yaml` is absent on `factory-artifacts` but `HANDOFF.md` has
`epic_status: complete` and `next_wave_stories: []`, the skill:

1. Reads `HANDOFF.md` only (not `wave-state.yaml`).
2. Injects `state_pointer` + `arch_files` from `HANDOFF.md`.
3. Emits `"Epic complete — no next-wave stories"`.
4. Exits 0.
5. Does NOT emit `RehydrationError` (absence of `wave-state.yaml` is expected on EPIC-COMPLETE).

This is BC-6.24.001 edge case EC-EPIC.

### Missing listed spec file

If a path listed in `wave-state.yaml` is not found on the filesystem, the script emits a
WARNING naming the missing path and continues injecting the remaining files. The skill
does NOT hard-block; the operator is informed of the gap (BC-6.24.001 postcondition 6).

## Forbidden Dependencies

This skill MUST NOT:
- Read from the working-tree path `.factory/wave-state.yaml` or any locally cached copy
  (BC-6.24.001 invariant 1 / postcondition 1).
- Fall back to RAG, semantic retrieval, vector similarity search, or LLM-based file
  matching (BC-6.24.001 invariant 3 / postcondition 8 / ADR-026 §Decision 4).
- Shell out to Python, jq, or any language runtime beyond bash.

All parsing uses POSIX-compatible bash + awk + grep + git.

## Postconditions Satisfied

| BC-6.24.001 Postcondition | Satisfied by |
|---------------------------|-------------|
| 1 — git-sourced read, no working-tree fallback | `git show factory-artifacts:wave-state.yaml` in `rehydrate-wave.sh` |
| 2 — exactly listed specs injected; INJECTED_FILE_COUNT sentinel | Set union + deduplication; `INJECTED_FILE_COUNT=<n>` on stdout |
| 3 — no stale prior-wave specs | Exact manifest list only; no filesystem scan |
| 4 — STATE.md pointer always injected | `state_pointer` always included in the union |
| 5 — operator confirmation required before proceeding | Confirmation prompt in S1 + manual confirm in S3 |
| 6 — missing spec files: warn, not block | `WARNING:` message + continue on missing paths |
| 7 — wave-state.yaml not found: hard block with RehydrationError | `RehydrationError:` message + exit 1 when absent and no EPIC-COMPLETE |
| 8 — no RAG fallback | No semantic retrieval code path in `rehydrate-wave.sh` |

## Architecture Anchors

- BC-6.24.001 v1.10 (rehydrate-wave skill — git-sourced manifest, exact-list injection,
  no stale prior-wave specs, RehydrationError on missing manifest, no RAG)
- VP-088 v1.1 (§1 git-sourced, §2 exact-list + INJECTED_FILE_COUNT, §3 RehydrationError,
  §4 EPIC-COMPLETE)
- ADR-026 §Decision 3 (prompt-the-human; operator clears session)
- ADR-026 §Decision 4 (curated wave-state.yaml manifest; RAG explicitly deferred;
  reads from factory-artifacts via git; working-tree not authoritative)
- S-18.03 (implementation story)
