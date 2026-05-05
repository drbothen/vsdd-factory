# State-Manager Wave-Gate Remediation-Burst Checklist

When remediating findings from an adversarial pass and committing to
`factory-artifacts`, the state-manager MUST update ALL of the artifacts in
this checklist as a single atomic burst.

This checklist exists because narrow-scope bursts that miss one or more of
these items have produced **6+ consecutive defect recurrences** in
real-world dogfood (see `docs/lessons-learned/wave-gate-bookkeeping.md`
for the full trajectory). The recurrence stops when the **Single-Commit
Burst Protocol** is followed exactly.

> **Use the `vsdd-factory:state-burst` skill to execute this protocol.** It
> wraps every step below with verification and refuses to ship known
> anti-patterns. Manual execution is supported but error-prone.

## History

This checklist previously prescribed a **Single Canonical SHA + Two-Commit
Protocol** (Stage 1 placeholder → commit → Stage 2 backfill SHA → commit).
That protocol was self-referential — STATE.md sits ON the factory-artifacts
branch, so committing STATE.md changes HEAD, instantly staling any
HEAD-SHA cite inside the same content. Stage 2's backfill had to update
the SHA in 8 specific cite locations; missing any one created a
"fix-the-fix" loop. That loop manifested 6× in one session, costing
5+ force-pushes.

TD-VSDD-053 (2026-05-04) retired the two-commit protocol by removing the
self-referential cite altogether: STATE.md and SESSION-HANDOFF.md no
longer claim the current factory-artifacts HEAD SHA in their "current
state" sections. Git itself owns that data — `git -C .factory log -1`
returns the current HEAD. Historical SHA references in changelog rows,
decisions log, and cycle manifests remain valid (immutable past burst
SHAs).

This checklist is now single-commit only.

---

## wave-state.yaml Bookkeeping (the recurring drift class)

Replace `<WAVE>` with the active wave (e.g. `wave_1`, `wave_1_5`,
`wave_2`). Replace `N` with the current pass number.

- [ ] **Top-level `next_gate_required:`** — update to the NEXT pass
  (`pass_N+1_pending`) after your burst.
- [ ] **`<WAVE>.gate_status:`** — update to
  `integration_gate_pass_N_remediated_awaiting_pass_N+1` (or analogous
  string for non-integration gates).
- [ ] **Add `<WAVE>.gate_pass_N:` record** with all required fields
  EXCEPT `remediation_sha` (see below):
  ```yaml
  gate_pass_N:
    verdict: BLOCKED|CLEAN
    findings: <int>
    remediated: <int>
    timestamp: YYYY-MM-DD
    passed: true|false
  ```
- [ ] **`remediation_sha:` field handling.** Two acceptable patterns:
  - **(a) Omit the field for THIS burst.** Look up pass→commit mapping
    later via `git log --all --oneline | grep "pass N"`. Simplest; no
    self-reference; recommended for new projects. Historical pass
    records (entries written by EARLIER bursts) keep their
    `remediation_sha` values — those are immutable history.
  - **(b) Write the field POST-COMMIT in a follow-up amendment.** After
    the single-commit burst lands, run a separate small commit that
    fills `remediation_sha:` for the just-completed pass with that
    commit's SHA. This creates a 2-commit chain ONLY when the
    record-keeping is critical AND the operator accepts the chain
    explicitly. NOT the default.
- [ ] **Extend `<WAVE>.notes:`** with a paragraph describing Pass N:
  outcome, findings, what was fixed. Reference the commit by message
  not SHA where possible.
- [ ] **Verify no in-progress placeholders** before pushing:
  ```bash
  grep -E "TBD|TODO|FIXME|this_burst|XXX" .factory/wave-state.yaml
  # Must return empty.
  ```
- [ ] **Verify pass record count** matches current pass:
  ```bash
  grep -c "gate_pass_[0-9]" .factory/wave-state.yaml
  ```

---

## Single-Commit Burst Rule (mandatory)

A burst is exactly ONE commit on factory-artifacts. No Stage 2 backfill.
No SHA placeholder. No 2-commit chain.

- Apply ALL fixes (documents, narrative, frontmatter, wave-state.yaml,
  hook updates, checklist updates) in one staged change.
- Write narrative in **past-tense / "REMEDIATED" voice** from the start.
  Never "in progress" voice.
- Commit with a normal fix-commit message — must NOT contain the word
  `backfill`. (That token is reserved for the retired Stage 2 pattern;
  using it would trigger `MULTI_COMMIT_CHAIN_NOT_ALLOWED` if a
  subsequent commit also uses it.)
