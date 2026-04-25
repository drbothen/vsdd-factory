# State-Manager Wave-Gate Remediation-Burst Checklist

When remediating findings from an adversarial pass and committing to
`factory-artifacts`, the state-manager MUST update ALL of the artifacts in
this checklist as a single burst.

This checklist exists because narrow-scope bursts that miss one or more of
these items have produced **6+ consecutive defect recurrences** in
real-world dogfood (see `docs/lessons-learned/wave-gate-bookkeeping.md`
for the full trajectory). The recurrence stops when the **Single
Canonical SHA + Two-Commit Protocol** is followed exactly.

> **Use the `vsdd-factory:state-burst` skill to execute this protocol.** It
> wraps every step below with verification and refuses to ship known
> anti-patterns. Manual execution is supported but error-prone.

---

## wave-state.yaml Bookkeeping (the recurring drift class)

Replace `<WAVE>` with the active wave (e.g. `wave_1`, `wave_1_5`,
`wave_2`). Replace `N` with the current pass number.

- [ ] **Top-level `next_gate_required:`** — update to the NEXT pass
  (`pass_N+1_pending`) after your burst.
- [ ] **`<WAVE>.gate_status:`** — update to
  `integration_gate_pass_N_remediated_awaiting_pass_N+1` (or analogous
  string for non-integration gates).
- [ ] **Add `<WAVE>.gate_pass_N:` record** with all required fields:
  ```yaml
  gate_pass_N:
    verdict: BLOCKED|CLEAN
    findings: <int>
    remediated: <int>
    remediation_sha: <SHA or 15fa97e6 placeholder>
    timestamp: YYYY-MM-DD
    passed: true|false
  ```
- [ ] **Extend `<WAVE>.notes:`** with a paragraph describing Pass N:
  outcome, findings, remediation SHA, what was fixed.
- [ ] **Verify no placeholders remain** before pushing Stage 2:
  ```bash
  grep -E "TBD|TODO|FIXME|this_burst|XXX|backfill" .factory/wave-state.yaml
  # Must return empty.
  ```
- [ ] **Verify pass record count** matches current pass:
  ```bash
  grep -c "gate_pass_[0-9]" .factory/wave-state.yaml
  ```

---

## Single Canonical SHA Rule (mandatory)

A burst MUST reference exactly ONE SHA value across ALL documents.

1. **Stage 1 (commit 1):** Apply ALL fixes — documents, narrative,
   frontmatter, wave-state.yaml, hook updates, checklist updates — using
   the literal placeholder `15fa97e6` everywhere a SHA is needed. Write
   narrative in **past-tense / "REMEDIATED" voice** from the start. Never
   "in progress" voice. Commit with a normal fix-commit message.
2. **Stage 2 (commit 2):** Read commit 1's SHA via
   `git -C .factory rev-parse HEAD`. Globally replace `15fa97e6` with that
   SHA across STATE.md, SESSION-HANDOFF.md, and wave-state.yaml. Commit
   message MUST contain the word `backfill`. Push both commits.
3. **NO third commit.** If you discover a missed fix after Stage 2:
   ```bash
   git -C .factory reset --soft HEAD~2
   ```
   then redo from Stage 1. Force-push requires human approval.

**Why not per-document SHA writes:** Writing SHAs document-by-document
during the burst creates a SHA chain where each intermediate commit is
cited somewhere. The recurring drift had this root cause. The placeholder
+ global replace approach guarantees exactly one SHA value is cited: the
Stage 1 commit.

**Exactly-2-commit-chain rule** (enforced by `verify-sha-currency.sh`):
- HEAD's commit message contains `backfill` (Stage 2 marker), AND
- HEAD^'s commit message does NOT contain `backfill` (Stage 1 must be
  the fix, not another backfill).
- If HEAD^ also contains `backfill`: `MULTI_COMMIT_CHAIN_NOT_ALLOWED`.

---

## Tense Flip Rule

Stage 1 writes the narrative as if the burst has already completed. Pass
N+1 is the future event, not the burst itself.

❌ Wrong (recurred across three consecutive passes in the originating
case study):
> "Pass N BLOCKED — REMEDIATION IN PROGRESS"
> "this burst remediates findings from..."

✅ Right (Pass 7 clean form):
> "Pass N — REMEDIATED. Findings closed at SHA 15fa97e6. Awaiting Pass
> N+1 (if CLEAN, 1st of 3 clean-pass window opens; if BLOCKED, remediate
> + Pass N+1)."

The `verify-sha-currency.sh` hook surfaces tense-flip violations as WARN
(not FAIL) so you can choose strict-mode enforcement at the pre-push
boundary.

---

