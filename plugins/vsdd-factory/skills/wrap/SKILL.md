---
name: wrap
description: Pause the VSDD factory and make pipeline state durable so the session can be cleared and resumed. Use when the user says "wrap", "wrap up", "wrap the session", or wants to pause the factory / end the session mid-pipeline without losing progress.
allowed-tools: Bash, Read, Skill, Agent
session_checkpoints_template: "${CLAUDE_PLUGIN_ROOT}/templates/session-checkpoints-template.md"
---

# /vsdd-factory:wrap — Pause Factory & Checkpoint for Session Clear

Bring the running VSDD factory to a safe stop: halt new work, persist everything
in-flight to disk, mark the pipeline PAUSED, write a Session Resume Checkpoint,
release the factory lock, and print resume instructions. After this completes,
the session can be `/clear`ed (or closed) with zero loss — a fresh session
resumes from `.factory/STATE.md` alone.

**Invariant (INV-1):** All STATE.md writes go through the `vsdd-factory:state-manager`
agent (BC-6.23.001 Invariant 5 / BC-6.28.001 INV-1). This skill NEVER edits
STATE.md directly. If state-manager is unavailable at Step 4, the wrap fails with a
clear error; it does NOT fall back to direct STATE.md editing.

**Invariant (INV-2):** The `## Factory Wrapped` report and the `Safe to /clear`
declaration are NOT emitted until all Step 6 durability checks pass completely.
Reporting success ahead of verified durability is a behavioral violation identical
in severity to data loss.

**Invariant (INV-3):** No new pipeline sub-agents (orchestrator, implementer,
test-writer, holdout-evaluator, etc.) are spawned after Step 1. The state-manager
dispatch at Step 4 is the only agent delegation allowed post-halt; it is not a
"pipeline sub-agent" — it is the checkpoint-write mechanism. The factory-unlock
invocation at Step 5 is a skill call, not an agent dispatch, and is permitted.

## Step 1 — Halt new work

- Do NOT spawn any new pipeline sub-agents from this point (INV-3).
- If sub-agents are currently running, let them finish their current atomic
  step (e.g., a micro-commit or a single test run) but record any that must be
  abandoned mid-step in the checkpoint notes at Step 4 (see PC-8 field (c)).

## Step 2 — Verify factory health

Run `/vsdd-factory:check-state-health`.

- **HEALTHY / WARNINGS:** proceed to Step 3.
- **NEEDS-COMPACT:** run `/vsdd-factory:compact-state` first so the checkpoint
  lands in a slim STATE.md. Verify all 16 postconditions against the compacted
  file.
- **STATE.md missing or corrupted:** run `/vsdd-factory:recover-state`, obtain
  human approval of the reconstruction, then continue. Step 7 is NOT emitted
  until PC-1 through PC-15 are satisfied on the reconstructed file.

## Step 3 — Persist uncommitted work

Nothing in-flight may live only in this conversation or in a dirty working tree.

1. **Product repo / story worktrees:** run `git status` in the main repo and any
   active `.worktrees/STORY-*` worktrees. For each with uncommitted changes,
   exactly one of the following two conditions MUST hold:
   - **(a) Committable WIP:** commit to the story's feature branch (NEVER to
     `main`, `develop`, or the default branch) with a message prefixed
     `wip(<STORY-ID>): session wrap checkpoint — <what is half-done>`, then
     push to remote if a remote is configured.
   - **(b) Uncommittable state** (mid-red-gate, merge conflict, build failure):
     do NOT force a broken commit. Document the exact state explicitly in the
     Session Resume Checkpoint at Step 4 (PC-8 field (c)) instead. Step 7
     shows the worktree name and its un-committed status.
2. **Factory worktree:** run `git status` in `.factory/`. Any uncommitted
   artifact files (specs, stories, cycle files) are committed by state-manager
   at Step 4 along with STATE.md.
3. **Push story branches** to remote if one is configured (`git push` per
   branch), so the checkpoint survives loss of the machine, not just the
   session. If push fails or no remote exists, note `pushed: no (<reason>)`;
   the working-tree clean check at Step 6 still applies locally.

## Step 4 — Pause + checkpoint via state-manager

Delegate to the `vsdd-factory:state-manager` agent (via the `Agent` tool) with
the following instruction set. The wrap skill makes ZERO direct Write or Edit
tool calls on STATE.md at this step or any other step (BC-6.28.001 INV-1;
PC-10). Wait for state-manager to report completion before proceeding to Step 5.

State-manager MUST perform ALL of the following in a single burst
(TD-VSDD-053 single-commit-per-burst):

1. Set `pipeline:` frontmatter to `PAUSED` (PC-1).
2. Refresh `timestamp:` to the pause instant ISO-8601 string (PC-2).
3. Update `phase:` to begin with `PAUSED <YYYY-MM-DD>.` and include the
   pipeline position string (PC-3).