- If you discover a missed fix:
  ```bash
  git -C .factory reset --soft HEAD
  ```
  then re-edit and re-commit.

**`MULTI_COMMIT_CHAIN_NOT_ALLOWED` regression guard** (enforced by
`verify-sha-currency.sh`):
- HEAD's commit message contains `backfill` AND HEAD^'s also contains
  `backfill` → FAIL. This means the retired two-commit pattern was
  accidentally reintroduced. Recover with `git reset --soft HEAD~2` and
  re-author as one commit.

---

## Tense Flip Rule

Burst narrative is written as if the burst has already completed. Pass
N+1 is the future event, not the burst itself.

❌ Wrong (recurred across three consecutive passes in the originating
case study):
> "Pass N BLOCKED — REMEDIATION IN PROGRESS"
> "this burst remediates findings from..."

✅ Right (Pass 7 clean form):
> "Pass N — REMEDIATED. Findings closed. Awaiting Pass N+1 (if CLEAN,
> 1st of 3 clean-pass window opens; if BLOCKED, remediate + Pass N+1)."

The `verify-sha-currency.sh` hook surfaces tense-flip violations as WARN
(not FAIL) so you can choose strict-mode enforcement at the pre-push
boundary.

---

## STATE.md Bookkeeping

- [ ] **Frontmatter `adversary_<wave>_pass_N_<gate>:`** — add new entry
  with `{passed, findings, remediated, timestamp}`. Same `remediation_sha`
  guidance as wave-state.yaml above.
- [ ] **Frontmatter `convergence_status:`** — advance to one of:
  - `<PHASE>_<WAVE>_GATE_PASS_N_REMEDIATED_AWAITING_PASS_N+1` (verdict
    BLOCKED)
  - `<PHASE>_<WAVE>_GATE_PASS_N_CLEAN_WINDOW_K_OF_3` (verdict CLEAN, K =
    cumulative count of consecutive clean passes)
  - `<PHASE>_<WAVE>_GATE_CONVERGED` (when K reaches 3)
- [ ] **Frontmatter `current_step:`** — update narrative to Pass N
  outcome.
