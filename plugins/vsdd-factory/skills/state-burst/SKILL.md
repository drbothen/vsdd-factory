---
name: state-burst
description: Execute the Single-Commit Burst Protocol (TD-VSDD-053) for state-manager remediation bursts. One atomic commit per burst — no Stage 2 backfill, no SHA placeholder, no chain. Refuses in-progress narrative voice and reintroduction of the retired two-commit pattern.
disable-model-invocation: false
allowed-tools: Read, Write, Edit, Bash
---

# State-Burst Protocol (Single-Commit)

This skill executes the canonical state-manager remediation burst safely.

## History

This skill previously implemented the two-commit "Single Canonical SHA +
Stage 2 Backfill" protocol. That pattern was self-referential: STATE.md
sits ON the factory-artifacts branch, so committing STATE.md changes
HEAD, instantly staling any HEAD-SHA cite inside the same content. The
two-commit workaround (Stage 1 placeholder → commit → Stage 2 backfill
SHA → commit) created "fix-the-fix" loops when any of 8 cite locations
was missed, manifesting **6 consecutive recurrences in one session
costing 5+ force-pushes** (see
`docs/lessons-learned/wave-gate-bookkeeping.md`).

TD-VSDD-053 (2026-05-04) retired the two-commit protocol by removing
the self-referential cite altogether: STATE.md and SESSION-HANDOFF.md
no longer claim the current factory-artifacts HEAD SHA in their
"current state" sections. Git itself owns that data — run
`git -C .factory log -1 --format='%h %s'` for it. Historical SHA
references in changelog rows, decisions log, and cycle manifests remain
valid (immutable PAST burst SHAs).

## Announce at Start

Before any other action, say verbatim:

> I'm using the state-burst skill to execute the Single-Commit Burst
> Protocol for this remediation burst. One atomic commit; no Stage 2
> backfill; no SHA placeholder. The current factory-artifacts HEAD SHA
> is not cited in STATE.md/HANDOFF.md "current state" sections.

## When to use

- You are remediating findings from an adversarial pass (Phase 3
  wave-gate convergence, or any analogous gate that produces a per-pass
  review).
- You need to update STATE.md, SESSION-HANDOFF.md, and wave-state.yaml
  in lockstep with a single commit.
- You're committing to the `factory-artifacts` branch (not `develop`).

If you only need to update one of those files for non-burst bookkeeping
(e.g., a session-checkpoint refresh), use the regular state-manager
update protocol — this skill is overkill.

## Pre-burst hygiene

Run before applying any changes:

```bash
git -C .factory status
```

If there are unrelated modifications (sidecar logs, etc.):
- Commit them separately first, OR
- Stash them with `git -C .factory stash push -u`.

Pre-existing modifications **must not** contaminate the burst commit.

## Apply changes (single atomic commit)

Apply every change required by the
[State-Manager Checklist](../../templates/state-manager-checklist-template.md):

1. **Remediation deltas** to source/spec files closing the adversarial
   findings.
