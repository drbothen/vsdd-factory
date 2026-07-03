---
name: wave-reset
description: Full wave-boundary reset procedure. Clears the current session, starts a new session, and runs /rehydrate-wave as the mandatory first step to restore exactly the next-wave spec context from wave-state.yaml on factory-artifacts. Pairs with /wave-handoff (which produces the manifest) to provide lossless context-window transitions (CAP-032).
argument-hint: ""

allowed-tools: Read, Bash
---

# Wave Reset

Execute a full wave-boundary session reset. This procedure clears the current session's
context and starts a new session with precisely the next-wave spec scope —
no stale prior-wave specs, no RAG hallucination, no working-tree override.

`/wave-reset` is the consumption side of `/wave-handoff`. After `wave-handoff` produces
`HANDOFF.md` + `wave-state.yaml` on `factory-artifacts`, the operator performs a
wave-boundary reset using this procedure to start the next wave in a clean session.

## When to Invoke

Invoke `/wave-reset` when:

1. The current wave's stories are complete (all merged, all terminal states in
   `sprint-state.yaml`).
2. `/wave-handoff` has been run and `wave-state.yaml` is committed to `factory-artifacts`.
3. The operator is ready to begin the next wave in a fresh session.

## Wave Reset Procedure

### Step 1 — Verify wave-handoff has run

Before resetting the session, confirm `wave-state.yaml` exists on `factory-artifacts`:

```bash
git show factory-artifacts:wave-state.yaml > /dev/null 2>&1 \
  && echo "PASS: wave-state.yaml present" \
  || echo "FAIL: run /wave-handoff first to produce the manifest"
```

If `wave-state.yaml` is absent and no EPIC-COMPLETE HANDOFF.md exists, run
`/wave-handoff` before proceeding.

### Step 2 — Close the current session

Perform a session clear to drop all prior-wave context from the active session window.
In Claude Code: start a **NEW session** (full clear). Do NOT use `/compact` for a wave
boundary — `/compact` summarizes rather than drops prior-wave context, so stale
prior-wave specs leak in (ADR-026 §Decision 1: a wave-boundary reset is a hard reset,
not a compaction; compaction is the Part B intra-wave safety net the design avoids
stacking).

**The session clear is the boundary event.** After clearing, the new session starts
with no prior-wave context loaded — neither specs, nor BCs, nor intermediate decisions
from the prior wave.

### Step 3 — Rehydrate in New Session

**This is the mandatory first step in the new session. Do not invoke any pipeline skill
before `/rehydrate-wave` completes.**

In the new session, invoke:

```
/rehydrate-wave \
  --repo . \
  --artifacts-worktree .factory
```

`/rehydrate-wave` (S-18.03):

1. Reads `wave-state.yaml` from `factory-artifacts` via
   `git show factory-artifacts:wave-state.yaml` — never from the working tree
   (BC-6.24.001 postcondition 1 / invariant 1).
2. Computes the injected file set:
   `Set(stories[*].spec_files) ∪ Set(arch_files) ∪ {state_pointer}` with deduplication
   (BC-6.24.001 postcondition 2 / invariant 2).
3. Emits `INJECTED_FILE_COUNT=<n>` sentinel for automated verification (VP-088 §2 PC2-SIGNAL).
4. Reads each listed file into the session context via the `Read` tool.
5. Pauses and presents a confirmation prompt listing all injected files
   (BC-6.24.001 postcondition 5 / invariant 4).

The new session MUST NOT proceed to any pipeline step until the operator confirms the
rehydration scope.

### Step 4 — Operator Confirmation

After `/rehydrate-wave` presents the injected file list, the operator:

1. Reviews the injected file list.
2. Confirms the rehydration scope is correct.
3. Explicitly tells the agent to proceed.

If any expected file is missing (the skill will have emitted a `WARNING:` line naming
it), the operator decides whether to proceed without that file or to resolve the gap
before beginning pipeline work (BC-6.24.001 postcondition 6).

### Step 5 — Proceed with Next-Wave Pipeline Work

After operator confirmation, the new session has exactly the next-wave spec scope
loaded. Proceed with the wave's pipeline tasks (story implementation, adversarial
review, PR cycles, etc.).

## EPIC-COMPLETE Variant

When the epic is complete (all stories terminal, `wave-handoff` emitted
`HANDOFF.md` with `epic_status: complete` and `next_wave_stories: []`),
`wave-state.yaml` is intentionally absent on `factory-artifacts`.

In this case, `/rehydrate-wave` detects the EPIC-COMPLETE path:

1. Reads `HANDOFF.md` (not `wave-state.yaml`).
2. Injects `state_pointer` + `arch_files` from `HANDOFF.md`.
3. Emits `"Epic complete — no next-wave stories"`.
4. Exits 0 without a `RehydrationError`.

The wave-reset procedure is identical; `/rehydrate-wave` handles the EPIC-COMPLETE
path automatically.

## Why This Procedure Exists

Wave-boundary resets without a structured rehydration procedure suffer from:

- **Context pollution**: stale prior-wave BCs, ADR files, and story specs remain in
  session context and may be cited erroneously by the next wave's agents.
- **RAG hallucination**: without an explicit manifest, agents may use semantic retrieval
  to guess what context to load — producing non-deterministic rehydration (ADR-026
  §Decision 4 prohibits this).
- **Working-tree drift**: locally edited `.factory/wave-state.yaml` may differ from the
  committed branch version, causing rehydration from stale data (BC-6.24.001 invariant 1
  prohibits this).

`/wave-reset` + `/rehydrate-wave` close these gaps by making the wave-boundary
transition explicit, git-sourced, and operator-confirmed.

## Architecture Anchors

- BC-6.24.001 v1.10 (rehydrate-wave skill — consumption side of wave-boundary reset)
- BC-5.41.001 (HANDOFF.md production — `/wave-handoff` prerequisite)
- BC-5.41.002 (wave-state.yaml production — `/wave-handoff` prerequisite)
- VP-088 v1.1 (rehydrate-wave verification properties)
- ADR-026 §Decision 3 (prompt-the-human; operator clears session)
- ADR-026 §Decision 4 (curated wave-state.yaml manifest; RAG explicitly deferred)
- CAP-032 (Guarantee lossless context-window transitions via wave-boundary checkpoint
  and PreCompact flush)
- S-18.03 (rehydrate-wave skill — T-3 deliverable)
- S-18.04a / S-18.04b (wave-handoff + wave-boundary checkpoint — prerequisites)