- [ ] **Frontmatter `awaiting:`** — outcome-neutral form ("if CLEAN…if
  BLOCKED…").
- [ ] **Frontmatter `convergence_window_progress:`** — update count.
- [ ] **Body Last Updated row** — describe Pass N.
- [ ] **Body Current Phase row** — pass count + window.
- [ ] **Body Phase Progress finding-progression cell** — append Pass N
  finding count to the trajectory shorthand (e.g., `29→24→21→7→4→3`).
- [ ] **Body Current Phase Steps** — append Pass N row (preserve audit
  trail).
- [ ] **Session Resume Checkpoint** — replace with current checkpoint
  (outcome-neutral next-steps); archive old to
  `cycles/<cycle>/session-checkpoints.md`.
- [ ] **Version bump** — minor for normal burst (X.Y → X.Y+1).
- [ ] **DO NOT** cite the current factory-artifacts HEAD SHA anywhere
  in "current state" prose — there is no such cite under TD-VSDD-053.
  Use `git -C .factory log -1` to look up the current SHA.

---

## SESSION-HANDOFF.md (if your project uses it)

- [ ] **Verify `develop HEAD`** is current. (Cross-branch cite — fine,
  no loop. The hook validates this.)
- [ ] **Verify PR / story-merged counts** are current.
- [ ] **Verify test counts** are current.
- [ ] **Next session priority** uses outcome-neutral language.
- [ ] **No references** to in-progress work that is now complete.
- [ ] **DO NOT** cite the current factory-artifacts HEAD SHA. Per
  TD-VSDD-053, the factory-artifacts HEAD is `git -C .factory log -1`,
  not a string in any artifact.

---

## Outcome-Neutral Language Rule

When writing next-steps or checkpoints **before** a pass runs:

- ❌ "Pass N — 1st of 3 required clean passes"
- ✅ "Pass N — if CLEAN, 1st of 3 clean-pass window opens; if BLOCKED,
  remediate + Pass N+1"

Outcome-presumptive language was flagged repeatedly across passes in the
originating case study. Use neutral framing always.

---

## Schema Semantics: `remediation_sha` for multi-pass closures

Historical pass records keep their original `remediation_sha` values
(immutable). Subsequent re-closures DO NOT advance the SHA backward.

Example:
- Pass 3 was incompletely remediated; the original `gate_pass_3.remediation_sha`
  was set under the prior two-commit protocol and remains as historical
  record.
- Pass 4 burst closes both Pass 3 leftovers AND Pass 4 findings under the
  new single-commit protocol.
- `gate_pass_3.remediation_sha` stays at its historical value (immutable).
- `gate_pass_4.remediation_sha` is omitted under pattern (a), or filled
  via post-commit amendment under pattern (b).
- Pass 4's `notes` field documents that the Pass 4 burst also closed
  Pass 3 leftovers.

The `verify-sha-currency.sh` hook's cross-record check enforces that
STATE.md frontmatter `remediation_sha` agrees with wave-state.yaml's
`gate_pass_N.remediation_sha` for every pass that has the field set in
both places.

---

## Pre-Burst Hygiene

Before starting the burst, run `git -C .factory status`. If unrelated
files are modified (sidecar logs, etc.), either commit them separately
first or stash them. Pre-existing modifications must NOT contaminate the
burst commit.

---

## Verification Commands

Run these in sequence before pushing:

```bash
# 1. SHA currency + burst hygiene (encapsulates 90% of the checks below)
bash .factory/hooks/verify-sha-currency.sh

# 2. No in-progress placeholders in wave-state.yaml
grep -E "TBD|TODO|FIXME|this_burst|XXX" .factory/wave-state.yaml

# 3. Pass record count matches current pass
grep -c "gate_pass_[0-9]" .factory/wave-state.yaml

# 4. next_gate_required is N+1, not N
grep "next_gate_required:" .factory/wave-state.yaml

# 5. STATE.md version bumped
grep "^version:" .factory/STATE.md

# 6. No accidental factory-artifacts HEAD self-cite
#    (per TD-VSDD-053 — STATE.md no longer cites its own branch HEAD)
grep -E "factory-artifacts HEAD\W+\`?[0-9a-f]{8}" .factory/STATE.md \
  .factory/SESSION-HANDOFF.md 2>/dev/null
# Must return empty (or only matches inside the historical Changelog
# section — those are PAST burst SHAs and are immutable).
```

---

## Recovery Procedures

### A 2nd commit accidentally landed during the burst

1. `git -C .factory log --oneline -5` — inspect the chain.
2. `git -C .factory reset --soft HEAD~N` — N = number of accidental
   extra commits.
3. `git -C .factory status` — inspect ALL staged files.
4. **Unstage anything this burst did not author** (e.g., session
   sidecar logs).
5. Re-commit cleanly as one commit.

### The burst was already pushed when drift surfaces

`git push --force-with-lease` requires human approval. Confirm before
proceeding. Document the episode in `SESSION-HANDOFF.md` "Recent Burst
Episodes" section if your project uses one.

---

## Failure Modes Observed (originating dogfood case study, retained for
historical context)

These are the failure modes that drove the original two-commit protocol.
Most are obviated under the single-commit protocol — they're listed here
so future maintainers understand why specific defenses still exist.

| Pass | What Was Missed | Root Cause | Status under single-commit |
|------|-----------------|------------|----------------------------|
| 1 | develop_head stale post-PR merge | Hook only checked factory-artifacts HEAD | Still relevant — develop SHA cite check preserved |
| 2 | Tense-flip in Stage 1 commit | No checklist forcing past-tense voice | Still relevant — tense-flip WARN preserved |
| 3 | Hook fired pre-push but missed multi-commit chain | Hook scope too narrow | Still relevant — MULTI_COMMIT_CHAIN_NOT_ALLOWED preserved |
| 4 | Burst chain extended to 4 commits | Multi-commit detection missing | Still relevant — MULTI_COMMIT_CHAIN_NOT_ALLOWED preserved |
| 5 | Per-document SHA writes fragmented citations | Single Canonical SHA Rule not yet codified | Obviated — single-commit means one commit, one inherent SHA |
| 6 | STATE.md frontmatter pass-3 SHA disagreed with wave-state.yaml | Cross-record verification missing | Still relevant — cross-record check preserved |
| 7 | (CLEAN) — manual orchestrator execution proved the protocol works | Captured + shipped as the prior two-commit checklist | Replaced by single-commit (TD-VSDD-053) |

Trajectory: **11 → 12 → 10 → 10 → 11 → 7 → 3 (CLEAN)** — six recurrences
before convergence under the prior two-commit protocol. The single-commit
protocol eliminates the self-referential cite class entirely.

---

## Non-burst Bookkeeping (incidental)

This checklist focuses on the burst-recurrence defect classes. For
generic STATE.md / SESSION-HANDOFF.md content rules (which sections to
update on phase transitions, where session checkpoints go, etc.), see
the agent prompt at `agents/state-manager.md` and the existing burst-log
template at `templates/burst-log-template.md`.