4. Update `current_step:` per the verbatim-strict chain
   (D-441/D-442/D-443/D-444/D-449): `SESSION-WRAP-PAUSE-<YYYY-MM-DD>` step
   description; no meta-commentary, no clause-reordering, no
   justification-suffix injection (PC-4).
5. Update `last_amended:` with today's date, version bump token, `state-manager`
   identity, and a single-line pause description (PC-5).
6. Bump `version:` by exactly one point; no double-bump (PC-6; TD-VSDD-053).
7. Update the banner `wc-l` claim and dual-margin to match the actual
   `wc -l .factory/STATE.md` after the pause commit (PC-7; BC-5.39.005).
8. Archive the prior `## Session Resume Checkpoint` section to
   `cycles/<current_cycle>/session-checkpoints.md` (using the template at
   `${CLAUDE_PLUGIN_ROOT}/templates/session-checkpoints-template.md`) before
   writing the new one (content-routing discipline; PC-8 preamble).
9. Write exactly ONE new `## Session Resume Checkpoint` with all six minimum
   fields (PC-8):
   - (a) date and current pipeline position (phase, step, story/wave, what is next)
   - (b) convergence counter if in an adversarial/convergence loop (streak, pass count)
   - (c) in-flight work (stories mid-TDD, PRs awaiting review/CI, sub-agent steps
         abandoned at Step 1 above)
   - (d) pending human decisions and unresolved blockers raised this session
   - (e) WIP branch list with SHAs (or "none")
   - (f) the exact resume command
10. Append a Phase Progress / Current Phase Steps table row named
    `SESSION-WRAP-PAUSE-<YYYY-MM-DD>` with status `COMPLETE`, agent
    `state-manager`, and a one-line output description (PC-9).
11. Commit with subject prefix `factory(pause):` including the phase and
    one-line position string (single commit; TD-VSDD-053; PC-11).
12. Push to remote (`git push origin factory-artifacts`) if a remote is
    configured (PC-12). If no remote is configured, the checkpoint is
    local-only; note `pushed: no (no remote configured)` in the Step 7 report.

## Step 5 — Release the factory lock

If this session holds the cross-session factory lock, run
`/vsdd-factory:factory-unlock` so the resuming session (or another machine)
can acquire it (BC-6.23.001 PC4 `factory.lock.released` event; PC-14).

Skip silently if no lock is held and note `Lock: not held` in the Step 7 report
(EC-001). A lock that remains held at the time the Step 7 report is emitted is
a wrap failure.

## Step 6 — Verify durability

All of the following MUST pass before declaring the wrap complete (INV-2).
If any check fails, fix it (re-delegate to state-manager if STATE.md is wrong;
re-run `git status` if the tree is dirty) and re-verify — do NOT report success
with a failing check.

- [ ] `git -C .factory status --porcelain` returns empty (factory-artifacts
      working tree clean; PC-12).
- [ ] STATE.md frontmatter reads `pipeline: PAUSED` (PC-1).
- [ ] Exactly one `## Session Resume Checkpoint` section exists in STATE.md and
      reflects today's date (PC-8).
- [ ] Product repo and all active story worktrees have no uncommitted changes
      that were intended to be saved, or their state is explicitly documented
      in the checkpoint (PC-8 field (c); PC-13 double-check).

## Step 7 — Report

Only emit this report after Step 6 passes completely (INV-2).

```
## Factory Wrapped

Pipeline: PAUSED at <phase> / <current_step>
Checkpoint: .factory/STATE.md (committed <sha>, pushed: yes)
WIP commits: <branch>@<sha>, ... (or: none)
Lock: released (or: not held)

Safe to /clear or close this session.

To resume: open a new session in this project and run
  /vsdd-factory:rehydrate-wave
then
  /vsdd-factory:next-step
Both steps are required. rehydrate-wave injects the correct wave scope;
next-step routes work from the checkpoint.
```

Report notes:
- `<phase>` and `<current_step>` are quoted verbatim from the updated STATE.md
  frontmatter fields (PC-16a).
- `committed <sha>` is the factory-artifacts commit SHA from Step 4 (PC-16b).
- `pushed: yes` or `pushed: no (<reason>)` per actual push outcome (PC-16b).
- `WIP commits:` lists `<branch>@<sha>` for every branch committed at Step 3,
  or `none` if no WIP was in-flight (PC-16c).
- `Lock:` is `released` if factory-unlock ran at Step 5, or `not held` if
  Step 5 was skipped silently (PC-16d).
- `Safe to /clear or close this session.` is the exact wording (PC-16e) — this
  is the human-facing signal that all postconditions have been verified.

## Out of scope

- Does NOT complete or merge in-flight stories — it checkpoints them.
- Does NOT modify spec or story content.
- Does NOT run compaction unless STATE.md health requires it (Step 2 routing).