## STATE.md Bookkeeping

- [ ] **Frontmatter `adversary_<wave>_pass_N_<gate>:`** — add new entry
  with `{passed, findings, remediated, remediation_sha, timestamp}`.
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

---

## SESSION-HANDOFF.md

- [ ] **Verify `develop HEAD`** is current.
- [ ] **Verify PR / story-merged counts** are current.
- [ ] **Verify test counts** are current.
- [ ] **Next session priority** uses outcome-neutral language.
- [ ] **No references** to in-progress work that is now complete.
- [ ] **`factory-artifacts HEAD`** must be a concrete SHA — never
  `(current after this burst)`, `TBD`, or any placeholder string. Use
  `15fa97e6` in Stage 1 and the global-replace pattern in Stage 2.

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

When a burst closes findings from MULTIPLE prior passes (because earlier
remediation was incomplete), each affected pass record's
`remediation_sha` is set to the SHA of the **closing burst's Stage 1
commit**. Subsequent re-closures DO NOT advance the SHA backward.

Example:
- Pass 3 was incompletely remediated at SHA `b1b145b3` (Stage 2 tense-flip
  skipped).
- Pass 4 burst at SHA `99563fd1` closes both Pass 3 leftovers AND Pass 4
  findings.
- `gate_pass_3.remediation_sha` stays `b1b145b3` (where Pass 3 was first
  remediated, even partially).
- `gate_pass_4.remediation_sha` is `99563fd1`.
- Pass 4's `notes` field documents that `99563fd1` also closed Pass 3
  leftovers.

The `verify-sha-currency.sh` hook's cross-record check enforces that
STATE.md frontmatter `remediation_sha` agrees with wave-state.yaml's
`gate_pass_N.remediation_sha` for every pass.

---

## Pre-Burst Hygiene

Before starting Stage 1, run `git -C .factory status`. If unrelated files
are modified (sidecar logs, etc.), either commit them separately first or
stash them. Pre-existing modifications must NOT contaminate the burst
commit.

---

## Verification Commands

Run these in sequence before pushing Stage 2:

```bash
# 1. SHA currency + burst hygiene (encapsulates 90% of the checks below)
bash .factory/hooks/verify-sha-currency.sh

# 2. No placeholders in wave-state.yaml (Stage 2 only — Stage 1 expected
#    to have them)
grep -E "TBD|TODO|FIXME|this_burst|XXX" .factory/wave-state.yaml

# 3. Pass record count matches current pass
grep -c "gate_pass_[0-9]" .factory/wave-state.yaml

# 4. next_gate_required is N+1, not N
grep "next_gate_required:" .factory/wave-state.yaml

# 5. STATE.md version bumped
grep "^version:" .factory/STATE.md
```

---

## Recovery Procedures

### A 3rd commit accidentally landed during Stage 1

1. `git -C .factory log --oneline -5` — inspect the chain.
2. `git -C .factory reset --soft HEAD~N` — N = number of accidental
   commits.
3. `git -C .factory status` — inspect ALL staged files.
4. **Unstage anything this burst did not author** (e.g., session
   sidecar logs).
5. Re-commit Stage 1 cleanly.

### Stage 2 was already pushed when drift surfaces

`git push --force-with-lease` requires human approval. Confirm before
proceeding. Document the episode in `SESSION-HANDOFF.md` "Recent Burst
Episodes" section.

---

## Failure Modes Observed (originating dogfood case study)

| Pass | What Was Missed | Root Cause |
|------|-----------------|------------|
| 1 | develop_head stale post-PR merge | Hook only checked factory-artifacts HEAD |
| 2 | Tense-flip in Stage 1 commit | No checklist forcing past-tense voice |
| 3 | Hook fired pre-push but missed multi-commit chain | Hook scope too narrow |
| 4 | Burst chain extended to 4 commits | Multi-commit detection missing |
| 5 | Per-document SHA writes fragmented citations | Single Canonical SHA Rule not yet codified |
| 6 | STATE.md frontmatter pass-3 SHA disagreed with wave-state.yaml | Cross-record verification missing |
| 7 | (CLEAN) — manual orchestrator execution proved the protocol works | Captured + shipped as this checklist |

Trajectory: **11 → 12 → 10 → 10 → 11 → 7 → 3 (CLEAN)** — six recurrences
before convergence.

---

## Non-burst Bookkeeping (incidental)

This checklist focuses on the burst-recurrence defect classes. For
generic STATE.md / SESSION-HANDOFF.md content rules (which sections to
update on phase transitions, where session checkpoints go, etc.), see
the agent prompt at `agents/state-manager.md` and the existing burst-log
template at `templates/burst-log-template.md`.
