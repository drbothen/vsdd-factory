---
name: state-burst
description: Execute the Single Canonical SHA + Two-Commit Protocol for state-manager remediation bursts. Refuses 3rd-commit chains, in-progress narrative voice, and unbackfilled placeholders. Eliminates the SHA-drift / narrative-staleness defect class.
disable-model-invocation: false
allowed-tools: Read, Write, Edit, Bash
---

# State-Burst Protocol

This skill executes the canonical state-manager remediation burst safely. It
is the structural fix for a defect class that produced **6 consecutive
recurrences** in real-world dogfood (see
`docs/lessons-learned/wave-gate-bookkeeping.md`).

## Announce at Start

Before any other action, say verbatim:

> I'm using the state-burst skill to execute the Single Canonical SHA +
> Two-Commit Protocol for this remediation burst. Stage 1 will write all
> fixes with the `15fa97e6` placeholder; Stage 2 will backfill the real
> SHA via global replace. No 3rd commit is permitted.

## When to use

- You are remediating findings from an adversarial pass (Phase 3 wave-gate
  convergence, or any analogous gate that produces a per-pass review).
- You need to update STATE.md, SESSION-HANDOFF.md, and wave-state.yaml in
  lockstep with a single commit's SHA.
- You're committing to the `factory-artifacts` branch (not `develop`).

If you only need to update one of those files for non-burst bookkeeping
(e.g., a session-checkpoint refresh), use the regular state-manager
update protocol — this skill is overkill.

## Pre-burst hygiene

Run before invoking Stage 1:

```bash
git -C .factory status
```

If there are unrelated modifications (sidecar logs, etc.):
- Commit them separately first, OR
- Stash them with `git -C .factory stash push -u`.

Pre-existing modifications **must not** contaminate the burst commit.
The recovery procedure (`git reset --soft HEAD~3` + manual unstage) is
spelled out in `templates/state-manager-checklist-template.md` if this
hygiene step is skipped.

## Stage 1 — Apply fixes with placeholder

Apply every change required by the
[State-Manager Checklist](../../templates/state-manager-checklist-template.md):

1. **Remediation deltas** to source/spec files closing the adversarial
   findings.
2. **STATE.md** updates:
   - Frontmatter `adversary_<wave>_pass_N_<gate>` entry with
     `remediation_sha: 15fa97e6` (literal placeholder).
   - Frontmatter `convergence_status` advanced to the
     `*_REMEDIATED_AWAITING_PASS_N+1` form (or `_CLEAN_WINDOW_K_OF_3`,
     `_CONVERGED`).
   - `awaiting:` field uses outcome-neutral language ("if CLEAN…if
     BLOCKED…").
   - Body table rows updated.
   - Session Resume Checkpoint replaced with current snapshot.
   - Version bumped (X.Y → X.Y+1).
3. **SESSION-HANDOFF.md** updates:
   - `factory-artifacts HEAD: 15fa97e6` (placeholder).
   - `develop HEAD` set to the actual current develop SHA.
   - PR / story / test counts current.
   - Next-session priority outcome-neutral.
4. **wave-state.yaml** updates:
   - `<wave>.gate_pass_N` record with `remediation_sha: 15fa97e6`
     placeholder.
   - `<wave>.gate_status` updated.
   - `<wave>.notes` extended.
   - `next_gate_required` advanced.

**Tense rule** (mandatory):
Write all narrative as if the burst has already completed. ❌ Never
"REMEDIATION IN PROGRESS" or "this burst remediates…". ✅ Always
"REMEDIATED — Awaiting Pass N+1".

When Stage 1 is staged:

```bash
git -C .factory add -A
git -C .factory commit -m "fix(<wave>): close pass N findings — REMEDIATED awaiting pass N+1"
```

The commit message must NOT contain the word `backfill` (Stage 2 owns
that token).

### Stage 1 verification

Before declaring Stage 1 done, run:

```bash
bash .factory/hooks/verify-sha-currency.sh
```

The hook will report STALE on the `15fa97e6` placeholder cites — that is
expected and the two-commit-protocol exception will accept it during
Stage 2. What you're checking now is:

- `develop` SHA is current
- No tense-flip WARN
- No multi-commit chain WARN
- No fabricated SHA cites

If Stage 1 verification surfaces issues other than the `15fa97e6`
placeholder, fix them before committing — or `git reset --soft HEAD` and
redo.

## Stage 2 — Global SHA replace + backfill commit

Read Stage 1's SHA:

```bash
STAGE1_SHA=$(git -C .factory rev-parse HEAD | cut -c1-8)
```

Globally replace the placeholder. Use a portable invocation:

```bash
# macOS BSD sed (use -i.bak suffix)
for f in .factory/STATE.md .factory/SESSION-HANDOFF.md .factory/wave-state.yaml; do
  sed -i.bak "s/15fa97e6/$STAGE1_SHA/g" "$f" && rm "${f}.bak"
done

# Or, on GNU sed (Linux):
# sed -i "s/15fa97e6/$STAGE1_SHA/g" .factory/STATE.md .factory/SESSION-HANDOFF.md .factory/wave-state.yaml
```

Verify exactly one SHA value is now cited:

```bash
grep -oE "[0-9a-f]{8}" .factory/STATE.md .factory/SESSION-HANDOFF.md .factory/wave-state.yaml \
  | sort -u | grep -v -E "(develop SHA values, listed separately)"
```

Commit Stage 2 with `backfill` in the message:

```bash
git -C .factory add -A
git -C .factory commit -m "chore(<wave>): backfill stage-1 SHA $STAGE1_SHA into pass-N records"
```

### Stage 2 verification

Run the hook again:

```bash
bash .factory/hooks/verify-sha-currency.sh
```

Must report `PASS`. If FAIL:
- Inspect the failure message.
- DO NOT add a third commit. Instead:
  ```bash
  git -C .factory reset --soft HEAD~2
  ```
  then redo from Stage 1.

## Push

Both commits get pushed together:

```bash
git -C .factory push origin factory-artifacts
```

After push, run the hook one more time to catch any push-side issues
(branch protection rejecting the bot, etc.):

```bash
bash .factory/hooks/verify-sha-currency.sh
```

## Anti-patterns this skill blocks

The skill refuses to proceed if any of these conditions are detected:

| Anti-pattern | Detection | Recovery |
|--------------|-----------|----------|
| 3rd commit on the burst chain | `verify-sha-currency.sh` reports `MULTI_COMMIT_CHAIN_NOT_ALLOWED` | `git reset --soft HEAD~2` + redo Stage 1 |
| Unbackfilled placeholder after Stage 2 | `grep 15fa97e6 .factory/STATE.md` returns hits | Re-run the global `sed` |
| In-progress voice in narrative | Hook tense-flip WARN | Edit narrative to past-tense before Stage 2 push |
| Stage 1 commit message contains `backfill` | Hook two-commit exception fails | Amend Stage 1 commit message |
| Stage 1 SHA does not exist as a git object | Hook FABRICATED warning | Investigate — usually means a typo in Stage 2 sed |
| Cross-record SHA drift between STATE.md and wave-state.yaml | Hook DRIFT report | Fix the disagreeing record (per Schema Semantics in checklist) |

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
- Originating defect class: a 6-recurrence wave-gate convergence cycle
  whose Pass 7 finally executed clean under this protocol. The case
  study doc enumerates each recurrence and root cause.