2. **STATE.md** updates:
   - Frontmatter `adversary_<wave>_pass_N_<gate>` entry (with
     `remediation_sha:` if your project still uses that field for
     historical record — that field is OK because once written it
     points at THIS burst's commit and never gets re-cited; future
     bursts add new entries, never modify this one).
   - Frontmatter `convergence_status` advanced to the
     `*_REMEDIATED_AWAITING_PASS_N+1` form (or `_CLEAN_WINDOW_K_OF_3`,
     `_CONVERGED`).
   - `awaiting:` field uses outcome-neutral language ("if CLEAN…if
     BLOCKED…").
   - Body table rows updated.
   - Session Resume Checkpoint replaced with current snapshot.
   - Version bumped (X.Y → X.Y+1).
   - **DO NOT** cite the current factory-artifacts HEAD SHA anywhere in
     "current state" prose. It's `git -C .factory log -1` — git owns it.
3. **SESSION-HANDOFF.md** updates (if your project uses it):
   - `develop HEAD` set to the actual current develop SHA (cross-branch
     cite — fine, no loop).
   - PR / story / test counts current.
   - Next-session priority outcome-neutral.
   - **DO NOT** cite the current factory-artifacts HEAD anywhere.
4. **wave-state.yaml** updates (write the eventual SHA AFTER the commit
   exists is no longer needed — see Stage-2-retirement note below):
   - `<wave>.gate_pass_N` record (the `remediation_sha:` field, if
     present, gets the SHA of the commit you're about to make; you
     can't know this in advance under the single-commit model. Two
     options:
     (a) Omit the SHA field for this burst — wave-state.yaml records
         only that pass N happened, and the historical lookup uses
         `git log` to map pass→commit by date/message.
     (b) Pre-compute the SHA via `git commit-tree` dry-run and write
         it before the actual commit. Most projects pick (a) — it
         avoids the loop AND the pre-compute complexity.
   - `<wave>.gate_status` updated.
   - `<wave>.notes` extended.
   - `next_gate_required` advanced.

**Tense rule** (mandatory):
Write all narrative as if the burst has already completed. ❌ Never
"REMEDIATION IN PROGRESS" or "this burst remediates…". ✅ Always
"REMEDIATED — Awaiting Pass N+1".

## Commit

When all changes are staged:

```bash
git -C .factory add -A
git -C .factory commit -m "fix(<wave>): close pass N findings — REMEDIATED awaiting pass N+1"
```

The commit message must NOT contain the word `backfill` (that token is
reserved for the retired Stage 2 pattern; using it would trigger
`MULTI_COMMIT_CHAIN_NOT_ALLOWED` if any subsequent commit also uses it).

## Verification

Run the hook:

```bash
bash .factory/hooks/verify-sha-currency.sh
```

Must report `PASS`. The hook now checks:
- `develop` SHA cited in STATE.md/HANDOFF.md matches actual develop HEAD
  (cross-branch cite — no loop)
- No `MULTI_COMMIT_CHAIN_NOT_ALLOWED` (the chain-shape regression guard)
- Cross-record agreement between wave-state.yaml `gate_pass_N`
  remediation_sha entries and STATE.md frontmatter (if both record the
  same SHA, they must agree)
- No tense-flip in active-pass narrative (advisory)

If FAIL:
- Inspect the failure message.
- DO NOT add a second commit. Instead:
  ```bash
  git -C .factory reset --soft HEAD
  ```
  then re-edit and re-commit.

## Push

```bash
git -C .factory push origin factory-artifacts
```

After push, run the hook one more time to catch any push-side issues:

```bash
bash .factory/hooks/verify-sha-currency.sh
```

## Anti-patterns this skill blocks

| Anti-pattern | Detection | Recovery |
|--------------|-----------|----------|
| Reintroducing two-commit chain (HEAD and HEAD^ both contain `backfill`) | `verify-sha-currency.sh` reports `MULTI_COMMIT_CHAIN_NOT_ALLOWED` | `git reset --soft HEAD~2` + re-author as one commit |
| Citing current factory-artifacts HEAD SHA in STATE.md/HANDOFF.md "current state" sections | Code review / per-burst editor discipline (the hook no longer enforces this since the cite is gone) | Edit out the cite; replace with "see `git -C .factory log -1`" if guidance is needed |
| In-progress voice in narrative | Hook tense-flip WARN | Edit narrative to past-tense before push |
| Cross-record SHA drift between STATE.md and wave-state.yaml | Hook DRIFT report | Fix the disagreeing record (per Schema Semantics in checklist) |
| Develop SHA in STATE.md does not match actual develop HEAD | Hook FAIL | Update the develop cite to the current develop HEAD |

## When to bypass

Bypassing this skill is acceptable for:
- The first state-manager burst on a brand-new project (no
  `wave-state.yaml` yet).
- Manual recovery after a force-push event (the protocol assumes a clean
  starting tree).

In both cases, document the bypass reason in
`SESSION-HANDOFF.md → Recent Burst Episodes`.

## Reference

- Checklist: `templates/state-manager-checklist-template.md`
- Hook: `templates/verify-sha-currency.sh`
- Case study: `docs/lessons-learned/wave-gate-bookkeeping.md`
- TD: TD-VSDD-053 (single-commit protocol replacing two-commit; resolves
  TD-VSDD-044 self-referential-cite loop)
